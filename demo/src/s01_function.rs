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

    /// `text` repeated `times` times. A Rust keyword as a name is a raw identifier: `r#loop` is `loop` to a
    /// script.
    pub fn r#loop(text: String, times: u32) -> String {
        text.repeat(times as usize)
    }
}
