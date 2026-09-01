// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Facebook */
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut test1_result: u64 = 0;

#[unsafe(link_section = "fentry/bpf_fentry_test1")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test1(a: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        test1_result = (a == 1) as u64;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test2_result: u64 = 0;

#[unsafe(link_section = "fentry/bpf_fentry_test2")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test2(a: ::core::ffi::c_int, b: u64) -> ::core::ffi::c_int {
    unsafe {
        test2_result = (a == 2 && b == 3) as u64;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test3_result: u64 = 0;

#[unsafe(link_section = "fentry/bpf_fentry_test3")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test3(
    a: ::core::ffi::c_char,
    b: ::core::ffi::c_int,
    c: u64,
) -> ::core::ffi::c_int {
    unsafe {
        test3_result = (a == 4 && b == 5 && c == 6) as u64;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test4_result: u64 = 0;

#[unsafe(link_section = "fentry/bpf_fentry_test4")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test4(
    a: *mut ::core::ffi::c_void,
    b: ::core::ffi::c_char,
    c: ::core::ffi::c_int,
    d: u64,
) -> ::core::ffi::c_int {
    unsafe {
        test4_result = (a == 7usize as *mut ::core::ffi::c_void
            && b == 8
            && c == 9
            && d == 10) as u64;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test5_result: u64 = 0;

#[unsafe(link_section = "fentry/bpf_fentry_test5")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test5(
    a: u64,
    b: *mut ::core::ffi::c_void,
    c: ::core::ffi::c_short,
    d: ::core::ffi::c_int,
    e: u64,
) -> ::core::ffi::c_int {
    unsafe {
        test5_result = (a == 11
            && b == 12usize as *mut ::core::ffi::c_void
            && c == 13
            && d == 14
            && e == 15) as u64;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test6_result: u64 = 0;

#[unsafe(link_section = "fentry/bpf_fentry_test6")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test6(
    a: u64,
    b: *mut ::core::ffi::c_void,
    c: ::core::ffi::c_short,
    d: ::core::ffi::c_int,
    e: *mut ::core::ffi::c_void,
    f: u64,
) -> ::core::ffi::c_int {
    unsafe {
        test6_result = (a == 16
            && b == 17usize as *mut ::core::ffi::c_void
            && c == 18
            && d == 19
            && e == 20usize as *mut ::core::ffi::c_void
            && f == 21) as u64;
    }
    0
}

#[repr(C)]
pub struct bpf_fentry_test_t {
    pub a: *mut bpf_fentry_test_t,
}

#[unsafe(no_mangle)]
pub static mut test7_result: u64 = 0;

#[unsafe(link_section = "fentry/bpf_fentry_test7")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test7(arg: *mut bpf_fentry_test_t) -> ::core::ffi::c_int {
    unsafe {
        if arg.is_null() {
            test7_result = 1;
        }
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test8_result: u64 = 0;

#[unsafe(link_section = "fentry/bpf_fentry_test8")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test8(arg: *mut bpf_fentry_test_t) -> ::core::ffi::c_int {
    unsafe {
        if (*arg).a == 0usize as *mut bpf_fentry_test_t {
            test8_result = 1;
        }
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
