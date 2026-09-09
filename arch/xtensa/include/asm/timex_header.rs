/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2013 Tensilica Inc.
 */

// Dependency: <asm/processor.h>

// Build-time selection equivalent to:
// XCHAL_NUM_TIMERS > 0 && XTENSA_INT_LEVEL(XCHAL_TIMER0_INTERRUPT) <= XCHAL_EXCM_LEVEL
pub const LINUX_TIMER: usize = 0;
// Selected interrupt: XCHAL_TIMER0_INTERRUPT
pub const LINUX_TIMER_INT: usize = XCHAL_TIMER0_INTERRUPT;

extern "C" {
    pub static mut ccount_freq: ::core::ffi::c_ulong;

    pub fn local_timer_setup(cpu: ::core::ffi::c_uint);
}

/*
 * Register access.
 */

#[inline]
pub unsafe fn get_ccount() -> ::core::ffi::c_ulong {
    xtensa_get_sr(ccount)
}

#[inline]
pub unsafe fn set_ccount(ccount: ::core::ffi::c_ulong) {
    xtensa_set_sr(ccount, ccount);
}

#[inline]
pub unsafe fn get_linux_timer() -> ::core::ffi::c_ulong {
    xtensa_get_sr(SREG_CCOMPARE + LINUX_TIMER)
}

#[inline]
pub unsafe fn set_linux_timer(ccompare: ::core::ffi::c_ulong) {
    xtensa_set_sr(ccompare, SREG_CCOMPARE + LINUX_TIMER);
}

// Dependency: <asm-generic/timex.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
