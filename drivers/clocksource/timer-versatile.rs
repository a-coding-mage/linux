// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2014 ARM Limited
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

pub const SYS_24MHZ: usize = 0x05c;
pub const OF_POPULATED: u32 = 0;

unsafe extern "C" {
    fn readl(addr: *const c_void) -> u32;
    fn of_iomap(node: *mut device_node, index: i32) -> *mut c_void;
    fn of_node_clear_flag(node: *mut device_node, flag: u32);
    fn sched_clock_register(read: unsafe extern "C" fn() -> u64, bits: u32, rate: u32);
}

static mut versatile_sys_24mhz: *mut c_void = core::ptr::null_mut();

unsafe extern "C" fn versatile_sys_24mhz_read() -> u64 {
    readl(versatile_sys_24mhz as *const c_void) as u64
}

unsafe extern "C" fn versatile_sched_clock_init(node: *mut device_node) -> i32 {
    let base: *mut c_void = of_iomap(node, 0);

    of_node_clear_flag(node, OF_POPULATED);

    if base.is_null() {
        return -6; // -ENXIO
    }

    versatile_sys_24mhz = (base as *mut u8).add(SYS_24MHZ) as *mut c_void;

    sched_clock_register(versatile_sys_24mhz_read, 32, 24000000);

    0
}

// TIMER_OF_DECLARE(vexpress, "arm,vexpress-sysreg", versatile_sched_clock_init);
#[no_mangle]
pub static vexpress: (&'static str, unsafe extern "C" fn(*mut device_node) -> i32) =
    ("arm,vexpress-sysreg", versatile_sched_clock_init);

// TIMER_OF_DECLARE(versatile, "arm,versatile-sysreg", versatile_sched_clock_init);
#[no_mangle]
pub static versatile: (&'static str, unsafe extern "C" fn(*mut device_node) -> i32) =
    ("arm,versatile-sysreg", versatile_sched_clock_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
