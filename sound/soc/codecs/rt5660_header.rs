/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rt5660.h  --  RT5660 ALSA SoC audio driver
 *
 * Copyright 2016 Realtek Semiconductor Corp.
 * Author: Oder Chiou <oder_chiou@realtek.com>
 */


// Dependency from C header: <linux/clk.h>
// Dependency from C header: <sound/rt5660.h>


/* Info */
pub const RT5660_RESET: u32 = 0x00u32;
pub const RT5660_VENDOR_ID: u32 = 0xfdu32;
pub const RT5660_VENDOR_ID1: u32 = 0xfeu32;
pub const RT5660_VENDOR_ID2: u32 = 0xffu32;
/*  I/O - Output */
pub const RT5660_SPK_VOL: u32 = 0x01u32;
pub const RT5660_LOUT_VOL: u32 = 0x02u32;
/* I/O - Input */
pub const RT5660_IN1_IN2: u32 = 0x0du32;
pub const RT5660_IN3_IN4: u32 = 0x0eu32;
/* I/O - ADC/DAC/DMIC */
pub const RT5660_DAC1_DIG_VOL: u32 = 0x19u32;
pub const RT5660_STO1_ADC_DIG_VOL: u32 = 0x1cu32;
pub const RT5660_ADC_BST_VOL1: u32 = 0x1eu32;
/* Mixer - D-D */
pub const RT5660_STO1_ADC_MIXER: u32 = 0x27u32;
pub const RT5660_AD_DA_MIXER: u32 = 0x29u32;
pub const RT5660_STO_DAC_MIXER: u32 = 0x2au32;
pub const RT5660_DIG_INF1_DATA: u32 = 0x2fu32;
/* Mixer - ADC */
pub const RT5660_REC_L1_MIXER: u32 = 0x3bu32;
pub const RT5660_REC_L2_MIXER: u32 = 0x3cu32;
pub const RT5660_REC_R1_MIXER: u32 = 0x3du32;
pub const RT5660_REC_R2_MIXER: u32 = 0x3eu32;
/* Mixer - DAC */
pub const RT5660_LOUT_MIXER: u32 = 0x45u32;
pub const RT5660_SPK_MIXER: u32 = 0x46u32;
pub const RT5660_SPO_MIXER: u32 = 0x48u32;
pub const RT5660_SPO_CLSD_RATIO: u32 = 0x4au32;
pub const RT5660_OUT_L_GAIN1: u32 = 0x4du32;
pub const RT5660_OUT_L_GAIN2: u32 = 0x4eu32;
pub const RT5660_OUT_L1_MIXER: u32 = 0x4fu32;
pub const RT5660_OUT_R_GAIN1: u32 = 0x50u32;
pub const RT5660_OUT_R_GAIN2: u32 = 0x51u32;
pub const RT5660_OUT_R1_MIXER: u32 = 0x52u32;
/* Power */
pub const RT5660_PWR_DIG1: u32 = 0x61u32;
pub const RT5660_PWR_DIG2: u32 = 0x62u32;
pub const RT5660_PWR_ANLG1: u32 = 0x63u32;
pub const RT5660_PWR_ANLG2: u32 = 0x64u32;
pub const RT5660_PWR_MIXER: u32 = 0x65u32;
pub const RT5660_PWR_VOL: u32 = 0x66u32;
/* Private Register Control */
pub const RT5660_PRIV_INDEX: u32 = 0x6au32;
pub const RT5660_PRIV_DATA: u32 = 0x6cu32;
/* Format - ADC/DAC */
pub const RT5660_I2S1_SDP: u32 = 0x70u32;
pub const RT5660_ADDA_CLK1: u32 = 0x73u32;
pub const RT5660_ADDA_CLK2: u32 = 0x74u32;
pub const RT5660_DMIC_CTRL1: u32 = 0x75u32;
/* Function - Analog */
pub const RT5660_GLB_CLK: u32 = 0x80u32;
pub const RT5660_PLL_CTRL1: u32 = 0x81u32;
pub const RT5660_PLL_CTRL2: u32 = 0x82u32;
pub const RT5660_CLSD_AMP_OC_CTRL: u32 = 0x8cu32;
pub const RT5660_CLSD_AMP_CTRL: u32 = 0x8du32;
pub const RT5660_LOUT_AMP_CTRL: u32 = 0x8eu32;
pub const RT5660_SPK_AMP_SPKVDD: u32 = 0x92u32;
pub const RT5660_MICBIAS: u32 = 0x93u32;
pub const RT5660_CLSD_OUT_CTRL1: u32 = 0xa1u32;
pub const RT5660_CLSD_OUT_CTRL2: u32 = 0xa2u32;
pub const RT5660_DIPOLE_MIC_CTRL1: u32 = 0xa3u32;
pub const RT5660_DIPOLE_MIC_CTRL2: u32 = 0xa4u32;
pub const RT5660_DIPOLE_MIC_CTRL3: u32 = 0xa5u32;
pub const RT5660_DIPOLE_MIC_CTRL4: u32 = 0xa6u32;
pub const RT5660_DIPOLE_MIC_CTRL5: u32 = 0xa7u32;
pub const RT5660_DIPOLE_MIC_CTRL6: u32 = 0xa8u32;
pub const RT5660_DIPOLE_MIC_CTRL7: u32 = 0xa9u32;
pub const RT5660_DIPOLE_MIC_CTRL8: u32 = 0xaau32;
pub const RT5660_DIPOLE_MIC_CTRL9: u32 = 0xabu32;
pub const RT5660_DIPOLE_MIC_CTRL10: u32 = 0xacu32;
pub const RT5660_DIPOLE_MIC_CTRL11: u32 = 0xadu32;
pub const RT5660_DIPOLE_MIC_CTRL12: u32 = 0xaeu32;
/* Function - Digital */
pub const RT5660_EQ_CTRL1: u32 = 0xb0u32;
pub const RT5660_EQ_CTRL2: u32 = 0xb1u32;
pub const RT5660_DRC_AGC_CTRL1: u32 = 0xb3u32;
pub const RT5660_DRC_AGC_CTRL2: u32 = 0xb4u32;
pub const RT5660_DRC_AGC_CTRL3: u32 = 0xb5u32;
pub const RT5660_DRC_AGC_CTRL4: u32 = 0xb6u32;
pub const RT5660_DRC_AGC_CTRL5: u32 = 0xb7u32;
pub const RT5660_JD_CTRL: u32 = 0xbbu32;
pub const RT5660_IRQ_CTRL1: u32 = 0xbdu32;
pub const RT5660_IRQ_CTRL2: u32 = 0xbeu32;
pub const RT5660_INT_IRQ_ST: u32 = 0xbfu32;
pub const RT5660_GPIO_CTRL1: u32 = 0xc0u32;
pub const RT5660_GPIO_CTRL2: u32 = 0xc2u32;
pub const RT5660_WIND_FILTER_CTRL1: u32 = 0xd3u32;
pub const RT5660_SV_ZCD1: u32 = 0xd9u32;
pub const RT5660_SV_ZCD2: u32 = 0xdau32;
pub const RT5660_DRC1_LM_CTRL1: u32 = 0xe0u32;
pub const RT5660_DRC1_LM_CTRL2: u32 = 0xe1u32;
pub const RT5660_DRC2_LM_CTRL1: u32 = 0xe2u32;
pub const RT5660_DRC2_LM_CTRL2: u32 = 0xe3u32;
pub const RT5660_MULTI_DRC_CTRL: u32 = 0xe4u32;
pub const RT5660_DRC2_CTRL1: u32 = 0xe5u32;
pub const RT5660_DRC2_CTRL2: u32 = 0xe6u32;
pub const RT5660_DRC2_CTRL3: u32 = 0xe7u32;
pub const RT5660_DRC2_CTRL4: u32 = 0xe8u32;
pub const RT5660_DRC2_CTRL5: u32 = 0xe9u32;
pub const RT5660_ALC_PGA_CTRL1: u32 = 0xeau32;
pub const RT5660_ALC_PGA_CTRL2: u32 = 0xebu32;
pub const RT5660_ALC_PGA_CTRL3: u32 = 0xecu32;
pub const RT5660_ALC_PGA_CTRL4: u32 = 0xedu32;
pub const RT5660_ALC_PGA_CTRL5: u32 = 0xeeu32;
pub const RT5660_ALC_PGA_CTRL6: u32 = 0xefu32;
pub const RT5660_ALC_PGA_CTRL7: u32 = 0xf0u32;

