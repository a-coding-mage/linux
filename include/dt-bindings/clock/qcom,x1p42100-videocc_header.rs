/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

/* VIDEO_CC clocks */
pub const VIDEO_CC_MVS0_CLK: u32 = 0;
pub const VIDEO_CC_MVS0_CLK_SRC: u32 = 1;
pub const VIDEO_CC_MVS0_DIV_CLK_SRC: u32 = 2;
pub const VIDEO_CC_MVS0C_CLK: u32 = 3;
pub const VIDEO_CC_MVS0C_DIV2_DIV_CLK_SRC: u32 = 4;
pub const VIDEO_CC_MVS1_CLK: u32 = 5;
pub const VIDEO_CC_MVS1_CLK_SRC: u32 = 6;
pub const VIDEO_CC_MVS1_DIV_CLK_SRC: u32 = 7;
pub const VIDEO_CC_MVS1C_CLK: u32 = 8;
pub const VIDEO_CC_MVS1C_DIV2_DIV_CLK_SRC: u32 = 9;
pub const VIDEO_CC_PLL0: u32 = 10;
pub const VIDEO_CC_PLL1: u32 = 11;
pub const VIDEO_CC_MVS0_SHIFT_CLK: u32 = 12;
pub const VIDEO_CC_MVS0C_SHIFT_CLK: u32 = 13;
pub const VIDEO_CC_MVS1_SHIFT_CLK: u32 = 14;
pub const VIDEO_CC_MVS1C_SHIFT_CLK: u32 = 15;
pub const VIDEO_CC_XO_CLK_SRC: u32 = 16;
pub const VIDEO_CC_MVS0_BSE_CLK: u32 = 17;
pub const VIDEO_CC_MVS0_BSE_CLK_SRC: u32 = 18;
pub const VIDEO_CC_MVS0_BSE_DIV4_DIV_CLK_SRC: u32 = 19;

/* VIDEO_CC power domains */
pub const VIDEO_CC_MVS0C_GDSC: u32 = 0;
pub const VIDEO_CC_MVS0_GDSC: u32 = 1;
pub const VIDEO_CC_MVS1C_GDSC: u32 = 2;
pub const VIDEO_CC_MVS1_GDSC: u32 = 3;

/* VIDEO_CC resets */
pub const CVP_VIDEO_CC_INTERFACE_BCR: u32 = 0;
pub const CVP_VIDEO_CC_MVS0_BCR: u32 = 1;
pub const CVP_VIDEO_CC_MVS0C_BCR: u32 = 2;
pub const CVP_VIDEO_CC_MVS1_BCR: u32 = 3;
pub const CVP_VIDEO_CC_MVS1C_BCR: u32 = 4;
pub const VIDEO_CC_MVS0C_CLK_ARES: u32 = 5;
pub const VIDEO_CC_MVS1C_CLK_ARES: u32 = 6;
pub const VIDEO_CC_XO_CLK_ARES: u32 = 7;
pub const VIDEO_CC_MVS0_BSE_BCR: u32 = 8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
