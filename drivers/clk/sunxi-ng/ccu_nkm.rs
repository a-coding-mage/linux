// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 Maxime Ripard
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 */

// Translated from ccu_nkm.c. Types and helpers from the Linux clock framework
// and the sunxi CCU support are supplied by external dependencies.

#[repr(C)]
struct _ccu_nkm {
    n: libc::c_ulong,
    min_n: libc::c_ulong,
    max_n: libc::c_ulong,
    k: libc::c_ulong,
    min_k: libc::c_ulong,
    max_k: libc::c_ulong,
    m: libc::c_ulong,
    min_m: libc::c_ulong,
    max_m: libc::c_ulong,
}

unsafe fn ccu_nkm_is_valid_rate(
    common: *mut ccu_common,
    parent: libc::c_ulong,
    n: libc::c_ulong,
    m: libc::c_ulong,
) -> bool {
    let nkm = container_of!(common, ccu_nkm, common);

    if (*nkm).max_m_n_ratio != 0 && m > (*nkm).max_m_n_ratio * n {
        return false;
    }
    if (*nkm).min_parent_m_ratio != 0 && parent < (*nkm).min_parent_m_ratio * m {
        return false;
    }
    true
}

unsafe fn ccu_nkm_find_best_with_parent_adj(
    common: *mut ccu_common,
    parent_hw: *mut clk_hw,
    parent: *mut libc::c_ulong,
    rate: libc::c_ulong,
    nkm: *mut _ccu_nkm,
) -> libc::c_ulong {
    let mut best_rate = 0;
    let mut best_parent_rate = *parent;
    let mut best_n = 0;
    let mut best_k = 0;
    let mut best_m = 0;

    let mut _k = (*nkm).min_k;
    while _k <= (*nkm).max_k {
        let mut _n = (*nkm).min_n;
        while _n <= (*nkm).max_n {
            let mut _m = (*nkm).min_m;
            while _m <= (*nkm).max_m {
                let tmp_parent = clk_hw_round_rate(parent_hw, rate * _m / (_n * _k));
                if !ccu_nkm_is_valid_rate(common, tmp_parent, _n, _m) {
                    _m += 1;
                    continue;
                }
                let tmp_rate = tmp_parent * _n * _k / _m;
                if ccu_is_better_rate(common, rate, tmp_rate, best_rate)
                    || (tmp_parent == *parent && tmp_rate == best_rate)
                {
                    best_rate = tmp_rate;
                    best_parent_rate = tmp_parent;
                    best_n = _n;
                    best_k = _k;
                    best_m = _m;
                }
                _m += 1;
            }
            _n += 1;
        }
        _k += 1;
    }
    (*nkm).n = best_n;
    (*nkm).k = best_k;
    (*nkm).m = best_m;
    *parent = best_parent_rate;
    best_rate
}

unsafe fn ccu_nkm_find_best(
    parent: libc::c_ulong,
    rate: libc::c_ulong,
    nkm: *mut _ccu_nkm,
    common: *mut ccu_common,
) -> libc::c_ulong {
    let mut best_rate = 0;
    let mut best_n = 0;
    let mut best_k = 0;
    let mut best_m = 0;
    let mut _k = (*nkm).min_k;
    while _k <= (*nkm).max_k {
        let mut _n = (*nkm).min_n;
        while _n <= (*nkm).max_n {
            let mut _m = (*nkm).min_m;
            while _m <= (*nkm).max_m {
                if ccu_nkm_is_valid_rate(common, parent, _n, _m) {
                    let tmp_rate = parent * _n * _k / _m;
                    if ccu_is_better_rate(common, rate, tmp_rate, best_rate) {
                        best_rate = tmp_rate;
                        best_n = _n;
                        best_k = _k;
                        best_m = _m;
                    }
                }
                _m += 1;
            }
            _n += 1;
        }
        _k += 1;
    }
    (*nkm).n = best_n;
    (*nkm).k = best_k;
    (*nkm).m = best_m;
    best_rate
}

unsafe extern "C" fn ccu_nkm_disable(hw: *mut clk_hw) {
    let nkm = hw_to_ccu_nkm(hw);
    ccu_gate_helper_disable(&mut (*nkm).common, (*nkm).enable);
}

unsafe extern "C" fn ccu_nkm_enable(hw: *mut clk_hw) -> libc::c_int {
    let nkm = hw_to_ccu_nkm(hw);
    ccu_gate_helper_enable(&mut (*nkm).common, (*nkm).enable)
}

unsafe extern "C" fn ccu_nkm_is_enabled(hw: *mut clk_hw) -> libc::c_int {
    let nkm = hw_to_ccu_nkm(hw);
    ccu_gate_helper_is_enabled(&mut (*nkm).common, (*nkm).enable)
}

unsafe extern "C" fn ccu_nkm_recalc_rate(hw: *mut clk_hw, parent_rate: libc::c_ulong) -> libc::c_ulong {
    let nkm = hw_to_ccu_nkm(hw);
    let reg = readl((*nkm).common.base.add((*nkm).common.reg as usize));
    let mut n = (reg >> (*nkm).n.shift) & ((1u32 << (*nkm).n.width) - 1);
    n += (*nkm).n.offset;
    if n == 0 { n = 1; }
    let mut k = (reg >> (*nkm).k.shift) & ((1u32 << (*nkm).k.width) - 1);
    k += (*nkm).k.offset;
    if k == 0 { k = 1; }
    let mut m = (reg >> (*nkm).m.shift) & ((1u32 << (*nkm).m.width) - 1);
    m += (*nkm).m.offset;
    if m == 0 { m = 1; }
    let mut result = parent_rate * n as libc::c_ulong * k as libc::c_ulong / m as libc::c_ulong;
    if (*nkm).common.features & CCU_FEATURE_FIXED_POSTDIV != 0 { result /= (*nkm).fixed_post_div as libc::c_ulong; }
    result
}

