// Source-level Rust translation of display_mode_core_structs.h
#![allow(non_camel_case_types, non_snake_case, dead_code)]

/* SPDX-License-Identifier: MIT */
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


// #include "display_mode_lib_defines.h"
// #include "dml_top_display_cfg_types.h"

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_project_id {
    dml_project_invalid = 0,
    dml_project_default = 1,
    dml_project_dcn32 = 1,
    dml_project_dcn321 = 2,
    dml_project_dcn35 = 3,
    dml_project_dcn351 = 4,
    dml_project_dcn401 = 5,
    dml_project_dcn36 = 6
},
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_prefetch_modes {
    dml_prefetch_support_uclk_fclk_and_stutter_if_possible = 0,
    dml_prefetch_support_uclk_fclk_and_stutter = 1,
    dml_prefetch_support_fclk_and_stutter = 2,
    dml_prefetch_support_stutter = 3,
    dml_prefetch_support_none = 4
},
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_use_mall_for_pstate_change_mode {
    dml_use_mall_pstate_change_disable = 0,
    dml_use_mall_pstate_change_full_frame = 1,
    dml_use_mall_pstate_change_sub_viewport = 2,
    dml_use_mall_pstate_change_phantom_pipe = 3,
    dml_use_mall_pstate_change_phantom_pipe_no_data_return = 4,
    dml_use_mall_pstate_change_imall = 5
},
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_use_mall_for_static_screen_mode {
    dml_use_mall_static_screen_disable = 0,
    dml_use_mall_static_screen_enable = 1,
    dml_use_mall_static_screen_optimize = 2
},
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_output_encoder_class {
    dml_dp = 0,
    dml_edp = 1,
    dml_dp2p0 = 2,
    dml_hdmi = 3,
    dml_hdmifrl = 4,
    dml_none = 5
},
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_output_link_dp_rate {
    dml_dp_rate_na = 0,
    dml_dp_rate_hbr = 1,
    dml_dp_rate_hbr2 = 2,
    dml_dp_rate_hbr3 = 3,
    dml_dp_rate_uhbr10 = 4,
    dml_dp_rate_uhbr13p5 = 5,
    dml_dp_rate_uhbr20 = 6
},
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_output_type_and_rate__type {
    dml_output_type_unknown = 0,
    dml_output_type_dp = 1,
    dml_output_type_edp = 2,
    dml_output_type_dp2p0 = 3,
    dml_output_type_hdmi = 4,
    dml_output_type_hdmifrl = 5
},
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_output_type_and_rate__rate {
    dml_output_rate_unknown = 0,
    dml_output_rate_dp_rate_hbr = 1,
    dml_output_rate_dp_rate_hbr2 = 2,
    dml_output_rate_dp_rate_hbr3 = 3,
    dml_output_rate_dp_rate_uhbr10 = 4,
    dml_output_rate_dp_rate_uhbr13p5 = 5,
    dml_output_rate_dp_rate_uhbr20 = 6,
    dml_output_rate_hdmi_rate_3x3 = 7,
    dml_output_rate_hdmi_rate_6x3 = 8,
    dml_output_rate_hdmi_rate_6x4 = 9,
    dml_output_rate_hdmi_rate_8x4 = 10,
    dml_output_rate_hdmi_rate_10x4 = 11,
    dml_output_rate_hdmi_rate_12x4 = 12
},
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_output_format_class {
    dml_444 = 0,
    dml_s422 = 1,
    dml_n422 = 2,
    dml_420 = 3
},
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_source_format_class {
    dml_444_8 = 0,
    dml_444_16 = 1,
    dml_444_32 = 2,
    dml_444_64 = 3,
    dml_420_8 = 4,
    dml_420_10 = 5,
    dml_420_12 = 6,
    dml_422_8 = 7,
    dml_422_10 = 8,
    dml_rgbe_alpha = 9,
    dml_rgbe = 10,
    dml_mono_8 = 11,
    dml_mono_16 = 12
},
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_output_bpc_class {
    dml_out_6 = 0,
    dml_out_8 = 1,
    dml_out_10 = 2,
    dml_out_12 = 3,
    dml_out_16 = 4
},
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_output_standard_class {
    dml_std_cvt = 0,
    dml_std_cea = 1,
    dml_std_cvtr2 = 2
},
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_rotation_angle {
    dml_rotation_0 = 0,
    dml_rotation_90 = 1,
    dml_rotation_180 = 2,
    dml_rotation_270 = 3,
    dml_rotation_0m = 4,
    dml_rotation_90m = 5,
    dml_rotation_180m = 6,
    dml_rotation_270m = 7
},
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_swizzle_mode {
    dml_sw_linear = 0,
    dml_sw_256b_s = 1,
    dml_sw_256b_d = 2,
    dml_sw_256b_r = 3,
    dml_sw_4kb_z = 4,
    dml_sw_4kb_s = 5,
    dml_sw_4kb_d = 6,
    dml_sw_4kb_r = 7,
    dml_sw_64kb_z = 8,
    dml_sw_64kb_s = 9,
    dml_sw_64kb_d = 10,
    dml_sw_64kb_r = 11,
    dml_sw_256kb_z = 12,
    dml_sw_256kb_s = 13,
    dml_sw_256kb_d = 14,
    dml_sw_256kb_r = 15,
    dml_sw_64kb_z_t = 16,
    dml_sw_64kb_s_t = 17,
    dml_sw_64kb_d_t = 18,
    dml_sw_64kb_r_t = 19,
    dml_sw_4kb_z_x = 20,
    dml_sw_4kb_s_x = 21,
    dml_sw_4kb_d_x = 22,
    dml_sw_4kb_r_x = 23,
    dml_sw_64kb_z_x = 24,
    dml_sw_64kb_s_x = 25,
    dml_sw_64kb_d_x = 26,
    dml_sw_64kb_r_x = 27,
    dml_sw_256kb_z_x = 28,
    dml_sw_256kb_s_x = 29,
    dml_sw_256kb_d_x = 30,
    dml_sw_256kb_r_x = 31,
    dml_sw_256b_2d = 32,
    dml_sw_4kb_2d = 33,
    dml_sw_64kb_2d = 34,
    dml_sw_256kb_2d = 35
},
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_lb_depth {
    dml_lb_6 = 0,
    dml_lb_8 = 1,
    dml_lb_10 = 2,
    dml_lb_12 = 3,
    dml_lb_16 = 4
},
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_voltage_state {
    dml_vmin_lv = 0,
    dml_vmin = 1,
    dml_vmid = 2,
    dml_vnom = 3,
    dml_vmax = 4
},
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_source_macro_tile_size {
    dml_4k_tile = 0,
    dml_64k_tile = 1,
    dml_256k_tile = 2
},
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_cursor_bpp {
    dml_cur_2bit = 0,
    dml_cur_32bit = 1,
    dml_cur_64bit = 2
},
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_dram_clock_change_support {
    dml_dram_clock_change_vactive = 0,
    dml_dram_clock_change_vblank = 1,
    dml_dram_clock_change_vblank_drr = 2,
    dml_dram_clock_change_vactive_w_mall_full_frame = 3,
    dml_dram_clock_change_vactive_w_mall_sub_vp = 4,
    dml_dram_clock_change_vblank_w_mall_full_frame = 5,
    dml_dram_clock_change_vblank_drr_w_mall_full_frame = 6,
    dml_dram_clock_change_vblank_w_mall_sub_vp = 7,
    dml_dram_clock_change_vblank_drr_w_mall_sub_vp = 8,
    dml_dram_clock_change_unsupported = 9
},
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_fclock_change_support {
    dml_fclock_change_vactive = 0,
    dml_fclock_change_vblank = 1,
    dml_fclock_change_unsupported = 2
},
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_dsc_enable {
    dml_dsc_disable = 0,
    dml_dsc_enable = 1,
    dml_dsc_enable_if_necessary = 2
},
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_mpc_use_policy {
    dml_mpc_disabled = 0,
    dml_mpc_as_possible = 1,
    dml_mpc_as_needed_for_voltage = 2,
    dml_mpc_as_needed_for_pstate_and_voltage = 3,
    dml_mpc_as_needed = 4,
    dml_mpc_2to1 = 5
},
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_odm_use_policy {
    dml_odm_use_policy_bypass = 0,
    dml_odm_use_policy_combine_as_needed = 1,
    dml_odm_use_policy_combine_2to1 = 2,
    dml_odm_use_policy_combine_3to1 = 3,
    dml_odm_use_policy_combine_4to1 = 4,
    dml_odm_use_policy_split_1to2 = 5,
    dml_odm_use_policy_mso_1to2 = 6,
    dml_odm_use_policy_mso_1to4 = 7
},
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_odm_mode {
    dml_odm_mode_bypass = 0,
    dml_odm_mode_combine_2to1 = 1,
    dml_odm_mode_combine_3to1 = 2,
    dml_odm_mode_combine_4to1 = 3,
    dml_odm_mode_split_1to2 = 4,
    dml_odm_mode_mso_1to2 = 5,
    dml_odm_mode_mso_1to4 = 6
},
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_writeback_configuration {
    dml_whole_buffer_for_single_stream_no_interleave = 0,
    dml_whole_buffer_for_single_stream_interleave = 1
},
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_immediate_flip_requirement {
    dml_immediate_flip_not_required = 0,
    dml_immediate_flip_required = 1,
    dml_immediate_flip_if_possible = 2
},
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_unbounded_requesting_policy {
    dml_unbounded_requesting_enable = 0,
    dml_unbounded_requesting_edp_only = 1,
    dml_unbounded_requesting_disable = 2
},
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dml_clk_cfg_policy {
    dml_use_required_freq = 0,
    dml_use_override_freq = 1,
    dml_use_state_freq = 2
},
#[repr(C)]
pub struct soc_state_bounding_box_st {
	socclk_mhz: dml_float_t,
	dscclk_mhz: dml_float_t,
	phyclk_mhz: dml_float_t,
	phyclk_d18_mhz: dml_float_t,
	phyclk_d32_mhz: dml_float_t,
	dtbclk_mhz: dml_float_t,
	fabricclk_mhz: dml_float_t,
	dcfclk_mhz: dml_float_t,
	dispclk_mhz: dml_float_t,
	dppclk_mhz: dml_float_t,
	dram_speed_mts: dml_float_t,
	urgent_latency_pixel_data_only_us: dml_float_t,
	urgent_latency_pixel_mixed_with_vm_data_us: dml_float_t,
	urgent_latency_vm_data_only_us: dml_float_t,
	writeback_latency_us: dml_float_t,
	urgent_latency_adjustment_fabric_clock_component_us: dml_float_t,
	urgent_latency_adjustment_fabric_clock_reference_mhz: dml_float_t,
	sr_exit_time_us: dml_float_t,
	sr_enter_plus_exit_time_us: dml_float_t,
	sr_exit_z8_time_us: dml_float_t,
	sr_enter_plus_exit_z8_time_us: dml_float_t,
	dram_clock_change_latency_us: dml_float_t,
	fclk_change_latency_us: dml_float_t,
	usr_retraining_latency_us: dml_float_t,
	use_ideal_dram_bw_strobe: dml_bool_t,
	g6_temp_read_blackout_us: dml_float_t,

	#[repr(C)]
pub struct Anonymous {
		urgent_ramp_uclk_cycles: dml_uint_t,
		trip_to_memory_uclk_cycles: dml_uint_t,
		meta_trip_to_memory_uclk_cycles: dml_uint_t,
		maximum_latency_when_urgent_uclk_cycles: dml_uint_t,
		average_latency_when_urgent_uclk_cycles: dml_uint_t,
		maximum_latency_when_non_urgent_uclk_cycles: dml_uint_t,
		average_latency_when_non_urgent_uclk_cycles: dml_uint_t
}  dml_dcn401_uclk_dpm_dependent_soc_qos_params
},
#[repr(C)]
pub struct soc_bounding_box_st {
	dprefclk_mhz: dml_float_t,
	xtalclk_mhz: dml_float_t,
	pcierefclk_mhz: dml_float_t,
	refclk_mhz: dml_float_t,
	amclk_mhz: dml_float_t,
	max_outstanding_reqs: dml_uint_t,
	pct_ideal_sdp_bw_after_urgent: dml_float_t,
	pct_ideal_fabric_bw_after_urgent: dml_float_t,
	pct_ideal_dram_bw_after_urgent_pixel_only: dml_float_t,
	pct_ideal_dram_bw_after_urgent_pixel_and_vm: dml_float_t,
	pct_ideal_dram_bw_after_urgent_vm_only: dml_float_t,
	pct_ideal_dram_bw_after_urgent_strobe: dml_float_t,
	max_avg_sdp_bw_use_normal_percent: dml_float_t,
	max_avg_fabric_bw_use_normal_percent: dml_float_t,
	max_avg_dram_bw_use_normal_percent: dml_float_t,
	max_avg_dram_bw_use_normal_strobe_percent: dml_float_t,

	svp_prefetch_pct_ideal_sdp_bw_after_urgent: dml_float_t,
	svp_prefetch_pct_ideal_fabric_bw_after_urgent: dml_float_t,
	svp_prefetch_pct_ideal_dram_bw_after_urgent_pixel_only: dml_float_t,
	svp_prefetch_pct_ideal_dram_bw_after_urgent_pixel_and_vm: dml_float_t,
	svp_prefetch_pct_ideal_dram_bw_after_urgent_vm_only: dml_float_t,
	svp_prefetch_max_avg_sdp_bw_use_normal_percent: dml_float_t,
	svp_prefetch_max_avg_fabric_bw_use_normal_percent: dml_float_t,
	svp_prefetch_max_avg_dram_bw_use_normal_percent: dml_float_t,

	round_trip_ping_latency_dcfclk_cycles: dml_uint_t,
	urgent_out_of_order_return_per_channel_pixel_only_bytes: dml_uint_t,
	urgent_out_of_order_return_per_channel_pixel_and_vm_bytes: dml_uint_t,
	urgent_out_of_order_return_per_channel_vm_only_bytes: dml_uint_t,
	num_chans: dml_uint_t,
	return_bus_width_bytes: dml_uint_t,
	dram_channel_width_bytes: dml_uint_t,
	fabric_datapath_to_dcn_data_return_bytes: dml_uint_t,
	hostvm_min_page_size_kbytes: dml_uint_t,
	gpuvm_min_page_size_kbytes: dml_uint_t,
	phy_downspread_percent: dml_float_t,
	dcn_downspread_percent: dml_float_t,
	smn_latency_us: dml_float_t,
	mall_allocated_for_dcn_mbytes: dml_uint_t,
	dispclk_dppclk_vco_speed_mhz: dml_float_t,
	do_urgent_latency_adjustment: dml_bool_t,

	mem_word_bytes: dml_uint_t,
	num_dcc_mcaches: dml_uint_t,
	mcache_size_bytes: dml_uint_t,
	mcache_line_size_bytes: dml_uint_t,

