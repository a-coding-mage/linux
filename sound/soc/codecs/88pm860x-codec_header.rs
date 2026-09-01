// SPDX-License-Identifier: GPL-2.0-only
/*
 * 88pm860x-codec.h -- 88PM860x ALSA SoC Audio Driver
 *
 * Copyright 2010 Marvell International Ltd.
 *	Haojian Zhuang <haojian.zhuang@marvell.com>
 */

pub const PM860X_PCM_IFACE_1: u32 = 0xb0;
pub const PM860X_PCM_IFACE_2: u32 = 0xb1;
pub const PM860X_PCM_IFACE_3: u32 = 0xb2;
pub const PM860X_PCM_RATE: u32 = 0xb3;
pub const PM860X_EC_PATH: u32 = 0xb4;
pub const PM860X_SIDETONE_L_GAIN: u32 = 0xb5;
pub const PM860X_SIDETONE_R_GAIN: u32 = 0xb6;
pub const PM860X_SIDETONE_SHIFT: u32 = 0xb7;
pub const PM860X_ADC_OFFSET_1: u32 = 0xb8;
pub const PM860X_ADC_OFFSET_2: u32 = 0xb9;
pub const PM860X_DMIC_DELAY: u32 = 0xba;

pub const PM860X_I2S_IFACE_1: u32 = 0xbb;
pub const PM860X_I2S_IFACE_2: u32 = 0xbc;
pub const PM860X_I2S_IFACE_3: u32 = 0xbd;
pub const PM860X_I2S_IFACE_4: u32 = 0xbe;
pub const PM860X_EQUALIZER_N0_1: u32 = 0xbf;
pub const PM860X_EQUALIZER_N0_2: u32 = 0xc0;
pub const PM860X_EQUALIZER_N1_1: u32 = 0xc1;
pub const PM860X_EQUALIZER_N1_2: u32 = 0xc2;
pub const PM860X_EQUALIZER_D1_1: u32 = 0xc3;
pub const PM860X_EQUALIZER_D1_2: u32 = 0xc4;
pub const PM860X_LOFI_GAIN_LEFT: u32 = 0xc5;
pub const PM860X_LOFI_GAIN_RIGHT: u32 = 0xc6;
pub const PM860X_HIFIL_GAIN_LEFT: u32 = 0xc7;
pub const PM860X_HIFIL_GAIN_RIGHT: u32 = 0xc8;
pub const PM860X_HIFIR_GAIN_LEFT: u32 = 0xc9;
pub const PM860X_HIFIR_GAIN_RIGHT: u32 = 0xca;
pub const PM860X_DAC_OFFSET: u32 = 0xcb;
pub const PM860X_OFFSET_LEFT_1: u32 = 0xcc;
pub const PM860X_OFFSET_LEFT_2: u32 = 0xcd;
pub const PM860X_OFFSET_RIGHT_1: u32 = 0xce;
pub const PM860X_OFFSET_RIGHT_2: u32 = 0xcf;
pub const PM860X_ADC_ANA_1: u32 = 0xd0;
pub const PM860X_ADC_ANA_2: u32 = 0xd1;
pub const PM860X_ADC_ANA_3: u32 = 0xd2;
pub const PM860X_ADC_ANA_4: u32 = 0xd3;
pub const PM860X_ANA_TO_ANA: u32 = 0xd4;
pub const PM860X_HS1_CTRL: u32 = 0xd5;
pub const PM860X_HS2_CTRL: u32 = 0xd6;
pub const PM860X_LO1_CTRL: u32 = 0xd7;
pub const PM860X_LO2_CTRL: u32 = 0xd8;
pub const PM860X_EAR_CTRL_1: u32 = 0xd9;
pub const PM860X_EAR_CTRL_2: u32 = 0xda;
pub const PM860X_AUDIO_SUPPLIES_1: u32 = 0xdb;
pub const PM860X_AUDIO_SUPPLIES_2: u32 = 0xdc;
pub const PM860X_ADC_EN_1: u32 = 0xdd;
pub const PM860X_ADC_EN_2: u32 = 0xde;
pub const PM860X_DAC_EN_1: u32 = 0xdf;
pub const PM860X_DAC_EN_2: u32 = 0xe1;
pub const PM860X_AUDIO_CAL_1: u32 = 0xe2;
pub const PM860X_AUDIO_CAL_2: u32 = 0xe3;
pub const PM860X_AUDIO_CAL_3: u32 = 0xe4;
pub const PM860X_AUDIO_CAL_4: u32 = 0xe5;
pub const PM860X_AUDIO_CAL_5: u32 = 0xe6;
pub const PM860X_ANA_INPUT_SEL_1: u32 = 0xe7;
pub const PM860X_ANA_INPUT_SEL_2: u32 = 0xe8;

pub const PM860X_PCM_IFACE_4: u32 = 0xe9;
pub const PM860X_I2S_IFACE_5: u32 = 0xea;

pub const PM860X_SHORTS: u32 = 0x3b;
pub const PM860X_PLL_ADJ_1: u32 = 0x3c;
pub const PM860X_PLL_ADJ_2: u32 = 0x3d;

/* bits definition */
pub const PM860X_CLK_DIR_IN: u32 = 0;
pub const PM860X_CLK_DIR_OUT: u32 = 1;

pub const PM860X_DET_HEADSET: u32 = 1 << 0;
pub const PM860X_DET_MIC: u32 = 1 << 1;
pub const PM860X_DET_HOOK: u32 = 1 << 2;
pub const PM860X_SHORT_HEADSET: u32 = 1 << 3;
pub const PM860X_SHORT_LINEOUT: u32 = 1 << 4;
pub const PM860X_DET_MASK: u32 = 0x1F;

unsafe extern "C" {
    pub fn pm860x_hs_jack_detect(
        arg1: *mut snd_soc_component,
        arg2: *mut snd_soc_jack,
        arg3: core::ffi::c_int,
        arg4: core::ffi::c_int,
        arg5: core::ffi::c_int,
        arg6: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub fn pm860x_mic_jack_detect(
        arg1: *mut snd_soc_component,
        arg2: *mut snd_soc_jack,
        arg3: core::ffi::c_int,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
