// SPDX-License-Identifier: GPL-2.0+

/*
 * Copyright 2018 IBM Corporation.
 */

pub const CACHELINE_SIZE: u32 = 128;

pub const PERF_L1D_READ_MISS_CONFIG: u64 = (PERF_COUNT_HW_CACHE_L1D as u64)
    | ((PERF_COUNT_HW_CACHE_OP_READ as u64) << 8)
    | ((PERF_COUNT_HW_CACHE_RESULT_MISS as u64) << 16);

unsafe extern "C" {
    pub fn syscall_loop(
        p: *mut core::ffi::c_char,
        iterations: core::ffi::c_ulong,
        zero_size: core::ffi::c_ulong,
    );

    pub fn syscall_loop_uaccess(
        p: *mut core::ffi::c_char,
        iterations: core::ffi::c_ulong,
        zero_size: core::ffi::c_ulong,
    );

    pub fn set_dscr(val: core::ffi::c_ulong);
}
