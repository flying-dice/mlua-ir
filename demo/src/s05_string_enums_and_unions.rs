//! 05: String literal unions, tagged unions.

#[mlua_ir::module("std:log")]
pub mod log {
    use mlua_ir::Shape;
    use serde::{Deserialize, Serialize};

    /// How serious a line is.
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, Shape)]
    #[serde(rename_all = "snake_case")]
    pub enum Level {
        /// Detail for whoever debugs it.
        Debug,
        /// Normal operation.
        Info,
        /// Something to look at.
        Warn,
        /// Something failed.
        Error,
    }

    /// Write one line to the host's log; returns the line as written.
    ///
    /// ```luau
    /// assert(log.write("warn", "disk nearly full") == "[warn] disk nearly full")
    /// ```
    pub fn write(level: Level, message: String) -> String {
        let level = serde_json::to_value(level).ok().and_then(|v| v.as_str().map(str::to_owned)).unwrap_or_default();
        format!("[{level}] {message}")
    }
}

#[mlua_ir::module("std:draw")]
pub mod draw {
    use mlua_ir::Shape;
    use serde::{Deserialize, Serialize};

    /// A figure to measure.
    #[derive(Debug, Serialize, Deserialize, Shape)]
    #[serde(tag = "kind", rename_all = "snake_case")]
    pub enum Figure {
        /// A circle of radius `r`.
        Circle { r: f64 },
        /// A `w` by `h` rectangle.
        Rect { w: f64, h: f64 },
    }

    /// The area of a figure.
    ///
    /// ```luau
    /// assert(draw.area({ kind = "rect", w = 2, h = 3 }) == 6)
    /// ```
    pub fn area(figure: Figure) -> f64 {
        match figure {
            Figure::Circle { r } => std::f64::consts::PI * r * r,
            Figure::Rect { w, h } => w * h,
        }
    }
}
