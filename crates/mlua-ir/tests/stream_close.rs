//! A stream closed while a script waits in `next()` or an `items()` iterator: the wait answers the end at once,
//! and nothing comes after. A read cancelled while it waits leaves the stream usable.

use std::task::{Context, Poll, Waker};

use futures::channel::mpsc;
use mlua::{AnyUserData, Function, Lua, MultiValue, ObjectLike, Value};
use mlua_ir::{IntoLuaRet, LuaError, Stream};
use serde::Serialize;

#[derive(Debug, Serialize, LuaError)]
#[allow(dead_code)] // the stream under test never fails
#[serde(tag = "kind", rename_all = "snake_case")]
enum Never {
    Failed,
}

impl std::fmt::Display for Never {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("failed")
    }
}

fn stream(lua: &Lua) -> (mpsc::UnboundedSender<Result<i64, Never>>, AnyUserData) {
    let (tx, rx) = mpsc::unbounded();
    let value = Stream::new(rx).into_lua_ret(lua, "test.stream").expect("a userdata");
    let Value::UserData(ud) = value else { panic!("a stream is a userdata") };
    (tx, ud)
}

fn poll<F: std::future::Future + Unpin>(f: &mut F) -> Poll<F::Output> {
    std::pin::Pin::new(f).poll(&mut Context::from_waker(Waker::noop()))
}

fn ended(values: &MultiValue) -> bool {
    values.iter().all(Value::is_nil)
}

#[test]
fn closing_while_next_is_pending_answers_the_end_and_nothing_follows() {
    let lua = Lua::new();
    let (tx, ud) = stream(&lua);
    let next: Function = ud.get("next").unwrap();
    let mut pending = Box::pin(next.call_async::<MultiValue>(ud.clone()));
    assert!(poll(&mut pending).is_pending());
    ud.call_method::<()>("close", ()).unwrap();
    tx.unbounded_send(Ok(1)).ok();
    match poll(&mut pending) {
        Poll::Ready(Ok(values)) => assert!(ended(&values), "the pending next answers (nil, nil)"),
        other => panic!("the pending next must end at once, got {other:?}"),
    }
    let mut after = Box::pin(next.call_async::<MultiValue>(ud.clone()));
    match poll(&mut after) {
        Poll::Ready(Ok(values)) => assert!(ended(&values), "no item after close"),
        other => panic!("{other:?}"),
    }
}

#[test]
fn closing_while_an_items_iterator_is_pending_ends_the_loop() {
    let lua = Lua::new();
    let (tx, ud) = stream(&lua);
    let iterator: Function = ud.call_method("items", ()).unwrap();
    let mut pending = Box::pin(iterator.call_async::<Value>(()));
    assert!(poll(&mut pending).is_pending());
    ud.call_method::<()>("close", ()).unwrap();
    tx.unbounded_send(Ok(1)).ok();
    assert!(matches!(poll(&mut pending), Poll::Ready(Ok(Value::Nil))), "the iterator ends the loop");
    let mut after = Box::pin(iterator.call_async::<Value>(()));
    assert!(matches!(poll(&mut after), Poll::Ready(Ok(Value::Nil))));
}

#[test]
fn a_cancelled_next_leaves_the_stream_readable() {
    let lua = Lua::new();
    let (tx, ud) = stream(&lua);
    let next: Function = ud.get("next").unwrap();
    let mut pending = Box::pin(next.call_async::<MultiValue>(ud.clone()));
    assert!(poll(&mut pending).is_pending());
    drop(pending);
    tx.unbounded_send(Ok(5)).unwrap();
    let mut again = Box::pin(next.call_async::<MultiValue>(ud.clone()));
    match poll(&mut again) {
        Poll::Ready(Ok(values)) => assert_eq!(values.iter().next().and_then(Value::as_i64), Some(5)),
        other => panic!("the stream must still be readable, got {other:?}"),
    }
}
