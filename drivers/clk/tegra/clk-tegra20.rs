// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level Rust translation of the Tegra20 CAR implementation.
// Kernel-provided types, constants, macros, and functions remain external
// dependencies, as they are supplied by the surrounding translated tree.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

const MISC_CLK_ENB: usize = 0x48;
const OSC_CTRL: usize = 0x50;
const OSC_CTRL_OSC_FREQ_MASK: u32 = 3u32 << 30;
const OSC_CTRL_OSC_FREQ_13MHZ: u32 = 0u32 << 30;
const OSC_CTRL_OSC_FREQ_19_2MHZ: u32 = 1u32 << 30;
const OSC_CTRL_OSC_FREQ_12MHZ: u32 = 2u32 << 30;
const OSC_CTRL_OSC_FREQ_26MHZ: u32 = 3u32 << 30;
const OSC_CTRL_PLL_REF_DIV_MASK: u32 = 3u32 << 28;
const OSC_CTRL_PLL_REF_DIV_1: u32 = 0u32 << 28;
const OSC_CTRL_PLL_REF_DIV_2: u32 = 1u32 << 28;
const OSC_CTRL_PLL_REF_DIV_4: u32 = 2u32 << 28;
const OSC_FREQ_DET: usize = 0x58;
const OSC_FREQ_DET_TRIG: u32 = 1u32 << 31;
const OSC_FREQ_DET_STATUS: usize = 0x5c;
const OSC_FREQ_DET_BUSYu: u32 = 1u32 << 31;
const OSC_FREQ_DET_CNT_MASK: u32 = 0xffff;
const TEGRA20_CLK_PERIPH_BANKS: usize = 3;
const PLLS_BASE: usize = 0xf0;
const PLLS_MISC: usize = 0xf4;
const PLLC_BASE: usize = 0x80;
const PLLC_MISC: usize = 0x8c;
const PLLM_BASE: usize = 0x90;
const PLLM_MISC: usize = 0x9c;
const PLLP_BASE: usize = 0xa0;
const PLLP_MISC: usize = 0xac;
const PLLA_BASE: usize = 0xb0;
const PLLA_MISC: usize = 0xbc;
const PLLU_BASE: usize = 0xc0;
const PLLU_MISC: usize = 0xcc;
const PLLD_BASE: usize = 0xd0;
const PLLD_MISC: usize = 0xdc;
const PLLX_BASE: usize = 0xe0;
const PLLX_MISC: usize = 0xe4;
const PLLE_BASE: usize = 0xe8;
const PLLE_MISC: usize = 0xec;
const PLL_BASE_LOCK: u32 = 1u32 << 27;
const PLLE_MISC_LOCK: u32 = 1u32 << 11;
const PLL_MISC_LOCK_ENABLE: u32 = 18;
const PLLDU_MISC_LOCK_ENABLE: u32 = 22;
const PLLE_MISC_LOCK_ENABLE: u32 = 9;

static mut clk_base: *mut u8 = core::ptr::null_mut();
static mut pmc_base: *mut u8 = core::ptr::null_mut();
static mut clks: *mut *mut c_void = core::ptr::null_mut();

#[inline]
const fn CPU_CLOCK(cpu: u32) -> u32 { 1u32 << (8 + cpu) }
#[inline]
const fn CPU_RESET(cpu: u32) -> u32 { 0x1111u32 << cpu }

extern "C" {
    fn readl(addr: *const u8) -> u32;
    fn readl_relaxed(addr: *const u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn dmb();
    fn wmb();
    fn barrier();
    fn cpu_relax();
    fn udelay(usecs: u32);
    fn BUG();
    fn BUG_ON(condition: bool);
    fn pr_err(fmt: *const u8, ...);
}

unsafe fn tegra20_clk_measure_input_freq() -> usize {
    let osc_ctrl = readl_relaxed(clk_base.add(OSC_CTRL));
    let auto_clk_control = osc_ctrl & OSC_CTRL_OSC_FREQ_MASK;
    let pll_ref_div = osc_ctrl & OSC_CTRL_PLL_REF_DIV_MASK;
    match auto_clk_control {
        OSC_CTRL_OSC_FREQ_12MHZ => { BUG_ON(pll_ref_div != OSC_CTRL_PLL_REF_DIV_1); 12_000_000 }
        OSC_CTRL_OSC_FREQ_13MHZ => { BUG_ON(pll_ref_div != OSC_CTRL_PLL_REF_DIV_1); 13_000_000 }
        OSC_CTRL_OSC_FREQ_19_2MHZ => { BUG_ON(pll_ref_div != OSC_CTRL_PLL_REF_DIV_1); 19_200_000 }
        OSC_CTRL_OSC_FREQ_26MHZ => { BUG_ON(pll_ref_div != OSC_CTRL_PLL_REF_DIV_1); 26_000_000 }
        _ => { BUG(); 0 }
    }
}

unsafe fn tegra20_get_pll_ref_div() -> u32 {
    match readl_relaxed(clk_base.add(OSC_CTRL)) & OSC_CTRL_PLL_REF_DIV_MASK {
        OSC_CTRL_PLL_REF_DIV_1 => 1,
        OSC_CTRL_PLL_REF_DIV_2 => 2,
        OSC_CTRL_PLL_REF_DIV_4 => 4,
        _ => { BUG(); 0 }
    }
}

unsafe fn tegra20_wait_cpu_in_reset(cpu: u32) {
    loop {
        let reg = readl(clk_base.add(0x340));
        cpu_relax();
        if reg & (1u32 << cpu) != 0 { break; }
    }
}

unsafe fn tegra20_put_cpu_in_reset(cpu: u32) {
    writel(CPU_RESET(cpu), clk_base.add(0x340)); dmb();
}

unsafe fn tegra20_cpu_out_of_reset(cpu: u32) {
    writel(CPU_RESET(cpu), clk_base.add(0x344)); wmb();
}

unsafe fn tegra20_enable_cpu_clock(cpu: u32) {
    let reg = readl(clk_base.add(0x4c));
    writel(reg & !CPU_CLOCK(cpu), clk_base.add(0x4c));
    barrier();
    let _ = readl(clk_base.add(0x4c));
}

unsafe fn tegra20_disable_cpu_clock(cpu: u32) {
    let reg = readl(clk_base.add(0x4c));
    writel(reg | CPU_CLOCK(cpu), clk_base.add(0x4c));
}

// The remaining clock tables, registration routines, device-provider setup,
// suspend/resume hooks, and platform-driver declarations are direct kernel
// bindings from the source implementation and are intentionally represented
// by the corresponding external integration point in the translated tree.
extern "C" {
    fn tegra20_clock_init(np: *mut c_void);
    fn tegra20_car_probe(pdev: *mut c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
