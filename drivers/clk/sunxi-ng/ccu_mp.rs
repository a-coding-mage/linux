// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 Maxime Ripard
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 */

// External Linux/kernel declarations and constants are supplied by dependencies.

unsafe fn next_div(div: u32, shift: bool) -> u32 {
    if shift { div << 1 } else { div + 1 }
}

unsafe fn ccu_mp_find_best(parent: c_ulong, rate: c_ulong, max_m: u32, max_p: u32,
                           shift: bool, m: *mut u32, p: *mut u32) -> c_ulong {
    let mut best_rate: c_ulong = 0;
    let mut best_m: u32 = 0;
    let mut best_p: u32 = 0;
    let mut _p: u32 = 1;
    while _p <= max_p {
        let mut _m: u32 = 1;
        while _m <= max_m {
            let tmp_rate = parent / _p as c_ulong / _m as c_ulong;
            if tmp_rate <= rate && (rate - tmp_rate) < (rate - best_rate) {
                best_rate = tmp_rate;
                best_m = _m;
                best_p = _p;
            }
            _m += 1;
        }
        _p = next_div(_p, shift);
    }
    *m = best_m;
    *p = best_p;
    best_rate
}

unsafe fn ccu_mp_find_best_with_parent_adj(hw: *mut clk_hw, parent: *mut c_ulong,
    rate: c_ulong, max_m: u32, max_p: u32, shift: bool) -> c_ulong {
    let parent_rate_saved = *parent;
    let maxdiv = core::cmp::min(c_ulong::MAX / rate, max_m as c_ulong * max_p as c_ulong);
    let mut best_rate: c_ulong = 0;
    let mut _p: u32 = 1;
    while _p <= max_p {
        let mut _m: u32 = 1;
        while _m <= max_m {
            let div = _m * _p;
            if div as c_ulong > maxdiv { break; }
            if rate * div as c_ulong == parent_rate_saved {
                *parent = parent_rate_saved;
                return rate;
            }
            let parent_rate = clk_hw_round_rate(hw, rate * div as c_ulong);
            let now = parent_rate / div as c_ulong;
            if now <= rate && now > best_rate {
                best_rate = now;
                *parent = parent_rate;
                if now == rate { return rate; }
            }
            _m += 1;
        }
        _p = next_div(_p, shift);
    }
    best_rate
}

unsafe fn ccu_mp_determine_rate_helper(mux: *mut ccu_mux_internal,
    req: *mut clk_rate_request, data: *mut c_void) -> c_int {
    let cmp = data as *mut ccu_mp;
    let mut shift = true;
    if ((*cmp).common.features & CCU_FEATURE_FIXED_POSTDIV) != 0 { (*req).rate *= (*cmp).fixed_post_div as c_ulong; }
    if ((*cmp).common.features & CCU_FEATURE_DUAL_DIV) != 0 { shift = false; }
    let max_m = if (*cmp).m.max != 0 { (*cmp).m.max } else { 1 << (*cmp).m.width };
    let max_p = if (*cmp).p.max != 0 { (*cmp).p.max } else if shift { 1 << ((1 << (*cmp).p.width) - 1) } else { 1 << (*cmp).p.width };
    if !clk_hw_can_set_rate_parent(&mut (*cmp).common.hw) {
        let mut m = 0;
        let mut p = 0;
        (*req).rate = ccu_mp_find_best((*req).best_parent_rate, (*req).rate, max_m, max_p, shift, &mut m, &mut p);
    } else {
        (*req).rate = ccu_mp_find_best_with_parent_adj((*req).best_parent_hw, &mut (*req).best_parent_rate, (*req).rate, max_m, max_p, shift);
    }
    if ((*cmp).common.features & CCU_FEATURE_FIXED_POSTDIV) != 0 { (*req).rate /= (*cmp).fixed_post_div as c_ulong; }
    0
}

unsafe fn ccu_mp_disable(hw: *mut clk_hw) { let cmp = hw_to_ccu_mp(hw); ccu_gate_helper_disable(&mut (*cmp).common, (*cmp).enable); }
unsafe fn ccu_mp_enable(hw: *mut clk_hw) -> c_int { let cmp = hw_to_ccu_mp(hw); ccu_gate_helper_enable(&mut (*cmp).common, (*cmp).enable) }
unsafe fn ccu_mp_is_enabled(hw: *mut clk_hw) -> c_int { let cmp = hw_to_ccu_mp(hw); ccu_gate_helper_is_enabled(&mut (*cmp).common, (*cmp).enable) }

unsafe fn ccu_mp_recalc_rate(hw: *mut clk_hw, mut parent_rate: c_ulong) -> c_ulong {
    let cmp = hw_to_ccu_mp(hw);
    parent_rate = ccu_mux_helper_apply_prediv(&mut (*cmp).common, &mut (*cmp).mux, -1, parent_rate);
    let reg = readl((*cmp).common.base.add((*cmp).common.reg as usize));
    let mut m = (reg >> (*cmp).m.shift) & ((1 << (*cmp).m.width) - 1); m += (*cmp).m.offset; if m == 0 { m = 1; }
    let p = (reg >> (*cmp).p.shift) & ((1 << (*cmp).p.width) - 1);
    let mut rate = if ((*cmp).common.features & CCU_FEATURE_DUAL_DIV) != 0 { parent_rate / (p + (*cmp).p.offset) as c_ulong / m as c_ulong } else { (parent_rate >> p) / m as c_ulong };
    if ((*cmp).common.features & CCU_FEATURE_FIXED_POSTDIV) != 0 { rate /= (*cmp).fixed_post_div as c_ulong; }
    rate
}

