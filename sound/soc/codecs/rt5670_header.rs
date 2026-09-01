/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rt5670.h  --  RT5670 ALSA SoC audio driver
 *
 * Copyright 2014 Realtek Microelectronics
 * Author: Bard Liao <bardliao@realtek.com>
 */

// Translated from C header rt5670.h.

/* SPDX-License-Identifier: GPL-2.0-only */
/*
// Untranslated C header line:  * rt5670.h  --  RT5670 ALSA SoC audio driver
// Untranslated C header line:  *
// Untranslated C header line:  * Copyright 2014 Realtek Microelectronics
// Untranslated C header line:  * Author: Bard Liao <bardliao@realtek.com>
// Untranslated C header line:  */


/* Info */
pub const RT5670_RESET: u32 = 0x00;
pub const RT5670_VENDOR_ID: u32 = 0xfd;
pub const RT5670_VENDOR_ID1: u32 = 0xfe;
pub const RT5670_VENDOR_ID2: u32 = 0xff;
/*  I/O - Output */
pub const RT5670_HP_VOL: u32 = 0x02;
pub const RT5670_LOUT1: u32 = 0x03;
/* I/O - Input */
pub const RT5670_CJ_CTRL1: u32 = 0x0a;
pub const RT5670_CJ_CTRL2: u32 = 0x0b;
pub const RT5670_CJ_CTRL3: u32 = 0x0c;
pub const RT5670_IN2: u32 = 0x0e;
pub const RT5670_INL1_INR1_VOL: u32 = 0x0f;
/* I/O - ADC/DAC/DMIC */
pub const RT5670_DAC1_DIG_VOL: u32 = 0x19;
pub const RT5670_DAC2_DIG_VOL: u32 = 0x1a;
pub const RT5670_DAC_CTRL: u32 = 0x1b;
pub const RT5670_STO1_ADC_DIG_VOL: u32 = 0x1c;
pub const RT5670_MONO_ADC_DIG_VOL: u32 = 0x1d;
pub const RT5670_ADC_BST_VOL1: u32 = 0x1e;
pub const RT5670_STO2_ADC_DIG_VOL: u32 = 0x1f;
/* Mixer - D-D */
pub const RT5670_ADC_BST_VOL2: u32 = 0x20;
pub const RT5670_STO2_ADC_MIXER: u32 = 0x26;
pub const RT5670_STO1_ADC_MIXER: u32 = 0x27;
pub const RT5670_MONO_ADC_MIXER: u32 = 0x28;
pub const RT5670_AD_DA_MIXER: u32 = 0x29;
pub const RT5670_STO_DAC_MIXER: u32 = 0x2a;
pub const RT5670_DD_MIXER: u32 = 0x2b;
pub const RT5670_DIG_MIXER: u32 = 0x2c;
pub const RT5670_DSP_PATH1: u32 = 0x2d;
pub const RT5670_DSP_PATH2: u32 = 0x2e;
pub const RT5670_DIG_INF1_DATA: u32 = 0x2f;
pub const RT5670_DIG_INF2_DATA: u32 = 0x30;
/* Mixer - PDM */
pub const RT5670_PDM_OUT_CTRL: u32 = 0x31;
pub const RT5670_PDM_DATA_CTRL1: u32 = 0x32;
pub const RT5670_PDM1_DATA_CTRL2: u32 = 0x33;
pub const RT5670_PDM1_DATA_CTRL3: u32 = 0x34;
pub const RT5670_PDM1_DATA_CTRL4: u32 = 0x35;
pub const RT5670_PDM2_DATA_CTRL2: u32 = 0x36;
pub const RT5670_PDM2_DATA_CTRL3: u32 = 0x37;
pub const RT5670_PDM2_DATA_CTRL4: u32 = 0x38;
/* Mixer - ADC */
pub const RT5670_REC_L1_MIXER: u32 = 0x3b;
pub const RT5670_REC_L2_MIXER: u32 = 0x3c;
pub const RT5670_REC_R1_MIXER: u32 = 0x3d;
pub const RT5670_REC_R2_MIXER: u32 = 0x3e;
/* Mixer - DAC */
pub const RT5670_HPO_MIXER: u32 = 0x45;
pub const RT5670_MONO_MIXER: u32 = 0x4c;
pub const RT5670_OUT_L1_MIXER: u32 = 0x4f;
pub const RT5670_OUT_R1_MIXER: u32 = 0x52;
pub const RT5670_LOUT_MIXER: u32 = 0x53;
/* Power */
pub const RT5670_PWR_DIG1: u32 = 0x61;
pub const RT5670_PWR_DIG2: u32 = 0x62;
pub const RT5670_PWR_ANLG1: u32 = 0x63;
pub const RT5670_PWR_ANLG2: u32 = 0x64;
pub const RT5670_PWR_MIXER: u32 = 0x65;
pub const RT5670_PWR_VOL: u32 = 0x66;
/* Private Register Control */
pub const RT5670_PRIV_INDEX: u32 = 0x6a;
pub const RT5670_PRIV_DATA: u32 = 0x6c;
/* Format - ADC/DAC */
pub const RT5670_I2S4_SDP: u32 = 0x6f;
pub const RT5670_I2S1_SDP: u32 = 0x70;
pub const RT5670_I2S2_SDP: u32 = 0x71;
pub const RT5670_I2S3_SDP: u32 = 0x72;
pub const RT5670_ADDA_CLK1: u32 = 0x73;
pub const RT5670_ADDA_CLK2: u32 = 0x74;
pub const RT5670_DMIC_CTRL1: u32 = 0x75;
pub const RT5670_DMIC_CTRL2: u32 = 0x76;
/* Format - TDM Control */
pub const RT5670_TDM_CTRL_1: u32 = 0x77;
pub const RT5670_TDM_CTRL_2: u32 = 0x78;
pub const RT5670_TDM_CTRL_3: u32 = 0x79;

/* Function - Analog */
pub const RT5670_DSP_CLK: u32 = 0x7f;
pub const RT5670_GLB_CLK: u32 = 0x80;
pub const RT5670_PLL_CTRL1: u32 = 0x81;
pub const RT5670_PLL_CTRL2: u32 = 0x82;
pub const RT5670_ASRC_1: u32 = 0x83;
pub const RT5670_ASRC_2: u32 = 0x84;
pub const RT5670_ASRC_3: u32 = 0x85;
pub const RT5670_ASRC_4: u32 = 0x86;
pub const RT5670_ASRC_5: u32 = 0x87;
pub const RT5670_ASRC_7: u32 = 0x89;
pub const RT5670_ASRC_8: u32 = 0x8a;
pub const RT5670_ASRC_9: u32 = 0x8b;
pub const RT5670_ASRC_10: u32 = 0x8c;
pub const RT5670_ASRC_11: u32 = 0x8d;
pub const RT5670_DEPOP_M1: u32 = 0x8e;
pub const RT5670_DEPOP_M2: u32 = 0x8f;
pub const RT5670_DEPOP_M3: u32 = 0x90;
pub const RT5670_CHARGE_PUMP: u32 = 0x91;
pub const RT5670_MICBIAS: u32 = 0x93;
pub const RT5670_A_JD_CTRL1: u32 = 0x94;
pub const RT5670_A_JD_CTRL2: u32 = 0x95;
pub const RT5670_ASRC_12: u32 = 0x97;
pub const RT5670_ASRC_13: u32 = 0x98;
pub const RT5670_ASRC_14: u32 = 0x99;
pub const RT5670_VAD_CTRL1: u32 = 0x9a;
pub const RT5670_VAD_CTRL2: u32 = 0x9b;
pub const RT5670_VAD_CTRL3: u32 = 0x9c;
pub const RT5670_VAD_CTRL4: u32 = 0x9d;
pub const RT5670_VAD_CTRL5: u32 = 0x9e;
/* Function - Digital */
pub const RT5670_ADC_EQ_CTRL1: u32 = 0xae;
pub const RT5670_ADC_EQ_CTRL2: u32 = 0xaf;
pub const RT5670_EQ_CTRL1: u32 = 0xb0;
pub const RT5670_EQ_CTRL2: u32 = 0xb1;
pub const RT5670_ALC_DRC_CTRL1: u32 = 0xb2;
pub const RT5670_ALC_DRC_CTRL2: u32 = 0xb3;
pub const RT5670_ALC_CTRL_1: u32 = 0xb4;
pub const RT5670_ALC_CTRL_2: u32 = 0xb5;
pub const RT5670_ALC_CTRL_3: u32 = 0xb6;
pub const RT5670_ALC_CTRL_4: u32 = 0xb7;
pub const RT5670_JD_CTRL: u32 = 0xbb;
pub const RT5670_IRQ_CTRL1: u32 = 0xbd;
pub const RT5670_IRQ_CTRL2: u32 = 0xbe;
pub const RT5670_INT_IRQ_ST: u32 = 0xbf;
pub const RT5670_GPIO_CTRL1: u32 = 0xc0;
pub const RT5670_GPIO_CTRL2: u32 = 0xc1;
pub const RT5670_GPIO_CTRL3: u32 = 0xc2;
pub const RT5670_SCRABBLE_FUN: u32 = 0xcd;
pub const RT5670_SCRABBLE_CTRL: u32 = 0xce;
pub const RT5670_BASE_BACK: u32 = 0xcf;
pub const RT5670_MP3_PLUS1: u32 = 0xd0;
pub const RT5670_MP3_PLUS2: u32 = 0xd1;
pub const RT5670_ADJ_HPF1: u32 = 0xd3;
pub const RT5670_ADJ_HPF2: u32 = 0xd4;
pub const RT5670_HP_CALIB_AMP_DET: u32 = 0xd6;
pub const RT5670_SV_ZCD1: u32 = 0xd9;
pub const RT5670_SV_ZCD2: u32 = 0xda;
pub const RT5670_IL_CMD: u32 = 0xdb;
pub const RT5670_IL_CMD2: u32 = 0xdc;
pub const RT5670_IL_CMD3: u32 = 0xdd;
pub const RT5670_DRC_HL_CTRL1: u32 = 0xe6;
pub const RT5670_DRC_HL_CTRL2: u32 = 0xe7;
pub const RT5670_ADC_MONO_HP_CTRL1: u32 = 0xec;
pub const RT5670_ADC_MONO_HP_CTRL2: u32 = 0xed;
pub const RT5670_ADC_STO2_HP_CTRL1: u32 = 0xee;
pub const RT5670_ADC_STO2_HP_CTRL2: u32 = 0xef;
pub const RT5670_JD_CTRL3: u32 = 0xf8;
pub const RT5670_JD_CTRL4: u32 = 0xf9;
/* General Control */
pub const RT5670_DIG_MISC: u32 = 0xfa;
pub const RT5670_GEN_CTRL2: u32 = 0xfb;
pub const RT5670_GEN_CTRL3: u32 = 0xfc;


/* Index of Codec Private Register definition */
pub const RT5670_DIG_VOL: u32 = 0x00;
pub const RT5670_PR_ALC_CTRL_1: u32 = 0x01;
pub const RT5670_PR_ALC_CTRL_2: u32 = 0x02;
pub const RT5670_PR_ALC_CTRL_3: u32 = 0x03;
pub const RT5670_PR_ALC_CTRL_4: u32 = 0x04;
pub const RT5670_PR_ALC_CTRL_5: u32 = 0x05;
pub const RT5670_PR_ALC_CTRL_6: u32 = 0x06;
pub const RT5670_BIAS_CUR1: u32 = 0x12;
pub const RT5670_BIAS_CUR3: u32 = 0x14;
pub const RT5670_CLSD_INT_REG1: u32 = 0x1c;
pub const RT5670_MAMP_INT_REG2: u32 = 0x37;
pub const RT5670_CHOP_DAC_ADC: u32 = 0x3d;
pub const RT5670_MIXER_INT_REG: u32 = 0x3f;
pub const RT5670_3D_SPK: u32 = 0x63;
pub const RT5670_WND_1: u32 = 0x6c;
pub const RT5670_WND_2: u32 = 0x6d;
pub const RT5670_WND_3: u32 = 0x6e;
pub const RT5670_WND_4: u32 = 0x6f;
pub const RT5670_WND_5: u32 = 0x70;
pub const RT5670_WND_8: u32 = 0x73;
pub const RT5670_DIP_SPK_INF: u32 = 0x75;
pub const RT5670_HP_DCC_INT1: u32 = 0x77;
pub const RT5670_EQ_BW_LOP: u32 = 0xa0;
pub const RT5670_EQ_GN_LOP: u32 = 0xa1;
pub const RT5670_EQ_FC_BP1: u32 = 0xa2;
pub const RT5670_EQ_BW_BP1: u32 = 0xa3;
pub const RT5670_EQ_GN_BP1: u32 = 0xa4;
pub const RT5670_EQ_FC_BP2: u32 = 0xa5;
pub const RT5670_EQ_BW_BP2: u32 = 0xa6;
pub const RT5670_EQ_GN_BP2: u32 = 0xa7;
pub const RT5670_EQ_FC_BP3: u32 = 0xa8;
pub const RT5670_EQ_BW_BP3: u32 = 0xa9;
pub const RT5670_EQ_GN_BP3: u32 = 0xaa;
pub const RT5670_EQ_FC_BP4: u32 = 0xab;
pub const RT5670_EQ_BW_BP4: u32 = 0xac;
pub const RT5670_EQ_GN_BP4: u32 = 0xad;
pub const RT5670_EQ_FC_HIP1: u32 = 0xae;
pub const RT5670_EQ_GN_HIP1: u32 = 0xaf;
pub const RT5670_EQ_FC_HIP2: u32 = 0xb0;
pub const RT5670_EQ_BW_HIP2: u32 = 0xb1;
pub const RT5670_EQ_GN_HIP2: u32 = 0xb2;
pub const RT5670_EQ_PRE_VOL: u32 = 0xb3;
pub const RT5670_EQ_PST_VOL: u32 = 0xb4;


/* global definition */
pub const RT5670_L_MUTE: u32 = 0x1 << 15;
pub const RT5670_L_MUTE_SFT: u32 = 15;
pub const RT5670_R_MUTE: u32 = 0x1 << 7;
pub const RT5670_R_MUTE_SFT: u32 = 7;
pub const RT5670_L_VOL_MASK: u32 = 0x3f << 8;
pub const RT5670_L_VOL_SFT: u32 = 8;
pub const RT5670_R_VOL_MASK: u32 = 0x3f;
pub const RT5670_R_VOL_SFT: u32 = 0;

/* SW Reset & Device ID (0x00) */
pub const RT5670_ID_MASK: u32 = 0x3 << 1;
pub const RT5670_ID_5670: u32 = 0x0 << 1;
pub const RT5670_ID_5672: u32 = 0x1 << 1;
pub const RT5670_ID_5671: u32 = 0x2 << 1;

/* Combo Jack Control 1 (0x0a) */
pub const RT5670_CBJ_BST1_MASK: u32 = 0xf << 12;
pub const RT5670_CBJ_BST1_SFT: u32 = 12;
pub const RT5670_CBJ_JD_HP_EN: u32 = 0x1 << 9;
pub const RT5670_CBJ_JD_MIC_EN: u32 = 0x1 << 8;
pub const RT5670_CBJ_BST1_EN: u32 = 0x1 << 2;

/* Combo Jack Control 1 (0x0b) */
pub const RT5670_CBJ_MN_JD: u32 = 0x1 << 12;
pub const RT5670_CAPLESS_EN: u32 = 0x1 << 11;
pub const RT5670_CBJ_DET_MODE: u32 = 0x1 << 7;

/* IN2 Control (0x0e) */
pub const RT5670_BST_MASK1: u32 = 0xf << 12;
pub const RT5670_BST_SFT1: u32 = 12;
pub const RT5670_BST_MASK2: u32 = 0xf << 8;
pub const RT5670_BST_SFT2: u32 = 8;
pub const RT5670_IN_DF1: u32 = 0x1 << 7;
pub const RT5670_IN_SFT1: u32 = 7;
pub const RT5670_IN_DF2: u32 = 0x1 << 6;
pub const RT5670_IN_SFT2: u32 = 6;

/* INL and INR Volume Control (0x0f) */
pub const RT5670_INL_SEL_MASK: u32 = 0x1 << 15;
pub const RT5670_INL_SEL_SFT: u32 = 15;
pub const RT5670_INL_SEL_IN4P: u32 = 0x0 << 15;
pub const RT5670_INL_SEL_MONOP: u32 = 0x1 << 15;
pub const RT5670_INL_VOL_MASK: u32 = 0x1f << 8;
pub const RT5670_INL_VOL_SFT: u32 = 8;
pub const RT5670_INR_SEL_MASK: u32 = 0x1 << 7;
pub const RT5670_INR_SEL_SFT: u32 = 7;
pub const RT5670_INR_SEL_IN4N: u32 = 0x0 << 7;
pub const RT5670_INR_SEL_MONON: u32 = 0x1 << 7;
pub const RT5670_INR_VOL_MASK: u32 = 0x1f;
pub const RT5670_INR_VOL_SFT: u32 = 0;

/* Sidetone Control (0x18) */
pub const RT5670_ST_SEL_MASK: u32 = 0x7 << 9;
pub const RT5670_ST_SEL_SFT: u32 = 9;
pub const RT5670_M_ST_DACR2: u32 = 0x1 << 8;
pub const RT5670_M_ST_DACR2_SFT: u32 = 8;
pub const RT5670_M_ST_DACL2: u32 = 0x1 << 7;
pub const RT5670_M_ST_DACL2_SFT: u32 = 7;
pub const RT5670_ST_EN: u32 = 0x1 << 6;
pub const RT5670_ST_EN_SFT: u32 = 6;

/* DAC1 Digital Volume (0x19) */
pub const RT5670_DAC_L1_VOL_MASK: u32 = 0xff << 8;
pub const RT5670_DAC_L1_VOL_SFT: u32 = 8;
pub const RT5670_DAC_R1_VOL_MASK: u32 = 0xff;
pub const RT5670_DAC_R1_VOL_SFT: u32 = 0;

