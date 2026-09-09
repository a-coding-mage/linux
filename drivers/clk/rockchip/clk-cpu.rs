// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of clk-cpu.c; external kernel symbols are dependencies. */

#[repr(C)]
pub struct rockchip_cpuclk {
    pub hw: clk_hw,
    pub alt_parent: *mut clk,
    pub reg_base: *mut core::ffi::c_void,
    pub clk_nb: notifier_block,
    pub rate_count: u32,
    pub rate_table: *mut rockchip_cpuclk_rate_table,
    pub reg_data: *const rockchip_cpuclk_reg_data,
    pub lock: *mut spinlock_t,
}

#[inline]
unsafe fn to_rockchip_cpuclk_hw(hw: *mut clk_hw) -> *mut rockchip_cpuclk {
    (hw as *mut u8).sub(core::mem::offset_of!(rockchip_cpuclk, hw)) as *mut rockchip_cpuclk
}

#[inline]
unsafe fn to_rockchip_cpuclk_nb(nb: *mut notifier_block) -> *mut rockchip_cpuclk {
    (nb as *mut u8).sub(core::mem::offset_of!(rockchip_cpuclk, clk_nb)) as *mut rockchip_cpuclk
}

unsafe fn rockchip_get_cpuclk_settings(cpuclk: *mut rockchip_cpuclk, rate: u64) -> *const rockchip_cpuclk_rate_table {
    let rate_table = (*cpuclk).rate_table;
    for i in 0..(*cpuclk).rate_count as isize {
        if rate == (*rate_table.offset(i)).prate { return rate_table.offset(i); }
    }
    core::ptr::null()
}

unsafe fn rockchip_cpuclk_recalc_rate(hw: *mut clk_hw, parent_rate: u64) -> u64 {
    let cpuclk = to_rockchip_cpuclk_hw(hw);
    let reg_data = (*cpuclk).reg_data;
    let mut clksel0 = readl_relaxed((*cpuclk).reg_base.add((*reg_data).core_reg[0] as usize));
    clksel0 >>= (*reg_data).div_core_shift[0];
    clksel0 &= (*reg_data).div_core_mask[0];
    parent_rate / (clksel0 as u64 + 1)
}

static rockchip_cpuclk_ops: clk_ops = clk_ops { recalc_rate: Some(rockchip_cpuclk_recalc_rate) };

unsafe fn rockchip_cpuclk_set_dividers(cpuclk: *mut rockchip_cpuclk, rate: *const rockchip_cpuclk_rate_table) {
    for i in 0..(*rate).divs.len() {
        let clksel = &(*rate).divs[i];
        if clksel.reg == 0 { continue; }
        pr_debug!("{}: setting reg 0x{:x} to 0x{:x}\n", "rockchip_cpuclk_set_dividers", clksel.reg, clksel.val);
        writel(clksel.val, (*cpuclk).reg_base.add(clksel.reg as usize));
    }
}

unsafe fn rockchip_cpuclk_set_pre_muxs(cpuclk: *mut rockchip_cpuclk, rate: *const rockchip_cpuclk_rate_table) {
    for clksel in (*rate).pre_muxs.iter() {
        if clksel.reg == 0 { break; }
        pr_debug!("{}: setting reg 0x{:x} to 0x{:x}\n", "rockchip_cpuclk_set_pre_muxs", clksel.reg, clksel.val);
        writel(clksel.val, (*cpuclk).reg_base.add(clksel.reg as usize));
    }
}

unsafe fn rockchip_cpuclk_set_post_muxs(cpuclk: *mut rockchip_cpuclk, rate: *const rockchip_cpuclk_rate_table) {
    for clksel in (*rate).post_muxs.iter() {
        if clksel.reg == 0 { break; }
        pr_debug!("{}: setting reg 0x{:x} to 0x{:x}\n", "rockchip_cpuclk_set_post_muxs", clksel.reg, clksel.val);
        writel(clksel.val, (*cpuclk).reg_base.add(clksel.reg as usize));
    }
}

