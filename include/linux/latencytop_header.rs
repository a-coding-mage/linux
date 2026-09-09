/* SPDX-License-Identifier: GPL-2.0 */
/*
 * latencytop.h: Infrastructure for displaying latency
 *
 * (C) Copyright 2008 Intel Corporation
 * Author: Arjan van de Ven <arjan@linux.intel.com>
 *
 */

// The C header guard and include are not executable Rust constructs.

use core::ffi::c_int;

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

// CONFIG_LATENCYTOP is a build-time C configuration condition.  Enable the
// following items when the corresponding Rust configuration is selected.
#[cfg(feature = "CONFIG_LATENCYTOP")]
pub const LT_SAVECOUNT: usize = 32;

#[cfg(feature = "CONFIG_LATENCYTOP")]
pub const LT_BACKTRACEDEPTH: usize = 12;

#[cfg(feature = "CONFIG_LATENCYTOP")]
#[repr(C)]
pub struct latency_record {
    pub backtrace: [usize; LT_BACKTRACEDEPTH],
    pub count: u32,
    pub time: usize,
    pub max: usize,
}

#[cfg(feature = "CONFIG_LATENCYTOP")]
extern "C" {
    pub static mut latencytop_enabled: c_int;
    pub fn __account_scheduler_latency(
        task: *mut task_struct,
        usecs: c_int,
        inter: c_int,
    );
}

#[cfg(feature = "CONFIG_LATENCYTOP")]
#[inline]
pub unsafe fn account_scheduler_latency(
    task: *mut task_struct,
    usecs: c_int,
    inter: c_int,
) {
    // C's unlikely() branch prediction hint has no required Rust equivalent.
    if latencytop_enabled != 0 {
        __account_scheduler_latency(task, usecs, inter);
    }
}

#[cfg(feature = "CONFIG_LATENCYTOP")]
extern "C" {
    pub fn clear_tsk_latency_tracing(p: *mut task_struct);
}

#[cfg(not(feature = "CONFIG_LATENCYTOP"))]
#[inline]
pub unsafe fn account_scheduler_latency(
    _task: *mut task_struct,
    _usecs: c_int,
    _inter: c_int,
) {
}

#[cfg(not(feature = "CONFIG_LATENCYTOP"))]
#[inline]
pub unsafe fn clear_tsk_latency_tracing(_p: *mut task_struct) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
