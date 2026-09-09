/*
 * Copyright 2015 Advanced Micro Devices, Inc.
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

/* Include basic type headers only:
 * dc_dp_types.h, signal_types.h, grph_object_id.h, fixed31_32.h
 */

/* Build-time C header guard __DC_LINK_HWSS_H__ removed in Rust. */

/* Forward declare dc core types. */
pub struct dc_link;
pub struct link_resource;
pub struct pipe_ctx;
pub struct encoder_set_dp_phy_pattern_param;
pub struct link_mst_stream_allocation_table;
pub struct audio_output;

/* Types supplied by the included headers are external dependencies:
 * dc_link_settings, signal_type, clock_source_id, dc_lane_settings,
 * fixed31_32, and LANE_COUNT_DP_MAX.
 */

#[repr(C)]
pub struct link_hwss_ext {
	/* Function pointers below may require checking for NULL if the caller
	 * considers missing implementation expected in some cases or non-critical
	 * to be investigated immediately.
	 * *********************************************************************
	 */
	pub set_hblank_min_symbol_width: Option<unsafe extern "C" fn(
		pipe_ctx: *mut pipe_ctx,
		link_settings: *const dc_link_settings,
		throttled_vcp_size: fixed31_32,
	)>,
	pub set_throttled_vcp_size: Option<unsafe extern "C" fn(
		pipe_ctx: *mut pipe_ctx,
		throttled_vcp_size: fixed31_32,
	)>,
	pub enable_dp_link_output: Option<unsafe extern "C" fn(
		link: *mut dc_link,
		link_res: *const link_resource,
		signal: signal_type,
		clock_source: clock_source_id,
		link_settings: *const dc_link_settings,
	)>,
	pub set_dp_link_test_pattern: Option<unsafe extern "C" fn(
		link: *mut dc_link,
		link_res: *const link_resource,
		tp_params: *mut encoder_set_dp_phy_pattern_param,
	)>,
	pub set_dp_lane_settings: Option<unsafe extern "C" fn(
		link: *mut dc_link,
		link_res: *const link_resource,
		link_settings: *const dc_link_settings,
		lane_settings: *const dc_lane_settings,
	)>,
	pub update_stream_allocation_table: Option<unsafe extern "C" fn(
		link: *mut dc_link,
		link_res: *const link_resource,
		table: *const link_mst_stream_allocation_table,
	)>,
}

#[repr(C)]
pub struct link_hwss {
	pub ext: link_hwss_ext,

	/* Function pointers below MUST be assigned to all types of link_hwss.
	 * *********************************************************************
	 */
	pub setup_stream_encoder: Option<unsafe extern "C" fn(pipe_ctx: *mut pipe_ctx)>,
	pub reset_stream_encoder: Option<unsafe extern "C" fn(pipe_ctx: *mut pipe_ctx)>,
	pub setup_stream_attribute: Option<unsafe extern "C" fn(pipe_ctx: *mut pipe_ctx)>,
	pub disable_link_output: Option<unsafe extern "C" fn(
		link: *mut dc_link,
		link_res: *const link_resource,
		signal: signal_type,
	)>,
	pub setup_audio_output: Option<unsafe extern "C" fn(
		pipe_ctx: *mut pipe_ctx,
		audio_output: *mut audio_output,
		audio_inst: u32,
	)>,
	pub enable_audio_packet: Option<unsafe extern "C" fn(pipe_ctx: *mut pipe_ctx)>,
	pub disable_audio_packet: Option<unsafe extern "C" fn(pipe_ctx: *mut pipe_ctx)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