unsafe fn rockchip_cpuclk_pre_rate_change(cpuclk: *mut rockchip_cpuclk, ndata: *mut clk_notifier_data) -> i32 {
    let rd = (*cpuclk).reg_data;
    let rate = rockchip_get_cpuclk_settings(cpuclk, (*ndata).new_rate);
    if rate.is_null() { pr_err!("Invalid rate : {} for cpuclk\n", (*ndata).new_rate); return -22; }
    let alt_prate = clk_get_rate((*cpuclk).alt_parent);
    let mut flags = 0u64;
    spin_lock_irqsave((*cpuclk).lock, &mut flags);
    if alt_prate > (*ndata).old_rate {
        let mut alt_div = (alt_prate + (*ndata).old_rate - 1) / (*ndata).old_rate - 1;
        if alt_div > (*rd).div_core_mask[0] as u64 { alt_div = (*rd).div_core_mask[0] as u64; }
        for i in 0..(*rd).num_cores as usize { writel(hiword_update(alt_div as u32, (*rd).div_core_mask[i], (*rd).div_core_shift[i]), (*cpuclk).reg_base.add((*rd).core_reg[i] as usize)); }
    }
    rockchip_cpuclk_set_pre_muxs(cpuclk, rate);
    let reg = if (*rd).mux_core_reg != 0 { (*rd).mux_core_reg } else { (*rd).core_reg[0] };
    writel(hiword_update((*rd).mux_core_alt, (*rd).mux_core_mask, (*rd).mux_core_shift), (*cpuclk).reg_base.add(reg as usize));
    spin_unlock_irqrestore((*cpuclk).lock, flags); 0
}

unsafe fn rockchip_cpuclk_post_rate_change(cpuclk: *mut rockchip_cpuclk, ndata: *mut clk_notifier_data) -> i32 {
    let rd = (*cpuclk).reg_data;
    let rate = rockchip_get_cpuclk_settings(cpuclk, (*ndata).new_rate);
    if rate.is_null() { return -22; }
    let mut flags = 0u64; spin_lock_irqsave((*cpuclk).lock, &mut flags);
    if (*ndata).old_rate < (*ndata).new_rate { rockchip_cpuclk_set_dividers(cpuclk, rate); }
    let reg = if (*rd).mux_core_reg != 0 { (*rd).mux_core_reg } else { (*rd).core_reg[0] };
    writel(hiword_update((*rd).mux_core_main, (*rd).mux_core_mask, (*rd).mux_core_shift), (*cpuclk).reg_base.add(reg as usize));
    rockchip_cpuclk_set_post_muxs(cpuclk, rate);
    for i in 0..(*rd).num_cores as usize { writel(hiword_update(0, (*rd).div_core_mask[i], (*rd).div_core_shift[i]), (*cpuclk).reg_base.add((*rd).core_reg[i] as usize)); }
    if (*ndata).old_rate > (*ndata).new_rate { rockchip_cpuclk_set_dividers(cpuclk, rate); }
    spin_unlock_irqrestore((*cpuclk).lock, flags); 0
}

unsafe fn rockchip_cpuclk_notifier_cb(nb: *mut notifier_block, event: u64, data: *mut core::ffi::c_void) -> i32 {
    let cpuclk = to_rockchip_cpuclk_nb(nb); let ndata = data as *mut clk_notifier_data;
    let ret = if event == PRE_RATE_CHANGE { rockchip_cpuclk_pre_rate_change(cpuclk, ndata) } else if event == POST_RATE_CHANGE { rockchip_cpuclk_post_rate_change(cpuclk, ndata) } else { 0 };
    notifier_from_errno(ret)
}

// The registration entry points below retain the kernel API and cleanup labels through explicit Rust control flow.
pub unsafe fn rockchip_clk_register_cpuclk(name: *const i8, parent_names: *const *const i8, num_parents: u8, reg_data: *const rockchip_cpuclk_reg_data, rates: *const rockchip_cpuclk_rate_table, nrates: i32, reg_base: *mut core::ffi::c_void, lock: *mut spinlock_t) -> *mut clk {
    if num_parents < 2 { return err_ptr(-22); }
    let cpuclk = kzalloc_cpuclk(); if cpuclk.is_null() { return err_ptr(-12); }
    (*cpuclk).reg_base = reg_base; (*cpuclk).lock = lock; (*cpuclk).reg_data = reg_data;
    (*cpuclk).clk_nb.notifier_call = Some(rockchip_cpuclk_notifier_cb);
    (*cpuclk).rate_count = nrates as u32;
    if nrates > 0 { (*cpuclk).rate_table = kmemdup_array(rates, nrates as usize, core::mem::size_of::<rockchip_cpuclk_rate_table>()); }
    clk_register_cpuclk_internal(name, parent_names, num_parents, cpuclk)
}

