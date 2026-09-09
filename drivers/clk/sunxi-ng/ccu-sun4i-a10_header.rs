/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2017 Priit Laes
 *
 * Priit Laes <plaes@plaes.org>
 */

// Dependencies supplied by the corresponding C dt-bindings headers:
// <dt-bindings/clock/sun4i-a10-ccu.h>
// <dt-bindings/clock/sun7i-a20-ccu.h>
// <dt-bindings/reset/sun4i-a10-ccu.h>

/* The HOSC is exported */
pub const CLK_PLL_CORE: u32 = 2;
pub const CLK_PLL_AUDIO_BASE: u32 = 3;
pub const CLK_PLL_AUDIO: u32 = 4;
pub const CLK_PLL_AUDIO_2X: u32 = 5;
pub const CLK_PLL_AUDIO_4X: u32 = 6;
pub const CLK_PLL_AUDIO_8X: u32 = 7;
pub const CLK_PLL_VIDEO0: u32 = 8;
/* The PLL_VIDEO0_2X clock is exported */
pub const CLK_PLL_VE: u32 = 10;
pub const CLK_PLL_DDR_BASE: u32 = 11;
pub const CLK_PLL_DDR: u32 = 12;
pub const CLK_PLL_DDR_OTHER: u32 = 13;
pub const CLK_PLL_PERIPH_BASE: u32 = 14;
pub const CLK_PLL_PERIPH: u32 = 15;
pub const CLK_PLL_PERIPH_SATA: u32 = 16;
pub const CLK_PLL_VIDEO1: u32 = 17;
/* The PLL_VIDEO1_2X clock is exported */
pub const CLK_PLL_GPU: u32 = 19;

/* The CPU clock is exported */
pub const CLK_AXI: u32 = 21;
pub const CLK_AXI_DRAM: u32 = 22;
pub const CLK_AHB: u32 = 23;
pub const CLK_APB0: u32 = 24;
pub const CLK_APB1: u32 = 25;

/* AHB gates are exported (23..68) */
/* APB0 gates are exported (69..78) */
/* APB1 gates are exported (79..95) */
/* IP module clocks are exported (96..128) */
/* DRAM gates are exported (129..142)*/
/* Media (display engine clocks & etc) are exported (143..169) */

pub const CLK_NUMBER_SUN4I: u32 = CLK_MBUS + 1;
pub const CLK_NUMBER_SUN7I: u32 = CLK_OUT_B + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
