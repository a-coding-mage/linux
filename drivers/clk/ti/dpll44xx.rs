// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP4-specific DPLL control functions
 *
 * Copyright (C) 2011 Texas Instruments, Inc.
 * Rajendra Nayak
 */

// External kernel types, operations, constants, and functions are supplied by
// the surrounding clock implementation.

const OMAP4_DPLL_LP_FINT_MAX: u64 = 1_000_000;
const OMAP4_DPLL_LP_FOUT_MAX: u64 = 100_000_000;

const OMAP4430_DPLL_CLKOUT_GATE_CTRL_MASK: u32 = 1u32 << 8;
const OMAP4430_DPLL_CLKOUTX2_GATE_CTRL_MASK: u32 = 1u32 << 10;
const OMAP4430_DPLL_REGM4XEN_MASK: u32 = 1u32 << 11;
const OMAP4430_REGM4XEN_MULT: u32 = 4;

unsafe fn omap4_dpllmx_allow_gatectrl(clk: *mut clk_hw_omap) {
    let mut v: u32;
    let mask: u32;

    if clk.is_null() {
        return;
    }

    mask = if (*clk).flags & CLOCK_CLKOUTX2 != 0 {
        OMAP4430_DPLL_CLKOUTX2_GATE_CTRL_MASK
    } else {
        OMAP4430_DPLL_CLKOUT_GATE_CTRL_MASK
    };

    v = (*ti_clk_ll_ops).clk_readl(&(*clk).clksel_reg);
    // Clear the bit to allow gatectrl
    v &= !mask;
    (*ti_clk_ll_ops).clk_writel(v, &mut (*clk).clksel_reg);
}

unsafe fn omap4_dpllmx_deny_gatectrl(clk: *mut clk_hw_omap) {
    let mut v: u32;
    let mask: u32;

    if clk.is_null() {
        return;
    }

    mask = if (*clk).flags & CLOCK_CLKOUTX2 != 0 {
        OMAP4430_DPLL_CLKOUTX2_GATE_CTRL_MASK
    } else {
        OMAP4430_DPLL_CLKOUT_GATE_CTRL_MASK
    };

    v = (*ti_clk_ll_ops).clk_readl(&(*clk).clksel_reg);
    // Set the bit to deny gatectrl
    v |= mask;
    (*ti_clk_ll_ops).clk_writel(v, &mut (*clk).clksel_reg);
}

pub static clkhwops_omap4_dpllmx: clk_hw_omap_ops = clk_hw_omap_ops {
    .allow_idle: Some(omap4_dpllmx_allow_gatectrl),
    .deny_idle: Some(omap4_dpllmx_deny_gatectrl),
};

/// omap4_dpll_lpmode_recalc - compute DPLL low-power setting
/// @dd: pointer to the dpll data structure
///
/// Calculates if low-power mode can be enabled based upon the last
/// multiplier and divider values calculated. If low-power mode can be
/// enabled, then the bit to enable low-power mode is stored in the
/// last_rounded_lpmode variable. This implementation is based upon the
/// criteria for enabling low-power mode as described in the OMAP4430/60
/// Public TRM section 3.6.3.3.2 "Enable Control, Status, and Low-Power
/// Operation Mode".
unsafe fn omap4_dpll_lpmode_recalc(dd: *mut dpll_data) {
    let fint = clk_hw_get_rate((*dd).clk_ref) / ((*dd).last_rounded_n + 1);
    let fout = fint * (*dd).last_rounded_m;

    if fint < OMAP4_DPLL_LP_FINT_MAX && fout < OMAP4_DPLL_LP_FOUT_MAX {
        (*dd).last_rounded_lpmode = 1;
    } else {
        (*dd).last_rounded_lpmode = 0;
    }
}

/// omap4_dpll_regm4xen_recalc - compute DPLL rate, considering REGM4XEN bit
/// @hw: pointer to the clock to compute the rate for
/// @parent_rate: clock rate of the DPLL parent
pub unsafe fn omap4_dpll_regm4xen_recalc(
    hw: *mut clk_hw,
    _parent_rate: c_ulong,
) -> c_ulong {
    let clk = to_clk_hw_omap(hw);
    let mut rate: c_ulong;
    let dd: *mut dpll_data;

    if clk.is_null() || (*clk).dpll_data.is_null() {
        return 0;
    }

    dd = (*clk).dpll_data;
    rate = omap2_get_dpll_rate(clk);

    // regm4xen adds a multiplier of 4 to DPLL calculations
    let v = (*ti_clk_ll_ops).clk_readl(&(*dd).control_reg);
    if v & OMAP4430_DPLL_REGM4XEN_MASK != 0 {
        rate *= OMAP4430_REGM4XEN_MULT as c_ulong;
    }

    rate
}

/// omap4_dpll_regm4xen_determine_rate - determine rate for a DPLL
/// @hw: pointer to the clock to determine rate for
/// @req: target rate request
pub unsafe fn omap4_dpll_regm4xen_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> c_int {
    let clk = to_clk_hw_omap(hw);
    let dd: *mut dpll_data;

    if (*req).rate == 0 {
        return -EINVAL;
    }

    dd = (*clk).dpll_data;
    if dd.is_null() {
        return -EINVAL;
    }

    if clk_hw_get_rate((*dd).clk_bypass) == (*req).rate
        && (*dd).modes & (1 << DPLL_LOW_POWER_BYPASS) != 0
    {
        (*req).best_parent_hw = (*dd).clk_bypass;
    } else {
        let mut tmp_req: clk_rate_request;
        clk_hw_init_rate_request(hw, &mut tmp_req, (*req).rate);
        (*dd).last_rounded_m4xen = 0;

        let mut r = omap2_dpll_determine_rate(hw, &mut tmp_req);
        if r < 0 {
            tmp_req.rate /= OMAP4430_REGM4XEN_MULT as c_ulong;
            r = omap2_dpll_determine_rate(hw, &mut tmp_req);
            if r < 0 {
                return r;
            }

            (*dd).last_rounded_rate *= OMAP4430_REGM4XEN_MULT as c_ulong;
            (*dd).last_rounded_m4xen = 1;
        }

        omap4_dpll_lpmode_recalc(dd);
        (*req).rate = (*dd).last_rounded_rate;
        (*req).best_parent_hw = (*dd).clk_ref;
    }

    (*req).best_parent_rate = (*req).rate;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
