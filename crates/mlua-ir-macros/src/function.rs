//! A Rust function as a Lua function: its IR, and the mlua closure that reads its arguments and returns its result.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, Pat, ReturnType, Signature, Type};

use crate::attrs::callback_names;

pub struct Param {
    pub name: String,
    pub ty: Type,
}

/// How a function asked for the Lua state: `lua: &Lua` or `lua: Lua`.
#[derive(Clone, Copy, PartialEq)]
pub enum Context {
    Ref,
    Owned,
}

pub struct Sig {
    /// The state parameter, if the function takes one: first, and not part of the script's signature.
    pub context: Option<Context>,
    pub params: Vec<Param>,
    pub ret: Type,
    pub is_async: bool,
}

fn context_of(ty: &Type) -> Option<Context> {
    let is_lua = |ty: &Type| matches!(ty, Type::Path(p) if p.qself.is_none() && p.path.segments.last().is_some_and(|s| s.ident == "Lua"));
    match ty {
        Type::Reference(r) if r.mutability.is_none() && is_lua(&r.elem) => Some(Context::Ref),
        ty if is_lua(ty) => Some(Context::Owned),
        _ => None,
    }
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
    let mut context = None;
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
                if let Some(kind) = context_of(&pat.ty) {
                    if !params.is_empty() || context.is_some() {
                        return Err(syn::Error::new_spanned(&pat.ty, "the Lua state is the first parameter"));
                    }
                    if kind == Context::Ref && sig.asyncness.is_some() {
                        return Err(syn::Error::new_spanned(&pat.ty, "an async function takes the state owned: `lua: Lua`"));
                    }
                    context = Some(kind);
                    continue;
                }
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
    Ok((receiver, Sig { context, params, ret, is_async: sig.asyncness.is_some() }))
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
pub fn read_args(sig: &Sig, op: &str, lua: &TokenStream) -> (TokenStream, Vec<TokenStream>) {
    let mut lets = Vec::new();
    let mut idents: Vec<TokenStream> = Vec::new();
    match sig.context {
        Some(Context::Ref) => idents.push(quote!(#lua)),
        Some(Context::Owned) => idents.push(quote!(__ctx)),
        None => {}
    }
    for (i, p) in sig.params.iter().enumerate() {
        let ident = format_ident!("__arg{}", i + 1);
        let ty = &p.ty;
        let pname = &p.name;
        let index = i + 1;
        lets.push(quote! { let #ident: #ty = ::mlua_ir::arg(#lua, __args.next(), #op, #index, #pname)?; });
        idents.push(quote!(#ident));
    }
    let count = sig.params.len();
    let own = matches!(sig.context, Some(Context::Owned)).then(|| quote!(let __ctx = ::core::clone::Clone::clone(#lua);));
    let body = quote! {
        #own
        let mut __args = __args.into_iter();
        #(#lets)*
        ::mlua_ir::no_more_args(__args, #op, #count)?;
    };
    (body, idents)
}

/// The `&Call` a host is told of: `module` as written, `name` the function (type first for a static or a method).
pub fn call_info(module: Option<&str>, name: &str, op: &str, hint: Option<&str>) -> TokenStream {
    let module = match module {
        Some(m) => quote!(::core::option::Option::Some(#m)),
        None => quote!(::core::option::Option::None),
    };
    let hint = match hint {
        Some(h) => quote!(::core::option::Option::Some(#h)),
        None => quote!(::core::option::Option::None),
    };
    quote! { &::mlua_ir::Call { module: #module, name: #name, op: #op, hint: #hint } }
}

/// A free function (or a static) as `lua.create_function(...)` / `create_async_function(...)`.
pub fn lua_function(call: &TokenStream, call_info: &TokenStream, sig: &Sig, op: &str) -> TokenStream {
    if sig.is_async {
        let (read, args) = read_args(sig, op, &quote!(&__lua));
        quote! {
            lua.create_async_function(|__lua: ::mlua_ir::__private::mlua::Lua, __args: ::mlua_ir::__private::mlua::MultiValue| async move {
                #read
                let __ret = ::mlua_ir::__private::call_async(#call_info, #call(#(#args),*)).await?;
                ::mlua_ir::IntoReturn::into_return(__ret, &__lua, #op)
            })?
        }
    } else {
        let (read, args) = read_args(sig, op, &quote!(__lua));
        quote! {
            lua.create_function(|__lua: &::mlua_ir::__private::mlua::Lua, __args: ::mlua_ir::__private::mlua::MultiValue| {
                #read
                ::mlua_ir::__private::call_sync(#call_info, || {
                    ::mlua_ir::IntoReturn::into_return(#call(#(#args),*), __lua, #op)
                })
            })?
        }
    }
}