/* DAC2 Digital Volume (0x1a) */
pub const RT5670_DAC_L2_VOL_MASK: u32 = 0xff << 8;
pub const RT5670_DAC_L2_VOL_SFT: u32 = 8;
pub const RT5670_DAC_R2_VOL_MASK: u32 = 0xff;
pub const RT5670_DAC_R2_VOL_SFT: u32 = 0;

/* DAC2 Control (0x1b) */
pub const RT5670_M_DAC_L2_VOL: u32 = 0x1 << 13;
pub const RT5670_M_DAC_L2_VOL_SFT: u32 = 13;
pub const RT5670_M_DAC_R2_VOL: u32 = 0x1 << 12;
pub const RT5670_M_DAC_R2_VOL_SFT: u32 = 12;
pub const RT5670_DAC2_L_SEL_MASK: u32 = 0x7 << 4;
pub const RT5670_DAC2_L_SEL_SFT: u32 = 4;
pub const RT5670_DAC2_R_SEL_MASK: u32 = 0x7 << 0;
pub const RT5670_DAC2_R_SEL_SFT: u32 = 0;

/* ADC Digital Volume Control (0x1c) */
pub const RT5670_ADC_L_VOL_MASK: u32 = 0x7f << 8;
pub const RT5670_ADC_L_VOL_SFT: u32 = 8;
pub const RT5670_ADC_R_VOL_MASK: u32 = 0x7f;
pub const RT5670_ADC_R_VOL_SFT: u32 = 0;

/* Mono ADC Digital Volume Control (0x1d) */
pub const RT5670_MONO_ADC_L_VOL_MASK: u32 = 0x7f << 8;
pub const RT5670_MONO_ADC_L_VOL_SFT: u32 = 8;
pub const RT5670_MONO_ADC_R_VOL_MASK: u32 = 0x7f;
pub const RT5670_MONO_ADC_R_VOL_SFT: u32 = 0;

/* ADC Boost Volume Control (0x1e) */
pub const RT5670_STO1_ADC_L_BST_MASK: u32 = 0x3 << 14;
pub const RT5670_STO1_ADC_L_BST_SFT: u32 = 14;
pub const RT5670_STO1_ADC_R_BST_MASK: u32 = 0x3 << 12;
pub const RT5670_STO1_ADC_R_BST_SFT: u32 = 12;
pub const RT5670_STO1_ADC_COMP_MASK: u32 = 0x3 << 10;
pub const RT5670_STO1_ADC_COMP_SFT: u32 = 10;
pub const RT5670_STO2_ADC_L_BST_MASK: u32 = 0x3 << 8;
pub const RT5670_STO2_ADC_L_BST_SFT: u32 = 8;
pub const RT5670_STO2_ADC_R_BST_MASK: u32 = 0x3 << 6;
pub const RT5670_STO2_ADC_R_BST_SFT: u32 = 6;
pub const RT5670_STO2_ADC_COMP_MASK: u32 = 0x3 << 4;
pub const RT5670_STO2_ADC_COMP_SFT: u32 = 4;

/* Stereo2 ADC Mixer Control (0x26) */
pub const RT5670_STO2_ADC_SRC_MASK: u32 = 0x1 << 15;
pub const RT5670_STO2_ADC_SRC_SFT: u32 = 15;

/* Stereo ADC Mixer Control (0x26 0x27) */
pub const RT5670_M_ADC_L1: u32 = 0x1 << 14;
pub const RT5670_M_ADC_L1_SFT: u32 = 14;
pub const RT5670_M_ADC_L2: u32 = 0x1 << 13;
pub const RT5670_M_ADC_L2_SFT: u32 = 13;
pub const RT5670_ADC_1_SRC_MASK: u32 = 0x1 << 12;
pub const RT5670_ADC_1_SRC_SFT: u32 = 12;
pub const RT5670_ADC_1_SRC_ADC: u32 = 0x1 << 12;
pub const RT5670_ADC_1_SRC_DACMIX: u32 = 0x0 << 12;
pub const RT5670_ADC_2_SRC_MASK: u32 = 0x1 << 11;
pub const RT5670_ADC_2_SRC_SFT: u32 = 11;
pub const RT5670_ADC_SRC_MASK: u32 = 0x1 << 10;
pub const RT5670_ADC_SRC_SFT: u32 = 10;
pub const RT5670_DMIC_SRC_MASK: u32 = 0x3 << 8;
pub const RT5670_DMIC_SRC_SFT: u32 = 8;
pub const RT5670_M_ADC_R1: u32 = 0x1 << 6;
pub const RT5670_M_ADC_R1_SFT: u32 = 6;
pub const RT5670_M_ADC_R2: u32 = 0x1 << 5;
pub const RT5670_M_ADC_R2_SFT: u32 = 5;
pub const RT5670_DMIC3_SRC_MASK: u32 = 0x1 << 1;
pub const RT5670_DMIC3_SRC_SFT: u32 = 0;

/* Mono ADC Mixer Control (0x28) */
pub const RT5670_M_MONO_ADC_L1: u32 = 0x1 << 14;
pub const RT5670_M_MONO_ADC_L1_SFT: u32 = 14;
pub const RT5670_M_MONO_ADC_L2: u32 = 0x1 << 13;
pub const RT5670_M_MONO_ADC_L2_SFT: u32 = 13;
pub const RT5670_MONO_ADC_L1_SRC_MASK: u32 = 0x1 << 12;
pub const RT5670_MONO_ADC_L1_SRC_SFT: u32 = 12;
pub const RT5670_MONO_ADC_L1_SRC_DACMIXL: u32 = 0x0 << 12;
pub const RT5670_MONO_ADC_L1_SRC_ADCL: u32 = 0x1 << 12;
pub const RT5670_MONO_ADC_L2_SRC_MASK: u32 = 0x1 << 11;
pub const RT5670_MONO_ADC_L2_SRC_SFT: u32 = 11;
pub const RT5670_MONO_ADC_L_SRC_MASK: u32 = 0x1 << 10;
pub const RT5670_MONO_ADC_L_SRC_SFT: u32 = 10;
pub const RT5670_MONO_DMIC_L_SRC_MASK: u32 = 0x3 << 8;
pub const RT5670_MONO_DMIC_L_SRC_SFT: u32 = 8;
pub const RT5670_M_MONO_ADC_R1: u32 = 0x1 << 6;
pub const RT5670_M_MONO_ADC_R1_SFT: u32 = 6;
pub const RT5670_M_MONO_ADC_R2: u32 = 0x1 << 5;
pub const RT5670_M_MONO_ADC_R2_SFT: u32 = 5;
pub const RT5670_MONO_ADC_R1_SRC_MASK: u32 = 0x1 << 4;
pub const RT5670_MONO_ADC_R1_SRC_SFT: u32 = 4;
pub const RT5670_MONO_ADC_R1_SRC_ADCR: u32 = 0x1 << 4;
pub const RT5670_MONO_ADC_R1_SRC_DACMIXR: u32 = 0x0 << 4;
pub const RT5670_MONO_ADC_R2_SRC_MASK: u32 = 0x1 << 3;
pub const RT5670_MONO_ADC_R2_SRC_SFT: u32 = 3;
pub const RT5670_MONO_DMIC_R_SRC_MASK: u32 = 0x3;
pub const RT5670_MONO_DMIC_R_SRC_SFT: u32 = 0;

/* ADC Mixer to DAC Mixer Control (0x29) */
pub const RT5670_M_ADCMIX_L: u32 = 0x1 << 15;
pub const RT5670_M_ADCMIX_L_SFT: u32 = 15;
pub const RT5670_M_DAC1_L: u32 = 0x1 << 14;
pub const RT5670_M_DAC1_L_SFT: u32 = 14;
pub const RT5670_DAC1_R_SEL_MASK: u32 = 0x3 << 10;
pub const RT5670_DAC1_R_SEL_SFT: u32 = 10;
pub const RT5670_DAC1_R_SEL_IF1: u32 = 0x0 << 10;
pub const RT5670_DAC1_R_SEL_IF2: u32 = 0x1 << 10;
pub const RT5670_DAC1_R_SEL_IF3: u32 = 0x2 << 10;
pub const RT5670_DAC1_R_SEL_IF4: u32 = 0x3 << 10;
pub const RT5670_DAC1_L_SEL_MASK: u32 = 0x3 << 8;
pub const RT5670_DAC1_L_SEL_SFT: u32 = 8;
pub const RT5670_DAC1_L_SEL_IF1: u32 = 0x0 << 8;
pub const RT5670_DAC1_L_SEL_IF2: u32 = 0x1 << 8;
pub const RT5670_DAC1_L_SEL_IF3: u32 = 0x2 << 8;
pub const RT5670_DAC1_L_SEL_IF4: u32 = 0x3 << 8;
pub const RT5670_M_ADCMIX_R: u32 = 0x1 << 7;
pub const RT5670_M_ADCMIX_R_SFT: u32 = 7;
pub const RT5670_M_DAC1_R: u32 = 0x1 << 6;
pub const RT5670_M_DAC1_R_SFT: u32 = 6;

/* Stereo DAC Mixer Control (0x2a) */
pub const RT5670_M_DAC_L1: u32 = 0x1 << 14;
pub const RT5670_M_DAC_L1_SFT: u32 = 14;
pub const RT5670_DAC_L1_STO_L_VOL_MASK: u32 = 0x1 << 13;
pub const RT5670_DAC_L1_STO_L_VOL_SFT: u32 = 13;
pub const RT5670_M_DAC_L2: u32 = 0x1 << 12;
pub const RT5670_M_DAC_L2_SFT: u32 = 12;
pub const RT5670_DAC_L2_STO_L_VOL_MASK: u32 = 0x1 << 11;
pub const RT5670_DAC_L2_STO_L_VOL_SFT: u32 = 11;
pub const RT5670_M_DAC_R1_STO_L: u32 = 0x1 << 9;
pub const RT5670_M_DAC_R1_STO_L_SFT: u32 = 9;
pub const RT5670_DAC_R1_STO_L_VOL_MASK: u32 = 0x1 << 8;
pub const RT5670_DAC_R1_STO_L_VOL_SFT: u32 = 8;
pub const RT5670_M_DAC_R1: u32 = 0x1 << 6;
pub const RT5670_M_DAC_R1_SFT: u32 = 6;
pub const RT5670_DAC_R1_STO_R_VOL_MASK: u32 = 0x1 << 5;
pub const RT5670_DAC_R1_STO_R_VOL_SFT: u32 = 5;
pub const RT5670_M_DAC_R2: u32 = 0x1 << 4;
pub const RT5670_M_DAC_R2_SFT: u32 = 4;
pub const RT5670_DAC_R2_STO_R_VOL_MASK: u32 = 0x1 << 3;
pub const RT5670_DAC_R2_STO_R_VOL_SFT: u32 = 3;
pub const RT5670_M_DAC_L1_STO_R: u32 = 0x1 << 1;
pub const RT5670_M_DAC_L1_STO_R_SFT: u32 = 1;
pub const RT5670_DAC_L1_STO_R_VOL_MASK: u32 = 0x1;
pub const RT5670_DAC_L1_STO_R_VOL_SFT: u32 = 0;

/* Mono DAC Mixer Control (0x2b) */
pub const RT5670_M_DAC_L1_MONO_L: u32 = 0x1 << 14;
pub const RT5670_M_DAC_L1_MONO_L_SFT: u32 = 14;
pub const RT5670_DAC_L1_MONO_L_VOL_MASK: u32 = 0x1 << 13;
pub const RT5670_DAC_L1_MONO_L_VOL_SFT: u32 = 13;
pub const RT5670_M_DAC_L2_MONO_L: u32 = 0x1 << 12;
pub const RT5670_M_DAC_L2_MONO_L_SFT: u32 = 12;
pub const RT5670_DAC_L2_MONO_L_VOL_MASK: u32 = 0x1 << 11;
pub const RT5670_DAC_L2_MONO_L_VOL_SFT: u32 = 11;
pub const RT5670_M_DAC_R2_MONO_L: u32 = 0x1 << 10;
pub const RT5670_M_DAC_R2_MONO_L_SFT: u32 = 10;
pub const RT5670_DAC_R2_MONO_L_VOL_MASK: u32 = 0x1 << 9;
pub const RT5670_DAC_R2_MONO_L_VOL_SFT: u32 = 9;
pub const RT5670_M_DAC_R1_MONO_R: u32 = 0x1 << 6;
pub const RT5670_M_DAC_R1_MONO_R_SFT: u32 = 6;
pub const RT5670_DAC_R1_MONO_R_VOL_MASK: u32 = 0x1 << 5;
pub const RT5670_DAC_R1_MONO_R_VOL_SFT: u32 = 5;
pub const RT5670_M_DAC_R2_MONO_R: u32 = 0x1 << 4;
pub const RT5670_M_DAC_R2_MONO_R_SFT: u32 = 4;
pub const RT5670_DAC_R2_MONO_R_VOL_MASK: u32 = 0x1 << 3;
pub const RT5670_DAC_R2_MONO_R_VOL_SFT: u32 = 3;
pub const RT5670_M_DAC_L2_MONO_R: u32 = 0x1 << 2;
pub const RT5670_M_DAC_L2_MONO_R_SFT: u32 = 2;
pub const RT5670_DAC_L2_MONO_R_VOL_MASK: u32 = 0x1 << 1;
pub const RT5670_DAC_L2_MONO_R_VOL_SFT: u32 = 1;

/* Digital Mixer Control (0x2c) */
pub const RT5670_M_STO_L_DAC_L: u32 = 0x1 << 15;
pub const RT5670_M_STO_L_DAC_L_SFT: u32 = 15;
pub const RT5670_STO_L_DAC_L_VOL_MASK: u32 = 0x1 << 14;
pub const RT5670_STO_L_DAC_L_VOL_SFT: u32 = 14;
pub const RT5670_M_DAC_L2_DAC_L: u32 = 0x1 << 13;
pub const RT5670_M_DAC_L2_DAC_L_SFT: u32 = 13;
pub const RT5670_DAC_L2_DAC_L_VOL_MASK: u32 = 0x1 << 12;
pub const RT5670_DAC_L2_DAC_L_VOL_SFT: u32 = 12;
pub const RT5670_M_STO_R_DAC_R: u32 = 0x1 << 11;
pub const RT5670_M_STO_R_DAC_R_SFT: u32 = 11;
pub const RT5670_STO_R_DAC_R_VOL_MASK: u32 = 0x1 << 10;
pub const RT5670_STO_R_DAC_R_VOL_SFT: u32 = 10;
pub const RT5670_M_DAC_R2_DAC_R: u32 = 0x1 << 9;
pub const RT5670_M_DAC_R2_DAC_R_SFT: u32 = 9;
pub const RT5670_DAC_R2_DAC_R_VOL_MASK: u32 = 0x1 << 8;
pub const RT5670_DAC_R2_DAC_R_VOL_SFT: u32 = 8;
pub const RT5670_M_DAC_R2_DAC_L: u32 = 0x1 << 7;
pub const RT5670_M_DAC_R2_DAC_L_SFT: u32 = 7;
pub const RT5670_DAC_R2_DAC_L_VOL_MASK: u32 = 0x1 << 6;
pub const RT5670_DAC_R2_DAC_L_VOL_SFT: u32 = 6;
pub const RT5670_M_DAC_L2_DAC_R: u32 = 0x1 << 5;
pub const RT5670_M_DAC_L2_DAC_R_SFT: u32 = 5;
pub const RT5670_DAC_L2_DAC_R_VOL_MASK: u32 = 0x1 << 4;
pub const RT5670_DAC_L2_DAC_R_VOL_SFT: u32 = 4;

/* DSP Path Control 1 (0x2d) */
pub const RT5670_RXDP_SEL_MASK: u32 = 0x7 << 13;
pub const RT5670_RXDP_SEL_SFT: u32 = 13;
pub const RT5670_RXDP_SRC_MASK: u32 = 0x3 << 11;
pub const RT5670_RXDP_SRC_SFT: u32 = 11;
pub const RT5670_RXDP_SRC_NOR: u32 = 0x0 << 11;
pub const RT5670_RXDP_SRC_DIV2: u32 = 0x1 << 11;
pub const RT5670_RXDP_SRC_DIV3: u32 = 0x2 << 11;
pub const RT5670_TXDP_SRC_MASK: u32 = 0x3 << 4;
pub const RT5670_TXDP_SRC_SFT: u32 = 4;
pub const RT5670_TXDP_SRC_NOR: u32 = 0x0 << 4;
pub const RT5670_TXDP_SRC_DIV2: u32 = 0x1 << 4;
pub const RT5670_TXDP_SRC_DIV3: u32 = 0x2 << 4;
pub const RT5670_TXDP_SLOT_SEL_MASK: u32 = 0x3 << 2;
pub const RT5670_TXDP_SLOT_SEL_SFT: u32 = 2;
pub const RT5670_DSP_UL_SEL: u32 = 0x1 << 1;
pub const RT5670_DSP_UL_SFT: u32 = 1;
pub const RT5670_DSP_DL_SEL: u32 = 0x1;
pub const RT5670_DSP_DL_SFT: u32 = 0;

/* DSP Path Control 2 (0x2e) */
pub const RT5670_TXDP_L_VOL_MASK: u32 = 0x7f << 8;
pub const RT5670_TXDP_L_VOL_SFT: u32 = 8;
pub const RT5670_TXDP_R_VOL_MASK: u32 = 0x7f;
pub const RT5670_TXDP_R_VOL_SFT: u32 = 0;

/* Digital Interface Data Control (0x2f) */
pub const RT5670_IF1_ADC2_IN_SEL: u32 = 0x1 << 15;
pub const RT5670_IF1_ADC2_IN_SFT: u32 = 15;
pub const RT5670_IF2_ADC_IN_MASK: u32 = 0x7 << 12;
pub const RT5670_IF2_ADC_IN_SFT: u32 = 12;
pub const RT5670_IF2_DAC_SEL_MASK: u32 = 0x3 << 10;
pub const RT5670_IF2_DAC_SEL_SFT: u32 = 10;
pub const RT5670_IF2_ADC_SEL_MASK: u32 = 0x3 << 8;
pub const RT5670_IF2_ADC_SEL_SFT: u32 = 8;

