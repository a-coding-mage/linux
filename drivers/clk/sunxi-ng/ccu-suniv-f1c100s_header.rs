/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright 2017 Icenowy Zheng <icenowy@aosc.io>
 *
 */

// Dependencies supplied by the corresponding dt-bindings headers:
// <dt-bindings/clock/suniv-ccu-f1c100s.h>
// <dt-bindings/reset/suniv-ccu-f1c100s.h>

pub const CLK_PLL_CPU: i32 = 0;
pub const CLK_PLL_AUDIO_BASE: i32 = 1;
pub const CLK_PLL_AUDIO: i32 = 2;
pub const CLK_PLL_AUDIO_2X: i32 = 3;
pub const CLK_PLL_AUDIO_4X: i32 = 4;
pub const CLK_PLL_AUDIO_8X: i32 = 5;
pub const CLK_PLL_VIDEO: i32 = 6;
pub const CLK_PLL_VIDEO_2X: i32 = 7;
pub const CLK_PLL_VE: i32 = 8;
pub const CLK_PLL_DDR0: i32 = 9;
pub const CLK_PLL_PERIPH: i32 = 10;

/* CPU clock is exported */

pub const CLK_AHB: i32 = 12;
pub const CLK_APB: i32 = 13;

/* All bus gates, DRAM gates and mod clocks are exported */

// CLK_IR is supplied by the corresponding clock dt-bindings header.
pub const CLK_NUMBER: i32 = CLK_IR + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
