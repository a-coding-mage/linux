/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

/* VIDEO_CC clocks */
pub const VIDEO_CC_AHB_CLK: u32 = 0;
pub const VIDEO_CC_AHB_CLK_SRC: u32 = 1;
pub const VIDEO_CC_CX_AXI0_CLK: u32 = 2;
pub const VIDEO_CC_CX_DBGCH_XO_CLK: u32 = 3;
pub const VIDEO_CC_CX_XO_CLK: u32 = 4;
pub const VIDEO_CC_DBGCH_XO_CLK: u32 = 5;
pub const VIDEO_CC_MVS0_CLK: u32 = 6;
pub const VIDEO_CC_MVS0_CLK_SRC: u32 = 7;
pub const VIDEO_CC_MVS0_SHIFT_CLK: u32 = 8;
pub const VIDEO_CC_MVS0_VPP0_CLK: u32 = 9;
pub const VIDEO_CC_MVS0_VPP0_VPP1_GATING_CLK: u32 = 10;
pub const VIDEO_CC_MVS0_VPP1_CLK: u32 = 11;
pub const VIDEO_CC_MVS0A_CLK: u32 = 12;
pub const VIDEO_CC_MVS0A_CLK_SRC: u32 = 13;
pub const VIDEO_CC_MVS0B_CLK: u32 = 14;
pub const VIDEO_CC_MVS0B_CLK_SRC: u32 = 15;
pub const VIDEO_CC_MVS0C_CLK: u32 = 16;
pub const VIDEO_CC_MVS0C_CLK_SRC: u32 = 17;
pub const VIDEO_CC_MVS0C_CTL_FREERUN_CLK: u32 = 18;
pub const VIDEO_CC_MVS0C_DEBUG_CLK: u32 = 19;
pub const VIDEO_CC_MVS0C_FREERUN_CLK: u32 = 20;
pub const VIDEO_CC_MVS0C_SHIFT_CLK: u32 = 21;
pub const VIDEO_CC_PLL0: u32 = 22;
pub const VIDEO_CC_PLL0_OUT_EVEN: u32 = 23;
pub const VIDEO_CC_PLL1: u32 = 24;
pub const VIDEO_CC_PLL2: u32 = 25;
pub const VIDEO_CC_PLL3: u32 = 26;
pub const VIDEO_CC_SLEEP_CLK: u32 = 27;
pub const VIDEO_CC_XO_CLK: u32 = 28;
pub const VIDEO_CC_XO_CLK_SRC: u32 = 29;

/* VIDEO_CC power domains */
pub const VIDEO_CC_AXI0_CX_INT_GDSC: u32 = 0;
pub const VIDEO_CC_MM_INT_GDSC: u32 = 1;
pub const VIDEO_CC_MVS0_GDSC: u32 = 2;
pub const VIDEO_CC_MVS0_VPP0_GDSC: u32 = 3;
pub const VIDEO_CC_MVS0_VPP1_GDSC: u32 = 4;
pub const VIDEO_CC_MVS0A_GDSC: u32 = 5;
pub const VIDEO_CC_MVS0C_GDSC: u32 = 6;

/* VIDEO_CC resets */
pub const VIDEO_CC_AXI0_CX_INT_BCR: u32 = 0;
pub const VIDEO_CC_INTERFACE_BCR: u32 = 1;
pub const VIDEO_CC_MM_INT_BCR: u32 = 2;
pub const VIDEO_CC_MVS0_BCR: u32 = 3;
pub const VIDEO_CC_MVS0_VPP0_BCR: u32 = 4;
pub const VIDEO_CC_MVS0_VPP1_BCR: u32 = 5;
pub const VIDEO_CC_MVS0A_BCR: u32 = 6;
pub const VIDEO_CC_MVS0C_CLK_ARES: u32 = 7;
pub const VIDEO_CC_MVS0C_BCR: u32 = 8;
pub const VIDEO_CC_MVS0C_CTL_FREERUN_CLK_ARES: u32 = 9;
pub const VIDEO_CC_MVS0C_FREERUN_CLK_ARES: u32 = 10;
pub const VIDEO_CC_XO_CLK_ARES: u32 = 11;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