	#[repr(C)]
pub struct Anonymous {
		UseNewDCN401SOCParameters: dml_bool_t,
		df_qos_response_time_fclk_cycles: dml_uint_t,
		max_round_trip_to_furthest_cs_fclk_cycles: dml_uint_t,
		mall_overhead_fclk_cycles: dml_uint_t,
		meta_trip_adder_fclk_cycles: dml_uint_t,
		average_transport_distance_fclk_cycles: dml_uint_t,
		umc_urgent_ramp_latency_margin: dml_float_t,
		umc_max_latency_margin: dml_float_t,
		umc_average_latency_margin: dml_float_t,
		fabric_max_transport_latency_margin: dml_float_t,
		fabric_average_transport_latency_margin: dml_float_t
}  dml_dcn401_soc_qos_params
},
#[repr(C)]
pub struct ip_params_st {
	vblank_nom_default_us: dml_uint_t,
	rob_buffer_size_kbytes: dml_uint_t,
	config_return_buffer_size_in_kbytes: dml_uint_t,
	config_return_buffer_segment_size_in_kbytes: dml_uint_t,
	compressed_buffer_segment_size_in_kbytes: dml_uint_t,
	meta_fifo_size_in_kentries: dml_uint_t,
	zero_size_buffer_entries: dml_uint_t,
	dpte_buffer_size_in_pte_reqs_luma: dml_uint_t,
	dpte_buffer_size_in_pte_reqs_chroma: dml_uint_t,
	dcc_meta_buffer_size_bytes: dml_uint_t,
	gpuvm_enable: dml_bool_t,
	hostvm_enable: dml_bool_t,
	gpuvm_max_page_table_levels: dml_uint_t,
	hostvm_max_page_table_levels: dml_uint_t,
	pixel_chunk_size_kbytes: dml_uint_t,
	alpha_pixel_chunk_size_kbytes: dml_uint_t,
	min_pixel_chunk_size_bytes: dml_uint_t,
	meta_chunk_size_kbytes: dml_uint_t,
	min_meta_chunk_size_bytes: dml_uint_t,
	writeback_chunk_size_kbytes: dml_uint_t,
	line_buffer_size_bits: dml_uint_t,
	max_line_buffer_lines: dml_uint_t,
	writeback_interface_buffer_size_kbytes: dml_uint_t,
	max_num_dpp: dml_uint_t,
	max_num_otg: dml_uint_t,
	max_num_wb: dml_uint_t,
	max_dchub_pscl_bw_pix_per_clk: dml_uint_t,
	max_pscl_lb_bw_pix_per_clk: dml_uint_t,
	max_lb_vscl_bw_pix_per_clk: dml_uint_t,
	max_vscl_hscl_bw_pix_per_clk: dml_uint_t,
	max_hscl_ratio: dml_float_t,
	max_vscl_ratio: dml_float_t,
	max_hscl_taps: dml_uint_t,
	max_vscl_taps: dml_uint_t,
	num_dsc: dml_uint_t,
	maximum_dsc_bits_per_component: dml_uint_t,
	maximum_pixels_per_line_per_dsc_unit: dml_uint_t,
	dsc422_native_support: dml_bool_t,
	cursor_64bpp_support: dml_bool_t,
	dispclk_ramp_margin_percent: dml_float_t,
	dppclk_delay_subtotal: dml_uint_t,
	dppclk_delay_scl: dml_uint_t,
	dppclk_delay_scl_lb_only: dml_uint_t,
	dppclk_delay_cnvc_formatter: dml_uint_t,
	dppclk_delay_cnvc_cursor: dml_uint_t,
	cursor_buffer_size: dml_uint_t,
	cursor_chunk_size: dml_uint_t,
	dispclk_delay_subtotal: dml_uint_t,
	dynamic_metadata_vm_enabled: dml_bool_t,
	max_inter_dcn_tile_repeaters: dml_uint_t,
	max_num_hdmi_frl_outputs: dml_uint_t,
	max_num_dp2p0_outputs: dml_uint_t,
	max_num_dp2p0_streams: dml_uint_t,
	dcc_supported: dml_bool_t,
	ptoi_supported: dml_bool_t,
	writeback_max_hscl_ratio: dml_float_t,
	writeback_max_vscl_ratio: dml_float_t,
	writeback_min_hscl_ratio: dml_float_t,
	writeback_min_vscl_ratio: dml_float_t,
	writeback_max_hscl_taps: dml_uint_t,
	writeback_max_vscl_taps: dml_uint_t,
	writeback_line_buffer_buffer_size: dml_uint_t
},
#[repr(C)]
pub struct DmlPipe {
	Dppclk: dml_float_t,
	Dispclk: dml_float_t,
	PixelClock: dml_float_t,
	DCFClkDeepSleep: dml_float_t,
	DPPPerSurface: dml_uint_t,
	ScalerEnabled: dml_bool_t,
	SourceScan: enum dml_rotation_angle,
	ViewportHeight: dml_uint_t,
	ViewportHeightChroma: dml_uint_t,
	BlockWidth256BytesY: dml_uint_t,
	BlockHeight256BytesY: dml_uint_t,
	BlockWidth256BytesC: dml_uint_t,
	BlockHeight256BytesC: dml_uint_t,
	BlockWidthY: dml_uint_t,
	BlockHeightY: dml_uint_t,
	BlockWidthC: dml_uint_t,
	BlockHeightC: dml_uint_t,
	InterlaceEnable: dml_uint_t,
	NumberOfCursors: dml_uint_t,
	VBlank: dml_uint_t,
	HTotal: dml_uint_t,
	HActive: dml_uint_t,
	DCCEnable: dml_bool_t,
	ODMMode: enum dml_odm_mode,
	SourcePixelFormat: enum dml_source_format_class,
	SurfaceTiling: enum dml_swizzle_mode,
	BytePerPixelY: dml_uint_t,
	BytePerPixelC: dml_uint_t,
	ProgressiveToInterlaceUnitInOPP: dml_bool_t,
	VRatio: dml_float_t,
	VRatioChroma: dml_float_t,
	VTaps: dml_uint_t,
	VTapsChroma: dml_uint_t,
	PitchY: dml_uint_t,
	DCCMetaPitchY: dml_uint_t,
	PitchC: dml_uint_t,
	DCCMetaPitchC: dml_uint_t,
	ViewportStationary: dml_bool_t,
	ViewportXStart: dml_uint_t,
	ViewportYStart: dml_uint_t,
	ViewportXStartC: dml_uint_t,
	ViewportYStartC: dml_uint_t,
	FORCE_ONE_ROW_FOR_FRAME: dml_bool_t,
	SwathHeightY: dml_uint_t,
	SwathHeightC: dml_uint_t
},
#[repr(C)]
pub struct Watermarks {
	UrgentWatermark: dml_float_t,
	WritebackUrgentWatermark: dml_float_t,
	DRAMClockChangeWatermark: dml_float_t,
	FCLKChangeWatermark: dml_float_t,
	WritebackDRAMClockChangeWatermark: dml_float_t,
	WritebackFCLKChangeWatermark: dml_float_t,
	StutterExitWatermark: dml_float_t,
	StutterEnterPlusExitWatermark: dml_float_t,
	Z8StutterExitWatermark: dml_float_t,
	Z8StutterEnterPlusExitWatermark: dml_float_t,
	USRRetrainingWatermark: dml_float_t
},
#[repr(C)]
pub struct SOCParametersList {
	UrgentLatency: dml_float_t,
	ExtraLatency: dml_float_t,
	WritebackLatency: dml_float_t,
	DRAMClockChangeLatency: dml_float_t,
	FCLKChangeLatency: dml_float_t,
	SRExitTime: dml_float_t,
	SREnterPlusExitTime: dml_float_t,
	SRExitZ8Time: dml_float_t,
	SREnterPlusExitZ8Time: dml_float_t,
	USRRetrainingLatency: dml_float_t,
	SMNLatency: dml_float_t
},
/// @brief Struct that represent Plane configration of a display cfg
#[repr(C)]
pub struct dml_plane_cfg_st {
	//
	// Pipe/Surface Parameters
	//
	GPUVMEnable: dml_bool_t, /// <brief Set if any pipe has GPUVM enable
	HostVMEnable: dml_bool_t, /// <brief Set if any pipe has HostVM enable

	GPUVMMaxPageTableLevels: dml_uint_t, /// <brief GPUVM level; max of all pipes'
	HostVMMaxPageTableLevels: dml_uint_t, /// <brief HostVM level; max of all pipes'; that is the number of non-cache HVM level

	GPUVMMinPageSizeKBytes: [dml_uint_t; __DML_NUM_PLANES__],
	ForceOneRowForFrame: [dml_bool_t; __DML_NUM_PLANES__],
	PTEBufferModeOverrideEn: [dml_bool_t; __DML_NUM_PLANES__], //< brief when override enable; the DML will only check the given pte buffer and will use the pte buffer mode as is
	PTEBufferMode: [dml_bool_t; __DML_NUM_PLANES__],
	ViewportWidth: [dml_uint_t; __DML_NUM_PLANES__],
	ViewportHeight: [dml_uint_t; __DML_NUM_PLANES__],
	ViewportWidthChroma: [dml_uint_t; __DML_NUM_PLANES__],
	ViewportHeightChroma: [dml_uint_t; __DML_NUM_PLANES__],
	ViewportXStart: [dml_uint_t; __DML_NUM_PLANES__],
	ViewportXStartC: [dml_uint_t; __DML_NUM_PLANES__],
	ViewportYStart: [dml_uint_t; __DML_NUM_PLANES__],
	ViewportYStartC: [dml_uint_t; __DML_NUM_PLANES__],
	ViewportStationary: [dml_bool_t; __DML_NUM_PLANES__],

	ScalerEnabled: [dml_bool_t; __DML_NUM_PLANES__],
	HRatio: [dml_float_t; __DML_NUM_PLANES__],
	VRatio: [dml_float_t; __DML_NUM_PLANES__],
	HRatioChroma: [dml_float_t; __DML_NUM_PLANES__],
	VRatioChroma: [dml_float_t; __DML_NUM_PLANES__],
	HTaps: [dml_uint_t; __DML_NUM_PLANES__],
	VTaps: [dml_uint_t; __DML_NUM_PLANES__],
	HTapsChroma: [dml_uint_t; __DML_NUM_PLANES__],
	VTapsChroma: [dml_uint_t; __DML_NUM_PLANES__],
	LBBitPerPixel: [dml_uint_t; __DML_NUM_PLANES__],

	SourceScan: [enum dml_rotation_angle; __DML_NUM_PLANES__],
	ScalerRecoutWidth: [dml_uint_t; __DML_NUM_PLANES__],

	DynamicMetadataEnable: [dml_bool_t; __DML_NUM_PLANES__],
	DynamicMetadataLinesBeforeActiveRequired: [dml_uint_t; __DML_NUM_PLANES__],
	DynamicMetadataTransmittedBytes: [dml_uint_t; __DML_NUM_PLANES__],
	DETSizeOverride: [dml_uint_t; __DML_NUM_PLANES__], /// <brief user can specify the desire DET buffer usage per-plane

	NumberOfCursors: [dml_uint_t; __DML_NUM_PLANES__],
	CursorWidth: [dml_uint_t; __DML_NUM_PLANES__],
	CursorBPP: [dml_uint_t; __DML_NUM_PLANES__],

	setup_for_tdlut: [dml_bool_t; __DML_NUM_PLANES__],
	tdlut_addressing_mode: [enum dml2_tdlut_addressing_mode; __DML_NUM_PLANES__],
	tdlut_width_mode: [enum dml2_tdlut_width_mode; __DML_NUM_PLANES__],

	UseMALLForStaticScreen: [enum dml_use_mall_for_static_screen_mode; __DML_NUM_PLANES__],
	UseMALLForPStateChange: [enum dml_use_mall_for_pstate_change_mode; __DML_NUM_PLANES__],

	BlendingAndTiming: [dml_uint_t; __DML_NUM_PLANES__], /// <brief From which timing group (like OTG) that this plane is getting its timing from. Mode check also need this info for example to check num OTG; encoder; dsc etc.
}; // dml_plane_cfg_st,
/// @brief Surface Parameters
#[repr(C)]
pub struct dml_surface_cfg_st {
	SurfaceTiling: [enum dml_swizzle_mode; __DML_NUM_PLANES__],
	SourcePixelFormat: [enum dml_source_format_class; __DML_NUM_PLANES__],
	PitchY: [dml_uint_t; __DML_NUM_PLANES__],
	SurfaceWidthY: [dml_uint_t; __DML_NUM_PLANES__],
	SurfaceHeightY: [dml_uint_t; __DML_NUM_PLANES__],
	PitchC: [dml_uint_t; __DML_NUM_PLANES__],
	SurfaceWidthC: [dml_uint_t; __DML_NUM_PLANES__],
	SurfaceHeightC: [dml_uint_t; __DML_NUM_PLANES__],

	DCCEnable: [dml_bool_t; __DML_NUM_PLANES__],
	DCCMetaPitchY: [dml_uint_t; __DML_NUM_PLANES__],
	DCCMetaPitchC: [dml_uint_t; __DML_NUM_PLANES__],

	DCCRateLuma: [dml_float_t; __DML_NUM_PLANES__],
	DCCRateChroma: [dml_float_t; __DML_NUM_PLANES__],
	DCCFractionOfZeroSizeRequestsLuma: [dml_float_t; __DML_NUM_PLANES__],
	DCCFractionOfZeroSizeRequestsChroma: [dml_float_t; __DML_NUM_PLANES__]
}; // dml_surface_cfg_st