/* Digital Interface Data Control (0x30) */
pub const RT5670_IF4_ADC_IN_MASK: u32 = 0x3 << 4;
pub const RT5670_IF4_ADC_IN_SFT: u32 = 4;

/* PDM Output Control (0x31) */
pub const RT5670_PDM1_L_MASK: u32 = 0x1 << 15;
pub const RT5670_PDM1_L_SFT: u32 = 15;
pub const RT5670_M_PDM1_L: u32 = 0x1 << 14;
pub const RT5670_M_PDM1_L_SFT: u32 = 14;
pub const RT5670_PDM1_R_MASK: u32 = 0x1 << 13;
pub const RT5670_PDM1_R_SFT: u32 = 13;
pub const RT5670_M_PDM1_R: u32 = 0x1 << 12;
pub const RT5670_M_PDM1_R_SFT: u32 = 12;
pub const RT5670_PDM2_L_MASK: u32 = 0x1 << 11;
pub const RT5670_PDM2_L_SFT: u32 = 11;
pub const RT5670_M_PDM2_L: u32 = 0x1 << 10;
pub const RT5670_M_PDM2_L_SFT: u32 = 10;
pub const RT5670_PDM2_R_MASK: u32 = 0x1 << 9;
pub const RT5670_PDM2_R_SFT: u32 = 9;
pub const RT5670_M_PDM2_R: u32 = 0x1 << 8;
pub const RT5670_M_PDM2_R_SFT: u32 = 8;
pub const RT5670_PDM2_BUSY: u32 = 0x1 << 7;
pub const RT5670_PDM1_BUSY: u32 = 0x1 << 6;
pub const RT5670_PDM_PATTERN: u32 = 0x1 << 5;
pub const RT5670_PDM_GAIN: u32 = 0x1 << 4;
pub const RT5670_PDM_DIV_MASK: u32 = 0x3;

/* REC Left Mixer Control 1 (0x3b) */
pub const RT5670_G_HP_L_RM_L_MASK: u32 = 0x7 << 13;
pub const RT5670_G_HP_L_RM_L_SFT: u32 = 13;
pub const RT5670_G_IN_L_RM_L_MASK: u32 = 0x7 << 10;
pub const RT5670_G_IN_L_RM_L_SFT: u32 = 10;
pub const RT5670_G_BST4_RM_L_MASK: u32 = 0x7 << 7;
pub const RT5670_G_BST4_RM_L_SFT: u32 = 7;
pub const RT5670_G_BST3_RM_L_MASK: u32 = 0x7 << 4;
pub const RT5670_G_BST3_RM_L_SFT: u32 = 4;
pub const RT5670_G_BST2_RM_L_MASK: u32 = 0x7 << 1;
pub const RT5670_G_BST2_RM_L_SFT: u32 = 1;

/* REC Left Mixer Control 2 (0x3c) */
pub const RT5670_G_BST1_RM_L_MASK: u32 = 0x7 << 13;
pub const RT5670_G_BST1_RM_L_SFT: u32 = 13;
pub const RT5670_M_IN_L_RM_L: u32 = 0x1 << 5;
pub const RT5670_M_IN_L_RM_L_SFT: u32 = 5;
pub const RT5670_M_BST2_RM_L: u32 = 0x1 << 3;
pub const RT5670_M_BST2_RM_L_SFT: u32 = 3;
pub const RT5670_M_BST1_RM_L: u32 = 0x1 << 1;
pub const RT5670_M_BST1_RM_L_SFT: u32 = 1;

/* REC Right Mixer Control 1 (0x3d) */
pub const RT5670_G_HP_R_RM_R_MASK: u32 = 0x7 << 13;
pub const RT5670_G_HP_R_RM_R_SFT: u32 = 13;
pub const RT5670_G_IN_R_RM_R_MASK: u32 = 0x7 << 10;
pub const RT5670_G_IN_R_RM_R_SFT: u32 = 10;
pub const RT5670_G_BST4_RM_R_MASK: u32 = 0x7 << 7;
pub const RT5670_G_BST4_RM_R_SFT: u32 = 7;
pub const RT5670_G_BST3_RM_R_MASK: u32 = 0x7 << 4;
pub const RT5670_G_BST3_RM_R_SFT: u32 = 4;
pub const RT5670_G_BST2_RM_R_MASK: u32 = 0x7 << 1;
pub const RT5670_G_BST2_RM_R_SFT: u32 = 1;

/* REC Right Mixer Control 2 (0x3e) */
pub const RT5670_G_BST1_RM_R_MASK: u32 = 0x7 << 13;
pub const RT5670_G_BST1_RM_R_SFT: u32 = 13;
pub const RT5670_M_IN_R_RM_R: u32 = 0x1 << 5;
pub const RT5670_M_IN_R_RM_R_SFT: u32 = 5;
pub const RT5670_M_BST2_RM_R: u32 = 0x1 << 3;
pub const RT5670_M_BST2_RM_R_SFT: u32 = 3;
pub const RT5670_M_BST1_RM_R: u32 = 0x1 << 1;
pub const RT5670_M_BST1_RM_R_SFT: u32 = 1;

/* HPMIX Control (0x45) */
pub const RT5670_M_DAC2_HM: u32 = 0x1 << 15;
pub const RT5670_M_DAC2_HM_SFT: u32 = 15;
pub const RT5670_M_HPVOL_HM: u32 = 0x1 << 14;
pub const RT5670_M_HPVOL_HM_SFT: u32 = 14;
pub const RT5670_M_DAC1_HM: u32 = 0x1 << 13;
pub const RT5670_M_DAC1_HM_SFT: u32 = 13;
pub const RT5670_G_HPOMIX_MASK: u32 = 0x1 << 12;
pub const RT5670_G_HPOMIX_SFT: u32 = 12;
pub const RT5670_M_INR1_HMR: u32 = 0x1 << 3;
pub const RT5670_M_INR1_HMR_SFT: u32 = 3;
pub const RT5670_M_DACR1_HMR: u32 = 0x1 << 2;
pub const RT5670_M_DACR1_HMR_SFT: u32 = 2;
pub const RT5670_M_INL1_HML: u32 = 0x1 << 1;
pub const RT5670_M_INL1_HML_SFT: u32 = 1;
pub const RT5670_M_DACL1_HML: u32 = 0x1;
pub const RT5670_M_DACL1_HML_SFT: u32 = 0;

/* Mono Output Mixer Control (0x4c) */
pub const RT5670_M_DAC_R2_MA: u32 = 0x1 << 15;
pub const RT5670_M_DAC_R2_MA_SFT: u32 = 15;
pub const RT5670_M_DAC_L2_MA: u32 = 0x1 << 14;
pub const RT5670_M_DAC_L2_MA_SFT: u32 = 14;
pub const RT5670_M_OV_R_MM: u32 = 0x1 << 13;
pub const RT5670_M_OV_R_MM_SFT: u32 = 13;
pub const RT5670_M_OV_L_MM: u32 = 0x1 << 12;
pub const RT5670_M_OV_L_MM_SFT: u32 = 12;
pub const RT5670_G_MONOMIX_MASK: u32 = 0x1 << 10;
pub const RT5670_G_MONOMIX_SFT: u32 = 10;
pub const RT5670_M_DAC_R2_MM: u32 = 0x1 << 9;
pub const RT5670_M_DAC_R2_MM_SFT: u32 = 9;
pub const RT5670_M_DAC_L2_MM: u32 = 0x1 << 8;
pub const RT5670_M_DAC_L2_MM_SFT: u32 = 8;
pub const RT5670_M_BST4_MM: u32 = 0x1 << 7;
pub const RT5670_M_BST4_MM_SFT: u32 = 7;

/* Output Left Mixer Control 1 (0x4d) */
pub const RT5670_G_BST3_OM_L_MASK: u32 = 0x7 << 13;
pub const RT5670_G_BST3_OM_L_SFT: u32 = 13;
pub const RT5670_G_BST2_OM_L_MASK: u32 = 0x7 << 10;
pub const RT5670_G_BST2_OM_L_SFT: u32 = 10;
pub const RT5670_G_BST1_OM_L_MASK: u32 = 0x7 << 7;
pub const RT5670_G_BST1_OM_L_SFT: u32 = 7;
pub const RT5670_G_IN_L_OM_L_MASK: u32 = 0x7 << 4;
pub const RT5670_G_IN_L_OM_L_SFT: u32 = 4;
pub const RT5670_G_RM_L_OM_L_MASK: u32 = 0x7 << 1;
pub const RT5670_G_RM_L_OM_L_SFT: u32 = 1;

/* Output Left Mixer Control 2 (0x4e) */
pub const RT5670_G_DAC_R2_OM_L_MASK: u32 = 0x7 << 13;
pub const RT5670_G_DAC_R2_OM_L_SFT: u32 = 13;
pub const RT5670_G_DAC_L2_OM_L_MASK: u32 = 0x7 << 10;
pub const RT5670_G_DAC_L2_OM_L_SFT: u32 = 10;
pub const RT5670_G_DAC_L1_OM_L_MASK: u32 = 0x7 << 7;
pub const RT5670_G_DAC_L1_OM_L_SFT: u32 = 7;

/* Output Left Mixer Control 3 (0x4f) */
pub const RT5670_M_BST1_OM_L: u32 = 0x1 << 5;
pub const RT5670_M_BST1_OM_L_SFT: u32 = 5;
pub const RT5670_M_IN_L_OM_L: u32 = 0x1 << 4;
pub const RT5670_M_IN_L_OM_L_SFT: u32 = 4;
pub const RT5670_M_DAC_L2_OM_L: u32 = 0x1 << 1;
pub const RT5670_M_DAC_L2_OM_L_SFT: u32 = 1;
pub const RT5670_M_DAC_L1_OM_L: u32 = 0x1;
pub const RT5670_M_DAC_L1_OM_L_SFT: u32 = 0;

/* Output Right Mixer Control 1 (0x50) */
pub const RT5670_G_BST4_OM_R_MASK: u32 = 0x7 << 13;
pub const RT5670_G_BST4_OM_R_SFT: u32 = 13;
pub const RT5670_G_BST2_OM_R_MASK: u32 = 0x7 << 10;
pub const RT5670_G_BST2_OM_R_SFT: u32 = 10;
pub const RT5670_G_BST1_OM_R_MASK: u32 = 0x7 << 7;
pub const RT5670_G_BST1_OM_R_SFT: u32 = 7;
pub const RT5670_G_IN_R_OM_R_MASK: u32 = 0x7 << 4;
pub const RT5670_G_IN_R_OM_R_SFT: u32 = 4;
pub const RT5670_G_RM_R_OM_R_MASK: u32 = 0x7 << 1;
pub const RT5670_G_RM_R_OM_R_SFT: u32 = 1;

/* Output Right Mixer Control 2 (0x51) */
pub const RT5670_G_DAC_L2_OM_R_MASK: u32 = 0x7 << 13;
pub const RT5670_G_DAC_L2_OM_R_SFT: u32 = 13;
pub const RT5670_G_DAC_R2_OM_R_MASK: u32 = 0x7 << 10;
pub const RT5670_G_DAC_R2_OM_R_SFT: u32 = 10;
pub const RT5670_G_DAC_R1_OM_R_MASK: u32 = 0x7 << 7;
pub const RT5670_G_DAC_R1_OM_R_SFT: u32 = 7;

/* Output Right Mixer Control 3 (0x52) */
pub const RT5670_M_BST2_OM_R: u32 = 0x1 << 6;
pub const RT5670_M_BST2_OM_R_SFT: u32 = 6;
pub const RT5670_M_IN_R_OM_R: u32 = 0x1 << 4;
pub const RT5670_M_IN_R_OM_R_SFT: u32 = 4;
pub const RT5670_M_DAC_R2_OM_R: u32 = 0x1 << 1;
pub const RT5670_M_DAC_R2_OM_R_SFT: u32 = 1;
pub const RT5670_M_DAC_R1_OM_R: u32 = 0x1;
pub const RT5670_M_DAC_R1_OM_R_SFT: u32 = 0;

/* LOUT Mixer Control (0x53) */
pub const RT5670_M_DAC_L1_LM: u32 = 0x1 << 15;
pub const RT5670_M_DAC_L1_LM_SFT: u32 = 15;
pub const RT5670_M_DAC_R1_LM: u32 = 0x1 << 14;
pub const RT5670_M_DAC_R1_LM_SFT: u32 = 14;
pub const RT5670_M_OV_L_LM: u32 = 0x1 << 13;
pub const RT5670_M_OV_L_LM_SFT: u32 = 13;
pub const RT5670_M_OV_R_LM: u32 = 0x1 << 12;
pub const RT5670_M_OV_R_LM_SFT: u32 = 12;
pub const RT5670_G_LOUTMIX_MASK: u32 = 0x1 << 11;
pub const RT5670_G_LOUTMIX_SFT: u32 = 11;

/* Power Management for Digital 1 (0x61) */
pub const RT5670_PWR_I2S1: u32 = 0x1 << 15;
pub const RT5670_PWR_I2S1_BIT: u32 = 15;
pub const RT5670_PWR_I2S2: u32 = 0x1 << 14;
pub const RT5670_PWR_I2S2_BIT: u32 = 14;
pub const RT5670_PWR_DAC_L1: u32 = 0x1 << 12;
pub const RT5670_PWR_DAC_L1_BIT: u32 = 12;
pub const RT5670_PWR_DAC_R1: u32 = 0x1 << 11;
pub const RT5670_PWR_DAC_R1_BIT: u32 = 11;
pub const RT5670_PWR_DAC_L2: u32 = 0x1 << 7;
pub const RT5670_PWR_DAC_L2_BIT: u32 = 7;
pub const RT5670_PWR_DAC_R2: u32 = 0x1 << 6;
pub const RT5670_PWR_DAC_R2_BIT: u32 = 6;
pub const RT5670_PWR_ADC_L: u32 = 0x1 << 2;
pub const RT5670_PWR_ADC_L_BIT: u32 = 2;
pub const RT5670_PWR_ADC_R: u32 = 0x1 << 1;
pub const RT5670_PWR_ADC_R_BIT: u32 = 1;
pub const RT5670_PWR_CLS_D: u32 = 0x1;
pub const RT5670_PWR_CLS_D_BIT: u32 = 0;

/* Power Management for Digital 2 (0x62) */
pub const RT5670_PWR_ADC_S1F: u32 = 0x1 << 15;
pub const RT5670_PWR_ADC_S1F_BIT: u32 = 15;
pub const RT5670_PWR_ADC_MF_L: u32 = 0x1 << 14;
pub const RT5670_PWR_ADC_MF_L_BIT: u32 = 14;
pub const RT5670_PWR_ADC_MF_R: u32 = 0x1 << 13;
pub const RT5670_PWR_ADC_MF_R_BIT: u32 = 13;
pub const RT5670_PWR_I2S_DSP: u32 = 0x1 << 12;
pub const RT5670_PWR_I2S_DSP_BIT: u32 = 12;
pub const RT5670_PWR_DAC_S1F: u32 = 0x1 << 11;
pub const RT5670_PWR_DAC_S1F_BIT: u32 = 11;
pub const RT5670_PWR_DAC_MF_L: u32 = 0x1 << 10;
pub const RT5670_PWR_DAC_MF_L_BIT: u32 = 10;
pub const RT5670_PWR_DAC_MF_R: u32 = 0x1 << 9;
pub const RT5670_PWR_DAC_MF_R_BIT: u32 = 9;
pub const RT5670_PWR_ADC_S2F: u32 = 0x1 << 8;
pub const RT5670_PWR_ADC_S2F_BIT: u32 = 8;
pub const RT5670_PWR_PDM1: u32 = 0x1 << 7;
pub const RT5670_PWR_PDM1_BIT: u32 = 7;
pub const RT5670_PWR_PDM2: u32 = 0x1 << 6;
pub const RT5670_PWR_PDM2_BIT: u32 = 6;

/* Power Management for Analog 1 (0x63) */
pub const RT5670_PWR_VREF1: u32 = 0x1 << 15;
pub const RT5670_PWR_VREF1_BIT: u32 = 15;
pub const RT5670_PWR_FV1: u32 = 0x1 << 14;
pub const RT5670_PWR_FV1_BIT: u32 = 14;
pub const RT5670_PWR_MB: u32 = 0x1 << 13;
pub const RT5670_PWR_MB_BIT: u32 = 13;
pub const RT5670_PWR_LM: u32 = 0x1 << 12;
pub const RT5670_PWR_LM_BIT: u32 = 12;
pub const RT5670_PWR_BG: u32 = 0x1 << 11;
pub const RT5670_PWR_BG_BIT: u32 = 11;
pub const RT5670_PWR_HP_L: u32 = 0x1 << 7;
pub const RT5670_PWR_HP_L_BIT: u32 = 7;
pub const RT5670_PWR_HP_R: u32 = 0x1 << 6;
pub const RT5670_PWR_HP_R_BIT: u32 = 6;
pub const RT5670_PWR_HA: u32 = 0x1 << 5;
pub const RT5670_PWR_HA_BIT: u32 = 5;
pub const RT5670_PWR_VREF2: u32 = 0x1 << 4;
pub const RT5670_PWR_VREF2_BIT: u32 = 4;
pub const RT5670_PWR_FV2: u32 = 0x1 << 3;
pub const RT5670_PWR_FV2_BIT: u32 = 3;
pub const RT5670_LDO_SEL_MASK: u32 = 0x7;
pub const RT5670_LDO_SEL_SFT: u32 = 0;

