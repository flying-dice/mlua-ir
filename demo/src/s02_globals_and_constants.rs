//! 02: A primitive in globals, module constants.

#[mlua_ir::module(globals)]
pub mod globals {
    use std::sync::OnceLock;
    use std::time::Instant;

    static STARTED: OnceLock<Instant> = OnceLock::new();

    /// The runtime's version.
    pub const VOS_VERSION: &str = env!("CARGO_PKG_VERSION");

    /// The most arguments a single call takes.
    pub const MAX_ARGS: u32 = 64;

    /// Seconds since the runtime started.
    ///
    /// ```luau
    /// assert(uptime() >= 0)
    /// ```
    pub fn uptime() -> f64 {
        STARTED.get_or_init(Instant::now).elapsed().as_secs_f64()
    }
}

#[mlua_ir::module("std:math2")]
pub mod math2 {
    /// One full turn in radians.
    pub const TAU: f64 = std::f64::consts::TAU;
}
