//! Reading what an item's attributes say: its doc, serde's names, the module it belongs to.

use heck::{ToKebabCase, ToLowerCamelCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, Expr, Lit, LitStr, Meta, Type};

/// The `///` lines, as written.
pub fn doc_lines(attrs: &[Attribute]) -> Vec<String> {
    attrs
        .iter()
        .filter(|a| a.path().is_ident("doc"))
        .filter_map(|a| match &a.meta {
            Meta::NameValue(nv) => match &nv.value {
                Expr::Lit(lit) => match &lit.lit {
                    Lit::Str(s) => Some(s.value()),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        })
        .collect()
}

/// `::mlua_ir::doc(&[...])`: the doc as an `Option<String>` expression.
pub fn doc_expr(attrs: &[Attribute]) -> TokenStream {
    let lines = doc_lines(attrs);
    quote! { ::mlua_ir::doc(&[#(#lines),*]) }
}

/// What `#[serde(...)]` says of an item or a field.
#[derive(Default)]
pub struct Serde {
    pub rename: Option<String>,
    pub rename_all: Option<String>,
    pub tag: Option<String>,
    pub skip: bool,
    pub default: bool,
    pub deny_unknown_fields: bool,
    pub untagged: bool,
    pub flatten: bool,
}

pub fn serde(attrs: &[Attribute]) -> syn::Result<Serde> {
    let mut out = Serde::default();
    for attr in attrs.iter().filter(|a| a.path().is_ident("serde")) {
        attr.parse_nested_meta(|meta| {
            let name = meta.path.get_ident().map(ToString::to_string).unwrap_or_default();
            let value = |meta: &syn::meta::ParseNestedMeta| -> syn::Result<Option<String>> {
                if meta.input.peek(syn::Token![=]) {
                    let lit: LitStr = meta.value()?.parse()?;
                    Ok(Some(lit.value()))
                } else if meta.input.peek(syn::token::Paren) {
                    meta.parse_nested_meta(|inner| {
                        if inner.input.peek(syn::Token![=]) {
                            let _: Expr = inner.value()?.parse()?;
                        }
                        Ok(())
                    })?;
                    Ok(None)
                } else {
                    Ok(None)
                }
            };
            match name.as_str() {
                "rename" => out.rename = value(&meta)?,
                "rename_all" => out.rename_all = value(&meta)?,
                "tag" => out.tag = value(&meta)?,
                "skip" | "skip_deserializing" => {
                    out.skip = true;
                    value(&meta)?;
                }
                "default" => {
                    out.default = true;
                    value(&meta)?;
                }
                "deny_unknown_fields" => out.deny_unknown_fields = true,
                "untagged" => out.untagged = true,
                "flatten" => out.flatten = true,
                _ => {
                    value(&meta)?;
                }
            }
            Ok(())
        })?;
    }
    Ok(out)
}

/// The name serde gives `ident` under `rename_all` (fields are snake_case idents, variants PascalCase).
pub fn renamed(ident: &str, rename_all: Option<&str>) -> String {
    let ident = ident.strip_prefix("r#").unwrap_or(ident);
    match rename_all {
        Some("lowercase") => ident.to_lowercase(),
        Some("UPPERCASE") => ident.to_uppercase(),
        Some("PascalCase") => ident.to_upper_camel_case(),
        Some("camelCase") => ident.to_lower_camel_case(),
        Some("snake_case") => ident.to_snake_case(),
        Some("SCREAMING_SNAKE_CASE") => ident.to_shouty_snake_case(),
        Some("kebab-case") => ident.to_kebab_case(),
        Some("SCREAMING-KEBAB-CASE") => ident.to_shouty_kebab_case(),
        _ => ident.to_owned(),
    }
}

/// `#[shape(module = "std:geo")]`, which `#[module]` writes onto the types inside it.
pub fn shape_module(attrs: &[Attribute]) -> syn::Result<Option<String>> {
    let mut module = None;
    for attr in attrs.iter().filter(|a| a.path().is_ident("shape")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("module") {
                let lit: LitStr = meta.value()?.parse()?;
                module = Some(lit.value());
                Ok(())
            } else {
                Err(meta.error("expected `module = \"...\"`"))
            }
        })?;
    }
    Ok(module)
}

/// The module as an `Option<&str>` expression.
pub fn module_expr(module: Option<&str>) -> TokenStream {
    match module {
        Some(m) => quote! { ::core::option::Option::Some(#m) },
        None => quote! { ::core::option::Option::None },
    }
}

/// The derives an item lists (last path segment: `Shape`, `Serialize`, ...).
pub fn derives(attrs: &[Attribute]) -> Vec<String> {
    let mut names = Vec::new();
    for attr in attrs.iter().filter(|a| a.path().is_ident("derive")) {
        let _ = attr.parse_nested_meta(|meta| {
            if let Some(last) = meta.path.segments.last() {
                names.push(last.ident.to_string());
            }
            Ok(())
        });
    }
    names
}

/// The parameter names of the first function-pointer type inside `ty`: `Callback<fn(left: T, right: T) -> R>`.
pub fn callback_names(ty: &Type) -> Vec<String> {
    match ty {
        Type::BareFn(f) => f
            .inputs
            .iter()
            .enumerate()
            .map(|(i, arg)| arg.name.as_ref().map_or_else(|| format!("arg{}", i + 1), |(name, _)| name.to_string()))
            .collect(),
        Type::Path(p) => p
            .path
            .segments
            .iter()
            .filter_map(|s| match &s.arguments {
                syn::PathArguments::AngleBracketed(args) => Some(args),
                _ => None,
            })
            .flat_map(|args| args.args.iter())
            .find_map(|arg| match arg {
                syn::GenericArgument::Type(inner) => Some(callback_names(inner)).filter(|n| !n.is_empty()),
                _ => None,
            })
            .unwrap_or_default(),
        Type::Paren(p) => callback_names(&p.elem),
        Type::Group(g) => callback_names(&g.elem),
        _ => Vec::new(),
    }
}

/// Takes `#[host("hint")]` off an item: the opaque word a host may read (`mlua_ir::Call::hint`).
pub fn take_host_hint(attrs: &mut Vec<Attribute>) -> syn::Result<Option<String>> {
    let mut hint = None;
    for attr in attrs.iter().filter(|a| a.path().is_ident("host")) {
        hint = Some(attr.parse_args::<LitStr>()?.value());
    }
    attrs.retain(|a| !a.path().is_ident("host"));
    Ok(hint)
}
