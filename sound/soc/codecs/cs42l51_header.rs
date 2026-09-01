/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * cs42l51.h
 *
 * ASoC Driver for Cirrus Logic CS42L51 codecs
 *
 * Copyright (c) 2010 Arnaud Patard <apatard@mandriva.com>
 */

use core::ffi::c_int;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static cs42l51_regmap: regmap_config;
    pub fn cs42l51_probe(dev: *mut device, regmap: *mut regmap) -> c_int;
    pub fn cs42l51_remove(dev: *mut device);
    /* C declaration used __maybe_unused. */
    pub fn cs42l51_suspend(dev: *mut device) -> c_int;
    /* C declaration used __maybe_unused. */
    pub fn cs42l51_resume(dev: *mut device) -> c_int;
}

pub const CS42L51_CHIP_ID: u32 = 0x1B;
pub const CS42L51_CHIP_REV_A: u32 = 0x00;
pub const CS42L51_CHIP_REV_B: u32 = 0x01;
pub const CS42L51_CHIP_REV_MASK: u32 = 0x07;

pub const CS42L51_CHIP_REV_ID: u32 = 0x01;
pub const fn CS42L51_MK_CHIP_REV(a: u32, b: u32) -> u32 {
    (a << 3) | b
}

pub const CS42L51_POWER_CTL1: u32 = 0x02;
pub const CS42L51_POWER_CTL1_PDN_DACB: u32 = 1 << 6;
pub const CS42L51_POWER_CTL1_PDN_DACA: u32 = 1 << 5;
pub const CS42L51_POWER_CTL1_PDN_PGAB: u32 = 1 << 4;
pub const CS42L51_POWER_CTL1_PDN_PGAA: u32 = 1 << 3;
pub const CS42L51_POWER_CTL1_PDN_ADCB: u32 = 1 << 2;
pub const CS42L51_POWER_CTL1_PDN_ADCA: u32 = 1 << 1;
pub const CS42L51_POWER_CTL1_PDN: u32 = 1 << 0;

pub const CS42L51_MIC_POWER_CTL: u32 = 0x03;
pub const CS42L51_MIC_POWER_CTL_AUTO: u32 = 1 << 7;
pub const fn CS42L51_MIC_POWER_CTL_SPEED(x: u32) -> u32 {
    (x & 3) << 5
}
pub const CS42L51_QSM_MODE: u32 = 3;
pub const CS42L51_HSM_MODE: u32 = 2;
pub const CS42L51_SSM_MODE: u32 = 1;
pub const CS42L51_DSM_MODE: u32 = 0;
pub const CS42L51_MIC_POWER_CTL_3ST_SP: u32 = 1 << 4;
pub const CS42L51_MIC_POWER_CTL_PDN_MICB: u32 = 1 << 3;
pub const CS42L51_MIC_POWER_CTL_PDN_MICA: u32 = 1 << 2;
pub const CS42L51_MIC_POWER_CTL_PDN_BIAS: u32 = 1 << 1;
pub const CS42L51_MIC_POWER_CTL_MCLK_DIV2: u32 = 1 << 0;

pub const CS42L51_INTF_CTL: u32 = 0x04;
pub const CS42L51_INTF_CTL_LOOPBACK: u32 = 1 << 7;
pub const CS42L51_INTF_CTL_MASTER: u32 = 1 << 6;
pub const fn CS42L51_INTF_CTL_DAC_FORMAT(x: u32) -> u32 {
    (x & 7) << 3
}
pub const CS42L51_DAC_DIF_LJ24: u32 = 0x00;
pub const CS42L51_DAC_DIF_I2S: u32 = 0x01;
pub const CS42L51_DAC_DIF_RJ24: u32 = 0x02;
pub const CS42L51_DAC_DIF_RJ20: u32 = 0x03;
pub const CS42L51_DAC_DIF_RJ18: u32 = 0x04;
pub const CS42L51_DAC_DIF_RJ16: u32 = 0x05;
pub const CS42L51_INTF_CTL_ADC_I2S: u32 = 1 << 2;
pub const CS42L51_INTF_CTL_DIGMIX: u32 = 1 << 1;
pub const CS42L51_INTF_CTL_MICMIX: u32 = 1 << 0;

pub const CS42L51_MIC_CTL: u32 = 0x05;
pub const CS42L51_MIC_CTL_ADC_SNGVOL: u32 = 1 << 7;
pub const CS42L51_MIC_CTL_ADCD_DBOOST: u32 = 1 << 6;
pub const CS42L51_MIC_CTL_ADCA_DBOOST: u32 = 1 << 5;
pub const CS42L51_MIC_CTL_MICBIAS_SEL: u32 = 1 << 4;
pub const fn CS42L51_MIC_CTL_MICBIAS_LVL(x: u32) -> u32 {
    (x & 3) << 2
}
pub const CS42L51_MIC_CTL_MICB_BOOST: u32 = 1 << 1;
pub const CS42L51_MIC_CTL_MICA_BOOST: u32 = 1 << 0;

pub const CS42L51_ADC_CTL: u32 = 0x06;
pub const CS42L51_ADC_CTL_ADCB_HPFEN: u32 = 1 << 7;
pub const CS42L51_ADC_CTL_ADCB_HPFRZ: u32 = 1 << 6;
pub const CS42L51_ADC_CTL_ADCA_HPFEN: u32 = 1 << 5;
pub const CS42L51_ADC_CTL_ADCA_HPFRZ: u32 = 1 << 4;
pub const CS42L51_ADC_CTL_SOFTB: u32 = 1 << 3;
pub const CS42L51_ADC_CTL_ZCROSSB: u32 = 1 << 2;
pub const CS42L51_ADC_CTL_SOFTA: u32 = 1 << 1;
pub const CS42L51_ADC_CTL_ZCROSSA: u32 = 1 << 0;

