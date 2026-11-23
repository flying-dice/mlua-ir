// ===== std:collections =====

/** A counter. Clones share one value. */
interface std_collections_Counter {
  /** The current value. */
  readonly value: number
  /** Add one; the new value. */
  increment(): number
  /** Add `n`; the new value. */
  add(n: number): number
  /** Set the value to `n`. */
  set(n: number): void
}

/** The statics of `std_collections_Counter`. */
interface std_collections_CounterStatics {
  /**
   * A counter starting at `initial` (default 0).
   *
   * ```luau
   * local c = collections.Counter.new(41)
   * assert(c:increment() == 42 and c.value == 42)
   * ```
   */
  "new"(this: void, initial?: number): std_collections_Counter
}

declare module "std:collections" {
  export const Counter: std_collections_CounterStatics
  export type Counter = std_collections_Counter
}
