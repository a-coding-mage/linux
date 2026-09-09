/* Direct Rust translation of dc_link_exports.c. External types and members are
 * supplied by the surrounding display-core bindings. */

pub unsafe fn dc_get_link_at_index(dc: *mut dc, link_index: u32) -> *mut dc_link {
    if link_index >= MAX_LINKS { return core::ptr::null_mut(); }
    (*dc).links[link_index as usize]
}

pub unsafe fn dc_get_edp_links(dc: *const dc, edp_links: *mut *mut dc_link, edp_num: *mut u32) {
    *edp_num = 0;
    let mut i = 0;
    while i < (*dc).link_count {
        if (*dc).links[i as usize].is_null() { i += 1; continue; }
        if (*(*dc).links[i as usize]).connector_signal == SIGNAL_TYPE_EDP {
            *edp_links.add(*edp_num as usize) = (*dc).links[i as usize];
            *edp_num += 1;
            if *edp_num == MAX_NUM_EDP { return; }
        }
        i += 1;
    }
}

pub unsafe fn dc_get_edp_link_panel_inst(dc: *const dc, link: *const dc_link, inst_out: *mut u32) -> bool {
    let mut edp_links: [*mut dc_link; MAX_NUM_EDP as usize] = [core::ptr::null_mut(); MAX_NUM_EDP as usize];
    let mut edp_num = 0;
    *inst_out = 0;
    if (*link).connector_signal != SIGNAL_TYPE_EDP { return false; }
    dc_get_edp_links(dc, edp_links.as_mut_ptr(), &mut edp_num);
    let mut i = 0;
    while i < edp_num {
        if link == edp_links[i as usize] { break; }
        *inst_out += 1; i += 1;
    }
    true
}

pub unsafe fn dc_link_detect(link: *mut dc_link, reason: dc_detect_reason) -> bool { ((*(*link).dc).link_srv).detect_link(link, reason) }
pub unsafe fn dc_link_detect_connection_type(link: *mut dc_link, ty: *mut dc_connection_type) -> bool { ((*(*link).dc).link_srv).detect_connection_type(link, ty) }
pub unsafe fn dc_link_get_status(link: *const dc_link) -> *const dc_link_status { ((*(*link).dc).link_srv).get_status(link) }
pub unsafe fn dc_link_is_hdcp14(link: *mut dc_link, signal: signal_type) -> bool { ((*(*link).dc).link_srv).is_hdcp1x_supported(link, signal) }
pub unsafe fn dc_link_is_hdcp22(link: *mut dc_link, signal: signal_type) -> bool { ((*(*link).dc).link_srv).is_hdcp2x_supported(link, signal) }
pub unsafe fn dc_link_clear_dprx_states(link: *mut dc_link) { ((*(*link).dc).link_srv).clear_dprx_states(link) }
pub unsafe fn dc_link_reset_cur_dp_mst_topology(link: *mut dc_link) -> bool { ((*(*link).dc).link_srv).reset_cur_dp_mst_topology(link) }
pub unsafe fn dc_link_bandwidth_kbps(link: *const dc_link, settings: *const dc_link_settings) -> u32 { ((*(*link).dc).link_srv).dp_link_bandwidth_kbps(link, settings) }
pub unsafe fn dc_link_frl_bandwidth_kbps(link: *const dc_link, rate: hdmi_frl_link_rate) -> u32 { ((*(*link).dc).link_srv).frl_link_bandwidth_kbps(rate) }
pub unsafe fn dc_link_frl_margin_check_uncompressed_video(link: *const dc_link, params: *mut frl_cap_chk_params_fixed31_32, inter: *mut frl_cap_chk_intermediates_fixed31_32) -> bool { ((*(*link).dc).link_srv).frl_margin_check_uncompressed_video(params, inter) }
pub unsafe fn dc_link_required_hblank_size_bytes(link: *const dc_link, params: *mut dp_audio_bandwidth_params) -> u32 { ((*(*link).dc).link_srv).dp_required_hblank_size_bytes(link, params) }
pub unsafe fn dc_get_cur_link_res_map(dc: *const dc, map: *mut u32) { ((*dc).link_srv).get_cur_res_map(dc, map) }
pub unsafe fn dc_restore_link_res_map(dc: *const dc, map: *mut u32) { ((*dc).link_srv).restore_res_map(dc, map) }
pub unsafe fn dc_link_wait_for_unlocked(link: *mut dc_link) { ((*(*link).dc).link_srv).wait_for_unlocked(link) }
pub unsafe fn dc_link_update_dsc_config(pipe_ctx: *mut pipe_ctx) -> bool { let link = (*(*pipe_ctx).stream).link; ((*(*link).dc).link_srv).update_dsc_config(pipe_ctx) }
pub unsafe fn dc_get_oem_i2c_device(dc: *mut dc) -> *mut ddc_service { (*dc).res_pool.oem_device }
pub unsafe fn dc_is_oem_i2c_device_present(dc: *mut dc, slave_address: usize) -> bool { if !(*dc).res_pool.oem_device.is_null() { return dce_i2c_oem_device_present((*dc).res_pool, (*dc).res_pool.oem_device, slave_address); } false }
pub unsafe fn dc_submit_i2c(dc: *mut dc, link_index: u32, cmd: *mut i2c_command) -> bool { let link = (*dc).links[link_index as usize]; dce_i2c_submit_command((*dc).res_pool, (*link).ddc.ddc_pin, cmd) }
pub unsafe fn dc_submit_i2c_oem(dc: *mut dc, cmd: *mut i2c_command) -> bool { let ddc = (*dc).res_pool.oem_device; if !ddc.is_null() { return dce_i2c_submit_command((*dc).res_pool, (*ddc).ddc_pin, cmd); } false }

