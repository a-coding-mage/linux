/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Backtrace support for Microblaze
 *
 * Copyright (C) 2010  Digital Design Corporation
 */

use core::ffi::{c_char, c_ulong};

#[repr(C)]
pub struct stack_trace {
    _private: [u8; 0],
}

#[repr(C)]
pub struct trap_handler_info {
    pub start_addr: c_ulong,
    pub end_addr: c_ulong,
    pub trap_name: *const c_char,
}

unsafe extern "C" {
    pub static mut microblaze_trap_handlers: trap_handler_info;

    pub static _hw_exception_handler: c_char;
    pub static ex_handler_unhandled: c_char;

    pub fn microblaze_unwind(
        task: *mut task_struct,
        trace: *mut stack_trace,
        loglvl: *const c_char,
    );
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