/// @brief structure that represents the timing configuration
#[repr(C)]
pub struct dml_timing_cfg_st {
	HTotal: [dml_uint_t; __DML_NUM_PLANES__],
	VTotal: [dml_uint_t; __DML_NUM_PLANES__],
	HBlankEnd: [dml_uint_t; __DML_NUM_PLANES__],
	VBlankEnd: [dml_uint_t; __DML_NUM_PLANES__],
	RefreshRate: [dml_uint_t; __DML_NUM_PLANES__],
	VFrontPorch: [dml_uint_t; __DML_NUM_PLANES__],
	PixelClock: [dml_float_t; __DML_NUM_PLANES__],
	HActive: [dml_uint_t; __DML_NUM_PLANES__],
	VActive: [dml_uint_t; __DML_NUM_PLANES__],
	Interlace: [dml_bool_t; __DML_NUM_PLANES__],
	DRRDisplay: [dml_bool_t; __DML_NUM_PLANES__],
	VBlankNom: [dml_uint_t; __DML_NUM_PLANES__]
}; // dml_timing_cfg_st,
/// @brief structure that represents the output stream
#[repr(C)]
pub struct dml_output_cfg_st {
	// Output Setting
	DSCInputBitPerComponent: [dml_uint_t; __DML_NUM_PLANES__],
	OutputFormat: [enum dml_output_format_class; __DML_NUM_PLANES__],
	OutputEncoder: [enum dml_output_encoder_class; __DML_NUM_PLANES__],
	OutputMultistreamId: [dml_uint_t; __DML_NUM_PLANES__],
	OutputMultistreamEn: [dml_bool_t; __DML_NUM_PLANES__],
	OutputBpp: [dml_float_t; __DML_NUM_PLANES__], //< brief Use by mode_programming to specify a output bpp; user can use the output from mode_support (support.OutputBpp)
	PixelClockBackEnd: [dml_float_t; __DML_NUM_PLANES__],
	DSCEnable: [enum dml_dsc_enable; __DML_NUM_PLANES__], //< brief for mode support check; use to determine if dsc is required
	OutputLinkDPLanes: [dml_uint_t; __DML_NUM_PLANES__],
	OutputLinkDPRate: [enum dml_output_link_dp_rate; __DML_NUM_PLANES__],
	ForcedOutputLinkBPP: [dml_float_t; __DML_NUM_PLANES__],
	AudioSampleRate: [dml_uint_t; __DML_NUM_PLANES__],
	AudioSampleLayout: [dml_uint_t; __DML_NUM_PLANES__],
	OutputDisabled: [dml_bool_t; __DML_NUM_PLANES__],
	DSCSlices: [dml_uint_t; __DML_NUM_PLANES__]
}; // dml_timing_cfg_st,
/// @brief Writeback Setting
#[repr(C)]
pub struct dml_writeback_cfg_st {
	WritebackPixelFormat: [enum dml_source_format_class; __DML_NUM_PLANES__],
	WritebackEnable: [dml_bool_t; __DML_NUM_PLANES__],
	ActiveWritebacksPerSurface: [dml_uint_t; __DML_NUM_PLANES__],
	WritebackDestinationWidth: [dml_uint_t; __DML_NUM_PLANES__],
	WritebackDestinationHeight: [dml_uint_t; __DML_NUM_PLANES__],
	WritebackSourceWidth: [dml_uint_t; __DML_NUM_PLANES__],
	WritebackSourceHeight: [dml_uint_t; __DML_NUM_PLANES__],
	WritebackHTaps: [dml_uint_t; __DML_NUM_PLANES__],
	WritebackVTaps: [dml_uint_t; __DML_NUM_PLANES__],
	WritebackHRatio: [dml_float_t; __DML_NUM_PLANES__],
	WritebackVRatio: [dml_float_t; __DML_NUM_PLANES__]
}; // dml_writeback_cfg_st,
/// @brief Hardware resource specific; mainly used by mode_programming when test/sw wants to do some specific setting
///        which are not the same as what the mode support stage derive.  When call mode_support with mode_programm; the hw-specific
//         resource will be set to what the mode_support layer recommends
#[repr(C)]
pub struct dml_hw_resource_st {
	ODMMode: [enum dml_odm_mode; __DML_NUM_PLANES__], /// <brief ODM mode that is chosen in the mode check stage and will be used in mode programming stage
	DPPPerSurface: [dml_uint_t; __DML_NUM_PLANES__], /// <brief How many DPPs are needed drive the surface to output. If MPCC or ODMC could be 2 or 4.
	DSCEnabled: [dml_bool_t; __DML_NUM_PLANES__], /// <brief Indicate if the DSC is enabled; used in mode_programming
	NumberOfDSCSlices: [dml_uint_t; __DML_NUM_PLANES__], /// <brief Indicate how many slices needed to support the given mode
	DLGRefClkFreqMHz: dml_float_t, /// <brief DLG Global Reference timer
},
/// @brief To control the clk usage for model programming
#[repr(C)]
pub struct dml_clk_cfg_st {
	dcfclk_option: enum dml_clk_cfg_policy, ///< brief Use for mode_program; user can select between use the min require clk req as calculated by DML or use the test-specific freq
	dispclk_option: enum dml_clk_cfg_policy, ///< brief Use for mode_program; user can select between use the min require clk req as calculated by DML or use the test-specific freq
	dppclk_option: [enum dml_clk_cfg_policy; __DML_NUM_PLANES__],

	dcfclk_mhz: dml_float_t,
	dispclk_mhz: dml_float_t,
	dppclk_mhz: [dml_float_t; __DML_NUM_PLANES__]
}; // dml_clk_cfg_st

/// @brief DML display configuration.
///        Describe how to display a surface in multi-plane setup and output to different output and writeback using the specified timgin
#[repr(C)]
pub struct dml_display_cfg_st {
	surface: struct dml_surface_cfg_st,
	plane: struct dml_plane_cfg_st,
	timing: struct dml_timing_cfg_st,
	output: struct dml_output_cfg_st,
	writeback: struct dml_writeback_cfg_st,
	num_surfaces: dml_uint_t,
	num_timings: dml_uint_t,

	hw: struct dml_hw_resource_st, //< brief for mode programming
	clk_overrides: struct dml_clk_cfg_st,   //< brief for mode programming clk override
}; // dml_display_cfg_st

/// @brief DML mode evaluation and programming policy
/// Those knobs that affect mode support and mode programming
#[repr(C)]
pub struct dml_mode_eval_policy_st {
	// -------------------
	// Policy
	// -------------------
	MPCCombineUse: [enum dml_mpc_use_policy; __DML_NUM_PLANES__], /// <brief MPC Combine mode as selected by the user; used in mode check stage
	ODMUse: [enum dml_odm_use_policy; __DML_NUM_PLANES__], /// <brief ODM mode as selected by the user; used in mode check stage
	UseUnboundedRequesting: enum dml_unbounded_requesting_policy, ///< brief Unbounded request mode preference
	ImmediateFlipRequirement: [enum dml_immediate_flip_requirement; __DML_NUM_PLANES__], /// <brief Is immediate flip a requirement for this plane. When host vm is present iflip is needed regardless
	AllowForPStateChangeOrStutterInVBlank: [enum dml_prefetch_modes; __DML_NUM_PLANES__], /// <brief To specify if the DML should calculate the values for support different pwr saving features (cstate; pstate; etc.) during vblank

	AllowForPStateChangeOrStutterInVBlankFinal: enum dml_prefetch_modes,
	dml_bool_t UseOnlyMaxPrefetchModes,
	UseMinimumRequiredDCFCLK: dml_bool_t, //<brief When set the mode_check stage will figure the min DCFCLK freq to support the given display configuration. User can tell use the output DCFCLK for mode programming.
	DRAMClockChangeRequirementFinal: dml_bool_t,
	FCLKChangeRequirementFinal: dml_bool_t,
	USRRetrainingRequiredFinal: dml_bool_t,
	EnhancedPrefetchScheduleAccelerationFinal: dml_bool_t,

	NomDETInKByteOverrideEnable: dml_bool_t, //<brief Nomimal DET buffer size for a pipe. If this size fit the required 2 swathes; DML will use this DET size
	NomDETInKByteOverrideValue: dml_uint_t,

	DCCProgrammingAssumesScanDirectionUnknownFinal: dml_bool_t,
	SynchronizeTimingsFinal: dml_bool_t,
	SynchronizeDRRDisplaysForUCLKPStateChangeFinal: dml_bool_t,
	AssumeModeSupportAtMaxPwrStateEvenDRAMClockChangeNotSupported: dml_bool_t, //<brief if set; the mode support will say mode is supported even though the DRAM clock change is not support (assuming the soc will be stay in max power state)
	AssumeModeSupportAtMaxPwrStateEvenFClockChangeNotSupported: dml_bool_t, //<brief if set; the mode support will say mode is supported even though the Fabric clock change is not support (assuming the soc will be stay in max power state
},
/// @brief Contains important information after the mode support steps. Also why a mode is not supported.
#[repr(C)]
pub struct dml_mode_support_info_st {
	//-----------------
	// Mode Support Information
	//-----------------
	ModeIsSupported: dml_bool_t, //<brief Is the mode support any voltage and combine setting
	ImmediateFlipSupport: dml_bool_t, //<brief Means mode support immediate flip at the max combine setting; determine in mode support and used in mode programming
	MaximumMPCCombine: dml_uint_t, //<brief If using MPC combine helps the power saving support; then this will be set to 1
	UnboundedRequestEnabled: dml_bool_t,
	CompressedBufferSizeInkByte: dml_uint_t,

	/* Mode Support Reason */
	WritebackLatencySupport: dml_bool_t,
	ScaleRatioAndTapsSupport: dml_bool_t,
	SourceFormatPixelAndScanSupport: dml_bool_t,
	MPCCombineMethodIncompatible: dml_bool_t,
	P2IWith420: dml_bool_t,
	DSCOnlyIfNecessaryWithBPP: dml_bool_t,
	DSC422NativeNotSupported: dml_bool_t,
	LinkRateDoesNotMatchDPVersion: dml_bool_t,
	LinkRateForMultistreamNotIndicated: dml_bool_t,
	BPPForMultistreamNotIndicated: dml_bool_t,
	MultistreamWithHDMIOreDP: dml_bool_t,
	MSOOrODMSplitWithNonDPLink: dml_bool_t,
	NotEnoughLanesForMSO: dml_bool_t,
	NumberOfOTGSupport: dml_bool_t,
	NumberOfHDMIFRLSupport: dml_bool_t,
	NumberOfDP2p0Support: dml_bool_t,
	NonsupportedDSCInputBPC: dml_bool_t,
	WritebackScaleRatioAndTapsSupport: dml_bool_t,
	CursorSupport: dml_bool_t,
	PitchSupport: dml_bool_t,
	ViewportExceedsSurface: dml_bool_t,
	ImmediateFlipRequiredButTheRequirementForEachSurfaceIsNotSpecified: dml_bool_t,
	ImmediateFlipOrHostVMAndPStateWithMALLFullFrameOrPhantomPipe: dml_bool_t,
	InvalidCombinationOfMALLUseForPStateAndStaticScreen: dml_bool_t,
	InvalidCombinationOfMALLUseForPState: dml_bool_t,
	ExceededMALLSize: dml_bool_t,
	EnoughWritebackUnits: dml_bool_t,

	ExceededMultistreamSlots: dml_bool_t,
	ODMCombineTwoToOneSupportCheckOK: dml_bool_t,
	ODMCombineFourToOneSupportCheckOK: dml_bool_t,
	NotEnoughDSCUnits: dml_bool_t,
	NotEnoughDSCSlices: dml_bool_t,
	PixelsPerLinePerDSCUnitSupport: dml_bool_t,
	DSCCLKRequiredMoreThanSupported: dml_bool_t,
	DTBCLKRequiredMoreThanSupported: dml_bool_t,
	LinkCapacitySupport: dml_bool_t,

	ROBSupport: [dml_bool_t; 2],
	PTEBufferSizeNotExceeded: [dml_bool_t; 2],
	DCCMetaBufferSizeNotExceeded: [dml_bool_t; 2],
	TotalVerticalActiveBandwidthSupport: [dml_bool_t; 2],
	DRAMClockChangeSupport: [enum dml_dram_clock_change_support; 2],
	ActiveDRAMClockChangeLatencyMargin: [dml_float_t; __DML_NUM_PLANES__],
	SubViewportLinesNeededInMALL: [dml_uint_t; __DML_NUM_PLANES__],
	FCLKChangeSupport: [enum dml_fclock_change_support; 2],
	USRRetrainingSupport: [dml_bool_t; 2],
	VActiveBandwithSupport: [dml_bool_t; 2],
	PrefetchSupported: [dml_bool_t; 2],
	DynamicMetadataSupported: [dml_bool_t; 2],
	VRatioInPrefetchSupported: [dml_bool_t; 2],
	DISPCLK_DPPCLK_Support: [dml_bool_t; 2],
	TotalAvailablePipesSupport: [dml_bool_t; 2],
	ModeSupport: [dml_bool_t; 2],
	ViewportSizeSupport: [dml_bool_t; 2],
	ImmediateFlipSupportedForState: [dml_bool_t; 2],

	dml_bool_t NoTimeForPrefetch[2][__DML_NUM_PLANES__],
	dml_bool_t NoTimeForDynamicMetadata[2][__DML_NUM_PLANES__],
	MPCCombineEnable: [dml_bool_t; __DML_NUM_PLANES__], /// <brief Indicate if the MPC Combine enable in the given state and optimize mpc combine setting
	ODMMode: [enum dml_odm_mode; __DML_NUM_PLANES__], /// <brief ODM mode that is chosen in the mode check stage and will be used in mode programming stage
	DPPPerSurface: [dml_uint_t; __DML_NUM_PLANES__], /// <brief How many DPPs are needed drive the surface to output. If MPCC or ODMC could be 2 or 4.
	DSCEnabled: [dml_bool_t; __DML_NUM_PLANES__], /// <brief Indicate if the DSC is actually required; used in mode_programming
	FECEnabled: [dml_bool_t; __DML_NUM_PLANES__], /// <brief Indicate if the FEC is actually required
	NumberOfDSCSlices: [dml_uint_t; __DML_NUM_PLANES__], /// <brief Indicate how many slices needed to support the given mode

	OutputBpp: [dml_float_t; __DML_NUM_PLANES__],
	OutputType: [enum dml_output_type_and_rate__type; __DML_NUM_PLANES__],
	OutputRate: [enum dml_output_type_and_rate__rate; __DML_NUM_PLANES__],

	AlignedDCCMetaPitchY: [dml_float_t; __DML_NUM_PLANES__], /// <brief Pitch value that is aligned to tiling setting
	AlignedDCCMetaPitchC: [dml_float_t; __DML_NUM_PLANES__],
	AlignedYPitch: [dml_float_t; __DML_NUM_PLANES__],
	AlignedCPitch: [dml_float_t; __DML_NUM_PLANES__],
	MaxTotalVerticalActiveAvailableBandwidth: [dml_float_t; 2], /// <brief nominal bw available for display
}; // dml_mode_support_info_st

/// @brief Treat this as the intermediate values and outputs of mode check function. User can query the content of the struct to know more about the result of mode evaluation.
#[repr(C)]
pub struct mode_support_st {
	ip: struct ip_params_st,
	soc: struct soc_bounding_box_st,
	state: struct soc_state_bounding_box_st, //<brief Per-state bbox values; only 1 state per compute
	policy: struct dml_mode_eval_policy_st,

	state_idx: dml_uint_t, //<brief The power state idx for the power state under this computation
	max_state_idx: dml_uint_t, //<brief The MAX power state idx
	max_state: struct soc_state_bounding_box_st, //<brief The MAX power state; some algo needs to know the max state info to determine if
	cache_display_cfg: struct dml_display_cfg_st, // <brief A copy of the current display cfg in consideration

	// Physical info; only using for programming
	num_active_planes: dml_uint_t, // <brief As determined by either e2e_pipe_param or display_cfg

	// Calculated Clocks
	RequiredDISPCLK: [dml_float_t; 2], /// <brief Required DISPCLK; depends on pixel rate; odm mode etc.
	RequiredDPPCLKThisState: [dml_float_t; __DML_NUM_PLANES__],
	DCFCLKState: [dml_float_t; 2], /// <brief recommended DCFCLK freq; calculated by DML. If UseMinimumRequiredDCFCLK is not set; then it will be just the state DCFCLK; else it will min DCFCLK for support
	dml_float_t RequiredDISPCLKPerSurface[2][__DML_NUM_PLANES__],
	dml_float_t RequiredDPPCLKPerSurface[2][__DML_NUM_PLANES__],
	FabricClock: dml_float_t, /// <brief Basically just the clock freq at the min (or given) state
	DRAMSpeed: dml_float_t, /// <brief Basically just the clock freq at the min (or given) state
	SOCCLK: dml_float_t, /// <brief Basically just the clock freq at the min (or given) state
	DCFCLK: dml_float_t, /// <brief Basically just the clock freq at the min (or given) state and max combine setting
	GlobalDPPCLK: dml_float_t, /// <brief the Max DPPCLK freq out of all pipes

