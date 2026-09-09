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

// C headers and build-time register-list macros are supplied by dependencies.

static mut CLK_MGR_REGS: clk_mgr_registers = clk_mgr_registers {
    CLK_COMMON_REG_LIST_DCN_201!()
};

static mut CLK_MGR_SHIFT: clk_mgr_shift = clk_mgr_shift {
    CLK_COMMON_MASK_SH_LIST_DCN201_BASE!(__SHIFT)
};

static mut CLK_MGR_MASK: clk_mgr_mask = clk_mgr_mask {
    CLK_COMMON_MASK_SH_LIST_DCN201_BASE!(_MASK)
};

unsafe fn dcn201_init_clocks(clk_mgr: *mut clk_mgr) {
    core::ptr::write_bytes(
        &mut (*clk_mgr).clks as *mut dc_clocks as *mut u8,
        0,
        core::mem::size_of::<dc_clocks>(),
    );
    (*clk_mgr).clks.p_state_change_support = true;
    (*clk_mgr).clks.prev_p_state_change_support = true;
    (*clk_mgr).clks.max_supported_dppclk_khz = 1200000;
    (*clk_mgr).clks.max_supported_dispclk_khz = 1200000;
}

unsafe fn dcn201_update_clocks(
    clk_mgr_base: *mut clk_mgr,
    context: *mut dc_state,
    safe_to_lower: bool,
) {
    let clk_mgr: *mut clk_mgr_internal = TO_CLK_MGR_INTERNAL!(clk_mgr_base);
    let new_clocks: *mut dc_clocks = &mut (*context).bw_ctx.bw.dcn.clk;
    let dc: *mut dc = (*clk_mgr_base).ctx.as_ref().unwrap().dc;
    let mut update_dppclk = false;
    let mut update_dispclk = false;
    let mut dpp_clock_lowered = false;
    let mut force_reset = false;
    let mut p_state_change_support: bool;
    let total_plane_count: i32;

    if (*dc).work_arounds.skip_clock_update {
        return;
    }

    if (*clk_mgr_base).clks.dispclk_khz == 0 || ((*dc).debug.force_clock_mode & 0x1) != 0 {
        /* this is from resume or boot up, if forced_clock cfg option
         * used, we bypass program dispclk and DPPCLK, but need set them
         * for S3.
         */
        force_reset = true;
        /* force_clock_mode 0x1:  force reset the clock even it is the
         * same clock as long as it is in Passive level.
         */
        dcn2_read_clocks_from_hw_dentist(clk_mgr_base);
    }

    if should_set_clock(safe_to_lower, (*new_clocks).phyclk_khz, (*clk_mgr_base).clks.phyclk_khz) {
        (*clk_mgr_base).clks.phyclk_khz = (*new_clocks).phyclk_khz;
    }

    if (*dc).debug.force_min_dcfclk_mhz > 0 {
        (*new_clocks).dcfclk_khz = if (*new_clocks).dcfclk_khz > (*dc).debug.force_min_dcfclk_mhz * 1000 {
            (*new_clocks).dcfclk_khz
        } else {
            (*dc).debug.force_min_dcfclk_mhz * 1000
        };
    }

    if should_set_clock(safe_to_lower, (*new_clocks).dcfclk_khz, (*clk_mgr_base).clks.dcfclk_khz) {
        (*clk_mgr_base).clks.dcfclk_khz = (*new_clocks).dcfclk_khz;
    }
    if should_set_clock(safe_to_lower, (*new_clocks).dcfclk_deep_sleep_khz, (*clk_mgr_base).clks.dcfclk_deep_sleep_khz) {
        (*clk_mgr_base).clks.dcfclk_deep_sleep_khz = (*new_clocks).dcfclk_deep_sleep_khz;
    }
    if should_set_clock(safe_to_lower, (*new_clocks).socclk_khz, (*clk_mgr_base).clks.socclk_khz) {
        (*clk_mgr_base).clks.socclk_khz = (*new_clocks).socclk_khz;
    }

    total_plane_count = clk_mgr_helper_get_active_plane_cnt(dc, context);
    p_state_change_support = (*new_clocks).p_state_change_support || total_plane_count == 0;
    if should_update_pstate_support(safe_to_lower, p_state_change_support, (*clk_mgr_base).clks.p_state_change_support) {
        (*clk_mgr_base).clks.prev_p_state_change_support = (*clk_mgr_base).clks.p_state_change_support;
        (*clk_mgr_base).clks.p_state_change_support = p_state_change_support;
    }

    if should_set_clock(safe_to_lower, (*new_clocks).dramclk_khz, (*clk_mgr_base).clks.dramclk_khz) {
        (*clk_mgr_base).clks.dramclk_khz = (*new_clocks).dramclk_khz;
    }
    if should_set_clock(safe_to_lower, (*new_clocks).dppclk_khz, (*clk_mgr).base.clks.dppclk_khz) {
        if (*clk_mgr).base.clks.dppclk_khz > (*new_clocks).dppclk_khz {
            dpp_clock_lowered = true;
        }
        (*clk_mgr).base.clks.dppclk_khz = (*new_clocks).dppclk_khz;
        update_dppclk = true;
    }
    if should_set_clock(safe_to_lower, (*new_clocks).dispclk_khz, (*clk_mgr_base).clks.dispclk_khz) {
        (*clk_mgr_base).clks.dispclk_khz = (*new_clocks).dispclk_khz;
        update_dispclk = true;
    }

    if !(*dc).config.forced_clocks || (force_reset && safe_to_lower) {
        if dpp_clock_lowered {
            // if clock is being lowered, increase DTO before lowering refclk
            dcn20_update_clocks_update_dpp_dto(clk_mgr, context, safe_to_lower);
            dcn20_update_clocks_update_dentist(clk_mgr, context);
        } else {
            // if clock is being raised, increase refclk before lowering DTO
            if update_dppclk || update_dispclk {
                dcn20_update_clocks_update_dentist(clk_mgr, context);
            }
            // always update dtos unless clock is lowered and not safe to lower
            dcn20_update_clocks_update_dpp_dto(clk_mgr, context, safe_to_lower);
        }
    }
}

