//! mlua-ir: write a Lua API as plain Rust, and get one intermediate representation of it that declaration
//! generators (Luau, TypeScript, anything) read.
//!
//! - The macros ([`module`], [`object`], the derives) gather names, docs and signatures.
//! - The traits ([`Shape`], [`IntoReturn`], [`FromLuaArg`], [`IntoLuaRet`]) work out every type, through
//!   generics and other crates' types, which a macro cannot see.
//! - [`schema`] collects every linked module into an [`ir::Ir`]; [`install`] puts them into a Lua state.

pub mod ir;

mod callback;
mod convert;
mod error;
mod foreign;
mod harvest;
mod object;
mod registry;
mod returns;
mod runtime;
mod shape;
mod stream;
mod unknown;

pub use callback::{Callback, name_params};
pub use convert::{FromLuaArg, IntoLuaRet, SerdeShape, expected, from_serde, to_serde};
pub use error::{Caught, LuaError, Raise, Raised, common_fields, error_value};
pub use foreign::Foreign;
pub use object::ObjectDef;
pub use registry::{Definition, Registry};
pub use returns::IntoReturn;
pub use runtime::{ModuleEntry, arg, doc, install, no_more_args, param, schema};
pub use shape::Shape;
pub use stream::Stream;
pub use unknown::{Unknown, UnknownReason};

pub use mlua_ir_macros::{LuaError, Object, OneOf, Options, Shape, Text, module, object};

/// `Send` when the `send` feature (mlua's `send`) is on, and nothing otherwise: what a stream, its items and an
/// async function's future must be.
#[cfg(feature = "send")]
pub trait MaybeSend: Send {}
#[cfg(feature = "send")]
impl<T: Send> MaybeSend for T {}

/// `Send` when the `send` feature (mlua's `send`) is on, and nothing otherwise.
#[cfg(not(feature = "send"))]
pub trait MaybeSend {}
#[cfg(not(feature = "send"))]
impl<T> MaybeSend for T {}

#[doc(hidden)]
pub mod __private {
    pub use inventory;
    pub use mlua;
    pub use serde_json;
}
