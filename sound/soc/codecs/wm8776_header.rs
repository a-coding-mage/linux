/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * wm8776.h  --  WM8776 ASoC driver
 *
 * Copyright 2009 Wolfson Microelectronics plc
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

/* Registers */

pub const WM8776_HPLVOL: u32 = 0x00;
pub const WM8776_HPRVOL: u32 = 0x01;
pub const WM8776_HPMASTER: u32 = 0x02;
pub const WM8776_DACLVOL: u32 = 0x03;
pub const WM8776_DACRVOL: u32 = 0x04;
pub const WM8776_DACMASTER: u32 = 0x05;
pub const WM8776_PHASESWAP: u32 = 0x06;
pub const WM8776_DACCTRL1: u32 = 0x07;
pub const WM8776_DACMUTE: u32 = 0x08;
pub const WM8776_DACCTRL2: u32 = 0x09;
pub const WM8776_DACIFCTRL: u32 = 0x0a;
pub const WM8776_ADCIFCTRL: u32 = 0x0b;
pub const WM8776_MSTRCTRL: u32 = 0x0c;
pub const WM8776_PWRDOWN: u32 = 0x0d;
pub const WM8776_ADCLVOL: u32 = 0x0e;
pub const WM8776_ADCRVOL: u32 = 0x0f;
pub const WM8776_ALCCTRL1: u32 = 0x10;
pub const WM8776_ALCCTRL2: u32 = 0x11;
pub const WM8776_ALCCTRL3: u32 = 0x12;
pub const WM8776_NOISEGATE: u32 = 0x13;
pub const WM8776_LIMITER: u32 = 0x14;
pub const WM8776_ADCMUX: u32 = 0x15;
pub const WM8776_OUTMUX: u32 = 0x16;
pub const WM8776_RESET: u32 = 0x17;

pub const WM8776_CACHEREGNUM: u32 = 0x17;

pub const WM8776_DAI_DAC: u32 = 0;
pub const WM8776_DAI_ADC: u32 = 1;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
