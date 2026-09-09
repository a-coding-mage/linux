// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 Maxime Ripard
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
struct _ccu_nkmp {
    n: ::core::ffi::c_ulong,
    min_n: ::core::ffi::c_ulong,
    max_n: ::core::ffi::c_ulong,
    k: ::core::ffi::c_ulong,
    min_k: ::core::ffi::c_ulong,
    max_k: ::core::ffi::c_ulong,
    m: ::core::ffi::c_ulong,
    min_m: ::core::ffi::c_ulong,
    max_m: ::core::ffi::c_ulong,
    p: ::core::ffi::c_ulong,
    min_p: ::core::ffi::c_ulong,
    max_p: ::core::ffi::c_ulong,
}

unsafe fn ccu_nkmp_calc_rate(
    parent: ::core::ffi::c_ulong,
    n: ::core::ffi::c_ulong,
    k: ::core::ffi::c_ulong,
    m: ::core::ffi::c_ulong,
    p: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    let mut rate = parent as u64;
    rate = rate.wrapping_mul(n.wrapping_mul(k) as u64);
    rate /= m.wrapping_mul(p) as u64;
    rate as ::core::ffi::c_ulong
}

unsafe fn ccu_nkmp_find_best(
    parent: ::core::ffi::c_ulong,
    rate: ::core::ffi::c_ulong,
    nkmp: *mut _ccu_nkmp,
) -> ::core::ffi::c_ulong {
    let mut best_rate = 0;
    let mut best_n = 0;
    let mut best_k = 0;
    let mut best_m = 0;
    let mut best_p = 0;
    let mut _k = (*nkmp).min_k;
    while _k <= (*nkmp).max_k {
        let mut _n = (*nkmp).min_n;
        while _n <= (*nkmp).max_n {
            let mut _m = (*nkmp).min_m;
            while _m <= (*nkmp).max_m {
                let mut _p = (*nkmp).min_p;
                while _p <= (*nkmp).max_p {
                    let tmp_rate = ccu_nkmp_calc_rate(parent, _n, _k, _m, _p);
                    if tmp_rate <= rate && rate - tmp_rate < rate - best_rate {
                        best_rate = tmp_rate;
                        best_n = _n;
                        best_k = _k;
                        best_m = _m;
                        best_p = _p;
                    }
                    _p <<= 1;
                }
                _m += 1;
            }
            _n += 1;
        }
        _k += 1;
    }
    (*nkmp).n = best_n;
    (*nkmp).k = best_k;
    (*nkmp).m = best_m;
    (*nkmp).p = best_p;
    best_rate
}

unsafe fn ccu_nkmp_disable(hw: *mut clk_hw) {
    let nkmp = hw_to_ccu_nkmp(hw);
    ccu_gate_helper_disable(&mut (*nkmp).common, (*nkmp).enable);
}

unsafe fn ccu_nkmp_enable(hw: *mut clk_hw) -> i32 {
    let nkmp = hw_to_ccu_nkmp(hw);
    ccu_gate_helper_enable(&mut (*nkmp).common, (*nkmp).enable)
}

unsafe fn ccu_nkmp_is_enabled(hw: *mut clk_hw) -> i32 {
    let nkmp = hw_to_ccu_nkmp(hw);
    ccu_gate_helper_is_enabled(&mut (*nkmp).common, (*nkmp).enable)
}

unsafe fn ccu_nkmp_recalc_rate(hw: *mut clk_hw, parent_rate: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    let nkmp = hw_to_ccu_nkmp(hw);
    let reg = readl((*nkmp).common.base.add((*nkmp).common.reg as usize));
    let mut n = (reg >> (*nkmp).n.shift) & ((1 << (*nkmp).n.width) - 1);
    n += (*nkmp).n.offset;
    if n == 0 { n += 1; }
    let mut k = (reg >> (*nkmp).k.shift) & ((1 << (*nkmp).k.width) - 1);
    k += (*nkmp).k.offset;
    if k == 0 { k += 1; }
    let mut m = (reg >> (*nkmp).m.shift) & ((1 << (*nkmp).m.width) - 1);
    m += (*nkmp).m.offset;
    if m == 0 { m += 1; }
    let p = (reg >> (*nkmp).p.shift) & ((1 << (*nkmp).p.width) - 1);
    let mut rate = ccu_nkmp_calc_rate(parent_rate, n, k, m, 1 << p);
    if (*nkmp).common.features & CCU_FEATURE_FIXED_POSTDIV != 0 {
        rate /= (*nkmp).fixed_post_div;
    }
    rate
}

