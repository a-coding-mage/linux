/* SPDX-License-Identifier: MIT */
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
 */

// Dependencies supplied by dml2_dc_types.h and display_mode_core_structs.h.

#[repr(C)]
pub struct dml2_svp_helper_select_best_svp_candidate_params {
    pub dml_config: *const dml_display_cfg_st,
    pub mode_support_info: *const dml_mode_support_info_st,
    pub blacklist: ::core::ffi::c_uint,
    pub candidate_index: *mut ::core::ffi::c_uint,
}

#[repr(C)]
pub struct dml2_context {
    _private: [u8; 0],
}

extern "C" {
    pub fn dml2_helper_calculate_num_ways_for_subvp(
        ctx: *mut dml2_context,
        context: *mut dc_state,
    ) -> ::core::ffi::c_uint;

    pub fn dml2_svp_add_phantom_pipe_to_dc_state(
        ctx: *mut dml2_context,
        state: *mut dc_state,
        mode_support_info: *mut dml_mode_support_info_st,
    ) -> bool;

    pub fn dml2_svp_remove_all_phantom_pipes(
        ctx: *mut dml2_context,
        state: *mut dc_state,
    ) -> bool;

    pub fn dml2_svp_validate_static_schedulability(
        ctx: *mut dml2_context,
        context: *mut dc_state,
        pstate_change_type: dml_dram_clock_change_support,
    ) -> bool;

    pub fn dml2_svp_drr_schedulable(
        ctx: *mut dml2_context,
        context: *mut dc_state,
        drr_timing: *mut dc_crtc_timing,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
