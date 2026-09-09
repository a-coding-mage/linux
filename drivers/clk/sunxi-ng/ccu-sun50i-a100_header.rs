/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2020 Yangtao Li <frank@allwinnertech.com>
 */

// Dependency declarations supplied by the corresponding clock and reset
// binding headers are expected to be available to this translation unit.

pub const CLK_OSC12M: u32 = 0;
pub const CLK_PLL_CPUX: u32 = 1;
pub const CLK_PLL_DDR0: u32 = 2;

/* PLL_PERIPH0 exported for PRCM */

pub const CLK_PLL_PERIPH0_2X: u32 = 4;
pub const CLK_PLL_PERIPH1: u32 = 5;
pub const CLK_PLL_PERIPH1_2X: u32 = 6;
pub const CLK_PLL_GPU: u32 = 7;
pub const CLK_PLL_VIDEO0: u32 = 8;
pub const CLK_PLL_VIDEO0_2X: u32 = 9;
pub const CLK_PLL_VIDEO0_4X: u32 = 10;
pub const CLK_PLL_VIDEO1: u32 = 11;
pub const CLK_PLL_VIDEO1_2X: u32 = 12;
pub const CLK_PLL_VIDEO1_4X: u32 = 13;
pub const CLK_PLL_VIDEO2: u32 = 14;
pub const CLK_PLL_VIDEO2_2X: u32 = 15;
pub const CLK_PLL_VIDEO2_4X: u32 = 16;
pub const CLK_PLL_VIDEO3: u32 = 17;
pub const CLK_PLL_VIDEO3_2X: u32 = 18;
pub const CLK_PLL_VIDEO3_4X: u32 = 19;
pub const CLK_PLL_VE: u32 = 20;
pub const CLK_PLL_COM: u32 = 21;
pub const CLK_PLL_COM_AUDIO: u32 = 22;
pub const CLK_PLL_AUDIO: u32 = 23;

/* CPUX clock exported for DVFS */

pub const CLK_AXI: u32 = 25;
pub const CLK_CPUX_APB: u32 = 26;
pub const CLK_PSI_AHB1_AHB2: u32 = 27;
pub const CLK_AHB3: u32 = 28;

/* APB1 clock exported for PIO */

pub const CLK_APB2: u32 = 30;

/* All module clocks and bus gates are exported except DRAM */

pub const CLK_BUS_DRAM: u32 = 58;

pub const CLK_NUMBER: u32 = CLK_CSI_ISP + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
