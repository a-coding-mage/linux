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
 *
 */

/* FILE POLICY AND INTENDED USAGE:
 *
 */

/* Dependency declarations are supplied by the corresponding translated headers. */

pub unsafe fn dp_perform_link_training_skip_aux(
    link: *mut dc_link,
    link_res: *const link_resource,
    link_setting: *const dc_link_settings,
) -> bool {
    let mut lt_settings: link_training_settings = core::mem::zeroed();

    dp_decide_training_settings(
        link,
        link_res,
        link_setting,
        &mut lt_settings,
    );
    override_training_settings(
        link,
        &(*link).preferred_training_settings,
        &mut lt_settings,
    );

    /* 1. Perform_clock_recovery_sequence. */

    /* transmit training pattern for clock recovery */
    dp_set_hw_training_pattern(link, link_res, lt_settings.pattern_for_cr, DPRX);

    /* call HWSS to set lane settings*/
    dp_set_hw_lane_settings(link, link_res, &lt_settings, DPRX);

    /* wait receiver to lock-on*/
    dp_wait_for_training_aux_rd_interval(link, lt_settings.cr_pattern_time);

    /* 2. Perform_channel_equalization_sequence. */

    /* transmit training pattern for channel equalization. */
    dp_set_hw_training_pattern(link, link_res, lt_settings.pattern_for_eq, DPRX);

    /* call HWSS to set lane settings*/
    dp_set_hw_lane_settings(link, link_res, &lt_settings, DPRX);

    /* wait receiver to lock-on. */
    dp_wait_for_training_aux_rd_interval(link, lt_settings.eq_pattern_time);

    /* 3. Perform_link_training_int. */

    /* Mainlink output idle pattern. */
    dp_set_hw_test_pattern(
        link,
        link_res,
        DP_TEST_PATTERN_VIDEO_MODE,
        core::ptr::null_mut(),
        0,
    );

    dp_log_training_result(link, &lt_settings, LINK_TRAINING_SUCCESS);

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
