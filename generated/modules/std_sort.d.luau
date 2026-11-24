-- ===== std:sort =====

--- The kinds of `std_sort_SortError`.
export type std_sort_SortErrorKind = "failed"

--- Why `fail` failed.
export type std_sort_SortError =
	--- Always.
	{ kind: "failed", reason: string, message: string, op: string }

export type std_sort = {
	--- `items` in the order `before` says: `before(left, right)` is `true` when `left` goes first. An error
	--- `before` raises passes through.
	---
	--- ```luau
	--- local sorted = sort.sort_by({ "pear", "fig", "apple" }, function(left, right)
	---     return #left < #right
	--- end)
	--- assert(sorted[1] == "fig" and sorted[3] == "apple")
	--- ```
	sort_by: (items: { string }, before: (left: string, right: string) -> boolean) -> { string },
	--- Calls `run(...)` with whatever arguments follow it; a callback with a variadic parameter is declared
	--- `(...T) -> ()`, so any function checks.
	call_with: (run: (...string) -> (), ...string) -> (),
	--- Fails with `reason`, always: a call that never returns normally is declared `never`. A doc that mentions a
	--- glob such as `src/**/*.rs` stays a valid TypeScript comment, and a parameter named `new` is renamed there.
	---
	--- Raises `std_sort_SortError` when it fails; `pcall` catches it.
	fail: (new: string) -> never,
	--- Calls `visit(item, index)` for each item, in order; how many it visited.
	each: (items: { string }, visit: (item: string, index: number) -> ()) -> number,
}
