/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2022, Advanced Micro Devices, Inc.
 */

// Translated from amd_xdma.h.
// External kernel dependency: `platform_device` is supplied by the platform
// device subsystem. `u32` corresponds to the kernel's 32-bit unsigned type.

use core::ffi::c_int;

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

extern "C" {
    pub fn xdma_enable_user_irq(pdev: *mut platform_device, irq_num: u32) -> c_int;
    pub fn xdma_disable_user_irq(pdev: *mut platform_device, irq_num: u32);
    pub fn xdma_get_user_irq(pdev: *mut platform_device, user_irq_index: u32) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
