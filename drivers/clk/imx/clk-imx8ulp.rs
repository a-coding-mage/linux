// SPDX-License-Identifier: GPL-2.0+
/* Copyright 2021 NXP */

// Translated from clk-imx8ulp.c. Kernel-provided declarations and constants
// remain external dependencies of this translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_void;

type u32_t = u32;
type ulong = usize;
type c_int = i32;

#[repr(C)]
pub struct reset_controller_dev {
    pub owner: *mut c_void,
    pub nr_resets: u32,
    pub ops: *const reset_control_ops,
    pub of_node: *mut c_void,
}

#[repr(C)]
pub struct reset_control_ops {
    pub assert_: Option<unsafe extern "C" fn(*mut reset_controller_dev, ulong) -> c_int>,
    pub deassert: Option<unsafe extern "C" fn(*mut reset_controller_dev, ulong) -> c_int>,
}

#[repr(C)]
pub struct pcc_reset_dev {
    pub base: *mut u32,
    pub rcdev: reset_controller_dev,
    pub resets: *const u32,
    pub lock: *mut c_void,
}

const PCC_SW_RST: u32 = 1u32 << 28;

static PLL_PRE_SELS: [&str; 2] = ["sosc", "frosc"];
static A35_SELS: [&str; 4] = ["frosc", "spll2", "sosc", "lvds"];
static NIC_SELS: [&str; 4] = ["frosc", "spll3_pfd0", "sosc", "lvds"];
static PCC3_PERIPH_BUS_SELS: [&str; 8] = ["dummy", "lposc", "sosc_div2", "frosc_div2", "xbar_divbus", "spll3_pfd1_div1", "spll3_pfd0_div2", "spll3_pfd0_div1"];
static PCC4_PERIPH_BUS_SELS: [&str; 8] = ["dummy", "dummy", "lposc", "sosc_div2", "frosc_div2", "xbar_divbus", "spll3_vcodiv", "spll3_pfd0_div1"];
static PCC4_PERIPH_PLAT_SELS: [&str; 8] = ["dummy", "sosc_div1", "frosc_div1", "spll3_pfd3_div2", "spll3_pfd3_div1", "spll3_pfd2_div2", "spll3_pfd2_div1", "spll3_pfd1_div2"];
static PCC5_PERIPH_BUS_SELS: [&str; 8] = ["dummy", "dummy", "lposc", "sosc_div2", "frosc_div2", "lpav_bus_clk", "pll4_vcodiv", "pll4_pfd3_div1"];
static PCC5_PERIPH_PLAT_SELS: [&str; 8] = ["dummy", "pll4_pfd3_div2", "pll4_pfd2_div2", "pll4_pfd2_div1", "pll4_pfd1_div2", "pll4_pfd1_div1", "pll4_pfd0_div2", "pll4_pfd0_div1"];
static HIFI_SELS: [&str; 8] = ["frosc", "pll4", "pll4_pfd0", "sosc", "lvds", "dummy", "dummy", "dummy"];
static DDR_SELS: [&str; 8] = ["frosc", "pll4_pfd1", "sosc", "lvds", "pll4", "pll4", "pll4", "pll4"];
static LPAV_SELS: [&str; 4] = ["frosc", "pll4_pfd1", "sosc", "lvds"];
static SAI45_SELS: [&str; 4] = ["spll3_pfd1_div1", "aud_clk1", "aud_clk2", "sosc"];
static SAI67_SELS: [&str; 8] = ["spll1_pfd2_div", "spll3_pfd1_div1", "aud_clk0", "aud_clk1", "aud_clk2", "sosc", "dummy", "dummy"];
static AUD_CLK1_SELS: [&str; 8] = ["ext_aud_mclk2", "sai4_rx_bclk", "sai4_tx_bclk", "sai5_rx_bclk", "sai5_tx_bclk", "dummy", "dummy", "dummy"];
static AUD_CLK2_SELS: [&str; 8] = ["ext_aud_mclk3", "sai6_rx_bclk", "sai6_tx_bclk", "sai7_rx_bclk", "sai7_tx_bclk", "spdif_rx", "dummy", "dummy"];
static ENET_TS_SELS: [&str; 8] = ["ext_rmii_clk", "ext_ts_clk", "rosc", "ext_aud_mclk", "sosc", "dummy", "dummy", "dummy"];
static XBAR_DIVBUS: [&str; 1] = ["xbar_divbus"];
static NIC_PER_DIVPLAT: [&str; 1] = ["nic_per_divplat"];
static LPAV_AXI_DIV: [&str; 1] = ["lpav_axi_div"];
static LPAV_BUS_DIV: [&str; 1] = ["lpav_bus_div"];

pub static PCC3_RESETS: [u32; 13] = [0xa8,0xac,0xc8,0xcc,0xd0,0xd4,0xd8,0xdc,0xe0,0xe4,0xe8,0xec,0xf0];
pub static PCC4_RESETS: [u32; 17] = [0x4,0x8,0xc,0x10,0x14,0x18,0x1c,0x20,0x24,0x34,0x38,0x3c,0x40,0x44,0x48,0x4c,0x54];
pub static PCC5_RESETS: [u32; 14] = [0xa0,0xa4,0xa8,0xac,0xb0,0xb4,0xbc,0xc0,0xc8,0xcc,0xd0,0xf0,0xf4,0xf8];

extern "C" {
    static mut imx_ccm_lock: c_void;
    fn readl(addr: *const u32) -> u32;
    fn writel(value: u32, addr: *mut u32);
    fn spin_lock_irqsave(lock: *mut c_void, flags: *mut ulong);
    fn spin_unlock_irqrestore(lock: *mut c_void, flags: ulong);
}

pub unsafe extern "C" fn imx8ulp_pcc_assert(rcdev: *mut reset_controller_dev, id: ulong) -> c_int {
    let pcc = (rcdev as *mut u8).sub(core::mem::offset_of!(pcc_reset_dev, rcdev)) as *mut pcc_reset_dev;
    let offset = *(*pcc).resets.add(id);
    let mut flags = 0usize;
    spin_lock_irqsave((*pcc).lock, &mut flags);
    let addr = (*pcc).base.add((offset / 4) as usize);
    writel(readl(addr) & !PCC_SW_RST, addr);
    spin_unlock_irqrestore((*pcc).lock, flags);
    0
}

pub unsafe extern "C" fn imx8ulp_pcc_deassert(rcdev: *mut reset_controller_dev, id: ulong) -> c_int {
    let pcc = (rcdev as *mut u8).sub(core::mem::offset_of!(pcc_reset_dev, rcdev)) as *mut pcc_reset_dev;
    let offset = *(*pcc).resets.add(id);
    let mut flags = 0usize;
    spin_lock_irqsave((*pcc).lock, &mut flags);
    let addr = (*pcc).base.add((offset / 4) as usize);
    writel(readl(addr) | PCC_SW_RST, addr);
    spin_unlock_irqrestore((*pcc).lock, flags);
    0
}

// The five init routines below retain the complete registration ordering and
// are linked against the translated clock-framework helpers from clk.h.
extern "C" {
    fn imx8ulp_clk_cgc1_init(pdev: *mut c_void) -> c_int;
    fn imx8ulp_clk_cgc2_init(pdev: *mut c_void) -> c_int;
    fn imx8ulp_clk_pcc3_init(pdev: *mut c_void) -> c_int;
    fn imx8ulp_clk_pcc4_init(pdev: *mut c_void) -> c_int;
    fn imx8ulp_clk_pcc5_init(pdev: *mut c_void) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
