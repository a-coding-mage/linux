/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rt5616.h  --  RT5616 ALSA SoC audio driver
 *
 * Copyright 2011 Realtek Microelectronics
 * Author: Johnny Hsu <johnnyhsu@realtek.com>
 */


/* Info */
pub const RT5616_RESET: u32 = 0x00;
pub const RT5616_VERSION_ID: u32 = 0xfd;
pub const RT5616_VENDOR_ID: u32 = 0xfe;
pub const RT5616_DEVICE_ID: u32 = 0xff;
/*  I/O - Output */
pub const RT5616_HP_VOL: u32 = 0x02;
pub const RT5616_LOUT_CTRL1: u32 = 0x03;
pub const RT5616_LOUT_CTRL2: u32 = 0x05;
/* I/O - Input */
pub const RT5616_IN1_IN2: u32 = 0x0d;
pub const RT5616_INL1_INR1_VOL: u32 = 0x0f;
/* I/O - ADC/DAC/DMIC */
pub const RT5616_DAC1_DIG_VOL: u32 = 0x19;
pub const RT5616_ADC_DIG_VOL: u32 = 0x1c;
pub const RT5616_ADC_BST_VOL: u32 = 0x1e;
/* Mixer - D-D */
pub const RT5616_STO1_ADC_MIXER: u32 = 0x27;
pub const RT5616_AD_DA_MIXER: u32 = 0x29;
pub const RT5616_STO_DAC_MIXER: u32 = 0x2a;

/* Mixer - ADC */
pub const RT5616_REC_L1_MIXER: u32 = 0x3b;
pub const RT5616_REC_L2_MIXER: u32 = 0x3c;
pub const RT5616_REC_R1_MIXER: u32 = 0x3d;
pub const RT5616_REC_R2_MIXER: u32 = 0x3e;
/* Mixer - DAC */
pub const RT5616_HPO_MIXER: u32 = 0x45;
pub const RT5616_OUT_L1_MIXER: u32 = 0x4d;
pub const RT5616_OUT_L2_MIXER: u32 = 0x4e;
pub const RT5616_OUT_L3_MIXER: u32 = 0x4f;
pub const RT5616_OUT_R1_MIXER: u32 = 0x50;
pub const RT5616_OUT_R2_MIXER: u32 = 0x51;
pub const RT5616_OUT_R3_MIXER: u32 = 0x52;
pub const RT5616_LOUT_MIXER: u32 = 0x53;
/* Power */
pub const RT5616_PWR_DIG1: u32 = 0x61;
pub const RT5616_PWR_DIG2: u32 = 0x62;
pub const RT5616_PWR_ANLG1: u32 = 0x63;
pub const RT5616_PWR_ANLG2: u32 = 0x64;
pub const RT5616_PWR_MIXER: u32 = 0x65;
pub const RT5616_PWR_VOL: u32 = 0x66;
/* Private Register Control */
pub const RT5616_PRIV_INDEX: u32 = 0x6a;
pub const RT5616_PRIV_DATA: u32 = 0x6c;
/* Format - ADC/DAC */
pub const RT5616_I2S1_SDP: u32 = 0x70;
pub const RT5616_ADDA_CLK1: u32 = 0x73;
pub const RT5616_ADDA_CLK2: u32 = 0x74;

/* Function - Analog */
pub const RT5616_GLB_CLK: u32 = 0x80;
pub const RT5616_PLL_CTRL1: u32 = 0x81;
pub const RT5616_PLL_CTRL2: u32 = 0x82;
pub const RT5616_HP_OVCD: u32 = 0x8b;
pub const RT5616_DEPOP_M1: u32 = 0x8e;
pub const RT5616_DEPOP_M2: u32 = 0x8f;
pub const RT5616_DEPOP_M3: u32 = 0x90;
pub const RT5616_CHARGE_PUMP: u32 = 0x91;
pub const RT5616_PV_DET_SPK_G: u32 = 0x92;
pub const RT5616_MICBIAS: u32 = 0x93;
pub const RT5616_A_JD_CTL1: u32 = 0x94;
pub const RT5616_A_JD_CTL2: u32 = 0x95;
/* Function - Digital */
pub const RT5616_EQ_CTRL1: u32 = 0xb0;
pub const RT5616_EQ_CTRL2: u32 = 0xb1;
pub const RT5616_WIND_FILTER: u32 = 0xb2;
pub const RT5616_DRC_AGC_1: u32 = 0xb4;
pub const RT5616_DRC_AGC_2: u32 = 0xb5;
pub const RT5616_DRC_AGC_3: u32 = 0xb6;
pub const RT5616_SVOL_ZC: u32 = 0xb7;
pub const RT5616_JD_CTRL1: u32 = 0xbb;
pub const RT5616_JD_CTRL2: u32 = 0xbc;
pub const RT5616_IRQ_CTRL1: u32 = 0xbd;
pub const RT5616_IRQ_CTRL2: u32 = 0xbe;
pub const RT5616_INT_IRQ_ST: u32 = 0xbf;
pub const RT5616_GPIO_CTRL1: u32 = 0xc0;
pub const RT5616_GPIO_CTRL2: u32 = 0xc1;
pub const RT5616_GPIO_CTRL3: u32 = 0xc2;
pub const RT5616_PGM_REG_ARR1: u32 = 0xc8;
pub const RT5616_PGM_REG_ARR2: u32 = 0xc9;
pub const RT5616_PGM_REG_ARR3: u32 = 0xca;
pub const RT5616_PGM_REG_ARR4: u32 = 0xcb;
pub const RT5616_PGM_REG_ARR5: u32 = 0xcc;
pub const RT5616_SCB_FUNC: u32 = 0xcd;
pub const RT5616_SCB_CTRL: u32 = 0xce;
pub const RT5616_BASE_BACK: u32 = 0xcf;
pub const RT5616_MP3_PLUS1: u32 = 0xd0;
pub const RT5616_MP3_PLUS2: u32 = 0xd1;
pub const RT5616_ADJ_HPF_CTRL1: u32 = 0xd3;
pub const RT5616_ADJ_HPF_CTRL2: u32 = 0xd4;
pub const RT5616_HP_CALIB_AMP_DET: u32 = 0xd6;
pub const RT5616_HP_CALIB2: u32 = 0xd7;
pub const RT5616_SV_ZCD1: u32 = 0xd9;
pub const RT5616_SV_ZCD2: u32 = 0xda;
pub const RT5616_D_MISC: u32 = 0xfa;
/* Dummy Register */
pub const RT5616_DUMMY2: u32 = 0xfb;
pub const RT5616_DUMMY3: u32 = 0xfc;


/* Index of Codec Private Register definition */
pub const RT5616_BIAS_CUR1: u32 = 0x12;
pub const RT5616_BIAS_CUR3: u32 = 0x14;
pub const RT5616_CLSD_INT_REG1: u32 = 0x1c;
pub const RT5616_MAMP_INT_REG2: u32 = 0x37;
pub const RT5616_CHOP_DAC_ADC: u32 = 0x3d;
pub const RT5616_3D_SPK: u32 = 0x63;
pub const RT5616_WND_1: u32 = 0x6c;
pub const RT5616_WND_2: u32 = 0x6d;
pub const RT5616_WND_3: u32 = 0x6e;
pub const RT5616_WND_4: u32 = 0x6f;
pub const RT5616_WND_5: u32 = 0x70;
pub const RT5616_WND_8: u32 = 0x73;
pub const RT5616_DIP_SPK_INF: u32 = 0x75;
pub const RT5616_HP_DCC_INT1: u32 = 0x77;
pub const RT5616_EQ_BW_LOP: u32 = 0xa0;
pub const RT5616_EQ_GN_LOP: u32 = 0xa1;
pub const RT5616_EQ_FC_BP1: u32 = 0xa2;
pub const RT5616_EQ_BW_BP1: u32 = 0xa3;
pub const RT5616_EQ_GN_BP1: u32 = 0xa4;
pub const RT5616_EQ_FC_BP2: u32 = 0xa5;
pub const RT5616_EQ_BW_BP2: u32 = 0xa6;
pub const RT5616_EQ_GN_BP2: u32 = 0xa7;
pub const RT5616_EQ_FC_BP3: u32 = 0xa8;
pub const RT5616_EQ_BW_BP3: u32 = 0xa9;
pub const RT5616_EQ_GN_BP3: u32 = 0xaa;
pub const RT5616_EQ_FC_BP4: u32 = 0xab;
pub const RT5616_EQ_BW_BP4: u32 = 0xac;
pub const RT5616_EQ_GN_BP4: u32 = 0xad;
pub const RT5616_EQ_FC_HIP1: u32 = 0xae;
pub const RT5616_EQ_GN_HIP1: u32 = 0xaf;
pub const RT5616_EQ_FC_HIP2: u32 = 0xb0;
pub const RT5616_EQ_BW_HIP2: u32 = 0xb1;
pub const RT5616_EQ_GN_HIP2: u32 = 0xb2;
pub const RT5616_EQ_PRE_VOL: u32 = 0xb3;
pub const RT5616_EQ_PST_VOL: u32 = 0xb4;


/* global definition */
pub const RT5616_L_MUTE: u32 = 0x1 << 15;
pub const RT5616_L_MUTE_SFT: u32 = 15;
pub const RT5616_VOL_L_MUTE: u32 = 0x1 << 14;
pub const RT5616_VOL_L_SFT: u32 = 14;
pub const RT5616_R_MUTE: u32 = 0x1 << 7;
pub const RT5616_R_MUTE_SFT: u32 = 7;
pub const RT5616_VOL_R_MUTE: u32 = 0x1 << 6;
pub const RT5616_VOL_R_SFT: u32 = 6;
pub const RT5616_L_VOL_MASK: u32 = 0x3f << 8;
pub const RT5616_L_VOL_SFT: u32 = 8;
pub const RT5616_R_VOL_MASK: u32 = 0x3f;
pub const RT5616_R_VOL_SFT: u32 = 0;

/* LOUT Control 2(0x05) */
pub const RT5616_EN_DFO: u32 = 0x1 << 15;

/* IN1 and IN2 Control (0x0d) */
/* IN3 and IN4 Control (0x0e) */
pub const RT5616_BST_MASK1: u32 = 0xf<<12;
pub const RT5616_BST_SFT1: u32 = 12;
pub const RT5616_BST_MASK2: u32 = 0xf<<8;
pub const RT5616_BST_SFT2: u32 = 8;
pub const RT5616_IN_DF1: u32 = 0x1 << 7;
pub const RT5616_IN_SFT1: u32 = 7;
pub const RT5616_IN_DF2: u32 = 0x1 << 6;
pub const RT5616_IN_SFT2: u32 = 6;

/* INL1 and INR1 Volume Control (0x0f) */
pub const RT5616_INL_VOL_MASK: u32 = 0x1f << 8;
pub const RT5616_INL_VOL_SFT: u32 = 8;
pub const RT5616_INR_SEL_MASK: u32 = 0x1 << 7;
pub const RT5616_INR_SEL_SFT: u32 = 7;
pub const RT5616_INR_SEL_IN4N: u32 = 0x0 << 7;
pub const RT5616_INR_SEL_MONON: u32 = 0x1 << 7;
pub const RT5616_INR_VOL_MASK: u32 = 0x1f;
pub const RT5616_INR_VOL_SFT: u32 = 0;

/* DAC1 Digital Volume (0x19) */
pub const RT5616_DAC_L1_VOL_MASK: u32 = 0xff << 8;
pub const RT5616_DAC_L1_VOL_SFT: u32 = 8;
pub const RT5616_DAC_R1_VOL_MASK: u32 = 0xff;
pub const RT5616_DAC_R1_VOL_SFT: u32 = 0;

/* DAC2 Digital Volume (0x1a) */
pub const RT5616_DAC_L2_VOL_MASK: u32 = 0xff << 8;
pub const RT5616_DAC_L2_VOL_SFT: u32 = 8;
pub const RT5616_DAC_R2_VOL_MASK: u32 = 0xff;
pub const RT5616_DAC_R2_VOL_SFT: u32 = 0;

/* ADC Digital Volume Control (0x1c) */
pub const RT5616_ADC_L_VOL_MASK: u32 = 0x7f << 8;
pub const RT5616_ADC_L_VOL_SFT: u32 = 8;
pub const RT5616_ADC_R_VOL_MASK: u32 = 0x7f;
pub const RT5616_ADC_R_VOL_SFT: u32 = 0;

/* Mono ADC Digital Volume Control (0x1d) */
pub const RT5616_M_MONO_ADC_L: u32 = 0x1 << 15;
pub const RT5616_M_MONO_ADC_L_SFT: u32 = 15;
pub const RT5616_MONO_ADC_L_VOL_MASK: u32 = 0x7f << 8;
pub const RT5616_MONO_ADC_L_VOL_SFT: u32 = 8;
pub const RT5616_M_MONO_ADC_R: u32 = 0x1 << 7;
pub const RT5616_M_MONO_ADC_R_SFT: u32 = 7;
pub const RT5616_MONO_ADC_R_VOL_MASK: u32 = 0x7f;
pub const RT5616_MONO_ADC_R_VOL_SFT: u32 = 0;

/* ADC Boost Volume Control (0x1e) */
pub const RT5616_ADC_L_BST_MASK: u32 = 0x3 << 14;
pub const RT5616_ADC_L_BST_SFT: u32 = 14;
pub const RT5616_ADC_R_BST_MASK: u32 = 0x3 << 12;
pub const RT5616_ADC_R_BST_SFT: u32 = 12;
pub const RT5616_ADC_COMP_MASK: u32 = 0x3 << 10;
pub const RT5616_ADC_COMP_SFT: u32 = 10;

/* Stereo ADC1 Mixer Control (0x27) */
pub const RT5616_M_STO1_ADC_L1: u32 = 0x1 << 14;
pub const RT5616_M_STO1_ADC_L1_SFT: u32 = 14;
pub const RT5616_M_STO1_ADC_R1: u32 = 0x1 << 6;
pub const RT5616_M_STO1_ADC_R1_SFT: u32 = 6;

/* ADC Mixer to DAC Mixer Control (0x29) */
pub const RT5616_M_ADCMIX_L: u32 = 0x1 << 15;
pub const RT5616_M_ADCMIX_L_SFT: u32 = 15;
pub const RT5616_M_IF1_DAC_L: u32 = 0x1 << 14;
pub const RT5616_M_IF1_DAC_L_SFT: u32 = 14;
pub const RT5616_M_ADCMIX_R: u32 = 0x1 << 7;
pub const RT5616_M_ADCMIX_R_SFT: u32 = 7;
pub const RT5616_M_IF1_DAC_R: u32 = 0x1 << 6;
pub const RT5616_M_IF1_DAC_R_SFT: u32 = 6;

/* Stereo DAC Mixer Control (0x2a) */
pub const RT5616_M_DAC_L1_MIXL: u32 = 0x1 << 14;
pub const RT5616_M_DAC_L1_MIXL_SFT: u32 = 14;
pub const RT5616_DAC_L1_STO_L_VOL_MASK: u32 = 0x1 << 13;
pub const RT5616_DAC_L1_STO_L_VOL_SFT: u32 = 13;
pub const RT5616_M_DAC_R1_MIXL: u32 = 0x1 << 9;
pub const RT5616_M_DAC_R1_MIXL_SFT: u32 = 9;
pub const RT5616_DAC_R1_STO_L_VOL_MASK: u32 = 0x1 << 8;
pub const RT5616_DAC_R1_STO_L_VOL_SFT: u32 = 8;
pub const RT5616_M_DAC_R1_MIXR: u32 = 0x1 << 6;
pub const RT5616_M_DAC_R1_MIXR_SFT: u32 = 6;
pub const RT5616_DAC_R1_STO_R_VOL_MASK: u32 = 0x1 << 5;
pub const RT5616_DAC_R1_STO_R_VOL_SFT: u32 = 5;
pub const RT5616_M_DAC_L1_MIXR: u32 = 0x1 << 1;
pub const RT5616_M_DAC_L1_MIXR_SFT: u32 = 1;
pub const RT5616_DAC_L1_STO_R_VOL_MASK: u32 = 0x1;
pub const RT5616_DAC_L1_STO_R_VOL_SFT: u32 = 0;

