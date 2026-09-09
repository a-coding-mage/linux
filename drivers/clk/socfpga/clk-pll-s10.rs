// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2017, Intel Corporation
 */

// Linux dependencies: slab, clk-provider, io, stratix10-clk.h, and clk.h.

const CLK_MGR_PLL_CLK_SRC_SHIFT: u32 = 16;
const CLK_MGR_PLL_CLK_SRC_MASK: u32 = 0x3;

const SOCFPGA_PLL_POWER: u32 = 0;
const SOCFPGA_PLL_RESET_MASK: u32 = 0x2;
const SOCFPGA_PLL_REFDIV_MASK: u32 = 0x00003f00;
const SOCFPGA_PLL_REFDIV_SHIFT: u32 = 8;
const SOCFPGA_PLL_AREFDIV_MASK: u32 = 0x00000f00;
const SOCFPGA_PLL_DREFDIV_MASK: u32 = 0x00003000;
const SOCFPGA_PLL_DREFDIV_SHIFT: u32 = 12;
const SOCFPGA_PLL_MDIV_MASK: u32 = 0xff000000;
const SOCFPGA_PLL_MDIV_SHIFT: u32 = 24;
const SOCFPGA_AGILEX_PLL_MDIV_MASK: u32 = 0x000003ff;
const SWCTRLBTCLKSEL_MASK: u32 = 0x200;
const SWCTRLBTCLKSEL_SHIFT: u32 = 9;

const SOCFPGA_N5X_PLLDIV_FDIV_MASK: u32 = 0x0001ff00;
const SOCFPGA_N5X_PLLDIV_FDIV_SHIFT: u32 = 8;
const SOCFPGA_N5X_PLLDIV_RDIV_MASK: u32 = 0x3f;
const SOCFPGA_N5X_PLLDIV_QDIV_MASK: u32 = 0x07000000;
const SOCFPGA_N5X_PLLDIV_QDIV_SHIFT: u32 = 24;
const SOCFPGA_BOOT_CLK: &str = "boot_clk";

unsafe fn to_socfpga_clk<'a>(p: *mut clk_hw) -> &'a mut socfpga_pll {
    // Equivalent to container_of(p, struct socfpga_pll, hw.hw).
    &mut *(p as *mut socfpga_pll)
}

unsafe fn n5x_clk_pll_recalc_rate(hwclk: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let socfpgaclk = to_socfpga_clk(hwclk);
    let mut reg: c_ulong;
    let mut fdiv: c_ulong;
    let mut rdiv: c_ulong;
    let mut qdiv: c_ulong;
    let mut power: u32 = 1;
    reg = readl(socfpgaclk.hw.reg.add(8));
    fdiv = ((reg as u32 & SOCFPGA_N5X_PLLDIV_FDIV_MASK) >> SOCFPGA_N5X_PLLDIV_FDIV_SHIFT) as c_ulong;
    rdiv = (reg as u32 & SOCFPGA_N5X_PLLDIV_RDIV_MASK) as c_ulong;
    qdiv = (((reg as u32 & SOCFPGA_N5X_PLLDIV_QDIV_MASK) >> SOCFPGA_N5X_PLLDIV_QDIV_SHIFT)) as c_ulong;
    while qdiv != 0 { power *= 2; qdiv -= 1; }
    (parent_rate * 2 * (fdiv + 1)) / ((rdiv + 1) * power as c_ulong)
}

unsafe fn agilex_clk_pll_recalc_rate(hwclk: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let socfpgaclk = to_socfpga_clk(hwclk);
    let reg = readl(socfpgaclk.hw.reg);
    let arefdiv = ((reg & SOCFPGA_PLL_AREFDIV_MASK) >> SOCFPGA_PLL_REFDIV_SHIFT) as c_ulong;
    let mut vco_freq = (parent_rate as c_ulonglong) / arefdiv as c_ulonglong;
    let reg = readl(socfpgaclk.hw.reg.add(0x24));
    let mdiv = (reg & SOCFPGA_AGILEX_PLL_MDIV_MASK) as c_ulonglong;
    vco_freq *= mdiv;
    vco_freq as c_ulong
}

