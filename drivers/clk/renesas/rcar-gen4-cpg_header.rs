/* SPDX-License-Identifier: GPL-2.0 */
/*
 * R-Car Gen4 Clock Pulse Generator
 *
 * Copyright (C) 2021 Renesas Electronics Corp.
 *
 */

// Dependency intent: CLK_TYPE_CUSTOM and DEF_BASE are supplied by other headers.

#[repr(i32)]
pub enum rcar_gen4_clk_types {
    CLK_TYPE_GEN4_MAIN = CLK_TYPE_CUSTOM,
    CLK_TYPE_GEN4_PLL1,
    CLK_TYPE_GEN4_PLL2X_3X, // r8a779a0 only
    CLK_TYPE_GEN4_PLL5,
    CLK_TYPE_GEN4_PLL_F8_25, // Fixed fractional 8.25 PLL
    CLK_TYPE_GEN4_PLL_V8_25, // Variable fractional 8.25 PLL
    CLK_TYPE_GEN4_PLL_F9_24, // Fixed fractional 9.24 PLL
    CLK_TYPE_GEN4_PLL_V9_24, // Variable fractional 9.24 PLL
    CLK_TYPE_GEN4_SDSRC,
    CLK_TYPE_GEN4_SDH,
    CLK_TYPE_GEN4_SD,
    CLK_TYPE_GEN4_MDSEL, // Select parent/divider using mode pin
    CLK_TYPE_GEN4_Z,
    CLK_TYPE_GEN4_OSC, // OSC EXTAL predivider and fixed divider
    CLK_TYPE_GEN4_RPCSRC,
    CLK_TYPE_GEN4_RPC,
    CLK_TYPE_GEN4_RPCD2,

    // SoC specific definitions start here
    CLK_TYPE_GEN4_SOC_BASE,
}

#[macro_export]
macro_rules! DEF_GEN4_SDH {
    ($name:expr, $id:expr, $parent:expr, $offset:expr) => {
        DEF_BASE!($name, $id, CLK_TYPE_GEN4_SDH, $parent, offset: $offset)
    };
}

#[macro_export]
macro_rules! DEF_GEN4_SD {
    ($name:expr, $id:expr, $parent:expr, $offset:expr) => {
        DEF_BASE!($name, $id, CLK_TYPE_GEN4_SD, $parent, offset: $offset)
    };
}

#[macro_export]
macro_rules! DEF_GEN4_MDSEL {
    ($name:expr, $id:expr, $md:expr, $parent0:expr, $div0:expr,
     $parent1:expr, $div1:expr) => {
        DEF_BASE!(
            $name,
            $id,
            CLK_TYPE_GEN4_MDSEL,
            (($parent0) << 16) | ($parent1),
            div: (($div0) << 16) | ($div1),
            offset: $md
        )
    };
}

#[macro_export]
macro_rules! DEF_GEN4_OSC {
    ($name:expr, $id:expr, $parent:expr, $div:expr) => {
        DEF_BASE!($name, $id, CLK_TYPE_GEN4_OSC, $parent, div: $div)
    };
}

#[macro_export]
macro_rules! DEF_GEN4_PLL_F8_25 {
    ($name:expr, $idx:expr, $id:expr, $parent:expr) => {
        DEF_BASE!($name, $id, CLK_TYPE_GEN4_PLL_F8_25, $parent, offset: $idx)
    };
}

#[macro_export]
macro_rules! DEF_GEN4_PLL_V8_25 {
    ($name:expr, $idx:expr, $id:expr, $parent:expr) => {
        DEF_BASE!($name, $id, CLK_TYPE_GEN4_PLL_V8_25, $parent, offset: $idx)
    };
}

#[macro_export]
macro_rules! DEF_GEN4_PLL_F9_24 {
    ($name:expr, $idx:expr, $id:expr, $parent:expr) => {
        DEF_BASE!($name, $id, CLK_TYPE_GEN4_PLL_F9_24, $parent, offset: $idx)
    };
}

#[macro_export]
macro_rules! DEF_GEN4_PLL_V9_24 {
    ($name:expr, $idx:expr, $id:expr, $parent:expr) => {
        DEF_BASE!($name, $id, CLK_TYPE_GEN4_PLL_V9_24, $parent, offset: $idx)
    };
}

#[macro_export]
macro_rules! DEF_GEN4_Z {
    ($name:expr, $id:expr, $type:expr, $parent:expr, $div:expr, $offset:expr) => {
        DEF_BASE!($name, $id, $type, $parent, div: $div, offset: $offset)
    };
}

#[repr(C)]
pub struct rcar_gen4_cpg_pll_config {
    pub extal_div: u8,
    pub pll1_mult: u8,
    pub pll1_div: u8,
    pub pll5_mult: u8,
    pub pll5_div: u8,
    pub osc_prediv: u8,
}

pub const CPG_SD0CKCR: u32 = 0x870; // SD-IF0 Clock Frequency Control Register
pub const CPG_CANFDCKCR: u32 = 0x878; // CAN-FD Clock Frequency Control Register
pub const CPG_MSOCKCR: u32 = 0x87c; // MSIOF Clock Frequency Control Register
pub const CPG_CSICKCR: u32 = 0x880; // CSI Clock Frequency Control Register
pub const CPG_DSIEXTCKCR: u32 = 0x884; // DSI Clock Frequency Control Register

extern "C" {
    pub fn rcar_gen4_cpg_clk_register(
        dev: *mut device,
        core: *const cpg_core_clk,
        info: *const cpg_mssr_info,
        pub_: *mut cpg_mssr_pub,
    ) -> *mut clk;

    pub fn rcar_gen4_cpg_init(
        config: *const rcar_gen4_cpg_pll_config,
        clk_extalr: ::core::ffi::c_uint,
        mode: u32,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
