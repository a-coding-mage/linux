// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * wm8753.h  --  audio driver for WM8753
 *
 * Copyright 2003 Wolfson Microelectronics PLC.
 * Author: Liam Girdwood <lrg@slimlogic.co.uk>
 */

/* WM8753 register space */

pub const WM8753_DAC: u32 = 0x01;
pub const WM8753_ADC: u32 = 0x02;
pub const WM8753_PCM: u32 = 0x03;
pub const WM8753_HIFI: u32 = 0x04;
pub const WM8753_IOCTL: u32 = 0x05;
pub const WM8753_SRATE1: u32 = 0x06;
pub const WM8753_SRATE2: u32 = 0x07;
pub const WM8753_LDAC: u32 = 0x08;
pub const WM8753_RDAC: u32 = 0x09;
pub const WM8753_BASS: u32 = 0x0a;
pub const WM8753_TREBLE: u32 = 0x0b;
pub const WM8753_ALC1: u32 = 0x0c;
pub const WM8753_ALC2: u32 = 0x0d;
pub const WM8753_ALC3: u32 = 0x0e;
pub const WM8753_NGATE: u32 = 0x0f;
pub const WM8753_LADC: u32 = 0x10;
pub const WM8753_RADC: u32 = 0x11;
pub const WM8753_ADCTL1: u32 = 0x12;
pub const WM8753_3D: u32 = 0x13;
pub const WM8753_PWR1: u32 = 0x14;
pub const WM8753_PWR2: u32 = 0x15;
pub const WM8753_PWR3: u32 = 0x16;
pub const WM8753_PWR4: u32 = 0x17;
pub const WM8753_ID: u32 = 0x18;
pub const WM8753_INTPOL: u32 = 0x19;
pub const WM8753_INTEN: u32 = 0x1a;
pub const WM8753_GPIO1: u32 = 0x1b;
pub const WM8753_GPIO2: u32 = 0x1c;
pub const WM8753_RESET: u32 = 0x1f;
pub const WM8753_RECMIX1: u32 = 0x20;
pub const WM8753_RECMIX2: u32 = 0x21;
pub const WM8753_LOUTM1: u32 = 0x22;
pub const WM8753_LOUTM2: u32 = 0x23;
pub const WM8753_ROUTM1: u32 = 0x24;
pub const WM8753_ROUTM2: u32 = 0x25;
pub const WM8753_MOUTM1: u32 = 0x26;
pub const WM8753_MOUTM2: u32 = 0x27;
pub const WM8753_LOUT1V: u32 = 0x28;
pub const WM8753_ROUT1V: u32 = 0x29;
pub const WM8753_LOUT2V: u32 = 0x2a;
pub const WM8753_ROUT2V: u32 = 0x2b;
pub const WM8753_MOUTV: u32 = 0x2c;
pub const WM8753_OUTCTL: u32 = 0x2d;
pub const WM8753_ADCIN: u32 = 0x2e;
pub const WM8753_INCTL1: u32 = 0x2f;
pub const WM8753_INCTL2: u32 = 0x30;
pub const WM8753_LINVOL: u32 = 0x31;
pub const WM8753_RINVOL: u32 = 0x32;
pub const WM8753_MICBIAS: u32 = 0x33;
pub const WM8753_CLOCK: u32 = 0x34;
pub const WM8753_PLL1CTL1: u32 = 0x35;
pub const WM8753_PLL1CTL2: u32 = 0x36;
pub const WM8753_PLL1CTL3: u32 = 0x37;
pub const WM8753_PLL1CTL4: u32 = 0x38;
pub const WM8753_PLL2CTL1: u32 = 0x39;
pub const WM8753_PLL2CTL2: u32 = 0x3a;
pub const WM8753_PLL2CTL3: u32 = 0x3b;
pub const WM8753_PLL2CTL4: u32 = 0x3c;
pub const WM8753_BIASCTL: u32 = 0x3d;
pub const WM8753_ADCTL2: u32 = 0x3f;

pub const WM8753_PLL1: u32 = 0;
pub const WM8753_PLL2: u32 = 1;

/* clock inputs */
pub const WM8753_MCLK: u32 = 0;
pub const WM8753_PCMCLK: u32 = 1;

/* clock divider id's */
pub const WM8753_PCMDIV: u32 = 0;
pub const WM8753_BCLKDIV: u32 = 1;
pub const WM8753_VXCLKDIV: u32 = 2;

/* PCM clock dividers */
pub const WM8753_PCM_DIV_1: u32 = 0 << 6;
pub const WM8753_PCM_DIV_3: u32 = 2 << 6;
pub const WM8753_PCM_DIV_5_5: u32 = 3 << 6;
pub const WM8753_PCM_DIV_2: u32 = 4 << 6;
pub const WM8753_PCM_DIV_4: u32 = 5 << 6;
pub const WM8753_PCM_DIV_6: u32 = 6 << 6;
pub const WM8753_PCM_DIV_8: u32 = 7 << 6;

/* BCLK clock dividers */
pub const WM8753_BCLK_DIV_1: u32 = 0 << 3;
pub const WM8753_BCLK_DIV_2: u32 = 1 << 3;
pub const WM8753_BCLK_DIV_4: u32 = 2 << 3;
pub const WM8753_BCLK_DIV_8: u32 = 3 << 3;
pub const WM8753_BCLK_DIV_16: u32 = 4 << 3;

/* VXCLK clock dividers */
pub const WM8753_VXCLK_DIV_1: u32 = 0 << 6;
pub const WM8753_VXCLK_DIV_2: u32 = 1 << 6;
pub const WM8753_VXCLK_DIV_4: u32 = 2 << 6;
pub const WM8753_VXCLK_DIV_8: u32 = 3 << 6;
pub const WM8753_VXCLK_DIV_16: u32 = 4 << 6;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
