//! The derives: `Shape`, `Options`, `LuaError`, `Object`, `Text`, `OneOf`.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitStr};

use crate::attrs::{doc_expr, module_expr, renamed, serde, shape_module};

fn no_generics(input: &DeriveInput) -> syn::Result<()> {
    if input.generics.params.is_empty() {
        Ok(())
    } else {
        Err(syn::Error::new_spanned(&input.generics, "a Lua shape has no generic parameters: name each instance"))
    }
}

/// `reg.named(...)` around a body: the type's shape, registered once.
fn named(input: &DeriveInput, body: TokenStream) -> syn::Result<TokenStream> {
    let short = input.ident.to_string();
    let module = module_expr(shape_module(&input.attrs)?.as_deref());
    let doc = doc_expr(&input.attrs);
    Ok(quote! {
        reg.named(#short, #module, ::core::any::type_name::<Self>(), |reg| {
            let _ = &reg;
            ::mlua_ir::Definition { doc: #doc, body: #body }
        })
    })
}

/// The IR of a struct's (or a variant's) named fields; `default` is the expression of a field's default value.
fn fields_ir(fields: &Fields, rename_all: Option<&str>, defaults: bool) -> syn::Result<Vec<TokenStream>> {
    let Fields::Named(named) = fields else {
        return Err(syn::Error::new_spanned(fields, "a Lua table has named fields: `{ x: f64 }`, not `(f64)`"));
    };
    let mut out = Vec::new();
    for field in &named.named {
        let attrs = serde(&field.attrs)?;
        if attrs.skip {
            continue;
        }
        if attrs.flatten {
            return Err(syn::Error::new_spanned(field, "`#[serde(flatten)]` is not supported in a Lua shape yet"));
        }
        let ident = field.ident.as_ref().map(ToString::to_string).unwrap_or_default();
        let name = attrs.rename.unwrap_or_else(|| renamed(&ident, rename_all));
        let ty = &field.ty;
        let doc = doc_expr(&field.attrs);
        let default = if defaults {
            quote! { __defaults.as_ref().and_then(|d| d.get(#name)).cloned().filter(|v| !v.is_null()) }
        } else {
            quote! { ::core::option::Option::None }
        };
        out.push(quote! {
            ::mlua_ir::ir::Field {
                name: #name.to_owned(),
                doc: #doc,
                ty: <#ty as ::mlua_ir::Shape>::shape(reg),
                default: #default,
                readonly: false,
            }
        });
    }
    Ok(out)
}

pub fn shape(input: DeriveInput) -> syn::Result<TokenStream> {
    no_generics(&input)?;
    let ident = &input.ident;
    let container = serde(&input.attrs)?;
    let body = match &input.data {
        Data::Struct(s) => {
            let fields = fields_ir(&s.fields, container.rename_all.as_deref(), false)?;
            quote! { ::mlua_ir::ir::TypeBody::Record { fields: ::std::vec![#(#fields),*] } }
        }
        Data::Enum(e) if e.variants.iter().all(|v| matches!(v.fields, Fields::Unit)) => {
            let mut variants = Vec::new();
            for v in &e.variants {
                let value = serde(&v.attrs)?.rename.unwrap_or_else(|| renamed(&v.ident.to_string(), container.rename_all.as_deref()));
                let doc = doc_expr(&v.attrs);
                variants.push(quote! { ::mlua_ir::ir::EnumVariant { value: #value.to_owned(), doc: #doc } });
            }
            quote! { ::mlua_ir::ir::TypeBody::StringEnum { variants: ::std::vec![#(#variants),*] } }
        }
        Data::Enum(e) => {
            let Some(tag) = container.tag.clone() else {
                return Err(syn::Error::new_spanned(
                    ident,
                    "an enum with data needs `#[serde(tag = \"kind\")]` (or another tag name): Luau and TypeScript tell \
                     tables apart by a string field, and serde's default shape `{ Circle = { r = 2 } }` reads badly in Lua",
                ));
            };
            variants_ir(e, &reserved_names(&tag, false), container.rename_all.as_deref(), |variants| {
                quote! { ::mlua_ir::ir::TypeBody::TaggedUnion { tag: #tag.to_owned(), variants: #variants } }
            })?
        }
        Data::Union(_) => return Err(syn::Error::new_spanned(ident, "a union has no Lua shape")),
    };
    let named = named(&input, body)?;
    Ok(quote! {
        impl ::mlua_ir::Shape for #ident {
            fn shape(reg: &mut ::mlua_ir::Registry) -> ::mlua_ir::ir::Ty { #named }
        }
        impl ::mlua_ir::SerdeShape for #ident {}
    })
}

/// The names a variant's fields may not take: the tag, and for an error the fields every error carries.
fn reserved_names(tag: &str, error: bool) -> Vec<String> {
    let mut names = vec![tag.to_owned()];
    if error {
        names.extend(["message".to_owned(), "op".to_owned()]);
    }
    names
}

fn variants_ir(
    e: &syn::DataEnum,
    reserved: &[String],
    rename_all: Option<&str>,
    wrap: impl FnOnce(TokenStream) -> TokenStream,
) -> syn::Result<TokenStream> {
    let mut variants = Vec::new();
    for v in &e.variants {
        let attrs = serde(&v.attrs)?;
        if let Fields::Named(named) = &v.fields {
            for field in &named.named {
                let field_attrs = serde(&field.attrs)?;
                let ident = field.ident.as_ref().map(ToString::to_string).unwrap_or_default();
                let name = field_attrs.rename.unwrap_or_else(|| renamed(&ident, attrs.rename_all.as_deref()));
                if reserved.contains(&name) {
                    return Err(syn::Error::new_spanned(
                        field,
                        format!(
                            "a field named `{name}` clashes with the `{name}` every value of this type carries ({}): name it something else (`reason`, `detail`)",
                            reserved.join(", ")
                        ),
                    ));
                }
            }
        }
        let tag_value = attrs.rename.unwrap_or_else(|| renamed(&v.ident.to_string(), rename_all));
        let doc = doc_expr(&v.attrs);
        let fields = match &v.fields {
            Fields::Unit => Vec::new(),
            fields @ Fields::Named(_) => fields_ir(fields, attrs.rename_all.as_deref(), false)?,
            Fields::Unnamed(_) => {
                return Err(syn::Error::new_spanned(v, "name this variant's fields: `NotFound { path: String }`"));
            }
        };
        variants.push(quote! {
            ::mlua_ir::ir::Variant { tag_value: #tag_value.to_owned(), doc: #doc, fields: ::std::vec![#(#fields),*] }
        });
    }
    Ok(wrap(quote! { ::std::vec![#(#variants),*] }))
}

pub fn options(input: DeriveInput) -> syn::Result<TokenStream> {
    no_generics(&input)?;
    let ident = &input.ident;
    let container = serde(&input.attrs)?;
    if !container.default || !container.deny_unknown_fields {
        return Err(syn::Error::new_spanned(
            ident,
            "an options table needs `#[serde(default, deny_unknown_fields)]` (written for you inside `#[module]`)",
        ));
    }
    let Data::Struct(s) = &input.data else {
        return Err(syn::Error::new_spanned(ident, "options are a struct"));
    };
    let fields = fields_ir(&s.fields, container.rename_all.as_deref(), true)?;
    let body = quote! {{
        let __defaults = ::mlua_ir::__private::serde_json::to_value(<#ident as ::core::default::Default>::default()).ok();
        ::mlua_ir::ir::TypeBody::Options { fields: ::std::vec![#(#fields),*] }
    }};
    let named = named(&input, body)?;
    Ok(quote! {
        impl ::mlua_ir::Shape for #ident {
            fn shape(reg: &mut ::mlua_ir::Registry) -> ::mlua_ir::ir::Ty { #named }
            const OPTIONAL_PARAM: bool = true;
        }
        impl ::mlua_ir::FromLuaArg for #ident {
            fn from_lua_arg(
                value: ::mlua_ir::__private::mlua::Value,
                lua: &::mlua_ir::__private::mlua::Lua,
            ) -> ::core::result::Result<Self, ::std::string::String> {
                match value {
                    ::mlua_ir::__private::mlua::Value::Nil => Ok(<Self as ::core::default::Default>::default()),
                    value => ::mlua_ir::from_checked(value, lua),
                }
            }
        }
    })
}

pub fn lua_error(input: DeriveInput) -> syn::Result<TokenStream> {
    no_generics(&input)?;
    let ident = &input.ident;
    let container = serde(&input.attrs)?;
    let Data::Enum(e) = &input.data else {
        return Err(syn::Error::new_spanned(ident, "an error is an enum: one variant per kind"));
    };
    if container.tag.as_deref() != Some("kind") {
        return Err(syn::Error::new_spanned(
            ident,
            "an error needs `#[serde(tag = \"kind\", rename_all = \"snake_case\")]`: its kinds are the declared union",
        ));
    }
    let body = variants_ir(e, &reserved_names("kind", true), container.rename_all.as_deref(), |variants| {
        quote! {
            ::mlua_ir::ir::TypeBody::Error { tag: "kind".to_owned(), variants: #variants, common: ::mlua_ir::common_fields() }
        }
    })?;
    let mut arms = Vec::new();
    for v in &e.variants {
        let vident = &v.ident;
        let kind = serde(&v.attrs)?.rename.unwrap_or_else(|| renamed(&vident.to_string(), container.rename_all.as_deref()));
        arms.push(match &v.fields {
            Fields::Unit => quote! { Self::#vident => #kind },
            _ => quote! { Self::#vident { .. } => #kind },
        });
    }
    let named = named(&input, body)?;
    Ok(quote! {
        impl ::mlua_ir::Shape for #ident {
            fn shape(reg: &mut ::mlua_ir::Registry) -> ::mlua_ir::ir::Ty { #named }
        }
        impl ::mlua_ir::IntoLuaRet for #ident {
            fn into_lua_ret(
                self,
                lua: &::mlua_ir::__private::mlua::Lua,
                op: &'static str,
            ) -> ::mlua_ir::__private::mlua::Result<::mlua_ir::__private::mlua::Value> {
                ::mlua_ir::error_value(&self, lua, op)
            }
        }
        impl ::mlua_ir::LuaError for #ident {
            fn kind(&self) -> &'static str {
                match self { #(#arms),* }
            }
        }
    })
}

pub fn object(input: DeriveInput) -> syn::Result<TokenStream> {
    no_generics(&input)?;
    let ident = &input.ident;
    let body = quote! {{
        let (fields, methods, statics) = <#ident as ::mlua_ir::ObjectDef>::describe(reg);
        ::mlua_ir::ir::TypeBody::Object { fields, methods, statics }
    }};
    let named = named(&input, body)?;
    let short = ident.to_string();
    Ok(quote! {
        impl ::mlua_ir::Shape for #ident {
            fn shape(reg: &mut ::mlua_ir::Registry) -> ::mlua_ir::ir::Ty { #named }
        }
        /// A userdata argument is the script's object, cloned (a handle: clones share their state).
        impl ::mlua_ir::FromLuaArg for #ident {
            fn from_lua_arg(
                value: ::mlua_ir::__private::mlua::Value,
                _: &::mlua_ir::__private::mlua::Lua,
            ) -> ::core::result::Result<Self, ::std::string::String> {
                match &value {
                    ::mlua_ir::__private::mlua::Value::UserData(ud) if ud.is::<Self>() => {
                        ud.borrow::<Self>().map(|r| ::core::clone::Clone::clone(&*r)).map_err(|e| e.to_string())
                    }
                    other => Err(::mlua_ir::expected(concat!("a ", #short), other)),
                }
            }
        }
        impl ::mlua_ir::IntoLuaRet for #ident {
            fn into_lua_ret(
                self,
                lua: &::mlua_ir::__private::mlua::Lua,
                _: &'static str,
            ) -> ::mlua_ir::__private::mlua::Result<::mlua_ir::__private::mlua::Value> {
                lua.create_userdata(self).map(::mlua_ir::__private::mlua::Value::UserData)
            }
        }
    })
}

pub fn text(input: DeriveInput) -> syn::Result<TokenStream> {
    no_generics(&input)?;
    let ident = &input.ident;
    let mut validate: Option<syn::Path> = None;
    for attr in input.attrs.iter().filter(|a| a.path().is_ident("text")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("validate") {
                let lit: LitStr = meta.value()?.parse()?;
                validate = Some(lit.parse()?);
                Ok(())
            } else {
                Err(meta.error("expected `validate = \"path::to::fn\"`"))
            }
        })?;
    }
    let Some(validate) = validate else {
        return Err(syn::Error::new_spanned(ident, "a text type says how it is checked: `#[text(validate = \"fn_name\")]`"));
    };
    let body = quote! { ::mlua_ir::ir::TypeBody::Alias { ty: ::mlua_ir::ir::Ty::String } };
    let named = named(&input, body)?;
    Ok(quote! {
        impl ::mlua_ir::Shape for #ident {
            fn shape(reg: &mut ::mlua_ir::Registry) -> ::mlua_ir::ir::Ty { #named }
        }
        impl ::mlua_ir::FromLuaArg for #ident {
            fn from_lua_arg(
                value: ::mlua_ir::__private::mlua::Value,
                lua: &::mlua_ir::__private::mlua::Lua,
            ) -> ::core::result::Result<Self, ::std::string::String> {
                let text = <::std::string::String as ::mlua_ir::FromLuaArg>::from_lua_arg(value, lua)?;
                #validate(&text)?;
                Ok(Self(text))
            }
        }
        impl ::mlua_ir::IntoLuaRet for #ident {
            fn into_lua_ret(
                self,
                lua: &::mlua_ir::__private::mlua::Lua,
                op: &'static str,
            ) -> ::mlua_ir::__private::mlua::Result<::mlua_ir::__private::mlua::Value> {
                ::mlua_ir::IntoLuaRet::into_lua_ret(self.0, lua, op)
            }
        }
    })
}

pub fn one_of(input: DeriveInput) -> syn::Result<TokenStream> {
    no_generics(&input)?;
    let ident = &input.ident;
    let Data::Enum(e) = &input.data else {
        return Err(syn::Error::new_spanned(ident, "`OneOf` is an enum of one-field variants: `Seconds(f64)`"));
    };
    let mut members = Vec::new();
    let mut tries = Vec::new();
    let mut arms = Vec::new();
    for v in &e.variants {
        let Fields::Unnamed(f) = &v.fields else {
            return Err(syn::Error::new_spanned(v, "each `OneOf` variant holds one type: `Seconds(f64)`"));
        };
        if f.unnamed.len() != 1 {
            return Err(syn::Error::new_spanned(v, "each `OneOf` variant holds one type: `Seconds(f64)`"));
        }
        let ty = &f.unnamed[0].ty;
        let vident = &v.ident;
        members.push(quote! { <#ty as ::mlua_ir::Shape>::shape(reg) });
        tries.push(quote! {
            match <#ty as ::mlua_ir::FromLuaArg>::from_lua_arg(value.clone(), lua) {
                Ok(v) => return Ok(Self::#vident(v)),
                Err(e) => reasons.push(e),
            }
        });
        arms.push(quote! { Self::#vident(v) => ::mlua_ir::IntoLuaRet::into_lua_ret(v, lua, op) });
    }
    let body = quote! {
        ::mlua_ir::ir::TypeBody::Alias { ty: ::mlua_ir::ir::Ty::Union { members: ::std::vec![#(#members),*] } }
    };
    let named = named(&input, body)?;
    Ok(quote! {
        impl ::mlua_ir::Shape for #ident {
            fn shape(reg: &mut ::mlua_ir::Registry) -> ::mlua_ir::ir::Ty { #named }
        }
        impl ::mlua_ir::FromLuaArg for #ident {
            fn from_lua_arg(
                value: ::mlua_ir::__private::mlua::Value,
                lua: &::mlua_ir::__private::mlua::Lua,
            ) -> ::core::result::Result<Self, ::std::string::String> {
                let mut reasons: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
                #(#tries)*
                Err(reasons.join("; or "))
            }
        }
        impl ::mlua_ir::IntoLuaRet for #ident {
            fn into_lua_ret(
                self,
                lua: &::mlua_ir::__private::mlua::Lua,
                op: &'static str,
            ) -> ::mlua_ir::__private::mlua::Result<::mlua_ir::__private::mlua::Value> {
                match self { #(#arms),* }
            }
        }
    })
}
