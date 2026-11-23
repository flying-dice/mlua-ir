//! Errors a script can tell apart.
//!
//! An error is a table: `{ kind = "not_found", message = "...", op = "std:fs.read", <the variant's fields> }`
//! whose `tostring` is `std:fs.read: not_found: <message>`. Its kinds are a declared union, so a script branches on
//! `err.kind` with the checker's help instead of parsing a string.
//!
//! Returned (`Result<T, E>`) is the default. Raised (`Result<T, Raise<E>>`) is for a module whose convention is
//! to raise; [`Caught`] and [`Raised`] read a raised error back, typed, from what `pcall` caught.

use std::any::type_name;
use std::fmt;
use std::marker::PhantomData;

use mlua::{Lua, Value};
use serde::Serialize;

use crate::convert::{FromLuaArg, IntoLuaRet, to_serde};
use crate::ir::{Field, Ty};
use crate::registry::Registry;
use crate::shape::Shape;

/// What `#[derive(LuaError)]` implements: a serde enum tagged on `kind`, with a `Display` message.
pub trait LuaError: Shape + IntoLuaRet + Serialize + fmt::Display + 'static {
    fn kind(&self) -> &'static str;
}

/// The fields every error carries besides its variant's own.
pub fn common_fields() -> Vec<Field> {
    vec![
        Field {
            name: "message".into(),
            doc: Some("What went wrong, for a person. Branch on `kind`, not on these words.".into()),
            ty: Ty::String,
            default: None,
            readonly: false,
        },
        Field {
            name: "op".into(),
            doc: Some("The call that failed (`std:fs.read`).".into()),
            ty: Ty::String,
            default: None,
            readonly: false,
        },
    ]
}

/// The error as a script sees it.
pub fn error_value<E: LuaError>(error: &E, lua: &Lua, op: &'static str) -> mlua::Result<Value> {
    let data = serde_json::to_value(error).map_err(mlua::Error::external)?;
    build(lua, op, error.kind(), &error.to_string(), &data)
}

fn build(lua: &Lua, op: &str, kind: &str, message: &str, data: &serde_json::Value) -> mlua::Result<Value> {
    let table = match to_serde(data, lua)? {
        Value::Table(table) => table,
        _ => lua.create_table()?,
    };
    table.raw_set("message", message)?;
    table.raw_set("op", op)?;
    let meta = lua.create_table()?;
    let text = format!("{op}: {kind}: {message}");
    meta.raw_set("__tostring", lua.create_function(move |_, _: Value| Ok(text.clone()))?)?;
    table.set_metatable(Some(meta))?;
    Ok(Value::Table(table))
}

/// An error to raise rather than return: `Result<T, Raise<E>>`. `?` converts an `E` into it.
pub struct Raise<E>(pub E);

impl<E> From<E> for Raise<E> {
    fn from(error: E) -> Self {
        Raise(error)
    }
}

/// What travels inside mlua's error while a raised error unwinds to the script's `pcall`.
#[derive(Debug)]
struct RaisedError {
    rust_type: &'static str,
    op: &'static str,
    kind: &'static str,
    message: String,
    data: serde_json::Value,
}

impl fmt::Display for RaisedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}: {}", self.op, self.kind, self.message)
    }
}

impl std::error::Error for RaisedError {}

pub(crate) fn raised<E: LuaError>(error: &E, op: &'static str) -> mlua::Error {
    mlua::Error::external(RaisedError {
        rust_type: type_name::<E>(),
        op,
        kind: error.kind(),
        message: error.to_string(),
        data: serde_json::to_value(error).unwrap_or(serde_json::Value::Null),
    })
}

/// Whatever a script's `pcall` caught. Its shape is genuinely unknown: any value can be raised.
pub struct Caught(Value);

impl FromLuaArg for Caught {
    fn from_lua_arg(value: Value, _: &Lua) -> Result<Self, String> {
        Ok(Caught(value))
    }
}

impl Shape for Caught {
    fn shape(_: &mut Registry) -> Ty {
        Ty::Unknown { reason: "whatever `pcall` caught: any value can be raised".into() }
    }
}

impl Caught {
    /// The error of type `E` this is, if it is one.
    pub fn of<E: LuaError>(self) -> Option<Raised<E>> {
        let Value::Error(error) = self.0 else { return None };
        let raised = error.downcast_ref::<RaisedError>()?;
        (raised.rust_type == type_name::<E>()).then(|| Raised {
            op: raised.op,
            kind: raised.kind,
            message: raised.message.clone(),
            data: raised.data.clone(),
            _error: PhantomData,
        })
    }
}

/// A raised `E`, read back from what `pcall` caught: the same table a returned `E` is.
pub struct Raised<E> {
    op: &'static str,
    kind: &'static str,
    message: String,
    data: serde_json::Value,
    _error: PhantomData<E>,
}

impl<E: LuaError> Shape for Raised<E> {
    fn shape(reg: &mut Registry) -> Ty {
        E::shape(reg)
    }
}

impl<E: LuaError> IntoLuaRet for Raised<E> {
    fn into_lua_ret(self, lua: &Lua, _: &'static str) -> mlua::Result<Value> {
        build(lua, self.op, self.kind, &self.message, &self.data)
    }
}
