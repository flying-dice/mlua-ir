-- ===== std:env =====

export type std_env = {
	--- The value of a variable in the demo's fixed environment, or `nil` when it is not set. Not being set is
	--- normal absence, not an error.
	---
	--- ```luau
	--- assert(env.var("HOME") == "/home/demo")
	--- assert(env.var("NOPE") == nil)
	--- ```
	var: (name: string) -> string?,
	--- `var(name)`, or `fallback` (default `""`) when it is not set.
	var_or: (name: string, fallback: string?) -> string,
}
