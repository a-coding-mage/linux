// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2000, 2001 Broadcom Corporation
 */

// Translated from the Linux kernel implementation.  The symbols supplied by
// the included kernel and SB1250 headers are intentionally left external.

const SB1250_HPT_NUM: u32 = 3;
// M_SCD_TIMER_CNT (maximum value), supplied by sb1250_scd.h.
const SB1250_HPT_VALUE: u64 = M_SCD_TIMER_CNT;

extern "C" {
    static M_SCD_TIMER_CNT: u64;
    static V_SCD_TIMER_FREQ: u64;

    fn IOADDR(address: usize) -> *mut core::ffi::c_void;
    fn A_SCD_TIMER_REGISTER(timer: u32, register: u32) -> usize;
    fn G_SCD_TIMER_CNT(value: u64) -> u32;
    fn __raw_readq(address: *mut core::ffi::c_void) -> u64;
    fn __raw_writeq(value: u64, address: *mut core::ffi::c_void);
    fn clocksource_register_hz(cs: *mut clocksource, hz: u64);
    fn sched_clock_register(read: unsafe extern "C" fn() -> u64, bits: u32, hz: u64);

    static mut bcm1250_clocksource: clocksource;
}

// Opaque dependency type corresponding to struct clocksource.
#[repr(C)]
pub struct clocksource {
    pub name: *const core::ffi::c_char,
    pub rating: i32,
    pub read: unsafe extern "C" fn(*mut clocksource) -> u64,
    pub mask: u64,
    pub flags: u32,
}

const CLOCK_SOURCE_IS_CONTINUOUS: u32 = 1;

// The HPT is free running from SB1250_HPT_VALUE down to 0 then starts over
// again.
#[inline]
unsafe extern "C" fn sb1250_hpt_get_cycles() -> u64 {
    let addr = IOADDR(A_SCD_TIMER_REGISTER(SB1250_HPT_NUM, R_SCD_TIMER_CNT));
    let count = G_SCD_TIMER_CNT(__raw_readq(addr));

    SB1250_HPT_VALUE.wrapping_sub(count as u64)
}

unsafe extern "C" fn sb1250_hpt_read(_cs: *mut clocksource) -> u64 {
    sb1250_hpt_get_cycles()
}

#[no_mangle]
pub static mut bcm1250_clocksource: clocksource = clocksource {
    name: b"bcm1250-counter-3\0".as_ptr() as *const core::ffi::c_char,
    rating: 200,
    read: sb1250_hpt_read,
    mask: (1u64 << 23) - 1,
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
};

unsafe extern "C" fn sb1250_read_sched_clock() -> u64 {
    sb1250_hpt_get_cycles()
}

pub unsafe extern "C" fn sb1250_clocksource_init() {
    let cs: *mut clocksource = &raw mut bcm1250_clocksource;

    // Setup hpt using timer #3 but do not enable irq for it
    __raw_writeq(
        0,
        IOADDR(A_SCD_TIMER_REGISTER(SB1250_HPT_NUM, R_SCD_TIMER_CFG)),
    );
    __raw_writeq(
        SB1250_HPT_VALUE,
        IOADDR(A_SCD_TIMER_REGISTER(SB1250_HPT_NUM, R_SCD_TIMER_INIT)),
    );
    __raw_writeq(
        M_SCD_TIMER_ENABLE | M_SCD_TIMER_MODE_CONTINUOUS,
        IOADDR(A_SCD_TIMER_REGISTER(SB1250_HPT_NUM, R_SCD_TIMER_CFG)),
    );

    clocksource_register_hz(cs, V_SCD_TIMER_FREQ);
    sched_clock_register(sb1250_read_sched_clock, 23, V_SCD_TIMER_FREQ);
}

extern "C" {
    static R_SCD_TIMER_CNT: u32;
    static R_SCD_TIMER_CFG: u32;
    static R_SCD_TIMER_INIT: u32;
    static M_SCD_TIMER_ENABLE: u64;
    static M_SCD_TIMER_MODE_CONTINUOUS: u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