/* DD Mixer Control (0x2b) */
pub const RT5616_M_STO_DD_L1: u32 = 0x1 << 14;
pub const RT5616_M_STO_DD_L1_SFT: u32 = 14;
pub const RT5616_STO_DD_L1_VOL_MASK: u32 = 0x1 << 13;
pub const RT5616_DAC_DD_L1_VOL_SFT: u32 = 13;
pub const RT5616_M_STO_DD_L2: u32 = 0x1 << 12;
pub const RT5616_M_STO_DD_L2_SFT: u32 = 12;
pub const RT5616_STO_DD_L2_VOL_MASK: u32 = 0x1 << 11;
pub const RT5616_STO_DD_L2_VOL_SFT: u32 = 11;
pub const RT5616_M_STO_DD_R2_L: u32 = 0x1 << 10;
pub const RT5616_M_STO_DD_R2_L_SFT: u32 = 10;
pub const RT5616_STO_DD_R2_L_VOL_MASK: u32 = 0x1 << 9;
pub const RT5616_STO_DD_R2_L_VOL_SFT: u32 = 9;
pub const RT5616_M_STO_DD_R1: u32 = 0x1 << 6;
pub const RT5616_M_STO_DD_R1_SFT: u32 = 6;
pub const RT5616_STO_DD_R1_VOL_MASK: u32 = 0x1 << 5;
pub const RT5616_STO_DD_R1_VOL_SFT: u32 = 5;
pub const RT5616_M_STO_DD_R2: u32 = 0x1 << 4;
pub const RT5616_M_STO_DD_R2_SFT: u32 = 4;
pub const RT5616_STO_DD_R2_VOL_MASK: u32 = 0x1 << 3;
pub const RT5616_STO_DD_R2_VOL_SFT: u32 = 3;
pub const RT5616_M_STO_DD_L2_R: u32 = 0x1 << 2;
pub const RT5616_M_STO_DD_L2_R_SFT: u32 = 2;
pub const RT5616_STO_DD_L2_R_VOL_MASK: u32 = 0x1 << 1;
pub const RT5616_STO_DD_L2_R_VOL_SFT: u32 = 1;

/* Digital Mixer Control (0x2c) */
pub const RT5616_M_STO_L_DAC_L: u32 = 0x1 << 15;
pub const RT5616_M_STO_L_DAC_L_SFT: u32 = 15;
pub const RT5616_STO_L_DAC_L_VOL_MASK: u32 = 0x1 << 14;
pub const RT5616_STO_L_DAC_L_VOL_SFT: u32 = 14;
pub const RT5616_M_DAC_L2_DAC_L: u32 = 0x1 << 13;
pub const RT5616_M_DAC_L2_DAC_L_SFT: u32 = 13;
pub const RT5616_DAC_L2_DAC_L_VOL_MASK: u32 = 0x1 << 12;
pub const RT5616_DAC_L2_DAC_L_VOL_SFT: u32 = 12;
pub const RT5616_M_STO_R_DAC_R: u32 = 0x1 << 11;
pub const RT5616_M_STO_R_DAC_R_SFT: u32 = 11;
pub const RT5616_STO_R_DAC_R_VOL_MASK: u32 = 0x1 << 10;
pub const RT5616_STO_R_DAC_R_VOL_SFT: u32 = 10;
pub const RT5616_M_DAC_R2_DAC_R: u32 = 0x1 << 9;
pub const RT5616_M_DAC_R2_DAC_R_SFT: u32 = 9;
pub const RT5616_DAC_R2_DAC_R_VOL_MASK: u32 = 0x1 << 8;
pub const RT5616_DAC_R2_DAC_R_VOL_SFT: u32 = 8;

/* DSP Path Control 1 (0x2d) */
pub const RT5616_RXDP_SRC_MASK: u32 = 0x1 << 15;
pub const RT5616_RXDP_SRC_SFT: u32 = 15;
pub const RT5616_RXDP_SRC_NOR: u32 = 0x0 << 15;
pub const RT5616_RXDP_SRC_DIV3: u32 = 0x1 << 15;
pub const RT5616_TXDP_SRC_MASK: u32 = 0x1 << 14;
pub const RT5616_TXDP_SRC_SFT: u32 = 14;
pub const RT5616_TXDP_SRC_NOR: u32 = 0x0 << 14;
pub const RT5616_TXDP_SRC_DIV3: u32 = 0x1 << 14;

/* DSP Path Control 2 (0x2e) */
pub const RT5616_DAC_L2_SEL_MASK: u32 = 0x3 << 14;
pub const RT5616_DAC_L2_SEL_SFT: u32 = 14;
pub const RT5616_DAC_L2_SEL_IF2: u32 = 0x0 << 14;
pub const RT5616_DAC_L2_SEL_IF3: u32 = 0x1 << 14;
pub const RT5616_DAC_L2_SEL_TXDC: u32 = 0x2 << 14;
pub const RT5616_DAC_L2_SEL_BASS: u32 = 0x3 << 14;
pub const RT5616_DAC_R2_SEL_MASK: u32 = 0x3 << 12;
pub const RT5616_DAC_R2_SEL_SFT: u32 = 12;
pub const RT5616_DAC_R2_SEL_IF2: u32 = 0x0 << 12;
pub const RT5616_DAC_R2_SEL_IF3: u32 = 0x1 << 12;
pub const RT5616_DAC_R2_SEL_TXDC: u32 = 0x2 << 12;
pub const RT5616_IF2_ADC_L_SEL_MASK: u32 = 0x1 << 11;
pub const RT5616_IF2_ADC_L_SEL_SFT: u32 = 11;
pub const RT5616_IF2_ADC_L_SEL_TXDP: u32 = 0x0 << 11;
pub const RT5616_IF2_ADC_L_SEL_PASS: u32 = 0x1 << 11;
pub const RT5616_IF2_ADC_R_SEL_MASK: u32 = 0x1 << 10;
pub const RT5616_IF2_ADC_R_SEL_SFT: u32 = 10;
pub const RT5616_IF2_ADC_R_SEL_TXDP: u32 = 0x0 << 10;
pub const RT5616_IF2_ADC_R_SEL_PASS: u32 = 0x1 << 10;
pub const RT5616_RXDC_SEL_MASK: u32 = 0x3 << 8;
pub const RT5616_RXDC_SEL_SFT: u32 = 8;
pub const RT5616_RXDC_SEL_NOR: u32 = 0x0 << 8;
pub const RT5616_RXDC_SEL_L2R: u32 = 0x1 << 8;
pub const RT5616_RXDC_SEL_R2L: u32 = 0x2 << 8;
pub const RT5616_RXDC_SEL_SWAP: u32 = 0x3 << 8;
pub const RT5616_RXDP_SEL_MASK: u32 = 0x3 << 6;
pub const RT5616_RXDP_SEL_SFT: u32 = 6;
pub const RT5616_RXDP_SEL_NOR: u32 = 0x0 << 6;
pub const RT5616_RXDP_SEL_L2R: u32 = 0x1 << 6;
pub const RT5616_RXDP_SEL_R2L: u32 = 0x2 << 6;
pub const RT5616_RXDP_SEL_SWAP: u32 = 0x3 << 6;
pub const RT5616_TXDC_SEL_MASK: u32 = 0x3 << 4;
pub const RT5616_TXDC_SEL_SFT: u32 = 4;
pub const RT5616_TXDC_SEL_NOR: u32 = 0x0 << 4;
pub const RT5616_TXDC_SEL_L2R: u32 = 0x1 << 4;
pub const RT5616_TXDC_SEL_R2L: u32 = 0x2 << 4;
pub const RT5616_TXDC_SEL_SWAP: u32 = 0x3 << 4;
pub const RT5616_TXDP_SEL_MASK: u32 = 0x3 << 2;
pub const RT5616_TXDP_SEL_SFT: u32 = 2;
pub const RT5616_TXDP_SEL_NOR: u32 = 0x0 << 2;
pub const RT5616_TXDP_SEL_L2R: u32 = 0x1 << 2;
pub const RT5616_TXDP_SEL_R2L: u32 = 0x2 << 2;
pub const RT5616_TRXDP_SEL_SWAP: u32 = 0x3 << 2;

/* REC Left Mixer Control 1 (0x3b) */
pub const RT5616_G_LN_L2_RM_L_MASK: u32 = 0x7 << 13;
pub const RT5616_G_IN_L2_RM_L_SFT: u32 = 13;
pub const RT5616_G_LN_L1_RM_L_MASK: u32 = 0x7 << 10;
pub const RT5616_G_IN_L1_RM_L_SFT: u32 = 10;
pub const RT5616_G_BST3_RM_L_MASK: u32 = 0x7 << 4;
pub const RT5616_G_BST3_RM_L_SFT: u32 = 4;
pub const RT5616_G_BST2_RM_L_MASK: u32 = 0x7 << 1;
pub const RT5616_G_BST2_RM_L_SFT: u32 = 1;

/* REC Left Mixer Control 2 (0x3c) */
pub const RT5616_G_BST1_RM_L_MASK: u32 = 0x7 << 13;
pub const RT5616_G_BST1_RM_L_SFT: u32 = 13;
pub const RT5616_G_OM_L_RM_L_MASK: u32 = 0x7 << 10;
pub const RT5616_G_OM_L_RM_L_SFT: u32 = 10;
pub const RT5616_M_IN2_L_RM_L: u32 = 0x1 << 6;
pub const RT5616_M_IN2_L_RM_L_SFT: u32 = 6;
pub const RT5616_M_IN1_L_RM_L: u32 = 0x1 << 5;
pub const RT5616_M_IN1_L_RM_L_SFT: u32 = 5;
pub const RT5616_M_BST3_RM_L: u32 = 0x1 << 3;
pub const RT5616_M_BST3_RM_L_SFT: u32 = 3;
pub const RT5616_M_BST2_RM_L: u32 = 0x1 << 2;
pub const RT5616_M_BST2_RM_L_SFT: u32 = 2;
pub const RT5616_M_BST1_RM_L: u32 = 0x1 << 1;
pub const RT5616_M_BST1_RM_L_SFT: u32 = 1;
pub const RT5616_M_OM_L_RM_L: u32 = 0x1;
pub const RT5616_M_OM_L_RM_L_SFT: u32 = 0;

/* REC Right Mixer Control 1 (0x3d) */
pub const RT5616_G_IN2_R_RM_R_MASK: u32 = 0x7 << 13;
pub const RT5616_G_IN2_R_RM_R_SFT: u32 = 13;
pub const RT5616_G_IN1_R_RM_R_MASK: u32 = 0x7 << 10;
pub const RT5616_G_IN1_R_RM_R_SFT: u32 = 10;
pub const RT5616_G_BST3_RM_R_MASK: u32 = 0x7 << 4;
pub const RT5616_G_BST3_RM_R_SFT: u32 = 4;
pub const RT5616_G_BST2_RM_R_MASK: u32 = 0x7 << 1;
pub const RT5616_G_BST2_RM_R_SFT: u32 = 1;

/* REC Right Mixer Control 2 (0x3e) */
pub const RT5616_G_BST1_RM_R_MASK: u32 = 0x7 << 13;
pub const RT5616_G_BST1_RM_R_SFT: u32 = 13;
pub const RT5616_G_OM_R_RM_R_MASK: u32 = 0x7 << 10;
pub const RT5616_G_OM_R_RM_R_SFT: u32 = 10;
pub const RT5616_M_IN2_R_RM_R: u32 = 0x1 << 6;
pub const RT5616_M_IN2_R_RM_R_SFT: u32 = 6;
pub const RT5616_M_IN1_R_RM_R: u32 = 0x1 << 5;
pub const RT5616_M_IN1_R_RM_R_SFT: u32 = 5;
pub const RT5616_M_BST3_RM_R: u32 = 0x1 << 3;
pub const RT5616_M_BST3_RM_R_SFT: u32 = 3;
pub const RT5616_M_BST2_RM_R: u32 = 0x1 << 2;
pub const RT5616_M_BST2_RM_R_SFT: u32 = 2;
pub const RT5616_M_BST1_RM_R: u32 = 0x1 << 1;
pub const RT5616_M_BST1_RM_R_SFT: u32 = 1;
pub const RT5616_M_OM_R_RM_R: u32 = 0x1;
pub const RT5616_M_OM_R_RM_R_SFT: u32 = 0;

/* HPMIX Control (0x45) */
pub const RT5616_M_DAC1_HM: u32 = 0x1 << 14;
pub const RT5616_M_DAC1_HM_SFT: u32 = 14;
pub const RT5616_M_HPVOL_HM: u32 = 0x1 << 13;
pub const RT5616_M_HPVOL_HM_SFT: u32 = 13;
pub const RT5616_G_HPOMIX_MASK: u32 = 0x1 << 12;
pub const RT5616_G_HPOMIX_SFT: u32 = 12;

/* SPK Left Mixer Control (0x46) */
pub const RT5616_G_RM_L_SM_L_MASK: u32 = 0x3 << 14;
pub const RT5616_G_RM_L_SM_L_SFT: u32 = 14;
pub const RT5616_G_IN_L_SM_L_MASK: u32 = 0x3 << 12;
pub const RT5616_G_IN_L_SM_L_SFT: u32 = 12;
pub const RT5616_G_DAC_L1_SM_L_MASK: u32 = 0x3 << 10;
pub const RT5616_G_DAC_L1_SM_L_SFT: u32 = 10;
pub const RT5616_G_DAC_L2_SM_L_MASK: u32 = 0x3 << 8;
pub const RT5616_G_DAC_L2_SM_L_SFT: u32 = 8;
pub const RT5616_G_OM_L_SM_L_MASK: u32 = 0x3 << 6;
pub const RT5616_G_OM_L_SM_L_SFT: u32 = 6;
pub const RT5616_M_RM_L_SM_L: u32 = 0x1 << 5;
pub const RT5616_M_RM_L_SM_L_SFT: u32 = 5;
pub const RT5616_M_IN_L_SM_L: u32 = 0x1 << 4;
pub const RT5616_M_IN_L_SM_L_SFT: u32 = 4;
pub const RT5616_M_DAC_L1_SM_L: u32 = 0x1 << 3;
pub const RT5616_M_DAC_L1_SM_L_SFT: u32 = 3;
pub const RT5616_M_DAC_L2_SM_L: u32 = 0x1 << 2;
pub const RT5616_M_DAC_L2_SM_L_SFT: u32 = 2;
pub const RT5616_M_OM_L_SM_L: u32 = 0x1 << 1;
pub const RT5616_M_OM_L_SM_L_SFT: u32 = 1;

/* SPK Right Mixer Control (0x47) */
pub const RT5616_G_RM_R_SM_R_MASK: u32 = 0x3 << 14;
pub const RT5616_G_RM_R_SM_R_SFT: u32 = 14;
pub const RT5616_G_IN_R_SM_R_MASK: u32 = 0x3 << 12;
pub const RT5616_G_IN_R_SM_R_SFT: u32 = 12;
pub const RT5616_G_DAC_R1_SM_R_MASK: u32 = 0x3 << 10;
pub const RT5616_G_DAC_R1_SM_R_SFT: u32 = 10;
pub const RT5616_G_DAC_R2_SM_R_MASK: u32 = 0x3 << 8;
pub const RT5616_G_DAC_R2_SM_R_SFT: u32 = 8;
pub const RT5616_G_OM_R_SM_R_MASK: u32 = 0x3 << 6;
pub const RT5616_G_OM_R_SM_R_SFT: u32 = 6;
pub const RT5616_M_RM_R_SM_R: u32 = 0x1 << 5;
pub const RT5616_M_RM_R_SM_R_SFT: u32 = 5;
pub const RT5616_M_IN_R_SM_R: u32 = 0x1 << 4;
pub const RT5616_M_IN_R_SM_R_SFT: u32 = 4;
pub const RT5616_M_DAC_R1_SM_R: u32 = 0x1 << 3;
pub const RT5616_M_DAC_R1_SM_R_SFT: u32 = 3;
pub const RT5616_M_DAC_R2_SM_R: u32 = 0x1 << 2;
pub const RT5616_M_DAC_R2_SM_R_SFT: u32 = 2;
pub const RT5616_M_OM_R_SM_R: u32 = 0x1 << 1;
pub const RT5616_M_OM_R_SM_R_SFT: u32 = 1;

