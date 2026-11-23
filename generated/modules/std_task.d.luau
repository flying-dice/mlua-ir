-- ===== std:task =====

--- A clock that ticks once per `wait`.
export type std_task_Clock = {
	--- Wait `ms` milliseconds, then tick; the number of ticks so far.
	---
	--- ```luau
	--- local clock = task.Clock.new()
	--- clock:wait(1)
	--- assert(clock:wait(1) == 2)
	--- ```
	---
	--- Yields the calling coroutine while it waits.
	wait: (self: std_task_Clock, ms: number) -> number,
}

--- The statics of `std_task_Clock`.
export type std_task_ClockStatics = {
	--- A clock at zero.
	new: () -> std_task_Clock,
}

export type std_task = {
	Clock: std_task_ClockStatics,
	--- Wait `ms` milliseconds without blocking other scripts; returns how long it waited.
	---
	--- ```luau
	--- assert(task.sleep(1) == 1)
	--- ```
	---
	--- Yields the calling coroutine while it waits.
	sleep: (ms: number) -> number,
}
