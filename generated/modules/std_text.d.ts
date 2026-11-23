// ===== std:text =====

declare module "std:text" {
  /**
   * Pad `text` on the right with spaces to `width` characters.
   *
   * ```luau
   * assert(text.pad("id", 6) == "id    ")
   * ```
   */
  export function pad(this: void, text: string, width: number): string
}
