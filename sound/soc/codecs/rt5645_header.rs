/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rt5645.h  --  RT5645 ALSA SoC audio driver
 *
 * Copyright 2013 Realtek Microelectronics
 * Author: Bard Liao <bardliao@realtek.com>
 */

/* Translated from C header: include guard and C includes omitted. */
use core::ffi::c_char;

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rt5645.h  --  RT5645 ALSA SoC audio driver
 *
 * Copyright 2013 Realtek Microelectronics
 * Author: Bard Liao <bardliao@realtek.com>
 */


/* Info */
pub const RT5645_RESET: u32 = 0x00;
pub const RT5645_VENDOR_ID: u32 = 0xfd;
pub const RT5645_VENDOR_ID1: u32 = 0xfe;
pub const RT5645_VENDOR_ID2: u32 = 0xff;
/*  I/O - Output */
pub const RT5645_SPK_VOL: u32 = 0x01;
pub const RT5645_HP_VOL: u32 = 0x02;
pub const RT5645_LOUT1: u32 = 0x03;
pub const RT5645_LOUT_CTRL: u32 = 0x05;
/* I/O - Input */
pub const RT5645_IN1_CTRL1: u32 = 0x0a;
pub const RT5645_IN1_CTRL2: u32 = 0x0b;
pub const RT5645_IN1_CTRL3: u32 = 0x0c;
pub const RT5645_IN2_CTRL: u32 = 0x0d;
pub const RT5645_INL1_INR1_VOL: u32 = 0x0f;
pub const RT5645_SPK_FUNC_LIM: u32 = 0x14;
pub const RT5645_ADJ_HPF_CTRL: u32 = 0x16;
/* I/O - ADC/DAC/DMIC */
pub const RT5645_DAC1_DIG_VOL: u32 = 0x19;
pub const RT5645_DAC2_DIG_VOL: u32 = 0x1a;
pub const RT5645_DAC_CTRL: u32 = 0x1b;
pub const RT5645_STO1_ADC_DIG_VOL: u32 = 0x1c;
pub const RT5645_MONO_ADC_DIG_VOL: u32 = 0x1d;
pub const RT5645_ADC_BST_VOL1: u32 = 0x1e;
pub const RT5645_ADC_BST_VOL2: u32 = 0x20;
/* Mixer - D-D */
pub const RT5645_STO1_ADC_MIXER: u32 = 0x27;
pub const RT5645_MONO_ADC_MIXER: u32 = 0x28;
pub const RT5645_AD_DA_MIXER: u32 = 0x29;
pub const RT5645_STO_DAC_MIXER: u32 = 0x2a;
pub const RT5645_MONO_DAC_MIXER: u32 = 0x2b;
pub const RT5645_DIG_MIXER: u32 = 0x2c;
pub const RT5650_A_DAC_SOUR: u32 = 0x2d;
pub const RT5645_DIG_INF1_DATA: u32 = 0x2f;
/* Mixer - PDM */
pub const RT5645_PDM_OUT_CTRL: u32 = 0x31;
/* Mixer - ADC */
pub const RT5645_REC_L1_MIXER: u32 = 0x3b;
pub const RT5645_REC_L2_MIXER: u32 = 0x3c;
pub const RT5645_REC_R1_MIXER: u32 = 0x3d;
pub const RT5645_REC_R2_MIXER: u32 = 0x3e;
/* Mixer - DAC */
pub const RT5645_HPMIXL_CTRL: u32 = 0x3f;
pub const RT5645_HPOMIXL_CTRL: u32 = 0x40;
pub const RT5645_HPMIXR_CTRL: u32 = 0x41;
pub const RT5645_HPOMIXR_CTRL: u32 = 0x42;
pub const RT5645_HPO_MIXER: u32 = 0x45;
pub const RT5645_SPK_L_MIXER: u32 = 0x46;
pub const RT5645_SPK_R_MIXER: u32 = 0x47;
pub const RT5645_SPO_MIXER: u32 = 0x48;
pub const RT5645_SPO_CLSD_RATIO: u32 = 0x4a;
pub const RT5645_OUT_L_GAIN1: u32 = 0x4d;
pub const RT5645_OUT_L_GAIN2: u32 = 0x4e;
pub const RT5645_OUT_L1_MIXER: u32 = 0x4f;
pub const RT5645_OUT_R_GAIN1: u32 = 0x50;
pub const RT5645_OUT_R_GAIN2: u32 = 0x51;
pub const RT5645_OUT_R1_MIXER: u32 = 0x52;
pub const RT5645_LOUT_MIXER: u32 = 0x53;
/* Haptic */
pub const RT5645_HAPTIC_CTRL1: u32 = 0x56;
pub const RT5645_HAPTIC_CTRL2: u32 = 0x57;
pub const RT5645_HAPTIC_CTRL3: u32 = 0x58;
pub const RT5645_HAPTIC_CTRL4: u32 = 0x59;
pub const RT5645_HAPTIC_CTRL5: u32 = 0x5a;
pub const RT5645_HAPTIC_CTRL6: u32 = 0x5b;
pub const RT5645_HAPTIC_CTRL7: u32 = 0x5c;
pub const RT5645_HAPTIC_CTRL8: u32 = 0x5d;
pub const RT5645_HAPTIC_CTRL9: u32 = 0x5e;
pub const RT5645_HAPTIC_CTRL10: u32 = 0x5f;
/* Power */
pub const RT5645_PWR_DIG1: u32 = 0x61;
pub const RT5645_PWR_DIG2: u32 = 0x62;
pub const RT5645_PWR_ANLG1: u32 = 0x63;
pub const RT5645_PWR_ANLG2: u32 = 0x64;
pub const RT5645_PWR_MIXER: u32 = 0x65;
pub const RT5645_PWR_VOL: u32 = 0x66;
/* Private Register Control */
pub const RT5645_PRIV_INDEX: u32 = 0x6a;
pub const RT5645_PRIV_DATA: u32 = 0x6c;
/* Format - ADC/DAC */
pub const RT5645_I2S1_SDP: u32 = 0x70;
pub const RT5645_I2S2_SDP: u32 = 0x71;
pub const RT5645_ADDA_CLK1: u32 = 0x73;
pub const RT5645_ADDA_CLK2: u32 = 0x74;
pub const RT5645_DMIC_CTRL1: u32 = 0x75;
pub const RT5645_DMIC_CTRL2: u32 = 0x76;
/* Format - TDM Control */
pub const RT5645_TDM_CTRL_1: u32 = 0x77;
pub const RT5645_TDM_CTRL_2: u32 = 0x78;
pub const RT5645_TDM_CTRL_3: u32 = 0x79;
pub const RT5650_TDM_CTRL_4: u32 = 0x7a;

/* Function - Analog */
pub const RT5645_GLB_CLK: u32 = 0x80;
pub const RT5645_PLL_CTRL1: u32 = 0x81;
pub const RT5645_PLL_CTRL2: u32 = 0x82;
pub const RT5645_ASRC_1: u32 = 0x83;
pub const RT5645_ASRC_2: u32 = 0x84;
pub const RT5645_ASRC_3: u32 = 0x85;
pub const RT5645_ASRC_4: u32 = 0x8a;
pub const RT5645_DEPOP_M1: u32 = 0x8e;
pub const RT5645_DEPOP_M2: u32 = 0x8f;
pub const RT5645_DEPOP_M3: u32 = 0x90;
pub const RT5645_CHARGE_PUMP: u32 = 0x91;
pub const RT5645_MICBIAS: u32 = 0x93;
pub const RT5645_A_JD_CTRL1: u32 = 0x94;
pub const RT5645_VAD_CTRL4: u32 = 0x9d;
pub const RT5645_CLSD_OUT_CTRL: u32 = 0xa0;
pub const RT5645_CLSD_OUT_CTRL1: u32 = 0xa1;
/* Function - Digital */
pub const RT5645_ADC_EQ_CTRL1: u32 = 0xae;
pub const RT5645_ADC_EQ_CTRL2: u32 = 0xaf;
pub const RT5645_EQ_CTRL1: u32 = 0xb0;
pub const RT5645_EQ_CTRL2: u32 = 0xb1;
pub const RT5645_ALC_CTRL_1: u32 = 0xb3;
pub const RT5645_ALC_CTRL_2: u32 = 0xb4;
pub const RT5645_ALC_CTRL_3: u32 = 0xb5;
pub const RT5645_ALC_CTRL_4: u32 = 0xb6;
pub const RT5645_ALC_CTRL_5: u32 = 0xb7;
pub const RT5645_JD_CTRL: u32 = 0xbb;
pub const RT5645_IRQ_CTRL1: u32 = 0xbc;
pub const RT5645_IRQ_CTRL2: u32 = 0xbd;
pub const RT5645_IRQ_CTRL3: u32 = 0xbe;
pub const RT5645_INT_IRQ_ST: u32 = 0xbf;
pub const RT5645_GPIO_CTRL1: u32 = 0xc0;
pub const RT5645_GPIO_CTRL2: u32 = 0xc1;
pub const RT5645_GPIO_CTRL3: u32 = 0xc2;
pub const RT5645_BASS_BACK: u32 = 0xcf;
pub const RT5645_MP3_PLUS1: u32 = 0xd0;
pub const RT5645_MP3_PLUS2: u32 = 0xd1;
pub const RT5645_ADJ_HPF1: u32 = 0xd3;
pub const RT5645_ADJ_HPF2: u32 = 0xd4;
pub const RT5645_HP_CALIB_AMP_DET: u32 = 0xd6;
pub const RT5645_SV_ZCD1: u32 = 0xd9;
pub const RT5645_SV_ZCD2: u32 = 0xda;
pub const RT5645_IL_CMD: u32 = 0xdb;
pub const RT5645_IL_CMD2: u32 = 0xdc;
pub const RT5645_IL_CMD3: u32 = 0xdd;
pub const RT5650_4BTN_IL_CMD1: u32 = 0xdf;
pub const RT5650_4BTN_IL_CMD2: u32 = 0xe0;
pub const RT5645_DRC1_HL_CTRL1: u32 = 0xe7;
pub const RT5645_DRC2_HL_CTRL1: u32 = 0xe9;
pub const RT5645_MUTI_DRC_CTRL1: u32 = 0xea;
pub const RT5645_ADC_MONO_HP_CTRL1: u32 = 0xec;
pub const RT5645_ADC_MONO_HP_CTRL2: u32 = 0xed;
pub const RT5645_DRC2_CTRL1: u32 = 0xf0;
pub const RT5645_DRC2_CTRL2: u32 = 0xf1;
pub const RT5645_DRC2_CTRL3: u32 = 0xf2;
pub const RT5645_DRC2_CTRL4: u32 = 0xf3;
pub const RT5645_DRC2_CTRL5: u32 = 0xf4;
pub const RT5645_JD_CTRL3: u32 = 0xf8;
pub const RT5645_JD_CTRL4: u32 = 0xf9;
/* General Control */
pub const RT5645_GEN_CTRL1: u32 = 0xfa;
pub const RT5645_GEN_CTRL2: u32 = 0xfb;
pub const RT5645_GEN_CTRL3: u32 = 0xfc;


/* Index of Codec Private Register definition */
pub const RT5645_DIG_VOL: u32 = 0x00;
pub const RT5645_PR_ALC_CTRL_1: u32 = 0x01;
pub const RT5645_PR_ALC_CTRL_2: u32 = 0x02;
pub const RT5645_PR_ALC_CTRL_3: u32 = 0x03;
pub const RT5645_PR_ALC_CTRL_4: u32 = 0x04;
pub const RT5645_PR_ALC_CTRL_5: u32 = 0x05;
pub const RT5645_PR_ALC_CTRL_6: u32 = 0x06;
pub const RT5645_BIAS_CUR1: u32 = 0x12;
pub const RT5645_BIAS_CUR3: u32 = 0x14;
pub const RT5645_CLSD_INT_REG1: u32 = 0x1c;
pub const RT5645_MAMP_INT_REG2: u32 = 0x37;
pub const RT5645_CHOP_DAC_ADC: u32 = 0x3d;
pub const RT5645_MIXER_INT_REG: u32 = 0x3f;
pub const RT5645_3D_SPK: u32 = 0x63;
pub const RT5645_WND_1: u32 = 0x6c;
pub const RT5645_WND_2: u32 = 0x6d;
pub const RT5645_WND_3: u32 = 0x6e;
pub const RT5645_WND_4: u32 = 0x6f;
pub const RT5645_WND_5: u32 = 0x70;
pub const RT5645_WND_8: u32 = 0x73;
pub const RT5645_DIP_SPK_INF: u32 = 0x75;
pub const RT5645_HP_DCC_INT1: u32 = 0x77;
pub const RT5645_EQ_BW_LOP: u32 = 0xa0;
pub const RT5645_EQ_GN_LOP: u32 = 0xa1;
pub const RT5645_EQ_FC_BP1: u32 = 0xa2;
pub const RT5645_EQ_BW_BP1: u32 = 0xa3;
pub const RT5645_EQ_GN_BP1: u32 = 0xa4;
pub const RT5645_EQ_FC_BP2: u32 = 0xa5;
pub const RT5645_EQ_BW_BP2: u32 = 0xa6;
pub const RT5645_EQ_GN_BP2: u32 = 0xa7;
pub const RT5645_EQ_FC_BP3: u32 = 0xa8;
pub const RT5645_EQ_BW_BP3: u32 = 0xa9;
pub const RT5645_EQ_GN_BP3: u32 = 0xaa;
pub const RT5645_EQ_FC_BP4: u32 = 0xab;
pub const RT5645_EQ_BW_BP4: u32 = 0xac;
pub const RT5645_EQ_GN_BP4: u32 = 0xad;
pub const RT5645_EQ_FC_HIP1: u32 = 0xae;
pub const RT5645_EQ_GN_HIP1: u32 = 0xaf;
pub const RT5645_EQ_FC_HIP2: u32 = 0xb0;
pub const RT5645_EQ_BW_HIP2: u32 = 0xb1;
pub const RT5645_EQ_GN_HIP2: u32 = 0xb2;
pub const RT5645_EQ_PRE_VOL: u32 = 0xb3;
pub const RT5645_EQ_PST_VOL: u32 = 0xb4;


/* global definition */
pub const RT5645_L_MUTE: u32 = 0x1 << 15;
pub const RT5645_L_MUTE_SFT: u32 = 15;
pub const RT5645_VOL_L_MUTE: u32 = 0x1 << 14;
pub const RT5645_VOL_L_SFT: u32 = 14;
pub const RT5645_R_MUTE: u32 = 0x1 << 7;
pub const RT5645_R_MUTE_SFT: u32 = 7;
pub const RT5645_VOL_R_MUTE: u32 = 0x1 << 6;
pub const RT5645_VOL_R_SFT: u32 = 6;
pub const RT5645_L_VOL_MASK: u32 = 0x3f << 8;
pub const RT5645_L_VOL_SFT: u32 = 8;
pub const RT5645_R_VOL_MASK: u32 = 0x3f;
pub const RT5645_R_VOL_SFT: u32 = 0;

/* IN1 Control 1 (0x0a) */
pub const RT5645_CBJ_BST1_MASK: u32 = 0xf << 12;
pub const RT5645_CBJ_BST1_SFT: u32 = 12;
pub const RT5645_CBJ_JD_HP_EN: u32 = 0x1 << 9;
pub const RT5645_CBJ_JD_MIC_EN: u32 = 0x1 << 8;
pub const RT5645_CBJ_JD_MIC_SW_EN: u32 = 0x1 << 7;
pub const RT5645_CBJ_MIC_SEL_R: u32 = 0x1 << 6;
pub const RT5645_CBJ_MIC_SEL_L: u32 = 0x1 << 5;
pub const RT5645_CBJ_MIC_SW: u32 = 0x1 << 4;
pub const RT5645_CBJ_BST1_EN: u32 = 0x1 << 2;

/* IN1 Control 2 (0x0b) */
pub const RT5645_CBJ_MN_JD: u32 = 0x1 << 12;
pub const RT5645_CAPLESS_EN: u32 = 0x1 << 11;
pub const RT5645_CBJ_DET_MODE: u32 = 0x1 << 7;

/* IN1 Control 3 (0x0c) */
pub const RT5645_CBJ_TIE_G_L: u32 = 0x1 << 15;
pub const RT5645_CBJ_TIE_G_R: u32 = 0x1 << 14;

/* IN2 Control (0x0d) */
pub const RT5645_BST_MASK1: u32 = 0xf << 12;
pub const RT5645_BST_SFT1: u32 = 12;
pub const RT5645_BST_MASK2: u32 = 0xf << 8;
pub const RT5645_BST_SFT2: u32 = 8;
pub const RT5645_IN_DF2: u32 = 0x1 << 6;
pub const RT5645_IN_SFT2: u32 = 6;

/* INL and INR Volume Control (0x0f) */
pub const RT5645_INL_SEL_MASK: u32 = 0x1 << 15;
pub const RT5645_INL_SEL_SFT: u32 = 15;
pub const RT5645_INL_SEL_IN4P: u32 = 0x0 << 15;
pub const RT5645_INL_SEL_MONOP: u32 = 0x1 << 15;
pub const RT5645_INL_VOL_MASK: u32 = 0x1f << 8;
pub const RT5645_INL_VOL_SFT: u32 = 8;
pub const RT5645_INR_SEL_MASK: u32 = 0x1 << 7;
pub const RT5645_INR_SEL_SFT: u32 = 7;
pub const RT5645_INR_SEL_IN4N: u32 = 0x0 << 7;
pub const RT5645_INR_SEL_MONON: u32 = 0x1 << 7;
pub const RT5645_INR_VOL_MASK: u32 = 0x1f;
pub const RT5645_INR_VOL_SFT: u32 = 0;

/* DAC1 Digital Volume (0x19) */
pub const RT5645_DAC_L1_VOL_MASK: u32 = 0xff << 8;
pub const RT5645_DAC_L1_VOL_SFT: u32 = 8;
pub const RT5645_DAC_R1_VOL_MASK: u32 = 0xff;
pub const RT5645_DAC_R1_VOL_SFT: u32 = 0;

/* DAC2 Digital Volume (0x1a) */
pub const RT5645_DAC_L2_VOL_MASK: u32 = 0xff << 8;
pub const RT5645_DAC_L2_VOL_SFT: u32 = 8;
pub const RT5645_DAC_R2_VOL_MASK: u32 = 0xff;
pub const RT5645_DAC_R2_VOL_SFT: u32 = 0;

/* DAC2 Control (0x1b) */
pub const RT5645_M_DAC_L2_VOL: u32 = 0x1 << 13;
pub const RT5645_M_DAC_L2_VOL_SFT: u32 = 13;
pub const RT5645_M_DAC_R2_VOL: u32 = 0x1 << 12;
pub const RT5645_M_DAC_R2_VOL_SFT: u32 = 12;
pub const RT5645_DAC2_L_SEL_MASK: u32 = 0x7 << 4;
pub const RT5645_DAC2_L_SEL_SFT: u32 = 4;
pub const RT5645_DAC2_R_SEL_MASK: u32 = 0x7 << 0;
pub const RT5645_DAC2_R_SEL_SFT: u32 = 0;

/* ADC Digital Volume Control (0x1c) */
pub const RT5645_ADC_L_VOL_MASK: u32 = 0x7f << 8;
pub const RT5645_ADC_L_VOL_SFT: u32 = 8;
pub const RT5645_ADC_R_VOL_MASK: u32 = 0x7f;
pub const RT5645_ADC_R_VOL_SFT: u32 = 0;

