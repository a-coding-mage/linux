/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2023 Intel Corporation */

// Dependency supplied by `adf_accel_devices.h` in the C source.

unsafe extern "C" {
    pub fn adf_gen4_dev_config(accel_dev: *mut adf_accel_dev) -> core::ffi::c_int;
    pub fn adf_gen4_cfg_dev_init(accel_dev: *mut adf_accel_dev) -> core::ffi::c_int;
    pub fn adf_crypto_dev_config(accel_dev: *mut adf_accel_dev) -> core::ffi::c_int;
    pub fn adf_comp_dev_config(accel_dev: *mut adf_accel_dev) -> core::ffi::c_int;
    pub fn adf_no_dev_config(accel_dev: *mut adf_accel_dev) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
