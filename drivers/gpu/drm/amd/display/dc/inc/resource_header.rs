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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependencies are supplied by the surrounding translation unit.

pub const MEMORY_TYPE_MULTIPLIER_CZ: i32 = 4;
pub const MEMORY_TYPE_HBM: i32 = 2;
pub const MAX_MCACHES: usize = 8;
pub const FREE_PIPE_INDEX_NOT_FOUND: i32 = -1;

// C macros, preserved as unsafe field-access helpers. `pipe_idx_syncd` is an
// externally defined field of the opaque pipe context type.
#[inline]
pub unsafe fn IS_PIPE_SYNCD_VALID(pipe: *const pipe_ctx) -> i32 { ((*pipe).pipe_idx_syncd & 0x80 != 0) as i32 }
#[inline]
pub unsafe fn GET_PIPE_SYNCD_FROM_PIPE(pipe: *const pipe_ctx) -> u8 { (*pipe).pipe_idx_syncd & 0x7f }
#[inline]
pub unsafe fn SET_PIPE_SYNCD_TO_PIPE(pipe: *mut pipe_ctx, pipe_syncd: u8) { (*pipe).pipe_idx_syncd = 0x80 | pipe_syncd; }

#[repr(C)]
pub struct resource_caps {
    pub num_timing_generator: i32, pub num_opp: i32, pub num_dpp: i32,
    pub num_video_plane: i32, pub num_audio: i32, pub num_stream_encoder: i32,
    pub num_analog_stream_encoder: i32, pub num_pll: i32, pub num_dwb: i32,
    pub num_ddc: i32, pub num_vmid: i32, pub num_dsc: i32,
    pub num_dig_link_enc: u32, // Total number of DIGs (digital encoders) in DIO.
    pub num_usb4_dpia: u32, // Total number of USB4 DPIA (DisplayPort Input Adapters).
    pub num_hpo_frl: i32, pub num_hpo_dp_stream_encoder: i32,
    pub num_hpo_dp_link_encoder: i32, pub num_mpc_3dlut: i32, pub num_mpc: i32,
    pub num_rmcm: i32, pub num_aux: i32,
}

#[repr(C)]
pub struct resource_straps { pub hdmi_disable: u32, pub dc_pinstraps_audio: u32, pub audio_stream_number: u32 }

#[repr(C)]
pub struct dc_mcache_allocations {
    pub global_mcache_ids_plane0: [i32; MAX_MCACHES + 1],
    pub global_mcache_ids_plane1: [i32; MAX_MCACHES + 1],
    pub global_mcache_ids_mall_plane0: [i32; MAX_MCACHES + 1],
    pub global_mcache_ids_mall_plane1: [i32; MAX_MCACHES + 1],
}

