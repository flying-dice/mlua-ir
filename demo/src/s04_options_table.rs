//! 04: A trailing options table.

#[mlua_ir::module("std:dir")]
pub mod dir {
    use mlua_ir::Options;
    use serde::{Deserialize, Serialize};

    /// How `list` walks a directory.
    #[derive(Debug, Default, Serialize, Deserialize, Options)]
    pub struct ListOptions {
        /// Go into subdirectories too.
        pub recursive: bool,
        /// Include names that start with `.`.
        pub hidden: bool,
        /// How deep `recursive` goes; unlimited when not given.
        pub max_depth: Option<u32>,
    }

    const TREE: [&str; 5] = ["src/main.rs", "src/.env", "src/lib/mod.rs", "src/lib/deep/x.rs", "README.md"];

    /// The paths under `path` in the demo's fixed tree, sorted.
    ///
    /// ```luau
    /// assert(#dir.list("src") == 1)
    /// assert(#dir.list("src", { recursive = true }) == 3)
    /// assert(#dir.list("src", { recursive = true, hidden = true, max_depth = 2 }) == 3)
    /// ```
    pub fn list(path: String, opts: ListOptions) -> Vec<String> {
        let prefix = format!("{}/", path.trim_end_matches('/'));
        let mut found: Vec<String> = TREE
            .iter()
            .filter_map(|p| p.strip_prefix(&prefix).map(|rest| (*p, rest)))
            .filter(|(_, rest)| {
                let depth = u32::try_from(rest.split('/').count()).unwrap_or(u32::MAX);
                let deep_enough = if opts.recursive { opts.max_depth.is_none_or(|max| depth <= max) } else { depth == 1 };
                deep_enough && (opts.hidden || !rest.split('/').any(|s| s.starts_with('.')))
            })
            .map(|(p, _)| p.to_owned())
            .collect();
        found.sort();
        found
    }
}
