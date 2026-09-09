// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2011-2012 Calxeda, Inc.
 */

// Kernel dependencies supplied externally.

const HB_PLL_LOCK_500: u32 = 0x20000000;
const HB_PLL_LOCK: u32 = 0x10000000;
const HB_PLL_DIVF_SHIFT: u32 = 20;
const HB_PLL_DIVF_MASK: u32 = 0x0ff00000;
const HB_PLL_DIVQ_SHIFT: u32 = 16;
const HB_PLL_DIVQ_MASK: u32 = 0x00070000;
const HB_PLL_DIVR_SHIFT: u32 = 8;
const HB_PLL_DIVR_MASK: u32 = 0x00001f00;
const HB_PLL_RANGE_SHIFT: u32 = 4;
const HB_PLL_RANGE_MASK: u32 = 0x00000070;
const HB_PLL_BYPASS: u32 = 0x00000008;
const HB_PLL_RESET: u32 = 0x00000004;
const HB_PLL_EXT_BYPASS: u32 = 0x00000002;
const HB_PLL_EXT_ENA: u32 = 0x00000001;

const HB_PLL_VCO_MIN_FREQ: u64 = 2133000000;
const HB_PLL_MAX_FREQ: u64 = HB_PLL_VCO_MIN_FREQ;
const HB_PLL_MIN_FREQ: u64 = HB_PLL_VCO_MIN_FREQ / 64;

const HB_A9_BCLK_DIV_MASK: u32 = 0x00000006;
const HB_A9_BCLK_DIV_SHIFT: u32 = 1;
const HB_A9_PCLK_DIV: u32 = 0x00000001;

#[repr(C)]
struct hb_clk {
    hw: clk_hw,
    reg: *mut core::ffi::c_void,
}

unsafe fn to_hb_clk(p: *mut clk_hw) -> *mut hb_clk {
    (p as *mut u8).sub(core::mem::offset_of!(hb_clk, hw)) as *mut hb_clk
}

unsafe fn clk_pll_prepare(hwclk: *mut clk_hw) -> i32 {
    let hbclk = &mut *to_hb_clk(hwclk);
    let mut reg: u32 = readl(hbclk.reg);
    reg &= !HB_PLL_RESET;
    writel(reg, hbclk.reg);
    while (readl(hbclk.reg) & HB_PLL_LOCK) == 0 {}
    while (readl(hbclk.reg) & HB_PLL_LOCK_500) == 0 {}
    0
}

unsafe fn clk_pll_unprepare(hwclk: *mut clk_hw) {
    let hbclk = &mut *to_hb_clk(hwclk);
    let mut reg = readl(hbclk.reg);
    reg |= HB_PLL_RESET;
    writel(reg, hbclk.reg);
}

unsafe fn clk_pll_enable(hwclk: *mut clk_hw) -> i32 {
    let hbclk = &mut *to_hb_clk(hwclk);
    let mut reg = readl(hbclk.reg);
    reg |= HB_PLL_EXT_ENA;
    writel(reg, hbclk.reg);
    0
}

unsafe fn clk_pll_disable(hwclk: *mut clk_hw) {
    let hbclk = &mut *to_hb_clk(hwclk);
    let mut reg = readl(hbclk.reg);
    reg &= !HB_PLL_EXT_ENA;
    writel(reg, hbclk.reg);
}

unsafe fn clk_pll_recalc_rate(hwclk: *mut clk_hw, parent_rate: u64) -> u64 {
    let hbclk = &mut *to_hb_clk(hwclk);
    let reg = readl(hbclk.reg) as u64;
    if (reg as u32 & HB_PLL_EXT_BYPASS) != 0 { return parent_rate; }
    let divf = ((reg as u32 & HB_PLL_DIVF_MASK) >> HB_PLL_DIVF_SHIFT) as u64;
    let divq = ((reg as u32 & HB_PLL_DIVQ_MASK) >> HB_PLL_DIVQ_SHIFT) as u32;
    let vco_freq = parent_rate * (divf + 1);
    vco_freq / (1u64 << divq)
}

