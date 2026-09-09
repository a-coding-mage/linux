/* SPDX-License-Identifier: MIT */
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

// Dependency supplied by the surrounding translation unit: clk_mgr_internal.h

extern "C" {
    pub fn dcn32_build_wm_range_table_fpu(clk_mgr: *mut clk_mgr_internal);

    pub fn dcn32_helper_populate_phantom_dlg_params(
        dc: *mut dc,
        context: *mut dc_state,
        pipes: *mut display_e2e_pipe_params_st,
        pipe_cnt: core::ffi::c_int,
    );

    pub fn dcn32_set_phantom_stream_timing(
        dc: *mut dc,
        context: *mut dc_state,
        ref_pipe: *mut pipe_ctx,
        phantom_stream: *mut dc_stream_state,
        pipes: *mut display_e2e_pipe_params_st,
        pipe_cnt: u32,
        dc_pipe_idx: u32,
    );

    pub fn dcn32_internal_validate_bw(
        dc: *mut dc,
        context: *mut dc_state,
        pipes: *mut display_e2e_pipe_params_st,
        pipe_cnt_out: *mut core::ffi::c_int,
        vlevel_out: *mut core::ffi::c_int,
        validate_mode: dc_validate_mode,
    ) -> bool;

    pub fn dcn32_calculate_wm_and_dlg_fpu(
        dc: *mut dc,
        context: *mut dc_state,
        pipes: *mut display_e2e_pipe_params_st,
        pipe_cnt: core::ffi::c_int,
        vlevel: core::ffi::c_int,
    );

    pub fn dcn32_update_bw_bounding_box_fpu(
        dc: *mut dc,
        bw_params: *mut clk_bw_params,
    );

    pub fn dcn32_find_dummy_latency_index_for_fw_based_mclk_switch(
        dc: *mut dc,
        context: *mut dc_state,
        pipes: *mut display_e2e_pipe_params_st,
        pipe_cnt: core::ffi::c_int,
        vlevel: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub fn dcn32_patch_dpm_table(bw_params: *mut clk_bw_params);

    pub fn dcn32_zero_pipe_dcc_fraction(
        pipes: *mut display_e2e_pipe_params_st,
        pipe_cnt: core::ffi::c_int,
    );

    pub fn dcn32_assign_fpo_vactive_candidate(
        dc: *mut dc,
        context: *const dc_state,
        fpo_candidate_stream: *mut *mut dc_stream_state,
    );

    pub fn dcn32_find_vactive_pipe(
        dc: *mut dc,
        context: *const dc_state,
        fpo_candidate_stream: *mut dc_stream_state,
        vactive_margin_req: u32,
    ) -> bool;

    pub fn dcn32_override_min_req_memclk(dc: *mut dc, context: *mut dc_state);

    pub fn dcn32_set_clock_limits(soc_bb: *const _vcs_dpi_soc_bounding_box_st);

    pub fn dcn32_get_max_dispclk_mhz(
        dc: *mut dc,
        context: *mut dc_state,
    ) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
