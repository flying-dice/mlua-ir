//! 19: The Lua state as a parameter, raw bytes, `tostring`, and a hint for the host.

#[mlua_ir::module("std:blob")]
pub mod blob {
    use mlua::Lua;
    use mlua_ir::{Bytes, LuaError, Object};
    use serde::Serialize;

    /// Why hex text could not be read.
    #[derive(Debug, Serialize, thiserror::Error, LuaError)]
    #[serde(tag = "kind", rename_all = "snake_case")]
    pub enum HexError {
        /// The text has an odd number of digits.
        #[error("odd number of hex digits: {len}")]
        OddLength { len: u32 },
        /// A character is not a hex digit.
        #[error("not a hex digit: {found}")]
        NotHex { found: String },
    }

    /// The hex text of raw bytes (any byte, including `\0`).
    ///
    /// ```luau
    /// assert(blob.to_hex("\0\255a") == "00ff61")
    /// ```
    pub fn to_hex(data: Bytes) -> String {
        data.iter().map(|b| format!("{b:02x}")).collect()
    }

    /// The raw bytes of hex text.
    ///
    /// ```luau
    /// local data = assert(blob.from_hex("00ff61"))
    /// assert(data == "\0\255a")
    /// local _, err = blob.from_hex("0")
    /// assert(err ~= nil and err.kind == "odd_length")
    /// ```
    pub fn from_hex(text: String) -> Result<Bytes, HexError> {
        if !text.len().is_multiple_of(2) {
            return Err(HexError::OddLength { len: text.len() as u32 });
        }
        let digit = |c: char| c.to_digit(16).ok_or_else(|| HexError::NotHex { found: c.to_string() });
        let digits: Vec<u32> = text.chars().map(digit).collect::<Result<_, _>>()?;
        Ok(Bytes(digits.chunks(2).map(|pair| (pair[0] * 16 + pair[1]) as u8).collect()))
    }

    /// A global of the script's own state: the Lua state is a parameter the script does not pass.
    ///
    /// ```luau
    /// assert(blob.global_type("print") == "function")
    /// ```
    pub fn global_type(lua: &Lua, name: String) -> String {
        match lua.globals().get::<mlua::Value>(name.as_str()) {
            Ok(value) => value.type_name().to_owned(),
            Err(_) => "nil".to_owned(),
        }
    }

    /// The same, after yielding once.
    ///
    /// ```luau
    /// assert(blob.global_type_later("print") == "function")
    /// ```
    #[host("slow")]
    pub async fn global_type_later(lua: Lua, name: String) -> String {
        tokio::task::yield_now().await;
        global_type(&lua, name)
    }

    /// A name that prints as itself.
    #[derive(Clone, Object)]
    pub struct Label {
        text: String,
    }

    #[mlua_ir::object]
    impl Label {
        /// A label.
        pub fn new(text: String) -> Label {
            Label { text }
        }

        /// What `tostring(label)` gives.
        ///
        /// ```luau
        /// assert(tostring(blob.Label.new("hi")) == "label:hi")
        /// ```
        #[to_string]
        pub fn display(&self) -> String {
            format!("label:{}", self.text)
        }

        /// The same, after yielding once, for a host to see a method.
        #[host("slow")]
        pub async fn text_later(&self, lua: Lua) -> String {
            let _ = lua;
            tokio::task::yield_now().await;
            self.text.clone()
        }
    }
}