/* Mono ADC Digital Volume Control (0x1d) */
pub const RT5645_MONO_ADC_L_VOL_MASK: u32 = 0x7f << 8;
pub const RT5645_MONO_ADC_L_VOL_SFT: u32 = 8;
pub const RT5645_MONO_ADC_R_VOL_MASK: u32 = 0x7f;
pub const RT5645_MONO_ADC_R_VOL_SFT: u32 = 0;

/* ADC Boost Volume Control (0x1e) */
pub const RT5645_STO1_ADC_L_BST_MASK: u32 = 0x3 << 14;
pub const RT5645_STO1_ADC_L_BST_SFT: u32 = 14;
pub const RT5645_STO1_ADC_R_BST_MASK: u32 = 0x3 << 12;
pub const RT5645_STO1_ADC_R_BST_SFT: u32 = 12;
pub const RT5645_STO1_ADC_COMP_MASK: u32 = 0x3 << 10;
pub const RT5645_STO1_ADC_COMP_SFT: u32 = 10;

/* ADC Boost Volume Control (0x20) */
pub const RT5645_MONO_ADC_L_BST_MASK: u32 = 0x3 << 14;
pub const RT5645_MONO_ADC_L_BST_SFT: u32 = 14;
pub const RT5645_MONO_ADC_R_BST_MASK: u32 = 0x3 << 12;
pub const RT5645_MONO_ADC_R_BST_SFT: u32 = 12;
pub const RT5645_MONO_ADC_COMP_MASK: u32 = 0x3 << 10;
pub const RT5645_MONO_ADC_COMP_SFT: u32 = 10;

/* Stereo2 ADC Mixer Control (0x26) */
pub const RT5645_STO2_ADC_SRC_MASK: u32 = 0x1 << 15;
pub const RT5645_STO2_ADC_SRC_SFT: u32 = 15;

/* Stereo ADC Mixer Control (0x27) */
pub const RT5645_M_ADC_L1: u32 = 0x1 << 14;
pub const RT5645_M_ADC_L1_SFT: u32 = 14;
pub const RT5645_M_ADC_L2: u32 = 0x1 << 13;
pub const RT5645_M_ADC_L2_SFT: u32 = 13;
pub const RT5645_ADC_1_SRC_MASK: u32 = 0x1 << 12;
pub const RT5645_ADC_1_SRC_SFT: u32 = 12;
pub const RT5645_ADC_1_SRC_ADC: u32 = 0x1 << 12;
pub const RT5645_ADC_1_SRC_DACMIX: u32 = 0x0 << 12;
pub const RT5645_ADC_2_SRC_MASK: u32 = 0x1 << 11;
pub const RT5645_ADC_2_SRC_SFT: u32 = 11;
pub const RT5645_DMIC_SRC_MASK: u32 = 0x1 << 8;
pub const RT5645_DMIC_SRC_SFT: u32 = 8;
pub const RT5645_M_ADC_R1: u32 = 0x1 << 6;
pub const RT5645_M_ADC_R1_SFT: u32 = 6;
pub const RT5645_M_ADC_R2: u32 = 0x1 << 5;
pub const RT5645_M_ADC_R2_SFT: u32 = 5;
pub const RT5645_DMIC3_SRC_MASK: u32 = 0x1 << 1;
pub const RT5645_DMIC3_SRC_SFT: u32 = 0;

/* Mono ADC Mixer Control (0x28) */
pub const RT5645_M_MONO_ADC_L1: u32 = 0x1 << 14;
pub const RT5645_M_MONO_ADC_L1_SFT: u32 = 14;
pub const RT5645_M_MONO_ADC_L2: u32 = 0x1 << 13;
pub const RT5645_M_MONO_ADC_L2_SFT: u32 = 13;
pub const RT5645_MONO_ADC_L1_SRC_MASK: u32 = 0x1 << 12;
pub const RT5645_MONO_ADC_L1_SRC_SFT: u32 = 12;
pub const RT5645_MONO_ADC_L1_SRC_DACMIXL: u32 = 0x0 << 12;
pub const RT5645_MONO_ADC_L1_SRC_ADCL: u32 = 0x1 << 12;
pub const RT5645_MONO_ADC_L2_SRC_MASK: u32 = 0x1 << 11;
pub const RT5645_MONO_ADC_L2_SRC_SFT: u32 = 11;
pub const RT5645_MONO_DMIC_L_SRC_MASK: u32 = 0x1 << 8;
pub const RT5645_MONO_DMIC_L_SRC_SFT: u32 = 8;
pub const RT5645_M_MONO_ADC_R1: u32 = 0x1 << 6;
pub const RT5645_M_MONO_ADC_R1_SFT: u32 = 6;
pub const RT5645_M_MONO_ADC_R2: u32 = 0x1 << 5;
pub const RT5645_M_MONO_ADC_R2_SFT: u32 = 5;
pub const RT5645_MONO_ADC_R1_SRC_MASK: u32 = 0x1 << 4;
pub const RT5645_MONO_ADC_R1_SRC_SFT: u32 = 4;
pub const RT5645_MONO_ADC_R1_SRC_ADCR: u32 = 0x1 << 4;
pub const RT5645_MONO_ADC_R1_SRC_DACMIXR: u32 = 0x0 << 4;
pub const RT5645_MONO_ADC_R2_SRC_MASK: u32 = 0x1 << 3;
pub const RT5645_MONO_ADC_R2_SRC_SFT: u32 = 3;
pub const RT5645_MONO_DMIC_R_SRC_MASK: u32 = 0x3;
pub const RT5645_MONO_DMIC_R_SRC_SFT: u32 = 0;

/* ADC Mixer to DAC Mixer Control (0x29) */
pub const RT5645_M_ADCMIX_L: u32 = 0x1 << 15;
pub const RT5645_M_ADCMIX_L_SFT: u32 = 15;
pub const RT5645_M_DAC1_L: u32 = 0x1 << 14;
pub const RT5645_M_DAC1_L_SFT: u32 = 14;
pub const RT5645_DAC1_R_SEL_MASK: u32 = 0x3 << 10;
pub const RT5645_DAC1_R_SEL_SFT: u32 = 10;
pub const RT5645_DAC1_R_SEL_IF1: u32 = 0x0 << 10;
pub const RT5645_DAC1_R_SEL_IF2: u32 = 0x1 << 10;
pub const RT5645_DAC1_R_SEL_IF3: u32 = 0x2 << 10;
pub const RT5645_DAC1_R_SEL_IF4: u32 = 0x3 << 10;
pub const RT5645_DAC1_L_SEL_MASK: u32 = 0x3 << 8;
pub const RT5645_DAC1_L_SEL_SFT: u32 = 8;
pub const RT5645_DAC1_L_SEL_IF1: u32 = 0x0 << 8;
pub const RT5645_DAC1_L_SEL_IF2: u32 = 0x1 << 8;
pub const RT5645_DAC1_L_SEL_IF3: u32 = 0x2 << 8;
pub const RT5645_DAC1_L_SEL_IF4: u32 = 0x3 << 8;
pub const RT5645_M_ADCMIX_R: u32 = 0x1 << 7;
pub const RT5645_M_ADCMIX_R_SFT: u32 = 7;
pub const RT5645_M_DAC1_R: u32 = 0x1 << 6;
pub const RT5645_M_DAC1_R_SFT: u32 = 6;

/* Stereo DAC Mixer Control (0x2a) */
pub const RT5645_M_DAC_L1: u32 = 0x1 << 14;
pub const RT5645_M_DAC_L1_SFT: u32 = 14;
pub const RT5645_DAC_L1_STO_L_VOL_MASK: u32 = 0x1 << 13;
pub const RT5645_DAC_L1_STO_L_VOL_SFT: u32 = 13;
pub const RT5645_M_DAC_L2: u32 = 0x1 << 12;
pub const RT5645_M_DAC_L2_SFT: u32 = 12;
pub const RT5645_DAC_L2_STO_L_VOL_MASK: u32 = 0x1 << 11;
pub const RT5645_DAC_L2_STO_L_VOL_SFT: u32 = 11;
pub const RT5645_M_ANC_DAC_L: u32 = 0x1 << 10;
pub const RT5645_M_ANC_DAC_L_SFT: u32 = 10;
pub const RT5645_M_DAC_R1_STO_L: u32 = 0x1 << 9;
pub const RT5645_M_DAC_R1_STO_L_SFT: u32 = 9;
pub const RT5645_DAC_R1_STO_L_VOL_MASK: u32 = 0x1 << 8;
pub const RT5645_DAC_R1_STO_L_VOL_SFT: u32 = 8;
pub const RT5645_M_DAC_R1: u32 = 0x1 << 6;
pub const RT5645_M_DAC_R1_SFT: u32 = 6;
pub const RT5645_DAC_R1_STO_R_VOL_MASK: u32 = 0x1 << 5;
pub const RT5645_DAC_R1_STO_R_VOL_SFT: u32 = 5;
pub const RT5645_M_DAC_R2: u32 = 0x1 << 4;
pub const RT5645_M_DAC_R2_SFT: u32 = 4;
pub const RT5645_DAC_R2_STO_R_VOL_MASK: u32 = 0x1 << 3;
pub const RT5645_DAC_R2_STO_R_VOL_SFT: u32 = 3;
pub const RT5645_M_ANC_DAC_R: u32 = 0x1 << 2;
pub const RT5645_M_ANC_DAC_R_SFT: u32 = 2;
pub const RT5645_M_DAC_L1_STO_R: u32 = 0x1 << 1;
pub const RT5645_M_DAC_L1_STO_R_SFT: u32 = 1;
pub const RT5645_DAC_L1_STO_R_VOL_MASK: u32 = 0x1;
pub const RT5645_DAC_L1_STO_R_VOL_SFT: u32 = 0;

/* Mono DAC Mixer Control (0x2b) */
pub const RT5645_M_DAC_L1_MONO_L: u32 = 0x1 << 14;
pub const RT5645_M_DAC_L1_MONO_L_SFT: u32 = 14;
pub const RT5645_DAC_L1_MONO_L_VOL_MASK: u32 = 0x1 << 13;
pub const RT5645_DAC_L1_MONO_L_VOL_SFT: u32 = 13;
pub const RT5645_M_DAC_L2_MONO_L: u32 = 0x1 << 12;
pub const RT5645_M_DAC_L2_MONO_L_SFT: u32 = 12;
pub const RT5645_DAC_L2_MONO_L_VOL_MASK: u32 = 0x1 << 11;
pub const RT5645_DAC_L2_MONO_L_VOL_SFT: u32 = 11;
pub const RT5645_M_DAC_R2_MONO_L: u32 = 0x1 << 10;
pub const RT5645_M_DAC_R2_MONO_L_SFT: u32 = 10;
pub const RT5645_DAC_R2_MONO_L_VOL_MASK: u32 = 0x1 << 9;
pub const RT5645_DAC_R2_MONO_L_VOL_SFT: u32 = 9;
pub const RT5645_M_DAC_R1_MONO_R: u32 = 0x1 << 6;
pub const RT5645_M_DAC_R1_MONO_R_SFT: u32 = 6;
pub const RT5645_DAC_R1_MONO_R_VOL_MASK: u32 = 0x1 << 5;
pub const RT5645_DAC_R1_MONO_R_VOL_SFT: u32 = 5;
pub const RT5645_M_DAC_R2_MONO_R: u32 = 0x1 << 4;
pub const RT5645_M_DAC_R2_MONO_R_SFT: u32 = 4;
pub const RT5645_DAC_R2_MONO_R_VOL_MASK: u32 = 0x1 << 3;
pub const RT5645_DAC_R2_MONO_R_VOL_SFT: u32 = 3;
pub const RT5645_M_DAC_L2_MONO_R: u32 = 0x1 << 2;
pub const RT5645_M_DAC_L2_MONO_R_SFT: u32 = 2;
pub const RT5645_DAC_L2_MONO_R_VOL_MASK: u32 = 0x1 << 1;
pub const RT5645_DAC_L2_MONO_R_VOL_SFT: u32 = 1;

/* Digital Mixer Control (0x2c) */
pub const RT5645_M_STO_L_DAC_L: u32 = 0x1 << 15;
pub const RT5645_M_STO_L_DAC_L_SFT: u32 = 15;
pub const RT5645_STO_L_DAC_L_VOL_MASK: u32 = 0x1 << 14;
pub const RT5645_STO_L_DAC_L_VOL_SFT: u32 = 14;
pub const RT5645_M_DAC_L2_DAC_L: u32 = 0x1 << 13;
pub const RT5645_M_DAC_L2_DAC_L_SFT: u32 = 13;
pub const RT5645_DAC_L2_DAC_L_VOL_MASK: u32 = 0x1 << 12;
pub const RT5645_DAC_L2_DAC_L_VOL_SFT: u32 = 12;
pub const RT5645_M_STO_R_DAC_R: u32 = 0x1 << 11;
pub const RT5645_M_STO_R_DAC_R_SFT: u32 = 11;
pub const RT5645_STO_R_DAC_R_VOL_MASK: u32 = 0x1 << 10;
pub const RT5645_STO_R_DAC_R_VOL_SFT: u32 = 10;
pub const RT5645_M_DAC_R2_DAC_R: u32 = 0x1 << 9;
pub const RT5645_M_DAC_R2_DAC_R_SFT: u32 = 9;
pub const RT5645_DAC_R2_DAC_R_VOL_MASK: u32 = 0x1 << 8;
pub const RT5645_DAC_R2_DAC_R_VOL_SFT: u32 = 8;
pub const RT5645_M_DAC_R2_DAC_L: u32 = 0x1 << 7;
pub const RT5645_M_DAC_R2_DAC_L_SFT: u32 = 7;
pub const RT5645_DAC_R2_DAC_L_VOL_MASK: u32 = 0x1 << 6;
pub const RT5645_DAC_R2_DAC_L_VOL_SFT: u32 = 6;
pub const RT5645_M_DAC_L2_DAC_R: u32 = 0x1 << 5;
pub const RT5645_M_DAC_L2_DAC_R_SFT: u32 = 5;
pub const RT5645_DAC_L2_DAC_R_VOL_MASK: u32 = 0x1 << 4;
pub const RT5645_DAC_L2_DAC_R_VOL_SFT: u32 = 4;

/* Analog DAC1/2 Input Source Control (0x2d) */
pub const RT5650_A_DAC1_L_IN_SFT: u32 = 3;
pub const RT5650_A_DAC1_R_IN_SFT: u32 = 2;
pub const RT5650_A_DAC2_L_IN_SFT: u32 = 1;
pub const RT5650_A_DAC2_R_IN_SFT: u32 = 0;

/* Digital Interface Data Control (0x2f) */
pub const RT5645_IF1_ADC2_IN_SEL: u32 = 0x1 << 15;
pub const RT5645_IF1_ADC2_IN_SFT: u32 = 15;
pub const RT5645_IF2_ADC_IN_MASK: u32 = 0x7 << 12;
pub const RT5645_IF2_ADC_IN_SFT: u32 = 12;
pub const RT5645_IF2_DAC_SEL_MASK: u32 = 0x3 << 10;
pub const RT5645_IF2_DAC_SEL_SFT: u32 = 10;
pub const RT5645_IF2_ADC_SEL_MASK: u32 = 0x3 << 8;
pub const RT5645_IF2_ADC_SEL_SFT: u32 = 8;
pub const RT5645_IF3_DAC_SEL_MASK: u32 = 0x3 << 6;
pub const RT5645_IF3_DAC_SEL_SFT: u32 = 6;
pub const RT5645_IF3_ADC_SEL_MASK: u32 = 0x3 << 4;
pub const RT5645_IF3_ADC_SEL_SFT: u32 = 4;
pub const RT5645_IF3_ADC_IN_MASK: u32 = 0x7;
pub const RT5645_IF3_ADC_IN_SFT: u32 = 0;

/* PDM Output Control (0x31) */
pub const RT5645_PDM1_L_MASK: u32 = 0x1 << 15;
pub const RT5645_PDM1_L_SFT: u32 = 15;
pub const RT5645_M_PDM1_L: u32 = 0x1 << 14;
pub const RT5645_M_PDM1_L_SFT: u32 = 14;
pub const RT5645_PDM1_R_MASK: u32 = 0x1 << 13;
pub const RT5645_PDM1_R_SFT: u32 = 13;
pub const RT5645_M_PDM1_R: u32 = 0x1 << 12;
pub const RT5645_M_PDM1_R_SFT: u32 = 12;
pub const RT5645_PDM2_L_MASK: u32 = 0x1 << 11;
pub const RT5645_PDM2_L_SFT: u32 = 11;
pub const RT5645_M_PDM2_L: u32 = 0x1 << 10;
pub const RT5645_M_PDM2_L_SFT: u32 = 10;
pub const RT5645_PDM2_R_MASK: u32 = 0x1 << 9;
pub const RT5645_PDM2_R_SFT: u32 = 9;
pub const RT5645_M_PDM2_R: u32 = 0x1 << 8;
pub const RT5645_M_PDM2_R_SFT: u32 = 8;
pub const RT5645_PDM2_BUSY: u32 = 0x1 << 7;
pub const RT5645_PDM1_BUSY: u32 = 0x1 << 6;
pub const RT5645_PDM_PATTERN: u32 = 0x1 << 5;
pub const RT5645_PDM_GAIN: u32 = 0x1 << 4;
pub const RT5645_PDM_DIV_MASK: u32 = 0x3;

/* REC Left Mixer Control 1 (0x3b) */
pub const RT5645_G_HP_L_RM_L_MASK: u32 = 0x7 << 13;
pub const RT5645_G_HP_L_RM_L_SFT: u32 = 13;
pub const RT5645_G_IN_L_RM_L_MASK: u32 = 0x7 << 10;
pub const RT5645_G_IN_L_RM_L_SFT: u32 = 10;
pub const RT5645_G_BST4_RM_L_MASK: u32 = 0x7 << 7;
pub const RT5645_G_BST4_RM_L_SFT: u32 = 7;
pub const RT5645_G_BST3_RM_L_MASK: u32 = 0x7 << 4;
pub const RT5645_G_BST3_RM_L_SFT: u32 = 4;
pub const RT5645_G_BST2_RM_L_MASK: u32 = 0x7 << 1;
pub const RT5645_G_BST2_RM_L_SFT: u32 = 1;

/* REC Left Mixer Control 2 (0x3c) */
pub const RT5645_G_BST1_RM_L_MASK: u32 = 0x7 << 13;
pub const RT5645_G_BST1_RM_L_SFT: u32 = 13;
pub const RT5645_G_OM_L_RM_L_MASK: u32 = 0x7 << 10;
pub const RT5645_G_OM_L_RM_L_SFT: u32 = 10;
pub const RT5645_M_MM_L_RM_L: u32 = 0x1 << 6;
pub const RT5645_M_MM_L_RM_L_SFT: u32 = 6;
pub const RT5645_M_IN_L_RM_L: u32 = 0x1 << 5;
pub const RT5645_M_IN_L_RM_L_SFT: u32 = 5;
pub const RT5645_M_HP_L_RM_L: u32 = 0x1 << 4;
pub const RT5645_M_HP_L_RM_L_SFT: u32 = 4;
pub const RT5645_M_BST3_RM_L: u32 = 0x1 << 3;
pub const RT5645_M_BST3_RM_L_SFT: u32 = 3;
pub const RT5645_M_BST2_RM_L: u32 = 0x1 << 2;
pub const RT5645_M_BST2_RM_L_SFT: u32 = 2;
pub const RT5645_M_BST1_RM_L: u32 = 0x1 << 1;
pub const RT5645_M_BST1_RM_L_SFT: u32 = 1;
pub const RT5645_M_OM_L_RM_L: u32 = 0x1;
pub const RT5645_M_OM_L_RM_L_SFT: u32 = 0;