unsafe fn rockchip_cpuclk_multi_pll_pre_rate_change(cpuclk: *mut rockchip_cpuclk, ndata: *mut clk_notifier_data) -> i32 {
    let new_rate = ((*ndata).new_rate + 999) / 1000 * 1000;
    let rate = rockchip_get_cpuclk_settings(cpuclk, new_rate);
    if rate.is_null() { return -22; }
    if new_rate > (*ndata).old_rate {
        let mut flags = 0u64; spin_lock_irqsave((*cpuclk).lock, &mut flags);
        rockchip_cpuclk_set_dividers(cpuclk, rate); spin_unlock_irqrestore((*cpuclk).lock, flags);
    } 0
}

unsafe fn rockchip_cpuclk_multi_pll_post_rate_change(cpuclk: *mut rockchip_cpuclk, ndata: *mut clk_notifier_data) -> i32 {
    let new_rate = ((*ndata).new_rate + 999) / 1000 * 1000;
    let rate = rockchip_get_cpuclk_settings(cpuclk, new_rate);
    if rate.is_null() { return -22; }
    if new_rate < (*ndata).old_rate {
        let mut flags = 0u64; spin_lock_irqsave((*cpuclk).lock, &mut flags);
        rockchip_cpuclk_set_dividers(cpuclk, rate); spin_unlock_irqrestore((*cpuclk).lock, flags);
    } 0
}

unsafe fn rockchip_cpuclk_multi_pll_notifier_cb(nb: *mut notifier_block, event: u64, data: *mut core::ffi::c_void) -> i32 {
    let cpuclk = to_rockchip_cpuclk_nb(nb); let ndata = data as *mut clk_notifier_data;
    let ret = if event == PRE_RATE_CHANGE { rockchip_cpuclk_multi_pll_pre_rate_change(cpuclk, ndata) } else if event == POST_RATE_CHANGE { rockchip_cpuclk_multi_pll_post_rate_change(cpuclk, ndata) } else { 0 };
    notifier_from_errno(ret)
}

pub unsafe fn rockchip_clk_register_cpuclk_multi_pll(name: *const i8, parent_names: *const *const i8, num_parents: u8, base: *mut core::ffi::c_void, muxdiv_offset: i32, mux_shift: u8, mux_width: u8, mux_flags: u8, div_offset: i32, div_shift: u8, div_width: u8, div_flags: u8, flags: u32, lock: *mut spinlock_t, rates: *const rockchip_cpuclk_rate_table, nrates: i32) -> *mut clk {
    let mux = if num_parents > 1 { alloc_clk_mux(base, muxdiv_offset, mux_shift, mux_width, mux_flags, lock) } else { core::ptr::null_mut() };
    let div = if div_width > 0 { alloc_clk_divider(base, if div_offset != 0 { div_offset } else { muxdiv_offset }, div_shift, div_width, div_flags, lock) } else { core::ptr::null_mut() };
    let hw = clk_hw_register_composite(name, parent_names, num_parents, mux, div, flags);
    if is_err(hw) { free_clk_divider(div); free_clk_mux(mux); return hw as *mut clk; }
    let cpuclk = kzalloc_cpuclk();
    if cpuclk.is_null() { clk_hw_unregister_composite(hw); free_clk_divider(div); free_clk_mux(mux); return err_ptr(-12); }
    (*cpuclk).reg_base = base; (*cpuclk).lock = lock; (*cpuclk).clk_nb.notifier_call = Some(rockchip_cpuclk_multi_pll_notifier_cb);
    let ret = clk_notifier_register((*hw).clk, &mut (*cpuclk).clk_nb);
    if ret != 0 { kfree_cpuclk(cpuclk); clk_hw_unregister_composite(hw); free_clk_divider(div); free_clk_mux(mux); return err_ptr(ret); }
    if nrates > 0 { (*cpuclk).rate_count = nrates as u32; (*cpuclk).rate_table = kmemdup(rates, core::mem::size_of::<rockchip_cpuclk_rate_table>() * nrates as usize); }
    (*hw).clk
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
