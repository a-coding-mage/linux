/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2023, Qualcomm Innovation Center, Inc. All rights reserved.
 * Copyright (c) 2025, Luca Weiss <luca.weiss@fairphone.com>
 */

/* DISP_CC clocks */
pub const DISP_CC_PLL0: i32 = 0;
pub const DISP_CC_MDSS_ACCU_CLK: i32 = 1;
pub const DISP_CC_MDSS_AHB1_CLK: i32 = 2;
pub const DISP_CC_MDSS_AHB_CLK: i32 = 3;
pub const DISP_CC_MDSS_AHB_CLK_SRC: i32 = 4;
pub const DISP_CC_MDSS_BYTE0_CLK: i32 = 5;
pub const DISP_CC_MDSS_BYTE0_CLK_SRC: i32 = 6;
pub const DISP_CC_MDSS_BYTE0_DIV_CLK_SRC: i32 = 7;
pub const DISP_CC_MDSS_BYTE0_INTF_CLK: i32 = 8;
pub const DISP_CC_MDSS_DPTX0_AUX_CLK: i32 = 9;
pub const DISP_CC_MDSS_DPTX0_AUX_CLK_SRC: i32 = 10;
pub const DISP_CC_MDSS_DPTX0_CRYPTO_CLK: i32 = 11;
pub const DISP_CC_MDSS_DPTX0_LINK_CLK: i32 = 12;
pub const DISP_CC_MDSS_DPTX0_LINK_CLK_SRC: i32 = 13;
pub const DISP_CC_MDSS_DPTX0_LINK_DIV_CLK_SRC: i32 = 14;
pub const DISP_CC_MDSS_DPTX0_LINK_INTF_CLK: i32 = 15;
pub const DISP_CC_MDSS_DPTX0_PIXEL0_CLK: i32 = 16;
pub const DISP_CC_MDSS_DPTX0_PIXEL0_CLK_SRC: i32 = 17;
pub const DISP_CC_MDSS_DPTX0_PIXEL1_CLK: i32 = 18;
pub const DISP_CC_MDSS_DPTX0_PIXEL1_CLK_SRC: i32 = 19;
pub const DISP_CC_MDSS_DPTX0_USB_ROUTER_LINK_INTF_CLK: i32 = 20;
pub const DISP_CC_MDSS_ESC0_CLK: i32 = 21;
pub const DISP_CC_MDSS_ESC0_CLK_SRC: i32 = 22;
pub const DISP_CC_MDSS_MDP1_CLK: i32 = 23;
pub const DISP_CC_MDSS_MDP_CLK: i32 = 24;
pub const DISP_CC_MDSS_MDP_CLK_SRC: i32 = 25;
pub const DISP_CC_MDSS_MDP_LUT1_CLK: i32 = 26;
pub const DISP_CC_MDSS_MDP_LUT_CLK: i32 = 27;
pub const DISP_CC_MDSS_NON_GDSC_AHB_CLK: i32 = 28;
pub const DISP_CC_MDSS_PCLK0_CLK: i32 = 29;
pub const DISP_CC_MDSS_PCLK0_CLK_SRC: i32 = 30;
pub const DISP_CC_MDSS_RSCC_AHB_CLK: i32 = 31;
pub const DISP_CC_MDSS_RSCC_VSYNC_CLK: i32 = 32;
pub const DISP_CC_MDSS_VSYNC1_CLK: i32 = 33;
pub const DISP_CC_MDSS_VSYNC_CLK: i32 = 34;
pub const DISP_CC_MDSS_VSYNC_CLK_SRC: i32 = 35;
pub const DISP_CC_SLEEP_CLK: i32 = 36;
pub const DISP_CC_SLEEP_CLK_SRC: i32 = 37;
pub const DISP_CC_XO_CLK: i32 = 38;
pub const DISP_CC_XO_CLK_SRC: i32 = 39;

/* DISP_CC resets */
pub const DISP_CC_MDSS_CORE_BCR: i32 = 0;
pub const DISP_CC_MDSS_CORE_INT2_BCR: i32 = 1;
pub const DISP_CC_MDSS_RSCC_BCR: i32 = 2;

/* DISP_CC power domains */
pub const DISP_CC_MDSS_CORE_GDSC: i32 = 0;
pub const DISP_CC_MDSS_CORE_INT2_GDSC: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
