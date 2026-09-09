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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

/* This file implements dp 8b/10b link training software policies and sequences. */

// Types, constants, unions, globals, and external functions are supplied by
// the surrounding Display Core translation unit.

unsafe fn get_default_8b_10b_lttpr_aux_rd_interval(
    training_rd_interval: *mut training_aux_rd_interval,
) {
    (*training_rd_interval).raw = 0x4;
}

unsafe fn get_cr_training_aux_rd_interval(
    link: *mut dc_link,
    link_settings: *const dc_link_settings,
    lttpr_mode: lttpr_mode,
) -> i32 {
    let mut training_rd_interval: training_aux_rd_interval = core::mem::zeroed();
    let mut wait_in_micro_secs: u32 = 100;
    if link_dp_get_encoding_format(link_settings) == DP_8b_10b_ENCODING {
        if (*link).dpcd_caps.dpcd_rev.raw >= DPCD_REV_12 {
            core_link_read_dpcd(link, DP_TRAINING_AUX_RD_INTERVAL,
                &mut training_rd_interval as *mut _ as *mut u8,
                core::mem::size_of::<training_aux_rd_interval>());
        } else if dp_is_lttpr_present(link) {
            get_default_8b_10b_lttpr_aux_rd_interval(&mut training_rd_interval);
        }
        if training_rd_interval.raw != 0 {
            if lttpr_mode != LTTPR_MODE_NON_TRANSPARENT { wait_in_micro_secs = 400; }
            if training_rd_interval.bits.TRAINIG_AUX_RD_INTERVAL != 0 {
                wait_in_micro_secs = training_rd_interval.bits.TRAINIG_AUX_RD_INTERVAL * 4000;
            }
        }
    }
    wait_in_micro_secs as i32
}

unsafe fn get_eq_training_aux_rd_interval(
    link: *mut dc_link, link_settings: *const dc_link_settings,
) -> u32 {
    let mut training_rd_interval: training_aux_rd_interval = core::mem::zeroed();
    if link_dp_get_encoding_format(link_settings) == DP_128b_132b_ENCODING {
        core_link_read_dpcd(link, DP_128B132B_TRAINING_AUX_RD_INTERVAL,
            &mut training_rd_interval as *mut _ as *mut u8,
            core::mem::size_of::<training_aux_rd_interval>());
    } else if link_dp_get_encoding_format(link_settings) == DP_8b_10b_ENCODING {
        if (*link).dpcd_caps.dpcd_rev.raw >= DPCD_REV_12 {
            core_link_read_dpcd(link, DP_TRAINING_AUX_RD_INTERVAL,
                &mut training_rd_interval as *mut _ as *mut u8,
                core::mem::size_of::<training_aux_rd_interval>());
        } else if dp_is_lttpr_present(link) {
            get_default_8b_10b_lttpr_aux_rd_interval(&mut training_rd_interval);
        }
    }
    match training_rd_interval.bits.TRAINIG_AUX_RD_INTERVAL {
        0 => 400, 1 => 4000, 2 => 8000, 3 => 12000, 4 => 16000,
        5 => 32000, 6 => 64000, _ => 400,
    }
}

pub unsafe fn decide_8b_10b_training_settings(
    link: *mut dc_link, link_res: *const link_resource,
    link_setting: *const dc_link_settings, lt_settings: *mut link_training_settings,
) {
    core::ptr::write_bytes(lt_settings, 0, 1);
    (*lt_settings).link_settings.use_link_rate_set = (*link_setting).use_link_rate_set;
    (*lt_settings).link_settings.link_rate_set = (*link_setting).link_rate_set;
    (*lt_settings).link_settings.link_rate = (*link_setting).link_rate;
    (*lt_settings).link_settings.lane_count = (*link_setting).lane_count;
    (*lt_settings).link_settings.link_spread = if (*link).dp_ss_off { LINK_SPREAD_DISABLED } else { LINK_SPREAD_05_DOWNSPREAD_30KHZ };
    (*lt_settings).eq_pattern_time = get_eq_training_aux_rd_interval(link, link_setting) as u16;
    (*lt_settings).pattern_for_cr = decide_cr_training_pattern(link_setting);
    (*lt_settings).pattern_for_eq = decide_eq_training_pattern(link, link_res, link_setting);
    (*lt_settings).enhanced_framing = 1;
    (*lt_settings).should_set_fec_ready = true;
    (*lt_settings).disallow_per_lane_settings = true;
    (*lt_settings).always_match_dpcd_with_hw_lane_settings = true;
    (*lt_settings).lttpr_mode = dp_decide_8b_10b_lttpr_mode(link);
    (*lt_settings).cr_pattern_time = get_cr_training_aux_rd_interval(link, link_setting, (*lt_settings).lttpr_mode) as u16;
    dp_hw_to_dpcd_lane_settings(lt_settings, (*lt_settings).hw_lane_settings, (*lt_settings).dpcd_lane_settings);
    if ((*link).chip_caps & AMD_EXT_DISPLAY_PATH_CAPS__EXT_CHIP_MASK) == AMD_EXT_DISPLAY_PATH_CAPS__DP_EARLY_8B10B_TPS2 {
        (*lt_settings).lttpr_early_tps2 = true;
    }
}