	// ----------------------------------
	// Mode Support Info and fail reason
	// ----------------------------------
	support: struct dml_mode_support_info_st,

	// These are calculated before the ModeSupport and ModeProgram step
	// They represent the bound for the return buffer sizing
	MaxTotalDETInKByte: dml_uint_t,
	NomDETInKByte: dml_uint_t,
	MinCompressedBufferSizeInKByte: dml_uint_t,

	// Info obtained at the end of mode support calculations
	// The reported info is at the "optimal" state and combine setting
	ReturnBW: dml_float_t,
	ReturnDRAMBW: dml_float_t,
	DETBufferSizeInKByte: [dml_uint_t; __DML_NUM_PLANES__], // <brief Recommended DET size configuration for this plane. All pipes under this plane should program the DET buffer size to the calculated value.
	DETBufferSizeY: [dml_uint_t; __DML_NUM_PLANES__],
	DETBufferSizeC: [dml_uint_t; __DML_NUM_PLANES__],
	SwathHeightY: [dml_uint_t; __DML_NUM_PLANES__],
	SwathHeightC: [dml_uint_t; __DML_NUM_PLANES__],

	// ----------------------------------
	// Intermediates/Informational
	// ----------------------------------
	TotImmediateFlipBytes: dml_uint_t,
	DCCEnabledInAnySurface: dml_bool_t,
	WritebackRequiredDISPCLK: dml_float_t,
	TimeCalc: dml_float_t,
	TWait: dml_float_t,

	dml_uint_t SwathWidthYAllStates[2][__DML_NUM_PLANES__],
	dml_uint_t SwathWidthCAllStates[2][__DML_NUM_PLANES__],
	dml_uint_t SwathHeightYAllStates[2][__DML_NUM_PLANES__],
	dml_uint_t SwathHeightCAllStates[2][__DML_NUM_PLANES__],
	SwathWidthYThisState: [dml_uint_t; __DML_NUM_PLANES__],
	SwathWidthCThisState: [dml_uint_t; __DML_NUM_PLANES__],
	SwathHeightYThisState: [dml_uint_t; __DML_NUM_PLANES__],
	SwathHeightCThisState: [dml_uint_t; __DML_NUM_PLANES__],
	dml_uint_t DETBufferSizeInKByteAllStates[2][__DML_NUM_PLANES__],
	dml_uint_t DETBufferSizeYAllStates[2][__DML_NUM_PLANES__],
	dml_uint_t DETBufferSizeCAllStates[2][__DML_NUM_PLANES__],
	UnboundedRequestEnabledAllStates: [dml_bool_t; 2],
	CompressedBufferSizeInkByteAllStates: [dml_uint_t; 2],
	UnboundedRequestEnabledThisState: dml_bool_t,
	CompressedBufferSizeInkByteThisState: dml_uint_t,
	DETBufferSizeInKByteThisState: [dml_uint_t; __DML_NUM_PLANES__],
	DETBufferSizeYThisState: [dml_uint_t; __DML_NUM_PLANES__],
	DETBufferSizeCThisState: [dml_uint_t; __DML_NUM_PLANES__],
	dml_float_t VRatioPreY[2][__DML_NUM_PLANES__],
	dml_float_t VRatioPreC[2][__DML_NUM_PLANES__],
	dml_uint_t swath_width_luma_ub_all_states[2][__DML_NUM_PLANES__],
	dml_uint_t swath_width_chroma_ub_all_states[2][__DML_NUM_PLANES__],
	swath_width_luma_ub_this_state: [dml_uint_t; __DML_NUM_PLANES__],
	swath_width_chroma_ub_this_state: [dml_uint_t; __DML_NUM_PLANES__],
	RequiredSlots: [dml_uint_t; __DML_NUM_PLANES__],
	dml_uint_t PDEAndMetaPTEBytesPerFrame[2][__DML_NUM_PLANES__],
	dml_uint_t MetaRowBytes[2][__DML_NUM_PLANES__],
	dml_uint_t DPTEBytesPerRow[2][__DML_NUM_PLANES__],
	dml_uint_t PrefetchLinesY[2][__DML_NUM_PLANES__],
	dml_uint_t PrefetchLinesC[2][__DML_NUM_PLANES__],
	MaxNumSwY: [dml_uint_t; __DML_NUM_PLANES__], /// <brief Max number of swath for prefetch
	MaxNumSwC: [dml_uint_t; __DML_NUM_PLANES__], /// <brief Max number of swath for prefetch
	PrefillY: [dml_uint_t; __DML_NUM_PLANES__],
	PrefillC: [dml_uint_t; __DML_NUM_PLANES__],

	PrefetchLinesYThisState: [dml_uint_t; __DML_NUM_PLANES__],
	PrefetchLinesCThisState: [dml_uint_t; __DML_NUM_PLANES__],
	DPTEBytesPerRowThisState: [dml_uint_t; __DML_NUM_PLANES__],
	PDEAndMetaPTEBytesPerFrameThisState: [dml_uint_t; __DML_NUM_PLANES__],
	MetaRowBytesThisState: [dml_uint_t; __DML_NUM_PLANES__],
	dml_bool_t use_one_row_for_frame[2][__DML_NUM_PLANES__],
	dml_bool_t use_one_row_for_frame_flip[2][__DML_NUM_PLANES__],
	use_one_row_for_frame_this_state: [dml_bool_t; __DML_NUM_PLANES__],
	use_one_row_for_frame_flip_this_state: [dml_bool_t; __DML_NUM_PLANES__],

	LineTimesForPrefetch: [dml_float_t; __DML_NUM_PLANES__],
	LinesForMetaPTE: [dml_float_t; __DML_NUM_PLANES__],
	LinesForMetaAndDPTERow: [dml_float_t; __DML_NUM_PLANES__],
	SwathWidthYSingleDPP: [dml_float_t; __DML_NUM_PLANES__],
	SwathWidthCSingleDPP: [dml_float_t; __DML_NUM_PLANES__],
	BytePerPixelY: [dml_uint_t; __DML_NUM_PLANES__],
	BytePerPixelC: [dml_uint_t; __DML_NUM_PLANES__],
	BytePerPixelInDETY: [dml_float_t; __DML_NUM_PLANES__],
	BytePerPixelInDETC: [dml_float_t; __DML_NUM_PLANES__],

	Read256BlockHeightY: [dml_uint_t; __DML_NUM_PLANES__],
	Read256BlockWidthY: [dml_uint_t; __DML_NUM_PLANES__],
	Read256BlockHeightC: [dml_uint_t; __DML_NUM_PLANES__],
	Read256BlockWidthC: [dml_uint_t; __DML_NUM_PLANES__],
	MacroTileHeightY: [dml_uint_t; __DML_NUM_PLANES__],
	MacroTileHeightC: [dml_uint_t; __DML_NUM_PLANES__],
	MacroTileWidthY: [dml_uint_t; __DML_NUM_PLANES__],
	MacroTileWidthC: [dml_uint_t; __DML_NUM_PLANES__],
	PSCL_FACTOR: [dml_float_t; __DML_NUM_PLANES__],
	PSCL_FACTOR_CHROMA: [dml_float_t; __DML_NUM_PLANES__],
	MaximumSwathWidthLuma: [dml_float_t; __DML_NUM_PLANES__],
	MaximumSwathWidthChroma: [dml_float_t; __DML_NUM_PLANES__],
	Tno_bw: [dml_float_t; __DML_NUM_PLANES__],
	DestinationLinesToRequestVMInImmediateFlip: [dml_float_t; __DML_NUM_PLANES__],
	DestinationLinesToRequestRowInImmediateFlip: [dml_float_t; __DML_NUM_PLANES__],
	WritebackDelayTime: [dml_float_t; __DML_NUM_PLANES__],
	dpte_group_bytes: [dml_uint_t; __DML_NUM_PLANES__],
	dpte_row_height: [dml_uint_t; __DML_NUM_PLANES__],
	dpte_row_height_chroma: [dml_uint_t; __DML_NUM_PLANES__],
	meta_row_height: [dml_uint_t; __DML_NUM_PLANES__],
	meta_row_height_chroma: [dml_uint_t; __DML_NUM_PLANES__],
	UrgLatency: dml_float_t,
	dml_float_t UrgentBurstFactorCursor[2][__DML_NUM_PLANES__],
	UrgentBurstFactorCursorPre: [dml_float_t; __DML_NUM_PLANES__],
	dml_float_t UrgentBurstFactorLuma[2][__DML_NUM_PLANES__],
	UrgentBurstFactorLumaPre: [dml_float_t; __DML_NUM_PLANES__],
	dml_float_t UrgentBurstFactorChroma[2][__DML_NUM_PLANES__],
	UrgentBurstFactorChromaPre: [dml_float_t; __DML_NUM_PLANES__],
	MaximumSwathWidthInLineBufferLuma: dml_float_t,
	MaximumSwathWidthInLineBufferChroma: dml_float_t,
	ExtraLatency: dml_float_t,

	// Backend
	RequiresDSC: [dml_bool_t; __DML_NUM_PLANES__],
	RequiresFEC: [dml_bool_t; __DML_NUM_PLANES__],
	OutputBppPerState: [dml_float_t; __DML_NUM_PLANES__],
	DSCDelayPerState: [dml_uint_t; __DML_NUM_PLANES__],
	OutputTypePerState: [enum dml_output_type_and_rate__type; __DML_NUM_PLANES__],
	OutputRatePerState: [enum dml_output_type_and_rate__rate; __DML_NUM_PLANES__],

	// Bandwidth Related Info
	BandwidthAvailableForImmediateFlip: dml_float_t,
	ReadBandwidthLuma: [dml_float_t; __DML_NUM_PLANES__],
	ReadBandwidthChroma: [dml_float_t; __DML_NUM_PLANES__],
	WriteBandwidth: [dml_float_t; __DML_NUM_PLANES__],
	RequiredPrefetchPixelDataBWLuma: [dml_float_t; __DML_NUM_PLANES__],
	RequiredPrefetchPixelDataBWChroma: [dml_float_t; __DML_NUM_PLANES__],
	cursor_bw: [dml_float_t; __DML_NUM_PLANES__],
	cursor_bw_pre: [dml_float_t; __DML_NUM_PLANES__],
	prefetch_vmrow_bw: [dml_float_t; __DML_NUM_PLANES__],
	final_flip_bw: [dml_float_t; __DML_NUM_PLANES__],
	meta_row_bandwidth_this_state: [dml_float_t; __DML_NUM_PLANES__],
	dpte_row_bandwidth_this_state: [dml_float_t; __DML_NUM_PLANES__],
	ReturnBWPerState: [dml_float_t; 2],
	ReturnDRAMBWPerState: [dml_float_t; 2],
	dml_float_t meta_row_bandwidth[2][__DML_NUM_PLANES__],
	dml_float_t dpte_row_bandwidth[2][__DML_NUM_PLANES__],
	// Something that should be feedback to caller
	ODMModePerState: [enum dml_odm_mode; __DML_NUM_PLANES__],
	ODMModeThisState: [enum dml_odm_mode; __DML_NUM_PLANES__],
	SurfaceSizeInMALL: [dml_uint_t; __DML_NUM_PLANES__],
	dml_uint_t NoOfDPP[2][__DML_NUM_PLANES__],
	NoOfDPPThisState: [dml_uint_t; __DML_NUM_PLANES__],
	dml_bool_t MPCCombine[2][__DML_NUM_PLANES__],
	MPCCombineThisState: [dml_bool_t; __DML_NUM_PLANES__],
	ProjectedDCFCLKDeepSleep: [dml_float_t; 2],
	MinDPPCLKUsingSingleDPP: [dml_float_t; __DML_NUM_PLANES__],
	SingleDPPViewportSizeSupportPerSurface: [dml_bool_t; __DML_NUM_PLANES__],
	ImmediateFlipSupportedForPipe: [dml_bool_t; __DML_NUM_PLANES__],
	NotUrgentLatencyHiding: [dml_bool_t; __DML_NUM_PLANES__],
	NotUrgentLatencyHidingPre: [dml_bool_t; __DML_NUM_PLANES__],
	PTEBufferSizeNotExceededPerState: [dml_bool_t; __DML_NUM_PLANES__],
	DCCMetaBufferSizeNotExceededPerState: [dml_bool_t; __DML_NUM_PLANES__],
	PrefetchMode: [dml_uint_t; __DML_NUM_PLANES__],
	TotalNumberOfActiveDPP: [dml_uint_t; 2],
	TotalNumberOfSingleDPPSurfaces: [dml_uint_t; 2],
	TotalNumberOfDCCActiveDPP: [dml_uint_t; 2],

	SubViewportLinesNeededInMALL: [dml_uint_t; __DML_NUM_PLANES__]
}; // mode_support_st

/// @brief A mega structure that houses various info for model programming step.
#[repr(C)]
pub struct mode_program_st {

	//-------------
	// Intermediate/Informational
	//-------------
	UrgentLatency: dml_float_t,
	UrgentLatencyWithUSRRetraining: dml_float_t,
	VInitPreFillY: [dml_uint_t; __DML_NUM_PLANES__],
	VInitPreFillC: [dml_uint_t; __DML_NUM_PLANES__],
	MaxNumSwathY: [dml_uint_t; __DML_NUM_PLANES__],
	MaxNumSwathC: [dml_uint_t; __DML_NUM_PLANES__],

	BytePerPixelDETY: [dml_float_t; __DML_NUM_PLANES__],
	BytePerPixelDETC: [dml_float_t; __DML_NUM_PLANES__],
	BytePerPixelY: [dml_uint_t; __DML_NUM_PLANES__],
	BytePerPixelC: [dml_uint_t; __DML_NUM_PLANES__],
	SwathWidthY: [dml_uint_t; __DML_NUM_PLANES__],
	SwathWidthC: [dml_uint_t; __DML_NUM_PLANES__],
	SwathWidthSingleDPPY: [dml_uint_t; __DML_NUM_PLANES__],
	SwathWidthSingleDPPC: [dml_uint_t; __DML_NUM_PLANES__],
	ReadBandwidthSurfaceLuma: [dml_float_t; __DML_NUM_PLANES__],
	ReadBandwidthSurfaceChroma: [dml_float_t; __DML_NUM_PLANES__],

	PixelPTEBytesPerRow: [dml_uint_t; __DML_NUM_PLANES__],
	PDEAndMetaPTEBytesFrame: [dml_uint_t; __DML_NUM_PLANES__],
	MetaRowByte: [dml_uint_t; __DML_NUM_PLANES__],
	PrefetchSourceLinesY: [dml_uint_t; __DML_NUM_PLANES__],
	RequiredPrefetchPixDataBWLuma: [dml_float_t; __DML_NUM_PLANES__],
	RequiredPrefetchPixDataBWChroma: [dml_float_t; __DML_NUM_PLANES__],
	PrefetchSourceLinesC: [dml_uint_t; __DML_NUM_PLANES__],
	PSCL_THROUGHPUT: [dml_float_t; __DML_NUM_PLANES__],
	PSCL_THROUGHPUT_CHROMA: [dml_float_t; __DML_NUM_PLANES__],
	DSCDelay: [dml_uint_t; __DML_NUM_PLANES__],
	DPPCLKUsingSingleDPP: [dml_float_t; __DML_NUM_PLANES__],

