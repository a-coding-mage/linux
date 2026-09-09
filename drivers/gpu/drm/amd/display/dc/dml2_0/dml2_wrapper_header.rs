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
 */

// Dependency declarations supplied by the surrounding translation unit.
pub const DML2_MAX_NUM_DPM_LVL: usize = 30;

#[repr(C)] pub struct dml2_context { _private: [u8; 0] }
#[repr(C)] pub struct display_mode_lib_st { _private: [u8; 0] }
#[repr(C)] pub struct dc { _private: [u8; 0] }
#[repr(C)] pub struct pipe_ctx { _private: [u8; 0] }
#[repr(C)] pub struct dc_plane_state { _private: [u8; 0] }
#[repr(C)] pub struct dc_sink { _private: [u8; 0] }
#[repr(C)] pub struct dc_stream_state { _private: [u8; 0] }
#[repr(C)] pub struct resource_context { _private: [u8; 0] }
#[repr(C)] pub struct display_stream_compressor { _private: [u8; 0] }
#[repr(C)] pub struct dc_mcache_params { _private: [u8; 0] }
#[repr(C)] pub struct dc_state { _private: [u8; 0] }
#[repr(C)] pub struct resource_pool { _private: [u8; 0] }
#[repr(C)] pub struct dc_stream_status { _private: [u8; 0] }
#[repr(C)] pub struct socbb_ip_params_external { _private: [u8; 0] }
pub type dc_status = i32;
pub type mall_stream_type = i32;
pub type dc_validate_mode = i32;
// MAX_PIPES is supplied by the surrounding dependencies.

#[repr(C)]
pub struct dml2_soc_mall_info {
    // Cache line size of 0 means MALL is not enabled/present
    pub cache_line_size_bytes: u32,
    pub cache_num_ways: u32,
    pub max_cab_allocation_bytes: u32,
    pub mblk_width_pixels: u32,
    pub mblk_size_bytes: u32,
    pub mblk_height_4bpe_pixels: u32,
    pub mblk_height_8bpe_pixels: u32,
}

#[repr(C)]
pub struct dml2_soc_alt_ch_info {
    pub region_size_bytes: [u32; 2],
    /* bits 47:16 of the base address */
    pub region_base_addr_47_16: [u32; 2],
}

#[repr(C)]
pub struct dml2_dcn_clocks {
    pub dispclk_khz: u32, pub dcfclk_khz: u32, pub fclk_khz: u32, pub uclk_mts: u32,
    pub phyclk_khz: u32, pub socclk_khz: u32, pub ref_dtbclk_khz: u32,
    pub p_state_supported: bool, pub cab_num_ways_required: u32, pub dcfclk_khz_ds: u32,
}

#[repr(C)]
pub struct dml2_dc_callbacks {
    pub dc: *mut dc,
    pub build_scaling_params: Option<unsafe extern "C" fn(*mut pipe_ctx) -> bool>,
    pub build_test_pattern_params: Option<unsafe extern "C" fn(*mut resource_context, *mut pipe_ctx)>,
    pub can_support_mclk_switch_using_fw_based_vblank_stretch: Option<unsafe extern "C" fn(*mut dc, *mut dc_state) -> bool>,
    pub acquire_secondary_pipe_for_mpc_odm: Option<unsafe extern "C" fn(*const dc, *mut dc_state, *mut pipe_ctx, *mut pipe_ctx, bool) -> bool>,
    pub update_pipes_for_stream_with_slice_count: Option<unsafe extern "C" fn(*mut dc_state, *const dc_state, *const resource_pool, *const dc_stream_state, i32) -> bool>,
    pub update_pipes_for_plane_with_slice_count: Option<unsafe extern "C" fn(*mut dc_state, *const dc_state, *const resource_pool, *const dc_plane_state, i32) -> bool>,
    pub get_odm_slice_index: Option<unsafe extern "C" fn(*const pipe_ctx) -> i32>,
    pub get_odm_slice_count: Option<unsafe extern "C" fn(*const pipe_ctx) -> i32>,
    pub get_mpc_slice_index: Option<unsafe extern "C" fn(*const pipe_ctx) -> i32>,
    pub get_mpc_slice_count: Option<unsafe extern "C" fn(*const pipe_ctx) -> i32>,
    pub get_opp_head: Option<unsafe extern "C" fn(*const pipe_ctx) -> *mut pipe_ctx>,
    pub get_otg_master_for_stream: Option<unsafe extern "C" fn(*mut resource_context, *const dc_stream_state) -> *mut pipe_ctx>,
    pub get_opp_heads_for_otg_master: Option<unsafe extern "C" fn(*const pipe_ctx, *mut resource_context, *mut pipe_ctx) -> i32>,
    pub get_dpp_pipes_for_plane: Option<unsafe extern "C" fn(*const dc_plane_state, *mut resource_context, *mut pipe_ctx) -> i32>,
    pub get_stream_status: Option<unsafe extern "C" fn(*mut dc_state, *const dc_stream_state) -> *mut dc_stream_status>,
    pub get_stream_from_id: Option<unsafe extern "C" fn(*const dc_state, u32) -> *mut dc_stream_state>,
    pub get_max_flickerless_instant_vtotal_increase: Option<unsafe extern "C" fn(*mut dc_stream_state, bool) -> u32>,
    pub allocate_mcache: Option<unsafe extern "C" fn(*mut dc_state, *const dc_mcache_params) -> bool>,
}

