// ===== std:fs =====

/** The kinds of `std_fs_FsError`. */
type std_fs_FsErrorKind = "not_found" | "permission_denied" | "too_large"

/** Why a file operation failed. */
type std_fs_FsError =
  /** There is no file at `path`. */
  | { kind: "not_found"; path: string; message: string; op: string }
  /** The file exists but this script may not read it. */
  | { kind: "permission_denied"; path: string; message: string; op: string }
  /** The file is larger than `limit` bytes. */
  | { kind: "too_large"; path: string; limit: number; message: string; op: string }

declare module "std:fs" {
  /**
   * The text of a file in the demo's fixed filesystem.
   *
   * ```luau
   * local text, err = fs.read("notes.txt")
   * assert(err == nil and text == "hello")
   * local missing, why = fs.read("nope.txt")
   * assert(missing == nil and why ~= nil and why.kind == "not_found")
   * ```
   */
  export function read(this: void, path: string): LuaMultiReturn<[string, undefined] | [undefined, std_fs_FsError]>
  /** Remove a file: `true`, or the error. */
  export function remove(this: void, path: string): LuaMultiReturn<[boolean, undefined] | [undefined, std_fs_FsError]>
  export type FsError = std_fs_FsError
}
