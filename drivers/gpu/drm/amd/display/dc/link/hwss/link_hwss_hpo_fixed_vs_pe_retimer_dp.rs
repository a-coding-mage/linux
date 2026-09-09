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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

unsafe fn dp_hpo_fixed_vs_pe_retimer_set_tx_ffe(
    link: *mut dc_link,
    hw_lane_settings: *const dc_lane_settings,
) {
    let vendor_ffe_preset_table: [u8; 16] = [
        0x01, 0x41, 0x61, 0x81, 0xB1, 0x05, 0x35, 0x65,
        0x85, 0xA5, 0x09, 0x39, 0x59, 0x89, 0x0F, 0x24,
    ];
    let mut ffe_mask = [0u8; 4];
    for i in 0..4 {
        let s = &(*hw_lane_settings.add(i)).FFE_PRESET.settings;
        ffe_mask[i] = (if s.no_deemphasis != 0 { 0x0F } else { 0xFF })
            & (if s.no_preshoot != 0 { 0xF1 } else { 0xFF });
    }
    let mut ffe_cfg = [0u8; 4];
    for i in 0..4 {
        ffe_cfg[i] = vendor_ffe_preset_table[(*hw_lane_settings.add(i)).FFE_PRESET.settings.level as usize]
            & ffe_mask[i];
    }
    let dp_type = dp_dio_fixed_vs_pe_retimer_lane_cfg_to_hw_cfg(link);
    let data = [
        [0x01, 0x50, dp_type, 0x0F],
        [0x01, 0x55, dp_type, ffe_cfg[0]],
        [0x01, 0x56, dp_type, ffe_cfg[1]],
        [0x01, 0x57, dp_type, ffe_cfg[2]],
        [0x01, 0x58, dp_type, ffe_cfg[3]],
    ];
    for entry in &data {
        (*(*link).dc).link_srv.configure_fixed_vs_pe_retimer((*link).ddc, entry.as_ptr(), entry.len());
    }
}

unsafe fn dp_hpo_fixed_vs_pe_retimer_program_override_test_pattern(
    link: *mut dc_link,
    tp_params: *mut encoder_set_dp_phy_pattern_param,
) {
    let _ = tp_params;
    let clk_src = 0xC4u8;
    let pattern = 0x4Fu8; // SQ128
    let data = [
        [0x1, 0x11, 0x0, 0x0], [0x1, 0x50, 0x50, clk_src],
        [0x1, 0x51, 0x50, clk_src], [0x1, 0x10, 0x58, 0x21],
        [0x1, 0x10, 0x59, 0x21], [0x1, 0x1C, 0x58, pattern],
        [0x1, 0x1C, 0x59, pattern], [0x1, 0x30, 0x51, 0x20],
        [0x1, 0x30, 0x52, 0x20], [0x1, 0x30, 0x54, 0x20],
        [0x1, 0x30, 0x55, 0x20],
    ];
    for (i, entry) in data.iter().enumerate() {
        if i == 7 && (*link).cur_link_settings.lane_count != LANE_COUNT_FOUR { continue; }
        if i == 9 && (*link).cur_link_settings.lane_count != LANE_COUNT_FOUR { continue; }
        (*(*link).dc).link_srv.configure_fixed_vs_pe_retimer((*link).ddc, entry.as_ptr(), entry.len());
    }
}

unsafe fn dp_hpo_fixed_vs_pe_retimer_set_override_test_pattern(
    link: *mut dc_link, link_res: *const link_resource,
    tp_params: *mut encoder_set_dp_phy_pattern_param, link_hwss: *const link_hwss,
) -> bool {
    let mut hw_tp_params: encoder_set_dp_phy_pattern_param = core::mem::zeroed();
    let exit_data = [0x1, 0x11, 0x0, 0x06];
    if !(*link).dpcd_caps.lttpr_caps.main_link_channel_coding.bits.DP_128b_132b_SUPPORTED || tp_params.is_null() { return false; }
    if !IS_DP_PHY_SQUARE_PATTERN((*tp_params).dp_phy_pattern) {
        if (*link).current_test_pattern == DP_TEST_PATTERN_80BIT_CUSTOM || (*link).current_test_pattern == DP_TEST_PATTERN_D102 {
            (*(*link).dc).link_srv.configure_fixed_vs_pe_retimer((*link).ddc, exit_data.as_ptr(), exit_data.len());
        } else if IS_DP_PHY_SQUARE_PATTERN((*link).current_test_pattern) { dp_dio_fixed_vs_pe_retimer_exit_manual_automation(link); }
        return false;
    }
    hw_tp_params.dp_phy_pattern = DP_TEST_PATTERN_PRBS31;
    hw_tp_params.dp_panel_mode = (*tp_params).dp_panel_mode;
    if let Some(f) = (*link_hwss).ext.set_dp_link_test_pattern { f(link, link_res, &mut hw_tp_params); }
    dp_hpo_fixed_vs_pe_retimer_program_override_test_pattern(link, tp_params);
    true
}

