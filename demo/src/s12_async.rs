//! 12: Async functions and methods.

#[mlua_ir::module("std:task")]
pub mod task {
    use std::cell::Cell;
    use std::rc::Rc;
    use std::time::Duration;

    use mlua_ir::Object;

    /// Wait `ms` milliseconds without blocking other scripts; returns how long it waited.
    ///
    /// ```luau
    /// assert(task.sleep(1) == 1)
    /// ```
    pub async fn sleep(ms: u32) -> u32 {
        tokio::time::sleep(Duration::from_millis(ms.into())).await;
        ms
    }

    /// A clock that ticks once per `wait`.
    #[derive(Clone, Default, Object)]
    pub struct Clock {
        ticks: Rc<Cell<u32>>,
    }

    #[mlua_ir::object]
    impl Clock {
        /// A clock at zero.
        pub fn new() -> Clock {
            Clock::default()
        }

        /// Wait `ms` milliseconds, then tick; the number of ticks so far.
        ///
        /// ```luau
        /// local clock = task.Clock.new()
        /// clock:wait(1)
        /// assert(clock:wait(1) == 2)
        /// ```
        pub async fn wait(&self, ms: u32) -> u32 {
            tokio::time::sleep(Duration::from_millis(ms.into())).await;
            self.ticks.set(self.ticks.get() + 1);
            self.ticks.get()
        }
    }
}