/* General Control */
pub const RT5660_GEN_CTRL1: u32 = 0xfau32;
pub const RT5660_GEN_CTRL2: u32 = 0xfbu32;
pub const RT5660_GEN_CTRL3: u32 = 0xfcu32;

/* Index of Codec Private Register definition */
pub const RT5660_CHOP_DAC_ADC: u32 = 0x3du32;

/* Global Definition */
pub const RT5660_L_MUTE: u32 = (0x1u32 << 15);
pub const RT5660_L_MUTE_SFT: u32 = 15;
pub const RT5660_VOL_L_MUTE: u32 = (0x1u32 << 14);
pub const RT5660_VOL_L_SFT: u32 = 14;
pub const RT5660_R_MUTE: u32 = (0x1u32 << 7);
pub const RT5660_R_MUTE_SFT: u32 = 7;
pub const RT5660_VOL_R_MUTE: u32 = (0x1u32 << 6);
pub const RT5660_VOL_R_SFT: u32 = 6;
pub const RT5660_L_VOL_MASK: u32 = (0x3fu32 << 8);
pub const RT5660_L_VOL_SFT: u32 = 8;
pub const RT5660_R_VOL_MASK: u32 = (0x3fu32);
pub const RT5660_R_VOL_SFT: u32 = 0;

/* IN1 and IN2 Control (0x0d) */
pub const RT5660_IN_DF1: u32 = (0x1u32 << 15);
pub const RT5660_IN_SFT1: u32 = 15;
pub const RT5660_BST_MASK1: u32 = (0x7fu32 << 8);
pub const RT5660_BST_SFT1: u32 = 8;
pub const RT5660_IN_DF2: u32 = (0x1u32 << 7);
pub const RT5660_IN_SFT2: u32 = 7;
pub const RT5660_BST_MASK2: u32 = (0x7fu32 << 0);
pub const RT5660_BST_SFT2: u32 = 0;

/* IN3 and IN4 Control (0x0e) */
pub const RT5660_IN_DF3: u32 = (0x1u32 << 15);
pub const RT5660_IN_SFT3: u32 = 15;
pub const RT5660_BST_MASK3: u32 = (0x7fu32 << 8);
pub const RT5660_BST_SFT3: u32 = 8;
pub const RT5660_IN_DF4: u32 = (0x1u32 << 7);
pub const RT5660_IN_SFT4: u32 = 7;
pub const RT5660_BST_MASK4: u32 = (0x7fu32 << 0);
pub const RT5660_BST_SFT4: u32 = 0;

/* DAC1 Digital Volume (0x19) */
pub const RT5660_DAC_L1_VOL_MASK: u32 = (0x7fu32 << 9);
pub const RT5660_DAC_L1_VOL_SFT: u32 = 9;
pub const RT5660_DAC_R1_VOL_MASK: u32 = (0x7fu32 << 1);
pub const RT5660_DAC_R1_VOL_SFT: u32 = 1;

/* ADC Digital Volume Control (0x1c) */
pub const RT5660_ADC_L_VOL_MASK: u32 = (0x3fu32 << 9);
pub const RT5660_ADC_L_VOL_SFT: u32 = 9;
pub const RT5660_ADC_R_VOL_MASK: u32 = (0x3fu32 << 1);
pub const RT5660_ADC_R_VOL_SFT: u32 = 1;

/* ADC Boost Volume Control (0x1e) */
pub const RT5660_STO1_ADC_L_BST_MASK: u32 = (0x3u32 << 14);
pub const RT5660_STO1_ADC_L_BST_SFT: u32 = 14;
pub const RT5660_STO1_ADC_R_BST_MASK: u32 = (0x3u32 << 12);
pub const RT5660_STO1_ADC_R_BST_SFT: u32 = 12;

/* Stereo ADC Mixer Control (0x27) */
pub const RT5660_M_ADC_L1: u32 = (0x1u32 << 14);
pub const RT5660_M_ADC_L1_SFT: u32 = 14;
pub const RT5660_M_ADC_L2: u32 = (0x1u32 << 13);
pub const RT5660_M_ADC_L2_SFT: u32 = 13;
pub const RT5660_M_ADC_R1: u32 = (0x1u32 << 6);
pub const RT5660_M_ADC_R1_SFT: u32 = 6;
pub const RT5660_M_ADC_R2: u32 = (0x1u32 << 5);
pub const RT5660_M_ADC_R2_SFT: u32 = 5;

/* ADC Mixer to DAC Mixer Control (0x29) */
pub const RT5660_M_ADCMIX_L: u32 = (0x1u32 << 15);
pub const RT5660_M_ADCMIX_L_SFT: u32 = 15;
pub const RT5660_M_DAC1_L: u32 = (0x1u32 << 14);
pub const RT5660_M_DAC1_L_SFT: u32 = 14;
pub const RT5660_M_ADCMIX_R: u32 = (0x1u32 << 7);
pub const RT5660_M_ADCMIX_R_SFT: u32 = 7;
pub const RT5660_M_DAC1_R: u32 = (0x1u32 << 6);
pub const RT5660_M_DAC1_R_SFT: u32 = 6;

/* Stereo DAC Mixer Control (0x2a) */
pub const RT5660_M_DAC_L1: u32 = (0x1u32 << 14);
pub const RT5660_M_DAC_L1_SFT: u32 = 14;
pub const RT5660_DAC_L1_STO_L_VOL_MASK: u32 = (0x1u32 << 13);
pub const RT5660_DAC_L1_STO_L_VOL_SFT: u32 = 13;
pub const RT5660_M_DAC_R1_STO_L: u32 = (0x1u32 << 9);
pub const RT5660_M_DAC_R1_STO_L_SFT: u32 = 9;
pub const RT5660_DAC_R1_STO_L_VOL_MASK: u32 = (0x1u32 << 8);
pub const RT5660_DAC_R1_STO_L_VOL_SFT: u32 = 8;
pub const RT5660_M_DAC_R1: u32 = (0x1u32 << 6);
pub const RT5660_M_DAC_R1_SFT: u32 = 6;
pub const RT5660_DAC_R1_STO_R_VOL_MASK: u32 = (0x1u32 << 5);
pub const RT5660_DAC_R1_STO_R_VOL_SFT: u32 = 5;
pub const RT5660_M_DAC_L1_STO_R: u32 = (0x1u32 << 1);
pub const RT5660_M_DAC_L1_STO_R_SFT: u32 = 1;
pub const RT5660_DAC_L1_STO_R_VOL_MASK: u32 = (0x1u32);
pub const RT5660_DAC_L1_STO_R_VOL_SFT: u32 = 0;