pub unsafe fn dp_decide_8b_10b_lttpr_mode(link: *mut dc_link) -> lttpr_mode {
    let present = dp_is_lttpr_present(link);
    let force_non_transparent = (*(*link).dc).caps.vbios_lttpr_enable;
    let aware = (*(*link).dc).caps.vbios_lttpr_aware;
    if !present { return LTTPR_MODE_NON_LTTPR; }
    if aware {
        if force_non_transparent { return LTTPR_MODE_NON_TRANSPARENT; }
        return LTTPR_MODE_TRANSPARENT;
    }
    if (*(*link).dc).config.allow_lttpr_non_transparent_mode.bits.DP1_4A && (*(*link).dc).caps.extended_aux_timeout_support {
        return LTTPR_MODE_NON_TRANSPARENT;
    }
    LTTPR_MODE_NON_LTTPR
}

unsafe fn set_link_settings_and_perform_early_tps2_retimer_pre_lt_sequence(link: *mut dc_link, link_res: *const link_resource, lt_settings: *mut link_training_settings, lttpr_count: u32) {
    let offset = dp_get_closest_lttpr_offset(lttpr_count as u8);
    let mut pattern: dpcd_training_pattern = core::mem::zeroed();
    pattern.v1_4.TRAINING_PATTERN_SET = 1;
    pattern.v1_4.SCRAMBLING_DISABLE = 1;
    dp_set_hw_training_pattern(link, link_res, DP_TRAINING_PATTERN_SEQUENCE_2, DPRX);
    dp_set_hw_lane_settings(link, link_res, lt_settings, DPRX);
    udelay(400);
    dpcd_set_link_settings(link, lt_settings);
    core_link_write_dpcd(link, DP_TRAINING_PATTERN_SET_PHY_REPEATER1 + offset, &pattern.raw, 1);
    udelay(1000);
}

pub unsafe fn perform_8b_10b_clock_recovery_sequence(link: *mut dc_link, link_res: *const link_resource, lt_settings: *mut link_training_settings, offset: u32) -> link_training_result {
    let mut retries_cr = 0u32;
    let mut retry_count = 0u32;
    let mut status: dc_status;
    let lane_count = (*lt_settings).link_settings.lane_count;
    let mut lane_status: [lane_status; LANE_COUNT_DP_MAX] = core::mem::zeroed();
    let mut updated: lane_align_status_updated = core::mem::zeroed();
    let mut adjust: [lane_adjust; LANE_COUNT_DP_MAX] = core::mem::zeroed();
    if !(*(*link).ctx).dc.work_arounds.lt_early_cr_pattern { dp_set_hw_training_pattern(link, link_res, (*lt_settings).pattern_for_cr, offset); }
    while retries_cr < LINK_TRAINING_MAX_RETRY_COUNT && retry_count < LINK_TRAINING_MAX_CR_RETRY {
        dp_set_hw_lane_settings(link, link_res, lt_settings, offset);
        if retry_count == 0 { dpcd_set_lt_pattern_and_lane_settings(link, lt_settings, (*lt_settings).pattern_for_cr, offset); } else { dpcd_set_lane_settings(link, lt_settings, offset); }
        dp_wait_for_training_aux_rd_interval(link, (*lt_settings).cr_pattern_time as u32);
        status = dp_get_lane_status_and_lane_adjust(link, lt_settings, lane_status.as_mut_ptr(), &mut updated, adjust.as_mut_ptr(), offset);
        if dp_check_dpcd_reqeust_status(link, status) { return LINK_TRAINING_ABORT; }
        if dp_is_cr_done(lane_count, lane_status.as_ptr()) { return LINK_TRAINING_SUCCESS; }
        if link_dp_get_encoding_format(&(*lt_settings).link_settings) == DP_8b_10b_ENCODING && dp_is_max_vs_reached(lt_settings) { break; }
        if link_dp_get_encoding_format(&(*lt_settings).link_settings) == DP_8b_10b_ENCODING && (*lt_settings).dpcd_lane_settings[0].bits.VOLTAGE_SWING_SET == adjust[0].bits.VOLTAGE_SWING_LANE || link_dp_get_encoding_format(&(*lt_settings).link_settings) == DP_128b_132b_ENCODING && (*lt_settings).dpcd_lane_settings[0].tx_ffe.PRESET_VALUE == adjust[0].tx_ffe.PRESET_VALUE { retries_cr += 1; } else { retries_cr = 0; }
        dp_decide_lane_settings(lt_settings, adjust.as_ptr(), (*lt_settings).hw_lane_settings, (*lt_settings).dpcd_lane_settings);
        retry_count += 1;
    }
    if retry_count >= LINK_TRAINING_MAX_CR_RETRY { ASSERT(0); }
    dp_get_cr_failure(lane_count, lane_status.as_ptr())
}

