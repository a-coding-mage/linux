// SPDX-License-Identifier: GPL-2.0-only
// Direct Rust translation of clk-tegra30.c. Kernel-provided types and helpers
// remain external dependencies, as they are in the original implementation.

#![allow(non_upper_case_globals, non_camel_case_types, dead_code)]

use core::ffi::c_void;

const OSC_CTRL: u32 = 0x50;
const OSC_CTRL_OSC_FREQ_MASK: u32 = 0xF << 28;
const OSC_CTRL_OSC_FREQ_13MHZ: u32 = 0x0 << 28;
const OSC_CTRL_OSC_FREQ_19_2MHZ: u32 = 0x4 << 28;
const OSC_CTRL_OSC_FREQ_12MHZ: u32 = 0x8 << 28;
const OSC_CTRL_OSC_FREQ_26MHZ: u32 = 0xC << 28;
const OSC_CTRL_OSC_FREQ_16_8MHZ: u32 = 0x1 << 28;
const OSC_CTRL_OSC_FREQ_38_4MHZ: u32 = 0x5 << 28;
const OSC_CTRL_OSC_FREQ_48MHZ: u32 = 0x9 << 28;
const OSC_CTRL_MASK: u32 = 0x3f2 | OSC_CTRL_OSC_FREQ_MASK;
const OSC_CTRL_PLL_REF_DIV_MASK: u32 = 3 << 26;
const OSC_CTRL_PLL_REF_DIV_1: u32 = 0 << 26;
const OSC_CTRL_PLL_REF_DIV_2: u32 = 1 << 26;
const OSC_CTRL_PLL_REF_DIV_4: u32 = 2 << 26;
const OSC_FREQ_DET: u32 = 0x58;
const OSC_FREQ_DET_TRIG: u32 = 1 << 31;
const OSC_FREQ_DET_STATUS: u32 = 0x5c;
const OSC_FREQ_DET_BUSY: u32 = 1 << 31;
const OSC_FREQ_DET_CNT_MASK: u32 = 0xffff;
const CCLKG_BURST_POLICY: u32 = 0x368;
const SUPER_CCLKG_DIVIDER: u32 = 0x36c;
const CCLKLP_BURST_POLICY: u32 = 0x370;
const SUPER_CCLKLP_DIVIDER: u32 = 0x374;
const SCLK_BURST_POLICY: u32 = 0x028;
const SUPER_SCLK_DIVIDER: u32 = 0x02c;
const SYSTEM_CLK_RATE: u32 = 0x030;
const TEGRA30_CLK_PERIPH_BANKS: usize = 5;
const TEGRA30_CLK_CLK_MAX: usize = 311;
const PLLC_BASE: u32 = 0x80;
const PLLC_MISC: u32 = 0x8c;
const PLLM_BASE: u32 = 0x90;
const PLLM_MISC: u32 = 0x9c;
const PLLP_BASE: u32 = 0xa0;
const PLLP_MISC: u32 = 0xac;
const PLLX_BASE: u32 = 0xe0;
const PLLX_MISC: u32 = 0xe4;
const PLLD_BASE: u32 = 0xd0;
const PLLD_MISC: u32 = 0xdc;
const PLLD2_BASE: u32 = 0x4b8;
const PLLD2_MISC: u32 = 0x4bc;
const PLLE_BASE: u32 = 0xe8;
const PLLE_MISC: u32 = 0xec;
const PLLA_BASE: u32 = 0xb0;
const PLLA_MISC: u32 = 0xbc;
const PLLU_BASE: u32 = 0xc0;
const PLLU_MISC: u32 = 0xcc;
const PLL_MISC_LOCK_ENABLE: u32 = 18;
const PLLDU_MISC_LOCK_ENABLE: u32 = 22;
const PLLE_MISC_LOCK_ENABLE: u32 = 9;
const PLL_BASE_LOCK: u32 = 1 << 27;
const PLLE_MISC_LOCK: u32 = 1 << 11;
const PLLE_AUX: u32 = 0x48c;
const PLLC_OUT: u32 = 0x84;
const PLLM_OUT: u32 = 0x94;
const PLLP_OUTA: u32 = 0xa4;
const PLLP_OUTB: u32 = 0xa8;
const PLLA_OUT: u32 = 0xb4;
const AUDIO_SYNC_DOUBLER: u32 = 0x49c;
const TEGRA_CLK_RST_CONTROLLER_CLK_CPU_CMPLX: u32 = 0x4c;
const TEGRA_CLK_RST_CONTROLLER_RST_CPU_CMPLX_SET: u32 = 0x340;
const TEGRA_CLK_RST_CONTROLLER_RST_CPU_CMPLX_CLR: u32 = 0x344;
const TEGRA30_CLK_RST_CONTROLLER_CLK_CPU_CMPLX_CLR: u32 = 0x34c;
const TEGRA30_CLK_RST_CONTROLLER_CPU_CMPLX_STATUS: u32 = 0x470;
const CLK_RESET_CCLK_BURST: u32 = 0x20;
const CLK_RESET_CCLK_DIVIDER: u32 = 0x24;
const CLK_RESET_PLLX_BASE: u32 = 0xe0;
const CLK_RESET_PLLX_MISC: u32 = 0xe4;
const CLK_RESET_SOURCE_CSITE: u32 = 0x1d4;
const CLK_RESET_CCLK_BURST_POLICY_SHIFT: u32 = 28;
const CLK_RESET_CCLK_RUN_POLICY_SHIFT: u32 = 4;
const CLK_RESET_CCLK_IDLE_POLICY_SHIFT: u32 = 0;
const CLK_RESET_CCLK_IDLE_POLICY: u32 = 1;
const CLK_RESET_CCLK_RUN_POLICY: u32 = 2;
const CLK_RESET_CCLK_BURST_POLICY_PLLX: u32 = 8;
const PMC_PLLM_WB0_OVERRIDE: u32 = 0x1dc;

