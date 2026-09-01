/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * cs4265.h -- CS4265 ALSA SoC audio driver
 *
 * Copyright 2014 Cirrus Logic, Inc.
 *
 * Author: Paul Handrigan <paul.handrigan@cirrus.com>
 */

pub const CS4265_CHIP_ID: u32 = 0x1;
pub const CS4265_CHIP_ID_VAL: u32 = 0xD0;
pub const CS4265_CHIP_ID_MASK: u32 = 0xF0;
pub const CS4265_REV_ID_MASK: u32 = 0x0F;

pub const CS4265_PWRCTL: u32 = 0x02;
pub const CS4265_PWRCTL_PDN: u32 = 1;

pub const CS4265_DAC_CTL: u32 = 0x3;
pub const CS4265_DAC_CTL_MUTE: u32 = 1 << 2;
pub const CS4265_DAC_CTL_DIF: u32 = 3 << 4;

pub const CS4265_ADC_CTL: u32 = 0x4;
pub const CS4265_ADC_MASTER: u32 = 1;
pub const CS4265_ADC_DIF: u32 = 1 << 4;
pub const CS4265_ADC_FM: u32 = 3 << 6;

pub const CS4265_MCLK_FREQ: u32 = 0x5;
pub const CS4265_MCLK_FREQ_MASK: u32 = 7 << 4;

pub const CS4265_SIG_SEL: u32 = 0x6;
pub const CS4265_SIG_SEL_LOOP: u32 = 1 << 1;

pub const CS4265_CHB_PGA_CTL: u32 = 0x7;
pub const CS4265_CHA_PGA_CTL: u32 = 0x8;

pub const CS4265_ADC_CTL2: u32 = 0x9;

pub const CS4265_DAC_CHA_VOL: u32 = 0xA;
pub const CS4265_DAC_CHB_VOL: u32 = 0xB;

pub const CS4265_DAC_CTL2: u32 = 0xC;

pub const CS4265_INT_STATUS: u32 = 0xD;
pub const CS4265_INT_MASK: u32 = 0xE;
pub const CS4265_STATUS_MODE_MSB: u32 = 0xF;
pub const CS4265_STATUS_MODE_LSB: u32 = 0x10;

pub const CS4265_SPDIF_CTL1: u32 = 0x11;

pub const CS4265_SPDIF_CTL2: u32 = 0x12;
pub const CS4265_SPDIF_CTL2_MUTE: u32 = 1 << 4;
pub const CS4265_SPDIF_CTL2_DIF: u32 = 3 << 6;

pub const CS4265_C_DATA_BUFF: u32 = 0x13;
pub const CS4265_MAX_REGISTER: u32 = 0x2A;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
