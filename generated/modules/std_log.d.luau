-- ===== std:log =====

--- How serious a line is.
--- - `debug`: Detail for whoever debugs it.
--- - `info`: Normal operation.
--- - `warn`: Something to look at.
--- - `error`: Something failed.
export type std_log_Level = "debug" | "info" | "warn" | "error"

export type std_log = {
	--- Write one line to the host's log; returns the line as written.
	---
	--- ```luau
	--- assert(log.write("warn", "disk nearly full") == "[warn] disk nearly full")
	--- ```
	write: (level: std_log_Level, message: string) -> string,
}
