/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/include/asm-parisc/timex.h
 *
 * PARISC architecture timex specifications
 */

// Dependency supplied by asm/special_insns.h.
use core::ffi::c_ulong;

pub type cycles_t = c_ulong;

unsafe extern "C" {
    fn mfctl(reg: c_ulong) -> c_ulong;
}

pub unsafe fn get_cycles() -> cycles_t {
    unsafe { mfctl(16) }
}

// C macro alias: #define get_cycles get_cycles

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