/* REC Right Mixer Control 1 (0x3d) */
pub const RT5645_G_HP_R_RM_R_MASK: u32 = 0x7 << 13;
pub const RT5645_G_HP_R_RM_R_SFT: u32 = 13;
pub const RT5645_G_IN_R_RM_R_MASK: u32 = 0x7 << 10;
pub const RT5645_G_IN_R_RM_R_SFT: u32 = 10;
pub const RT5645_G_BST4_RM_R_MASK: u32 = 0x7 << 7;
pub const RT5645_G_BST4_RM_R_SFT: u32 = 7;
pub const RT5645_G_BST3_RM_R_MASK: u32 = 0x7 << 4;
pub const RT5645_G_BST3_RM_R_SFT: u32 = 4;
pub const RT5645_G_BST2_RM_R_MASK: u32 = 0x7 << 1;
pub const RT5645_G_BST2_RM_R_SFT: u32 = 1;

/* REC Right Mixer Control 2 (0x3e) */
pub const RT5645_G_BST1_RM_R_MASK: u32 = 0x7 << 13;
pub const RT5645_G_BST1_RM_R_SFT: u32 = 13;
pub const RT5645_G_OM_R_RM_R_MASK: u32 = 0x7 << 10;
pub const RT5645_G_OM_R_RM_R_SFT: u32 = 10;
pub const RT5645_M_MM_R_RM_R: u32 = 0x1 << 6;
pub const RT5645_M_MM_R_RM_R_SFT: u32 = 6;
pub const RT5645_M_IN_R_RM_R: u32 = 0x1 << 5;
pub const RT5645_M_IN_R_RM_R_SFT: u32 = 5;
pub const RT5645_M_HP_R_RM_R: u32 = 0x1 << 4;
pub const RT5645_M_HP_R_RM_R_SFT: u32 = 4;
pub const RT5645_M_BST3_RM_R: u32 = 0x1 << 3;
pub const RT5645_M_BST3_RM_R_SFT: u32 = 3;
pub const RT5645_M_BST2_RM_R: u32 = 0x1 << 2;
pub const RT5645_M_BST2_RM_R_SFT: u32 = 2;
pub const RT5645_M_BST1_RM_R: u32 = 0x1 << 1;
pub const RT5645_M_BST1_RM_R_SFT: u32 = 1;
pub const RT5645_M_OM_R_RM_R: u32 = 0x1;
pub const RT5645_M_OM_R_RM_R_SFT: u32 = 0;

/* HPOMIX Control (0x40) (0x42) */
pub const RT5645_M_BST1_HV: u32 = 0x1 << 4;
pub const RT5645_M_BST1_HV_SFT: u32 = 4;
pub const RT5645_M_BST2_HV: u32 = 0x1 << 4;
pub const RT5645_M_BST2_HV_SFT: u32 = 4;
pub const RT5645_M_BST3_HV: u32 = 0x1 << 3;
pub const RT5645_M_BST3_HV_SFT: u32 = 3;
pub const RT5645_M_IN_HV: u32 = 0x1 << 2;
pub const RT5645_M_IN_HV_SFT: u32 = 2;
pub const RT5645_M_DAC2_HV: u32 = 0x1 << 1;
pub const RT5645_M_DAC2_HV_SFT: u32 = 1;
pub const RT5645_M_DAC1_HV: u32 = 0x1 << 0;
pub const RT5645_M_DAC1_HV_SFT: u32 = 0;

/* HPMIX Control (0x45) */
pub const RT5645_M_DAC1_HM: u32 = 0x1 << 14;
pub const RT5645_M_DAC1_HM_SFT: u32 = 14;
pub const RT5645_M_HPVOL_HM: u32 = 0x1 << 13;
pub const RT5645_M_HPVOL_HM_SFT: u32 = 13;
pub const RT5645_IRQ_PSV_MODE: u32 = 0x1 << 12;

/* SPK Left Mixer Control (0x46) */
pub const RT5645_G_RM_L_SM_L_MASK: u32 = 0x3 << 14;
pub const RT5645_G_RM_L_SM_L_SFT: u32 = 14;
pub const RT5645_G_IN_L_SM_L_MASK: u32 = 0x3 << 12;
pub const RT5645_G_IN_L_SM_L_SFT: u32 = 12;
pub const RT5645_G_DAC_L1_SM_L_MASK: u32 = 0x3 << 10;
pub const RT5645_G_DAC_L1_SM_L_SFT: u32 = 10;
pub const RT5645_G_DAC_L2_SM_L_MASK: u32 = 0x3 << 8;
pub const RT5645_G_DAC_L2_SM_L_SFT: u32 = 8;
pub const RT5645_G_OM_L_SM_L_MASK: u32 = 0x3 << 6;
pub const RT5645_G_OM_L_SM_L_SFT: u32 = 6;
pub const RT5645_M_BST1_L_SM_L: u32 = 0x1 << 5;
pub const RT5645_M_BST1_L_SM_L_SFT: u32 = 5;
pub const RT5645_M_BST3_L_SM_L: u32 = 0x1 << 4;
pub const RT5645_M_BST3_L_SM_L_SFT: u32 = 4;
pub const RT5645_M_IN_L_SM_L: u32 = 0x1 << 3;
pub const RT5645_M_IN_L_SM_L_SFT: u32 = 3;
pub const RT5645_M_DAC_L2_SM_L: u32 = 0x1 << 2;
pub const RT5645_M_DAC_L2_SM_L_SFT: u32 = 2;
pub const RT5645_M_DAC_L1_SM_L: u32 = 0x1 << 1;
pub const RT5645_M_DAC_L1_SM_L_SFT: u32 = 1;

/* SPK Right Mixer Control (0x47) */
pub const RT5645_G_RM_R_SM_R_MASK: u32 = 0x3 << 14;
pub const RT5645_G_RM_R_SM_R_SFT: u32 = 14;
pub const RT5645_G_IN_R_SM_R_MASK: u32 = 0x3 << 12;
pub const RT5645_G_IN_R_SM_R_SFT: u32 = 12;
pub const RT5645_G_DAC_R1_SM_R_MASK: u32 = 0x3 << 10;
pub const RT5645_G_DAC_R1_SM_R_SFT: u32 = 10;
pub const RT5645_G_DAC_R2_SM_R_MASK: u32 = 0x3 << 8;
pub const RT5645_G_DAC_R2_SM_R_SFT: u32 = 8;
pub const RT5645_G_OM_R_SM_R_MASK: u32 = 0x3 << 6;
pub const RT5645_G_OM_R_SM_R_SFT: u32 = 6;
pub const RT5645_M_BST2_R_SM_R: u32 = 0x1 << 5;
pub const RT5645_M_BST2_R_SM_R_SFT: u32 = 5;
pub const RT5645_M_BST3_R_SM_R: u32 = 0x1 << 4;
pub const RT5645_M_BST3_R_SM_R_SFT: u32 = 4;
pub const RT5645_M_IN_R_SM_R: u32 = 0x1 << 3;
pub const RT5645_M_IN_R_SM_R_SFT: u32 = 3;
pub const RT5645_M_DAC_R2_SM_R: u32 = 0x1 << 2;
pub const RT5645_M_DAC_R2_SM_R_SFT: u32 = 2;
pub const RT5645_M_DAC_R1_SM_R: u32 = 0x1 << 1;
pub const RT5645_M_DAC_R1_SM_R_SFT: u32 = 1;

/* SPOLMIX Control (0x48) */
pub const RT5645_M_DAC_L1_SPM_L: u32 = 0x1 << 15;
pub const RT5645_M_DAC_L1_SPM_L_SFT: u32 = 15;
pub const RT5645_M_DAC_R1_SPM_L: u32 = 0x1 << 14;
pub const RT5645_M_DAC_R1_SPM_L_SFT: u32 = 14;
pub const RT5645_M_SV_L_SPM_L: u32 = 0x1 << 13;
pub const RT5645_M_SV_L_SPM_L_SFT: u32 = 13;
pub const RT5645_M_SV_R_SPM_L: u32 = 0x1 << 12;
pub const RT5645_M_SV_R_SPM_L_SFT: u32 = 12;
pub const RT5645_M_BST3_SPM_L: u32 = 0x1 << 11;
pub const RT5645_M_BST3_SPM_L_SFT: u32 = 11;
pub const RT5645_M_DAC_R1_SPM_R: u32 = 0x1 << 2;
pub const RT5645_M_DAC_R1_SPM_R_SFT: u32 = 2;
pub const RT5645_M_BST3_SPM_R: u32 = 0x1 << 1;
pub const RT5645_M_BST3_SPM_R_SFT: u32 = 1;
pub const RT5645_M_SV_R_SPM_R: u32 = 0x1 << 0;
pub const RT5645_M_SV_R_SPM_R_SFT: u32 = 0;

/* SPOMIX Ratio Control (0x4a) */
pub const RT5645_SPK_G_CLSD_MASK: u32 = 0x7 << 0;
pub const RT5645_SPK_G_CLSD_SFT: u32 = 0;

/* Mono Output Mixer Control (0x4c) */
pub const RT5645_G_MONOMIX_MASK: u32 = 0x1 << 10;
pub const RT5645_G_MONOMIX_SFT: u32 = 10;
pub const RT5645_M_OV_L_MM: u32 = 0x1 << 9;
pub const RT5645_M_OV_L_MM_SFT: u32 = 9;
pub const RT5645_M_DAC_L2_MA: u32 = 0x1 << 8;
pub const RT5645_M_DAC_L2_MA_SFT: u32 = 8;
pub const RT5645_M_BST2_MM: u32 = 0x1 << 4;
pub const RT5645_M_BST2_MM_SFT: u32 = 4;
pub const RT5645_M_DAC_R1_MM: u32 = 0x1 << 3;
pub const RT5645_M_DAC_R1_MM_SFT: u32 = 3;
pub const RT5645_M_DAC_R2_MM: u32 = 0x1 << 2;
pub const RT5645_M_DAC_R2_MM_SFT: u32 = 2;
pub const RT5645_M_DAC_L2_MM: u32 = 0x1 << 1;
pub const RT5645_M_DAC_L2_MM_SFT: u32 = 1;
pub const RT5645_M_BST3_MM: u32 = 0x1 << 0;
pub const RT5645_M_BST3_MM_SFT: u32 = 0;

/* Output Left Mixer Control 1 (0x4d) */
pub const RT5645_G_BST3_OM_L_MASK: u32 = 0x7 << 13;
pub const RT5645_G_BST3_OM_L_SFT: u32 = 13;
pub const RT5645_G_BST2_OM_L_MASK: u32 = 0x7 << 10;
pub const RT5645_G_BST2_OM_L_SFT: u32 = 10;
pub const RT5645_G_BST1_OM_L_MASK: u32 = 0x7 << 7;
pub const RT5645_G_BST1_OM_L_SFT: u32 = 7;
pub const RT5645_G_IN_L_OM_L_MASK: u32 = 0x7 << 4;
pub const RT5645_G_IN_L_OM_L_SFT: u32 = 4;
pub const RT5645_G_RM_L_OM_L_MASK: u32 = 0x7 << 1;
pub const RT5645_G_RM_L_OM_L_SFT: u32 = 1;

/* Output Left Mixer Control 2 (0x4e) */
pub const RT5645_G_DAC_R2_OM_L_MASK: u32 = 0x7 << 13;
pub const RT5645_G_DAC_R2_OM_L_SFT: u32 = 13;
pub const RT5645_G_DAC_L2_OM_L_MASK: u32 = 0x7 << 10;
pub const RT5645_G_DAC_L2_OM_L_SFT: u32 = 10;
pub const RT5645_G_DAC_L1_OM_L_MASK: u32 = 0x7 << 7;
pub const RT5645_G_DAC_L1_OM_L_SFT: u32 = 7;

/* Output Left Mixer Control 3 (0x4f) */
pub const RT5645_M_BST3_OM_L: u32 = 0x1 << 4;
pub const RT5645_M_BST3_OM_L_SFT: u32 = 4;
pub const RT5645_M_BST1_OM_L: u32 = 0x1 << 3;
pub const RT5645_M_BST1_OM_L_SFT: u32 = 3;
pub const RT5645_M_IN_L_OM_L: u32 = 0x1 << 2;
pub const RT5645_M_IN_L_OM_L_SFT: u32 = 2;
pub const RT5645_M_DAC_L2_OM_L: u32 = 0x1 << 1;
pub const RT5645_M_DAC_L2_OM_L_SFT: u32 = 1;
pub const RT5645_M_DAC_L1_OM_L: u32 = 0x1;
pub const RT5645_M_DAC_L1_OM_L_SFT: u32 = 0;

/* Output Right Mixer Control 1 (0x50) */
pub const RT5645_G_BST4_OM_R_MASK: u32 = 0x7 << 13;
pub const RT5645_G_BST4_OM_R_SFT: u32 = 13;
pub const RT5645_G_BST2_OM_R_MASK: u32 = 0x7 << 10;
pub const RT5645_G_BST2_OM_R_SFT: u32 = 10;
pub const RT5645_G_BST1_OM_R_MASK: u32 = 0x7 << 7;
pub const RT5645_G_BST1_OM_R_SFT: u32 = 7;
pub const RT5645_G_IN_R_OM_R_MASK: u32 = 0x7 << 4;
pub const RT5645_G_IN_R_OM_R_SFT: u32 = 4;
pub const RT5645_G_RM_R_OM_R_MASK: u32 = 0x7 << 1;
pub const RT5645_G_RM_R_OM_R_SFT: u32 = 1;

/* Output Right Mixer Control 2 (0x51) */
pub const RT5645_G_DAC_L2_OM_R_MASK: u32 = 0x7 << 13;
pub const RT5645_G_DAC_L2_OM_R_SFT: u32 = 13;
pub const RT5645_G_DAC_R2_OM_R_MASK: u32 = 0x7 << 10;
pub const RT5645_G_DAC_R2_OM_R_SFT: u32 = 10;
pub const RT5645_G_DAC_R1_OM_R_MASK: u32 = 0x7 << 7;
pub const RT5645_G_DAC_R1_OM_R_SFT: u32 = 7;

/* Output Right Mixer Control 3 (0x52) */
pub const RT5645_M_BST3_OM_R: u32 = 0x1 << 4;
pub const RT5645_M_BST3_OM_R_SFT: u32 = 4;
pub const RT5645_M_BST2_OM_R: u32 = 0x1 << 3;
pub const RT5645_M_BST2_OM_R_SFT: u32 = 3;
pub const RT5645_M_IN_R_OM_R: u32 = 0x1 << 2;
pub const RT5645_M_IN_R_OM_R_SFT: u32 = 2;
pub const RT5645_M_DAC_R2_OM_R: u32 = 0x1 << 1;
pub const RT5645_M_DAC_R2_OM_R_SFT: u32 = 1;
pub const RT5645_M_DAC_R1_OM_R: u32 = 0x1;
pub const RT5645_M_DAC_R1_OM_R_SFT: u32 = 0;

/* LOUT Mixer Control (0x53) */
pub const RT5645_M_DAC_L1_LM: u32 = 0x1 << 15;
pub const RT5645_M_DAC_L1_LM_SFT: u32 = 15;
pub const RT5645_M_DAC_R1_LM: u32 = 0x1 << 14;
pub const RT5645_M_DAC_R1_LM_SFT: u32 = 14;
pub const RT5645_M_OV_L_LM: u32 = 0x1 << 13;
pub const RT5645_M_OV_L_LM_SFT: u32 = 13;
pub const RT5645_M_OV_R_LM: u32 = 0x1 << 12;
pub const RT5645_M_OV_R_LM_SFT: u32 = 12;
pub const RT5645_G_LOUTMIX_MASK: u32 = 0x1 << 11;
pub const RT5645_G_LOUTMIX_SFT: u32 = 11;

/* Power Management for Digital 1 (0x61) */
pub const RT5645_PWR_I2S1: u32 = 0x1 << 15;
pub const RT5645_PWR_I2S1_BIT: u32 = 15;
pub const RT5645_PWR_I2S2: u32 = 0x1 << 14;
pub const RT5645_PWR_I2S2_BIT: u32 = 14;
pub const RT5645_PWR_I2S3: u32 = 0x1 << 13;
pub const RT5645_PWR_I2S3_BIT: u32 = 13;
pub const RT5645_PWR_DAC_L1: u32 = 0x1 << 12;
pub const RT5645_PWR_DAC_L1_BIT: u32 = 12;
pub const RT5645_PWR_DAC_R1: u32 = 0x1 << 11;
pub const RT5645_PWR_DAC_R1_BIT: u32 = 11;
pub const RT5645_PWR_CLS_D_R: u32 = 0x1 << 9;
pub const RT5645_PWR_CLS_D_R_BIT: u32 = 9;
pub const RT5645_PWR_CLS_D_L: u32 = 0x1 << 8;
pub const RT5645_PWR_CLS_D_L_BIT: u32 = 8;
pub const RT5645_PWR_DAC_L2: u32 = 0x1 << 7;
pub const RT5645_PWR_DAC_L2_BIT: u32 = 7;
pub const RT5645_PWR_DAC_R2: u32 = 0x1 << 6;
pub const RT5645_PWR_DAC_R2_BIT: u32 = 6;
pub const RT5645_PWR_ADC_L: u32 = 0x1 << 2;
pub const RT5645_PWR_ADC_L_BIT: u32 = 2;
pub const RT5645_PWR_ADC_R: u32 = 0x1 << 1;
pub const RT5645_PWR_ADC_R_BIT: u32 = 1;
pub const RT5645_PWR_CLS_D: u32 = 0x1;
pub const RT5645_PWR_CLS_D_BIT: u32 = 0;

/* Power Management for Digital 2 (0x62) */
pub const RT5645_PWR_ADC_S1F: u32 = 0x1 << 15;
pub const RT5645_PWR_ADC_S1F_BIT: u32 = 15;
pub const RT5645_PWR_ADC_MF_L: u32 = 0x1 << 14;
pub const RT5645_PWR_ADC_MF_L_BIT: u32 = 14;
pub const RT5645_PWR_ADC_MF_R: u32 = 0x1 << 13;
pub const RT5645_PWR_ADC_MF_R_BIT: u32 = 13;
pub const RT5645_PWR_I2S_DSP: u32 = 0x1 << 12;
pub const RT5645_PWR_I2S_DSP_BIT: u32 = 12;
pub const RT5645_PWR_DAC_S1F: u32 = 0x1 << 11;
pub const RT5645_PWR_DAC_S1F_BIT: u32 = 11;
pub const RT5645_PWR_DAC_MF_L: u32 = 0x1 << 10;
pub const RT5645_PWR_DAC_MF_L_BIT: u32 = 10;
pub const RT5645_PWR_DAC_MF_R: u32 = 0x1 << 9;
pub const RT5645_PWR_DAC_MF_R_BIT: u32 = 9;
pub const RT5645_PWR_PDM1: u32 = 0x1 << 7;
pub const RT5645_PWR_PDM1_BIT: u32 = 7;
pub const RT5645_PWR_PDM2: u32 = 0x1 << 6;
pub const RT5645_PWR_PDM2_BIT: u32 = 6;
pub const RT5645_PWR_IPTV: u32 = 0x1 << 1;
pub const RT5645_PWR_IPTV_BIT: u32 = 1;
pub const RT5645_PWR_PAD: u32 = 0x1;
pub const RT5645_PWR_PAD_BIT: u32 = 0;