/* Digital Interface Data Control (0x2f) */
pub const RT5660_IF1_DAC_IN_SEL: u32 = (0x3u32 << 14);
pub const RT5660_IF1_DAC_IN_SFT: u32 = 14;
pub const RT5660_IF1_ADC_IN_SEL: u32 = (0x3u32 << 12);
pub const RT5660_IF1_ADC_IN_SFT: u32 = 12;

/* REC Left Mixer Control 1 (0x3b) */
pub const RT5660_G_BST3_RM_L_MASK: u32 = (0x7u32 << 4);
pub const RT5660_G_BST3_RM_L_SFT: u32 = 4;
pub const RT5660_G_BST2_RM_L_MASK: u32 = (0x7u32 << 1);
pub const RT5660_G_BST2_RM_L_SFT: u32 = 1;

/* REC Left Mixer Control 2 (0x3c) */
pub const RT5660_G_BST1_RM_L_MASK: u32 = (0x7u32 << 13);
pub const RT5660_G_BST1_RM_L_SFT: u32 = 13;
pub const RT5660_G_OM_L_RM_L_MASK: u32 = (0x7u32 << 10);
pub const RT5660_G_OM_L_RM_L_SFT: u32 = 10;
pub const RT5660_M_BST3_RM_L: u32 = (0x1u32 << 3);
pub const RT5660_M_BST3_RM_L_SFT: u32 = 3;
pub const RT5660_M_BST2_RM_L: u32 = (0x1u32 << 2);
pub const RT5660_M_BST2_RM_L_SFT: u32 = 2;
pub const RT5660_M_BST1_RM_L: u32 = (0x1u32 << 1);
pub const RT5660_M_BST1_RM_L_SFT: u32 = 1;
pub const RT5660_M_OM_L_RM_L: u32 = (0x1u32);
pub const RT5660_M_OM_L_RM_L_SFT: u32 = 0;

/* REC Right Mixer Control 1 (0x3d) */
pub const RT5660_G_BST3_RM_R_MASK: u32 = (0x7u32 << 4);
pub const RT5660_G_BST3_RM_R_SFT: u32 = 4;
pub const RT5660_G_BST2_RM_R_MASK: u32 = (0x7u32 << 1);
pub const RT5660_G_BST2_RM_R_SFT: u32 = 1;

/* REC Right Mixer Control 2 (0x3e) */
pub const RT5660_G_BST1_RM_R_MASK: u32 = (0x7u32 << 13);
pub const RT5660_G_BST1_RM_R_SFT: u32 = 13;
pub const RT5660_G_OM_R_RM_R_MASK: u32 = (0x7u32 << 10);
pub const RT5660_G_OM_R_RM_R_SFT: u32 = 10;
pub const RT5660_M_BST3_RM_R: u32 = (0x1u32 << 3);
pub const RT5660_M_BST3_RM_R_SFT: u32 = 3;
pub const RT5660_M_BST2_RM_R: u32 = (0x1u32 << 2);
pub const RT5660_M_BST2_RM_R_SFT: u32 = 2;
pub const RT5660_M_BST1_RM_R: u32 = (0x1u32 << 1);
pub const RT5660_M_BST1_RM_R_SFT: u32 = 1;
pub const RT5660_M_OM_R_RM_R: u32 = (0x1u32);
pub const RT5660_M_OM_R_RM_R_SFT: u32 = 0;

/* LOUTMIX Control (0x45) */
pub const RT5660_M_DAC1_LM: u32 = (0x1u32 << 14);
pub const RT5660_M_DAC1_LM_SFT: u32 = 14;
pub const RT5660_M_LOVOL_M: u32 = (0x1u32 << 13);
pub const RT5660_M_LOVOL_LM_SFT: u32 = 13;

/* SPK Mixer Control (0x46) */
pub const RT5660_G_BST3_SM_MASK: u32 = (0x3u32 << 14);
pub const RT5660_G_BST3_SM_SFT: u32 = 14;
pub const RT5660_G_BST1_SM_MASK: u32 = (0x3u32 << 12);
pub const RT5660_G_BST1_SM_SFT: u32 = 12;
pub const RT5660_G_DACl_SM_MASK: u32 = (0x3u32 << 10);
pub const RT5660_G_DACl_SM_SFT: u32 = 10;
pub const RT5660_G_DACR_SM_MASK: u32 = (0x3u32 << 8);
pub const RT5660_G_DACR_SM_SFT: u32 = 8;
pub const RT5660_G_OM_L_SM_MASK: u32 = (0x3u32 << 6);
pub const RT5660_G_OM_L_SM_SFT: u32 = 6;
pub const RT5660_M_DACR_SM: u32 = (0x1u32 << 5);
pub const RT5660_M_DACR_SM_SFT: u32 = 5;
pub const RT5660_M_BST1_SM: u32 = (0x1u32 << 4);
pub const RT5660_M_BST1_SM_SFT: u32 = 4;
pub const RT5660_M_BST3_SM: u32 = (0x1u32 << 3);
pub const RT5660_M_BST3_SM_SFT: u32 = 3;
pub const RT5660_M_DACL_SM: u32 = (0x1u32 << 2);
pub const RT5660_M_DACL_SM_SFT: u32 = 2;
pub const RT5660_M_OM_L_SM: u32 = (0x1u32 << 1);
pub const RT5660_M_OM_L_SM_SFT: u32 = 1;

/* SPOMIX Control (0x48) */
pub const RT5660_M_DAC_R_SPM: u32 = (0x1u32 << 14);
pub const RT5660_M_DAC_R_SPM_SFT: u32 = 14;
pub const RT5660_M_DAC_L_SPM: u32 = (0x1u32 << 13);
pub const RT5660_M_DAC_L_SPM_SFT: u32 = 13;
pub const RT5660_M_SV_SPM: u32 = (0x1u32 << 12);
pub const RT5660_M_SV_SPM_SFT: u32 = 12;
pub const RT5660_M_BST1_SPM: u32 = (0x1u32 << 11);
pub const RT5660_M_BST1_SPM_SFT: u32 = 11;

/* Output Left Mixer Control 1 (0x4d) */
pub const RT5660_G_BST3_OM_L_MASK: u32 = (0x7u32 << 13);
pub const RT5660_G_BST3_OM_L_SFT: u32 = 13;
pub const RT5660_G_BST2_OM_L_MASK: u32 = (0x7u32 << 10);
pub const RT5660_G_BST2_OM_L_SFT: u32 = 10;
pub const RT5660_G_BST1_OM_L_MASK: u32 = (0x7u32 << 7);
pub const RT5660_G_BST1_OM_L_SFT: u32 = 7;
pub const RT5660_G_RM_L_OM_L_MASK: u32 = (0x7u32 << 1);
pub const RT5660_G_RM_L_OM_L_SFT: u32 = 1;

/* Output Left Mixer Control 2 (0x4e) */
pub const RT5660_G_DAC_R1_OM_L_MASK: u32 = (0x7u32 << 10);
pub const RT5660_G_DAC_R1_OM_L_SFT: u32 = 10;
pub const RT5660_G_DAC_L1_OM_L_MASK: u32 = (0x7u32 << 7);
pub const RT5660_G_DAC_L1_OM_L_SFT: u32 = 7;

/* Output Left Mixer Control 3 (0x4f) */
pub const RT5660_M_BST3_OM_L: u32 = (0x1u32 << 5);
pub const RT5660_M_BST3_OM_L_SFT: u32 = 5;
pub const RT5660_M_BST2_OM_L: u32 = (0x1u32 << 4);
pub const RT5660_M_BST2_OM_L_SFT: u32 = 4;
pub const RT5660_M_BST1_OM_L: u32 = (0x1u32 << 3);
pub const RT5660_M_BST1_OM_L_SFT: u32 = 3;
pub const RT5660_M_RM_L_OM_L: u32 = (0x1u32 << 2);
pub const RT5660_M_RM_L_OM_L_SFT: u32 = 2;
pub const RT5660_M_DAC_R_OM_L: u32 = (0x1u32 << 1);
pub const RT5660_M_DAC_R_OM_L_SFT: u32 = 1;
pub const RT5660_M_DAC_L_OM_L: u32 = (0x1u32);
pub const RT5660_M_DAC_L_OM_L_SFT: u32 = 0;

