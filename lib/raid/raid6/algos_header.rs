/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2003 H. Peter Anvin - All Rights Reserved
 */

/* Dependency intent: linux/init.h and linux/raid/pq_tables.h. */

use core::ffi::{c_char, c_int, c_uint, c_void};

/* Routine choices */
#[repr(C)]
pub struct raid6_calls {
    pub name: *const c_char,
    pub gen_syndrome: Option<unsafe extern "C" fn(disks: c_int, bytes: usize, ptrs: *mut *mut c_void)>,
    pub xor_syndrome: Option<unsafe extern "C" fn(
        disks: c_int,
        start: c_int,
        stop: c_int,
        bytes: usize,
        ptrs: *mut *mut c_void,
    )>,
}

#[repr(C)]
pub struct raid6_recov_calls {
    pub name: *const c_char,
    pub data2: Option<unsafe extern "C" fn(
        disks: c_int,
        bytes: usize,
        faila: c_int,
        failb: c_int,
        ptrs: *mut *mut c_void,
    )>,
    pub datap: Option<unsafe extern "C" fn(
        disks: c_int,
        bytes: usize,
        faila: c_int,
        ptrs: *mut *mut c_void,
    )>,
}

/* The C __init annotation has no direct Rust equivalent. */
unsafe extern "C" {
    pub fn raid6_algo_add(algo: *const raid6_calls);
    pub fn raid6_algo_add_default();
    pub fn raid6_recov_algo_add(algo: *const raid6_recov_calls);

    /* for the kunit test */
    pub fn raid6_algo_find(idx: c_uint) -> *const raid6_calls;
    pub fn raid6_recov_algo_find(idx: c_uint) -> *const raid6_recov_calls;

    /* generic implementations */
    pub static raid6_intx1: raid6_calls;
    pub static raid6_intx2: raid6_calls;
    pub static raid6_intx4: raid6_calls;
    pub static raid6_intx8: raid6_calls;
    pub static raid6_recov_intx1: raid6_recov_calls;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
