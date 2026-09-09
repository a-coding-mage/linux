/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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

// Dependency supplied by the surrounding translation unit: "core_types.h".

extern "C" {
    pub fn dcn20_populate_dml_writeback_from_context(
        dc: *mut crate::dc,
        res_ctx: *mut crate::resource_context,
        pipes: *mut crate::display_e2e_pipe_params_st,
    );

    pub fn dcn20_fpu_set_wb_arb_params(
        wb_arb_params: *mut crate::mcif_arb_params,
        context: *mut crate::dc_state,
        pipes: *mut crate::display_e2e_pipe_params_st,
        pipe_cnt: ::core::ffi::c_int,
        i: ::core::ffi::c_int,
    );

    pub fn dcn20_calculate_dlg_params(
        dc: *mut crate::dc,
        context: *mut crate::dc_state,
        pipes: *mut crate::display_e2e_pipe_params_st,
        pipe_cnt: ::core::ffi::c_int,
        vlevel: ::core::ffi::c_int,
    );

    pub fn dcn20_populate_dml_pipes_from_context(
        dc: *mut crate::dc,
        context: *mut crate::dc_state,
        pipes: *mut crate::display_e2e_pipe_params_st,
        validate_mode: crate::dc_validate_mode,
    ) -> ::core::ffi::c_int;

    pub fn dcn20_calculate_wm(
        dc: *mut crate::dc,
        context: *mut crate::dc_state,
        pipes: *mut crate::display_e2e_pipe_params_st,
        out_pipe_cnt: *mut ::core::ffi::c_int,
        pipe_split_from: *mut ::core::ffi::c_int,
        vlevel: ::core::ffi::c_int,
        validate_mode: crate::dc_validate_mode,
    );

    pub fn dcn20_cap_soc_clocks(
        bb: *mut crate::_vcs_dpi_soc_bounding_box_st,
        max_clocks: crate::pp_smu_nv_clock_table,
    );

    pub fn dcn20_update_bounding_box(
        dc: *mut crate::dc,
        bb: *mut crate::_vcs_dpi_soc_bounding_box_st,
        max_clocks: *mut crate::pp_smu_nv_clock_table,
        uclk_states: *mut ::core::ffi::c_uint,
        num_states: ::core::ffi::c_uint,
    );

    pub fn dcn20_patch_bounding_box(
        dc: *mut crate::dc,
        bb: *mut crate::_vcs_dpi_soc_bounding_box_st,
    );

    pub fn dcn20_validate_bandwidth_fp(
        dc: *mut crate::dc,
        context: *mut crate::dc_state,
        validate_mode: crate::dc_validate_mode,
        pipes: *mut crate::display_e2e_pipe_params_st,
    ) -> bool;

    pub fn dcn20_fpu_set_wm_ranges(
        i: ::core::ffi::c_int,
        ranges: *mut crate::pp_smu_wm_range_sets,
        loaded_bb: *mut crate::_vcs_dpi_soc_bounding_box_st,
    );

    pub fn dcn20_fpu_adjust_dppclk(
        v: *mut crate::vba_vars_st,
        vlevel: ::core::ffi::c_int,
        max_mpc_comb: ::core::ffi::c_int,
        pipe_idx: ::core::ffi::c_int,
        is_validating_bw: bool,
    );

    pub fn dcn21_populate_dml_pipes_from_context(
        dc: *mut crate::dc,
        context: *mut crate::dc_state,
        pipes: *mut crate::display_e2e_pipe_params_st,
        validate_mode: crate::dc_validate_mode,
    ) -> ::core::ffi::c_int;

    pub fn dcn21_validate_bandwidth_fp(
        dc: *mut crate::dc,
        context: *mut crate::dc_state,
        validate_mode: crate::dc_validate_mode,
        pipes: *mut crate::display_e2e_pipe_params_st,
    ) -> bool;

    pub fn dcn21_update_bw_bounding_box_fpu(
        dc: *mut crate::dc,
        bw_params: *mut crate::clk_bw_params,
    );

    pub fn dcn21_clk_mgr_set_bw_params_wm_table(
        bw_params: *mut crate::clk_bw_params,
    );

    pub fn dcn201_populate_dml_writeback_from_context_fpu(
        dc: *mut crate::dc,
        res_ctx: *mut crate::resource_context,
        pipes: *mut crate::display_e2e_pipe_params_st,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
