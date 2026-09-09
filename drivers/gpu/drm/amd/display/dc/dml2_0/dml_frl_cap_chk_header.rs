/*
 * Copyright 2022 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 *
 */

// Dependency supplied by the surrounding translation unit: os_types.h

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum enum_pixel_encoding {
    PIXEL_ENCODING_444,
    PIXEL_ENCODING_422,
    PIXEL_ENCODING_420,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum enum_borrow_mode {
    BORROW_MODE_NONE,
    BORROW_MODE_FROM_ACTIVE,
    BORROW_MODE_FROM_BLANK,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum frl_cap_chk_result {
    FRL_CAP_CHK_OK = 0,

    FRL_CAP_CHK_ERROR_AUDIO_BW = -1,
    FRL_CAP_CHK_ERROR_BORROW = -2,
    FRL_CAP_CHK_ERROR_MAX_BORROW = -3,
    FRL_CAP_CHK_ERROR_MARGIN = -4,

    FRL_CAP_CHK_ERROR_UNSUPPORTED_AUDIO = -1000,
}

#[repr(C)]
pub struct frl_cap_chk_intermediates {
    pub c_frl_sb: ::std::os::raw::c_int,
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
    pub audio_packets_line: ::std::os::raw::c_int,
    pub blank_audio_min: ::std::os::raw::c_int,
}

#[repr(C)]
pub struct frl_cap_chk_params {
    pub lanes: ::std::os::raw::c_int,
    pub f_pixel_clock_nominal: f64, /* Pixel Clock rate (Hz) */
    pub r_bit_nominal: f64, /* FRL bitrate (bps) */
    pub audio_packet_type: ::std::os::raw::c_int,
    pub f_audio: f64, /* Audio rate (Hz) */
    pub h_active: ::std::os::raw::c_int, /* Active pixels per line */
    pub h_blank: ::std::os::raw::c_int, /* Blanking pixels per line */
    pub bpc: ::std::os::raw::c_int, /* Bits per component */

    pub pixel_encoding: enum_pixel_encoding,

    pub compressed: bool,
    pub bypass_hc_target_calc: bool,

    /* DSC parameters */
    pub slices: ::std::os::raw::c_int,
    pub slice_width: ::std::os::raw::c_int,
    pub bpp_target: f64,

    pub layout: ::std::os::raw::c_int, /* not supported */
    pub acat: ::std::os::raw::c_int, /* not supported */

    /* outputs */
    pub audio_packets_line: ::std::os::raw::c_int,

    /* inputs or outputs */
    pub hc_active_target: ::std::os::raw::c_int,
    pub hc_blank_target: ::std::os::raw::c_int,

    pub borrow_mode: enum_borrow_mode,
}

extern "C" {
    pub fn frl_cap_chk(params: *mut frl_cap_chk_params) -> frl_cap_chk_result;
    pub fn frl_cap_chk_inter(
        params: *mut frl_cap_chk_params,
        inter: *mut frl_cap_chk_intermediates,
    ) -> frl_cap_chk_result;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
