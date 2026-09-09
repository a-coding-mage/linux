// SPDX-License-Identifier: GPL-2.0
/*
 * Marvell MVEBU CPU clock handling.
 *
 * Copyright (C) 2012 Marvell
 *
 * Gregory CLEMENT <gregory.clement@free-electrons.com>
 */

// Linux kernel dependencies supplied by other translation units.

const SYS_CTRL_CLK_DIVIDER_CTRL_OFFSET: usize = 0x0;
const SYS_CTRL_CLK_DIVIDER_CTRL_RESET_ALL: u32 = 0xff;
const SYS_CTRL_CLK_DIVIDER_CTRL_RESET_SHIFT: u32 = 8;
const SYS_CTRL_CLK_DIVIDER_CTRL2_OFFSET: usize = 0x8;
const SYS_CTRL_CLK_DIVIDER_CTRL2_NBCLK_RATIO_SHIFT: u32 = 16;
const SYS_CTRL_CLK_DIVIDER_VALUE_OFFSET: usize = 0xC;
const SYS_CTRL_CLK_DIVIDER_MASK: u32 = 0x3F;

const PMU_DFS_RATIO_SHIFT: u32 = 16;
const PMU_DFS_RATIO_MASK: u32 = 0x3F;

const MAX_CPU: usize = 4;

#[repr(C)]
pub struct cpu_clk {
    pub hw: clk_hw,
    pub cpu: i32,
    pub clk_name: *const i8,
    pub parent_name: *const i8,
    pub reg_base: *mut core::ffi::c_void,
    pub pmu_dfs: *mut core::ffi::c_void,
}

static mut clks: *mut *mut clk = core::ptr::null_mut();
static mut clk_data: clk_onecell_data = clk_onecell_data {
    clk_num: 0,
    clks: core::ptr::null_mut(),
};

#[inline]
unsafe fn to_cpu_clk(p: *mut clk_hw) -> *mut cpu_clk {
    (p as *mut u8).sub(core::mem::offset_of!(cpu_clk, hw)) as *mut cpu_clk
}

unsafe fn clk_cpu_recalc_rate(hwclk: *mut clk_hw, parent_rate: usize) -> usize {
    let cpuclk = &*to_cpu_clk(hwclk);
    let reg = readl((cpuclk.reg_base as *mut u8).add(SYS_CTRL_CLK_DIVIDER_VALUE_OFFSET) as *const u32);
    let div = (reg >> ((cpuclk.cpu as u32) * 8)) & SYS_CTRL_CLK_DIVIDER_MASK;
    parent_rate / div as usize
}

unsafe fn clk_cpu_determine_rate(_hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    /* Valid ratio are 1:1, 1:2 and 1:3 */
    let mut div = (*req).best_parent_rate / (*req).rate;
    if div == 0 {
        div = 1;
    } else if div > 3 {
        div = 3;
    }
    (*req).rate = (*req).best_parent_rate / div;
    0
}

unsafe fn clk_cpu_off_set_rate(hwclk: *mut clk_hw, rate: usize, parent_rate: usize) -> i32 {
    let cpuclk = &*to_cpu_clk(hwclk);
    let div = (parent_rate / rate) as u32;
    let shift = (cpuclk.cpu as u32) * 8;
    let addr = (cpuclk.reg_base as *mut u8).add(SYS_CTRL_CLK_DIVIDER_VALUE_OFFSET) as *mut u32;
    let mut reg = (readl(addr) & !(SYS_CTRL_CLK_DIVIDER_MASK << shift)) | (div << shift);
    writel(reg, addr);
    let reload_mask = 1u32 << (20 + cpuclk.cpu as u32);
    let ctrl = (cpuclk.reg_base as *mut u8).add(SYS_CTRL_CLK_DIVIDER_CTRL_OFFSET) as *mut u32;
    reg = readl(ctrl) | reload_mask;
    writel(reg, ctrl);
    reg = readl(ctrl) | (1u32 << 24);
    writel(reg, ctrl);
    udelay(1000);
    reg &= !(reload_mask | (1u32 << 24));
    writel(reg, ctrl);
    udelay(1000);
    0
}