pub const CS42L51_ADC_INPUT: u32 = 0x07;
pub const fn CS42L51_ADC_INPUT_AINB_MUX(x: u32) -> u32 {
    (x & 3) << 6
}
pub const fn CS42L51_ADC_INPUT_AINA_MUX(x: u32) -> u32 {
    (x & 3) << 4
}
pub const CS42L51_ADC_INPUT_INV_ADCB: u32 = 1 << 3;
pub const CS42L51_ADC_INPUT_INV_ADCA: u32 = 1 << 2;
pub const CS42L51_ADC_INPUT_ADCB_MUTE: u32 = 1 << 1;
pub const CS42L51_ADC_INPUT_ADCA_MUTE: u32 = 1 << 0;

pub const CS42L51_DAC_OUT_CTL: u32 = 0x08;
pub const fn CS42L51_DAC_OUT_CTL_HP_GAIN(x: u32) -> u32 {
    (x & 7) << 5
}
pub const CS42L51_DAC_OUT_CTL_DAC_SNGVOL: u32 = 1 << 4;
pub const CS42L51_DAC_OUT_CTL_INV_PCMB: u32 = 1 << 3;
pub const CS42L51_DAC_OUT_CTL_INV_PCMA: u32 = 1 << 2;
pub const CS42L51_DAC_OUT_CTL_DACB_MUTE: u32 = 1 << 1;
pub const CS42L51_DAC_OUT_CTL_DACA_MUTE: u32 = 1 << 0;

pub const CS42L51_DAC_CTL: u32 = 0x09;
pub const fn CS42L51_DAC_CTL_DATA_SEL(x: u32) -> u32 {
    (x & 3) << 6
}
pub const CS42L51_DAC_CTL_FREEZE: u32 = 1 << 5;
pub const CS42L51_DAC_CTL_DEEMPH: u32 = 1 << 3;
pub const CS42L51_DAC_CTL_AMUTE: u32 = 1 << 2;
pub const fn CS42L51_DAC_CTL_DACSZ(x: u32) -> u32 {
    (x & 3) << 0
}

pub const CS42L51_ALC_PGA_CTL: u32 = 0x0A;
pub const CS42L51_ALC_PGB_CTL: u32 = 0x0B;
pub const CS42L51_ALC_PGX_ALCX_SRDIS: u32 = 1 << 7;
pub const CS42L51_ALC_PGX_ALCX_ZCDIS: u32 = 1 << 6;
pub const fn CS42L51_ALC_PGX_PGX_VOL(x: u32) -> u32 {
    (x & 0x1f) << 0
}

pub const CS42L51_ADCA_ATT: u32 = 0x0C;
pub const CS42L51_ADCB_ATT: u32 = 0x0D;

pub const CS42L51_ADCA_VOL: u32 = 0x0E;
pub const CS42L51_ADCB_VOL: u32 = 0x0F;
pub const CS42L51_PCMA_VOL: u32 = 0x10;
pub const CS42L51_PCMB_VOL: u32 = 0x11;
pub const CS42L51_MIX_MUTE_ADCMIX: u32 = 1 << 7;
pub const fn CS42L51_MIX_VOLUME(x: u32) -> u32 {
    (x & 0x7f) << 0
}

pub const CS42L51_BEEP_FREQ: u32 = 0x12;
pub const CS42L51_BEEP_VOL: u32 = 0x13;
pub const CS42L51_BEEP_CONF: u32 = 0x14;

pub const CS42L51_TONE_CTL: u32 = 0x15;
pub const fn CS42L51_TONE_CTL_TREB(x: u32) -> u32 {
    (x & 0xf) << 4
}
pub const fn CS42L51_TONE_CTL_BASS(x: u32) -> u32 {
    (x & 0xf) << 0
}

pub const CS42L51_AOUTA_VOL: u32 = 0x16;
pub const CS42L51_AOUTB_VOL: u32 = 0x17;
pub const CS42L51_PCM_MIXER: u32 = 0x18;
pub const CS42L51_LIMIT_THRES_DIS: u32 = 0x19;
pub const CS42L51_LIMIT_REL: u32 = 0x1A;
pub const CS42L51_LIMIT_ATT: u32 = 0x1B;
pub const CS42L51_ALC_EN: u32 = 0x1C;
pub const CS42L51_ALC_REL: u32 = 0x1D;
pub const CS42L51_ALC_THRES: u32 = 0x1E;
pub const CS42L51_NOISE_CONF: u32 = 0x1F;

pub const CS42L51_STATUS: u32 = 0x20;
pub const CS42L51_STATUS_SP_CLKERR: u32 = 1 << 6;
pub const CS42L51_STATUS_SPEA_OVFL: u32 = 1 << 5;
pub const CS42L51_STATUS_SPEB_OVFL: u32 = 1 << 4;
pub const CS42L51_STATUS_PCMA_OVFL: u32 = 1 << 3;
pub const CS42L51_STATUS_PCMB_OVFL: u32 = 1 << 2;
pub const CS42L51_STATUS_ADCA_OVFL: u32 = 1 << 1;
pub const CS42L51_STATUS_ADCB_OVFL: u32 = 1 << 0;

pub const CS42L51_CHARGE_FREQ: u32 = 0x21;

pub const CS42L51_FIRSTREG: u32 = 0x01;
/*
 * Hack: with register 0x21, it makes 33 registers. Looks like someone in the
 * i2c layer doesn't like i2c smbus block read of 33 regs. Workaround by using
 * 32 regs
 */
pub const CS42L51_LASTREG: u32 = 0x20;
pub const CS42L51_NUMREGS: u32 = CS42L51_LASTREG - CS42L51_FIRSTREG + 1;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
