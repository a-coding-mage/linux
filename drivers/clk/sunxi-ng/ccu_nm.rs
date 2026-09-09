// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 Maxime Ripard
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 */

// Dependencies supplied by the surrounding clock implementation are intentionally
// referenced here rather than reimplemented.

#[repr(C)]
struct _ccu_nm {
    n: ::core::ffi::c_ulong,
    min_n: ::core::ffi::c_ulong,
    max_n: ::core::ffi::c_ulong,
    m: ::core::ffi::c_ulong,
    min_m: ::core::ffi::c_ulong,
    max_m: ::core::ffi::c_ulong,
}

unsafe fn ccu_nm_calc_rate(
    parent: ::core::ffi::c_ulong,
    n: ::core::ffi::c_ulong,
    m: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    let mut rate = parent as u64;
    rate = rate.wrapping_mul(n as u64);
    rate /= m as u64;
    rate as ::core::ffi::c_ulong
}

unsafe fn ccu_nm_find_best(
    common: *mut ccu_common,
    parent: ::core::ffi::c_ulong,
    rate: ::core::ffi::c_ulong,
    nm: *mut _ccu_nm,
) -> ::core::ffi::c_ulong {
    let mut best_rate = 0;
    let mut best_n = 0;
    let mut best_m = 0;
    let mut _n = (*nm).min_n;
    while _n <= (*nm).max_n {
        let mut _m = (*nm).min_m;
        while _m <= (*nm).max_m {
            let tmp_rate = ccu_nm_calc_rate(parent, _n, _m);
            if ccu_is_better_rate(common, rate, tmp_rate, best_rate) {
                best_rate = tmp_rate;
                best_n = _n;
                best_m = _m;
            }
            _m += 1;
        }
        _n += 1;
    }
    (*nm).n = best_n;
    (*nm).m = best_m;
    best_rate
}

unsafe fn ccu_nm_disable(hw: *mut clk_hw) {
    let nm = hw_to_ccu_nm(hw);
    ccu_gate_helper_disable(&mut (*nm).common, (*nm).enable);
}

unsafe fn ccu_nm_enable(hw: *mut clk_hw) -> i32 {
    let nm = hw_to_ccu_nm(hw);
    ccu_gate_helper_enable(&mut (*nm).common, (*nm).enable)
}

unsafe fn ccu_nm_is_enabled(hw: *mut clk_hw) -> i32 {
    let nm = hw_to_ccu_nm(hw);
    ccu_gate_helper_is_enabled(&mut (*nm).common, (*nm).enable)
}

unsafe fn ccu_nm_recalc_rate(hw: *mut clk_hw, parent_rate: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    let nm = hw_to_ccu_nm(hw);
    let rate;
    if ccu_frac_helper_is_enabled(&mut (*nm).common, &mut (*nm).frac) {
        rate = ccu_frac_helper_read_rate(&mut (*nm).common, &mut (*nm).frac);
        if (*nm).common.features & CCU_FEATURE_FIXED_POSTDIV != 0 {
            return rate / (*nm).fixed_post_div;
        }
        return rate;
    }
    let reg = readl((*nm).common.base.add((*nm).common.reg));
    let mut n = (reg >> (*nm).n.shift) & ((1 << (*nm).n.width) - 1);
    n += (*nm).n.offset;
    if n == 0 { n += 1; }
    let mut m = (reg >> (*nm).m.shift) & ((1 << (*nm).m.width) - 1);
    m += (*nm).m.offset;
    if m == 0 { m += 1; }
    let result = if ccu_sdm_helper_is_enabled(&mut (*nm).common, &mut (*nm).sdm) {
        ccu_sdm_helper_read_rate(&mut (*nm).common, &mut (*nm).sdm, m, n)
    } else {
        ccu_nm_calc_rate(parent_rate, n, m)
    };
    if (*nm).common.features & CCU_FEATURE_FIXED_POSTDIV != 0 { result / (*nm).fixed_post_div } else { result }
}

