// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2000,2001,2004 Broadcom Corporation
 */

// Translated from csrc-bcm1480.c.
// External kernel, architecture, and register definitions are supplied by
// the surrounding translation unit.

use core::ffi::c_void;

#[repr(C)]
pub struct clocksource {
    pub name: *const u8,
    pub rating: u32,
    pub read: Option<unsafe extern "C" fn(cs: *mut clocksource) -> u64>,
    pub mask: u64,
    pub flags: u32,
}

extern "C" {
    fn __raw_readq(addr: usize) -> u64;
    fn IOADDR(addr: usize) -> usize;
    fn G_BCM1480_SYS_PLL_DIV(value: u64) -> u64;
    fn clocksource_register_hz(cs: *mut clocksource, hz: u64);
    fn sched_clock_register(read: unsafe extern "C" fn() -> u64, bits: u32, rate: u64);
}

const A_SCD_ZBBUS_CYCLE_COUNT: usize = 0;
const A_SCD_SYSTEM_CFG: usize = 0;
const CLOCK_SOURCE_IS_CONTINUOUS: u32 = 0;

const fn clocksource_mask(bits: u32) -> u64 {
    if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 }
}

unsafe extern "C" fn bcm1480_hpt_read(_cs: *mut clocksource) -> u64 {
    __raw_readq(IOADDR(A_SCD_ZBBUS_CYCLE_COUNT))
}

#[no_mangle]
pub static mut bcm1480_clocksource: clocksource = clocksource {
    name: b"zbbus-cycles\0".as_ptr(),
    rating: 200,
    read: Some(bcm1480_hpt_read),
    mask: clocksource_mask(64),
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
};

unsafe extern "C" fn sb1480_read_sched_clock() -> u64 {
    __raw_readq(IOADDR(A_SCD_ZBBUS_CYCLE_COUNT))
}

#[no_mangle]
pub unsafe extern "C" fn sb1480_clocksource_init() {
    let cs: *mut clocksource = &raw mut bcm1480_clocksource;
    let plldiv: u64;
    let zbbus: u64;

    plldiv = G_BCM1480_SYS_PLL_DIV(__raw_readq(IOADDR(A_SCD_SYSTEM_CFG)));
    zbbus = ((plldiv >> 1).wrapping_mul(50_000_000))
        .wrapping_add((plldiv & 1).wrapping_mul(25_000_000));
    clocksource_register_hz(cs, zbbus);

    sched_clock_register(sb1480_read_sched_clock, 64, zbbus);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
