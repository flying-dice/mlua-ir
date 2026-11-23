//! 08: Expected failure returned as `(value, err)`.

#[mlua_ir::module("std:fs")]
pub mod fs {
    use mlua_ir::LuaError;
    use serde::Serialize;

    /// Why a file operation failed.
    #[derive(Debug, Serialize, thiserror::Error, LuaError)]
    #[serde(tag = "kind", rename_all = "snake_case")]
    pub enum FsError {
        /// There is no file at `path`.
        #[error("no such file: {path}")]
        NotFound { path: String },
        /// The file exists but this script may not read it.
        #[error("permission denied: {path}")]
        PermissionDenied { path: String },
        /// The file is larger than `limit` bytes.
        #[error("{path} is larger than {limit} bytes")]
        TooLarge { path: String, limit: u32 },
    }

    /// The text of a file in the demo's fixed filesystem.
    ///
    /// ```luau
    /// local text, err = fs.read("notes.txt")
    /// assert(err == nil and text == "hello")
    /// local missing, why = fs.read("nope.txt")
    /// assert(missing == nil and why ~= nil and why.kind == "not_found")
    /// ```
    pub fn read(path: String) -> Result<String, FsError> {
        match path.as_str() {
            "notes.txt" => Ok("hello".to_owned()),
            "secret.txt" => Err(FsError::PermissionDenied { path }),
            "huge.bin" => Err(FsError::TooLarge { path, limit: 1024 }),
            _ => Err(FsError::NotFound { path }),
        }
    }

    /// Remove a file: `true`, or the error.
    pub fn remove(path: String) -> Result<(), FsError> {
        read(path).map(|_| ())
    }
}