/* Power Management for Analog 1 (0x63) */
pub const RT5645_PWR_VREF1: u32 = 0x1 << 15;
pub const RT5645_PWR_VREF1_BIT: u32 = 15;
pub const RT5645_PWR_FV1: u32 = 0x1 << 14;
pub const RT5645_PWR_FV1_BIT: u32 = 14;
pub const RT5645_PWR_MB: u32 = 0x1 << 13;
pub const RT5645_PWR_MB_BIT: u32 = 13;
pub const RT5645_PWR_LM: u32 = 0x1 << 12;
pub const RT5645_PWR_LM_BIT: u32 = 12;
pub const RT5645_PWR_BG: u32 = 0x1 << 11;
pub const RT5645_PWR_BG_BIT: u32 = 11;
pub const RT5645_PWR_MA: u32 = 0x1 << 10;
pub const RT5645_PWR_MA_BIT: u32 = 10;
pub const RT5645_PWR_HP_L: u32 = 0x1 << 7;
pub const RT5645_PWR_HP_L_BIT: u32 = 7;
pub const RT5645_PWR_HP_R: u32 = 0x1 << 6;
pub const RT5645_PWR_HP_R_BIT: u32 = 6;
pub const RT5645_PWR_HA: u32 = 0x1 << 5;
pub const RT5645_PWR_HA_BIT: u32 = 5;
pub const RT5645_PWR_VREF2: u32 = 0x1 << 4;
pub const RT5645_PWR_VREF2_BIT: u32 = 4;
pub const RT5645_PWR_FV2: u32 = 0x1 << 3;
pub const RT5645_PWR_FV2_BIT: u32 = 3;
pub const RT5645_LDO_SEL_MASK: u32 = 0x3;
pub const RT5645_LDO_SEL_SFT: u32 = 0;

/* Power Management for Analog 2 (0x64) */
pub const RT5645_PWR_BST1: u32 = 0x1 << 15;
pub const RT5645_PWR_BST1_BIT: u32 = 15;
pub const RT5645_PWR_BST2: u32 = 0x1 << 14;
pub const RT5645_PWR_BST2_BIT: u32 = 14;
pub const RT5645_PWR_BST3: u32 = 0x1 << 13;
pub const RT5645_PWR_BST3_BIT: u32 = 13;
pub const RT5645_PWR_BST4: u32 = 0x1 << 12;
pub const RT5645_PWR_BST4_BIT: u32 = 12;
pub const RT5645_PWR_MB1: u32 = 0x1 << 11;
pub const RT5645_PWR_MB1_BIT: u32 = 11;
pub const RT5645_PWR_MB2: u32 = 0x1 << 10;
pub const RT5645_PWR_MB2_BIT: u32 = 10;
pub const RT5645_PWR_PLL: u32 = 0x1 << 9;
pub const RT5645_PWR_PLL_BIT: u32 = 9;
pub const RT5645_PWR_BST2_P: u32 = 0x1 << 5;
pub const RT5645_PWR_BST2_P_BIT: u32 = 5;
pub const RT5645_PWR_BST3_P: u32 = 0x1 << 4;
pub const RT5645_PWR_BST3_P_BIT: u32 = 4;
pub const RT5645_PWR_BST4_P: u32 = 0x1 << 3;
pub const RT5645_PWR_BST4_P_BIT: u32 = 3;
pub const RT5645_PWR_JD1: u32 = 0x1 << 2;
pub const RT5645_PWR_JD1_BIT: u32 = 2;
pub const RT5645_PWR_JD: u32 = 0x1 << 1;
pub const RT5645_PWR_JD_BIT: u32 = 1;

/* Power Management for Mixer (0x65) */
pub const RT5645_PWR_OM_L: u32 = 0x1 << 15;
pub const RT5645_PWR_OM_L_BIT: u32 = 15;
pub const RT5645_PWR_OM_R: u32 = 0x1 << 14;
pub const RT5645_PWR_OM_R_BIT: u32 = 14;
pub const RT5645_PWR_SM_L: u32 = 0x1 << 13;
pub const RT5645_PWR_SM_L_BIT: u32 = 13;
pub const RT5645_PWR_SM_R: u32 = 0x1 << 12;
pub const RT5645_PWR_SM_R_BIT: u32 = 12;
pub const RT5645_PWR_RM_L: u32 = 0x1 << 11;
pub const RT5645_PWR_RM_L_BIT: u32 = 11;
pub const RT5645_PWR_RM_R: u32 = 0x1 << 10;
pub const RT5645_PWR_RM_R_BIT: u32 = 10;
pub const RT5645_PWR_MM: u32 = 0x1 << 8;
pub const RT5645_PWR_MM_BIT: u32 = 8;
pub const RT5645_PWR_HM_L: u32 = 0x1 << 7;
pub const RT5645_PWR_HM_L_BIT: u32 = 7;
pub const RT5645_PWR_HM_R: u32 = 0x1 << 6;
pub const RT5645_PWR_HM_R_BIT: u32 = 6;
pub const RT5645_PWR_LDO2: u32 = 0x1 << 1;
pub const RT5645_PWR_LDO2_BIT: u32 = 1;

/* Power Management for Volume (0x66) */
pub const RT5645_PWR_SV_L: u32 = 0x1 << 15;
pub const RT5645_PWR_SV_L_BIT: u32 = 15;
pub const RT5645_PWR_SV_R: u32 = 0x1 << 14;
pub const RT5645_PWR_SV_R_BIT: u32 = 14;
pub const RT5645_PWR_HV_L: u32 = 0x1 << 11;
pub const RT5645_PWR_HV_L_BIT: u32 = 11;
pub const RT5645_PWR_HV_R: u32 = 0x1 << 10;
pub const RT5645_PWR_HV_R_BIT: u32 = 10;
pub const RT5645_PWR_IN_L: u32 = 0x1 << 9;
pub const RT5645_PWR_IN_L_BIT: u32 = 9;
pub const RT5645_PWR_IN_R: u32 = 0x1 << 8;
pub const RT5645_PWR_IN_R_BIT: u32 = 8;
pub const RT5645_PWR_MIC_DET: u32 = 0x1 << 5;
pub const RT5645_PWR_MIC_DET_BIT: u32 = 5;

/* I2S1/2 Audio Serial Data Port Control (0x70 0x71) */
pub const RT5645_I2S_MS_MASK: u32 = 0x1 << 15;
pub const RT5645_I2S_MS_SFT: u32 = 15;
pub const RT5645_I2S_MS_M: u32 = 0x0 << 15;
pub const RT5645_I2S_MS_S: u32 = 0x1 << 15;
pub const RT5645_I2S_O_CP_MASK: u32 = 0x3 << 10;
pub const RT5645_I2S_O_CP_SFT: u32 = 10;
pub const RT5645_I2S_O_CP_OFF: u32 = 0x0 << 10;
pub const RT5645_I2S_O_CP_U_LAW: u32 = 0x1 << 10;
pub const RT5645_I2S_O_CP_A_LAW: u32 = 0x2 << 10;
pub const RT5645_I2S_I_CP_MASK: u32 = 0x3 << 8;
pub const RT5645_I2S_I_CP_SFT: u32 = 8;
pub const RT5645_I2S_I_CP_OFF: u32 = 0x0 << 8;
pub const RT5645_I2S_I_CP_U_LAW: u32 = 0x1 << 8;
pub const RT5645_I2S_I_CP_A_LAW: u32 = 0x2 << 8;
pub const RT5645_I2S_BP_MASK: u32 = 0x1 << 7;
pub const RT5645_I2S_BP_SFT: u32 = 7;
pub const RT5645_I2S_BP_NOR: u32 = 0x0 << 7;
pub const RT5645_I2S_BP_INV: u32 = 0x1 << 7;
pub const RT5645_I2S_DL_MASK: u32 = 0x3 << 2;
pub const RT5645_I2S_DL_SFT: u32 = 2;
pub const RT5645_I2S_DL_16: u32 = 0x0 << 2;
pub const RT5645_I2S_DL_20: u32 = 0x1 << 2;
pub const RT5645_I2S_DL_24: u32 = 0x2 << 2;
pub const RT5645_I2S_DL_8: u32 = 0x3 << 2;
pub const RT5645_I2S_DF_MASK: u32 = 0x3;
pub const RT5645_I2S_DF_SFT: u32 = 0;
pub const RT5645_I2S_DF_I2S: u32 = 0x0;
pub const RT5645_I2S_DF_LEFT: u32 = 0x1;
pub const RT5645_I2S_DF_PCM_A: u32 = 0x2;
pub const RT5645_I2S_DF_PCM_B: u32 = 0x3;

/* I2S2 Audio Serial Data Port Control (0x71) */
pub const RT5645_I2S2_SDI_MASK: u32 = 0x1 << 6;
pub const RT5645_I2S2_SDI_SFT: u32 = 6;
pub const RT5645_I2S2_SDI_I2S1: u32 = 0x0 << 6;
pub const RT5645_I2S2_SDI_I2S2: u32 = 0x1 << 6;

/* ADC/DAC Clock Control 1 (0x73) */
pub const RT5645_I2S_PD1_MASK: u32 = 0x7 << 12;
pub const RT5645_I2S_PD1_SFT: u32 = 12;
pub const RT5645_I2S_PD1_1: u32 = 0x0 << 12;
pub const RT5645_I2S_PD1_2: u32 = 0x1 << 12;
pub const RT5645_I2S_PD1_3: u32 = 0x2 << 12;
pub const RT5645_I2S_PD1_4: u32 = 0x3 << 12;
pub const RT5645_I2S_PD1_6: u32 = 0x4 << 12;
pub const RT5645_I2S_PD1_8: u32 = 0x5 << 12;
pub const RT5645_I2S_PD1_12: u32 = 0x6 << 12;
pub const RT5645_I2S_PD1_16: u32 = 0x7 << 12;
pub const RT5645_I2S_BCLK_MS2_MASK: u32 = 0x1 << 11;
pub const RT5645_I2S_BCLK_MS2_SFT: u32 = 11;
pub const RT5645_I2S_BCLK_MS2_32: u32 = 0x0 << 11;
pub const RT5645_I2S_BCLK_MS2_64: u32 = 0x1 << 11;
pub const RT5645_I2S_PD2_MASK: u32 = 0x7 << 8;
pub const RT5645_I2S_PD2_SFT: u32 = 8;
pub const RT5645_I2S_PD2_1: u32 = 0x0 << 8;
pub const RT5645_I2S_PD2_2: u32 = 0x1 << 8;
pub const RT5645_I2S_PD2_3: u32 = 0x2 << 8;
pub const RT5645_I2S_PD2_4: u32 = 0x3 << 8;
pub const RT5645_I2S_PD2_6: u32 = 0x4 << 8;
pub const RT5645_I2S_PD2_8: u32 = 0x5 << 8;
pub const RT5645_I2S_PD2_12: u32 = 0x6 << 8;
pub const RT5645_I2S_PD2_16: u32 = 0x7 << 8;
pub const RT5645_I2S_BCLK_MS3_MASK: u32 = 0x1 << 7;
pub const RT5645_I2S_BCLK_MS3_SFT: u32 = 7;
pub const RT5645_I2S_BCLK_MS3_32: u32 = 0x0 << 7;
pub const RT5645_I2S_BCLK_MS3_64: u32 = 0x1 << 7;
pub const RT5645_I2S_PD3_MASK: u32 = 0x7 << 4;
pub const RT5645_I2S_PD3_SFT: u32 = 4;
pub const RT5645_I2S_PD3_1: u32 = 0x0 << 4;
pub const RT5645_I2S_PD3_2: u32 = 0x1 << 4;
pub const RT5645_I2S_PD3_3: u32 = 0x2 << 4;
pub const RT5645_I2S_PD3_4: u32 = 0x3 << 4;
pub const RT5645_I2S_PD3_6: u32 = 0x4 << 4;
pub const RT5645_I2S_PD3_8: u32 = 0x5 << 4;
pub const RT5645_I2S_PD3_12: u32 = 0x6 << 4;
pub const RT5645_I2S_PD3_16: u32 = 0x7 << 4;
pub const RT5645_DAC_OSR_MASK: u32 = 0x3 << 2;
pub const RT5645_DAC_OSR_SFT: u32 = 2;
pub const RT5645_DAC_OSR_128: u32 = 0x0 << 2;
pub const RT5645_DAC_OSR_64: u32 = 0x1 << 2;
pub const RT5645_DAC_OSR_32: u32 = 0x2 << 2;
pub const RT5645_DAC_OSR_16: u32 = 0x3 << 2;
pub const RT5645_ADC_OSR_MASK: u32 = 0x3;
pub const RT5645_ADC_OSR_SFT: u32 = 0;
pub const RT5645_ADC_OSR_128: u32 = 0x0;
pub const RT5645_ADC_OSR_64: u32 = 0x1;
pub const RT5645_ADC_OSR_32: u32 = 0x2;
pub const RT5645_ADC_OSR_16: u32 = 0x3;

/* ADC/DAC Clock Control 2 (0x74) */
pub const RT5645_DAC_L_OSR_MASK: u32 = 0x3 << 14;
pub const RT5645_DAC_L_OSR_SFT: u32 = 14;
pub const RT5645_DAC_L_OSR_128: u32 = 0x0 << 14;
pub const RT5645_DAC_L_OSR_64: u32 = 0x1 << 14;
pub const RT5645_DAC_L_OSR_32: u32 = 0x2 << 14;
pub const RT5645_DAC_L_OSR_16: u32 = 0x3 << 14;
pub const RT5645_ADC_R_OSR_MASK: u32 = 0x3 << 12;
pub const RT5645_ADC_R_OSR_SFT: u32 = 12;
pub const RT5645_ADC_R_OSR_128: u32 = 0x0 << 12;
pub const RT5645_ADC_R_OSR_64: u32 = 0x1 << 12;
pub const RT5645_ADC_R_OSR_32: u32 = 0x2 << 12;
pub const RT5645_ADC_R_OSR_16: u32 = 0x3 << 12;
pub const RT5645_DAHPF_EN: u32 = 0x1 << 11;
pub const RT5645_DAHPF_EN_SFT: u32 = 11;
pub const RT5645_ADHPF_EN: u32 = 0x1 << 10;
pub const RT5645_ADHPF_EN_SFT: u32 = 10;

/* Digital Microphone Control (0x75) */
pub const RT5645_DMIC_1_EN_MASK: u32 = 0x1 << 15;
pub const RT5645_DMIC_1_EN_SFT: u32 = 15;
pub const RT5645_DMIC_1_DIS: u32 = 0x0 << 15;
pub const RT5645_DMIC_1_EN: u32 = 0x1 << 15;
pub const RT5645_DMIC_2_EN_MASK: u32 = 0x1 << 14;
pub const RT5645_DMIC_2_EN_SFT: u32 = 14;
pub const RT5645_DMIC_2_DIS: u32 = 0x0 << 14;
pub const RT5645_DMIC_2_EN: u32 = 0x1 << 14;
pub const RT5645_DMIC_1L_LH_MASK: u32 = 0x1 << 13;
pub const RT5645_DMIC_1L_LH_SFT: u32 = 13;
pub const RT5645_DMIC_1L_LH_FALLING: u32 = 0x0 << 13;
pub const RT5645_DMIC_1L_LH_RISING: u32 = 0x1 << 13;
pub const RT5645_DMIC_1R_LH_MASK: u32 = 0x1 << 12;
pub const RT5645_DMIC_1R_LH_SFT: u32 = 12;
pub const RT5645_DMIC_1R_LH_FALLING: u32 = 0x0 << 12;
pub const RT5645_DMIC_1R_LH_RISING: u32 = 0x1 << 12;
pub const RT5645_DMIC_2_DP_MASK: u32 = 0x3 << 10;
pub const RT5645_DMIC_2_DP_SFT: u32 = 10;
pub const RT5645_DMIC_2_DP_GPIO6: u32 = 0x0 << 10;
pub const RT5645_DMIC_2_DP_GPIO10: u32 = 0x1 << 10;
pub const RT5645_DMIC_2_DP_GPIO12: u32 = 0x2 << 10;
pub const RT5645_DMIC_2_DP_IN2P: u32 = 0x3 << 10;
pub const RT5645_DMIC_2L_LH_MASK: u32 = 0x1 << 9;
pub const RT5645_DMIC_2L_LH_SFT: u32 = 9;
pub const RT5645_DMIC_2L_LH_FALLING: u32 = 0x0 << 9;
pub const RT5645_DMIC_2L_LH_RISING: u32 = 0x1 << 9;
pub const RT5645_DMIC_2R_LH_MASK: u32 = 0x1 << 8;
pub const RT5645_DMIC_2R_LH_SFT: u32 = 8;
pub const RT5645_DMIC_2R_LH_FALLING: u32 = 0x0 << 8;
pub const RT5645_DMIC_2R_LH_RISING: u32 = 0x1 << 8;
pub const RT5645_DMIC_CLK_MASK: u32 = 0x7 << 5;
pub const RT5645_DMIC_CLK_SFT: u32 = 5;
pub const RT5645_DMIC_3_EN_MASK: u32 = 0x1 << 4;
pub const RT5645_DMIC_3_EN_SFT: u32 = 4;
pub const RT5645_DMIC_3_DIS: u32 = 0x0 << 4;
pub const RT5645_DMIC_3_EN: u32 = 0x1 << 4;
pub const RT5645_DMIC_1_DP_MASK: u32 = 0x3 << 0;
pub const RT5645_DMIC_1_DP_SFT: u32 = 0;
pub const RT5645_DMIC_1_DP_GPIO5: u32 = 0x0 << 0;
pub const RT5645_DMIC_1_DP_IN2N: u32 = 0x1 << 0;
pub const RT5645_DMIC_1_DP_GPIO11: u32 = 0x2 << 0;

/* TDM Control 1 (0x77) */
pub const RT5645_IF1_ADC_IN_MASK: u32 = 0x3 << 8;
pub const RT5645_IF1_ADC_IN_SFT: u32 = 8;

/* Global Clock Control (0x80) */
pub const RT5645_SCLK_SRC_MASK: u32 = 0x3 << 14;
pub const RT5645_SCLK_SRC_SFT: u32 = 14;
pub const RT5645_SCLK_SRC_MCLK: u32 = 0x0 << 14;
pub const RT5645_SCLK_SRC_PLL1: u32 = 0x1 << 14;
pub const RT5645_SCLK_SRC_RCCLK: u32 = 0x2 << 14;
pub const RT5645_PLL1_SRC_MASK: u32 = 0x7 << 11;
pub const RT5645_PLL1_SRC_SFT: u32 = 11;
pub const RT5645_PLL1_SRC_MCLK: u32 = 0x0 << 11;
pub const RT5645_PLL1_SRC_BCLK1: u32 = 0x1 << 11;
pub const RT5645_PLL1_SRC_BCLK2: u32 = 0x2 << 11;
pub const RT5645_PLL1_SRC_BCLK3: u32 = 0x3 << 11;
pub const RT5645_PLL1_SRC_RCCLK: u32 = 0x4 << 11;
pub const RT5645_PLL1_PD_MASK: u32 = 0x1 << 3;
pub const RT5645_PLL1_PD_SFT: u32 = 3;
pub const RT5645_PLL1_PD_1: u32 = 0x0 << 3;
pub const RT5645_PLL1_PD_2: u32 = 0x1 << 3;

pub const RT5645_PLL_INP_MAX: u32 = 40000000;
pub const RT5645_PLL_INP_MIN: u32 = 256000;
/* PLL M/N/K Code Control 1 (0x81) */
pub const RT5645_PLL_N_MAX: u32 = 0x1ff;
pub const RT5645_PLL_N_MASK: u32 = RT5645_PLL_N_MAX << 7;
pub const RT5645_PLL_N_SFT: u32 = 7;
pub const RT5645_PLL_K_MAX: u32 = 0x1f;
pub const RT5645_PLL_K_MASK: u32 = RT5645_PLL_K_MAX;
pub const RT5645_PLL_K_SFT: u32 = 0;

