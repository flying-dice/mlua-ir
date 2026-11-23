//! The modules a binary links, collected: installed into a Lua state, or read as one [`Ir`].

use std::collections::HashMap;

use mlua::{Lua, Table, Value};

use crate::convert::FromLuaArg;
use crate::ir::{Ir, Module, ModuleKind, Param};
use crate::registry::Registry;
use crate::shape::Shape;

/// What `#[module]` submits for each module.
pub struct ModuleEntry {
    pub name: &'static str,
    pub kind: ModuleKind,
    pub describe: fn(&mut Registry) -> Module,
    pub install: fn(&Lua) -> mlua::Result<Table>,
}

inventory::collect!(ModuleEntry);

/// Every module this binary links, by name: for a host that puts them into its own `require`. Call a module's
/// `install` for its table; a [`ModuleKind::Globals`] module's table holds the globals to set.
pub fn modules() -> Vec<&'static ModuleEntry> {
    let mut entries: Vec<&ModuleEntry> = inventory::iter::<ModuleEntry>.into_iter().collect();
    entries.sort_by_key(|e| e.name);
    entries
}

/// The whole surface as IR: every module this binary links, and every type they reach.
pub fn schema(version: &str) -> Ir {
    let mut reg = Registry::new();
    let modules = modules().into_iter().map(|e| (e.describe)(&mut reg)).collect();
    let mut types = reg.into_types();
    types.sort_by(|a, b| a.name.cmp(&b.name));
    Ir { version: version.to_owned(), modules, types }
}

/// Installs every module: globals into the globals, the rest behind `require("<name>")`.
pub fn install(lua: &Lua) -> mlua::Result<()> {
    let globals = lua.globals();
    let mut required: HashMap<&'static str, Table> = HashMap::new();
    for entry in modules() {
        let table = (entry.install)(lua)?;
        match entry.kind {
            ModuleKind::Globals => {
                for pair in table.pairs::<Value, Value>() {
                    let (key, value) = pair?;
                    globals.raw_set(key, value)?;
                }
            }
            ModuleKind::Require => {
                required.insert(entry.name, table);
            }
        }
    }
    let require = lua.create_function(move |_, name: String| {
        required
            .get(name.as_str())
            .cloned()
            .ok_or_else(|| mlua::Error::runtime(format!("no module named \"{name}\"")))
    })?;
    globals.raw_set("require", require)?;
    Ok(())
}

/// Reads argument `index` (`name`) of `op`; a value the declarations would refuse is raised, naming it.
pub fn arg<T: FromLuaArg>(lua: &Lua, value: Option<Value>, op: &str, index: usize, name: &str) -> mlua::Result<T> {
    T::from_lua_arg(value.unwrap_or(Value::Nil), lua)
        .map_err(|reason| mlua::Error::runtime(format!("{op}: argument #{index} '{name}': {reason}")))
}

/// Refuses arguments past the last parameter (Lua would drop them silently).
pub fn no_more_args(mut rest: impl Iterator<Item = Value>, op: &str, taken: usize) -> mlua::Result<()> {
    match rest.next() {
        None => Ok(()),
        Some(_) => Err(mlua::Error::runtime(format!(
            "{op}: takes {taken} argument{}, got more",
            if taken == 1 { "" } else { "s" }
        ))),
    }
}

/// A parameter's IR; `callback_names` are the names a `Callback<fn(name: T) -> R>` parameter's signature gives.
pub fn param<T: Shape>(reg: &mut Registry, name: &str, callback_names: &[&str]) -> Param {
    let mut ty = T::shape(reg);
    crate::callback::name_params(&mut ty, callback_names);
    Param { name: name.to_owned(), ty, optional: T::OPTIONAL_PARAM }
}

/// Collapses a `///` doc into the IR's text: `None` when there is none.
pub fn doc(lines: &[&str]) -> Option<String> {
    let text = lines.iter().map(|l| l.strip_prefix(' ').unwrap_or(l)).collect::<Vec<_>>().join("\n");
    let text = text.trim();
    (!text.is_empty()).then(|| text.to_owned())
}
