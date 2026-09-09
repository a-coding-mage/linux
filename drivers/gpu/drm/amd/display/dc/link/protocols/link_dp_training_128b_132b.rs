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
 */

/* This file implements dp 128b/132b link training software policies and sequences. */

unsafe fn dpcd_128b_132b_set_lane_settings(
    link: *mut dc_link,
    link_training_setting: *const link_training_settings,
) -> dc_status {
    let status = core_link_write_dpcd(
        link,
        DP_TRAINING_LANE0_SET,
        (*link_training_setting).dpcd_lane_settings as *mut u8,
        core::mem::size_of_val(&(*link_training_setting).dpcd_lane_settings),
    );
    DC_LOG_HW_LINK_TRAINING!("{}:\n 0x{:X} TX_FFE_PRESET_VALUE = {:x}\n", "dpcd_128b_132b_set_lane_settings", DP_TRAINING_LANE0_SET, (*link_training_setting).dpcd_lane_settings[0].tx_ffe.PRESET_VALUE);
    status
}

unsafe fn dpcd_128b_132b_get_aux_rd_interval(link: *mut dc_link, interval_in_us: *mut u32) {
    let mut dpcd_interval: dp_128b_132b_training_aux_rd_interval = core::mem::zeroed();
    let mut interval_unit: u32 = 0;
    core_link_read_dpcd(link, DP_128B132B_TRAINING_AUX_RD_INTERVAL, &mut dpcd_interval.raw, core::mem::size_of_val(&dpcd_interval.raw));
    interval_unit = if dpcd_interval.bits.UNIT { 1 } else { 2 }; /* 0b = 2 ms, 1b = 1 ms */
    /* (128b/132b_TRAINING_AUX_RD_INTERVAL value + 1) * INTERVAL_UNIT. */
    *interval_in_us = (dpcd_interval.bits.VALUE + 1) * interval_unit * 1000;
}

unsafe fn dp_perform_128b_132b_channel_eq_done_sequence(link: *mut dc_link, link_res: *const link_resource, lt_settings: *mut link_training_settings) -> link_training_result {
    let mut loop_count: u8 = 1;
    let mut aux_rd_interval: u32 = 0;
    let mut wait_time: u32 = 0;
    let mut dpcd_lane_status_updated: lane_align_status_updated = core::mem::zeroed();
    let mut dpcd_lane_status: [lane_status; LANE_COUNT_DP_MAX] = core::mem::zeroed();
    let mut dpcd_lane_adjust: [lane_adjust; LANE_COUNT_DP_MAX] = core::mem::zeroed();
    let mut status = DC_OK;
    let mut result = LINK_TRAINING_SUCCESS;
    dp_set_hw_training_pattern(link, link_res, (*lt_settings).pattern_for_cr, DPRX);
    dpcd_set_training_pattern(link, (*lt_settings).pattern_for_cr);
    dpcd_128b_132b_get_aux_rd_interval(link, &mut aux_rd_interval);
    dp_get_lane_status_and_lane_adjust(link, lt_settings, dpcd_lane_status.as_mut_ptr(), &mut dpcd_lane_status_updated, dpcd_lane_adjust.as_mut_ptr(), DPRX);
    dp_decide_lane_settings(lt_settings, dpcd_lane_adjust.as_mut_ptr(), (*lt_settings).hw_lane_settings.as_mut_ptr(), (*lt_settings).dpcd_lane_settings.as_mut_ptr());
    dp_set_hw_lane_settings(link, link_res, lt_settings, DPRX);
    dp_set_hw_training_pattern(link, link_res, (*lt_settings).pattern_for_eq, DPRX);
    dpcd_set_lt_pattern_and_lane_settings(link, lt_settings, (*lt_settings).pattern_for_eq, DPRX);
    while result == LINK_TRAINING_SUCCESS {
        dp_wait_for_training_aux_rd_interval(link, aux_rd_interval);
        wait_time += aux_rd_interval;
        status = dp_get_lane_status_and_lane_adjust(link, lt_settings, dpcd_lane_status.as_mut_ptr(), &mut dpcd_lane_status_updated, dpcd_lane_adjust.as_mut_ptr(), DPRX);
        dp_decide_lane_settings(lt_settings, dpcd_lane_adjust.as_mut_ptr(), (*lt_settings).hw_lane_settings.as_mut_ptr(), (*lt_settings).dpcd_lane_settings.as_mut_ptr());
        dpcd_128b_132b_get_aux_rd_interval(link, &mut aux_rd_interval);
        if status != DC_OK { result = LINK_TRAINING_ABORT; }
        else if dp_is_ch_eq_done((*lt_settings).link_settings.lane_count, dpcd_lane_status.as_mut_ptr()) { break; }
        else if loop_count >= (*lt_settings).eq_loop_count_limit { result = DP_128b_132b_MAX_LOOP_COUNT_REACHED; }
        else if dpcd_lane_status_updated.bits.LT_FAILED_128b_132b { result = DP_128b_132b_LT_FAILED; }
        else { dp_set_hw_lane_settings(link, link_res, lt_settings, DPRX); dpcd_128b_132b_set_lane_settings(link, lt_settings); }
        loop_count += 1;
    }
    while result == LINK_TRAINING_SUCCESS {
        if status != DC_OK { result = LINK_TRAINING_ABORT; }
        else if dpcd_lane_status_updated.bits.EQ_INTERLANE_ALIGN_DONE_128b_132b { break; }
        else if wait_time >= (*lt_settings).eq_wait_time_limit { result = DP_128b_132b_CHANNEL_EQ_DONE_TIMEOUT; }
        else if dpcd_lane_status_updated.bits.LT_FAILED_128b_132b { result = DP_128b_132b_LT_FAILED; }
        else { dp_wait_for_training_aux_rd_interval(link, (*lt_settings).eq_pattern_time); wait_time += (*lt_settings).eq_pattern_time; status = dp_get_lane_status_and_lane_adjust(link, lt_settings, dpcd_lane_status.as_mut_ptr(), &mut dpcd_lane_status_updated, dpcd_lane_adjust.as_mut_ptr(), DPRX); }
    }
    result
}

