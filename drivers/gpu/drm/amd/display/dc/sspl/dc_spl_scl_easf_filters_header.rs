/* SPDX-License-Identifier: MIT */

/* Copyright 2024 Advanced Micro Devices, Inc. */

/* Translated from dc_spl_scl_easf_filters.h. */
/* The C SPL_NAMESPACE(...) macro supplies the configured namespace. */
/* The declarations below use their underlying names. */

#[repr(C)]
pub struct scale_ratio_to_reg_value_lookup {
    pub numer: ::core::ffi::c_int,
    pub denom: ::core::ffi::c_int,
    pub reg_value: u32,
}

unsafe extern "C" {
    pub fn spl_set_filters_data(
        dscl_prog_data: *mut dscl_prog_data,
        data: *const spl_scaler_data,
        enable_easf_v: bool,
        enable_easf_h: bool,
    );

    pub fn spl_get_v_bf3_mode(ratio: spl_fixed31_32) -> u32;
    pub fn spl_get_h_bf3_mode(ratio: spl_fixed31_32) -> u32;
    pub fn spl_get_reducer_gain6(taps: ::core::ffi::c_int, ratio: spl_fixed31_32) -> u32;
    pub fn spl_get_reducer_gain4(taps: ::core::ffi::c_int, ratio: spl_fixed31_32) -> u32;
    pub fn spl_get_gainRing6(taps: ::core::ffi::c_int, ratio: spl_fixed31_32) -> u32;
    pub fn spl_get_gainRing4(taps: ::core::ffi::c_int, ratio: spl_fixed31_32) -> u32;
    pub fn spl_get_3tap_dntilt_uptilt_offset(
        taps: ::core::ffi::c_int,
        ratio: spl_fixed31_32,
    ) -> u32;
    pub fn spl_get_3tap_uptilt_maxval(taps: ::core::ffi::c_int, ratio: spl_fixed31_32) -> u32;
    pub fn spl_get_3tap_dntilt_slope(taps: ::core::ffi::c_int, ratio: spl_fixed31_32) -> u32;
    pub fn spl_get_3tap_uptilt1_slope(taps: ::core::ffi::c_int, ratio: spl_fixed31_32) -> u32;
    pub fn spl_get_3tap_uptilt2_slope(taps: ::core::ffi::c_int, ratio: spl_fixed31_32) -> u32;
    pub fn spl_get_3tap_uptilt2_offset(taps: ::core::ffi::c_int, ratio: spl_fixed31_32) -> u32;

    /* public API */
    pub fn spl_dscl_get_easf_filter_coeffs_64p(
        taps: ::core::ffi::c_int,
        ratio: spl_fixed31_32,
    ) -> *const u16;
    pub fn spl_dscl_get_easf_filter_coeffs_64p_s1_10(
        taps: ::core::ffi::c_int,
        ratio: spl_fixed31_32,
    ) -> *const u16;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
