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

// Dependencies supplied by the surrounding display driver.

const REG: () = (); // C macro: (clk_mgr->regs->reg)

static mut DISP_CLK_REGS: clk_mgr_registers = clk_mgr_registers { _opaque: 0 };
static mut DISP_CLK_SHIFT: clk_mgr_shift = clk_mgr_shift { _opaque: 0 };
static mut DISP_CLK_MASK: clk_mgr_mask = clk_mgr_mask { _opaque: 0 };

pub unsafe fn dentist_get_divider_from_did(mut did: u32) -> u32 {
    if did < DENTIST_BASE_DID_1 { did = DENTIST_BASE_DID_1; }
    if did > DENTIST_MAX_DID { did = DENTIST_MAX_DID; }

    if did < DENTIST_BASE_DID_2 {
        DENTIST_DIVIDER_RANGE_1_START + DENTIST_DIVIDER_RANGE_1_STEP * (did - DENTIST_BASE_DID_1)
    } else if did < DENTIST_BASE_DID_3 {
        DENTIST_DIVIDER_RANGE_2_START + DENTIST_DIVIDER_RANGE_2_STEP * (did - DENTIST_BASE_DID_2)
    } else if did < DENTIST_BASE_DID_4 {
        DENTIST_DIVIDER_RANGE_3_START + DENTIST_DIVIDER_RANGE_3_STEP * (did - DENTIST_BASE_DID_3)
    } else {
        DENTIST_DIVIDER_RANGE_4_START + DENTIST_DIVIDER_RANGE_4_STEP * (did - DENTIST_BASE_DID_4)
    }
}

/* SW will adjust DP REF Clock average value for all purposes
 * (DP DTO / DP Audio DTO and DP GTC)
 if clock is spread for all cases:
 -if SS enabled on DP Ref clock and HW de-spreading enabled with SW
 calculations for DS_INCR/DS_MODULO (this is planned to be default case)
 -if SS enabled on DP Ref clock and HW de-spreading enabled with HW
 calculations (not planned to be used, but average clock should still
 be valid)
 -if SS enabled on DP Ref clock and HW de-spreading disabled
 (should not be case with CIK) then SW should program all rates
 generated according to average value (case as with previous ASICs)
 */
pub unsafe fn dce_adjust_dp_ref_freq_for_ss(clk_mgr_dce: *mut clk_mgr_internal, mut dp_ref_clk_khz: i32) -> i32 {
    if (*clk_mgr_dce).ss_on_dprefclk && (*clk_mgr_dce).dprefclk_ss_divider != 0 {
        let mut ss_percentage = dc_fixpt_div_int(
            dc_fixpt_from_fraction((*clk_mgr_dce).dprefclk_ss_percentage, (*clk_mgr_dce).dprefclk_ss_divider), 200);
        ss_percentage = dc_fixpt_sub(dc_fixpt_one(), ss_percentage);
        let adj_dp_ref_clk_khz = dc_fixpt_mul_int(ss_percentage, dp_ref_clk_khz);
        dp_ref_clk_khz = dc_fixpt_floor(adj_dp_ref_clk_khz);
    }
    dp_ref_clk_khz
}

unsafe fn dce60_get_dp_ref_freq_khz(clk_mgr_base: *mut clk_mgr) -> i32 {
    let clk_mgr = TO_CLK_MGR_INTERNAL(clk_mgr_base);
    let ctx = (*clk_mgr_base).ctx;
    let dp_ref_clk_khz = if ASIC_REV_IS_TAHITI_P((*ctx).asic_id.hw_internal_rev) {
        (*(*ctx).dc_bios).fw_info.default_display_engine_pll_frequency
    } else { (*clk_mgr_base).clks.dispclk_khz };
    dce_adjust_dp_ref_freq_for_ss(clk_mgr, dp_ref_clk_khz)
}

pub unsafe fn dce_get_dp_ref_freq_khz(clk_mgr_base: *mut clk_mgr) -> i32 {
    let clk_mgr = TO_CLK_MGR_INTERNAL(clk_mgr_base);
    if (*clk_mgr_base).ctx.dce_version <= DCE_VERSION_6_4 { return dce60_get_dp_ref_freq_khz(clk_mgr_base); }
    let mut dprefclk_src_sel = 0u32;
    REG_GET(DPREFCLK_CNTL, DPREFCLK_SRC_SEL, &mut dprefclk_src_sel);
    ASSERT(dprefclk_src_sel == 0);
    let mut dprefclk_wdivider = 0u32;
    REG_GET(DENTIST_DISPCLK_CNTL, DENTIST_DPREFCLK_WDIVIDER, &mut dprefclk_wdivider);
    let target_div = dentist_get_divider_from_did(dprefclk_wdivider);
    let dp_ref_clk_khz = (DENTIST_DIVIDER_RANGE_SCALE_FACTOR * (*clk_mgr).base.dentist_vco_freq_khz) / target_div as i32;
    dce_adjust_dp_ref_freq_for_ss(clk_mgr, dp_ref_clk_khz)
}

