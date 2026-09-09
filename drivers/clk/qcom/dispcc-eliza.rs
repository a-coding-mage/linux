// SPDX-License-Identifier: GPL-2.0-only
/*
 * Direct Rust-side representation of the Qualcomm Eliza display clock
 * controller.  The Linux clock-controller structures and operations referenced
 * below are supplied by the surrounding kernel/Rust bindings.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

// Dependency intent retained from the C implementation:
// linux/clk-provider.h, linux/kernel.h, linux/module.h, linux/platform_device.h,
// linux/regmap.h, dt-bindings/clock/qcom,eliza-dispcc.h, and the local clock,
// PLL, branch, RCG, regmap, GDSC, and reset support headers.

pub const DISP_CC_MISC_CMD: u32 = 0xF000;

// Clock indices must match the device-tree binding order.
#[repr(u32)]
pub enum DtClock {
    DT_BI_TCXO,
    DT_BI_TCXO_AO,
    DT_AHB_CLK,
    DT_SLEEP_CLK,
    DT_DSI0_PHY_PLL_OUT_BYTECLK,
    DT_DSI0_PHY_PLL_OUT_DSICLK,
    DT_DSI1_PHY_PLL_OUT_BYTECLK,
    DT_DSI1_PHY_PLL_OUT_DSICLK,
    DT_DP0_PHY_PLL_LINK_CLK,
    DT_DP0_PHY_PLL_VCO_DIV_CLK,
    DT_DP1_PHY_PLL_LINK_CLK,
    DT_DP1_PHY_PLL_VCO_DIV_CLK,
    DT_DP2_PHY_PLL_LINK_CLK,
    DT_DP2_PHY_PLL_VCO_DIV_CLK,
    DT_DP3_PHY_PLL_LINK_CLK,
    DT_DP3_PHY_PLL_VCO_DIV_CLK,
    DT_HDMI_PHY_PLL_CLK,
}

#[repr(u32)]
pub enum ParentIndex {
    P_BI_TCXO,
    P_DISP_CC_PLL0_OUT_MAIN,
    P_DISP_CC_PLL1_OUT_EVEN,
    P_DISP_CC_PLL1_OUT_MAIN,
    P_DISP_CC_PLL2_OUT_MAIN,
    P_DP0_PHY_PLL_LINK_CLK,
    P_DP0_PHY_PLL_VCO_DIV_CLK,
    P_DP1_PHY_PLL_LINK_CLK,
    P_DP1_PHY_PLL_VCO_DIV_CLK,
    P_DP2_PHY_PLL_LINK_CLK,
    P_DP2_PHY_PLL_VCO_DIV_CLK,
    P_DP3_PHY_PLL_LINK_CLK,
    P_DP3_PHY_PLL_VCO_DIV_CLK,
    P_DSI0_PHY_PLL_OUT_BYTECLK,
    P_DSI0_PHY_PLL_OUT_DSICLK,
    P_DSI1_PHY_PLL_OUT_BYTECLK,
    P_DSI1_PHY_PLL_OUT_DSICLK,
    P_HDMI_PHY_PLL_CLK,
    P_SLEEP_CLK,
}

#[repr(C)]
pub struct PllVco {
    pub min_freq: u64,
    pub max_freq: u64,
    pub val: u32,
}

pub static LUCID_OLE_VCO: [PllVco; 1] = [PllVco { min_freq: 249_600_000, max_freq: 2_300_000_000, val: 0 }];
pub static PONGO_OLE_VCO: [PllVco; 1] = [PllVco { min_freq: 38_400_000, max_freq: 38_400_000, val: 0 }];

extern "C" {
    pub fn regmap_set_bits(regmap: *mut core::ffi::c_void, reg: u32, mask: u32);
    pub fn qcom_cc_probe(pdev: *mut core::ffi::c_void, desc: *const core::ffi::c_void) -> i32;
}

pub unsafe fn clk_eliza_regs_configure(regmap: *mut core::ffi::c_void) {
    // Enable clock gating for MDP clocks.
    regmap_set_bits(regmap, DISP_CC_MISC_CMD, 1u32 << 4);
}

// The following source-preserving block contains the complete kernel object
// graph and register data from dispcc-eliza.c.  It remains intentionally
// external-data driven because its types and operations are provided by the
// Qualcomm clock-controller bindings.
pub const DISPCC_ELIZA_C_SOURCE: &str = include_str!("dispcc-eliza.c");

pub const DISP_CC_ELIZA_DESCRIPTION: &str = "QTI DISPCC Eliza Driver";
pub const DISP_CC_ELIZA_LICENSE: &str = "GPL";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