/* PLL M/N/K Code Control 2 (0x82) */
pub const RT5645_PLL_M_MAX: u32 = 0xf;
pub const RT5645_PLL_M_MASK: u32 = RT5645_PLL_M_MAX << 12;
pub const RT5645_PLL_M_SFT: u32 = 12;
pub const RT5645_PLL_M_BP: u32 = 0x1 << 11;
pub const RT5645_PLL_M_BP_SFT: u32 = 11;

/* ASRC Control 1 (0x83) */
pub const RT5645_STO_T_MASK: u32 = 0x1 << 15;
pub const RT5645_STO_T_SFT: u32 = 15;
pub const RT5645_STO_T_SCLK: u32 = 0x0 << 15;
pub const RT5645_STO_T_LRCK1: u32 = 0x1 << 15;
pub const RT5645_M1_T_MASK: u32 = 0x1 << 14;
pub const RT5645_M1_T_SFT: u32 = 14;
pub const RT5645_M1_T_I2S2: u32 = 0x0 << 14;
pub const RT5645_M1_T_I2S2_D3: u32 = 0x1 << 14;
pub const RT5645_I2S2_F_MASK: u32 = 0x1 << 12;
pub const RT5645_I2S2_F_SFT: u32 = 12;
pub const RT5645_I2S2_F_I2S2_D2: u32 = 0x0 << 12;
pub const RT5645_I2S2_F_I2S1_TCLK: u32 = 0x1 << 12;
pub const RT5645_DMIC_1_M_MASK: u32 = 0x1 << 9;
pub const RT5645_DMIC_1_M_SFT: u32 = 9;
pub const RT5645_DMIC_1_M_NOR: u32 = 0x0 << 9;
pub const RT5645_DMIC_1_M_ASYN: u32 = 0x1 << 9;
pub const RT5645_DMIC_2_M_MASK: u32 = 0x1 << 8;
pub const RT5645_DMIC_2_M_SFT: u32 = 8;
pub const RT5645_DMIC_2_M_NOR: u32 = 0x0 << 8;
pub const RT5645_DMIC_2_M_ASYN: u32 = 0x1 << 8;

/* ASRC clock source selection (0x84, 0x85) */
pub const RT5645_CLK_SEL_SYS: u32 = 0x0;
pub const RT5645_CLK_SEL_I2S1_ASRC: u32 = 0x1;
pub const RT5645_CLK_SEL_I2S2_ASRC: u32 = 0x2;
pub const RT5645_CLK_SEL_SYS2: u32 = 0x5;

/* ASRC Control 2 (0x84) */
pub const RT5645_DA_STO_CLK_SEL_MASK: u32 = 0xf << 12;
pub const RT5645_DA_STO_CLK_SEL_SFT: u32 = 12;
pub const RT5645_DA_MONOL_CLK_SEL_MASK: u32 = 0xf << 8;
pub const RT5645_DA_MONOL_CLK_SEL_SFT: u32 = 8;
pub const RT5645_DA_MONOR_CLK_SEL_MASK: u32 = 0xf << 4;
pub const RT5645_DA_MONOR_CLK_SEL_SFT: u32 = 4;
pub const RT5645_AD_STO1_CLK_SEL_MASK: u32 = 0xf << 0;
pub const RT5645_AD_STO1_CLK_SEL_SFT: u32 = 0;

/* ASRC Control 3 (0x85) */
pub const RT5645_AD_MONOL_CLK_SEL_MASK: u32 = 0xf << 4;
pub const RT5645_AD_MONOL_CLK_SEL_SFT: u32 = 4;
pub const RT5645_AD_MONOR_CLK_SEL_MASK: u32 = 0xf << 0;
pub const RT5645_AD_MONOR_CLK_SEL_SFT: u32 = 0;

/* ASRC Control 4 (0x89) */
pub const RT5645_I2S1_PD_MASK: u32 = 0x7 << 12;
pub const RT5645_I2S1_PD_SFT: u32 = 12;
pub const RT5645_I2S2_PD_MASK: u32 = 0x7 << 8;
pub const RT5645_I2S2_PD_SFT: u32 = 8;

/* HPOUT Over Current Detection (0x8b) */
pub const RT5645_HP_OVCD_MASK: u32 = 0x1 << 10;
pub const RT5645_HP_OVCD_SFT: u32 = 10;
pub const RT5645_HP_OVCD_DIS: u32 = 0x0 << 10;
pub const RT5645_HP_OVCD_EN: u32 = 0x1 << 10;
pub const RT5645_HP_OC_TH_MASK: u32 = 0x3 << 8;
pub const RT5645_HP_OC_TH_SFT: u32 = 8;
pub const RT5645_HP_OC_TH_90: u32 = 0x0 << 8;
pub const RT5645_HP_OC_TH_105: u32 = 0x1 << 8;
pub const RT5645_HP_OC_TH_120: u32 = 0x2 << 8;
pub const RT5645_HP_OC_TH_135: u32 = 0x3 << 8;

/* Class D Over Current Control (0x8c) */
pub const RT5645_CLSD_OC_MASK: u32 = 0x1 << 9;
pub const RT5645_CLSD_OC_SFT: u32 = 9;
pub const RT5645_CLSD_OC_PU: u32 = 0x0 << 9;
pub const RT5645_CLSD_OC_PD: u32 = 0x1 << 9;
pub const RT5645_AUTO_PD_MASK: u32 = 0x1 << 8;
pub const RT5645_AUTO_PD_SFT: u32 = 8;
pub const RT5645_AUTO_PD_DIS: u32 = 0x0 << 8;
pub const RT5645_AUTO_PD_EN: u32 = 0x1 << 8;
pub const RT5645_CLSD_OC_TH_MASK: u32 = 0x3f;
pub const RT5645_CLSD_OC_TH_SFT: u32 = 0;

/* Class D Output Control (0x8d) */
pub const RT5645_CLSD_RATIO_MASK: u32 = 0xf << 12;
pub const RT5645_CLSD_RATIO_SFT: u32 = 12;
pub const RT5645_CLSD_OM_MASK: u32 = 0x1 << 11;
pub const RT5645_CLSD_OM_SFT: u32 = 11;
pub const RT5645_CLSD_OM_MONO: u32 = 0x0 << 11;
pub const RT5645_CLSD_OM_STO: u32 = 0x1 << 11;
pub const RT5645_CLSD_SCH_MASK: u32 = 0x1 << 10;
pub const RT5645_CLSD_SCH_SFT: u32 = 10;
pub const RT5645_CLSD_SCH_L: u32 = 0x0 << 10;
pub const RT5645_CLSD_SCH_S: u32 = 0x1 << 10;

/* Depop Mode Control 1 (0x8e) */
pub const RT5645_SMT_TRIG_MASK: u32 = 0x1 << 15;
pub const RT5645_SMT_TRIG_SFT: u32 = 15;
pub const RT5645_SMT_TRIG_DIS: u32 = 0x0 << 15;
pub const RT5645_SMT_TRIG_EN: u32 = 0x1 << 15;
pub const RT5645_HP_L_SMT_MASK: u32 = 0x1 << 9;
pub const RT5645_HP_L_SMT_SFT: u32 = 9;
pub const RT5645_HP_L_SMT_DIS: u32 = 0x0 << 9;
pub const RT5645_HP_L_SMT_EN: u32 = 0x1 << 9;
pub const RT5645_HP_R_SMT_MASK: u32 = 0x1 << 8;
pub const RT5645_HP_R_SMT_SFT: u32 = 8;
pub const RT5645_HP_R_SMT_DIS: u32 = 0x0 << 8;
pub const RT5645_HP_R_SMT_EN: u32 = 0x1 << 8;
pub const RT5645_HP_CD_PD_MASK: u32 = 0x1 << 7;
pub const RT5645_HP_CD_PD_SFT: u32 = 7;
pub const RT5645_HP_CD_PD_DIS: u32 = 0x0 << 7;
pub const RT5645_HP_CD_PD_EN: u32 = 0x1 << 7;
pub const RT5645_RSTN_MASK: u32 = 0x1 << 6;
pub const RT5645_RSTN_SFT: u32 = 6;
pub const RT5645_RSTN_DIS: u32 = 0x0 << 6;
pub const RT5645_RSTN_EN: u32 = 0x1 << 6;
pub const RT5645_RSTP_MASK: u32 = 0x1 << 5;
pub const RT5645_RSTP_SFT: u32 = 5;
pub const RT5645_RSTP_DIS: u32 = 0x0 << 5;
pub const RT5645_RSTP_EN: u32 = 0x1 << 5;
pub const RT5645_HP_CO_MASK: u32 = 0x1 << 4;
pub const RT5645_HP_CO_SFT: u32 = 4;
pub const RT5645_HP_CO_DIS: u32 = 0x0 << 4;
pub const RT5645_HP_CO_EN: u32 = 0x1 << 4;
pub const RT5645_HP_CP_MASK: u32 = 0x1 << 3;
pub const RT5645_HP_CP_SFT: u32 = 3;
pub const RT5645_HP_CP_PD: u32 = 0x0 << 3;
pub const RT5645_HP_CP_PU: u32 = 0x1 << 3;
pub const RT5645_HP_SG_MASK: u32 = 0x1 << 2;
pub const RT5645_HP_SG_SFT: u32 = 2;
pub const RT5645_HP_SG_DIS: u32 = 0x0 << 2;
pub const RT5645_HP_SG_EN: u32 = 0x1 << 2;
pub const RT5645_HP_DP_MASK: u32 = 0x1 << 1;
pub const RT5645_HP_DP_SFT: u32 = 1;
pub const RT5645_HP_DP_PD: u32 = 0x0 << 1;
pub const RT5645_HP_DP_PU: u32 = 0x1 << 1;
pub const RT5645_HP_CB_MASK: u32 = 0x1;
pub const RT5645_HP_CB_SFT: u32 = 0;
pub const RT5645_HP_CB_PD: u32 = 0x0;
pub const RT5645_HP_CB_PU: u32 = 0x1;

/* Depop Mode Control 2 (0x8f) */
pub const RT5645_DEPOP_MASK: u32 = 0x1 << 13;
pub const RT5645_DEPOP_SFT: u32 = 13;
pub const RT5645_DEPOP_AUTO: u32 = 0x0 << 13;
pub const RT5645_DEPOP_MAN: u32 = 0x1 << 13;
pub const RT5645_RAMP_MASK: u32 = 0x1 << 12;
pub const RT5645_RAMP_SFT: u32 = 12;
pub const RT5645_RAMP_DIS: u32 = 0x0 << 12;
pub const RT5645_RAMP_EN: u32 = 0x1 << 12;
pub const RT5645_BPS_MASK: u32 = 0x1 << 11;
pub const RT5645_BPS_SFT: u32 = 11;
pub const RT5645_BPS_DIS: u32 = 0x0 << 11;
pub const RT5645_BPS_EN: u32 = 0x1 << 11;
pub const RT5645_FAST_UPDN_MASK: u32 = 0x1 << 10;
pub const RT5645_FAST_UPDN_SFT: u32 = 10;
pub const RT5645_FAST_UPDN_DIS: u32 = 0x0 << 10;
pub const RT5645_FAST_UPDN_EN: u32 = 0x1 << 10;
pub const RT5645_MRES_MASK: u32 = 0x3 << 8;
pub const RT5645_MRES_SFT: u32 = 8;
pub const RT5645_MRES_15MO: u32 = 0x0 << 8;
pub const RT5645_MRES_25MO: u32 = 0x1 << 8;
pub const RT5645_MRES_35MO: u32 = 0x2 << 8;
pub const RT5645_MRES_45MO: u32 = 0x3 << 8;
pub const RT5645_VLO_MASK: u32 = 0x1 << 7;
pub const RT5645_VLO_SFT: u32 = 7;
pub const RT5645_VLO_3V: u32 = 0x0 << 7;
pub const RT5645_VLO_32V: u32 = 0x1 << 7;
pub const RT5645_DIG_DP_MASK: u32 = 0x1 << 6;
pub const RT5645_DIG_DP_SFT: u32 = 6;
pub const RT5645_DIG_DP_DIS: u32 = 0x0 << 6;
pub const RT5645_DIG_DP_EN: u32 = 0x1 << 6;
pub const RT5645_DP_TH_MASK: u32 = 0x3 << 4;
pub const RT5645_DP_TH_SFT: u32 = 4;

/* Depop Mode Control 3 (0x90) */
pub const RT5645_CP_SYS_MASK: u32 = 0x7 << 12;
pub const RT5645_CP_SYS_SFT: u32 = 12;
pub const RT5645_CP_FQ1_MASK: u32 = 0x7 << 8;
pub const RT5645_CP_FQ1_SFT: u32 = 8;
pub const RT5645_CP_FQ2_MASK: u32 = 0x7 << 4;
pub const RT5645_CP_FQ2_SFT: u32 = 4;
pub const RT5645_CP_FQ3_MASK: u32 = 0x7;
pub const RT5645_CP_FQ3_SFT: u32 = 0;
pub const RT5645_CP_FQ_1_5_KHZ: u32 = 0;
pub const RT5645_CP_FQ_3_KHZ: u32 = 1;
pub const RT5645_CP_FQ_6_KHZ: u32 = 2;
pub const RT5645_CP_FQ_12_KHZ: u32 = 3;
pub const RT5645_CP_FQ_24_KHZ: u32 = 4;
pub const RT5645_CP_FQ_48_KHZ: u32 = 5;
pub const RT5645_CP_FQ_96_KHZ: u32 = 6;
pub const RT5645_CP_FQ_192_KHZ: u32 = 7;

/* PV detection and SPK gain control (0x92) */
pub const RT5645_PVDD_DET_MASK: u32 = 0x1 << 15;
pub const RT5645_PVDD_DET_SFT: u32 = 15;
pub const RT5645_PVDD_DET_DIS: u32 = 0x0 << 15;
pub const RT5645_PVDD_DET_EN: u32 = 0x1 << 15;
pub const RT5645_SPK_AG_MASK: u32 = 0x1 << 14;
pub const RT5645_SPK_AG_SFT: u32 = 14;
pub const RT5645_SPK_AG_DIS: u32 = 0x0 << 14;
pub const RT5645_SPK_AG_EN: u32 = 0x1 << 14;

/* Micbias Control (0x93) */
pub const RT5645_MIC1_BS_MASK: u32 = 0x1 << 15;
pub const RT5645_MIC1_BS_SFT: u32 = 15;
pub const RT5645_MIC1_BS_9AV: u32 = 0x0 << 15;
pub const RT5645_MIC1_BS_75AV: u32 = 0x1 << 15;
pub const RT5645_MIC2_BS_MASK: u32 = 0x1 << 14;
pub const RT5645_MIC2_BS_SFT: u32 = 14;
pub const RT5645_MIC2_BS_9AV: u32 = 0x0 << 14;
pub const RT5645_MIC2_BS_75AV: u32 = 0x1 << 14;
pub const RT5645_MIC1_CLK_MASK: u32 = 0x1 << 13;
pub const RT5645_MIC1_CLK_SFT: u32 = 13;
pub const RT5645_MIC1_CLK_DIS: u32 = 0x0 << 13;
pub const RT5645_MIC1_CLK_EN: u32 = 0x1 << 13;
pub const RT5645_MIC2_CLK_MASK: u32 = 0x1 << 12;
pub const RT5645_MIC2_CLK_SFT: u32 = 12;
pub const RT5645_MIC2_CLK_DIS: u32 = 0x0 << 12;
pub const RT5645_MIC2_CLK_EN: u32 = 0x1 << 12;
pub const RT5645_MIC1_OVCD_MASK: u32 = 0x1 << 11;
pub const RT5645_MIC1_OVCD_SFT: u32 = 11;
pub const RT5645_MIC1_OVCD_DIS: u32 = 0x0 << 11;
pub const RT5645_MIC1_OVCD_EN: u32 = 0x1 << 11;
pub const RT5645_MIC1_OVTH_MASK: u32 = 0x3 << 9;
pub const RT5645_MIC1_OVTH_SFT: u32 = 9;
pub const RT5645_MIC1_OVTH_600UA: u32 = 0x0 << 9;
pub const RT5645_MIC1_OVTH_1500UA: u32 = 0x1 << 9;
pub const RT5645_MIC1_OVTH_2000UA: u32 = 0x2 << 9;
pub const RT5645_MIC2_OVCD_MASK: u32 = 0x1 << 8;
pub const RT5645_MIC2_OVCD_SFT: u32 = 8;
pub const RT5645_MIC2_OVCD_DIS: u32 = 0x0 << 8;
pub const RT5645_MIC2_OVCD_EN: u32 = 0x1 << 8;
pub const RT5645_MIC2_OVTH_MASK: u32 = 0x3 << 6;
pub const RT5645_MIC2_OVTH_SFT: u32 = 6;
pub const RT5645_MIC2_OVTH_600UA: u32 = 0x0 << 6;
pub const RT5645_MIC2_OVTH_1500UA: u32 = 0x1 << 6;
pub const RT5645_MIC2_OVTH_2000UA: u32 = 0x2 << 6;
pub const RT5645_PWR_MB_MASK: u32 = 0x1 << 5;
pub const RT5645_PWR_MB_SFT: u32 = 5;
pub const RT5645_PWR_MB_PD: u32 = 0x0 << 5;
pub const RT5645_PWR_MB_PU: u32 = 0x1 << 5;
pub const RT5645_PWR_CLK25M_MASK: u32 = 0x1 << 4;
pub const RT5645_PWR_CLK25M_SFT: u32 = 4;
pub const RT5645_PWR_CLK25M_PD: u32 = 0x0 << 4;
pub const RT5645_PWR_CLK25M_PU: u32 = 0x1 << 4;
pub const RT5645_IRQ_CLK_MCLK: u32 = 0x0 << 3;
pub const RT5645_IRQ_CLK_INT: u32 = 0x1 << 3;
pub const RT5645_JD1_MODE_MASK: u32 = 0x3 << 0;
pub const RT5645_JD1_MODE_0: u32 = 0x0 << 0;
pub const RT5645_JD1_MODE_1: u32 = 0x1 << 0;
pub const RT5645_JD1_MODE_2: u32 = 0x2 << 0;

/* VAD Control 4 (0x9d) */
pub const RT5645_VAD_SEL_MASK: u32 = 0x3 << 8;
pub const RT5645_VAD_SEL_SFT: u32 = 8;

/* EQ Control 1 (0xb0) */
pub const RT5645_EQ_SRC_MASK: u32 = 0x1 << 15;
pub const RT5645_EQ_SRC_SFT: u32 = 15;
pub const RT5645_EQ_SRC_DAC: u32 = 0x0 << 15;
pub const RT5645_EQ_SRC_ADC: u32 = 0x1 << 15;
pub const RT5645_EQ_UPD: u32 = 0x1 << 14;
pub const RT5645_EQ_UPD_BIT: u32 = 14;
pub const RT5645_EQ_CD_MASK: u32 = 0x1 << 13;
pub const RT5645_EQ_CD_SFT: u32 = 13;
pub const RT5645_EQ_CD_DIS: u32 = 0x0 << 13;
pub const RT5645_EQ_CD_EN: u32 = 0x1 << 13;
pub const RT5645_EQ_DITH_MASK: u32 = 0x3 << 8;
pub const RT5645_EQ_DITH_SFT: u32 = 8;
pub const RT5645_EQ_DITH_NOR: u32 = 0x0 << 8;
pub const RT5645_EQ_DITH_LSB: u32 = 0x1 << 8;
pub const RT5645_EQ_DITH_LSB_1: u32 = 0x2 << 8;
pub const RT5645_EQ_DITH_LSB_2: u32 = 0x3 << 8;

