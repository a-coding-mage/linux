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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

pub unsafe fn dp_dio_fixed_vs_pe_retimer_lane_cfg_to_hw_cfg(link: *mut dc_link) -> u8 {
    // TODO: Get USB-C cable orientation
    if (*link).cur_link_settings.lane_count == LANE_COUNT_FOUR {
        0xF2
    } else {
        0x12
    }
}

pub unsafe fn dp_dio_fixed_vs_pe_retimer_exit_manual_automation(link: *mut dc_link) {
    let dp_type = dp_dio_fixed_vs_pe_retimer_lane_cfg_to_hw_cfg(link);
    let vendor_lttpr_exit_manual_automation_0: [u8; 4] = [0x1, 0x11, 0x0, 0x06];
    let vendor_lttpr_exit_manual_automation_1: [u8; 4] = [0x1, 0x50, dp_type, 0x0];
    let vendor_lttpr_exit_manual_automation_2: [u8; 4] = [0x1, 0x50, 0x50, 0x0];
    let vendor_lttpr_exit_manual_automation_3: [u8; 4] = [0x1, 0x51, 0x50, 0x0];
    let vendor_lttpr_exit_manual_automation_4: [u8; 4] = [0x1, 0x10, 0x58, 0x0];
    let vendor_lttpr_exit_manual_automation_5: [u8; 4] = [0x1, 0x10, 0x59, 0x0];
    let vendor_lttpr_exit_manual_automation_6: [u8; 4] = [0x1, 0x30, 0x51, 0x0];
    let vendor_lttpr_exit_manual_automation_7: [u8; 4] = [0x1, 0x30, 0x52, 0x0];
    let vendor_lttpr_exit_manual_automation_8: [u8; 4] = [0x1, 0x30, 0x54, 0x0];
    let vendor_lttpr_exit_manual_automation_9: [u8; 4] = [0x1, 0x30, 0x55, 0x0];
    let data = [
        &vendor_lttpr_exit_manual_automation_0,
        &vendor_lttpr_exit_manual_automation_1,
        &vendor_lttpr_exit_manual_automation_2,
        &vendor_lttpr_exit_manual_automation_3,
        &vendor_lttpr_exit_manual_automation_4,
        &vendor_lttpr_exit_manual_automation_5,
        &vendor_lttpr_exit_manual_automation_6,
        &vendor_lttpr_exit_manual_automation_7,
        &vendor_lttpr_exit_manual_automation_8,
        &vendor_lttpr_exit_manual_automation_9,
    ];
    for item in data {
        ((*(*link).dc).link_srv).configure_fixed_vs_pe_retimer((*link).ddc, item.as_ptr(), item.len());
    }
}

unsafe fn set_dio_fixed_vs_pe_retimer_dp_link_test_pattern_override(
    link: *mut dc_link,
    link_res: *const link_resource,
    tp_params: *mut encoder_set_dp_phy_pattern_param,
    link_hwss: *const link_hwss,
) -> bool {
    let mut hw_tp_params: encoder_set_dp_phy_pattern_param = core::mem::zeroed();
    let pltpat_custom: [u8; 10] = [0x1F, 0x7C, 0xF0, 0xC1, 0x07, 0x1F, 0x7C, 0xF0, 0xC1, 0x07];
    let vendor_lttpr_write_data_pg0: [u8; 4] = [0x1, 0x11, 0x0, 0x0];
    let vendor_lttpr_exit_manual_automation_0: [u8; 4] = [0x1, 0x11, 0x0, 0x06];

    if !(*link).dpcd_caps.lttpr_caps.main_link_channel_coding.bits.DP_128b_132b_SUPPORTED { return false; }
    if tp_params.is_null() { return false; }
    if IS_DP_PHY_SQUARE_PATTERN((*link).current_test_pattern) {
        dp_dio_fixed_vs_pe_retimer_exit_manual_automation(link);
    }
    match (*tp_params).dp_phy_pattern {
        DP_TEST_PATTERN_80BIT_CUSTOM => {
            if (*tp_params).custom_pattern_size == 0 ||
                libc::memcmp((*tp_params).custom_pattern as *const _, pltpat_custom.as_ptr() as *const _, (*tp_params).custom_pattern_size) != 0 { return false; }
            hw_tp_params.custom_pattern = (*tp_params).custom_pattern;
            hw_tp_params.custom_pattern_size = (*tp_params).custom_pattern_size;
        }
        DP_TEST_PATTERN_D102 => {}
        _ => {
            if (*link).current_test_pattern == DP_TEST_PATTERN_80BIT_CUSTOM || (*link).current_test_pattern == DP_TEST_PATTERN_D102 {
                ((*(*link).dc).link_srv).configure_fixed_vs_pe_retimer((*link).ddc, vendor_lttpr_exit_manual_automation_0.as_ptr(), vendor_lttpr_exit_manual_automation_0.len());
            }
            return false;
        }
    }
    hw_tp_params.dp_phy_pattern = (*tp_params).dp_phy_pattern;
    hw_tp_params.dp_panel_mode = (*tp_params).dp_panel_mode;
    if !(*link_hwss).ext.set_dp_link_test_pattern.is_none() {
        ((*link_hwss).ext.set_dp_link_test_pattern.unwrap())(link, link_res, &mut hw_tp_params);
    }
    ((*(*link).dc).link_srv).configure_fixed_vs_pe_retimer((*link).ddc, vendor_lttpr_write_data_pg0.as_ptr(), vendor_lttpr_write_data_pg0.len());
    true
}

