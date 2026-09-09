/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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

// Dependency supplied by the corresponding link-service translation.

pub const TOLERANCE_AUDIO_CLOCK: u32 = 1000;

extern "C" {
    pub fn link_validate_mode_timing(
        stream: *const dc_stream_state,
        link: *mut dc_link,
        timing: *const dc_crtc_timing,
    ) -> dc_status;

    pub fn link_validate_dp_tunnel_bandwidth(
        dc: *const dc,
        new_ctx: *const dc_state,
    ) -> dc_status;

    pub fn frl_validate_mode_timing(
        link: *mut dc_link,
        timing: *const dc_crtc_timing,
        frl_link_settings: *mut dc_hdmi_frl_link_settings,
    ) -> bool;

    pub fn dp_link_bandwidth_kbps(
        link: *const dc_link,
        link_settings: *const dc_link_settings,
    ) -> u32;

    pub fn frl_link_bandwidth_kbps(link_rate: hdmi_frl_link_rate) -> u32;

    pub fn link_timing_bandwidth_kbps(timing: *const dc_crtc_timing) -> u32;

    pub fn frl_capacity_computations_common(
        params: *mut frl_cap_chk_params_fixed31_32,
        inter: *mut frl_cap_chk_intermediates_fixed31_32,
    ) -> bool;

    pub fn frl_capacity_computations_uncompressed_video(
        params: *mut frl_cap_chk_params_fixed31_32,
        inter: *mut frl_cap_chk_intermediates_fixed31_32,
    ) -> bool;

    pub fn dp_required_hblank_size_bytes(
        link: *const dc_link,
        audio_params: *mut dp_audio_bandwidth_params,
    ) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