pub unsafe fn dc_link_dp_handle_automated_test(link: *mut dc_link) { ((*(*link).dc).link_srv).dp_handle_automated_test(link) }
pub unsafe fn dc_link_dp_set_test_pattern(link: *mut dc_link, pattern: dp_test_pattern, color: dp_test_pattern_color_space, settings: *const link_training_settings, custom: *const u8, size: u32) -> bool { ((*(*link).dc).link_srv).dp_set_test_pattern(link, pattern, color, settings, custom, size) }
pub unsafe fn dc_link_set_drive_settings(dc: *mut dc, settings: *mut link_training_settings, link: *mut dc_link) { let mut res = core::mem::MaybeUninit::<link_resource>::uninit(); ((*dc).link_srv).get_cur_link_res(link, res.as_mut_ptr()); ((*dc).link_srv).dp_set_drive_settings(link, res.as_mut_ptr(), settings) }
pub unsafe fn dc_link_set_preferred_link_settings(dc: *mut dc, setting: *mut dc_link_settings, link: *mut dc_link) { ((*dc).link_srv).dp_set_preferred_link_settings(dc, setting, link) }
pub unsafe fn dc_link_set_preferred_training_settings(dc: *mut dc, setting: *mut dc_link_settings, overrides: *mut dc_link_training_overrides, link: *mut dc_link, skip: bool) { ((*dc).link_srv).dp_set_preferred_training_settings(dc, setting, overrides, link, skip) }
pub unsafe fn dc_dp_trace_is_initialized(link: *mut dc_link) -> bool { ((*(*link).dc).link_srv).dp_trace_is_initialized(link) }
pub unsafe fn dc_dp_trace_set_is_logged_flag(link: *mut dc_link, detection: bool, logged: bool) { ((*(*link).dc).link_srv).dp_trace_set_is_logged_flag(link, detection, logged) }
pub unsafe fn dc_dp_trace_is_logged(link: *mut dc_link, detection: bool) -> bool { ((*(*link).dc).link_srv).dp_trace_is_logged(link, detection) }
pub unsafe fn dc_dp_trace_get_lt_end_timestamp(link: *mut dc_link, detection: bool) -> u64 { ((*(*link).dc).link_srv).dp_trace_get_lt_end_timestamp(link, detection) }
pub unsafe fn dc_dp_trace_get_lt_counts(link: *mut dc_link, detection: bool) -> *const dp_trace_lt_counts { ((*(*link).dc).link_srv).dp_trace_get_lt_counts(link, detection) }
pub unsafe fn dc_dp_trace_get_link_loss_count(link: *mut dc_link) -> u32 { ((*(*link).dc).link_srv).dp_trace_get_link_loss_count(link) }
pub unsafe fn dc_link_add_remote_sink(link: *mut dc_link, edid: *const u8, len: u32, init: *mut dc_sink_init_data) -> *mut dc_sink { ((*(*link).dc).link_srv).add_remote_sink(link, edid, len, init) }
pub unsafe fn dc_link_remove_remote_sink(link: *mut dc_link, sink: *mut dc_sink) { ((*(*link).dc).link_srv).remove_remote_sink(link, sink) }
pub unsafe fn dc_link_aux_transfer_raw(ddc: *mut ddc_service, payload: *mut aux_payload, result: *mut aux_return_code_type) -> i32 { ((*(*(*ddc).link).dc).link_srv).aux_transfer_raw(ddc, payload, result) }
pub unsafe fn dc_link_bw_kbps_from_raw_frl_link_rate_data(dc: *const dc, bw: u8) -> u32 { ((*dc).link_srv).bw_kbps_from_raw_frl_link_rate_data(bw) }
pub unsafe fn dc_link_decide_edp_link_settings(link: *mut dc_link, setting: *mut dc_link_settings, bw: u32) -> bool { ((*(*link).dc).link_srv).edp_decide_link_settings(link, setting, bw) }
pub unsafe fn dc_link_dp_get_max_link_enc_cap(link: *const dc_link, cap: *mut dc_link_settings) -> bool { ((*(*link).dc).link_srv).dp_get_max_link_enc_cap(link, cap) }
pub unsafe fn dc_link_dp_mst_decide_link_encoding_format(link: *const dc_link) -> dp_link_encoding { ((*(*link).dc).link_srv).mst_decide_link_encoding_format(link) }
pub unsafe fn dc_link_get_link_cap(link: *const dc_link) -> *const dc_link_settings { ((*(*link).dc).link_srv).dp_get_verified_link_cap(link) }

