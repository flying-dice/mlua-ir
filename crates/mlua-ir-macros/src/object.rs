//! `#[object] impl T { ... }`: `&self` fns are `:` methods, `#[get]` fns are read-only fields, a `#[to_string]` fn is `tostring(x)`, fns without
//! `self` are statics (constructors), `async fn` yields the script while it waits.

use proc_macro2::TokenStream;
use syn::ext::IdentExt;
use quote::quote;
use syn::{ImplItem, ItemImpl, LitStr, Type, Visibility};

use crate::attrs::{doc_expr, take_host_hint};
use crate::function::{self, Receiver};

pub fn expand(args: TokenStream, mut block: ItemImpl) -> syn::Result<TokenStream> {
    let mut module: Option<String> = None;
    if !args.is_empty() {
        let parser = syn::meta::parser(|meta| {
            if meta.path.is_ident("module") {
                let lit: LitStr = meta.value()?.parse()?;
                module = Some(lit.value());
                Ok(())
            } else {
                Err(meta.error("expected `module = \"...\"`"))
            }
        });
        syn::parse::Parser::parse2(parser, args)?;
    }
    let self_ty = (*block.self_ty).clone();
    let short = match &self_ty {
        Type::Path(p) => p.path.segments.last().map(|s| s.ident.to_string()).unwrap_or_default(),
        other => return Err(syn::Error::new_spanned(other, "`#[object]` goes on `impl SomeType`")),
    };
    // The name its declaration has (`std_time_Duration`): `typeof(x)` returns a userdata's `__type`.
    let type_name = match &module {
        Some(m) => format!("{}_{short}", m.replace([':', '.', '-', '/'], "_")),
        None => short.clone(),
    };
    let base = match &module {
        Some(m) => format!("{m}.{short}"),
        None => short.clone(),
    };

    let mut fields_ir = Vec::new();
    let mut fields_install = Vec::new();
    let mut methods_ir = Vec::new();
    let mut methods_install = Vec::new();
    let mut to_string_install = Vec::new();
    let mut statics_ir = Vec::new();
    let mut statics_install = Vec::new();

    for item in &mut block.items {
        let ImplItem::Fn(f) = item else { continue };
        let is_getter = f.attrs.iter().any(|a| a.path().is_ident("get"));
        let is_to_string = f.attrs.iter().any(|a| a.path().is_ident("to_string"));
        f.attrs.retain(|a| !a.path().is_ident("get") && !a.path().is_ident("to_string"));
        if !matches!(f.vis, Visibility::Public(_)) {
            continue;
        }
        let hint = take_host_hint(&mut f.attrs)?;
        let ident = f.sig.ident.clone();
        let name = ident.unraw().to_string();
        let qualified = format!("{short}.{name}");
        let doc = doc_expr(&f.attrs);
        let (receiver, sig) = function::read(&f.sig)?;
        match receiver {
            Receiver::Ref if is_to_string => {
                if !sig.params.is_empty() || sig.is_async || !matches!(&sig.ret, Type::Path(p) if p.path.is_ident("String")) {
                    return Err(syn::Error::new_spanned(&f.sig, "a `#[to_string]` method takes only `&self`, is not async, and returns `String`"));
                }
                let op = format!("{base}:{name}");
                to_string_install.push(quote! {
                    methods.add_meta_method(::mlua_ir::__private::mlua::MetaMethod::ToString, |__lua, this, (): ()| {
                        ::mlua_ir::IntoLuaRet::into_lua_ret(this.#ident(), __lua, #op)
                    });
                });
            }
            Receiver::Ref if is_getter => {
                if !sig.params.is_empty() || sig.is_async {
                    return Err(syn::Error::new_spanned(&f.sig, "a `#[get]` field takes only `&self` and is not async"));
                }
                let ret = &sig.ret;
                let op = format!("{base}.{name}");
                fields_ir.push(quote! {
                    ::mlua_ir::ir::Field {
                        name: #name.to_owned(),
                        doc: #doc,
                        ty: <#ret as ::mlua_ir::Shape>::shape(reg),
                        default: ::core::option::Option::None,
                        readonly: true,
                    }
                });
                fields_install.push(quote! {
                    fields.add_field_method_get(#name, |__lua, this| ::mlua_ir::IntoLuaRet::into_lua_ret(this.#ident(), __lua, #op));
                });
            }
            Receiver::Ref | Receiver::Mut => {
                let op = format!("{base}:{name}");
                methods_ir.push(function::ir(&name, &doc, &sig));
                let mutable = matches!(receiver, Receiver::Mut);
                methods_install.push(if sig.is_async {
                    if mutable {
                        return Err(syn::Error::new_spanned(&f.sig, "an async method takes `&self`"));
                    }
                    let (read, args) = function::read_args(&sig, &op, &quote!(&__lua));
                    let info = function::call_info(module.as_deref(), &qualified, &op, hint.as_deref());
                    quote! {
                        methods.add_async_method(#name, |__lua, this: ::mlua_ir::__private::mlua::UserDataRef<Self>, __args: ::mlua_ir::__private::mlua::MultiValue| async move {
                            #read
                            let __ret = ::mlua_ir::__private::call_async(#info, async move { this.#ident(#(#args),*).await }).await?;
                            ::mlua_ir::IntoReturn::into_return(__ret, &__lua, #op)
                        });
                    }
                } else {
                    let (read, args) = function::read_args(&sig, &op, &quote!(__lua));
                    let add = if mutable { quote!(add_method_mut) } else { quote!(add_method) };
                    let info = function::call_info(module.as_deref(), &qualified, &op, hint.as_deref());
                    quote! {
                        methods.#add(#name, |__lua, this, __args: ::mlua_ir::__private::mlua::MultiValue| {
                            #read
                            ::mlua_ir::__private::call_sync(#info, || {
                                ::mlua_ir::IntoReturn::into_return(this.#ident(#(#args),*), __lua, #op)
                            })
                        });
                    }
                });
            }
            Receiver::None => {
                let op = format!("{base}.{name}");
                statics_ir.push(function::ir(&name, &doc, &sig));
                let info = function::call_info(module.as_deref(), &qualified, &op, hint.as_deref());
                let value = function::lua_function(&quote!(<#self_ty>::#ident), &info, &sig, &op);
                statics_install.push(quote! { table.raw_set(#name, #value)?; });
            }
        }
    }

    Ok(quote! {
        #block

        impl ::mlua_ir::ObjectDef for #self_ty {
            fn describe(
                reg: &mut ::mlua_ir::Registry,
            ) -> (::std::vec::Vec<::mlua_ir::ir::Field>, ::std::vec::Vec<::mlua_ir::ir::Function>, ::std::vec::Vec<::mlua_ir::ir::Function>) {
                (::std::vec![#(#fields_ir),*], ::std::vec![#(#methods_ir),*], ::std::vec![#(#statics_ir),*])
            }

            fn statics(
                lua: &::mlua_ir::__private::mlua::Lua,
            ) -> ::mlua_ir::__private::mlua::Result<::mlua_ir::__private::mlua::Table> {
                let table = lua.create_table()?;
                #(#statics_install)*
                table.set_readonly(true);
                Ok(table)
            }
        }

        impl ::mlua_ir::__private::mlua::UserData for #self_ty {
            fn add_fields<F: ::mlua_ir::__private::mlua::UserDataFields<Self>>(fields: &mut F) {
                fields.add_meta_field("__type", #type_name);
                #(#fields_install)*
            }

            fn add_methods<M: ::mlua_ir::__private::mlua::UserDataMethods<Self>>(methods: &mut M) {
                let _ = &methods;
                #(#methods_install)*
                #(#to_string_install)*
            }
        }
    })
}
