// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2014, Michael Ellerman, IBM Corp.
 */

// C header dependency: "utils.h" provides u8/u64/bool equivalents.

pub const TRACE_TYPE_REG: u8 = 1;
pub const TRACE_TYPE_COUNTER: u8 = 2;
pub const TRACE_TYPE_STRING: u8 = 3;
pub const TRACE_TYPE_INDENT: u8 = 4;
pub const TRACE_TYPE_OUTDENT: u8 = 5;

#[repr(C)]
pub struct trace_entry {
    pub type_: u8,
    pub length: u8,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct trace_buffer {
    pub size: u64,
    pub overflow: bool,
    pub tail: *mut core::ffi::c_void,
    pub data: [u8; 0],
}

unsafe extern "C" {
    pub fn trace_buffer_allocate(size: u64) -> *mut trace_buffer;
    pub fn trace_log_reg(tb: *mut trace_buffer, reg: u64, value: u64) -> core::ffi::c_int;
    pub fn trace_log_counter(tb: *mut trace_buffer, value: u64) -> core::ffi::c_int;
    pub fn trace_log_string(tb: *mut trace_buffer, str: *mut core::ffi::c_char) -> core::ffi::c_int;
    pub fn trace_log_indent(tb: *mut trace_buffer) -> core::ffi::c_int;
    pub fn trace_log_outdent(tb: *mut trace_buffer) -> core::ffi::c_int;
    pub fn trace_buffer_print(tb: *mut trace_buffer);
    pub fn trace_print_location(tb: *mut trace_buffer);
}