	MacroTileWidthY: [dml_uint_t; __DML_NUM_PLANES__],
	MacroTileWidthC: [dml_uint_t; __DML_NUM_PLANES__],
	BlockHeight256BytesY: [dml_uint_t; __DML_NUM_PLANES__],
	BlockHeight256BytesC: [dml_uint_t; __DML_NUM_PLANES__],
	BlockWidth256BytesY: [dml_uint_t; __DML_NUM_PLANES__],
	BlockWidth256BytesC: [dml_uint_t; __DML_NUM_PLANES__],

	BlockHeightY: [dml_uint_t; __DML_NUM_PLANES__],
	BlockHeightC: [dml_uint_t; __DML_NUM_PLANES__],
	BlockWidthY: [dml_uint_t; __DML_NUM_PLANES__],
	BlockWidthC: [dml_uint_t; __DML_NUM_PLANES__],

	SurfaceSizeInTheMALL: [dml_uint_t; __DML_NUM_PLANES__],
	VRatioPrefetchY: [dml_float_t; __DML_NUM_PLANES__],
	VRatioPrefetchC: [dml_float_t; __DML_NUM_PLANES__],
	Tno_bw: [dml_float_t; __DML_NUM_PLANES__],
	final_flip_bw: [dml_float_t; __DML_NUM_PLANES__],
	prefetch_vmrow_bw: [dml_float_t; __DML_NUM_PLANES__],
	cursor_bw: [dml_float_t; __DML_NUM_PLANES__],
	cursor_bw_pre: [dml_float_t; __DML_NUM_PLANES__],
	WritebackDelay: [dml_float_t; __DML_NUM_PLANES__],
	dpte_row_height: [dml_uint_t; __DML_NUM_PLANES__],
	dpte_row_height_linear: [dml_uint_t; __DML_NUM_PLANES__],
	meta_req_width: [dml_uint_t; __DML_NUM_PLANES__],
	meta_req_height: [dml_uint_t; __DML_NUM_PLANES__],
	meta_row_width: [dml_uint_t; __DML_NUM_PLANES__],
	meta_row_height: [dml_uint_t; __DML_NUM_PLANES__],
	dpte_row_width_luma_ub: [dml_uint_t; __DML_NUM_PLANES__],
	dpte_row_width_chroma_ub: [dml_uint_t; __DML_NUM_PLANES__],
	dpte_row_height_chroma: [dml_uint_t; __DML_NUM_PLANES__],
	dpte_row_height_linear_chroma: [dml_uint_t; __DML_NUM_PLANES__],
	meta_req_width_chroma: [dml_uint_t; __DML_NUM_PLANES__],
	meta_req_height_chroma: [dml_uint_t; __DML_NUM_PLANES__],
	meta_row_width_chroma: [dml_uint_t; __DML_NUM_PLANES__],
	meta_row_height_chroma: [dml_uint_t; __DML_NUM_PLANES__],
	vm_group_bytes: [dml_uint_t; __DML_NUM_PLANES__],
	dpte_group_bytes: [dml_uint_t; __DML_NUM_PLANES__],
	meta_row_bw: [dml_float_t; __DML_NUM_PLANES__],
	dpte_row_bw: [dml_float_t; __DML_NUM_PLANES__],
	UrgBurstFactorCursor: [dml_float_t; __DML_NUM_PLANES__],
	UrgBurstFactorCursorPre: [dml_float_t; __DML_NUM_PLANES__],
	UrgBurstFactorLuma: [dml_float_t; __DML_NUM_PLANES__],
	UrgBurstFactorLumaPre: [dml_float_t; __DML_NUM_PLANES__],
	UrgBurstFactorChroma: [dml_float_t; __DML_NUM_PLANES__],
	UrgBurstFactorChromaPre: [dml_float_t; __DML_NUM_PLANES__],

	swath_width_luma_ub: [dml_uint_t; __DML_NUM_PLANES__],
	swath_width_chroma_ub: [dml_uint_t; __DML_NUM_PLANES__],
	PixelPTEReqWidthY: [dml_uint_t; __DML_NUM_PLANES__],
	PixelPTEReqHeightY: [dml_uint_t; __DML_NUM_PLANES__],
	PTERequestSizeY: [dml_uint_t; __DML_NUM_PLANES__],
	PixelPTEReqWidthC: [dml_uint_t; __DML_NUM_PLANES__],
	PixelPTEReqHeightC: [dml_uint_t; __DML_NUM_PLANES__],
	PTERequestSizeC: [dml_uint_t; __DML_NUM_PLANES__],

	Tdmdl_vm: [dml_float_t; __DML_NUM_PLANES__],
	Tdmdl: [dml_float_t; __DML_NUM_PLANES__],
	TSetup: [dml_float_t; __DML_NUM_PLANES__],
	dpde0_bytes_per_frame_ub_l: [dml_uint_t; __DML_NUM_PLANES__],
	meta_pte_bytes_per_frame_ub_l: [dml_uint_t; __DML_NUM_PLANES__],
	dpde0_bytes_per_frame_ub_c: [dml_uint_t; __DML_NUM_PLANES__],
	meta_pte_bytes_per_frame_ub_c: [dml_uint_t; __DML_NUM_PLANES__],

	UnboundedRequestEnabled: dml_bool_t,
	compbuf_reserved_space_64b: dml_uint_t,
	compbuf_reserved_space_zs: dml_uint_t,
	CompressedBufferSizeInkByte: dml_uint_t,

	NoUrgentLatencyHiding: [dml_bool_t; __DML_NUM_PLANES__],
	NoUrgentLatencyHidingPre: [dml_bool_t; __DML_NUM_PLANES__],
	UrgentExtraLatency: dml_float_t,
	PrefetchAndImmediateFlipSupported: dml_bool_t,
	TotalDataReadBandwidth: dml_float_t,
	BandwidthAvailableForImmediateFlip: dml_float_t,
	NotEnoughTimeForDynamicMetadata: [dml_bool_t; __DML_NUM_PLANES__],

	ReadBandwidthLuma: [dml_float_t; __DML_NUM_PLANES__],
	ReadBandwidthChroma: [dml_float_t; __DML_NUM_PLANES__],

	total_dcn_read_bw_with_flip: dml_float_t,
	total_dcn_read_bw_with_flip_no_urgent_burst: dml_float_t,
	TotalDataReadBandwidthNotIncludingMALLPrefetch: dml_float_t,
	total_dcn_read_bw_with_flip_not_including_MALL_prefetch: dml_float_t,
	non_urgent_total_dcn_read_bw_with_flip: dml_float_t,
	non_urgent_total_dcn_read_bw_with_flip_not_including_MALL_prefetch: dml_float_t,

	use_one_row_for_frame: [dml_bool_t; __DML_NUM_PLANES__],
	use_one_row_for_frame_flip: [dml_bool_t; __DML_NUM_PLANES__],

	TCalc: dml_float_t,
	TotImmediateFlipBytes: dml_uint_t,

	// -------------------
	// Output
	// -------------------
	pipe_plane: [dml_uint_t; __DML_NUM_PLANES__], // <brief used mainly by dv to map the pipe inst to plane index within DML core; the plane idx of a pipe
	num_active_pipes: dml_uint_t,

	NoTimeToPrefetch: [dml_bool_t; __DML_NUM_PLANES__], /// <brief Prefetch schedule calculation result

	// Support
	PrefetchMode: [dml_uint_t; __DML_NUM_PLANES__], /// <brief prefetch mode used for prefetch support check in mode programming step
	PrefetchModeSupported: dml_bool_t, /// <brief Is the prefetch mode (bandwidth and latency) supported
	ImmediateFlipSupported: dml_bool_t,
	ImmediateFlipSupportedForPipe: [dml_bool_t; __DML_NUM_PLANES__],

	// Clock
	Dcfclk: dml_float_t,
	Dispclk: dml_float_t, /// <brief dispclk being used in mode programming
	Dppclk: [dml_float_t; __DML_NUM_PLANES__], /// <brief dppclk being used in mode programming
	WritebackDISPCLK: dml_float_t,
	GlobalDPPCLK: dml_float_t,

	//@ brief These "calculated" dispclk and dppclk clocks are calculated in the mode programming step.
	// Depends on the dml_clk_cfg_st option; these calculated values may not used in subsequent calculation.
	// Possible DV usage: Calculated values fetched by test once after mode_programming step and then possibly
	// use the values as min and adjust the actual freq used for the 2nd pass
	Dispclk_calculated: dml_float_t,
	Dppclk_calculated: [dml_float_t; __DML_NUM_PLANES__],

	DSCCLK_calculated: [dml_float_t; __DML_NUM_PLANES__], //< brief Required DSCCLK freq. Backend; not used in any subsequent calculations for now
	DCFCLKDeepSleep: dml_float_t,

	// ARB reg
	DCHUBBUB_ARB_CSTATE_MAX_CAP_MODE: dml_bool_t,
	Watermark: struct Watermarks,

	// DCC compression control
	DCCYMaxUncompressedBlock: [dml_uint_t; __DML_NUM_PLANES__],
	DCCYMaxCompressedBlock: [dml_uint_t; __DML_NUM_PLANES__],
	DCCYIndependentBlock: [dml_uint_t; __DML_NUM_PLANES__],
	DCCCMaxUncompressedBlock: [dml_uint_t; __DML_NUM_PLANES__],
	DCCCMaxCompressedBlock: [dml_uint_t; __DML_NUM_PLANES__],
	DCCCIndependentBlock: [dml_uint_t; __DML_NUM_PLANES__],

	// Stutter Efficiency
	StutterEfficiency: dml_float_t,
	StutterEfficiencyNotIncludingVBlank: dml_float_t,
	NumberOfStutterBurstsPerFrame: dml_uint_t,
	Z8StutterEfficiency: dml_float_t,
	Z8NumberOfStutterBurstsPerFrame: dml_uint_t,
	Z8StutterEfficiencyNotIncludingVBlank: dml_float_t,
	StutterPeriod: dml_float_t,
	Z8StutterEfficiencyBestCase: dml_float_t,
	Z8NumberOfStutterBurstsPerFrameBestCase: dml_uint_t,
	Z8StutterEfficiencyNotIncludingVBlankBestCase: dml_float_t,
	StutterPeriodBestCase: dml_float_t,

	// DLG TTU reg
	MIN_DST_Y_NEXT_START: [dml_float_t; __DML_NUM_PLANES__],
	VREADY_AT_OR_AFTER_VSYNC: [dml_bool_t; __DML_NUM_PLANES__],
	DSTYAfterScaler: [dml_uint_t; __DML_NUM_PLANES__],
	DSTXAfterScaler: [dml_uint_t; __DML_NUM_PLANES__],
	DestinationLinesForPrefetch: [dml_float_t; __DML_NUM_PLANES__],
	DestinationLinesToRequestVMInVBlank: [dml_float_t; __DML_NUM_PLANES__],
	DestinationLinesToRequestRowInVBlank: [dml_float_t; __DML_NUM_PLANES__],
	DestinationLinesToRequestVMInImmediateFlip: [dml_float_t; __DML_NUM_PLANES__],
	DestinationLinesToRequestRowInImmediateFlip: [dml_float_t; __DML_NUM_PLANES__],
	MinTTUVBlank: [dml_float_t; __DML_NUM_PLANES__],
	DisplayPipeLineDeliveryTimeLuma: [dml_float_t; __DML_NUM_PLANES__],
	DisplayPipeLineDeliveryTimeChroma: [dml_float_t; __DML_NUM_PLANES__],
	DisplayPipeLineDeliveryTimeLumaPrefetch: [dml_float_t; __DML_NUM_PLANES__],
	DisplayPipeLineDeliveryTimeChromaPrefetch: [dml_float_t; __DML_NUM_PLANES__],
	DisplayPipeRequestDeliveryTimeLuma: [dml_float_t; __DML_NUM_PLANES__],
	DisplayPipeRequestDeliveryTimeChroma: [dml_float_t; __DML_NUM_PLANES__],
	DisplayPipeRequestDeliveryTimeLumaPrefetch: [dml_float_t; __DML_NUM_PLANES__],
	DisplayPipeRequestDeliveryTimeChromaPrefetch: [dml_float_t; __DML_NUM_PLANES__],
	CursorRequestDeliveryTime: [dml_float_t; __DML_NUM_PLANES__],
	CursorRequestDeliveryTimePrefetch: [dml_float_t; __DML_NUM_PLANES__],

	DST_Y_PER_PTE_ROW_NOM_L: [dml_float_t; __DML_NUM_PLANES__],
	DST_Y_PER_PTE_ROW_NOM_C: [dml_float_t; __DML_NUM_PLANES__],
	DST_Y_PER_META_ROW_NOM_L: [dml_float_t; __DML_NUM_PLANES__],
	DST_Y_PER_META_ROW_NOM_C: [dml_float_t; __DML_NUM_PLANES__],
	TimePerMetaChunkNominal: [dml_float_t; __DML_NUM_PLANES__],
	TimePerChromaMetaChunkNominal: [dml_float_t; __DML_NUM_PLANES__],
	TimePerMetaChunkVBlank: [dml_float_t; __DML_NUM_PLANES__],
	TimePerChromaMetaChunkVBlank: [dml_float_t; __DML_NUM_PLANES__],
	TimePerMetaChunkFlip: [dml_float_t; __DML_NUM_PLANES__],
	TimePerChromaMetaChunkFlip: [dml_float_t; __DML_NUM_PLANES__],
	time_per_pte_group_nom_luma: [dml_float_t; __DML_NUM_PLANES__],
	time_per_pte_group_nom_chroma: [dml_float_t; __DML_NUM_PLANES__],
	time_per_pte_group_vblank_luma: [dml_float_t; __DML_NUM_PLANES__],
	time_per_pte_group_vblank_chroma: [dml_float_t; __DML_NUM_PLANES__],
	time_per_pte_group_flip_luma: [dml_float_t; __DML_NUM_PLANES__],
	time_per_pte_group_flip_chroma: [dml_float_t; __DML_NUM_PLANES__],
	TimePerVMGroupVBlank: [dml_float_t; __DML_NUM_PLANES__],
	TimePerVMGroupFlip: [dml_float_t; __DML_NUM_PLANES__],
	TimePerVMRequestVBlank: [dml_float_t; __DML_NUM_PLANES__],
	TimePerVMRequestFlip: [dml_float_t; __DML_NUM_PLANES__],

	FractionOfUrgentBandwidth: dml_float_t,
	FractionOfUrgentBandwidthImmediateFlip: dml_float_t,

	// RQ registers
	PTE_BUFFER_MODE: [dml_bool_t; __DML_NUM_PLANES__],
	BIGK_FRAGMENT_SIZE: [dml_uint_t; __DML_NUM_PLANES__],

