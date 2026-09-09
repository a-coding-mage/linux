// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
/*
 * Copyright (c) 2016 AmLogic, Inc.
 * Author: Michael Turquette <mturquette@baylibre.com>
 */

/*
 * MultiPhase Locked Loops are outputs from a PLL with additional frequency
 * scaling capabilities. MPLL rates are calculated as:
 *
 * f(N2_integer, SDM_IN ) = 2.0G/(N2_integer + SDM_IN/16384)
 */

// External kernel and driver dependencies are supplied by other translation units.

const SDM_DEN: u64 = 16384;
const N2_MIN: u64 = 4;
const N2_MAX: u64 = 511;

#[inline]
unsafe fn meson_clk_mpll_data(clk: *mut clk_regmap) -> *mut meson_clk_mpll_data {
    (*clk).data as *mut meson_clk_mpll_data
}

unsafe fn rate_from_params(parent_rate: c_ulong, sdm: c_uint, n2: c_uint) -> c_long {
    let divisor = SDM_DEN * n2 as u64 + sdm as u64;

    if n2 as u64 < N2_MIN {
        return -EINVAL as c_long;
    }

    div_round_up_ull(parent_rate as u64 * SDM_DEN, divisor) as c_long
}

unsafe fn params_from_rate(
    requested_rate: c_ulong,
    parent_rate: c_ulong,
    sdm: *mut c_uint,
    n2: *mut c_uint,
    flags: u8,
) {
    let mut div = parent_rate as u64;
    let mut frac = div % requested_rate as u64;
    div /= requested_rate as u64;

    frac *= SDM_DEN;

    if flags & CLK_MESON_MPLL_ROUND_CLOSEST as u8 != 0 {
        *sdm = div_round_closest_ull(frac, requested_rate as u64) as c_uint;
    } else {
        *sdm = div_round_up_ull(frac, requested_rate as u64) as c_uint;
    }

    if *sdm == SDM_DEN as c_uint {
        *sdm = 0;
        div += 1;
    }

    if div < N2_MIN {
        *n2 = N2_MIN as c_uint;
        *sdm = 0;
    } else if div > N2_MAX {
        *n2 = N2_MAX as c_uint;
        *sdm = (SDM_DEN - 1) as c_uint;
    } else {
        *n2 = div as c_uint;
    }
}

unsafe fn mpll_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let clk = to_clk_regmap(hw);
    let mpll = meson_clk_mpll_data(clk);
    let sdm = meson_parm_read((*clk).map, &(*mpll).sdm);
    let n2 = meson_parm_read((*clk).map, &(*mpll).n2);

    let rate = rate_from_params(parent_rate, sdm, n2);
    if rate < 0 { 0 } else { rate as c_ulong }
}

unsafe fn mpll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int {
    let clk = to_clk_regmap(hw);
    let mpll = meson_clk_mpll_data(clk);
    let mut sdm: c_uint = 0;
    let mut n2: c_uint = 0;

    params_from_rate((*req).rate, (*req).best_parent_rate, &mut sdm, &mut n2, (*mpll).flags);

    let rate = rate_from_params((*req).best_parent_rate, sdm, n2);
    if rate < 0 {
        return rate as c_int;
    }

    (*req).rate = rate as c_ulong;
    0
}

unsafe fn mpll_set_rate(
    hw: *mut clk_hw,
    rate: c_ulong,
    parent_rate: c_ulong,
) -> c_int {
    let clk = to_clk_regmap(hw);
    let mpll = meson_clk_mpll_data(clk);
    let mut sdm: c_uint = 0;
    let mut n2: c_uint = 0;

    params_from_rate(rate, parent_rate, &mut sdm, &mut n2, (*mpll).flags);

    // Set the fractional part
    meson_parm_write((*clk).map, &(*mpll).sdm, sdm);

    // Set the integer divider part
    meson_parm_write((*clk).map, &(*mpll).n2, n2);

    0
}

unsafe fn mpll_init(hw: *mut clk_hw) -> c_int {
    let clk = to_clk_regmap(hw);
    let mpll = meson_clk_mpll_data(clk);

    let ret = clk_regmap_init(hw);
    if ret != 0 {
        return ret;
    }

    if (*mpll).init_count != 0 {
        regmap_multi_reg_write((*clk).map, (*mpll).init_regs, (*mpll).init_count);
    }

    // Enable the fractional part
    meson_parm_write((*clk).map, &(*mpll).sdm_en, 1);

    // Set spread spectrum if possible
    if meson_parm_applicable(&(*mpll).ssen) {
        let ss = if (*mpll).flags & CLK_MESON_MPLL_SPREAD_SPECTRUM as u8 != 0 { 1 } else { 0 };
        meson_parm_write((*clk).map, &(*mpll).ssen, ss);
    }

    // Set the magic misc bit if required
    if meson_parm_applicable(&(*mpll).misc) {
        meson_parm_write((*clk).map, &(*mpll).misc, 1);
    }

    0
}

#[no_mangle]
pub static meson_clk_mpll_ro_ops: clk_ops = clk_ops {
    init: Some(clk_regmap_init),
    recalc_rate: Some(mpll_recalc_rate),
    determine_rate: Some(mpll_determine_rate),
};

#[no_mangle]
pub static meson_clk_mpll_ops: clk_ops = clk_ops {
    recalc_rate: Some(mpll_recalc_rate),
    determine_rate: Some(mpll_determine_rate),
    set_rate: Some(mpll_set_rate),
    init: Some(mpll_init),
};

// MODULE_DESCRIPTION("Amlogic MPLL driver");
// MODULE_AUTHOR("Michael Turquette <mturquette@baylibre.com>");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("CLK_MESON");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
