/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2026 Advanced Micro Devices, Inc.
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

pub struct amdgpu_device;
pub struct amdgpu_dm_connector;
pub struct amdgpu_display_manager;
pub struct amdgpu_encoder;
pub struct amdgpu_i2c_adapter;
pub struct dc_crtc_timing;
pub struct dc_link;
pub struct dc_state;
pub struct dc_stream_state;
pub struct ddc_service;
pub struct dm_connector_state;
pub struct drm_atomic_commit;
pub struct drm_device;
pub struct drm_encoder_helper_funcs;
pub struct drm_connector;
pub struct drm_connector_state;
pub struct drm_crtc;
pub struct drm_display_mode;
pub struct drm_edid;
pub struct drm_property;
pub struct dc_info_packet;
pub struct i2c_adapter;
pub struct i2c_msg;
pub struct edid;
pub struct amdgpu_hdmi_vsdb_info;
pub struct dc;
pub struct drm_crtc_state;
pub struct drm_display_info;

pub type uint = ::core::ffi::c_uint;

extern "C" {
    pub fn amdgpu_dm_connector_funcs_reset(connector: *mut drm_connector);
    pub fn amdgpu_dm_connector_atomic_duplicate_state(connector: *mut drm_connector) -> *mut drm_connector_state;
    pub fn amdgpu_dm_connector_atomic_set_property(connector: *mut drm_connector, connector_state: *mut drm_connector_state, property: *mut drm_property, val: u64) -> ::core::ffi::c_int;
    pub fn amdgpu_dm_connector_atomic_get_property(connector: *mut drm_connector, state: *const drm_connector_state, property: *mut drm_property, val: *mut u64) -> ::core::ffi::c_int;
    pub fn amdgpu_dm_connector_init_helper(dm: *mut amdgpu_display_manager, aconnector: *mut amdgpu_dm_connector, connector_type: ::core::ffi::c_int, link: *mut dc_link, link_index: ::core::ffi::c_int);
    pub fn amdgpu_dm_connector_mode_valid(connector: *mut drm_connector, mode: *const drm_display_mode) -> drm_mode_status;
    pub fn dm_restore_drm_connector_state(dev: *mut drm_device, connector: *mut drm_connector);
    pub fn amdgpu_dm_update_freesync_caps(connector: *mut drm_connector, drm_edid: *const drm_edid, do_mccs: bool);
    pub fn amdgpu_dm_update_connector_after_detect(aconnector: *mut amdgpu_dm_connector);
    pub fn amdgpu_dm_hdmi_cec_set_edid(aconnector: *mut amdgpu_dm_connector);
    pub fn amdgpu_dm_initialize_hdmi_connector(aconnector: *mut amdgpu_dm_connector) -> ::core::ffi::c_int;
    pub fn amdgpu_dm_find_first_crtc_matching_connector(state: *mut drm_atomic_commit, crtc: *mut drm_crtc) -> *mut drm_connector;
    pub fn amdgpu_dm_convert_dc_color_depth_into_bpc(display_color_depth: dc_color_depth) -> ::core::ffi::c_int;
    pub fn amdgpu_dm_create_validate_stream_for_sink(connector: *mut drm_connector, drm_mode: *const drm_display_mode, dm_state: *const dm_connector_state, old_stream: *const dc_stream_state) -> *mut dc_stream_state;
    pub fn amdgpu_dm_connector_init(dm: *mut amdgpu_display_manager, amdgpu_dm_connector: *mut amdgpu_dm_connector, link_index: u32, amdgpu_encoder: *mut amdgpu_encoder) -> ::core::ffi::c_int;
    pub fn amdgpu_dm_s3_handle_hdmi_cec(ddev: *mut drm_device, suspend: bool);
    pub fn amdgpu_dm_detect_mst_link_for_all_connectors(dev: *mut drm_device) -> ::core::ffi::c_int;
    pub fn amdgpu_set_panel_orientation(connector: *mut drm_connector);
    pub fn amdgpu_dm_convert_color_depth_from_display_info(connector: *const drm_connector, is_y420: bool, requested_bpc: ::core::ffi::c_int) -> dc_color_depth;
    pub fn amdgpu_dm_update_stream_scaling_settings(dev: *mut drm_device, mode: *const drm_display_mode, dm_state: *const dm_connector_state, stream: *mut dc_stream_state);
    pub fn amdgpu_dm_is_freesync_video_mode(mode: *const drm_display_mode, aconnector: *mut amdgpu_dm_connector) -> bool;
    pub fn amdgpu_dm_fill_hdr_info_packet(state: *const drm_connector_state, out: *mut dc_info_packet) -> ::core::ffi::c_int;
    pub fn amdgpu_dm_get_output_color_space(dc_crtc_timing: *const dc_crtc_timing, connector_state: *const drm_connector_state) -> dc_color_space;
    pub fn amdgpu_dm_get_highest_refresh_rate_mode(aconnector: *mut amdgpu_dm_connector, use_probed_modes: bool) -> *mut drm_display_mode;
    pub fn amdgpu_dm_create_i2c(ddc_service: *mut ddc_service, oem: bool) -> *mut amdgpu_i2c_adapter;

    pub static amdgpu_dm_encoder_helper_funcs: drm_encoder_helper_funcs;
    pub fn amdgpu_dm_get_encoder_crtc_mask(adev: *mut amdgpu_device) -> ::core::ffi::c_int;
    pub fn amdgpu_dm_encoder_init(dev: *mut drm_device, aencoder: *mut amdgpu_encoder, link_index: u32) -> ::core::ffi::c_int;

    /* C condition: IS_ENABLED(CONFIG_DRM_AMD_DC_KUNIT_TEST). */
    pub fn amdgpu_dm_i2c_xfer(i2c_adap: *mut i2c_adapter, msgs: *mut i2c_msg, num: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn amdgpu_dm_i2c_func(adap: *mut i2c_adapter) -> u32;
    pub fn parse_edid_displayid_vrr(connector: *mut drm_connector, edid: *const edid);
    pub fn get_amd_vsdb(aconnector: *mut amdgpu_dm_connector, vsdb_info: *mut amdgpu_hdmi_vsdb_info) -> ::core::ffi::c_int;
    pub fn parse_hdmi_amd_vsdb(aconnector: *mut amdgpu_dm_connector, edid: *const edid, vsdb_info: *mut amdgpu_hdmi_vsdb_info) -> ::core::ffi::c_int;
    pub fn amdgpu_dm_connector_funcs_force(connector: *mut drm_connector);
    pub fn dm_validate_stream_and_context(dc: *mut dc, stream: *mut dc_stream_state) -> dc_status;
    pub fn amdgpu_dm_connector_to_encoder(connector: *mut drm_connector) -> *mut drm_encoder;
    pub fn amdgpu_dm_get_native_mode(connector: *mut drm_connector);
    pub fn amdgpu_dm_create_common_mode(encoder: *mut drm_encoder, name: *const ::core::ffi::c_char, hdisplay: ::core::ffi::c_int, vdisplay: ::core::ffi::c_int) -> *mut drm_display_mode;
    pub fn amdgpu_dm_connector_add_common_modes(encoder: *mut drm_encoder, connector: *mut drm_connector);
    pub fn amdgpu_dm_connector_ddc_get_modes(connector: *mut drm_connector, drm_edid: *const drm_edid);
    pub fn add_fs_modes(aconnector: *mut amdgpu_dm_connector) -> uint;
    pub fn amdgpu_dm_connector_add_freesync_modes(connector: *mut drm_connector, drm_edid: *const drm_edid);
    pub fn hdmi_cec_unset_edid(aconnector: *mut amdgpu_dm_connector);
    pub fn create_eml_sink(aconnector: *mut amdgpu_dm_connector);
    pub fn handle_edid_mgmt(aconnector: *mut amdgpu_dm_connector);
    pub fn dm_encoder_helper_disable(encoder: *mut drm_encoder);
    pub fn dm_encoder_helper_atomic_check(encoder: *mut drm_encoder, crtc_state: *mut drm_crtc_state, conn_state: *mut drm_connector_state) -> ::core::ffi::c_int;
    pub fn get_subconnector_type(link: *mut dc_link) -> drm_mode_subconnector;
    pub fn update_subconnector_property(aconnector: *mut amdgpu_dm_connector);
    pub fn amdgpu_dm_fbc_init(connector: *mut drm_connector);
    pub fn amdgpu_dm_set_panel_type(aconnector: *mut amdgpu_dm_connector);
    pub fn amdgpu_dm_update_cacp_caps(aconnector: *mut amdgpu_dm_connector);
    pub fn fill_stream_properties_from_drm_display_mode(stream: *mut dc_stream_state, mode_in: *const drm_display_mode, connector: *const drm_connector, connector_state: *const drm_connector_state, old_stream: *const dc_stream_state, requested_bpc: ::core::ffi::c_int, requested_encoding: dc_pixel_encoding, is_hdmi_ep: bool);
    pub fn create_stream_for_sink(connector: *mut drm_connector, drm_mode: *const drm_display_mode, dm_state: *const dm_connector_state, old_stream: *const dc_stream_state, requested_bpc: ::core::ffi::c_int, requested_encoding: dc_pixel_encoding, is_hdmi_ep: bool) -> *mut dc_stream_state;
    pub fn amdgpu_dm_connector_poll(aconnector: *mut amdgpu_dm_connector, force: bool) -> drm_connector_status;
    pub fn amdgpu_dm_connector_detect(connector: *mut drm_connector, force: bool) -> drm_connector_status;
    pub fn amdgpu_dm_connector_unregister(connector: *mut drm_connector);
    pub fn amdgpu_dm_connector_late_register(connector: *mut drm_connector) -> ::core::ffi::c_int;
    pub fn amdgpu_dm_connector_destroy(connector: *mut drm_connector);
    pub fn get_output_content_type(connector_state: *const drm_connector_state) -> display_content_type;
    pub fn adjust_colour_depth_from_display_info(timing_out: *mut dc_crtc_timing, info: *const drm_display_info) -> bool;
    pub fn to_drm_connector_type(st: signal_type, connector_id: u32) -> ::core::ffi::c_int;
    pub fn is_duplicate_mode(aconnector: *mut amdgpu_dm_connector, mode: *mut drm_display_mode) -> bool;
    pub fn get_aspect_ratio(mode_in: *const drm_display_mode) -> dc_aspect_ratio;
    pub fn copy_crtc_timing_for_drm_display_mode(src_mode: *const drm_display_mode, dst_mode: *mut drm_display_mode);
    pub fn decide_crtc_timing_for_drm_display_mode(drm_mode: *mut drm_display_mode, native_mode: *const drm_display_mode, scale_enabled: bool);
    pub fn amdgpu_dm_set_panel_type(aconnector: *mut amdgpu_dm_connector);
    pub fn amdgpu_dm_update_cacp_caps(aconnector: *mut amdgpu_dm_connector);
}

pub const DDC_MANUFACTURERNAME_SAMSUNG: u32 = 0x2D4C;

pub enum signal_type {}
pub enum drm_mode_status {}
pub enum dc_color_depth {}
pub enum dc_color_space {}
pub enum dc_status {}
pub enum drm_mode_subconnector {}
pub enum drm_connector_status {}
pub enum display_content_type {}
pub enum dc_aspect_ratio {}
pub enum dc_pixel_encoding {}
pub enum drm_encoder {}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
