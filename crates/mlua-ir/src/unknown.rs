//! [`Unknown`]: the one allowed "anything", and it must say why.
//!
//! `Unknown<R>` is declared `unknown` (never `any`): a script has to check what it got before using it. `R` names
//! the reason, so an `Unknown` without one does not compile:
//!
//! ```ignore
//! mlua_ir::unknown_reason!(JsonText = "whatever JSON text was parsed: its shape is the text's, not the API's");
//! pub fn decode(text: String) -> Result<Unknown<JsonText>, JsonError> { ... }
//! ```

use std::marker::PhantomData;

use mlua::{Lua, Value};

use crate::convert::{FromLuaArg, IntoLuaRet, from_serde, to_serde};
use crate::ir::Ty;
use crate::registry::Registry;
use crate::shape::Shape;

/// Why a value's shape cannot be known: see [`unknown_reason!`](crate::unknown_reason).
pub trait UnknownReason {
    const REASON: &'static str;
}

/// Declares a reason for [`Unknown`]: `unknown_reason!(Name = "why no shape exists");`.
#[macro_export]
macro_rules! unknown_reason {
    ($(#[$meta:meta])* $name:ident = $reason:literal) => {
        $(#[$meta])*
        pub enum $name {}
        impl $crate::UnknownReason for $name {
            const REASON: &'static str = $reason;
        }
    };
}

/// Plain data of a shape no one controls.
pub struct Unknown<R> {
    pub value: serde_json::Value,
    _reason: PhantomData<R>,
}

impl<R> Unknown<R> {
    pub fn new(value: serde_json::Value) -> Self {
        Self { value, _reason: PhantomData }
    }
}

impl<R: UnknownReason> Shape for Unknown<R> {
    fn shape(_: &mut Registry) -> Ty {
        Ty::Unknown { reason: R::REASON.to_owned() }
    }
}

impl<R> FromLuaArg for Unknown<R> {
    fn from_lua_arg(value: Value, lua: &Lua) -> Result<Self, String> {
        from_serde(value, lua).map(Self::new)
    }
}

impl<R> IntoLuaRet for Unknown<R> {
    fn into_lua_ret(self, lua: &Lua, _: &'static str) -> mlua::Result<Value> {
        to_serde(&self.value, lua)
    }
}
