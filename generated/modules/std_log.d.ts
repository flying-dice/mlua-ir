// ===== std:log =====

/**
 * How serious a line is.
 * - `debug`: Detail for whoever debugs it.
 * - `info`: Normal operation.
 * - `warn`: Something to look at.
 * - `error`: Something failed.
 */
type std_log_Level = "debug" | "info" | "warn" | "error"

declare module "std:log" {
  /**
   * Write one line to the host's log; returns the line as written.
   *
   * ```luau
   * assert(log.write("warn", "disk nearly full") == "[warn] disk nearly full")
   * ```
   */
  export function write(this: void, level: std_log_Level, message: string): string
  export type Level = std_log_Level
}
