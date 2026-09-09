/*
 * Copyright 2018 Advanced Micro Devices, Inc.
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

// External declarations supplied by the surrounding DC implementation.

unsafe fn rv1_init_clocks(clk_mgr: *mut clk_mgr) {
    core::ptr::write_bytes(&mut (*clk_mgr).clks as *mut dc_clocks as *mut u8, 0, core::mem::size_of::<dc_clocks>());
}

unsafe fn rv1_determine_dppclk_threshold(clk_mgr: *mut clk_mgr_internal, new_clocks: *mut dc_clocks) -> i32 {
    let request_dpp_div = (*new_clocks).dispclk_khz > (*new_clocks).dppclk_khz;
    let dispclk_increase = (*new_clocks).dispclk_khz > (*clk_mgr).base.clks.dispclk_khz;
    let disp_clk_threshold = (*new_clocks).max_supported_dppclk_khz;
    let cur_dpp_div = (*clk_mgr).base.clks.dispclk_khz > (*clk_mgr).base.clks.dppclk_khz;

    if dispclk_increase {
        if cur_dpp_div { return (*new_clocks).dispclk_khz; }
        if (*new_clocks).dispclk_khz <= disp_clk_threshold { return (*new_clocks).dispclk_khz; }
        if !request_dpp_div { return (*new_clocks).dispclk_khz; }
    } else {
        if !cur_dpp_div { return (*new_clocks).dispclk_khz; }
        if (*clk_mgr).base.clks.dispclk_khz <= disp_clk_threshold { return (*new_clocks).dispclk_khz; }
        if request_dpp_div { return (*new_clocks).dispclk_khz; }
    }
    disp_clk_threshold
}

unsafe fn ramp_up_dispclk_with_dpp(clk_mgr: *mut clk_mgr_internal, dc: *mut dc, new_clocks: *mut dc_clocks, safe_to_lower: bool) {
    let mut request_dpp_div = (*new_clocks).dispclk_khz > (*new_clocks).dppclk_khz;
    let dispclk_to_dpp_threshold = rv1_determine_dppclk_threshold(clk_mgr, new_clocks);

    if !safe_to_lower { request_dpp_div = false; }
    ((*clk_mgr).funcs.set_dispclk)(clk_mgr, dispclk_to_dpp_threshold);
    ((*clk_mgr).funcs.set_dprefclk)(clk_mgr);

    for i in 0..(*dc).res_pool.pipe_count {
        let pipe_ctx = &mut (*(*dc).current_state).res_ctx.pipe_ctx[i as usize];
        if pipe_ctx.plane_state.is_null() { continue; }
        ((*pipe_ctx.plane_res.dpp).funcs.dpp_dppclk_control)(pipe_ctx.plane_res.dpp, request_dpp_div, true);
    }

    if dispclk_to_dpp_threshold != (*new_clocks).dispclk_khz {
        ((*clk_mgr).funcs.set_dispclk)(clk_mgr, (*new_clocks).dispclk_khz);
        ((*clk_mgr).funcs.set_dprefclk)(clk_mgr);
    }
    (*clk_mgr).base.clks.dispclk_khz = (*new_clocks).dispclk_khz;
    (*clk_mgr).base.clks.dppclk_khz = (*new_clocks).dppclk_khz;
    (*clk_mgr).base.clks.max_supported_dppclk_khz = (*new_clocks).max_supported_dppclk_khz;
}

unsafe fn rv1_update_clocks(clk_mgr_base: *mut clk_mgr, context: *mut dc_state, safe_to_lower: bool) {
    let clk_mgr = TO_CLK_MGR_INTERNAL(clk_mgr_base);
    let dc = (*(*clk_mgr_base).ctx).dc;
    let debug = &mut (*dc).debug;
    let new_clocks = &mut (*context).bw_ctx.bw.dcn.clk;
    let pp_smu = &mut (*(*clk_mgr).pp_smu).rv_funcs;
    let mut send_request_to_increase = false;
    let mut send_request_to_lower = false;
    let display_count = clk_mgr_helper_get_active_display_cnt(dc, context);
    let enter_display_off = display_count == 0;

    ASSERT(!(*clk_mgr).pp_smu.is_null());
    if (*dc).work_arounds.skip_clock_update { return; }

    if enter_display_off == safe_to_lower {
        if let Some(set_display_count) = pp_smu.set_display_count {
            set_display_count(&mut pp_smu.pp_smu, display_count);
        }
    }
    if new_clocks.dispclk_khz > (*clk_mgr_base).clks.dispclk_khz || new_clocks.phyclk_khz > (*clk_mgr_base).clks.phyclk_khz || new_clocks.fclk_khz > (*clk_mgr_base).clks.fclk_khz || new_clocks.dcfclk_khz > (*clk_mgr_base).clks.dcfclk_khz { send_request_to_increase = true; }
    if should_set_clock(safe_to_lower, new_clocks.phyclk_khz, (*clk_mgr_base).clks.phyclk_khz) { (*clk_mgr_base).clks.phyclk_khz = new_clocks.phyclk_khz; send_request_to_lower = true; }
    if debug.force_fclk_khz != 0 { new_clocks.fclk_khz = debug.force_fclk_khz; }
    if should_set_clock(safe_to_lower, new_clocks.fclk_khz, (*clk_mgr_base).clks.fclk_khz) { (*clk_mgr_base).clks.fclk_khz = new_clocks.fclk_khz; send_request_to_lower = true; }
    if should_set_clock(safe_to_lower, new_clocks.dcfclk_khz, (*clk_mgr_base).clks.dcfclk_khz) { (*clk_mgr_base).clks.dcfclk_khz = new_clocks.dcfclk_khz; send_request_to_lower = true; }
    if should_set_clock(safe_to_lower, new_clocks.dcfclk_deep_sleep_khz, (*clk_mgr_base).clks.dcfclk_deep_sleep_khz) { (*clk_mgr_base).clks.dcfclk_deep_sleep_khz = new_clocks.dcfclk_deep_sleep_khz; send_request_to_lower = true; }

    if send_request_to_increase || (!send_request_to_increase && send_request_to_lower) {
        if let (Some(set_f), Some(set_d), Some(set_ds)) = (pp_smu.set_hard_min_fclk_by_freq, pp_smu.set_hard_min_dcfclk_by_freq, pp_smu.set_min_deep_sleep_dcfclk) {
            set_f(&mut pp_smu.pp_smu, khz_to_mhz_ceil(new_clocks.fclk_khz));
            set_d(&mut pp_smu.pp_smu, khz_to_mhz_ceil(new_clocks.dcfclk_khz));
            set_ds(&mut pp_smu.pp_smu, khz_to_mhz_ceil(new_clocks.dcfclk_deep_sleep_khz));
        }
    }
    if should_set_clock(safe_to_lower, new_clocks.dispclk_khz, (*clk_mgr_base).clks.dispclk_khz) || new_clocks.dispclk_khz == (*clk_mgr_base).clks.dispclk_khz {
        ramp_up_dispclk_with_dpp(clk_mgr, dc, new_clocks, safe_to_lower);
        (*clk_mgr_base).clks.dispclk_khz = new_clocks.dispclk_khz;
        send_request_to_lower = true;
    }
}

unsafe fn rv1_enable_pme_wa(clk_mgr_base: *mut clk_mgr) {
    let clk_mgr = TO_CLK_MGR_INTERNAL(clk_mgr_base);
    if !(*clk_mgr).pp_smu.is_null() {
        let pp_smu = &mut (*(*clk_mgr).pp_smu).rv_funcs;
        if let Some(set_pme_wa_enable) = pp_smu.set_pme_wa_enable { set_pme_wa_enable(&mut pp_smu.pp_smu); }
    }
}

static mut rv1_clk_funcs: clk_mgr_funcs = clk_mgr_funcs {
    init_clocks: Some(rv1_init_clocks), get_dp_ref_clk_frequency: Some(dce12_get_dp_ref_freq_khz), update_clocks: Some(rv1_update_clocks), enable_pme_wa: Some(rv1_enable_pme_wa),
};

static mut rv1_clk_internal_funcs: clk_mgr_internal_funcs = clk_mgr_internal_funcs {
    set_dispclk: Some(rv1_vbios_smu_set_dispclk), set_dprefclk: Some(dce112_set_dprefclk),
};

pub unsafe fn rv1_clk_mgr_construct(ctx: *mut dc_context, clk_mgr: *mut clk_mgr_internal, pp_smu: *mut pp_smu_funcs) {
    let debug = &mut (*(*ctx).dc).debug;
    let bp = (*ctx).dc_bios;
    (*clk_mgr).base.ctx = ctx;
    (*clk_mgr).pp_smu = pp_smu;
    (*clk_mgr).base.funcs = &mut rv1_clk_funcs;
    (*clk_mgr).funcs = &mut rv1_clk_internal_funcs;
    (*clk_mgr).dfs_bypass_disp_clk = 0;
    (*clk_mgr).dprefclk_ss_percentage = 0;
    (*clk_mgr).dprefclk_ss_divider = 1000;
    (*clk_mgr).ss_on_dprefclk = false;
    (*clk_mgr).base.dprefclk_khz = 600000;
    if !(*bp).integrated_info.is_null() { (*clk_mgr).base.dentist_vco_freq_khz = (*bp).integrated_info.dentist_vco_freq; }
    if (*bp).fw_info_valid && (*clk_mgr).base.dentist_vco_freq_khz == 0 {
        (*clk_mgr).base.dentist_vco_freq_khz = (*bp).fw_info.smu_gpu_pll_output_freq;
        if (*clk_mgr).base.dentist_vco_freq_khz == 0 { (*clk_mgr).base.dentist_vco_freq_khz = 3600000; }
    }
    if !debug.disable_dfs_bypass && !(*bp).integrated_info.is_null() && ((*bp).integrated_info.gpu_cap_info & DFS_BYPASS_ENABLE) != 0 { (*clk_mgr).dfs_bypass_enabled = true; }
    dce_clock_read_ss_info(clk_mgr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