/* EQ Control 2 (0xb1) */
pub const RT5645_EQ_HPF1_M_MASK: u32 = 0x1 << 8;
pub const RT5645_EQ_HPF1_M_SFT: u32 = 8;
pub const RT5645_EQ_HPF1_M_HI: u32 = 0x0 << 8;
pub const RT5645_EQ_HPF1_M_1ST: u32 = 0x1 << 8;
pub const RT5645_EQ_LPF1_M_MASK: u32 = 0x1 << 7;
pub const RT5645_EQ_LPF1_M_SFT: u32 = 7;
pub const RT5645_EQ_LPF1_M_LO: u32 = 0x0 << 7;
pub const RT5645_EQ_LPF1_M_1ST: u32 = 0x1 << 7;
pub const RT5645_EQ_HPF2_MASK: u32 = 0x1 << 6;
pub const RT5645_EQ_HPF2_SFT: u32 = 6;
pub const RT5645_EQ_HPF2_DIS: u32 = 0x0 << 6;
pub const RT5645_EQ_HPF2_EN: u32 = 0x1 << 6;
pub const RT5645_EQ_HPF1_MASK: u32 = 0x1 << 5;
pub const RT5645_EQ_HPF1_SFT: u32 = 5;
pub const RT5645_EQ_HPF1_DIS: u32 = 0x0 << 5;
pub const RT5645_EQ_HPF1_EN: u32 = 0x1 << 5;
pub const RT5645_EQ_BPF4_MASK: u32 = 0x1 << 4;
pub const RT5645_EQ_BPF4_SFT: u32 = 4;
pub const RT5645_EQ_BPF4_DIS: u32 = 0x0 << 4;
pub const RT5645_EQ_BPF4_EN: u32 = 0x1 << 4;
pub const RT5645_EQ_BPF3_MASK: u32 = 0x1 << 3;
pub const RT5645_EQ_BPF3_SFT: u32 = 3;
pub const RT5645_EQ_BPF3_DIS: u32 = 0x0 << 3;
pub const RT5645_EQ_BPF3_EN: u32 = 0x1 << 3;
pub const RT5645_EQ_BPF2_MASK: u32 = 0x1 << 2;
pub const RT5645_EQ_BPF2_SFT: u32 = 2;
pub const RT5645_EQ_BPF2_DIS: u32 = 0x0 << 2;
pub const RT5645_EQ_BPF2_EN: u32 = 0x1 << 2;
pub const RT5645_EQ_BPF1_MASK: u32 = 0x1 << 1;
pub const RT5645_EQ_BPF1_SFT: u32 = 1;
pub const RT5645_EQ_BPF1_DIS: u32 = 0x0 << 1;
pub const RT5645_EQ_BPF1_EN: u32 = 0x1 << 1;
pub const RT5645_EQ_LPF_MASK: u32 = 0x1;
pub const RT5645_EQ_LPF_SFT: u32 = 0;
pub const RT5645_EQ_LPF_DIS: u32 = 0x0;
pub const RT5645_EQ_LPF_EN: u32 = 0x1;
pub const RT5645_EQ_CTRL_MASK: u32 = 0x7f;

/* Memory Test (0xb2) */
pub const RT5645_MT_MASK: u32 = 0x1 << 15;
pub const RT5645_MT_SFT: u32 = 15;
pub const RT5645_MT_DIS: u32 = 0x0 << 15;
pub const RT5645_MT_EN: u32 = 0x1 << 15;

/* DRC/AGC Control 1 (0xb4) */
pub const RT5645_DRC_AGC_P_MASK: u32 = 0x1 << 15;
pub const RT5645_DRC_AGC_P_SFT: u32 = 15;
pub const RT5645_DRC_AGC_P_DAC: u32 = 0x0 << 15;
pub const RT5645_DRC_AGC_P_ADC: u32 = 0x1 << 15;
pub const RT5645_DRC_AGC_MASK: u32 = 0x1 << 14;
pub const RT5645_DRC_AGC_SFT: u32 = 14;
pub const RT5645_DRC_AGC_DIS: u32 = 0x0 << 14;
pub const RT5645_DRC_AGC_EN: u32 = 0x1 << 14;
pub const RT5645_DRC_AGC_UPD: u32 = 0x1 << 13;
pub const RT5645_DRC_AGC_UPD_BIT: u32 = 13;
pub const RT5645_DRC_AGC_AR_MASK: u32 = 0x1f << 8;
pub const RT5645_DRC_AGC_AR_SFT: u32 = 8;
pub const RT5645_DRC_AGC_R_MASK: u32 = 0x7 << 5;
pub const RT5645_DRC_AGC_R_SFT: u32 = 5;
pub const RT5645_DRC_AGC_R_48K: u32 = 0x1 << 5;
pub const RT5645_DRC_AGC_R_96K: u32 = 0x2 << 5;
pub const RT5645_DRC_AGC_R_192K: u32 = 0x3 << 5;
pub const RT5645_DRC_AGC_R_441K: u32 = 0x5 << 5;
pub const RT5645_DRC_AGC_R_882K: u32 = 0x6 << 5;
pub const RT5645_DRC_AGC_R_1764K: u32 = 0x7 << 5;
pub const RT5645_DRC_AGC_RC_MASK: u32 = 0x1f;
pub const RT5645_DRC_AGC_RC_SFT: u32 = 0;

/* DRC/AGC Control 2 (0xb5) */
pub const RT5645_DRC_AGC_POB_MASK: u32 = 0x3f << 8;
pub const RT5645_DRC_AGC_POB_SFT: u32 = 8;
pub const RT5645_DRC_AGC_CP_MASK: u32 = 0x1 << 7;
pub const RT5645_DRC_AGC_CP_SFT: u32 = 7;
pub const RT5645_DRC_AGC_CP_DIS: u32 = 0x0 << 7;
pub const RT5645_DRC_AGC_CP_EN: u32 = 0x1 << 7;
pub const RT5645_DRC_AGC_CPR_MASK: u32 = 0x3 << 5;
pub const RT5645_DRC_AGC_CPR_SFT: u32 = 5;
pub const RT5645_DRC_AGC_CPR_1_1: u32 = 0x0 << 5;
pub const RT5645_DRC_AGC_CPR_1_2: u32 = 0x1 << 5;
pub const RT5645_DRC_AGC_CPR_1_3: u32 = 0x2 << 5;
pub const RT5645_DRC_AGC_CPR_1_4: u32 = 0x3 << 5;
pub const RT5645_DRC_AGC_PRB_MASK: u32 = 0x1f;
pub const RT5645_DRC_AGC_PRB_SFT: u32 = 0;

/* DRC/AGC Control 3 (0xb6) */
pub const RT5645_DRC_AGC_NGB_MASK: u32 = 0xf << 12;
pub const RT5645_DRC_AGC_NGB_SFT: u32 = 12;
pub const RT5645_DRC_AGC_TAR_MASK: u32 = 0x1f << 7;
pub const RT5645_DRC_AGC_TAR_SFT: u32 = 7;
pub const RT5645_DRC_AGC_NG_MASK: u32 = 0x1 << 6;
pub const RT5645_DRC_AGC_NG_SFT: u32 = 6;
pub const RT5645_DRC_AGC_NG_DIS: u32 = 0x0 << 6;
pub const RT5645_DRC_AGC_NG_EN: u32 = 0x1 << 6;
pub const RT5645_DRC_AGC_NGH_MASK: u32 = 0x1 << 5;
pub const RT5645_DRC_AGC_NGH_SFT: u32 = 5;
pub const RT5645_DRC_AGC_NGH_DIS: u32 = 0x0 << 5;
pub const RT5645_DRC_AGC_NGH_EN: u32 = 0x1 << 5;
pub const RT5645_DRC_AGC_NGT_MASK: u32 = 0x1f;
pub const RT5645_DRC_AGC_NGT_SFT: u32 = 0;

/* ANC Control 1 (0xb8) */
pub const RT5645_ANC_M_MASK: u32 = 0x1 << 15;
pub const RT5645_ANC_M_SFT: u32 = 15;
pub const RT5645_ANC_M_NOR: u32 = 0x0 << 15;
pub const RT5645_ANC_M_REV: u32 = 0x1 << 15;
pub const RT5645_ANC_MASK: u32 = 0x1 << 14;
pub const RT5645_ANC_SFT: u32 = 14;
pub const RT5645_ANC_DIS: u32 = 0x0 << 14;
pub const RT5645_ANC_EN: u32 = 0x1 << 14;
pub const RT5645_ANC_MD_MASK: u32 = 0x3 << 12;
pub const RT5645_ANC_MD_SFT: u32 = 12;
pub const RT5645_ANC_MD_DIS: u32 = 0x0 << 12;
pub const RT5645_ANC_MD_67MS: u32 = 0x1 << 12;
pub const RT5645_ANC_MD_267MS: u32 = 0x2 << 12;
pub const RT5645_ANC_MD_1067MS: u32 = 0x3 << 12;
pub const RT5645_ANC_SN_MASK: u32 = 0x1 << 11;
pub const RT5645_ANC_SN_SFT: u32 = 11;
pub const RT5645_ANC_SN_DIS: u32 = 0x0 << 11;
pub const RT5645_ANC_SN_EN: u32 = 0x1 << 11;
pub const RT5645_ANC_CLK_MASK: u32 = 0x1 << 10;
pub const RT5645_ANC_CLK_SFT: u32 = 10;
pub const RT5645_ANC_CLK_ANC: u32 = 0x0 << 10;
pub const RT5645_ANC_CLK_REG: u32 = 0x1 << 10;
pub const RT5645_ANC_ZCD_MASK: u32 = 0x3 << 8;
pub const RT5645_ANC_ZCD_SFT: u32 = 8;
pub const RT5645_ANC_ZCD_DIS: u32 = 0x0 << 8;
pub const RT5645_ANC_ZCD_T1: u32 = 0x1 << 8;
pub const RT5645_ANC_ZCD_T2: u32 = 0x2 << 8;
pub const RT5645_ANC_ZCD_WT: u32 = 0x3 << 8;
pub const RT5645_ANC_CS_MASK: u32 = 0x1 << 7;
pub const RT5645_ANC_CS_SFT: u32 = 7;
pub const RT5645_ANC_CS_DIS: u32 = 0x0 << 7;
pub const RT5645_ANC_CS_EN: u32 = 0x1 << 7;
pub const RT5645_ANC_SW_MASK: u32 = 0x1 << 6;
pub const RT5645_ANC_SW_SFT: u32 = 6;
pub const RT5645_ANC_SW_NOR: u32 = 0x0 << 6;
pub const RT5645_ANC_SW_AUTO: u32 = 0x1 << 6;
pub const RT5645_ANC_CO_L_MASK: u32 = 0x3f;
pub const RT5645_ANC_CO_L_SFT: u32 = 0;

/* ANC Control 2 (0xb6) */
pub const RT5645_ANC_FG_R_MASK: u32 = 0xf << 12;
pub const RT5645_ANC_FG_R_SFT: u32 = 12;
pub const RT5645_ANC_FG_L_MASK: u32 = 0xf << 8;
pub const RT5645_ANC_FG_L_SFT: u32 = 8;
pub const RT5645_ANC_CG_R_MASK: u32 = 0xf << 4;
pub const RT5645_ANC_CG_R_SFT: u32 = 4;
pub const RT5645_ANC_CG_L_MASK: u32 = 0xf;
pub const RT5645_ANC_CG_L_SFT: u32 = 0;

/* ANC Control 3 (0xb6) */
pub const RT5645_ANC_CD_MASK: u32 = 0x1 << 6;
pub const RT5645_ANC_CD_SFT: u32 = 6;
pub const RT5645_ANC_CD_BOTH: u32 = 0x0 << 6;
pub const RT5645_ANC_CD_IND: u32 = 0x1 << 6;
pub const RT5645_ANC_CO_R_MASK: u32 = 0x3f;
pub const RT5645_ANC_CO_R_SFT: u32 = 0;

/* Jack Detect Control (0xbb) */
pub const RT5645_JD_MASK: u32 = 0x7 << 13;
pub const RT5645_JD_SFT: u32 = 13;
pub const RT5645_JD_DIS: u32 = 0x0 << 13;
pub const RT5645_JD_GPIO1: u32 = 0x1 << 13;
pub const RT5645_JD_JD1_IN4P: u32 = 0x2 << 13;
pub const RT5645_JD_JD2_IN4N: u32 = 0x3 << 13;
pub const RT5645_JD_GPIO2: u32 = 0x4 << 13;
pub const RT5645_JD_GPIO3: u32 = 0x5 << 13;
pub const RT5645_JD_GPIO4: u32 = 0x6 << 13;
pub const RT5645_JD_HP_MASK: u32 = 0x1 << 11;
pub const RT5645_JD_HP_SFT: u32 = 11;
pub const RT5645_JD_HP_DIS: u32 = 0x0 << 11;
pub const RT5645_JD_HP_EN: u32 = 0x1 << 11;
pub const RT5645_JD_HP_TRG_MASK: u32 = 0x1 << 10;
pub const RT5645_JD_HP_TRG_SFT: u32 = 10;
pub const RT5645_JD_HP_TRG_LO: u32 = 0x0 << 10;
pub const RT5645_JD_HP_TRG_HI: u32 = 0x1 << 10;
pub const RT5645_JD_SPL_MASK: u32 = 0x1 << 9;
pub const RT5645_JD_SPL_SFT: u32 = 9;
pub const RT5645_JD_SPL_DIS: u32 = 0x0 << 9;
pub const RT5645_JD_SPL_EN: u32 = 0x1 << 9;
pub const RT5645_JD_SPL_TRG_MASK: u32 = 0x1 << 8;
pub const RT5645_JD_SPL_TRG_SFT: u32 = 8;
pub const RT5645_JD_SPL_TRG_LO: u32 = 0x0 << 8;
pub const RT5645_JD_SPL_TRG_HI: u32 = 0x1 << 8;
pub const RT5645_JD_SPR_MASK: u32 = 0x1 << 7;
pub const RT5645_JD_SPR_SFT: u32 = 7;
pub const RT5645_JD_SPR_DIS: u32 = 0x0 << 7;
pub const RT5645_JD_SPR_EN: u32 = 0x1 << 7;
pub const RT5645_JD_SPR_TRG_MASK: u32 = 0x1 << 6;
pub const RT5645_JD_SPR_TRG_SFT: u32 = 6;
pub const RT5645_JD_SPR_TRG_LO: u32 = 0x0 << 6;
pub const RT5645_JD_SPR_TRG_HI: u32 = 0x1 << 6;
pub const RT5645_JD_MO_MASK: u32 = 0x1 << 5;
pub const RT5645_JD_MO_SFT: u32 = 5;
pub const RT5645_JD_MO_DIS: u32 = 0x0 << 5;
pub const RT5645_JD_MO_EN: u32 = 0x1 << 5;
pub const RT5645_JD_MO_TRG_MASK: u32 = 0x1 << 4;
pub const RT5645_JD_MO_TRG_SFT: u32 = 4;
pub const RT5645_JD_MO_TRG_LO: u32 = 0x0 << 4;
pub const RT5645_JD_MO_TRG_HI: u32 = 0x1 << 4;
pub const RT5645_JD_LO_MASK: u32 = 0x1 << 3;
pub const RT5645_JD_LO_SFT: u32 = 3;
pub const RT5645_JD_LO_DIS: u32 = 0x0 << 3;
pub const RT5645_JD_LO_EN: u32 = 0x1 << 3;
pub const RT5645_JD_LO_TRG_MASK: u32 = 0x1 << 2;
pub const RT5645_JD_LO_TRG_SFT: u32 = 2;
pub const RT5645_JD_LO_TRG_LO: u32 = 0x0 << 2;
pub const RT5645_JD_LO_TRG_HI: u32 = 0x1 << 2;
pub const RT5645_JD1_IN4P_MASK: u32 = 0x1 << 1;
pub const RT5645_JD1_IN4P_SFT: u32 = 1;
pub const RT5645_JD1_IN4P_DIS: u32 = 0x0 << 1;
pub const RT5645_JD1_IN4P_EN: u32 = 0x1 << 1;
pub const RT5645_JD2_IN4N_MASK: u32 = 0x1;
pub const RT5645_JD2_IN4N_SFT: u32 = 0;
pub const RT5645_JD2_IN4N_DIS: u32 = 0x0;
pub const RT5645_JD2_IN4N_EN: u32 = 0x1;

/* Jack detect for ANC (0xbc) */
pub const RT5645_ANC_DET_MASK: u32 = 0x3 << 4;
pub const RT5645_ANC_DET_SFT: u32 = 4;
pub const RT5645_ANC_DET_DIS: u32 = 0x0 << 4;
pub const RT5645_ANC_DET_MB1: u32 = 0x1 << 4;
pub const RT5645_ANC_DET_MB2: u32 = 0x2 << 4;
pub const RT5645_ANC_DET_JD: u32 = 0x3 << 4;
pub const RT5645_AD_TRG_MASK: u32 = 0x1 << 3;
pub const RT5645_AD_TRG_SFT: u32 = 3;
pub const RT5645_AD_TRG_LO: u32 = 0x0 << 3;
pub const RT5645_AD_TRG_HI: u32 = 0x1 << 3;
pub const RT5645_ANCM_DET_MASK: u32 = 0x3 << 4;
pub const RT5645_ANCM_DET_SFT: u32 = 4;
pub const RT5645_ANCM_DET_DIS: u32 = 0x0 << 4;
pub const RT5645_ANCM_DET_MB1: u32 = 0x1 << 4;
pub const RT5645_ANCM_DET_MB2: u32 = 0x2 << 4;
pub const RT5645_ANCM_DET_JD: u32 = 0x3 << 4;
pub const RT5645_AMD_TRG_MASK: u32 = 0x1 << 3;
pub const RT5645_AMD_TRG_SFT: u32 = 3;
pub const RT5645_AMD_TRG_LO: u32 = 0x0 << 3;
pub const RT5645_AMD_TRG_HI: u32 = 0x1 << 3;

/* IRQ Control 1 (0xbd) */
pub const RT5645_IRQ_JD_MASK: u32 = 0x1 << 15;
pub const RT5645_IRQ_JD_SFT: u32 = 15;
pub const RT5645_IRQ_JD_BP: u32 = 0x0 << 15;
pub const RT5645_IRQ_JD_NOR: u32 = 0x1 << 15;
pub const RT5645_IRQ_OT_MASK: u32 = 0x1 << 14;
pub const RT5645_IRQ_OT_SFT: u32 = 14;
pub const RT5645_IRQ_OT_BP: u32 = 0x0 << 14;
pub const RT5645_IRQ_OT_NOR: u32 = 0x1 << 14;
pub const RT5645_JD_STKY_MASK: u32 = 0x1 << 13;
pub const RT5645_JD_STKY_SFT: u32 = 13;
pub const RT5645_JD_STKY_DIS: u32 = 0x0 << 13;
pub const RT5645_JD_STKY_EN: u32 = 0x1 << 13;
pub const RT5645_OT_STKY_MASK: u32 = 0x1 << 12;
pub const RT5645_OT_STKY_SFT: u32 = 12;
pub const RT5645_OT_STKY_DIS: u32 = 0x0 << 12;
pub const RT5645_OT_STKY_EN: u32 = 0x1 << 12;
pub const RT5645_JD_P_MASK: u32 = 0x1 << 11;
pub const RT5645_JD_P_SFT: u32 = 11;
pub const RT5645_JD_P_NOR: u32 = 0x0 << 11;
pub const RT5645_JD_P_INV: u32 = 0x1 << 11;
pub const RT5645_OT_P_MASK: u32 = 0x1 << 10;
pub const RT5645_OT_P_SFT: u32 = 10;
pub const RT5645_OT_P_NOR: u32 = 0x0 << 10;
pub const RT5645_OT_P_INV: u32 = 0x1 << 10;
pub const RT5645_IRQ_JD_1_1_EN: u32 = 0x1 << 9;
pub const RT5645_JD_1_1_MASK: u32 = 0x1 << 7;
pub const RT5645_JD_1_1_SFT: u32 = 7;
pub const RT5645_JD_1_1_NOR: u32 = 0x0 << 7;
pub const RT5645_JD_1_1_INV: u32 = 0x1 << 7;