/* Output Right Mixer Control 1 (0x50) */
pub const RT5660_G_BST2_OM_R_MASK: u32 = (0x7u32 << 10);
pub const RT5660_G_BST2_OM_R_SFT: u32 = 10;
pub const RT5660_G_BST1_OM_R_MASK: u32 = (0x7u32 << 7);
pub const RT5660_G_BST1_OM_R_SFT: u32 = 7;
pub const RT5660_G_RM_R_OM_R_MASK: u32 = (0x7u32 << 1);
pub const RT5660_G_RM_R_OM_R_SFT: u32 = 1;

/* Output Right Mixer Control 2 (0x51) */
pub const RT5660_G_DAC_L_OM_R_MASK: u32 = (0x7u32 << 10);
pub const RT5660_G_DAC_L_OM_R_SFT: u32 = 10;
pub const RT5660_G_DAC_R_OM_R_MASK: u32 = (0x7u32 << 7);
pub const RT5660_G_DAC_R_OM_R_SFT: u32 = 7;

/* Output Right Mixer Control 3 (0x52) */
pub const RT5660_M_BST2_OM_R: u32 = (0x1u32 << 4);
pub const RT5660_M_BST2_OM_R_SFT: u32 = 4;
pub const RT5660_M_BST1_OM_R: u32 = (0x1u32 << 3);
pub const RT5660_M_BST1_OM_R_SFT: u32 = 3;
pub const RT5660_M_RM_R_OM_R: u32 = (0x1u32 << 2);
pub const RT5660_M_RM_R_OM_R_SFT: u32 = 2;
pub const RT5660_M_DAC_L_OM_R: u32 = (0x1u32 << 1);
pub const RT5660_M_DAC_L_OM_R_SFT: u32 = 1;
pub const RT5660_M_DAC_R_OM_R: u32 = (0x1u32);
pub const RT5660_M_DAC_R_OM_R_SFT: u32 = 0;

/* Power Management for Digital 1 (0x61) */
pub const RT5660_PWR_I2S1: u32 = (0x1u32 << 15);
pub const RT5660_PWR_I2S1_BIT: u32 = 15;
pub const RT5660_PWR_DAC_L1: u32 = (0x1u32 << 12);
pub const RT5660_PWR_DAC_L1_BIT: u32 = 12;
pub const RT5660_PWR_DAC_R1: u32 = (0x1u32 << 11);
pub const RT5660_PWR_DAC_R1_BIT: u32 = 11;
pub const RT5660_PWR_ADC_L: u32 = (0x1u32 << 2);
pub const RT5660_PWR_ADC_L_BIT: u32 = 2;
pub const RT5660_PWR_ADC_R: u32 = (0x1u32 << 1);
pub const RT5660_PWR_ADC_R_BIT: u32 = 1;
pub const RT5660_PWR_CLS_D: u32 = (0x1u32);
pub const RT5660_PWR_CLS_D_BIT: u32 = 0;

/* Power Management for Digital 2 (0x62) */
pub const RT5660_PWR_ADC_S1F: u32 = (0x1u32 << 15);
pub const RT5660_PWR_ADC_S1F_BIT: u32 = 15;
pub const RT5660_PWR_DAC_S1F: u32 = (0x1u32 << 11);
pub const RT5660_PWR_DAC_S1F_BIT: u32 = 11;

/* Power Management for Analog 1 (0x63) */
pub const RT5660_PWR_VREF1: u32 = (0x1u32 << 15);
pub const RT5660_PWR_VREF1_BIT: u32 = 15;
pub const RT5660_PWR_FV1: u32 = (0x1u32 << 14);
pub const RT5660_PWR_FV1_BIT: u32 = 14;
pub const RT5660_PWR_MB: u32 = (0x1u32 << 13);
pub const RT5660_PWR_MB_BIT: u32 = 13;
pub const RT5660_PWR_BG: u32 = (0x1u32 << 11);
pub const RT5660_PWR_BG_BIT: u32 = 11;
pub const RT5660_PWR_HP_L: u32 = (0x1u32 << 7);
pub const RT5660_PWR_HP_L_BIT: u32 = 7;
pub const RT5660_PWR_HP_R: u32 = (0x1u32 << 6);
pub const RT5660_PWR_HP_R_BIT: u32 = 6;
pub const RT5660_PWR_HA: u32 = (0x1u32 << 5);
pub const RT5660_PWR_HA_BIT: u32 = 5;
pub const RT5660_PWR_VREF2: u32 = (0x1u32 << 4);
pub const RT5660_PWR_VREF2_BIT: u32 = 4;
pub const RT5660_PWR_FV2: u32 = (0x1u32 << 3);
pub const RT5660_PWR_FV2_BIT: u32 = 3;
pub const RT5660_PWR_LDO2: u32 = (0x1u32 << 2);
pub const RT5660_PWR_LDO2_BIT: u32 = 2;

/* Power Management for Analog 2 (0x64) */
pub const RT5660_PWR_BST1: u32 = (0x1u32 << 15);
pub const RT5660_PWR_BST1_BIT: u32 = 15;
pub const RT5660_PWR_BST2: u32 = (0x1u32 << 14);
pub const RT5660_PWR_BST2_BIT: u32 = 14;
pub const RT5660_PWR_BST3: u32 = (0x1u32 << 13);
pub const RT5660_PWR_BST3_BIT: u32 = 13;
pub const RT5660_PWR_MB1: u32 = (0x1u32 << 11);
pub const RT5660_PWR_MB1_BIT: u32 = 11;
pub const RT5660_PWR_MB2: u32 = (0x1u32 << 10);
pub const RT5660_PWR_MB2_BIT: u32 = 10;
pub const RT5660_PWR_PLL: u32 = (0x1u32 << 9);
pub const RT5660_PWR_PLL_BIT: u32 = 9;

/* Power Management for Mixer (0x65) */
pub const RT5660_PWR_OM_L: u32 = (0x1u32 << 15);
pub const RT5660_PWR_OM_L_BIT: u32 = 15;
pub const RT5660_PWR_OM_R: u32 = (0x1u32 << 14);
pub const RT5660_PWR_OM_R_BIT: u32 = 14;
pub const RT5660_PWR_SM: u32 = (0x1u32 << 13);
pub const RT5660_PWR_SM_BIT: u32 = 13;
pub const RT5660_PWR_RM_L: u32 = (0x1u32 << 11);
pub const RT5660_PWR_RM_L_BIT: u32 = 11;
pub const RT5660_PWR_RM_R: u32 = (0x1u32 << 10);
pub const RT5660_PWR_RM_R_BIT: u32 = 10;

/* Power Management for Volume (0x66) */
pub const RT5660_PWR_SV: u32 = (0x1u32 << 15);
pub const RT5660_PWR_SV_BIT: u32 = 15;
pub const RT5660_PWR_LV_L: u32 = (0x1u32 << 11);
pub const RT5660_PWR_LV_L_BIT: u32 = 11;
pub const RT5660_PWR_LV_R: u32 = (0x1u32 << 10);
pub const RT5660_PWR_LV_R_BIT: u32 = 10;

