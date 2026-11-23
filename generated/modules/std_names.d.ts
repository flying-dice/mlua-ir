// ===== std:names =====

/** The name or id of a container: letters, digits and `_.-`, starting with a letter or digit. */
type std_names_ContainerName = string

declare module "std:names" {
  /**
   * How a container is shown.
   *
   * ```luau
   * assert(names.describe("web-1") == "container web-1")
   * ```
   */
  export function describe(this: void, name: std_names_ContainerName): string
  export type ContainerName = std_names_ContainerName
}
