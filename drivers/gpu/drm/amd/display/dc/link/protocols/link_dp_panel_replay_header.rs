/*
 * Copyright 2025 Advanced Micro Devices, Inc.
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

// Dependency supplied by the corresponding link service module.

pub struct dc_link;
pub struct dc_stream_state;
pub struct dc;
pub struct replay_context;
pub struct dmub_cmd_pr_update_state_data;
pub struct dmub_cmd_pr_general_cmd_data;

extern "C" {
    pub fn dp_setup_replay(link: *mut dc_link, stream: *const dc_stream_state) -> bool;
    pub fn dp_pr_get_panel_inst(
        dc: *const dc,
        link: *const dc_link,
        inst_out: *mut core::ffi::c_uint,
    ) -> bool;
    pub fn dp_pr_enable(link: *mut dc_link, enable: bool) -> bool;
    pub fn dp_pr_copy_settings(link: *mut dc_link, replay_context: *mut replay_context) -> bool;
    pub fn dp_pr_update_state(
        link: *mut dc_link,
        update_state_data: *mut dmub_cmd_pr_update_state_data,
    ) -> bool;
    pub fn dp_pr_set_general_cmd(
        link: *mut dc_link,
        general_cmd_data: *mut dmub_cmd_pr_general_cmd_data,
    ) -> bool;
    pub fn dp_pr_get_state(link: *const dc_link, state: *mut u64) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
