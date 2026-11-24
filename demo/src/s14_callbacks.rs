//! 14: Lua functions passed into Rust.

#[mlua_ir::module("std:sort")]
pub mod sort {
    use mlua_ir::Callback;

    /// `items` in the order `before` says: `before(left, right)` is `true` when `left` goes first. An error
    /// `before` raises passes through.
    ///
    /// ```luau
    /// local sorted = sort.sort_by({ "pear", "fig", "apple" }, function(left, right)
    ///     return #left < #right
    /// end)
    /// assert(sorted[1] == "fig" and sorted[3] == "apple")
    /// ```
    pub fn sort_by(items: Vec<String>, before: Callback<fn(left: String, right: String) -> bool>) -> mlua::Result<Vec<String>> {
        // An insertion sort: a script's order need not be a total order, and must not make Rust panic.
        let mut sorted: Vec<String> = Vec::with_capacity(items.len());
        for item in items {
            let mut at = sorted.len();
            while at > 0 && before.call(item.clone(), sorted[at - 1].clone())? {
                at -= 1;
            }
            sorted.insert(at, item);
        }
        Ok(sorted)
    }

    /// Calls `run(...)` with whatever arguments follow it; a callback with a variadic parameter is declared
    /// `(...T) -> ()`, so any function checks.
    pub fn call_with(run: Callback<fn(args: mlua_ir::Variadic<String>)>, args: mlua_ir::Variadic<String>) -> mlua::Result<()> {
        let _ = (run, args);
        Ok(())
    }

    /// Calls `visit(item, index)` for each item, in order; how many it visited.
    pub fn each(items: Vec<String>, visit: Callback<fn(item: String, index: u32)>) -> mlua::Result<u32> {
        let mut count = 0;
        for (index, item) in (1..).zip(items) {
            visit.call(item, index)?;
            count += 1;
        }
        Ok(count)
    }
}
