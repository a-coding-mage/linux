/* SPDX-License-Identifier: GPL-2.0-only */

/* Copyright (c) 2019-2020, The Linux Foundation. All rights reserved. */
/* Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries. */

use core::ffi::c_void;

#[repr(C)]
pub struct mhi_controller {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

extern "C" {
    pub fn qaic_mhi_register_controller(
        pci_dev: *mut pci_dev,
        mhi_bar: *mut c_void,
        mhi_irq: i32,
        shared_msi: bool,
        family: i32,
    ) -> *mut mhi_controller;

    pub fn qaic_mhi_free_controller(mhi_cntrl: *mut mhi_controller, link_up: bool);
    pub fn qaic_mhi_start_reset(mhi_cntrl: *mut mhi_controller);
    pub fn qaic_mhi_reset_done(mhi_cntrl: *mut mhi_controller);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