	SubViewportLinesNeededInMALL: [dml_uint_t; __DML_NUM_PLANES__],
	UsesMALLForStaticScreen: [dml_bool_t; __DML_NUM_PLANES__],

	// OTG
	VStartupMin: [dml_uint_t; __DML_NUM_PLANES__], /// <brief Minimum vstartup to meet the prefetch schedule (i.e. the prefetch solution can be found at this vstartup time); not the actual global sync vstartup pos.
	VStartup: [dml_uint_t; __DML_NUM_PLANES__], /// <brief The vstartup value for OTG programming (will set to max vstartup; but now bounded by min(vblank_nom. actual vblank))
	VUpdateOffsetPix: [dml_uint_t; __DML_NUM_PLANES__],
	VUpdateWidthPix: [dml_uint_t; __DML_NUM_PLANES__],
	VReadyOffsetPix: [dml_uint_t; __DML_NUM_PLANES__],

	// Latency and Support
	MaxActiveFCLKChangeLatencySupported: dml_float_t,
	USRRetrainingSupport: dml_bool_t,
	FCLKChangeSupport: enum dml_fclock_change_support,
	DRAMClockChangeSupport: enum dml_dram_clock_change_support,
	MaxActiveDRAMClockChangeLatencySupported: [dml_float_t; __DML_NUM_PLANES__],
	WritebackAllowFCLKChangeEndPosition: [dml_float_t; __DML_NUM_PLANES__],
	WritebackAllowDRAMClockChangeEndPosition: [dml_float_t; __DML_NUM_PLANES__],

	// buffer sizing
	DETBufferSizeInKByte: [dml_uint_t; __DML_NUM_PLANES__],  // <brief Recommended DET size configuration for this plane.  All pipes under this plane should program the DET buffer size to the calculated value.
	DETBufferSizeY: [dml_uint_t; __DML_NUM_PLANES__],
	DETBufferSizeC: [dml_uint_t; __DML_NUM_PLANES__],
	SwathHeightY: [dml_uint_t; __DML_NUM_PLANES__],
	SwathHeightC: [dml_uint_t; __DML_NUM_PLANES__]
}; // mode_program_st

#[repr(C)]
pub struct soc_states_st {
	num_states: dml_uint_t, /// <brief num of soc pwr states
	state_array: [struct soc_state_bounding_box_st; __DML_MAX_STATE_ARRAY_SIZE__], /// <brief fixed size array that holds states struct
},
#[repr(C)]
pub struct UseMinimumDCFCLK_params_st {
	enum dml_use_mall_for_pstate_change_mode *UseMALLForPStateChange,
	dml_bool_t *DRRDisplay,
	SynchronizeDRRDisplaysForUCLKPStateChangeFinal: dml_bool_t,
	MaxInterDCNTileRepeaters: dml_uint_t,
	MaxPrefetchMode: dml_uint_t,
	DRAMClockChangeLatencyFinal: dml_float_t,
	FCLKChangeLatency: dml_float_t,
	SREnterPlusExitTime: dml_float_t,
	ReturnBusWidth: dml_uint_t,
	RoundTripPingLatencyCycles: dml_uint_t,
	ReorderingBytes: dml_uint_t,
	PixelChunkSizeInKByte: dml_uint_t,
	MetaChunkSize: dml_uint_t,
	GPUVMEnable: dml_bool_t,
	GPUVMMaxPageTableLevels: dml_uint_t,
	HostVMEnable: dml_bool_t,
	NumberOfActiveSurfaces: dml_uint_t,
	HostVMMinPageSize: dml_uint_t,
	HostVMMaxNonCachedPageTableLevels: dml_uint_t,
	DynamicMetadataVMEnabled: dml_bool_t,
	ImmediateFlipRequirement: dml_bool_t,
	ProgressiveToInterlaceUnitInOPP: dml_bool_t,
	MaxAveragePercentOfIdealSDPPortBWDisplayCanUseInNormalSystemOperation: dml_float_t,
	PercentOfIdealSDPPortBWReceivedAfterUrgLatency: dml_float_t,
	dml_uint_t *VTotal,
	dml_uint_t *VActive,
	dml_uint_t *DynamicMetadataTransmittedBytes,
	dml_uint_t *DynamicMetadataLinesBeforeActiveRequired,
	dml_bool_t *Interlace,
	RequiredDPPCLKPerSurface: *mut [dml_float_t; __DML_NUM_PLANES__],
	dml_float_t *RequiredDISPCLK,
	UrgLatency: dml_float_t,
	NoOfDPP: *mut [dml_uint_t; __DML_NUM_PLANES__],
	dml_float_t *ProjectedDCFCLKDeepSleep,
	MaximumVStartup: *mut [dml_uint_t; __DML_NUM_PLANES__],
	dml_uint_t *TotalNumberOfActiveDPP,
	dml_uint_t *TotalNumberOfDCCActiveDPP,
	dml_uint_t *dpte_group_bytes,
	PrefetchLinesY: *mut [dml_uint_t; __DML_NUM_PLANES__],
	PrefetchLinesC: *mut [dml_uint_t; __DML_NUM_PLANES__],
	swath_width_luma_ub_all_states: *mut [dml_uint_t; __DML_NUM_PLANES__],
	swath_width_chroma_ub_all_states: *mut [dml_uint_t; __DML_NUM_PLANES__],
	dml_uint_t *BytePerPixelY,
	dml_uint_t *BytePerPixelC,
	dml_uint_t *HTotal,
	dml_float_t *PixelClock,
	PDEAndMetaPTEBytesPerFrame: *mut [dml_uint_t; __DML_NUM_PLANES__],
	DPTEBytesPerRow: *mut [dml_uint_t; __DML_NUM_PLANES__],
	MetaRowBytes: *mut [dml_uint_t; __DML_NUM_PLANES__],
	dml_bool_t *DynamicMetadataEnable,
	dml_float_t *ReadBandwidthLuma,
	dml_float_t *ReadBandwidthChroma,
	DCFCLKPerState: dml_float_t,
	dml_float_t *DCFCLKState
},
#[repr(C)]
pub struct CalculateWatermarksMALLUseAndDRAMSpeedChangeSupport_params_st {
	USRRetrainingRequiredFinal: dml_bool_t,
	enum dml_use_mall_for_pstate_change_mode *UseMALLForPStateChange,
	dml_uint_t *PrefetchMode,
	NumberOfActiveSurfaces: dml_uint_t,
	MaxLineBufferLines: dml_uint_t,
	LineBufferSize: dml_uint_t,
	WritebackInterfaceBufferSize: dml_uint_t,
	DCFCLK: dml_float_t,
	ReturnBW: dml_float_t,
	SynchronizeTimingsFinal: dml_bool_t,
	SynchronizeDRRDisplaysForUCLKPStateChangeFinal: dml_bool_t,
	dml_bool_t *DRRDisplay,
	dml_uint_t *dpte_group_bytes,
	dml_uint_t *meta_row_height,
	dml_uint_t *meta_row_height_chroma,
	mmSOCParameters: struct SOCParametersList,
	WritebackChunkSize: dml_uint_t,
	SOCCLK: dml_float_t,
	DCFClkDeepSleep: dml_float_t,
	dml_uint_t *DETBufferSizeY,
	dml_uint_t *DETBufferSizeC,
	dml_uint_t *SwathHeightY,
	dml_uint_t *SwathHeightC,
	dml_uint_t *LBBitPerPixel,
	dml_uint_t *SwathWidthY,
	dml_uint_t *SwathWidthC,
	dml_float_t *HRatio,
	dml_float_t *HRatioChroma,
	dml_uint_t *VTaps,
	dml_uint_t *VTapsChroma,
	dml_float_t *VRatio,
	dml_float_t *VRatioChroma,
	dml_uint_t *HTotal,
	dml_uint_t *VTotal,
	dml_uint_t *VActive,
	dml_float_t *PixelClock,
	dml_uint_t *BlendingAndTiming,
	dml_uint_t *DPPPerSurface,
	dml_float_t *BytePerPixelDETY,
	dml_float_t *BytePerPixelDETC,
	dml_uint_t *DSTXAfterScaler,
	dml_uint_t *DSTYAfterScaler,
	dml_bool_t *WritebackEnable,
	enum dml_source_format_class *WritebackPixelFormat,
	dml_uint_t *WritebackDestinationWidth,
	dml_uint_t *WritebackDestinationHeight,
	dml_uint_t *WritebackSourceHeight,
	UnboundedRequestEnabled: dml_bool_t,
	CompressedBufferSizeInkByte: dml_uint_t,

