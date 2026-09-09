/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2018, The Linux Foundation. All rights reserved.
 */

/* VIDEO_CC clock registers */
pub const VIDEO_CC_APB_CLK: i32 = 0;
pub const VIDEO_CC_AT_CLK: i32 = 1;
pub const VIDEO_CC_QDSS_TRIG_CLK: i32 = 2;
pub const VIDEO_CC_QDSS_TSCTR_DIV8_CLK: i32 = 3;
pub const VIDEO_CC_VCODEC0_AXI_CLK: i32 = 4;
pub const VIDEO_CC_VCODEC0_CORE_CLK: i32 = 5;
pub const VIDEO_CC_VCODEC1_AXI_CLK: i32 = 6;
pub const VIDEO_CC_VCODEC1_CORE_CLK: i32 = 7;
pub const VIDEO_CC_VENUS_AHB_CLK: i32 = 8;
pub const VIDEO_CC_VENUS_CLK_SRC: i32 = 9;
pub const VIDEO_CC_VENUS_CTL_AXI_CLK: i32 = 10;
pub const VIDEO_CC_VENUS_CTL_CORE_CLK: i32 = 11;
pub const VIDEO_PLL0: i32 = 12;

/* VIDEO_CC Resets */
pub const VIDEO_CC_VENUS_BCR: i32 = 0;
pub const VIDEO_CC_VCODEC0_BCR: i32 = 1;
pub const VIDEO_CC_VCODEC1_BCR: i32 = 2;
pub const VIDEO_CC_INTERFACE_BCR: i32 = 3;

/* VIDEO_CC GDSCRs */
pub const VENUS_GDSC: i32 = 0;
pub const VCODEC0_GDSC: i32 = 1;
pub const VCODEC1_GDSC: i32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