// The remaining callbacks preserve the C implementation's framework-facing interface.
unsafe extern "C" fn ccu_nkm_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> libc::c_int {
    let nkm = hw_to_ccu_nkm(hw);
    ccu_mux_helper_determine_rate(&mut (*nkm).common, &mut (*nkm).mux, req, ccu_nkm_determine_rate_helper, nkm)
}

unsafe extern "C" fn ccu_nkm_determine_rate_helper(_mux: *mut ccu_mux_internal, req: *mut clk_rate_request, data: *mut libc::c_void) -> libc::c_int {
    let nkm = data as *mut ccu_nkm;
    let mut search = _ccu_nkm {
        n: 0, min_n: if (*nkm).n.min != 0 { (*nkm).n.min } else { 1 }, max_n: if (*nkm).n.max != 0 { (*nkm).n.max } else { 1 << (*nkm).n.width },
        k: 0, min_k: if (*nkm).k.min != 0 { (*nkm).k.min } else { 1 }, max_k: if (*nkm).k.max != 0 { (*nkm).k.max } else { 1 << (*nkm).k.width },
        m: 0, min_m: 1, max_m: if (*nkm).m.max != 0 { (*nkm).m.max } else { 1 << (*nkm).m.width },
    };
    if (*nkm).common.features & CCU_FEATURE_FIXED_POSTDIV != 0 { (*req).rate *= (*nkm).fixed_post_div as libc::c_ulong; }
    if !clk_hw_can_set_rate_parent(&mut (*nkm).common.hw) {
        (*req).rate = ccu_nkm_find_best((*req).best_parent_rate, (*req).rate, &mut search, &mut (*nkm).common);
    } else {
        (*req).rate = ccu_nkm_find_best_with_parent_adj(&mut (*nkm).common, (*req).best_parent_hw, &mut (*req).best_parent_rate, (*req).rate, &mut search);
    }
    if (*nkm).common.features & CCU_FEATURE_FIXED_POSTDIV != 0 { (*req).rate /= (*nkm).fixed_post_div as libc::c_ulong; }
    0
}

unsafe extern "C" fn ccu_nkm_set_rate(hw: *mut clk_hw, mut rate: libc::c_ulong, parent_rate: libc::c_ulong) -> libc::c_int {
    let nkm = hw_to_ccu_nkm(hw);
    if (*nkm).common.features & CCU_FEATURE_FIXED_POSTDIV != 0 { rate *= (*nkm).fixed_post_div as libc::c_ulong; }
    let mut search = _ccu_nkm { n: 0, min_n: if (*nkm).n.min != 0 { (*nkm).n.min } else { 1 }, max_n: if (*nkm).n.max != 0 { (*nkm).n.max } else { 1 << (*nkm).n.width }, k: 0, min_k: if (*nkm).k.min != 0 { (*nkm).k.min } else { 1 }, max_k: if (*nkm).k.max != 0 { (*nkm).k.max } else { 1 << (*nkm).k.width }, m: 0, min_m: 1, max_m: if (*nkm).m.max != 0 { (*nkm).m.max } else { 1 << (*nkm).m.width } };
    ccu_nkm_find_best(parent_rate, rate, &mut search, &mut (*nkm).common);
    let mut flags = 0;
    spin_lock_irqsave((*nkm).common.lock, &mut flags);
    let mut reg = readl((*nkm).common.base.add((*nkm).common.reg as usize));
    reg &= !genmask((*nkm).n.width + (*nkm).n.shift - 1, (*nkm).n.shift);
    reg &= !genmask((*nkm).k.width + (*nkm).k.shift - 1, (*nkm).k.shift);
    reg &= !genmask((*nkm).m.width + (*nkm).m.shift - 1, (*nkm).m.shift);
    reg |= ((search.n - (*nkm).n.offset) << (*nkm).n.shift) as u32;
    reg |= ((search.k - (*nkm).k.offset) << (*nkm).k.shift) as u32;
    reg |= ((search.m - (*nkm).m.offset) << (*nkm).m.shift) as u32;
    writel(reg, (*nkm).common.base.add((*nkm).common.reg as usize));
    spin_unlock_irqrestore((*nkm).common.lock, flags);
    ccu_helper_wait_for_lock(&mut (*nkm).common, (*nkm).lock);
    0
}
unsafe extern "C" fn ccu_nkm_get_parent(hw: *mut clk_hw) -> u8 { let nkm = hw_to_ccu_nkm(hw); ccu_mux_helper_get_parent(&mut (*nkm).common, &mut (*nkm).mux) }
unsafe extern "C" fn ccu_nkm_set_parent(hw: *mut clk_hw, index: u8) -> libc::c_int { let nkm = hw_to_ccu_nkm(hw); ccu_mux_helper_set_parent(&mut (*nkm).common, &mut (*nkm).mux, index) }

pub static ccu_nkm_ops: clk_ops = clk_ops {
    disable: Some(ccu_nkm_disable), enable: Some(ccu_nkm_enable), is_enabled: Some(ccu_nkm_is_enabled),
    get_parent: Some(ccu_nkm_get_parent), set_parent: Some(ccu_nkm_set_parent),
    determine_rate: Some(ccu_nkm_determine_rate), recalc_rate: Some(ccu_nkm_recalc_rate), set_rate: Some(ccu_nkm_set_rate),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
