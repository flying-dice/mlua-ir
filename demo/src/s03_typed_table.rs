//! 03: A typed table in and out.

#[mlua_ir::module("std:geo")]
pub mod geo {
    use mlua_ir::Shape;
    use serde::{Deserialize, Serialize};

    /// A point on the plane.
    #[derive(Debug, Clone, Serialize, Deserialize, Shape)]
    pub struct Point {
        /// Distance along the horizontal axis.
        pub x: f64,
        /// Distance along the vertical axis.
        pub y: f64,
        /// A name to show beside it.
        pub label: Option<String>,
    }

    /// The point halfway between `a` and `b`. It carries no label.
    ///
    /// ```luau
    /// local m = geo.midpoint({ x = 0, y = 0 }, { x = 4, y = 2 })
    /// assert(m.x == 2 and m.y == 1 and m.label == nil)
    /// ```
    pub fn midpoint(a: Point, b: Point) -> Point {
        Point { x: (a.x + b.x) / 2.0, y: (a.y + b.y) / 2.0, label: None }
    }

    /// The distance between two points.
    pub fn distance(a: Point, b: Point) -> f64 {
        (a.x - b.x).hypot(a.y - b.y)
    }
}
