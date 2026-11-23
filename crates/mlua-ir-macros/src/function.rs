//! A Rust function as a Lua function: its IR, and the mlua closure that reads its arguments and returns its result.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, Pat, ReturnType, Signature, Type};

use crate::attrs::callback_names;

pub struct Param {
    pub name: String,
    pub ty: Type,
}

pub struct Sig {
    pub params: Vec<Param>,
    pub ret: Type,
    pub is_async: bool,
}

pub enum Receiver {
    None,
    Ref,
    Mut,
}

/// Reads a signature: owned, named parameters only (a script's value cannot be borrowed past the call).
pub fn read(sig: &Signature) -> syn::Result<(Receiver, Sig)> {
    let mut receiver = Receiver::None;
    let mut params = Vec::new();
    for input in &sig.inputs {
        match input {
            FnArg::Receiver(r) => {
                if r.reference.is_none() {
                    return Err(syn::Error::new_spanned(r, "a Lua method takes `&self` or `&mut self`"));
                }
                receiver = if r.mutability.is_some() { Receiver::Mut } else { Receiver::Ref };
            }
            FnArg::Typed(pat) => {
                let Pat::Ident(ident) = &*pat.pat else {
                    return Err(syn::Error::new_spanned(&pat.pat, "a Lua function's parameters are plain names"));
                };
                if let Type::Reference(r) = &*pat.ty {
                    return Err(syn::Error::new_spanned(
                        r,
                        "take an owned value (`String`, not `&str`): a script's value is read before the call",
                    ));
                }
                params.push(Param { name: ident.ident.to_string(), ty: (*pat.ty).clone() });
            }
        }
    }
    let ret = match &sig.output {
        ReturnType::Default => syn::parse_quote!(()),
        ReturnType::Type(_, ty) => (**ty).clone(),
    };
    Ok((receiver, Sig { params, ret, is_async: sig.asyncness.is_some() }))
}

/// The function's IR: `::mlua_ir::ir::Function { ... }` (needs `reg` in scope).
pub fn ir(name: &str, doc: &TokenStream, sig: &Sig) -> TokenStream {
    let ret = &sig.ret;
    let is_async = sig.is_async;
    let params = sig.params.iter().map(|p| {
        let ty = &p.ty;
        let pname = &p.name;
        let names = callback_names(ty);
        quote! { ::mlua_ir::param::<#ty>(reg, #pname, &[#(#names),*]) }
    });
    quote! {
        ::mlua_ir::ir::Function {
            name: #name.to_owned(),
            doc: #doc,
            params: ::std::vec![#(#params),*],
            returns: <#ret as ::mlua_ir::IntoReturn>::returns(reg),
            raises: <#ret as ::mlua_ir::IntoReturn>::raises(reg),
            is_async: #is_async,
        }
    }
}

/// `let __arg1: T = ::mlua_ir::arg(...)?; ...` and the names to call with.
pub fn read_args(sig: &Sig, op: &str, lua: &TokenStream) -> (TokenStream, Vec<syn::Ident>) {
    let mut lets = Vec::new();
    let mut idents = Vec::new();
    for (i, p) in sig.params.iter().enumerate() {
        let ident = format_ident!("__arg{}", i + 1);
        let ty = &p.ty;
        let pname = &p.name;
        let index = i + 1;
        lets.push(quote! { let #ident: #ty = ::mlua_ir::arg(#lua, __args.next(), #op, #index, #pname)?; });
        idents.push(ident);
    }
    let count = sig.params.len();
    let body = quote! {
        let mut __args = __args.into_iter();
        #(#lets)*
        ::mlua_ir::no_more_args(__args, #op, #count)?;
    };
    (body, idents)
}

/// A free function (or a static) as `lua.create_function(...)` / `create_async_function(...)`.
pub fn lua_function(call: &TokenStream, sig: &Sig, op: &str) -> TokenStream {
    if sig.is_async {
        let (read, args) = read_args(sig, op, &quote!(&__lua));
        quote! {
            lua.create_async_function(|__lua: ::mlua_ir::__private::mlua::Lua, __args: ::mlua_ir::__private::mlua::MultiValue| async move {
                #read
                ::mlua_ir::IntoReturn::into_return(#call(#(#args),*).await, &__lua, #op)
            })?
        }
    } else {
        let (read, args) = read_args(sig, op, &quote!(__lua));
        quote! {
            lua.create_function(|__lua: &::mlua_ir::__private::mlua::Lua, __args: ::mlua_ir::__private::mlua::MultiValue| {
                #read
                ::mlua_ir::IntoReturn::into_return(#call(#(#args),*), __lua, #op)
            })?
        }
    }
}
