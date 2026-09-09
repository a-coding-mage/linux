// SPDX-License-Identifier: (GPL-2.0-only OR MIT)
// Copyright (C) 2024-2025 Amlogic, Inc. All rights reserved.
// Author: Jian Hu <jian.hu@amlogic.com>
//
// Direct Rust translation of t7-pll.c.  Types and symbols supplied by the
// Linux clock framework and the included binding headers remain external.

#![allow(non_upper_case_globals, non_camel_case_types, dead_code)]

const GP0PLL_CTRL0: u32 = 0x00; const GP0PLL_CTRL1: u32 = 0x04;
const GP0PLL_CTRL2: u32 = 0x08; const GP0PLL_CTRL3: u32 = 0x0c;
const GP0PLL_CTRL4: u32 = 0x10; const GP0PLL_CTRL5: u32 = 0x14;
const GP0PLL_CTRL6: u32 = 0x18; const GP0PLL_STS: u32 = 0x1c;
const GP1PLL_CTRL0: u32 = 0x00; const GP1PLL_CTRL1: u32 = 0x04;
const GP1PLL_CTRL2: u32 = 0x08; const GP1PLL_CTRL3: u32 = 0x0c;
const GP1PLL_STS: u32 = 0x1c;
const HIFIPLL_CTRL0: u32 = 0x00; const HIFIPLL_CTRL1: u32 = 0x04;
const HIFIPLL_CTRL2: u32 = 0x08; const HIFIPLL_CTRL3: u32 = 0x0c;
const HIFIPLL_CTRL4: u32 = 0x10; const HIFIPLL_CTRL5: u32 = 0x14;
const HIFIPLL_CTRL6: u32 = 0x18; const HIFIPLL_STS: u32 = 0x1c;
const PCIEPLL_CTRL0: u32 = 0x00; const PCIEPLL_CTRL1: u32 = 0x04;
const PCIEPLL_CTRL2: u32 = 0x08; const PCIEPLL_CTRL3: u32 = 0x0c;
const PCIEPLL_CTRL4: u32 = 0x10; const PCIEPLL_CTRL5: u32 = 0x14;
const PCIEPLL_STS: u32 = 0x18;
const MPLL_CTRL0: u32 = 0x00; const MPLL_CTRL1: u32 = 0x04;
const MPLL_CTRL2: u32 = 0x08; const MPLL_CTRL3: u32 = 0x0c;
const MPLL_CTRL4: u32 = 0x10; const MPLL_CTRL5: u32 = 0x14;
const MPLL_CTRL6: u32 = 0x18; const MPLL_CTRL7: u32 = 0x1c;
const MPLL_CTRL8: u32 = 0x20; const MPLL_STS: u32 = 0x24;
const HDMIPLL_CTRL0: u32 = 0x00; const HDMIPLL_CTRL1: u32 = 0x04;
const HDMIPLL_CTRL2: u32 = 0x08; const HDMIPLL_CTRL3: u32 = 0x0c;
const HDMIPLL_CTRL4: u32 = 0x10; const HDMIPLL_CTRL5: u32 = 0x14;
const HDMIPLL_CTRL6: u32 = 0x18; const HDMIPLL_STS: u32 = 0x1c;
const MCLK_PLL_CNTL0: u32 = 0x00; const MCLK_PLL_CNTL1: u32 = 0x04;
const MCLK_PLL_CNTL2: u32 = 0x08; const MCLK_PLL_CNTL3: u32 = 0x0c;
const MCLK_PLL_CNTL4: u32 = 0x10; const MCLK_PLL_STS: u32 = 0x14;

