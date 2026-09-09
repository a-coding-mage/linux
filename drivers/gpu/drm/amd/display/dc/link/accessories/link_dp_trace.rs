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

// Dependencies supplied by the surrounding translation unit.

pub unsafe fn dp_trace_init(link: *mut dc_link) {
    core::ptr::write_bytes(core::ptr::addr_of_mut!((*link).dp_trace), 0, 1);
    (*link).dp_trace.is_initialized = true;
}

pub unsafe fn dp_trace_reset(link: *mut dc_link) {
    core::ptr::write_bytes(core::ptr::addr_of_mut!((*link).dp_trace), 0, 1);
}

pub unsafe fn dp_trace_is_initialized(link: *mut dc_link) -> bool {
    (*link).dp_trace.is_initialized
}

pub unsafe fn dp_trace_detect_lt_init(link: *mut dc_link) {
    core::ptr::write_bytes(core::ptr::addr_of_mut!((*link).dp_trace.detect_lt_trace), 0, 1);
}

pub unsafe fn dp_trace_commit_lt_init(link: *mut dc_link) {
    core::ptr::write_bytes(core::ptr::addr_of_mut!((*link).dp_trace.commit_lt_trace), 0, 1);
}

pub unsafe fn dp_trace_link_loss_increment(link: *mut dc_link) {
    (*link).dp_trace.link_loss_count = (*link).dp_trace.link_loss_count.wrapping_add(1);
}

pub unsafe fn dp_trace_lt_fail_count_update(link: *mut dc_link, fail_count: u32, in_detection: bool) {
    if in_detection { (*link).dp_trace.detect_lt_trace.counts.fail = fail_count; }
    else { (*link).dp_trace.commit_lt_trace.counts.fail = fail_count; }
}

pub unsafe fn dp_trace_lt_total_count_increment(link: *mut dc_link, in_detection: bool) {
    if in_detection { (*link).dp_trace.detect_lt_trace.counts.total = (*link).dp_trace.detect_lt_trace.counts.total.wrapping_add(1); }
    else { (*link).dp_trace.commit_lt_trace.counts.total = (*link).dp_trace.commit_lt_trace.counts.total.wrapping_add(1); }
}

pub unsafe fn dp_trace_set_is_logged_flag(link: *mut dc_link, in_detection: bool, is_logged: bool) {
    if in_detection { (*link).dp_trace.detect_lt_trace.is_logged = is_logged; }
    else { (*link).dp_trace.commit_lt_trace.is_logged = is_logged; }
}

pub unsafe fn dp_trace_is_logged(link: *mut dc_link, in_detection: bool) -> bool {
    if in_detection { (*link).dp_trace.detect_lt_trace.is_logged } else { (*link).dp_trace.commit_lt_trace.is_logged }
}

pub unsafe fn dp_trace_lt_result_update(link: *mut dc_link, result: link_training_result, in_detection: bool) {
    if in_detection { (*link).dp_trace.detect_lt_trace.result = result; }
    else { (*link).dp_trace.commit_lt_trace.result = result; }
}

pub unsafe fn dp_trace_set_lt_start_timestamp(link: *mut dc_link, in_detection: bool) {
    if in_detection { (*link).dp_trace.detect_lt_trace.timestamps.start = dm_get_timestamp((*link).dc.ctx); }
    else { (*link).dp_trace.commit_lt_trace.timestamps.start = dm_get_timestamp((*link).dc.ctx); }
}

pub unsafe fn dp_trace_set_lt_end_timestamp(link: *mut dc_link, in_detection: bool) {
    if in_detection { (*link).dp_trace.detect_lt_trace.timestamps.end = dm_get_timestamp((*link).dc.ctx); }
    else { (*link).dp_trace.commit_lt_trace.timestamps.end = dm_get_timestamp((*link).dc.ctx); }
}

pub unsafe fn dp_trace_get_lt_end_timestamp(link: *mut dc_link, in_detection: bool) -> u64 {
    if in_detection { (*link).dp_trace.detect_lt_trace.timestamps.end } else { (*link).dp_trace.commit_lt_trace.timestamps.end }
}

pub unsafe fn dp_trace_get_lt_counts(link: *mut dc_link, in_detection: bool) -> *const dp_trace_lt_counts {
    if in_detection { core::ptr::addr_of!((*link).dp_trace.detect_lt_trace.counts) }
    else { core::ptr::addr_of!((*link).dp_trace.commit_lt_trace.counts) }
}

pub unsafe fn dp_trace_get_link_loss_count(link: *mut dc_link) -> u32 {
    (*link).dp_trace.link_loss_count
}

pub unsafe fn dp_trace_set_edp_power_timestamp(link: *mut dc_link, power_up: bool) {
    if !power_up {
        // save driver power off time stamp
        (*link).dp_trace.edp_trace_power_timestamps.poweroff = dm_get_timestamp((*link).dc.ctx);
    } else {
        (*link).dp_trace.edp_trace_power_timestamps.poweron = dm_get_timestamp((*link).dc.ctx);
    }
}

pub unsafe fn dp_trace_get_edp_poweron_timestamp(link: *mut dc_link) -> u64 {
    (*link).dp_trace.edp_trace_power_timestamps.poweron
}

pub unsafe fn dp_trace_get_edp_poweroff_timestamp(link: *mut dc_link) -> u64 {
    (*link).dp_trace.edp_trace_power_timestamps.poweroff
}

pub unsafe fn dp_trace_source_sequence(link: *mut dc_link, dp_test_mode: u8) {
    if !link.is_null() && (*link).dc.debug.enable_driver_sequence_debug {
        core_link_write_dpcd(link, DP_SOURCE_SEQUENCE, core::ptr::addr_of!(dp_test_mode), core::mem::size_of_val(&dp_test_mode));
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
