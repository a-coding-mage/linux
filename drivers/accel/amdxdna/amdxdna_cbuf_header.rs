/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2026, Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the corresponding C headers:
// #include "amdxdna_pci_drv.h"
// #include <drm/drm_device.h>
// #include <linux/dma-buf.h>

use core::ffi::c_int;

pub enum amdxdna_dev {}
pub enum drm_device {}
pub enum dma_buf {}

extern "C" {
    pub fn amdxdna_use_carveout(xdna: *mut amdxdna_dev) -> bool;
    pub fn amdxdna_carveout_init(
        xdna: *mut amdxdna_dev,
        carveout_addr: u64,
        carveout_size: u64,
    ) -> c_int;
    pub fn amdxdna_carveout_fini(xdna: *mut amdxdna_dev);
    pub fn amdxdna_get_carveout_conf(
        xdna: *mut amdxdna_dev,
        addr: *mut u64,
        size: *mut u64,
    );
    pub fn amdxdna_get_cbuf(
        dev: *mut drm_device,
        size: usize,
        alignment: u64,
    ) -> *mut dma_buf;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
