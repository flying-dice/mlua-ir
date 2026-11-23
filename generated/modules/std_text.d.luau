-- ===== std:text =====

export type std_text = {
	--- Pad `text` on the right with spaces to `width` characters.
	---
	--- ```luau
	--- assert(text.pad("id", 6) == "id    ")
	--- ```
	pad: (text: string, width: number) -> string,
	--- `text` repeated `times` times. A Rust keyword as a name is a raw identifier: `r#loop` is `loop` to a
	--- script.
	loop: (text: string, times: number) -> string,
}
