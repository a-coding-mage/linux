// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * wm8580.h  --  audio driver for WM8580
 *
 * Copyright 2008 Samsung Electronics.
 * Author: Ryu Euiyoul
 *         ryu.real@gmail.com
 */

pub const WM8580_PLLA: u32 = 1;
pub const WM8580_PLLB: u32 = 2;

pub const WM8580_MCLK: u32 = 1;
pub const WM8580_CLKOUTSRC: u32 = 2;

pub const WM8580_CLKSRC_MCLK: u32 = 1;
pub const WM8580_CLKSRC_PLLA: u32 = 2;
pub const WM8580_CLKSRC_PLLB: u32 = 3;
pub const WM8580_CLKSRC_OSC: u32 = 4;
pub const WM8580_CLKSRC_NONE: u32 = 5;
pub const WM8580_CLKSRC_ADCMCLK: u32 = 6;

pub const WM8580_DAI_PAIFRX: u32 = 0;
pub const WM8580_DAI_PAIFTX: u32 = 1;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
