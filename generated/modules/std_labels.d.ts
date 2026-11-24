// ===== std:labels =====

/** The kinds of `std_labels_LabelError`. */
type std_labels_LabelErrorKind = "no_equals"

/** Why `parse` refused its lines. */
type std_labels_LabelError =
  /** A line without `=`. */
  | { kind: "no_equals"; line: number; text: string; message: string; op: string }

declare module "std:labels" {
  /**
   * `"key=value"` lines as a map.
   *
   * ```luau
   * local map = assert(labels.parse({ "app=web", "tier=front" }))
   * assert(map.app == "web")
   * ```
   */
  export function parse(this: void, lines: string[]): LuaMultiReturn<[{ [key: string]: string }, undefined] | [undefined, std_labels_LabelError]>
  /**
   * How many values were passed, `nil`s included: `labels.count("a", nil)` is 2. A variadic last parameter
   * (`mlua_ir::Variadic<T>`) keeps the trailing `nil` a list would drop.
   *
   * ```luau
   * assert(labels.count("a", nil) == 2)
   * ```
   */
  export function count(this: void, ...values: (string | undefined)[]): number
  /** The keys of a map, sorted. */
  export function keys(this: void, labels: { [key: string]: string }): string[]
  /** The first key (if any) and how many there are. */
  export function summary(this: void, labels: { [key: string]: string }): LuaMultiReturn<[string | undefined, number]>
  export type LabelError = std_labels_LabelError
}
