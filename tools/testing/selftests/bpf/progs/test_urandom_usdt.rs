// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2022 Meta Platforms, Inc. and affiliates. */

// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/usdt.bpf.h>

#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use core::sync::atomic::{AtomicI32, Ordering};

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
}

unsafe fn __sync_fetch_and_add_i32(ptr: *mut i32, val: i32) -> i32 {
    unsafe { AtomicI32::from_ptr(ptr).fetch_add(val, Ordering::SeqCst) }
}

#[no_mangle]
pub static mut urand_pid: i32 = 0;

#[no_mangle]
pub static mut urand_read_without_sema_call_cnt: i32 = 0;
#[no_mangle]
pub static mut urand_read_without_sema_buf_sz_sum: i32 = 0;

#[no_mangle]
#[link_section = "usdt/./urandom_read:urand:read_without_sema"]
pub unsafe extern "C" fn urand_read_without_sema(
    iter_num: i32,
    iter_cnt: i32,
    buf_sz: i32,
) -> i32 {
    if unsafe { urand_pid } != (unsafe { bpf_get_current_pid_tgid() } >> 32) as i32 {
        return 0;
    }

    unsafe {
        __sync_fetch_and_add_i32(&raw mut urand_read_without_sema_call_cnt, 1);
        __sync_fetch_and_add_i32(&raw mut urand_read_without_sema_buf_sz_sum, buf_sz);
    }

    return 0;
}

#[no_mangle]
pub static mut urand_read_with_sema_call_cnt: i32 = 0;
#[no_mangle]
pub static mut urand_read_with_sema_buf_sz_sum: i32 = 0;

#[no_mangle]
#[link_section = "usdt/./urandom_read:urand:read_with_sema"]
pub unsafe extern "C" fn urand_read_with_sema(
    iter_num: i32,
    iter_cnt: i32,
    buf_sz: i32,
) -> i32 {
    if unsafe { urand_pid } != (unsafe { bpf_get_current_pid_tgid() } >> 32) as i32 {
        return 0;
    }

    unsafe {
        __sync_fetch_and_add_i32(&raw mut urand_read_with_sema_call_cnt, 1);
        __sync_fetch_and_add_i32(&raw mut urand_read_with_sema_buf_sz_sum, buf_sz);
    }

    return 0;
}

#[no_mangle]
pub static mut urandlib_read_without_sema_call_cnt: i32 = 0;
#[no_mangle]
pub static mut urandlib_read_without_sema_buf_sz_sum: i32 = 0;

#[no_mangle]
#[link_section = "usdt/./liburandom_read.so:urandlib:read_without_sema"]
pub unsafe extern "C" fn urandlib_read_without_sema(
    iter_num: i32,
    iter_cnt: i32,
    buf_sz: i32,
) -> i32 {
    if unsafe { urand_pid } != (unsafe { bpf_get_current_pid_tgid() } >> 32) as i32 {
        return 0;
    }

    unsafe {
        __sync_fetch_and_add_i32(&raw mut urandlib_read_without_sema_call_cnt, 1);
        __sync_fetch_and_add_i32(&raw mut urandlib_read_without_sema_buf_sz_sum, buf_sz);
    }

    return 0;
}

#[no_mangle]
pub static mut urandlib_read_with_sema_call_cnt: i32 = 0;
#[no_mangle]
pub static mut urandlib_read_with_sema_buf_sz_sum: i32 = 0;

#[no_mangle]
#[link_section = "usdt/./liburandom_read.so:urandlib:read_with_sema"]
pub unsafe extern "C" fn urandlib_read_with_sema(
    iter_num: i32,
    iter_cnt: i32,
    buf_sz: i32,
) -> i32 {
    if unsafe { urand_pid } != (unsafe { bpf_get_current_pid_tgid() } >> 32) as i32 {
        return 0;
    }

    unsafe {
        __sync_fetch_and_add_i32(&raw mut urandlib_read_with_sema_call_cnt, 1);
        __sync_fetch_and_add_i32(&raw mut urandlib_read_with_sema_buf_sz_sum, buf_sz);
    }

    return 0;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