	// Output
	Watermark: *mut Watermarks,
	enum dml_dram_clock_change_support *DRAMClockChangeSupport,
	dml_float_t *MaxActiveDRAMClockChangeLatencySupported,
	dml_uint_t *SubViewportLinesNeededInMALL,
	enum dml_fclock_change_support *FCLKChangeSupport,
	dml_float_t *MaxActiveFCLKChangeLatencySupported,
	dml_bool_t *USRRetrainingSupport,
	dml_float_t *ActiveDRAMClockChangeLatencyMargin
},
#[repr(C)]
pub struct CalculateVMRowAndSwath_params_st {
	NumberOfActiveSurfaces: dml_uint_t,
	myPipe: *mut DmlPipe,
	dml_uint_t *SurfaceSizeInMALL,
	PTEBufferSizeInRequestsLuma: dml_uint_t,
	PTEBufferSizeInRequestsChroma: dml_uint_t,
	DCCMetaBufferSizeBytes: dml_uint_t,
	enum dml_use_mall_for_static_screen_mode *UseMALLForStaticScreen,
	enum dml_use_mall_for_pstate_change_mode *UseMALLForPStateChange,
	MALLAllocatedForDCN: dml_uint_t,
	dml_uint_t *SwathWidthY,
	dml_uint_t *SwathWidthC,
	GPUVMEnable: dml_bool_t,
	HostVMEnable: dml_bool_t,
	HostVMMaxNonCachedPageTableLevels: dml_uint_t,
	GPUVMMaxPageTableLevels: dml_uint_t,
	dml_uint_t *GPUVMMinPageSizeKBytes,
	HostVMMinPageSize: dml_uint_t,
	dml_bool_t *PTEBufferModeOverrideEn,
	dml_bool_t *PTEBufferModeOverrideVal,
	// Output
	dml_bool_t *PTEBufferSizeNotExceeded,
	dml_bool_t *DCCMetaBufferSizeNotExceeded,
	dml_uint_t *dpte_row_width_luma_ub,
	dml_uint_t *dpte_row_width_chroma_ub,
	dml_uint_t *dpte_row_height_luma,
	dml_uint_t *dpte_row_height_chroma,
	dml_uint_t *dpte_row_height_linear_luma; // VBA_DELTA
	dml_uint_t *dpte_row_height_linear_chroma; // VBA_DELTA
	dml_uint_t *meta_req_width,
	dml_uint_t *meta_req_width_chroma,
	dml_uint_t *meta_req_height,
	dml_uint_t *meta_req_height_chroma,
	dml_uint_t *meta_row_width,
	dml_uint_t *meta_row_width_chroma,
	dml_uint_t *meta_row_height,
	dml_uint_t *meta_row_height_chroma,
	dml_uint_t *vm_group_bytes,
	dml_uint_t *dpte_group_bytes,
	dml_uint_t *PixelPTEReqWidthY,
	dml_uint_t *PixelPTEReqHeightY,
	dml_uint_t *PTERequestSizeY,
	dml_uint_t *PixelPTEReqWidthC,
	dml_uint_t *PixelPTEReqHeightC,
	dml_uint_t *PTERequestSizeC,
	dml_uint_t *dpde0_bytes_per_frame_ub_l,
	dml_uint_t *meta_pte_bytes_per_frame_ub_l,
	dml_uint_t *dpde0_bytes_per_frame_ub_c,
	dml_uint_t *meta_pte_bytes_per_frame_ub_c,
	dml_uint_t *PrefetchSourceLinesY,
	dml_uint_t *PrefetchSourceLinesC,
	dml_uint_t *VInitPreFillY,
	dml_uint_t *VInitPreFillC,
	dml_uint_t *MaxNumSwathY,
	dml_uint_t *MaxNumSwathC,
	dml_float_t *meta_row_bw,
	dml_float_t *dpte_row_bw,
	dml_uint_t *PixelPTEBytesPerRow,
	dml_uint_t *PDEAndMetaPTEBytesFrame,
	dml_uint_t *MetaRowByte,
	dml_bool_t *use_one_row_for_frame,
	dml_bool_t *use_one_row_for_frame_flip,
	dml_bool_t *UsesMALLForStaticScreen,
	dml_bool_t *PTE_BUFFER_MODE,
	dml_uint_t *BIGK_FRAGMENT_SIZE
},
#[repr(C)]
pub struct CalculateSwathAndDETConfiguration_params_st {
	dml_uint_t *DETSizeOverride,
	enum dml_use_mall_for_pstate_change_mode *UseMALLForPStateChange,
	ConfigReturnBufferSizeInKByte: dml_uint_t,
	ROBBufferSizeInKByte: dml_uint_t,
	MaxTotalDETInKByte: dml_uint_t,
	MinCompressedBufferSizeInKByte: dml_uint_t,
	PixelChunkSizeInKByte: dml_uint_t,
	ForceSingleDPP: dml_bool_t,
	NumberOfActiveSurfaces: dml_uint_t,
	nomDETInKByte: dml_uint_t,
	UseUnboundedRequestingFinal: enum dml_unbounded_requesting_policy,
	ConfigReturnBufferSegmentSizeInkByte: dml_uint_t,
	CompressedBufferSegmentSizeInkByteFinal: dml_uint_t,
	enum dml_output_encoder_class *Output,
	dml_float_t *ReadBandwidthLuma,
	dml_float_t *ReadBandwidthChroma,
	dml_float_t *MaximumSwathWidthLuma,
	dml_float_t *MaximumSwathWidthChroma,
	enum dml_rotation_angle *SourceScan,
	dml_bool_t *ViewportStationary,
	enum dml_source_format_class *SourcePixelFormat,
	enum dml_swizzle_mode *SurfaceTiling,
	dml_uint_t *ViewportWidth,
	dml_uint_t *ViewportHeight,
	dml_uint_t *ViewportXStart,
	dml_uint_t *ViewportYStart,
	dml_uint_t *ViewportXStartC,
	dml_uint_t *ViewportYStartC,
	dml_uint_t *SurfaceWidthY,
	dml_uint_t *SurfaceWidthC,
	dml_uint_t *SurfaceHeightY,
	dml_uint_t *SurfaceHeightC,
	dml_uint_t *Read256BytesBlockHeightY,
	dml_uint_t *Read256BytesBlockHeightC,
	dml_uint_t *Read256BytesBlockWidthY,
	dml_uint_t *Read256BytesBlockWidthC,
	enum dml_odm_mode *ODMMode,
	dml_uint_t *BlendingAndTiming,
	dml_uint_t *BytePerPixY,
	dml_uint_t *BytePerPixC,
	dml_float_t *BytePerPixDETY,
	dml_float_t *BytePerPixDETC,
	dml_uint_t *HActive,
	dml_float_t *HRatio,
	dml_float_t *HRatioChroma,
	dml_uint_t *DPPPerSurface,
	dml_uint_t *swath_width_luma_ub,
	dml_uint_t *swath_width_chroma_ub,
	dml_uint_t *SwathWidth,
	dml_uint_t *SwathWidthChroma,
	dml_uint_t *SwathHeightY,
	dml_uint_t *SwathHeightC,
	dml_uint_t *DETBufferSizeInKByte,
	dml_uint_t *DETBufferSizeY,
	dml_uint_t *DETBufferSizeC,
	dml_bool_t *UnboundedRequestEnabled,
	dml_uint_t *compbuf_reserved_space_64b,
	dml_uint_t *compbuf_reserved_space_zs,
	dml_uint_t *CompressedBufferSizeInkByte,
	dml_bool_t *ViewportSizeSupportPerSurface,
	dml_bool_t *ViewportSizeSupport
},
#[repr(C)]
pub struct CalculateStutterEfficiency_params_st {
	CompressedBufferSizeInkByte: dml_uint_t,
	enum dml_use_mall_for_pstate_change_mode *UseMALLForPStateChange,
	UnboundedRequestEnabled: dml_bool_t,
	MetaFIFOSizeInKEntries: dml_uint_t,
	ZeroSizeBufferEntries: dml_uint_t,
	PixelChunkSizeInKByte: dml_uint_t,
	NumberOfActiveSurfaces: dml_uint_t,
	ROBBufferSizeInKByte: dml_uint_t,
	TotalDataReadBandwidth: dml_float_t,
	DCFCLK: dml_float_t,
	ReturnBW: dml_float_t,
	CompbufReservedSpace64B: dml_uint_t,
	CompbufReservedSpaceZs: dml_uint_t,
	SRExitTime: dml_float_t,
	SRExitZ8Time: dml_float_t,
	SynchronizeTimingsFinal: dml_bool_t,
	dml_uint_t *BlendingAndTiming,
	StutterEnterPlusExitWatermark: dml_float_t,
	Z8StutterEnterPlusExitWatermark: dml_float_t,
	ProgressiveToInterlaceUnitInOPP: dml_bool_t,
	dml_bool_t *Interlace,
	dml_float_t *MinTTUVBlank,
	dml_uint_t *DPPPerSurface,
	dml_uint_t *DETBufferSizeY,
	dml_uint_t *BytePerPixelY,
	dml_float_t *BytePerPixelDETY,
	dml_uint_t *SwathWidthY,
	dml_uint_t *SwathHeightY,
	dml_uint_t *SwathHeightC,
	dml_float_t *NetDCCRateLuma,
	dml_float_t *NetDCCRateChroma,
	dml_float_t *DCCFractionOfZeroSizeRequestsLuma,
	dml_float_t *DCCFractionOfZeroSizeRequestsChroma,
	dml_uint_t *HTotal,
	dml_uint_t *VTotal,
	dml_float_t *PixelClock,
	dml_float_t *VRatio,
	enum dml_rotation_angle *SourceScan,
	dml_uint_t *BlockHeight256BytesY,
	dml_uint_t *BlockWidth256BytesY,
	dml_uint_t *BlockHeight256BytesC,
	dml_uint_t *BlockWidth256BytesC,
	dml_uint_t *DCCYMaxUncompressedBlock,
	dml_uint_t *DCCCMaxUncompressedBlock,
	dml_uint_t *VActive,
	dml_bool_t *DCCEnable,
	dml_bool_t *WritebackEnable,
	dml_float_t *ReadBandwidthSurfaceLuma,
	dml_float_t *ReadBandwidthSurfaceChroma,
	dml_float_t *meta_row_bw,
	dml_float_t *dpte_row_bw,
	dml_float_t *StutterEfficiencyNotIncludingVBlank,
	dml_float_t *StutterEfficiency,
	dml_uint_t *NumberOfStutterBurstsPerFrame,
	dml_float_t *Z8StutterEfficiencyNotIncludingVBlank,
	dml_float_t *Z8StutterEfficiency,
	dml_uint_t *Z8NumberOfStutterBurstsPerFrame,
	dml_float_t *StutterPeriod,
	dml_bool_t *DCHUBBUB_ARB_CSTATE_MAX_CAP_MODE
},
#[repr(C)]
pub struct CalculatePrefetchSchedule_params_st {
	EnhancedPrefetchScheduleAccelerationFinal: dml_bool_t,
	HostVMInefficiencyFactor: dml_float_t,
	myPipe: *mut DmlPipe,
	DSCDelay: dml_uint_t,
	DPPCLKDelaySubtotalPlusCNVCFormater: dml_float_t,
	DPPCLKDelaySCL: dml_float_t,
	DPPCLKDelaySCLLBOnly: dml_float_t,
	DPPCLKDelayCNVCCursor: dml_float_t,
	DISPCLKDelaySubtotal: dml_float_t,
	DPP_RECOUT_WIDTH: dml_uint_t,
	OutputFormat: enum dml_output_format_class,
	MaxInterDCNTileRepeaters: dml_uint_t,
	VStartup: dml_uint_t,
	MaxVStartup: dml_uint_t,
	GPUVMPageTableLevels: dml_uint_t,
	GPUVMEnable: dml_bool_t,
	HostVMEnable: dml_bool_t,
	HostVMMaxNonCachedPageTableLevels: dml_uint_t,
	HostVMMinPageSize: dml_uint_t,
	DynamicMetadataEnable: dml_bool_t,
	DynamicMetadataVMEnabled: dml_bool_t,
	DynamicMetadataLinesBeforeActiveRequired: i32,
	DynamicMetadataTransmittedBytes: dml_uint_t,
	UrgentLatency: dml_float_t,
	UrgentExtraLatency: dml_float_t,
	TCalc: dml_float_t,
	PDEAndMetaPTEBytesFrame: dml_uint_t,
	MetaRowByte: dml_uint_t,
	PixelPTEBytesPerRow: dml_uint_t,
	PrefetchSourceLinesY: dml_float_t,
	VInitPreFillY: dml_uint_t,
	MaxNumSwathY: dml_uint_t,
	PrefetchSourceLinesC: dml_float_t,
	VInitPreFillC: dml_uint_t,
	MaxNumSwathC: dml_uint_t,
	swath_width_luma_ub: dml_uint_t,
	swath_width_chroma_ub: dml_uint_t,
	SwathHeightY: dml_uint_t,
	SwathHeightC: dml_uint_t,
	TWait: dml_float_t,
	dml_uint_t *DSTXAfterScaler,
	dml_uint_t *DSTYAfterScaler,
	dml_float_t *DestinationLinesForPrefetch,
	dml_float_t *DestinationLinesToRequestVMInVBlank,
	dml_float_t *DestinationLinesToRequestRowInVBlank,
	dml_float_t *VRatioPrefetchY,
	dml_float_t *VRatioPrefetchC,
	dml_float_t *RequiredPrefetchPixDataBWLuma,
	dml_float_t *RequiredPrefetchPixDataBWChroma,
	dml_bool_t *NotEnoughTimeForDynamicMetadata,
	dml_float_t *Tno_bw,
	dml_float_t *prefetch_vmrow_bw,
	dml_float_t *Tdmdl_vm,
	dml_float_t *Tdmdl,
	dml_float_t *TSetup,
	dml_uint_t *VUpdateOffsetPix,
	dml_uint_t *VUpdateWidthPix,
	dml_uint_t *VReadyOffsetPix
},
#[repr(C)]
pub struct dml_core_mode_support_locals_st {
	dummy_boolean: [dml_bool_t; 2],
	dummy_integer: [dml_uint_t; 3],
	dml_uint_t dummy_integer_array[22][__DML_NUM_PLANES__],
	dummy_odm_mode: [enum dml_odm_mode; __DML_NUM_PLANES__],
	dml_bool_t dummy_boolean_array[2][__DML_NUM_PLANES__],
	MaxVStartupAllPlanes: [dml_uint_t; 2],
	dml_uint_t MaximumVStartup[2][__DML_NUM_PLANES__],
	DSTYAfterScaler: [dml_uint_t; __DML_NUM_PLANES__],
	DSTXAfterScaler: [dml_uint_t; __DML_NUM_PLANES__],
	NextPrefetchMode: [dml_uint_t; __DML_NUM_PLANES__],
	MinPrefetchMode: [dml_uint_t; __DML_NUM_PLANES__],
	MaxPrefetchMode: [dml_uint_t; __DML_NUM_PLANES__],
	dummy_single: [dml_float_t; 3],
	dummy_single_array: [dml_float_t; __DML_NUM_PLANES__],
	dummy_watermark: struct Watermarks,
	mSOCParameters: struct SOCParametersList,
	myPipe: struct DmlPipe,
	SurfParameters: [struct DmlPipe; __DML_NUM_PLANES__],
	TotalNumberOfActiveWriteback: dml_uint_t,
	MaximumSwathWidthSupportLuma: dml_uint_t,
	MaximumSwathWidthSupportChroma: dml_uint_t,
	MPCCombineMethodAsNeededForPStateChangeAndVoltage: dml_bool_t,
	MPCCombineMethodAsPossible: dml_bool_t,
	TotalAvailablePipesSupportNoDSC: dml_bool_t,
	NumberOfDPPNoDSC: dml_uint_t,
	ODMModeNoDSC: enum dml_odm_mode,
	RequiredDISPCLKPerSurfaceNoDSC: dml_float_t,
	TotalAvailablePipesSupportDSC: dml_bool_t,
	NumberOfDPPDSC: dml_uint_t,
	ODMModeDSC: enum dml_odm_mode,
	RequiredDISPCLKPerSurfaceDSC: dml_float_t,
	NoChromaOrLinear: dml_bool_t,
	BWOfNonCombinedSurfaceOfMaximumBandwidth: dml_float_t,
	NumberOfNonCombinedSurfaceOfMaximumBandwidth: dml_uint_t,
	TotalNumberOfActiveOTG: dml_uint_t,
	TotalNumberOfActiveHDMIFRL: dml_uint_t,
	TotalNumberOfActiveDP2p0: dml_uint_t,
	TotalNumberOfActiveDP2p0Outputs: dml_uint_t,
	TotalSlots: dml_uint_t,
	DSCFormatFactor: dml_uint_t,
	TotalDSCUnitsRequired: dml_uint_t,
	ReorderingBytes: dml_uint_t,
	ImmediateFlipRequiredFinal: dml_bool_t,
	FullFrameMALLPStateMethod: dml_bool_t,
	SubViewportMALLPStateMethod: dml_bool_t,
	PhantomPipeMALLPStateMethod: dml_bool_t,
	SubViewportMALLRefreshGreaterThan120Hz: dml_bool_t,
	MaxTotalVActiveRDBandwidth: dml_float_t,
	VMDataOnlyReturnBWPerState: dml_float_t,
	HostVMInefficiencyFactor: dml_float_t,
	NextMaxVStartup: dml_uint_t,
	MaxVStartup: dml_uint_t,
	AllPrefetchModeTested: dml_bool_t,
	AnyLinesForVMOrRowTooLarge: dml_bool_t,
	is_max_pwr_state: dml_bool_t,
	is_max_dram_pwr_state: dml_bool_t,
	dram_clock_change_support: dml_bool_t,
	f_clock_change_support: dml_bool_t
},
#[repr(C)]
pub struct dml_core_mode_programming_locals_st {
	DSCFormatFactor: dml_uint_t,
	dml_uint_t dummy_integer_array[2][__DML_NUM_PLANES__],
	dummy_output_encoder_array: [enum dml_output_encoder_class; __DML_NUM_PLANES__],
	dml_float_t dummy_single_array[2][__DML_NUM_PLANES__],
	dml_uint_t dummy_long_array[4][__DML_NUM_PLANES__],
	dml_bool_t dummy_boolean_array[2][__DML_NUM_PLANES__],
	dummy_boolean: [dml_bool_t; 1],
	SurfaceParameters: [struct DmlPipe; __DML_NUM_PLANES__],
	ReorderBytes: dml_uint_t,
	VMDataOnlyReturnBW: dml_float_t,
	HostVMInefficiencyFactor: dml_float_t,
	TotalDCCActiveDPP: dml_uint_t,
	TotalActiveDPP: dml_uint_t,
	VStartupLines: dml_uint_t,
	MaxVStartupLines: [dml_uint_t; __DML_NUM_PLANES__], /// <brief more like vblank for the plane's OTG
	MaxVStartupAllPlanes: dml_uint_t,
	ImmediateFlipRequirementFinal: dml_bool_t,
	iteration: i32,
	MaxTotalRDBandwidth: dml_float_t,
	MaxTotalRDBandwidthNoUrgentBurst: dml_float_t,
	DestinationLineTimesForPrefetchLessThan2: dml_bool_t,
	VRatioPrefetchMoreThanMax: dml_bool_t,
	MaxTotalRDBandwidthNotIncludingMALLPrefetch: dml_float_t,
	NextPrefetchMode: [dml_uint_t; __DML_NUM_PLANES__],
	MinPrefetchMode: [dml_uint_t; __DML_NUM_PLANES__],
	MaxPrefetchMode: [dml_uint_t; __DML_NUM_PLANES__],
	AllPrefetchModeTested: dml_bool_t,
	dummy_unit_vector: [dml_float_t; __DML_NUM_PLANES__],
	NonUrgentMaxTotalRDBandwidth: dml_float_t,
	NonUrgentMaxTotalRDBandwidthNotIncludingMALLPrefetch: dml_float_t,
	dummy_single: [dml_float_t; 2],
	mmSOCParameters: struct SOCParametersList,
	Tvstartup_margin: dml_float_t,
	dlg_vblank_start: dml_float_t,
	LSetup: dml_float_t,
	blank_lines_remaining: dml_float_t,
	old_MIN_DST_Y_NEXT_START: dml_float_t,
	TotalWRBandwidth: dml_float_t,
	WRBandwidth: dml_float_t,
	dummy_watermark: struct Watermarks,
	myPipe: struct DmlPipe
},
#[repr(C)]
pub struct CalculateWatermarksMALLUseAndDRAMSpeedChangeSupport_locals_st {
	ActiveDRAMClockChangeLatencyMargin: [dml_float_t; __DML_NUM_PLANES__],
	ActiveFCLKChangeLatencyMargin: [dml_float_t; __DML_NUM_PLANES__],
	USRRetrainingLatencyMargin: [dml_float_t; __DML_NUM_PLANES__],

	dml_bool_t SynchronizedSurfaces[__DML_NUM_PLANES__][__DML_NUM_PLANES__],
	EffectiveLBLatencyHidingY: dml_float_t,
	EffectiveLBLatencyHidingC: dml_float_t,
	LinesInDETY: [dml_float_t; __DML_NUM_PLANES__],
	LinesInDETC: [dml_float_t; __DML_NUM_PLANES__],
	LinesInDETYRoundedDownToSwath: [dml_uint_t; __DML_NUM_PLANES__],
	LinesInDETCRoundedDownToSwath: [dml_uint_t; __DML_NUM_PLANES__],
	FullDETBufferingTimeY: dml_float_t,
	FullDETBufferingTimeC: dml_float_t,
	WritebackDRAMClockChangeLatencyMargin: dml_float_t,
	WritebackFCLKChangeLatencyMargin: dml_float_t,
	WritebackLatencyHiding: dml_float_t,

