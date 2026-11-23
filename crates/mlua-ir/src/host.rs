//! [`Host`]: how an embedding program takes part in every call a script makes into a module.
//!
//! Without a host a call is a plain mlua call. A host, set once with [`set_host`], is asked about each call
//! ([`Call`]): to open a tracing span around a synchronous one ([`Host::enter`]) and to run the body of an async
//! one under its own rules ([`Host::wrap`]: a scheduler's turn, a timeout, a span). It sees functions, statics and
//! userdata methods alike.
//!
//! A function or method can hand the host an opaque hint with `#[host("...")]`; mlua-ir never reads it.

use std::any::Any;
use std::future::Future;
use std::sync::OnceLock;

use crate::MaybeSend;

/// What a host is told of a call.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Call {
    /// The module (`std:time`); `None` for a global.
    pub module: Option<&'static str>,
    /// The function (`sleep`); for a static or a method, the type first (`Duration.from_secs`).
    pub name: &'static str,
    /// The call as errors name it: `std:time.sleep`, `std:time.Duration:plus`.
    pub op: &'static str,
    /// What `#[host("...")]` said, uninterpreted.
    pub hint: Option<&'static str>,
}

/// A value a call's body produced, type-erased on its way through the host.
#[cfg(feature = "send")]
pub type Erased = Box<dyn Any + Send>;
#[cfg(not(feature = "send"))]
pub type Erased = Box<dyn Any>;

/// An async call's body.
#[cfg(feature = "send")]
pub type Body = futures::future::BoxFuture<'static, mlua::Result<Erased>>;
#[cfg(not(feature = "send"))]
pub type Body = futures::future::LocalBoxFuture<'static, mlua::Result<Erased>>;

/// The embedding program's say in every call. Both methods default to doing nothing.
pub trait Host: Send + Sync + 'static {
    /// A synchronous call begins. What is returned is dropped when the call ends: hold a span's guard in it.
    fn enter(&self, _call: &Call) -> Option<Box<dyn Any>> {
        None
    }

    /// An async call's body (its arguments are already read, its result is not yet given to the script). Return
    /// the body, run however the host's rules say.
    fn wrap(&self, _call: &Call, body: Body) -> Body {
        body
    }
}

static HOST: OnceLock<Box<dyn Host>> = OnceLock::new();

/// Sets the host, once for the process. `false` when one was already set.
pub fn set_host(host: impl Host) -> bool {
    HOST.set(Box::new(host)).is_ok()
}

#[doc(hidden)]
pub fn call_sync<R>(call: &Call, f: impl FnOnce() -> R) -> R {
    let _guard = HOST.get().and_then(|host| host.enter(call));
    f()
}

#[doc(hidden)]
pub async fn call_async<R: MaybeSend + 'static>(
    call: &Call,
    body: impl Future<Output = R> + MaybeSend + 'static,
) -> mlua::Result<R> {
    let Some(host) = HOST.get() else { return Ok(body.await) };
    let body: Body = Box::pin(async move { Ok(Box::new(body.await) as Erased) });
    let erased = host.wrap(call, body).await?;
    Ok(*erased.downcast::<R>().expect("a host returns the body it was given"))
}
