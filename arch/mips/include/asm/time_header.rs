/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2001, 2002, MontaVista Software Inc.
 * Author: Jun Sun, jsun@mvista.com or jsun@junsun.net
 * Copyright (c) 2003  Maciej W. Rozycki
 *
 * include/asm-mips/time.h
 *     header file for the new style time.c file and time services.
 */

// C dependencies supplied by other translated headers are intentionally
// referenced here rather than reimplemented.

extern "C" {
    pub static mut rtc_lock: spinlock_t;

    /*
     * board specific routines required by time_init().
     */
    pub fn plat_time_init();

    /*
     * mips_hpt_frequency - must be set if you intend to use an R4k-compatible
     * counter as a timer interrupt source.
     */
    pub static mut mips_hpt_frequency: ::core::ffi::c_uint;

    /*
     * The performance counter IRQ on MIPS is a close relative to the timer IRQ
     * so it lives here.
     */
    pub static mut perf_irq: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>;
    // __weak declaration; weak-linkage support is supplied by the build system.
    pub fn get_c0_perfcount_int() -> ::core::ffi::c_int;

    /*
     * Initialize the calling CPU's compare interrupt as clockevent device
     */
    pub fn get_c0_compare_int() -> ::core::ffi::c_uint;
    pub fn r4k_clockevent_init() -> ::core::ffi::c_int;

    /*
     * Initialize the count register as a clocksource
     */
    pub fn init_r4k_clocksource() -> ::core::ffi::c_int;

    pub fn clockevents_calc_mult_shift(
        cd: *mut clock_event_device,
        clock: ::core::ffi::c_uint,
        shift: ::core::ffi::c_uint,
    );
}

#[inline]
pub unsafe fn mips_clockevent_init() -> ::core::ffi::c_int {
    #[cfg(CONFIG_CEVT_R4K)]
    {
        r4k_clockevent_init()
    }
    #[cfg(not(CONFIG_CEVT_R4K))]
    {
        -ENXIO
    }
}

#[inline]
pub unsafe fn init_mips_clocksource() -> ::core::ffi::c_int {
    #[cfg(CONFIG_CSRC_R4K)]
    {
        init_r4k_clocksource()
    }
    #[cfg(not(CONFIG_CSRC_R4K))]
    {
        0
    }
}

#[inline]
pub unsafe fn clockevent_set_clock(cd: *mut clock_event_device, clock: ::core::ffi::c_uint) {
    clockevents_calc_mult_shift(cd, clock, 4);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
