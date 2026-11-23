//! A host takes part in every call: functions, statics and methods, sync and async. Its own test binary, because a
//! host is set once per process.

use std::any::Any;
use std::sync::Mutex;

use mlua_ir::{Body, Call, Host};

static SEEN: Mutex<Vec<String>> = Mutex::new(Vec::new());

struct Recording;

struct Leaving(String);
impl Drop for Leaving {
    fn drop(&mut self) {
        SEEN.lock().expect("seen").push(format!("leave {}", self.0));
    }
}

impl Host for Recording {
    fn enter(&self, call: &Call) -> Option<Box<dyn Any>> {
        SEEN.lock().expect("seen").push(format!("enter {} [{}] in {:?}", call.name, call.op, call.module));
        Some(Box::new(Leaving(call.name.to_owned())))
    }

    fn wrap(&self, call: &Call, body: Body) -> Body {
        let note = format!("async {} hint={:?}", call.name, call.hint);
        Box::pin(async move {
            SEEN.lock().expect("seen").push(note);
            body.await
        })
    }
}

#[tokio::test]
async fn a_host_sees_sync_and_async_calls_and_their_hints() {
    assert!(mlua_ir::set_host(Recording));
    assert!(!mlua_ir::set_host(Recording), "a host is set once");
    let lua = mlua_ir_demo::lua().expect("a state");
    mlua_ir_demo::run(
        &lua,
        "host",
        r#"
        local blob = require("std:blob")
        assert(blob.to_hex("a") == "61")
        assert(blob.global_type_later("print") == "function")
        assert(blob.Label.new("x"):text_later() == "x")
        "#,
    )
    .await
    .expect("the script runs under a host");
    let seen = SEEN.lock().expect("seen").clone();
    for expected in [
        "enter to_hex [std:blob.to_hex] in Some(\"std:blob\")",
        "leave to_hex",
        "async global_type_later hint=Some(\"slow\")",
        "enter Label.new [std:blob.Label.new] in Some(\"std:blob\")",
        "async Label.text_later hint=Some(\"slow\")",
    ] {
        assert!(seen.iter().any(|s| s == expected), "{expected:?} is in {seen:#?}");
    }
}

#[test]
fn modules_lists_every_linked_module() {
    let names: Vec<&str> = mlua_ir::modules().iter().map(|m| m.name).collect();
    assert!(names.contains(&"std:blob") && names.windows(2).all(|w| w[0] <= w[1]), "{names:?}");
}