pub unsafe fn perform_8b_10b_channel_equalization_sequence(link: *mut dc_link, link_res: *const link_resource, lt_settings: *mut link_training_settings, offset: u32) -> link_training_result {
    let mut pattern = (*lt_settings).pattern_for_eq;
    let lane_count = (*lt_settings).link_settings.lane_count;
    if is_repeater(lt_settings, offset) && link_dp_get_encoding_format(&(*lt_settings).link_settings) == DP_8b_10b_ENCODING { pattern = DP_TRAINING_PATTERN_SEQUENCE_4; }
    dp_set_hw_training_pattern(link, link_res, pattern, offset);
    let mut updated: lane_align_status_updated = core::mem::zeroed();
    let mut statuses: [lane_status; LANE_COUNT_DP_MAX] = core::mem::zeroed();
    let mut adjust: [lane_adjust; LANE_COUNT_DP_MAX] = core::mem::zeroed();
    for retry in 0..=LINK_TRAINING_MAX_RETRY_COUNT {
        dp_set_hw_lane_settings(link, link_res, lt_settings, offset);
        if retry == 0 { dpcd_set_lt_pattern_and_lane_settings(link, lt_settings, pattern, offset); } else { dpcd_set_lane_settings(link, lt_settings, offset); }
        let wait = dp_get_eq_aux_rd_interval(link, lt_settings, offset, retry as u8);
        dp_wait_for_training_aux_rd_interval(link, wait);
        let status = dp_get_lane_status_and_lane_adjust(link, lt_settings, statuses.as_mut_ptr(), &mut updated, adjust.as_mut_ptr(), offset);
        if dp_check_dpcd_reqeust_status(link, status) { return LINK_TRAINING_ABORT; }
        if !dp_is_cr_done(lane_count, statuses.as_ptr()) { return if statuses[0].bits.CR_DONE_0 { LINK_TRAINING_EQ_FAIL_CR_PARTIAL } else { LINK_TRAINING_EQ_FAIL_CR }; }
        if dp_is_ch_eq_done(lane_count, statuses.as_ptr()) && dp_is_symbol_locked(lane_count, statuses.as_ptr()) && dp_check_interlane_aligned(updated, link, retry as u8) { return LINK_TRAINING_SUCCESS; }
        dp_decide_lane_settings(lt_settings, adjust.as_ptr(), (*lt_settings).hw_lane_settings, (*lt_settings).dpcd_lane_settings);
    }
    LINK_TRAINING_EQ_FAIL_EQ
}

pub unsafe fn dp_perform_8b_10b_link_training(link: *mut dc_link, link_res: *const link_resource, lt_settings: *mut link_training_settings) -> link_training_result {
    let mut status = LINK_TRAINING_SUCCESS;
    let count = dp_parse_lttpr_repeater_count((*link).dpcd_caps.lttpr_caps.phy_repeater_cnt);
    if (*(*link).ctx).dc.work_arounds.lt_early_cr_pattern { start_clock_recovery_pattern_early(link, link_res, lt_settings, DPRX); }
    if (*lt_settings).lttpr_early_tps2 { set_link_settings_and_perform_early_tps2_retimer_pre_lt_sequence(link, link_res, lt_settings, count as u32); } else { dpcd_set_link_settings(link, lt_settings); }
    if (*lt_settings).lttpr_mode == LTTPR_MODE_NON_TRANSPARENT {
        for id in (1..=count).rev() {
            status = perform_8b_10b_clock_recovery_sequence(link, link_res, lt_settings, id as u32);
            if status != LINK_TRAINING_SUCCESS { repeater_training_done(link, id); break; }
            status = perform_8b_10b_channel_equalization_sequence(link, link_res, lt_settings, id as u32);
            repeater_training_done(link, id);
            if status != LINK_TRAINING_SUCCESS { break; }
            for lane in 0..LANE_COUNT_DP_MAX { (*lt_settings).dpcd_lane_settings[lane].raw = 0; (*lt_settings).hw_lane_settings[lane].VOLTAGE_SWING = 0; (*lt_settings).hw_lane_settings[lane].PRE_EMPHASIS = 0; }
        }
    }
    if status == LINK_TRAINING_SUCCESS { status = perform_8b_10b_clock_recovery_sequence(link, link_res, lt_settings, DPRX); if status == LINK_TRAINING_SUCCESS { status = perform_8b_10b_channel_equalization_sequence(link, link_res, lt_settings, DPRX); } }
    status
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
