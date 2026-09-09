/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * (C) Copyright 2009 Intel Corporation
 * Author: Jacob Pan (jacob.jun.pan@intel.com)
 *
 * Shared with ARM platforms, Jamie Iles, Picochip 2011
 *
 * Support for the Synopsys DesignWare APB Timers.
 */

// Dependencies supplied by the corresponding kernel interfaces.
use core::ffi::{c_char, c_int, c_ulong, c_uint, c_void};

pub const APBTMRS_REG_SIZE: c_uint = 0x14;

#[repr(C)]
pub struct dw_apb_timer {
    pub base: *mut c_void,
    pub freq: c_ulong,
    pub irq: c_int,
}

#[repr(C)]
pub struct dw_apb_clock_event_device {
    pub ced: clock_event_device,
    pub timer: dw_apb_timer,
    pub eoi: Option<unsafe extern "C" fn(*mut dw_apb_timer)>,
}

#[repr(C)]
pub struct dw_apb_clocksource {
    pub timer: dw_apb_timer,
    pub cs: clocksource,
}

extern "C" {
    pub fn dw_apb_clockevent_register(dw_ced: *mut dw_apb_clock_event_device);

    pub fn dw_apb_clockevent_init(
        cpu: c_int,
        name: *const c_char,
        rating: c_uint,
        base: *mut c_void,
        irq: c_int,
        freq: c_ulong,
    ) -> *mut dw_apb_clock_event_device;

    pub fn dw_apb_clocksource_init(
        rating: c_uint,
        name: *const c_char,
        base: *mut c_void,
        freq: c_ulong,
    ) -> *mut dw_apb_clocksource;

    pub fn dw_apb_clocksource_register(dw_cs: *mut dw_apb_clocksource);
    pub fn dw_apb_clocksource_start(dw_cs: *mut dw_apb_clocksource);
    pub fn dw_apb_clocksource_read(dw_cs: *mut dw_apb_clocksource) -> u64;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
