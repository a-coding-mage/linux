/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2016 Maxime Ripard
 *
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 */

// Dependencies supplied by the device-tree clock and reset bindings.

/* The HOSC is exported */
pub const CLK_PLL_CORE: u32 = 2;
pub const CLK_PLL_AUDIO_BASE: u32 = 3;
pub const CLK_PLL_AUDIO: u32 = 4;
pub const CLK_PLL_AUDIO_2X: u32 = 5;
pub const CLK_PLL_AUDIO_4X: u32 = 6;
pub const CLK_PLL_AUDIO_8X: u32 = 7;
pub const CLK_PLL_VIDEO0: u32 = 8;

/* The PLL_VIDEO0_2X is exported for HDMI */

pub const CLK_PLL_VE: u32 = 10;
pub const CLK_PLL_DDR_BASE: u32 = 11;
pub const CLK_PLL_DDR: u32 = 12;
pub const CLK_PLL_DDR_OTHER: u32 = 13;
pub const CLK_PLL_PERIPH: u32 = 14;
pub const CLK_PLL_VIDEO1: u32 = 15;

/* The PLL_VIDEO1_2X is exported for HDMI */
/* The CPU clock is exported */

pub const CLK_AXI: u32 = 18;
pub const CLK_AHB: u32 = 19;
pub const CLK_APB0: u32 = 20;
pub const CLK_APB1: u32 = 21;
pub const CLK_DRAM_AXI: u32 = 22;

/* AHB gates are exported */
/* APB0 gates are exported */
/* APB1 gates are exported */
/* Modules clocks are exported */
/* USB clocks are exported */
/* GPS clock is exported */
/* DRAM gates are exported */
/* More display modules clocks are exported */

pub const CLK_TCON_CH1_SCLK: u32 = 91;

/* The rest of the module clocks are exported */

// CLK_IEP is supplied by the sun5i-ccu clock bindings.
pub const CLK_NUMBER: u32 = CLK_IEP + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
