-- ===== std:time =====

--- A length of time.
declare extern type std_time_Duration with
	--- The length in seconds. Read-only.
	as_seconds: number
	--- This and `other` together.
	---
	--- ```luau
	--- local d = time.Duration.seconds(1):plus(time.Duration.seconds(0.5))
	--- assert(d.as_seconds == 1.5)
	--- ```
	plus: (self: std_time_Duration, other: std_time_Duration) -> std_time_Duration
end

--- The statics of `std_time_Duration`.
export type std_time_DurationStatics = {
	--- `n` seconds.
	seconds: (n: number) -> std_time_Duration,
}

--- A timeout: a `Duration`, or a number of seconds.
export type std_time_Timeout = std_time_Duration | number

export type std_time = {
	Duration: std_time_DurationStatics,
	--- How a timeout reads to a person.
	---
	--- ```luau
	--- assert(time.describe(1.5) == "1.5s")
	--- assert(time.describe(time.Duration.seconds(2)) == "2s")
	--- ```
	describe: (timeout: std_time_Timeout) -> string,
}
