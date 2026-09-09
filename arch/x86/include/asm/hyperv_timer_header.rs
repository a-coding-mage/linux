/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by asm/msr.h.
unsafe extern "C" {
    fn rdtsc_ordered() -> u64;
}

#[inline]
pub unsafe fn hv_get_raw_timer() -> u64 {
    rdtsc_ordered()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
