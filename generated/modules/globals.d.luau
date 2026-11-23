-- ===== globals =====

--- The runtime's version.
declare VOS_VERSION: string
--- The most arguments a single call takes.
declare MAX_ARGS: number
--- Seconds since the runtime started.
---
--- ```luau
--- assert(uptime() >= 0)
--- ```
declare function uptime(): number