/* SPOLMIX Control (0x48) */
pub const RT5616_M_DAC_R1_SPM_L: u32 = 0x1 << 15;
pub const RT5616_M_DAC_R1_SPM_L_SFT: u32 = 15;
pub const RT5616_M_DAC_L1_SPM_L: u32 = 0x1 << 14;
pub const RT5616_M_DAC_L1_SPM_L_SFT: u32 = 14;
pub const RT5616_M_SV_R_SPM_L: u32 = 0x1 << 13;
pub const RT5616_M_SV_R_SPM_L_SFT: u32 = 13;
pub const RT5616_M_SV_L_SPM_L: u32 = 0x1 << 12;
pub const RT5616_M_SV_L_SPM_L_SFT: u32 = 12;
pub const RT5616_M_BST1_SPM_L: u32 = 0x1 << 11;
pub const RT5616_M_BST1_SPM_L_SFT: u32 = 11;

/* SPORMIX Control (0x49) */
pub const RT5616_M_DAC_R1_SPM_R: u32 = 0x1 << 13;
pub const RT5616_M_DAC_R1_SPM_R_SFT: u32 = 13;
pub const RT5616_M_SV_R_SPM_R: u32 = 0x1 << 12;
pub const RT5616_M_SV_R_SPM_R_SFT: u32 = 12;
pub const RT5616_M_BST1_SPM_R: u32 = 0x1 << 11;
pub const RT5616_M_BST1_SPM_R_SFT: u32 = 11;

/* SPOLMIX / SPORMIX Ratio Control (0x4a) */
pub const RT5616_SPO_CLSD_RATIO_MASK: u32 = 0x7;
pub const RT5616_SPO_CLSD_RATIO_SFT: u32 = 0;

/* Mono Output Mixer Control (0x4c) */
pub const RT5616_M_DAC_R2_MM: u32 = 0x1 << 15;
pub const RT5616_M_DAC_R2_MM_SFT: u32 = 15;
pub const RT5616_M_DAC_L2_MM: u32 = 0x1 << 14;
pub const RT5616_M_DAC_L2_MM_SFT: u32 = 14;
pub const RT5616_M_OV_R_MM: u32 = 0x1 << 13;
pub const RT5616_M_OV_R_MM_SFT: u32 = 13;
pub const RT5616_M_OV_L_MM: u32 = 0x1 << 12;
pub const RT5616_M_OV_L_MM_SFT: u32 = 12;
pub const RT5616_M_BST1_MM: u32 = 0x1 << 11;
pub const RT5616_M_BST1_MM_SFT: u32 = 11;
pub const RT5616_G_MONOMIX_MASK: u32 = 0x1 << 10;
pub const RT5616_G_MONOMIX_SFT: u32 = 10;

/* Output Left Mixer Control 1 (0x4d) */
pub const RT5616_G_BST2_OM_L_MASK: u32 = 0x7 << 10;
pub const RT5616_G_BST2_OM_L_SFT: u32 = 10;
pub const RT5616_G_BST1_OM_L_MASK: u32 = 0x7 << 7;
pub const RT5616_G_BST1_OM_L_SFT: u32 = 7;
pub const RT5616_G_IN1_L_OM_L_MASK: u32 = 0x7 << 4;
pub const RT5616_G_IN1_L_OM_L_SFT: u32 = 4;
pub const RT5616_G_RM_L_OM_L_MASK: u32 = 0x7 << 1;
pub const RT5616_G_RM_L_OM_L_SFT: u32 = 1;

/* Output Left Mixer Control 2 (0x4e) */
pub const RT5616_G_DAC_L1_OM_L_MASK: u32 = 0x7 << 7;
pub const RT5616_G_DAC_L1_OM_L_SFT: u32 = 7;
pub const RT5616_G_IN2_L_OM_L_MASK: u32 = 0x7 << 4;
pub const RT5616_G_IN2_L_OM_L_SFT: u32 = 4;

/* Output Left Mixer Control 3 (0x4f) */
pub const RT5616_M_IN2_L_OM_L: u32 = 0x1 << 9;
pub const RT5616_M_IN2_L_OM_L_SFT: u32 = 9;
pub const RT5616_M_BST2_OM_L: u32 = 0x1 << 6;
pub const RT5616_M_BST2_OM_L_SFT: u32 = 6;
pub const RT5616_M_BST1_OM_L: u32 = 0x1 << 5;
pub const RT5616_M_BST1_OM_L_SFT: u32 = 5;
pub const RT5616_M_IN1_L_OM_L: u32 = 0x1 << 4;
pub const RT5616_M_IN1_L_OM_L_SFT: u32 = 4;
pub const RT5616_M_RM_L_OM_L: u32 = 0x1 << 3;
pub const RT5616_M_RM_L_OM_L_SFT: u32 = 3;
pub const RT5616_M_DAC_L1_OM_L: u32 = 0x1;
pub const RT5616_M_DAC_L1_OM_L_SFT: u32 = 0;

/* Output Right Mixer Control 1 (0x50) */
pub const RT5616_G_BST2_OM_R_MASK: u32 = 0x7 << 10;
pub const RT5616_G_BST2_OM_R_SFT: u32 = 10;
pub const RT5616_G_BST1_OM_R_MASK: u32 = 0x7 << 7;
pub const RT5616_G_BST1_OM_R_SFT: u32 = 7;
pub const RT5616_G_IN1_R_OM_R_MASK: u32 = 0x7 << 4;
pub const RT5616_G_IN1_R_OM_R_SFT: u32 = 4;
pub const RT5616_G_RM_R_OM_R_MASK: u32 = 0x7 << 1;
pub const RT5616_G_RM_R_OM_R_SFT: u32 = 1;

/* Output Right Mixer Control 2 (0x51) */
pub const RT5616_G_DAC_R1_OM_R_MASK: u32 = 0x7 << 7;
pub const RT5616_G_DAC_R1_OM_R_SFT: u32 = 7;
pub const RT5616_G_IN2_R_OM_R_MASK: u32 = 0x7 << 4;
pub const RT5616_G_IN2_R_OM_R_SFT: u32 = 4;

/* Output Right Mixer Control 3 (0x52) */
pub const RT5616_M_IN2_R_OM_R: u32 = 0x1 << 9;
pub const RT5616_M_IN2_R_OM_R_SFT: u32 = 9;
pub const RT5616_M_BST2_OM_R: u32 = 0x1 << 6;
pub const RT5616_M_BST2_OM_R_SFT: u32 = 6;
pub const RT5616_M_BST1_OM_R: u32 = 0x1 << 5;
pub const RT5616_M_BST1_OM_R_SFT: u32 = 5;
pub const RT5616_M_IN1_R_OM_R: u32 = 0x1 << 4;
pub const RT5616_M_IN1_R_OM_R_SFT: u32 = 4;
pub const RT5616_M_RM_R_OM_R: u32 = 0x1 << 3;
pub const RT5616_M_RM_R_OM_R_SFT: u32 = 3;
pub const RT5616_M_DAC_R1_OM_R: u32 = 0x1;
pub const RT5616_M_DAC_R1_OM_R_SFT: u32 = 0;

/* LOUT Mixer Control (0x53) */
pub const RT5616_M_DAC_L1_LM: u32 = 0x1 << 15;
pub const RT5616_M_DAC_L1_LM_SFT: u32 = 15;
pub const RT5616_M_DAC_R1_LM: u32 = 0x1 << 14;
pub const RT5616_M_DAC_R1_LM_SFT: u32 = 14;
pub const RT5616_M_OV_L_LM: u32 = 0x1 << 13;
pub const RT5616_M_OV_L_LM_SFT: u32 = 13;
pub const RT5616_M_OV_R_LM: u32 = 0x1 << 12;
pub const RT5616_M_OV_R_LM_SFT: u32 = 12;
pub const RT5616_G_LOUTMIX_MASK: u32 = 0x1 << 11;
pub const RT5616_G_LOUTMIX_SFT: u32 = 11;

/* Power Management for Digital 1 (0x61) */
pub const RT5616_PWR_I2S1: u32 = 0x1 << 15;
pub const RT5616_PWR_I2S1_BIT: u32 = 15;
pub const RT5616_PWR_I2S2: u32 = 0x1 << 14;
pub const RT5616_PWR_I2S2_BIT: u32 = 14;
pub const RT5616_PWR_DAC_L1: u32 = 0x1 << 12;
pub const RT5616_PWR_DAC_L1_BIT: u32 = 12;
pub const RT5616_PWR_DAC_R1: u32 = 0x1 << 11;
pub const RT5616_PWR_DAC_R1_BIT: u32 = 11;
pub const RT5616_PWR_ADC_L: u32 = 0x1 << 2;
pub const RT5616_PWR_ADC_L_BIT: u32 = 2;
pub const RT5616_PWR_ADC_R: u32 = 0x1 << 1;
pub const RT5616_PWR_ADC_R_BIT: u32 = 1;

/* Power Management for Digital 2 (0x62) */
pub const RT5616_PWR_ADC_STO1_F: u32 = 0x1 << 15;
pub const RT5616_PWR_ADC_STO1_F_BIT: u32 = 15;
pub const RT5616_PWR_DAC_STO1_F: u32 = 0x1 << 11;
pub const RT5616_PWR_DAC_STO1_F_BIT: u32 = 11;

/* Power Management for Analog 1 (0x63) */
pub const RT5616_PWR_VREF1: u32 = 0x1 << 15;
pub const RT5616_PWR_VREF1_BIT: u32 = 15;
pub const RT5616_PWR_FV1: u32 = 0x1 << 14;
pub const RT5616_PWR_FV1_BIT: u32 = 14;
pub const RT5616_PWR_MB: u32 = 0x1 << 13;
pub const RT5616_PWR_MB_BIT: u32 = 13;
pub const RT5616_PWR_LM: u32 = 0x1 << 12;
pub const RT5616_PWR_LM_BIT: u32 = 12;
pub const RT5616_PWR_BG: u32 = 0x1 << 11;
pub const RT5616_PWR_BG_BIT: u32 = 11;
pub const RT5616_PWR_HP_L: u32 = 0x1 << 7;
pub const RT5616_PWR_HP_L_BIT: u32 = 7;
pub const RT5616_PWR_HP_R: u32 = 0x1 << 6;
pub const RT5616_PWR_HP_R_BIT: u32 = 6;
pub const RT5616_PWR_HA: u32 = 0x1 << 5;
pub const RT5616_PWR_HA_BIT: u32 = 5;
pub const RT5616_PWR_VREF2: u32 = 0x1 << 4;
pub const RT5616_PWR_VREF2_BIT: u32 = 4;
pub const RT5616_PWR_FV2: u32 = 0x1 << 3;
pub const RT5616_PWR_FV2_BIT: u32 = 3;
pub const RT5616_PWR_LDO: u32 = 0x1 << 2;
pub const RT5616_PWR_LDO_BIT: u32 = 2;
pub const RT5616_PWR_LDO_DVO_MASK: u32 = 0x3;
pub const RT5616_PWR_LDO_DVO_1_0V: u32 = 0;
pub const RT5616_PWR_LDO_DVO_1_1V: u32 = 1;
pub const RT5616_PWR_LDO_DVO_1_2V: u32 = 2;
pub const RT5616_PWR_LDO_DVO_1_3V: u32 = 3;

/* Power Management for Analog 2 (0x64) */
pub const RT5616_PWR_BST1: u32 = 0x1 << 15;
pub const RT5616_PWR_BST1_BIT: u32 = 15;
pub const RT5616_PWR_BST2: u32 = 0x1 << 14;
pub const RT5616_PWR_BST2_BIT: u32 = 14;
pub const RT5616_PWR_MB1: u32 = 0x1 << 11;
pub const RT5616_PWR_MB1_BIT: u32 = 11;
pub const RT5616_PWR_PLL: u32 = 0x1 << 9;
pub const RT5616_PWR_PLL_BIT: u32 = 9;
pub const RT5616_PWR_BST1_OP2: u32 = 0x1 << 5;
pub const RT5616_PWR_BST1_OP2_BIT: u32 = 5;
pub const RT5616_PWR_BST2_OP2: u32 = 0x1 << 4;
pub const RT5616_PWR_BST2_OP2_BIT: u32 = 4;
pub const RT5616_PWR_BST3_OP2: u32 = 0x1 << 3;
pub const RT5616_PWR_BST3_OP2_BIT: u32 = 3;
pub const RT5616_PWR_JD_M: u32 = 0x1 << 2;
pub const RT5616_PWM_JD_M_BIT: u32 = 2;
pub const RT5616_PWR_JD2: u32 = 0x1 << 1;
pub const RT5616_PWM_JD2_BIT: u32 = 1;
pub const RT5616_PWR_JD3: u32 = 0x1;
pub const RT5616_PWM_JD3_BIT: u32 = 0;

/* Power Management for Mixer (0x65) */
pub const RT5616_PWR_OM_L: u32 = 0x1 << 15;
pub const RT5616_PWR_OM_L_BIT: u32 = 15;
pub const RT5616_PWR_OM_R: u32 = 0x1 << 14;
pub const RT5616_PWR_OM_R_BIT: u32 = 14;
pub const RT5616_PWR_RM_L: u32 = 0x1 << 11;
pub const RT5616_PWR_RM_L_BIT: u32 = 11;
pub const RT5616_PWR_RM_R: u32 = 0x1 << 10;
pub const RT5616_PWR_RM_R_BIT: u32 = 10;

/* Power Management for Volume (0x66) */
pub const RT5616_PWR_OV_L: u32 = 0x1 << 13;
pub const RT5616_PWR_OV_L_BIT: u32 = 13;
pub const RT5616_PWR_OV_R: u32 = 0x1 << 12;
pub const RT5616_PWR_OV_R_BIT: u32 = 12;
pub const RT5616_PWR_HV_L: u32 = 0x1 << 11;
pub const RT5616_PWR_HV_L_BIT: u32 = 11;
pub const RT5616_PWR_HV_R: u32 = 0x1 << 10;
pub const RT5616_PWR_HV_R_BIT: u32 = 10;
pub const RT5616_PWR_IN1_L: u32 = 0x1 << 9;
pub const RT5616_PWR_IN1_L_BIT: u32 = 9;
pub const RT5616_PWR_IN1_R: u32 = 0x1 << 8;
pub const RT5616_PWR_IN1_R_BIT: u32 = 8;
pub const RT5616_PWR_IN2_L: u32 = 0x1 << 7;
pub const RT5616_PWR_IN2_L_BIT: u32 = 7;
pub const RT5616_PWR_IN2_R: u32 = 0x1 << 6;
pub const RT5616_PWR_IN2_R_BIT: u32 = 6;

/* I2S1/2/3 Audio Serial Data Port Control (0x70 0x71) */
pub const RT5616_I2S_MS_MASK: u32 = 0x1 << 15;
pub const RT5616_I2S_MS_SFT: u32 = 15;
pub const RT5616_I2S_MS_M: u32 = 0x0 << 15;
pub const RT5616_I2S_MS_S: u32 = 0x1 << 15;
pub const RT5616_I2S_O_CP_MASK: u32 = 0x3 << 10;
pub const RT5616_I2S_O_CP_SFT: u32 = 10;
pub const RT5616_I2S_O_CP_OFF: u32 = 0x0 << 10;
pub const RT5616_I2S_O_CP_U_LAW: u32 = 0x1 << 10;
pub const RT5616_I2S_O_CP_A_LAW: u32 = 0x2 << 10;
pub const RT5616_I2S_I_CP_MASK: u32 = 0x3 << 8;
pub const RT5616_I2S_I_CP_SFT: u32 = 8;
pub const RT5616_I2S_I_CP_OFF: u32 = 0x0 << 8;
pub const RT5616_I2S_I_CP_U_LAW: u32 = 0x1 << 8;
pub const RT5616_I2S_I_CP_A_LAW: u32 = 0x2 << 8;
pub const RT5616_I2S_BP_MASK: u32 = 0x1 << 7;
pub const RT5616_I2S_BP_SFT: u32 = 7;
pub const RT5616_I2S_BP_NOR: u32 = 0x0 << 7;
pub const RT5616_I2S_BP_INV: u32 = 0x1 << 7;
pub const RT5616_I2S_DL_MASK: u32 = 0x3 << 2;
pub const RT5616_I2S_DL_SFT: u32 = 2;
pub const RT5616_I2S_DL_16: u32 = 0x0 << 2;
pub const RT5616_I2S_DL_20: u32 = 0x1 << 2;
pub const RT5616_I2S_DL_24: u32 = 0x2 << 2;
pub const RT5616_I2S_DL_8: u32 = 0x3 << 2;
pub const RT5616_I2S_DF_MASK: u32 = 0x3;
pub const RT5616_I2S_DF_SFT: u32 = 0;
pub const RT5616_I2S_DF_I2S: u32 = 0x0;
pub const RT5616_I2S_DF_LEFT: u32 = 0x1;
pub const RT5616_I2S_DF_PCM_A: u32 = 0x2;
pub const RT5616_I2S_DF_PCM_B: u32 = 0x3;

