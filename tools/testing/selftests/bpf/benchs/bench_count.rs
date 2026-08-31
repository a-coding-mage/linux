// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
// Depends on declarations from bench.h.
use core::ffi::c_void;
use core::ptr;

use crate::*;

/* COUNT-GLOBAL benchmark */

#[repr(C)]
struct count_global_ctx {
    hits: counter,
}

static mut count_global_ctx: count_global_ctx = unsafe { core::mem::zeroed() };

unsafe extern "C" fn count_global_producer(_input: *mut c_void) -> *mut c_void {
    let ctx: *mut count_global_ctx = &raw mut count_global_ctx;

    loop {
        atomic_inc(&raw mut (*ctx).hits.value);
    }
}

unsafe extern "C" fn count_global_measure(res: *mut bench_res) {
    let ctx: *mut count_global_ctx = &raw mut count_global_ctx;

    (*res).hits = atomic_swap(&raw mut (*ctx).hits.value, 0);
}

/* COUNT-local benchmark */

#[repr(C)]
struct count_local_ctx {
    hits: *mut counter,
}

static mut count_local_ctx: count_local_ctx = count_local_ctx {
    hits: ptr::null_mut(),
};

unsafe extern "C" fn count_local_setup() {
    let ctx: *mut count_local_ctx = &raw mut count_local_ctx;

    (*ctx).hits = calloc(env.producer_cnt as usize, core::mem::size_of::<counter>()) as *mut counter;
    if (*ctx).hits.is_null() {
        exit(1);
    }
}

unsafe extern "C" fn count_local_producer(input: *mut c_void) -> *mut c_void {
    let ctx: *mut count_local_ctx = &raw mut count_local_ctx;
    let idx: i32 = input as isize as i64 as i32;

    loop {
        atomic_inc(&raw mut (*(*ctx).hits.add(idx as usize)).value);
    }
}

unsafe extern "C" fn count_local_measure(res: *mut bench_res) {
    let ctx: *mut count_local_ctx = &raw mut count_local_ctx;
    let mut i: i32;

    i = 0;
    while i < env.producer_cnt {
        (*res).hits += atomic_swap(&raw mut (*(*ctx).hits.add(i as usize)).value, 0);
        i += 1;
    }
}

pub static bench_count_global: bench = bench {
    name: c"count-global".as_ptr(),
    producer_thread: Some(count_global_producer),
    measure: Some(count_global_measure),
    report_progress: Some(hits_drops_report_progress),
    report_final: Some(hits_drops_report_final),
    ..unsafe { core::mem::zeroed() }
};

pub static bench_count_local: bench = bench {
    name: c"count-local".as_ptr(),
    setup: Some(count_local_setup),
    producer_thread: Some(count_local_producer),
    measure: Some(count_local_measure),
    report_progress: Some(hits_drops_report_progress),
    report_final: Some(hits_drops_report_final),
    ..unsafe { core::mem::zeroed() }
};
