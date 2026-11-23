// ===== std:time =====

/** A length of time. */
interface std_time_Duration {
  /** The length in seconds. */
  readonly as_seconds: number
  /**
   * This and `other` together.
   *
   * ```luau
   * local d = time.Duration.seconds(1):plus(time.Duration.seconds(0.5))
   * assert(d.as_seconds == 1.5)
   * ```
   */
  plus(other: std_time_Duration): std_time_Duration
}

/** The statics of `std_time_Duration`. */
interface std_time_DurationStatics {
  /** `n` seconds. */
  seconds(this: void, n: number): std_time_Duration
}

/** A timeout: a `Duration`, or a number of seconds. */
type std_time_Timeout = std_time_Duration | number

declare module "std:time" {
  export const Duration: std_time_DurationStatics
  /**
   * How a timeout reads to a person.
   *
   * ```luau
   * assert(time.describe(1.5) == "1.5s")
   * assert(time.describe(time.Duration.seconds(2)) == "2s")
   * ```
   */
  export function describe(this: void, timeout: std_time_Timeout): string
  export type Duration = std_time_Duration
  export type Timeout = std_time_Timeout
}
