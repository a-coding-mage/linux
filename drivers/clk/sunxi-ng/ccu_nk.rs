// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2016 Maxime Ripard
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 */

// C dependencies: linux/clk-provider.h, linux/io.h, ccu_gate.h, ccu_nk.h.

#[repr(C)]
struct _ccu_nk {
    n: ::core::ffi::c_ulong,
    min_n: ::core::ffi::c_ulong,
    max_n: ::core::ffi::c_ulong,
    k: ::core::ffi::c_ulong,
    min_k: ::core::ffi::c_ulong,
    max_k: ::core::ffi::c_ulong,
}

unsafe fn ccu_nk_find_best(
    parent: ::core::ffi::c_ulong,
    rate: ::core::ffi::c_ulong,
    nk: *mut _ccu_nk,
) -> ::core::ffi::c_ulong {
    let mut best_rate: ::core::ffi::c_ulong = 0;
    let mut best_k: u32 = 0;
    let mut best_n: u32 = 0;
    let mut _k: u32 = (*nk).min_k as u32;

    while _k <= (*nk).max_k as u32 {
        let mut _n: u32 = (*nk).min_n as u32;
        while _n <= (*nk).max_n as u32 {
            let tmp_rate = parent.wrapping_mul(_n as _).wrapping_mul(_k as _);
            if tmp_rate <= rate && rate - tmp_rate < rate - best_rate {
                best_rate = tmp_rate;
                best_k = _k;
                best_n = _n;
            }
            _n = _n.wrapping_add(1);
        }
        _k = _k.wrapping_add(1);
    }

    (*nk).k = best_k as _;
    (*nk).n = best_n as _;
    best_rate
}

unsafe fn ccu_nk_disable(hw: *mut clk_hw) {
    let nk = hw_to_ccu_nk(hw);
    ccu_gate_helper_disable(&mut (*nk).common, (*nk).enable);
}

unsafe fn ccu_nk_enable(hw: *mut clk_hw) -> i32 {
    let nk = hw_to_ccu_nk(hw);
    ccu_gate_helper_enable(&mut (*nk).common, (*nk).enable)
}

unsafe fn ccu_nk_is_enabled(hw: *mut clk_hw) -> i32 {
    let nk = hw_to_ccu_nk(hw);
    ccu_gate_helper_is_enabled(&mut (*nk).common, (*nk).enable)
}

unsafe fn ccu_nk_recalc_rate(hw: *mut clk_hw, parent_rate: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    let nk = hw_to_ccu_nk(hw);
    let reg = readl((*nk).common.base.add((*nk).common.reg as usize));
    let mut n = (reg >> (*nk).n.shift) & ((1u32 << (*nk).n.width) - 1);
    n += (*nk).n.offset;
    if n == 0 { n += 1; }
    let mut k = (reg >> (*nk).k.shift) & ((1u32 << (*nk).k.width) - 1);
    k += (*nk).k.offset;
    if k == 0 { k += 1; }
    let mut rate = parent_rate.wrapping_mul(n as _).wrapping_mul(k as _);
    if (*nk).common.features & CCU_FEATURE_FIXED_POSTDIV != 0 { rate /= (*nk).fixed_post_div; }
    rate
}

unsafe fn ccu_nk_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let nk = hw_to_ccu_nk(hw);
    let mut _nk = _ccu_nk { n: 0, min_n: 0, max_n: 0, k: 0, min_k: 0, max_k: 0 };
    if (*nk).common.features & CCU_FEATURE_FIXED_POSTDIV != 0 { (*req).rate *= (*nk).fixed_post_div; }
    _nk.min_n = if (*nk).n.min != 0 { (*nk).n.min as _ } else { 1 };
    _nk.max_n = if (*nk).n.max != 0 { (*nk).n.max as _ } else { 1 << (*nk).n.width };
    _nk.min_k = if (*nk).k.min != 0 { (*nk).k.min as _ } else { 1 };
    _nk.max_k = if (*nk).k.max != 0 { (*nk).k.max as _ } else { 1 << (*nk).k.width };
    (*req).rate = ccu_nk_find_best((*req).best_parent_rate, (*req).rate, &mut _nk);
    if (*nk).common.features & CCU_FEATURE_FIXED_POSTDIV != 0 { (*req).rate /= (*nk).fixed_post_div; }
    0
}

unsafe fn ccu_nk_set_rate(hw: *mut clk_hw, rate: ::core::ffi::c_ulong, parent_rate: ::core::ffi::c_ulong) -> i32 {
    let nk = hw_to_ccu_nk(hw);
    let mut _nk = _ccu_nk { n: 0, min_n: 0, max_n: 0, k: 0, min_k: 0, max_k: 0 };
    let mut rate = rate;
    if (*nk).common.features & CCU_FEATURE_FIXED_POSTDIV != 0 { rate = rate.wrapping_mul((*nk).fixed_post_div); }
    _nk.min_n = if (*nk).n.min != 0 { (*nk).n.min as _ } else { 1 };
    _nk.max_n = if (*nk).n.max != 0 { (*nk).n.max as _ } else { 1 << (*nk).n.width };
    _nk.min_k = if (*nk).k.min != 0 { (*nk).k.min as _ } else { 1 };
    _nk.max_k = if (*nk).k.max != 0 { (*nk).k.max as _ } else { 1 << (*nk).k.width };
    ccu_nk_find_best(parent_rate, rate, &mut _nk);

    let mut flags: ::core::ffi::c_ulong = 0;
    spin_lock_irqsave((*nk).common.lock, &mut flags);
    let mut reg = readl((*nk).common.base.add((*nk).common.reg as usize));
    reg &= !genmask((*nk).n.width + (*nk).n.shift - 1, (*nk).n.shift);
    reg &= !genmask((*nk).k.width + (*nk).k.shift - 1, (*nk).k.shift);
    reg |= ((_nk.k - (*nk).k.offset) as u32) << (*nk).k.shift;
    reg |= ((_nk.n - (*nk).n.offset) as u32) << (*nk).n.shift;
    writel(reg, (*nk).common.base.add((*nk).common.reg as usize));
    spin_unlock_irqrestore((*nk).common.lock, flags);
    ccu_helper_wait_for_lock(&mut (*nk).common, (*nk).lock);
    0
}

#[repr(C)]
pub struct clk_ops {
    pub disable: Option<unsafe fn(*mut clk_hw)>,
    pub enable: Option<unsafe fn(*mut clk_hw) -> i32>,
    pub is_enabled: Option<unsafe fn(*mut clk_hw) -> i32>,
    pub recalc_rate: Option<unsafe fn(*mut clk_hw, ::core::ffi::c_ulong) -> ::core::ffi::c_ulong>,
    pub determine_rate: Option<unsafe fn(*mut clk_hw, *mut clk_rate_request) -> i32>,
    pub set_rate: Option<unsafe fn(*mut clk_hw, ::core::ffi::c_ulong, ::core::ffi::c_ulong) -> i32>,
}

pub static ccu_nk_ops: clk_ops = clk_ops {
    disable: Some(ccu_nk_disable),
    enable: Some(ccu_nk_enable),
    is_enabled: Some(ccu_nk_is_enabled),
    recalc_rate: Some(ccu_nk_recalc_rate),
    determine_rate: Some(ccu_nk_determine_rate),
    set_rate: Some(ccu_nk_set_rate),
};

// EXPORT_SYMBOL_NS_GPL(ccu_nk_ops, "SUNXI_CCU");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