pub unsafe fn dce12_get_dp_ref_freq_khz(clk_mgr_base: *mut clk_mgr) -> i32 {
    let clk_mgr_dce = TO_CLK_MGR_INTERNAL(clk_mgr_base);
    dce_adjust_dp_ref_freq_for_ss(clk_mgr_dce, (*clk_mgr_base).dprefclk_khz)
}

/* unit: in_khz before mode set, get pixel clock from context. ASIC register
 * may not be programmed yet
 */
pub unsafe fn dce_get_max_pixel_clock_for_all_paths(context: *mut dc_state) -> u32 {
    let mut max_pix_clk = 0u32;
    for i in 0..MAX_PIPES {
        let pipe_ctx = &mut (*context).res_ctx.pipe_ctx[i as usize];
        if pipe_ctx.stream.is_null() || pipe_ctx.top_pipe { continue; }
        let pix = pipe_ctx.stream_res.pix_clk_params.requested_pix_clk_100hz / 10;
        if pix > max_pix_clk { max_pix_clk = pix; }
        if dc_is_dp_signal((*pipe_ctx.stream).signal) && pipe_ctx.stream_res.pix_clk_params.requested_sym_clk > max_pix_clk {
            max_pix_clk = pipe_ctx.stream_res.pix_clk_params.requested_sym_clk;
        }
    }
    max_pix_clk
}

/* TODO: remove use the two broken down functions */
pub unsafe fn dce_set_clock(clk_mgr_base: *mut clk_mgr, mut requested_clk_khz: i32) -> i32 {
    let clk_mgr_dce = TO_CLK_MGR_INTERNAL(clk_mgr_base);
    let mut pxl_clk_params: bp_pixel_clock_parameters = core::mem::zeroed();
    let bp = (*clk_mgr_base).ctx.dc_bios;
    let mut actual_clock = requested_clk_khz;
    let dmcu = (*(*clk_mgr_dce).base.ctx).dc.res_pool.dmcu;
    if requested_clk_khz > 0 { requested_clk_khz = max(requested_clk_khz, (*clk_mgr_dce).base.dentist_vco_freq_khz / 64); }
    pxl_clk_params.target_pixel_clock_100hz = requested_clk_khz * 10;
    pxl_clk_params.pll_id = CLOCK_SOURCE_ID_DFS;
    if (*clk_mgr_base).ctx.dce_version == DCE_VERSION_6_0 || (*clk_mgr_base).ctx.dce_version == DCE_VERSION_6_4 { pxl_clk_params.pll_id = CLOCK_SOURCE_ID_PLL0; }
    if (*clk_mgr_dce).dfs_bypass_active { pxl_clk_params.flags.SET_DISPCLK_DFS_BYPASS = true; }
    (*(*bp).funcs).program_display_engine_pll(bp, &mut pxl_clk_params);
    if (*clk_mgr_dce).dfs_bypass_active { (*clk_mgr_dce).dfs_bypass_disp_clk = pxl_clk_params.dfs_bypass_display_clock; actual_clock = pxl_clk_params.dfs_bypass_display_clock; }
    if !dmcu.is_null() && (*(*dmcu).funcs).is_dmcu_initialized(dmcu) { (*(*dmcu).funcs).set_psr_wait_loop(dmcu, actual_clock / 1000 / 7); }
    actual_clock
}

unsafe fn dce_clock_read_integrated_info(clk_mgr_dce: *mut clk_mgr_internal) {
    let debug = &(*(*(*clk_mgr_dce).base.ctx).dc).debug;
    let bp = (*(*clk_mgr_dce).base.ctx).dc_bios;
    if !(*bp).integrated_info.is_null() { (*clk_mgr_dce).base.dentist_vco_freq_khz = (*(*bp).integrated_info).dentist_vco_freq; }
    if (*clk_mgr_dce).base.dentist_vco_freq_khz == 0 { (*clk_mgr_dce).base.dentist_vco_freq_khz = (*bp).fw_info.smu_gpu_pll_output_freq; if (*clk_mgr_dce).base.dentist_vco_freq_khz == 0 { (*clk_mgr_dce).base.dentist_vco_freq_khz = 3600000; } }
    if !debug.disable_dfs_bypass && !(*bp).integrated_info.is_null() && ((*(*bp).integrated_info).gpu_cap_info & DFS_BYPASS_ENABLE) != 0 { (*clk_mgr_dce).dfs_bypass_enabled = true; }
}

