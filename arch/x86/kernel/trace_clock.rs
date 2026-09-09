// SPDX-License-Identifier: GPL-2.0
/*
 * X86 trace clocks
 */

// External declarations supplied by the corresponding architecture headers:
// asm/trace_clock.h, asm/barrier.h, asm/tsc.h

extern "C" {
    fn rdtsc_ordered() -> u64;
}

/*
 * trace_clock_x86_tsc(): A clock that is just the cycle counter.
 *
 * Unlike the other clocks, this is not in nanoseconds.
 */
pub unsafe fn trace_clock_x86_tsc() -> u64 {
    unsafe { rdtsc_ordered() }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
