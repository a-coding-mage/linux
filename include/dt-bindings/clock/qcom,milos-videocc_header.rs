/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2023, Qualcomm Innovation Center, Inc. All rights reserved.
 * Copyright (c) 2025, Luca Weiss <luca.weiss@fairphone.com>
 */

/* VIDEO_CC clocks */
pub const VIDEO_CC_PLL0: u32 = 0;
pub const VIDEO_CC_AHB_CLK: u32 = 1;
pub const VIDEO_CC_AHB_CLK_SRC: u32 = 2;
pub const VIDEO_CC_MVS0_CLK: u32 = 3;
pub const VIDEO_CC_MVS0_CLK_SRC: u32 = 4;
pub const VIDEO_CC_MVS0_DIV_CLK_SRC: u32 = 5;
pub const VIDEO_CC_MVS0_SHIFT_CLK: u32 = 6;
pub const VIDEO_CC_MVS0C_CLK: u32 = 7;
pub const VIDEO_CC_MVS0C_DIV2_DIV_CLK_SRC: u32 = 8;
pub const VIDEO_CC_MVS0C_SHIFT_CLK: u32 = 9;
pub const VIDEO_CC_SLEEP_CLK: u32 = 10;
pub const VIDEO_CC_SLEEP_CLK_SRC: u32 = 11;
pub const VIDEO_CC_XO_CLK: u32 = 12;
pub const VIDEO_CC_XO_CLK_SRC: u32 = 13;

/* VIDEO_CC resets */
pub const VIDEO_CC_INTERFACE_BCR: u32 = 0;
pub const VIDEO_CC_MVS0_BCR: u32 = 1;
pub const VIDEO_CC_MVS0C_CLK_ARES: u32 = 2;
pub const VIDEO_CC_MVS0C_BCR: u32 = 3;

/* VIDEO_CC power domains */
pub const VIDEO_CC_MVS0_GDSC: u32 = 0;
pub const VIDEO_CC_MVS0C_GDSC: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
