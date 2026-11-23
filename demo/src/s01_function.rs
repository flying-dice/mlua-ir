//! 01: A module function.

#[mlua_ir::module("std:text")]
pub mod text {
    /// Pad `text` on the right with spaces to `width` characters.
    ///
    /// ```luau
    /// assert(text.pad("id", 6) == "id    ")
    /// ```
    pub fn pad(text: String, width: u32) -> String {
        format!("{text:<width$}", width = width as usize)
    }
}
