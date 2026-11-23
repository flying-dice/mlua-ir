// ===== lib:json =====

/** The kinds of `lib_json_JsonError`. */
type lib_json_JsonErrorKind = "syntax"

/** Why `decode` refused its text. */
type lib_json_JsonError =
  /** The text is not JSON. */
  | { kind: "syntax"; line: number; column: number; message: string; op: string }

declare module "lib:json" {
  /**
   * Parse JSON text. The result is `unknown`: check what it is before using it.
   *
   * ```luau
   * local value, err = json.decode("{\"n\": 1}")
   * assert(err == nil)
   * if type(value) == "table" then
   *     local object = value :: { [string]: unknown } -- checked above: it is a table
   *     assert(object.n == 1)
   * end
   * ```
   */
  export function decode(this: void, text: string): LuaMultiReturn<[unknown, undefined] | [undefined, lib_json_JsonError]>
  /**
   * `value` itself, not a copy: a table comes back as the same table (`mlua_ir::Live`).
   *
   * ```luau
   * local t = {}
   * assert(json.identity(t) == t)
   * ```
   */
  export function identity(this: void, value: unknown): unknown
  /** Write plain data as JSON text. */
  export function encode(this: void, value: unknown): string
  export type JsonError = lib_json_JsonError
}
