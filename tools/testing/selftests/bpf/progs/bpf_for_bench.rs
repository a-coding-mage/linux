// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// C dependencies omitted from executable Rust:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

use core::ffi::c_void;
use core::ptr;
use core::sync::atomic::{AtomicI64, Ordering};

extern "C" {
    fn bpf_loop(
        nr_loops: u32,
        callback_fn: unsafe extern "C" fn(index: u32, data: *mut c_void) -> i32,
        callback_ctx: *mut c_void,
        flags: u64,
    ) -> i64;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut nr_loops: i32 = 0;

#[no_mangle]
pub static mut hits: i64 = 0;

unsafe extern "C" fn outer_loop(index: u32, data: *mut c_void) -> i32 {
    let mut i: i32;

    /*
     * Empty body: the work being measured is the open-coded numeric iterator itself
     * (bpf_iter_num_new/next/destroy behind bpf_for()).
     */
    i = 0;
    while i < nr_loops {
        i = i.wrapping_add(1);
    }

    let hits_atomic = &*(ptr::addr_of!(hits) as *const AtomicI64);
    hits_atomic.fetch_add(nr_loops as i64, Ordering::SeqCst);
    let _ = index;
    let _ = data;
    0
}

#[no_mangle]
#[link_section = "fentry/sys_getpgid"]
pub unsafe extern "C" fn benchmark(ctx: *mut c_void) -> i32 {
    bpf_loop(1000, outer_loop, core::ptr::null_mut(), 0);
    let _ = ctx;
    0
}