#[repr(C)]
pub struct resource_create_funcs {
    pub read_dce_straps: Option<unsafe extern "C" fn(*mut dc_context, *mut resource_straps)>,
    pub create_audio: Option<unsafe extern "C" fn(*mut dc_context, u32) -> *mut audio>,
    pub create_stream_encoder: Option<unsafe extern "C" fn(engine_id, *mut dc_context) -> *mut stream_encoder>,
    pub create_hpo_frl_stream_encoder: Option<unsafe extern "C" fn(engine_id, *mut dc_context) -> *mut hpo_frl_stream_encoder>,
    pub create_hpo_frl_link_encoder: Option<unsafe extern "C" fn(engine_id, *mut dc_context) -> *mut hpo_frl_link_encoder>,
    pub create_hpo_dp_stream_encoder: Option<unsafe extern "C" fn(engine_id, *mut dc_context) -> *mut hpo_dp_stream_encoder>,
    pub create_hpo_dp_link_encoder: Option<unsafe extern "C" fn(u8, *mut dc_context) -> *mut hpo_dp_link_encoder>,
    pub create_hwseq: Option<unsafe extern "C" fn(*mut dc_context) -> *mut dce_hwseq>,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pipe_type { FREE_PIPE, OTG_MASTER, OPP_HEAD, DPP_PIPE }

extern "C" {
    pub fn resource_parse_asic_id(asic_id: hw_asic_id) -> dce_version;
    pub fn resource_construct(num_virtual_links: u32, dc: *mut dc, pool: *mut resource_pool, create_funcs: *const resource_create_funcs) -> bool;
    pub fn dc_create_resource_pool(dc: *mut dc, init_data: *const dc_init_data, dc_version: dce_version) -> *mut resource_pool;
    pub fn dc_destroy_resource_pool(dc: *mut dc);
    pub fn resource_map_pool_resources(dc: *const dc, context: *mut dc_state, stream: *mut dc_stream_state) -> dc_status;
    pub fn resource_build_test_pattern_params(res_ctx: *mut resource_context, pipe_ctx: *mut pipe_ctx);
    pub fn resource_is_upsp_required(format: surface_pixel_format) -> upsp_mode;
    pub fn resource_build_scaling_params(pipe_ctx: *mut pipe_ctx) -> bool;
    pub fn resource_build_scaling_params_for_context(dc: *const dc, context: *mut dc_state) -> dc_status;
    pub fn resource_build_info_frame(pipe_ctx: *mut pipe_ctx);
    pub fn resource_unreference_clock_source(res_ctx: *mut resource_context, pool: *const resource_pool, clock_source: *mut clock_source);
    pub fn resource_reference_clock_source(res_ctx: *mut resource_context, pool: *const resource_pool, clock_source: *mut clock_source);
    pub fn resource_get_clock_source_reference(res_ctx: *mut resource_context, pool: *const resource_pool, clock_source: *mut clock_source) -> i32;
    pub fn resource_are_streams_timing_synchronizable(stream1: *mut dc_stream_state, stream2: *mut dc_stream_state) -> bool;
    pub fn resource_are_vblanks_synchronizable(stream1: *mut dc_stream_state, stream2: *mut dc_stream_state) -> bool;
    pub fn resource_find_used_clk_src_for_sharing(res_ctx: *mut resource_context, pipe_ctx: *mut pipe_ctx) -> *mut clock_source;
    pub fn dc_resource_find_first_free_pll(res_ctx: *mut resource_context, pool: *const resource_pool) -> *mut clock_source;
    pub fn resource_attach_surfaces_to_context(plane_state: *const *mut dc_plane_state, surface_count: i32, dc_stream: *mut dc_stream_state, context: *mut dc_state, pool: *const resource_pool) -> bool;
    pub fn resource_can_pipe_disable_cursor(pipe_ctx: *mut pipe_ctx) -> bool;
    pub fn resource_is_pipe_type(pipe_ctx: *const pipe_ctx, type_: pipe_type) -> bool;
    pub fn resource_add_otg_master_for_stream_output(new_ctx: *mut dc_state, pool: *const resource_pool, stream: *mut dc_stream_state) -> dc_status;
    pub fn resource_remove_otg_master_for_stream_output(new_ctx: *mut dc_state, pool: *const resource_pool, stream: *mut dc_stream_state);
    pub fn resource_append_dpp_pipes_for_plane_composition(new_ctx: *mut dc_state, cur_ctx: *mut dc_state, pool: *mut resource_pool, otg_master_pipe: *mut pipe_ctx, plane_state: *mut dc_plane_state) -> bool;
    pub fn resource_remove_dpp_pipes_for_plane_composition(context: *mut dc_state, pool: *const resource_pool, plane_state: *const dc_plane_state);
    pub fn resource_update_pipes_for_stream_with_slice_count(new_ctx: *mut dc_state, cur_ctx: *const dc_state, pool: *const resource_pool, stream: *const dc_stream_state, new_slice_count: i32) -> bool;
    pub fn resource_update_pipes_for_plane_with_slice_count(new_ctx: *mut dc_state, cur_ctx: *const dc_state, pool: *const resource_pool, plane: *const dc_plane_state, slice_count: i32) -> bool;
    pub fn resource_get_otg_master_for_stream(res_ctx: *mut resource_context, stream: *const dc_stream_state) -> *mut pipe_ctx;
    pub fn resource_get_opp_heads_for_otg_master(otg_master: *const pipe_ctx, res_ctx: *mut resource_context, opp_heads: *mut *mut pipe_ctx) -> i32;
    pub fn resource_get_dpp_pipes_for_opp_head(opp_head: *const pipe_ctx, res_ctx: *mut resource_context, dpp_pipes: *mut *mut pipe_ctx) -> i32;
    pub fn resource_get_dpp_pipes_for_plane(plane: *const dc_plane_state, res_ctx: *mut resource_context, dpp_pipes: *mut *mut pipe_ctx) -> i32;
    pub fn resource_get_otg_master(pipe_ctx: *const pipe_ctx) -> *mut pipe_ctx;
    pub fn resource_get_opp_head(pipe_ctx: *const pipe_ctx) -> *mut pipe_ctx;
    pub fn resource_get_primary_dpp_pipe(dpp_pipe: *const pipe_ctx) -> *mut pipe_ctx;
    pub fn resource_get_mpc_slice_index(dpp_pipe: *const pipe_ctx) -> i32;
    pub fn resource_get_mpc_slice_count(pipe: *const pipe_ctx) -> i32;
    pub fn resource_get_odm_slice_count(pipe: *const pipe_ctx) -> i32;
    pub fn resource_get_odm_slice_index(opp_head: *const pipe_ctx) -> i32;
    pub fn resource_get_odm_slice_src_rect(pipe_ctx: *mut pipe_ctx) -> rect;
    pub fn resource_get_odm_slice_dst_rect(pipe_ctx: *mut pipe_ctx) -> rect;
    pub fn resource_get_odm_slice_dst_width(otg_master: *mut pipe_ctx, is_last_segment: bool) -> i32;
    pub fn resource_is_pipe_topology_changed(state_a: *const dc_state, state_b: *const dc_state) -> bool;
    pub fn resource_is_odm_topology_changed(otg_master_a: *const pipe_ctx, otg_master_b: *const pipe_ctx) -> bool;
    pub fn resource_log_pipe_topology_update(dc: *mut dc, state: *mut dc_state);
    pub fn resource_find_free_pipe_used_as_sec_opp_head_by_cur_otg_master(cur_res_ctx: *const resource_context, new_res_ctx: *mut resource_context, cur_otg_master: *const pipe_ctx) -> i32;
    pub fn resource_find_free_pipe_used_in_cur_mpc_blending_tree(cur_res_ctx: *const resource_context, new_res_ctx: *mut resource_context, cur_opp_head: *const pipe_ctx) -> i32;
    pub fn recource_find_free_pipe_not_used_in_cur_res_ctx(cur_res_ctx: *const resource_context, new_res_ctx: *mut resource_context, pool: *const resource_pool) -> i32;
    pub fn recource_find_free_pipe_used_as_otg_master_in_cur_res_ctx(cur_res_ctx: *const resource_context, new_res_ctx: *mut resource_context, pool: *const resource_pool) -> i32;
    pub fn resource_find_free_pipe_used_as_cur_sec_dpp(cur_res_ctx: *const resource_context, new_res_ctx: *mut resource_context, pool: *const resource_pool) -> i32;
    pub fn resource_find_free_pipe_used_as_cur_sec_dpp_in_mpcc_combine(cur_res_ctx: *const resource_context, new_res_ctx: *mut resource_context, pool: *const resource_pool) -> i32;
    pub fn resource_find_any_free_pipe(new_res_ctx: *mut resource_context, pool: *const resource_pool) -> i32;
    pub fn resource_find_free_secondary_pipe_legacy(res_ctx: *mut resource_context, pool: *const resource_pool, primary_pipe: *const pipe_ctx) -> *mut pipe_ctx;
    pub fn resource_validate_attach_surfaces(set: *const dc_validation_set, old_context: *const dc_state, context: *mut dc_state, pool: *const resource_pool) -> bool;
    pub fn resource_validate_probe_set(dc: *mut dc, probes: *const dc_probe_state, probe_count: u8) -> dc_status;
    pub fn resource_map_clock_resources(dc: *const dc, context: *mut dc_state, stream: *mut dc_stream_state) -> dc_status;
    pub fn resource_map_phy_clock_resources(dc: *const dc, context: *mut dc_state, stream: *mut dc_stream_state) -> dc_status;
    pub fn pipe_need_reprogram(pipe_ctx_old: *mut pipe_ctx, pipe_ctx: *mut pipe_ctx) -> bool;
    pub fn resource_build_bit_depth_reduction_params(stream: *mut dc_stream_state, fmt_bit_depth: *mut bit_depth_reduction_params);
    pub fn update_audio_usage(res_ctx: *mut resource_context, pool: *const resource_pool, audio: *mut audio, acquired: bool);
    pub fn resource_pixel_format_to_bpp(format: surface_pixel_format) -> u32;
    pub fn get_temp_dp_link_res(link: *mut dc_link, link_res: *mut link_resource, link_settings: *mut dc_link_settings) -> bool;
    pub fn get_temp_frl_link_res(link: *mut dc_link, link_res: *mut link_resource) -> bool;
    pub fn reset_syncd_pipes_from_disabled_pipes(dc: *mut dc, context: *mut dc_state);
    pub fn check_syncd_pipes_for_disabled_master_pipe(dc: *mut dc, context: *mut dc_state, disabled_master_pipe_idx: u8);
    pub fn reset_sync_context_for_pipe(dc: *const dc, context: *mut dc_state, pipe_idx: u8);
    pub fn resource_transmitter_to_phy_idx(dc: *const dc, transmitter: transmitter) -> u8;
    pub fn get_link_hwss(link: *const dc_link, link_res: *const link_resource) -> *const link_hwss;
    pub fn is_h_timing_divisible_by_2(stream: *mut dc_stream_state) -> bool;
    pub fn dc_resource_acquire_secondary_pipe_for_mpc_odm_legacy(dc: *const dc, state: *mut dc_state, pri_pipe: *mut pipe_ctx, sec_pipe: *mut pipe_ctx, odm: bool) -> bool;
    pub fn update_dp_encoder_resources_for_test_harness(dc: *const dc, context: *mut dc_state, pipe_ctx: *mut pipe_ctx) -> dc_status;
    pub fn resource_get_dscl_prog_data(pipe_ctx: *mut pipe_ctx) -> *mut dscl_prog_data;
    pub fn resource_init_common_dml2_callbacks(dc: *mut dc, dml2_options: *mut dml2_configuration_options);
    pub fn resource_calculate_det_for_stream(state: *mut dc_state, otg_master: *mut pipe_ctx) -> i32;
    pub fn resource_is_hpo_acquired(context: *mut dc_state) -> bool;
    pub fn get_temp_dio_link_enc(res_ctx: *const resource_context, pool: *const resource_pool, link: *const dc_link) -> *mut link_encoder;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