/* Power Management for Analog 2 (0x64) */
pub const RT5670_PWR_BST1: u32 = 0x1 << 15;
pub const RT5670_PWR_BST1_BIT: u32 = 15;
pub const RT5670_PWR_BST2: u32 = 0x1 << 13;
pub const RT5670_PWR_BST2_BIT: u32 = 13;
pub const RT5670_PWR_MB1: u32 = 0x1 << 11;
pub const RT5670_PWR_MB1_BIT: u32 = 11;
pub const RT5670_PWR_MB2: u32 = 0x1 << 10;
pub const RT5670_PWR_MB2_BIT: u32 = 10;
pub const RT5670_PWR_PLL: u32 = 0x1 << 9;
pub const RT5670_PWR_PLL_BIT: u32 = 9;
pub const RT5670_PWR_BST1_P: u32 = 0x1 << 6;
pub const RT5670_PWR_BST1_P_BIT: u32 = 6;
pub const RT5670_PWR_BST2_P: u32 = 0x1 << 4;
pub const RT5670_PWR_BST2_P_BIT: u32 = 4;
pub const RT5670_PWR_JD1: u32 = 0x1 << 2;
pub const RT5670_PWR_JD1_BIT: u32 = 2;
pub const RT5670_PWR_JD: u32 = 0x1 << 1;
pub const RT5670_PWR_JD_BIT: u32 = 1;

/* Power Management for Mixer (0x65) */
pub const RT5670_PWR_OM_L: u32 = 0x1 << 15;
pub const RT5670_PWR_OM_L_BIT: u32 = 15;
pub const RT5670_PWR_OM_R: u32 = 0x1 << 14;
pub const RT5670_PWR_OM_R_BIT: u32 = 14;
pub const RT5670_PWR_RM_L: u32 = 0x1 << 11;
pub const RT5670_PWR_RM_L_BIT: u32 = 11;
pub const RT5670_PWR_RM_R: u32 = 0x1 << 10;
pub const RT5670_PWR_RM_R_BIT: u32 = 10;

/* Power Management for Volume (0x66) */
pub const RT5670_PWR_HV_L: u32 = 0x1 << 11;
pub const RT5670_PWR_HV_L_BIT: u32 = 11;
pub const RT5670_PWR_HV_R: u32 = 0x1 << 10;
pub const RT5670_PWR_HV_R_BIT: u32 = 10;
pub const RT5670_PWR_IN_L: u32 = 0x1 << 9;
pub const RT5670_PWR_IN_L_BIT: u32 = 9;
pub const RT5670_PWR_IN_R: u32 = 0x1 << 8;
pub const RT5670_PWR_IN_R_BIT: u32 = 8;
pub const RT5670_PWR_MIC_DET: u32 = 0x1 << 5;
pub const RT5670_PWR_MIC_DET_BIT: u32 = 5;

/* I2S1/2/3 Audio Serial Data Port Control (0x70 0x71 0x72) */
pub const RT5670_I2S_MS_MASK: u32 = 0x1 << 15;
pub const RT5670_I2S_MS_SFT: u32 = 15;
pub const RT5670_I2S_MS_M: u32 = 0x0 << 15;
pub const RT5670_I2S_MS_S: u32 = 0x1 << 15;
pub const RT5670_I2S_IF_MASK: u32 = 0x7 << 12;
pub const RT5670_I2S_IF_SFT: u32 = 12;
pub const RT5670_I2S_O_CP_MASK: u32 = 0x3 << 10;
pub const RT5670_I2S_O_CP_SFT: u32 = 10;
pub const RT5670_I2S_O_CP_OFF: u32 = 0x0 << 10;
pub const RT5670_I2S_O_CP_U_LAW: u32 = 0x1 << 10;
pub const RT5670_I2S_O_CP_A_LAW: u32 = 0x2 << 10;
pub const RT5670_I2S_I_CP_MASK: u32 = 0x3 << 8;
pub const RT5670_I2S_I_CP_SFT: u32 = 8;
pub const RT5670_I2S_I_CP_OFF: u32 = 0x0 << 8;
pub const RT5670_I2S_I_CP_U_LAW: u32 = 0x1 << 8;
pub const RT5670_I2S_I_CP_A_LAW: u32 = 0x2 << 8;
pub const RT5670_I2S_BP_MASK: u32 = 0x1 << 7;
pub const RT5670_I2S_BP_SFT: u32 = 7;
pub const RT5670_I2S_BP_NOR: u32 = 0x0 << 7;
pub const RT5670_I2S_BP_INV: u32 = 0x1 << 7;
pub const RT5670_I2S_DL_MASK: u32 = 0x3 << 2;
pub const RT5670_I2S_DL_SFT: u32 = 2;
pub const RT5670_I2S_DL_16: u32 = 0x0 << 2;
pub const RT5670_I2S_DL_20: u32 = 0x1 << 2;
pub const RT5670_I2S_DL_24: u32 = 0x2 << 2;
pub const RT5670_I2S_DL_8: u32 = 0x3 << 2;
pub const RT5670_I2S_DF_MASK: u32 = 0x3;
pub const RT5670_I2S_DF_SFT: u32 = 0;
pub const RT5670_I2S_DF_I2S: u32 = 0x0;
pub const RT5670_I2S_DF_LEFT: u32 = 0x1;
pub const RT5670_I2S_DF_PCM_A: u32 = 0x2;
pub const RT5670_I2S_DF_PCM_B: u32 = 0x3;

/* I2S2 Audio Serial Data Port Control (0x71) */
pub const RT5670_I2S2_SDI_MASK: u32 = 0x1 << 6;
pub const RT5670_I2S2_SDI_SFT: u32 = 6;
pub const RT5670_I2S2_SDI_I2S1: u32 = 0x0 << 6;
pub const RT5670_I2S2_SDI_I2S2: u32 = 0x1 << 6;

/* ADC/DAC Clock Control 1 (0x73) */
pub const RT5670_I2S_BCLK_MS1_MASK: u32 = 0x1 << 15;
pub const RT5670_I2S_BCLK_MS1_SFT: u32 = 15;
pub const RT5670_I2S_BCLK_MS1_32: u32 = 0x0 << 15;
pub const RT5670_I2S_BCLK_MS1_64: u32 = 0x1 << 15;
pub const RT5670_I2S_PD1_MASK: u32 = 0x7 << 12;
pub const RT5670_I2S_PD1_SFT: u32 = 12;
pub const RT5670_I2S_PD1_1: u32 = 0x0 << 12;
pub const RT5670_I2S_PD1_2: u32 = 0x1 << 12;
pub const RT5670_I2S_PD1_3: u32 = 0x2 << 12;
pub const RT5670_I2S_PD1_4: u32 = 0x3 << 12;
pub const RT5670_I2S_PD1_6: u32 = 0x4 << 12;
pub const RT5670_I2S_PD1_8: u32 = 0x5 << 12;
pub const RT5670_I2S_PD1_12: u32 = 0x6 << 12;
pub const RT5670_I2S_PD1_16: u32 = 0x7 << 12;
pub const RT5670_I2S_BCLK_MS2_MASK: u32 = 0x1 << 11;
pub const RT5670_I2S_BCLK_MS2_SFT: u32 = 11;
pub const RT5670_I2S_BCLK_MS2_32: u32 = 0x0 << 11;
pub const RT5670_I2S_BCLK_MS2_64: u32 = 0x1 << 11;
pub const RT5670_I2S_PD2_MASK: u32 = 0x7 << 8;
pub const RT5670_I2S_PD2_SFT: u32 = 8;
pub const RT5670_I2S_PD2_1: u32 = 0x0 << 8;
pub const RT5670_I2S_PD2_2: u32 = 0x1 << 8;
pub const RT5670_I2S_PD2_3: u32 = 0x2 << 8;
pub const RT5670_I2S_PD2_4: u32 = 0x3 << 8;
pub const RT5670_I2S_PD2_6: u32 = 0x4 << 8;
pub const RT5670_I2S_PD2_8: u32 = 0x5 << 8;
pub const RT5670_I2S_PD2_12: u32 = 0x6 << 8;
pub const RT5670_I2S_PD2_16: u32 = 0x7 << 8;
pub const RT5670_I2S_BCLK_MS3_MASK: u32 = 0x1 << 7;
pub const RT5670_I2S_BCLK_MS3_SFT: u32 = 7;
pub const RT5670_I2S_BCLK_MS3_32: u32 = 0x0 << 7;
pub const RT5670_I2S_BCLK_MS3_64: u32 = 0x1 << 7;
pub const RT5670_I2S_PD3_MASK: u32 = 0x7 << 4;
pub const RT5670_I2S_PD3_SFT: u32 = 4;
pub const RT5670_I2S_PD3_1: u32 = 0x0 << 4;
pub const RT5670_I2S_PD3_2: u32 = 0x1 << 4;
pub const RT5670_I2S_PD3_3: u32 = 0x2 << 4;
pub const RT5670_I2S_PD3_4: u32 = 0x3 << 4;
pub const RT5670_I2S_PD3_6: u32 = 0x4 << 4;
pub const RT5670_I2S_PD3_8: u32 = 0x5 << 4;
pub const RT5670_I2S_PD3_12: u32 = 0x6 << 4;
pub const RT5670_I2S_PD3_16: u32 = 0x7 << 4;
pub const RT5670_DAC_OSR_MASK: u32 = 0x3 << 2;
pub const RT5670_DAC_OSR_SFT: u32 = 2;
pub const RT5670_DAC_OSR_128: u32 = 0x0 << 2;
pub const RT5670_DAC_OSR_64: u32 = 0x1 << 2;
pub const RT5670_DAC_OSR_32: u32 = 0x2 << 2;
pub const RT5670_DAC_OSR_16: u32 = 0x3 << 2;
pub const RT5670_ADC_OSR_MASK: u32 = 0x3;
pub const RT5670_ADC_OSR_SFT: u32 = 0;
pub const RT5670_ADC_OSR_128: u32 = 0x0;
pub const RT5670_ADC_OSR_64: u32 = 0x1;
pub const RT5670_ADC_OSR_32: u32 = 0x2;
pub const RT5670_ADC_OSR_16: u32 = 0x3;

/* ADC/DAC Clock Control 2 (0x74) */
pub const RT5670_DAC_L_OSR_MASK: u32 = 0x3 << 14;
pub const RT5670_DAC_L_OSR_SFT: u32 = 14;
pub const RT5670_DAC_L_OSR_128: u32 = 0x0 << 14;
pub const RT5670_DAC_L_OSR_64: u32 = 0x1 << 14;
pub const RT5670_DAC_L_OSR_32: u32 = 0x2 << 14;
pub const RT5670_DAC_L_OSR_16: u32 = 0x3 << 14;
pub const RT5670_ADC_R_OSR_MASK: u32 = 0x3 << 12;
pub const RT5670_ADC_R_OSR_SFT: u32 = 12;
pub const RT5670_ADC_R_OSR_128: u32 = 0x0 << 12;
pub const RT5670_ADC_R_OSR_64: u32 = 0x1 << 12;
pub const RT5670_ADC_R_OSR_32: u32 = 0x2 << 12;
pub const RT5670_ADC_R_OSR_16: u32 = 0x3 << 12;
pub const RT5670_DAHPF_EN: u32 = 0x1 << 11;
pub const RT5670_DAHPF_EN_SFT: u32 = 11;
pub const RT5670_ADHPF_EN: u32 = 0x1 << 10;
pub const RT5670_ADHPF_EN_SFT: u32 = 10;

/* Digital Microphone Control (0x75) */
pub const RT5670_DMIC_1_EN_MASK: u32 = 0x1 << 15;
pub const RT5670_DMIC_1_EN_SFT: u32 = 15;
pub const RT5670_DMIC_1_DIS: u32 = 0x0 << 15;
pub const RT5670_DMIC_1_EN: u32 = 0x1 << 15;
pub const RT5670_DMIC_2_EN_MASK: u32 = 0x1 << 14;
pub const RT5670_DMIC_2_EN_SFT: u32 = 14;
pub const RT5670_DMIC_2_DIS: u32 = 0x0 << 14;
pub const RT5670_DMIC_2_EN: u32 = 0x1 << 14;
pub const RT5670_DMIC_1L_LH_MASK: u32 = 0x1 << 13;
pub const RT5670_DMIC_1L_LH_SFT: u32 = 13;
pub const RT5670_DMIC_1L_LH_FALLING: u32 = 0x0 << 13;
pub const RT5670_DMIC_1L_LH_RISING: u32 = 0x1 << 13;
pub const RT5670_DMIC_1R_LH_MASK: u32 = 0x1 << 12;
pub const RT5670_DMIC_1R_LH_SFT: u32 = 12;
pub const RT5670_DMIC_1R_LH_FALLING: u32 = 0x0 << 12;
pub const RT5670_DMIC_1R_LH_RISING: u32 = 0x1 << 12;
pub const RT5670_DMIC_2_DP_MASK: u32 = 0x1 << 10;
pub const RT5670_DMIC_2_DP_SFT: u32 = 10;
pub const RT5670_DMIC_2_DP_GPIO8: u32 = 0x0 << 10;
pub const RT5670_DMIC_2_DP_IN3N: u32 = 0x1 << 10;
pub const RT5670_DMIC_2L_LH_MASK: u32 = 0x1 << 9;
pub const RT5670_DMIC_2L_LH_SFT: u32 = 9;
pub const RT5670_DMIC_2L_LH_FALLING: u32 = 0x0 << 9;
pub const RT5670_DMIC_2L_LH_RISING: u32 = 0x1 << 9;
pub const RT5670_DMIC_2R_LH_MASK: u32 = 0x1 << 8;
pub const RT5670_DMIC_2R_LH_SFT: u32 = 8;
pub const RT5670_DMIC_2R_LH_FALLING: u32 = 0x0 << 8;
pub const RT5670_DMIC_2R_LH_RISING: u32 = 0x1 << 8;
pub const RT5670_DMIC_CLK_MASK: u32 = 0x7 << 5;
pub const RT5670_DMIC_CLK_SFT: u32 = 5;
pub const RT5670_DMIC_3_EN_MASK: u32 = 0x1 << 4;
pub const RT5670_DMIC_3_EN_SFT: u32 = 4;
pub const RT5670_DMIC_3_DIS: u32 = 0x0 << 4;
pub const RT5670_DMIC_3_EN: u32 = 0x1 << 4;
pub const RT5670_DMIC_1_DP_MASK: u32 = 0x3 << 0;
pub const RT5670_DMIC_1_DP_SFT: u32 = 0;
pub const RT5670_DMIC_1_DP_GPIO6: u32 = 0x0 << 0;
pub const RT5670_DMIC_1_DP_IN2P: u32 = 0x1 << 0;
pub const RT5670_DMIC_1_DP_GPIO7: u32 = 0x2 << 0;

/* Digital Microphone Control2 (0x76) */
pub const RT5670_DMIC_3_DP_MASK: u32 = 0x3 << 6;
pub const RT5670_DMIC_3_DP_SFT: u32 = 6;
pub const RT5670_DMIC_3_DP_GPIO9: u32 = 0x0 << 6;
pub const RT5670_DMIC_3_DP_GPIO10: u32 = 0x1 << 6;
pub const RT5670_DMIC_3_DP_GPIO5: u32 = 0x2 << 6;

/* Global Clock Control (0x80) */
pub const RT5670_SCLK_SRC_MASK: u32 = 0x3 << 14;
pub const RT5670_SCLK_SRC_SFT: u32 = 14;
pub const RT5670_SCLK_SRC_MCLK: u32 = 0x0 << 14;
pub const RT5670_SCLK_SRC_PLL1: u32 = 0x1 << 14;
pub const RT5670_SCLK_SRC_RCCLK: u32 = 0x2 << 14; /* 15MHz */
pub const RT5670_PLL1_SRC_MASK: u32 = 0x7 << 11;
pub const RT5670_PLL1_SRC_SFT: u32 = 11;
pub const RT5670_PLL1_SRC_MCLK: u32 = 0x0 << 11;
pub const RT5670_PLL1_SRC_BCLK1: u32 = 0x1 << 11;
pub const RT5670_PLL1_SRC_BCLK2: u32 = 0x2 << 11;
pub const RT5670_PLL1_SRC_BCLK3: u32 = 0x3 << 11;
pub const RT5670_PLL1_PD_MASK: u32 = 0x1 << 3;
pub const RT5670_PLL1_PD_SFT: u32 = 3;
pub const RT5670_PLL1_PD_1: u32 = 0x0 << 3;
pub const RT5670_PLL1_PD_2: u32 = 0x1 << 3;

pub const RT5670_PLL_INP_MAX: u32 = 40000000;
pub const RT5670_PLL_INP_MIN: u32 = 256000;
/* PLL M/N/K Code Control 1 (0x81) */
pub const RT5670_PLL_N_MAX: u32 = 0x1ff;
pub const RT5670_PLL_N_MASK: u32 = RT5670_PLL_N_MAX << 7;
pub const RT5670_PLL_N_SFT: u32 = 7;
pub const RT5670_PLL_K_MAX: u32 = 0x1f;
pub const RT5670_PLL_K_MASK: u32 = RT5670_PLL_K_MAX;
pub const RT5670_PLL_K_SFT: u32 = 0;

/* PLL M/N/K Code Control 2 (0x82) */
pub const RT5670_PLL_M_MAX: u32 = 0xf;
pub const RT5670_PLL_M_MASK: u32 = RT5670_PLL_M_MAX << 12;
pub const RT5670_PLL_M_SFT: u32 = 12;
pub const RT5670_PLL_M_BP: u32 = 0x1 << 11;
pub const RT5670_PLL_M_BP_SFT: u32 = 11;

