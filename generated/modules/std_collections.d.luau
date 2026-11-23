-- ===== std:collections =====

--- A counter. Clones share one value.
export type std_collections_Counter = {
	--- The current value. Read-only.
	value: number,
	--- Add one; the new value.
	increment: (self: std_collections_Counter) -> number,
	--- Add `n`; the new value.
	add: (self: std_collections_Counter, n: number) -> number,
	--- Set the value to `n`.
	set: (self: std_collections_Counter, n: number) -> (),
}

--- The statics of `std_collections_Counter`.
export type std_collections_CounterStatics = {
	--- A counter starting at `initial` (default 0).
	---
	--- ```luau
	--- local c = collections.Counter.new(41)
	--- assert(c:increment() == 42 and c.value == 42)
	--- ```
	new: (initial: number?) -> std_collections_Counter,
}

export type std_collections = {
	Counter: std_collections_CounterStatics,
}
