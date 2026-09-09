/* SPDX-License-Identifier: GPL-2.0 */

// HZ is supplied by asm/param.h.
// NSEC_PER_SEC is supplied by vdso/time64.h.

/* TICK_NSEC is the time between ticks in nsec */
pub const TICK_NSEC: u64 = (NSEC_PER_SEC + HZ / 2) / HZ;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
