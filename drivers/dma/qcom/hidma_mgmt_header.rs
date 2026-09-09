/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Qualcomm Technologies HIDMA Management common header
 *
 * Copyright (c) 2015, The Linux Foundation. All rights reserved.
 */

/* Opaque types supplied by dependent headers. */
pub struct kobject;
pub struct platform_device;

#[repr(C)]
pub struct hidma_mgmt_dev {
    pub hw_version_major: u8,
    pub hw_version_minor: u8,

    pub max_wr_xactions: u32,
    pub max_rd_xactions: u32,
    pub max_write_request: u32,
    pub max_read_request: u32,
    pub dma_channels: u32,
    pub chreset_timeout_cycles: u32,
    pub hw_version: u32,
    pub priority: *mut u32,
    pub weight: *mut u32,

    /* Hardware device constants */
    /* __iomem */
    pub virtaddr: *mut core::ffi::c_void,
    pub addrsize: resource_size_t,

    pub chroots: *mut *mut kobject,
    pub pdev: *mut platform_device,
}

unsafe extern "C" {
    pub fn hidma_mgmt_init_sys(dev: *mut hidma_mgmt_dev) -> i32;
    pub fn hidma_mgmt_setup(mgmtdev: *mut hidma_mgmt_dev) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
