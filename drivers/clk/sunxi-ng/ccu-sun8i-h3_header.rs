/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2016 Maxime Ripard
 *
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 */

// Dependencies supplied by the corresponding device-tree clock and reset
// bindings are intentionally left external to this translation.

pub const CLK_PLL_CPUX: i32 = 0;
pub const CLK_PLL_AUDIO_BASE: i32 = 1;
pub const CLK_PLL_AUDIO: i32 = 2;
pub const CLK_PLL_AUDIO_2X: i32 = 3;
pub const CLK_PLL_AUDIO_4X: i32 = 4;
pub const CLK_PLL_AUDIO_8X: i32 = 5;

/* PLL_VIDEO is exported */

pub const CLK_PLL_VE: i32 = 7;
pub const CLK_PLL_DDR: i32 = 8;

/* PLL_PERIPH0 exported for PRCM */

pub const CLK_PLL_PERIPH0_2X: i32 = 10;
pub const CLK_PLL_GPU: i32 = 11;
pub const CLK_PLL_PERIPH1: i32 = 12;
pub const CLK_PLL_DE: i32 = 13;

/* The CPUX clock is exported */

pub const CLK_AXI: i32 = 15;
pub const CLK_AHB1: i32 = 16;
pub const CLK_APB1: i32 = 17;
pub const CLK_APB2: i32 = 18;
pub const CLK_AHB2: i32 = 19;

/* All the bus gates are exported */

/* The first bunch of module clocks are exported */

/* All the DRAM gates are exported */

/* Some more module clocks are exported */

pub const CLK_NUMBER_H3: i32 = CLK_GPU + 1;
pub const CLK_NUMBER_H5: i32 = CLK_BUS_SCR1 + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
