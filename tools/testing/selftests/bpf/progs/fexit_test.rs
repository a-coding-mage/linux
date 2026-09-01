// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Facebook */
// C dependencies removed from executable Rust:
// <linux/bpf.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

pub type __u64 = u64;

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut test1_result: __u64 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "fexit/bpf_fentry_test1")]
pub unsafe extern "C" fn test1(a: ::core::ffi::c_int, ret: ::core::ffi::c_int) -> ::core::ffi::c_int {
    unsafe {
        test1_result = (a == 1 && ret == 2) as __u64;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test2_result: __u64 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "fexit/bpf_fentry_test2")]
pub unsafe extern "C" fn test2(
    a: ::core::ffi::c_int,
    b: __u64,
    ret: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        test2_result = (a == 2 && b == 3 && ret == 5) as __u64;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test3_result: __u64 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "fexit/bpf_fentry_test3")]
pub unsafe extern "C" fn test3(
    a: ::core::ffi::c_char,
    b: ::core::ffi::c_int,
    c: __u64,
    ret: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        test3_result = (a == 4 && b == 5 && c == 6 && ret == 15) as __u64;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test4_result: __u64 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "fexit/bpf_fentry_test4")]
pub unsafe extern "C" fn test4(
    a: *mut ::core::ffi::c_void,
    b: ::core::ffi::c_char,
    c: ::core::ffi::c_int,
    d: __u64,
    ret: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        test4_result = (a == 7usize as *mut ::core::ffi::c_void
            && b == 8
            && c == 9
            && d == 10
            && ret == 34) as __u64;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test5_result: __u64 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "fexit/bpf_fentry_test5")]
pub unsafe extern "C" fn test5(
    a: __u64,
    b: *mut ::core::ffi::c_void,
    c: ::core::ffi::c_short,
    d: ::core::ffi::c_int,
    e: __u64,
    ret: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        test5_result = (a == 11
            && b == 12usize as *mut ::core::ffi::c_void
            && c == 13
            && d == 14
            && e == 15
            && ret == 65) as __u64;
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test6_result: __u64 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "fexit/bpf_fentry_test6")]
pub unsafe extern "C" fn test6(
    a: __u64,
    b: *mut ::core::ffi::c_void,
    c: ::core::ffi::c_short,
    d: ::core::ffi::c_int,
    e: *mut ::core::ffi::c_void,
    f: __u64,
    ret: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        test6_result = (a == 16
            && b == 17usize as *mut ::core::ffi::c_void
            && c == 18
            && d == 19
            && e == 20usize as *mut ::core::ffi::c_void
            && f == 21
            && ret == 111) as __u64;
    }
    0
}

#[repr(C)]
pub struct bpf_fentry_test;

#[repr(C)]
pub struct bpf_fentry_test_t {
    pub a: *mut bpf_fentry_test,
}

#[unsafe(no_mangle)]
pub static mut test7_result: __u64 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "fexit/bpf_fentry_test7")]
pub unsafe extern "C" fn test7(arg: *mut bpf_fentry_test_t) -> ::core::ffi::c_int {
    unsafe {
        if arg.is_null() {
            test7_result = 1;
        }
    }
    0
}

#[unsafe(no_mangle)]
pub static mut test8_result: __u64 = 0;

#[unsafe(no_mangle)]
#[unsafe(link_section = "fexit/bpf_fentry_test8")]
pub unsafe extern "C" fn test8(arg: *mut bpf_fentry_test_t) -> ::core::ffi::c_int {
    unsafe {
        if (*arg).a.is_null() {
            test8_result = 1;
        }
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
