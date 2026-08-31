// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Tencent */

// C dependency intent:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut test1_result: u64 = 0;

#[unsafe(link_section = "fentry/bpf_testmod_fentry_test7")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test1(
    a: u64,
    b: *mut core::ffi::c_void,
    c: i16,
    d: i32,
    e: *mut core::ffi::c_void,
    f: i8,
    g: i32,
) -> i32 {
    unsafe {
        test1_result = (a == 16
            && b == 17 as *mut core::ffi::c_void
            && c == 18
            && d == 19
            && e == 20 as *mut core::ffi::c_void
            && f == 21
            && g == 22) as u64;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test2_result: u64 = 0;

#[unsafe(link_section = "fentry/bpf_testmod_fentry_test11")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test2(
    a: u64,
    b: *mut core::ffi::c_void,
    c: i16,
    d: i32,
    e: *mut core::ffi::c_void,
    f: i8,
    g: i32,
    h: u32,
    i: core::ffi::c_long,
    j: u64,
    k: core::ffi::c_ulong,
) -> i32 {
    unsafe {
        test2_result = (a == 16
            && b == 17 as *mut core::ffi::c_void
            && c == 18
            && d == 19
            && e == 20 as *mut core::ffi::c_void
            && f == 21
            && g == 22
            && h == 23
            && i == 24
            && j == 25
            && k == 26) as u64;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test3_result: u64 = 0;

#[unsafe(link_section = "fentry/bpf_testmod_fentry_test11")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test3(
    a: u64,
    b: u64,
    c: u64,
    d: u64,
    e: u64,
    f: u64,
    g: u64,
    h: u64,
    i: u64,
    j: u64,
    k: u64,
) -> i32 {
    unsafe {
        test3_result = (a == 16
            && b == 17
            && c == 18
            && d == 19
            && e == 20
            && f == 21
            && g == 22
            && h == 23
            && i == 24
            && j == 25
            && k == 26) as u64;
    }
    0
}
