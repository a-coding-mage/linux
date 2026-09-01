// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

// C dependencies: <limits.h>, <linux/errno.h>, "vmlinux.h",
// <bpf/bpf_helpers.h>, and "bpf_misc.h".

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type __s64 = i64;

const INT_MIN: i32 = i32::MIN;
const INT_MAX: i32 = i32::MAX;
const EINVAL: i32 = 22;
const E2BIG: i32 = 7;

// From bpf_misc.h in the original C translation unit.
extern "C" {
    static BPF_MAX_LOOPS: i32;
}

// From vmlinux.h. The concrete layout is supplied by the BPF target headers.
#[repr(C)]
pub struct bpf_iter_num {
    _opaque: [u8; 0],
}

extern "C" {
    fn bpf_iter_num_new(it: *mut bpf_iter_num, start: i32, end: i32) -> i32;
    fn bpf_iter_num_next(it: *mut bpf_iter_num) -> *mut i32;
    fn bpf_iter_num_destroy(it: *mut bpf_iter_num);
}

#[no_mangle]
pub static exp_empty_zero: __s64 = 0 + 1;
#[no_mangle]
pub static mut res_empty_zero: __s64 = 0;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn num_empty_zero(ctx: *const core::ffi::c_void) -> i32 {
    let mut sum: __s64 = 0;

    let mut i: __s64 = 0;
    while i < 0 {
        sum += i;
        i += 1;
    }
    res_empty_zero = 1 + sum;

    0
}

#[no_mangle]
pub static exp_empty_int_min: __s64 = 0 + 2;
#[no_mangle]
pub static mut res_empty_int_min: __s64 = 0;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn num_empty_int_min(ctx: *const core::ffi::c_void) -> i32 {
    let mut sum: __s64 = 0;

    let mut i: __s64 = INT_MIN as __s64;
    while i < INT_MIN as __s64 {
        sum += i;
        i += 1;
    }
    res_empty_int_min = 2 + sum;

    0
}

#[no_mangle]
pub static exp_empty_int_max: __s64 = 0 + 3;
#[no_mangle]
pub static mut res_empty_int_max: __s64 = 0;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn num_empty_int_max(ctx: *const core::ffi::c_void) -> i32 {
    let mut sum: __s64 = 0;

    let mut i: __s64 = INT_MAX as __s64;
    while i < INT_MAX as __s64 {
        sum += i;
        i += 1;
    }
    res_empty_int_max = 3 + sum;

    0
}

#[no_mangle]
pub static exp_empty_minus_one: __s64 = 0 + 4;
#[no_mangle]
pub static mut res_empty_minus_one: __s64 = 0;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn num_empty_minus_one(ctx: *const core::ffi::c_void) -> i32 {
    let mut sum: __s64 = 0;

    let mut i: __s64 = -1;
    while i < -1 {
        sum += i;
        i += 1;
    }
    res_empty_minus_one = 4 + sum;

    0
}

#[no_mangle]
pub static exp_simple_sum: __s64 = 9 * 10 / 2;
#[no_mangle]
pub static mut res_simple_sum: __s64 = 0;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn num_simple_sum(ctx: *const core::ffi::c_void) -> i32 {
    let mut sum: __s64 = 0;

    let mut i: __s64 = 0;
    while i < 10 {
        sum += i;
        i += 1;
    }
    res_simple_sum = sum;

    0
}

#[no_mangle]
pub static exp_neg_sum: __s64 = -11 * 10 / 2;
#[no_mangle]
pub static mut res_neg_sum: __s64 = 0;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn num_neg_sum(ctx: *const core::ffi::c_void) -> i32 {
    let mut sum: __s64 = 0;

    let mut i: __s64 = -10;
    while i < 0 {
        sum += i;
        i += 1;
    }
    res_neg_sum = sum;

    0
}

#[no_mangle]
pub static exp_very_neg_sum: __s64 = INT_MIN as __s64 + (INT_MIN + 1) as __s64;
#[no_mangle]
pub static mut res_very_neg_sum: __s64 = 0;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn num_very_neg_sum(ctx: *const core::ffi::c_void) -> i32 {
    let mut sum: __s64 = 0;

    let mut i: __s64 = INT_MIN as __s64;
    while i < (INT_MIN + 2) as __s64 {
        sum += i;
        i += 1;
    }
    res_very_neg_sum = sum;

    0
}

#[no_mangle]
pub static exp_very_big_sum: __s64 = (INT_MAX - 1) as __s64 + (INT_MAX - 2) as __s64;
#[no_mangle]
pub static mut res_very_big_sum: __s64 = 0;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn num_very_big_sum(ctx: *const core::ffi::c_void) -> i32 {
    let mut sum: __s64 = 0;

    let mut i: __s64 = (INT_MAX - 2) as __s64;
    while i < INT_MAX as __s64 {
        sum += i;
        i += 1;
    }
    res_very_big_sum = sum;

    0
}

