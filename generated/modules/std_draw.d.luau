-- ===== std:draw =====

--- A figure to measure.
export type std_draw_Figure =
	--- A circle of radius `r`.
	{ kind: "circle", r: number }
	--- A `w` by `h` rectangle.
	| { kind: "rect", w: number, h: number }

export type std_draw = {
	--- The area of a figure.
	---
	--- ```luau
	--- assert(draw.area({ kind = "rect", w = 2, h = 3 }) == 6)
	--- ```
	area: (figure: std_draw_Figure) -> number,
}
