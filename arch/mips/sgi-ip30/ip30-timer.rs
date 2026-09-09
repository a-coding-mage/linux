// SPDX-License-Identifier: GPL-2.0
/*
 * ip30-timer.c: Clocksource/clockevent support for the
 *               HEART chip in SGI Octane (IP30) systems.
 *
 * Copyright (C) 2004-2007 Stanislaw Skowronek <skylark@unaligned.org>
 * Copyright (C) 2009 Johannes Dickgreber <tanzy@gmx.de>
 * Copyright (C) 2011 Joshua Kinard <linux@kumba.dev>
 */

// Linux kernel headers and architecture-specific declarations are supplied by
// the surrounding translation unit.

use core::ffi::c_void;

pub const CLOCK_SOURCE_IS_CONTINUOUS: u32 = 1 << 0;
pub const CLOCK_SOURCE_VALID_FOR_HRES: u32 = 1 << 1;
pub const IRQ_TYPE_NONE: u32 = 0;

#[repr(C)]
pub struct HeartRegs {
    pub count: u64,
}

#[repr(C)]
pub struct Clocksource {
    pub name: *const u8,
    pub rating: i32,
    pub read: Option<unsafe extern "C" fn(*mut Clocksource) -> u64>,
    pub mask: u64,
    pub flags: u32,
}

extern "C" {
    pub static mut heart_regs: *mut HeartRegs;
    pub static mut cp0_timer_irq_installed: i32;
    pub static mut mips_clockevent_device: c_void;
    pub static mut HEART_CYCLES_PER_SEC: u32;

    pub fn heart_read(address: *const u64) -> u64;
    pub fn clocksource_register_hz(cs: *mut Clocksource, hz: u32);
    pub fn sched_clock_register(read: unsafe extern "C" fn() -> u64, bits: u32, hz: u32);
    pub fn get_c0_compare_int() -> i32;
    pub fn irq_set_handler(irq: i32, handler: unsafe extern "C" fn());
    pub fn handle_percpu_devid_irq();
    pub fn irq_set_percpu_devid(irq: i32);
    pub fn request_percpu_irq(irq: i32, handler: unsafe extern "C" fn(), name: *const u8, dev: *mut c_void) -> i32;
    pub fn enable_percpu_irq(irq: i32, irq_type: u32);
    pub fn c0_compare_interrupt();
    pub fn warn_on(condition: bool) -> bool;
}

#[inline]
unsafe fn clocksource_mask(bits: u32) -> u64 {
    if bits == 64 { u64::MAX } else { (1u64 << bits) - 1 }
}

unsafe extern "C" fn ip30_heart_counter_read(_cs: *mut Clocksource) -> u64 {
    heart_read(&(*heart_regs).count as *const u64)
}

pub static mut ip30_heart_clocksource: Clocksource = Clocksource {
    name: b"HEART\0".as_ptr(),
    rating: 400,
    read: Some(ip30_heart_counter_read),
    mask: 0xfffffffffffff,
    flags: CLOCK_SOURCE_IS_CONTINUOUS | CLOCK_SOURCE_VALID_FOR_HRES,
};

unsafe extern "C" fn ip30_heart_read_sched_clock() -> u64 {
    heart_read(&(*heart_regs).count as *const u64)
}

unsafe fn ip30_heart_clocksource_init() {
    let cs: *mut Clocksource = &raw mut ip30_heart_clocksource;

    clocksource_register_hz(cs, HEART_CYCLES_PER_SEC);

    sched_clock_register(ip30_heart_read_sched_clock, 52, HEART_CYCLES_PER_SEC);
}

pub unsafe extern "C" fn plat_time_init() {
    let irq: i32 = get_c0_compare_int();

    cp0_timer_irq_installed = 1;
    irq_set_handler(irq, handle_percpu_devid_irq);
    irq_set_percpu_devid(irq);
    let _ = warn_on(request_percpu_irq(
        irq,
        c0_compare_interrupt,
        b"timer\0".as_ptr(),
        &raw mut mips_clockevent_device,
    ) != 0);
    enable_percpu_irq(irq, IRQ_TYPE_NONE);

    ip30_heart_clocksource_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
