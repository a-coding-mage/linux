// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2023 Meta Platforms, Inc. and affiliates. */

// C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

pub type __s64 = i64;
pub type s64 = i64;
pub type u64 = u64;

#[repr(C)]
pub struct bpf_iter_testmod_seq {
    pub __bindgen_padding_0: u64,
    pub __bindgen_padding_1: u64,
}

extern "C" {
    // __ksym
    pub fn bpf_iter_testmod_seq_new(it: *mut bpf_iter_testmod_seq, value: s64, cnt: i32) -> i32;
    // __ksym
    pub fn bpf_iter_testmod_seq_next(it: *mut bpf_iter_testmod_seq) -> *mut s64;
    // __ksym
    pub fn bpf_iter_testmod_seq_value(blah: i32, it: *mut bpf_iter_testmod_seq) -> s64;
    // __ksym
    pub fn bpf_iter_testmod_seq_destroy(it: *mut bpf_iter_testmod_seq);
}

// const volatile
#[no_mangle]
pub static exp_empty: __s64 = 0 + 1;
#[no_mangle]
pub static mut res_empty: __s64 = 0;

// SEC("raw_tp/sys_enter")
// __success __log_level(2)
// __msg("fp-16=iter_testmod_seq(id=1,state=active,depth=0)")
// __msg("fp-16=iter_testmod_seq(id=1,state=drained,depth=0)")
// __msg("call bpf_iter_testmod_seq_destroy")
#[no_mangle]
pub unsafe extern "C" fn testmod_seq_empty(ctx: *const core::ffi::c_void) -> i32 {
    let mut sum: __s64 = 0;
    let mut i: *mut __s64;
    let mut it = core::mem::MaybeUninit::<bpf_iter_testmod_seq>::uninit();

    let _ = ctx;
    bpf_iter_testmod_seq_new(it.as_mut_ptr(), 1000, 0);
    loop {
        i = bpf_iter_testmod_seq_next(it.as_mut_ptr());
        if i.is_null() {
            break;
        }
        sum += *i;
    }
    bpf_iter_testmod_seq_destroy(it.as_mut_ptr());
    res_empty = 1 + sum;

    0
}

// const volatile
#[no_mangle]
pub static exp_full: __s64 = 1000000;
#[no_mangle]
pub static mut res_full: __s64 = 0;

// SEC("raw_tp/sys_enter")
// __success __log_level(2)
// __msg("fp-16=iter_testmod_seq(id=1,state=active,depth=0)")
// __msg("fp-16=iter_testmod_seq(id=1,state=drained,depth=0)")
// __msg("call bpf_iter_testmod_seq_destroy")
#[no_mangle]
pub unsafe extern "C" fn testmod_seq_full(ctx: *const core::ffi::c_void) -> i32 {
    let mut sum: __s64 = 0;
    let mut i: *mut __s64;
    let mut it = core::mem::MaybeUninit::<bpf_iter_testmod_seq>::uninit();

    let _ = ctx;
    bpf_iter_testmod_seq_new(it.as_mut_ptr(), 1000, 1000);
    loop {
        i = bpf_iter_testmod_seq_next(it.as_mut_ptr());
        if i.is_null() {
            break;
        }
        sum += *i;
    }
    bpf_iter_testmod_seq_destroy(it.as_mut_ptr());
    res_full = sum;

    0
}

// const volatile
#[no_mangle]
pub static exp_truncated: __s64 = 10 * 1000000;
#[no_mangle]
pub static mut res_truncated: __s64 = 0;

// static volatile
static mut zero: i32 = 0;

// SEC("raw_tp/sys_enter")
// __success __log_level(2)
// __msg("fp-16=iter_testmod_seq(id=1,state=active,depth=0)")
// __msg("fp-16=iter_testmod_seq(id=1,state=drained,depth=0)")
// __msg("call bpf_iter_testmod_seq_destroy")
#[no_mangle]
pub unsafe extern "C" fn testmod_seq_truncated(ctx: *const core::ffi::c_void) -> i32 {
    let mut sum: __s64 = 0;
    let mut i: *mut __s64;
    let mut cnt: i32 = core::ptr::read_volatile(core::ptr::addr_of!(zero));
    let mut it = core::mem::MaybeUninit::<bpf_iter_testmod_seq>::uninit();

    let _ = ctx;
    bpf_iter_testmod_seq_new(it.as_mut_ptr(), 10, 2000000);
    loop {
        i = bpf_iter_testmod_seq_next(it.as_mut_ptr());
        if i.is_null() {
            break;
        }
        sum += *i;
        cnt += 1;
        if cnt >= 1000000 {
            break;
        }
    }
    bpf_iter_testmod_seq_destroy(it.as_mut_ptr());
    res_truncated = sum;

    0
}

// SEC("?raw_tp")
// __failure
// __msg("expected an initialized iter_testmod_seq as R2")
#[no_mangle]
pub unsafe extern "C" fn testmod_seq_getter_before_bad(ctx: *const core::ffi::c_void) -> i32 {
    let mut it = core::mem::MaybeUninit::<bpf_iter_testmod_seq>::uninit();

    let _ = ctx;
    bpf_iter_testmod_seq_value(0, it.as_mut_ptr()) as i32
}

// SEC("?raw_tp")
// __failure
// __msg("expected an initialized iter_testmod_seq as R2")
#[no_mangle]
pub unsafe extern "C" fn testmod_seq_getter_after_bad(ctx: *const core::ffi::c_void) -> i32 {
    let mut it = core::mem::MaybeUninit::<bpf_iter_testmod_seq>::uninit();
    let mut sum: s64 = 0;
    let mut v: *mut s64;

    let _ = ctx;
    bpf_iter_testmod_seq_new(it.as_mut_ptr(), 100, 100);

    loop {
        v = bpf_iter_testmod_seq_next(it.as_mut_ptr());
        if v.is_null() {
            break;
        }
        sum += *v;
    }

    bpf_iter_testmod_seq_destroy(it.as_mut_ptr());

    (sum + bpf_iter_testmod_seq_value(0, it.as_mut_ptr())) as i32
}

// SEC("?socket")
// __success __retval(1000000)
#[no_mangle]
pub unsafe extern "C" fn testmod_seq_getter_good(ctx: *const core::ffi::c_void) -> i32 {
    let mut it = core::mem::MaybeUninit::<bpf_iter_testmod_seq>::uninit();
    let mut sum: s64 = 0;
    let mut v: *mut s64;

    let _ = ctx;
    bpf_iter_testmod_seq_new(it.as_mut_ptr(), 100, 100);

    loop {
        v = bpf_iter_testmod_seq_next(it.as_mut_ptr());
        if v.is_null() {
            break;
        }
        sum += *v;
    }

    sum *= bpf_iter_testmod_seq_value(0, it.as_mut_ptr());

    bpf_iter_testmod_seq_destroy(it.as_mut_ptr());

    sum as i32
}

// char _license[] SEC("license") = "GPL";
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