unsafe fn dp_perform_128b_132b_cds_done_sequence(link: *mut dc_link, _link_res: *const link_resource, lt_settings: *mut link_training_settings) -> link_training_result {
    let mut status = DC_OK;
    let mut result = LINK_TRAINING_SUCCESS;
    let mut updated: lane_align_status_updated = core::mem::zeroed();
    let mut lane_statuses: [lane_status; LANE_COUNT_DP_MAX] = core::mem::zeroed();
    let mut lane_adjusts: [lane_adjust; LANE_COUNT_DP_MAX] = core::mem::zeroed();
    let mut wait_time = 0;
    dpcd_set_training_pattern(link, (*lt_settings).pattern_for_cds);
    while result == LINK_TRAINING_SUCCESS {
        dp_wait_for_training_aux_rd_interval(link, (*lt_settings).cds_pattern_time);
        wait_time += (*lt_settings).cds_pattern_time;
        status = dp_get_lane_status_and_lane_adjust(link, lt_settings, lane_statuses.as_mut_ptr(), &mut updated, lane_adjusts.as_mut_ptr(), DPRX);
        if status != DC_OK { result = LINK_TRAINING_ABORT; }
        else if dp_is_symbol_locked((*lt_settings).link_settings.lane_count, lane_statuses.as_mut_ptr()) && updated.bits.CDS_INTERLANE_ALIGN_DONE_128b_132b { break; }
        else if updated.bits.LT_FAILED_128b_132b { result = DP_128b_132b_LT_FAILED; }
        else if wait_time >= (*lt_settings).cds_wait_time_limit { result = DP_128b_132b_CDS_DONE_TIMEOUT; }
    }
    result
}

pub unsafe fn dp_perform_128b_132b_link_training(link: *mut dc_link, link_res: *const link_resource, lt_settings: *mut link_training_settings) -> link_training_result {
    let mut result = LINK_TRAINING_SUCCESS;
    if (*(*link).dc).debug.legacy_dp2_lt {
        let mut legacy_settings: link_training_settings = core::mem::zeroed();
        decide_8b_10b_training_settings(link, link_res, &(*lt_settings).link_settings, &mut legacy_settings);
        return dp_perform_8b_10b_link_training(link, link_res, &mut legacy_settings);
    }
    dpcd_set_link_settings(link, lt_settings);
    if result == LINK_TRAINING_SUCCESS { result = dp_perform_128b_132b_channel_eq_done_sequence(link, link_res, lt_settings); }
    if result == LINK_TRAINING_SUCCESS { result = dp_perform_128b_132b_cds_done_sequence(link, link_res, lt_settings); }
    result
}

pub unsafe fn decide_128b_132b_training_settings(link: *mut dc_link, link_res: *const link_resource, link_settings: *const dc_link_settings, lt_settings: *mut link_training_settings) {
    core::ptr::write_bytes(lt_settings, 0, 1);
    (*lt_settings).link_settings = *link_settings;
    (*lt_settings).link_settings.link_spread = if (*link).dp_ss_off { LINK_SPREAD_DISABLED } else { LINK_SPREAD_05_DOWNSPREAD_30KHZ };
    (*lt_settings).pattern_for_cr = decide_cr_training_pattern(link_settings);
    (*lt_settings).pattern_for_eq = decide_eq_training_pattern(link, link_res, link_settings);
    (*lt_settings).eq_pattern_time = 2500;
    (*lt_settings).eq_wait_time_limit = 400000;
    (*lt_settings).eq_loop_count_limit = 20;
    (*lt_settings).pattern_for_cds = DP_128b_132b_TPS2_CDS;
    (*lt_settings).cds_pattern_time = 2500;
    (*lt_settings).cds_wait_time_limit = (dp_parse_lttpr_repeater_count((*link).dpcd_caps.lttpr_caps.phy_repeater_cnt) + 1) * 20000;
    (*lt_settings).disallow_per_lane_settings = true;
    (*lt_settings).lttpr_mode = dp_decide_128b_132b_lttpr_mode(link);
    dp_hw_to_dpcd_lane_settings(lt_settings, (*lt_settings).hw_lane_settings.as_mut_ptr(), (*lt_settings).dpcd_lane_settings.as_mut_ptr());
}

pub unsafe fn dp_decide_128b_132b_lttpr_mode(link: *mut dc_link) -> lttpr_mode {
    let mut mode = LTTPR_MODE_NON_LTTPR;
    if dp_is_lttpr_present(link) { mode = LTTPR_MODE_NON_TRANSPARENT; }
    DC_LOG_DC!("128b_132b chose LTTPR_MODE {}.\n", mode);
    mode
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
