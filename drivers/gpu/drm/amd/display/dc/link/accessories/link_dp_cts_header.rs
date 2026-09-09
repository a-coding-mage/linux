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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 *
 */

// Dependency supplied by the translated link_service declarations.

extern "C" {
    pub fn dp_handle_automated_test(link: *mut dc_link);

    pub fn dp_set_test_pattern(
        link: *mut dc_link,
        test_pattern: dp_test_pattern,
        test_pattern_color_space: dp_test_pattern_color_space,
        p_link_settings: *const link_training_settings,
        p_custom_pattern: *const u8,
        cust_pattern_size: u32,
    ) -> bool;

    pub fn dp_set_preferred_link_settings(
        dc: *mut dc,
        link_setting: *mut dc_link_settings,
        link: *mut dc_link,
    );

    pub fn dp_set_preferred_training_settings(
        dc: *mut dc,
        link_setting: *mut dc_link_settings,
        lt_overrides: *mut dc_link_training_overrides,
        link: *mut dc_link,
        skip_immediate_retrain: bool,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
