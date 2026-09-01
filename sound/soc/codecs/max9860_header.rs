// SPDX-License-Identifier: GPL-2.0
/*
 * Driver for the MAX9860 Mono Audio Voice Codec
 *
 * Author: Peter Rosin <peda@axentia.s>
 *         Copyright 2016 Axentia Technologies
 */

pub const MAX9860_INTRSTATUS: u32 = 0x00;
pub const MAX9860_MICREADBACK: u32 = 0x01;
pub const MAX9860_INTEN: u32 = 0x02;
pub const MAX9860_SYSCLK: u32 = 0x03;
pub const MAX9860_AUDIOCLKHIGH: u32 = 0x04;
pub const MAX9860_AUDIOCLKLOW: u32 = 0x05;
pub const MAX9860_IFC1A: u32 = 0x06;
pub const MAX9860_IFC1B: u32 = 0x07;
pub const MAX9860_VOICEFLTR: u32 = 0x08;
pub const MAX9860_DACATTN: u32 = 0x09;
pub const MAX9860_ADCLEVEL: u32 = 0x0a;
pub const MAX9860_DACGAIN: u32 = 0x0b;
pub const MAX9860_MICGAIN: u32 = 0x0c;
pub const MAX9860_RESERVED: u32 = 0x0d;
pub const MAX9860_MICADC: u32 = 0x0e;
pub const MAX9860_NOISEGATE: u32 = 0x0f;
pub const MAX9860_PWRMAN: u32 = 0x10;
pub const MAX9860_REVISION: u32 = 0xff;

pub const MAX9860_MAX_REGISTER: u32 = 0xff;

/* INTRSTATUS */
pub const MAX9860_CLD: u32 = 0x80;
pub const MAX9860_SLD: u32 = 0x40;
pub const MAX9860_ULK: u32 = 0x20;

/* MICREADBACK */
pub const MAX9860_NG: u32 = 0xe0;
pub const MAX9860_AGC: u32 = 0x1f;

/* INTEN */
pub const MAX9860_ICLD: u32 = 0x80;
pub const MAX9860_ISLD: u32 = 0x40;
pub const MAX9860_IULK: u32 = 0x20;

/* SYSCLK */
pub const MAX9860_PSCLK: u32 = 0x30;
pub const MAX9860_PSCLK_OFF: u32 = 0x00;
pub const MAX9860_PSCLK_SHIFT: u32 = 4;
pub const MAX9860_FREQ: u32 = 0x06;
pub const MAX9860_FREQ_NORMAL: u32 = 0x00;
pub const MAX9860_FREQ_12MHZ: u32 = 0x02;
pub const MAX9860_FREQ_13MHZ: u32 = 0x04;
pub const MAX9860_FREQ_19_2MHZ: u32 = 0x06;
pub const MAX9860_16KHZ: u32 = 0x01;

/* AUDIOCLKHIGH */
pub const MAX9860_PLL: u32 = 0x80;
pub const MAX9860_NHI: u32 = 0x7f;

/* AUDIOCLKLOW */
pub const MAX9860_NLO: u32 = 0xff;

/* IFC1A */
pub const MAX9860_MASTER: u32 = 0x80;
pub const MAX9860_WCI: u32 = 0x40;
pub const MAX9860_DBCI: u32 = 0x20;
pub const MAX9860_DDLY: u32 = 0x10;
pub const MAX9860_HIZ: u32 = 0x08;
pub const MAX9860_TDM: u32 = 0x04;

/* IFC1B */
pub const MAX9860_ABCI: u32 = 0x20;
pub const MAX9860_ADLY: u32 = 0x10;
pub const MAX9860_ST: u32 = 0x08;
pub const MAX9860_BSEL: u32 = 0x07;
pub const MAX9860_BSEL_OFF: u32 = 0x00;
pub const MAX9860_BSEL_64X: u32 = 0x01;
pub const MAX9860_BSEL_48X: u32 = 0x02;
pub const MAX9860_BSEL_PCLK_2: u32 = 0x04;
pub const MAX9860_BSEL_PCLK_4: u32 = 0x05;
pub const MAX9860_BSEL_PCLK_8: u32 = 0x06;
pub const MAX9860_BSEL_PCLK_16: u32 = 0x07;

