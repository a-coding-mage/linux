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

// Dependencies supplied by the surrounding DAL implementation.

// set register offset
// C macro: SR(reg_name) => .reg_name = mm##reg_name

// set register offset with instance
// C macro: SRI(reg_name, block, id) => .reg_name = mm##block##id##_##reg_name

static DISP_CLK_REGS: clk_mgr_registers = clk_mgr_registers {
    CLK_COMMON_REG_LIST_DCE_BASE!()
};

static DISP_CLK_SHIFT: clk_mgr_shift = clk_mgr_shift {
    CLK_COMMON_MASK_SH_LIST_DCE_COMMON_BASE!(__SHIFT)
};

static DISP_CLK_MASK: clk_mgr_mask = clk_mgr_mask {
    CLK_COMMON_MASK_SH_LIST_DCE_COMMON_BASE!(_MASK)
};

// TODO: remove use the two broken down functions
pub unsafe fn dce112_set_clock(
    clk_mgr_base: *mut clk_mgr,
    mut requested_clk_khz: i32,
) -> i32 {
    let clk_mgr_dce: *mut clk_mgr_internal = TO_CLK_MGR_INTERNAL!(clk_mgr_base);
    let mut dce_clk_params: bp_set_dce_clock_parameters = core::mem::zeroed();
    let bp: *mut dc_bios = (*(*clk_mgr_base).ctx).dc_bios;
    let dc: *mut dc = (*(*clk_mgr_base).ctx).dc;
    let dmcu: *mut dmcu = (*(*dc).res_pool).dmcu;
    let mut actual_clock: i32 = requested_clk_khz;

    // Prepare to program display clock
    // memset(&dce_clk_params, 0, sizeof(dce_clk_params));

    // Make sure requested clock isn't lower than minimum threshold
    requested_clk_khz = core::cmp::max(
        requested_clk_khz,
        (*clk_mgr_dce).base.dentist_vco_freq_khz / 62,
    );

    dce_clk_params.target_clock_frequency = requested_clk_khz;
    dce_clk_params.pll_id = CLOCK_SOURCE_ID_DFS;
    dce_clk_params.clock_type = DCECLOCK_TYPE_DISPLAY_CLOCK;

    ((*(*bp).funcs).set_dce_clock)(bp, &mut dce_clk_params);
    actual_clock = dce_clk_params.target_clock_frequency;

    // Program DP ref Clock
    // VBIOS will determine DPREFCLK frequency, so we don't set it
    dce_clk_params.target_clock_frequency = 0;
    dce_clk_params.clock_type = DCECLOCK_TYPE_DPREFCLK;

    if !(((*(*clk_mgr_base).ctx).asic_id.chip_family == FAMILY_AI)
        && ASICREV_IS_VEGA20_P!((*(*clk_mgr_base).ctx).asic_id.hw_internal_rev))
    {
        dce_clk_params.flags.USE_GENLOCK_AS_SOURCE_FOR_DPREFCLK =
            dce_clk_params.pll_id == CLOCK_SOURCE_COMBO_DISPLAY_PLL0;
    } else {
        dce_clk_params.flags.USE_GENLOCK_AS_SOURCE_FOR_DPREFCLK = false;
    }

    ((*(*bp).funcs).set_dce_clock)(bp, &mut dce_clk_params);

    if !dmcu.is_null() && ((*(*dmcu).funcs).is_dmcu_initialized)(dmcu) {
        if (*clk_mgr_dce).dfs_bypass_disp_clk != actual_clock {
            ((*(*dmcu).funcs).set_psr_wait_loop)(dmcu, actual_clock / 1000 / 7);
        }
    }

    (*clk_mgr_dce).dfs_bypass_disp_clk = actual_clock;
    actual_clock
}

