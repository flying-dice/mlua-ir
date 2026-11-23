//! The derives and attributes of mlua-ir. They only gather names, docs and signatures; every type is worked out by
//! mlua-ir's traits, which can see what a macro cannot.

use proc_macro::TokenStream;
use syn::{DeriveInput, ItemImpl, ItemMod, LitStr, parse_macro_input};

mod attrs;
mod derive;
mod function;
mod module;
mod object;

fn emit(result: syn::Result<proc_macro2::TokenStream>) -> TokenStream {
    result.unwrap_or_else(syn::Error::into_compile_error).into()
}

/// `#[module("std:text")] pub mod text { ... }` exports every `pub fn`, `pub const` and `pub` userdata type inside;
/// `#[module(globals)]` installs them as globals.
#[proc_macro_attribute]
pub fn module(args: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemMod);
    let args2: proc_macro2::TokenStream = args.into();
    let kind = if args2.to_string() == "globals" {
        Ok(module::Kind::Globals)
    } else {
        syn::parse2::<LitStr>(args2).map(|lit| module::Kind::Require(lit.value()))
    };
    emit(kind.and_then(|kind| module::expand(kind, item)))
}

/// `#[object] impl T { ... }`: the methods, fields (`#[get]`) and statics of a userdata.
#[proc_macro_attribute]
pub fn object(args: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemImpl);
    emit(object::expand(args.into(), item))
}

/// A struct (a table), a unit enum (a string union) or a tagged enum (a discriminated union), through serde.
#[proc_macro_derive(Shape, attributes(shape))]
pub fn derive_shape(input: TokenStream) -> TokenStream {
    emit(derive::shape(parse_macro_input!(input as DeriveInput)))
}

/// A trailing options table: optional, defaults from `Default`, unknown keys refused.
#[proc_macro_derive(Options, attributes(shape))]
pub fn derive_options(input: TokenStream) -> TokenStream {
    emit(derive::options(parse_macro_input!(input as DeriveInput)))
}

/// An error enum: each variant a kind, its fields the error's fields.
#[proc_macro_derive(LuaError, attributes(shape))]
pub fn derive_lua_error(input: TokenStream) -> TokenStream {
    emit(derive::lua_error(parse_macro_input!(input as DeriveInput)))
}

/// A userdata; its members come from `#[object] impl`.
#[proc_macro_derive(Object, attributes(shape))]
pub fn derive_object(input: TokenStream) -> TokenStream {
    emit(derive::object(parse_macro_input!(input as DeriveInput)))
}

/// A validated string: `#[text(validate = "fn")] struct Name(String);`.
#[proc_macro_derive(Text, attributes(shape, text))]
pub fn derive_text(input: TokenStream) -> TokenStream {
    emit(derive::text(parse_macro_input!(input as DeriveInput)))
}

/// An argument that may be any of several types: `enum Timeout { Duration(Duration), Seconds(f64) }`.
#[proc_macro_derive(OneOf, attributes(shape))]
pub fn derive_one_of(input: TokenStream) -> TokenStream {
    emit(derive::one_of(parse_macro_input!(input as DeriveInput)))
}
