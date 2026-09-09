/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2016 Chen-Yu Tsai
 *
 * Chen-Yu Tsai <wens@csie.org>
 */

// Dependency intent from the original header:
// #include <dt-bindings/clock/sun9i-a80-ccu.h>
// #include <dt-bindings/reset/sun9i-a80-ccu.h>

pub const CLK_PLL_C0CPUX: u32 = 0;
pub const CLK_PLL_C1CPUX: u32 = 1;

/* pll-audio and pll-periph0 are exported to the PRCM block */

pub const CLK_PLL_VE: u32 = 4;
pub const CLK_PLL_DDR: u32 = 5;
pub const CLK_PLL_VIDEO0: u32 = 6;
pub const CLK_PLL_VIDEO1: u32 = 7;
pub const CLK_PLL_GPU: u32 = 8;
pub const CLK_PLL_DE: u32 = 9;
pub const CLK_PLL_ISP: u32 = 10;
pub const CLK_PLL_PERIPH1: u32 = 11;

/* The CPUX clocks are exported */

pub const CLK_ATB0: u32 = 14;
pub const CLK_AXI0: u32 = 15;
pub const CLK_ATB1: u32 = 16;
pub const CLK_AXI1: u32 = 17;
pub const CLK_GTBUS: u32 = 18;
pub const CLK_AHB0: u32 = 19;
pub const CLK_AHB1: u32 = 20;
pub const CLK_AHB2: u32 = 21;
pub const CLK_APB0: u32 = 22;
pub const CLK_APB1: u32 = 23;
pub const CLK_CCI400: u32 = 24;
pub const CLK_ATS: u32 = 25;
pub const CLK_TRACE: u32 = 26;

/* module clocks and bus gates exported */

// CLK_BUS_UART5 is supplied by the included clock bindings.
pub const CLK_NUMBER: u32 = CLK_BUS_UART5 + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
