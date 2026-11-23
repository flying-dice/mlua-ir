-- ===== std:blob =====

--- The kinds of `std_blob_HexError`.
export type std_blob_HexErrorKind = "odd_length" | "not_hex"

--- Why hex text could not be read.
export type std_blob_HexError =
	--- The text has an odd number of digits.
	{ kind: "odd_length", len: number, message: string, op: string }
	--- A character is not a hex digit.
	| { kind: "not_hex", found: string, message: string, op: string }

--- A name that prints as itself.
export type std_blob_Label = {
	--- The same, after yielding once, for a host to see a method.
	---
	--- Yields the calling coroutine while it waits.
	text_later: (self: std_blob_Label) -> string,
}

--- The statics of `std_blob_Label`.
export type std_blob_LabelStatics = {
	--- A label.
	new: (text: string) -> std_blob_Label,
}

export type std_blob = {
	Label: std_blob_LabelStatics,
	--- The hex text of raw bytes (any byte, including `\0`).
	---
	--- ```luau
	--- assert(blob.to_hex("\0\255a") == "00ff61")
	--- ```
	to_hex: (data: string) -> string,
	--- The raw bytes of hex text.
	---
	--- ```luau
	--- local data = assert(blob.from_hex("00ff61"))
	--- assert(data == "\0\255a")
	--- local _, err = blob.from_hex("0")
	--- assert(err ~= nil and err.kind == "odd_length")
	--- ```
	from_hex: (text: string) -> (string?, std_blob_HexError?),
	--- A global of the script's own state: the Lua state is a parameter the script does not pass.
	---
	--- ```luau
	--- assert(blob.global_type("print") == "function")
	--- ```
	global_type: (name: string) -> string,
	--- The same, after yielding once.
	---
	--- ```luau
	--- assert(blob.global_type_later("print") == "function")
	--- ```
	---
	--- Yields the calling coroutine while it waits.
	global_type_later: (name: string) -> string,
}
