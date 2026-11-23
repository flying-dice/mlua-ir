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
  /**
   * `text` repeated `times` times. A Rust keyword as a name is a raw identifier: `r#loop` is `loop` to a
   * script.
   */
  export function loop(this: void, text: string, times: number): string
}
