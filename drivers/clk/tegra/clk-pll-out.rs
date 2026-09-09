// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/kernel.h, linux/io.h, linux/err.h, linux/delay.h, linux/slab.h,
// linux/clk-provider.h, and clk.h.

#[inline]
unsafe fn pll_out_enb(p: *const tegra_clk_pll_out) -> u32 {
    BIT((*p).enb_bit_idx)
}

#[inline]
unsafe fn pll_out_rst(p: *const tegra_clk_pll_out) -> u32 {
    BIT((*p).rst_bit_idx)
}

unsafe fn clk_pll_out_is_enabled(hw: *mut clk_hw) -> i32 {
    let pll_out: *mut tegra_clk_pll_out = to_clk_pll_out(hw);
    let val: u32 = readl_relaxed((*pll_out).reg);
    let mut state: i32;

    state = if (val & pll_out_enb(pll_out)) != 0 { 1 } else { 0 };
    if (val & pll_out_rst(pll_out)) == 0 {
        state = 0;
    }
    state
}

unsafe fn clk_pll_out_enable(hw: *mut clk_hw) -> i32 {
    let pll_out: *mut tegra_clk_pll_out = to_clk_pll_out(hw);
    let mut flags: c_ulong = 0;
    let mut val: u32;

    if !(*pll_out).lock.is_null() {
        spin_lock_irqsave((*pll_out).lock, &mut flags);
    }

    val = readl_relaxed((*pll_out).reg);
    val |= pll_out_enb(pll_out) | pll_out_rst(pll_out);

    writel_relaxed(val, (*pll_out).reg);
    udelay(2);

    if !(*pll_out).lock.is_null() {
        spin_unlock_irqrestore((*pll_out).lock, flags);
    }

    0
}

unsafe fn clk_pll_out_disable(hw: *mut clk_hw) {
    let pll_out: *mut tegra_clk_pll_out = to_clk_pll_out(hw);
    let mut flags: c_ulong = 0;
    let mut val: u32;

    if !(*pll_out).lock.is_null() {
        spin_lock_irqsave((*pll_out).lock, &mut flags);
    }

    val = readl_relaxed((*pll_out).reg);
    val &= !(pll_out_enb(pll_out) | pll_out_rst(pll_out));

    writel_relaxed(val, (*pll_out).reg);
    udelay(2);

    if !(*pll_out).lock.is_null() {
        spin_unlock_irqrestore((*pll_out).lock, flags);
    }
}

unsafe fn tegra_clk_pll_out_restore_context(hw: *mut clk_hw) {
    if __clk_get_enable_count((*hw).clk) == 0 {
        clk_pll_out_disable(hw);
    } else {
        clk_pll_out_enable(hw);
    }
}

pub static tegra_clk_pll_out_ops: clk_ops = clk_ops {
    is_enabled: Some(clk_pll_out_is_enabled),
    enable: Some(clk_pll_out_enable),
    disable: Some(clk_pll_out_disable),
    restore_context: Some(tegra_clk_pll_out_restore_context),
};

pub unsafe fn tegra_clk_register_pll_out(
    name: *const c_char,
    parent_name: *const c_char,
    reg: *mut c_void,
    enb_bit_idx: u8,
    rst_bit_idx: u8,
    flags: c_ulong,
    pll_out_flags: u8,
    lock: *mut spinlock_t,
) -> *mut clk {
    let pll_out: *mut tegra_clk_pll_out = kzalloc_obj::<tegra_clk_pll_out>();
    let clk: *mut clk;
    let mut init: clk_init_data;

    if pll_out.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    init.name = name;
    init.ops = &tegra_clk_pll_out_ops;
    init.parent_names = if !parent_name.is_null() {
        &parent_name
    } else {
        core::ptr::null()
    };
    init.num_parents = if !parent_name.is_null() { 1 } else { 0 };
    init.flags = flags;

    (*pll_out).reg = reg;
    (*pll_out).enb_bit_idx = enb_bit_idx;
    (*pll_out).rst_bit_idx = rst_bit_idx;
    (*pll_out).flags = pll_out_flags;
    (*pll_out).lock = lock;

    /* Data in .init is copied by clk_register(), so stack variable OK */
    (*pll_out).hw.init = &mut init;

    clk = clk_register(core::ptr::null_mut(), &mut (*pll_out).hw);
    if IS_ERR(clk) {
        kfree(pll_out);
    }

    clk
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
