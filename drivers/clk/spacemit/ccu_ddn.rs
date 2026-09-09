// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2024 SpacemiT Technology Co. Ltd
 * Copyright (c) 2024-2025 Haylen Chu <heylenay@4d2.org>
 *
 * DDN stands for "Divider Denominator Numerator", it's M/N clock with a
 * constant x2 factor. This clock hardware follows the equation below,
 *
 *            numerator       Fin
 *      2 * ------------- = -------
 *          denominator      Fout
 *
 * Thus, Fout could be calculated with,
 *
 *          Fin    denominator
 * Fout = ----- * -------------
 *           2     numerator
 */

// Linux clock-provider and rational helpers, plus ccu_ddn.h, supply the
// external types, functions, constants, and macros referenced below.

unsafe fn ccu_ddn_calc_rate(
    prate: libc::c_ulong,
    num: libc::c_ulong,
    den: libc::c_ulong,
    pre_div: libc::c_uint,
) -> libc::c_ulong {
    prate.wrapping_mul(den) / pre_div as libc::c_ulong / num
}

unsafe fn ccu_ddn_calc_best_rate(
    ddn: *mut ccu_ddn,
    rate: libc::c_ulong,
    prate: libc::c_ulong,
    num: *mut libc::c_ulong,
    den: *mut libc::c_ulong,
) -> libc::c_ulong {
    rational_best_approximation(
        rate,
        prate / (*ddn).pre_div as libc::c_ulong,
        (*ddn).den_mask >> (*ddn).den_shift,
        (*ddn).num_mask >> (*ddn).num_shift,
        den,
        num,
    );
    ccu_ddn_calc_rate(prate, *num, *den, (*ddn).pre_div)
}

unsafe extern "C" fn ccu_ddn_determine_rate(
    hw: *mut clk_hw,
    req: *mut clk_rate_request,
) -> libc::c_int {
    let ddn = hw_to_ccu_ddn(hw);
    let mut num: libc::c_ulong = 0;
    let mut den: libc::c_ulong = 0;

    (*req).rate = ccu_ddn_calc_best_rate(
        ddn,
        (*req).rate,
        (*req).best_parent_rate,
        &mut num,
        &mut den,
    );

    0
}

unsafe extern "C" fn ccu_ddn_recalc_rate(
    hw: *mut clk_hw,
    prate: libc::c_ulong,
) -> libc::c_ulong {
    let ddn = hw_to_ccu_ddn(hw);
    let val: libc::c_uint = ccu_read(&mut (*ddn).common, ctrl);

    let num = (val & (*ddn).num_mask) >> (*ddn).num_shift;
    let den = (val & (*ddn).den_mask) >> (*ddn).den_shift;

    ccu_ddn_calc_rate(prate, num as libc::c_ulong, den as libc::c_ulong, (*ddn).pre_div)
}

unsafe extern "C" fn ccu_ddn_set_rate(
    hw: *mut clk_hw,
    rate: libc::c_ulong,
    prate: libc::c_ulong,
) -> libc::c_int {
    let ddn = hw_to_ccu_ddn(hw);
    let mut num: libc::c_ulong = 0;
    let mut den: libc::c_ulong = 0;

    ccu_ddn_calc_best_rate(ddn, rate, prate, &mut num, &mut den);

    ccu_update(
        &mut (*ddn).common,
        ctrl,
        (*ddn).num_mask | (*ddn).den_mask,
        (num << (*ddn).num_shift) | (den << (*ddn).den_shift),
    );

    0
}

pub static spacemit_ccu_ddn_ops: clk_ops = clk_ops {
    recalc_rate: Some(ccu_ddn_recalc_rate),
    determine_rate: Some(ccu_ddn_determine_rate),
    set_rate: Some(ccu_ddn_set_rate),
};

// EXPORT_SYMBOL_NS_GPL(spacemit_ccu_ddn_ops, "CLK_SPACEMIT");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
