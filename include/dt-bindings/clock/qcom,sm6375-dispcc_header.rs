/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2021, The Linux Foundation. All rights reserved.
 * Copyright (c) 2022, Linaro Limited
 */

/* Clocks */
pub const DISP_CC_PLL0: u32 = 0;
pub const DISP_CC_MDSS_AHB_CLK: u32 = 1;
pub const DISP_CC_MDSS_AHB_CLK_SRC: u32 = 2;
pub const DISP_CC_MDSS_BYTE0_CLK: u32 = 3;
pub const DISP_CC_MDSS_BYTE0_CLK_SRC: u32 = 4;
pub const DISP_CC_MDSS_BYTE0_DIV_CLK_SRC: u32 = 5;
pub const DISP_CC_MDSS_BYTE0_INTF_CLK: u32 = 6;
pub const DISP_CC_MDSS_ESC0_CLK: u32 = 7;
pub const DISP_CC_MDSS_ESC0_CLK_SRC: u32 = 8;
pub const DISP_CC_MDSS_MDP_CLK: u32 = 9;
pub const DISP_CC_MDSS_MDP_CLK_SRC: u32 = 10;
pub const DISP_CC_MDSS_MDP_LUT_CLK: u32 = 11;
pub const DISP_CC_MDSS_NON_GDSC_AHB_CLK: u32 = 12;
pub const DISP_CC_MDSS_PCLK0_CLK: u32 = 13;
pub const DISP_CC_MDSS_PCLK0_CLK_SRC: u32 = 14;
pub const DISP_CC_MDSS_ROT_CLK: u32 = 15;
pub const DISP_CC_MDSS_ROT_CLK_SRC: u32 = 16;
pub const DISP_CC_MDSS_RSCC_AHB_CLK: u32 = 17;
pub const DISP_CC_MDSS_RSCC_VSYNC_CLK: u32 = 18;
pub const DISP_CC_MDSS_VSYNC_CLK: u32 = 19;
pub const DISP_CC_MDSS_VSYNC_CLK_SRC: u32 = 20;
pub const DISP_CC_SLEEP_CLK: u32 = 21;
pub const DISP_CC_XO_CLK: u32 = 22;

/* Resets */
pub const DISP_CC_MDSS_CORE_BCR: u32 = 0;
pub const DISP_CC_MDSS_RSCC_BCR: u32 = 1;

/* GDSCs */
pub const MDSS_GDSC: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
