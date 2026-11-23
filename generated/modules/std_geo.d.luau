-- ===== std:geo =====

--- A point on the plane.
export type std_geo_Point = {
	--- Distance along the horizontal axis.
	x: number,
	--- Distance along the vertical axis.
	y: number,
	--- A name to show beside it.
	label: string?,
}

export type std_geo = {
	--- The point halfway between `a` and `b`. It carries no label.
	---
	--- ```luau
	--- local m = geo.midpoint({ x = 0, y = 0 }, { x = 4, y = 2 })
	--- assert(m.x == 2 and m.y == 1 and m.label == nil)
	--- ```
	midpoint: (a: std_geo_Point, b: std_geo_Point) -> std_geo_Point,
	--- The distance between two points.
	distance: (a: std_geo_Point, b: std_geo_Point) -> number,
}
