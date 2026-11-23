//! 10: A userdata with methods, fields, statics.

#[mlua_ir::module("std:collections")]
pub mod collections {
    use std::cell::Cell;
    use std::rc::Rc;

    use mlua_ir::Object;

    /// A counter. Clones share one value.
    #[derive(Clone, Object)]
    pub struct Counter {
        value: Rc<Cell<i64>>,
    }

    #[mlua_ir::object]
    impl Counter {
        /// A counter starting at `initial` (default 0).
        ///
        /// ```luau
        /// local c = collections.Counter.new(41)
        /// assert(c:increment() == 42 and c.value == 42)
        /// ```
        pub fn new(initial: Option<i64>) -> Counter {
            Counter { value: Rc::new(Cell::new(initial.unwrap_or(0))) }
        }

        /// The current value.
        #[get]
        pub fn value(&self) -> i64 {
            self.value.get()
        }

        /// Add one; the new value.
        pub fn increment(&self) -> i64 {
            self.add(1)
        }

        /// Add `n`; the new value.
        pub fn add(&self, n: i64) -> i64 {
            self.value.set(self.value.get() + n);
            self.value.get()
        }

        /// Set the value to `n`.
        pub fn set(&self, n: i64) {
            self.value.set(n);
        }
    }
}
