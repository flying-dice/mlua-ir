// ===== std:sort =====

/** The kinds of `std_sort_SortError`. */
type std_sort_SortErrorKind = "failed"

/** Why `fail` failed. */
type std_sort_SortError =
  /** Always. */
  | { kind: "failed"; reason: string; message: string; op: string }

declare module "std:sort" {
  /**
   * `items` in the order `before` says: `before(left, right)` is `true` when `left` goes first. An error
   * `before` raises passes through.
   *
   * ```luau
   * local sorted = sort.sort_by({ "pear", "fig", "apple" }, function(left, right)
   *     return #left < #right
   * end)
   * assert(sorted[1] == "fig" and sorted[3] == "apple")
   * ```
   */
  export function sort_by(this: void, items: string[], before: (this: void, left: string, right: string) => boolean): string[]
  /**
   * Calls `run(...)` with whatever arguments follow it; a callback with a variadic parameter is declared
   * `(...T) -> ()`, so any function checks.
   */
  export function call_with(this: void, run: (this: void, ...args: string[]) => void, ...args: string[]): void
  /**
   * Fails with `reason`, always: a call that never returns normally is declared `never`. A doc that mentions a
   * glob such as `src/**∕*.rs` stays a valid TypeScript comment, and a parameter named `new` is renamed there.
   *
   * @throws {std_sort_SortError} when it fails; `pcall` catches it.
   */
  export function fail(this: void, new_: string): never
  /** Calls `visit(item, index)` for each item, in order; how many it visited. */
  export function each(this: void, items: string[], visit: (this: void, item: string, index: number) => void): number
  export type SortError = std_sort_SortError
}
