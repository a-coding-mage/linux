// SPDX-License-Identifier: GPL-2.0-only
/*
 * da732x.h -- Dialog DA732X ALSA SoC Audio Driver Header File
 *
 * Copyright (C) 2012 Dialog Semiconductor GmbH
 *
 * Author: Michal Hajduk <Michal.Hajduk@diasemi.com>
 */

// C header dependency: <sound/soc.h>

/* General */
pub const DA732X_U8_MASK: u32 = 0xFF;
pub const DA732X_4BYTES: u32 = 4;
pub const DA732X_3BYTES: u32 = 3;
pub const DA732X_2BYTES: u32 = 2;
pub const DA732X_1BYTE: u32 = 1;
pub const DA732X_1BYTE_SHIFT: u32 = 8;
pub const DA732X_2BYTES_SHIFT: u32 = 16;
pub const DA732X_3BYTES_SHIFT: u32 = 24;
pub const DA732X_4BYTES_SHIFT: u32 = 32;

pub const DA732X_DACS_DIS: u32 = 0x0;
pub const DA732X_HP_DIS: u32 = 0x0;
pub const DA732X_CLEAR_REG: u32 = 0x0;

/* Calibration */
pub const DA732X_DAC_OFFSET_STEP: u32 = 0x20;
pub const DA732X_OUTPUT_OFFSET_STEP: u32 = 0x80;
pub const DA732X_HP_OUT_TRIM_VAL: u32 = 0x0;
pub const DA732X_WAIT_FOR_STABILIZATION: u32 = 1;
pub const DA732X_HPL_DAC: u32 = 0;
pub const DA732X_HPR_DAC: u32 = 1;
pub const DA732X_HP_DACS: u32 = 2;
pub const DA732X_HPL_AMP: u32 = 0;
pub const DA732X_HPR_AMP: u32 = 1;
pub const DA732X_HP_AMPS: u32 = 2;

/* Clock settings */
pub const DA732X_STARTUP_DELAY: u32 = 100;
pub const DA732X_PLL_OUT_196608: u32 = 196608000;
pub const DA732X_PLL_OUT_180634: u32 = 180633600;
pub const DA732X_PLL_OUT_SRM: u32 = 188620800;
pub const DA732X_MCLK_10MHZ: u32 = 10000000;
pub const DA732X_MCLK_20MHZ: u32 = 20000000;
pub const DA732X_MCLK_40MHZ: u32 = 40000000;
pub const DA732X_MCLK_54MHZ: u32 = 54000000;
pub const DA732X_MCLK_VAL_0_10MHZ: u32 = 0;
pub const DA732X_MCLK_VAL_10_20MHZ: u32 = 1;
pub const DA732X_MCLK_VAL_20_40MHZ: u32 = 2;
pub const DA732X_MCLK_VAL_40_54MHZ: u32 = 3;
pub const DA732X_DAI_ID1: u32 = 0;
pub const DA732X_DAI_ID2: u32 = 1;
pub const DA732X_SRCCLK_PLL: u32 = 0;
pub const DA732X_SRCCLK_MCLK: u32 = 1;

pub const DA732X_LIN_LP_VOL: u32 = 0x4F;
pub const DA732X_LP_VOL: u32 = 0x40;

/* Kcontrols */
pub const DA732X_DAC_EN_MAX: u32 = 2;
pub const DA732X_ADCL_MUX_MAX: u32 = 2;
pub const DA732X_ADCR_MUX_MAX: u32 = 3;
pub const DA732X_HPF_MODE_MAX: u32 = 3;
pub const DA732X_HPF_MODE_SHIFT: u32 = 4;
pub const DA732X_HPF_MUSIC_SHIFT: u32 = 0;
pub const DA732X_HPF_MUSIC_MAX: u32 = 4;
pub const DA732X_HPF_VOICE_SHIFT: u32 = 4;
pub const DA732X_HPF_VOICE_MAX: u32 = 8;
pub const DA732X_EQ_EN_MAX: u32 = 1;
pub const DA732X_HPF_VOICE: u32 = 1;
pub const DA732X_HPF_MUSIC: u32 = 2;
pub const DA732X_HPF_DISABLED: u32 = 0;
pub const DA732X_NO_INVERT: u32 = 0;
pub const DA732X_INVERT: u32 = 1;
pub const DA732X_SWITCH_MAX: u32 = 1;
pub const DA732X_ENABLE_CP: u32 = 1;
pub const DA732X_DISABLE_CP: u32 = 0;
pub const DA732X_DISABLE_ALL_CLKS: u32 = 0;
pub const DA732X_RESET_ADCS: u32 = 0;

/* dB values */
pub const DA732X_MIC_VOL_DB_MIN: i32 = 0;
pub const DA732X_MIC_VOL_DB_INC: i32 = 50;
pub const DA732X_MIC_PRE_VOL_DB_MIN: i32 = 0;
pub const DA732X_MIC_PRE_VOL_DB_INC: i32 = 600;
pub const DA732X_AUX_VOL_DB_MIN: i32 = -6000;
pub const DA732X_AUX_VOL_DB_INC: i32 = 150;
pub const DA732X_HP_VOL_DB_MIN: i32 = -2250;
pub const DA732X_HP_VOL_DB_INC: i32 = 150;
pub const DA732X_LIN2_VOL_DB_MIN: i32 = -1650;
pub const DA732X_LIN2_VOL_DB_INC: i32 = 150;
pub const DA732X_LIN3_VOL_DB_MIN: i32 = -1650;
pub const DA732X_LIN3_VOL_DB_INC: i32 = 150;
pub const DA732X_LIN4_VOL_DB_MIN: i32 = -2250;
pub const DA732X_LIN4_VOL_DB_INC: i32 = 150;
pub const DA732X_EQ_BAND_VOL_DB_MIN: i32 = -1050;
pub const DA732X_EQ_BAND_VOL_DB_INC: i32 = 150;
pub const DA732X_DAC_VOL_DB_MIN: i32 = -7725;
pub const DA732X_DAC_VOL_DB_INC: i32 = 75;
pub const DA732X_ADC_VOL_DB_MIN: i32 = 0;
pub const DA732X_ADC_VOL_DB_INC: i32 = -1;
pub const DA732X_EQ_OVERALL_VOL_DB_MIN: i32 = -1800;
pub const DA732X_EQ_OVERALL_VOL_DB_INC: i32 = 600;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum da732x_sysctl {
    DA732X_SR_8KHZ = 0x1,
    DA732X_SR_11_025KHZ = 0x2,
    DA732X_SR_12KHZ = 0x3,
    DA732X_SR_16KHZ = 0x5,
    DA732X_SR_22_05KHZ = 0x6,
    DA732X_SR_24KHZ = 0x7,
    DA732X_SR_32KHZ = 0x9,
    DA732X_SR_44_1KHZ = 0xA,
    DA732X_SR_48KHZ = 0xB,
    DA732X_SR_88_1KHZ = 0xE,
    DA732X_SR_96KHZ = 0xF,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