/* ASRC Control 1 (0x83) */
pub const RT5670_STO_T_MASK: u32 = 0x1 << 15;
pub const RT5670_STO_T_SFT: u32 = 15;
pub const RT5670_STO_T_SCLK: u32 = 0x0 << 15;
pub const RT5670_STO_T_LRCK1: u32 = 0x1 << 15;
pub const RT5670_M1_T_MASK: u32 = 0x1 << 14;
pub const RT5670_M1_T_SFT: u32 = 14;
pub const RT5670_M1_T_I2S2: u32 = 0x0 << 14;
pub const RT5670_M1_T_I2S2_D3: u32 = 0x1 << 14;
pub const RT5670_I2S2_F_MASK: u32 = 0x1 << 12;
pub const RT5670_I2S2_F_SFT: u32 = 12;
pub const RT5670_I2S2_F_I2S2_D2: u32 = 0x0 << 12;
pub const RT5670_I2S2_F_I2S1_TCLK: u32 = 0x1 << 12;
pub const RT5670_DMIC_1_M_MASK: u32 = 0x1 << 9;
pub const RT5670_DMIC_1_M_SFT: u32 = 9;
pub const RT5670_DMIC_1_M_NOR: u32 = 0x0 << 9;
pub const RT5670_DMIC_1_M_ASYN: u32 = 0x1 << 9;
pub const RT5670_DMIC_2_M_MASK: u32 = 0x1 << 8;
pub const RT5670_DMIC_2_M_SFT: u32 = 8;
pub const RT5670_DMIC_2_M_NOR: u32 = 0x0 << 8;
pub const RT5670_DMIC_2_M_ASYN: u32 = 0x1 << 8;

/* ASRC clock source selection (0x84, 0x85) */
pub const RT5670_CLK_SEL_SYS: u32 = 0x0;
pub const RT5670_CLK_SEL_I2S1_ASRC: u32 = 0x1;
pub const RT5670_CLK_SEL_I2S2_ASRC: u32 = 0x2;
pub const RT5670_CLK_SEL_I2S3_ASRC: u32 = 0x3;
pub const RT5670_CLK_SEL_SYS2: u32 = 0x5;
pub const RT5670_CLK_SEL_SYS3: u32 = 0x6;

/* ASRC Control 2 (0x84) */
pub const RT5670_DA_STO_CLK_SEL_MASK: u32 = 0xf << 12;
pub const RT5670_DA_STO_CLK_SEL_SFT: u32 = 12;
pub const RT5670_DA_MONOL_CLK_SEL_MASK: u32 = 0xf << 8;
pub const RT5670_DA_MONOL_CLK_SEL_SFT: u32 = 8;
pub const RT5670_DA_MONOR_CLK_SEL_MASK: u32 = 0xf << 4;
pub const RT5670_DA_MONOR_CLK_SEL_SFT: u32 = 4;
pub const RT5670_AD_STO1_CLK_SEL_MASK: u32 = 0xf << 0;
pub const RT5670_AD_STO1_CLK_SEL_SFT: u32 = 0;

/* ASRC Control 3 (0x85) */
pub const RT5670_UP_CLK_SEL_MASK: u32 = 0xf << 12;
pub const RT5670_UP_CLK_SEL_SFT: u32 = 12;
pub const RT5670_DOWN_CLK_SEL_MASK: u32 = 0xf << 8;
pub const RT5670_DOWN_CLK_SEL_SFT: u32 = 8;
pub const RT5670_AD_MONOL_CLK_SEL_MASK: u32 = 0xf << 4;
pub const RT5670_AD_MONOL_CLK_SEL_SFT: u32 = 4;
pub const RT5670_AD_MONOR_CLK_SEL_MASK: u32 = 0xf << 0;
pub const RT5670_AD_MONOR_CLK_SEL_SFT: u32 = 0;

/* ASRC Control 4 (0x89) */
pub const RT5670_I2S1_PD_MASK: u32 = 0x7 << 12;
pub const RT5670_I2S1_PD_SFT: u32 = 12;
pub const RT5670_I2S2_PD_MASK: u32 = 0x7 << 8;
pub const RT5670_I2S2_PD_SFT: u32 = 8;

/* HPOUT Over Current Detection (0x8b) */
pub const RT5670_HP_OVCD_MASK: u32 = 0x1 << 10;
pub const RT5670_HP_OVCD_SFT: u32 = 10;
pub const RT5670_HP_OVCD_DIS: u32 = 0x0 << 10;
pub const RT5670_HP_OVCD_EN: u32 = 0x1 << 10;
pub const RT5670_HP_OC_TH_MASK: u32 = 0x3 << 8;
pub const RT5670_HP_OC_TH_SFT: u32 = 8;
pub const RT5670_HP_OC_TH_90: u32 = 0x0 << 8;
pub const RT5670_HP_OC_TH_105: u32 = 0x1 << 8;
pub const RT5670_HP_OC_TH_120: u32 = 0x2 << 8;
pub const RT5670_HP_OC_TH_135: u32 = 0x3 << 8;

/* Class D Over Current Control (0x8c) */
pub const RT5670_CLSD_OC_MASK: u32 = 0x1 << 9;
pub const RT5670_CLSD_OC_SFT: u32 = 9;
pub const RT5670_CLSD_OC_PU: u32 = 0x0 << 9;
pub const RT5670_CLSD_OC_PD: u32 = 0x1 << 9;
pub const RT5670_AUTO_PD_MASK: u32 = 0x1 << 8;
pub const RT5670_AUTO_PD_SFT: u32 = 8;
pub const RT5670_AUTO_PD_DIS: u32 = 0x0 << 8;
pub const RT5670_AUTO_PD_EN: u32 = 0x1 << 8;
pub const RT5670_CLSD_OC_TH_MASK: u32 = 0x3f;
pub const RT5670_CLSD_OC_TH_SFT: u32 = 0;

/* Class D Output Control (0x8d) */
pub const RT5670_CLSD_RATIO_MASK: u32 = 0xf << 12;
pub const RT5670_CLSD_RATIO_SFT: u32 = 12;
pub const RT5670_CLSD_OM_MASK: u32 = 0x1 << 11;
pub const RT5670_CLSD_OM_SFT: u32 = 11;
pub const RT5670_CLSD_OM_MONO: u32 = 0x0 << 11;
pub const RT5670_CLSD_OM_STO: u32 = 0x1 << 11;
pub const RT5670_CLSD_SCH_MASK: u32 = 0x1 << 10;
pub const RT5670_CLSD_SCH_SFT: u32 = 10;
pub const RT5670_CLSD_SCH_L: u32 = 0x0 << 10;
pub const RT5670_CLSD_SCH_S: u32 = 0x1 << 10;

/* Depop Mode Control 1 (0x8e) */
pub const RT5670_SMT_TRIG_MASK: u32 = 0x1 << 15;
pub const RT5670_SMT_TRIG_SFT: u32 = 15;
pub const RT5670_SMT_TRIG_DIS: u32 = 0x0 << 15;
pub const RT5670_SMT_TRIG_EN: u32 = 0x1 << 15;
pub const RT5670_HP_L_SMT_MASK: u32 = 0x1 << 9;
pub const RT5670_HP_L_SMT_SFT: u32 = 9;
pub const RT5670_HP_L_SMT_DIS: u32 = 0x0 << 9;
pub const RT5670_HP_L_SMT_EN: u32 = 0x1 << 9;
pub const RT5670_HP_R_SMT_MASK: u32 = 0x1 << 8;
pub const RT5670_HP_R_SMT_SFT: u32 = 8;
pub const RT5670_HP_R_SMT_DIS: u32 = 0x0 << 8;
pub const RT5670_HP_R_SMT_EN: u32 = 0x1 << 8;
pub const RT5670_HP_CD_PD_MASK: u32 = 0x1 << 7;
pub const RT5670_HP_CD_PD_SFT: u32 = 7;
pub const RT5670_HP_CD_PD_DIS: u32 = 0x0 << 7;
pub const RT5670_HP_CD_PD_EN: u32 = 0x1 << 7;
pub const RT5670_RSTN_MASK: u32 = 0x1 << 6;
pub const RT5670_RSTN_SFT: u32 = 6;
pub const RT5670_RSTN_DIS: u32 = 0x0 << 6;
pub const RT5670_RSTN_EN: u32 = 0x1 << 6;
pub const RT5670_RSTP_MASK: u32 = 0x1 << 5;
pub const RT5670_RSTP_SFT: u32 = 5;
pub const RT5670_RSTP_DIS: u32 = 0x0 << 5;
pub const RT5670_RSTP_EN: u32 = 0x1 << 5;
pub const RT5670_HP_CO_MASK: u32 = 0x1 << 4;
pub const RT5670_HP_CO_SFT: u32 = 4;
pub const RT5670_HP_CO_DIS: u32 = 0x0 << 4;
pub const RT5670_HP_CO_EN: u32 = 0x1 << 4;
pub const RT5670_HP_CP_MASK: u32 = 0x1 << 3;
pub const RT5670_HP_CP_SFT: u32 = 3;
pub const RT5670_HP_CP_PD: u32 = 0x0 << 3;
pub const RT5670_HP_CP_PU: u32 = 0x1 << 3;
pub const RT5670_HP_SG_MASK: u32 = 0x1 << 2;
pub const RT5670_HP_SG_SFT: u32 = 2;
pub const RT5670_HP_SG_DIS: u32 = 0x0 << 2;
pub const RT5670_HP_SG_EN: u32 = 0x1 << 2;
pub const RT5670_HP_DP_MASK: u32 = 0x1 << 1;
pub const RT5670_HP_DP_SFT: u32 = 1;
pub const RT5670_HP_DP_PD: u32 = 0x0 << 1;
pub const RT5670_HP_DP_PU: u32 = 0x1 << 1;
pub const RT5670_HP_CB_MASK: u32 = 0x1;
pub const RT5670_HP_CB_SFT: u32 = 0;
pub const RT5670_HP_CB_PD: u32 = 0x0;
pub const RT5670_HP_CB_PU: u32 = 0x1;

/* Depop Mode Control 2 (0x8f) */
pub const RT5670_DEPOP_MASK: u32 = 0x1 << 13;
pub const RT5670_DEPOP_SFT: u32 = 13;
pub const RT5670_DEPOP_AUTO: u32 = 0x0 << 13;
pub const RT5670_DEPOP_MAN: u32 = 0x1 << 13;
pub const RT5670_RAMP_MASK: u32 = 0x1 << 12;
pub const RT5670_RAMP_SFT: u32 = 12;
pub const RT5670_RAMP_DIS: u32 = 0x0 << 12;
pub const RT5670_RAMP_EN: u32 = 0x1 << 12;
pub const RT5670_BPS_MASK: u32 = 0x1 << 11;
pub const RT5670_BPS_SFT: u32 = 11;
pub const RT5670_BPS_DIS: u32 = 0x0 << 11;
pub const RT5670_BPS_EN: u32 = 0x1 << 11;
pub const RT5670_FAST_UPDN_MASK: u32 = 0x1 << 10;
pub const RT5670_FAST_UPDN_SFT: u32 = 10;
pub const RT5670_FAST_UPDN_DIS: u32 = 0x0 << 10;
pub const RT5670_FAST_UPDN_EN: u32 = 0x1 << 10;
pub const RT5670_MRES_MASK: u32 = 0x3 << 8;
pub const RT5670_MRES_SFT: u32 = 8;
pub const RT5670_MRES_15MO: u32 = 0x0 << 8;
pub const RT5670_MRES_25MO: u32 = 0x1 << 8;
pub const RT5670_MRES_35MO: u32 = 0x2 << 8;
pub const RT5670_MRES_45MO: u32 = 0x3 << 8;
pub const RT5670_VLO_MASK: u32 = 0x1 << 7;
pub const RT5670_VLO_SFT: u32 = 7;
pub const RT5670_VLO_3V: u32 = 0x0 << 7;
pub const RT5670_VLO_32V: u32 = 0x1 << 7;
pub const RT5670_DIG_DP_MASK: u32 = 0x1 << 6;
pub const RT5670_DIG_DP_SFT: u32 = 6;
pub const RT5670_DIG_DP_DIS: u32 = 0x0 << 6;
pub const RT5670_DIG_DP_EN: u32 = 0x1 << 6;
pub const RT5670_DP_TH_MASK: u32 = 0x3 << 4;
pub const RT5670_DP_TH_SFT: u32 = 4;

/* Depop Mode Control 3 (0x90) */
pub const RT5670_CP_SYS_MASK: u32 = 0x7 << 12;
pub const RT5670_CP_SYS_SFT: u32 = 12;
pub const RT5670_CP_FQ1_MASK: u32 = 0x7 << 8;
pub const RT5670_CP_FQ1_SFT: u32 = 8;
pub const RT5670_CP_FQ2_MASK: u32 = 0x7 << 4;
pub const RT5670_CP_FQ2_SFT: u32 = 4;
pub const RT5670_CP_FQ3_MASK: u32 = 0x7;
pub const RT5670_CP_FQ3_SFT: u32 = 0;
pub const RT5670_CP_FQ_1_5_KHZ: u32 = 0;
pub const RT5670_CP_FQ_3_KHZ: u32 = 1;
pub const RT5670_CP_FQ_6_KHZ: u32 = 2;
pub const RT5670_CP_FQ_12_KHZ: u32 = 3;
pub const RT5670_CP_FQ_24_KHZ: u32 = 4;
pub const RT5670_CP_FQ_48_KHZ: u32 = 5;
pub const RT5670_CP_FQ_96_KHZ: u32 = 6;
pub const RT5670_CP_FQ_192_KHZ: u32 = 7;

/* HPOUT charge pump (0x91) */
pub const RT5670_OSW_L_MASK: u32 = 0x1 << 11;
pub const RT5670_OSW_L_SFT: u32 = 11;
pub const RT5670_OSW_L_DIS: u32 = 0x0 << 11;
pub const RT5670_OSW_L_EN: u32 = 0x1 << 11;
pub const RT5670_OSW_R_MASK: u32 = 0x1 << 10;
pub const RT5670_OSW_R_SFT: u32 = 10;
pub const RT5670_OSW_R_DIS: u32 = 0x0 << 10;
pub const RT5670_OSW_R_EN: u32 = 0x1 << 10;
pub const RT5670_PM_HP_MASK: u32 = 0x3 << 8;
pub const RT5670_PM_HP_SFT: u32 = 8;
pub const RT5670_PM_HP_LV: u32 = 0x0 << 8;
pub const RT5670_PM_HP_MV: u32 = 0x1 << 8;
pub const RT5670_PM_HP_HV: u32 = 0x2 << 8;
pub const RT5670_IB_HP_MASK: u32 = 0x3 << 6;
pub const RT5670_IB_HP_SFT: u32 = 6;
pub const RT5670_IB_HP_125IL: u32 = 0x0 << 6;
pub const RT5670_IB_HP_25IL: u32 = 0x1 << 6;
pub const RT5670_IB_HP_5IL: u32 = 0x2 << 6;
pub const RT5670_IB_HP_1IL: u32 = 0x3 << 6;

/* PV detection and SPK gain control (0x92) */
pub const RT5670_PVDD_DET_MASK: u32 = 0x1 << 15;
pub const RT5670_PVDD_DET_SFT: u32 = 15;
pub const RT5670_PVDD_DET_DIS: u32 = 0x0 << 15;
pub const RT5670_PVDD_DET_EN: u32 = 0x1 << 15;
pub const RT5670_SPK_AG_MASK: u32 = 0x1 << 14;
pub const RT5670_SPK_AG_SFT: u32 = 14;
pub const RT5670_SPK_AG_DIS: u32 = 0x0 << 14;
pub const RT5670_SPK_AG_EN: u32 = 0x1 << 14;

/* Micbias Control (0x93) */
pub const RT5670_MIC1_BS_MASK: u32 = 0x1 << 15;
pub const RT5670_MIC1_BS_SFT: u32 = 15;
pub const RT5670_MIC1_BS_9AV: u32 = 0x0 << 15;
pub const RT5670_MIC1_BS_75AV: u32 = 0x1 << 15;
pub const RT5670_MIC2_BS_MASK: u32 = 0x1 << 14;
pub const RT5670_MIC2_BS_SFT: u32 = 14;
pub const RT5670_MIC2_BS_9AV: u32 = 0x0 << 14;
pub const RT5670_MIC2_BS_75AV: u32 = 0x1 << 14;
pub const RT5670_MIC1_CLK_MASK: u32 = 0x1 << 13;
pub const RT5670_MIC1_CLK_SFT: u32 = 13;
pub const RT5670_MIC1_CLK_DIS: u32 = 0x0 << 13;
pub const RT5670_MIC1_CLK_EN: u32 = 0x1 << 13;
pub const RT5670_MIC2_CLK_MASK: u32 = 0x1 << 12;
pub const RT5670_MIC2_CLK_SFT: u32 = 12;
pub const RT5670_MIC2_CLK_DIS: u32 = 0x0 << 12;
pub const RT5670_MIC2_CLK_EN: u32 = 0x1 << 12;
pub const RT5670_MIC1_OVCD_MASK: u32 = 0x1 << 11;
pub const RT5670_MIC1_OVCD_SFT: u32 = 11;
pub const RT5670_MIC1_OVCD_DIS: u32 = 0x0 << 11;
pub const RT5670_MIC1_OVCD_EN: u32 = 0x1 << 11;
pub const RT5670_MIC1_OVTH_MASK: u32 = 0x3 << 9;
pub const RT5670_MIC1_OVTH_SFT: u32 = 9;
pub const RT5670_MIC1_OVTH_600UA: u32 = 0x0 << 9;
pub const RT5670_MIC1_OVTH_1500UA: u32 = 0x1 << 9;
pub const RT5670_MIC1_OVTH_2000UA: u32 = 0x2 << 9;
pub const RT5670_MIC2_OVCD_MASK: u32 = 0x1 << 8;
pub const RT5670_MIC2_OVCD_SFT: u32 = 8;
pub const RT5670_MIC2_OVCD_DIS: u32 = 0x0 << 8;
pub const RT5670_MIC2_OVCD_EN: u32 = 0x1 << 8;
pub const RT5670_MIC2_OVTH_MASK: u32 = 0x3 << 6;
pub const RT5670_MIC2_OVTH_SFT: u32 = 6;
pub const RT5670_MIC2_OVTH_600UA: u32 = 0x0 << 6;
pub const RT5670_MIC2_OVTH_1500UA: u32 = 0x1 << 6;
pub const RT5670_MIC2_OVTH_2000UA: u32 = 0x2 << 6;
pub const RT5670_PWR_MB_MASK: u32 = 0x1 << 5;
pub const RT5670_PWR_MB_SFT: u32 = 5;
pub const RT5670_PWR_MB_PD: u32 = 0x0 << 5;
pub const RT5670_PWR_MB_PU: u32 = 0x1 << 5;
pub const RT5670_PWR_CLK25M_MASK: u32 = 0x1 << 4;
pub const RT5670_PWR_CLK25M_SFT: u32 = 4;
pub const RT5670_PWR_CLK25M_PD: u32 = 0x0 << 4;
pub const RT5670_PWR_CLK25M_PU: u32 = 0x1 << 4;

