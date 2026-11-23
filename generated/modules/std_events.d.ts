// ===== std:events =====

/** The kinds of `std_events_EventsError`. */
type std_events_EventsErrorKind = "interrupted"

/** Why a stream stopped early. */
type std_events_EventsError =
  /** The source stopped before the end. */
  | { kind: "interrupted"; after: number; message: string; op: string }

/** One tick of a stream. */
interface std_events_Tick {
  /** Which tick, from 1. */
  n: number
  /** `true` for the last one. */
  last: boolean
}

declare module "std:events" {
  /**
   * Ticks `1..=n`, one per millisecond.
   *
   * ```luau
   * local seen = 0
   * for tick in events.count_to(3):items() do
   *     seen += tick.n
   * end
   * assert(seen == 6)
   * ```
   */
  export function count_to(this: void, n: number): mlua_Stream<std_events_Tick, std_events_EventsError>
  /** Ticks that stop with an error after `after` of them. */
  export function failing(this: void, after: number): mlua_Stream<std_events_Tick, std_events_EventsError>
  export type EventsError = std_events_EventsError
  export type Tick = std_events_Tick
}
