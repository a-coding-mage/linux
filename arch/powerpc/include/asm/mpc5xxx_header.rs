/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2007 Freescale Semiconductor, Inc. All rights reserved.
 *
 * Author: John Rigby, <jrigby@freescale.com>, Friday Apr 13 2007
 *
 * Description:
 * MPC5xxx Prototypes and definitions
 */

// Dependency supplied by the Linux property and device interfaces.

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

extern "C" {
    pub fn mpc5xxx_fwnode_get_bus_frequency(fwnode: *mut fwnode_handle) -> ::core::ffi::c_ulong;

    // Supplied by the device interface included by the original header.
    pub fn dev_fwnode(dev: *const device) -> *mut fwnode_handle;
}

pub unsafe fn mpc5xxx_get_bus_frequency(dev: *const device) -> ::core::ffi::c_ulong {
    mpc5xxx_fwnode_get_bus_frequency(dev_fwnode(dev))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
