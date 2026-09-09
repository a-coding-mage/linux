// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP2/3/4 DPLL clock functions
 *
 * Copyright (C) 2005-2008 Texas Instruments, Inc.
 * Copyright (C) 2004-2010 Nokia Corporation
 *
 * Contacts:
 * Richard Woodruff <r-woodruff2@ti.com>
 * Paul Walmsley
 */

// Dependencies supplied by the surrounding kernel translation unit.

/* DPLL rate rounding: minimum DPLL multiplier, divider values */
const DPLL_MIN_MULTIPLIER: i32 = 2;
const DPLL_MIN_DIVIDER: i32 = 1;

/* Possible error results from _dpll_test_mult */
const DPLL_MULT_UNDERFLOW: i32 = -1;

/* Scale factor to mitigate roundoff errors in DPLL rate rounding. */
const DPLL_SCALE_FACTOR: i32 = 64;
const DPLL_SCALE_BASE: i32 = 2;
const DPLL_ROUNDING_VAL: i32 = (DPLL_SCALE_BASE / 2) * (DPLL_SCALE_FACTOR / DPLL_SCALE_BASE);

/* DPLL valid Fint frequency range for OMAP36xx and OMAP4xxx. */
const OMAP3PLUS_DPLL_FINT_JTYPE_MIN: i64 = 500000;
const OMAP3PLUS_DPLL_FINT_JTYPE_MAX: i64 = 2500000;

/* _dpll_test_fint() return codes */
const DPLL_FINT_UNDERFLOW: i32 = -1;
const DPLL_FINT_INVALID: i32 = -2;

/*
 * _dpll_test_fint - test whether an Fint value is valid for the DPLL
 */
unsafe fn _dpll_test_fint(clk: *mut clk_hw_omap, n: u32) -> i32 {
    let dd = (*clk).dpll_data;
    let mut fint_min: i64;
    let mut fint_max: i64;
    let mut ret: i32 = 0;

    let fint = (clk_hw_get_rate(clk_hw_get_parent(&(*clk).hw)) / n as u64) as i64;

    if (*dd).flags & DPLL_J_TYPE != 0 {
        fint_min = OMAP3PLUS_DPLL_FINT_JTYPE_MIN;
        fint_max = OMAP3PLUS_DPLL_FINT_JTYPE_MAX;
    } else {
        fint_min = ti_clk_get_features().fint_min as i64;
        fint_max = ti_clk_get_features().fint_max as i64;
    }

    if fint_min == 0 || fint_max == 0 {
        WARN(1, "No fint limits available!\n");
        return DPLL_FINT_INVALID;
    }

    if fint < ti_clk_get_features().fint_min as i64 {
        pr_debug("rejecting n=%d due to Fint failure, lowering max_divider\n", n);
        (*dd).max_divider = n;
        ret = DPLL_FINT_UNDERFLOW;
    } else if fint > ti_clk_get_features().fint_max as i64 {
        pr_debug("rejecting n=%d due to Fint failure, boosting min_divider\n", n);
        (*dd).min_divider = n;
        ret = DPLL_FINT_INVALID;
    } else if fint > ti_clk_get_features().fint_band1_max as i64
        && fint < ti_clk_get_features().fint_band2_min as i64
    {
        pr_debug("rejecting n=%d due to Fint failure\n", n);
        ret = DPLL_FINT_INVALID;
    }
    ret
}

unsafe fn _dpll_compute_new_rate(parent_rate: u64, m: u32, n: u32) -> u64 {
    (parent_rate * m as u64) / n as u64
}

unsafe fn _dpll_test_mult(
    m: *mut i32,
    n: i32,
    new_rate: *mut u64,
    target_rate: u64,
    parent_rate: u64,
) -> i32 {
    let mut r: i32 = 0;
    let mut carry: i32 = 0;

    if (*m % DPLL_SCALE_FACTOR) >= DPLL_ROUNDING_VAL {
        carry = 1;
    }
    *m = (*m / DPLL_SCALE_FACTOR) + carry;

    *new_rate = _dpll_compute_new_rate(parent_rate, *m as u32, n as u32);
    if *new_rate > target_rate {
        *m -= 1;
        *new_rate = 0;
    }

    if *m < DPLL_MIN_MULTIPLIER {
        *m = DPLL_MIN_MULTIPLIER;
        *new_rate = 0;
        r = DPLL_MULT_UNDERFLOW;
    }

    if *new_rate == 0 {
        *new_rate = _dpll_compute_new_rate(parent_rate, *m as u32, n as u32);
    }
    r
}