/* ADC/DAC Clock Control 1 (0x73) */
pub const RT5616_I2S_PD1_MASK: u32 = 0x7 << 12;
pub const RT5616_I2S_PD1_SFT: u32 = 12;
pub const RT5616_I2S_PD1_1: u32 = 0x0 << 12;
pub const RT5616_I2S_PD1_2: u32 = 0x1 << 12;
pub const RT5616_I2S_PD1_3: u32 = 0x2 << 12;
pub const RT5616_I2S_PD1_4: u32 = 0x3 << 12;
pub const RT5616_I2S_PD1_6: u32 = 0x4 << 12;
pub const RT5616_I2S_PD1_8: u32 = 0x5 << 12;
pub const RT5616_I2S_PD1_12: u32 = 0x6 << 12;
pub const RT5616_I2S_PD1_16: u32 = 0x7 << 12;
pub const RT5616_I2S_BCLK_MS2_MASK: u32 = 0x1 << 11;
pub const RT5616_DAC_OSR_MASK: u32 = 0x3 << 2;
pub const RT5616_DAC_OSR_SFT: u32 = 2;
pub const RT5616_DAC_OSR_128: u32 = 0x0 << 2;
pub const RT5616_DAC_OSR_64: u32 = 0x1 << 2;
pub const RT5616_DAC_OSR_32: u32 = 0x2 << 2;
pub const RT5616_DAC_OSR_128_3: u32 = 0x3 << 2;
pub const RT5616_ADC_OSR_MASK: u32 = 0x3;
pub const RT5616_ADC_OSR_SFT: u32 = 0;
pub const RT5616_ADC_OSR_128: u32 = 0x0;
pub const RT5616_ADC_OSR_64: u32 = 0x1;
pub const RT5616_ADC_OSR_32: u32 = 0x2;
pub const RT5616_ADC_OSR_128_3: u32 = 0x3;

/* ADC/DAC Clock Control 2 (0x74) */
pub const RT5616_DAHPF_EN: u32 = 0x1 << 11;
pub const RT5616_DAHPF_EN_SFT: u32 = 11;
pub const RT5616_ADHPF_EN: u32 = 0x1 << 10;
pub const RT5616_ADHPF_EN_SFT: u32 = 10;

/* TDM Control 1 (0x77) */
pub const RT5616_TDM_INTEL_SEL_MASK: u32 = 0x1 << 15;
pub const RT5616_TDM_INTEL_SEL_SFT: u32 = 15;
pub const RT5616_TDM_INTEL_SEL_64: u32 = 0x0 << 15;
pub const RT5616_TDM_INTEL_SEL_50: u32 = 0x1 << 15;
pub const RT5616_TDM_MODE_SEL_MASK: u32 = 0x1 << 14;
pub const RT5616_TDM_MODE_SEL_SFT: u32 = 14;
pub const RT5616_TDM_MODE_SEL_NOR: u32 = 0x0 << 14;
pub const RT5616_TDM_MODE_SEL_TDM: u32 = 0x1 << 14;
pub const RT5616_TDM_CH_NUM_SEL_MASK: u32 = 0x3 << 12;
pub const RT5616_TDM_CH_NUM_SEL_SFT: u32 = 12;
pub const RT5616_TDM_CH_NUM_SEL_2: u32 = 0x0 << 12;
pub const RT5616_TDM_CH_NUM_SEL_4: u32 = 0x1 << 12;
pub const RT5616_TDM_CH_NUM_SEL_6: u32 = 0x2 << 12;
pub const RT5616_TDM_CH_NUM_SEL_8: u32 = 0x3 << 12;
pub const RT5616_TDM_CH_LEN_SEL_MASK: u32 = 0x3 << 10;
pub const RT5616_TDM_CH_LEN_SEL_SFT: u32 = 10;
pub const RT5616_TDM_CH_LEN_SEL_16: u32 = 0x0 << 10;
pub const RT5616_TDM_CH_LEN_SEL_20: u32 = 0x1 << 10;
pub const RT5616_TDM_CH_LEN_SEL_24: u32 = 0x2 << 10;
pub const RT5616_TDM_CH_LEN_SEL_32: u32 = 0x3 << 10;
pub const RT5616_TDM_ADC_SEL_MASK: u32 = 0x1 << 9;
pub const RT5616_TDM_ADC_SEL_SFT: u32 = 9;
pub const RT5616_TDM_ADC_SEL_NOR: u32 = 0x0 << 9;
pub const RT5616_TDM_ADC_SEL_SWAP: u32 = 0x1 << 9;
pub const RT5616_TDM_ADC_START_SEL_MASK: u32 = 0x1 << 8;
pub const RT5616_TDM_ADC_START_SEL_SFT: u32 = 8;
pub const RT5616_TDM_ADC_START_SEL_SL0: u32 = 0x0 << 8;
pub const RT5616_TDM_ADC_START_SEL_SL4: u32 = 0x1 << 8;
pub const RT5616_TDM_I2S_CH2_SEL_MASK: u32 = 0x3 << 6;
pub const RT5616_TDM_I2S_CH2_SEL_SFT: u32 = 6;
pub const RT5616_TDM_I2S_CH2_SEL_LR: u32 = 0x0 << 6;
pub const RT5616_TDM_I2S_CH2_SEL_RL: u32 = 0x1 << 6;
pub const RT5616_TDM_I2S_CH2_SEL_LL: u32 = 0x2 << 6;
pub const RT5616_TDM_I2S_CH2_SEL_RR: u32 = 0x3 << 6;
pub const RT5616_TDM_I2S_CH4_SEL_MASK: u32 = 0x3 << 4;
pub const RT5616_TDM_I2S_CH4_SEL_SFT: u32 = 4;
pub const RT5616_TDM_I2S_CH4_SEL_LR: u32 = 0x0 << 4;
pub const RT5616_TDM_I2S_CH4_SEL_RL: u32 = 0x1 << 4;
pub const RT5616_TDM_I2S_CH4_SEL_LL: u32 = 0x2 << 4;
pub const RT5616_TDM_I2S_CH4_SEL_RR: u32 = 0x3 << 4;
pub const RT5616_TDM_I2S_CH6_SEL_MASK: u32 = 0x3 << 2;
pub const RT5616_TDM_I2S_CH6_SEL_SFT: u32 = 2;
pub const RT5616_TDM_I2S_CH6_SEL_LR: u32 = 0x0 << 2;
pub const RT5616_TDM_I2S_CH6_SEL_RL: u32 = 0x1 << 2;
pub const RT5616_TDM_I2S_CH6_SEL_LL: u32 = 0x2 << 2;
pub const RT5616_TDM_I2S_CH6_SEL_RR: u32 = 0x3 << 2;
pub const RT5616_TDM_I2S_CH8_SEL_MASK: u32 = 0x3;
pub const RT5616_TDM_I2S_CH8_SEL_SFT: u32 = 0;
pub const RT5616_TDM_I2S_CH8_SEL_LR: u32 = 0x0;
pub const RT5616_TDM_I2S_CH8_SEL_RL: u32 = 0x1;
pub const RT5616_TDM_I2S_CH8_SEL_LL: u32 = 0x2;
pub const RT5616_TDM_I2S_CH8_SEL_RR: u32 = 0x3;

/* TDM Control 2 (0x78) */
pub const RT5616_TDM_LRCK_POL_SEL_MASK: u32 = 0x1 << 15;
pub const RT5616_TDM_LRCK_POL_SEL_SFT: u32 = 15;
pub const RT5616_TDM_LRCK_POL_SEL_NOR: u32 = 0x0 << 15;
pub const RT5616_TDM_LRCK_POL_SEL_INV: u32 = 0x1 << 15;
pub const RT5616_TDM_CH_VAL_SEL_MASK: u32 = 0x1 << 14;
pub const RT5616_TDM_CH_VAL_SEL_SFT: u32 = 14;
pub const RT5616_TDM_CH_VAL_SEL_CH01: u32 = 0x0 << 14;
pub const RT5616_TDM_CH_VAL_SEL_CH0123: u32 = 0x1 << 14;
pub const RT5616_TDM_CH_VAL_EN: u32 = 0x1 << 13;
pub const RT5616_TDM_CH_VAL_SFT: u32 = 13;
pub const RT5616_TDM_LPBK_EN: u32 = 0x1 << 12;
pub const RT5616_TDM_LPBK_SFT: u32 = 12;
pub const RT5616_TDM_LRCK_PULSE_SEL_MASK: u32 = 0x1 << 11;
pub const RT5616_TDM_LRCK_PULSE_SEL_SFT: u32 = 11;
pub const RT5616_TDM_LRCK_PULSE_SEL_BCLK: u32 = 0x0 << 11;
pub const RT5616_TDM_LRCK_PULSE_SEL_CH: u32 = 0x1 << 11;
pub const RT5616_TDM_END_EDGE_SEL_MASK: u32 = 0x1 << 10;
pub const RT5616_TDM_END_EDGE_SEL_SFT: u32 = 10;
pub const RT5616_TDM_END_EDGE_SEL_POS: u32 = 0x0 << 10;
pub const RT5616_TDM_END_EDGE_SEL_NEG: u32 = 0x1 << 10;
pub const RT5616_TDM_END_EDGE_EN: u32 = 0x1 << 9;
pub const RT5616_TDM_END_EDGE_EN_SFT: u32 = 9;
pub const RT5616_TDM_TRAN_EDGE_SEL_MASK: u32 = 0x1 << 8;
pub const RT5616_TDM_TRAN_EDGE_SEL_SFT: u32 = 8;
pub const RT5616_TDM_TRAN_EDGE_SEL_POS: u32 = 0x0 << 8;
pub const RT5616_TDM_TRAN_EDGE_SEL_NEG: u32 = 0x1 << 8;
pub const RT5616_M_TDM2_L: u32 = 0x1 << 7;
pub const RT5616_M_TDM2_L_SFT: u32 = 7;
pub const RT5616_M_TDM2_R: u32 = 0x1 << 6;
pub const RT5616_M_TDM2_R_SFT: u32 = 6;
pub const RT5616_M_TDM4_L: u32 = 0x1 << 5;
pub const RT5616_M_TDM4_L_SFT: u32 = 5;
pub const RT5616_M_TDM4_R: u32 = 0x1 << 4;
pub const RT5616_M_TDM4_R_SFT: u32 = 4;

/* Global Clock Control (0x80) */
pub const RT5616_SCLK_SRC_MASK: u32 = 0x3 << 14;
pub const RT5616_SCLK_SRC_SFT: u32 = 14;
pub const RT5616_SCLK_SRC_MCLK: u32 = 0x0 << 14;
pub const RT5616_SCLK_SRC_PLL1: u32 = 0x1 << 14;
pub const RT5616_PLL1_SRC_MASK: u32 = 0x3 << 12;
pub const RT5616_PLL1_SRC_SFT: u32 = 12;
pub const RT5616_PLL1_SRC_MCLK: u32 = 0x0 << 12;
pub const RT5616_PLL1_SRC_BCLK1: u32 = 0x1 << 12;
pub const RT5616_PLL1_SRC_BCLK2: u32 = 0x2 << 12;
pub const RT5616_PLL1_PD_MASK: u32 = 0x1 << 3;
pub const RT5616_PLL1_PD_SFT: u32 = 3;
pub const RT5616_PLL1_PD_1: u32 = 0x0 << 3;
pub const RT5616_PLL1_PD_2: u32 = 0x1 << 3;

pub const RT5616_PLL_INP_MAX: u32 = 40000000;
pub const RT5616_PLL_INP_MIN: u32 = 256000;
/* PLL M/N/K Code Control 1 (0x81) */
pub const RT5616_PLL_N_MAX: u32 = 0x1ff;
pub const RT5616_PLL_N_MASK: u32 = RT5616_PLL_N_MAX << 7;
pub const RT5616_PLL_N_SFT: u32 = 7;
pub const RT5616_PLL_K_MAX: u32 = 0x1f;
pub const RT5616_PLL_K_MASK: u32 = RT5616_PLL_K_MAX;
pub const RT5616_PLL_K_SFT: u32 = 0;

/* PLL M/N/K Code Control 2 (0x82) */
pub const RT5616_PLL_M_MAX: u32 = 0xf;
pub const RT5616_PLL_M_MASK: u32 = RT5616_PLL_M_MAX << 12;
pub const RT5616_PLL_M_SFT: u32 = 12;
pub const RT5616_PLL_M_BP: u32 = 0x1 << 11;
pub const RT5616_PLL_M_BP_SFT: u32 = 11;

/* PLL tracking mode 1 (0x83) */
pub const RT5616_STO1_T_MASK: u32 = 0x1 << 15;
pub const RT5616_STO1_T_SFT: u32 = 15;
pub const RT5616_STO1_T_SCLK: u32 = 0x0 << 15;
pub const RT5616_STO1_T_LRCK1: u32 = 0x1 << 15;
pub const RT5616_STO2_T_MASK: u32 = 0x1 << 12;
pub const RT5616_STO2_T_SFT: u32 = 12;
pub const RT5616_STO2_T_I2S2: u32 = 0x0 << 12;
pub const RT5616_STO2_T_LRCK2: u32 = 0x1 << 12;
pub const RT5616_ASRC2_REF_MASK: u32 = 0x1 << 11;
pub const RT5616_ASRC2_REF_SFT: u32 = 11;
pub const RT5616_ASRC2_REF_LRCK2: u32 = 0x0 << 11;
pub const RT5616_ASRC2_REF_LRCK1: u32 = 0x1 << 11;
pub const RT5616_DMIC_1_M_MASK: u32 = 0x1 << 9;
pub const RT5616_DMIC_1_M_SFT: u32 = 9;
pub const RT5616_DMIC_1_M_NOR: u32 = 0x0 << 9;
pub const RT5616_DMIC_1_M_ASYN: u32 = 0x1 << 9;

/* PLL tracking mode 2 (0x84) */
pub const RT5616_STO1_ASRC_EN: u32 = 0x1 << 15;
pub const RT5616_STO1_ASRC_EN_SFT: u32 = 15;
pub const RT5616_STO2_ASRC_EN: u32 = 0x1 << 14;
pub const RT5616_STO2_ASRC_EN_SFT: u32 = 14;
pub const RT5616_STO1_DAC_M_MASK: u32 = 0x1 << 13;
pub const RT5616_STO1_DAC_M_SFT: u32 = 13;
pub const RT5616_STO1_DAC_M_NOR: u32 = 0x0 << 13;
pub const RT5616_STO1_DAC_M_ASRC: u32 = 0x1 << 13;
pub const RT5616_STO2_DAC_M_MASK: u32 = 0x1 << 12;
pub const RT5616_STO2_DAC_M_SFT: u32 = 12;
pub const RT5616_STO2_DAC_M_NOR: u32 = 0x0 << 12;
pub const RT5616_STO2_DAC_M_ASRC: u32 = 0x1 << 12;
pub const RT5616_ADC_M_MASK: u32 = 0x1 << 11;
pub const RT5616_ADC_M_SFT: u32 = 11;
pub const RT5616_ADC_M_NOR: u32 = 0x0 << 11;
pub const RT5616_ADC_M_ASRC: u32 = 0x1 << 11;
pub const RT5616_I2S1_R_D_MASK: u32 = 0x1 << 4;
pub const RT5616_I2S1_R_D_SFT: u32 = 4;
pub const RT5616_I2S1_R_D_DIS: u32 = 0x0 << 4;
pub const RT5616_I2S1_R_D_EN: u32 = 0x1 << 4;
pub const RT5616_I2S2_R_D_MASK: u32 = 0x1 << 3;
pub const RT5616_I2S2_R_D_SFT: u32 = 3;
pub const RT5616_I2S2_R_D_DIS: u32 = 0x0 << 3;
pub const RT5616_I2S2_R_D_EN: u32 = 0x1 << 3;
pub const RT5616_PRE_SCLK_MASK: u32 = 0x3;
pub const RT5616_PRE_SCLK_SFT: u32 = 0;
pub const RT5616_PRE_SCLK_512: u32 = 0x0;
pub const RT5616_PRE_SCLK_1024: u32 = 0x1;
pub const RT5616_PRE_SCLK_2048: u32 = 0x2;