	TotalActiveWriteback: dml_uint_t,
	LBLatencyHidingSourceLinesY: [dml_uint_t; __DML_NUM_PLANES__],
	LBLatencyHidingSourceLinesC: [dml_uint_t; __DML_NUM_PLANES__],
	TotalPixelBW: dml_float_t,
	EffectiveDETBufferSizeY: dml_float_t,
	ActiveClockChangeLatencyHidingY: dml_float_t,
	ActiveClockChangeLatencyHidingC: dml_float_t,
	ActiveClockChangeLatencyHiding: dml_float_t,
	FoundCriticalSurface: dml_bool_t,
	LastSurfaceWithoutMargin: dml_uint_t,
	FCLKChangeSupportNumber: dml_uint_t,
	DRAMClockChangeMethod: dml_uint_t,
	DRAMClockChangeSupportNumber: dml_uint_t,
	dst_y_pstate: dml_uint_t,
	src_y_pstate_l: dml_uint_t,
	src_y_pstate_c: dml_uint_t,
	src_y_ahead_l: dml_uint_t,
	src_y_ahead_c: dml_uint_t,
	sub_vp_lines_l: dml_uint_t,
	sub_vp_lines_c: dml_uint_t
},
#[repr(C)]
pub struct CalculateVMRowAndSwath_locals_st {
	PTEBufferSizeInRequestsForLuma: [dml_uint_t; __DML_NUM_PLANES__],
	PTEBufferSizeInRequestsForChroma: [dml_uint_t; __DML_NUM_PLANES__],
	PDEAndMetaPTEBytesFrameY: dml_uint_t,
	PDEAndMetaPTEBytesFrameC: dml_uint_t,
	MetaRowByteY: [dml_uint_t; __DML_NUM_PLANES__],
	MetaRowByteC: [dml_uint_t; __DML_NUM_PLANES__],
	PixelPTEBytesPerRowY: [dml_uint_t; __DML_NUM_PLANES__],
	PixelPTEBytesPerRowC: [dml_uint_t; __DML_NUM_PLANES__],
	PixelPTEBytesPerRowStorageY: [dml_uint_t; __DML_NUM_PLANES__],
	PixelPTEBytesPerRowStorageC: [dml_uint_t; __DML_NUM_PLANES__],
	PixelPTEBytesPerRowY_one_row_per_frame: [dml_uint_t; __DML_NUM_PLANES__],
	PixelPTEBytesPerRowC_one_row_per_frame: [dml_uint_t; __DML_NUM_PLANES__],
	dpte_row_width_luma_ub_one_row_per_frame: [dml_uint_t; __DML_NUM_PLANES__],
	dpte_row_height_luma_one_row_per_frame: [dml_uint_t; __DML_NUM_PLANES__],
	dpte_row_width_chroma_ub_one_row_per_frame: [dml_uint_t; __DML_NUM_PLANES__],
	dpte_row_height_chroma_one_row_per_frame: [dml_uint_t; __DML_NUM_PLANES__],
	one_row_per_frame_fits_in_buffer: [dml_bool_t; __DML_NUM_PLANES__],

	HostVMDynamicLevels: dml_uint_t
},
#[repr(C)]
pub struct UseMinimumDCFCLK_locals_st {
	dummy1: dml_uint_t,
	dummy2: dml_uint_t,
	dummy3: dml_uint_t,
	NormalEfficiency: dml_float_t,
	TotalMaxPrefetchFlipDPTERowBandwidth: [dml_float_t; 2],

	PixelDCFCLKCyclesRequiredInPrefetch: [dml_float_t; __DML_NUM_PLANES__],
	PrefetchPixelLinesTime: [dml_float_t; __DML_NUM_PLANES__],
	DCFCLKRequiredForPeakBandwidthPerSurface: [dml_float_t; __DML_NUM_PLANES__],
	DynamicMetadataVMExtraLatency: [dml_float_t; __DML_NUM_PLANES__],
	MinimumTWait: dml_float_t,
	DPTEBandwidth: dml_float_t,
	DCFCLKRequiredForAverageBandwidth: dml_float_t,
	ExtraLatencyBytes: dml_uint_t,
	ExtraLatencyCycles: dml_float_t,
	DCFCLKRequiredForPeakBandwidth: dml_float_t,
	NoOfDPPState: [dml_uint_t; __DML_NUM_PLANES__],
	MinimumTvmPlus2Tr0: dml_float_t
},
#[repr(C)]
pub struct CalculatePrefetchSchedule_locals_st {
	MyError: dml_bool_t,
	DPPCycles: dml_uint_t,
	DISPCLKCycles: dml_uint_t,
	DSTTotalPixelsAfterScaler: dml_float_t,
	LineTime: dml_float_t,
	dst_y_prefetch_equ: dml_float_t,
	prefetch_bw_oto: dml_float_t,
	Tvm_oto: dml_float_t,
	Tr0_oto: dml_float_t,
	Tvm_oto_lines: dml_float_t,
	Tr0_oto_lines: dml_float_t,
	dst_y_prefetch_oto: dml_float_t,
	TimeForFetchingMetaPTE: dml_float_t,
	TimeForFetchingRowInVBlank: dml_float_t,
	LinesToRequestPrefetchPixelData: dml_float_t,
	HostVMDynamicLevelsTrips: dml_uint_t,
	trip_to_mem: dml_float_t,
	Tvm_trips: dml_float_t,
	Tr0_trips: dml_float_t,
	Tvm_trips_rounded: dml_float_t,
	Tr0_trips_rounded: dml_float_t,
	max_Tsw: dml_float_t,
	Lsw_oto: dml_float_t,
	Tpre_rounded: dml_float_t,
	prefetch_bw_equ: dml_float_t,
	Tvm_equ: dml_float_t,
	Tr0_equ: dml_float_t,
	Tdmbf: dml_float_t,
	Tdmec: dml_float_t,
	Tdmsks: dml_float_t,
	prefetch_sw_bytes: dml_float_t,
	prefetch_bw_pr: dml_float_t,
	bytes_pp: dml_float_t,
	dep_bytes: dml_float_t,
	min_Lsw_oto: dml_float_t,
	Tsw_est1: dml_float_t,
	Tsw_est3: dml_float_t,
	PrefetchBandwidth1: dml_float_t,
	PrefetchBandwidth2: dml_float_t,
	PrefetchBandwidth3: dml_float_t,
	PrefetchBandwidth4: dml_float_t
},
/// @brief To minimize stack usage; function locals are instead placed into this scratch structure which is allocated per context
#[repr(C)]
pub struct display_mode_lib_scratch_st {
	// Scratch space for function locals
	dml_core_mode_support_locals: struct dml_core_mode_support_locals_st,
	dml_core_mode_programming_locals: struct dml_core_mode_programming_locals_st,
	CalculateWatermarksMALLUseAndDRAMSpeedChangeSupport_locals: struct CalculateWatermarksMALLUseAndDRAMSpeedChangeSupport_locals_st,
	CalculateVMRowAndSwath_locals: struct CalculateVMRowAndSwath_locals_st,
	UseMinimumDCFCLK_locals: struct UseMinimumDCFCLK_locals_st,
	CalculatePrefetchSchedule_locals: struct CalculatePrefetchSchedule_locals_st,

	// Scratch space for function params
	CalculateWatermarksMALLUseAndDRAMSpeedChangeSupport_params: struct CalculateWatermarksMALLUseAndDRAMSpeedChangeSupport_params_st,
	CalculateVMRowAndSwath_params: struct CalculateVMRowAndSwath_params_st,
	UseMinimumDCFCLK_params: struct UseMinimumDCFCLK_params_st,
	CalculateSwathAndDETConfiguration_params: struct CalculateSwathAndDETConfiguration_params_st,
	CalculateStutterEfficiency_params: struct CalculateStutterEfficiency_params_st,
	CalculatePrefetchSchedule_params: struct CalculatePrefetchSchedule_params_st
},
/// @brief Represent the overall soc/ip environment. It contains data structure represent the soc/ip characteristic and also structures that hold calculation output
#[repr(C)]
pub struct display_mode_lib_st {
	project: dml_uint_t,

	//@brief Mode evaluation and programming policy
	policy: struct dml_mode_eval_policy_st,

	//@brief IP/SOC characteristic
	ip: struct ip_params_st,
	soc: struct soc_bounding_box_st,
	states: struct soc_states_st,

	//@brief Mode Support and Mode programming struct
	// Used to hold input; intermediate and output of the calculations
	ms: struct mode_support_st, // struct for mode support
	mp: struct mode_program_st, // struct for mode programming

	scratch: struct display_mode_lib_scratch_st
},
#[repr(C)]
pub struct dml_mode_support_ex_params_st {
	mode_lib: *mut display_mode_lib_st,
	in_display_cfg: *const dml_display_cfg_st,
	in_start_state_idx: dml_uint_t,
	out_lowest_state_idx: dml_uint_t,
	out_evaluation_info: *mut dml_mode_support_info_st
},
pub type dml_display_rq_regs_st = _vcs_dpi_dml_display_rq_regs_st,
pub type dml_display_dlg_regs_st = _vcs_dpi_dml_display_dlg_regs_st,
pub type dml_display_ttu_regs_st = _vcs_dpi_dml_display_ttu_regs_st,
pub type dml_display_arb_params_st = _vcs_dpi_dml_display_arb_params_st,
pub type dml_display_plane_rq_regs_st = _vcs_dpi_dml_display_plane_rq_regs_st,
#[repr(C)]
pub struct _vcs_dpi_dml_display_dlg_regs_st {
	refcyc_h_blank_end: dml_uint_t,
	dlg_vblank_end: dml_uint_t,
	min_dst_y_next_start: dml_uint_t,
	refcyc_per_htotal: dml_uint_t,
	refcyc_x_after_scaler: dml_uint_t,
	dst_y_after_scaler: dml_uint_t,
	dst_y_prefetch: dml_uint_t,
	dst_y_per_vm_vblank: dml_uint_t,
	dst_y_per_row_vblank: dml_uint_t,
	dst_y_per_vm_flip: dml_uint_t,
	dst_y_per_row_flip: dml_uint_t,
	ref_freq_to_pix_freq: dml_uint_t,
	vratio_prefetch: dml_uint_t,
	vratio_prefetch_c: dml_uint_t,
	refcyc_per_pte_group_vblank_l: dml_uint_t,
	refcyc_per_pte_group_vblank_c: dml_uint_t,
	refcyc_per_meta_chunk_vblank_l: dml_uint_t,
	refcyc_per_meta_chunk_vblank_c: dml_uint_t,
	refcyc_per_pte_group_flip_l: dml_uint_t,
	refcyc_per_pte_group_flip_c: dml_uint_t,
	refcyc_per_meta_chunk_flip_l: dml_uint_t,
	refcyc_per_meta_chunk_flip_c: dml_uint_t,
	dst_y_per_pte_row_nom_l: dml_uint_t,
	dst_y_per_pte_row_nom_c: dml_uint_t,
	refcyc_per_pte_group_nom_l: dml_uint_t,
	refcyc_per_pte_group_nom_c: dml_uint_t,
	dst_y_per_meta_row_nom_l: dml_uint_t,
	dst_y_per_meta_row_nom_c: dml_uint_t,
	refcyc_per_meta_chunk_nom_l: dml_uint_t,
	refcyc_per_meta_chunk_nom_c: dml_uint_t,
	refcyc_per_line_delivery_pre_l: dml_uint_t,
	refcyc_per_line_delivery_pre_c: dml_uint_t,
	refcyc_per_line_delivery_l: dml_uint_t,
	refcyc_per_line_delivery_c: dml_uint_t,
	refcyc_per_vm_group_vblank: dml_uint_t,
	refcyc_per_vm_group_flip: dml_uint_t,
	refcyc_per_vm_req_vblank: dml_uint_t,
	refcyc_per_vm_req_flip: dml_uint_t,
	dst_y_offset_cur0: dml_uint_t,
	chunk_hdl_adjust_cur0: dml_uint_t,
	dst_y_offset_cur1: dml_uint_t,
	chunk_hdl_adjust_cur1: dml_uint_t,
	vready_after_vcount0: dml_uint_t,
	dst_y_delta_drq_limit: dml_uint_t,
	refcyc_per_vm_dmdata: dml_uint_t,
	dmdata_dl_delta: dml_uint_t
},
#[repr(C)]
pub struct _vcs_dpi_dml_display_ttu_regs_st {
	qos_level_low_wm: dml_uint_t,
	qos_level_high_wm: dml_uint_t,
	min_ttu_vblank: dml_uint_t,
	qos_level_flip: dml_uint_t,
	refcyc_per_req_delivery_l: dml_uint_t,
	refcyc_per_req_delivery_c: dml_uint_t,
	refcyc_per_req_delivery_cur0: dml_uint_t,
	refcyc_per_req_delivery_cur1: dml_uint_t,
	refcyc_per_req_delivery_pre_l: dml_uint_t,
	refcyc_per_req_delivery_pre_c: dml_uint_t,
	refcyc_per_req_delivery_pre_cur0: dml_uint_t,
	refcyc_per_req_delivery_pre_cur1: dml_uint_t,
	qos_level_fixed_l: dml_uint_t,
	qos_level_fixed_c: dml_uint_t,
	qos_level_fixed_cur0: dml_uint_t,
	qos_level_fixed_cur1: dml_uint_t,
	qos_ramp_disable_l: dml_uint_t,
	qos_ramp_disable_c: dml_uint_t,
	qos_ramp_disable_cur0: dml_uint_t,
	qos_ramp_disable_cur1: dml_uint_t
},
#[repr(C)]
pub struct _vcs_dpi_dml_display_arb_params_st {
	max_req_outstanding: dml_uint_t,
	min_req_outstanding: dml_uint_t,
	sat_level_us: dml_uint_t,
	hvm_max_qos_commit_threshold: dml_uint_t,
	hvm_min_req_outstand_commit_threshold: dml_uint_t,
	compbuf_reserved_space_kbytes: dml_uint_t
},
#[repr(C)]
pub struct _vcs_dpi_dml_display_plane_rq_regs_st {
	chunk_size: dml_uint_t,
	min_chunk_size: dml_uint_t,
	meta_chunk_size: dml_uint_t,
	min_meta_chunk_size: dml_uint_t,
	dpte_group_size: dml_uint_t,
	mpte_group_size: dml_uint_t,
	swath_height: dml_uint_t,
	pte_row_height_linear: dml_uint_t
},
#[repr(C)]
pub struct _vcs_dpi_dml_display_rq_regs_st {
	dml_display_plane_rq_regs_st    rq_regs_l,
	dml_display_plane_rq_regs_st    rq_regs_c,
	drq_expansion_mode: dml_uint_t,
	prq_expansion_mode: dml_uint_t,
	mrq_expansion_mode: dml_uint_t,
	crq_expansion_mode: dml_uint_t,
	plane1_base_address: dml_uint_t
},

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
