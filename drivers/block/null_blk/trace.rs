// SPDX-License-Identifier: GPL-2.0
/*
 * null_blk trace related helpers.
 *
 * Copyright (C) 2020 Western Digital Corporation or its affiliates.
 */

// Dependency declarations supplied by trace.h and the surrounding kernel.
use core::ffi::c_char;

#[repr(C)]
pub struct trace_seq {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn trace_seq_buffer_ptr(p: *mut trace_seq) -> *const c_char;
    fn trace_seq_printf(p: *mut trace_seq, fmt: *const c_char, ...);
    fn trace_seq_putc(p: *mut trace_seq, c: i32);
}

/*
 * Helper to use for all null_blk traces to extract disk name.
 */
pub unsafe extern "C" fn nullb_trace_disk_name(
    p: *mut trace_seq,
    name: *mut c_char,
) -> *const c_char {
    let ret = unsafe { trace_seq_buffer_ptr(p) };

    if !name.is_null() && unsafe { *name } != 0 {
        static DISK_FMT: &[u8] = b"disk=%s, \0";
        unsafe { trace_seq_printf(p, DISK_FMT.as_ptr() as *const c_char, name) };
    }
    unsafe { trace_seq_putc(p, 0) };

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
