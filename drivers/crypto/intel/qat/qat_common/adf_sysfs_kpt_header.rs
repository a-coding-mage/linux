/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2026 Intel Corporation */

// Opaque declaration corresponding to `struct adf_accel_dev`.
#[repr(C)]
pub struct adf_accel_dev {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn adf_sysfs_init_kpt(accel_dev: *mut adf_accel_dev) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
