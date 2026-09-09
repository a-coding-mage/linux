/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2018, The Linux Foundation. All rights reserved.
 * Copyright (c) 2024, Danila Tikhonov <danila@jiaxyga.com>
 * Copyright (c) 2024, David Wronek <david@mainlining.org>
 */

/* DISPCC clock registers */
pub const DISPCC_PLL0: i32 = 0;
pub const DISPCC_MDSS_AHB_CLK: i32 = 1;
pub const DISPCC_MDSS_AHB_CLK_SRC: i32 = 2;
pub const DISPCC_MDSS_BYTE0_CLK: i32 = 3;
pub const DISPCC_MDSS_BYTE0_CLK_SRC: i32 = 4;
pub const DISPCC_MDSS_BYTE0_DIV_CLK_SRC: i32 = 5;
pub const DISPCC_MDSS_BYTE0_INTF_CLK: i32 = 6;
pub const DISPCC_MDSS_BYTE1_CLK: i32 = 7;
pub const DISPCC_MDSS_BYTE1_CLK_SRC: i32 = 8;
pub const DISPCC_MDSS_BYTE1_DIV_CLK_SRC: i32 = 9;
pub const DISPCC_MDSS_BYTE1_INTF_CLK: i32 = 10;
pub const DISPCC_MDSS_DP_AUX_CLK: i32 = 11;
pub const DISPCC_MDSS_DP_AUX_CLK_SRC: i32 = 12;
pub const DISPCC_MDSS_DP_CRYPTO_CLK: i32 = 13;
pub const DISPCC_MDSS_DP_CRYPTO_CLK_SRC: i32 = 14;
pub const DISPCC_MDSS_DP_LINK_CLK: i32 = 15;
pub const DISPCC_MDSS_DP_LINK_CLK_SRC: i32 = 16;
pub const DISPCC_MDSS_DP_LINK_INTF_CLK: i32 = 17;
pub const DISPCC_MDSS_DP_PIXEL1_CLK: i32 = 18;
pub const DISPCC_MDSS_DP_PIXEL1_CLK_SRC: i32 = 19;
pub const DISPCC_MDSS_DP_PIXEL_CLK: i32 = 20;
pub const DISPCC_MDSS_DP_PIXEL_CLK_SRC: i32 = 21;
pub const DISPCC_MDSS_ESC0_CLK: i32 = 22;
pub const DISPCC_MDSS_ESC0_CLK_SRC: i32 = 23;
pub const DISPCC_MDSS_ESC1_CLK: i32 = 24;
pub const DISPCC_MDSS_ESC1_CLK_SRC: i32 = 25;
pub const DISPCC_MDSS_MDP_CLK: i32 = 26;
pub const DISPCC_MDSS_MDP_CLK_SRC: i32 = 27;
pub const DISPCC_MDSS_MDP_LUT_CLK: i32 = 28;
pub const DISPCC_MDSS_NON_GDSC_AHB_CLK: i32 = 29;
pub const DISPCC_MDSS_PCLK0_CLK: i32 = 30;
pub const DISPCC_MDSS_PCLK0_CLK_SRC: i32 = 31;
pub const DISPCC_MDSS_PCLK1_CLK: i32 = 32;
pub const DISPCC_MDSS_PCLK1_CLK_SRC: i32 = 33;
pub const DISPCC_MDSS_ROT_CLK: i32 = 34;
pub const DISPCC_MDSS_ROT_CLK_SRC: i32 = 35;
pub const DISPCC_MDSS_RSCC_AHB_CLK: i32 = 36;
pub const DISPCC_MDSS_RSCC_VSYNC_CLK: i32 = 37;
pub const DISPCC_MDSS_VSYNC_CLK: i32 = 38;
pub const DISPCC_MDSS_VSYNC_CLK_SRC: i32 = 39;
pub const DISPCC_XO_CLK_SRC: i32 = 40;
pub const DISPCC_SLEEP_CLK: i32 = 41;
pub const DISPCC_SLEEP_CLK_SRC: i32 = 42;

/* DISPCC resets */
pub const DISPCC_MDSS_CORE_BCR: i32 = 0;

/* DISPCC GDSCR */
pub const MDSS_GDSC: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