/* Analog JD Control 1 (0x94) */
pub const RT5670_JD1_MODE_MASK: u32 = 0x3 << 0;
pub const RT5670_JD1_MODE_0: u32 = 0x0 << 0;
pub const RT5670_JD1_MODE_1: u32 = 0x1 << 0;
pub const RT5670_JD1_MODE_2: u32 = 0x2 << 0;

/* VAD Control 4 (0x9d) */
pub const RT5670_VAD_SEL_MASK: u32 = 0x3 << 8;
pub const RT5670_VAD_SEL_SFT: u32 = 8;

/* EQ Control 1 (0xb0) */
pub const RT5670_EQ_SRC_MASK: u32 = 0x1 << 15;
pub const RT5670_EQ_SRC_SFT: u32 = 15;
pub const RT5670_EQ_SRC_DAC: u32 = 0x0 << 15;
pub const RT5670_EQ_SRC_ADC: u32 = 0x1 << 15;
pub const RT5670_EQ_UPD: u32 = 0x1 << 14;
pub const RT5670_EQ_UPD_BIT: u32 = 14;
pub const RT5670_EQ_CD_MASK: u32 = 0x1 << 13;
pub const RT5670_EQ_CD_SFT: u32 = 13;
pub const RT5670_EQ_CD_DIS: u32 = 0x0 << 13;
pub const RT5670_EQ_CD_EN: u32 = 0x1 << 13;
pub const RT5670_EQ_DITH_MASK: u32 = 0x3 << 8;
pub const RT5670_EQ_DITH_SFT: u32 = 8;
pub const RT5670_EQ_DITH_NOR: u32 = 0x0 << 8;
pub const RT5670_EQ_DITH_LSB: u32 = 0x1 << 8;
pub const RT5670_EQ_DITH_LSB_1: u32 = 0x2 << 8;
pub const RT5670_EQ_DITH_LSB_2: u32 = 0x3 << 8;

/* EQ Control 2 (0xb1) */
pub const RT5670_EQ_HPF1_M_MASK: u32 = 0x1 << 8;
pub const RT5670_EQ_HPF1_M_SFT: u32 = 8;
pub const RT5670_EQ_HPF1_M_HI: u32 = 0x0 << 8;
pub const RT5670_EQ_HPF1_M_1ST: u32 = 0x1 << 8;
pub const RT5670_EQ_LPF1_M_MASK: u32 = 0x1 << 7;
pub const RT5670_EQ_LPF1_M_SFT: u32 = 7;
pub const RT5670_EQ_LPF1_M_LO: u32 = 0x0 << 7;
pub const RT5670_EQ_LPF1_M_1ST: u32 = 0x1 << 7;
pub const RT5670_EQ_HPF2_MASK: u32 = 0x1 << 6;
pub const RT5670_EQ_HPF2_SFT: u32 = 6;
pub const RT5670_EQ_HPF2_DIS: u32 = 0x0 << 6;
pub const RT5670_EQ_HPF2_EN: u32 = 0x1 << 6;
pub const RT5670_EQ_HPF1_MASK: u32 = 0x1 << 5;
pub const RT5670_EQ_HPF1_SFT: u32 = 5;
pub const RT5670_EQ_HPF1_DIS: u32 = 0x0 << 5;
pub const RT5670_EQ_HPF1_EN: u32 = 0x1 << 5;
pub const RT5670_EQ_BPF4_MASK: u32 = 0x1 << 4;
pub const RT5670_EQ_BPF4_SFT: u32 = 4;
pub const RT5670_EQ_BPF4_DIS: u32 = 0x0 << 4;
pub const RT5670_EQ_BPF4_EN: u32 = 0x1 << 4;
pub const RT5670_EQ_BPF3_MASK: u32 = 0x1 << 3;
pub const RT5670_EQ_BPF3_SFT: u32 = 3;
pub const RT5670_EQ_BPF3_DIS: u32 = 0x0 << 3;
pub const RT5670_EQ_BPF3_EN: u32 = 0x1 << 3;
pub const RT5670_EQ_BPF2_MASK: u32 = 0x1 << 2;
pub const RT5670_EQ_BPF2_SFT: u32 = 2;
pub const RT5670_EQ_BPF2_DIS: u32 = 0x0 << 2;
pub const RT5670_EQ_BPF2_EN: u32 = 0x1 << 2;
pub const RT5670_EQ_BPF1_MASK: u32 = 0x1 << 1;
pub const RT5670_EQ_BPF1_SFT: u32 = 1;
pub const RT5670_EQ_BPF1_DIS: u32 = 0x0 << 1;
pub const RT5670_EQ_BPF1_EN: u32 = 0x1 << 1;
pub const RT5670_EQ_LPF_MASK: u32 = 0x1;
pub const RT5670_EQ_LPF_SFT: u32 = 0;
pub const RT5670_EQ_LPF_DIS: u32 = 0x0;
pub const RT5670_EQ_LPF_EN: u32 = 0x1;
pub const RT5670_EQ_CTRL_MASK: u32 = 0x7f;

/* Memory Test (0xb2) */
pub const RT5670_MT_MASK: u32 = 0x1 << 15;
pub const RT5670_MT_SFT: u32 = 15;
pub const RT5670_MT_DIS: u32 = 0x0 << 15;
pub const RT5670_MT_EN: u32 = 0x1 << 15;

/* DRC/AGC Control 1 (0xb4) */
pub const RT5670_DRC_AGC_P_MASK: u32 = 0x1 << 15;
pub const RT5670_DRC_AGC_P_SFT: u32 = 15;
pub const RT5670_DRC_AGC_P_DAC: u32 = 0x0 << 15;
pub const RT5670_DRC_AGC_P_ADC: u32 = 0x1 << 15;
pub const RT5670_DRC_AGC_MASK: u32 = 0x1 << 14;
pub const RT5670_DRC_AGC_SFT: u32 = 14;
pub const RT5670_DRC_AGC_DIS: u32 = 0x0 << 14;
pub const RT5670_DRC_AGC_EN: u32 = 0x1 << 14;
pub const RT5670_DRC_AGC_UPD: u32 = 0x1 << 13;
pub const RT5670_DRC_AGC_UPD_BIT: u32 = 13;
pub const RT5670_DRC_AGC_AR_MASK: u32 = 0x1f << 8;
pub const RT5670_DRC_AGC_AR_SFT: u32 = 8;
pub const RT5670_DRC_AGC_R_MASK: u32 = 0x7 << 5;
pub const RT5670_DRC_AGC_R_SFT: u32 = 5;
pub const RT5670_DRC_AGC_R_48K: u32 = 0x1 << 5;
pub const RT5670_DRC_AGC_R_96K: u32 = 0x2 << 5;
pub const RT5670_DRC_AGC_R_192K: u32 = 0x3 << 5;
pub const RT5670_DRC_AGC_R_441K: u32 = 0x5 << 5;
pub const RT5670_DRC_AGC_R_882K: u32 = 0x6 << 5;
pub const RT5670_DRC_AGC_R_1764K: u32 = 0x7 << 5;
pub const RT5670_DRC_AGC_RC_MASK: u32 = 0x1f;
pub const RT5670_DRC_AGC_RC_SFT: u32 = 0;

/* DRC/AGC Control 2 (0xb5) */
pub const RT5670_DRC_AGC_POB_MASK: u32 = 0x3f << 8;
pub const RT5670_DRC_AGC_POB_SFT: u32 = 8;
pub const RT5670_DRC_AGC_CP_MASK: u32 = 0x1 << 7;
pub const RT5670_DRC_AGC_CP_SFT: u32 = 7;
pub const RT5670_DRC_AGC_CP_DIS: u32 = 0x0 << 7;
pub const RT5670_DRC_AGC_CP_EN: u32 = 0x1 << 7;
pub const RT5670_DRC_AGC_CPR_MASK: u32 = 0x3 << 5;
pub const RT5670_DRC_AGC_CPR_SFT: u32 = 5;
pub const RT5670_DRC_AGC_CPR_1_1: u32 = 0x0 << 5;
pub const RT5670_DRC_AGC_CPR_1_2: u32 = 0x1 << 5;
pub const RT5670_DRC_AGC_CPR_1_3: u32 = 0x2 << 5;
pub const RT5670_DRC_AGC_CPR_1_4: u32 = 0x3 << 5;
pub const RT5670_DRC_AGC_PRB_MASK: u32 = 0x1f;
pub const RT5670_DRC_AGC_PRB_SFT: u32 = 0;

/* DRC/AGC Control 3 (0xb6) */
pub const RT5670_DRC_AGC_NGB_MASK: u32 = 0xf << 12;
pub const RT5670_DRC_AGC_NGB_SFT: u32 = 12;
pub const RT5670_DRC_AGC_TAR_MASK: u32 = 0x1f << 7;
pub const RT5670_DRC_AGC_TAR_SFT: u32 = 7;
pub const RT5670_DRC_AGC_NG_MASK: u32 = 0x1 << 6;
pub const RT5670_DRC_AGC_NG_SFT: u32 = 6;
pub const RT5670_DRC_AGC_NG_DIS: u32 = 0x0 << 6;
pub const RT5670_DRC_AGC_NG_EN: u32 = 0x1 << 6;
pub const RT5670_DRC_AGC_NGH_MASK: u32 = 0x1 << 5;
pub const RT5670_DRC_AGC_NGH_SFT: u32 = 5;
pub const RT5670_DRC_AGC_NGH_DIS: u32 = 0x0 << 5;
pub const RT5670_DRC_AGC_NGH_EN: u32 = 0x1 << 5;
pub const RT5670_DRC_AGC_NGT_MASK: u32 = 0x1f;
pub const RT5670_DRC_AGC_NGT_SFT: u32 = 0;

/* Jack Detect Control (0xbb) */
pub const RT5670_JD_MASK: u32 = 0x7 << 13;
pub const RT5670_JD_SFT: u32 = 13;
pub const RT5670_JD_DIS: u32 = 0x0 << 13;
pub const RT5670_JD_GPIO1: u32 = 0x1 << 13;
pub const RT5670_JD_JD1_IN4P: u32 = 0x2 << 13;
pub const RT5670_JD_JD2_IN4N: u32 = 0x3 << 13;
pub const RT5670_JD_GPIO2: u32 = 0x4 << 13;
pub const RT5670_JD_GPIO3: u32 = 0x5 << 13;
pub const RT5670_JD_GPIO4: u32 = 0x6 << 13;
pub const RT5670_JD_HP_MASK: u32 = 0x1 << 11;
pub const RT5670_JD_HP_SFT: u32 = 11;
pub const RT5670_JD_HP_DIS: u32 = 0x0 << 11;
pub const RT5670_JD_HP_EN: u32 = 0x1 << 11;
pub const RT5670_JD_HP_TRG_MASK: u32 = 0x1 << 10;
pub const RT5670_JD_HP_TRG_SFT: u32 = 10;
pub const RT5670_JD_HP_TRG_LO: u32 = 0x0 << 10;
pub const RT5670_JD_HP_TRG_HI: u32 = 0x1 << 10;
pub const RT5670_JD_SPL_MASK: u32 = 0x1 << 9;
pub const RT5670_JD_SPL_SFT: u32 = 9;
pub const RT5670_JD_SPL_DIS: u32 = 0x0 << 9;
pub const RT5670_JD_SPL_EN: u32 = 0x1 << 9;
pub const RT5670_JD_SPL_TRG_MASK: u32 = 0x1 << 8;
pub const RT5670_JD_SPL_TRG_SFT: u32 = 8;
pub const RT5670_JD_SPL_TRG_LO: u32 = 0x0 << 8;
pub const RT5670_JD_SPL_TRG_HI: u32 = 0x1 << 8;
pub const RT5670_JD_SPR_MASK: u32 = 0x1 << 7;
pub const RT5670_JD_SPR_SFT: u32 = 7;
pub const RT5670_JD_SPR_DIS: u32 = 0x0 << 7;
pub const RT5670_JD_SPR_EN: u32 = 0x1 << 7;
pub const RT5670_JD_SPR_TRG_MASK: u32 = 0x1 << 6;
pub const RT5670_JD_SPR_TRG_SFT: u32 = 6;
pub const RT5670_JD_SPR_TRG_LO: u32 = 0x0 << 6;
pub const RT5670_JD_SPR_TRG_HI: u32 = 0x1 << 6;
pub const RT5670_JD_MO_MASK: u32 = 0x1 << 5;
pub const RT5670_JD_MO_SFT: u32 = 5;
pub const RT5670_JD_MO_DIS: u32 = 0x0 << 5;
pub const RT5670_JD_MO_EN: u32 = 0x1 << 5;
pub const RT5670_JD_MO_TRG_MASK: u32 = 0x1 << 4;
pub const RT5670_JD_MO_TRG_SFT: u32 = 4;
pub const RT5670_JD_MO_TRG_LO: u32 = 0x0 << 4;
pub const RT5670_JD_MO_TRG_HI: u32 = 0x1 << 4;
pub const RT5670_JD_LO_MASK: u32 = 0x1 << 3;
pub const RT5670_JD_LO_SFT: u32 = 3;
pub const RT5670_JD_LO_DIS: u32 = 0x0 << 3;
pub const RT5670_JD_LO_EN: u32 = 0x1 << 3;
pub const RT5670_JD_LO_TRG_MASK: u32 = 0x1 << 2;
pub const RT5670_JD_LO_TRG_SFT: u32 = 2;
pub const RT5670_JD_LO_TRG_LO: u32 = 0x0 << 2;
pub const RT5670_JD_LO_TRG_HI: u32 = 0x1 << 2;
pub const RT5670_JD1_IN4P_MASK: u32 = 0x1 << 1;
pub const RT5670_JD1_IN4P_SFT: u32 = 1;
pub const RT5670_JD1_IN4P_DIS: u32 = 0x0 << 1;
pub const RT5670_JD1_IN4P_EN: u32 = 0x1 << 1;
pub const RT5670_JD2_IN4N_MASK: u32 = 0x1;
pub const RT5670_JD2_IN4N_SFT: u32 = 0;
pub const RT5670_JD2_IN4N_DIS: u32 = 0x0;
pub const RT5670_JD2_IN4N_EN: u32 = 0x1;

/* IRQ Control 1 (0xbd) */
pub const RT5670_IRQ_JD_MASK: u32 = 0x1 << 15;
pub const RT5670_IRQ_JD_SFT: u32 = 15;
pub const RT5670_IRQ_JD_BP: u32 = 0x0 << 15;
pub const RT5670_IRQ_JD_NOR: u32 = 0x1 << 15;
pub const RT5670_IRQ_OT_MASK: u32 = 0x1 << 14;
pub const RT5670_IRQ_OT_SFT: u32 = 14;
pub const RT5670_IRQ_OT_BP: u32 = 0x0 << 14;
pub const RT5670_IRQ_OT_NOR: u32 = 0x1 << 14;
pub const RT5670_JD_STKY_MASK: u32 = 0x1 << 13;
pub const RT5670_JD_STKY_SFT: u32 = 13;
pub const RT5670_JD_STKY_DIS: u32 = 0x0 << 13;
pub const RT5670_JD_STKY_EN: u32 = 0x1 << 13;
pub const RT5670_OT_STKY_MASK: u32 = 0x1 << 12;
pub const RT5670_OT_STKY_SFT: u32 = 12;
pub const RT5670_OT_STKY_DIS: u32 = 0x0 << 12;
pub const RT5670_OT_STKY_EN: u32 = 0x1 << 12;
pub const RT5670_JD_P_MASK: u32 = 0x1 << 11;
pub const RT5670_JD_P_SFT: u32 = 11;
pub const RT5670_JD_P_NOR: u32 = 0x0 << 11;
pub const RT5670_JD_P_INV: u32 = 0x1 << 11;
pub const RT5670_OT_P_MASK: u32 = 0x1 << 10;
pub const RT5670_OT_P_SFT: u32 = 10;
pub const RT5670_OT_P_NOR: u32 = 0x0 << 10;
pub const RT5670_OT_P_INV: u32 = 0x1 << 10;
pub const RT5670_JD1_1_EN_MASK: u32 = 0x1 << 9;
pub const RT5670_JD1_1_EN_SFT: u32 = 9;
pub const RT5670_JD1_1_DIS: u32 = 0x0 << 9;
pub const RT5670_JD1_1_EN: u32 = 0x1 << 9;

