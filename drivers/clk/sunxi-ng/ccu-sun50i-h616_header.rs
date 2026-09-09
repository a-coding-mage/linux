/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2020 Arm Ltd.
 */

// C header guard: _CCU_SUN50I_H616_H_

// External clock and reset binding declarations are supplied by the
// corresponding dependency headers.

pub const CLK_OSC12M: u32 = 0;
pub const CLK_PLL_CPUX: u32 = 1;
pub const CLK_PLL_DDR0: u32 = 2;
pub const CLK_PLL_DDR1: u32 = 3;

/* PLL_PERIPH0 exported for PRCM */

pub const CLK_PLL_PERIPH0_2X: u32 = 5;
pub const CLK_PLL_PERIPH1: u32 = 6;
pub const CLK_PLL_PERIPH1_2X: u32 = 7;
pub const CLK_PLL_GPU: u32 = 8;
pub const CLK_PLL_VIDEO0: u32 = 9;
pub const CLK_PLL_VIDEO0_4X: u32 = 10;
pub const CLK_PLL_VIDEO1: u32 = 11;
pub const CLK_PLL_VIDEO1_4X: u32 = 12;
pub const CLK_PLL_VIDEO2: u32 = 13;
pub const CLK_PLL_VIDEO2_4X: u32 = 14;
pub const CLK_PLL_VE: u32 = 15;
pub const CLK_PLL_DE: u32 = 16;
pub const CLK_PLL_AUDIO_HS: u32 = 17;
pub const CLK_PLL_AUDIO_1X: u32 = 18;
pub const CLK_PLL_AUDIO_2X: u32 = 19;
pub const CLK_PLL_AUDIO_4X: u32 = 20;

/* CPUX clock exported for DVFS */

pub const CLK_AXI: u32 = 22;
pub const CLK_CPUX_APB: u32 = 23;
pub const CLK_PSI_AHB1_AHB2: u32 = 24;
pub const CLK_AHB3: u32 = 25;

/* APB1 clock exported for PIO */

pub const CLK_APB2: u32 = 27;
pub const CLK_MBUS: u32 = 28;

/* All module clocks and bus gates are exported except DRAM */

pub const CLK_DRAM: u32 = 49;

pub const CLK_BUS_DRAM: u32 = 56;

pub const CLK_NUMBER: u32 = CLK_BUS_TCON_LCD1 + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
