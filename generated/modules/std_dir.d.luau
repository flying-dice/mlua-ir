-- ===== std:dir =====

--- How `list` walks a directory.
export type std_dir_ListOptions = {
	--- Go into subdirectories too. Default: `false`.
	recursive: boolean?,
	--- Include names that start with `.`. Default: `false`.
	hidden: boolean?,
	--- How deep `recursive` goes; unlimited when not given.
	max_depth: number?,
}

export type std_dir = {
	--- The paths under `path` in the demo's fixed tree, sorted.
	---
	--- ```luau
	--- assert(#dir.list("src") == 1)
	--- assert(#dir.list("src", { recursive = true }) == 3)
	--- assert(#dir.list("src", { recursive = true, hidden = true, max_depth = 2 }) == 3)
	--- ```
	list: (path: string, opts: std_dir_ListOptions?) -> { string },
}
