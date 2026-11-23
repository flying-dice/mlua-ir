//! 07: Normal absence (`nil`).

#[mlua_ir::module("std:env")]
pub mod env {
    const ENV: [(&str, &str); 2] = [("HOME", "/home/demo"), ("LANG", "en_GB.UTF-8")];

    /// The value of a variable in the demo's fixed environment, or `nil` when it is not set. Not being set is
    /// normal absence, not an error.
    ///
    /// ```luau
    /// assert(env.var("HOME") == "/home/demo")
    /// assert(env.var("NOPE") == nil)
    /// ```
    pub fn var(name: String) -> Option<String> {
        ENV.iter().find(|(k, _)| *k == name).map(|(_, v)| (*v).to_owned())
    }

    /// `var(name)`, or `fallback` (default `""`) when it is not set.
    pub fn var_or(name: String, fallback: Option<String>) -> String {
        var(name).or(fallback).unwrap_or_default()
    }
}
