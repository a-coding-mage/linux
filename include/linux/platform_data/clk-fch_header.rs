/* SPDX-License-Identifier: MIT */
/*
 * clock framework for AMD misc clocks
 *
 * Copyright 2018 Advanced Micro Devices, Inc.
 */

// Dependency intent: `__iomem` is a Linux annotation for memory-mapped I/O.

#[repr(C)]
pub struct fch_clk_data {
    pub base: *mut core::ffi::c_void,
    pub name: *mut core::ffi::c_char,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