pub unsafe fn dce112_set_dispclk(
    clk_mgr: *mut clk_mgr_internal,
    mut requested_clk_khz: i32,
) -> i32 {
    let mut dce_clk_params: bp_set_dce_clock_parameters = core::mem::zeroed();
    let bp: *mut dc_bios = (*(*clk_mgr).base.ctx).dc_bios;
    let dc: *mut dc = (*(*clk_mgr).base.ctx).dc;
    let dmcu: *mut dmcu = (*(*dc).res_pool).dmcu;
    let mut actual_clock: i32 = requested_clk_khz;

    // Prepare to program display clock
    // memset(&dce_clk_params, 0, sizeof(dce_clk_params));

    // Make sure requested clock isn't lower than minimum threshold
    if requested_clk_khz > 0 {
        requested_clk_khz = core::cmp::max(
            requested_clk_khz,
            (*clk_mgr).base.dentist_vco_freq_khz / 62,
        );
    }

    dce_clk_params.target_clock_frequency = requested_clk_khz;
    dce_clk_params.pll_id = CLOCK_SOURCE_ID_DFS;
    dce_clk_params.clock_type = DCECLOCK_TYPE_DISPLAY_CLOCK;

    ((*(*bp).funcs).set_dce_clock)(bp, &mut dce_clk_params);
    actual_clock = dce_clk_params.target_clock_frequency;

    if !dmcu.is_null() && ((*(*dmcu).funcs).is_dmcu_initialized)(dmcu) {
        if (*clk_mgr).dfs_bypass_disp_clk != actual_clock {
            ((*(*dmcu).funcs).set_psr_wait_loop)(dmcu, actual_clock / 1000 / 7);
        }
    }

    (*clk_mgr).dfs_bypass_disp_clk = actual_clock;
    actual_clock
}

pub unsafe fn dce112_set_dprefclk(clk_mgr: *mut clk_mgr_internal) -> i32 {
    let mut dce_clk_params: bp_set_dce_clock_parameters = core::mem::zeroed();
    let bp: *mut dc_bios = (*(*clk_mgr).base.ctx).dc_bios;

    // memset(&dce_clk_params, 0, sizeof(dce_clk_params));

    // Program DP ref Clock
    // VBIOS will determine DPREFCLK frequency, so we don't set it
    dce_clk_params.target_clock_frequency = 0;
    dce_clk_params.pll_id = CLOCK_SOURCE_ID_DFS;
    dce_clk_params.clock_type = DCECLOCK_TYPE_DPREFCLK;
    if !(((*(*clk_mgr).base.ctx).asic_id.chip_family == FAMILY_AI)
        && ASICREV_IS_VEGA20_P!((*(*clk_mgr).base.ctx).asic_id.hw_internal_rev))
    {
        dce_clk_params.flags.USE_GENLOCK_AS_SOURCE_FOR_DPREFCLK =
            dce_clk_params.pll_id == CLOCK_SOURCE_COMBO_DISPLAY_PLL0;
    } else {
        dce_clk_params.flags.USE_GENLOCK_AS_SOURCE_FOR_DPREFCLK = false;
    }

    ((*(*bp).funcs).set_dce_clock)(bp, &mut dce_clk_params);

    // Returns the dp_refclk that was set
    dce_clk_params.target_clock_frequency
}

unsafe fn dce112_update_clocks(
    clk_mgr_base: *mut clk_mgr,
    context: *mut dc_state,
    safe_to_lower: bool,
) {
    let clk_mgr_dce: *mut clk_mgr_internal = TO_CLK_MGR_INTERNAL!(clk_mgr_base);
    let mut patched_disp_clk: i32 = (*context).bw_ctx.bw.dce.dispclk_khz;

    // TODO: W/A for dal3 linux, investigate why this works
    if !(*clk_mgr_dce).dfs_bypass_active {
        patched_disp_clk = patched_disp_clk * 115 / 100;
    }

    if should_set_clock!(safe_to_lower, patched_disp_clk, (*clk_mgr_base).clks.dispclk_khz) {
        patched_disp_clk = dce112_set_clock(clk_mgr_base, patched_disp_clk);
        (*clk_mgr_base).clks.dispclk_khz = patched_disp_clk;
    }
    dce11_pplib_apply_display_requirements!((*(*clk_mgr_base).ctx).dc, context);
}

static mut DCE112_FUNCS: clk_mgr_funcs = clk_mgr_funcs {
    get_dp_ref_clk_frequency: dce_get_dp_ref_freq_khz,
    update_clocks: dce112_update_clocks,
};

pub unsafe fn dce112_clk_mgr_construct(
    ctx: *mut dc_context,
    clk_mgr: *mut clk_mgr_internal,
) {
    dce_clk_mgr_construct!(ctx, clk_mgr);

    (*clk_mgr).regs = &DISP_CLK_REGS;
    (*clk_mgr).clk_mgr_shift = &DISP_CLK_SHIFT;
    (*clk_mgr).clk_mgr_mask = &DISP_CLK_MASK;
    (*clk_mgr).base.funcs = &raw mut DCE112_FUNCS;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