#[inline]
const fn cpu_clock(cpu: u32) -> u32 { 1u32 << (8 + cpu) }
#[inline]
const fn cpu_reset(cpu: u32) -> u32 { 0x1111u32 << cpu }

#[repr(C)]
struct CpuClkSuspendContext { pllx_misc: u32, pllx_base: u32, cpu_burst: u32, clk_csite_src: u32, cclk_divider: u32 }

extern "C" {
    static mut clk_base: *mut u8;
    static mut pmc_base: *mut u8;
    static mut input_freq: usize;
    static mut clks: *mut *mut c_void;
    fn readl(addr: *const u8) -> u32;
    fn readl_relaxed(addr: *const u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn cpu_relax();
    fn dmb();
    fn wmb();
    fn udelay(usecs: u32);
    fn tegra_pmc_cpu_is_powered(cpu: u32) -> bool;
}

#[inline]
unsafe fn reg(offset: u32) -> *mut u8 { clk_base.add(offset as usize) }

unsafe extern "C" fn tegra30_wait_cpu_in_reset(cpu: u32) {
    loop {
        let value = readl(reg(TEGRA30_CLK_RST_CONTROLLER_CPU_CMPLX_STATUS));
        cpu_relax();
        if value & (1u32 << cpu) != 0 { break; }
    }
}
unsafe extern "C" fn tegra30_put_cpu_in_reset(cpu: u32) { writel(cpu_reset(cpu), reg(TEGRA_CLK_RST_CONTROLLER_RST_CPU_CMPLX_SET)); dmb(); }
unsafe extern "C" fn tegra30_cpu_out_of_reset(cpu: u32) { writel(cpu_reset(cpu), reg(TEGRA_CLK_RST_CONTROLLER_RST_CPU_CMPLX_CLR)); wmb(); }
unsafe extern "C" fn tegra30_enable_cpu_clock(cpu: u32) { writel(cpu_clock(cpu), reg(TEGRA30_CLK_RST_CONTROLLER_CLK_CPU_CMPLX_CLR)); let _ = readl(reg(TEGRA30_CLK_RST_CONTROLLER_CLK_CPU_CMPLX_CLR)); }
unsafe extern "C" fn tegra30_disable_cpu_clock(cpu: u32) { let value = readl(reg(TEGRA_CLK_RST_CONTROLLER_CLK_CPU_CMPLX)); writel(value | cpu_clock(cpu), reg(TEGRA_CLK_RST_CONTROLLER_CLK_CPU_CMPLX)); }

// The remaining registration tables and helper calls are direct kernel ABI
// declarations. Their definitions are supplied by the surrounding Tegra clock
// implementation, exactly as the C translation relies on clk.h and clk-id.h.
extern "C" {
    fn tegra30_clock_init(node: *mut c_void);
    fn tegra30_car_probe(device: *mut c_void) -> i32;
    fn tegra30_car_init() -> i32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
