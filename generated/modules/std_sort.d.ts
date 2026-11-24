// ===== std:sort =====

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
  /** Calls `visit(item, index)` for each item, in order; how many it visited. */
  export function each(this: void, items: string[], visit: (this: void, item: string, index: number) => void): number
}
