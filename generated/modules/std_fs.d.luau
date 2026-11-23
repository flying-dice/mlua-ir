-- ===== std:fs =====

--- The kinds of `std_fs_FsError`.
export type std_fs_FsErrorKind = "not_found" | "permission_denied" | "too_large"

--- Why a file operation failed.
export type std_fs_FsError =
	--- There is no file at `path`.
	{ kind: "not_found", path: string, message: string, op: string }
	--- The file exists but this script may not read it.
	| { kind: "permission_denied", path: string, message: string, op: string }
	--- The file is larger than `limit` bytes.
	| { kind: "too_large", path: string, limit: number, message: string, op: string }

export type std_fs = {
	--- The text of a file in the demo's fixed filesystem.
	---
	--- ```luau
	--- local text, err = fs.read("notes.txt")
	--- assert(err == nil and text == "hello")
	--- local missing, why = fs.read("nope.txt")
	--- assert(missing == nil and why ~= nil and why.kind == "not_found")
	--- ```
	read: (path: string) -> (string?, std_fs_FsError?),
	--- Remove a file: `true`, or the error.
	remove: (path: string) -> (boolean?, std_fs_FsError?),
}
