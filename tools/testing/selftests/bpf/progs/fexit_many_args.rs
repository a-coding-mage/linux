// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Tencent */

use core::ffi::c_void;

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut test1_result: u64 = 0;

#[no_mangle]
#[link_section = "fexit/bpf_testmod_fentry_test7"]
pub unsafe extern "C" fn test1(
    a: u64,
    b: *mut c_void,
    c: i16,
    d: i32,
    e: *mut c_void,
    f: i8,
    g: i32,
    ret: i32,
) -> i32 {
    test1_result = (a == 16
        && b == 17 as *mut c_void
        && c == 18
        && d == 19
        && e == 20 as *mut c_void
        && f == 21
        && g == 22
        && ret == 133) as u64;
    0
}

#[no_mangle]
pub static mut test2_result: u64 = 0;

#[no_mangle]
#[link_section = "fexit/bpf_testmod_fentry_test11"]
pub unsafe extern "C" fn test2(
    a: u64,
    b: *mut c_void,
    c: i16,
    d: i32,
    e: *mut c_void,
    f: i8,
    g: i32,
    h: u32,
    i: i64,
    j: u64,
    k: u64,
    ret: i32,
) -> i32 {
    test2_result = (a == 16
        && b == 17 as *mut c_void
        && c == 18
        && d == 19
        && e == 20 as *mut c_void
        && f == 21
        && g == 22
        && h == 23
        && i == 24
        && j == 25
        && k == 26
        && ret == 231) as u64;
    0
}

#[no_mangle]
pub static mut test3_result: u64 = 0;

#[no_mangle]
#[link_section = "fexit/bpf_testmod_fentry_test11"]
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
    ret: u64,
) -> i32 {
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
        && k == 26
        && ret == 231) as u64;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
