/*
 * Copyright 2019 Advanced Micro Devices, Inc.
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
 * Author: AMD
 */

/* Put it here temporarily until Linux has the new addresses officially defined. */
/* DP Extended DSC Capabilities */
pub const DP_DSC_BRANCH_OVERALL_THROUGHPUT_0: u32 = 0x0a0; /* DP 1.4a SCR */
pub const DP_DSC_BRANCH_OVERALL_THROUGHPUT_1: u32 = 0x0a1;
pub const DP_DSC_BRANCH_MAX_LINE_WIDTH: u32 = 0x0a2;

#[repr(C)]
pub struct dc_dsc_bw_range {
    pub min_kbps: u32, /* Bandwidth if min_target_bpp_x16 is used */
    pub min_target_bpp_x16: u32,
    pub max_kbps: u32, /* Bandwidth if max_target_bpp_x16 is used */
    pub max_target_bpp_x16: u32,
    pub stream_kbps: u32, /* Uncompressed stream bandwidth */
}

#[repr(C)]
pub struct display_stream_compressor {
    pub funcs: *const dsc_funcs,
    pub ctx: *mut dc_context,
    pub inst: i32,
}

#[repr(C)]
pub struct dc_dsc_policy {
    pub use_min_slices_h: bool,
    pub max_slices_h: i32, // Maximum available if 0
    pub min_slice_height: i32, // Must not be less than 8
    pub max_target_bpp: u32,
    pub min_target_bpp: u32,
    pub enable_dsc_when_not_needed: bool,
    pub ycbcr422_simple: bool,
}

#[repr(C)]
pub struct dc_dsc_config_options {
    pub dsc_min_slice_height_override: u32,
    pub max_target_bpp_limit_override_x16: u32,
    pub slice_height_granularity: u32,
    pub dsc_force_odm_hslice_override: u32,
    pub force_dsc_when_not_needed: bool,
}

#[repr(C)]
pub struct dc_dsc_primary_bpp {
    pub vic: u32,
    pub target_bpp: u32,
}

extern "C" {
    pub fn dc_dsc_parse_dsc_dpcd(
        dc: *const dc,
        dpcd_dsc_basic_data: *const u8,
        dpcd_dsc_ext_data: *const u8,
        dsc_sink_caps: *mut dsc_dec_dpcd_caps,
    ) -> bool;

    pub fn dc_dsc_parse_dsc_edid(
        dc: *const dc,
        edid_caps: *const dc_edid_caps,
        dsc_sink_caps: *mut dsc_dec_dpcd_caps,
    ) -> bool;

    pub fn dc_dsc_compute_bandwidth_range(
        dsc: *const display_stream_compressor,
        dsc_min_slice_height_override: u32,
        min_bpp_x16: u32,
        max_bpp_x16: u32,
        dsc_sink_caps: *const dsc_dec_dpcd_caps,
        timing: *const dc_crtc_timing,
        link_encoding: dc_link_encoding_format,
        range: *mut dc_dsc_bw_range,
    ) -> bool;

    pub fn dc_dsc_compute_config(
        dsc: *const display_stream_compressor,
        dsc_sink_caps: *const dsc_dec_dpcd_caps,
        options: *const dc_dsc_config_options,
        target_bandwidth_kbps: u32,
        timing: *const dc_crtc_timing,
        link_encoding: dc_link_encoding_format,
        dsc_cfg: *mut dc_dsc_config,
    ) -> bool;

    pub fn dc_dsc_stream_bandwidth_in_kbps(
        timing: *const dc_crtc_timing,
        bpp_x16: u32,
        num_slices_h: u32,
        is_dp: bool,
    ) -> u32;

    pub fn dc_dsc_stream_bandwidth_overhead_in_kbps(
        timing: *const dc_crtc_timing,
        num_slices_h: u32,
        is_dp: bool,
    ) -> u32;

    pub fn dc_dsc_dump_decoder_caps(
        dsc: *const display_stream_compressor,
        dsc_sink_caps: *const dsc_dec_dpcd_caps,
    );

    pub fn dc_dsc_dump_encoder_caps(
        dsc: *const display_stream_compressor,
        timing: *const dc_crtc_timing,
    );

    /* TODO - Hardware/specs limitation should be owned by dc dsc and returned to DM,
     * and DM can choose to OVERRIDE the limitation on CASE BY CASE basis.
     * Hardware/specs limitation should not be writable by DM.
     * It should be decoupled from DM specific policy and named differently.
     */
    pub fn dc_dsc_get_policy_for_timing(
        timing: *const dc_crtc_timing,
        max_target_bpp_limit_override_x16: u32,
        policy: *mut dc_dsc_policy,
        link_encoding: dc_link_encoding_format,
    );

    pub fn dc_dsc_policy_set_max_target_bpp_limit(limit: u32);

    pub fn dc_dsc_policy_set_enable_dsc_when_not_needed(enable: bool);

    pub fn dc_dsc_policy_set_disable_dsc_stream_overhead(disable: bool);

    pub fn dc_dsc_get_default_config_option(dc: *const dc, options: *mut dc_dsc_config_options);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
