//! `#[module("std:text")] pub mod text { ... }`: every `pub fn`, `pub const` and userdata type inside is the
//! module's surface. Nothing is registered by hand.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Item, ItemMod, LitStr, Visibility, parse_quote};

use crate::attrs::{derives, doc_expr, serde};
use crate::function;

pub enum Kind {
    Require(String),
    Globals,
}

pub fn expand(kind: Kind, mut module: ItemMod) -> syn::Result<TokenStream> {
    let Some((_, items)) = module.content.as_mut() else {
        return Err(syn::Error::new_spanned(&module, "`#[module]` goes on a module with a body: `mod text { ... }`"));
    };
    let (name, kind_tokens, module_name) = match &kind {
        Kind::Require(name) => (name.clone(), quote!(::mlua_ir::ir::ModuleKind::Require), Some(name.clone())),
        Kind::Globals => (module.ident.to_string(), quote!(::mlua_ir::ir::ModuleKind::Globals), None),
    };
    let op_prefix = match &kind {
        Kind::Require(name) => format!("{name}."),
        Kind::Globals => String::new(),
    };

    let mut functions_ir = Vec::new();
    let mut functions_install = Vec::new();
    let mut constants_ir = Vec::new();
    let mut constants_install = Vec::new();
    let mut objects_ir = Vec::new();
    let mut objects_install = Vec::new();

    for item in items.iter_mut() {
        match item {
            Item::Struct(s) => {
                let names = derives(&s.attrs);
                tag_type(&mut s.attrs, &names, module_name.as_deref(), true)?;
                if names.iter().any(|d| d == "Object") && matches!(s.vis, Visibility::Public(_)) {
                    let ident = &s.ident;
                    let short = ident.to_string();
                    // Exported under its name only when it has statics: a type a script only receives has no table.
                    objects_ir.push(quote! {{
                        let ty = <#ident as ::mlua_ir::Shape>::shape(reg);
                        let (_, _, statics) = <#ident as ::mlua_ir::ObjectDef>::describe(reg);
                        (!statics.is_empty()).then(|| ::mlua_ir::ir::ObjectExport {
                            name: #short.to_owned(),
                            type_name: ty.name().unwrap_or_default().to_owned(),
                        })
                    }});
                    objects_install.push(quote! {{
                        let statics = <#ident as ::mlua_ir::ObjectDef>::statics(lua)?;
                        if statics.pairs::<::mlua_ir::__private::mlua::Value, ::mlua_ir::__private::mlua::Value>().next().is_some() {
                            table.raw_set(#short, statics)?;
                        }
                    }});
                }
            }
            Item::Enum(e) => {
                let names = derives(&e.attrs);
                let tagged = serde(&e.attrs)?.tag.is_some();
                tag_type(&mut e.attrs, &names, module_name.as_deref(), tagged)?;
            }
            Item::Impl(i) => {
                for attr in &mut i.attrs {
                    let is_object = attr.path().segments.last().is_some_and(|s| s.ident == "object");
                    if is_object && let Some(m) = &module_name {
                        *attr = parse_quote!(#[::mlua_ir::object(module = #m)]);
                    }
                }
            }
            Item::Fn(f) if matches!(f.vis, Visibility::Public(_)) => {
                let ident = &f.sig.ident;
                let fname = ident.to_string();
                let op = format!("{op_prefix}{fname}");
                let (receiver, sig) = function::read(&f.sig)?;
                if !matches!(receiver, function::Receiver::None) {
                    return Err(syn::Error::new_spanned(&f.sig, "a module function has no `self`"));
                }
                let doc = doc_expr(&f.attrs);
                functions_ir.push(function::ir(&fname, &doc, &sig));
                let value = function::lua_function(&quote!(#ident), &sig, &op);
                functions_install.push(quote! { table.raw_set(#fname, #value)?; });
            }
            Item::Const(c) if matches!(c.vis, Visibility::Public(_)) => {
                let ident = &c.ident;
                let cname = ident.to_string();
                let ty = &c.ty;
                let doc = doc_expr(&c.attrs);
                let op = format!("{op_prefix}{cname}");
                constants_ir.push(quote! {
                    ::mlua_ir::ir::Constant { name: #cname.to_owned(), doc: #doc, ty: <#ty as ::mlua_ir::Shape>::shape(reg) }
                });
                constants_install.push(quote! {
                    table.raw_set(#cname, ::mlua_ir::IntoLuaRet::into_lua_ret(#ident, lua, #op)?)?;
                });
            }
            _ => {}
        }
    }

    let doc = doc_expr(&module.attrs);
    let (enter, leave) = match &kind {
        Kind::Require(name) => (quote!(reg.enter_module(#name);), quote!(reg.leave_module();)),
        Kind::Globals => (quote!(), quote!()),
    };
    let generated: Vec<Item> = vec![
        parse_quote! {
            #[doc(hidden)]
            pub fn __mlua_ir_describe(reg: &mut ::mlua_ir::Registry) -> ::mlua_ir::ir::Module {
                #enter
                let module = ::mlua_ir::ir::Module {
                    name: #name.to_owned(),
                    kind: #kind_tokens,
                    doc: #doc,
                    functions: ::std::vec![#(#functions_ir),*],
                    constants: ::std::vec![#(#constants_ir),*],
                    objects: {
                        let all: ::std::vec::Vec<::core::option::Option<::mlua_ir::ir::ObjectExport>> =
                            ::std::vec![#(#objects_ir),*];
                        all.into_iter().flatten().collect()
                    },
                };
                #leave
                module
            }
        },
        parse_quote! {
            #[doc(hidden)]
            pub fn __mlua_ir_install(
                lua: &::mlua_ir::__private::mlua::Lua,
            ) -> ::mlua_ir::__private::mlua::Result<::mlua_ir::__private::mlua::Table> {
                let table = lua.create_table()?;
                #(#functions_install)*
                #(#constants_install)*
                #(#objects_install)*
                table.set_readonly(true);
                Ok(table)
            }
        },
        parse_quote! {
            ::mlua_ir::__private::inventory::submit! {
                ::mlua_ir::ModuleEntry {
                    name: #name,
                    kind: #kind_tokens,
                    describe: __mlua_ir_describe,
                    install: __mlua_ir_install,
                }
            }
        },
    ];
    items.extend(generated);
    Ok(quote!(#module))
}

/// Writes `#[shape(module = ...)]` onto a type that derives one of ours, and the serde strictness its shape
/// promises: a record refuses unknown keys, an options table also fills what is missing from `Default`.
fn tag_type(attrs: &mut Vec<syn::Attribute>, names: &[String], module: Option<&str>, strict: bool) -> syn::Result<()> {
    const OURS: [&str; 6] = ["Shape", "Options", "LuaError", "Object", "Text", "OneOf"];
    if !names.iter().any(|d| OURS.contains(&d.as_str())) {
        return Ok(());
    }
    if let Some(m) = module {
        let lit = LitStr::new(m, proc_macro2::Span::call_site());
        attrs.push(parse_quote!(#[shape(module = #lit)]));
    }
    let existing = serde(attrs)?;
    if names.iter().any(|d| d == "Options") {
        if !existing.default {
            attrs.push(parse_quote!(#[serde(default)]));
        }
        if !existing.deny_unknown_fields {
            attrs.push(parse_quote!(#[serde(deny_unknown_fields)]));
        }
    } else if names.iter().any(|d| d == "Shape") && strict && !existing.deny_unknown_fields {
        attrs.push(parse_quote!(#[serde(deny_unknown_fields)]));
    }
    Ok(())
}