unsafe fn clk_pll_calc(mut rate: u64, ref_freq: u64, pdivq: *mut u32, pdivf: *mut u32) {
    if rate < HB_PLL_MIN_FREQ { rate = HB_PLL_MIN_FREQ; }
    if rate > HB_PLL_MAX_FREQ { rate = HB_PLL_MAX_FREQ; }
    let mut divq: u32 = 1;
    while divq <= 6 {
        if rate * (1u64 << divq) >= HB_PLL_VCO_MIN_FREQ { break; }
        divq += 1;
    }
    let vco_freq = rate * (1u64 << divq);
    let divf = (vco_freq + ref_freq / 2) / ref_freq - 1;
    *pdivq = divq;
    *pdivf = divf as u32;
}

unsafe fn clk_pll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let mut divq = 0u32;
    let mut divf = 0u32;
    let ref_freq = (*req).best_parent_rate;
    clk_pll_calc((*req).rate, ref_freq, &mut divq, &mut divf);
    (*req).rate = (ref_freq * (divf as u64 + 1)) / (1u64 << divq);
    0
}

unsafe fn clk_pll_set_rate(hwclk: *mut clk_hw, rate: u64, parent_rate: u64) -> i32 {
    let hbclk = &mut *to_hb_clk(hwclk);
    let mut divq = 0u32; let mut divf = 0u32;
    clk_pll_calc(rate, parent_rate, &mut divq, &mut divf);
    let mut reg = readl(hbclk.reg);
    if divf != ((reg & HB_PLL_DIVF_MASK) >> HB_PLL_DIVF_SHIFT) {
        reg |= HB_PLL_EXT_BYPASS;
        writel(reg | HB_PLL_EXT_BYPASS, hbclk.reg);
        writel(reg | HB_PLL_RESET, hbclk.reg);
        reg &= !(HB_PLL_DIVF_MASK | HB_PLL_DIVQ_MASK);
        reg |= (divf << HB_PLL_DIVF_SHIFT) | (divq << HB_PLL_DIVQ_SHIFT);
        writel(reg | HB_PLL_RESET, hbclk.reg);
        writel(reg, hbclk.reg);
        while (readl(hbclk.reg) & HB_PLL_LOCK == 0) {}
        while (readl(hbclk.reg) & HB_PLL_LOCK_500 == 0) {}
        reg |= HB_PLL_EXT_ENA;
        reg &= !HB_PLL_EXT_BYPASS;
    } else {
        writel(reg | HB_PLL_EXT_BYPASS, hbclk.reg);
        reg &= !HB_PLL_DIVQ_MASK;
        reg |= divq << HB_PLL_DIVQ_SHIFT;
        writel(reg | HB_PLL_EXT_BYPASS, hbclk.reg);
    }
    writel(reg, hbclk.reg);
    0
}

static mut clk_pll_ops: clk_ops = clk_ops { prepare: Some(clk_pll_prepare), unprepare: Some(clk_pll_unprepare), enable: Some(clk_pll_enable), disable: Some(clk_pll_disable), recalc_rate: Some(clk_pll_recalc_rate), determine_rate: Some(clk_pll_determine_rate), set_rate: Some(clk_pll_set_rate) };

unsafe fn clk_cpu_periphclk_recalc_rate(hwclk: *mut clk_hw, parent_rate: u64) -> u64 {
    let hbclk = &mut *to_hb_clk(hwclk);
    parent_rate / if readl(hbclk.reg) & HB_A9_PCLK_DIV != 0 { 8 } else { 4 }
}
static mut a9periphclk_ops: clk_ops = clk_ops { recalc_rate: Some(clk_cpu_periphclk_recalc_rate), ..clk_ops::empty() };

unsafe fn clk_cpu_a9bclk_recalc_rate(hwclk: *mut clk_hw, parent_rate: u64) -> u64 {
    let hbclk = &mut *to_hb_clk(hwclk);
    let div = (readl(hbclk.reg) & HB_A9_BCLK_DIV_MASK) >> HB_A9_BCLK_DIV_SHIFT;
    parent_rate / (div as u64 + 2)
}
static mut a9bclk_ops: clk_ops = clk_ops { recalc_rate: Some(clk_cpu_a9bclk_recalc_rate), ..clk_ops::empty() };

