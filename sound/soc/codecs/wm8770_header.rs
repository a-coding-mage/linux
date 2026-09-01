/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * wm8770.h  --  WM8770 ASoC driver
 *
 * Copyright 2010 Wolfson Microelectronics plc
 *
 * Author: Dimitris Papastamos <dp@opensource.wolfsonmicro.com>
 */

/* Registers */
pub const WM8770_VOUT1LVOL: u32 = 0;
pub const WM8770_VOUT1RVOL: u32 = 0x1;
pub const WM8770_VOUT2LVOL: u32 = 0x2;
pub const WM8770_VOUT2RVOL: u32 = 0x3;
pub const WM8770_VOUT3LVOL: u32 = 0x4;
pub const WM8770_VOUT3RVOL: u32 = 0x5;
pub const WM8770_VOUT4LVOL: u32 = 0x6;
pub const WM8770_VOUT4RVOL: u32 = 0x7;
pub const WM8770_MSALGVOL: u32 = 0x8;
pub const WM8770_DAC1LVOL: u32 = 0x9;
pub const WM8770_DAC1RVOL: u32 = 0xa;
pub const WM8770_DAC2LVOL: u32 = 0xb;
pub const WM8770_DAC2RVOL: u32 = 0xc;
pub const WM8770_DAC3LVOL: u32 = 0xd;
pub const WM8770_DAC3RVOL: u32 = 0xe;
pub const WM8770_DAC4LVOL: u32 = 0xf;
pub const WM8770_DAC4RVOL: u32 = 0x10;
pub const WM8770_MSDIGVOL: u32 = 0x11;
pub const WM8770_DACPHASE: u32 = 0x12;
pub const WM8770_DACCTRL1: u32 = 0x13;
pub const WM8770_DACMUTE: u32 = 0x14;
pub const WM8770_DACCTRL2: u32 = 0x15;
pub const WM8770_IFACECTRL: u32 = 0x16;
pub const WM8770_MSTRCTRL: u32 = 0x17;
pub const WM8770_PWDNCTRL: u32 = 0x18;
pub const WM8770_ADCLCTRL: u32 = 0x19;
pub const WM8770_ADCRCTRL: u32 = 0x1a;
pub const WM8770_ADCMUX: u32 = 0x1b;
pub const WM8770_OUTMUX1: u32 = 0x1c;
pub const WM8770_OUTMUX2: u32 = 0x1d;
pub const WM8770_RESET: u32 = 0x31;

pub const WM8770_CACHEREGNUM: u32 = 0x20;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
