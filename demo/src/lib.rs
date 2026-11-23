//! Every scenario of the DX as a working module. Each file is one shape of Lua API, smallest first; `docs/`
//! shows each next to the vanilla mlua way and the declarations generated from it.

pub mod s01_function;
pub mod s02_globals_and_constants;
pub mod s03_typed_table;
pub mod s04_options_table;
pub mod s05_string_enums_and_unions;
pub mod s06_lists_maps_tuples;
pub mod s07_absence;
pub mod s08_returned_errors;
pub mod s09_raised_errors;
pub mod s10_userdata_object;
pub mod s11_userdata_as_argument;
pub mod s12_async;
pub mod s13_streams;
pub mod s14_callbacks;
pub mod s15_validated_text;
pub mod s16_foreign_types;
pub mod s17_genuinely_unknown;
pub mod s18_whole_module_bollard;

/// The version the schema is published under.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// A Lua state with every module installed.
pub fn lua() -> mlua::Result<mlua::Lua> {
    let lua = mlua::Lua::new();
    mlua_ir::install(&lua)?;
    Ok(lua)
}

/// Runs a script to the end (async calls included).
pub async fn run(lua: &mlua::Lua, name: &str, code: &str) -> mlua::Result<()> {
    lua.load(code).set_name(format!("@{name}")).exec_async().await
}

/// Forces the linker to keep every scenario's inventory entries when only `lua()` / `schema()` are used.
pub fn schema() -> mlua_ir::ir::Ir {
    mlua_ir::schema(VERSION)
}