/* IRQ Control 2 (0xbe) */
pub const RT5670_IRQ_MB1_OC_MASK: u32 = 0x1 << 15;
pub const RT5670_IRQ_MB1_OC_SFT: u32 = 15;
pub const RT5670_IRQ_MB1_OC_BP: u32 = 0x0 << 15;
pub const RT5670_IRQ_MB1_OC_NOR: u32 = 0x1 << 15;
pub const RT5670_IRQ_MB2_OC_MASK: u32 = 0x1 << 14;
pub const RT5670_IRQ_MB2_OC_SFT: u32 = 14;
pub const RT5670_IRQ_MB2_OC_BP: u32 = 0x0 << 14;
pub const RT5670_IRQ_MB2_OC_NOR: u32 = 0x1 << 14;
pub const RT5670_MB1_OC_STKY_MASK: u32 = 0x1 << 11;
pub const RT5670_MB1_OC_STKY_SFT: u32 = 11;
pub const RT5670_MB1_OC_STKY_DIS: u32 = 0x0 << 11;
pub const RT5670_MB1_OC_STKY_EN: u32 = 0x1 << 11;
pub const RT5670_MB2_OC_STKY_MASK: u32 = 0x1 << 10;
pub const RT5670_MB2_OC_STKY_SFT: u32 = 10;
pub const RT5670_MB2_OC_STKY_DIS: u32 = 0x0 << 10;
pub const RT5670_MB2_OC_STKY_EN: u32 = 0x1 << 10;
pub const RT5670_MB1_OC_P_MASK: u32 = 0x1 << 7;
pub const RT5670_MB1_OC_P_SFT: u32 = 7;
pub const RT5670_MB1_OC_P_NOR: u32 = 0x0 << 7;
pub const RT5670_MB1_OC_P_INV: u32 = 0x1 << 7;
pub const RT5670_MB2_OC_P_MASK: u32 = 0x1 << 6;
pub const RT5670_MB2_OC_P_SFT: u32 = 6;
pub const RT5670_MB2_OC_P_NOR: u32 = 0x0 << 6;
pub const RT5670_MB2_OC_P_INV: u32 = 0x1 << 6;
pub const RT5670_MB1_OC_CLR: u32 = 0x1 << 3;
pub const RT5670_MB1_OC_CLR_SFT: u32 = 3;
pub const RT5670_MB2_OC_CLR: u32 = 0x1 << 2;
pub const RT5670_MB2_OC_CLR_SFT: u32 = 2;

/* GPIO Control 1 (0xc0) */
pub const RT5670_GP1_PIN_MASK: u32 = 0x1 << 15;
pub const RT5670_GP1_PIN_SFT: u32 = 15;
pub const RT5670_GP1_PIN_GPIO1: u32 = 0x0 << 15;
pub const RT5670_GP1_PIN_IRQ: u32 = 0x1 << 15;
pub const RT5670_GP2_PIN_MASK: u32 = 0x1 << 14;
pub const RT5670_GP2_PIN_SFT: u32 = 14;
pub const RT5670_GP2_PIN_GPIO2: u32 = 0x0 << 14;
pub const RT5670_GP2_PIN_DMIC1_SCL: u32 = 0x1 << 14;
pub const RT5670_GP3_PIN_MASK: u32 = 0x3 << 12;
pub const RT5670_GP3_PIN_SFT: u32 = 12;
pub const RT5670_GP3_PIN_GPIO3: u32 = 0x0 << 12;
pub const RT5670_GP3_PIN_DMIC1_SDA: u32 = 0x1 << 12;
pub const RT5670_GP3_PIN_IRQ: u32 = 0x2 << 12;
pub const RT5670_GP4_PIN_MASK: u32 = 0x1 << 11;
pub const RT5670_GP4_PIN_SFT: u32 = 11;
pub const RT5670_GP4_PIN_GPIO4: u32 = 0x0 << 11;
pub const RT5670_GP4_PIN_DMIC2_SDA: u32 = 0x1 << 11;
pub const RT5670_DP_SIG_MASK: u32 = 0x1 << 10;
pub const RT5670_DP_SIG_SFT: u32 = 10;
pub const RT5670_DP_SIG_TEST: u32 = 0x0 << 10;
pub const RT5670_DP_SIG_AP: u32 = 0x1 << 10;
pub const RT5670_GPIO_M_MASK: u32 = 0x1 << 9;
pub const RT5670_GPIO_M_SFT: u32 = 9;
pub const RT5670_GPIO_M_FLT: u32 = 0x0 << 9;
pub const RT5670_GPIO_M_PH: u32 = 0x1 << 9;
pub const RT5670_I2S2_PIN_MASK: u32 = 0x1 << 8;
pub const RT5670_I2S2_PIN_SFT: u32 = 8;
pub const RT5670_I2S2_PIN_I2S: u32 = 0x0 << 8;
pub const RT5670_I2S2_PIN_GPIO: u32 = 0x1 << 8;
pub const RT5670_GP5_PIN_MASK: u32 = 0x1 << 7;
pub const RT5670_GP5_PIN_SFT: u32 = 7;
pub const RT5670_GP5_PIN_GPIO5: u32 = 0x0 << 7;
pub const RT5670_GP5_PIN_DMIC3_SDA: u32 = 0x1 << 7;
pub const RT5670_GP6_PIN_MASK: u32 = 0x1 << 6;
pub const RT5670_GP6_PIN_SFT: u32 = 6;
pub const RT5670_GP6_PIN_GPIO6: u32 = 0x0 << 6;
pub const RT5670_GP6_PIN_DMIC1_SDA: u32 = 0x1 << 6;
pub const RT5670_GP7_PIN_MASK: u32 = 0x3 << 4;
pub const RT5670_GP7_PIN_SFT: u32 = 4;
pub const RT5670_GP7_PIN_GPIO7: u32 = 0x0 << 4;
pub const RT5670_GP7_PIN_DMIC1_SDA: u32 = 0x1 << 4;
pub const RT5670_GP7_PIN_PDM_SCL2: u32 = 0x2 << 4;
pub const RT5670_GP8_PIN_MASK: u32 = 0x1 << 3;
pub const RT5670_GP8_PIN_SFT: u32 = 3;
pub const RT5670_GP8_PIN_GPIO8: u32 = 0x0 << 3;
pub const RT5670_GP8_PIN_DMIC2_SDA: u32 = 0x1 << 3;
pub const RT5670_GP9_PIN_MASK: u32 = 0x1 << 2;
pub const RT5670_GP9_PIN_SFT: u32 = 2;
pub const RT5670_GP9_PIN_GPIO9: u32 = 0x0 << 2;
pub const RT5670_GP9_PIN_DMIC3_SDA: u32 = 0x1 << 2;
pub const RT5670_GP10_PIN_MASK: u32 = 0x3;
pub const RT5670_GP10_PIN_SFT: u32 = 0;
pub const RT5670_GP10_PIN_GPIO9: u32 = 0x0;
pub const RT5670_GP10_PIN_DMIC3_SDA: u32 = 0x1;
pub const RT5670_GP10_PIN_PDM_ADT2: u32 = 0x2;

/* GPIO Control 2 (0xc1) */
pub const RT5670_GP4_PF_MASK: u32 = 0x1 << 11;
pub const RT5670_GP4_PF_SFT: u32 = 11;
pub const RT5670_GP4_PF_IN: u32 = 0x0 << 11;
pub const RT5670_GP4_PF_OUT: u32 = 0x1 << 11;
pub const RT5670_GP4_OUT_MASK: u32 = 0x1 << 10;
pub const RT5670_GP4_OUT_SFT: u32 = 10;
pub const RT5670_GP4_OUT_LO: u32 = 0x0 << 10;
pub const RT5670_GP4_OUT_HI: u32 = 0x1 << 10;
pub const RT5670_GP4_P_MASK: u32 = 0x1 << 9;
pub const RT5670_GP4_P_SFT: u32 = 9;
pub const RT5670_GP4_P_NOR: u32 = 0x0 << 9;
pub const RT5670_GP4_P_INV: u32 = 0x1 << 9;
pub const RT5670_GP3_PF_MASK: u32 = 0x1 << 8;
pub const RT5670_GP3_PF_SFT: u32 = 8;
pub const RT5670_GP3_PF_IN: u32 = 0x0 << 8;
pub const RT5670_GP3_PF_OUT: u32 = 0x1 << 8;
pub const RT5670_GP3_OUT_MASK: u32 = 0x1 << 7;
pub const RT5670_GP3_OUT_SFT: u32 = 7;
pub const RT5670_GP3_OUT_LO: u32 = 0x0 << 7;
pub const RT5670_GP3_OUT_HI: u32 = 0x1 << 7;
pub const RT5670_GP3_P_MASK: u32 = 0x1 << 6;
pub const RT5670_GP3_P_SFT: u32 = 6;
pub const RT5670_GP3_P_NOR: u32 = 0x0 << 6;
pub const RT5670_GP3_P_INV: u32 = 0x1 << 6;
pub const RT5670_GP2_PF_MASK: u32 = 0x1 << 5;
pub const RT5670_GP2_PF_SFT: u32 = 5;
pub const RT5670_GP2_PF_IN: u32 = 0x0 << 5;
pub const RT5670_GP2_PF_OUT: u32 = 0x1 << 5;
pub const RT5670_GP2_OUT_MASK: u32 = 0x1 << 4;
pub const RT5670_GP2_OUT_SFT: u32 = 4;
pub const RT5670_GP2_OUT_LO: u32 = 0x0 << 4;
pub const RT5670_GP2_OUT_HI: u32 = 0x1 << 4;
pub const RT5670_GP2_P_MASK: u32 = 0x1 << 3;
pub const RT5670_GP2_P_SFT: u32 = 3;
pub const RT5670_GP2_P_NOR: u32 = 0x0 << 3;
pub const RT5670_GP2_P_INV: u32 = 0x1 << 3;
pub const RT5670_GP1_PF_MASK: u32 = 0x1 << 2;
pub const RT5670_GP1_PF_SFT: u32 = 2;
pub const RT5670_GP1_PF_IN: u32 = 0x0 << 2;
pub const RT5670_GP1_PF_OUT: u32 = 0x1 << 2;
pub const RT5670_GP1_OUT_MASK: u32 = 0x1 << 1;
pub const RT5670_GP1_OUT_SFT: u32 = 1;
pub const RT5670_GP1_OUT_LO: u32 = 0x0 << 1;
pub const RT5670_GP1_OUT_HI: u32 = 0x1 << 1;
pub const RT5670_GP1_P_MASK: u32 = 0x1;
pub const RT5670_GP1_P_SFT: u32 = 0;
pub const RT5670_GP1_P_NOR: u32 = 0x0;
pub const RT5670_GP1_P_INV: u32 = 0x1;

/* Scramble Function (0xcd) */
pub const RT5670_SCB_KEY_MASK: u32 = 0xff;
pub const RT5670_SCB_KEY_SFT: u32 = 0;

/* Scramble Control (0xce) */
pub const RT5670_SCB_SWAP_MASK: u32 = 0x1 << 15;
pub const RT5670_SCB_SWAP_SFT: u32 = 15;
pub const RT5670_SCB_SWAP_DIS: u32 = 0x0 << 15;
pub const RT5670_SCB_SWAP_EN: u32 = 0x1 << 15;
pub const RT5670_SCB_MASK: u32 = 0x1 << 14;
pub const RT5670_SCB_SFT: u32 = 14;
pub const RT5670_SCB_DIS: u32 = 0x0 << 14;
pub const RT5670_SCB_EN: u32 = 0x1 << 14;

/* Baseback Control (0xcf) */
pub const RT5670_BB_MASK: u32 = 0x1 << 15;
pub const RT5670_BB_SFT: u32 = 15;
pub const RT5670_BB_DIS: u32 = 0x0 << 15;
pub const RT5670_BB_EN: u32 = 0x1 << 15;
pub const RT5670_BB_CT_MASK: u32 = 0x7 << 12;
pub const RT5670_BB_CT_SFT: u32 = 12;
pub const RT5670_BB_CT_A: u32 = 0x0 << 12;
pub const RT5670_BB_CT_B: u32 = 0x1 << 12;
pub const RT5670_BB_CT_C: u32 = 0x2 << 12;
pub const RT5670_BB_CT_D: u32 = 0x3 << 12;
pub const RT5670_M_BB_L_MASK: u32 = 0x1 << 9;
pub const RT5670_M_BB_L_SFT: u32 = 9;
pub const RT5670_M_BB_R_MASK: u32 = 0x1 << 8;
pub const RT5670_M_BB_R_SFT: u32 = 8;
pub const RT5670_M_BB_HPF_L_MASK: u32 = 0x1 << 7;
pub const RT5670_M_BB_HPF_L_SFT: u32 = 7;
pub const RT5670_M_BB_HPF_R_MASK: u32 = 0x1 << 6;
pub const RT5670_M_BB_HPF_R_SFT: u32 = 6;
pub const RT5670_G_BB_BST_MASK: u32 = 0x3f;
pub const RT5670_G_BB_BST_SFT: u32 = 0;

/* MP3 Plus Control 1 (0xd0) */
pub const RT5670_M_MP3_L_MASK: u32 = 0x1 << 15;
pub const RT5670_M_MP3_L_SFT: u32 = 15;
pub const RT5670_M_MP3_R_MASK: u32 = 0x1 << 14;
pub const RT5670_M_MP3_R_SFT: u32 = 14;
pub const RT5670_M_MP3_MASK: u32 = 0x1 << 13;
pub const RT5670_M_MP3_SFT: u32 = 13;
pub const RT5670_M_MP3_DIS: u32 = 0x0 << 13;
pub const RT5670_M_MP3_EN: u32 = 0x1 << 13;
pub const RT5670_EG_MP3_MASK: u32 = 0x1f << 8;
pub const RT5670_EG_MP3_SFT: u32 = 8;
pub const RT5670_MP3_HLP_MASK: u32 = 0x1 << 7;
pub const RT5670_MP3_HLP_SFT: u32 = 7;
pub const RT5670_MP3_HLP_DIS: u32 = 0x0 << 7;
pub const RT5670_MP3_HLP_EN: u32 = 0x1 << 7;
pub const RT5670_M_MP3_ORG_L_MASK: u32 = 0x1 << 6;
pub const RT5670_M_MP3_ORG_L_SFT: u32 = 6;
pub const RT5670_M_MP3_ORG_R_MASK: u32 = 0x1 << 5;
pub const RT5670_M_MP3_ORG_R_SFT: u32 = 5;

/* MP3 Plus Control 2 (0xd1) */
pub const RT5670_MP3_WT_MASK: u32 = 0x1 << 13;
pub const RT5670_MP3_WT_SFT: u32 = 13;
pub const RT5670_MP3_WT_1_4: u32 = 0x0 << 13;
pub const RT5670_MP3_WT_1_2: u32 = 0x1 << 13;
pub const RT5670_OG_MP3_MASK: u32 = 0x1f << 8;
pub const RT5670_OG_MP3_SFT: u32 = 8;
pub const RT5670_HG_MP3_MASK: u32 = 0x3f;
pub const RT5670_HG_MP3_SFT: u32 = 0;

/* 3D HP Control 1 (0xd2) */
pub const RT5670_3D_CF_MASK: u32 = 0x1 << 15;
pub const RT5670_3D_CF_SFT: u32 = 15;
pub const RT5670_3D_CF_DIS: u32 = 0x0 << 15;
pub const RT5670_3D_CF_EN: u32 = 0x1 << 15;
pub const RT5670_3D_HP_MASK: u32 = 0x1 << 14;
pub const RT5670_3D_HP_SFT: u32 = 14;
pub const RT5670_3D_HP_DIS: u32 = 0x0 << 14;
pub const RT5670_3D_HP_EN: u32 = 0x1 << 14;
pub const RT5670_3D_BT_MASK: u32 = 0x1 << 13;
pub const RT5670_3D_BT_SFT: u32 = 13;
pub const RT5670_3D_BT_DIS: u32 = 0x0 << 13;
pub const RT5670_3D_BT_EN: u32 = 0x1 << 13;
pub const RT5670_3D_1F_MIX_MASK: u32 = 0x3 << 11;
pub const RT5670_3D_1F_MIX_SFT: u32 = 11;
pub const RT5670_3D_HP_M_MASK: u32 = 0x1 << 10;
pub const RT5670_3D_HP_M_SFT: u32 = 10;
pub const RT5670_3D_HP_M_SUR: u32 = 0x0 << 10;
pub const RT5670_3D_HP_M_FRO: u32 = 0x1 << 10;
pub const RT5670_M_3D_HRTF_MASK: u32 = 0x1 << 9;
pub const RT5670_M_3D_HRTF_SFT: u32 = 9;
pub const RT5670_M_3D_D2H_MASK: u32 = 0x1 << 8;
pub const RT5670_M_3D_D2H_SFT: u32 = 8;
pub const RT5670_M_3D_D2R_MASK: u32 = 0x1 << 7;
pub const RT5670_M_3D_D2R_SFT: u32 = 7;
pub const RT5670_M_3D_REVB_MASK: u32 = 0x1 << 6;
pub const RT5670_M_3D_REVB_SFT: u32 = 6;

/* Adjustable high pass filter control 1 (0xd3) */
pub const RT5670_2ND_HPF_MASK: u32 = 0x1 << 15;
pub const RT5670_2ND_HPF_SFT: u32 = 15;
pub const RT5670_2ND_HPF_DIS: u32 = 0x0 << 15;
pub const RT5670_2ND_HPF_EN: u32 = 0x1 << 15;
pub const RT5670_HPF_CF_L_MASK: u32 = 0x7 << 12;
pub const RT5670_HPF_CF_L_SFT: u32 = 12;
pub const RT5670_1ST_HPF_MASK: u32 = 0x1 << 11;
pub const RT5670_1ST_HPF_SFT: u32 = 11;
pub const RT5670_1ST_HPF_DIS: u32 = 0x0 << 11;
pub const RT5670_1ST_HPF_EN: u32 = 0x1 << 11;
pub const RT5670_HPF_CF_R_MASK: u32 = 0x7 << 8;
pub const RT5670_HPF_CF_R_SFT: u32 = 8;
pub const RT5670_ZD_T_MASK: u32 = 0x3 << 6;
pub const RT5670_ZD_T_SFT: u32 = 6;
pub const RT5670_ZD_F_MASK: u32 = 0x3 << 4;
pub const RT5670_ZD_F_SFT: u32 = 4;
pub const RT5670_ZD_F_IM: u32 = 0x0 << 4;
pub const RT5670_ZD_F_ZC_IM: u32 = 0x1 << 4;
pub const RT5670_ZD_F_ZC_IOD: u32 = 0x2 << 4;
pub const RT5670_ZD_F_UN: u32 = 0x3 << 4;

