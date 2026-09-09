/*
 * Copyright 2020-2021 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding translation unit:
// #include "core_types.h"
// #include "dcn20/dcn20_optc.h"

extern "C" {
    pub fn dcn30_fpu_populate_dml_writeback_from_context(
        dc: *mut dc,
        res_ctx: *mut resource_context,
        pipes: *mut display_e2e_pipe_params_st,
    );

    pub fn dcn30_fpu_set_mcif_arb_params(
        wb_arb_params: *mut mcif_arb_params,
        dml: *mut display_mode_lib,
        pipes: *mut display_e2e_pipe_params_st,
        pipe_cnt: i32,
        cur_pipe: i32,
    );

    pub fn dcn30_fpu_update_soc_for_wm_a(dc: *mut dc, context: *mut dc_state);

    pub fn dcn30_fpu_calculate_wm_and_dlg(
        dc: *mut dc,
        context: *mut dc_state,
        pipes: *mut display_e2e_pipe_params_st,
        pipe_cnt: i32,
        vlevel: i32,
    );

    pub fn dcn30_fpu_update_dram_channel_width_bytes(dc: *mut dc);

    pub fn dcn30_fpu_update_max_clk(dcn30_bb_max_clk: *mut dc_bounding_box_max_clk);

    pub fn dcn30_fpu_get_optimal_dcfclk_fclk_for_uclk(
        uclk_mts: u32,
        optimal_dcfclk: *mut u32,
        optimal_fclk: *mut u32,
    );

    pub fn dcn30_fpu_update_bw_bounding_box(
        dc: *mut dc,
        bw_params: *mut clk_bw_params,
        dcn30_bb_max_clk: *mut dc_bounding_box_max_clk,
        dcfclk_mhz: *mut u32,
        dram_speed_mts: *mut u32,
    );

    pub fn dcn30_find_dummy_latency_index_for_fw_based_mclk_switch(
        dc: *mut dc,
        context: *mut dc_state,
        pipes: *mut display_e2e_pipe_params_st,
        pipe_cnt: i32,
        vlevel: i32,
    ) -> i32;

    pub fn dcn3_fpu_build_wm_range_table(base: *mut clk_mgr);

    pub fn patch_dcn30_soc_bounding_box(
        dc: *mut dc,
        dcn3_0_ip: *mut _vcs_dpi_soc_bounding_box_st,
    );

    pub fn hpo_fpu_enc3_validate_hdmi_frl_output_link(
        enc: *mut hpo_frl_stream_encoder,
        frl_link_settings: *mut dc_hdmi_frl_link_settings,
        frl_params: *mut frl_cap_chk_params,
        timing: *const dc_crtc_timing,
        dsc_max_rate: u32,
    );

    pub fn hpo_fpu_enc3_validate_hdmi_frl_output_timing(
        timing: *const dc_crtc_timing,
        audio: *const audio_check,
        frl_params: *mut frl_cap_chk_params,
    );

    pub fn frl_fpu_cap_chk_common(
        enc: *mut hpo_frl_stream_encoder,
        inter: *mut frl_cap_chk_intermediates,
        params: *mut frl_cap_chk_params,
    ) -> frl_cap_chk_result;

    pub fn frl_fpu_cap_chk_uncompressed(
        enc: *mut hpo_frl_stream_encoder,
        params: *mut frl_cap_chk_params,
        inter: *mut frl_cap_chk_intermediates,
    ) -> frl_cap_chk_result;

    pub fn frl_fpu_cap_chk_compressed(
        enc: *mut hpo_frl_stream_encoder,
        params: *mut frl_cap_chk_params,
        inter: *mut frl_cap_chk_intermediates,
    ) -> frl_cap_chk_result;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
