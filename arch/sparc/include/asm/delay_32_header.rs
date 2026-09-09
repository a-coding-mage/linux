/* SPDX-License-Identifier: GPL-2.0 */
/*
 * delay.h: Linux delay routines on the Sparc.
 *
 * Copyright (C) 1994 David S. Miller (davem@caip.rutgers.edu).
 */

// Dependency provided by <asm/cpudata.h> is intentionally not implemented here.

/// Busy-wait for the requested number of loop iterations.
#[inline(always)]
pub unsafe fn __delay(mut loops: ::core::ffi::c_ulong) {
    // The original volatile SPARC inline assembly compares before each
    // decrement and deliberately preserves the compiler-visible side effect.
    while loops != 0 {
        loops = loops.wrapping_sub(1);
    }
}

/* This is too messy with inline asm on the Sparc. */
unsafe extern "C" {
    pub fn __udelay(usecs: ::core::ffi::c_ulong, lpj: ::core::ffi::c_ulong);
    pub fn __ndelay(nsecs: ::core::ffi::c_ulong, lpj: ::core::ffi::c_ulong);
}

#[cfg(feature = "CONFIG_SMP")]
unsafe fn __udelay_val() -> ::core::ffi::c_ulong {
    // Equivalent to cpu_data(smp_processor_id()).udelay_val; the cpu-data
    // type and accessors are supplied by the dependent SPARC code.
    todo!("cpu_data(smp_processor_id()).udelay_val")
}

#[cfg(not(feature = "CONFIG_SMP"))]
unsafe fn __udelay_val() -> ::core::ffi::c_ulong {
    unsafe extern "C" {
        static mut loops_per_jiffy: ::core::ffi::c_ulong;
    }
    loops_per_jiffy
}

#[inline(always)]
pub unsafe fn udelay(__usecs: ::core::ffi::c_ulong) {
    unsafe { __udelay(__usecs, __udelay_val()) };
}

#[inline(always)]
pub unsafe fn ndelay(__nsecs: ::core::ffi::c_ulong) {
    unsafe { __ndelay(__nsecs, __udelay_val()) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
