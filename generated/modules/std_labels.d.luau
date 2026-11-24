-- ===== std:labels =====

--- The kinds of `std_labels_LabelError`.
export type std_labels_LabelErrorKind = "no_equals"

--- Why `parse` refused its lines.
export type std_labels_LabelError =
	--- A line without `=`.
	{ kind: "no_equals", line: number, text: string, message: string, op: string }

export type std_labels = {
	--- `"key=value"` lines as a map.
	---
	--- ```luau
	--- local map = assert(labels.parse({ "app=web", "tier=front" }))
	--- assert(map.app == "web")
	--- ```
	parse: (lines: { string }) -> ({ [string]: string }?, std_labels_LabelError?),
	--- How many values were passed, `nil`s included: `labels.count("a", nil)` is 2. A variadic last parameter
	--- (`mlua_ir::Variadic<T>`) keeps the trailing `nil` a list would drop.
	---
	--- ```luau
	--- assert(labels.count("a", nil) == 2)
	--- ```
	count: (...string?) -> number,
	--- The keys of a map, sorted.
	keys: (labels: { [string]: string }) -> { string },
	--- The first key (if any) and how many there are.
	summary: (labels: { [string]: string }) -> (string?, number),
}
