//! [`Bytes`]: a Lua string as raw bytes, in and out (declared `string`; a `String` argument refuses non-UTF-8).

use std::ops::Deref;

use mlua::{Lua, Value};

use crate::convert::{FromLuaArg, IntoLuaRet, expected};
use crate::ir::Ty;
use crate::registry::Registry;
use crate::shape::Shape;

/// Binary data: base64 in, a hash out, a file's content.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Bytes(pub Vec<u8>);

impl Deref for Bytes {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.0
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(bytes: Vec<u8>) -> Self {
        Bytes(bytes)
    }
}

impl Shape for Bytes {
    fn shape(_: &mut Registry) -> Ty {
        Ty::String
    }
}

impl FromLuaArg for Bytes {
    fn from_lua_arg(value: Value, _: &Lua) -> Result<Self, String> {
        match value {
            Value::String(s) => Ok(Bytes(s.as_bytes().to_vec())),
            other => Err(expected("a string", &other)),
        }
    }
}

impl IntoLuaRet for Bytes {
    fn into_lua_ret(self, lua: &Lua, _: &'static str) -> mlua::Result<Value> {
        lua.create_string(&self.0).map(Value::String)
    }
}
