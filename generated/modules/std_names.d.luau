-- ===== std:names =====

--- The name or id of a container: letters, digits and `_.-`, starting with a letter or digit.
export type std_names_ContainerName = string

export type std_names = {
	--- How a container is shown.
	---
	--- ```luau
	--- assert(names.describe("web-1") == "container web-1")
	--- ```
	describe: (name: std_names_ContainerName) -> string,
}
