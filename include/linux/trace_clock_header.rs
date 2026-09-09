/* SPDX-License-Identifier: GPL-2.0 */

/*
 * 3 trace clock variants, with differing scalability/precision
 * tradeoffs:
 *
 *  -   local: CPU-local trace clock
 *  -   medium: scalable global clock with some jitter
 *  -   global: globally monotonic, serialized clock
 */

/* Declarations supplied by the Linux compiler, type, and architecture headers. */

extern "C" {
    pub fn trace_clock_local() -> u64;
    pub fn trace_clock() -> u64;
    pub fn trace_clock_jiffies() -> u64;
    pub fn trace_clock_global() -> u64;
    pub fn trace_clock_counter() -> u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
