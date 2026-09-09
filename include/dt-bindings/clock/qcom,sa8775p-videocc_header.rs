/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2024, Qualcomm Innovation Center, Inc. All rights reserved.
 */

// VIDEO_CC clocks
pub const VIDEO_CC_AHB_CLK: u32 = 0;
pub const VIDEO_CC_AHB_CLK_SRC: u32 = 1;
pub const VIDEO_CC_MVS0_CLK: u32 = 2;
pub const VIDEO_CC_MVS0_CLK_SRC: u32 = 3;
pub const VIDEO_CC_MVS0_DIV_CLK_SRC: u32 = 4;
pub const VIDEO_CC_MVS0C_CLK: u32 = 5;
pub const VIDEO_CC_MVS0C_DIV2_DIV_CLK_SRC: u32 = 6;
pub const VIDEO_CC_MVS1_CLK: u32 = 7;
pub const VIDEO_CC_MVS1_CLK_SRC: u32 = 8;
pub const VIDEO_CC_MVS1_DIV_CLK_SRC: u32 = 9;
pub const VIDEO_CC_MVS1C_CLK: u32 = 10;
pub const VIDEO_CC_MVS1C_DIV2_DIV_CLK_SRC: u32 = 11;
pub const VIDEO_CC_PLL_LOCK_MONITOR_CLK: u32 = 12;
pub const VIDEO_CC_SLEEP_CLK: u32 = 13;
pub const VIDEO_CC_SLEEP_CLK_SRC: u32 = 14;
pub const VIDEO_CC_SM_DIV_CLK_SRC: u32 = 15;
pub const VIDEO_CC_SM_OBS_CLK: u32 = 16;
pub const VIDEO_CC_XO_CLK: u32 = 17;
pub const VIDEO_CC_XO_CLK_SRC: u32 = 18;
pub const VIDEO_PLL0: u32 = 19;
pub const VIDEO_PLL1: u32 = 20;

// VIDEO_CC power domains
pub const VIDEO_CC_MVS0C_GDSC: u32 = 0;
pub const VIDEO_CC_MVS0_GDSC: u32 = 1;
pub const VIDEO_CC_MVS1C_GDSC: u32 = 2;
pub const VIDEO_CC_MVS1_GDSC: u32 = 3;

// VIDEO_CC resets
pub const VIDEO_CC_INTERFACE_BCR: u32 = 0;
pub const VIDEO_CC_MVS0_BCR: u32 = 1;
pub const VIDEO_CC_MVS0C_CLK_ARES: u32 = 2;
pub const VIDEO_CC_MVS0C_BCR: u32 = 3;
pub const VIDEO_CC_MVS1_BCR: u32 = 4;
pub const VIDEO_CC_MVS1C_CLK_ARES: u32 = 5;
pub const VIDEO_CC_MVS1C_BCR: u32 = 6;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