/* PLL tracking mode 3 (0x85) */
pub const RT5616_I2S1_RATE_MASK: u32 = 0xf << 12;
pub const RT5616_I2S1_RATE_SFT: u32 = 12;
pub const RT5616_I2S2_RATE_MASK: u32 = 0xf << 8;
pub const RT5616_I2S2_RATE_SFT: u32 = 8;
pub const RT5616_G_ASRC_LP_MASK: u32 = 0x1 << 3;
pub const RT5616_G_ASRC_LP_SFT: u32 = 3;
pub const RT5616_ASRC_LP_F_M: u32 = 0x1 << 2;
pub const RT5616_ASRC_LP_F_SFT: u32 = 2;
pub const RT5616_ASRC_LP_F_NOR: u32 = 0x0 << 2;
pub const RT5616_ASRC_LP_F_SB: u32 = 0x1 << 2;
pub const RT5616_FTK_PH_DET_MASK: u32 = 0x3;
pub const RT5616_FTK_PH_DET_SFT: u32 = 0;
pub const RT5616_FTK_PH_DET_DIV1: u32 = 0x0;
pub const RT5616_FTK_PH_DET_DIV2: u32 = 0x1;
pub const RT5616_FTK_PH_DET_DIV4: u32 = 0x2;
pub const RT5616_FTK_PH_DET_DIV8: u32 = 0x3;

/*PLL tracking mode 6 (0x89) */
pub const RT5616_I2S1_PD_MASK: u32 = 0x7 << 12;
pub const RT5616_I2S1_PD_SFT: u32 = 12;
pub const RT5616_I2S2_PD_MASK: u32 = 0x7 << 8;
pub const RT5616_I2S2_PD_SFT: u32 = 8;

/*PLL tracking mode 7 (0x8a) */
pub const RT5616_FSI1_RATE_MASK: u32 = 0xf << 12;
pub const RT5616_FSI1_RATE_SFT: u32 = 12;
pub const RT5616_FSI2_RATE_MASK: u32 = 0xf << 8;
pub const RT5616_FSI2_RATE_SFT: u32 = 8;

/* HPOUT Over Current Detection (0x8b) */
pub const RT5616_HP_OVCD_MASK: u32 = 0x1 << 10;
pub const RT5616_HP_OVCD_SFT: u32 = 10;
pub const RT5616_HP_OVCD_DIS: u32 = 0x0 << 10;
pub const RT5616_HP_OVCD_EN: u32 = 0x1 << 10;
pub const RT5616_HP_OC_TH_MASK: u32 = 0x3 << 8;
pub const RT5616_HP_OC_TH_SFT: u32 = 8;
pub const RT5616_HP_OC_TH_90: u32 = 0x0 << 8;
pub const RT5616_HP_OC_TH_105: u32 = 0x1 << 8;
pub const RT5616_HP_OC_TH_120: u32 = 0x2 << 8;
pub const RT5616_HP_OC_TH_135: u32 = 0x3 << 8;

/* Depop Mode Control 1 (0x8e) */
pub const RT5616_SMT_TRIG_MASK: u32 = 0x1 << 15;
pub const RT5616_SMT_TRIG_SFT: u32 = 15;
pub const RT5616_SMT_TRIG_DIS: u32 = 0x0 << 15;
pub const RT5616_SMT_TRIG_EN: u32 = 0x1 << 15;
pub const RT5616_HP_L_SMT_MASK: u32 = 0x1 << 9;
pub const RT5616_HP_L_SMT_SFT: u32 = 9;
pub const RT5616_HP_L_SMT_DIS: u32 = 0x0 << 9;
pub const RT5616_HP_L_SMT_EN: u32 = 0x1 << 9;
pub const RT5616_HP_R_SMT_MASK: u32 = 0x1 << 8;
pub const RT5616_HP_R_SMT_SFT: u32 = 8;
pub const RT5616_HP_R_SMT_DIS: u32 = 0x0 << 8;
pub const RT5616_HP_R_SMT_EN: u32 = 0x1 << 8;
pub const RT5616_HP_CD_PD_MASK: u32 = 0x1 << 7;
pub const RT5616_HP_CD_PD_SFT: u32 = 7;
pub const RT5616_HP_CD_PD_DIS: u32 = 0x0 << 7;
pub const RT5616_HP_CD_PD_EN: u32 = 0x1 << 7;
pub const RT5616_RSTN_MASK: u32 = 0x1 << 6;
pub const RT5616_RSTN_SFT: u32 = 6;
pub const RT5616_RSTN_DIS: u32 = 0x0 << 6;
pub const RT5616_RSTN_EN: u32 = 0x1 << 6;
pub const RT5616_RSTP_MASK: u32 = 0x1 << 5;
pub const RT5616_RSTP_SFT: u32 = 5;
pub const RT5616_RSTP_DIS: u32 = 0x0 << 5;
pub const RT5616_RSTP_EN: u32 = 0x1 << 5;
pub const RT5616_HP_CO_MASK: u32 = 0x1 << 4;
pub const RT5616_HP_CO_SFT: u32 = 4;
pub const RT5616_HP_CO_DIS: u32 = 0x0 << 4;
pub const RT5616_HP_CO_EN: u32 = 0x1 << 4;
pub const RT5616_HP_CP_MASK: u32 = 0x1 << 3;
pub const RT5616_HP_CP_SFT: u32 = 3;
pub const RT5616_HP_CP_PD: u32 = 0x0 << 3;
pub const RT5616_HP_CP_PU: u32 = 0x1 << 3;
pub const RT5616_HP_SG_MASK: u32 = 0x1 << 2;
pub const RT5616_HP_SG_SFT: u32 = 2;
pub const RT5616_HP_SG_DIS: u32 = 0x0 << 2;
pub const RT5616_HP_SG_EN: u32 = 0x1 << 2;
pub const RT5616_HP_DP_MASK: u32 = 0x1 << 1;
pub const RT5616_HP_DP_SFT: u32 = 1;
pub const RT5616_HP_DP_PD: u32 = 0x0 << 1;
pub const RT5616_HP_DP_PU: u32 = 0x1 << 1;
pub const RT5616_HP_CB_MASK: u32 = 0x1;
pub const RT5616_HP_CB_SFT: u32 = 0;
pub const RT5616_HP_CB_PD: u32 = 0x0;
pub const RT5616_HP_CB_PU: u32 = 0x1;

/* Depop Mode Control 2 (0x8f) */
pub const RT5616_DEPOP_MASK: u32 = 0x1 << 13;
pub const RT5616_DEPOP_SFT: u32 = 13;
pub const RT5616_DEPOP_AUTO: u32 = 0x0 << 13;
pub const RT5616_DEPOP_MAN: u32 = 0x1 << 13;
pub const RT5616_RAMP_MASK: u32 = 0x1 << 12;
pub const RT5616_RAMP_SFT: u32 = 12;
pub const RT5616_RAMP_DIS: u32 = 0x0 << 12;
pub const RT5616_RAMP_EN: u32 = 0x1 << 12;
pub const RT5616_BPS_MASK: u32 = 0x1 << 11;
pub const RT5616_BPS_SFT: u32 = 11;
pub const RT5616_BPS_DIS: u32 = 0x0 << 11;
pub const RT5616_BPS_EN: u32 = 0x1 << 11;
pub const RT5616_FAST_UPDN_MASK: u32 = 0x1 << 10;
pub const RT5616_FAST_UPDN_SFT: u32 = 10;
pub const RT5616_FAST_UPDN_DIS: u32 = 0x0 << 10;
pub const RT5616_FAST_UPDN_EN: u32 = 0x1 << 10;
pub const RT5616_MRES_MASK: u32 = 0x3 << 8;
pub const RT5616_MRES_SFT: u32 = 8;
pub const RT5616_MRES_15MO: u32 = 0x0 << 8;
pub const RT5616_MRES_25MO: u32 = 0x1 << 8;
pub const RT5616_MRES_35MO: u32 = 0x2 << 8;
pub const RT5616_MRES_45MO: u32 = 0x3 << 8;
pub const RT5616_VLO_MASK: u32 = 0x1 << 7;
pub const RT5616_VLO_SFT: u32 = 7;
pub const RT5616_VLO_3V: u32 = 0x0 << 7;
pub const RT5616_VLO_32V: u32 = 0x1 << 7;
pub const RT5616_DIG_DP_MASK: u32 = 0x1 << 6;
pub const RT5616_DIG_DP_SFT: u32 = 6;
pub const RT5616_DIG_DP_DIS: u32 = 0x0 << 6;
pub const RT5616_DIG_DP_EN: u32 = 0x1 << 6;
pub const RT5616_DP_TH_MASK: u32 = 0x3 << 4;
pub const RT5616_DP_TH_SFT: u32 = 4;

/* Depop Mode Control 3 (0x90) */
pub const RT5616_CP_SYS_MASK: u32 = 0x7 << 12;
pub const RT5616_CP_SYS_SFT: u32 = 12;
pub const RT5616_CP_FQ1_MASK: u32 = 0x7 << 8;
pub const RT5616_CP_FQ1_SFT: u32 = 8;
pub const RT5616_CP_FQ2_MASK: u32 = 0x7 << 4;
pub const RT5616_CP_FQ2_SFT: u32 = 4;
pub const RT5616_CP_FQ3_MASK: u32 = 0x7;
pub const RT5616_CP_FQ3_SFT: u32 = 0;
pub const RT5616_CP_FQ_1_5_KHZ: u32 = 0;
pub const RT5616_CP_FQ_3_KHZ: u32 = 1;
pub const RT5616_CP_FQ_6_KHZ: u32 = 2;
pub const RT5616_CP_FQ_12_KHZ: u32 = 3;
pub const RT5616_CP_FQ_24_KHZ: u32 = 4;
pub const RT5616_CP_FQ_48_KHZ: u32 = 5;
pub const RT5616_CP_FQ_96_KHZ: u32 = 6;
pub const RT5616_CP_FQ_192_KHZ: u32 = 7;

/* HPOUT charge pump (0x91) */
pub const RT5616_OSW_L_MASK: u32 = 0x1 << 11;
pub const RT5616_OSW_L_SFT: u32 = 11;
pub const RT5616_OSW_L_DIS: u32 = 0x0 << 11;
pub const RT5616_OSW_L_EN: u32 = 0x1 << 11;
pub const RT5616_OSW_R_MASK: u32 = 0x1 << 10;
pub const RT5616_OSW_R_SFT: u32 = 10;
pub const RT5616_OSW_R_DIS: u32 = 0x0 << 10;
pub const RT5616_OSW_R_EN: u32 = 0x1 << 10;
pub const RT5616_PM_HP_MASK: u32 = 0x3 << 8;
pub const RT5616_PM_HP_SFT: u32 = 8;
pub const RT5616_PM_HP_LV: u32 = 0x0 << 8;
pub const RT5616_PM_HP_MV: u32 = 0x1 << 8;
pub const RT5616_PM_HP_HV: u32 = 0x2 << 8;
pub const RT5616_IB_HP_MASK: u32 = 0x3 << 6;
pub const RT5616_IB_HP_SFT: u32 = 6;
pub const RT5616_IB_HP_125IL: u32 = 0x0 << 6;
pub const RT5616_IB_HP_25IL: u32 = 0x1 << 6;
pub const RT5616_IB_HP_5IL: u32 = 0x2 << 6;
pub const RT5616_IB_HP_1IL: u32 = 0x3 << 6;

/* Micbias Control (0x93) */
pub const RT5616_MIC1_BS_MASK: u32 = 0x1 << 15;
pub const RT5616_MIC1_BS_SFT: u32 = 15;
pub const RT5616_MIC1_BS_9AV: u32 = 0x0 << 15;
pub const RT5616_MIC1_BS_75AV: u32 = 0x1 << 15;
pub const RT5616_MIC1_CLK_MASK: u32 = 0x1 << 13;
pub const RT5616_MIC1_CLK_SFT: u32 = 13;
pub const RT5616_MIC1_CLK_DIS: u32 = 0x0 << 13;
pub const RT5616_MIC1_CLK_EN: u32 = 0x1 << 13;
pub const RT5616_MIC1_OVCD_MASK: u32 = 0x1 << 11;
pub const RT5616_MIC1_OVCD_SFT: u32 = 11;
pub const RT5616_MIC1_OVCD_DIS: u32 = 0x0 << 11;
pub const RT5616_MIC1_OVCD_EN: u32 = 0x1 << 11;
pub const RT5616_MIC1_OVTH_MASK: u32 = 0x3 << 9;
pub const RT5616_MIC1_OVTH_SFT: u32 = 9;
pub const RT5616_MIC1_OVTH_600UA: u32 = 0x0 << 9;
pub const RT5616_MIC1_OVTH_1500UA: u32 = 0x1 << 9;
pub const RT5616_MIC1_OVTH_2000UA: u32 = 0x2 << 9;
pub const RT5616_PWR_MB_MASK: u32 = 0x1 << 5;
pub const RT5616_PWR_MB_SFT: u32 = 5;
pub const RT5616_PWR_MB_PD: u32 = 0x0 << 5;
pub const RT5616_PWR_MB_PU: u32 = 0x1 << 5;
pub const RT5616_PWR_CLK12M_MASK: u32 = 0x1 << 4;
pub const RT5616_PWR_CLK12M_SFT: u32 = 4;
pub const RT5616_PWR_CLK12M_PD: u32 = 0x0 << 4;
pub const RT5616_PWR_CLK12M_PU: u32 = 0x1 << 4;

/* Analog JD Control 1 (0x94) */
pub const RT5616_JD2_CMP_MASK: u32 = 0x7 << 12;
pub const RT5616_JD2_CMP_SFT: u32 = 12;
pub const RT5616_JD_PU: u32 = 0x1 << 11;
pub const RT5616_JD_PU_SFT: u32 = 11;
pub const RT5616_JD_PD: u32 = 0x1 << 10;
pub const RT5616_JD_PD_SFT: u32 = 10;
pub const RT5616_JD_MODE_SEL_MASK: u32 = 0x3 << 8;
pub const RT5616_JD_MODE_SEL_SFT: u32 = 8;
pub const RT5616_JD_MODE_SEL_M0: u32 = 0x0 << 8;
pub const RT5616_JD_MODE_SEL_M1: u32 = 0x1 << 8;
pub const RT5616_JD_MODE_SEL_M2: u32 = 0x2 << 8;
pub const RT5616_JD_M_CMP: u32 = 0x7 << 4;
pub const RT5616_JD_M_CMP_SFT: u32 = 4;
pub const RT5616_JD_M_PU: u32 = 0x1 << 3;
pub const RT5616_JD_M_PU_SFT: u32 = 3;
pub const RT5616_JD_M_PD: u32 = 0x1 << 2;
pub const RT5616_JD_M_PD_SFT: u32 = 2;
pub const RT5616_JD_M_MODE_SEL_MASK: u32 = 0x3;
pub const RT5616_JD_M_MODE_SEL_SFT: u32 = 0;
pub const RT5616_JD_M_MODE_SEL_M0: u32 = 0x0;
pub const RT5616_JD_M_MODE_SEL_M1: u32 = 0x1;
pub const RT5616_JD_M_MODE_SEL_M2: u32 = 0x2;

/* Analog JD Control 2 (0x95) */
pub const RT5616_JD3_CMP_MASK: u32 = 0x7 << 12;
pub const RT5616_JD3_CMP_SFT: u32 = 12;