unsafe fn ccu_nkmp_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let nkmp = hw_to_ccu_nkmp(hw);
    let mut v = _ccu_nkmp { n: 0, min_n: (*nkmp).n.min, max_n: (*nkmp).n.max, k: 0, min_k: (*nkmp).k.min, max_k: (*nkmp).k.max, m: 0, min_m: 1, max_m: (*nkmp).m.max, p: 0, min_p: 1, max_p: (*nkmp).p.max };
    if (*nkmp).common.features & CCU_FEATURE_FIXED_POSTDIV != 0 { (*req).rate *= (*nkmp).fixed_post_div; }
    if (*nkmp).max_rate != 0 && (*req).rate > (*nkmp).max_rate { (*req).rate = (*nkmp).max_rate; if (*nkmp).common.features & CCU_FEATURE_FIXED_POSTDIV != 0 { (*req).rate /= (*nkmp).fixed_post_div; } return 0; }
    v.min_n = if v.min_n != 0 { v.min_n } else { 1 }; v.max_n = if v.max_n != 0 { v.max_n } else { 1 << (*nkmp).n.width };
    v.min_k = if v.min_k != 0 { v.min_k } else { 1 }; v.max_k = if v.max_k != 0 { v.max_k } else { 1 << (*nkmp).k.width };
    v.max_m = if v.max_m != 0 { v.max_m } else { 1 << (*nkmp).m.width }; v.max_p = if v.max_p != 0 { v.max_p } else { 1 << ((1 << (*nkmp).p.width) - 1) };
    (*req).rate = ccu_nkmp_find_best((*req).best_parent_rate, (*req).rate, &mut v);
    if (*nkmp).common.features & CCU_FEATURE_FIXED_POSTDIV != 0 { (*req).rate /= (*nkmp).fixed_post_div; } 0
}

unsafe fn ccu_nkmp_set_rate(hw: *mut clk_hw, mut rate: ::core::ffi::c_ulong, parent_rate: ::core::ffi::c_ulong) -> i32 {
    let nkmp = hw_to_ccu_nkmp(hw);
    let mut v = _ccu_nkmp { n: 0, min_n: (*nkmp).n.min, max_n: (*nkmp).n.max, k: 0, min_k: (*nkmp).k.min, max_k: (*nkmp).k.max, m: 0, min_m: 1, max_m: (*nkmp).m.max, p: 0, min_p: 1, max_p: (*nkmp).p.max };
    if (*nkmp).common.features & CCU_FEATURE_FIXED_POSTDIV != 0 { rate *= (*nkmp).fixed_post_div; }
    v.min_n = if v.min_n != 0 { v.min_n } else { 1 }; v.max_n = if v.max_n != 0 { v.max_n } else { 1 << (*nkmp).n.width };
    v.min_k = if v.min_k != 0 { v.min_k } else { 1 }; v.max_k = if v.max_k != 0 { v.max_k } else { 1 << (*nkmp).k.width };
    v.max_m = if v.max_m != 0 { v.max_m } else { 1 << (*nkmp).m.width }; v.max_p = if v.max_p != 0 { v.max_p } else { 1 << ((1 << (*nkmp).p.width) - 1) };
    ccu_nkmp_find_best(parent_rate, rate, &mut v);
    let n_mask = if (*nkmp).n.width != 0 { GENMASK((*nkmp).n.width + (*nkmp).n.shift - 1, (*nkmp).n.shift) } else { 0 };
    let k_mask = if (*nkmp).k.width != 0 { GENMASK((*nkmp).k.width + (*nkmp).k.shift - 1, (*nkmp).k.shift) } else { 0 };
    let m_mask = if (*nkmp).m.width != 0 { GENMASK((*nkmp).m.width + (*nkmp).m.shift - 1, (*nkmp).m.shift) } else { 0 };
    let p_mask = if (*nkmp).p.width != 0 { GENMASK((*nkmp).p.width + (*nkmp).p.shift - 1, (*nkmp).p.shift) } else { 0 };
    let mut flags = 0;
    spin_lock_irqsave((*nkmp).common.lock, &mut flags);
    let ptr = (*nkmp).common.base.add((*nkmp).common.reg as usize);
    let mut reg = readl(ptr);
    reg &= !(n_mask | k_mask | m_mask | p_mask);
    reg |= ((v.n - (*nkmp).n.offset) << (*nkmp).n.shift) & n_mask;
    reg |= ((v.k - (*nkmp).k.offset) << (*nkmp).k.shift) & k_mask;
    reg |= ((v.m - (*nkmp).m.offset) << (*nkmp).m.shift) & m_mask;
    reg |= (ilog2(v.p) << (*nkmp).p.shift) & p_mask;
    writel(reg, ptr);
    spin_unlock_irqrestore((*nkmp).common.lock, flags);
    ccu_helper_wait_for_lock(&mut (*nkmp).common, (*nkmp).lock);
    0
}

static ccu_nkmp_ops: clk_ops = clk_ops { disable: Some(ccu_nkmp_disable), enable: Some(ccu_nkmp_enable), is_enabled: Some(ccu_nkmp_is_enabled), recalc_rate: Some(ccu_nkmp_recalc_rate), determine_rate: Some(ccu_nkmp_determine_rate), set_rate: Some(ccu_nkmp_set_rate) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