unsafe fn set_dio_fixed_vs_pe_retimer_dp_link_test_pattern(link: *mut dc_link, link_res: *const link_resource, tp_params: *mut encoder_set_dp_phy_pattern_param) {
    let mut link_enc = (*link_res).dio_link_enc;
    if !(*(*link).dc).config.unify_link_enc_assignment { link_enc = link_enc_cfg_get_link_enc(link); }
    if !set_dio_fixed_vs_pe_retimer_dp_link_test_pattern_override(link, link_res, tp_params, get_dio_link_hwss()) {
        ((*link_enc).funcs).dp_set_phy_pattern(link_enc, tp_params);
    }
    ((*(*link).dc).link_srv).dp_trace_source_sequence(link, DPCD_SOURCE_SEQ_AFTER_SET_SOURCE_PATTERN);
}

pub unsafe fn enable_dio_fixed_vs_pe_retimer_program_4lane_output(link: *mut dc_link) {
    let data: [[u8; 4]; 5] = [[0x1,0x6E,0xF2,0x19],[0x1,0x6B,0xF2,0x01],[0x1,0x6D,0xF2,0x18],[0x1,0x6C,0xF2,0x03],[0x1,0x03,0xF3,0x06]];
    for item in data { ((*(*link).dc).link_srv).configure_fixed_vs_pe_retimer((*link).ddc, item.as_ptr(), item.len()); }
}

unsafe fn enable_dio_fixed_vs_pe_retimer_dp_link_output(link: *mut dc_link, link_res: *const link_resource, signal: signal_type, clock_source: clock_source_id, link_settings: *const dc_link_settings) {
    if (*link_settings).lane_count == LANE_COUNT_FOUR { enable_dio_fixed_vs_pe_retimer_program_4lane_output(link); }
    enable_dio_dp_link_output(link, link_res, signal, clock_source, link_settings);
}

static mut dio_fixed_vs_pe_retimer_link_hwss: link_hwss = link_hwss {
    setup_stream_encoder: Some(setup_dio_stream_encoder), reset_stream_encoder: Some(reset_dio_stream_encoder), setup_stream_attribute: Some(setup_dio_stream_attribute), disable_link_output: Some(disable_dio_link_output), setup_audio_output: Some(setup_dio_audio_output), enable_audio_packet: Some(enable_dio_audio_packet), disable_audio_packet: Some(disable_dio_audio_packet), ext: link_hwss_ext { set_throttled_vcp_size: Some(set_dio_throttled_vcp_size), enable_dp_link_output: Some(enable_dio_fixed_vs_pe_retimer_dp_link_output), set_dp_link_test_pattern: Some(set_dio_fixed_vs_pe_retimer_dp_link_test_pattern), set_dp_lane_settings: Some(set_dio_dp_lane_settings), update_stream_allocation_table: Some(update_dio_stream_allocation_table) },
};

pub unsafe fn requires_fixed_vs_pe_retimer_dio_link_hwss(link: *const dc_link) -> bool { ((*link).chip_caps & AMD_EXT_DISPLAY_PATH_CAPS__EXT_CHIP_MASK) == AMD_EXT_DISPLAY_PATH_CAPS__DP_FIXED_VS_EN }
pub unsafe fn get_dio_fixed_vs_pe_retimer_link_hwss() -> *const link_hwss { &raw const dio_fixed_vs_pe_retimer_link_hwss }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
