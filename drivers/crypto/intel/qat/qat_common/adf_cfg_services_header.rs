/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2023 Intel Corporation */

/* Dependency supplied by adf_cfg_strings.h. */

use core::ffi::c_char;

#[repr(C)]
pub struct adf_accel_dev {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum adf_base_services {
    SVC_ASYM = 0,
    SVC_SYM,
    SVC_DC,
    SVC_DECOMP,
    SVC_BASE_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum adf_extended_services {
    SVC_DCC = adf_base_services::SVC_BASE_COUNT as isize,
    SVC_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum adf_composed_services {
    SVC_SYM_ASYM = adf_extended_services::SVC_COUNT as isize,
    SVC_SYM_DC,
    SVC_ASYM_DC,
}

pub const ADF_ONE_SERVICE: u32 = 1;
pub const ADF_TWO_SERVICES: u32 = 2;
pub const ADF_THREE_SERVICES: u32 = 3;

pub const MAX_NUM_CONCURR_SVC: u32 = ADF_THREE_SERVICES;

extern "C" {
    pub fn adf_parse_service_string(
        accel_dev: *mut adf_accel_dev,
        input: *const c_char,
        out: *mut c_char,
        out_len: usize,
    ) -> i32;

    pub fn adf_get_service_enabled(accel_dev: *mut adf_accel_dev) -> i32;

    pub fn adf_get_service_mask(accel_dev: *mut adf_accel_dev, mask: *mut usize) -> i32;

    pub fn adf_srv_to_cfg_svc_type(svc: adf_base_services) -> adf_cfg_service_type;

    pub fn adf_is_service_enabled(
        accel_dev: *mut adf_accel_dev,
        svc: adf_base_services,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