#[no_mangle]
pub static exp_neg_pos_sum: __s64 = -3;
#[no_mangle]
pub static mut res_neg_pos_sum: __s64 = 0;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn num_neg_pos_sum(ctx: *const core::ffi::c_void) -> i32 {
    let mut sum: __s64 = 0;

    let mut i: __s64 = -3;
    while i < 3 {
        sum += i;
        i += 1;
    }
    res_neg_pos_sum = sum;

    0
}

#[no_mangle]
pub static exp_invalid_range: __s64 = -(EINVAL as __s64);
#[no_mangle]
pub static mut res_invalid_range: __s64 = 0;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn num_invalid_range(ctx: *const core::ffi::c_void) -> i32 {
    let mut it = core::mem::MaybeUninit::<bpf_iter_num>::uninit();

    res_invalid_range = bpf_iter_num_new(it.as_mut_ptr(), 1, 0) as __s64;
    bpf_iter_num_destroy(it.as_mut_ptr());

    0
}

#[no_mangle]
pub static exp_max_range: __s64 = 0 + 10;
#[no_mangle]
pub static mut res_max_range: __s64 = 0;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn num_max_range(ctx: *const core::ffi::c_void) -> i32 {
    let mut it = core::mem::MaybeUninit::<bpf_iter_num>::uninit();

    res_max_range = 10 + bpf_iter_num_new(it.as_mut_ptr(), 0, BPF_MAX_LOOPS) as __s64;
    bpf_iter_num_destroy(it.as_mut_ptr());

    0
}

#[no_mangle]
pub static exp_e2big_range: __s64 = -(E2BIG as __s64);
#[no_mangle]
pub static mut res_e2big_range: __s64 = 0;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn num_e2big_range(ctx: *const core::ffi::c_void) -> i32 {
    let mut it = core::mem::MaybeUninit::<bpf_iter_num>::uninit();

    res_e2big_range = bpf_iter_num_new(it.as_mut_ptr(), -1, BPF_MAX_LOOPS) as __s64;
    bpf_iter_num_destroy(it.as_mut_ptr());

    0
}

#[no_mangle]
pub static exp_succ_elem_cnt: __s64 = 10;
#[no_mangle]
pub static mut res_succ_elem_cnt: __s64 = 0;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn num_succ_elem_cnt(ctx: *const core::ffi::c_void) -> i32 {
    let mut it = core::mem::MaybeUninit::<bpf_iter_num>::uninit();
    let mut cnt: i32 = 0;
    let mut v: *mut i32;

    bpf_iter_num_new(it.as_mut_ptr(), 0, 10);
    loop {
        v = bpf_iter_num_next(it.as_mut_ptr());
        if v.is_null() {
            break;
        }
        cnt += 1;
    }
    bpf_iter_num_destroy(it.as_mut_ptr());

    res_succ_elem_cnt = cnt as __s64;

    0
}

#[no_mangle]
pub static exp_overfetched_elem_cnt: __s64 = 5;
#[no_mangle]
pub static mut res_overfetched_elem_cnt: __s64 = 0;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn num_overfetched_elem_cnt(ctx: *const core::ffi::c_void) -> i32 {
    let mut it = core::mem::MaybeUninit::<bpf_iter_num>::uninit();
    let mut cnt: i32 = 0;
    let mut v: *mut i32;
    let mut i: i32;

    bpf_iter_num_new(it.as_mut_ptr(), 0, 5);
    i = 0;
    while i < 10 {
        v = bpf_iter_num_next(it.as_mut_ptr());
        if !v.is_null() {
            cnt += 1;
        }
        i += 1;
    }
    bpf_iter_num_destroy(it.as_mut_ptr());

    res_overfetched_elem_cnt = cnt as __s64;

    0
}

#[no_mangle]
pub static exp_fail_elem_cnt: __s64 = 20 + 0;
#[no_mangle]
pub static mut res_fail_elem_cnt: __s64 = 0;

#[no_mangle]
#[link_section = "raw_tp/sys_enter"]
pub unsafe extern "C" fn num_fail_elem_cnt(ctx: *const core::ffi::c_void) -> i32 {
    let mut it = core::mem::MaybeUninit::<bpf_iter_num>::uninit();
    let mut cnt: i32 = 0;
    let mut v: *mut i32;
    let mut i: i32;

    bpf_iter_num_new(it.as_mut_ptr(), 100, 10);
    i = 0;
    while i < 10 {
        v = bpf_iter_num_next(it.as_mut_ptr());
        if !v.is_null() {
            cnt += 1;
        }
        i += 1;
    }
    bpf_iter_num_destroy(it.as_mut_ptr());

    res_fail_elem_cnt = 20 + cnt as __s64;

    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
