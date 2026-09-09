/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2026 Intel Corporation */

// #include <linux/types.h>

#[macro_export]
macro_rules! GET_KPT_CFG_DATA {
    ($accel_dev:expr) => {
        &mut (*$accel_dev).hw_device.kpt_data
    };
}

#[macro_export]
macro_rules! GET_KPT_USER_DATA {
    ($accel_dev:expr) => {
        &mut (*$accel_dev).hw_device.kpt_data.user_input
    };
}

#[repr(C)]
pub struct adf_accel_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct adf_kpt_interface_data {
    pub enable: bool,
    pub swk_shared: bool,
    pub swk_cnt_per_fn: ::core::ffi::c_uint,
    pub swk_cnt_per_pasid: ::core::ffi::c_uint,
    pub swk_max_ttl: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct adf_kpt_hw_data {
    pub max_swk_cnt_per_fn_pasid: ::core::ffi::c_uint,
    pub max_swk_ttl: ::core::ffi::c_uint,
    pub user_input: adf_kpt_interface_data,
}

unsafe extern "C" {
    pub fn adf_enable_kpt(accel_dev: *mut adf_accel_dev) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