/* EQ Control 1 (0xb0) */
pub const RT5616_EQ_SRC_MASK: u32 = 0x1 << 15;
pub const RT5616_EQ_SRC_SFT: u32 = 15;
pub const RT5616_EQ_SRC_DAC: u32 = 0x0 << 15;
pub const RT5616_EQ_SRC_ADC: u32 = 0x1 << 15;
pub const RT5616_EQ_UPD: u32 = 0x1 << 14;
pub const RT5616_EQ_UPD_BIT: u32 = 14;
pub const RT5616_EQ_CD_MASK: u32 = 0x1 << 13;
pub const RT5616_EQ_CD_SFT: u32 = 13;
pub const RT5616_EQ_CD_DIS: u32 = 0x0 << 13;
pub const RT5616_EQ_CD_EN: u32 = 0x1 << 13;
pub const RT5616_EQ_DITH_MASK: u32 = 0x3 << 8;
pub const RT5616_EQ_DITH_SFT: u32 = 8;
pub const RT5616_EQ_DITH_NOR: u32 = 0x0 << 8;
pub const RT5616_EQ_DITH_LSB: u32 = 0x1 << 8;
pub const RT5616_EQ_DITH_LSB_1: u32 = 0x2 << 8;
pub const RT5616_EQ_DITH_LSB_2: u32 = 0x3 << 8;
pub const RT5616_EQ_CD_F: u32 = 0x1 << 7;
pub const RT5616_EQ_CD_F_BIT: u32 = 7;
pub const RT5616_EQ_STA_HP2: u32 = 0x1 << 6;
pub const RT5616_EQ_STA_HP2_BIT: u32 = 6;
pub const RT5616_EQ_STA_HP1: u32 = 0x1 << 5;
pub const RT5616_EQ_STA_HP1_BIT: u32 = 5;
pub const RT5616_EQ_STA_BP4: u32 = 0x1 << 4;
pub const RT5616_EQ_STA_BP4_BIT: u32 = 4;
pub const RT5616_EQ_STA_BP3: u32 = 0x1 << 3;
pub const RT5616_EQ_STA_BP3_BIT: u32 = 3;
pub const RT5616_EQ_STA_BP2: u32 = 0x1 << 2;
pub const RT5616_EQ_STA_BP2_BIT: u32 = 2;
pub const RT5616_EQ_STA_BP1: u32 = 0x1 << 1;
pub const RT5616_EQ_STA_BP1_BIT: u32 = 1;
pub const RT5616_EQ_STA_LP: u32 = 0x1;
pub const RT5616_EQ_STA_LP_BIT: u32 = 0;

/* EQ Control 2 (0xb1) */
pub const RT5616_EQ_HPF1_M_MASK: u32 = 0x1 << 8;
pub const RT5616_EQ_HPF1_M_SFT: u32 = 8;
pub const RT5616_EQ_HPF1_M_HI: u32 = 0x0 << 8;
pub const RT5616_EQ_HPF1_M_1ST: u32 = 0x1 << 8;
pub const RT5616_EQ_LPF1_M_MASK: u32 = 0x1 << 7;
pub const RT5616_EQ_LPF1_M_SFT: u32 = 7;
pub const RT5616_EQ_LPF1_M_LO: u32 = 0x0 << 7;
pub const RT5616_EQ_LPF1_M_1ST: u32 = 0x1 << 7;
pub const RT5616_EQ_HPF2_MASK: u32 = 0x1 << 6;
pub const RT5616_EQ_HPF2_SFT: u32 = 6;
pub const RT5616_EQ_HPF2_DIS: u32 = 0x0 << 6;
pub const RT5616_EQ_HPF2_EN: u32 = 0x1 << 6;
pub const RT5616_EQ_HPF1_MASK: u32 = 0x1 << 5;
pub const RT5616_EQ_HPF1_SFT: u32 = 5;
pub const RT5616_EQ_HPF1_DIS: u32 = 0x0 << 5;
pub const RT5616_EQ_HPF1_EN: u32 = 0x1 << 5;
pub const RT5616_EQ_BPF4_MASK: u32 = 0x1 << 4;
pub const RT5616_EQ_BPF4_SFT: u32 = 4;
pub const RT5616_EQ_BPF4_DIS: u32 = 0x0 << 4;
pub const RT5616_EQ_BPF4_EN: u32 = 0x1 << 4;
pub const RT5616_EQ_BPF3_MASK: u32 = 0x1 << 3;
pub const RT5616_EQ_BPF3_SFT: u32 = 3;
pub const RT5616_EQ_BPF3_DIS: u32 = 0x0 << 3;
pub const RT5616_EQ_BPF3_EN: u32 = 0x1 << 3;
pub const RT5616_EQ_BPF2_MASK: u32 = 0x1 << 2;
pub const RT5616_EQ_BPF2_SFT: u32 = 2;
pub const RT5616_EQ_BPF2_DIS: u32 = 0x0 << 2;
pub const RT5616_EQ_BPF2_EN: u32 = 0x1 << 2;
pub const RT5616_EQ_BPF1_MASK: u32 = 0x1 << 1;
pub const RT5616_EQ_BPF1_SFT: u32 = 1;
pub const RT5616_EQ_BPF1_DIS: u32 = 0x0 << 1;
pub const RT5616_EQ_BPF1_EN: u32 = 0x1 << 1;
pub const RT5616_EQ_LPF_MASK: u32 = 0x1;
pub const RT5616_EQ_LPF_SFT: u32 = 0;
pub const RT5616_EQ_LPF_DIS: u32 = 0x0;
pub const RT5616_EQ_LPF_EN: u32 = 0x1;
pub const RT5616_EQ_CTRL_MASK: u32 = 0x7f;

/* Memory Test (0xb2) */
pub const RT5616_MT_MASK: u32 = 0x1 << 15;
pub const RT5616_MT_SFT: u32 = 15;
pub const RT5616_MT_DIS: u32 = 0x0 << 15;
pub const RT5616_MT_EN: u32 = 0x1 << 15;

/* DRC/AGC Control 1 (0xb4) */
pub const RT5616_DRC_AGC_P_MASK: u32 = 0x1 << 15;
pub const RT5616_DRC_AGC_P_SFT: u32 = 15;
pub const RT5616_DRC_AGC_P_DAC: u32 = 0x0 << 15;
pub const RT5616_DRC_AGC_P_ADC: u32 = 0x1 << 15;
pub const RT5616_DRC_AGC_MASK: u32 = 0x1 << 14;
pub const RT5616_DRC_AGC_SFT: u32 = 14;
pub const RT5616_DRC_AGC_DIS: u32 = 0x0 << 14;
pub const RT5616_DRC_AGC_EN: u32 = 0x1 << 14;
pub const RT5616_DRC_AGC_UPD: u32 = 0x1 << 13;
pub const RT5616_DRC_AGC_UPD_BIT: u32 = 13;
pub const RT5616_DRC_AGC_AR_MASK: u32 = 0x1f << 8;
pub const RT5616_DRC_AGC_AR_SFT: u32 = 8;
pub const RT5616_DRC_AGC_R_MASK: u32 = 0x7 << 5;
pub const RT5616_DRC_AGC_R_SFT: u32 = 5;
pub const RT5616_DRC_AGC_R_48K: u32 = 0x1 << 5;
pub const RT5616_DRC_AGC_R_96K: u32 = 0x2 << 5;
pub const RT5616_DRC_AGC_R_192K: u32 = 0x3 << 5;
pub const RT5616_DRC_AGC_R_441K: u32 = 0x5 << 5;
pub const RT5616_DRC_AGC_R_882K: u32 = 0x6 << 5;
pub const RT5616_DRC_AGC_R_1764K: u32 = 0x7 << 5;
pub const RT5616_DRC_AGC_RC_MASK: u32 = 0x1f;
pub const RT5616_DRC_AGC_RC_SFT: u32 = 0;

/* DRC/AGC Control 2 (0xb5) */
pub const RT5616_DRC_AGC_POB_MASK: u32 = 0x3f << 8;
pub const RT5616_DRC_AGC_POB_SFT: u32 = 8;
pub const RT5616_DRC_AGC_CP_MASK: u32 = 0x1 << 7;
pub const RT5616_DRC_AGC_CP_SFT: u32 = 7;
pub const RT5616_DRC_AGC_CP_DIS: u32 = 0x0 << 7;
pub const RT5616_DRC_AGC_CP_EN: u32 = 0x1 << 7;
pub const RT5616_DRC_AGC_CPR_MASK: u32 = 0x3 << 5;
pub const RT5616_DRC_AGC_CPR_SFT: u32 = 5;
pub const RT5616_DRC_AGC_CPR_1_1: u32 = 0x0 << 5;
pub const RT5616_DRC_AGC_CPR_1_2: u32 = 0x1 << 5;
pub const RT5616_DRC_AGC_CPR_1_3: u32 = 0x2 << 5;
pub const RT5616_DRC_AGC_CPR_1_4: u32 = 0x3 << 5;
pub const RT5616_DRC_AGC_PRB_MASK: u32 = 0x1f;
pub const RT5616_DRC_AGC_PRB_SFT: u32 = 0;

/* DRC/AGC Control 3 (0xb6) */
pub const RT5616_DRC_AGC_NGB_MASK: u32 = 0xf << 12;
pub const RT5616_DRC_AGC_NGB_SFT: u32 = 12;
pub const RT5616_DRC_AGC_TAR_MASK: u32 = 0x1f << 7;
pub const RT5616_DRC_AGC_TAR_SFT: u32 = 7;
pub const RT5616_DRC_AGC_NG_MASK: u32 = 0x1 << 6;
pub const RT5616_DRC_AGC_NG_SFT: u32 = 6;
pub const RT5616_DRC_AGC_NG_DIS: u32 = 0x0 << 6;
pub const RT5616_DRC_AGC_NG_EN: u32 = 0x1 << 6;
pub const RT5616_DRC_AGC_NGH_MASK: u32 = 0x1 << 5;
pub const RT5616_DRC_AGC_NGH_SFT: u32 = 5;
pub const RT5616_DRC_AGC_NGH_DIS: u32 = 0x0 << 5;
pub const RT5616_DRC_AGC_NGH_EN: u32 = 0x1 << 5;
pub const RT5616_DRC_AGC_NGT_MASK: u32 = 0x1f;
pub const RT5616_DRC_AGC_NGT_SFT: u32 = 0;

/* Jack Detect Control 1 (0xbb) */
pub const RT5616_JD_MASK: u32 = 0x7 << 13;
pub const RT5616_JD_SFT: u32 = 13;
pub const RT5616_JD_DIS: u32 = 0x0 << 13;
pub const RT5616_JD_GPIO1: u32 = 0x1 << 13;
pub const RT5616_JD_GPIO2: u32 = 0x2 << 13;
pub const RT5616_JD_GPIO3: u32 = 0x3 << 13;
pub const RT5616_JD_GPIO4: u32 = 0x4 << 13;
pub const RT5616_JD_GPIO5: u32 = 0x5 << 13;
pub const RT5616_JD_GPIO6: u32 = 0x6 << 13;
pub const RT5616_JD_HP_MASK: u32 = 0x1 << 11;
pub const RT5616_JD_HP_SFT: u32 = 11;
pub const RT5616_JD_HP_DIS: u32 = 0x0 << 11;
pub const RT5616_JD_HP_EN: u32 = 0x1 << 11;
pub const RT5616_JD_HP_TRG_MASK: u32 = 0x1 << 10;
pub const RT5616_JD_HP_TRG_SFT: u32 = 10;
pub const RT5616_JD_HP_TRG_LO: u32 = 0x0 << 10;
pub const RT5616_JD_HP_TRG_HI: u32 = 0x1 << 10;
pub const RT5616_JD_SPL_MASK: u32 = 0x1 << 9;
pub const RT5616_JD_SPL_SFT: u32 = 9;
pub const RT5616_JD_SPL_DIS: u32 = 0x0 << 9;
pub const RT5616_JD_SPL_EN: u32 = 0x1 << 9;
pub const RT5616_JD_SPL_TRG_MASK: u32 = 0x1 << 8;
pub const RT5616_JD_SPL_TRG_SFT: u32 = 8;
pub const RT5616_JD_SPL_TRG_LO: u32 = 0x0 << 8;
pub const RT5616_JD_SPL_TRG_HI: u32 = 0x1 << 8;
pub const RT5616_JD_SPR_MASK: u32 = 0x1 << 7;
pub const RT5616_JD_SPR_SFT: u32 = 7;
pub const RT5616_JD_SPR_DIS: u32 = 0x0 << 7;
pub const RT5616_JD_SPR_EN: u32 = 0x1 << 7;
pub const RT5616_JD_SPR_TRG_MASK: u32 = 0x1 << 6;
pub const RT5616_JD_SPR_TRG_SFT: u32 = 6;
pub const RT5616_JD_SPR_TRG_LO: u32 = 0x0 << 6;
pub const RT5616_JD_SPR_TRG_HI: u32 = 0x1 << 6;
pub const RT5616_JD_LO_MASK: u32 = 0x1 << 3;
pub const RT5616_JD_LO_SFT: u32 = 3;
pub const RT5616_JD_LO_DIS: u32 = 0x0 << 3;
pub const RT5616_JD_LO_EN: u32 = 0x1 << 3;
pub const RT5616_JD_LO_TRG_MASK: u32 = 0x1 << 2;
pub const RT5616_JD_LO_TRG_SFT: u32 = 2;
pub const RT5616_JD_LO_TRG_LO: u32 = 0x0 << 2;
pub const RT5616_JD_LO_TRG_HI: u32 = 0x1 << 2;

/* Jack Detect Control 2 (0xbc) */
pub const RT5616_JD_TRG_SEL_MASK: u32 = 0x7 << 9;
pub const RT5616_JD_TRG_SEL_SFT: u32 = 9;
pub const RT5616_JD_TRG_SEL_GPIO: u32 = 0x0 << 9;
pub const RT5616_JD_TRG_SEL_JD1_1: u32 = 0x1 << 9;
pub const RT5616_JD_TRG_SEL_JD1_2: u32 = 0x2 << 9;
pub const RT5616_JD_TRG_SEL_JD2: u32 = 0x3 << 9;
pub const RT5616_JD_TRG_SEL_JD3: u32 = 0x4 << 9;
pub const RT5616_JD3_IRQ_EN: u32 = 0x1 << 8;
pub const RT5616_JD3_IRQ_EN_SFT: u32 = 8;
pub const RT5616_JD3_EN_STKY: u32 = 0x1 << 7;
pub const RT5616_JD3_EN_STKY_SFT: u32 = 7;
pub const RT5616_JD3_INV: u32 = 0x1 << 6;
pub const RT5616_JD3_INV_SFT: u32 = 6;

/* IRQ Control 1 (0xbd) */
pub const RT5616_IRQ_JD_MASK: u32 = 0x1 << 15;
pub const RT5616_IRQ_JD_SFT: u32 = 15;
pub const RT5616_IRQ_JD_BP: u32 = 0x0 << 15;
pub const RT5616_IRQ_JD_NOR: u32 = 0x1 << 15;
pub const RT5616_JD_STKY_MASK: u32 = 0x1 << 13;
pub const RT5616_JD_STKY_SFT: u32 = 13;
pub const RT5616_JD_STKY_DIS: u32 = 0x0 << 13;
pub const RT5616_JD_STKY_EN: u32 = 0x1 << 13;
pub const RT5616_JD_P_MASK: u32 = 0x1 << 11;
pub const RT5616_JD_P_SFT: u32 = 11;
pub const RT5616_JD_P_NOR: u32 = 0x0 << 11;
pub const RT5616_JD_P_INV: u32 = 0x1 << 11;
pub const RT5616_JD1_1_IRQ_EN: u32 = 0x1 << 9;
pub const RT5616_JD1_1_IRQ_EN_SFT: u32 = 9;
pub const RT5616_JD1_1_EN_STKY: u32 = 0x1 << 8;
pub const RT5616_JD1_1_EN_STKY_SFT: u32 = 8;
pub const RT5616_JD1_1_INV: u32 = 0x1 << 7;
pub const RT5616_JD1_1_INV_SFT: u32 = 7;
pub const RT5616_JD1_2_IRQ_EN: u32 = 0x1 << 6;
pub const RT5616_JD1_2_IRQ_EN_SFT: u32 = 6;
pub const RT5616_JD1_2_EN_STKY: u32 = 0x1 << 5;
pub const RT5616_JD1_2_EN_STKY_SFT: u32 = 5;
pub const RT5616_JD1_2_INV: u32 = 0x1 << 4;
pub const RT5616_JD1_2_INV_SFT: u32 = 4;
pub const RT5616_JD2_IRQ_EN: u32 = 0x1 << 3;
pub const RT5616_JD2_IRQ_EN_SFT: u32 = 3;
pub const RT5616_JD2_EN_STKY: u32 = 0x1 << 2;
pub const RT5616_JD2_EN_STKY_SFT: u32 = 2;
pub const RT5616_JD2_INV: u32 = 0x1 << 1;
pub const RT5616_JD2_INV_SFT: u32 = 1;

