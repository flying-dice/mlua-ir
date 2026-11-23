// ===== std:env =====

declare module "std:env" {
  /**
   * The value of a variable in the demo's fixed environment, or `nil` when it is not set. Not being set is
   * normal absence, not an error.
   *
   * ```luau
   * assert(env.var("HOME") == "/home/demo")
   * assert(env.var("NOPE") == nil)
   * ```
   */
  function var_(this: void, name: string): string | undefined
  /** `var(name)`, or `fallback` (default `""`) when it is not set. */
  export function var_or(this: void, name: string, fallback?: string): string
  export { var_ as var }
}