#[repr(C)]
pub struct dml2_dc_svp_callbacks {
    pub dc: *mut dc,
    pub build_scaling_params: Option<unsafe extern "C" fn(*mut pipe_ctx) -> bool>,
    pub create_phantom_stream: Option<unsafe extern "C" fn(*const dc, *mut dc_state, *mut dc_stream_state) -> *mut dc_stream_state>,
    pub create_phantom_plane: Option<unsafe extern "C" fn(*const dc, *mut dc_state, *mut dc_plane_state) -> *mut dc_plane_state>,
    pub add_phantom_stream: Option<unsafe extern "C" fn(*const dc, *mut dc_state, *mut dc_stream_state, *mut dc_stream_state) -> dc_status>,
    pub add_phantom_plane: Option<unsafe extern "C" fn(*const dc, *mut dc_stream_state, *mut dc_plane_state, *mut dc_state) -> bool>,
    pub remove_phantom_plane: Option<unsafe extern "C" fn(*const dc, *mut dc_stream_state, *mut dc_plane_state, *mut dc_state) -> bool>,
    pub remove_phantom_stream: Option<unsafe extern "C" fn(*const dc, *mut dc_state, *mut dc_stream_state) -> dc_status>,
    pub release_phantom_plane: Option<unsafe extern "C" fn(*const dc, *mut dc_state, *mut dc_plane_state)>,
    pub release_phantom_stream: Option<unsafe extern "C" fn(*const dc, *mut dc_state, *mut dc_stream_state)>,
    pub release_dsc: Option<unsafe extern "C" fn(*mut resource_context, *const resource_pool, *mut *mut display_stream_compressor)>,
    pub get_pipe_subvp_type: Option<unsafe extern "C" fn(*const dc_state, *const pipe_ctx) -> mall_stream_type>,
    pub get_stream_subvp_type: Option<unsafe extern "C" fn(*const dc_state, *const dc_stream_state) -> mall_stream_type>,
    pub get_paired_subvp_stream: Option<unsafe extern "C" fn(*const dc_state, *const dc_stream_state) -> *mut dc_stream_state>,
    pub remove_phantom_streams_and_planes: Option<unsafe extern "C" fn(*const dc, *mut dc_state) -> bool>,
    pub release_phantom_streams_and_planes: Option<unsafe extern "C" fn(*const dc, *mut dc_state)>,
    pub calculate_mall_ways_from_bytes: Option<unsafe extern "C" fn(*const dc, u32) -> u32>,
}

#[repr(C)] pub struct dml2_clks_table_entry { pub dcfclk_mhz:u32, pub fclk_mhz:u32, pub memclk_mhz:u32, pub socclk_mhz:u32, pub dtbclk_mhz:u32, pub dispclk_mhz:u32, pub dppclk_mhz:u32, pub dram_speed_mts:u32 }
#[repr(C)] pub struct dml2_clks_num_entries { pub num_dcfclk_levels:u32, pub num_fclk_levels:u32, pub num_memclk_levels:u32, pub num_socclk_levels:u32, pub num_dtbclk_levels:u32, pub num_dispclk_levels:u32, pub num_dppclk_levels:u32 }
#[repr(C)] pub struct dml2_clks_limit_table { pub clk_entries:[dml2_clks_table_entry; DML2_MAX_NUM_DPM_LVL], pub num_entries_per_clk:dml2_clks_num_entries, pub num_states:u32 }

