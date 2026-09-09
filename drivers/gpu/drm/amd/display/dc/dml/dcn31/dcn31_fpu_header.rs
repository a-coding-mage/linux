/*
 * Copyright 2019-2021 Advanced Micro Devices, Inc.
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

// External types supplied by the surrounding translation unit/dependencies.

pub const DCN3_1_DEFAULT_DET_SIZE: i32 = 384;
pub const DCN3_15_DEFAULT_DET_SIZE: i32 = 192;
pub const DCN3_15_MIN_COMPBUF_SIZE_KB: i32 = 128;
pub const DCN3_16_DEFAULT_DET_SIZE: i32 = 192;
pub const DCN3_16_MIN_COMPBUF_SIZE_KB: i32 = 128;

extern "C" {
    pub fn dcn31_zero_pipe_dcc_fraction(
        pipes: *mut display_e2e_pipe_params_st,
        pipe_cnt: i32,
    );

    pub fn dcn31_update_soc_for_wm_a(dc: *mut dc, context: *mut dc_state);
    pub fn dcn315_update_soc_for_wm_a(dc: *mut dc, context: *mut dc_state);

    pub fn dcn31_calculate_wm_and_dlg_fp(
        dc: *mut dc,
        context: *mut dc_state,
        pipes: *mut display_e2e_pipe_params_st,
        pipe_cnt: i32,
        vlevel: i32,
    );

    pub fn dcn31_update_bw_bounding_box_fpu(dc: *mut dc, bw_params: *mut clk_bw_params);
    pub fn dcn315_update_bw_bounding_box_fpu(dc: *mut dc, bw_params: *mut clk_bw_params);
    pub fn dcn316_update_bw_bounding_box_fpu(dc: *mut dc, bw_params: *mut clk_bw_params);
    pub fn dcn_get_max_non_odm_pix_rate_100hz(soc: *mut _vcs_dpi_soc_bounding_box_st) -> i32;
    pub fn dcn_get_approx_det_segs_required_for_pstate(
        soc: *mut _vcs_dpi_soc_bounding_box_st,
        pix_clk_100hz: i32,
        bpp: i32,
        seg_size_kb: i32,
    ) -> i32;

    pub fn dcn31x_populate_dml_pipes_from_context(
        dc: *mut dc,
        context: *mut dc_state,
        pipes: *mut display_e2e_pipe_params_st,
        validate_mode: dc_validate_mode,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
