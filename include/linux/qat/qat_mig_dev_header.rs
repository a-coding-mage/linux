/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2024 Intel Corporation */

use core::ffi::c_void;

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct qat_mig_dev {
    pub parent_accel_dev: *mut c_void,
    pub state: *mut u8,
    pub setup_size: u32,
    pub remote_setup_size: u32,
    pub state_size: u32,
    pub vf_id: i32,
}

extern "C" {
    pub fn qat_vfmig_create(pdev: *mut pci_dev, vf_id: i32) -> *mut qat_mig_dev;
    pub fn qat_vfmig_init(mdev: *mut qat_mig_dev) -> i32;
    pub fn qat_vfmig_cleanup(mdev: *mut qat_mig_dev);
    pub fn qat_vfmig_reset(mdev: *mut qat_mig_dev);
    pub fn qat_vfmig_open(mdev: *mut qat_mig_dev) -> i32;
    pub fn qat_vfmig_close(mdev: *mut qat_mig_dev);
    pub fn qat_vfmig_suspend(mdev: *mut qat_mig_dev) -> i32;
    pub fn qat_vfmig_resume(mdev: *mut qat_mig_dev) -> i32;
    pub fn qat_vfmig_save_state(mdev: *mut qat_mig_dev) -> i32;
    pub fn qat_vfmig_save_setup(mdev: *mut qat_mig_dev) -> i32;
    pub fn qat_vfmig_load_state(mdev: *mut qat_mig_dev) -> i32;
    pub fn qat_vfmig_load_setup(mdev: *mut qat_mig_dev, size: i32) -> i32;
    pub fn qat_vfmig_destroy(mdev: *mut qat_mig_dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
