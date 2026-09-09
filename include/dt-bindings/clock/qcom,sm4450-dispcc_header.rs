/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2024, Qualcomm Innovation Center, Inc. All rights reserved.
 */

// Header guard: _DT_BINDINGS_CLK_QCOM_DISP_CC_SM4450_H

/* DISP_CC clocks */
pub const DISP_CC_MDSS_AHB1_CLK: u32 = 0;
pub const DISP_CC_MDSS_AHB_CLK: u32 = 1;
pub const DISP_CC_MDSS_AHB_CLK_SRC: u32 = 2;
pub const DISP_CC_MDSS_BYTE0_CLK: u32 = 3;
pub const DISP_CC_MDSS_BYTE0_CLK_SRC: u32 = 4;
pub const DISP_CC_MDSS_BYTE0_DIV_CLK_SRC: u32 = 5;
pub const DISP_CC_MDSS_BYTE0_INTF_CLK: u32 = 6;
pub const DISP_CC_MDSS_ESC0_CLK: u32 = 7;
pub const DISP_CC_MDSS_ESC0_CLK_SRC: u32 = 8;
pub const DISP_CC_MDSS_MDP1_CLK: u32 = 9;
pub const DISP_CC_MDSS_MDP_CLK: u32 = 10;
pub const DISP_CC_MDSS_MDP_CLK_SRC: u32 = 11;
pub const DISP_CC_MDSS_MDP_LUT1_CLK: u32 = 12;
pub const DISP_CC_MDSS_MDP_LUT_CLK: u32 = 13;
pub const DISP_CC_MDSS_NON_GDSC_AHB_CLK: u32 = 14;
pub const DISP_CC_MDSS_PCLK0_CLK: u32 = 15;
pub const DISP_CC_MDSS_PCLK0_CLK_SRC: u32 = 16;
pub const DISP_CC_MDSS_ROT1_CLK: u32 = 17;
pub const DISP_CC_MDSS_ROT_CLK: u32 = 18;
pub const DISP_CC_MDSS_ROT_CLK_SRC: u32 = 19;
pub const DISP_CC_MDSS_RSCC_AHB_CLK: u32 = 20;
pub const DISP_CC_MDSS_RSCC_VSYNC_CLK: u32 = 21;
pub const DISP_CC_MDSS_VSYNC1_CLK: u32 = 22;
pub const DISP_CC_MDSS_VSYNC_CLK: u32 = 23;
pub const DISP_CC_MDSS_VSYNC_CLK_SRC: u32 = 24;
pub const DISP_CC_PLL0: u32 = 25;
pub const DISP_CC_PLL1: u32 = 26;
pub const DISP_CC_SLEEP_CLK: u32 = 27;
pub const DISP_CC_SLEEP_CLK_SRC: u32 = 28;
pub const DISP_CC_XO_CLK: u32 = 29;
pub const DISP_CC_XO_CLK_SRC: u32 = 30;

/* DISP_CC power domains */
pub const DISP_CC_MDSS_CORE_GDSC: u32 = 0;
pub const DISP_CC_MDSS_CORE_INT2_GDSC: u32 = 1;

/* DISP_CC resets */
pub const DISP_CC_MDSS_CORE_BCR: u32 = 0;
pub const DISP_CC_MDSS_CORE_INT2_BCR: u32 = 1;
pub const DISP_CC_MDSS_RSCC_BCR: u32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