/* I2S1 Audio Serial Data Port Control (0x70) */
pub const RT5660_I2S_MS_MASK: u32 = (0x1u32 << 15);
pub const RT5660_I2S_MS_SFT: u32 = 15;
pub const RT5660_I2S_MS_M: u32 = (0x0u32 << 15);
pub const RT5660_I2S_MS_S: u32 = (0x1u32 << 15);
pub const RT5660_I2S_O_CP_MASK: u32 = (0x3u32 << 10);
pub const RT5660_I2S_O_CP_SFT: u32 = 10;
pub const RT5660_I2S_O_CP_OFF: u32 = (0x0u32 << 10);
pub const RT5660_I2S_O_CP_U_LAW: u32 = (0x1u32 << 10);
pub const RT5660_I2S_O_CP_A_LAW: u32 = (0x2u32 << 10);
pub const RT5660_I2S_I_CP_MASK: u32 = (0x3u32 << 8);
pub const RT5660_I2S_I_CP_SFT: u32 = 8;
pub const RT5660_I2S_I_CP_OFF: u32 = (0x0u32 << 8);
pub const RT5660_I2S_I_CP_U_LAW: u32 = (0x1u32 << 8);
pub const RT5660_I2S_I_CP_A_LAW: u32 = (0x2u32 << 8);
pub const RT5660_I2S_BP_MASK: u32 = (0x1u32 << 7);
pub const RT5660_I2S_BP_SFT: u32 = 7;
pub const RT5660_I2S_BP_NOR: u32 = (0x0u32 << 7);
pub const RT5660_I2S_BP_INV: u32 = (0x1u32 << 7);
pub const RT5660_I2S_DL_MASK: u32 = (0x3u32 << 2);
pub const RT5660_I2S_DL_SFT: u32 = 2;
pub const RT5660_I2S_DL_16: u32 = (0x0u32 << 2);
pub const RT5660_I2S_DL_20: u32 = (0x1u32 << 2);
pub const RT5660_I2S_DL_24: u32 = (0x2u32 << 2);
pub const RT5660_I2S_DL_8: u32 = (0x3u32 << 2);
pub const RT5660_I2S_DF_MASK: u32 = (0x3u32);
pub const RT5660_I2S_DF_SFT: u32 = 0;
pub const RT5660_I2S_DF_I2S: u32 = (0x0u32);
pub const RT5660_I2S_DF_LEFT: u32 = (0x1u32);
pub const RT5660_I2S_DF_PCM_A: u32 = (0x2u32);
pub const RT5660_I2S_DF_PCM_B: u32 = (0x3u32);

/* ADC/DAC Clock Control 1 (0x73) */
pub const RT5660_I2S_BCLK_MS1_MASK: u32 = (0x1u32 << 15);
pub const RT5660_I2S_BCLK_MS1_SFT: u32 = 15;
pub const RT5660_I2S_BCLK_MS1_32: u32 = (0x0u32 << 15);
pub const RT5660_I2S_BCLK_MS1_64: u32 = (0x1u32 << 15);
pub const RT5660_I2S_PD1_MASK: u32 = (0x7u32 << 12);
pub const RT5660_I2S_PD1_SFT: u32 = 12;
pub const RT5660_I2S_PD1_1: u32 = (0x0u32 << 12);
pub const RT5660_I2S_PD1_2: u32 = (0x1u32 << 12);
pub const RT5660_I2S_PD1_3: u32 = (0x2u32 << 12);
pub const RT5660_I2S_PD1_4: u32 = (0x3u32 << 12);
pub const RT5660_I2S_PD1_6: u32 = (0x4u32 << 12);
pub const RT5660_I2S_PD1_8: u32 = (0x5u32 << 12);
pub const RT5660_I2S_PD1_12: u32 = (0x6u32 << 12);
pub const RT5660_I2S_PD1_16: u32 = (0x7u32 << 12);
pub const RT5660_DAC_OSR_MASK: u32 = (0x3u32 << 2);
pub const RT5660_DAC_OSR_SFT: u32 = 2;
pub const RT5660_DAC_OSR_128: u32 = (0x0u32 << 2);
pub const RT5660_DAC_OSR_64: u32 = (0x1u32 << 2);
pub const RT5660_DAC_OSR_32: u32 = (0x2u32 << 2);
pub const RT5660_DAC_OSR_16: u32 = (0x3u32 << 2);
pub const RT5660_ADC_OSR_MASK: u32 = (0x3u32);
pub const RT5660_ADC_OSR_SFT: u32 = 0;
pub const RT5660_ADC_OSR_128: u32 = (0x0u32);
pub const RT5660_ADC_OSR_64: u32 = (0x1u32);
pub const RT5660_ADC_OSR_32: u32 = (0x2u32);
pub const RT5660_ADC_OSR_16: u32 = (0x3u32);

/* ADC/DAC Clock Control 2 (0x74) */
pub const RT5660_RESET_ADF: u32 = (0x1u32 << 13);
pub const RT5660_RESET_ADF_SFT: u32 = 13;
pub const RT5660_RESET_DAF: u32 = (0x1u32 << 12);
pub const RT5660_RESET_DAF_SFT: u32 = 12;
pub const RT5660_DAHPF_EN: u32 = (0x1u32 << 11);
pub const RT5660_DAHPF_EN_SFT: u32 = 11;
pub const RT5660_ADHPF_EN: u32 = (0x1u32 << 10);
pub const RT5660_ADHPF_EN_SFT: u32 = 10;

/* Digital Microphone Control (0x75) */
pub const RT5660_DMIC_1_EN_MASK: u32 = (0x1u32 << 15);
pub const RT5660_DMIC_1_EN_SFT: u32 = 15;
pub const RT5660_DMIC_1_DIS: u32 = (0x0u32 << 15);
pub const RT5660_DMIC_1_EN: u32 = (0x1u32 << 15);
pub const RT5660_DMIC_1L_LH_MASK: u32 = (0x1u32 << 13);
pub const RT5660_DMIC_1L_LH_SFT: u32 = 13;
pub const RT5660_DMIC_1L_LH_RISING: u32 = (0x0u32 << 13);
pub const RT5660_DMIC_1L_LH_FALLING: u32 = (0x1u32 << 13);
pub const RT5660_DMIC_1R_LH_MASK: u32 = (0x1u32 << 12);
pub const RT5660_DMIC_1R_LH_SFT: u32 = 12;
pub const RT5660_DMIC_1R_LH_RISING: u32 = (0x0u32 << 12);
pub const RT5660_DMIC_1R_LH_FALLING: u32 = (0x1u32 << 12);
pub const RT5660_SEL_DMIC_DATA_MASK: u32 = (0x1u32 << 11);
pub const RT5660_SEL_DMIC_DATA_SFT: u32 = 11;
pub const RT5660_SEL_DMIC_DATA_GPIO2: u32 = (0x0u32 << 11);
pub const RT5660_SEL_DMIC_DATA_IN1P: u32 = (0x1u32 << 11);
pub const RT5660_DMIC_CLK_MASK: u32 = (0x7u32 << 5);
pub const RT5660_DMIC_CLK_SFT: u32 = 5;