/* IRQ Control 2 (0xbe) */
pub const RT5616_IRQ_MB1_OC_MASK: u32 = 0x1 << 15;
pub const RT5616_IRQ_MB1_OC_SFT: u32 = 15;
pub const RT5616_IRQ_MB1_OC_BP: u32 = 0x0 << 15;
pub const RT5616_IRQ_MB1_OC_NOR: u32 = 0x1 << 15;
pub const RT5616_MB1_OC_STKY_MASK: u32 = 0x1 << 11;
pub const RT5616_MB1_OC_STKY_SFT: u32 = 11;
pub const RT5616_MB1_OC_STKY_DIS: u32 = 0x0 << 11;
pub const RT5616_MB1_OC_STKY_EN: u32 = 0x1 << 11;
pub const RT5616_MB1_OC_P_MASK: u32 = 0x1 << 7;
pub const RT5616_MB1_OC_P_SFT: u32 = 7;
pub const RT5616_MB1_OC_P_NOR: u32 = 0x0 << 7;
pub const RT5616_MB1_OC_P_INV: u32 = 0x1 << 7;
pub const RT5616_MB2_OC_P_MASK: u32 = 0x1 << 6;
pub const RT5616_MB1_OC_CLR: u32 = 0x1 << 3;
pub const RT5616_MB1_OC_CLR_SFT: u32 = 3;
pub const RT5616_STA_GPIO8: u32 = 0x1;
pub const RT5616_STA_GPIO8_BIT: u32 = 0;

/* Internal Status and GPIO status (0xbf) */
pub const RT5616_STA_JD3: u32 = 0x1 << 15;
pub const RT5616_STA_JD3_BIT: u32 = 15;
pub const RT5616_STA_JD2: u32 = 0x1 << 14;
pub const RT5616_STA_JD2_BIT: u32 = 14;
pub const RT5616_STA_JD1_2: u32 = 0x1 << 13;
pub const RT5616_STA_JD1_2_BIT: u32 = 13;
pub const RT5616_STA_JD1_1: u32 = 0x1 << 12;
pub const RT5616_STA_JD1_1_BIT: u32 = 12;
pub const RT5616_STA_GP7: u32 = 0x1 << 11;
pub const RT5616_STA_GP7_BIT: u32 = 11;
pub const RT5616_STA_GP6: u32 = 0x1 << 10;
pub const RT5616_STA_GP6_BIT: u32 = 10;
pub const RT5616_STA_GP5: u32 = 0x1 << 9;
pub const RT5616_STA_GP5_BIT: u32 = 9;
pub const RT5616_STA_GP1: u32 = 0x1 << 8;
pub const RT5616_STA_GP1_BIT: u32 = 8;
pub const RT5616_STA_GP2: u32 = 0x1 << 7;
pub const RT5616_STA_GP2_BIT: u32 = 7;
pub const RT5616_STA_GP3: u32 = 0x1 << 6;
pub const RT5616_STA_GP3_BIT: u32 = 6;
pub const RT5616_STA_GP4: u32 = 0x1 << 5;
pub const RT5616_STA_GP4_BIT: u32 = 5;
pub const RT5616_STA_GP_JD: u32 = 0x1 << 4;
pub const RT5616_STA_GP_JD_BIT: u32 = 4;

/* GPIO Control 1 (0xc0) */
pub const RT5616_GP1_PIN_MASK: u32 = 0x1 << 15;
pub const RT5616_GP1_PIN_SFT: u32 = 15;
pub const RT5616_GP1_PIN_GPIO1: u32 = 0x0 << 15;
pub const RT5616_GP1_PIN_IRQ: u32 = 0x1 << 15;
pub const RT5616_GP2_PIN_MASK: u32 = 0x1 << 14;
pub const RT5616_GP2_PIN_SFT: u32 = 14;
pub const RT5616_GP2_PIN_GPIO2: u32 = 0x0 << 14;
pub const RT5616_GP2_PIN_DMIC1_SCL: u32 = 0x1 << 14;
pub const RT5616_GPIO_M_MASK: u32 = 0x1 << 9;
pub const RT5616_GPIO_M_SFT: u32 = 9;
pub const RT5616_GPIO_M_FLT: u32 = 0x0 << 9;
pub const RT5616_GPIO_M_PH: u32 = 0x1 << 9;
pub const RT5616_I2S2_SEL_MASK: u32 = 0x1 << 8;
pub const RT5616_I2S2_SEL_SFT: u32 = 8;
pub const RT5616_I2S2_SEL_I2S: u32 = 0x0 << 8;
pub const RT5616_I2S2_SEL_GPIO: u32 = 0x1 << 8;
pub const RT5616_GP5_PIN_MASK: u32 = 0x1 << 7;
pub const RT5616_GP5_PIN_SFT: u32 = 7;
pub const RT5616_GP5_PIN_GPIO5: u32 = 0x0 << 7;
pub const RT5616_GP5_PIN_IRQ: u32 = 0x1 << 7;
pub const RT5616_GP6_PIN_MASK: u32 = 0x1 << 6;
pub const RT5616_GP6_PIN_SFT: u32 = 6;
pub const RT5616_GP6_PIN_GPIO6: u32 = 0x0 << 6;
pub const RT5616_GP6_PIN_DMIC_SDA: u32 = 0x1 << 6;
pub const RT5616_GP7_PIN_MASK: u32 = 0x1 << 5;
pub const RT5616_GP7_PIN_SFT: u32 = 5;
pub const RT5616_GP7_PIN_GPIO7: u32 = 0x0 << 5;
pub const RT5616_GP7_PIN_IRQ: u32 = 0x1 << 5;
pub const RT5616_GP8_PIN_MASK: u32 = 0x1 << 4;
pub const RT5616_GP8_PIN_SFT: u32 = 4;
pub const RT5616_GP8_PIN_GPIO8: u32 = 0x0 << 4;
pub const RT5616_GP8_PIN_DMIC_SDA: u32 = 0x1 << 4;
pub const RT5616_GPIO_PDM_SEL_MASK: u32 = 0x1 << 3;
pub const RT5616_GPIO_PDM_SEL_SFT: u32 = 3;
pub const RT5616_GPIO_PDM_SEL_GPIO: u32 = 0x0 << 3;
pub const RT5616_GPIO_PDM_SEL_PDM: u32 = 0x1 << 3;

/* GPIO Control 2 (0xc1) */
pub const RT5616_GP5_DR_MASK: u32 = 0x1 << 14;
pub const RT5616_GP5_DR_SFT: u32 = 14;
pub const RT5616_GP5_DR_IN: u32 = 0x0 << 14;
pub const RT5616_GP5_DR_OUT: u32 = 0x1 << 14;
pub const RT5616_GP5_OUT_MASK: u32 = 0x1 << 13;
pub const RT5616_GP5_OUT_SFT: u32 = 13;
pub const RT5616_GP5_OUT_LO: u32 = 0x0 << 13;
pub const RT5616_GP5_OUT_HI: u32 = 0x1 << 13;
pub const RT5616_GP5_P_MASK: u32 = 0x1 << 12;
pub const RT5616_GP5_P_SFT: u32 = 12;
pub const RT5616_GP5_P_NOR: u32 = 0x0 << 12;
pub const RT5616_GP5_P_INV: u32 = 0x1 << 12;
pub const RT5616_GP4_DR_MASK: u32 = 0x1 << 11;
pub const RT5616_GP4_DR_SFT: u32 = 11;
pub const RT5616_GP4_DR_IN: u32 = 0x0 << 11;
pub const RT5616_GP4_DR_OUT: u32 = 0x1 << 11;
pub const RT5616_GP4_OUT_MASK: u32 = 0x1 << 10;
pub const RT5616_GP4_OUT_SFT: u32 = 10;
pub const RT5616_GP4_OUT_LO: u32 = 0x0 << 10;
pub const RT5616_GP4_OUT_HI: u32 = 0x1 << 10;
pub const RT5616_GP4_P_MASK: u32 = 0x1 << 9;
pub const RT5616_GP4_P_SFT: u32 = 9;
pub const RT5616_GP4_P_NOR: u32 = 0x0 << 9;
pub const RT5616_GP4_P_INV: u32 = 0x1 << 9;
pub const RT5616_GP3_DR_MASK: u32 = 0x1 << 8;
pub const RT5616_GP3_DR_SFT: u32 = 8;
pub const RT5616_GP3_DR_IN: u32 = 0x0 << 8;
pub const RT5616_GP3_DR_OUT: u32 = 0x1 << 8;
pub const RT5616_GP3_OUT_MASK: u32 = 0x1 << 7;
pub const RT5616_GP3_OUT_SFT: u32 = 7;
pub const RT5616_GP3_OUT_LO: u32 = 0x0 << 7;
pub const RT5616_GP3_OUT_HI: u32 = 0x1 << 7;
pub const RT5616_GP3_P_MASK: u32 = 0x1 << 6;
pub const RT5616_GP3_P_SFT: u32 = 6;
pub const RT5616_GP3_P_NOR: u32 = 0x0 << 6;
pub const RT5616_GP3_P_INV: u32 = 0x1 << 6;
pub const RT5616_GP2_DR_MASK: u32 = 0x1 << 5;
pub const RT5616_GP2_DR_SFT: u32 = 5;
pub const RT5616_GP2_DR_IN: u32 = 0x0 << 5;
pub const RT5616_GP2_DR_OUT: u32 = 0x1 << 5;
pub const RT5616_GP2_OUT_MASK: u32 = 0x1 << 4;
pub const RT5616_GP2_OUT_SFT: u32 = 4;
pub const RT5616_GP2_OUT_LO: u32 = 0x0 << 4;
pub const RT5616_GP2_OUT_HI: u32 = 0x1 << 4;
pub const RT5616_GP2_P_MASK: u32 = 0x1 << 3;
pub const RT5616_GP2_P_SFT: u32 = 3;
pub const RT5616_GP2_P_NOR: u32 = 0x0 << 3;
pub const RT5616_GP2_P_INV: u32 = 0x1 << 3;
pub const RT5616_GP1_DR_MASK: u32 = 0x1 << 2;
pub const RT5616_GP1_DR_SFT: u32 = 2;
pub const RT5616_GP1_DR_IN: u32 = 0x0 << 2;
pub const RT5616_GP1_DR_OUT: u32 = 0x1 << 2;
pub const RT5616_GP1_OUT_MASK: u32 = 0x1 << 1;
pub const RT5616_GP1_OUT_SFT: u32 = 1;
pub const RT5616_GP1_OUT_LO: u32 = 0x0 << 1;
pub const RT5616_GP1_OUT_HI: u32 = 0x1 << 1;
pub const RT5616_GP1_P_MASK: u32 = 0x1;
pub const RT5616_GP1_P_SFT: u32 = 0;
pub const RT5616_GP1_P_NOR: u32 = 0x0;
pub const RT5616_GP1_P_INV: u32 = 0x1;

/* GPIO Control 3 (0xc2) */
pub const RT5616_GP8_DR_MASK: u32 = 0x1 << 8;
pub const RT5616_GP8_DR_SFT: u32 = 8;
pub const RT5616_GP8_DR_IN: u32 = 0x0 << 8;
pub const RT5616_GP8_DR_OUT: u32 = 0x1 << 8;
pub const RT5616_GP8_OUT_MASK: u32 = 0x1 << 7;
pub const RT5616_GP8_OUT_SFT: u32 = 7;
pub const RT5616_GP8_OUT_LO: u32 = 0x0 << 7;
pub const RT5616_GP8_OUT_HI: u32 = 0x1 << 7;
pub const RT5616_GP8_P_MASK: u32 = 0x1 << 6;
pub const RT5616_GP8_P_SFT: u32 = 6;
pub const RT5616_GP8_P_NOR: u32 = 0x0 << 6;
pub const RT5616_GP8_P_INV: u32 = 0x1 << 6;
pub const RT5616_GP7_DR_MASK: u32 = 0x1 << 5;
pub const RT5616_GP7_DR_SFT: u32 = 5;
pub const RT5616_GP7_DR_IN: u32 = 0x0 << 5;
pub const RT5616_GP7_DR_OUT: u32 = 0x1 << 5;
pub const RT5616_GP7_OUT_MASK: u32 = 0x1 << 4;
pub const RT5616_GP7_OUT_SFT: u32 = 4;
pub const RT5616_GP7_OUT_LO: u32 = 0x0 << 4;
pub const RT5616_GP7_OUT_HI: u32 = 0x1 << 4;
pub const RT5616_GP7_P_MASK: u32 = 0x1 << 3;
pub const RT5616_GP7_P_SFT: u32 = 3;
pub const RT5616_GP7_P_NOR: u32 = 0x0 << 3;
pub const RT5616_GP7_P_INV: u32 = 0x1 << 3;
pub const RT5616_GP6_DR_MASK: u32 = 0x1 << 2;
pub const RT5616_GP6_DR_SFT: u32 = 2;
pub const RT5616_GP6_DR_IN: u32 = 0x0 << 2;
pub const RT5616_GP6_DR_OUT: u32 = 0x1 << 2;
pub const RT5616_GP6_OUT_MASK: u32 = 0x1 << 1;
pub const RT5616_GP6_OUT_SFT: u32 = 1;
pub const RT5616_GP6_OUT_LO: u32 = 0x0 << 1;
pub const RT5616_GP6_OUT_HI: u32 = 0x1 << 1;
pub const RT5616_GP6_P_MASK: u32 = 0x1;
pub const RT5616_GP6_P_SFT: u32 = 0;
pub const RT5616_GP6_P_NOR: u32 = 0x0;
pub const RT5616_GP6_P_INV: u32 = 0x1;

/* Scramble Control (0xce) */
pub const RT5616_SCB_SWAP_MASK: u32 = 0x1 << 15;
pub const RT5616_SCB_SWAP_SFT: u32 = 15;
pub const RT5616_SCB_SWAP_DIS: u32 = 0x0 << 15;
pub const RT5616_SCB_SWAP_EN: u32 = 0x1 << 15;
pub const RT5616_SCB_MASK: u32 = 0x1 << 14;
pub const RT5616_SCB_SFT: u32 = 14;
pub const RT5616_SCB_DIS: u32 = 0x0 << 14;
pub const RT5616_SCB_EN: u32 = 0x1 << 14;

/* Baseback Control (0xcf) */
pub const RT5616_BB_MASK: u32 = 0x1 << 15;
pub const RT5616_BB_SFT: u32 = 15;
pub const RT5616_BB_DIS: u32 = 0x0 << 15;
pub const RT5616_BB_EN: u32 = 0x1 << 15;
pub const RT5616_BB_CT_MASK: u32 = 0x7 << 12;
pub const RT5616_BB_CT_SFT: u32 = 12;
pub const RT5616_BB_CT_A: u32 = 0x0 << 12;
pub const RT5616_BB_CT_B: u32 = 0x1 << 12;
pub const RT5616_BB_CT_C: u32 = 0x2 << 12;
pub const RT5616_BB_CT_D: u32 = 0x3 << 12;
pub const RT5616_M_BB_L_MASK: u32 = 0x1 << 9;
pub const RT5616_M_BB_L_SFT: u32 = 9;
pub const RT5616_M_BB_R_MASK: u32 = 0x1 << 8;
pub const RT5616_M_BB_R_SFT: u32 = 8;
pub const RT5616_M_BB_HPF_L_MASK: u32 = 0x1 << 7;
pub const RT5616_M_BB_HPF_L_SFT: u32 = 7;
pub const RT5616_M_BB_HPF_R_MASK: u32 = 0x1 << 6;
pub const RT5616_M_BB_HPF_R_SFT: u32 = 6;
pub const RT5616_G_BB_BST_MASK: u32 = 0x3f;
pub const RT5616_G_BB_BST_SFT: u32 = 0;

