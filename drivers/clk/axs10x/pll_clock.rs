// SPDX-License-Identifier: GPL-2.0-only
/*
 * Synopsys AXS10X SDP Generic PLL clock driver
 *
 * Copyright (C) 2017 Synopsys
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

const PLL_REG_IDIV: u32 = 0x0;
const PLL_REG_FBDIV: u32 = 0x4;
const PLL_REG_ODIV: u32 = 0x8;

const PLL_LOCK: u32 = 1 << 0;
const PLL_ERROR: u32 = 1 << 1;
const PLL_MAX_LOCK_TIME: u32 = 100; // 100 us

#[repr(C)]
struct Axs10xPllCfg {
    rate: u32,
    idiv: u32,
    fbdiv: u32,
    odiv: u32,
}

static ARC_PLL_CFG: [Axs10xPllCfg; 7] = [
    Axs10xPllCfg { rate: 33333333, idiv: 1, fbdiv: 1, odiv: 1 },
    Axs10xPllCfg { rate: 50000000, idiv: 1, fbdiv: 30, odiv: 20 },
    Axs10xPllCfg { rate: 75000000, idiv: 2, fbdiv: 45, odiv: 10 },
    Axs10xPllCfg { rate: 90000000, idiv: 2, fbdiv: 54, odiv: 10 },
    Axs10xPllCfg { rate: 100000000, idiv: 1, fbdiv: 30, odiv: 10 },
    Axs10xPllCfg { rate: 125000000, idiv: 2, fbdiv: 45, odiv: 6 },
    Axs10xPllCfg { rate: 0, idiv: 0, fbdiv: 0, odiv: 0 },
];

static PGU_PLL_CFG: [Axs10xPllCfg; 4] = [
    Axs10xPllCfg { rate: 25200000, idiv: 1, fbdiv: 84, odiv: 90 },
    Axs10xPllCfg { rate: 50000000, idiv: 1, fbdiv: 100, odiv: 54 },
    Axs10xPllCfg { rate: 74250000, idiv: 1, fbdiv: 44, odiv: 16 },
    Axs10xPllCfg { rate: 0, idiv: 0, fbdiv: 0, odiv: 0 },
];

#[repr(C)]
struct Axs10xPllClk {
    hw: ClkHw,
    base: *mut core::ffi::c_void,
    lock: *mut core::ffi::c_void,
    pll_cfg: *const Axs10xPllCfg,
    dev: *mut Device,
}

unsafe fn axs10x_pll_write(clk: *mut Axs10xPllClk, reg: u32, val: u32) {
    iowrite32(val, (*clk).base.add(reg as usize));
}

unsafe fn axs10x_pll_read(clk: *mut Axs10xPllClk, reg: u32) -> u32 {
    ioread32((*clk).base.add(reg as usize))
}

unsafe fn to_axs10x_pll_clk(hw: *mut ClkHw) -> *mut Axs10xPllClk {
    container_of!(hw, Axs10xPllClk, hw)
}

#[inline]
fn axs10x_div_get_value(reg: u32) -> u32 {
    if reg & (1 << 13) != 0 { 1 } else { ((reg & (0x3f << 6)) >> 6) + (reg & 0x3f) }
}

#[inline]
fn axs10x_encode_div(id: u32, upd: i32) -> u32 {
    let mut div = 0u32;
    div |= (((if id % 2 == 0 { id >> 1 } else { (id >> 1) + 1 }) & 0x3f) << 0);
    div |= ((id >> 1) & 0x3f) << 6;
    div |= (id % 2 & 0x01) << 12;
    div |= ((if id == 1 { 1 } else { 0 }) & 0x01) << 13;
    div |= ((if upd == 0 { 1 } else { 0 }) & 0x01) << 14;
    div
}

unsafe fn axs10x_pll_recalc_rate(hw: *mut ClkHw, parent_rate: u64) -> u64 {
    let clk = to_axs10x_pll_clk(hw);
    let idiv = axs10x_div_get_value(axs10x_pll_read(clk, PLL_REG_IDIV));
    let fbdiv = axs10x_div_get_value(axs10x_pll_read(clk, PLL_REG_FBDIV));
    let odiv = axs10x_div_get_value(axs10x_pll_read(clk, PLL_REG_ODIV));
    parent_rate.wrapping_mul(fbdiv as u64) / (idiv.wrapping_mul(odiv) as u64)
}

unsafe fn axs10x_pll_determine_rate(hw: *mut ClkHw, req: *mut ClkRateRequest) -> i32 {
    let clk = to_axs10x_pll_clk(hw);
    let cfg = (*clk).pll_cfg;
    if (*cfg).rate == 0 { return -22; }
    let mut best_rate = (*cfg).rate as i64;
    let mut i = 1usize;
    while (*cfg.add(i)).rate != 0 {
        let rate = (*cfg.add(i)).rate as i64;
        if ((*req).rate as i64 - rate).abs() < ((*req).rate as i64 - best_rate).abs() { best_rate = rate; }
        i += 1;
    }
    (*req).rate = best_rate as u64;
    0
}

unsafe fn axs10x_pll_set_rate(hw: *mut ClkHw, rate: u64, parent_rate: u64) -> i32 {
    let clk = to_axs10x_pll_clk(hw);
    let cfg = (*clk).pll_cfg;
    let mut i = 0usize;
    while (*cfg.add(i)).rate != 0 {
        if (*cfg.add(i)).rate as u64 == rate {
            axs10x_pll_write(clk, PLL_REG_IDIV, axs10x_encode_div((*cfg.add(i)).idiv, 0));
            axs10x_pll_write(clk, PLL_REG_FBDIV, axs10x_encode_div((*cfg.add(i)).fbdiv, 0));
            axs10x_pll_write(clk, PLL_REG_ODIV, axs10x_encode_div((*cfg.add(i)).odiv, 1));
            // Wait until CGU relocks and check error status.
            udelay(PLL_MAX_LOCK_TIME);
            if ioread32((*clk).lock) & PLL_LOCK == 0 { return -110; }
            if ioread32((*clk).lock) & PLL_ERROR != 0 { return -22; }
            return 0;
        }
        i += 1;
    }
    dev_err((*clk).dev, "invalid rate=%ld, parent_rate=%ld\n", rate, parent_rate);
    -22
}

// External kernel types, functions, registration macros, and module metadata
// correspond directly to the declarations supplied by the Linux environment.
const AXS10X_PLL_CLOCK_COMPATIBLE: &str = "snps,axs10x-arc-pll-clock";
const AXS10X_PGU_PLL_CLOCK_COMPATIBLE: &str = "snps,axs10x-pgu-pll-clock";
const AXS10X_PLL_CLOCK_DRIVER_NAME: &str = "axs10x-pll-clock";
const MODULE_AUTHOR: &str = "Vlad Zakharov <vzakhar@synopsys.com>";
const MODULE_DESCRIPTION: &str = "Synopsys AXS10X SDP Generic PLL Clock Driver";
const MODULE_LICENSE: &str = "GPL v2";

unsafe fn axs10x_pll_clk_probe(pdev: *mut PlatformDevice) -> i32 {
    let dev = &mut (*pdev).dev;
    let pll_clk = devm_kzalloc(dev, core::mem::size_of::<Axs10xPllClk>(), GFP_KERNEL)
        as *mut Axs10xPllClk;
    if pll_clk.is_null() { return -12; }
    (*pll_clk).base = devm_platform_ioremap_resource(pdev, 0);
    if is_err((*pll_clk).base) { return ptr_err((*pll_clk).base); }
    (*pll_clk).lock = devm_platform_ioremap_resource(pdev, 1);
    if is_err((*pll_clk).lock) { return ptr_err((*pll_clk).lock); }
    let parent_name = of_clk_get_parent_name((*dev).of_node, 0);
    let mut init = ClkInitData::default();
    init.name = (*dev).of_node.name;
    init.ops = &AXS10X_PLL_OPS;
    init.parent_names = &parent_name;
    init.num_parents = 1;
    (*pll_clk).hw.init = &init;
    (*pll_clk).dev = dev;
    (*pll_clk).pll_cfg = of_device_get_match_data(dev);
    if (*pll_clk).pll_cfg.is_null() { dev_err(dev, "No OF match data provided\n"); return -22; }
    let ret = devm_clk_hw_register(dev, &mut (*pll_clk).hw);
    if ret != 0 { dev_err(dev, "failed to register %s clock\n", init.name); return ret; }
    devm_of_clk_add_hw_provider(dev, of_clk_hw_simple_get, &mut (*pll_clk).hw)
}

unsafe fn of_axs10x_pll_clk_setup(node: *mut DeviceNode) {
    let pll_clk = kzalloc(core::mem::size_of::<Axs10xPllClk>()) as *mut Axs10xPllClk;
    if pll_clk.is_null() { return; }
    (*pll_clk).base = of_iomap(node, 0);
    if (*pll_clk).base.is_null() { pr_err("failed to map pll div registers\n"); goto_free!(pll_clk); return; }
    (*pll_clk).lock = of_iomap(node, 1);
    if (*pll_clk).lock.is_null() { pr_err("failed to map pll lock register\n"); iounmap((*pll_clk).base); goto_free!(pll_clk); return; }
    let parent_name = of_clk_get_parent_name(node, 0);
    let mut init = ClkInitData::default();
    init.name = (*node).name;
    init.ops = &AXS10X_PLL_OPS;
    init.parent_names = &parent_name;
    init.num_parents = if parent_name.is_null() { 0 } else { 1 };
    (*pll_clk).hw.init = &init;
    (*pll_clk).pll_cfg = ARC_PLL_CFG.as_ptr();
    let ret = clk_hw_register(core::ptr::null_mut(), &mut (*pll_clk).hw);
    if ret != 0 { pr_err("failed to register %pOFn clock\n", node); iounmap((*pll_clk).lock); iounmap((*pll_clk).base); goto_free!(pll_clk); return; }
    let ret = of_clk_add_hw_provider(node, of_clk_hw_simple_get, &mut (*pll_clk).hw);
    if ret != 0 { pr_err("failed to add hw provider for %pOFn clock\n", node); clk_hw_unregister(&mut (*pll_clk).hw); iounmap((*pll_clk).lock); iounmap((*pll_clk).base); goto_free!(pll_clk); }
}

// Equivalent registration declarations:
// CLK_OF_DECLARE(axs10x_pll_clock, "snps,axs10x-arc-pll-clock", of_axs10x_pll_clk_setup);
// MODULE_DEVICE_TABLE(of, axs10x_pll_clk_id); builtin_platform_driver(axs10x_pll_clk_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
