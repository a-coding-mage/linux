// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependency intent: declarations from "dml2_external_lib_deps.h" are supplied
// by the surrounding translation unit.

pub extern "C" {
    pub static DML2_FRL_CHK_TB_BORROWED_MAX: ::core::ffi::c_int;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum lib_frl_cap_check_pixel_encoding {
    LIB_FRL_CAP_CHECK_PIXEL_ENCODING_444,
    LIB_FRL_CAP_CHECK_PIXEL_ENCODING_422,
    LIB_FRL_CAP_CHECK_PIXEL_ENCODING_420,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum lib_frl_cap_check_borrow_mode {
    LIB_FRL_CAP_CHECK_BORROW_MODE_NONE,
    LIB_FRL_CAP_CHECK_BORROW_MODE_FROM_ACTIVE,
    LIB_FRL_CAP_CHECK_BORROW_MODE_FROM_BLANK,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum lib_frl_cap_check_status {
    LIB_FRL_CAP_CHECK_OK = 0,
    LIB_FRL_CAP_CHECK_ERROR_AUDIO_BW = -1,
    LIB_FRL_CAP_CHECK_ERROR_BORROW = -2,
    LIB_FRL_CAP_CHECK_ERROR_MAX_BORROW = -3,
    LIB_FRL_CAP_CHECK_ERROR_MARGIN = -4,
    LIB_FRL_CAP_CHECK_ERROR_UNSUPPORTED_AUDIO = -1000,
}

#[repr(C)]
pub struct lib_frl_cap_check_intermediates {
    pub c_frl_sb: ::core::ffi::c_int,
    pub overhead_sb: f64,
    pub overhead_rs: f64,
    pub overhead_map: f64,
    pub overhead_min: f64,
    pub overhead_max: f64,
    pub f_pixel_clock_max: f64,
    pub t_line: f64,
    pub r_bit_min: f64,
    pub r_frl_char_min: f64,
    pub c_frl_line: f64,
    pub ap: f64,
    pub r_ap: f64,
    pub avg_audio_packets_line: f64,
    pub audio_packets_line: ::core::ffi::c_int,
    pub blank_audio_min: ::core::ffi::c_int,
}

#[repr(C)]
pub struct lib_frl_cap_check_params {
    pub lanes: ::core::ffi::c_int,
    pub f_pixel_clock_nominal: f64, // Pixel Clock rate (Hz)
    pub r_bit_nominal: f64, // FRL bitrate (bps)
    pub audio_packet_type: ::core::ffi::c_int,
    pub f_audio: f64, // Audio rate (Hz)
    pub h_active: ::core::ffi::c_int, // Active pixels per line
    pub h_blank: ::core::ffi::c_int, // Blanking pixels per line
    pub bpc: ::core::ffi::c_int, // Bits per component

    pub pixel_encoding: lib_frl_cap_check_pixel_encoding,

    pub compressed: bool,
    pub bypass_hc_target_calc: bool,

    // DSC parameters
    pub slices: ::core::ffi::c_int,
    pub slice_width: ::core::ffi::c_int,
    pub bpp_target: f64,

    pub layout: ::core::ffi::c_int, // not supported
    pub acat: ::core::ffi::c_int, // not supported

    // outputs
    pub audio_packets_line: ::core::ffi::c_int,

    // inputs or outputs
    pub hc_active_target: ::core::ffi::c_int,
    pub hc_blank_target: ::core::ffi::c_int,

    pub borrow_mode: lib_frl_cap_check_borrow_mode,
}

extern "C" {
    pub fn frl_cap_check(params: *mut lib_frl_cap_check_params) -> lib_frl_cap_check_status;
    pub fn frl_cap_check_intermediates(
        params: *mut lib_frl_cap_check_params,
        inter: *mut lib_frl_cap_check_intermediates,
    ) -> lib_frl_cap_check_status;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
