/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2021, The Linux Foundation. All rights reserved.
 */

/* VIDEO_CC clocks */
pub const VIDEO_PLL0: u32 = 0;
pub const VIDEO_CC_IRIS_AHB_CLK: u32 = 1;
pub const VIDEO_CC_IRIS_CLK_SRC: u32 = 2;
pub const VIDEO_CC_MVS0_AXI_CLK: u32 = 3;
pub const VIDEO_CC_MVS0_CORE_CLK: u32 = 4;
pub const VIDEO_CC_MVSC_CORE_CLK: u32 = 5;
pub const VIDEO_CC_MVSC_CTL_AXI_CLK: u32 = 6;
pub const VIDEO_CC_SLEEP_CLK: u32 = 7;
pub const VIDEO_CC_SLEEP_CLK_SRC: u32 = 8;
pub const VIDEO_CC_VENUS_AHB_CLK: u32 = 9;
pub const VIDEO_CC_XO_CLK: u32 = 10;
pub const VIDEO_CC_XO_CLK_SRC: u32 = 11;

/* VIDEO_CC power domains */
pub const MVS0_GDSC: u32 = 0;
pub const MVSC_GDSC: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