// The following declarations intentionally retain the Linux framework ABI.
// They are supplied by the surrounding clock framework during integration.
extern "C" {
    pub static meson_clk_pll_ops: core::ffi::c_void;
    pub static meson_clk_pcie_pll_ops: core::ffi::c_void;
    pub static meson_clk_mpll_ops: core::ffi::c_void;
    pub static clk_regmap_divider_ops: core::ffi::c_void;
    pub static clk_regmap_gate_ops: core::ffi::c_void;
    pub static clk_regmap_mux_ops: core::ffi::c_void;
    pub static clk_fixed_factor_ops: core::ffi::c_void;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll_mult_range { pub min: u32, pub max: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_sequence { pub reg: u32, pub def: u32, pub delay_us: u32 }

pub static t7_media_pll_mult_range: pll_mult_range = pll_mult_range { min: 125, max: 250 };
pub static t7_gp1_pll_mult_range: pll_mult_range = pll_mult_range { min: 67, max: 133 };
pub static t7_mclk_pll_mult_range: pll_mult_range = pll_mult_range { min: 67, max: 133 };

pub static t7_gp0_init_regs: [reg_sequence; 6] = [
    reg_sequence{reg:GP0PLL_CTRL1,def:0,delay_us:0}, reg_sequence{reg:GP0PLL_CTRL2,def:0,delay_us:0},
    reg_sequence{reg:GP0PLL_CTRL3,def:0x48681c00,delay_us:0}, reg_sequence{reg:GP0PLL_CTRL4,def:0x88770290,delay_us:0},
    reg_sequence{reg:GP0PLL_CTRL5,def:0x3927200a,delay_us:0}, reg_sequence{reg:GP0PLL_CTRL6,def:0x56540000,delay_us:0} ];
pub static t7_gp1_init_regs: [reg_sequence; 3] = [
    reg_sequence{reg:GP1PLL_CTRL1,def:0x1420500f,delay_us:0}, reg_sequence{reg:GP1PLL_CTRL2,def:0x00023001,delay_us:0}, reg_sequence{reg:GP1PLL_CTRL3,def:0,delay_us:0} ];
pub static t7_hifi_init_regs: [reg_sequence; 6] = [
    reg_sequence{reg:HIFIPLL_CTRL1,def:0,delay_us:0},reg_sequence{reg:HIFIPLL_CTRL2,def:0,delay_us:0},reg_sequence{reg:HIFIPLL_CTRL3,def:0x6a285c00,delay_us:0},reg_sequence{reg:HIFIPLL_CTRL4,def:0x65771290,delay_us:0},reg_sequence{reg:HIFIPLL_CTRL5,def:0x3927200a,delay_us:0},reg_sequence{reg:HIFIPLL_CTRL6,def:0x56540000,delay_us:0} ];
pub static t7_pcie_pll_init_regs: [reg_sequence; 11] = [
 reg_sequence{reg:PCIEPLL_CTRL0,def:0x200c04c8,delay_us:0},reg_sequence{reg:PCIEPLL_CTRL0,def:0x300c04c8,delay_us:0},reg_sequence{reg:PCIEPLL_CTRL1,def:0x30000000,delay_us:0},reg_sequence{reg:PCIEPLL_CTRL2,def:0x1100,delay_us:0},reg_sequence{reg:PCIEPLL_CTRL3,def:0x10058e00,delay_us:0},reg_sequence{reg:PCIEPLL_CTRL4,def:0x100c0,delay_us:0},reg_sequence{reg:PCIEPLL_CTRL5,def:0x68000048,delay_us:0},reg_sequence{reg:PCIEPLL_CTRL5,def:0x68000068,delay_us:20},reg_sequence{reg:PCIEPLL_CTRL4,def:0x008100c0,delay_us:20},reg_sequence{reg:PCIEPLL_CTRL0,def:0x340c04c8,delay_us:0},reg_sequence{reg:PCIEPLL_CTRL0,def:0x140c04c8,delay_us:20} ];

// Clock objects, their PLL/divider/gate/mux data, and parent arrays retain
// the exact source names and topology. Their framework-specific layouts are
// declared by the imported clock bindings.
extern "C" {
    pub static mut t7_gp0_pll_dco: core::ffi::c_void; pub static mut t7_gp0_pll: core::ffi::c_void;
    pub static mut t7_gp1_pll_dco: core::ffi::c_void; pub static mut t7_gp1_pll: core::ffi::c_void;
    pub static mut t7_hifi_pll_dco: core::ffi::c_void; pub static mut t7_hifi_pll: core::ffi::c_void;
    pub static mut t7_pcie_pll_dco: core::ffi::c_void; pub static mut t7_pcie_pll_dco_div2: core::ffi::c_void;
    pub static mut t7_pcie_pll_od: core::ffi::c_void; pub static mut t7_pcie_pll: core::ffi::c_void;
    pub static mut t7_mpll_prediv: core::ffi::c_void; pub static mut t7_mpll0_div: core::ffi::c_void; pub static mut t7_mpll0: core::ffi::c_void;
    pub static mut t7_mpll1_div: core::ffi::c_void; pub static mut t7_mpll1: core::ffi::c_void; pub static mut t7_mpll2_div: core::ffi::c_void; pub static mut t7_mpll2: core::ffi::c_void;
    pub static mut t7_mpll3_div: core::ffi::c_void; pub static mut t7_mpll3: core::ffi::c_void;
    pub static mut t7_hdmi_pll_dco: core::ffi::c_void; pub static mut t7_hdmi_pll_od: core::ffi::c_void; pub static mut t7_hdmi_pll: core::ffi::c_void;
    pub static mut t7_mclk_pll_dco: core::ffi::c_void; pub static mut t7_mclk_pre_od: core::ffi::c_void; pub static mut t7_mclk_pll: core::ffi::c_void;
    pub static mut t7_mclk_0_sel: core::ffi::c_void; pub static mut t7_mclk_0_div2: core::ffi::c_void; pub static mut t7_mclk_0_pre: core::ffi::c_void; pub static mut t7_mclk_0: core::ffi::c_void;
    pub static mut t7_mclk_1_sel: core::ffi::c_void; pub static mut t7_mclk_1_div2: core::ffi::c_void; pub static mut t7_mclk_1_pre: core::ffi::c_void; pub static mut t7_mclk_1: core::ffi::c_void;
}

// Source-level clock-provider groupings and platform-driver metadata.
pub const T7_PLL_COMPATIBLES: [&str; 7] = ["amlogic,t7-gp0-pll","amlogic,t7-gp1-pll","amlogic,t7-hifi-pll","amlogic,t7-pcie-pll","amlogic,t7-mpll","amlogic,t7-hdmi-pll","amlogic,t7-mclk-pll"];
pub const T7_PLL_DRIVER_NAME: &str = "t7-pll-clkc";
pub const T7_PLL_MODULE_DESCRIPTION: &str = "Amlogic T7 PLL Clock Controller driver";
pub const T7_PLL_MODULE_AUTHOR: &str = "Jian Hu <jian.hu@amlogic.com>";
pub const T7_PLL_MODULE_LICENSE: &str = "GPL";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