unsafe fn clk_periclk_recalc_rate(hwclk: *mut clk_hw, parent_rate: u64) -> u64 {
    let hbclk = &mut *to_hb_clk(hwclk);
    let mut div = readl(hbclk.reg) & 0x1f;
    div += 1; div *= 2;
    parent_rate / div as u64
}

unsafe fn clk_periclk_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let mut div = (*req).best_parent_rate / (*req).rate;
    div += 1; div &= !0x1;
    (*req).rate = (*req).best_parent_rate / div;
    0
}

unsafe fn clk_periclk_set_rate(hwclk: *mut clk_hw, rate: u64, parent_rate: u64) -> i32 {
    let hbclk = &mut *to_hb_clk(hwclk);
    let div = parent_rate / rate;
    if div & 0x1 != 0 { return -22; }
    writel((div >> 1) as u32, hbclk.reg);
    0
}
static mut periclk_ops: clk_ops = clk_ops { recalc_rate: Some(clk_periclk_recalc_rate), determine_rate: Some(clk_periclk_determine_rate), set_rate: Some(clk_periclk_set_rate), ..clk_ops::empty() };

unsafe fn hb_clk_init(node: *mut device_node, ops: *const clk_ops, clkflags: u64) {
    let mut reg = 0u32;
    let mut hb_clk: *mut hb_clk;
    let mut clk_name = (*node).name;
    let mut parent_name: *const core::ffi::c_char;
    let mut init: clk_init_data;
    let srnp: *mut device_node;
    let rc = of_property_read_u32(node, b"reg\0".as_ptr() as _, &mut reg);
    if WARN_ON(rc != 0) { return; }
    hb_clk = kzalloc_obj::<hb_clk>();
    if WARN_ON(hb_clk.is_null()) { return; }
    srnp = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), b"calxeda,hb-sregs\0".as_ptr() as _);
    (*hb_clk).reg = of_iomap(srnp, 0);
    of_node_put(srnp);
    BUG_ON((*hb_clk).reg.is_null());
    (*hb_clk).reg = ((*hb_clk).reg as *mut u8).add(reg as usize) as _;
    of_property_read_string(node, b"clock-output-names\0".as_ptr() as _, &mut clk_name);
    init.name = clk_name; init.ops = ops; init.flags = clkflags;
    parent_name = of_clk_get_parent_name(node, 0);
    init.parent_names = &parent_name; init.num_parents = 1;
    (*hb_clk).hw.init = &mut init;
    let rc = clk_hw_register(core::ptr::null_mut(), &mut (*hb_clk).hw);
    if WARN_ON(rc != 0) { kfree(hb_clk as _); return; }
    of_clk_add_hw_provider(node, of_clk_hw_simple_get, &mut (*hb_clk).hw);
}

unsafe fn hb_pll_init(node: *mut device_node) { hb_clk_init(node, &clk_pll_ops, 0); }
CLK_OF_DECLARE!(hb_pll, "calxeda,hb-pll-clock", hb_pll_init);
unsafe fn hb_a9periph_init(node: *mut device_node) { hb_clk_init(node, &a9periphclk_ops, 0); }
CLK_OF_DECLARE!(hb_a9periph, "calxeda,hb-a9periph-clock", hb_a9periph_init);
unsafe fn hb_a9bus_init(node: *mut device_node) { hb_clk_init(node, &a9bclk_ops, CLK_IS_CRITICAL); }
CLK_OF_DECLARE!(hb_a9bus, "calxeda,hb-a9bus-clock", hb_a9bus_init);
unsafe fn hb_emmc_init(node: *mut device_node) { hb_clk_init(node, &periclk_ops, 0); }
CLK_OF_DECLARE!(hb_emmc, "calxeda,hb-emmc-clock", hb_emmc_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