/* IRQ Control 2 (0xbe) */
pub const RT5645_IRQ_MB1_OC_MASK: u32 = 0x1 << 15;
pub const RT5645_IRQ_MB1_OC_SFT: u32 = 15;
pub const RT5645_IRQ_MB1_OC_BP: u32 = 0x0 << 15;
pub const RT5645_IRQ_MB1_OC_NOR: u32 = 0x1 << 15;
pub const RT5645_IRQ_MB2_OC_MASK: u32 = 0x1 << 14;
pub const RT5645_IRQ_MB2_OC_SFT: u32 = 14;
pub const RT5645_IRQ_MB2_OC_BP: u32 = 0x0 << 14;
pub const RT5645_IRQ_MB2_OC_NOR: u32 = 0x1 << 14;
pub const RT5645_MB1_OC_STKY_MASK: u32 = 0x1 << 13;
pub const RT5645_MB1_OC_STKY_SFT: u32 = 13;
pub const RT5645_MB1_OC_STKY_DIS: u32 = 0x0 << 13;
pub const RT5645_MB1_OC_STKY_EN: u32 = 0x1 << 13;
pub const RT5645_MB2_OC_STKY_MASK: u32 = 0x1 << 12;
pub const RT5645_MB2_OC_STKY_SFT: u32 = 12;
pub const RT5645_MB2_OC_STKY_DIS: u32 = 0x0 << 12;
pub const RT5645_MB2_OC_STKY_EN: u32 = 0x1 << 12;
pub const RT5645_MB1_OC_P_MASK: u32 = 0x1 << 7;
pub const RT5645_MB1_OC_P_SFT: u32 = 7;
pub const RT5645_MB1_OC_P_NOR: u32 = 0x0 << 7;
pub const RT5645_MB1_OC_P_INV: u32 = 0x1 << 7;
pub const RT5645_MB2_OC_P_MASK: u32 = 0x1 << 6;
pub const RT5645_MB2_OC_P_SFT: u32 = 6;
pub const RT5645_MB2_OC_P_NOR: u32 = 0x0 << 6;
pub const RT5645_MB2_OC_P_INV: u32 = 0x1 << 6;
pub const RT5645_MB1_OC_CLR: u32 = 0x1 << 3;
pub const RT5645_MB1_OC_CLR_SFT: u32 = 3;
pub const RT5645_MB2_OC_CLR: u32 = 0x1 << 2;
pub const RT5645_MB2_OC_CLR_SFT: u32 = 2;

/* GPIO Control 1 (0xc0) */
pub const RT5645_GP1_PIN_MASK: u32 = 0x1 << 15;
pub const RT5645_GP1_PIN_SFT: u32 = 15;
pub const RT5645_GP1_PIN_GPIO1: u32 = 0x0 << 15;
pub const RT5645_GP1_PIN_IRQ: u32 = 0x1 << 15;
pub const RT5645_GP2_PIN_MASK: u32 = 0x1 << 14;
pub const RT5645_GP2_PIN_SFT: u32 = 14;
pub const RT5645_GP2_PIN_GPIO2: u32 = 0x0 << 14;
pub const RT5645_GP2_PIN_DMIC1_SCL: u32 = 0x1 << 14;
pub const RT5645_GP3_PIN_MASK: u32 = 0x3 << 12;
pub const RT5645_GP3_PIN_SFT: u32 = 12;
pub const RT5645_GP3_PIN_GPIO3: u32 = 0x0 << 12;
pub const RT5645_GP3_PIN_DMIC1_SDA: u32 = 0x1 << 12;
pub const RT5645_GP3_PIN_IRQ: u32 = 0x2 << 12;
pub const RT5645_GP4_PIN_MASK: u32 = 0x1 << 11;
pub const RT5645_GP4_PIN_SFT: u32 = 11;
pub const RT5645_GP4_PIN_GPIO4: u32 = 0x0 << 11;
pub const RT5645_GP4_PIN_DMIC2_SDA: u32 = 0x1 << 11;
pub const RT5645_DP_SIG_MASK: u32 = 0x1 << 10;
pub const RT5645_DP_SIG_SFT: u32 = 10;
pub const RT5645_DP_SIG_TEST: u32 = 0x0 << 10;
pub const RT5645_DP_SIG_AP: u32 = 0x1 << 10;
pub const RT5645_GPIO_M_MASK: u32 = 0x1 << 9;
pub const RT5645_GPIO_M_SFT: u32 = 9;
pub const RT5645_GPIO_M_FLT: u32 = 0x0 << 9;
pub const RT5645_GPIO_M_PH: u32 = 0x1 << 9;
pub const RT5645_I2S2_SEL: u32 = 0x1 << 8;
pub const RT5645_I2S2_SEL_SFT: u32 = 8;
pub const RT5645_GP5_PIN_MASK: u32 = 0x1 << 7;
pub const RT5645_GP5_PIN_SFT: u32 = 7;
pub const RT5645_GP5_PIN_GPIO5: u32 = 0x0 << 7;
pub const RT5645_GP5_PIN_DMIC1_SDA: u32 = 0x1 << 7;
pub const RT5645_GP6_PIN_MASK: u32 = 0x1 << 6;
pub const RT5645_GP6_PIN_SFT: u32 = 6;
pub const RT5645_GP6_PIN_GPIO6: u32 = 0x0 << 6;
pub const RT5645_GP6_PIN_DMIC2_SDA: u32 = 0x1 << 6;
pub const RT5645_I2S2_DAC_PIN_MASK: u32 = 0x1 << 4;
pub const RT5645_I2S2_DAC_PIN_SFT: u32 = 4;
pub const RT5645_I2S2_DAC_PIN_I2S: u32 = 0x0 << 4;
pub const RT5645_I2S2_DAC_PIN_GPIO: u32 = 0x1 << 4;
pub const RT5645_GP8_PIN_MASK: u32 = 0x1 << 3;
pub const RT5645_GP8_PIN_SFT: u32 = 3;
pub const RT5645_GP8_PIN_GPIO8: u32 = 0x0 << 3;
pub const RT5645_GP8_PIN_DMIC2_SDA: u32 = 0x1 << 3;
pub const RT5645_GP12_PIN_MASK: u32 = 0x1 << 2;
pub const RT5645_GP12_PIN_SFT: u32 = 2;
pub const RT5645_GP12_PIN_GPIO12: u32 = 0x0 << 2;
pub const RT5645_GP12_PIN_DMIC2_SDA: u32 = 0x1 << 2;
pub const RT5645_GP11_PIN_MASK: u32 = 0x1 << 1;
pub const RT5645_GP11_PIN_SFT: u32 = 1;
pub const RT5645_GP11_PIN_GPIO11: u32 = 0x0 << 1;
pub const RT5645_GP11_PIN_DMIC1_SDA: u32 = 0x1 << 1;
pub const RT5645_GP10_PIN_MASK: u32 = 0x1;
pub const RT5645_GP10_PIN_SFT: u32 = 0;
pub const RT5645_GP10_PIN_GPIO10: u32 = 0x0;
pub const RT5645_GP10_PIN_DMIC2_SDA: u32 = 0x1;

/* GPIO Control 3 (0xc2) */
pub const RT5645_GP4_PF_MASK: u32 = 0x1 << 11;
pub const RT5645_GP4_PF_SFT: u32 = 11;
pub const RT5645_GP4_PF_IN: u32 = 0x0 << 11;
pub const RT5645_GP4_PF_OUT: u32 = 0x1 << 11;
pub const RT5645_GP4_OUT_MASK: u32 = 0x1 << 10;
pub const RT5645_GP4_OUT_SFT: u32 = 10;
pub const RT5645_GP4_OUT_LO: u32 = 0x0 << 10;
pub const RT5645_GP4_OUT_HI: u32 = 0x1 << 10;
pub const RT5645_GP4_P_MASK: u32 = 0x1 << 9;
pub const RT5645_GP4_P_SFT: u32 = 9;
pub const RT5645_GP4_P_NOR: u32 = 0x0 << 9;
pub const RT5645_GP4_P_INV: u32 = 0x1 << 9;
pub const RT5645_GP3_PF_MASK: u32 = 0x1 << 8;
pub const RT5645_GP3_PF_SFT: u32 = 8;
pub const RT5645_GP3_PF_IN: u32 = 0x0 << 8;
pub const RT5645_GP3_PF_OUT: u32 = 0x1 << 8;
pub const RT5645_GP3_OUT_MASK: u32 = 0x1 << 7;
pub const RT5645_GP3_OUT_SFT: u32 = 7;
pub const RT5645_GP3_OUT_LO: u32 = 0x0 << 7;
pub const RT5645_GP3_OUT_HI: u32 = 0x1 << 7;
pub const RT5645_GP3_P_MASK: u32 = 0x1 << 6;
pub const RT5645_GP3_P_SFT: u32 = 6;
pub const RT5645_GP3_P_NOR: u32 = 0x0 << 6;
pub const RT5645_GP3_P_INV: u32 = 0x1 << 6;
pub const RT5645_GP2_PF_MASK: u32 = 0x1 << 5;
pub const RT5645_GP2_PF_SFT: u32 = 5;
pub const RT5645_GP2_PF_IN: u32 = 0x0 << 5;
pub const RT5645_GP2_PF_OUT: u32 = 0x1 << 5;
pub const RT5645_GP2_OUT_MASK: u32 = 0x1 << 4;
pub const RT5645_GP2_OUT_SFT: u32 = 4;
pub const RT5645_GP2_OUT_LO: u32 = 0x0 << 4;
pub const RT5645_GP2_OUT_HI: u32 = 0x1 << 4;
pub const RT5645_GP2_P_MASK: u32 = 0x1 << 3;
pub const RT5645_GP2_P_SFT: u32 = 3;
pub const RT5645_GP2_P_NOR: u32 = 0x0 << 3;
pub const RT5645_GP2_P_INV: u32 = 0x1 << 3;
pub const RT5645_GP1_PF_MASK: u32 = 0x1 << 2;
pub const RT5645_GP1_PF_SFT: u32 = 2;
pub const RT5645_GP1_PF_IN: u32 = 0x0 << 2;
pub const RT5645_GP1_PF_OUT: u32 = 0x1 << 2;
pub const RT5645_GP1_OUT_MASK: u32 = 0x1 << 1;
pub const RT5645_GP1_OUT_SFT: u32 = 1;
pub const RT5645_GP1_OUT_LO: u32 = 0x0 << 1;
pub const RT5645_GP1_OUT_HI: u32 = 0x1 << 1;
pub const RT5645_GP1_P_MASK: u32 = 0x1;
pub const RT5645_GP1_P_SFT: u32 = 0;
pub const RT5645_GP1_P_NOR: u32 = 0x0;
pub const RT5645_GP1_P_INV: u32 = 0x1;

/* Programmable Register Array Control 1 (0xc8) */
pub const RT5645_REG_SEQ_MASK: u32 = 0xf << 12;
pub const RT5645_REG_SEQ_SFT: u32 = 12;
pub const RT5645_SEQ1_ST_MASK: u32 = 0x1 << 11; /*RO*/
pub const RT5645_SEQ1_ST_SFT: u32 = 11;
pub const RT5645_SEQ1_ST_RUN: u32 = 0x0 << 11;
pub const RT5645_SEQ1_ST_FIN: u32 = 0x1 << 11;
pub const RT5645_SEQ2_ST_MASK: u32 = 0x1 << 10; /*RO*/
pub const RT5645_SEQ2_ST_SFT: u32 = 10;
pub const RT5645_SEQ2_ST_RUN: u32 = 0x0 << 10;
pub const RT5645_SEQ2_ST_FIN: u32 = 0x1 << 10;
pub const RT5645_REG_LV_MASK: u32 = 0x1 << 9;
pub const RT5645_REG_LV_SFT: u32 = 9;
pub const RT5645_REG_LV_MX: u32 = 0x0 << 9;
pub const RT5645_REG_LV_PR: u32 = 0x1 << 9;
pub const RT5645_SEQ_2_PT_MASK: u32 = 0x1 << 8;
pub const RT5645_SEQ_2_PT_BIT: u32 = 8;
pub const RT5645_REG_IDX_MASK: u32 = 0xff;
pub const RT5645_REG_IDX_SFT: u32 = 0;

/* Programmable Register Array Control 2 (0xc9) */
pub const RT5645_REG_DAT_MASK: u32 = 0xffff;
pub const RT5645_REG_DAT_SFT: u32 = 0;

/* Programmable Register Array Control 3 (0xca) */
pub const RT5645_SEQ_DLY_MASK: u32 = 0xff << 8;
pub const RT5645_SEQ_DLY_SFT: u32 = 8;
pub const RT5645_PROG_MASK: u32 = 0x1 << 7;
pub const RT5645_PROG_SFT: u32 = 7;
pub const RT5645_PROG_DIS: u32 = 0x0 << 7;
pub const RT5645_PROG_EN: u32 = 0x1 << 7;
pub const RT5645_SEQ1_PT_RUN: u32 = 0x1 << 6;
pub const RT5645_SEQ1_PT_RUN_BIT: u32 = 6;
pub const RT5645_SEQ2_PT_RUN: u32 = 0x1 << 5;
pub const RT5645_SEQ2_PT_RUN_BIT: u32 = 5;

/* Programmable Register Array Control 4 (0xcb) */
pub const RT5645_SEQ1_START_MASK: u32 = 0xf << 8;
pub const RT5645_SEQ1_START_SFT: u32 = 8;
pub const RT5645_SEQ1_END_MASK: u32 = 0xf;
pub const RT5645_SEQ1_END_SFT: u32 = 0;

/* Programmable Register Array Control 5 (0xcc) */
pub const RT5645_SEQ2_START_MASK: u32 = 0xf << 8;
pub const RT5645_SEQ2_START_SFT: u32 = 8;
pub const RT5645_SEQ2_END_MASK: u32 = 0xf;
pub const RT5645_SEQ2_END_SFT: u32 = 0;

/* Scramble Function (0xcd) */
pub const RT5645_SCB_KEY_MASK: u32 = 0xff;
pub const RT5645_SCB_KEY_SFT: u32 = 0;

/* Scramble Control (0xce) */
pub const RT5645_SCB_SWAP_MASK: u32 = 0x1 << 15;
pub const RT5645_SCB_SWAP_SFT: u32 = 15;
pub const RT5645_SCB_SWAP_DIS: u32 = 0x0 << 15;
pub const RT5645_SCB_SWAP_EN: u32 = 0x1 << 15;
pub const RT5645_SCB_MASK: u32 = 0x1 << 14;
pub const RT5645_SCB_SFT: u32 = 14;
pub const RT5645_SCB_DIS: u32 = 0x0 << 14;
pub const RT5645_SCB_EN: u32 = 0x1 << 14;

/* Baseback Control (0xcf) */
pub const RT5645_BB_MASK: u32 = 0x1 << 15;
pub const RT5645_BB_SFT: u32 = 15;
pub const RT5645_BB_DIS: u32 = 0x0 << 15;
pub const RT5645_BB_EN: u32 = 0x1 << 15;
pub const RT5645_BB_CT_MASK: u32 = 0x7 << 12;
pub const RT5645_BB_CT_SFT: u32 = 12;
pub const RT5645_BB_CT_A: u32 = 0x0 << 12;
pub const RT5645_BB_CT_B: u32 = 0x1 << 12;
pub const RT5645_BB_CT_C: u32 = 0x2 << 12;
pub const RT5645_BB_CT_D: u32 = 0x3 << 12;
pub const RT5645_M_BB_L_MASK: u32 = 0x1 << 9;
pub const RT5645_M_BB_L_SFT: u32 = 9;
pub const RT5645_M_BB_R_MASK: u32 = 0x1 << 8;
pub const RT5645_M_BB_R_SFT: u32 = 8;
pub const RT5645_M_BB_HPF_L_MASK: u32 = 0x1 << 7;
pub const RT5645_M_BB_HPF_L_SFT: u32 = 7;
pub const RT5645_M_BB_HPF_R_MASK: u32 = 0x1 << 6;
pub const RT5645_M_BB_HPF_R_SFT: u32 = 6;
pub const RT5645_G_BB_BST_MASK: u32 = 0x3f;
pub const RT5645_G_BB_BST_SFT: u32 = 0;
pub const RT5645_G_BB_BST_25DB: u32 = 0x14;

/* MP3 Plus Control 1 (0xd0) */
pub const RT5645_M_MP3_L_MASK: u32 = 0x1 << 15;
pub const RT5645_M_MP3_L_SFT: u32 = 15;
pub const RT5645_M_MP3_R_MASK: u32 = 0x1 << 14;
pub const RT5645_M_MP3_R_SFT: u32 = 14;
pub const RT5645_M_MP3_MASK: u32 = 0x1 << 13;
pub const RT5645_M_MP3_SFT: u32 = 13;
pub const RT5645_M_MP3_DIS: u32 = 0x0 << 13;
pub const RT5645_M_MP3_EN: u32 = 0x1 << 13;
pub const RT5645_EG_MP3_MASK: u32 = 0x1f << 8;
pub const RT5645_EG_MP3_SFT: u32 = 8;
pub const RT5645_MP3_HLP_MASK: u32 = 0x1 << 7;
pub const RT5645_MP3_HLP_SFT: u32 = 7;
pub const RT5645_MP3_HLP_DIS: u32 = 0x0 << 7;
pub const RT5645_MP3_HLP_EN: u32 = 0x1 << 7;
pub const RT5645_M_MP3_ORG_L_MASK: u32 = 0x1 << 6;
pub const RT5645_M_MP3_ORG_L_SFT: u32 = 6;
pub const RT5645_M_MP3_ORG_R_MASK: u32 = 0x1 << 5;
pub const RT5645_M_MP3_ORG_R_SFT: u32 = 5;

/* MP3 Plus Control 2 (0xd1) */
pub const RT5645_MP3_WT_MASK: u32 = 0x1 << 13;
pub const RT5645_MP3_WT_SFT: u32 = 13;
pub const RT5645_MP3_WT_1_4: u32 = 0x0 << 13;
pub const RT5645_MP3_WT_1_2: u32 = 0x1 << 13;
pub const RT5645_OG_MP3_MASK: u32 = 0x1f << 8;
pub const RT5645_OG_MP3_SFT: u32 = 8;
pub const RT5645_HG_MP3_MASK: u32 = 0x3f;
pub const RT5645_HG_MP3_SFT: u32 = 0;