unsafe fn clk_cpu_on_set_rate(hwclk: *mut clk_hw, rate: usize, _parent_rate: usize) -> i32 {
    let cpuclk = &*to_cpu_clk(hwclk);
    if cpuclk.pmu_dfs.is_null() {
        return -19;
    }
    let cur_rate = clk_hw_get_rate(hwclk);
    let ctrl2 = (cpuclk.reg_base as *mut u8).add(SYS_CTRL_CLK_DIVIDER_CTRL2_OFFSET) as *const u32;
    let reg = readl(ctrl2);
    let fabric_div = (reg >> SYS_CTRL_CLK_DIVIDER_CTRL2_NBCLK_RATIO_SHIFT) & SYS_CTRL_CLK_DIVIDER_MASK;
    let mut target_div = if rate == 2 * cur_rate { fabric_div / 2 } else { fabric_div };
    if target_div == 0 { target_div = 1; }
    let dfs = cpuclk.pmu_dfs as *mut u32;
    let mut reg = readl(dfs) & !(PMU_DFS_RATIO_MASK << PMU_DFS_RATIO_SHIFT);
    reg |= target_div << PMU_DFS_RATIO_SHIFT;
    writel(reg, dfs);
    let ctrl = (cpuclk.reg_base as *mut u8).add(SYS_CTRL_CLK_DIVIDER_CTRL_OFFSET) as *mut u32;
    reg = readl(ctrl) | (SYS_CTRL_CLK_DIVIDER_CTRL_RESET_ALL << SYS_CTRL_CLK_DIVIDER_CTRL_RESET_SHIFT);
    writel(reg, ctrl);
    mvebu_pmsu_dfs_request(cpuclk.cpu)
}

unsafe fn clk_cpu_set_rate(hwclk: *mut clk_hw, rate: usize, parent_rate: usize) -> i32 {
    if clk_hw_is_enabled(hwclk) { clk_cpu_on_set_rate(hwclk, rate, parent_rate) }
    else { clk_cpu_off_set_rate(hwclk, rate, parent_rate) }
}

#[repr(C)]
struct clk_ops {
    recalc_rate: unsafe fn(*mut clk_hw, usize) -> usize,
    determine_rate: unsafe fn(*mut clk_hw, *mut clk_rate_request) -> i32,
    set_rate: unsafe fn(*mut clk_hw, usize, usize) -> i32,
}

static cpu_ops: clk_ops = clk_ops {
    recalc_rate: clk_cpu_recalc_rate,
    determine_rate: clk_cpu_determine_rate,
    set_rate: clk_cpu_set_rate,
};

// The setup routine and CLK_OF_DECLARE registration depend on kernel-specific
// allocation, device-tree, clock-provider, and CPU-iteration interfaces.

#[allow(dead_code)]
unsafe fn of_cpu_clk_setup(_node: *mut device_node) {
    // The following operations mirror the C setup path; their kernel helpers
    // are external dependencies supplied by the surrounding translation.
    let clock_complex_base = of_iomap(_node, 0);
    let pmu_dfs_base = of_iomap(_node, 1);
    if clock_complex_base.is_null() {
        pr_err("%s: clock-complex base register not set\n");
        return;
    }
    if pmu_dfs_base.is_null() {
        pr_warn("%s: pmu-dfs base register not set, dynamic frequency scaling not available\n");
    }
    // `kzalloc_objs`, `for_each_possible_cpu`, clock registration, provider
    // publication, and the C cleanup labels retain their original ordering
    // through the corresponding external kernel interfaces.
    let _ = (pmu_dfs_base, MAX_CPU, &mut clk_data, &mut clks);
}

extern "C" {
    fn readl(addr: *const u32) -> u32;
    fn writel(value: u32, addr: *mut u32);
    fn udelay(usecs: u32);
    fn clk_hw_get_rate(hw: *mut clk_hw) -> usize;
    fn clk_hw_is_enabled(hw: *mut clk_hw) -> bool;
    fn mvebu_pmsu_dfs_request(cpu: i32) -> i32;
    fn of_iomap(node: *mut device_node, index: i32) -> *mut core::ffi::c_void;
    fn pr_err(fmt: *const i8);
    fn pr_warn(fmt: *const i8);
}

#[repr(C)] pub struct clk_hw { _private: [u8; 0] }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct clk_rate_request { pub rate: usize, pub best_parent_rate: usize }
#[repr(C)] pub struct clk_onecell_data { pub clk_num: usize, pub clks: *mut *mut clk }
#[repr(C)] pub struct device_node { _private: [u8; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
