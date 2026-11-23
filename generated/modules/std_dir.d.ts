// ===== std:dir =====

/** How `list` walks a directory. */
interface std_dir_ListOptions {
  /** Go into subdirectories too. @default false */
  recursive?: boolean
  /** Include names that start with `.`. @default false */
  hidden?: boolean
  /** How deep `recursive` goes; unlimited when not given. */
  max_depth?: number
}

declare module "std:dir" {
  /**
   * The paths under `path` in the demo's fixed tree, sorted.
   *
   * ```luau
   * assert(#dir.list("src") == 1)
   * assert(#dir.list("src", { recursive = true }) == 3)
   * assert(#dir.list("src", { recursive = true, hidden = true, max_depth = 2 }) == 3)
   * ```
   */
  export function list(this: void, path: string, opts?: std_dir_ListOptions): string[]
  export type ListOptions = std_dir_ListOptions
}