/* Global Clock Control (0x80) */
pub const RT5660_SCLK_SRC_MASK: u32 = (0x3u32 << 14);
pub const RT5660_SCLK_SRC_SFT: u32 = 14;
pub const RT5660_SCLK_SRC_MCLK: u32 = (0x0u32 << 14);
pub const RT5660_SCLK_SRC_PLL1: u32 = (0x1u32 << 14);
pub const RT5660_SCLK_SRC_RCCLK: u32 = (0x2u32 << 14);
pub const RT5660_PLL1_SRC_MASK: u32 = (0x3u32 << 12);
pub const RT5660_PLL1_SRC_SFT: u32 = 12;
pub const RT5660_PLL1_SRC_MCLK: u32 = (0x0u32 << 12);
pub const RT5660_PLL1_SRC_BCLK1: u32 = (0x1u32 << 12);
pub const RT5660_PLL1_SRC_RCCLK: u32 = (0x2u32 << 12);
pub const RT5660_PLL1_PD_MASK: u32 = (0x1u32 << 3);
pub const RT5660_PLL1_PD_SFT: u32 = 3;
pub const RT5660_PLL1_PD_1: u32 = (0x0u32 << 3);
pub const RT5660_PLL1_PD_2: u32 = (0x1u32 << 3);

pub const RT5660_PLL_INP_MAX: u32 = 40000000;
pub const RT5660_PLL_INP_MIN: u32 = 256000;
/* PLL M/N/K Code Control 1 (0x81) */
pub const RT5660_PLL_N_MAX: u32 = 0x1ffu32;
pub const RT5660_PLL_N_MASK: u32 = ((RT5660_PLL_N_MAX as u32) << 7);
pub const RT5660_PLL_N_SFT: u32 = 7;
pub const RT5660_PLL_K_MAX: u32 = 0x1fu32;
pub const RT5660_PLL_K_MASK: u32 = RT5660_PLL_K_MAX;
pub const RT5660_PLL_K_SFT: u32 = 0;

/* PLL M/N/K Code Control 2 (0x82) */
pub const RT5660_PLL_M_MAX: u32 = 0xfu32;
pub const RT5660_PLL_M_MASK: u32 = ((RT5660_PLL_M_MAX as u32) << 12);
pub const RT5660_PLL_M_SFT: u32 = 12;
pub const RT5660_PLL_M_BP: u32 = (0x1u32 << 11);
pub const RT5660_PLL_M_BP_SFT: u32 = 11;

/* Class D Over Current Control (0x8c) */
pub const RT5660_CLSD_OC_MASK: u32 = (0x1u32 << 9);
pub const RT5660_CLSD_OC_SFT: u32 = 9;
pub const RT5660_CLSD_OC_PU: u32 = (0x0u32 << 9);
pub const RT5660_CLSD_OC_PD: u32 = (0x1u32 << 9);
pub const RT5660_AUTO_PD_MASK: u32 = (0x1u32 << 8);
pub const RT5660_AUTO_PD_SFT: u32 = 8;
pub const RT5660_AUTO_PD_DIS: u32 = (0x0u32 << 8);
pub const RT5660_AUTO_PD_EN: u32 = (0x1u32 << 8);
pub const RT5660_CLSD_OC_TH_MASK: u32 = (0x3fu32);
pub const RT5660_CLSD_OC_TH_SFT: u32 = 0;

/* Class D Output Control (0x8d) */
pub const RT5660_CLSD_RATIO_MASK: u32 = (0xfu32 << 12);
pub const RT5660_CLSD_RATIO_SFT: u32 = 12;

/* Lout Amp Control 1 (0x8e) */
pub const RT5660_LOUT_CO_MASK: u32 = (0x1u32 << 4);
pub const RT5660_LOUT_CO_SFT: u32 = 4;
pub const RT5660_LOUT_CO_DIS: u32 = (0x0u32 << 4);
pub const RT5660_LOUT_CO_EN: u32 = (0x1u32 << 4);
pub const RT5660_LOUT_CB_MASK: u32 = (0x1u32);
pub const RT5660_LOUT_CB_SFT: u32 = 0;
pub const RT5660_LOUT_CB_PD: u32 = (0x0u32);
pub const RT5660_LOUT_CB_PU: u32 = (0x1u32);

/* SPKVDD detection control (0x92) */
pub const RT5660_SPKVDD_DET_MASK: u32 = (0x1u32 << 15);
pub const RT5660_SPKVDD_DET_SFT: u32 = 15;
pub const RT5660_SPKVDD_DET_DIS: u32 = (0x0u32 << 15);
pub const RT5660_SPKVDD_DET_EN: u32 = (0x1u32 << 15);
pub const RT5660_SPK_AG_MASK: u32 = (0x1u32 << 14);
pub const RT5660_SPK_AG_SFT: u32 = 14;
pub const RT5660_SPK_AG_DIS: u32 = (0x0u32 << 14);
pub const RT5660_SPK_AG_EN: u32 = (0x1u32 << 14);

/* Micbias Control (0x93) */
pub const RT5660_MIC1_BS_MASK: u32 = (0x1u32 << 15);
pub const RT5660_MIC1_BS_SFT: u32 = 15;
pub const RT5660_MIC1_BS_9AV: u32 = (0x0u32 << 15);
pub const RT5660_MIC1_BS_75AV: u32 = (0x1u32 << 15);
pub const RT5660_MIC2_BS_MASK: u32 = (0x1u32 << 14);
pub const RT5660_MIC2_BS_SFT: u32 = 14;
pub const RT5660_MIC2_BS_9AV: u32 = (0x0u32 << 14);
pub const RT5660_MIC2_BS_75AV: u32 = (0x1u32 << 14);
pub const RT5660_MIC1_OVCD_MASK: u32 = (0x1u32 << 11);
pub const RT5660_MIC1_OVCD_SFT: u32 = 11;
pub const RT5660_MIC1_OVCD_DIS: u32 = (0x0u32 << 11);
pub const RT5660_MIC1_OVCD_EN: u32 = (0x1u32 << 11);
pub const RT5660_MIC1_OVTH_MASK: u32 = (0x3u32 << 9);
pub const RT5660_MIC1_OVTH_SFT: u32 = 9;
pub const RT5660_MIC1_OVTH_600UA: u32 = (0x0u32 << 9);
pub const RT5660_MIC1_OVTH_1500UA: u32 = (0x1u32 << 9);
pub const RT5660_MIC1_OVTH_2000UA: u32 = (0x2u32 << 9);
pub const RT5660_MIC2_OVCD_MASK: u32 = (0x1u32 << 8);
pub const RT5660_MIC2_OVCD_SFT: u32 = 8;
pub const RT5660_MIC2_OVCD_DIS: u32 = (0x0u32 << 8);
pub const RT5660_MIC2_OVCD_EN: u32 = (0x1u32 << 8);
pub const RT5660_MIC2_OVTH_MASK: u32 = (0x3u32 << 6);
pub const RT5660_MIC2_OVTH_SFT: u32 = 6;
pub const RT5660_MIC2_OVTH_600UA: u32 = (0x0u32 << 6);
pub const RT5660_MIC2_OVTH_1500UA: u32 = (0x1u32 << 6);
pub const RT5660_MIC2_OVTH_2000UA: u32 = (0x2u32 << 6);
pub const RT5660_PWR_CLK25M_MASK: u32 = (0x1u32 << 4);
pub const RT5660_PWR_CLK25M_SFT: u32 = 4;
pub const RT5660_PWR_CLK25M_PD: u32 = (0x0u32 << 4);
pub const RT5660_PWR_CLK25M_PU: u32 = (0x1u32 << 4);

/* EQ Control 1 (0xb0) */
pub const RT5660_EQ_SRC_MASK: u32 = (0x1u32 << 15);
pub const RT5660_EQ_SRC_SFT: u32 = 15;
pub const RT5660_EQ_SRC_DAC: u32 = (0x0u32 << 15);
pub const RT5660_EQ_SRC_ADC: u32 = (0x1u32 << 15);
pub const RT5660_EQ_UPD: u32 = (0x1u32 << 14);
pub const RT5660_EQ_UPD_BIT: u32 = 14;

