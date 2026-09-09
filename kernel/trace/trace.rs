// SPDX-License-Identifier: GPL-2.0
/*
 * Direct low-level Rust translation of trace.c.
 * Kernel-provided types, constants, macros, and functions remain external
 * dependencies, as they do in the original implementation.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

// The Linux trace implementation depends on declarations supplied by the
// surrounding kernel translation unit.
extern "C" {
    static mut tracing_disabled: c_int;
    fn trace_clock_local() -> u64;
    fn trace_clock_global() -> u64;
    fn trace_clock_counter() -> u64;
    fn trace_clock_jiffies() -> u64;
    fn trace_clock() -> u64;
    fn ktime_get_mono_fast_ns() -> u64;
    fn ktime_get_raw_fast_ns() -> u64;
    fn ktime_get_boot_fast_ns() -> u64;
    fn ktime_get_tai_fast_ns() -> u64;
    fn trace_set_ring_buffer_expanded(tr: *mut trace_array);
    fn trace_array_destroy(tr: *mut trace_array);
    fn ring_buffer_record_on(buffer: *mut trace_buffer);
    fn ring_buffer_record_off(buffer: *mut trace_buffer);
    fn ring_buffer_record_is_set_on(buffer: *mut trace_buffer) -> bool;
    fn tracing_snapshot_instance(tr: *mut trace_array);
    fn tracing_alloc_snapshot_instance(tr: *mut trace_array) -> c_int;
    fn security_locked_down(reason: c_int) -> c_int;
    fn trace_array_get(tr: *mut trace_array) -> c_int;
    fn trace_array_put(tr: *mut trace_array);
}

#[repr(C)]
pub struct trace_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct array_buffer {
    pub buffer: *mut trace_buffer,
    pub cpu: c_int,
    pub time_start: u64,
}

#[repr(C)]
pub struct trace_array {
    pub array_buffer: array_buffer,
    pub buffer_disabled: c_int,
    pub stop_count: c_int,
    pub ring_buffer_expanded: bool,
}

static mut GLOBAL_TRACE: trace_array = trace_array {
    array_buffer: array_buffer { buffer: core::ptr::null_mut(), cpu: 0, time_start: 0 },
    buffer_disabled: 1,
    stop_count: 0,
    ring_buffer_expanded: false,
};

#[no_mangle]
pub unsafe extern "C" fn ns2usecs(mut nsec: u64) -> u64 {
    nsec = nsec.wrapping_add(500);
    nsec / 1000
}

#[no_mangle]
pub unsafe extern "C" fn nsecs_to_usecs(nsecs: c_ulong) -> c_ulong {
    nsecs / 1000
}

pub type c_ulong = usize;

#[no_mangle]
pub unsafe extern "C" fn trace_set_ring_buffer_expanded_public(tr: *mut trace_array) {
    let target = if tr.is_null() { &raw mut GLOBAL_TRACE } else { tr };
    (*target).ring_buffer_expanded = true;
}

#[no_mangle]
pub unsafe extern "C" fn tracer_tracing_on(tr: *mut trace_array) {
    if !(*tr).array_buffer.buffer.is_null() {
        ring_buffer_record_on((*tr).array_buffer.buffer);
    }
    (*tr).buffer_disabled = 0;
}

#[no_mangle]
pub unsafe extern "C" fn tracing_on() {
    tracer_tracing_on(&raw mut GLOBAL_TRACE);
}

#[no_mangle]
pub unsafe extern "C" fn tracer_tracing_off(tr: *mut trace_array) {
    if !(*tr).array_buffer.buffer.is_null() {
        ring_buffer_record_off((*tr).array_buffer.buffer);
    }
    (*tr).buffer_disabled = 1;
}

#[no_mangle]
pub unsafe extern "C" fn tracing_off() {
    tracer_tracing_off(&raw mut GLOBAL_TRACE);
}

#[no_mangle]
pub unsafe extern "C" fn tracer_tracing_is_on(tr: *mut trace_array) -> bool {
    if !(*tr).array_buffer.buffer.is_null() {
        ring_buffer_record_is_set_on((*tr).array_buffer.buffer)
    } else {
        (*tr).buffer_disabled == 0
    }
}

#[no_mangle]
pub unsafe extern "C" fn tracing_is_enabled() -> c_int {
    (!GLOBAL_TRACE.buffer_disabled) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn tracing_is_on() -> c_int {
    tracer_tracing_is_on(&raw mut GLOBAL_TRACE) as c_int
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
