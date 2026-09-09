/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2014-2015, The Linux Foundation. All rights reserved.
 */
// C translation notes:
// TRACE_SYSTEM is `clk`; the Linux tracepoint header and define_trace header
// are supplied by the surrounding kernel integration.

use core::ffi::c_char;

#[repr(C)]
pub struct clk_core {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_duty {
    pub num: u32,
    pub den: u32,
}

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_rate_request {
    pub core: *mut clk_core,
    pub best_parent_hw: *mut clk_hw,
    pub min_rate: c_ulong,
    pub max_rate: c_ulong,
    pub best_parent_rate: c_ulong,
}

pub type c_ulong = usize;

#[repr(C)]
pub struct clk_event {
    pub name: *const c_char,
}

#[repr(C)]
pub struct clk_rate_event {
    pub name: *const c_char,
    pub rate: c_ulong,
}

#[repr(C)]
pub struct clk_rate_range_event {
    pub name: *const c_char,
    pub min: c_ulong,
    pub max: c_ulong,
}

#[repr(C)]
pub struct clk_parent_event {
    pub name: *const c_char,
    pub pname: *const c_char,
}

#[repr(C)]
pub struct clk_phase_event {
    pub name: *const c_char,
    pub phase: i32,
}

#[repr(C)]
pub struct clk_duty_cycle_event {
    pub name: *const c_char,
    pub num: u32,
    pub den: u32,
}

#[repr(C)]
pub struct clk_rate_request_event {
    pub name: *const c_char,
    pub pname: *const c_char,
    pub min: c_ulong,
    pub max: c_ulong,
    pub prate: c_ulong,
}

extern "C" {
    pub fn clk_enable(core: *mut clk_core);
    pub fn clk_enable_complete(core: *mut clk_core);
    pub fn clk_disable(core: *mut clk_core);
    pub fn clk_disable_complete(core: *mut clk_core);
    pub fn clk_prepare(core: *mut clk_core);
    pub fn clk_prepare_complete(core: *mut clk_core);
    pub fn clk_unprepare(core: *mut clk_core);
    pub fn clk_unprepare_complete(core: *mut clk_core);

    pub fn clk_set_rate(core: *mut clk_core, rate: c_ulong);
    pub fn clk_set_rate_complete(core: *mut clk_core, rate: c_ulong);
    pub fn clk_set_min_rate(core: *mut clk_core, rate: c_ulong);
    pub fn clk_set_max_rate(core: *mut clk_core, rate: c_ulong);
    pub fn clk_set_rate_range(core: *mut clk_core, min: c_ulong, max: c_ulong);

    pub fn clk_set_parent(core: *mut clk_core, parent: *mut clk_core);
    pub fn clk_set_parent_complete(core: *mut clk_core, parent: *mut clk_core);

    pub fn clk_set_phase(core: *mut clk_core, phase: i32);
    pub fn clk_set_phase_complete(core: *mut clk_core, phase: i32);

    pub fn clk_set_duty_cycle(core: *mut clk_core, duty: *mut clk_duty);
    pub fn clk_set_duty_cycle_complete(core: *mut clk_core, duty: *mut clk_duty);

    pub fn clk_rate_request_start(req: *mut clk_rate_request);
    pub fn clk_rate_request_done(req: *mut clk_rate_request);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