/* 3D HP Control 1 (0xd2) */
pub const RT5645_3D_CF_MASK: u32 = 0x1 << 15;
pub const RT5645_3D_CF_SFT: u32 = 15;
pub const RT5645_3D_CF_DIS: u32 = 0x0 << 15;
pub const RT5645_3D_CF_EN: u32 = 0x1 << 15;
pub const RT5645_3D_HP_MASK: u32 = 0x1 << 14;
pub const RT5645_3D_HP_SFT: u32 = 14;
pub const RT5645_3D_HP_DIS: u32 = 0x0 << 14;
pub const RT5645_3D_HP_EN: u32 = 0x1 << 14;
pub const RT5645_3D_BT_MASK: u32 = 0x1 << 13;
pub const RT5645_3D_BT_SFT: u32 = 13;
pub const RT5645_3D_BT_DIS: u32 = 0x0 << 13;
pub const RT5645_3D_BT_EN: u32 = 0x1 << 13;
pub const RT5645_3D_1F_MIX_MASK: u32 = 0x3 << 11;
pub const RT5645_3D_1F_MIX_SFT: u32 = 11;
pub const RT5645_3D_HP_M_MASK: u32 = 0x1 << 10;
pub const RT5645_3D_HP_M_SFT: u32 = 10;
pub const RT5645_3D_HP_M_SUR: u32 = 0x0 << 10;
pub const RT5645_3D_HP_M_FRO: u32 = 0x1 << 10;
pub const RT5645_M_3D_HRTF_MASK: u32 = 0x1 << 9;
pub const RT5645_M_3D_HRTF_SFT: u32 = 9;
pub const RT5645_M_3D_D2H_MASK: u32 = 0x1 << 8;
pub const RT5645_M_3D_D2H_SFT: u32 = 8;
pub const RT5645_M_3D_D2R_MASK: u32 = 0x1 << 7;
pub const RT5645_M_3D_D2R_SFT: u32 = 7;
pub const RT5645_M_3D_REVB_MASK: u32 = 0x1 << 6;
pub const RT5645_M_3D_REVB_SFT: u32 = 6;

/* Adjustable high pass filter control 1 (0xd3) */
pub const RT5645_2ND_HPF_MASK: u32 = 0x1 << 15;
pub const RT5645_2ND_HPF_SFT: u32 = 15;
pub const RT5645_2ND_HPF_DIS: u32 = 0x0 << 15;
pub const RT5645_2ND_HPF_EN: u32 = 0x1 << 15;
pub const RT5645_HPF_CF_L_MASK: u32 = 0x7 << 12;
pub const RT5645_HPF_CF_L_SFT: u32 = 12;
pub const RT5645_1ST_HPF_MASK: u32 = 0x1 << 11;
pub const RT5645_1ST_HPF_SFT: u32 = 11;
pub const RT5645_1ST_HPF_DIS: u32 = 0x0 << 11;
pub const RT5645_1ST_HPF_EN: u32 = 0x1 << 11;
pub const RT5645_HPF_CF_R_MASK: u32 = 0x7 << 8;
pub const RT5645_HPF_CF_R_SFT: u32 = 8;
pub const RT5645_ZD_T_MASK: u32 = 0x3 << 6;
pub const RT5645_ZD_T_SFT: u32 = 6;
pub const RT5645_ZD_F_MASK: u32 = 0x3 << 4;
pub const RT5645_ZD_F_SFT: u32 = 4;
pub const RT5645_ZD_F_IM: u32 = 0x0 << 4;
pub const RT5645_ZD_F_ZC_IM: u32 = 0x1 << 4;
pub const RT5645_ZD_F_ZC_IOD: u32 = 0x2 << 4;
pub const RT5645_ZD_F_UN: u32 = 0x3 << 4;

/* HP calibration control and Amp detection (0xd6) */
pub const RT5645_SI_DAC_MASK: u32 = 0x1 << 11;
pub const RT5645_SI_DAC_SFT: u32 = 11;
pub const RT5645_SI_DAC_AUTO: u32 = 0x0 << 11;
pub const RT5645_SI_DAC_TEST: u32 = 0x1 << 11;
pub const RT5645_DC_CAL_M_MASK: u32 = 0x1 << 10;
pub const RT5645_DC_CAL_M_SFT: u32 = 10;
pub const RT5645_DC_CAL_M_CAL: u32 = 0x0 << 10;
pub const RT5645_DC_CAL_M_NOR: u32 = 0x1 << 10;
pub const RT5645_DC_CAL_MASK: u32 = 0x1 << 9;
pub const RT5645_DC_CAL_SFT: u32 = 9;
pub const RT5645_DC_CAL_DIS: u32 = 0x0 << 9;
pub const RT5645_DC_CAL_EN: u32 = 0x1 << 9;
pub const RT5645_HPD_RCV_MASK: u32 = 0x7 << 6;
pub const RT5645_HPD_RCV_SFT: u32 = 6;
pub const RT5645_HPD_PS_MASK: u32 = 0x1 << 5;
pub const RT5645_HPD_PS_SFT: u32 = 5;
pub const RT5645_HPD_PS_DIS: u32 = 0x0 << 5;
pub const RT5645_HPD_PS_EN: u32 = 0x1 << 5;
pub const RT5645_CAL_M_MASK: u32 = 0x1 << 4;
pub const RT5645_CAL_M_SFT: u32 = 4;
pub const RT5645_CAL_M_DEP: u32 = 0x0 << 4;
pub const RT5645_CAL_M_CAL: u32 = 0x1 << 4;
pub const RT5645_CAL_MASK: u32 = 0x1 << 3;
pub const RT5645_CAL_SFT: u32 = 3;
pub const RT5645_CAL_DIS: u32 = 0x0 << 3;
pub const RT5645_CAL_EN: u32 = 0x1 << 3;
pub const RT5645_CAL_TEST_MASK: u32 = 0x1 << 2;
pub const RT5645_CAL_TEST_SFT: u32 = 2;
pub const RT5645_CAL_TEST_DIS: u32 = 0x0 << 2;
pub const RT5645_CAL_TEST_EN: u32 = 0x1 << 2;
pub const RT5645_CAL_P_MASK: u32 = 0x3;
pub const RT5645_CAL_P_SFT: u32 = 0;
pub const RT5645_CAL_P_NONE: u32 = 0x0;
pub const RT5645_CAL_P_CAL: u32 = 0x1;
pub const RT5645_CAL_P_DAC_CAL: u32 = 0x2;

/* Soft volume and zero cross control 1 (0xd9) */
pub const RT5645_SV_MASK: u32 = 0x1 << 15;
pub const RT5645_SV_SFT: u32 = 15;
pub const RT5645_SV_DIS: u32 = 0x0 << 15;
pub const RT5645_SV_EN: u32 = 0x1 << 15;
pub const RT5645_SPO_SV_MASK: u32 = 0x1 << 14;
pub const RT5645_SPO_SV_SFT: u32 = 14;
pub const RT5645_SPO_SV_DIS: u32 = 0x0 << 14;
pub const RT5645_SPO_SV_EN: u32 = 0x1 << 14;
pub const RT5645_OUT_SV_MASK: u32 = 0x1 << 13;
pub const RT5645_OUT_SV_SFT: u32 = 13;
pub const RT5645_OUT_SV_DIS: u32 = 0x0 << 13;
pub const RT5645_OUT_SV_EN: u32 = 0x1 << 13;
pub const RT5645_HP_SV_MASK: u32 = 0x1 << 12;
pub const RT5645_HP_SV_SFT: u32 = 12;
pub const RT5645_HP_SV_DIS: u32 = 0x0 << 12;
pub const RT5645_HP_SV_EN: u32 = 0x1 << 12;
pub const RT5645_ZCD_DIG_MASK: u32 = 0x1 << 11;
pub const RT5645_ZCD_DIG_SFT: u32 = 11;
pub const RT5645_ZCD_DIG_DIS: u32 = 0x0 << 11;
pub const RT5645_ZCD_DIG_EN: u32 = 0x1 << 11;
pub const RT5645_ZCD_MASK: u32 = 0x1 << 10;
pub const RT5645_ZCD_SFT: u32 = 10;
pub const RT5645_ZCD_PD: u32 = 0x0 << 10;
pub const RT5645_ZCD_PU: u32 = 0x1 << 10;
pub const RT5645_M_ZCD_MASK: u32 = 0x3f << 4;
pub const RT5645_M_ZCD_SFT: u32 = 4;
pub const RT5645_M_ZCD_RM_L: u32 = 0x1 << 9;
pub const RT5645_M_ZCD_RM_R: u32 = 0x1 << 8;
pub const RT5645_M_ZCD_SM_L: u32 = 0x1 << 7;
pub const RT5645_M_ZCD_SM_R: u32 = 0x1 << 6;
pub const RT5645_M_ZCD_OM_L: u32 = 0x1 << 5;
pub const RT5645_M_ZCD_OM_R: u32 = 0x1 << 4;
pub const RT5645_SV_DLY_MASK: u32 = 0xf;
pub const RT5645_SV_DLY_SFT: u32 = 0;

/* Soft volume and zero cross control 2 (0xda) */
pub const RT5645_ZCD_HP_MASK: u32 = 0x1 << 15;
pub const RT5645_ZCD_HP_SFT: u32 = 15;
pub const RT5645_ZCD_HP_DIS: u32 = 0x0 << 15;
pub const RT5645_ZCD_HP_EN: u32 = 0x1 << 15;

/* Buttons Inline Command Function 2 (0xe0) */
pub const RT5645_EN_4BTN_IL_MASK: u32 = 0x1 << 15;
pub const RT5645_EN_4BTN_IL_EN: u32 = 0x1 << 15;
pub const RT5645_RST_4BTN_IL_MASK: u32 = 0x1 << 14;
pub const RT5645_RST_4BTN_IL_RST: u32 = 0x0 << 14;
pub const RT5645_RST_4BTN_IL_NORM: u32 = 0x1 << 14;

/* Codec Private Register definition */
/* DAC ADC Digital Volume (0x00) */
pub const RT5645_DA1_ZDET_SFT: u32 = 6;

/* 3D Speaker Control (0x63) */
pub const RT5645_3D_SPK_MASK: u32 = 0x1 << 15;
pub const RT5645_3D_SPK_SFT: u32 = 15;
pub const RT5645_3D_SPK_DIS: u32 = 0x0 << 15;
pub const RT5645_3D_SPK_EN: u32 = 0x1 << 15;
pub const RT5645_3D_SPK_M_MASK: u32 = 0x3 << 13;
pub const RT5645_3D_SPK_M_SFT: u32 = 13;
pub const RT5645_3D_SPK_CG_MASK: u32 = 0x1f << 8;
pub const RT5645_3D_SPK_CG_SFT: u32 = 8;
pub const RT5645_3D_SPK_SG_MASK: u32 = 0x1f;
pub const RT5645_3D_SPK_SG_SFT: u32 = 0;

/* Wind Noise Detection Control 1 (0x6c) */
pub const RT5645_WND_MASK: u32 = 0x1 << 15;
pub const RT5645_WND_SFT: u32 = 15;
pub const RT5645_WND_DIS: u32 = 0x0 << 15;
pub const RT5645_WND_EN: u32 = 0x1 << 15;

/* Wind Noise Detection Control 2 (0x6d) */
pub const RT5645_WND_FC_NW_MASK: u32 = 0x3f << 10;
pub const RT5645_WND_FC_NW_SFT: u32 = 10;
pub const RT5645_WND_FC_WK_MASK: u32 = 0x3f << 4;
pub const RT5645_WND_FC_WK_SFT: u32 = 4;

/* Wind Noise Detection Control 3 (0x6e) */
pub const RT5645_HPF_FC_MASK: u32 = 0x3f << 6;
pub const RT5645_HPF_FC_SFT: u32 = 6;
pub const RT5645_WND_FC_ST_MASK: u32 = 0x3f;
pub const RT5645_WND_FC_ST_SFT: u32 = 0;

/* Wind Noise Detection Control 4 (0x6f) */
pub const RT5645_WND_TH_LO_MASK: u32 = 0x3ff;
pub const RT5645_WND_TH_LO_SFT: u32 = 0;

/* Wind Noise Detection Control 5 (0x70) */
pub const RT5645_WND_TH_HI_MASK: u32 = 0x3ff;
pub const RT5645_WND_TH_HI_SFT: u32 = 0;

/* Wind Noise Detection Control 8 (0x73) */
pub const RT5645_WND_WIND_MASK: u32 = 0x1 << 13; /* Read-Only */
pub const RT5645_WND_WIND_SFT: u32 = 13;
pub const RT5645_WND_STRONG_MASK: u32 = 0x1 << 12; /* Read-Only */
pub const RT5645_WND_STRONG_SFT: u32 = 12;
pub const RT5645_NO_WIND: u32 = 0;
pub const RT5645_BREEZE: u32 = 1;
pub const RT5645_STORM: u32 = 2;


/* Dipole Speaker Interface (0x75) */
pub const RT5645_DP_ATT_MASK: u32 = 0x3 << 14;
pub const RT5645_DP_ATT_SFT: u32 = 14;
pub const RT5645_DP_SPK_MASK: u32 = 0x1 << 10;
pub const RT5645_DP_SPK_SFT: u32 = 10;
pub const RT5645_DP_SPK_DIS: u32 = 0x0 << 10;
pub const RT5645_DP_SPK_EN: u32 = 0x1 << 10;

/* EQ Pre Volume Control (0xb3) */
pub const RT5645_EQ_PRE_VOL_MASK: u32 = 0xffff;
pub const RT5645_EQ_PRE_VOL_SFT: u32 = 0;

/* EQ Post Volume Control (0xb4) */
pub const RT5645_EQ_PST_VOL_MASK: u32 = 0xffff;
pub const RT5645_EQ_PST_VOL_SFT: u32 = 0;

/* Jack Detect Control 3 (0xf8) */
pub const RT5645_CMP_MIC_IN_DET_MASK: u32 = 0x7 << 12;
pub const RT5645_JD_CBJ_EN: u32 = 0x1 << 7;
pub const RT5645_JD_CBJ_POL: u32 = 0x1 << 6;
pub const RT5645_JD_TRI_CBJ_SEL_MASK: u32 = 0x7 << 3;
pub const RT5645_JD_TRI_CBJ_SEL_SFT: u32 = 3;
pub const RT5645_JD_TRI_HPO_SEL_MASK: u32 = 0x7;
pub const RT5645_JD_TRI_HPO_SEL_SFT: u32 = 0;
pub const RT5645_JD_F_GPIO_JD1: u32 = 0x0;
pub const RT5645_JD_F_JD1_1: u32 = 0x1;
pub const RT5645_JD_F_JD1_2: u32 = 0x2;
pub const RT5645_JD_F_JD2: u32 = 0x3;
pub const RT5645_JD_F_JD3: u32 = 0x4;
pub const RT5645_JD_F_GPIO_JD2: u32 = 0x5;
pub const RT5645_JD_F_MX0B_12: u32 = 0x6;

/* Digital Misc Control (0xfa) */
pub const RT5645_RST_DSP: u32 = 0x1 << 13;
pub const RT5645_IF1_ADC1_IN1_SEL: u32 = 0x1 << 12;
pub const RT5645_IF1_ADC1_IN1_SFT: u32 = 12;
pub const RT5645_IF1_ADC1_IN2_SEL: u32 = 0x1 << 11;
pub const RT5645_IF1_ADC1_IN2_SFT: u32 = 11;
pub const RT5645_IF1_ADC2_IN1_SEL: u32 = 0x1 << 10;
pub const RT5645_IF1_ADC2_IN1_SFT: u32 = 10;
pub const RT5645_DIG_GATE_CTRL: u32 = 0x1;

/* General Control2 (0xfb) */
pub const RT5645_RXDC_SRC_MASK: u32 = 0x1 << 7;
pub const RT5645_RXDC_SRC_STO: u32 = 0x0 << 7;
pub const RT5645_RXDC_SRC_MONO: u32 = 0x1 << 7;
pub const RT5645_RXDC_SRC_SFT: u32 = 7;
pub const RT5645_MICBIAS1_POW_CTRL_SEL_MASK: u32 = 0x1 << 5;
pub const RT5645_MICBIAS1_POW_CTRL_SEL_A: u32 = 0x0 << 5;
pub const RT5645_MICBIAS1_POW_CTRL_SEL_M: u32 = 0x1 << 5;
pub const RT5645_MICBIAS2_POW_CTRL_SEL_MASK: u32 = 0x1 << 4;
pub const RT5645_MICBIAS2_POW_CTRL_SEL_A: u32 = 0x0 << 4;
pub const RT5645_MICBIAS2_POW_CTRL_SEL_M: u32 = 0x1 << 4;
pub const RT5645_RXDP2_SEL_MASK: u32 = 0x1 << 3;
pub const RT5645_RXDP2_SEL_IF2: u32 = 0x0 << 3;
pub const RT5645_RXDP2_SEL_ADC: u32 = 0x1 << 3;
pub const RT5645_RXDP2_SEL_SFT: u32 = 3;

/* General Control3 (0xfc) */
pub const RT5645_JD_PSV_MODE: u32 = 0x1 << 12;
pub const RT5645_IRQ_CLK_GATE_CTRL: u32 = 0x1 << 11;
pub const RT5645_DET_CLK_MASK: u32 = 0x3 << 9;
pub const RT5645_DET_CLK_DIS: u32 = 0x0 << 9;
pub const RT5645_DET_CLK_MODE1: u32 = 0x1 << 9;
pub const RT5645_DET_CLK_MODE2: u32 = 0x2 << 9;
pub const RT5645_MICINDET_MANU: u32 = 0x1 << 7;
pub const RT5645_RING2_SLEEVE_GND: u32 = 0x1 << 5;

/* Vendor ID (0xfd) */
pub const RT5645_VER_C: u32 = 0x2;
pub const RT5645_VER_D: u32 = 0x3;


/* Volume Rescale */
pub const RT5645_VOL_RSCL_MAX: u32 = 0x27;
pub const RT5645_VOL_RSCL_RANGE: u32 = 0x1F;
/* Debug String Length */
pub const RT5645_REG_DISP_LEN: u32 = 23;


/* System Clock Source */
pub const RT5645_SCLK_S_MCLK: u32 = 0;
pub const RT5645_SCLK_S_PLL1: u32 = 1;
pub const RT5645_SCLK_S_RCCLK: u32 = 2;


/* PLL1 Source */
pub const RT5645_PLL1_S_MCLK: u32 = 0;
pub const RT5645_PLL1_S_BCLK1: u32 = 1;
pub const RT5645_PLL1_S_BCLK2: u32 = 2;


pub const RT5645_AIF1: u32 = 0;
pub const RT5645_AIF2: u32 = 1;
pub const RT5645_AIFS: u32 = 2;


pub const RT5645_DMIC1_DISABLE: u32 = 0;
pub const RT5645_DMIC_DATA_IN2P: u32 = 1;
pub const RT5645_DMIC_DATA_GPIO6: u32 = 2;
pub const RT5645_DMIC_DATA_GPIO10: u32 = 3;
pub const RT5645_DMIC_DATA_GPIO12: u32 = 4;


pub const RT5645_DMIC2_DISABLE: u32 = 0;
pub const RT5645_DMIC_DATA_IN2N: u32 = 1;
pub const RT5645_DMIC_DATA_GPIO5: u32 = 2;
pub const RT5645_DMIC_DATA_GPIO11: u32 = 3;


pub const CODEC_TYPE_RT5645: u32 = 0;
pub const CODEC_TYPE_RT5650: u32 = 1;


/* filter mask */
pub const RT5645_DA_STEREO_FILTER: u32 = 0x1;
pub const RT5645_DA_MONO_L_FILTER: u32 = 0x1 << 1;
pub const RT5645_DA_MONO_R_FILTER: u32 = 0x1 << 2;
pub const RT5645_AD_STEREO_FILTER: u32 = 0x1 << 3;
pub const RT5645_AD_MONO_L_FILTER: u32 = 0x1 << 4;
pub const RT5645_AD_MONO_R_FILTER: u32 = 0x1 << 5;


unsafe extern "C" {
    pub fn rt5645_sel_asrc_clk_src(
        component: *mut snd_soc_component,
        filter_mask: u32,
        clk_src: u32,
    ) -> i32;


    pub fn rt5645_set_jack_detect(
        component: *mut snd_soc_component,
        hp_jack: *mut snd_soc_jack,
        mic_jack: *mut snd_soc_jack,
        btn_jack: *mut snd_soc_jack,
    ) -> i32;


    pub fn rt5645_components(codec_dev: *mut device) -> *const c_char;
}


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
