/* SPDX-License-Identifier: MIT */
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

// Dependency supplied externally by the translated source tree.

pub const TB_BORROWED_MAX: i32 = 400;
pub const C_FRL_CB: i32 = 510;
pub const TOLERANCE_FRL_BIT: i32 = 300; // ppm
pub const ACR_RATE_MAX: i32 = 1500;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hdmi_frl_pixel_encoding {
    HDMI_FRL_PIXEL_ENCODING_444,
    HDMI_FRL_PIXEL_ENCODING_422,
    HDMI_FRL_PIXEL_ENCODING_420,
}

#[repr(i32)]
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
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum frl_borrow_mode {
    FRL_BORROW_MODE_NONE,
    FRL_BORROW_MODE_FROM_ACTIVE,
    FRL_BORROW_MODE_FROM_BLANK,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum frl_link_rate {
    FRL_LINK_RATE_DISABLE = 0,
    FRL_LINK_RATE_3GBPS,
    FRL_LINK_RATE_6GBPS,
    FRL_LINK_RATE_6GBPS_4LANE,
    FRL_LINK_RATE_8GBPS,
    FRL_LINK_RATE_10GBPS,
    FRL_LINK_RATE_12GBPS,
    FRL_LINK_RATE_16GBPS,
    FRL_LINK_RATE_20GBPS,
    FRL_LINK_RATE_24GBPS,
}

#[repr(C)]
pub struct frl_dml_borrow_params {
    pub audio_packets_line: i32,
    pub hc_active_target: i32,
    pub hc_blank_target: i32,
    pub borrow_mode: frl_borrow_mode,
}

#[repr(C)]
pub struct frl_primary_format {
    pub vic: u32,
    pub frl_rate: u32,
    pub frl_lanes: u32,
    pub hc_active: u32,
    pub hc_blank: u32,
}

#[repr(C)]
pub struct frl_cap_chk_intermediates {
    pub c_frl_sb: i32,
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
    pub audio_packets_line: i32,
    pub blank_audio_min: i32,
}

#[repr(C)]
pub struct frl_cap_chk_params {
    pub lanes: i32,
    pub f_pixel_clock_nominal: f64,
    pub r_bit_nominal: f64,
    pub audio_packet_type: i32,
    pub f_audio: f64,
    pub h_active: i32,
    pub h_blank: i32,
    pub bpc: i32,
    pub vic: i32,
    pub pixel_encoding: hdmi_frl_pixel_encoding,
    pub compressed: bool,
    pub bypass_hc_target_calc: bool,
    pub allow_all_bpp: bool,
    pub slices: i32,
    pub slice_width: i32,
    pub bpp_target: f64,
    pub is_ovt: bool,
    pub layout: i32,
    pub acat: i32,
    pub borrow_params: frl_dml_borrow_params,
    pub average_tribyte_rate: i32,
}

extern "C" {
    pub fn dml1_frl_cap_chk(params: *mut frl_cap_chk_params) -> frl_cap_chk_result;
    pub fn dml1_frl_cap_chk_inter(
        params: *mut frl_cap_chk_params,
        inter: *mut frl_cap_chk_intermediates,
    ) -> frl_cap_chk_result;
    pub fn dml1_frl_cap_chk_common(
        inter: *mut frl_cap_chk_intermediates,
        params: *mut frl_cap_chk_params,
    ) -> frl_cap_chk_result;
    pub fn dml1_frl_cap_chk_uncompressed(
        params: *mut frl_cap_chk_params,
        inter: *mut frl_cap_chk_intermediates,
    ) -> frl_cap_chk_result;
    pub fn dml1_frl_cap_chk_compressed(
        params: *mut frl_cap_chk_params,
        inter: *mut frl_cap_chk_intermediates,
    ) -> frl_cap_chk_result;

    pub fn frl_modified_pix_clock_for_dsc_padding(
        hc_active_target: i32,
        hc_blank_target: i32,
        frl_num_lanes: u8,
        pix_clk_100hz: u32,
        frl_link_rate: i32,
        h_addressable: u32,
        h_border_left: u32,
        h_border_right: u32,
        h_total: u32,
        h_addressable_otg: u32,
        pix_clk_100hz_otg: *mut u32,
        h_total_otg: *mut u32,
    );

    pub fn frl_modify_borrow_mode_for_dsc_padding(
        pix_clk_100hz: u32,
        h_active: u32,
        h_active_padded: u32,
        h_blank: u32,
        h_blank_padded: u32,
        hc_active: i32,
        hc_blank: i32,
        frl_num_lanes: u8,
        frl_link_rate: i32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