unsafe fn clk_pll_recalc_rate(hwclk: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let socfpgaclk = to_socfpga_clk(hwclk);
    let reg = readl(socfpgaclk.hw.reg);
    let refdiv = ((reg & SOCFPGA_PLL_REFDIV_MASK) >> SOCFPGA_PLL_REFDIV_SHIFT) as c_ulonglong;
    let mut vco_freq = parent_rate as c_ulonglong / refdiv;
    let reg = readl(socfpgaclk.hw.reg.add(4));
    let mdiv = ((reg & SOCFPGA_PLL_MDIV_MASK) >> SOCFPGA_PLL_MDIV_SHIFT) as c_ulonglong;
    vco_freq *= mdiv + 6;
    vco_freq as c_ulong
}

unsafe fn clk_boot_clk_recalc_rate(hwclk: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let socfpgaclk = to_socfpga_clk(hwclk);
    let mut div = ((readl(socfpgaclk.hw.reg) & SWCTRLBTCLKSEL_MASK) >> SWCTRLBTCLKSEL_SHIFT) + 1;
    parent_rate / div as c_ulong
}

unsafe fn clk_pll_get_parent(hwclk: *mut clk_hw) -> u8 {
    let socfpgaclk = to_socfpga_clk(hwclk);
    ((readl(socfpgaclk.hw.reg) >> CLK_MGR_PLL_CLK_SRC_SHIFT) & CLK_MGR_PLL_CLK_SRC_MASK) as u8
}

unsafe fn clk_boot_get_parent(hwclk: *mut clk_hw) -> u8 {
    let socfpgaclk = to_socfpga_clk(hwclk);
    ((readl(socfpgaclk.hw.reg) >> SWCTRLBTCLKSEL_SHIFT) & SWCTRLBTCLKSEL_MASK) as u8
}

unsafe fn clk_pll_prepare(hwclk: *mut clk_hw) -> c_int {
    let socfpgaclk = to_socfpga_clk(hwclk);
    let mut reg = readl(socfpgaclk.hw.reg);
    reg |= SOCFPGA_PLL_RESET_MASK;
    writel(reg, socfpgaclk.hw.reg);
    0
}

unsafe fn n5x_clk_pll_prepare(hwclk: *mut clk_hw) -> c_int {
    let socfpgaclk = to_socfpga_clk(hwclk);
    let mut reg = readl(socfpgaclk.hw.reg.add(4));
    reg |= SOCFPGA_PLL_RESET_MASK;
    writel(reg, socfpgaclk.hw.reg.add(4));
    0
}

// The following ops tables and registration functions retain the C interfaces;
// their referenced Linux and platform types/functions are supplied externally.
static N5X_CLK_PLL_OPS: clk_ops = clk_ops { recalc_rate: Some(n5x_clk_pll_recalc_rate), get_parent: Some(clk_pll_get_parent), prepare: Some(n5x_clk_pll_prepare) };
static AGILEX_CLK_PLL_OPS: clk_ops = clk_ops { recalc_rate: Some(agilex_clk_pll_recalc_rate), get_parent: Some(clk_pll_get_parent), prepare: Some(clk_pll_prepare) };
static CLK_PLL_OPS: clk_ops = clk_ops { recalc_rate: Some(clk_pll_recalc_rate), get_parent: Some(clk_pll_get_parent), prepare: Some(clk_pll_prepare) };
static CLK_BOOT_OPS: clk_ops = clk_ops { recalc_rate: Some(clk_boot_clk_recalc_rate), get_parent: Some(clk_boot_get_parent), prepare: Some(clk_pll_prepare) };

unsafe fn s10_register_pll(clks: *const stratix10_pll_clock, reg: *mut u8) -> *mut clk_hw {
    let pll_clk = kzalloc_obj::<socfpga_pll>();
    if WARN_ON(pll_clk.is_null()) { return core::ptr::null_mut(); }
    (*pll_clk).hw.reg = reg.add((*clks).offset);
    let mut init: clk_init_data = core::mem::zeroed();
    init.ops = if streq((*clks).name, SOCFPGA_BOOT_CLK) { &CLK_BOOT_OPS } else { &CLK_PLL_OPS };
    init.name = (*clks).name;
    init.flags = (*clks).flags;
    init.num_parents = (*clks).num_parents;
    init.parent_names = core::ptr::null();
    init.parent_data = (*clks).parent_data;
    (*pll_clk).hw.hw.init = &init;
    (*pll_clk).hw.bit_idx = SOCFPGA_PLL_POWER;
    let hw_clk = &mut (*pll_clk).hw.hw as *mut clk_hw;
    let ret = clk_hw_register(core::ptr::null_mut(), hw_clk);
    if ret != 0 { kfree(pll_clk); return ERR_PTR(ret); }
    hw_clk
}

