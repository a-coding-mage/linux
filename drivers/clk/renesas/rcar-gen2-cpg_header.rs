/* SPDX-License-Identifier: GPL-2.0 */
/*
 * R-Car Gen2 Clock Pulse Generator
 *
 * Copyright (C) 2016 Cogent Embedded Inc.
 */

/* Translated from the C header; CLK_TYPE_CUSTOM is supplied externally. */
pub const CLK_TYPE_GEN2_MAIN: i32 = CLK_TYPE_CUSTOM;
pub const CLK_TYPE_GEN2_PLL0: i32 = CLK_TYPE_GEN2_MAIN + 1;
pub const CLK_TYPE_GEN2_PLL1: i32 = CLK_TYPE_GEN2_PLL0 + 1;
pub const CLK_TYPE_GEN2_PLL3: i32 = CLK_TYPE_GEN2_PLL1 + 1;
pub const CLK_TYPE_GEN2_Z: i32 = CLK_TYPE_GEN2_PLL3 + 1;
pub const CLK_TYPE_GEN2_LB: i32 = CLK_TYPE_GEN2_Z + 1;
pub const CLK_TYPE_GEN2_ADSP: i32 = CLK_TYPE_GEN2_LB + 1;
pub const CLK_TYPE_GEN2_SDH: i32 = CLK_TYPE_GEN2_ADSP + 1;
pub const CLK_TYPE_GEN2_SD0: i32 = CLK_TYPE_GEN2_SDH + 1;
pub const CLK_TYPE_GEN2_SD1: i32 = CLK_TYPE_GEN2_SD0 + 1;
pub const CLK_TYPE_GEN2_QSPI: i32 = CLK_TYPE_GEN2_SD1 + 1;
pub const CLK_TYPE_GEN2_RCAN: i32 = CLK_TYPE_GEN2_QSPI + 1;

#[repr(C)]
pub struct rcar_gen2_cpg_pll_config {
    pub extal_div: u8,
    pub pll1_mult: u8,
    pub pll3_mult: u8,
    /* leave as zero if PLL0CR exists */
    pub pll0_mult: u8,
}

extern "C" {
    pub fn rcar_gen2_cpg_clk_register(
        dev: *mut device,
        core: *const cpg_core_clk,
        info: *const cpg_mssr_info,
        pub_: *mut cpg_mssr_pub,
    ) -> *mut clk;

    pub fn rcar_gen2_cpg_init(
        config: *const rcar_gen2_cpg_pll_config,
        pll0_div: u32,
        mode: u32,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
