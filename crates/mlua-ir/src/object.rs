//! Userdata: what `#[derive(Object)]` and `#[object] impl` produce.

use mlua::{Lua, Table};

use crate::ir::{Field, Function};
use crate::registry::Registry;

/// What `#[object] impl T { ... }` says of `T`.
pub trait ObjectDef: Sized + 'static {
    /// Its getters, its `:` methods and its `.` statics.
    fn describe(reg: &mut Registry) -> (Vec<Field>, Vec<Function>, Vec<Function>);

    /// The table of its statics a module exports under the type's name (`collections.Counter.new`).
    fn statics(lua: &Lua) -> mlua::Result<Table>;
}