/* Jack Detect Control (0xbb) */
pub const RT5660_JD_MASK: u32 = (0x3u32 << 14);
pub const RT5660_JD_SFT: u32 = 14;
pub const RT5660_JD_DIS: u32 = (0x0u32 << 14);
pub const RT5660_JD_GPIO1: u32 = (0x1u32 << 14);
pub const RT5660_JD_GPIO2: u32 = (0x2u32 << 14);
pub const RT5660_JD_LOUT_MASK: u32 = (0x1u32 << 11);
pub const RT5660_JD_LOUT_SFT: u32 = 11;
pub const RT5660_JD_LOUT_DIS: u32 = (0x0u32 << 11);
pub const RT5660_JD_LOUT_EN: u32 = (0x1u32 << 11);
pub const RT5660_JD_LOUT_TRG_MASK: u32 = (0x1u32 << 10);
pub const RT5660_JD_LOUT_TRG_SFT: u32 = 10;
pub const RT5660_JD_LOUT_TRG_LO: u32 = (0x0u32 << 10);
pub const RT5660_JD_LOUT_TRG_HI: u32 = (0x1u32 << 10);
pub const RT5660_JD_SPO_MASK: u32 = (0x1u32 << 9);
pub const RT5660_JD_SPO_SFT: u32 = 9;
pub const RT5660_JD_SPO_DIS: u32 = (0x0u32 << 9);
pub const RT5660_JD_SPO_EN: u32 = (0x1u32 << 9);
pub const RT5660_JD_SPO_TRG_MASK: u32 = (0x1u32 << 8);
pub const RT5660_JD_SPO_TRG_SFT: u32 = 8;
pub const RT5660_JD_SPO_TRG_LO: u32 = (0x0u32 << 8);
pub const RT5660_JD_SPO_TRG_HI: u32 = (0x1u32 << 8);

/* IRQ Control 1 (0xbd) */
pub const RT5660_IRQ_JD_MASK: u32 = (0x1u32 << 15);
pub const RT5660_IRQ_JD_SFT: u32 = 15;
pub const RT5660_IRQ_JD_BP: u32 = (0x0u32 << 15);
pub const RT5660_IRQ_JD_NOR: u32 = (0x1u32 << 15);
pub const RT5660_IRQ_OT_MASK: u32 = (0x1u32 << 14);
pub const RT5660_IRQ_OT_SFT: u32 = 14;
pub const RT5660_IRQ_OT_BP: u32 = (0x0u32 << 14);
pub const RT5660_IRQ_OT_NOR: u32 = (0x1u32 << 14);
pub const RT5660_JD_STKY_MASK: u32 = (0x1u32 << 13);
pub const RT5660_JD_STKY_SFT: u32 = 13;
pub const RT5660_JD_STKY_DIS: u32 = (0x0u32 << 13);
pub const RT5660_JD_STKY_EN: u32 = (0x1u32 << 13);
pub const RT5660_OT_STKY_MASK: u32 = (0x1u32 << 12);
pub const RT5660_OT_STKY_SFT: u32 = 12;
pub const RT5660_OT_STKY_DIS: u32 = (0x0u32 << 12);
pub const RT5660_OT_STKY_EN: u32 = (0x1u32 << 12);
pub const RT5660_JD_P_MASK: u32 = (0x1u32 << 11);
pub const RT5660_JD_P_SFT: u32 = 11;
pub const RT5660_JD_P_NOR: u32 = (0x0u32 << 11);
pub const RT5660_JD_P_INV: u32 = (0x1u32 << 11);
pub const RT5660_OT_P_MASK: u32 = (0x1u32 << 10);
pub const RT5660_OT_P_SFT: u32 = 10;
pub const RT5660_OT_P_NOR: u32 = (0x0u32 << 10);
pub const RT5660_OT_P_INV: u32 = (0x1u32 << 10);

/* IRQ Control 2 (0xbe) */
pub const RT5660_IRQ_MB1_OC_MASK: u32 = (0x1u32 << 15);
pub const RT5660_IRQ_MB1_OC_SFT: u32 = 15;
pub const RT5660_IRQ_MB1_OC_BP: u32 = (0x0u32 << 15);
pub const RT5660_IRQ_MB1_OC_NOR: u32 = (0x1u32 << 15);
pub const RT5660_IRQ_MB2_OC_MASK: u32 = (0x1u32 << 14);
pub const RT5660_IRQ_MB2_OC_SFT: u32 = 14;
pub const RT5660_IRQ_MB2_OC_BP: u32 = (0x0u32 << 14);
pub const RT5660_IRQ_MB2_OC_NOR: u32 = (0x1u32 << 14);
pub const RT5660_MB1_OC_STKY_MASK: u32 = (0x1u32 << 11);
pub const RT5660_MB1_OC_STKY_SFT: u32 = 11;
pub const RT5660_MB1_OC_STKY_DIS: u32 = (0x0u32 << 11);
pub const RT5660_MB1_OC_STKY_EN: u32 = (0x1u32 << 11);
pub const RT5660_MB2_OC_STKY_MASK: u32 = (0x1u32 << 10);
pub const RT5660_MB2_OC_STKY_SFT: u32 = 10;
pub const RT5660_MB2_OC_STKY_DIS: u32 = (0x0u32 << 10);
pub const RT5660_MB2_OC_STKY_EN: u32 = (0x1u32 << 10);
pub const RT5660_MB1_OC_P_MASK: u32 = (0x1u32 << 7);
pub const RT5660_MB1_OC_P_SFT: u32 = 7;
pub const RT5660_MB1_OC_P_NOR: u32 = (0x0u32 << 7);
pub const RT5660_MB1_OC_P_INV: u32 = (0x1u32 << 7);
pub const RT5660_MB2_OC_P_MASK: u32 = (0x1u32 << 6);
pub const RT5660_MB2_OC_P_SFT: u32 = 6;
pub const RT5660_MB2_OC_P_NOR: u32 = (0x0u32 << 6);
pub const RT5660_MB2_OC_P_INV: u32 = (0x1u32 << 6);
pub const RT5660_MB1_OC_CLR: u32 = (0x1u32 << 3);
pub const RT5660_MB1_OC_CLR_SFT: u32 = 3;
pub const RT5660_MB2_OC_CLR: u32 = (0x1u32 << 2);
pub const RT5660_MB2_OC_CLR_SFT: u32 = 2;

/* GPIO Control 1 (0xc0) */
pub const RT5660_GP2_PIN_MASK: u32 = (0x1u32 << 14);
pub const RT5660_GP2_PIN_SFT: u32 = 14;
pub const RT5660_GP2_PIN_GPIO2: u32 = (0x0u32 << 14);
pub const RT5660_GP2_PIN_DMIC1_SDA: u32 = (0x1u32 << 14);
pub const RT5660_GP1_PIN_MASK: u32 = (0x3u32 << 12);
pub const RT5660_GP1_PIN_SFT: u32 = 12;
pub const RT5660_GP1_PIN_GPIO1: u32 = (0x0u32 << 12);
pub const RT5660_GP1_PIN_DMIC1_SCL: u32 = (0x1u32 << 12);
pub const RT5660_GP1_PIN_IRQ: u32 = (0x2u32 << 12);
pub const RT5660_GPIO_M_MASK: u32 = (0x1u32 << 9);
pub const RT5660_GPIO_M_SFT: u32 = 9;
pub const RT5660_GPIO_M_FLT: u32 = (0x0u32 << 9);
pub const RT5660_GPIO_M_PH: u32 = (0x1u32 << 9);

