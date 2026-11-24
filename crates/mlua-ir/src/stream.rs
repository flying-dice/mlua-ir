//! [`Stream`]: a Rust stream a script reads item by item.
//!
//! ```luau
//! local ticks = events.count_to(3)
//! while true do
//!     local tick, err = ticks:next()       -- (item, nil), (nil, error), or (nil, nil) at the end
//!     if err ~= nil then error(err) end
//!     if tick == nil then break end
//! end
//! for tick in events.count_to(3):items() do end   -- the same, raising the error
//! ```
//!
//! With the `send` feature (mlua's `send`), the stream and its items must be `Send`.

use std::sync::{Arc, Mutex};

use futures::stream::StreamExt;
use mlua::{Lua, MultiValue, UserData, UserDataMethods, UserDataRef, Value};

use crate::convert::IntoLuaRet;
use crate::error::{LuaError, raised};
use crate::ir::Ty;
use crate::registry::Registry;
use crate::shape::Shape;
use crate::MaybeSend;

#[cfg(feature = "send")]
type Boxed<T, E> = futures::stream::BoxStream<'static, Result<T, E>>;
#[cfg(not(feature = "send"))]
type Boxed<T, E> = futures::stream::LocalBoxStream<'static, Result<T, E>>;

enum State<T, E> {
    Ready(Boxed<T, E>),
    /// A `next` is waiting on it.
    Busy,
    Done,
}

pub struct Stream<T, E> {
    state: Arc<Mutex<State<T, E>>>,
    op: &'static str,
}

impl<T: MaybeSend + 'static, E: MaybeSend + 'static> Stream<T, E> {
    pub fn new(stream: impl futures::Stream<Item = Result<T, E>> + MaybeSend + 'static) -> Self {
        #[cfg(feature = "send")]
        let boxed = stream.boxed();
        #[cfg(not(feature = "send"))]
        let boxed = stream.boxed_local();
        Self { state: Arc::new(Mutex::new(State::Ready(boxed))), op: "" }
    }
}

impl<T: Shape, E: LuaError> Shape for Stream<T, E> {
    fn shape(reg: &mut Registry) -> Ty {
        Ty::Stream { item: Box::new(T::shape(reg)), error: Box::new(E::shape(reg)) }
    }
}

impl<T: IntoLuaRet + MaybeSend + 'static, E: LuaError + MaybeSend> IntoLuaRet for Stream<T, E> {
    fn into_lua_ret(mut self, lua: &Lua, op: &'static str) -> mlua::Result<Value> {
        self.op = op;
        lua.create_userdata(self).map(Value::UserData)
    }
}

fn replace<T, E>(state: &Mutex<State<T, E>>, next: State<T, E>) -> State<T, E> {
    // A poisoned lock means a panic while swapping a state in: the stream is over.
    let mut guard = state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    std::mem::replace(&mut *guard, next)
}

/// The next item: `Ok(Some)`, `Ok(None)` at the end, or the stream's error. The lock is never held across the wait.
async fn advance<T, E>(state: &Mutex<State<T, E>>) -> Result<Option<Result<T, E>>, mlua::Error> {
    let mut stream = match replace(state, State::Busy) {
        State::Ready(stream) => stream,
        State::Done => {
            replace(state, State::Done);
            return Ok(None);
        }
        State::Busy => return Err(mlua::Error::runtime("the stream is already being read")),
    };
    let item = stream.next().await;
    replace(state, if item.is_some() { State::Ready(stream) } else { State::Done });
    Ok(item)
}

/// Waits for the next item through the host, like any async call: a host that runs async bodies under its own
/// rules (yielding the VM's turn while it waits) does so for a stream read too, so a task parked in `next()` does
/// not hold up the rest of the script.
async fn wait_for_next<T: MaybeSend + 'static, E: MaybeSend + 'static>(
    op: &'static str,
    name: &'static str,
    state: Arc<Mutex<State<T, E>>>,
) -> mlua::Result<Option<Result<T, E>>> {
    let call = crate::host::Call { module: None, name, op, hint: None };
    crate::host::call_async(&call, async move { advance(&state).await }).await?
}

impl<T: IntoLuaRet + MaybeSend + 'static, E: LuaError + MaybeSend> UserData for Stream<T, E> {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_async_method("next", |lua, this: UserDataRef<Self>, ()| async move {
            let op = this.op;
            let state = this.state.clone();
            drop(this);
            // Hold the state weakly across the wait: a pending `next` must not keep a dropped engine alive.
            let weak = lua.weak();
            drop(lua);
            let item = wait_for_next(op, "Stream:next", state).await?;
            let lua = weak.upgrade();
            Ok(match item {
                Some(Ok(item)) => MultiValue::from_vec(vec![item.into_lua_ret(&lua, op)?, Value::Nil]),
                Some(Err(error)) => MultiValue::from_vec(vec![Value::Nil, error.into_lua_ret(&lua, op)?]),
                None => MultiValue::from_vec(vec![Value::Nil, Value::Nil]),
            })
        });
        methods.add_method("items", |lua, this, ()| {
            let state = this.state.clone();
            let op = this.op;
            lua.create_async_function(move |lua, ()| {
                let state = state.clone();
                async move {
                    let weak = lua.weak();
                    drop(lua);
                    let item = wait_for_next(op, "Stream:items", state).await?;
                    let lua = weak.upgrade();
                    match item {
                        Some(Ok(item)) => item.into_lua_ret(&lua, op),
                        Some(Err(error)) => Err(raised(&error, op)),
                        None => Ok(Value::Nil),
                    }
                }
            })
        });
        methods.add_method("close", |_, this, ()| {
            replace(&this.state, State::Done);
            Ok(())
        });
    }
}
