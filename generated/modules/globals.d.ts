// ===== globals =====

/** The runtime's version. */
declare const VOS_VERSION: string
/** The most arguments a single call takes. */
declare const MAX_ARGS: number
/**
 * Seconds since the runtime started.
 *
 * ```luau
 * assert(uptime() >= 0)
 * ```
 */
declare function uptime(this: void): number
