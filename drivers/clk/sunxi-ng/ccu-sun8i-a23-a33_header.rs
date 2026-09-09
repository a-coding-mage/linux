/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2016 Maxime Ripard
 *
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 */

// C header guard: _CCU_SUN8I_A23_A33_H_
// External dependencies:
// #include <dt-bindings/clock/sun8i-a23-a33-ccu.h>
// #include <dt-bindings/reset/sun8i-a23-a33-ccu.h>

pub const CLK_PLL_CPUX: u32 = 0;
pub const CLK_PLL_AUDIO_BASE: u32 = 1;
pub const CLK_PLL_AUDIO: u32 = 2;
pub const CLK_PLL_AUDIO_2X: u32 = 3;
pub const CLK_PLL_AUDIO_4X: u32 = 4;
pub const CLK_PLL_AUDIO_8X: u32 = 5;
pub const CLK_PLL_VIDEO: u32 = 6;
pub const CLK_PLL_VIDEO_2X: u32 = 7;
pub const CLK_PLL_VE: u32 = 8;
pub const CLK_PLL_DDR0: u32 = 9;
pub const CLK_PLL_PERIPH: u32 = 10;
pub const CLK_PLL_PERIPH_2X: u32 = 11;
pub const CLK_PLL_GPU: u32 = 12;

/* The PLL MIPI clock is exported */

pub const CLK_PLL_HSIC: u32 = 14;
pub const CLK_PLL_DE: u32 = 15;
pub const CLK_PLL_DDR1: u32 = 16;
pub const CLK_PLL_DDR: u32 = 17;

/* The CPUX clock is exported */

pub const CLK_AXI: u32 = 19;
pub const CLK_AHB1: u32 = 20;
pub const CLK_APB1: u32 = 21;
pub const CLK_APB2: u32 = 22;

/* All the bus gates are exported */

/* The first part of the mod clocks is exported */

pub const CLK_DRAM: u32 = 79;

/* Some more module clocks are exported */

pub const CLK_MBUS: u32 = 95;

/* And the last module clocks are exported */

pub const CLK_NUMBER: u32 = CLK_ATS + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
