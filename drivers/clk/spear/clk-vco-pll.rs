// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012 ST Microelectronics
 * Viresh Kumar <vireshk@kernel.org>
 *
 * VCO-PLL clock implementation
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented in this isolated translation.

const PLL_MODE_NORMAL: u32 = 0;
const PLL_MODE_FRACTION: u32 = 1;
const PLL_MODE_DITH_DSM: u32 = 2;
const PLL_MODE_DITH_SSM: u32 = 3;
const PLL_MODE_MASK: u32 = 3;
const PLL_MODE_SHIFT: u32 = 3;
const PLL_ENABLE: u32 = 2;
const PLL_LOCK_SHIFT: u32 = 0;
const PLL_LOCK_MASK: u32 = 1;
const PLL_NORM_FDBK_M_MASK: u32 = 0xff;
const PLL_NORM_FDBK_M_SHIFT: u32 = 24;
const PLL_DITH_FDBK_M_MASK: u32 = 0xffff;
const PLL_DITH_FDBK_M_SHIFT: u32 = 16;
const PLL_DIV_P_MASK: u32 = 0x7;
const PLL_DIV_P_SHIFT: u32 = 8;
const PLL_DIV_N_MASK: u32 = 0xff;
const PLL_DIV_N_SHIFT: u32 = 0;

unsafe fn pll_calc_rate(
    rtbl: *mut pll_rate_tbl,
    prate: c_ulong,
    index: c_int,
    pll_rate: *mut c_ulong,
) -> c_ulong {
    let mut rate = prate;
    let mode: c_uint = if (*rtbl.add(index as usize)).mode != 0 { 256 } else { 1 };
    let entry = &*rtbl.add(index as usize);
    rate = (((2 * (rate / 10000)) * entry.m) / (mode * entry.n));
    if !pll_rate.is_null() {
        *pll_rate = (rate / (1 << entry.p)) * 10000;
    }
    rate * 10000
}

unsafe fn clk_pll_round_rate_index(
    hw: *mut clk_hw, drate: c_ulong, prate: *mut c_ulong, index: *mut c_int,
) -> c_long {
    let pll = to_clk_pll(hw);
    let mut prev_rate: c_ulong;
    let mut vco_prev_rate: c_ulong;
    let mut rate: c_ulong = 0;
    let vco_parent_rate = clk_hw_get_rate(clk_hw_get_parent(clk_hw_get_parent(hw)));
    if prate.is_null() {
        pr_err("%s: prate is must for pll clk\n", __func__);
        return -EINVAL as c_long;
    }
    *index = 0;
    while *index < (*pll).vco.rtbl_cnt {
        prev_rate = rate;
        vco_prev_rate = *prate;
        *prate = pll_calc_rate((*pll).vco.rtbl, vco_parent_rate, *index, &mut rate);
        if drate < rate {
            if *index != 0 { rate = prev_rate; *prate = vco_prev_rate; *index -= 1; }
            break;
        }
        *index += 1;
    }
    rate as c_long
}

unsafe fn clk_pll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int {
    let mut unused = 0;
    (*req).rate = clk_pll_round_rate_index(hw, (*req).rate, &mut (*req).best_parent_rate, &mut unused) as c_ulong;
    0
}

unsafe fn clk_pll_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let pll = to_clk_pll(hw); let mut flags = 0; let mut p;
    if !(*pll).vco.lock.is_null() { spin_lock_irqsave((*pll).vco.lock, &mut flags); }
    p = readl_relaxed((*pll).vco.cfg_reg);
    if !(*pll).vco.lock.is_null() { spin_unlock_irqrestore((*pll).vco.lock, flags); }
    p = (p >> PLL_DIV_P_SHIFT) & PLL_DIV_P_MASK;
    parent_rate / (1 << p)
}

unsafe fn clk_pll_set_rate(hw: *mut clk_hw, drate: c_ulong, _prate: c_ulong) -> c_int {
    let pll = to_clk_pll(hw); let rtbl = (*pll).vco.rtbl; let mut flags = 0; let mut val; let mut i = 0;
    clk_pll_round_rate_index(hw, drate, core::ptr::null_mut(), &mut i);
    if !(*pll).vco.lock.is_null() { spin_lock_irqsave((*pll).vco.lock, &mut flags); }
    val = readl_relaxed((*pll).vco.cfg_reg);
    val &= !(PLL_DIV_P_MASK << PLL_DIV_P_SHIFT);
    val |= ((*rtbl.add(i as usize)).p & PLL_DIV_P_MASK) << PLL_DIV_P_SHIFT;
    writel_relaxed(val, (*pll).vco.cfg_reg);
    if !(*pll).vco.lock.is_null() { spin_unlock_irqrestore((*pll).vco.lock, flags); }
    0
}

unsafe fn vco_calc_rate(hw: *mut clk_hw, prate: c_ulong, index: c_int) -> c_ulong {
    pll_calc_rate((*to_clk_vco(hw)).rtbl, prate, index, core::ptr::null_mut())
}

unsafe fn clk_vco_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int {
    let vco = to_clk_vco(hw); let mut unused = 0;
    (*req).rate = clk_round_rate_index(hw, (*req).rate, (*req).best_parent_rate, vco_calc_rate, (*vco).rtbl_cnt, &mut unused);
    0
}