static mut DCN201_FUNCS: clk_mgr_funcs = clk_mgr_funcs {
    get_dp_ref_clk_frequency: Some(dce12_get_dp_ref_freq_khz),
    update_clocks: Some(dcn201_update_clocks),
    init_clocks: Some(dcn201_init_clocks),
    get_clock: Some(dcn2_get_clock),
};

unsafe fn dcn201_clk_mgr_construct(
    ctx: *mut dc_context,
    clk_mgr: *mut clk_mgr_internal,
    pp_smu: *mut pp_smu_funcs,
    dccg: *mut dccg,
) {
    let debug: *mut dc_debug_options = &mut (*(*ctx).dc).debug;
    let bp: *mut dc_bios = (*ctx).dc_bios;
    (*clk_mgr).base.ctx = ctx;
    (*clk_mgr).base.funcs = &mut DCN201_FUNCS;
    (*clk_mgr).regs = &CLK_MGR_REGS;
    (*clk_mgr).clk_mgr_shift = &CLK_MGR_SHIFT;
    (*clk_mgr).clk_mgr_mask = &CLK_MGR_MASK;
    (*clk_mgr).dccg = dccg;
    (*clk_mgr).dfs_bypass_disp_clk = 0;
    (*clk_mgr).dprefclk_ss_percentage = 0;
    (*clk_mgr).dprefclk_ss_divider = 1000;
    (*clk_mgr).ss_on_dprefclk = false;
    (*clk_mgr).base.dprefclk_khz = REG_READ!(CLK4_CLK2_CURRENT_CNT);
    (*clk_mgr).base.dprefclk_khz *= 100;
    if (*clk_mgr).base.dprefclk_khz == 0 {
        (*clk_mgr).base.dprefclk_khz = 600000;
    }
    REG_GET!(CLK4_CLK_PLL_REQ, FbMult_int, &mut (*clk_mgr).base.dentist_vco_freq_khz);
    (*clk_mgr).base.dentist_vco_freq_khz *= 100000;
    if (*clk_mgr).base.dentist_vco_freq_khz == 0 {
        (*clk_mgr).base.dentist_vco_freq_khz = 3000000;
    }
    if !(*debug).disable_dfs_bypass && !(*bp).integrated_info.is_null()
        && ((*(*bp).integrated_info).gpu_cap_info & DFS_BYPASS_ENABLE) != 0
    {
        (*clk_mgr).dfs_bypass_enabled = true;
    }
    dce_clock_read_ss_info(clk_mgr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