/* VOICEFLTR */
pub const MAX9860_AVFLT: u32 = 0xf0;
pub const MAX9860_AVFLT_SHIFT: u32 = 4;
pub const MAX9860_AVFLT_COUNT: u32 = 6;
pub const MAX9860_DVFLT: u32 = 0x0f;
pub const MAX9860_DVFLT_SHIFT: u32 = 0;
pub const MAX9860_DVFLT_COUNT: u32 = 6;

/* DACATTN */
pub const MAX9860_DVA: u32 = 0xfe;
pub const MAX9860_DVA_SHIFT: u32 = 1;
pub const MAX9860_DVA_MUTE: u32 = 0x5e;

/* ADCLEVEL */
pub const MAX9860_ADCRL: u32 = 0xf0;
pub const MAX9860_ADCRL_SHIFT: u32 = 4;
pub const MAX9860_ADCLL: u32 = 0x0f;
pub const MAX9860_ADCLL_SHIFT: u32 = 0;
pub const MAX9860_ADCxL_MIN: u32 = 15;

/* DACGAIN */
pub const MAX9860_DVG: u32 = 0x60;
pub const MAX9860_DVG_SHIFT: u32 = 5;
pub const MAX9860_DVG_MAX: u32 = 3;
pub const MAX9860_DVST: u32 = 0x1f;
pub const MAX9860_DVST_SHIFT: u32 = 0;
pub const MAX9860_DVST_MIN: u32 = 31;

/* MICGAIN */
pub const MAX9860_PAM: u32 = 0x60;
pub const MAX9860_PAM_SHIFT: u32 = 5;
pub const MAX9860_PAM_MAX: u32 = 3;
pub const MAX9860_PGAM: u32 = 0x1f;
pub const MAX9860_PGAM_SHIFT: u32 = 0;
pub const MAX9860_PGAM_MIN: u32 = 20;

/* MICADC */
pub const MAX9860_AGCSRC: u32 = 0x80;
pub const MAX9860_AGCSRC_SHIFT: u32 = 7;
pub const MAX9860_AGCSRC_COUNT: u32 = 2;
pub const MAX9860_AGCRLS: u32 = 0x70;
pub const MAX9860_AGCRLS_SHIFT: u32 = 4;
pub const MAX9860_AGCRLS_COUNT: u32 = 8;
pub const MAX9860_AGCATK: u32 = 0x0c;
pub const MAX9860_AGCATK_SHIFT: u32 = 2;
pub const MAX9860_AGCATK_COUNT: u32 = 4;
pub const MAX9860_AGCHLD: u32 = 0x03;
pub const MAX9860_AGCHLD_OFF: u32 = 0x00;
pub const MAX9860_AGCHLD_SHIFT: u32 = 0;
pub const MAX9860_AGCHLD_COUNT: u32 = 4;

/* NOISEGATE */
pub const MAX9860_ANTH: u32 = 0xf0;
pub const MAX9860_ANTH_SHIFT: u32 = 4;
pub const MAX9860_ANTH_MAX: u32 = 15;
pub const MAX9860_AGCTH: u32 = 0x0f;
pub const MAX9860_AGCTH_SHIFT: u32 = 0;
pub const MAX9860_AGCTH_MIN: u32 = 15;

/* PWRMAN */
pub const MAX9860_SHDN: u32 = 0x80;
pub const MAX9860_DACEN: u32 = 0x08;
pub const MAX9860_DACEN_SHIFT: u32 = 3;
pub const MAX9860_ADCLEN: u32 = 0x02;
pub const MAX9860_ADCLEN_SHIFT: u32 = 1;
pub const MAX9860_ADCREN: u32 = 0x01;
pub const MAX9860_ADCREN_SHIFT: u32 = 0;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
