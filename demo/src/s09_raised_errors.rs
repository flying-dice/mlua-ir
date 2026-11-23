//! 09: Raised errors, `pcall`, bugs.

#[mlua_ir::module("std:parse")]
pub mod parse {
    use mlua_ir::{Caught, LuaError, Raise, Raised};
    use serde::Serialize;

    /// Why `number` refused its text.
    #[derive(Debug, Serialize, thiserror::Error, LuaError)]
    #[serde(tag = "kind", rename_all = "snake_case")]
    pub enum ParseError {
        /// The text is empty.
        #[error("the text is empty")]
        Empty,
        /// The text is not a number.
        #[error("not a number: {text}")]
        NotANumber { text: String },
    }

    /// The number `text` spells. This module's convention is to raise: a failure is a `ParseError`, raised.
    pub fn number(text: String) -> Result<f64, Raise<ParseError>> {
        if text.trim().is_empty() {
            return Err(ParseError::Empty.into());
        }
        text.trim().parse().map_err(|_| ParseError::NotANumber { text }.into())
    }

    /// The `ParseError` a call of this module raised, read back from what `pcall` caught; `nil` for anything else.
    ///
    /// ```luau
    /// local ok, caught = pcall(parse.number, "x")
    /// assert(not ok)
    /// local err = parse.error_of(caught)
    /// assert(err ~= nil and err.kind == "not_a_number")
    /// ```
    pub fn error_of(caught: Caught) -> Option<Raised<ParseError>> {
        caught.of()
    }
}
