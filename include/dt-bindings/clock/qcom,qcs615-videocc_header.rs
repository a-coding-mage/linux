/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2024, Qualcomm Innovation Center, Inc. All rights reserved.
 */

/* VIDEO_CC clocks */
pub const VIDEO_CC_SLEEP_CLK: u32 = 0;
pub const VIDEO_CC_SLEEP_CLK_SRC: u32 = 1;
pub const VIDEO_CC_VCODEC0_AXI_CLK: u32 = 2;
pub const VIDEO_CC_VCODEC0_CORE_CLK: u32 = 3;
pub const VIDEO_CC_VENUS_AHB_CLK: u32 = 4;
pub const VIDEO_CC_VENUS_CLK_SRC: u32 = 5;
pub const VIDEO_CC_VENUS_CTL_AXI_CLK: u32 = 6;
pub const VIDEO_CC_VENUS_CTL_CORE_CLK: u32 = 7;
pub const VIDEO_CC_XO_CLK: u32 = 8;
pub const VIDEO_PLL0: u32 = 9;

/* VIDEO_CC power domains */
pub const VCODEC0_GDSC: u32 = 0;
pub const VENUS_GDSC: u32 = 1;

/* VIDEO_CC resets */
pub const VIDEO_CC_INTERFACE_BCR: u32 = 0;
pub const VIDEO_CC_VCODEC0_BCR: u32 = 1;
pub const VIDEO_CC_VENUS_BCR: u32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
