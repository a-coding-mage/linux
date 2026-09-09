/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2017 Icenowy Zheng <icenowy@aosc.io>
 */

// Dependency declarations from:
// <dt-bindings/clock/sun8i-r40-ccu.h>
// <dt-bindings/reset/sun8i-r40-ccu.h>

pub const CLK_OSC_12M: u32 = 0;
pub const CLK_PLL_CPU: u32 = 1;
pub const CLK_PLL_AUDIO_BASE: u32 = 2;
pub const CLK_PLL_AUDIO: u32 = 3;
pub const CLK_PLL_AUDIO_2X: u32 = 4;
pub const CLK_PLL_AUDIO_4X: u32 = 5;
pub const CLK_PLL_AUDIO_8X: u32 = 6;

/* PLL_VIDEO0 is exported */

pub const CLK_PLL_VIDEO0_2X: u32 = 8;
pub const CLK_PLL_VE: u32 = 9;
pub const CLK_PLL_DDR0: u32 = 10;
pub const CLK_PLL_PERIPH0: u32 = 11;
pub const CLK_PLL_PERIPH0_SATA: u32 = 12;
pub const CLK_PLL_PERIPH0_2X: u32 = 13;
pub const CLK_PLL_PERIPH1: u32 = 14;
pub const CLK_PLL_PERIPH1_2X: u32 = 15;

/* PLL_VIDEO1 is exported */

pub const CLK_PLL_VIDEO1_2X: u32 = 17;
pub const CLK_PLL_SATA: u32 = 18;
pub const CLK_PLL_SATA_OUT: u32 = 19;
pub const CLK_PLL_GPU: u32 = 20;
pub const CLK_PLL_MIPI: u32 = 21;
pub const CLK_PLL_DE: u32 = 22;
pub const CLK_PLL_DDR1: u32 = 23;

/* The CPU clock is exported */

pub const CLK_AXI: u32 = 25;
pub const CLK_AHB1: u32 = 26;
pub const CLK_APB1: u32 = 27;
pub const CLK_APB2: u32 = 28;

/* All the bus gates are exported */

/* The first bunch of module clocks are exported */

pub const CLK_DRAM: u32 = 132;

/* All the DRAM gates are exported */

/* Some more module clocks are exported */

// CLK_OUTB is provided by the included clock binding header.
pub const CLK_NUMBER: u32 = CLK_OUTB + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
