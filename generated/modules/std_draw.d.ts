// ===== std:draw =====

/** A figure to measure. */
type std_draw_Figure =
  /** A circle of radius `r`. */
  | { kind: "circle"; r: number }
  /** A `w` by `h` rectangle. */
  | { kind: "rect"; w: number; h: number }

declare module "std:draw" {
  /**
   * The area of a figure.
   *
   * ```luau
   * assert(draw.area({ kind = "rect", w = 2, h = 3 }) == 6)
   * ```
   */
  export function area(this: void, figure: std_draw_Figure): number
  export type Figure = std_draw_Figure
}
