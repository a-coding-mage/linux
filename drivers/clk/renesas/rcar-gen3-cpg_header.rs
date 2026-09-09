/* SPDX-License-Identifier: GPL-2.0 */
/*
 * R-Car Gen3 Clock Pulse Generator
 *
 * Copyright (C) 2015-2018 Glider bvba
 * Copyright (C) 2018 Renesas Electronics Corp.
 */

/* C enum rcar_gen3_clk_types. CLK_TYPE_CUSTOM is supplied externally. */
pub const CLK_TYPE_GEN3_MAIN: u32 = CLK_TYPE_CUSTOM;
pub const CLK_TYPE_GEN3_PLL0: u32 = CLK_TYPE_GEN3_MAIN + 1;
pub const CLK_TYPE_GEN3_PLL1: u32 = CLK_TYPE_GEN3_PLL0 + 1;
pub const CLK_TYPE_GEN3_PLL2: u32 = CLK_TYPE_GEN3_PLL1 + 1;
pub const CLK_TYPE_GEN3_PLL3: u32 = CLK_TYPE_GEN3_PLL2 + 1;
pub const CLK_TYPE_GEN3_PLL4: u32 = CLK_TYPE_GEN3_PLL3 + 1;
pub const CLK_TYPE_GEN3_SDH: u32 = CLK_TYPE_GEN3_PLL4 + 1;
pub const CLK_TYPE_GEN3_SD: u32 = CLK_TYPE_GEN3_SDH + 1;
pub const CLK_TYPE_GEN3_R: u32 = CLK_TYPE_GEN3_SD + 1;
pub const CLK_TYPE_GEN3_MDSEL: u32 = CLK_TYPE_GEN3_R + 1; // Select parent/divider using mode pin
pub const CLK_TYPE_GEN3_Z: u32 = CLK_TYPE_GEN3_MDSEL + 1;
pub const CLK_TYPE_GEN3_ZG: u32 = CLK_TYPE_GEN3_Z + 1;
pub const CLK_TYPE_GEN3_OSC: u32 = CLK_TYPE_GEN3_ZG + 1; // OSC EXTAL predivider and fixed divider
pub const CLK_TYPE_GEN3_RCKSEL: u32 = CLK_TYPE_GEN3_OSC + 1; // Select parent/divider using RCKCR.CKSEL
pub const CLK_TYPE_GEN3_RPCSRC: u32 = CLK_TYPE_GEN3_RCKSEL + 1;
pub const CLK_TYPE_GEN3_E3_RPCSRC: u32 = CLK_TYPE_GEN3_RPCSRC + 1; // Select parent/divider using RPCCKCR.DIV
pub const CLK_TYPE_GEN3_RPC: u32 = CLK_TYPE_GEN3_E3_RPCSRC + 1;
pub const CLK_TYPE_GEN3_RPCD2: u32 = CLK_TYPE_GEN3_RPC + 1;
pub const CLK_TYPE_GEN3_SOC_BASE: u32 = CLK_TYPE_GEN3_RPCD2 + 1;

macro_rules! DEF_GEN3_SDH {
    ($name:expr, $id:expr, $parent:expr, $offset:expr) => {
        DEF_BASE!($name, $id, CLK_TYPE_GEN3_SDH, $parent, offset: $offset)
    };
}

macro_rules! DEF_GEN3_SD {
    ($name:expr, $id:expr, $parent:expr, $offset:expr) => {
        DEF_BASE!($name, $id, CLK_TYPE_GEN3_SD, $parent, offset: $offset)
    };
}

macro_rules! DEF_GEN3_MDSEL {
    ($name:expr, $id:expr, $md:expr, $parent0:expr, $div0:expr, $parent1:expr, $div1:expr) => {
        DEF_BASE!($name, $id, CLK_TYPE_GEN3_MDSEL,
            (($parent0) << 16) | ($parent1),
            div: (($div0) << 16) | ($div1), offset: $md)
    };
}

macro_rules! DEF_GEN3_PE {
    ($name:expr, $id:expr, $parent_sscg:expr, $div_sscg:expr, $parent_clean:expr, $div_clean:expr) => {
        DEF_GEN3_MDSEL!($name, $id, 12, $parent_sscg, $div_sscg, $parent_clean, $div_clean)
    };
}

macro_rules! DEF_GEN3_OSC {
    ($name:expr, $id:expr, $parent:expr, $div:expr) => {
        DEF_BASE!($name, $id, CLK_TYPE_GEN3_OSC, $parent, div: $div)
    };
}

macro_rules! DEF_GEN3_RCKSEL {
    ($name:expr, $id:expr, $parent0:expr, $div0:expr, $parent1:expr, $div1:expr) => {
        DEF_BASE!($name, $id, CLK_TYPE_GEN3_RCKSEL,
            (($parent0) << 16) | ($parent1), div: (($div0) << 16) | ($div1))
    };
}

macro_rules! DEF_GEN3_Z {
    ($name:expr, $id:expr, $type:expr, $parent:expr, $div:expr, $offset:expr) => {
        DEF_BASE!($name, $id, $type, $parent, div: $div, offset: $offset)
    };
}

macro_rules! DEF_FIXED_RPCSRC_E3 {
    ($name:expr, $id:expr, $parent0:expr, $parent1:expr) => {
        DEF_BASE!($name, $id, CLK_TYPE_GEN3_E3_RPCSRC,
            (($parent0) << 16) | ($parent1), div: 8)
    };
}

macro_rules! DEF_FIXED_RPCSRC_D3 {
    ($name:expr, $id:expr, $parent0:expr, $parent1:expr) => {
        DEF_BASE!($name, $id, CLK_TYPE_GEN3_E3_RPCSRC,
            (($parent0) << 16) | ($parent1), div: 5)
    };
}

#[repr(C)]
pub struct rcar_gen3_cpg_pll_config {
    pub extal_div: u8,
    pub pll1_mult: u8,
    pub pll1_div: u8,
    pub pll3_mult: u8,
    pub pll3_div: u8,
    pub osc_prediv: u8,
}

pub const CPG_RPCCKCR: u32 = 0x238;
pub const CPG_RCKCR: u32 = 0x240;

extern "C" {
    pub fn rcar_gen3_cpg_clk_register(
        dev: *mut device,
        core: *const cpg_core_clk,
        info: *const cpg_mssr_info,
        pub_: *mut cpg_mssr_pub,
    ) -> *mut clk;
    pub fn rcar_gen3_cpg_init(
        config: *const rcar_gen3_cpg_pll_config,
        clk_extalr: u32,
        mode: u32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
