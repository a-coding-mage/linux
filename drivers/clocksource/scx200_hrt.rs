// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2006 Jim Cromie
 *
 * This is a clocksource driver for the Geode SCx200's 1 or 27 MHz
 * high-resolution timer.  The Geode SC-1100 (at least) has a buggy
 * time stamp counter (TSC), which loses time unless 'idle=poll' is
 * given as a boot-arg. In its absence, the Generic Timekeeping code
 * will detect and de-rate the bad TSC, allowing this timer to take
 * over timekeeping duties.
 *
 * Based on work by John Stultz, and Ted Phelps (in a 2.6.12-rc6 patch)
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const NAME: &[u8] = b"scx200_hrt\0";

static mut mhz27: c_int = 0;
static mut ppm: c_int = 0;

/* HiRes Timer configuration register address */
const SCX200_TMCNFG_OFFSET: c_ulong = SCX200_TIMER_OFFSET + 5;

/* and config settings */
const HR_TMEN: u8 = 1 << 0; /* timer interrupt enable */
const HR_TMCLKSEL: u8 = 1 << 1; /* 1|0 counts at 27|1 MHz */
const HR_TM27MPD: u8 = 1 << 2; /* 1 turns off input clock (power-down) */

/* The base timer frequency, * 27 if selected */
const HRT_FREQ: u32 = 1000000;

#[repr(C)]
pub struct clocksource {
    pub name: *const c_char,
    pub rating: c_int,
    pub read: Option<unsafe extern "C" fn(*mut clocksource) -> u64>,
    pub mask: u64,
    pub flags: u32,
    pub owner: *mut c_void,
}

extern "C" {
    static mut scx200_cb_base: c_ulong;
    fn scx200_cb_present() -> bool;
    fn request_region(start: c_ulong, n: c_ulong, name: *const c_char) -> *mut c_void;
    fn inl(addr: c_ulong) -> u32;
    fn outb(value: u8, addr: c_ulong);
    fn clocksource_register_hz(cs: *mut clocksource, hz: u32) -> c_int;
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
}

const SCX200_TIMER_OFFSET: c_ulong = 0; // supplied by linux/scx200.h
const SCX200_TIMER_SIZE: c_ulong = 0; // supplied by linux/scx200.h
const CLOCK_SOURCE_IS_CONTINUOUS: u32 = 1; // supplied by linux/clocksource.h
const CLOCKSOURCE_MASK_32: u64 = 0xffff_ffff;
const THIS_MODULE: *mut c_void = core::ptr::null_mut();

unsafe extern "C" fn read_hrt(_cs: *mut clocksource) -> u64 {
    /* Read the timer value */
    inl(scx200_cb_base + SCX200_TIMER_OFFSET) as u64
}

static mut cs_hrt: clocksource = clocksource {
    name: b"scx200_hrt\0".as_ptr() as *const c_char,
    rating: 250,
    read: Some(read_hrt),
    mask: CLOCKSOURCE_MASK_32,
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
    /* mult, shift are set based on mhz27 flag */
    owner: THIS_MODULE,
};

unsafe extern "C" fn init_hrt_clocksource() -> c_int {
    let mut freq: u32;
    /* Make sure scx200 has initialized the configuration block */
    if !scx200_cb_present() {
        return -19; // -ENODEV
    }

    /* Reserve the timer's ISA io-region for ourselves */
    if request_region(
        scx200_cb_base + SCX200_TIMER_OFFSET,
        SCX200_TIMER_SIZE,
        b"NatSemi SCx200 High-Resolution Timer\0".as_ptr() as *const c_char,
    )
    .is_null()
    {
        pr_warn(b"unable to lock timer region\n\0".as_ptr() as *const c_char);
        return -19; // -ENODEV
    }

    /* write timer config */
    outb(
        HR_TMEN | (if mhz27 != 0 { HR_TMCLKSEL } else { 0 }),
        scx200_cb_base + SCX200_TMCNFG_OFFSET,
    );

    freq = HRT_FREQ.wrapping_add(ppm as u32);
    if mhz27 != 0 {
        freq = freq.wrapping_mul(27);
    }

    pr_info(
        b"enabling scx200 high-res timer (%s MHz +%d ppm)\n\0".as_ptr() as *const c_char,
        if mhz27 != 0 { b"27\0".as_ptr() } else { b"1\0".as_ptr() },
        ppm,
    );

    clocksource_register_hz(&raw mut cs_hrt, freq)
}

// module_init(init_hrt_clocksource);
// MODULE_AUTHOR("Jim Cromie <jim.cromie@gmail.com>");
// MODULE_DESCRIPTION("clocksource on SCx200 HiRes Timer");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