unsafe fn set_hpo_fixed_vs_pe_retimer_dp_link_test_pattern(link: *mut dc_link, link_res: *const link_resource, tp_params: *mut encoder_set_dp_phy_pattern_param) {
    if !dp_hpo_fixed_vs_pe_retimer_set_override_test_pattern(link, link_res, tp_params, get_hpo_dp_link_hwss()) { (*(*link_res).hpo_dp_link_enc).funcs.set_link_test_pattern((*link_res).hpo_dp_link_enc, tp_params); }
    (*(*link).dc).link_srv.dp_trace_source_sequence(link, DPCD_SOURCE_SEQ_AFTER_SET_SOURCE_PATTERN);
    if (*tp_params).dp_phy_pattern != DP_TEST_PATTERN_128b_132b_TPS2_TRAINING_MODE { msleep(50); }
}

unsafe fn set_hpo_fixed_vs_pe_retimer_dp_lane_settings(link: *mut dc_link, link_res: *const link_resource, link_settings: *const dc_link_settings, lane_settings: *const dc_lane_settings) {
    if IS_DP_PHY_PATTERN((*link).pending_test_pattern) {
        if IS_DP_PHY_SQUARE_PATTERN((*link).pending_test_pattern) { dp_hpo_fixed_vs_pe_retimer_set_tx_ffe(link, lane_settings); }
    } else { (*(*link_res).hpo_dp_link_enc).funcs.set_ffe((*link_res).hpo_dp_link_enc, link_settings, (*lane_settings).FFE_PRESET.raw); }
}

unsafe fn enable_hpo_fixed_vs_pe_retimer_dp_link_output(link: *mut dc_link, link_res: *const link_resource, signal: enum_signal_type, clock_source: enum_clock_source_id, link_settings: *const dc_link_settings) {
    if (*link_settings).lane_count == LANE_COUNT_FOUR { enable_dio_fixed_vs_pe_retimer_program_4lane_output(link); }
    enable_hpo_dp_link_output(link, link_res, signal, clock_source, link_settings);
}

static mut hpo_fixed_vs_pe_retimer_dp_link_hwss: link_hwss = link_hwss {
    setup_stream_encoder: Some(setup_hpo_dp_stream_encoder), reset_stream_encoder: Some(reset_hpo_dp_stream_encoder), setup_stream_attribute: Some(setup_hpo_dp_stream_attribute), disable_link_output: Some(disable_hpo_dp_link_output), setup_audio_output: Some(setup_hpo_dp_audio_output), enable_audio_packet: Some(enable_hpo_dp_audio_packet), disable_audio_packet: Some(disable_hpo_dp_audio_packet),
    ext: link_hwss_ext { set_throttled_vcp_size: Some(set_hpo_dp_throttled_vcp_size), set_hblank_min_symbol_width: Some(set_hpo_dp_hblank_min_symbol_width), enable_dp_link_output: Some(enable_hpo_fixed_vs_pe_retimer_dp_link_output), set_dp_link_test_pattern: Some(set_hpo_fixed_vs_pe_retimer_dp_link_test_pattern), set_dp_lane_settings: Some(set_hpo_fixed_vs_pe_retimer_dp_lane_settings), update_stream_allocation_table: Some(update_hpo_dp_stream_allocation_table) },
};

unsafe fn requires_fixed_vs_pe_retimer_hpo_link_hwss(link: *const dc_link) -> bool { requires_fixed_vs_pe_retimer_dio_link_hwss(link) }
unsafe fn get_hpo_fixed_vs_pe_retimer_dp_link_hwss() -> *const link_hwss { &raw const hpo_fixed_vs_pe_retimer_dp_link_hwss }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
