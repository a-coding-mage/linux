/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * the following register names are from:
 * wm8776.h  --  WM8776 ASoC driver
 *
 * Copyright 2009 Wolfson Microelectronics plc
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

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

/* HPLVOL/HPRVOL/HPMASTER */
pub const WM8776_HPATT_MASK: u32 = 0x07f;
pub const WM8776_HPZCEN: u32 = 0x080;
pub const WM8776_UPDATE: u32 = 0x100;

/* DACLVOL/DACRVOL/DACMASTER */
pub const WM8776_DATT_MASK: u32 = 0x0ff;
/*#define WM8776_UPDATE		0x100*/

/* PHASESWAP */
pub const WM8776_PH_MASK: u32 = 0x003;

/* DACCTRL1 */
pub const WM8776_DZCEN: u32 = 0x001;
pub const WM8776_ATC: u32 = 0x002;
pub const WM8776_IZD: u32 = 0x004;
pub const WM8776_TOD: u32 = 0x008;
pub const WM8776_PL_LEFT_MASK: u32 = 0x030;
pub const WM8776_PL_LEFT_MUTE: u32 = 0x000;
pub const WM8776_PL_LEFT_LEFT: u32 = 0x010;
pub const WM8776_PL_LEFT_RIGHT: u32 = 0x020;
pub const WM8776_PL_LEFT_LRMIX: u32 = 0x030;
pub const WM8776_PL_RIGHT_MASK: u32 = 0x0c0;
pub const WM8776_PL_RIGHT_MUTE: u32 = 0x000;
pub const WM8776_PL_RIGHT_LEFT: u32 = 0x040;
pub const WM8776_PL_RIGHT_RIGHT: u32 = 0x080;
pub const WM8776_PL_RIGHT_LRMIX: u32 = 0x0c0;

/* DACMUTE */
pub const WM8776_DMUTE: u32 = 0x001;

/* DACCTRL2 */
pub const WM8776_DEEMPH: u32 = 0x001;
pub const WM8776_DZFM_MASK: u32 = 0x006;
pub const WM8776_DZFM_NONE: u32 = 0x000;
pub const WM8776_DZFM_LR: u32 = 0x002;
pub const WM8776_DZFM_BOTH: u32 = 0x004;
pub const WM8776_DZFM_EITHER: u32 = 0x006;

/* DACIFCTRL */
pub const WM8776_DACFMT_MASK: u32 = 0x003;
pub const WM8776_DACFMT_RJUST: u32 = 0x000;
pub const WM8776_DACFMT_LJUST: u32 = 0x001;
pub const WM8776_DACFMT_I2S: u32 = 0x002;
pub const WM8776_DACFMT_DSP: u32 = 0x003;
pub const WM8776_DACLRP: u32 = 0x004;
pub const WM8776_DACBCP: u32 = 0x008;
pub const WM8776_DACWL_MASK: u32 = 0x030;
pub const WM8776_DACWL_16: u32 = 0x000;
pub const WM8776_DACWL_20: u32 = 0x010;
pub const WM8776_DACWL_24: u32 = 0x020;
pub const WM8776_DACWL_32: u32 = 0x030;

/* ADCIFCTRL */
pub const WM8776_ADCFMT_MASK: u32 = 0x003;
pub const WM8776_ADCFMT_RJUST: u32 = 0x000;
pub const WM8776_ADCFMT_LJUST: u32 = 0x001;
pub const WM8776_ADCFMT_I2S: u32 = 0x002;
pub const WM8776_ADCFMT_DSP: u32 = 0x003;
pub const WM8776_ADCLRP: u32 = 0x004;
pub const WM8776_ADCBCP: u32 = 0x008;
pub const WM8776_ADCWL_MASK: u32 = 0x030;
pub const WM8776_ADCWL_16: u32 = 0x000;
pub const WM8776_ADCWL_20: u32 = 0x010;
pub const WM8776_ADCWL_24: u32 = 0x020;
pub const WM8776_ADCWL_32: u32 = 0x030;
pub const WM8776_ADCMCLK: u32 = 0x040;
pub const WM8776_ADCHPD: u32 = 0x100;

/* MSTRCTRL */
pub const WM8776_ADCRATE_MASK: u32 = 0x007;
pub const WM8776_ADCRATE_256: u32 = 0x002;
pub const WM8776_ADCRATE_384: u32 = 0x003;
pub const WM8776_ADCRATE_512: u32 = 0x004;
pub const WM8776_ADCRATE_768: u32 = 0x005;
pub const WM8776_ADCOSR: u32 = 0x008;
pub const WM8776_DACRATE_MASK: u32 = 0x070;
pub const WM8776_DACRATE_128: u32 = 0x000;
pub const WM8776_DACRATE_192: u32 = 0x010;
pub const WM8776_DACRATE_256: u32 = 0x020;
pub const WM8776_DACRATE_384: u32 = 0x030;
pub const WM8776_DACRATE_512: u32 = 0x040;
pub const WM8776_DACRATE_768: u32 = 0x050;
pub const WM8776_DACMS: u32 = 0x080;
pub const WM8776_ADCMS: u32 = 0x100;

/* PWRDOWN */
pub const WM8776_PDWN: u32 = 0x001;
pub const WM8776_ADCPD: u32 = 0x002;
pub const WM8776_DACPD: u32 = 0x004;
pub const WM8776_HPPD: u32 = 0x008;
pub const WM8776_AINPD: u32 = 0x040;

/* ADCLVOL/ADCRVOL */
pub const WM8776_AGMASK: u32 = 0x0ff;
pub const WM8776_ZCA: u32 = 0x100;

/* ALCCTRL1 */
pub const WM8776_LCT_MASK: u32 = 0x00f;
pub const WM8776_MAXGAIN_MASK: u32 = 0x070;
pub const WM8776_LCSEL_MASK: u32 = 0x180;
pub const WM8776_LCSEL_LIMITER: u32 = 0x000;
pub const WM8776_LCSEL_ALC_RIGHT: u32 = 0x080;
pub const WM8776_LCSEL_ALC_LEFT: u32 = 0x100;
pub const WM8776_LCSEL_ALC_STEREO: u32 = 0x180;

/* ALCCTRL2 */
pub const WM8776_HLD_MASK: u32 = 0x00f;
pub const WM8776_ALCZC: u32 = 0x080;
pub const WM8776_LCEN: u32 = 0x100;

/* ALCCTRL3 */
pub const WM8776_ATK_MASK: u32 = 0x00f;
pub const WM8776_DCY_MASK: u32 = 0x0f0;

/* NOISEGATE */
pub const WM8776_NGAT: u32 = 0x001;
pub const WM8776_NGTH_MASK: u32 = 0x01c;

/* LIMITER */
pub const WM8776_MAXATTEN_MASK: u32 = 0x00f;
pub const WM8776_TRANWIN_MASK: u32 = 0x070;

/* ADCMUX */
pub const WM8776_AMX_MASK: u32 = 0x01f;
pub const WM8776_MUTERA: u32 = 0x040;
pub const WM8776_MUTELA: u32 = 0x080;
pub const WM8776_LRBOTH: u32 = 0x100;

/* OUTMUX */
pub const WM8776_MX_DAC: u32 = 0x001;
pub const WM8776_MX_AUX: u32 = 0x002;
pub const WM8776_MX_BYPASS: u32 = 0x004;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
