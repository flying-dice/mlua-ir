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

use std::future::poll_fn;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::Poll;

use futures::stream::StreamExt;
use futures::task::AtomicWaker;
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

/// What the userdata, its `items` iterators and any pending read share.
///
/// The stream stays inside the lock while a read waits on it, so a cancelled read (its future dropped) only
/// releases the lock: the stream is still there for the next read. `close` is terminal: it sets `closed`, wakes
/// the pending read (which then drops the stream and answers the end), and drops the stream itself when no read
/// holds it, so the producer stops (a Docker event watcher's request is dropped).
struct Shared<T, E> {
    /// `None` once the stream ended or was closed.
    stream: futures::lock::Mutex<Option<Boxed<T, E>>>,
    closed: AtomicBool,
    /// The read waiting on the stream, woken by `close`.
    reader: AtomicWaker,
}

impl<T, E> Shared<T, E> {
    fn close(&self) {
        self.closed.store(true, Ordering::SeqCst);
        self.reader.wake();
        if let Some(mut stream) = self.stream.try_lock() {
            *stream = None;
        }
    }
}

pub struct Stream<T, E> {
    shared: Arc<Shared<T, E>>,
    op: &'static str,
}

impl<T: MaybeSend + 'static, E: MaybeSend + 'static> Stream<T, E> {
    pub fn new(stream: impl futures::Stream<Item = Result<T, E>> + MaybeSend + 'static) -> Self {
        #[cfg(feature = "send")]
        let boxed = stream.boxed();
        #[cfg(not(feature = "send"))]
        let boxed = stream.boxed_local();
        let shared = Shared {
            stream: futures::lock::Mutex::new(Some(boxed)),
            closed: AtomicBool::new(false),
            reader: AtomicWaker::new(),
        };
        Self { shared: Arc::new(shared), op: "" }
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

/// The next item: `Ok(Some)`, `Ok(None)` at the end or once closed, or the stream's error.
///
/// One read at a time: a second read while one waits is refused ("the stream is already being read"). A read
/// cancelled while it waits leaves the stream as it was. Once `close` is called, a pending read answers the end
/// promptly and no further item is ever given, even one the producer completes at the same moment.
async fn advance<T, E>(shared: &Shared<T, E>) -> Result<Option<Result<T, E>>, mlua::Error> {
    if shared.closed.load(Ordering::SeqCst) {
        return Ok(None);
    }
    let Some(mut guard) = shared.stream.try_lock() else {
        return Err(mlua::Error::runtime("the stream is already being read"));
    };
    let polled = {
        let Some(stream) = guard.as_mut() else { return Ok(None) };
        poll_fn(|cx| {
            // Registered before `closed` is read, so a close between the two still wakes this read.
            shared.reader.register(cx.waker());
            if shared.closed.load(Ordering::SeqCst) {
                return Poll::Ready(None);
            }
            stream.poll_next_unpin(cx).map(Some)
        })
        .await
    };
    match polled {
        Some(Some(item)) if !shared.closed.load(Ordering::SeqCst) => Ok(Some(item)),
        // Closed (before or while the item arrived), or the stream ended: drop it, answer the end.
        _ => {
            *guard = None;
            Ok(None)
        }
    }
}

/// Waits for the next item through the host, like any async call: a host that runs async bodies under its own
/// rules (yielding the VM's turn while it waits) does so for a stream read too, so a task parked in `next()` does
/// not hold up the rest of the script.
async fn wait_for_next<T: MaybeSend + 'static, E: MaybeSend + 'static>(
    op: &'static str,
    name: &'static str,
    shared: Arc<Shared<T, E>>,
) -> mlua::Result<Option<Result<T, E>>> {
    let call = crate::host::Call { module: None, name, op, hint: None };
    crate::host::call_async(&call, async move { advance(&shared).await }).await?
}

impl<T: IntoLuaRet + MaybeSend + 'static, E: LuaError + MaybeSend> UserData for Stream<T, E> {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_async_method("next", |lua, this: UserDataRef<Self>, ()| async move {
            let op = this.op;
            let shared = this.shared.clone();
            drop(this);
            // Hold the state weakly across the wait: a pending `next` must not keep a dropped engine alive.
            let weak = lua.weak();
            drop(lua);
            let item = wait_for_next(op, "Stream:next", shared).await?;
            let lua = weak.upgrade();
            Ok(match item {
                Some(Ok(item)) => MultiValue::from_vec(vec![item.into_lua_ret(&lua, op)?, Value::Nil]),
                Some(Err(error)) => MultiValue::from_vec(vec![Value::Nil, error.into_lua_ret(&lua, op)?]),
                None => MultiValue::from_vec(vec![Value::Nil, Value::Nil]),
            })
        });
        methods.add_method("items", |lua, this, ()| {
            let shared = this.shared.clone();
            let op = this.op;
            lua.create_async_function(move |lua, ()| {
                let shared = shared.clone();
                async move {
                    let weak = lua.weak();
                    drop(lua);
                    let item = wait_for_next(op, "Stream:items", shared).await?;
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
            this.shared.close();
            Ok(())
        });
    }
}

#[cfg(test)]
mod tests {
    use std::pin::Pin;
    use std::sync::atomic::AtomicUsize;
    use std::task::{Context, Wake, Waker};

    use futures::channel::mpsc;

    use super::*;

    /// Counts how often it was woken.
    struct Counter(AtomicUsize);

    impl Wake for Counter {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    type Read<'a> = Pin<Box<dyn std::future::Future<Output = mlua::Result<Option<Result<i32, String>>>> + 'a>>;

    fn shared() -> (mpsc::UnboundedSender<Result<i32, String>>, Shared<i32, String>) {
        let (tx, rx) = mpsc::unbounded();
        let shared = Shared {
            stream: futures::lock::Mutex::new(Some(rx.boxed_local_or_send())),
            closed: AtomicBool::new(false),
            reader: AtomicWaker::new(),
        };
        (tx, shared)
    }

    trait BoxAny<T, E> {
        fn boxed_local_or_send(self) -> Boxed<T, E>;
    }

    impl<S: futures::Stream<Item = Result<i32, String>> + Send + 'static> BoxAny<i32, String> for S {
        fn boxed_local_or_send(self) -> Boxed<i32, String> {
            #[cfg(feature = "send")]
            return self.boxed();
            #[cfg(not(feature = "send"))]
            return self.boxed_local();
        }
    }

    fn poll(read: &mut Read<'_>, waker: &Waker) -> Poll<mlua::Result<Option<Result<i32, String>>>> {
        read.as_mut().poll(&mut Context::from_waker(waker))
    }

    fn read(shared: &Shared<i32, String>) -> Read<'_> {
        Box::pin(advance(shared))
    }

    #[test]
    fn close_wakes_a_pending_read_which_answers_the_end() {
        let (_tx, shared) = shared();
        let counter = Arc::new(Counter(AtomicUsize::new(0)));
        let waker = Waker::from(counter.clone());
        let mut pending = read(&shared);
        assert!(poll(&mut pending, &waker).is_pending());
        shared.close();
        assert_eq!(counter.0.load(Ordering::SeqCst), 1, "close wakes the pending read");
        assert!(matches!(poll(&mut pending, &waker), Poll::Ready(Ok(None))));
    }

    #[test]
    fn no_item_is_given_after_close_even_one_completed_at_the_same_moment() {
        let (tx, shared) = shared();
        let waker = Waker::noop();
        let mut pending = read(&shared);
        assert!(poll(&mut pending, waker).is_pending());
        tx.unbounded_send(Ok(1)).unwrap();
        shared.close();
        assert!(matches!(poll(&mut pending, waker), Poll::Ready(Ok(None))), "the item that raced the close is dropped");
        drop(pending);
        tx.unbounded_send(Ok(2)).ok();
        assert!(matches!(poll(&mut read(&shared), waker), Poll::Ready(Ok(None))), "a closed stream stays ended");
        assert!(shared.stream.try_lock().is_some_and(|s| s.is_none()), "the stream (its producer) is dropped");
    }

    #[test]
    fn close_without_a_pending_read_drops_the_producer_at_once() {
        let (tx, shared) = shared();
        shared.close();
        assert!(tx.is_closed(), "the receiving end was dropped, so the producer sees it");
        assert!(matches!(poll(&mut read(&shared), Waker::noop()), Poll::Ready(Ok(None))));
    }

    #[test]
    fn a_cancelled_read_leaves_the_stream_usable() {
        let (tx, shared) = shared();
        let waker = Waker::noop();
        let mut pending = read(&shared);
        assert!(poll(&mut pending, waker).is_pending());
        drop(pending);
        tx.unbounded_send(Ok(7)).unwrap();
        assert!(matches!(poll(&mut read(&shared), waker), Poll::Ready(Ok(Some(Ok(7))))));
    }

    #[test]
    fn a_second_read_while_one_waits_is_refused() {
        let (_tx, shared) = shared();
        let waker = Waker::noop();
        let mut first = read(&shared);
        assert!(poll(&mut first, waker).is_pending());
        match poll(&mut read(&shared), waker) {
            Poll::Ready(Err(e)) => assert!(e.to_string().contains("already being read"), "{e}"),
            _ => panic!("a concurrent read must be refused"),
        }
    }

    #[test]
    fn the_stream_ends_once_and_stays_ended() {
        let (tx, shared) = shared();
        tx.unbounded_send(Ok(1)).unwrap();
        drop(tx);
        let waker = Waker::noop();
        assert!(matches!(poll(&mut read(&shared), waker), Poll::Ready(Ok(Some(Ok(1))))));
        assert!(matches!(poll(&mut read(&shared), waker), Poll::Ready(Ok(None))));
        assert!(matches!(poll(&mut read(&shared), waker), Poll::Ready(Ok(None))));
    }
}
