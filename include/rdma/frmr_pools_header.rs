/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
 *
 * Copyright (c) 2025, NVIDIA CORPORATION & AFFILIATES. All rights reserved.
 */

// Translated from frmr_pools.h.
// Dependencies supplied by the surrounding kernel/Rust translation are
// intentionally referenced here rather than implemented in this header.

use core::ffi::c_int;

#[repr(C)]
pub struct ib_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ib_mr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ib_frmr_key {
    pub vendor_key: u64,
    /* A pool with non-zero kernel_vendor_key is a kernel-only pool. */
    pub kernel_vendor_key: u64,
    pub num_dma_blocks: usize,
    pub access_flags: c_int,
    // C: u8 ats:1;
    pub ats: u8,
}

#[repr(C)]
pub struct ib_frmr_pool_ops {
    pub create_frmrs: Option<
        unsafe extern "C" fn(
            device: *mut ib_device,
            key: *mut ib_frmr_key,
            handles: *mut u32,
            count: u32,
        ) -> c_int,
    >,
    pub destroy_frmrs: Option<
        unsafe extern "C" fn(
            device: *mut ib_device,
            handles: *mut u32,
            count: u32,
        ),
    >,
    pub build_key: Option<
        unsafe extern "C" fn(
            device: *mut ib_device,
            input: *const ib_frmr_key,
            output: *mut ib_frmr_key,
        ) -> c_int,
    >,
}

unsafe extern "C" {
    pub fn ib_frmr_pools_init(
        device: *mut ib_device,
        pool_ops: *const ib_frmr_pool_ops,
    ) -> c_int;
    pub fn ib_frmr_pools_cleanup(device: *mut ib_device);
    pub fn ib_frmr_pool_pop(device: *mut ib_device, mr: *mut ib_mr) -> c_int;
    pub fn ib_frmr_pool_push(device: *mut ib_device, mr: *mut ib_mr);
    pub fn ib_frmr_pool_drop(mr: *mut ib_mr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
