-- ===== std:parse =====

--- The kinds of `std_parse_ParseError`.
export type std_parse_ParseErrorKind = "empty" | "not_a_number"

--- Why `number` refused its text.
export type std_parse_ParseError =
	--- The text is empty.
	{ kind: "empty", message: string, op: string }
	--- The text is not a number.
	| { kind: "not_a_number", text: string, message: string, op: string }

export type std_parse = {
	--- The number `text` spells. This module's convention is to raise: a failure is a `ParseError`, raised.
	---
	--- Raises `std_parse_ParseError` when it fails; `pcall` catches it.
	number: (text: string) -> number,
	--- The `ParseError` a call of this module raised, read back from what `pcall` caught; `nil` for anything else.
	---
	--- ```luau
	--- local ok, caught = pcall(parse.number, "x")
	--- assert(not ok)
	--- local err = parse.error_of(caught)
	--- assert(err ~= nil and err.kind == "not_a_number")
	--- ```
	error_of: (caught: unknown) -> std_parse_ParseError?,
}
