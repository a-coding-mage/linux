// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Translated from dc_spl_isharp_filters.h.
// Dependency declarations supplied by dc_spl_types.h remain external.

pub const NUM_SHARPNESS_ADJ_LEVELS: usize = 6;

#[repr(C)]
pub struct scale_ratio_to_sharpness_level_adj {
    pub ratio_numer: ::core::ffi::c_uint,
    pub ratio_denom: ::core::ffi::c_uint,
    pub level_down_adj: ::core::ffi::c_uint, // adjust sharpness level down
}

#[repr(C)]
pub struct isharp_1D_lut_pregen {
    pub sharpness_numer: ::core::ffi::c_uint,
    pub sharpness_denom: ::core::ffi::c_uint,
    pub value: [u32; ISHARP_LUT_TABLE_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum system_setup {
    SDR_NL = 0,
    SDR_L,
    HDR_NL,
    HDR_L,
    NUM_SHARPNESS_SETUPS,
}

extern "C" {
    pub fn spl_set_blur_scale_data(
        dscl_prog_data: *mut dscl_prog_data,
        data: *const spl_scaler_data,
    );

    pub fn spl_build_isharp_1dlut_from_reference_curve(
        ratio: spl_fixed31_32,
        setup: system_setup,
        sharpness: adaptive_sharpness,
        scale_to_sharpness_policy: scale_to_sharpness_policy,
    );

    pub fn spl_get_pregen_filter_isharp_1D_lut(
        setup: system_setup,
    ) -> *mut u32;

    // public API
    pub fn spl_dscl_get_blur_scale_coeffs_64p(taps: ::core::ffi::c_int) -> *const u16;
    pub fn spl_dscl_get_blur_scale_coeffs_64p_s1_10(taps: ::core::ffi::c_int) -> *const u16;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
