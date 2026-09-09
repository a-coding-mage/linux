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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependency supplied by the surrounding translation unit: link_service.h

extern "C" {
    pub fn perform_link_training_with_retries(
        link_setting: *const dc_link_settings,
        skip_video_pattern: bool,
        attempts: i32,
        pipe_ctx: *mut pipe_ctx,
        signal: signal_type,
        do_fallback: bool,
    ) -> bool;

    pub fn dp_perform_link_training(
        link: *mut dc_link,
        link_res: *const link_resource,
        link_settings: *const dc_link_settings,
        skip_video_pattern: bool,
    ) -> link_training_result;

    pub fn dp_set_hw_training_pattern(
        link: *mut dc_link,
        link_res: *const link_resource,
        pattern: dc_dp_training_pattern,
        offset: u32,
    ) -> bool;

    pub fn dp_set_hw_test_pattern(
        link: *mut dc_link,
        link_res: *const link_resource,
        test_pattern: dp_test_pattern,
        custom_pattern: *mut u8,
        custom_pattern_size: u32,
    );

    pub fn dpcd_set_training_pattern(
        link: *mut dc_link,
        training_pattern: dc_dp_training_pattern,
    ) -> dc_status;

    // Write DPCD drive settings.
    pub fn dpcd_set_lane_settings(
        link: *mut dc_link,
        link_training_setting: *const link_training_settings,
        offset: u32,
    ) -> dc_status;

    // Write DPCD link configuration data.
    pub fn dpcd_set_link_settings(
        link: *mut dc_link,
        lt_settings: *const link_training_settings,
    ) -> dc_status;

    pub fn dpcd_set_lt_pattern_and_lane_settings(
        link: *mut dc_link,
        lt_settings: *const link_training_settings,
        pattern: dc_dp_training_pattern,
        offset: u32,
    );

    // Read training status and adjustment requests from DPCD.
    pub fn dp_get_lane_status_and_lane_adjust(
        link: *mut dc_link,
        link_training_setting: *const link_training_settings,
        ln_status: *mut lane_status,
        ln_align: *mut lane_align_status_updated,
        ln_adjust: *mut lane_adjust,
        offset: u32,
    ) -> dc_status;

    pub fn dpcd_configure_lttpr_mode(
        link: *mut dc_link,
        lt_settings: *mut link_training_settings,
    ) -> dc_status;

    pub fn configure_lttpr_mode_transparent(link: *mut dc_link) -> dc_status;

    pub fn dpcd_configure_channel_coding(
        link: *mut dc_link,
        lt_settings: *mut link_training_settings,
    ) -> dc_status;

    pub fn repeater_training_done(link: *mut dc_link, offset: u32);

    pub fn start_clock_recovery_pattern_early(
        link: *mut dc_link,
        link_res: *const link_resource,
        lt_settings: *mut link_training_settings,
        offset: u32,
    );

    pub fn dp_decide_training_settings(
        link: *mut dc_link,
        link_res: *const link_resource,
        link_settings: *const dc_link_settings,
        lt_settings: *mut link_training_settings,
    );

    pub fn dp_decide_lane_settings(
        lt_settings: *const link_training_settings,
        ln_adjust: *const lane_adjust,
        hw_lane_settings: *mut dc_lane_settings,
        dpcd_lane_settings: *mut dpcd_training_lane,
    );

    pub fn decide_cr_training_pattern(
        link_settings: *const dc_link_settings,
    ) -> dc_dp_training_pattern;

    pub fn decide_eq_training_pattern(
        link: *mut dc_link,
        link_res: *const link_resource,
        link_settings: *const dc_link_settings,
    ) -> dc_dp_training_pattern;

    pub fn dp_decide_lttpr_mode(
        link: *mut dc_link,
        link_setting: *mut dc_link_settings,
    ) -> lttpr_mode;

    pub fn dp_get_lttpr_mode_override(link: *mut dc_link, override_: *mut lttpr_mode);

    pub fn override_training_settings(
        link: *mut dc_link,
        overrides: *const dc_link_training_overrides,
        lt_settings: *mut link_training_settings,
    );

    // Check DPCD training status registers to detect link loss.
    pub fn dp_check_link_loss_status(
        link: *mut dc_link,
        link_training_setting: *const link_training_settings,
    ) -> link_training_result;

    pub fn dp_is_cr_done(ln_count: dc_lane_count, dpcd_lane_status: *mut lane_status) -> bool;

    pub fn dp_is_ch_eq_done(ln_count: dc_lane_count, dpcd_lane_status: *mut lane_status) -> bool;
    pub fn dp_is_symbol_locked(ln_count: dc_lane_count, dpcd_lane_status: *mut lane_status) -> bool;
    pub fn dp_is_interlane_aligned(align_status: lane_align_status_updated) -> bool;

    pub fn is_repeater(lt_settings: *const link_training_settings, offset: u32) -> bool;

    pub fn dp_is_max_vs_reached(lt_settings: *const link_training_settings) -> bool;

    pub fn get_dpcd_link_rate(link_settings: *const dc_link_settings) -> u8;

    pub fn dp_get_cr_failure(
        ln_count: dc_lane_count,
        dpcd_lane_status: *mut lane_status,
    ) -> link_training_result;

    pub fn dp_hw_to_dpcd_lane_settings(
        lt_settings: *const link_training_settings,
        hw_lane_settings: *const dc_lane_settings,
        dpcd_lane_settings: *mut dpcd_training_lane,
    );

    pub fn dp_wait_for_training_aux_rd_interval(link: *mut dc_link, wait_in_micro_secs: u32);

    pub fn dp_training_pattern_to_dpcd_training_pattern(
        link: *mut dc_link,
        pattern: dc_dp_training_pattern,
    ) -> dpcd_training_patterns;

    pub fn dp_initialize_scrambling_data_symbols(
        link: *mut dc_link,
        pattern: dc_dp_training_pattern,
    ) -> u8;

    pub fn dp_log_training_result(
        link: *mut dc_link,
        lt_settings: *const link_training_settings,
        status: link_training_result,
    );

    pub fn dp_translate_training_aux_read_interval(dpcd_aux_read_interval: u32) -> u32;

    pub fn dp_get_nibble_at_index(buf: *const u8, index: u32) -> u8;

    pub fn dp_check_interlane_aligned(
        align_status: lane_align_status_updated,
        link: *mut dc_link,
        retries: u8,
    ) -> bool;

    pub fn dp_get_eq_aux_rd_interval(
        link: *const dc_link,
        lt_settings: *const link_training_settings,
        offset: u32,
        retries: u8,
    ) -> u32;

    pub fn dp_check_dpcd_reqeust_status(link: *const dc_link, status: dc_status) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
