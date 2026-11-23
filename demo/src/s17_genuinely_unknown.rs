//! 17: The one allowed escape: genuinely unknown data.

#[mlua_ir::module("lib:json")]
pub mod json {
    use mlua_ir::{LuaError, Unknown};
    use serde::Serialize;

    mlua_ir::unknown_reason!(
        /// Why `decode`'s result has no declared shape.
        JsonText = "whatever JSON text was parsed: its shape is the text's, not this API's"
    );
    mlua_ir::unknown_reason!(
        /// Why `encode` takes any plain data.
        AnyData = "any plain data a script built: tables, strings, numbers, booleans"
    );

    /// Why `decode` refused its text.
    #[derive(Debug, Serialize, thiserror::Error, LuaError)]
    #[serde(tag = "kind", rename_all = "snake_case")]
    pub enum JsonError {
        /// The text is not JSON.
        #[error("not JSON at line {line}, column {column}")]
        Syntax { line: u32, column: u32 },
    }

    /// Parse JSON text. The result is `unknown`: check what it is before using it.
    ///
    /// ```luau
    /// local value, err = json.decode("{\"n\": 1}")
    /// assert(err == nil)
    /// if type(value) == "table" then
    ///     local object = value :: { [string]: unknown } -- checked above: it is a table
    ///     assert(object.n == 1)
    /// end
    /// ```
    pub fn decode(text: String) -> Result<Unknown<JsonText>, JsonError> {
        serde_json::from_str(&text).map(Unknown::new).map_err(|e| JsonError::Syntax {
            line: u32::try_from(e.line()).unwrap_or(u32::MAX),
            column: u32::try_from(e.column()).unwrap_or(u32::MAX),
        })
    }

    mlua_ir::unknown_reason!(
        /// Why `identity` passes its value through untyped.
        Anything = "whatever the script passed: it comes back as it is, the same table or function"
    );

    /// `value` itself, not a copy: a table comes back as the same table (`mlua_ir::Live`).
    ///
    /// ```luau
    /// local t = {}
    /// assert(json.identity(t) == t)
    /// ```
    pub fn identity(value: mlua_ir::Live<Anything>) -> mlua_ir::Live<Anything> {
        value
    }

    /// Write plain data as JSON text.
    pub fn encode(value: Unknown<AnyData>) -> String {
        value.value.to_string()
    }
}
