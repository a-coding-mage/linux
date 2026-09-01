// SPDX-License-Identifier: GPL-2.0-only
/*
 * Audio support for Philips UDA1380
 *
 * Copyright (c) 2005 Giorgio Padrin <giorgio@mandarinlogiq.org>
 */

pub const UDA1380_CLK: u16 = 0x00;
pub const UDA1380_IFACE: u16 = 0x01;
pub const UDA1380_PM: u16 = 0x02;
pub const UDA1380_AMIX: u16 = 0x03;
pub const UDA1380_HP: u16 = 0x04;
pub const UDA1380_MVOL: u16 = 0x10;
pub const UDA1380_MIXVOL: u16 = 0x11;
pub const UDA1380_MODE: u16 = 0x12;
pub const UDA1380_DEEMP: u16 = 0x13;
pub const UDA1380_MIXER: u16 = 0x14;
pub const UDA1380_INTSTAT: u16 = 0x18;
pub const UDA1380_DEC: u16 = 0x20;
pub const UDA1380_PGA: u16 = 0x21;
pub const UDA1380_ADC: u16 = 0x22;
pub const UDA1380_AGC: u16 = 0x23;
pub const UDA1380_DECSTAT: u16 = 0x28;
pub const UDA1380_RESET: u16 = 0x7f;

pub const UDA1380_CACHEREGNUM: u16 = 0x24;

/* Register flags */
pub const R00_EN_ADC: u16 = 0x0800;
pub const R00_EN_DEC: u16 = 0x0400;
pub const R00_EN_DAC: u16 = 0x0200;
pub const R00_EN_INT: u16 = 0x0100;
pub const R00_DAC_CLK: u16 = 0x0010;
pub const R01_SFORI_I2S: u16 = 0x0000;
pub const R01_SFORI_LSB16: u16 = 0x0100;
pub const R01_SFORI_LSB18: u16 = 0x0200;
pub const R01_SFORI_LSB20: u16 = 0x0300;
pub const R01_SFORI_MSB: u16 = 0x0500;
pub const R01_SFORI_MASK: u16 = 0x0700;
pub const R01_SFORO_I2S: u16 = 0x0000;
pub const R01_SFORO_LSB16: u16 = 0x0001;
pub const R01_SFORO_LSB18: u16 = 0x0002;
pub const R01_SFORO_LSB20: u16 = 0x0003;
pub const R01_SFORO_LSB24: u16 = 0x0004;
pub const R01_SFORO_MSB: u16 = 0x0005;
pub const R01_SFORO_MASK: u16 = 0x0007;
pub const R01_SEL_SOURCE: u16 = 0x0040;
pub const R01_SIM: u16 = 0x0010;
pub const R02_PON_PLL: u16 = 0x8000;
pub const R02_PON_HP: u16 = 0x2000;
pub const R02_PON_DAC: u16 = 0x0400;
pub const R02_PON_BIAS: u16 = 0x0100;
pub const R02_EN_AVC: u16 = 0x0080;
pub const R02_PON_AVC: u16 = 0x0040;
pub const R02_PON_LNA: u16 = 0x0010;
pub const R02_PON_PGAL: u16 = 0x0008;
pub const R02_PON_ADCL: u16 = 0x0004;
pub const R02_PON_PGAR: u16 = 0x0002;
pub const R02_PON_ADCR: u16 = 0x0001;
pub const R13_MTM: u16 = 0x4000;
pub const R14_SILENCE: u16 = 0x0080;
pub const R14_SDET_ON: u16 = 0x0040;
pub const R21_MT_ADC: u16 = 0x8000;
pub const R22_SEL_LNA: u16 = 0x0008;
pub const R22_SEL_MIC: u16 = 0x0004;
pub const R22_SKIP_DCFIL: u16 = 0x0002;
pub const R23_AGC_EN: u16 = 0x0001;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