/* HP calibration control and Amp detection (0xd6) */
pub const RT5670_SI_DAC_MASK: u32 = 0x1 << 11;
pub const RT5670_SI_DAC_SFT: u32 = 11;
pub const RT5670_SI_DAC_AUTO: u32 = 0x0 << 11;
pub const RT5670_SI_DAC_TEST: u32 = 0x1 << 11;
pub const RT5670_DC_CAL_M_MASK: u32 = 0x1 << 10;
pub const RT5670_DC_CAL_M_SFT: u32 = 10;
pub const RT5670_DC_CAL_M_CAL: u32 = 0x0 << 10;
pub const RT5670_DC_CAL_M_NOR: u32 = 0x1 << 10;
pub const RT5670_DC_CAL_MASK: u32 = 0x1 << 9;
pub const RT5670_DC_CAL_SFT: u32 = 9;
pub const RT5670_DC_CAL_DIS: u32 = 0x0 << 9;
pub const RT5670_DC_CAL_EN: u32 = 0x1 << 9;
pub const RT5670_HPD_RCV_MASK: u32 = 0x7 << 6;
pub const RT5670_HPD_RCV_SFT: u32 = 6;
pub const RT5670_HPD_PS_MASK: u32 = 0x1 << 5;
pub const RT5670_HPD_PS_SFT: u32 = 5;
pub const RT5670_HPD_PS_DIS: u32 = 0x0 << 5;
pub const RT5670_HPD_PS_EN: u32 = 0x1 << 5;
pub const RT5670_CAL_M_MASK: u32 = 0x1 << 4;
pub const RT5670_CAL_M_SFT: u32 = 4;
pub const RT5670_CAL_M_DEP: u32 = 0x0 << 4;
pub const RT5670_CAL_M_CAL: u32 = 0x1 << 4;
pub const RT5670_CAL_MASK: u32 = 0x1 << 3;
pub const RT5670_CAL_SFT: u32 = 3;
pub const RT5670_CAL_DIS: u32 = 0x0 << 3;
pub const RT5670_CAL_EN: u32 = 0x1 << 3;
pub const RT5670_CAL_TEST_MASK: u32 = 0x1 << 2;
pub const RT5670_CAL_TEST_SFT: u32 = 2;
pub const RT5670_CAL_TEST_DIS: u32 = 0x0 << 2;
pub const RT5670_CAL_TEST_EN: u32 = 0x1 << 2;
pub const RT5670_CAL_P_MASK: u32 = 0x3;
pub const RT5670_CAL_P_SFT: u32 = 0;
pub const RT5670_CAL_P_NONE: u32 = 0x0;
pub const RT5670_CAL_P_CAL: u32 = 0x1;
pub const RT5670_CAL_P_DAC_CAL: u32 = 0x2;

/* Soft volume and zero cross control 1 (0xd9) */
pub const RT5670_SV_MASK: u32 = 0x1 << 15;
pub const RT5670_SV_SFT: u32 = 15;
pub const RT5670_SV_DIS: u32 = 0x0 << 15;
pub const RT5670_SV_EN: u32 = 0x1 << 15;
pub const RT5670_SPO_SV_MASK: u32 = 0x1 << 14;
pub const RT5670_SPO_SV_SFT: u32 = 14;
pub const RT5670_SPO_SV_DIS: u32 = 0x0 << 14;
pub const RT5670_SPO_SV_EN: u32 = 0x1 << 14;
pub const RT5670_OUT_SV_MASK: u32 = 0x1 << 13;
pub const RT5670_OUT_SV_SFT: u32 = 13;
pub const RT5670_OUT_SV_DIS: u32 = 0x0 << 13;
pub const RT5670_OUT_SV_EN: u32 = 0x1 << 13;
pub const RT5670_HP_SV_MASK: u32 = 0x1 << 12;
pub const RT5670_HP_SV_SFT: u32 = 12;
pub const RT5670_HP_SV_DIS: u32 = 0x0 << 12;
pub const RT5670_HP_SV_EN: u32 = 0x1 << 12;
pub const RT5670_ZCD_DIG_MASK: u32 = 0x1 << 11;
pub const RT5670_ZCD_DIG_SFT: u32 = 11;
pub const RT5670_ZCD_DIG_DIS: u32 = 0x0 << 11;
pub const RT5670_ZCD_DIG_EN: u32 = 0x1 << 11;
pub const RT5670_ZCD_MASK: u32 = 0x1 << 10;
pub const RT5670_ZCD_SFT: u32 = 10;
pub const RT5670_ZCD_PD: u32 = 0x0 << 10;
pub const RT5670_ZCD_PU: u32 = 0x1 << 10;
pub const RT5670_M_ZCD_MASK: u32 = 0x3f << 4;
pub const RT5670_M_ZCD_SFT: u32 = 4;
pub const RT5670_M_ZCD_RM_L: u32 = 0x1 << 9;
pub const RT5670_M_ZCD_RM_R: u32 = 0x1 << 8;
pub const RT5670_M_ZCD_SM_L: u32 = 0x1 << 7;
pub const RT5670_M_ZCD_SM_R: u32 = 0x1 << 6;
pub const RT5670_M_ZCD_OM_L: u32 = 0x1 << 5;
pub const RT5670_M_ZCD_OM_R: u32 = 0x1 << 4;
pub const RT5670_SV_DLY_MASK: u32 = 0xf;
pub const RT5670_SV_DLY_SFT: u32 = 0;

/* Soft volume and zero cross control 2 (0xda) */
pub const RT5670_ZCD_HP_MASK: u32 = 0x1 << 15;
pub const RT5670_ZCD_HP_SFT: u32 = 15;
pub const RT5670_ZCD_HP_DIS: u32 = 0x0 << 15;
pub const RT5670_ZCD_HP_EN: u32 = 0x1 << 15;

/* General Control 3 (0xfc) */
pub const RT5670_TDM_DATA_MODE_SEL: u32 = 0x1 << 11;
pub const RT5670_TDM_DATA_MODE_NOR: u32 = 0x0 << 11;
pub const RT5670_TDM_DATA_MODE_50FS: u32 = 0x1 << 11;

/* Codec Private Register definition */
/* 3D Speaker Control (0x63) */
pub const RT5670_3D_SPK_MASK: u32 = 0x1 << 15;
pub const RT5670_3D_SPK_SFT: u32 = 15;
pub const RT5670_3D_SPK_DIS: u32 = 0x0 << 15;
pub const RT5670_3D_SPK_EN: u32 = 0x1 << 15;
pub const RT5670_3D_SPK_M_MASK: u32 = 0x3 << 13;
pub const RT5670_3D_SPK_M_SFT: u32 = 13;
pub const RT5670_3D_SPK_CG_MASK: u32 = 0x1f << 8;
pub const RT5670_3D_SPK_CG_SFT: u32 = 8;
pub const RT5670_3D_SPK_SG_MASK: u32 = 0x1f;
pub const RT5670_3D_SPK_SG_SFT: u32 = 0;

/* Wind Noise Detection Control 1 (0x6c) */
pub const RT5670_WND_MASK: u32 = 0x1 << 15;
pub const RT5670_WND_SFT: u32 = 15;
pub const RT5670_WND_DIS: u32 = 0x0 << 15;
pub const RT5670_WND_EN: u32 = 0x1 << 15;

/* Wind Noise Detection Control 2 (0x6d) */
pub const RT5670_WND_FC_NW_MASK: u32 = 0x3f << 10;
pub const RT5670_WND_FC_NW_SFT: u32 = 10;
pub const RT5670_WND_FC_WK_MASK: u32 = 0x3f << 4;
pub const RT5670_WND_FC_WK_SFT: u32 = 4;

/* Wind Noise Detection Control 3 (0x6e) */
pub const RT5670_HPF_FC_MASK: u32 = 0x3f << 6;
pub const RT5670_HPF_FC_SFT: u32 = 6;
pub const RT5670_WND_FC_ST_MASK: u32 = 0x3f;
pub const RT5670_WND_FC_ST_SFT: u32 = 0;

/* Wind Noise Detection Control 4 (0x6f) */
pub const RT5670_WND_TH_LO_MASK: u32 = 0x3ff;
pub const RT5670_WND_TH_LO_SFT: u32 = 0;

/* Wind Noise Detection Control 5 (0x70) */
pub const RT5670_WND_TH_HI_MASK: u32 = 0x3ff;
pub const RT5670_WND_TH_HI_SFT: u32 = 0;

/* Wind Noise Detection Control 8 (0x73) */
pub const RT5670_WND_WIND_MASK: u32 = 0x1 << 13; /* Read-Only */
pub const RT5670_WND_WIND_SFT: u32 = 13;
pub const RT5670_WND_STRONG_MASK: u32 = 0x1 << 12; /* Read-Only */
pub const RT5670_WND_STRONG_SFT: u32 = 12;

pub const RT5670_NO_WIND: u32 = 0;
pub const RT5670_BREEZE: u32 = 1;
pub const RT5670_STORM: u32 = 2;


/* Dipole Speaker Interface (0x75) */
pub const RT5670_DP_ATT_MASK: u32 = 0x3 << 14;
pub const RT5670_DP_ATT_SFT: u32 = 14;
pub const RT5670_DP_SPK_MASK: u32 = 0x1 << 10;
pub const RT5670_DP_SPK_SFT: u32 = 10;
pub const RT5670_DP_SPK_DIS: u32 = 0x0 << 10;
pub const RT5670_DP_SPK_EN: u32 = 0x1 << 10;

/* EQ Pre Volume Control (0xb3) */
pub const RT5670_EQ_PRE_VOL_MASK: u32 = 0xffff;
pub const RT5670_EQ_PRE_VOL_SFT: u32 = 0;

/* EQ Post Volume Control (0xb4) */
pub const RT5670_EQ_PST_VOL_MASK: u32 = 0xffff;
pub const RT5670_EQ_PST_VOL_SFT: u32 = 0;

/* Jack Detect Control 3 (0xf8) */
pub const RT5670_CMP_MIC_IN_DET_MASK: u32 = 0x7 << 12;
pub const RT5670_JD_CBJ_EN: u32 = 0x1 << 7;
pub const RT5670_JD_CBJ_POL: u32 = 0x1 << 6;
pub const RT5670_JD_TRI_CBJ_SEL_MASK: u32 = 0x7 << 3;
pub const RT5670_JD_TRI_CBJ_SEL_SFT: u32 = 3;
pub const RT5670_JD_CBJ_GPIO_JD1: u32 = 0x0 << 3;
pub const RT5670_JD_CBJ_JD1_1: u32 = 0x1 << 3;
pub const RT5670_JD_CBJ_JD1_2: u32 = 0x2 << 3;
pub const RT5670_JD_CBJ_JD2: u32 = 0x3 << 3;
pub const RT5670_JD_CBJ_JD3: u32 = 0x4 << 3;
pub const RT5670_JD_CBJ_GPIO_JD2: u32 = 0x5 << 3;
pub const RT5670_JD_CBJ_MX0B_12: u32 = 0x6 << 3;
pub const RT5670_JD_TRI_HPO_SEL_MASK: u32 = 0x7 << 3;
pub const RT5670_JD_TRI_HPO_SEL_SFT: u32 = 0;
pub const RT5670_JD_HPO_GPIO_JD1: u32 = 0x0;
pub const RT5670_JD_HPO_JD1_1: u32 = 0x1;
pub const RT5670_JD_HPO_JD1_2: u32 = 0x2;
pub const RT5670_JD_HPO_JD2: u32 = 0x3;
pub const RT5670_JD_HPO_JD3: u32 = 0x4;
pub const RT5670_JD_HPO_GPIO_JD2: u32 = 0x5;
pub const RT5670_JD_HPO_MX0B_12: u32 = 0x6;

/* Digital Misc Control (0xfa) */
pub const RT5670_RST_DSP: u32 = 0x1 << 13;
pub const RT5670_IF1_ADC1_IN1_SEL: u32 = 0x1 << 12;
pub const RT5670_IF1_ADC1_IN1_SFT: u32 = 12;
pub const RT5670_IF1_ADC1_IN2_SEL: u32 = 0x1 << 11;
pub const RT5670_IF1_ADC1_IN2_SFT: u32 = 11;
pub const RT5670_IF1_ADC2_IN1_SEL: u32 = 0x1 << 10;
pub const RT5670_IF1_ADC2_IN1_SFT: u32 = 10;
pub const RT5670_MCLK_DET: u32 = 0x1 << 3;

/* General Control2 (0xfb) */
pub const RT5670_RXDC_SRC_MASK: u32 = 0x1 << 7;
pub const RT5670_RXDC_SRC_STO: u32 = 0x0 << 7;
pub const RT5670_RXDC_SRC_MONO: u32 = 0x1 << 7;
pub const RT5670_RXDC_SRC_SFT: u32 = 7;
pub const RT5670_RXDP2_SEL_MASK: u32 = 0x1 << 3;
pub const RT5670_RXDP2_SEL_IF2: u32 = 0x0 << 3;
pub const RT5670_RXDP2_SEL_ADC: u32 = 0x1 << 3;
pub const RT5670_RXDP2_SEL_SFT: u32 = 3;

/* System Clock Source */

pub const RT5670_SCLK_S_MCLK: u32 = 0;
pub const RT5670_SCLK_S_PLL1: u32 = 1;
pub const RT5670_SCLK_S_RCCLK: u32 = 2;


/* PLL1 Source */

pub const RT5670_PLL1_S_MCLK: u32 = 0;
pub const RT5670_PLL1_S_BCLK1: u32 = 1;
pub const RT5670_PLL1_S_BCLK2: u32 = 2;
pub const RT5670_PLL1_S_BCLK3: u32 = 3;
pub const RT5670_PLL1_S_BCLK4: u32 = 4;



pub const RT5670_AIF1: u32 = 0;
pub const RT5670_AIF2: u32 = 1;
pub const RT5670_AIF3: u32 = 2;
pub const RT5670_AIF4: u32 = 3;
pub const RT5670_AIFS: u32 = 4;



pub const RT5670_DMIC1_DISABLED: u32 = 0;
pub const RT5670_DMIC_DATA_GPIO6: u32 = 1;
pub const RT5670_DMIC_DATA_IN2P: u32 = 2;
pub const RT5670_DMIC_DATA_GPIO7: u32 = 3;



pub const RT5670_DMIC2_DISABLED: u32 = 0;
pub const RT5670_DMIC_DATA_GPIO8: u32 = 1;
pub const RT5670_DMIC_DATA_IN3N: u32 = 2;



pub const RT5670_DMIC3_DISABLED: u32 = 0;
pub const RT5670_DMIC_DATA_GPIO9: u32 = 1;
pub const RT5670_DMIC_DATA_GPIO10: u32 = 2;
pub const RT5670_DMIC_DATA_GPIO5: u32 = 3;


/* filter mask */

pub const RT5670_DA_STEREO_FILTER: u32 = 0x1;
pub const RT5670_DA_MONO_L_FILTER: u32 = 0x1  <<  1;
pub const RT5670_DA_MONO_R_FILTER: u32 = 0x1  <<  2;
pub const RT5670_AD_STEREO_FILTER: u32 = 0x1  <<  3;
pub const RT5670_AD_MONO_L_FILTER: u32 = 0x1  <<  4;
pub const RT5670_AD_MONO_R_FILTER: u32 = 0x1  <<  5;
pub const RT5670_UP_RATE_FILTER: u32 = 0x1  <<  6;
pub const RT5670_DOWN_RATE_FILTER: u32 = 0x1  <<  7;


unsafe extern "C" {
    pub fn rt5670_sel_asrc_clk_src(component: *mut snd_soc_component, filter_mask: ::core::ffi::c_uint, clk_src: ::core::ffi::c_uint) -> ::core::ffi::c_int;

}

#[repr(C)]
pub struct rt5670_priv {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub jack: *mut snd_soc_jack,
    pub hp_gpio: snd_soc_jack_gpio,

    pub jd_mode: ::core::ffi::c_int,
    pub in2_diff: bool,
    pub gpio1_is_irq: bool,
    pub gpio1_is_ext_spk_en: bool,

    pub dmic_en: bool,
    pub dmic1_data_pin: ::core::ffi::c_uint,
    /* 0 = GPIO6; 1 = IN2P; 3 = GPIO7*/
    pub dmic2_data_pin: ::core::ffi::c_uint,
    /* 0 = GPIO8; 1 = IN3N; */
    pub dmic3_data_pin: ::core::ffi::c_uint,
    /* 0 = GPIO9; 1 = GPIO10; 2 = GPIO5*/

    pub sysclk: ::core::ffi::c_int,
    pub sysclk_src: ::core::ffi::c_int,
    pub lrck: [::core::ffi::c_int; RT5670_AIFS as usize],
    pub bclk: [::core::ffi::c_int; RT5670_AIFS as usize],
    pub master: [::core::ffi::c_int; RT5670_AIFS as usize],

    pub pll_src: ::core::ffi::c_int,
    pub pll_in: ::core::ffi::c_int,
    pub pll_out: ::core::ffi::c_int,

    pub dsp_sw: ::core::ffi::c_int, /* expected parameter setting */
    pub dsp_rate: ::core::ffi::c_int,
    pub jack_type: ::core::ffi::c_int,
    pub jack_type_saved: ::core::ffi::c_int,

    pub dac1_mixl_dac1_switch: bool,
    pub dac1_mixr_dac1_switch: bool,
    pub dac1_playback_switch_l: bool,
    pub dac1_playback_switch_r: bool,
}








unsafe extern "C" {
    pub fn rt5670_jack_suspend(component: *mut snd_soc_component);
    pub fn rt5670_jack_resume(component: *mut snd_soc_component);
    pub fn rt5670_set_jack_detect(component: *mut snd_soc_component, jack: *mut snd_soc_jack) -> ::core::ffi::c_int;
    pub fn rt5670_components() -> *const ::core::ffi::c_char;
}


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
