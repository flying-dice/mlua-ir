//! The macros' output under mlua's `send` (as neoc uses it): `cargo test -p mlua-ir --features send`.
#![cfg(feature = "send")]

#[mlua_ir::module("test:shared")]
pub mod shared {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicI64, Ordering};

    use mlua_ir::{Callback, LuaError, Object, Stream};
    use serde::Serialize;

    /// Why a call failed.
    #[derive(Debug, Serialize, LuaError)]
    #[serde(tag = "kind", rename_all = "snake_case")]
    pub enum SharedError {
        /// It was asked to fail.
        Asked { n: i64 },
    }

    impl std::fmt::Display for SharedError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Asked { n } => write!(f, "asked to fail at {n}"),
            }
        }
    }

    /// A counter that can cross threads.
    #[derive(Clone, Default, Object)]
    pub struct Total {
        value: Arc<AtomicI64>,
    }

    #[mlua_ir::object]
    impl Total {
        /// A total at zero.
        pub fn new() -> Total {
            Total::default()
        }

        /// The current total.
        #[get]
        pub fn value(&self) -> i64 {
            self.value.load(Ordering::SeqCst)
        }

        /// Add `n` after yielding once; the new total.
        pub async fn add_later(&self, n: i64) -> i64 {
            tokio::task::yield_now().await;
            self.value.fetch_add(n, Ordering::SeqCst) + n
        }
    }

    /// `n` doubled, after yielding once; a negative `n` is the error.
    pub async fn twice(n: i64) -> Result<i64, SharedError> {
        tokio::task::yield_now().await;
        if n < 0 { Err(SharedError::Asked { n }) } else { Ok(n * 2) }
    }

    /// `1..=n` as a stream.
    pub fn upto(n: i64) -> Stream<i64, SharedError> {
        Stream::new(futures::stream::iter((1..=n).map(Ok)))
    }

    /// `f(n)`.
    pub fn apply(n: i64, f: Callback<fn(n: i64) -> i64>) -> mlua::Result<i64> {
        f.call(n)
    }
}

#[test]
fn the_macros_work_under_send() {
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().expect("a runtime");
    runtime.block_on(async {
        let lua = mlua::Lua::new();
        mlua_ir::install(&lua).expect("the modules install");
        lua.load(
            r#"
            local shared = require("test:shared")
            local t = shared.Total.new()
            assert(t:add_later(2) == 2 and t.value == 2)
            assert(shared.twice(4) == 8)
            local _, err = shared.twice(-1)
            assert(err ~= nil and err.kind == "asked" and err.n == -1)
            local sum = 0
            for n in shared.upto(3):items() do
                sum += n
            end
            assert(sum == 6)
            assert(shared.apply(5, function(n) return n + 1 end) == 6)
            "#,
        )
        .exec_async()
        .await
        .expect("the script runs");
    });
    let ir = mlua_ir::schema("test");
    let shared = ir.modules.iter().find(|m| m.name == "test:shared").expect("the module is in the schema");
    assert_eq!(shared.functions.len(), 3);
    assert_eq!(shared.objects.len(), 1);
}
