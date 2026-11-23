-- ===== lib:json =====

--- The kinds of `lib_json_JsonError`.
export type lib_json_JsonErrorKind = "syntax"

--- Why `decode` refused its text.
export type lib_json_JsonError =
	--- The text is not JSON.
	{ kind: "syntax", line: number, column: number, message: string, op: string }

export type lib_json = {
	--- Parse JSON text. The result is `unknown`: check what it is before using it.
	---
	--- ```luau
	--- local value, err = json.decode("{\"n\": 1}")
	--- assert(err == nil)
	--- if type(value) == "table" then
	---     local object = value :: { [string]: unknown } -- checked above: it is a table
	---     assert(object.n == 1)
	--- end
	--- ```
	decode: (text: string) -> (unknown, lib_json_JsonError?),
	--- Write plain data as JSON text.
	encode: (value: unknown) -> string,
}