unsafe fn ccu_nm_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let nm = hw_to_ccu_nm(hw);
    let mut _nm = _ccu_nm { n: 0, min_n: 0, max_n: 0, m: 0, min_m: 0, max_m: 0 };
    if (*nm).common.features & CCU_FEATURE_FIXED_POSTDIV != 0 { (*req).rate *= (*nm).fixed_post_div; }
    if (*req).rate < (*nm).min_rate { (*req).rate = (*nm).min_rate; if (*nm).common.features & CCU_FEATURE_FIXED_POSTDIV != 0 { (*req).rate /= (*nm).fixed_post_div; } return 0; }
    if (*nm).max_rate != 0 && (*req).rate > (*nm).max_rate { (*req).rate = (*nm).max_rate; if (*nm).common.features & CCU_FEATURE_FIXED_POSTDIV != 0 { (*req).rate /= (*nm).fixed_post_div; } return 0; }
    if ccu_frac_helper_has_rate(&mut (*nm).common, &mut (*nm).frac, (*req).rate) || ccu_sdm_helper_has_rate(&mut (*nm).common, &mut (*nm).sdm, (*req).rate) { if (*nm).common.features & CCU_FEATURE_FIXED_POSTDIV != 0 { (*req).rate /= (*nm).fixed_post_div; } return 0; }
    _nm.min_n = if (*nm).n.min != 0 { (*nm).n.min } else { 1 };
    _nm.max_n = if (*nm).n.max != 0 { (*nm).n.max } else { 1 << (*nm).n.width };
    _nm.min_m = 1;
    _nm.max_m = if (*nm).m.max != 0 { (*nm).m.max } else { 1 << (*nm).m.width };
    (*req).rate = ccu_nm_find_best(&mut (*nm).common, (*req).best_parent_rate, (*req).rate, &mut _nm);
    if (*nm).common.features & CCU_FEATURE_FIXED_POSTDIV != 0 { (*req).rate /= (*nm).fixed_post_div; }
    0
}

// The rate-setting implementation follows the same low-level register and helper
// operations as the C source; external helper and clock types are supplied by the
// surrounding translation unit.
unsafe fn ccu_nm_set_rate(hw: *mut clk_hw, mut rate: ::core::ffi::c_ulong, parent_rate: ::core::ffi::c_ulong) -> i32 {
    let nm = hw_to_ccu_nm(hw);
    let mut _nm = _ccu_nm { n: 0, min_n: 0, max_n: 0, m: 0, min_m: 0, max_m: 0 };
    let mut flags: ::core::ffi::c_ulong = 0;
    if (*nm).common.features & CCU_FEATURE_FIXED_POSTDIV != 0 { rate *= (*nm).fixed_post_div; }
    if ccu_frac_helper_has_rate(&mut (*nm).common, &mut (*nm).frac, rate) {
        spin_lock_irqsave((*nm).common.lock, &mut flags);
        let mut reg = readl((*nm).common.base.add((*nm).common.reg));
        reg &= !genmask((*nm).m.width + (*nm).m.shift - 1, (*nm).m.shift);
        writel(reg, (*nm).common.base.add((*nm).common.reg));
        spin_unlock_irqrestore((*nm).common.lock, flags);
        ccu_frac_helper_enable(&mut (*nm).common, &mut (*nm).frac);
        return ccu_frac_helper_set_rate(&mut (*nm).common, &mut (*nm).frac, rate, (*nm).lock);
    } else { ccu_frac_helper_disable(&mut (*nm).common, &mut (*nm).frac); }
    _nm.min_n = if (*nm).n.min != 0 { (*nm).n.min } else { 1 };
    _nm.max_n = if (*nm).n.max != 0 { (*nm).n.max } else { 1 << (*nm).n.width };
    _nm.min_m = 1;
    _nm.max_m = if (*nm).m.max != 0 { (*nm).m.max } else { 1 << (*nm).m.width };
    if ccu_sdm_helper_has_rate(&mut (*nm).common, &mut (*nm).sdm, rate) {
        ccu_sdm_helper_enable(&mut (*nm).common, &mut (*nm).sdm, rate);
        ccu_sdm_helper_get_factors(&mut (*nm).common, &mut (*nm).sdm, rate, &mut _nm.m, &mut _nm.n);
    } else {
        ccu_sdm_helper_disable(&mut (*nm).common, &mut (*nm).sdm);
        ccu_nm_find_best(&mut (*nm).common, parent_rate, rate, &mut _nm);
    }
    spin_lock_irqsave((*nm).common.lock, &mut flags);
    let mut reg = readl((*nm).common.base.add((*nm).common.reg));
    reg &= !genmask((*nm).n.width + (*nm).n.shift - 1, (*nm).n.shift);
    reg &= !genmask((*nm).m.width + (*nm).m.shift - 1, (*nm).m.shift);
    reg |= (_nm.n - (*nm).n.offset) << (*nm).n.shift;
    reg |= (_nm.m - (*nm).m.offset) << (*nm).m.shift;
    writel(reg, (*nm).common.base.add((*nm).common.reg));
    spin_unlock_irqrestore((*nm).common.lock, flags);
    ccu_helper_wait_for_lock(&mut (*nm).common, (*nm).lock);
    0
}

const ccu_nm_ops: clk_ops = clk_ops {
    disable: Some(ccu_nm_disable), enable: Some(ccu_nm_enable), is_enabled: Some(ccu_nm_is_enabled),
    recalc_rate: Some(ccu_nm_recalc_rate), determine_rate: Some(ccu_nm_determine_rate), set_rate: Some(ccu_nm_set_rate),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