#[repr(C)] pub struct dml2_soc_bbox_overrides { pub xtalclk_mhz:f64, pub dchub_refclk_mhz:f64, pub dprefclk_mhz:f64, pub disp_pll_vco_speed_mhz:f64, pub urgent_latency_us:f64, pub sr_exit_latency_us:f64, pub sr_enter_plus_exit_latency_us:f64, pub sr_exit_z8_time_us:f64, pub sr_enter_plus_exit_z8_time_us:f64, pub dram_clock_change_latency_us:f64, pub fclk_change_latency_us:f64, pub dram_num_chan:u32, pub dram_chanel_width_bytes:u32, pub clks_table:dml2_clks_limit_table }

#[repr(C)] pub enum dml2_force_pstate_methods { dml2_force_pstate_method_auto=0, dml2_force_pstate_method_vactive, dml2_force_pstate_method_vblank, dml2_force_pstate_method_drr, dml2_force_pstate_method_subvp, dml2_force_pstate_method_alternate }

#[repr(C)] pub struct dml2_configuration_options {
    pub dcn_pipe_count:i32, pub use_native_pstate_optimization:bool, pub enable_windowed_mpo_odm:bool, pub use_native_soc_bb_construction:bool, pub skip_hw_state_mapping:bool, pub optimize_odm_4to1:bool, pub minimize_dispclk_using_odm:bool, pub override_det_buffer_size_kbytes:bool,
    pub callbacks:dml2_dc_callbacks,
    pub svp_pstate: dml2_svp_pstate,
    pub mall_cfg:dml2_soc_mall_info, pub alt_ch_cfg:dml2_soc_alt_ch_info, pub bbox_overrides:dml2_soc_bbox_overrides,
    pub max_segments_per_hubp:u32, pub det_segment_size:u32, pub external_socbb_ip_params:*mut socbb_ip_params_external,
    pub pmo:dml2_pmo, pub map_dc_pipes_with_callbacks:bool, pub use_clock_dc_limits:bool, pub gpuvm_enable:bool, pub hostvm_enable:bool, pub force_tdlut_enable:bool, pub bb_from_dmub:*mut core::ffi::c_void,
}
#[repr(C)] pub struct dml2_svp_pstate { pub force_disable_subvp:bool, pub force_enable_subvp:bool, pub subvp_fw_processing_delay_us:u32, pub subvp_pstate_allow_width_us:u32, pub subvp_prefetch_end_to_mall_start_us:u32, pub subvp_swath_height_margin_lines:u32, pub callbacks:dml2_dc_svp_callbacks }
#[repr(C)] pub struct dml2_pmo { pub force_mandatory_uclk_pstate_support:bool, pub force_pstate_method_enable:bool, pub force_pstate_method_values:[dml2_force_pstate_methods; MAX_PIPES] }

extern "C" {
    pub fn dml2_create(in_dc:*const dc, config:*const dml2_configuration_options, dml2:*mut *mut dml2_context) -> bool;
    pub fn dml2_destroy(dml2:*mut dml2_context);
    pub fn dml2_copy(dst_dml2:*mut dml2_context, src_dml2:*mut dml2_context);
    pub fn dml2_create_copy(dst_dml2:*mut *mut dml2_context, src_dml2:*mut dml2_context) -> bool;
    pub fn dml2_reinit(in_dc:*const dc, config:*const dml2_configuration_options, dml2:*mut *mut dml2_context);
    pub fn dml2_validate(in_dc:*const dc, context:*mut dc_state, dml2:*mut dml2_context, validate_mode:dc_validate_mode) -> bool;
    pub fn dml2_extract_dram_and_fclk_change_support(dml2:*mut dml2_context, fclk_change_support:*mut u32, dram_clk_change_support:*mut u32);
    pub fn dml2_prepare_mcache_programming(in_dc:*mut dc, context:*mut dc_state, dml2:*mut dml2_context);
    pub fn dml2_apply_debug_options(dc:*const dc, dml2:*mut dml2_context);
    pub fn dml2_validate_only(context:*mut dc_state, validate_mode:dc_validate_mode) -> bool;
    pub fn dml2_validate_and_build_resource(in_dc:*const dc, context:*mut dc_state, validate_mode:dc_validate_mode) -> bool;
    pub fn dml2_allocate_memory() -> *mut dml2_context;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
