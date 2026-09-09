/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2016 Chen-Yu Tsai
 *
 * Chen-Yu Tsai <wens@csie.org>
 */

// Dependencies supplied by the corresponding clock and reset binding headers
// are intentionally not implemented here.

pub const CLK_PLL_C0CPUX: u32 = 0;
pub const CLK_PLL_C1CPUX: u32 = 1;
pub const CLK_PLL_AUDIO: u32 = 2;
pub const CLK_PLL_VIDEO0: u32 = 3;
pub const CLK_PLL_VE: u32 = 4;
pub const CLK_PLL_DDR: u32 = 5;

/* pll-periph is exported to the PRCM block */

pub const CLK_PLL_GPU: u32 = 7;
pub const CLK_PLL_HSIC: u32 = 8;

/* pll-de is exported for the display engine */

pub const CLK_PLL_VIDEO1: u32 = 10;

/* The CPUX clocks are exported */

pub const CLK_AXI0: u32 = 13;
pub const CLK_AXI1: u32 = 14;
pub const CLK_AHB1: u32 = 15;
pub const CLK_AHB2: u32 = 16;
pub const CLK_APB1: u32 = 17;
pub const CLK_APB2: u32 = 18;

/* bus gates exported */

pub const CLK_CCI400: u32 = 58;

/* module and usb clocks exported */

pub const CLK_DRAM: u32 = 82;

/* dram gates and more module clocks exported */

pub const CLK_MBUS: u32 = 95;

/* more module clocks exported */

// CLK_GPU_HYD is supplied by the corresponding clock binding header.
pub const CLK_NUMBER: u32 = CLK_GPU_HYD + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
