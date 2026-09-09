// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) ST-Ericsson SA 2011
 *
 * Author: Mattias Wallin <mattias.wallin@stericsson.com> for ST-Ericsson
 * Author: Sundar Iyer for ST-Ericsson
 * sched_clock implementation is based on:
 * plat-nomadik/timer.c Linus Walleij <linus.walleij@stericsson.com>
 *
 * DBx500-PRCMU Timer
 * The PRCMU has 5 timers which are available in a always-on
 * power domain.  We use the Timer 4 for our always-on clock
 * source on DB8500.
 */

// Dependencies supplied by other translated files.
extern "C" {
    fn readl_relaxed(addr: *mut core::ffi::c_void) -> u32;
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn of_iomap(node: *mut device_node, index: i32) -> *mut core::ffi::c_void;
    fn clocksource_register_hz(cs: *mut clocksource, hz: u32) -> i32;
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clocksource {
    pub name: *const core::ffi::c_char,
    pub rating: i32,
    pub read: Option<unsafe extern "C" fn(*mut clocksource) -> u64>,
    pub mask: u32,
    pub flags: u32,
}

const RATE_32K: u32 = 32768;

const TIMER_MODE_CONTINUOUS: u32 = 0x1;
const TIMER_DOWNCOUNT_VAL: u32 = 0xffff_ffff;

const PRCMU_TIMER_REF: usize = 0;
const PRCMU_TIMER_DOWNCOUNT: usize = 0x4;
const PRCMU_TIMER_MODE: usize = 0x8;

const CLOCK_SOURCE_IS_CONTINUOUS: u32 = 1 << 0;
const CLOCK_SOURCE_SUSPEND_NONSTOP: u32 = 1 << 1;

static mut clksrc_dbx500_timer_base: *mut core::ffi::c_void = core::ptr::null_mut();

unsafe extern "C" fn clksrc_dbx500_prcmu_read(_cs: *mut clocksource) -> u64 {
    let base = clksrc_dbx500_timer_base;
    let (mut count, mut count2): (u32, u32);

    loop {
        count = readl_relaxed(base.add(PRCMU_TIMER_DOWNCOUNT));
        count2 = readl_relaxed(base.add(PRCMU_TIMER_DOWNCOUNT));
        if count2 == count {
            break;
        }
    }

    /* Negate because the timer is a decrementing counter */
    (!count) as u64
}

static mut clocksource_dbx500_prcmu: clocksource = clocksource {
    name: b"dbx500-prcmu-timer\0".as_ptr() as *const core::ffi::c_char,
    rating: 100,
    read: Some(clksrc_dbx500_prcmu_read),
    mask: 0xffff_ffff,
    flags: CLOCK_SOURCE_IS_CONTINUOUS | CLOCK_SOURCE_SUSPEND_NONSTOP,
};

unsafe extern "C" fn clksrc_dbx500_prcmu_init(node: *mut device_node) -> i32 {
    clksrc_dbx500_timer_base = of_iomap(node, 0);

    /*
     * The A9 sub system expects the timer to be configured as
     * a continuous looping timer.
     * The PRCMU should configure it but if it for some reason
     * don't we do it here.
     */
    if readl(clksrc_dbx500_timer_base.add(PRCMU_TIMER_MODE)) != TIMER_MODE_CONTINUOUS {
        writel(
            TIMER_MODE_CONTINUOUS,
            clksrc_dbx500_timer_base.add(PRCMU_TIMER_MODE),
        );
        writel(
            TIMER_DOWNCOUNT_VAL,
            clksrc_dbx500_timer_base.add(PRCMU_TIMER_REF),
        );
    }
    clocksource_register_hz(&raw mut clocksource_dbx500_prcmu, RATE_32K)
}

// TIMER_OF_DECLARE(dbx500_prcmu, "stericsson,db8500-prcmu-timer-4",
//                  clksrc_dbx500_prcmu_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
