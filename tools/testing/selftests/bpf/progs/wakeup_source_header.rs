/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright 2026 Google LLC */

pub const WAKEUP_NAME_LEN: usize = 128;

#[repr(C)]
pub struct wakeup_event_t {
    pub active_count: core::ffi::c_ulong,
    pub active_time_ns: core::ffi::c_longlong,
    pub event_count: core::ffi::c_ulong,
    pub expire_count: core::ffi::c_ulong,
    pub last_time_ns: core::ffi::c_longlong,
    pub max_time_ns: core::ffi::c_longlong,
    pub prevent_sleep_time_ns: core::ffi::c_longlong,
    pub total_time_ns: core::ffi::c_longlong,
    pub wakeup_count: core::ffi::c_ulong,
    pub name: [core::ffi::c_char; WAKEUP_NAME_LEN],
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