pub unsafe fn dce_clock_read_ss_info(clk_mgr_dce: *mut clk_mgr_internal) {
    let bp = (*(*clk_mgr_dce).base.ctx).dc_bios;
    let ss_info_num = (*(*bp).funcs).get_ss_entry_number(bp, AS_SIGNAL_TYPE_GPU_PLL);
    if ss_info_num != 0 {
        let mut info: spread_spectrum_info = core::mem::zeroed();
        let mut result = (*(*bp).funcs).get_spread_spectrum_info(bp, AS_SIGNAL_TYPE_GPU_PLL, 0, &mut info);
        if result == BP_RESULT_OK && info.spread_spectrum_percentage != 0 {
            (*clk_mgr_dce).ss_on_dprefclk = true; (*clk_mgr_dce).dprefclk_ss_divider = info.spread_percentage_divider;
            if info.type_.CENTER_MODE == 0 { (*clk_mgr_dce).dprefclk_ss_percentage = info.spread_spectrum_percentage; }
            return;
        }
        result = (*(*bp).funcs).get_spread_spectrum_info(bp, AS_SIGNAL_TYPE_DISPLAY_PORT, 0, &mut info);
        if result == BP_RESULT_OK && info.spread_spectrum_percentage != 0 {
            (*clk_mgr_dce).ss_on_dprefclk = true; (*clk_mgr_dce).dprefclk_ss_divider = info.spread_percentage_divider;
            if info.type_.CENTER_MODE == 0 { (*clk_mgr_dce).dprefclk_ss_percentage = info.spread_spectrum_percentage; }
            if (*(*(*clk_mgr_dce).base.ctx).dc).config.ignore_dpref_ss { (*clk_mgr_dce).dprefclk_ss_percentage = 0; }
        }
    }
}

unsafe fn dce_pplib_apply_display_requirements(dc: *mut dc, context: *mut dc_state) {
    let pp_display_cfg = &mut (*context).pp_display_cfg;
    dce110_fill_display_configs(context, pp_display_cfg);
    if core::ptr::read(&(*(*dc).current_state).pp_display_cfg) != core::ptr::read(pp_display_cfg) { dm_pp_apply_display_requirements((*dc).ctx, pp_display_cfg); }
}

unsafe fn dce_update_clocks(clk_mgr_base: *mut clk_mgr, context: *mut dc_state, safe_to_lower: bool) {
    let max_disp_clk = (*clk_mgr_base).clks.max_supported_dispclk_khz;
    let mut patched_disp_clk = MIN(max_disp_clk, (*context).bw_ctx.bw.dce.dispclk_khz);
    if should_set_clock(safe_to_lower, patched_disp_clk, (*clk_mgr_base).clks.dispclk_khz) { patched_disp_clk = dce_set_clock(clk_mgr_base, patched_disp_clk); (*clk_mgr_base).clks.dispclk_khz = patched_disp_clk; }
    dce_pplib_apply_display_requirements((*clk_mgr_base).ctx.dc, context);
}

static mut DCE_FUNCS: clk_mgr_funcs = clk_mgr_funcs { get_dp_ref_clk_frequency: dce_get_dp_ref_freq_khz, update_clocks: dce_update_clocks };

pub unsafe fn dce_clk_mgr_construct(ctx: *mut dc_context, clk_mgr: *mut clk_mgr_internal) {
    let base = &mut (*clk_mgr).base;
    base.ctx = ctx; base.funcs = &mut DCE_FUNCS;
    if (*ctx).dce_version >= DCE_VERSION_8_0 { (*clk_mgr).regs = &mut DISP_CLK_REGS; (*clk_mgr).clk_mgr_shift = &mut DISP_CLK_SHIFT; (*clk_mgr).clk_mgr_mask = &mut DISP_CLK_MASK; }
    (*clk_mgr).dfs_bypass_disp_clk = 0; (*clk_mgr).dprefclk_ss_percentage = 0; (*clk_mgr).dprefclk_ss_divider = 1000; (*clk_mgr).ss_on_dprefclk = false;
    base.clks.max_supported_dispclk_khz = if (*ctx).dce_version >= DCE_VERSION_12_0 { 1133000 } else if (*ctx).dce_version >= DCE_VERSION_11_2 { 1132000 } else if (*ctx).dce_version >= DCE_VERSION_11_0 { 643000 } else if (*ctx).dce_version >= DCE_VERSION_8_0 { 625000 } else { 600000 };
    dce_clock_read_integrated_info(clk_mgr); dce_clock_read_ss_info(clk_mgr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
