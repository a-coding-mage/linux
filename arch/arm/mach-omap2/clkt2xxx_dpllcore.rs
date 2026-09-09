// SPDX-License-Identifier: GPL-2.0-only
/*
 * DPLL + CORE_CLK composite clock functions
 *
 * Copyright (C) 2005-2008 Texas Instruments, Inc.
 * Copyright (C) 2004-2010 Nokia Corporation
 *
 * Contacts:
 * Richard Woodruff <r-woodruff2@ti.com>
 * Paul Walmsley
 *
 * Based on earlier work by Tuukka Tikkanen, Tony Lindgren,
 * Gordon McNutt and RidgeRun, Inc.
 *
 * XXX The DPLL and CORE clocks should be split into two separate clock
 * types.
 */

// C dependencies supplied by the surrounding OMAP2xxx clock implementation.

/* #define DOWN_VARIABLE_DPLL 1 */ // Experimental

static mut dpll_core_ck: *mut clk_hw_omap = core::ptr::null_mut();

pub unsafe fn omap2xxx_clk_get_core_rate() -> c_ulong {
    let mut core_clk: i64;
    let v: u32;

    WARN_ON(dpll_core_ck.is_null());

    core_clk = omap2_get_dpll_rate(dpll_core_ck);
    v = omap2xxx_cm_get_core_clk_src();

    if v == CORE_CLK_SRC_32K {
        core_clk = 32768;
    } else {
        core_clk = core_clk.wrapping_mul(v as i64);
    }

    core_clk as c_ulong
}

static unsafe fn omap2_dpllcore_round_rate(target_rate: c_ulong) -> i64 {
    let high: u32;
    let low: u32;
    let core_clk_src: u32;

    core_clk_src = omap2xxx_cm_get_core_clk_src();

    if core_clk_src == CORE_CLK_SRC_DPLL {
        high = (*curr_prcm_set).dpll_speed.wrapping_mul(2);
        low = (*curr_prcm_set).dpll_speed;
    } else {
        high = (*curr_prcm_set).dpll_speed;
        low = (*curr_prcm_set).dpll_speed / 2;
    }

    // DOWN_VARIABLE_DPLL is a build-time configuration option in the C source.
    if target_rate > low as c_ulong {
        high as i64
    } else {
        low as i64
    }
}

pub unsafe fn omap2_dpllcore_recalc(
    _hw: *mut clk_hw,
    _parent_rate: c_ulong,
) -> c_ulong {
    omap2xxx_clk_get_core_rate()
}

pub unsafe fn omap2_reprogram_dpllcore(
    hw: *mut clk_hw,
    rate: c_ulong,
    _parent_rate: c_ulong,
) -> i32 {
    let clk: *mut clk_hw_omap = to_clk_hw_omap(hw);
    let cur_rate: c_ulong;
    let mut low: u32;
    let mut mult: u32;
    let mut div: u32;
    let valid_rate: i64;
    let mut done_rate: u32;
    let mut bypass: u32 = 0;
    let mut tmpset: prcm_config = core::mem::zeroed();
    let dd: *const dpll_data;

    cur_rate = omap2xxx_clk_get_core_rate();
    mult = omap2xxx_cm_get_core_clk_src();

    if rate == cur_rate / 2 && mult == 2 {
        omap2xxx_sdrc_reprogram(CORE_CLK_SRC_DPLL, 1);
    } else if rate == cur_rate.wrapping_mul(2) && mult == 1 {
        omap2xxx_sdrc_reprogram(CORE_CLK_SRC_DPLL_X2, 1);
    } else if rate != cur_rate {
        valid_rate = omap2_dpllcore_round_rate(rate);
        if valid_rate != rate as i64 {
            return -EINVAL;
        }

        if mult == 1 {
            low = (*curr_prcm_set).dpll_speed;
        } else {
            low = (*curr_prcm_set).dpll_speed / 2;
        }

        dd = (*clk).dpll_data;
        if dd.is_null() {
            return -EINVAL;
        }

        tmpset.cm_clksel1_pll = omap_clk_ll_ops.clk_readl(&(*dd).mult_div1_reg);
        tmpset.cm_clksel1_pll &= !((*dd).mult_mask | (*dd).div1_mask);
        div = ((*curr_prcm_set).xtal_speed / 1_000_000).wrapping_sub(1);
        tmpset.cm_clksel2_pll = omap2xxx_cm_get_core_pll_config();
        tmpset.cm_clksel2_pll &= !OMAP24XX_CORE_CLK_SRC_MASK;

        if rate > low as c_ulong {
            tmpset.cm_clksel2_pll |= CORE_CLK_SRC_DPLL_X2;
            mult = ((rate / 2) / 1_000_000) as u32;
            done_rate = CORE_CLK_SRC_DPLL_X2;
        } else {
            tmpset.cm_clksel2_pll |= CORE_CLK_SRC_DPLL;
            mult = (rate / 1_000_000) as u32;
            done_rate = CORE_CLK_SRC_DPLL;
        }

        tmpset.cm_clksel1_pll |= div << __ffs((*dd).mult_mask);
        tmpset.cm_clksel1_pll |= mult << __ffs((*dd).div1_mask);
        tmpset.base_sdrc_rfr = SDRC_RFR_CTRL_BYPASS;

        if rate == (*curr_prcm_set).xtal_speed {
            bypass = 1;
        }

        omap2xxx_sdrc_reprogram(CORE_CLK_SRC_DPLL_X2, 1);
        omap2_set_prcm(tmpset.cm_clksel1_pll, tmpset.base_sdrc_rfr, bypass);
        omap2xxx_sdrc_init_params(omap2xxx_sdrc_dll_is_unlocked());
        omap2xxx_sdrc_reprogram(done_rate, 0);
    }

    0
}

pub unsafe fn omap2xxx_clkt_dpllcore_init(hw: *mut clk_hw) {
    WARN(dpll_core_ck, "dpll_core_ck already set - should never happen");
    dpll_core_ck = to_clk_hw_omap(hw);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
