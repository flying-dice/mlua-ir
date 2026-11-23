//! 11: Userdata as an argument; one-of arguments.

#[mlua_ir::module("std:time")]
pub mod time {
    use mlua_ir::{Object, OneOf};

    /// A length of time.
    #[derive(Clone, Copy, Object)]
    pub struct Duration {
        secs: f64,
    }

    #[mlua_ir::object]
    impl Duration {
        /// `n` seconds.
        pub fn seconds(n: f64) -> Duration {
            Duration { secs: n }
        }

        /// The length in seconds.
        #[get]
        pub fn as_seconds(&self) -> f64 {
            self.secs
        }

        /// This and `other` together.
        ///
        /// ```luau
        /// local d = time.Duration.seconds(1):plus(time.Duration.seconds(0.5))
        /// assert(d.as_seconds == 1.5)
        /// ```
        pub fn plus(&self, other: Duration) -> Duration {
            Duration { secs: self.secs + other.secs }
        }
    }

    /// A timeout: a `Duration`, or a number of seconds.
    #[derive(OneOf)]
    pub enum Timeout {
        Duration(Duration),
        Seconds(f64),
    }

    /// How a timeout reads to a person.
    ///
    /// ```luau
    /// assert(time.describe(1.5) == "1.5s")
    /// assert(time.describe(time.Duration.seconds(2)) == "2s")
    /// ```
    pub fn describe(timeout: Timeout) -> String {
        let secs = match timeout {
            Timeout::Duration(d) => d.secs,
            Timeout::Seconds(s) => s,
        };
        format!("{secs}s")
    }
}
