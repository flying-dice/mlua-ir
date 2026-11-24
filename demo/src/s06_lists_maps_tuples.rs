//! 06: Lists, maps, multiple returns.

#[mlua_ir::module("std:labels")]
pub mod labels {
    use std::collections::BTreeMap;

    use mlua_ir::LuaError;
    use serde::Serialize;

    /// Why `parse` refused its lines.
    #[derive(Debug, Serialize, thiserror::Error, LuaError)]
    #[serde(tag = "kind", rename_all = "snake_case")]
    pub enum LabelError {
        /// A line without `=`.
        #[error("line {line} has no `=`: {text}")]
        NoEquals { line: u32, text: String },
    }

    /// `"key=value"` lines as a map.
    ///
    /// ```luau
    /// local map = assert(labels.parse({ "app=web", "tier=front" }))
    /// assert(map.app == "web")
    /// ```
    pub fn parse(lines: Vec<String>) -> Result<BTreeMap<String, String>, LabelError> {
        let mut map = BTreeMap::new();
        for (line, text) in (1..).zip(lines) {
            let Some((key, value)) = text.split_once('=') else {
                return Err(LabelError::NoEquals { line, text });
            };
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(map)
    }

    /// How many values were passed, `nil`s included: `labels.count("a", nil)` is 2. A variadic last parameter
    /// (`mlua_ir::Variadic<T>`) keeps the trailing `nil` a list would drop.
    ///
    /// ```luau
    /// assert(labels.count("a", nil) == 2)
    /// ```
    pub fn count(values: mlua_ir::Variadic<Option<String>>) -> usize {
        values.0.len()
    }

    /// The keys of a map, sorted.
    pub fn keys(labels: BTreeMap<String, String>) -> Vec<String> {
        labels.into_keys().collect()
    }

    /// The first key (if any) and how many there are.
    pub fn summary(labels: BTreeMap<String, String>) -> (Option<String>, usize) {
        let count = labels.len();
        (labels.into_keys().next(), count)
    }
}