unsafe fn clk_vco_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let vco = to_clk_vco(hw); let mut flags = 0; let mut num = 2; let mut den = 0; let mut val; let mode;
    if !(*vco).lock.is_null() { spin_lock_irqsave((*vco).lock, &mut flags); }
    mode = (readl_relaxed((*vco).mode_reg) >> PLL_MODE_SHIFT) & PLL_MODE_MASK;
    val = readl_relaxed((*vco).cfg_reg);
    if !(*vco).lock.is_null() { spin_unlock_irqrestore((*vco).lock, flags); }
    den = (val >> PLL_DIV_N_SHIFT) & PLL_DIV_N_MASK;
    if mode == 0 { num *= (val >> PLL_NORM_FDBK_M_SHIFT) & PLL_NORM_FDBK_M_MASK; }
    else { num *= (val >> PLL_DITH_FDBK_M_SHIFT) & PLL_DITH_FDBK_M_MASK; den *= 256; }
    if den == 0 { WARN(1, "%s: denominator can't be zero\n", __func__); return 0; }
    (((parent_rate / 10000) * num) / den) * 10000
}

unsafe fn clk_vco_set_rate(hw: *mut clk_hw, drate: c_ulong, prate: c_ulong) -> c_int {
    let vco = to_clk_vco(hw); let rtbl = (*vco).rtbl; let mut flags = 0; let mut val; let mut i = 0;
    clk_round_rate_index(hw, drate, prate, vco_calc_rate, (*vco).rtbl_cnt, &mut i);
    if !(*vco).lock.is_null() { spin_lock_irqsave((*vco).lock, &mut flags); }
    let e = &*rtbl.add(i as usize);
    val = readl_relaxed((*vco).mode_reg); val &= !(PLL_MODE_MASK << PLL_MODE_SHIFT); val |= (e.mode & PLL_MODE_MASK) << PLL_MODE_SHIFT; writel_relaxed(val, (*vco).mode_reg);
    val = readl_relaxed((*vco).cfg_reg); val &= !(PLL_DIV_N_MASK << PLL_DIV_N_SHIFT); val |= (e.n & PLL_DIV_N_MASK) << PLL_DIV_N_SHIFT; val &= !(PLL_DITH_FDBK_M_MASK << PLL_DITH_FDBK_M_SHIFT);
    if e.mode != 0 { val |= (e.m & PLL_DITH_FDBK_M_MASK) << PLL_DITH_FDBK_M_SHIFT; } else { val |= (e.m & PLL_NORM_FDBK_M_MASK) << PLL_NORM_FDBK_M_SHIFT; }
    writel_relaxed(val, (*vco).cfg_reg);
    if !(*vco).lock.is_null() { spin_unlock_irqrestore((*vco).lock, flags); }
    0
}

pub unsafe fn clk_register_vco_pll(vco_name: *const c_char, pll_name: *const c_char, vco_gate_name: *const c_char, parent_name: *const c_char, flags: c_ulong, mode_reg: *mut core::ffi::c_void, cfg_reg: *mut core::ffi::c_void, rtbl: *mut pll_rate_tbl, rtbl_cnt: u8, lock: *mut spinlock_t, pll_clk: *mut *mut clk, vco_gate_clk: *mut *mut clk) -> *mut clk {
    if vco_name.is_null() || pll_name.is_null() || parent_name.is_null() || mode_reg.is_null() || cfg_reg.is_null() || rtbl.is_null() || rtbl_cnt == 0 { pr_err("Invalid arguments passed"); return ERR_PTR(-EINVAL); }
    let vco = kzalloc_obj::<clk_vco>(); if vco.is_null() { return ERR_PTR(-ENOMEM); }
    let pll = kzalloc_obj::<clk_pll>(); if pll.is_null() { kfree(vco as *mut _); return ERR_PTR(-ENOMEM); }
    (*vco).mode_reg = mode_reg; (*vco).cfg_reg = cfg_reg; (*vco).rtbl = rtbl; (*vco).rtbl_cnt = rtbl_cnt; (*vco).lock = lock;
    (*pll).vco = vco; let mut vco_init: clk_init_data = core::mem::zeroed(); let mut pll_init: clk_init_data = core::mem::zeroed();
    (*vco).hw.init = &mut vco_init; (*pll).hw.init = &mut pll_init;
    let gate = if !vco_gate_name.is_null() { let g = clk_register_gate(core::ptr::null_mut(), vco_gate_name, parent_name, 0, mode_reg, PLL_ENABLE, 0, lock); if IS_ERR_OR_NULL(g) { kfree(pll as *mut _); kfree(vco as *mut _); return ERR_PTR(-ENOMEM); } if !vco_gate_clk.is_null() { *vco_gate_clk = g; } g } else { core::ptr::null_mut() };
    vco_init.name = vco_name; vco_init.flags = flags; vco_init.parent_names = if !gate.is_null() { &vco_gate_name } else { &parent_name }; vco_init.num_parents = 1;
    pll_init.name = pll_name; pll_init.flags = CLK_SET_RATE_PARENT; pll_init.parent_names = &vco_name; pll_init.num_parents = 1;
    let vco_clk = clk_register(core::ptr::null_mut(), &mut (*vco).hw); if IS_ERR_OR_NULL(vco_clk) { kfree(pll as *mut _); kfree(vco as *mut _); return ERR_PTR(-ENOMEM); }
    let pll_clk_local = clk_register(core::ptr::null_mut(), &mut (*pll).hw); if IS_ERR_OR_NULL(pll_clk_local) { clk_unregister(vco_clk); kfree(pll as *mut _); kfree(vco as *mut _); return ERR_PTR(-ENOMEM); }
    if !pll_clk.is_null() { *pll_clk = pll_clk_local; } vco_clk
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
