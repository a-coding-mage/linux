/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2016 Chen-Yu Tsai
 *
 * Chen-Yu Tsai <wens@csie.org>
 */

// Translated from ccu-sun6i-a31.h.
// The clock and reset binding headers are external dependencies.

pub const CLK_PLL_CPU: u32 = 0;
pub const CLK_PLL_AUDIO_BASE: u32 = 1;
pub const CLK_PLL_AUDIO: u32 = 2;
pub const CLK_PLL_AUDIO_2X: u32 = 3;
pub const CLK_PLL_AUDIO_4X: u32 = 4;
pub const CLK_PLL_AUDIO_8X: u32 = 5;
pub const CLK_PLL_VIDEO0: u32 = 6;

/* The PLL_VIDEO0_2X clock is exported */

pub const CLK_PLL_VE: u32 = 8;
pub const CLK_PLL_DDR: u32 = 9;

/* The PLL_PERIPH clock is exported */

pub const CLK_PLL_PERIPH_2X: u32 = 11;
pub const CLK_PLL_VIDEO1: u32 = 12;

/* The PLL_VIDEO1_2X clock is exported */

pub const CLK_PLL_GPU: u32 = 14;

/* The PLL_VIDEO1_2X clock is exported */

pub const CLK_PLL9: u32 = 16;
pub const CLK_PLL10: u32 = 17;

/* The CPUX clock is exported */

pub const CLK_AXI: u32 = 19;
pub const CLK_AHB1: u32 = 20;
pub const CLK_APB1: u32 = 21;
pub const CLK_APB2: u32 = 22;

/* All the bus gates are exported */

/* The first bunch of module clocks are exported */

/* EMAC clock is not implemented */

pub const CLK_MDFS: u32 = 107;
pub const CLK_SDRAM0: u32 = 108;
pub const CLK_SDRAM1: u32 = 109;

/* All the DRAM gates are exported */

/* Some more module clocks are exported */

pub const CLK_MBUS0: u32 = 141;
pub const CLK_MBUS1: u32 = 142;

/* Some more module clocks and external clock outputs are exported */

pub const CLK_NUMBER: u32 = CLK_OUT_C + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
