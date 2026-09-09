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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Translated from link_service.h. Types supplied by core_types.h remain external.

unsafe extern "C" {
    pub fn link_create_link_service() -> *mut link_service;
    pub fn link_destroy_link_service(link_srv: *mut *mut link_service);
}

#[repr(C)]
pub struct link_init_data {
    pub dc: *const dc,
    pub ctx: *mut dc_context,
    pub connector_index: u32,
    pub link_index: u32,
    pub is_dpia_link: bool,
}

#[repr(C)]
pub struct ddc_service_init_data {
    pub id: graphics_object_id,
    pub ctx: *mut dc_context,
    pub link: *mut dc_link,
    pub is_dpia_link: bool,
}

#[repr(C)]
pub struct link_service {
    pub create_link: Option<unsafe extern "C" fn(*const link_init_data) -> *mut dc_link>,
    pub destroy_link: Option<unsafe extern "C" fn(*mut *mut dc_link)>,
    pub detect_link: Option<unsafe extern "C" fn(*mut dc_link, dc_detect_reason) -> bool>,
    pub detect_connection_type: Option<unsafe extern "C" fn(*mut dc_link, *mut dc_connection_type) -> bool>,
    pub add_remote_sink: Option<unsafe extern "C" fn(*mut dc_link, *const u8, u32, *mut dc_sink_init_data) -> *mut dc_sink>,
    pub remove_remote_sink: Option<unsafe extern "C" fn(*mut dc_link, *mut dc_sink)>,
    pub get_hpd_state: Option<unsafe extern "C" fn(*mut dc_link) -> bool>,
    pub enable_hpd: Option<unsafe extern "C" fn(*const dc_link)>,
    pub disable_hpd: Option<unsafe extern "C" fn(*const dc_link)>,
    pub enable_hpd_filter: Option<unsafe extern "C" fn(*mut dc_link, bool)>,
    pub reset_cur_dp_mst_topology: Option<unsafe extern "C" fn(*mut dc_link) -> bool>,
    pub get_status: Option<unsafe extern "C" fn(*const dc_link) -> *const dc_link_status>,
    pub is_hdcp1x_supported: Option<unsafe extern "C" fn(*mut dc_link, signal_type) -> bool>,
    pub is_hdcp2x_supported: Option<unsafe extern "C" fn(*mut dc_link, signal_type) -> bool>,
    pub clear_dprx_states: Option<unsafe extern "C" fn(*mut dc_link)>,
    pub get_cur_res_map: Option<unsafe extern "C" fn(*const dc, *mut u32)>,
    pub restore_res_map: Option<unsafe extern "C" fn(*const dc, *mut u32)>,
    pub get_cur_link_res: Option<unsafe extern "C" fn(*const dc_link, *mut link_resource)>,
    pub validate_mode_timing: Option<unsafe extern "C" fn(*const dc_stream_state, *mut dc_link, *const dc_crtc_timing) -> dc_status>,
    pub dp_link_bandwidth_kbps: Option<unsafe extern "C" fn(*const dc_link, *const dc_link_settings) -> u32>,
    pub validate_dp_tunnel_bandwidth: Option<unsafe extern "C" fn(*const dc, *const dc_state) -> dc_status>,
    pub frl_link_bandwidth_kbps: Option<unsafe extern "C" fn(hdmi_frl_link_rate) -> u32>,
    pub frl_margin_check_uncompressed_video: Option<unsafe extern "C" fn(*mut frl_cap_chk_params_fixed31_32, *mut frl_cap_chk_intermediates_fixed31_32) -> bool>,
    pub dp_required_hblank_size_bytes: Option<unsafe extern "C" fn(*const dc_link, *mut dp_audio_bandwidth_params) -> u32>,
    pub set_dpms_on: Option<unsafe extern "C" fn(*mut dc_state, *mut pipe_ctx) -> dc_status>,
    pub set_dpms_off: Option<unsafe extern "C" fn(*mut pipe_ctx) -> dc_status>,
    pub resume: Option<unsafe extern "C" fn(*mut dc_link)>,
    pub blank_all_dp_displays: Option<unsafe extern "C" fn(*mut dc)>,
    pub blank_all_edp_displays: Option<unsafe extern "C" fn(*mut dc)>,
    pub blank_dp_stream: Option<unsafe extern "C" fn(*mut dc_link, bool)>,
    pub increase_mst_payload: Option<unsafe extern "C" fn(*mut pipe_ctx, u32) -> dc_status>,
    pub reduce_mst_payload: Option<unsafe extern "C" fn(*mut pipe_ctx, u32) -> dc_status>,
    pub set_dsc_on_stream: Option<unsafe extern "C" fn(*mut pipe_ctx, bool)>,
    pub set_dsc_enable: Option<unsafe extern "C" fn(*mut pipe_ctx, bool) -> bool>,
    pub update_dsc_config: Option<unsafe extern "C" fn(*mut pipe_ctx) -> bool>,
    pub wait_for_unlocked: Option<unsafe extern "C" fn(*mut dc_link)>,
    pub create_ddc_service: Option<unsafe extern "C" fn(*mut ddc_service_init_data) -> *mut ddc_service>,
    pub destroy_ddc_service: Option<unsafe extern "C" fn(*mut *mut ddc_service)>,
    pub query_ddc_data: Option<unsafe extern "C" fn(*mut ddc_service, u32, *mut u8, u32, *mut u8, u32) -> bool>,
    pub aux_transfer_raw: Option<unsafe extern "C" fn(*mut ddc_service, *mut aux_payload, *mut aux_return_code_type) -> i32>,
    pub configure_fixed_vs_pe_retimer: Option<unsafe extern "C" fn(*mut ddc_service, *const u8, u32) -> bool>,
    pub aux_transfer_with_retries_no_mutex: Option<unsafe extern "C" fn(*mut ddc_service, *mut aux_payload) -> bool>,
    pub is_in_aux_transaction_mode: Option<unsafe extern "C" fn(*mut ddc_service) -> bool>,
    pub get_aux_defer_delay: Option<unsafe extern "C" fn(*mut ddc_service) -> u32>,
    pub get_ddc_aux_inst: Option<unsafe extern "C" fn(*const dc_link) -> u8>,
    pub dp_is_sink_present: Option<unsafe extern "C" fn(*mut dc_link) -> bool>,
    pub dp_is_fec_supported: Option<unsafe extern "C" fn(*const dc_link) -> bool>,
    pub dp_is_128b_132b_signal: Option<unsafe extern "C" fn(*mut pipe_ctx) -> bool>,
    pub dp_get_max_link_enc_cap: Option<unsafe extern "C" fn(*const dc_link, *mut dc_link_settings) -> bool>,
    pub dp_get_verified_link_cap: Option<unsafe extern "C" fn(*const dc_link) -> *const dc_link_settings>,
    pub dp_get_encoding_format: Option<unsafe extern "C" fn(*const dc_link_settings) -> dp_link_encoding>,
    pub dp_should_enable_fec: Option<unsafe extern "C" fn(*const dc_link) -> bool>,
    pub dp_decide_link_settings: Option<unsafe extern "C" fn(*mut dc_stream_state, *mut dc_link_settings) -> bool>,
    pub dp_decide_tunnel_settings: Option<unsafe extern "C" fn(*mut dc_stream_state, *mut dc_tunnel_settings)>,
    pub mst_decide_link_encoding_format: Option<unsafe extern "C" fn(*const dc_link) -> dp_link_encoding>,
    pub edp_decide_link_settings: Option<unsafe extern "C" fn(*mut dc_link, *mut dc_link_settings, u32) -> bool>,
    pub bw_kbps_from_raw_frl_link_rate_data: Option<unsafe extern "C" fn(u8) -> u32>,
    pub dp_overwrite_extended_receiver_cap: Option<unsafe extern "C" fn(*mut dc_link) -> bool>,
    pub dp_decide_lttpr_mode: Option<unsafe extern "C" fn(*mut dc_link, *mut dc_link_settings) -> lttpr_mode>,
    pub dp_get_lttpr_count: Option<unsafe extern "C" fn(*mut dc_link) -> u8>,
    pub edp_get_alpm_support: Option<unsafe extern "C" fn(*mut dc_link, *mut bool, *mut bool)>,
    pub dpia_handle_usb4_bandwidth_allocation_for_link: Option<unsafe extern "C" fn(*mut dc_link, i32)>,
    pub dp_set_drive_settings: Option<unsafe extern "C" fn(*mut dc_link, *const link_resource, *mut link_training_settings)>,
    pub dpcd_write_rx_power_ctrl: Option<unsafe extern "C" fn(*mut dc_link, bool)>,
    pub dp_parse_link_loss_status: Option<unsafe extern "C" fn(*mut dc_link, *mut hpd_irq_data) -> bool>,
    pub dp_should_allow_hpd_rx_irq: Option<unsafe extern "C" fn(*const dc_link) -> bool>,
    pub dp_handle_link_loss: Option<unsafe extern "C" fn(*mut dc_link)>,
    pub dp_read_hpd_rx_irq_data: Option<unsafe extern "C" fn(*mut dc_link, *mut hpd_irq_data) -> dc_status>,
    pub dp_handle_hpd_rx_irq: Option<unsafe extern "C" fn(*mut dc_link, *mut hpd_irq_data, *mut bool, bool, *mut bool) -> bool>,
    pub edp_panel_backlight_power_on: Option<unsafe extern "C" fn(*mut dc_link, bool)>,
    pub edp_get_backlight_level: Option<unsafe extern "C" fn(*const dc_link) -> i32>,
    pub edp_get_backlight_level_nits: Option<unsafe extern "C" fn(*mut dc_link, *mut u32, *mut u32) -> bool>,
    pub edp_set_backlight_level: Option<unsafe extern "C" fn(*const dc_link, *mut set_backlight_level_params) -> bool>,
    pub edp_set_backlight_level_nits: Option<unsafe extern "C" fn(*mut dc_link, bool, u32, u32) -> bool>,
    pub edp_get_target_backlight_pwm: Option<unsafe extern "C" fn(*const dc_link) -> i32>,
    pub edp_get_psr_state: Option<unsafe extern "C" fn(*const dc_link, *mut dc_psr_state) -> bool>,
    pub edp_set_psr_allow_active: Option<unsafe extern "C" fn(*mut dc_link, *const bool, bool, bool, *const u32) -> bool>,
    pub edp_setup_psr: Option<unsafe extern "C" fn(*mut dc_link, *const dc_stream_state, *mut psr_config, *mut psr_context) -> bool>,
    pub edp_set_sink_vtotal_in_psr_active: Option<unsafe extern "C" fn(*const dc_link, u16, u16) -> bool>,
    pub edp_get_psr_residency: Option<unsafe extern "C" fn(*const dc_link, *mut u32, psr_residency_mode)>,
    pub edp_get_replay_state: Option<unsafe extern "C" fn(*const dc_link, *mut u64) -> bool>,
    pub edp_set_replay_allow_active: Option<unsafe extern "C" fn(*mut dc_link, *const bool, bool, bool, *const u32) -> bool>,
    pub edp_send_replay_cmd: Option<unsafe extern "C" fn(*mut dc_link, replay_FW_Message_type, *mut dmub_replay_cmd_set) -> bool>,
    pub edp_set_coasting_vtotal: Option<unsafe extern "C" fn(*mut dc_link, u32, u16) -> bool>,
    pub edp_replay_residency: Option<unsafe extern "C" fn(*const dc_link, *mut u32, bool, pr_residency_mode) -> bool>,
    pub edp_set_replay_power_opt_and_coasting_vtotal: Option<unsafe extern "C" fn(*mut dc_link, *const u32, u32, u16) -> bool>,
    pub edp_wait_for_t12: Option<unsafe extern "C" fn(*mut dc_link) -> bool>,
    pub edp_is_ilr_optimization_required: Option<unsafe extern "C" fn(*mut dc_link, *mut dc_crtc_timing) -> bool>,
    pub edp_backlight_enable_aux: Option<unsafe extern "C" fn(*mut dc_link, bool) -> bool>,
    pub edp_add_delay_for_T9: Option<unsafe extern "C" fn(*mut dc_link)>,
    pub edp_receiver_ready_T9: Option<unsafe extern "C" fn(*mut dc_link) -> bool>,
    pub edp_receiver_ready_T7: Option<unsafe extern "C" fn(*mut dc_link) -> bool>,
    pub edp_power_alpm_dpcd_enable: Option<unsafe extern "C" fn(*mut dc_link, bool) -> bool>,
    pub dp_setup_replay: Option<unsafe extern "C" fn(*mut dc_link, *const dc_stream_state) -> bool>,
    pub dp_pr_get_panel_inst: Option<unsafe extern "C" fn(*const dc, *const dc_link, *mut u32) -> bool>,
    pub dp_pr_enable: Option<unsafe extern "C" fn(*mut dc_link, bool) -> bool>,
    pub dp_pr_update_state: Option<unsafe extern "C" fn(*mut dc_link, *mut dmub_cmd_pr_update_state_data) -> bool>,
    pub dp_pr_set_general_cmd: Option<unsafe extern "C" fn(*mut dc_link, *mut dmub_cmd_pr_general_cmd_data) -> bool>,
    pub dp_pr_get_state: Option<unsafe extern "C" fn(*const dc_link, *mut u64) -> bool>,
    pub edp_set_panel_power: Option<unsafe extern "C" fn(*mut dc_link, bool)>,
    pub hdmi_frl_poll_status_flag: Option<unsafe extern "C" fn(*mut dc_link) -> bool>,
    pub hdmi_frl_get_verified_link_cap: Option<unsafe extern "C" fn(*mut dc_link) -> *mut dc_hdmi_frl_link_settings>,
    pub hdmi_frl_set_preferred_link_settings: Option<unsafe extern "C" fn(*mut dc, *mut dc_hdmi_frl_link_settings, *mut dc_hdmi_frl_link_training_overrides, *mut dc_link)>,
    pub dp_handle_automated_test: Option<unsafe extern "C" fn(*mut dc_link)>,
    pub dp_set_test_pattern: Option<unsafe extern "C" fn(*mut dc_link, dp_test_pattern, dp_test_pattern_color_space, *const link_training_settings, *const u8, u32) -> bool>,
    pub dp_set_preferred_link_settings: Option<unsafe extern "C" fn(*mut dc, *mut dc_link_settings, *mut dc_link)>,
    pub dp_set_preferred_training_settings: Option<unsafe extern "C" fn(*mut dc, *mut dc_link_settings, *mut dc_link_training_overrides, *mut dc_link, bool)>,
    pub dp_trace_is_initialized: Option<unsafe extern "C" fn(*mut dc_link) -> bool>,
    pub dp_trace_set_is_logged_flag: Option<unsafe extern "C" fn(*mut dc_link, bool, bool)>,
    pub dp_trace_is_logged: Option<unsafe extern "C" fn(*mut dc_link, bool) -> bool>,
    pub dp_trace_get_lt_end_timestamp: Option<unsafe extern "C" fn(*mut dc_link, bool) -> u64>,
    pub dp_trace_get_lt_counts: Option<unsafe extern "C" fn(*mut dc_link, bool) -> *const dp_trace_lt_counts>,
    pub dp_trace_get_link_loss_count: Option<unsafe extern "C" fn(*mut dc_link) -> u32>,
    pub dp_trace_set_edp_power_timestamp: Option<unsafe extern "C" fn(*mut dc_link, bool)>,
    pub dp_trace_get_edp_poweron_timestamp: Option<unsafe extern "C" fn(*mut dc_link) -> u64>,
    pub dp_trace_get_edp_poweroff_timestamp: Option<unsafe extern "C" fn(*mut dc_link) -> u64>,
    pub dp_trace_source_sequence: Option<unsafe extern "C" fn(*mut dc_link, u8)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
