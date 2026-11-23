-- ===== std:sort =====

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
	--- Calls `visit(item, index)` for each item, in order; how many it visited.
	each: (items: { string }, visit: (item: string, index: number) -> ()) -> number,
}
