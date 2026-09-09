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

// Dependency supplied by the translated link_dp_training header.

/* to avoid infinite loop where-in the receiver
 * switches between different VS
 */
pub const LINK_TRAINING_MAX_CR_RETRY: u32 = 100;
pub const LINK_TRAINING_MAX_RETRY_COUNT: u32 = 5;

extern "C" {
    pub fn dp_perform_8b_10b_link_training(
        link: *mut crate::dc_link,
        link_res: *const crate::link_resource,
        lt_settings: *mut crate::link_training_settings,
    ) -> crate::link_training_result;

    pub fn perform_8b_10b_clock_recovery_sequence(
        link: *mut crate::dc_link,
        link_res: *const crate::link_resource,
        lt_settings: *mut crate::link_training_settings,
        offset: u32,
    ) -> crate::link_training_result;

    pub fn perform_8b_10b_channel_equalization_sequence(
        link: *mut crate::dc_link,
        link_res: *const crate::link_resource,
        lt_settings: *mut crate::link_training_settings,
        offset: u32,
    ) -> crate::link_training_result;

    pub fn dp_decide_8b_10b_lttpr_mode(
        link: *mut crate::dc_link,
    ) -> crate::lttpr_mode;

    pub fn decide_8b_10b_training_settings(
        link: *mut crate::dc_link,
        link_res: *const crate::link_resource,
        link_setting: *const crate::dc_link_settings,
        lt_settings: *mut crate::link_training_settings,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
