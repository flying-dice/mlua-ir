//! 13: Streams and iteration.

#[mlua_ir::module("std:events")]
pub mod events {
    use std::time::Duration;

    use futures::StreamExt;
    use mlua_ir::{LuaError, Shape, Stream};
    use serde::{Deserialize, Serialize};

    /// One tick of a stream.
    #[derive(Debug, Serialize, Deserialize, Shape)]
    pub struct Tick {
        /// Which tick, from 1.
        pub n: u32,
        /// `true` for the last one.
        pub last: bool,
    }

    /// Why a stream stopped early.
    #[derive(Debug, Serialize, thiserror::Error, LuaError)]
    #[serde(tag = "kind", rename_all = "snake_case")]
    pub enum EventsError {
        /// The source stopped before the end.
        #[error("interrupted after {after} ticks")]
        Interrupted { after: u32 },
    }

    /// Ticks `1..=n`, one per millisecond.
    ///
    /// ```luau
    /// local seen = 0
    /// for tick in events.count_to(3):items() do
    ///     seen += tick.n
    /// end
    /// assert(seen == 6)
    /// ```
    pub fn count_to(n: u32) -> Stream<Tick, EventsError> {
        Stream::new(futures::stream::iter(1..=n).then(move |i| async move {
            tokio::time::sleep(Duration::from_millis(1)).await;
            Ok(Tick { n: i, last: i == n })
        }))
    }

    /// Ticks that stop with an error after `after` of them.
    pub fn failing(after: u32) -> Stream<Tick, EventsError> {
        Stream::new(futures::stream::iter(1..=after + 1).map(move |i| {
            if i > after { Err(EventsError::Interrupted { after }) } else { Ok(Tick { n: i, last: false }) }
        }))
    }
}
