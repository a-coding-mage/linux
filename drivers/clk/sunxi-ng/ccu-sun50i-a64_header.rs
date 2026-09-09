/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2016 Maxime Ripard
 *
 * Maxime Ripard <maxime.ripard@free-electrons.com>
 */

// Dependency declarations from:
// <dt-bindings/clock/sun50i-a64-ccu.h>
// <dt-bindings/reset/sun50i-a64-ccu.h>

pub const CLK_OSC_12M: i32 = 0;
pub const CLK_PLL_CPUX: i32 = 1;
pub const CLK_PLL_AUDIO_BASE: i32 = 2;
pub const CLK_PLL_AUDIO: i32 = 3;
pub const CLK_PLL_AUDIO_2X: i32 = 4;
pub const CLK_PLL_AUDIO_4X: i32 = 5;
pub const CLK_PLL_AUDIO_8X: i32 = 6;

/* PLL_VIDEO0 exported for HDMI PHY */

pub const CLK_PLL_VE: i32 = 9;
pub const CLK_PLL_DDR0: i32 = 10;

/* PLL_PERIPH0 exported for PRCM */

pub const CLK_PLL_PERIPH0_2X: i32 = 12;
pub const CLK_PLL_PERIPH1: i32 = 13;
pub const CLK_PLL_PERIPH1_2X: i32 = 14;
pub const CLK_PLL_VIDEO1: i32 = 15;
pub const CLK_PLL_GPU: i32 = 16;
pub const CLK_PLL_HSIC: i32 = 18;
pub const CLK_PLL_DE: i32 = 19;
pub const CLK_PLL_DDR1: i32 = 20;
pub const CLK_AXI: i32 = 22;
pub const CLK_APB: i32 = 23;
pub const CLK_AHB1: i32 = 24;
pub const CLK_APB1: i32 = 25;
pub const CLK_APB2: i32 = 26;
pub const CLK_AHB2: i32 = 27;

/* All the bus gates are exported */

/* The first bunch of module clocks are exported */

pub const CLK_USB_OHCI0_12M: i32 = 90;

pub const CLK_USB_OHCI1_12M: i32 = 92;

/* All the DRAM gates are exported */

/* And the DSI and GPU module clock is exported */

pub const CLK_NUMBER: i32 = CLK_GPU + 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
