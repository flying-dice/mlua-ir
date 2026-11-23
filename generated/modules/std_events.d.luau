-- ===== std:events =====

--- The kinds of `std_events_EventsError`.
export type std_events_EventsErrorKind = "interrupted"

--- Why a stream stopped early.
export type std_events_EventsError =
	--- The source stopped before the end.
	{ kind: "interrupted", after: number, message: string, op: string }

--- One tick of a stream.
export type std_events_Tick = {
	--- Which tick, from 1.
	n: number,
	--- `true` for the last one.
	last: boolean,
}

export type std_events = {
	--- Ticks `1..=n`, one per millisecond.
	---
	--- ```luau
	--- local seen = 0
	--- for tick in events.count_to(3):items() do
	---     seen += tick.n
	--- end
	--- assert(seen == 6)
	--- ```
	count_to: (n: number) -> mlua_Stream<std_events_Tick, std_events_EventsError>,
	--- Ticks that stop with an error after `after` of them.
	failing: (after: number) -> mlua_Stream<std_events_Tick, std_events_EventsError>,
}