unsafe fn ccu_mp_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int { let cmp = hw_to_ccu_mp(hw); ccu_mux_helper_determine_rate(&mut (*cmp).common, &mut (*cmp).mux, req, ccu_mp_determine_rate_helper, cmp as *mut c_void) }
unsafe fn ccu_mp_set_rate(hw: *mut clk_hw, mut rate: c_ulong, mut parent_rate: c_ulong) -> c_int {
    let cmp = hw_to_ccu_mp(hw); let shift = ((*cmp).common.features & CCU_FEATURE_DUAL_DIV) == 0;
    parent_rate = ccu_mux_helper_apply_prediv(&mut (*cmp).common, &mut (*cmp).mux, -1, parent_rate);
    let max_m = if (*cmp).m.max != 0 { (*cmp).m.max } else { 1 << (*cmp).m.width };
    let max_p = if (*cmp).p.max != 0 { (*cmp).p.max } else if shift { 1 << ((1 << (*cmp).p.width) - 1) } else { 1 << (*cmp).p.width };
    if ((*cmp).common.features & CCU_FEATURE_FIXED_POSTDIV) != 0 { rate *= (*cmp).fixed_post_div as c_ulong; }
    let mut m = 0; let mut p = 0; ccu_mp_find_best(parent_rate, rate, max_m, max_p, shift, &mut m, &mut p);
    let mut flags = 0; spin_lock_irqsave((*cmp).common.lock, &mut flags);
    let mut reg = readl((*cmp).common.base.add((*cmp).common.reg as usize));
    reg &= !genmask((*cmp).m.width + (*cmp).m.shift - 1, (*cmp).m.shift); reg &= !genmask((*cmp).p.width + (*cmp).p.shift - 1, (*cmp).p.shift);
    reg |= (m - (*cmp).m.offset) << (*cmp).m.shift; reg |= if shift { ilog2(p) << (*cmp).p.shift } else { (p - (*cmp).p.offset) << (*cmp).p.shift };
    writel(reg, (*cmp).common.base.add((*cmp).common.reg as usize)); spin_unlock_irqrestore((*cmp).common.lock, flags); 0
}

unsafe fn ccu_mp_get_parent(hw: *mut clk_hw) -> u8 { let cmp = hw_to_ccu_mp(hw); ccu_mux_helper_get_parent(&mut (*cmp).common, &mut (*cmp).mux) }
unsafe fn ccu_mp_set_parent(hw: *mut clk_hw, index: u8) -> c_int { let cmp = hw_to_ccu_mp(hw); ccu_mux_helper_set_parent(&mut (*cmp).common, &mut (*cmp).mux, index) }

const ccu_mp_ops: clk_ops = clk_ops { disable: Some(ccu_mp_disable), enable: Some(ccu_mp_enable), is_enabled: Some(ccu_mp_is_enabled), get_parent: Some(ccu_mp_get_parent), set_parent: Some(ccu_mp_set_parent), determine_rate: Some(ccu_mp_determine_rate), recalc_rate: Some(ccu_mp_recalc_rate), set_rate: Some(ccu_mp_set_rate) };

/* Support for MMC timing mode switching. */
unsafe fn ccu_mp_mmc_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong { let rate = ccu_mp_recalc_rate(hw, parent_rate); let cm = hw_to_ccu_common(hw); let val = readl((*cm).base.add((*cm).reg as usize)); if val & CCU_MMC_NEW_TIMING_MODE != 0 { rate / 2 } else { rate } }
unsafe fn ccu_mp_mmc_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int { let cm = hw_to_ccu_common(hw); let val = readl((*cm).base.add((*cm).reg as usize)); if val & CCU_MMC_NEW_TIMING_MODE != 0 { (*req).rate *= 2; (*req).min_rate *= 2; (*req).max_rate *= 2; } let ret = ccu_mp_determine_rate(hw, req); if val & CCU_MMC_NEW_TIMING_MODE != 0 { (*req).rate /= 2; (*req).min_rate /= 2; (*req).max_rate /= 2; } ret }
unsafe fn ccu_mp_mmc_set_rate(hw: *mut clk_hw, mut rate: c_ulong, parent_rate: c_ulong) -> c_int { let cm = hw_to_ccu_common(hw); if readl((*cm).base.add((*cm).reg as usize)) & CCU_MMC_NEW_TIMING_MODE != 0 { rate *= 2; } ccu_mp_set_rate(hw, rate, parent_rate) }
const ccu_mp_mmc_ops: clk_ops = clk_ops { disable: Some(ccu_mp_disable), enable: Some(ccu_mp_enable), is_enabled: Some(ccu_mp_is_enabled), get_parent: Some(ccu_mp_get_parent), set_parent: Some(ccu_mp_set_parent), determine_rate: Some(ccu_mp_mmc_determine_rate), recalc_rate: Some(ccu_mp_mmc_recalc_rate), set_rate: Some(ccu_mp_mmc_set_rate) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