/* GPIO Control 3 (0xc2) */
pub const RT5660_GP2_PF_MASK: u32 = (0x1u32 << 5);
pub const RT5660_GP2_PF_SFT: u32 = 5;
pub const RT5660_GP2_PF_IN: u32 = (0x0u32 << 5);
pub const RT5660_GP2_PF_OUT: u32 = (0x1u32 << 5);
pub const RT5660_GP2_OUT_MASK: u32 = (0x1u32 << 4);
pub const RT5660_GP2_OUT_SFT: u32 = 4;
pub const RT5660_GP2_OUT_LO: u32 = (0x0u32 << 4);
pub const RT5660_GP2_OUT_HI: u32 = (0x1u32 << 4);
pub const RT5660_GP2_P_MASK: u32 = (0x1u32 << 3);
pub const RT5660_GP2_P_SFT: u32 = 3;
pub const RT5660_GP2_P_NOR: u32 = (0x0u32 << 3);
pub const RT5660_GP2_P_INV: u32 = (0x1u32 << 3);
pub const RT5660_GP1_PF_MASK: u32 = (0x1u32 << 2);
pub const RT5660_GP1_PF_SFT: u32 = 2;
pub const RT5660_GP1_PF_IN: u32 = (0x0u32 << 2);
pub const RT5660_GP1_PF_OUT: u32 = (0x1u32 << 2);
pub const RT5660_GP1_OUT_MASK: u32 = (0x1u32 << 1);
pub const RT5660_GP1_OUT_SFT: u32 = 1;
pub const RT5660_GP1_OUT_LO: u32 = (0x0u32 << 1);
pub const RT5660_GP1_OUT_HI: u32 = (0x1u32 << 1);
pub const RT5660_GP1_P_MASK: u32 = (0x1u32);
pub const RT5660_GP1_P_SFT: u32 = 0;
pub const RT5660_GP1_P_NOR: u32 = (0x0u32);
pub const RT5660_GP1_P_INV: u32 = (0x1u32);

/* Soft volume and zero cross control 1 (0xd9) */
pub const RT5660_SV_MASK: u32 = (0x1u32 << 15);
pub const RT5660_SV_SFT: u32 = 15;
pub const RT5660_SV_DIS: u32 = (0x0u32 << 15);
pub const RT5660_SV_EN: u32 = (0x1u32 << 15);
pub const RT5660_SPO_SV_MASK: u32 = (0x1u32 << 14);
pub const RT5660_SPO_SV_SFT: u32 = 14;
pub const RT5660_SPO_SV_DIS: u32 = (0x0u32 << 14);
pub const RT5660_SPO_SV_EN: u32 = (0x1u32 << 14);
pub const RT5660_OUT_SV_MASK: u32 = (0x1u32 << 12);
pub const RT5660_OUT_SV_SFT: u32 = 12;
pub const RT5660_OUT_SV_DIS: u32 = (0x0u32 << 12);
pub const RT5660_OUT_SV_EN: u32 = (0x1u32 << 12);
pub const RT5660_ZCD_DIG_MASK: u32 = (0x1u32 << 11);
pub const RT5660_ZCD_DIG_SFT: u32 = 11;
pub const RT5660_ZCD_DIG_DIS: u32 = (0x0u32 << 11);
pub const RT5660_ZCD_DIG_EN: u32 = (0x1u32 << 11);
pub const RT5660_ZCD_MASK: u32 = (0x1u32 << 10);
pub const RT5660_ZCD_SFT: u32 = 10;
pub const RT5660_ZCD_PD: u32 = (0x0u32 << 10);
pub const RT5660_ZCD_PU: u32 = (0x1u32 << 10);
pub const RT5660_SV_DLY_MASK: u32 = (0xfu32);
pub const RT5660_SV_DLY_SFT: u32 = 0;

/* Soft volume and zero cross control 2 (0xda) */
pub const RT5660_ZCD_SPO_MASK: u32 = (0x1u32 << 15);
pub const RT5660_ZCD_SPO_SFT: u32 = 15;
pub const RT5660_ZCD_SPO_DIS: u32 = (0x0u32 << 15);
pub const RT5660_ZCD_SPO_EN: u32 = (0x1u32 << 15);
pub const RT5660_ZCD_OMR_MASK: u32 = (0x1u32 << 8);
pub const RT5660_ZCD_OMR_SFT: u32 = 8;
pub const RT5660_ZCD_OMR_DIS: u32 = (0x0u32 << 8);
pub const RT5660_ZCD_OMR_EN: u32 = (0x1u32 << 8);
pub const RT5660_ZCD_OML_MASK: u32 = (0x1u32 << 7);
pub const RT5660_ZCD_OML_SFT: u32 = 7;
pub const RT5660_ZCD_OML_DIS: u32 = (0x0u32 << 7);
pub const RT5660_ZCD_OML_EN: u32 = (0x1u32 << 7);
pub const RT5660_ZCD_SPM_MASK: u32 = (0x1u32 << 6);
pub const RT5660_ZCD_SPM_SFT: u32 = 6;
pub const RT5660_ZCD_SPM_DIS: u32 = (0x0u32 << 6);
pub const RT5660_ZCD_SPM_EN: u32 = (0x1u32 << 6);
pub const RT5660_ZCD_RMR_MASK: u32 = (0x1u32 << 5);
pub const RT5660_ZCD_RMR_SFT: u32 = 5;
pub const RT5660_ZCD_RMR_DIS: u32 = (0x0u32 << 5);
pub const RT5660_ZCD_RMR_EN: u32 = (0x1u32 << 5);
pub const RT5660_ZCD_RML_MASK: u32 = (0x1u32 << 4);
pub const RT5660_ZCD_RML_SFT: u32 = 4;
pub const RT5660_ZCD_RML_DIS: u32 = (0x0u32 << 4);
pub const RT5660_ZCD_RML_EN: u32 = (0x1u32 << 4);

/* General Control 1 (0xfa) */
pub const RT5660_PWR_VREF_HP: u32 = (0x1u32 << 11);
pub const RT5660_PWR_VREF_HP_SFT: u32 = 11;
pub const RT5660_AUTO_DIS_AMP: u32 = (0x1u32 << 6);
pub const RT5660_MCLK_DET: u32 = (0x1u32 << 5);
pub const RT5660_POW_CLKDET: u32 = (0x1u32 << 1);
pub const RT5660_DIG_GATE_CTRL: u32 = (0x1u32);
pub const RT5660_DIG_GATE_CTRL_SFT: u32 = 0;

/* System Clock Source */
pub const RT5660_SCLK_S_MCLK: u32 = 0;
pub const RT5660_SCLK_S_PLL1: u32 = 1;
pub const RT5660_SCLK_S_RCCLK: u32 = 2;

/* PLL1 Source */
pub const RT5660_PLL1_S_MCLK: u32 = 0;
pub const RT5660_PLL1_S_BCLK: u32 = 1;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Rt5660Aif {
    RT5660_AIF1 = 0,
    RT5660_AIFS = 1,
}

pub const RT5660_AIF1: usize = Rt5660Aif::RT5660_AIF1 as usize;
pub const RT5660_AIFS: usize = Rt5660Aif::RT5660_AIFS as usize;


#[repr(C)]
pub struct rt5660_priv {
    pub component: *mut snd_soc_component,
    pub pdata: rt5660_platform_data,
    pub regmap: *mut regmap,
    pub mclk: *mut clk,

    pub sysclk: i32,
    pub sysclk_src: i32,
    pub lrck: [i32; RT5660_AIFS],
    pub bclk: [i32; RT5660_AIFS],
    pub master: [i32; RT5660_AIFS],

    pub pll_src: i32,
    pub pll_in: i32,
    pub pll_out: i32,
}


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
