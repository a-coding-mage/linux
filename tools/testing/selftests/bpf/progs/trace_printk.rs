// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2020, Oracle and/or its affiliates.

// C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>
// #include "bpf_misc.h"

#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_void};

unsafe extern "C" {
    fn bpf_trace_printk(fmt: *const c_char, fmt_size: u32, ...) -> i64;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [c_char; 4] = [b'G' as c_char, b'P' as c_char, b'L' as c_char, 0];

#[unsafe(no_mangle)]
pub static mut trace_printk_ret: c_int = 0;
#[unsafe(no_mangle)]
pub static mut trace_printk_ran: c_int = 0;
#[unsafe(no_mangle)]
pub static mut trace_printk_invalid_spec_ret: c_int = 0;
#[unsafe(no_mangle)]
pub static mut trace_printk_utf8_ret: c_int = 0;
#[unsafe(no_mangle)]
pub static mut trace_printk_utf8_ran: c_int = 0;

#[unsafe(no_mangle)]
pub static fmt: [c_char; 20] = [
    b'T' as c_char,
    b'e' as c_char,
    b's' as c_char,
    b't' as c_char,
    b'i' as c_char,
    b'n' as c_char,
    b'g' as c_char,
    b',' as c_char,
    b't' as c_char,
    b'e' as c_char,
    b's' as c_char,
    b't' as c_char,
    b'i' as c_char,
    b'n' as c_char,
    b'g' as c_char,
    b' ' as c_char,
    b'%' as c_char,
    b'd' as c_char,
    b'\n' as c_char,
    0,
];

static utf8_fmt: [c_char; 18] = [
    0xe4u8 as c_char,
    0xb8u8 as c_char,
    0xadu8 as c_char,
    0xe6u8 as c_char,
    0x96u8 as c_char,
    0x87u8 as c_char,
    b',' as c_char,
    0xe6u8 as c_char,
    0xb5u8 as c_char,
    0x8bu8 as c_char,
    0xe8u8 as c_char,
    0xafu8 as c_char,
    0x95u8 as c_char,
    b' ' as c_char,
    b'%' as c_char,
    b'd' as c_char,
    b'\n' as c_char,
    0,
];

/* Non-ASCII bytes after '%' must still be rejected. */
static invalid_spec_fmt: [c_char; 4] = [b'%' as c_char, 0x80u8 as c_char, b'\n' as c_char, 0];

// Original section: SEC("fentry/" SYS_PREFIX "sys_nanosleep")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_enter(ctx: *mut c_void) -> c_int {
    let _ = ctx;

    trace_printk_ran += 1;
    trace_printk_ret = bpf_trace_printk(
        fmt.as_ptr(),
        core::mem::size_of_val(&fmt) as u32,
        trace_printk_ran,
    ) as c_int;

    trace_printk_utf8_ran += 1;
    trace_printk_utf8_ret = bpf_trace_printk(
        utf8_fmt.as_ptr(),
        core::mem::size_of_val(&utf8_fmt) as u32,
        trace_printk_utf8_ran,
    ) as c_int;

    trace_printk_invalid_spec_ret = bpf_trace_printk(
        invalid_spec_fmt.as_ptr(),
        core::mem::size_of_val(&invalid_spec_fmt) as u32,
    ) as c_int;

    0
}