unsafe fn agilex_register_pll(clks: *const stratix10_pll_clock, reg: *mut u8) -> *mut clk_hw {
    let pll_clk = kzalloc_obj::<socfpga_pll>();
    if WARN_ON(pll_clk.is_null()) { return core::ptr::null_mut(); }
    (*pll_clk).hw.reg = reg.add((*clks).offset);
    let mut init: clk_init_data = core::mem::zeroed();
    init.ops = if streq((*clks).name, SOCFPGA_BOOT_CLK) { &CLK_BOOT_OPS } else { &AGILEX_CLK_PLL_OPS };
    init.name = (*clks).name; init.flags = (*clks).flags;
    init.num_parents = (*clks).num_parents; init.parent_names = core::ptr::null(); init.parent_data = (*clks).parent_data;
    (*pll_clk).hw.hw.init = &init; (*pll_clk).hw.bit_idx = SOCFPGA_PLL_POWER;
    let hw_clk = &mut (*pll_clk).hw.hw as *mut clk_hw;
    let ret = clk_hw_register(core::ptr::null_mut(), hw_clk);
    if ret != 0 { kfree(pll_clk); return ERR_PTR(ret); } hw_clk
}

unsafe fn n5x_register_pll(clks: *const stratix10_pll_clock, reg: *mut u8) -> *mut clk_hw {
    let pll_clk = kzalloc_obj::<socfpga_pll>();
    if WARN_ON(pll_clk.is_null()) { return core::ptr::null_mut(); }
    (*pll_clk).hw.reg = reg.add((*clks).offset);
    let mut init: clk_init_data = core::mem::zeroed();
    init.ops = if streq((*clks).name, SOCFPGA_BOOT_CLK) { &CLK_BOOT_OPS } else { &N5X_CLK_PLL_OPS };
    init.name = (*clks).name; init.flags = (*clks).flags; init.num_parents = (*clks).num_parents;
    init.parent_names = core::ptr::null(); init.parent_data = (*clks).parent_data;
    (*pll_clk).hw.hw.init = &init; (*pll_clk).hw.bit_idx = SOCFPGA_PLL_POWER;
    let hw_clk = &mut (*pll_clk).hw.hw as *mut clk_hw; let ret = clk_hw_register(core::ptr::null_mut(), hw_clk);
    if ret != 0 { kfree(pll_clk); return ERR_PTR(ret); } hw_clk
}

unsafe fn agilex5_register_pll(clks: *const agilex5_pll_clock, reg: *mut u8) -> *mut clk_hw {
    let pll_clk = kzalloc_obj::<socfpga_pll>();
    if WARN_ON(pll_clk.is_null()) { return core::ptr::null_mut(); }
    (*pll_clk).hw.reg = reg.add((*clks).offset);
    let mut init: clk_init_data = core::mem::zeroed();
    init.ops = if streq((*clks).name, SOCFPGA_BOOT_CLK) { &CLK_BOOT_OPS } else { &AGILEX_CLK_PLL_OPS };
    init.name = (*clks).name; init.flags = (*clks).flags; init.num_parents = (*clks).num_parents;
    init.parent_names = (*clks).parent_names; (*pll_clk).hw.hw.init = &init; (*pll_clk).hw.bit_idx = SOCFPGA_PLL_POWER;
    let hw_clk = &mut (*pll_clk).hw.hw as *mut clk_hw; let ret = clk_hw_register(core::ptr::null_mut(), hw_clk);
    if ret != 0 { kfree(pll_clk); return ERR_PTR(ret); } hw_clk
}

// c_ulong, c_ulonglong, c_int, clk_hw, socfpga_pll, clk_ops, stratix10_pll_clock,
// agilex5_pll_clock, readl, writel, kzalloc_obj, WARN_ON, streq, clk_hw_register,
// kfree, and ERR_PTR are external dependencies from the original headers/kernel.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