unsafe fn _omap2_dpll_is_in_bypass(v: u32) -> i32 {
    let mut mask = ti_clk_get_features().dpll_bypass_vals as u8;
    while mask != 0 {
        let val = __ffs(mask as u32) as u8;
        mask ^= 1u8 << val;
        if v == val as u32 {
            return 1;
        }
    }
    0
}

/* Public functions */
pub unsafe fn omap2_init_dpll_parent(hw: *mut clk_hw) -> u8 {
    let clk = to_clk_hw_omap(hw);
    let dd = (*clk).dpll_data;
    if dd.is_null() {
        return (-EINVAL) as u8;
    }

    let mut v = ti_clk_ll_ops.clk_readl(&(*dd).control_reg);
    v &= (*dd).enable_mask;
    v >>= __ffs((*dd).enable_mask);

    if _omap2_dpll_is_in_bypass(v) != 0 { 1 } else { 0 }
}

pub unsafe fn omap2_get_dpll_rate(clk: *mut clk_hw_omap) -> u64 {
    let dd = (*clk).dpll_data;
    if dd.is_null() {
        return 0;
    }

    let mut v = ti_clk_ll_ops.clk_readl(&(*dd).control_reg);
    v &= (*dd).enable_mask;
    v >>= __ffs((*dd).enable_mask);
    if _omap2_dpll_is_in_bypass(v) != 0 {
        return clk_hw_get_rate((*dd).clk_bypass);
    }

    v = ti_clk_ll_ops.clk_readl(&(*dd).mult_div1_reg);
    let dpll_mult = (v & (*dd).mult_mask) >> __ffs((*dd).mult_mask);
    let dpll_div = (v & (*dd).div1_mask) >> __ffs((*dd).div1_mask);
    (clk_hw_get_rate((*dd).clk_ref) * dpll_mult as u64) / (dpll_div as u64 + 1)
}

pub unsafe fn omap2_dpll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let clk = to_clk_hw_omap(hw);
    if clk.is_null() || (*clk).dpll_data.is_null() {
        return -EINVAL;
    }
    let dd = (*clk).dpll_data;
    if (*dd).max_rate != 0 && (*req).rate > (*dd).max_rate {
        (*req).rate = (*dd).max_rate;
    }

    let ref_rate = clk_hw_get_rate((*dd).clk_ref);
    let clk_name = clk_hw_get_name(hw);
    pr_debug("clock: %s: starting DPLL round_rate, target rate %lu\n", clk_name, (*req).rate);

    let scaled_rt_rp = (*req).rate / (ref_rate / DPLL_SCALE_FACTOR as u64);
    let scaled_max_m = (*dd).max_multiplier * DPLL_SCALE_FACTOR as u32;
    (*dd).last_rounded_rate = 0;

    let mut min_delta_m: i32 = i32::MAX;
    let mut min_delta_n: i32 = i32::MAX;
    let mut prev_min_delta: i64 = i64::MAX;
    let mut new_rate: u64 = 0;

    for n in (*dd).min_divider..=(*dd).max_divider {
        let r = _dpll_test_fint(clk, n);
        if r == DPLL_FINT_UNDERFLOW { break; }
        if r == DPLL_FINT_INVALID { continue; }

        let mut m = (scaled_rt_rp * n as u64) as i32;
        if m as u32 > scaled_max_m { break; }
        if _dpll_test_mult(&mut m, n as i32, &mut new_rate, (*req).rate, ref_rate) == DPLL_MULT_UNDERFLOW { continue; }

        let delta = (*req).rate as i64 - new_rate as i64;
        if delta < 0 { continue; }
        if delta < prev_min_delta {
            prev_min_delta = delta;
            min_delta_m = m;
            min_delta_n = n as i32;
        }
        pr_debug("clock: %s: m = %d: n = %d: new_rate = %lu\n", clk_name, m, n, new_rate);
        if delta == 0 { break; }
    }

    if prev_min_delta == i64::MAX {
        pr_debug("clock: %s: cannot round to rate %lu\n", clk_name, (*req).rate);
        return -EINVAL;
    }
    (*dd).last_rounded_m = min_delta_m;
    (*dd).last_rounded_n = min_delta_n;
    (*dd).last_rounded_rate = (*req).rate - prev_min_delta as u64;
    (*req).rate = (*dd).last_rounded_rate;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