pub unsafe fn dc_link_get_highest_encoding_format(link: *const dc_link) -> dc_link_encoding_format {
    if dc_is_dp_signal((*link).connector_signal) {
        if (*link).dpcd_caps.dongle_type >= DISPLAY_DONGLE_DP_DVI_DONGLE && (*link).dpcd_caps.dongle_type <= DISPLAY_DONGLE_DP_HDMI_MISMATCHED_DONGLE { return DC_LINK_ENCODING_HDMI_TMDS; }
        let encoding = ((*(*link).dc).link_srv).dp_get_encoding_format(&(*link).verified_link_cap);
        if encoding == DP_8b_10b_ENCODING { return DC_LINK_ENCODING_DP_8b_10b; }
        if encoding == DP_128b_132b_ENCODING { return DC_LINK_ENCODING_DP_128b_132b; }
    } else if dc_is_hdmi_signal((*link).connector_signal) {
        let rate = (*link).frl_verified_link_cap.frl_link_rate;
        if rate == HDMI_FRL_LINK_RATE_DISABLE { return DC_LINK_ENCODING_HDMI_TMDS; }
        if rate >= HDMI_FRL_LINK_RATE_3GBPS && rate <= HDMI_FRL_LINK_RATE_12GBPS { return DC_LINK_ENCODING_HDMI_FRL; }
    }
    DC_LINK_ENCODING_UNSPECIFIED
}

