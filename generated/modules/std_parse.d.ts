// ===== std:parse =====

/** The kinds of `std_parse_ParseError`. */
type std_parse_ParseErrorKind = "empty" | "not_a_number"

/** Why `number` refused its text. */
type std_parse_ParseError =
  /** The text is empty. */
  | { kind: "empty"; message: string; op: string }
  /** The text is not a number. */
  | { kind: "not_a_number"; text: string; message: string; op: string }

declare module "std:parse" {
  /**
   * The number `text` spells. This module's convention is to raise: a failure is a `ParseError`, raised.
   *
   * @throws {std_parse_ParseError} when it fails; `pcall` catches it.
   */
  export function number(this: void, text: string): number
  /**
   * The `ParseError` a call of this module raised, read back from what `pcall` caught; `nil` for anything else.
   *
   * ```luau
   * local ok, caught = pcall(parse.number, "x")
   * assert(not ok)
   * local err = parse.error_of(caught)
   * assert(err ~= nil and err.kind == "not_a_number")
   * ```
   */
  export function error_of(this: void, caught: unknown): std_parse_ParseError | undefined
  export type ParseError = std_parse_ParseError
}
