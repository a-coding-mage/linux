/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2016 Icenowy Zheng <icenowy@aosc.xyz>
 *
 * Based on ccu-sun8i-h3.h, which is:
 * Copyright (c) 2016 Maxime Ripard <maxime.ripard@free-electrons.com>
 */

// Dependency declarations from <dt-bindings/clock/sun8i-v3s-ccu.h> and
// <dt-bindings/reset/sun8i-v3s-ccu.h> are supplied by other files.

pub const CLK_PLL_CPU: u32 = 0;
pub const CLK_PLL_AUDIO_BASE: u32 = 1;
pub const CLK_PLL_AUDIO: u32 = 2;
pub const CLK_PLL_AUDIO_2X: u32 = 3;
pub const CLK_PLL_AUDIO_4X: u32 = 4;
pub const CLK_PLL_AUDIO_8X: u32 = 5;
pub const CLK_PLL_VIDEO: u32 = 6;
pub const CLK_PLL_VE: u32 = 7;
pub const CLK_PLL_DDR0: u32 = 8;
pub const CLK_PLL_PERIPH0: u32 = 9;
pub const CLK_PLL_PERIPH0_2X: u32 = 10;
pub const CLK_PLL_ISP: u32 = 11;
pub const CLK_PLL_PERIPH1: u32 = 12;
// Reserve one number for not implemented and not used PLL_DDR1

// The CPU clock is exported

pub const CLK_AXI: u32 = 15;
pub const CLK_AHB1: u32 = 16;
pub const CLK_APB1: u32 = 17;
pub const CLK_APB2: u32 = 18;
pub const CLK_AHB2: u32 = 19;

// All the bus gates are exported

// The first bunch of module clocks are exported

pub const CLK_DRAM: u32 = 58;

// All the DRAM gates are exported

// Some more module clocks are exported

pub const CLK_MBUS: u32 = 72;

// And the GPU module clock is exported

pub const CLK_PLL_DDR1: u32 = 74;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
