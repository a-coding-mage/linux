// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2015, NVIDIA CORPORATION.  All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/clk-provider.h, linux/io.h, and clk.h.

unsafe fn to_tegra_clk_periph_fixed(
    hw: *mut clk_hw,
) -> *mut tegra_clk_periph_fixed {
    container_of!(hw, tegra_clk_periph_fixed, hw)
}

unsafe fn tegra_clk_periph_fixed_is_enabled(hw: *mut clk_hw) -> i32 {
    let fixed = to_tegra_clk_periph_fixed(hw);
    let mask: u32 = 1u32 << ((*fixed).num % 32);
    let mut value: u32;

    value = readl((*fixed).base.add((*fixed).regs.enb_reg as usize));
    if value & mask != 0 {
        value = readl((*fixed).base.add((*fixed).regs.rst_reg as usize));
        if value & mask == 0 {
            return 1;
        }
    }

    0
}

unsafe fn tegra_clk_periph_fixed_enable(hw: *mut clk_hw) -> i32 {
    let fixed = to_tegra_clk_periph_fixed(hw);
    let mask: u32 = 1u32 << ((*fixed).num % 32);

    writel(mask, (*fixed).base.add((*fixed).regs.enb_set_reg as usize));

    0
}

unsafe fn tegra_clk_periph_fixed_disable(hw: *mut clk_hw) {
    let fixed = to_tegra_clk_periph_fixed(hw);
    let mask: u32 = 1u32 << ((*fixed).num % 32);

    writel(mask, (*fixed).base.add((*fixed).regs.enb_clr_reg as usize));
}

unsafe fn tegra_clk_periph_fixed_recalc_rate(
    hw: *mut clk_hw,
    parent_rate: c_ulong,
) -> c_ulong {
    let fixed = to_tegra_clk_periph_fixed(hw);
    let mut rate: u64 = parent_rate as u64 * (*fixed).mul as u64;

    rate /= (*fixed).div as u64;

    rate as c_ulong
}

static tegra_clk_periph_fixed_ops: clk_ops = clk_ops {
    is_enabled: Some(tegra_clk_periph_fixed_is_enabled),
    enable: Some(tegra_clk_periph_fixed_enable),
    disable: Some(tegra_clk_periph_fixed_disable),
    recalc_rate: Some(tegra_clk_periph_fixed_recalc_rate),
};

unsafe fn tegra_clk_register_periph_fixed(
    name: *const c_char,
    parent: *const c_char,
    flags: c_ulong,
    base: *mut c_void,
    mul: c_uint,
    div: c_uint,
    num: c_uint,
) -> *mut clk {
    let regs: *const tegra_clk_periph_regs;
    let fixed: *mut tegra_clk_periph_fixed;
    let mut init: clk_init_data;
    let clk: *mut clk;

    regs = get_reg_bank(num);
    if regs.is_null() {
        return ERR_PTR(-EINVAL);
    }

    fixed = kzalloc_obj!(*fixed);
    if fixed.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    init.name = name;
    init.flags = flags;
    init.parent_names = if !parent.is_null() { &parent } else { core::ptr::null() };
    init.num_parents = if !parent.is_null() { 1 } else { 0 };
    init.ops = &tegra_clk_periph_fixed_ops;

    (*fixed).base = base;
    (*fixed).regs = regs;
    (*fixed).mul = mul;
    (*fixed).div = div;
    (*fixed).num = num;

    (*fixed).hw.init = &init;

    clk = clk_register(core::ptr::null_mut(), &mut (*fixed).hw);
    if IS_ERR(clk) {
        kfree(fixed);
    }

    clk
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
