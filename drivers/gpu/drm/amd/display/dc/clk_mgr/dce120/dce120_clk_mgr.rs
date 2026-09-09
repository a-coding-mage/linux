/*
 * Copyright 2012-16 Advanced Micro Devices, Inc.
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
// core_types.h, clk_mgr_internal.h, dce112/dce112_clk_mgr.h,
// dce110/dce110_clk_mgr.h, dce120_clk_mgr.h, dce100/dce_clk_mgr.h,
// and dce120/dce120_hwseq.h.

/// dce121_clock_patch_xgmi_ss_info() - Save XGMI spread spectrum info.
unsafe fn dce121_clock_patch_xgmi_ss_info(clk_mgr_dce: *mut clk_mgr_internal) {
    let mut info: spread_spectrum_info = core::mem::zeroed();
    let bp: *mut dc_bios = (*(*clk_mgr_dce).base.ctx).dc_bios;
    let mut result: bp_result;

    (*clk_mgr_dce).xgmi_enabled = false;

    result = ((*(*bp).funcs).get_spread_spectrum_info)(
        bp,
        AS_SIGNAL_TYPE_XGMI,
        0,
        &mut info,
    );
    if result == BP_RESULT_OK && info.spread_spectrum_percentage != 0 {
        (*clk_mgr_dce).xgmi_enabled = true;
        (*clk_mgr_dce).ss_on_dprefclk = true;
        (*clk_mgr_dce).dprefclk_ss_divider = info.spread_percentage_divider;

        if info.type_.CENTER_MODE == 0 {
            /*
             * Currently for DP Reference clock we
             * need only SS percentage for
             * downspread
             */
            (*clk_mgr_dce).dprefclk_ss_percentage = info.spread_spectrum_percentage;
        }
    }
}

unsafe fn dce12_update_clocks(
    clk_mgr_base: *mut clk_mgr,
    context: *mut dc_state,
    safe_to_lower: bool,
) {
    let clk_mgr_dce: *mut clk_mgr_internal = TO_CLK_MGR_INTERNAL(clk_mgr_base);
    let mut clock_voltage_req: dm_pp_clock_for_voltage_req = core::mem::zeroed();
    let max_pix_clk: i32 = dce_get_max_pixel_clock_for_all_paths(context);
    let mut patched_disp_clk: i32 = (*context).bw_ctx.bw.dce.dispclk_khz;

    /* TODO: W/A for dal3 linux, investigate why this works */
    if !(*clk_mgr_dce).dfs_bypass_active {
        patched_disp_clk = patched_disp_clk * 115 / 100;
    }

    if should_set_clock(safe_to_lower, patched_disp_clk, (*clk_mgr_base).clks.dispclk_khz) {
        clock_voltage_req.clk_type = DM_PP_CLOCK_TYPE_DISPLAY_CLK;
        /*
         * When xGMI is enabled, the display clk needs to be adjusted
         * with the WAFL link's SS percentage.
         */
        if (*clk_mgr_dce).xgmi_enabled {
            patched_disp_clk = dce_adjust_dp_ref_freq_for_ss(clk_mgr_dce, patched_disp_clk);
        }
        clock_voltage_req.clocks_in_khz = patched_disp_clk;
        (*clk_mgr_base).clks.dispclk_khz = dce112_set_clock(clk_mgr_base, patched_disp_clk);
        dm_pp_apply_clock_for_voltage_request((*clk_mgr_base).ctx, &mut clock_voltage_req);
    }

    if should_set_clock(safe_to_lower, max_pix_clk, (*clk_mgr_base).clks.phyclk_khz) {
        clock_voltage_req.clk_type = DM_PP_CLOCK_TYPE_DISPLAYPHYCLK;
        clock_voltage_req.clocks_in_khz = max_pix_clk;
        (*clk_mgr_base).clks.phyclk_khz = max_pix_clk;
        dm_pp_apply_clock_for_voltage_request((*clk_mgr_base).ctx, &mut clock_voltage_req);
    }
    dce11_pplib_apply_display_requirements((*(*clk_mgr_base).ctx).dc, context);
}

static mut dce120_funcs: clk_mgr_funcs = clk_mgr_funcs {
    get_dp_ref_clk_frequency: dce12_get_dp_ref_freq_khz,
    update_clocks: dce12_update_clocks,
};

pub unsafe fn dce120_clk_mgr_construct(ctx: *mut dc_context, clk_mgr: *mut clk_mgr_internal) {
    dce_clk_mgr_construct(ctx, clk_mgr);
    (*clk_mgr).base.dprefclk_khz = 600000;
    (*clk_mgr).base.funcs = &raw mut dce120_funcs;
}

pub unsafe fn dce121_clk_mgr_construct(ctx: *mut dc_context, clk_mgr: *mut clk_mgr_internal) {
    dce120_clk_mgr_construct(ctx, clk_mgr);
    (*clk_mgr).base.dprefclk_khz = 625000;

    /*
     * The xGMI enabled info is used to determine if audio and display
     * clocks need to be adjusted with the WAFL link's SS info.
     */
    if dce121_xgmi_enabled((*(*ctx).dc).hwseq) {
        dce121_clock_patch_xgmi_ss_info(clk_mgr);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