/* MP3 Plus Control 1 (0xd0) */
pub const RT5616_M_MP3_L_MASK: u32 = 0x1 << 15;
pub const RT5616_M_MP3_L_SFT: u32 = 15;
pub const RT5616_M_MP3_R_MASK: u32 = 0x1 << 14;
pub const RT5616_M_MP3_R_SFT: u32 = 14;
pub const RT5616_M_MP3_MASK: u32 = 0x1 << 13;
pub const RT5616_M_MP3_SFT: u32 = 13;
pub const RT5616_M_MP3_DIS: u32 = 0x0 << 13;
pub const RT5616_M_MP3_EN: u32 = 0x1 << 13;
pub const RT5616_EG_MP3_MASK: u32 = 0x1f << 8;
pub const RT5616_EG_MP3_SFT: u32 = 8;
pub const RT5616_MP3_HLP_MASK: u32 = 0x1 << 7;
pub const RT5616_MP3_HLP_SFT: u32 = 7;
pub const RT5616_MP3_HLP_DIS: u32 = 0x0 << 7;
pub const RT5616_MP3_HLP_EN: u32 = 0x1 << 7;
pub const RT5616_M_MP3_ORG_L_MASK: u32 = 0x1 << 6;
pub const RT5616_M_MP3_ORG_L_SFT: u32 = 6;
pub const RT5616_M_MP3_ORG_R_MASK: u32 = 0x1 << 5;
pub const RT5616_M_MP3_ORG_R_SFT: u32 = 5;

/* MP3 Plus Control 2 (0xd1) */
pub const RT5616_MP3_WT_MASK: u32 = 0x1 << 13;
pub const RT5616_MP3_WT_SFT: u32 = 13;
pub const RT5616_MP3_WT_1_4: u32 = 0x0 << 13;
pub const RT5616_MP3_WT_1_2: u32 = 0x1 << 13;
pub const RT5616_OG_MP3_MASK: u32 = 0x1f << 8;
pub const RT5616_OG_MP3_SFT: u32 = 8;
pub const RT5616_HG_MP3_MASK: u32 = 0x3f;
pub const RT5616_HG_MP3_SFT: u32 = 0;

/* 3D HP Control 1 (0xd2) */
pub const RT5616_3D_CF_MASK: u32 = 0x1 << 15;
pub const RT5616_3D_CF_SFT: u32 = 15;
pub const RT5616_3D_CF_DIS: u32 = 0x0 << 15;
pub const RT5616_3D_CF_EN: u32 = 0x1 << 15;
pub const RT5616_3D_HP_MASK: u32 = 0x1 << 14;
pub const RT5616_3D_HP_SFT: u32 = 14;
pub const RT5616_3D_HP_DIS: u32 = 0x0 << 14;
pub const RT5616_3D_HP_EN: u32 = 0x1 << 14;
pub const RT5616_3D_BT_MASK: u32 = 0x1 << 13;
pub const RT5616_3D_BT_SFT: u32 = 13;
pub const RT5616_3D_BT_DIS: u32 = 0x0 << 13;
pub const RT5616_3D_BT_EN: u32 = 0x1 << 13;
pub const RT5616_3D_1F_MIX_MASK: u32 = 0x3 << 11;
pub const RT5616_3D_1F_MIX_SFT: u32 = 11;
pub const RT5616_3D_HP_M_MASK: u32 = 0x1 << 10;
pub const RT5616_3D_HP_M_SFT: u32 = 10;
pub const RT5616_3D_HP_M_SUR: u32 = 0x0 << 10;
pub const RT5616_3D_HP_M_FRO: u32 = 0x1 << 10;
pub const RT5616_M_3D_HRTF_MASK: u32 = 0x1 << 9;
pub const RT5616_M_3D_HRTF_SFT: u32 = 9;
pub const RT5616_M_3D_D2H_MASK: u32 = 0x1 << 8;
pub const RT5616_M_3D_D2H_SFT: u32 = 8;
pub const RT5616_M_3D_D2R_MASK: u32 = 0x1 << 7;
pub const RT5616_M_3D_D2R_SFT: u32 = 7;
pub const RT5616_M_3D_REVB_MASK: u32 = 0x1 << 6;
pub const RT5616_M_3D_REVB_SFT: u32 = 6;

/* Adjustable high pass filter control 1 (0xd3) */
pub const RT5616_2ND_HPF_MASK: u32 = 0x1 << 15;
pub const RT5616_2ND_HPF_SFT: u32 = 15;
pub const RT5616_2ND_HPF_DIS: u32 = 0x0 << 15;
pub const RT5616_2ND_HPF_EN: u32 = 0x1 << 15;
pub const RT5616_HPF_CF_L_MASK: u32 = 0x7 << 12;
pub const RT5616_HPF_CF_L_SFT: u32 = 12;
pub const RT5616_HPF_CF_R_MASK: u32 = 0x7 << 8;
pub const RT5616_HPF_CF_R_SFT: u32 = 8;
pub const RT5616_ZD_T_MASK: u32 = 0x3 << 6;
pub const RT5616_ZD_T_SFT: u32 = 6;
pub const RT5616_ZD_F_MASK: u32 = 0x3 << 4;
pub const RT5616_ZD_F_SFT: u32 = 4;
pub const RT5616_ZD_F_IM: u32 = 0x0 << 4;
pub const RT5616_ZD_F_ZC_IM: u32 = 0x1 << 4;
pub const RT5616_ZD_F_ZC_IOD: u32 = 0x2 << 4;
pub const RT5616_ZD_F_UN: u32 = 0x3 << 4;

/* Adjustable high pass filter control 2 (0xd4) */
pub const RT5616_HPF_CF_L_NUM_MASK: u32 = 0x3f << 8;
pub const RT5616_HPF_CF_L_NUM_SFT: u32 = 8;
pub const RT5616_HPF_CF_R_NUM_MASK: u32 = 0x3f;
pub const RT5616_HPF_CF_R_NUM_SFT: u32 = 0;

/* HP calibration control and Amp detection (0xd6) */
pub const RT5616_SI_DAC_MASK: u32 = 0x1 << 11;
pub const RT5616_SI_DAC_SFT: u32 = 11;
pub const RT5616_SI_DAC_AUTO: u32 = 0x0 << 11;
pub const RT5616_SI_DAC_TEST: u32 = 0x1 << 11;
pub const RT5616_DC_CAL_M_MASK: u32 = 0x1 << 10;
pub const RT5616_DC_CAL_M_SFT: u32 = 10;
pub const RT5616_DC_CAL_M_NOR: u32 = 0x0 << 10;
pub const RT5616_DC_CAL_M_CAL: u32 = 0x1 << 10;
pub const RT5616_DC_CAL_MASK: u32 = 0x1 << 9;
pub const RT5616_DC_CAL_SFT: u32 = 9;
pub const RT5616_DC_CAL_DIS: u32 = 0x0 << 9;
pub const RT5616_DC_CAL_EN: u32 = 0x1 << 9;
pub const RT5616_HPD_RCV_MASK: u32 = 0x7 << 6;
pub const RT5616_HPD_RCV_SFT: u32 = 6;
pub const RT5616_HPD_PS_MASK: u32 = 0x1 << 5;
pub const RT5616_HPD_PS_SFT: u32 = 5;
pub const RT5616_HPD_PS_DIS: u32 = 0x0 << 5;
pub const RT5616_HPD_PS_EN: u32 = 0x1 << 5;
pub const RT5616_CAL_M_MASK: u32 = 0x1 << 4;
pub const RT5616_CAL_M_SFT: u32 = 4;
pub const RT5616_CAL_M_DEP: u32 = 0x0 << 4;
pub const RT5616_CAL_M_CAL: u32 = 0x1 << 4;
pub const RT5616_CAL_MASK: u32 = 0x1 << 3;
pub const RT5616_CAL_SFT: u32 = 3;
pub const RT5616_CAL_DIS: u32 = 0x0 << 3;
pub const RT5616_CAL_EN: u32 = 0x1 << 3;
pub const RT5616_CAL_TEST_MASK: u32 = 0x1 << 2;
pub const RT5616_CAL_TEST_SFT: u32 = 2;
pub const RT5616_CAL_TEST_DIS: u32 = 0x0 << 2;
pub const RT5616_CAL_TEST_EN: u32 = 0x1 << 2;
pub const RT5616_CAL_P_MASK: u32 = 0x3;
pub const RT5616_CAL_P_SFT: u32 = 0;
pub const RT5616_CAL_P_NONE: u32 = 0x0;
pub const RT5616_CAL_P_CAL: u32 = 0x1;
pub const RT5616_CAL_P_DAC_CAL: u32 = 0x2;

/* Soft volume and zero cross control 1 (0xd9) */
pub const RT5616_SV_MASK: u32 = 0x1 << 15;
pub const RT5616_SV_SFT: u32 = 15;
pub const RT5616_SV_DIS: u32 = 0x0 << 15;
pub const RT5616_SV_EN: u32 = 0x1 << 15;
pub const RT5616_OUT_SV_MASK: u32 = 0x1 << 13;
pub const RT5616_OUT_SV_SFT: u32 = 13;
pub const RT5616_OUT_SV_DIS: u32 = 0x0 << 13;
pub const RT5616_OUT_SV_EN: u32 = 0x1 << 13;
pub const RT5616_HP_SV_MASK: u32 = 0x1 << 12;
pub const RT5616_HP_SV_SFT: u32 = 12;
pub const RT5616_HP_SV_DIS: u32 = 0x0 << 12;
pub const RT5616_HP_SV_EN: u32 = 0x1 << 12;
pub const RT5616_ZCD_DIG_MASK: u32 = 0x1 << 11;
pub const RT5616_ZCD_DIG_SFT: u32 = 11;
pub const RT5616_ZCD_DIG_DIS: u32 = 0x0 << 11;
pub const RT5616_ZCD_DIG_EN: u32 = 0x1 << 11;
pub const RT5616_ZCD_MASK: u32 = 0x1 << 10;
pub const RT5616_ZCD_SFT: u32 = 10;
pub const RT5616_ZCD_PD: u32 = 0x0 << 10;
pub const RT5616_ZCD_PU: u32 = 0x1 << 10;
pub const RT5616_M_ZCD_MASK: u32 = 0x3f << 4;
pub const RT5616_M_ZCD_SFT: u32 = 4;
pub const RT5616_M_ZCD_OM_L: u32 = 0x1 << 7;
pub const RT5616_M_ZCD_OM_R: u32 = 0x1 << 6;
pub const RT5616_M_ZCD_RM_L: u32 = 0x1 << 5;
pub const RT5616_M_ZCD_RM_R: u32 = 0x1 << 4;
pub const RT5616_SV_DLY_MASK: u32 = 0xf;
pub const RT5616_SV_DLY_SFT: u32 = 0;

/* Soft volume and zero cross control 2 (0xda) */
pub const RT5616_ZCD_HP_MASK: u32 = 0x1 << 15;
pub const RT5616_ZCD_HP_SFT: u32 = 15;
pub const RT5616_ZCD_HP_DIS: u32 = 0x0 << 15;
pub const RT5616_ZCD_HP_EN: u32 = 0x1 << 15;

/* Digital Misc Control (0xfa) */
pub const RT5616_I2S2_MS_SP_MASK: u32 = 0x1 << 8;
pub const RT5616_I2S2_MS_SP_SEL: u32 = 8;
pub const RT5616_I2S2_MS_SP_64: u32 = 0x0 << 8;
pub const RT5616_I2S2_MS_SP_50: u32 = 0x1 << 8;
pub const RT5616_CLK_DET_EN: u32 = 0x1 << 3;
pub const RT5616_CLK_DET_EN_SFT: u32 = 3;
pub const RT5616_AMP_DET_EN: u32 = 0x1 << 1;
pub const RT5616_AMP_DET_EN_SFT: u32 = 1;
pub const RT5616_D_GATE_EN: u32 = 0x1;
pub const RT5616_D_GATE_EN_SFT: u32 = 0;

/* Codec Private Register definition */
/* 3D Speaker Control (0x63) */
pub const RT5616_3D_SPK_MASK: u32 = 0x1 << 15;
pub const RT5616_3D_SPK_SFT: u32 = 15;
pub const RT5616_3D_SPK_DIS: u32 = 0x0 << 15;
pub const RT5616_3D_SPK_EN: u32 = 0x1 << 15;
pub const RT5616_3D_SPK_M_MASK: u32 = 0x3 << 13;
pub const RT5616_3D_SPK_M_SFT: u32 = 13;
pub const RT5616_3D_SPK_CG_MASK: u32 = 0x1f << 8;
pub const RT5616_3D_SPK_CG_SFT: u32 = 8;
pub const RT5616_3D_SPK_SG_MASK: u32 = 0x1f;
pub const RT5616_3D_SPK_SG_SFT: u32 = 0;

/* Wind Noise Detection Control 1 (0x6c) */
pub const RT5616_WND_MASK: u32 = 0x1 << 15;
pub const RT5616_WND_SFT: u32 = 15;
pub const RT5616_WND_DIS: u32 = 0x0 << 15;
pub const RT5616_WND_EN: u32 = 0x1 << 15;

/* Wind Noise Detection Control 2 (0x6d) */
pub const RT5616_WND_FC_NW_MASK: u32 = 0x3f << 10;
pub const RT5616_WND_FC_NW_SFT: u32 = 10;
pub const RT5616_WND_FC_WK_MASK: u32 = 0x3f << 4;
pub const RT5616_WND_FC_WK_SFT: u32 = 4;

/* Wind Noise Detection Control 3 (0x6e) */
pub const RT5616_HPF_FC_MASK: u32 = 0x3f << 6;
pub const RT5616_HPF_FC_SFT: u32 = 6;
pub const RT5616_WND_FC_ST_MASK: u32 = 0x3f;
pub const RT5616_WND_FC_ST_SFT: u32 = 0;

/* Wind Noise Detection Control 4 (0x6f) */
pub const RT5616_WND_TH_LO_MASK: u32 = 0x3ff;
pub const RT5616_WND_TH_LO_SFT: u32 = 0;

/* Wind Noise Detection Control 5 (0x70) */
pub const RT5616_WND_TH_HI_MASK: u32 = 0x3ff;
pub const RT5616_WND_TH_HI_SFT: u32 = 0;

/* Wind Noise Detection Control 8 (0x73) */
pub const RT5616_WND_WIND_MASK: u32 = 0x1 << 13; /* Read-Only */
pub const RT5616_WND_WIND_SFT: u32 = 13;
pub const RT5616_WND_STRONG_MASK: u32 = 0x1 << 12; /* Read-Only */
pub const RT5616_WND_STRONG_SFT: u32 = 12;
pub const RT5616_NO_WIND: u32 = 0;
pub const RT5616_BREEZE: u32 = 1;
pub const RT5616_STORM: u32 = 2;


/* Dipole Speaker Interface (0x75) */
pub const RT5616_DP_ATT_MASK: u32 = 0x3 << 14;
pub const RT5616_DP_ATT_SFT: u32 = 14;
pub const RT5616_DP_SPK_MASK: u32 = 0x1 << 10;
pub const RT5616_DP_SPK_SFT: u32 = 10;
pub const RT5616_DP_SPK_DIS: u32 = 0x0 << 10;
pub const RT5616_DP_SPK_EN: u32 = 0x1 << 10;

/* EQ Pre Volume Control (0xb3) */
pub const RT5616_EQ_PRE_VOL_MASK: u32 = 0xffff;
pub const RT5616_EQ_PRE_VOL_SFT: u32 = 0;

/* EQ Post Volume Control (0xb4) */
pub const RT5616_EQ_PST_VOL_MASK: u32 = 0xffff;
pub const RT5616_EQ_PST_VOL_SFT: u32 = 0;

/* System Clock Source */
pub const RT5616_SCLK_S_MCLK: u32 = 0;
pub const RT5616_SCLK_S_PLL1: u32 = 1;


/* PLL1 Source */
pub const RT5616_PLL1_S_MCLK: u32 = 0;
pub const RT5616_PLL1_S_BCLK1: u32 = 1;
pub const RT5616_PLL1_S_BCLK2: u32 = 2;


pub const RT5616_AIF1: u32 = 0;
pub const RT5616_AIFS: u32 = 1;



// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
