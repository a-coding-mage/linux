/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2010-2015, 2018-2019 The Linux Foundation. All rights reserved.
 * Copyright (C) 2015 Linaro Ltd.
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

use core::ffi::c_void;

// C dependencies: linux/device.h, linux/err.h, linux/io.h, linux/types.h

pub type PhysAddr = u64;
pub type DmaAddr = u64;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource_table {
    _private: [u8; 0],
}

extern "C" {
    pub fn ioremap_wc(addr: PhysAddr, size: usize) -> *mut c_void;
    pub fn dev_err(dev: *mut device, fmt: *const u8, ...);
}

#[repr(C)]
pub struct qcom_pas_context {
    pub dev: *mut device,
    pub pas_id: u32,
    pub mem_phys: PhysAddr,
    pub mem_size: usize,
    pub ptr: *mut c_void,
    pub phys: DmaAddr,
    pub size: isize,
    pub use_tzmem: bool,
}

#[inline]
pub unsafe fn qcom_pas_ctx_map(ctx: *mut qcom_pas_context) -> *mut c_void {
    let ptr = ioremap_wc((*ctx).mem_phys, (*ctx).mem_size);

    if ptr.is_null() {
        dev_err(
            (*ctx).dev,
            b"unable to map memory region: %pa+%zx\n\0".as_ptr(),
            &(*ctx).mem_phys,
            (*ctx).mem_size,
        );
    }
    ptr
}

extern "C" {
    pub fn qcom_pas_is_available() -> bool;
    pub fn devm_qcom_pas_context_alloc(
        dev: *mut device,
        pas_id: u32,
        mem_phys: PhysAddr,
        mem_size: usize,
    ) -> *mut qcom_pas_context;
    pub fn qcom_pas_init_image(
        pas_id: u32,
        metadata: *const c_void,
        size: usize,
        ctx: *mut qcom_pas_context,
    ) -> i32;
    pub fn qcom_pas_get_rsc_table(
        ctx: *mut qcom_pas_context,
        input_rt: *mut c_void,
        input_rt_size: usize,
        output_rt_size: *mut usize,
    ) -> *mut resource_table;
    pub fn qcom_pas_mem_setup(pas_id: u32, addr: PhysAddr, size: PhysAddr) -> i32;
    pub fn qcom_pas_auth_and_reset(pas_id: u32) -> i32;
    pub fn qcom_pas_prepare_and_auth_reset(ctx: *mut qcom_pas_context) -> i32;
    pub fn qcom_pas_set_remote_state(state: u32, pas_id: u32) -> i32;
    pub fn qcom_pas_shutdown(pas_id: u32) -> i32;
    pub fn qcom_pas_supported(pas_id: u32) -> bool;
    pub fn qcom_pas_metadata_release(ctx: *mut qcom_pas_context);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
