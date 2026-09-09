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

// Dependency declarations supplied by the surrounding translation unit:
// link_service.h

unsafe extern "C" {
    pub fn dp_trace_init(link: *mut crate::dc_link);
    pub fn dp_trace_reset(link: *mut crate::dc_link);
    pub fn dp_trace_is_initialized(link: *mut crate::dc_link) -> bool;
    pub fn dp_trace_detect_lt_init(link: *mut crate::dc_link);
    pub fn dp_trace_commit_lt_init(link: *mut crate::dc_link);
    pub fn dp_trace_link_loss_increment(link: *mut crate::dc_link);
    pub fn dp_trace_lt_fail_count_update(
        link: *mut crate::dc_link,
        fail_count: u32,
        in_detection: bool,
    );
    pub fn dp_trace_lt_total_count_increment(link: *mut crate::dc_link, in_detection: bool);
    pub fn dp_trace_set_is_logged_flag(
        link: *mut crate::dc_link,
        in_detection: bool,
        is_logged: bool,
    );
    pub fn dp_trace_is_logged(link: *mut crate::dc_link, in_detection: bool) -> bool;
    pub fn dp_trace_lt_result_update(
        link: *mut crate::dc_link,
        result: crate::link_training_result,
        in_detection: bool,
    );
    pub fn dp_trace_set_lt_start_timestamp(link: *mut crate::dc_link, in_detection: bool);
    pub fn dp_trace_set_lt_end_timestamp(link: *mut crate::dc_link, in_detection: bool);
    pub fn dp_trace_get_lt_end_timestamp(link: *mut crate::dc_link, in_detection: bool) -> u64;
    pub fn dp_trace_get_lt_counts(
        link: *mut crate::dc_link,
        in_detection: bool,
    ) -> *const crate::dp_trace_lt_counts;
    pub fn dp_trace_get_link_loss_count(link: *mut crate::dc_link) -> u32;
    pub fn dp_trace_set_edp_power_timestamp(link: *mut crate::dc_link, power_up: bool);
    pub fn dp_trace_get_edp_poweron_timestamp(link: *mut crate::dc_link) -> u64;
    pub fn dp_trace_get_edp_poweroff_timestamp(link: *mut crate::dc_link) -> u64;
    pub fn dp_trace_source_sequence(link: *mut crate::dc_link, dp_test_mode: u8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
