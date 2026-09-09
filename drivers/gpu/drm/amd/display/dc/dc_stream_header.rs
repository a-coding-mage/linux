/*
 * Copyright 2012-14 Advanced Micro Devices, Inc.
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

#[repr(C)]
pub struct timing_sync_info { pub group_id: i32, pub group_size: i32, pub master: bool }

#[repr(C)]
pub struct mall_stream_config {
    pub type_: mall_stream_type,
    pub paired_stream: *mut dc_stream_state,
    pub subvp_limit_cursor_size: bool,
    pub cursor_size_limit_subvp: bool,
}

#[repr(C)]
pub struct dc_stream_status {
    pub primary_otg_inst: i32,
    pub stream_enc_inst: i32,
    pub plane_count: i32,
    pub audio_inst: i32,
    pub timing_sync_info: timing_sync_info,
    pub plane_states: [*mut dc_plane_state; MAX_SURFACES],
    pub is_abm_supported: bool,
    pub mall_stream_config: mall_stream_config,
    pub fpo_in_use: bool,
}

#[repr(C)]
pub struct dc_writeback_info {
    pub wb_enabled: bool,
    pub dwb_pipe_inst: i32,
    pub dwb_params: dc_dwb_params,
    pub mcif_buf_params: mcif_buf_params,
    pub mcif_warmup_params: mcif_warmup_params,
    pub writeback_source_plane: *mut dc_plane_state,
    pub mpcc_inst: i32,
}

#[repr(C)]
pub struct dc_writeback_update {
    pub num_wb_info: u32,
    pub writeback_info: [dc_writeback_info; MAX_DWB_PIPES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum vertical_interrupt_ref_point { START_V_UPDATE = 0, START_V_SYNC, INVALID_POINT }

#[repr(C)]
pub struct periodic_interrupt_config { pub ref_point: vertical_interrupt_ref_point, pub lines_offset: i32 }

#[repr(C)]
pub struct dc_mst_stream_bw_update { pub is_increase: bool, pub mst_stream_bw: u32 }

#[repr(C)]
pub struct stream_update_flags_bits {
    pub scaling: u32, pub out_tf: u32, pub out_csc: u32, pub abm_level: u32,
    pub dpms_off: u32, pub gamut_remap: u32, pub wb_update: u32, pub dsc_changed: u32,
    pub mst_bw: u32, pub crtc_timing_adjust: u32, pub fams_changed: u32,
    pub scaler_sharpener: u32, pub sharpening_required: u32, pub cursor_attr: u32,
    pub cursor_pos: u32, pub periodic_interrupt: u32, pub info_frame: u32,
    pub dmdata: u32, pub dither: u32,
}

#[repr(C)]
pub union stream_update_flags { pub bits: stream_update_flags_bits, pub raw: u32 }

pub unsafe fn stream_update_flags_clear(flags: *mut stream_update_flags) { (*flags).raw = 0; }

pub unsafe fn stream_update_flags_set_full(flags: *mut stream_update_flags) {
    stream_update_flags_clear(flags);
    (*flags).bits.scaling = 1; (*flags).bits.out_tf = 1; (*flags).bits.out_csc = 1;
    (*flags).bits.abm_level = 1; (*flags).bits.dpms_off = 1; (*flags).bits.gamut_remap = 1;
    (*flags).bits.wb_update = 1; (*flags).bits.dsc_changed = 1; (*flags).bits.mst_bw = 1;
    (*flags).bits.crtc_timing_adjust = 1; (*flags).bits.fams_changed = 1;
    (*flags).bits.scaler_sharpener = 1; (*flags).bits.sharpening_required = 1;
    (*flags).bits.cursor_attr = 1; (*flags).bits.cursor_pos = 1;
    (*flags).bits.periodic_interrupt = 1; (*flags).bits.info_frame = 1;
    (*flags).bits.dmdata = 1; (*flags).bits.dither = 1;
}

#[repr(C)]
pub struct test_pattern {
    pub type_: dp_test_pattern,
    pub color_space: dp_test_pattern_color_space,
    pub p_link_settings: *const link_training_settings,
    pub p_custom_pattern: *const u8,
    pub cust_pattern_size: u32,
}

pub const SUBVP_DRR_MARGIN_US: u32 = 100;

#[repr(C)]
pub struct dc_stream_debug_options { pub force_odm_combine_segments: u8, pub allow_transition_for_forced_odm: u8 }

pub const LUMINANCE_DATA_TABLE_SIZE: usize = 10;

#[repr(C)]
pub struct luminance_data {
    pub is_valid: bool,
    pub refresh_rate_hz: [i32; LUMINANCE_DATA_TABLE_SIZE],
    pub luminance_millinits: [i32; LUMINANCE_DATA_TABLE_SIZE],
    pub flicker_criteria_milli_nits_GAMING: i32,
    pub flicker_criteria_milli_nits_STATIC: i32,
    pub nominal_refresh_rate: u32,
    pub dm_max_decrease_from_nominal: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dc_drr_trigger_mode { DRR_TRIGGER_ON_FLIP = 0, DRR_TRIGGER_ON_FLIP_AND_CURSOR }

#[repr(C)]
pub struct dc_stream_state {
    pub sink: *mut dc_sink, pub link: *mut dc_link, pub link_enc: *mut link_encoder,
    pub debug: dc_stream_debug_options, pub sink_patches: dc_panel_patch,
    pub timing: dc_crtc_timing, pub adjust: dc_crtc_timing_adjust,
    pub vrr_infopacket: dc_info_packet, pub vsc_infopacket: dc_info_packet,
    pub vsp_infopacket: dc_info_packet, pub hfvsif_infopacket: dc_info_packet,
    pub vtem_infopacket: dc_info_packet, pub adaptive_sync_infopacket: dc_info_packet,
    pub avi_infopacket: dc_info_packet, pub dsc_packed_pps: [u8; 128],
    pub src: rect, pub dst: rect, pub audio_info: audio_info,
    pub hdr_static_metadata: dc_info_packet, pub dmdata_address: PHYSICAL_ADDRESS_LOC,
    pub use_dynamic_meta: bool, pub out_transfer_func: dc_transfer_func,
    pub gamut_remap_matrix: colorspace_transform, pub csc_color_matrix: dc_csc_transform,
    pub output_color_space: dc_color_space, pub content_type: display_content_type,
    pub dither_option: dc_dither_option, pub view_format: view_3d_format,
    pub use_vsc_sdp_for_colorimetry: bool, pub ignore_msa_timing_param: bool,
    pub allow_freesync: bool, pub vrr_active_variable: bool, pub freesync_on_desktop: bool,
    pub vrr_active_fixed: bool, pub converter_disable_audio: bool, pub qs_bit: u8, pub qy_bit: u8,
    pub abm_level: u32, pub periodic_interrupt: periodic_interrupt_config, pub ctx: *mut dc_context,
    pub bit_depth_params: bit_depth_reduction_params, pub clamping: clamping_and_pixel_encoding_params,
    pub phy_pix_clk: i32, pub signal: signal_type, pub dpms_off: bool, pub dm_stream_context: *mut core::ffi::c_void,
    pub cursor_attributes: dc_cursor_attributes, pub cursor_position: dc_cursor_position, pub hw_cursor_req: bool,
    pub sdr_white_level: u32, pub refcount: kref, pub triggered_crtc_reset: crtc_trigger_info,
    pub num_wb_info: u32, pub writeback_info: [dc_writeback_info; MAX_DWB_PIPES],
    pub func_shaper: *const dc_transfer_func, pub lut3d_func: *const dc_3dlut, pub mode_changed: bool,
    pub out: dc_stream_state_out, pub apply_edp_fast_boot_optimization: bool,
    pub apply_seamless_boot_optimization: bool, pub apply_boot_odm_mode: u32, pub stream_id: u32,
    pub test_pattern: test_pattern, pub update_flags: stream_update_flags,
    pub has_non_synchronizable_pclk: bool, pub vblank_synchronized: bool, pub is_phantom: bool,
    pub lumin_data: luminance_data, pub scaler_sharpener_update: bool, pub sharpening_required: bool,
    pub drr_trigger_mode: dc_drr_trigger_mode, pub blending_linearity: dc_blending_linearity,
    pub update_scratch: *mut dc_update_scratch_space, pub firmware_controlled_hdr_info_packet: bool,
}

#[repr(C)] pub struct dc_stream_state_out { pub otg_offset: u8 }
pub const ABM_LEVEL_IMMEDIATE_DISABLE: u32 = 255;

#[repr(C)]
pub struct dc_stream_update {
    pub stream: *mut dc_stream_state, pub src: rect, pub dst: rect,
    pub out_transfer_func: *mut dc_transfer_func, pub hdr_static_metadata: *mut dc_info_packet,
    pub abm_level: *mut u32, pub periodic_interrupt: *mut periodic_interrupt_config,
    pub vrr_infopacket: *mut dc_info_packet, pub vsc_infopacket: *mut dc_info_packet,
    pub vsp_infopacket: *mut dc_info_packet, pub hfvsif_infopacket: *mut dc_info_packet,
    pub vtem_infopacket: *mut dc_info_packet, pub adaptive_sync_infopacket: *mut dc_info_packet,
    pub avi_infopacket: *mut dc_info_packet, pub dpms_off: *mut bool, pub integer_scaling_update: bool,
    pub allow_freesync: *mut bool, pub vrr_active_variable: *mut bool, pub vrr_active_fixed: *mut bool,
    pub gamut_remap: *mut colorspace_transform, pub output_color_space: *mut dc_color_space,
    pub dither_option: *mut dc_dither_option, pub output_csc_transform: *mut dc_csc_transform,
    pub wb_update: *mut dc_writeback_update, pub dsc_config: *mut dc_dsc_config,
    pub mst_bw_update: *mut dc_mst_stream_bw_update, pub func_shaper: *mut dc_transfer_func,
    pub lut3d_func: *mut dc_3dlut, pub pending_test_pattern: *mut test_pattern,
    pub crtc_timing_adjust: *mut dc_crtc_timing_adjust, pub cursor_attributes: *mut dc_cursor_attributes,
    pub cursor_position: *mut dc_cursor_position, pub hw_cursor_req: *mut bool,
    pub scaler_sharpener_update: *mut bool, pub sharpening_required: *mut bool,
    pub blending_linearity: *mut dc_blending_linearity, pub drr_trigger_mode: *mut dc_drr_trigger_mode,
}

extern "C" {
    pub fn dc_is_stream_unchanged(old_stream: *mut dc_stream_state, stream: *mut dc_stream_state) -> bool;
    pub fn dc_is_stream_scaling_unchanged(old_stream: *mut dc_stream_state, stream: *mut dc_stream_state) -> bool;
    pub fn dc_update_planes_and_stream(dc: *mut dc, surface_updates: *mut dc_surface_update, surface_count: i32, dc_stream: *mut dc_stream_state, stream_update: *mut dc_stream_update) -> bool;
    pub fn dc_commit_updates_for_stream(dc: *mut dc, srf_updates: *mut dc_surface_update, surface_count: i32, stream: *mut dc_stream_state, stream_update: *mut dc_stream_update, state: *mut dc_state);
    pub fn dc_stream_log(dc: *const dc, stream: *const dc_stream_state);
    pub fn dc_get_current_stream_count(dc: *mut dc) -> u8;
    pub fn dc_get_stream_at_index(dc: *mut dc, i: u8) -> *mut dc_stream_state;
    pub fn dc_stream_get_vblank_counter(stream: *const dc_stream_state) -> u32;
    pub fn dc_stream_send_dp_sdp(stream: *const dc_stream_state, custom_sdp_message: *const u8, sdp_message_size: u32) -> bool;
    pub fn dc_stream_get_scanoutpos(stream: *const dc_stream_state, v_blank_start: *mut u32, v_blank_end: *mut u32, h_position: *mut u32, v_position: *mut u32) -> bool;
    pub fn dc_stream_add_writeback(dc: *mut dc, stream: *mut dc_stream_state, wb_info: *mut dc_writeback_info) -> bool;
    pub fn dc_stream_fc_disable_writeback(dc: *mut dc, stream: *mut dc_stream_state, dwb_pipe_inst: u32) -> bool;
    pub fn dc_stream_remove_writeback(dc: *mut dc, stream: *mut dc_stream_state, dwb_pipe_inst: u32) -> bool;
    pub fn dc_stream_add_dsc_to_resource(dc: *mut dc, state: *mut dc_state, stream: *mut dc_stream_state) -> dc_status;
    pub fn dc_stream_dmdata_status_done(dc: *mut dc, stream: *mut dc_stream_state) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