pub unsafe fn dc_link_is_dp_sink_present(link: *mut dc_link) -> bool { ((*(*link).dc).link_srv).dp_is_sink_present(link) }
pub unsafe fn dc_link_is_fec_supported(link: *const dc_link) -> bool { ((*(*link).dc).link_srv).dp_is_fec_supported(link) }
pub unsafe fn dc_link_overwrite_extended_receiver_cap(link: *mut dc_link) { ((*(*link).dc).link_srv).dp_overwrite_extended_receiver_cap(link) }
pub unsafe fn dc_link_should_enable_fec(link: *const dc_link) -> bool { ((*(*link).dc).link_srv).dp_should_enable_fec(link) }
pub unsafe fn dc_link_dp_dpia_handle_usb4_bandwidth_allocation_for_link(link: *mut dc_link, peak_bw: i32) { ((*(*link).dc).link_srv).dpia_handle_usb4_bandwidth_allocation_for_link(link, peak_bw) }
pub unsafe fn dc_link_check_link_loss_status(link: *mut dc_link, data: *mut hpd_irq_data) -> bool { ((*(*link).dc).link_srv).dp_parse_link_loss_status(link, data) }
pub unsafe fn dc_link_dp_allow_hpd_rx_irq(link: *const dc_link) -> bool { ((*(*link).dc).link_srv).dp_should_allow_hpd_rx_irq(link) }
pub unsafe fn dc_link_dp_handle_link_loss(link: *mut dc_link) { ((*(*link).dc).link_srv).dp_handle_link_loss(link) }
pub unsafe fn dc_link_dp_read_hpd_rx_irq_data(link: *mut dc_link, data: *mut hpd_irq_data) -> dc_status { ((*(*link).dc).link_srv).dp_read_hpd_rx_irq_data(link, data) }
pub unsafe fn dc_link_handle_hpd_rx_irq(link: *mut dc_link, data: *mut hpd_irq_data, loss: *mut bool, defer: bool, work: *mut bool) -> bool { ((*(*link).dc).link_srv).dp_handle_hpd_rx_irq(link, data, loss, defer, work) }
pub unsafe fn dc_link_dp_receiver_power_ctrl(link: *mut dc_link, on: bool) { ((*(*link).dc).link_srv).dpcd_write_rx_power_ctrl(link, on) }
pub unsafe fn dc_link_decide_lttpr_mode(link: *mut dc_link, setting: *mut dc_link_settings) -> lttpr_mode { ((*(*link).dc).link_srv).dp_decide_lttpr_mode(link, setting) }
pub unsafe fn dc_link_edp_panel_backlight_power_on(link: *mut dc_link, wait: bool) { ((*(*link).dc).link_srv).edp_panel_backlight_power_on(link, wait) }
pub unsafe fn dc_link_get_backlight_level(link: *const dc_link) -> i32 { ((*(*link).dc).link_srv).edp_get_backlight_level(link) }
pub unsafe fn dc_link_get_backlight_level_nits(link: *mut dc_link, avg: *mut u32, peak: *mut u32) -> bool { ((*(*link).dc).link_srv).edp_get_backlight_level_nits(link, avg, peak) }
pub unsafe fn dc_link_set_backlight_level(link: *const dc_link, params: *mut set_backlight_level_params) -> bool { ((*(*link).dc).link_srv).edp_set_backlight_level(link, params) }
pub unsafe fn dc_link_set_backlight_level_nits(link: *mut dc_link, hdr: bool, nits: u32, transition: u32) -> bool { ((*(*link).dc).link_srv).edp_set_backlight_level_nits(link, hdr, nits, transition) }
pub unsafe fn dc_link_get_target_backlight_pwm(link: *const dc_link) -> i32 { ((*(*link).dc).link_srv).edp_get_target_backlight_pwm(link) }
pub unsafe fn dc_link_get_psr_state(link: *const dc_link, state: *mut dc_psr_state) -> bool { ((*(*link).dc).link_srv).edp_get_psr_state(link, state) }
pub unsafe fn dc_link_set_psr_allow_active(link: *mut dc_link, allow: *const bool, wait: bool, force_static: bool, opts: *const u32) -> bool { ((*(*link).dc).link_srv).edp_set_psr_allow_active(link, allow, wait, force_static, opts) }
pub unsafe fn dc_link_setup_psr(link: *mut dc_link, stream: *const dc_stream_state, config: *mut psr_config, context: *mut psr_context) -> bool { ((*(*link).dc).link_srv).edp_setup_psr(link, stream, config, context) }
pub unsafe fn dc_link_set_replay_allow_active(link: *mut dc_link, allow: *const bool, wait: bool, force_static: bool, opts: *const u32) -> bool { ((*(*link).dc).link_srv).edp_set_replay_allow_active(link, allow, wait, force_static, opts) }
pub unsafe fn dc_link_get_replay_state(link: *const dc_link, state: *mut u64) -> bool { ((*(*link).dc).link_srv).edp_get_replay_state(link, state) }
pub unsafe fn dc_link_set_pr_enable(link: *mut dc_link, enable: bool) -> bool { ((*(*link).dc).link_srv).dp_pr_enable(link, enable) }
pub unsafe fn dc_link_update_pr_state(link: *mut dc_link, data: *mut dmub_cmd_pr_update_state_data) -> bool { ((*(*link).dc).link_srv).dp_pr_update_state(link, data) }
pub unsafe fn dc_link_set_pr_general_cmd(link: *mut dc_link, data: *mut dmub_cmd_pr_general_cmd_data) -> bool { ((*(*link).dc).link_srv).dp_pr_set_general_cmd(link, data) }
pub unsafe fn dc_link_edp_replay_residency(link: *const dc_link, residency: *mut u32, start: bool, mode: pr_residency_mode) { if !link.is_null() && !(*link).dc.is_null() && !(*(*link).dc).link_srv.is_null() { ((*(*link).dc).link_srv).edp_replay_residency(link, residency, start, mode); } }
pub unsafe fn dc_link_get_pr_state(link: *const dc_link, state: *mut u64) -> bool { ((*(*link).dc).link_srv).dp_pr_get_state(link, state) }
pub unsafe fn dc_link_wait_for_t12(link: *mut dc_link) -> bool { ((*(*link).dc).link_srv).edp_wait_for_t12(link) }
pub unsafe fn dc_link_frl_poll_status_flag(link: *mut dc_link) -> bool { ((*(*link).dc).link_srv).hdmi_frl_poll_status_flag(link) }
pub unsafe fn dc_link_get_frl_link_cap(link: *mut dc_link) -> *mut dc_hdmi_frl_link_settings { ((*(*link).dc).link_srv).hdmi_frl_get_verified_link_cap(link) }
pub unsafe fn dc_link_set_preferred_frl_link_settings(dc: *mut dc, setting: *mut dc_hdmi_frl_link_settings, overrides: *mut dc_hdmi_frl_link_training_overrides, link: *mut dc_link) { ((*(*link).dc).link_srv).hdmi_frl_set_preferred_link_settings(dc, setting, overrides, link) }
pub unsafe fn dc_link_get_hpd_state(link: *mut dc_link) -> bool { ((*(*link).dc).link_srv).get_hpd_state(link) }
pub unsafe fn dc_link_enable_hpd(link: *const dc_link) { ((*(*link).dc).link_srv).enable_hpd(link) }
pub unsafe fn dc_link_disable_hpd(link: *const dc_link) { ((*(*link).dc).link_srv).disable_hpd(link) }
pub unsafe fn dc_link_enable_hpd_filter(link: *mut dc_link, enable: bool) { ((*(*link).dc).link_srv).enable_hpd_filter(link, enable) }
pub unsafe fn dc_link_validate_dp_tunneling_bandwidth(dc: *const dc, ctx: *const dc_state) -> dc_status { ((*dc).link_srv).validate_dp_tunnel_bandwidth(dc, ctx) }
pub unsafe fn dc_link_get_alpm_support(link: *mut dc_link, auxless: *mut bool, auxwake: *mut bool) { ((*(*link).dc).link_srv).edp_get_alpm_support(link, auxless, auxwake) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
