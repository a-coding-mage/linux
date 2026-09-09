/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by vdso/jiffies.h in the source repository:
// TICK_NSEC

/*
 * The resolution of the clocks. The resolution value is returned in
 * the clock_getres() system call to give application programmers an
 * idea of the (in)accuracy of timers. Timer values are rounded up to
 * this resolution values.
 */
pub const LOW_RES_NSEC: u64 = TICK_NSEC;
pub const KTIME_LOW_RES: u64 = LOW_RES_NSEC;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
