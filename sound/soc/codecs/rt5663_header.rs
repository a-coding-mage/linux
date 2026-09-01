/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rt5663.h  --  RT5663 ALSA SoC audio driver
 *
 * Copyright 2016 Realtek Microelectronics
 * Author: Jack Yu <jack.yu@realtek.com>
 */

/* Translated from C header ./rt5663.h. */
/* Dependency from C include <sound/rt5663.h> is expected to be provided externally. */

/*
 *
 */



/* Info */
pub const RT5663_RESET: u32 = 0x0000u32;
pub const RT5663_VENDOR_ID: u32 = 0x00fdu32;
pub const RT5663_VENDOR_ID_1: u32 = 0x00feu32;
pub const RT5663_VENDOR_ID_2: u32 = 0x00ffu32;

pub const RT5663_LOUT_CTRL: u32 = 0x0001u32;
pub const RT5663_HP_AMP_2: u32 = 0x0003u32;
pub const RT5663_MONO_OUT: u32 = 0x0004u32;
pub const RT5663_MONO_GAIN: u32 = 0x0007u32;

pub const RT5663_AEC_BST: u32 = 0x000bu32;
pub const RT5663_IN1_IN2: u32 = 0x000cu32;
pub const RT5663_IN3_IN4: u32 = 0x000du32;
pub const RT5663_INL1_INR1: u32 = 0x000fu32;
pub const RT5663_CBJ_TYPE_2: u32 = 0x0011u32;
pub const RT5663_CBJ_TYPE_3: u32 = 0x0012u32;
pub const RT5663_CBJ_TYPE_4: u32 = 0x0013u32;
pub const RT5663_CBJ_TYPE_5: u32 = 0x0014u32;
pub const RT5663_CBJ_TYPE_8: u32 = 0x0017u32;

/* I/O - ADC/DAC/DMIC */
pub const RT5663_DAC3_DIG_VOL: u32 = 0x001au32;
pub const RT5663_DAC3_CTRL: u32 = 0x001bu32;
pub const RT5663_MONO_ADC_DIG_VOL: u32 = 0x001du32;
pub const RT5663_STO2_ADC_DIG_VOL: u32 = 0x001eu32;
pub const RT5663_MONO_ADC_BST_GAIN: u32 = 0x0020u32;
pub const RT5663_STO2_ADC_BST_GAIN: u32 = 0x0021u32;
pub const RT5663_SIDETONE_CTRL: u32 = 0x0024u32;
/* Mixer - D-D */
pub const RT5663_MONO1_ADC_MIXER: u32 = 0x0027u32;
pub const RT5663_STO2_ADC_MIXER: u32 = 0x0028u32;
pub const RT5663_MONO_DAC_MIXER: u32 = 0x002bu32;
pub const RT5663_DAC2_SRC_CTRL: u32 = 0x002eu32;
pub const RT5663_IF_3_4_DATA_CTL: u32 = 0x002fu32;
pub const RT5663_IF_5_DATA_CTL: u32 = 0x0030u32;
pub const RT5663_PDM_OUT_CTL: u32 = 0x0031u32;
pub const RT5663_PDM_I2C_DATA_CTL1: u32 = 0x0032u32;
pub const RT5663_PDM_I2C_DATA_CTL2: u32 = 0x0033u32;
pub const RT5663_PDM_I2C_DATA_CTL3: u32 = 0x0034u32;
pub const RT5663_PDM_I2C_DATA_CTL4: u32 = 0x0035u32;

/*Mixer - Analog*/
pub const RT5663_RECMIX1_NEW: u32 = 0x003au32;
pub const RT5663_RECMIX1L_0: u32 = 0x003bu32;
pub const RT5663_RECMIX1L: u32 = 0x003cu32;
pub const RT5663_RECMIX1R_0: u32 = 0x003du32;
pub const RT5663_RECMIX1R: u32 = 0x003eu32;
pub const RT5663_RECMIX2_NEW: u32 = 0x003fu32;
pub const RT5663_RECMIX2_L_2: u32 = 0x0041u32;
pub const RT5663_RECMIX2_R: u32 = 0x0042u32;
pub const RT5663_RECMIX2_R_2: u32 = 0x0043u32;
pub const RT5663_CALIB_REC_LR: u32 = 0x0044u32;
pub const RT5663_ALC_BK_GAIN: u32 = 0x0049u32;
pub const RT5663_MONOMIX_GAIN: u32 = 0x004au32;
pub const RT5663_MONOMIX_IN_GAIN: u32 = 0x004bu32;
pub const RT5663_OUT_MIXL_GAIN: u32 = 0x004du32;
pub const RT5663_OUT_LMIX_IN_GAIN: u32 = 0x004eu32;
pub const RT5663_OUT_RMIX_IN_GAIN: u32 = 0x004fu32;
pub const RT5663_OUT_RMIX_IN_GAIN1: u32 = 0x0050u32;
pub const RT5663_LOUT_MIXER_CTRL: u32 = 0x0052u32;
/* Power */
pub const RT5663_PWR_VOL: u32 = 0x0067u32;

pub const RT5663_ADCDAC_RST: u32 = 0x006du32;
/* Format - ADC/DAC */
pub const RT5663_I2S34_SDP: u32 = 0x0071u32;
pub const RT5663_I2S5_SDP: u32 = 0x0072u32;

/* Function - Analog */
pub const RT5663_ASRC_3: u32 = 0x0085u32;
pub const RT5663_ASRC_6: u32 = 0x0088u32;
pub const RT5663_ASRC_7: u32 = 0x0089u32;
pub const RT5663_PLL_TRK_13: u32 = 0x0099u32;
pub const RT5663_I2S_M_CLK_CTL: u32 = 0x00a0u32;
pub const RT5663_FDIV_I2S34_M_CLK: u32 = 0x00a1u32;
pub const RT5663_FDIV_I2S34_M_CLK2: u32 = 0x00a2u32;
pub const RT5663_FDIV_I2S5_M_CLK: u32 = 0x00a3u32;
pub const RT5663_FDIV_I2S5_M_CLK2: u32 = 0x00a4u32;

/* Function - Digital */
pub const RT5663_V2_IRQ_4: u32 = 0x00b9u32;
pub const RT5663_GPIO_3: u32 = 0x00c2u32;
pub const RT5663_GPIO_4: u32 = 0x00c3u32;
pub const RT5663_GPIO_STA2: u32 = 0x00c4u32;
pub const RT5663_HP_AMP_DET1: u32 = 0x00d0u32;
pub const RT5663_HP_AMP_DET2: u32 = 0x00d1u32;
pub const RT5663_HP_AMP_DET3: u32 = 0x00d2u32;
pub const RT5663_MID_BD_HP_AMP: u32 = 0x00d3u32;
pub const RT5663_LOW_BD_HP_AMP: u32 = 0x00d4u32;
pub const RT5663_SOF_VOL_ZC2: u32 = 0x00dau32;
pub const RT5663_ADC_STO2_ADJ1: u32 = 0x00eeu32;
pub const RT5663_ADC_STO2_ADJ2: u32 = 0x00efu32;
/* General Control */
pub const RT5663_A_JD_CTRL: u32 = 0x00f0u32;
pub const RT5663_JD1_TRES_CTRL: u32 = 0x00f1u32;
pub const RT5663_JD2_TRES_CTRL: u32 = 0x00f2u32;
pub const RT5663_V2_JD_CTRL2: u32 = 0x00f7u32;
pub const RT5663_DUM_REG_2: u32 = 0x00fbu32;
pub const RT5663_DUM_REG_3: u32 = 0x00fcu32;


pub const RT5663_DACADC_DIG_VOL2: u32 = 0x0101u32;
pub const RT5663_DIG_IN_PIN2: u32 = 0x0133u32;
pub const RT5663_PAD_DRV_CTL1: u32 = 0x0136u32;
pub const RT5663_SOF_RAM_DEPOP: u32 = 0x0138u32;
pub const RT5663_VOL_TEST: u32 = 0x013fu32;
pub const RT5663_MONO_DYNA_1: u32 = 0x0170u32;
pub const RT5663_MONO_DYNA_2: u32 = 0x0171u32;
pub const RT5663_MONO_DYNA_3: u32 = 0x0172u32;
pub const RT5663_MONO_DYNA_4: u32 = 0x0173u32;
pub const RT5663_MONO_DYNA_5: u32 = 0x0174u32;
pub const RT5663_MONO_DYNA_6: u32 = 0x0175u32;
pub const RT5663_STO1_SIL_DET: u32 = 0x0190u32;
pub const RT5663_MONOL_SIL_DET: u32 = 0x0191u32;
pub const RT5663_MONOR_SIL_DET: u32 = 0x0192u32;
pub const RT5663_STO2_DAC_SIL: u32 = 0x0193u32;
pub const RT5663_PWR_SAV_CTL1: u32 = 0x0194u32;
pub const RT5663_PWR_SAV_CTL2: u32 = 0x0195u32;
pub const RT5663_PWR_SAV_CTL3: u32 = 0x0196u32;
pub const RT5663_PWR_SAV_CTL4: u32 = 0x0197u32;
pub const RT5663_PWR_SAV_CTL5: u32 = 0x0198u32;
pub const RT5663_PWR_SAV_CTL6: u32 = 0x0199u32;
pub const RT5663_MONO_AMP_CAL1: u32 = 0x01a0u32;
pub const RT5663_MONO_AMP_CAL2: u32 = 0x01a1u32;
pub const RT5663_MONO_AMP_CAL3: u32 = 0x01a2u32;
pub const RT5663_MONO_AMP_CAL4: u32 = 0x01a3u32;
pub const RT5663_MONO_AMP_CAL5: u32 = 0x01a4u32;
pub const RT5663_MONO_AMP_CAL6: u32 = 0x01a5u32;
pub const RT5663_MONO_AMP_CAL7: u32 = 0x01a6u32;
pub const RT5663_MONO_AMP_CAL_ST1: u32 = 0x01a7u32;
pub const RT5663_MONO_AMP_CAL_ST2: u32 = 0x01a8u32;
pub const RT5663_MONO_AMP_CAL_ST3: u32 = 0x01a9u32;
pub const RT5663_MONO_AMP_CAL_ST4: u32 = 0x01aau32;
pub const RT5663_MONO_AMP_CAL_ST5: u32 = 0x01abu32;
pub const RT5663_V2_HP_IMP_SEN_13: u32 = 0x01b9u32;
pub const RT5663_V2_HP_IMP_SEN_14: u32 = 0x01bau32;
pub const RT5663_V2_HP_IMP_SEN_6: u32 = 0x01bbu32;
pub const RT5663_V2_HP_IMP_SEN_7: u32 = 0x01bcu32;
pub const RT5663_V2_HP_IMP_SEN_8: u32 = 0x01bdu32;
pub const RT5663_V2_HP_IMP_SEN_9: u32 = 0x01beu32;
pub const RT5663_V2_HP_IMP_SEN_10: u32 = 0x01bfu32;
pub const RT5663_HP_LOGIC_3: u32 = 0x01dcu32;
pub const RT5663_HP_CALIB_ST10: u32 = 0x01f3u32;
pub const RT5663_HP_CALIB_ST11: u32 = 0x01f4u32;
pub const RT5663_PRO_REG_TBL_4: u32 = 0x0203u32;
pub const RT5663_PRO_REG_TBL_5: u32 = 0x0204u32;
pub const RT5663_PRO_REG_TBL_6: u32 = 0x0205u32;
pub const RT5663_PRO_REG_TBL_7: u32 = 0x0206u32;
pub const RT5663_PRO_REG_TBL_8: u32 = 0x0207u32;
pub const RT5663_PRO_REG_TBL_9: u32 = 0x0208u32;
pub const RT5663_SAR_ADC_INL_1: u32 = 0x0210u32;
pub const RT5663_SAR_ADC_INL_2: u32 = 0x0211u32;
pub const RT5663_SAR_ADC_INL_3: u32 = 0x0212u32;
pub const RT5663_SAR_ADC_INL_4: u32 = 0x0213u32;
pub const RT5663_SAR_ADC_INL_5: u32 = 0x0214u32;
pub const RT5663_SAR_ADC_INL_6: u32 = 0x0215u32;
pub const RT5663_SAR_ADC_INL_7: u32 = 0x0216u32;
pub const RT5663_SAR_ADC_INL_8: u32 = 0x0217u32;
pub const RT5663_SAR_ADC_INL_9: u32 = 0x0218u32;
pub const RT5663_SAR_ADC_INL_10: u32 = 0x0219u32;
pub const RT5663_SAR_ADC_INL_11: u32 = 0x021au32;
pub const RT5663_SAR_ADC_INL_12: u32 = 0x021bu32;
pub const RT5663_DRC_CTRL_1: u32 = 0x02ffu32;
pub const RT5663_DRC1_CTRL_2: u32 = 0x0301u32;
pub const RT5663_DRC1_CTRL_3: u32 = 0x0302u32;
pub const RT5663_DRC1_CTRL_4: u32 = 0x0303u32;
pub const RT5663_DRC1_CTRL_5: u32 = 0x0304u32;
pub const RT5663_DRC1_CTRL_6: u32 = 0x0305u32;
pub const RT5663_DRC1_HD_CTRL_1: u32 = 0x0306u32;
pub const RT5663_DRC1_HD_CTRL_2: u32 = 0x0307u32;
pub const RT5663_DRC1_PRI_REG_1: u32 = 0x0310u32;
pub const RT5663_DRC1_PRI_REG_2: u32 = 0x0311u32;
pub const RT5663_DRC1_PRI_REG_3: u32 = 0x0312u32;
pub const RT5663_DRC1_PRI_REG_4: u32 = 0x0313u32;
pub const RT5663_DRC1_PRI_REG_5: u32 = 0x0314u32;
pub const RT5663_DRC1_PRI_REG_6: u32 = 0x0315u32;
pub const RT5663_DRC1_PRI_REG_7: u32 = 0x0316u32;
pub const RT5663_DRC1_PRI_REG_8: u32 = 0x0317u32;
pub const RT5663_ALC_PGA_CTL_1: u32 = 0x0330u32;
pub const RT5663_ALC_PGA_CTL_2: u32 = 0x0331u32;
pub const RT5663_ALC_PGA_CTL_3: u32 = 0x0332u32;
pub const RT5663_ALC_PGA_CTL_4: u32 = 0x0333u32;
pub const RT5663_ALC_PGA_CTL_5: u32 = 0x0334u32;
pub const RT5663_ALC_PGA_CTL_6: u32 = 0x0335u32;
pub const RT5663_ALC_PGA_CTL_7: u32 = 0x0336u32;
pub const RT5663_ALC_PGA_CTL_8: u32 = 0x0337u32;
pub const RT5663_ALC_PGA_REG_1: u32 = 0x0338u32;
pub const RT5663_ALC_PGA_REG_2: u32 = 0x0339u32;
pub const RT5663_ALC_PGA_REG_3: u32 = 0x033au32;
pub const RT5663_ADC_EQ_RECOV_1: u32 = 0x03c0u32;
pub const RT5663_ADC_EQ_RECOV_2: u32 = 0x03c1u32;
pub const RT5663_ADC_EQ_RECOV_3: u32 = 0x03c2u32;
pub const RT5663_ADC_EQ_RECOV_4: u32 = 0x03c3u32;
pub const RT5663_ADC_EQ_RECOV_5: u32 = 0x03c4u32;
pub const RT5663_ADC_EQ_RECOV_6: u32 = 0x03c5u32;
pub const RT5663_ADC_EQ_RECOV_7: u32 = 0x03c6u32;
pub const RT5663_ADC_EQ_RECOV_8: u32 = 0x03c7u32;
pub const RT5663_ADC_EQ_RECOV_9: u32 = 0x03c8u32;
pub const RT5663_ADC_EQ_RECOV_10: u32 = 0x03c9u32;
pub const RT5663_ADC_EQ_RECOV_11: u32 = 0x03cau32;
pub const RT5663_ADC_EQ_RECOV_12: u32 = 0x03cbu32;
pub const RT5663_ADC_EQ_RECOV_13: u32 = 0x03ccu32;
pub const RT5663_VID_HIDDEN: u32 = 0x03feu32;
pub const RT5663_VID_CUSTOMER: u32 = 0x03ffu32;
pub const RT5663_SCAN_MODE: u32 = 0x07f0u32;
pub const RT5663_I2C_BYPA: u32 = 0x07fau32;

/* Headphone Amp Control 2 (0x0003) */
pub const RT5663_EN_DAC_HPO_MASK: u32 = (0x1u32 << 14u32);
pub const RT5663_EN_DAC_HPO_SHIFT: u32 = 14u32;
pub const RT5663_EN_DAC_HPO_DIS: u32 = (0x0u32 << 14u32);
pub const RT5663_EN_DAC_HPO_EN: u32 = (0x1u32 << 14u32);

/*Headphone Amp L/R Analog Gain and Digital NG2 Gain Control (0x0005 0x0006)*/
pub const RT5663_GAIN_HP: u32 = (0x1fu32 << 8u32);
pub const RT5663_GAIN_HP_SHIFT: u32 = 8u32;

/* AEC BST Control (0x000b) */
pub const RT5663_GAIN_CBJ_MASK: u32 = (0xfu32 << 8u32);
pub const RT5663_GAIN_CBJ_SHIFT: u32 = 8u32;

/* IN1 Control / MIC GND REF (0x000c) */
pub const RT5663_IN1_DF_MASK: u32 = (0x1u32 << 15u32);
pub const RT5663_IN1_DF_SHIFT: u32 = 15u32;

/* Combo Jack and Type Detection Control 1 (0x0010) */
pub const RT5663_DET_TYPE_MASK: u32 = (0x1u32 << 12u32);
pub const RT5663_DET_TYPE_SHIFT: u32 = 12u32;
pub const RT5663_DET_TYPE_WLCSP: u32 = (0x0u32 << 12u32);
pub const RT5663_DET_TYPE_QFN: u32 = (0x1u32 << 12u32);
pub const RT5663_VREF_BIAS_MASK: u32 = (0x1u32 << 6u32);
pub const RT5663_VREF_BIAS_SHIFT: u32 = 6u32;
pub const RT5663_VREF_BIAS_FSM: u32 = (0x0u32 << 6u32);
pub const RT5663_VREF_BIAS_REG: u32 = (0x1u32 << 6u32);

/* REC Left Mixer Control 2 (0x003c) */
pub const RT5663_RECMIX1L_BST1_CBJ: u32 = (0x1u32 << 7u32);
pub const RT5663_RECMIX1L_BST1_CBJ_SHIFT: u32 = 7u32;
pub const RT5663_RECMIX1L_BST2: u32 = (0x1u32 << 4u32);
pub const RT5663_RECMIX1L_BST2_SHIFT: u32 = 4u32;

/* REC Right Mixer Control 2 (0x003e) */
pub const RT5663_RECMIX1R_BST2: u32 = (0x1u32 << 4u32);
pub const RT5663_RECMIX1R_BST2_SHIFT: u32 = 4u32;

/* DAC1 Digital Volume (0x0019) */
pub const RT5663_DAC_L1_VOL_MASK: u32 = (0xffu32 << 8u32);
pub const RT5663_DAC_L1_VOL_SHIFT: u32 = 8u32;
pub const RT5663_DAC_R1_VOL_MASK: u32 = (0xffu32);
pub const RT5663_DAC_R1_VOL_SHIFT: u32 = 0u32;

/* ADC Digital Volume Control (0x001c) */
pub const RT5663_ADC_L_MUTE_MASK: u32 = (0x1u32 << 15u32);
pub const RT5663_ADC_L_MUTE_SHIFT: u32 = 15u32;
pub const RT5663_ADC_L_VOL_MASK: u32 = (0x7fu32 << 8u32);
pub const RT5663_ADC_L_VOL_SHIFT: u32 = 8u32;
pub const RT5663_ADC_R_MUTE_MASK: u32 = (0x1u32 << 7u32);
pub const RT5663_ADC_R_MUTE_SHIFT: u32 = 7u32;
pub const RT5663_ADC_R_VOL_MASK: u32 = (0x7fu32);
pub const RT5663_ADC_R_VOL_SHIFT: u32 = 0u32;

/* Stereo ADC Mixer Control (0x0026) */
pub const RT5663_M_STO1_ADC_L1: u32 = (0x1u32 << 15u32);
pub const RT5663_M_STO1_ADC_L1_SHIFT: u32 = 15u32;
pub const RT5663_M_STO1_ADC_L2: u32 = (0x1u32 << 14u32);
pub const RT5663_M_STO1_ADC_L2_SHIFT: u32 = 14u32;
pub const RT5663_STO1_ADC_L1_SRC: u32 = (0x1u32 << 13u32);
pub const RT5663_STO1_ADC_L1_SRC_SHIFT: u32 = 13u32;
pub const RT5663_STO1_ADC_L2_SRC: u32 = (0x1u32 << 12u32);
pub const RT5663_STO1_ADC_L2_SRC_SHIFT: u32 = 12u32;
pub const RT5663_STO1_ADC_L_SRC: u32 = (0x3u32 << 10u32);
pub const RT5663_STO1_ADC_L_SRC_SHIFT: u32 = 10u32;
pub const RT5663_M_STO1_ADC_R1: u32 = (0x1u32 << 7u32);
pub const RT5663_M_STO1_ADC_R1_SHIFT: u32 = 7u32;
pub const RT5663_M_STO1_ADC_R2: u32 = (0x1u32 << 6u32);
pub const RT5663_M_STO1_ADC_R2_SHIFT: u32 = 6u32;
pub const RT5663_STO1_ADC_R1_SRC: u32 = (0x1u32 << 5u32);
pub const RT5663_STO1_ADC_R1_SRC_SHIFT: u32 = 5u32;
pub const RT5663_STO1_ADC_R2_SRC: u32 = (0x1u32 << 4u32);
pub const RT5663_STO1_ADC_R2_SRC_SHIFT: u32 = 4u32;
pub const RT5663_STO1_ADC_R_SRC: u32 = (0x3u32 << 2u32);
pub const RT5663_STO1_ADC_R_SRC_SHIFT: u32 = 2u32;

/* ADC Mixer to DAC Mixer Control (0x0029) */
pub const RT5663_M_ADCMIX_L: u32 = (0x1u32 << 15u32);
pub const RT5663_M_ADCMIX_L_SHIFT: u32 = 15u32;
pub const RT5663_M_DAC1_L: u32 = (0x1u32 << 14u32);
pub const RT5663_M_DAC1_L_SHIFT: u32 = 14u32;
pub const RT5663_M_ADCMIX_R: u32 = (0x1u32 << 7u32);
pub const RT5663_M_ADCMIX_R_SHIFT: u32 = 7u32;
pub const RT5663_M_DAC1_R: u32 = (0x1u32 << 6u32);
pub const RT5663_M_DAC1_R_SHIFT: u32 = 6u32;

/* Stereo DAC Mixer Control (0x002a) */
pub const RT5663_M_DAC_L1_STO_L: u32 = (0x1u32 << 15u32);
pub const RT5663_M_DAC_L1_STO_L_SHIFT: u32 = 15u32;
pub const RT5663_M_DAC_R1_STO_L: u32 = (0x1u32 << 13u32);
pub const RT5663_M_DAC_R1_STO_L_SHIFT: u32 = 13u32;
pub const RT5663_M_DAC_L1_STO_R: u32 = (0x1u32 << 7u32);
pub const RT5663_M_DAC_L1_STO_R_SHIFT: u32 = 7u32;
pub const RT5663_M_DAC_R1_STO_R: u32 = (0x1u32 << 5u32);
pub const RT5663_M_DAC_R1_STO_R_SHIFT: u32 = 5u32;

/* Power Management for Digital 1 (0x0061) */
pub const RT5663_PWR_I2S1: u32 = (0x1u32 << 15u32);
pub const RT5663_PWR_I2S1_SHIFT: u32 = 15u32;
pub const RT5663_PWR_DAC_L1: u32 = (0x1u32 << 11u32);
pub const RT5663_PWR_DAC_L1_SHIFT: u32 = 11u32;
pub const RT5663_PWR_DAC_R1: u32 = (0x1u32 << 10u32);
pub const RT5663_PWR_DAC_R1_SHIFT: u32 = 10u32;
pub const RT5663_PWR_LDO_DACREF_MASK: u32 = (0x1u32 << 8u32);
pub const RT5663_PWR_LDO_DACREF_SHIFT: u32 = 8u32;
pub const RT5663_PWR_LDO_DACREF_ON: u32 = (0x1u32 << 8u32);
pub const RT5663_PWR_LDO_DACREF_DOWN: u32 = (0x0u32 << 8u32);
pub const RT5663_PWR_LDO_SHIFT: u32 = 8u32;
pub const RT5663_PWR_ADC_L1: u32 = (0x1u32 << 4u32);
pub const RT5663_PWR_ADC_L1_SHIFT: u32 = 4u32;
pub const RT5663_PWR_ADC_R1: u32 = (0x1u32 << 3u32);
pub const RT5663_PWR_ADC_R1_SHIFT: u32 = 3u32;

/* Power Management for Digital 2 (0x0062) */
pub const RT5663_PWR_ADC_S1F: u32 = (0x1u32 << 15u32);
pub const RT5663_PWR_ADC_S1F_SHIFT: u32 = 15u32;
pub const RT5663_PWR_DAC_S1F: u32 = (0x1u32 << 10u32);
pub const RT5663_PWR_DAC_S1F_SHIFT: u32 = 10u32;

/* Power Management for Analog 1 (0x0063) */
pub const RT5663_PWR_VREF1: u32 = (0x1u32 << 15u32);
pub const RT5663_PWR_VREF1_MASK: u32 = (0x1u32 << 15u32);
pub const RT5663_PWR_VREF1_SHIFT: u32 = 15u32;
pub const RT5663_PWR_FV1: u32 = (0x1u32 << 14u32);
pub const RT5663_PWR_FV1_MASK: u32 = (0x1u32 << 14u32);
pub const RT5663_PWR_FV1_SHIFT: u32 = 14u32;
pub const RT5663_PWR_VREF2: u32 = (0x1u32 << 13u32);
pub const RT5663_PWR_VREF2_MASK: u32 = (0x1u32 << 13u32);
pub const RT5663_PWR_VREF2_SHIFT: u32 = 13u32;
pub const RT5663_PWR_FV2: u32 = (0x1u32 << 12u32);
pub const RT5663_PWR_FV2_MASK: u32 = (0x1u32 << 12u32);
pub const RT5663_PWR_FV2_SHIFT: u32 = 12u32;
pub const RT5663_PWR_MB: u32 = (0x1u32 << 9u32);
pub const RT5663_PWR_MB_MASK: u32 = (0x1u32 << 9u32);
pub const RT5663_PWR_MB_SHIFT: u32 = 9u32;
pub const RT5663_AMP_HP_MASK: u32 = (0x3u32 << 2u32);
pub const RT5663_AMP_HP_SHIFT: u32 = 2u32;
pub const RT5663_AMP_HP_1X: u32 = (0x0u32 << 2u32);
pub const RT5663_AMP_HP_3X: u32 = (0x1u32 << 2u32);
pub const RT5663_AMP_HP_5X: u32 = (0x3u32 << 2u32);
pub const RT5663_LDO1_DVO_MASK: u32 = (0x3u32);
pub const RT5663_LDO1_DVO_SHIFT: u32 = 0u32;
pub const RT5663_LDO1_DVO_0_9V: u32 = (0x0u32);
pub const RT5663_LDO1_DVO_1_0V: u32 = (0x1u32);
pub const RT5663_LDO1_DVO_1_2V: u32 = (0x2u32);
pub const RT5663_LDO1_DVO_1_4V: u32 = (0x3u32);

/* Power Management for Analog 2 (0x0064) */
pub const RT5663_PWR_BST1: u32 = (0x1u32 << 15u32);
pub const RT5663_PWR_BST1_MASK: u32 = (0x1u32 << 15u32);
pub const RT5663_PWR_BST1_SHIFT: u32 = 15u32;
pub const RT5663_PWR_BST1_OFF: u32 = (0x0u32 << 15u32);
pub const RT5663_PWR_BST1_ON: u32 = (0x1u32 << 15u32);
pub const RT5663_PWR_BST2: u32 = (0x1u32 << 14u32);
pub const RT5663_PWR_BST2_MASK: u32 = (0x1u32 << 14u32);
pub const RT5663_PWR_BST2_SHIFT: u32 = 14u32;
pub const RT5663_PWR_MB1: u32 = (0x1u32 << 11u32);
pub const RT5663_PWR_MB1_SHIFT: u32 = 11u32;
pub const RT5663_PWR_MB2: u32 = (0x1u32 << 10u32);
pub const RT5663_PWR_MB2_SHIFT: u32 = 10u32;
pub const RT5663_PWR_BST2_OP: u32 = (0x1u32 << 6u32);
pub const RT5663_PWR_BST2_OP_MASK: u32 = (0x1u32 << 6u32);
pub const RT5663_PWR_BST2_OP_SHIFT: u32 = 6u32;
pub const RT5663_PWR_JD1: u32 = (0x1u32 << 3u32);
pub const RT5663_PWR_JD1_MASK: u32 = (0x1u32 << 3u32);
pub const RT5663_PWR_JD1_SHIFT: u32 = 3u32;
pub const RT5663_PWR_JD2: u32 = (0x1u32 << 2u32);
pub const RT5663_PWR_JD2_MASK: u32 = (0x1u32 << 2u32);
pub const RT5663_PWR_JD2_SHIFT: u32 = 2u32;
pub const RT5663_PWR_RECMIX1: u32 = (0x1u32 << 1u32);
pub const RT5663_PWR_RECMIX1_SHIFT: u32 = 1u32;
pub const RT5663_PWR_RECMIX2: u32 = (0x1u32);
pub const RT5663_PWR_RECMIX2_SHIFT: u32 = 0u32;

/* Power Management for Analog 3 (0x0065) */
pub const RT5663_PWR_CBJ_MASK: u32 = (0x1u32 << 9u32);
pub const RT5663_PWR_CBJ_SHIFT: u32 = 9u32;
pub const RT5663_PWR_CBJ_OFF: u32 = (0x0u32 << 9u32);
pub const RT5663_PWR_CBJ_ON: u32 = (0x1u32 << 9u32);
pub const RT5663_PWR_PLL: u32 = (0x1u32 << 6u32);
pub const RT5663_PWR_PLL_SHIFT: u32 = 6u32;
pub const RT5663_PWR_LDO2: u32 = (0x1u32 << 2u32);
pub const RT5663_PWR_LDO2_SHIFT: u32 = 2u32;

/* Power Management for Volume (0x0067) */
pub const RT5663_V2_PWR_MIC_DET: u32 = (0x1u32 << 5u32);
pub const RT5663_V2_PWR_MIC_DET_SHIFT: u32 = 5u32;

/* MCLK and System Clock Detection Control (0x006b) */
pub const RT5663_EN_ANA_CLK_DET_MASK: u32 = (0x1u32 << 15u32);
pub const RT5663_EN_ANA_CLK_DET_SHIFT: u32 = 15u32;
pub const RT5663_EN_ANA_CLK_DET_DIS: u32 = (0x0u32 << 15u32);
pub const RT5663_EN_ANA_CLK_DET_AUTO: u32 = (0x1u32 << 15u32);
pub const RT5663_PWR_CLK_DET_MASK: u32 = (0x1u32);
pub const RT5663_PWR_CLK_DET_SHIFT: u32 = 0u32;
pub const RT5663_PWR_CLK_DET_DIS: u32 = (0x0u32);
pub const RT5663_PWR_CLK_DET_EN: u32 = (0x1u32);

/* I2S1 Audio Serial Data Port Control (0x0070) */
pub const RT5663_I2S_MS_MASK: u32 = (0x1u32 << 15u32);
pub const RT5663_I2S_MS_SHIFT: u32 = 15u32;
pub const RT5663_I2S_MS_M: u32 = (0x0u32 << 15u32);
pub const RT5663_I2S_MS_S: u32 = (0x1u32 << 15u32);
pub const RT5663_I2S_BP_MASK: u32 = (0x1u32 << 8u32);
pub const RT5663_I2S_BP_SHIFT: u32 = 8u32;
pub const RT5663_I2S_BP_NOR: u32 = (0x0u32 << 8u32);
pub const RT5663_I2S_BP_INV: u32 = (0x1u32 << 8u32);
pub const RT5663_I2S_DL_MASK: u32 = (0x3u32 << 4u32);
pub const RT5663_I2S_DL_SHIFT: u32 = 4u32;
pub const RT5663_I2S_DL_16: u32 = (0x0u32 << 4u32);
pub const RT5663_I2S_DL_20: u32 = (0x1u32 << 4u32);
pub const RT5663_I2S_DL_24: u32 = (0x2u32 << 4u32);
pub const RT5663_I2S_DL_8: u32 = (0x3u32 << 4u32);
pub const RT5663_I2S_DF_MASK: u32 = (0x7u32);
pub const RT5663_I2S_DF_SHIFT: u32 = 0u32;
pub const RT5663_I2S_DF_I2S: u32 = (0x0u32);
pub const RT5663_I2S_DF_LEFT: u32 = (0x1u32);
pub const RT5663_I2S_DF_PCM_A: u32 = (0x2u32);
pub const RT5663_I2S_DF_PCM_B: u32 = (0x3u32);
pub const RT5663_I2S_DF_PCM_A_N: u32 = (0x6u32);
pub const RT5663_I2S_DF_PCM_B_N: u32 = (0x7u32);

/* ADC/DAC Clock Control 1 (0x0073) */
pub const RT5663_I2S_PD1_MASK: u32 = (0x7u32 << 12u32);
pub const RT5663_I2S_PD1_SHIFT: u32 = 12u32;
pub const RT5663_M_I2S_DIV_MASK: u32 = (0x7u32 << 8u32);
pub const RT5663_M_I2S_DIV_SHIFT: u32 = 8u32;
pub const RT5663_CLK_SRC_MASK: u32 = (0x3u32 << 4u32);
pub const RT5663_CLK_SRC_MCLK: u32 = (0x0u32 << 4u32);
pub const RT5663_CLK_SRC_PLL_OUT: u32 = (0x1u32 << 4u32);
pub const RT5663_CLK_SRC_DIV: u32 = (0x2u32 << 4u32);
pub const RT5663_CLK_SRC_RC: u32 = (0x3u32 << 4u32);
pub const RT5663_DAC_OSR_MASK: u32 = (0x3u32 << 2u32);
pub const RT5663_DAC_OSR_SHIFT: u32 = 2u32;
pub const RT5663_DAC_OSR_128: u32 = (0x0u32 << 2u32);
pub const RT5663_DAC_OSR_64: u32 = (0x1u32 << 2u32);
pub const RT5663_DAC_OSR_32: u32 = (0x2u32 << 2u32);
pub const RT5663_ADC_OSR_MASK: u32 = (0x3u32);
pub const RT5663_ADC_OSR_SHIFT: u32 = 0u32;
pub const RT5663_ADC_OSR_128: u32 = (0x0u32);
pub const RT5663_ADC_OSR_64: u32 = (0x1u32);
pub const RT5663_ADC_OSR_32: u32 = (0x2u32);

/* TDM1 control 1 (0x0078) */
pub const RT5663_TDM_MODE_MASK: u32 = (0x1u32 << 15u32);
pub const RT5663_TDM_MODE_SHIFT: u32 = 15u32;
pub const RT5663_TDM_MODE_I2S: u32 = (0x0u32 << 15u32);
pub const RT5663_TDM_MODE_TDM: u32 = (0x1u32 << 15u32);
pub const RT5663_TDM_IN_CH_MASK: u32 = (0x3u32 << 10u32);
pub const RT5663_TDM_IN_CH_SHIFT: u32 = 10u32;
pub const RT5663_TDM_IN_CH_2: u32 = (0x0u32 << 10u32);
pub const RT5663_TDM_IN_CH_4: u32 = (0x1u32 << 10u32);
pub const RT5663_TDM_IN_CH_6: u32 = (0x2u32 << 10u32);
pub const RT5663_TDM_IN_CH_8: u32 = (0x3u32 << 10u32);
pub const RT5663_TDM_OUT_CH_MASK: u32 = (0x3u32 << 8u32);
pub const RT5663_TDM_OUT_CH_SHIFT: u32 = 8u32;
pub const RT5663_TDM_OUT_CH_2: u32 = (0x0u32 << 8u32);
pub const RT5663_TDM_OUT_CH_4: u32 = (0x1u32 << 8u32);
pub const RT5663_TDM_OUT_CH_6: u32 = (0x2u32 << 8u32);
pub const RT5663_TDM_OUT_CH_8: u32 = (0x3u32 << 8u32);
pub const RT5663_TDM_IN_LEN_MASK: u32 = (0x3u32 << 6u32);
pub const RT5663_TDM_IN_LEN_SHIFT: u32 = 6u32;
pub const RT5663_TDM_IN_LEN_16: u32 = (0x0u32 << 6u32);
pub const RT5663_TDM_IN_LEN_20: u32 = (0x1u32 << 6u32);
pub const RT5663_TDM_IN_LEN_24: u32 = (0x2u32 << 6u32);
pub const RT5663_TDM_IN_LEN_32: u32 = (0x3u32 << 6u32);
pub const RT5663_TDM_OUT_LEN_MASK: u32 = (0x3u32 << 4u32);
pub const RT5663_TDM_OUT_LEN_SHIFT: u32 = 4u32;
pub const RT5663_TDM_OUT_LEN_16: u32 = (0x0u32 << 4u32);
pub const RT5663_TDM_OUT_LEN_20: u32 = (0x1u32 << 4u32);
pub const RT5663_TDM_OUT_LEN_24: u32 = (0x2u32 << 4u32);
pub const RT5663_TDM_OUT_LEN_32: u32 = (0x3u32 << 4u32);

/* Global Clock Control (0x0080) */
pub const RT5663_SCLK_SRC_MASK: u32 = (0x3u32 << 14u32);
pub const RT5663_SCLK_SRC_SHIFT: u32 = 14u32;
pub const RT5663_SCLK_SRC_MCLK: u32 = (0x0u32 << 14u32);
pub const RT5663_SCLK_SRC_PLL1: u32 = (0x1u32 << 14u32);
pub const RT5663_SCLK_SRC_RCCLK: u32 = (0x2u32 << 14u32);
pub const RT5663_PLL1_SRC_MASK: u32 = (0x7u32 << 11u32);
pub const RT5663_PLL1_SRC_SHIFT: u32 = 11u32;
pub const RT5663_PLL1_SRC_MCLK: u32 = (0x0u32 << 11u32);
pub const RT5663_PLL1_SRC_BCLK1: u32 = (0x1u32 << 11u32);
pub const RT5663_V2_PLL1_SRC_MASK: u32 = (0x7u32 << 8u32);
pub const RT5663_V2_PLL1_SRC_SHIFT: u32 = 8u32;
pub const RT5663_V2_PLL1_SRC_MCLK: u32 = (0x0u32 << 8u32);
pub const RT5663_V2_PLL1_SRC_BCLK1: u32 = (0x1u32 << 8u32);
pub const RT5663_PLL1_PD_MASK: u32 = (0x1u32 << 4u32);
pub const RT5663_PLL1_PD_SHIFT: u32 = 4u32;

pub const RT5663_PLL_INP_MAX: u32 = 40000000u32;
pub const RT5663_PLL_INP_MIN: u32 = 256000u32;
/* PLL M/N/K Code Control 1 (0x0081) */
pub const RT5663_PLL_N_MAX: u32 = 0x001ffu32;
pub const RT5663_PLL_N_MASK: u32 = (RT5663_PLL_N_MAX << 7u32);
pub const RT5663_PLL_N_SHIFT: u32 = 7u32;
pub const RT5663_PLL_K_MAX: u32 = 0x001fu32;
pub const RT5663_PLL_K_MASK: u32 = (RT5663_PLL_K_MAX);
pub const RT5663_PLL_K_SHIFT: u32 = 0u32;

/* PLL M/N/K Code Control 2 (0x0082) */
pub const RT5663_PLL_M_MAX: u32 = 0x00fu32;
pub const RT5663_PLL_M_MASK: u32 = (RT5663_PLL_M_MAX << 12u32);
pub const RT5663_PLL_M_SHIFT: u32 = 12u32;
pub const RT5663_PLL_M_BP: u32 = (0x1u32 << 11u32);
pub const RT5663_PLL_M_BP_SHIFT: u32 = 11u32;

/* PLL tracking mode 1 (0x0083) */
pub const RT5663_V2_I2S1_ASRC_MASK: u32 = (0x1u32 << 13u32);
pub const RT5663_V2_I2S1_ASRC_SHIFT: u32 = 13u32;
pub const RT5663_V2_DAC_STO1_ASRC_MASK: u32 = (0x1u32 << 12u32);
pub const RT5663_V2_DAC_STO1_ASRC_SHIFT: u32 = 12u32;
pub const RT5663_V2_ADC_STO1_ASRC_MASK: u32 = (0x1u32 << 4u32);
pub const RT5663_V2_ADC_STO1_ASRC_SHIFT: u32 = 4u32;

/* PLL tracking mode 2 (0x0084)*/

/* PLL tracking mode 3 (0x0085)*/
pub const RT5663_V2_AD_STO1_TRACK_MASK: u32 = (0x7u32 << 12u32);
pub const RT5663_V2_AD_STO1_TRACK_SHIFT: u32 = 12u32;
pub const RT5663_V2_AD_STO1_TRACK_SYSCLK: u32 = (0x0u32 << 12u32);
pub const RT5663_V2_AD_STO1_TRACK_I2S1: u32 = (0x1u32 << 12u32);

/* HPOUT Charge pump control 1 (0x0091) */
pub const RT5663_OSW_HP_L_MASK: u32 = (0x1u32 << 11u32);
pub const RT5663_OSW_HP_L_SHIFT: u32 = 11u32;
pub const RT5663_OSW_HP_L_EN: u32 = (0x1u32 << 11u32);
pub const RT5663_OSW_HP_L_DIS: u32 = (0x0u32 << 11u32);
pub const RT5663_OSW_HP_R_MASK: u32 = (0x1u32 << 10u32);
pub const RT5663_OSW_HP_R_SHIFT: u32 = 10u32;
pub const RT5663_OSW_HP_R_EN: u32 = (0x1u32 << 10u32);
pub const RT5663_OSW_HP_R_DIS: u32 = (0x0u32 << 10u32);
pub const RT5663_SEL_PM_HP_MASK: u32 = (0x3u32 << 8u32);
pub const RT5663_SEL_PM_HP_SHIFT: u32 = 8u32;
pub const RT5663_SEL_PM_HP_0_6: u32 = (0x0u32 << 8u32);
pub const RT5663_SEL_PM_HP_0_9: u32 = (0x1u32 << 8u32);
pub const RT5663_SEL_PM_HP_1_8: u32 = (0x2u32 << 8u32);
pub const RT5663_SEL_PM_HP_HIGH: u32 = (0x3u32 << 8u32);
pub const RT5663_OVCD_HP_MASK: u32 = (0x1u32 << 2u32);
pub const RT5663_OVCD_HP_SHIFT: u32 = 2u32;
pub const RT5663_OVCD_HP_EN: u32 = (0x1u32 << 2u32);
pub const RT5663_OVCD_HP_DIS: u32 = (0x0u32 << 2u32);

/* RC Clock Control (0x0094) */
pub const RT5663_DIG_25M_CLK_MASK: u32 = (0x1u32 << 9u32);
pub const RT5663_DIG_25M_CLK_SHIFT: u32 = 9u32;
pub const RT5663_DIG_25M_CLK_DIS: u32 = (0x0u32 << 9u32);
pub const RT5663_DIG_25M_CLK_EN: u32 = (0x1u32 << 9u32);
pub const RT5663_DIG_1M_CLK_MASK: u32 = (0x1u32 << 8u32);
pub const RT5663_DIG_1M_CLK_SHIFT: u32 = 8u32;
pub const RT5663_DIG_1M_CLK_DIS: u32 = (0x0u32 << 8u32);
pub const RT5663_DIG_1M_CLK_EN: u32 = (0x1u32 << 8u32);

/* Auto Turn On 1M RC CLK (0x009f) */
pub const RT5663_IRQ_POW_SAV_MASK: u32 = (0x1u32 << 15u32);
pub const RT5663_IRQ_POW_SAV_SHIFT: u32 = 15u32;
pub const RT5663_IRQ_POW_SAV_DIS: u32 = (0x0u32 << 15u32);
pub const RT5663_IRQ_POW_SAV_EN: u32 = (0x1u32 << 15u32);
pub const RT5663_IRQ_POW_SAV_JD1_MASK: u32 = (0x1u32 << 14u32);
pub const RT5663_IRQ_POW_SAV_JD1_SHIFT: u32 = 14u32;
pub const RT5663_IRQ_POW_SAV_JD1_DIS: u32 = (0x0u32 << 14u32);
pub const RT5663_IRQ_POW_SAV_JD1_EN: u32 = (0x1u32 << 14u32);
pub const RT5663_IRQ_MANUAL_MASK: u32 = (0x1u32 << 8u32);
pub const RT5663_IRQ_MANUAL_SHIFT: u32 = 8u32;
pub const RT5663_IRQ_MANUAL_DIS: u32 = (0x0u32 << 8u32);
pub const RT5663_IRQ_MANUAL_EN: u32 = (0x1u32 << 8u32);

/* IRQ Control 1 (0x00b6) */
pub const RT5663_EN_CB_JD_MASK: u32 = (0x1u32 << 3u32);
pub const RT5663_EN_CB_JD_SHIFT: u32 = 3u32;
pub const RT5663_EN_CB_JD_EN: u32 = (0x1u32 << 3u32);
pub const RT5663_EN_CB_JD_DIS: u32 = (0x0u32 << 3u32);

/* IRQ Control 3 (0x00b8) */
pub const RT5663_V2_EN_IRQ_INLINE_MASK: u32 = (0x1u32 << 6u32);
pub const RT5663_V2_EN_IRQ_INLINE_SHIFT: u32 = 6u32;
pub const RT5663_V2_EN_IRQ_INLINE_BYP: u32 = (0x0u32 << 6u32);
pub const RT5663_V2_EN_IRQ_INLINE_NOR: u32 = (0x1u32 << 6u32);

/* GPIO Control 1 (0x00c0) */
pub const RT5663_GP1_PIN_MASK: u32 = (0x1u32 << 15u32);
pub const RT5663_GP1_PIN_SHIFT: u32 = 15u32;
pub const RT5663_GP1_PIN_GPIO1: u32 = (0x0u32 << 15u32);
pub const RT5663_GP1_PIN_IRQ: u32 = (0x1u32 << 15u32);

/* GPIO Control 2 (0x00c1) */
pub const RT5663_GP4_PIN_CONF_MASK: u32 = (0x1u32 << 5u32);
pub const RT5663_GP4_PIN_CONF_SHIFT: u32 = 5u32;
pub const RT5663_GP4_PIN_CONF_INPUT: u32 = (0x0u32 << 5u32);
pub const RT5663_GP4_PIN_CONF_OUTPUT: u32 = (0x1u32 << 5u32);

/* GPIO Control 2 (0x00c2) */
pub const RT5663_GP8_PIN_CONF_MASK: u32 = (0x1u32 << 13u32);
pub const RT5663_GP8_PIN_CONF_SHIFT: u32 = 13u32;
pub const RT5663_GP8_PIN_CONF_INPUT: u32 = (0x0u32 << 13u32);
pub const RT5663_GP8_PIN_CONF_OUTPUT: u32 = (0x1u32 << 13u32);

/* 4 Buttons Inline Command Function 1 (0x00df) */
pub const RT5663_4BTN_CLK_DEB_MASK: u32 = (0x3u32 << 2u32);
pub const RT5663_4BTN_CLK_DEB_SHIFT: u32 = 2u32;
pub const RT5663_4BTN_CLK_DEB_8MS: u32 = (0x0u32 << 2u32);
pub const RT5663_4BTN_CLK_DEB_16MS: u32 = (0x1u32 << 2u32);
pub const RT5663_4BTN_CLK_DEB_32MS: u32 = (0x2u32 << 2u32);
pub const RT5663_4BTN_CLK_DEB_65MS: u32 = (0x3u32 << 2u32);

/* Inline Command Function 6 (0x00e0) */
pub const RT5663_EN_4BTN_INL_MASK: u32 = (0x1u32 << 15u32);
pub const RT5663_EN_4BTN_INL_SHIFT: u32 = 15u32;
pub const RT5663_EN_4BTN_INL_DIS: u32 = (0x0u32 << 15u32);
pub const RT5663_EN_4BTN_INL_EN: u32 = (0x1u32 << 15u32);
pub const RT5663_RESET_4BTN_INL_MASK: u32 = (0x1u32 << 14u32);
pub const RT5663_RESET_4BTN_INL_SHIFT: u32 = 14u32;
pub const RT5663_RESET_4BTN_INL_RESET: u32 = (0x0u32 << 14u32);
pub const RT5663_RESET_4BTN_INL_NOR: u32 = (0x1u32 << 14u32);

/* Digital Misc Control (0x00fa) */
pub const RT5663_DIG_GATE_CTRL_MASK: u32 = 0x1u32;
pub const RT5663_DIG_GATE_CTRL_SHIFT: u32 = (0u32);
pub const RT5663_DIG_GATE_CTRL_DIS: u32 = 0x0u32;
pub const RT5663_DIG_GATE_CTRL_EN: u32 = 0x1u32;

/* Chopper and Clock control for DAC L (0x013a)*/
pub const RT5663_CKXEN_DAC1_MASK: u32 = (0x1u32 << 13u32);
pub const RT5663_CKXEN_DAC1_SHIFT: u32 = 13u32;
pub const RT5663_CKGEN_DAC1_MASK: u32 = (0x1u32 << 12u32);
pub const RT5663_CKGEN_DAC1_SHIFT: u32 = 12u32;

/* Chopper and Clock control for ADC (0x013b)*/
pub const RT5663_CKXEN_ADCC_MASK: u32 = (0x1u32 << 13u32);
pub const RT5663_CKXEN_ADCC_SHIFT: u32 = 13u32;
pub const RT5663_CKGEN_ADCC_MASK: u32 = (0x1u32 << 12u32);
pub const RT5663_CKGEN_ADCC_SHIFT: u32 = 12u32;

/* HP Behavior Logic Control 2 (0x01db) */
pub const RT5663_HP_SIG_SRC1_MASK: u32 = (0x3u32);
pub const RT5663_HP_SIG_SRC1_SHIFT: u32 = 0u32;
pub const RT5663_HP_SIG_SRC1_HP_DC: u32 = (0x0u32);
pub const RT5663_HP_SIG_SRC1_HP_CALIB: u32 = (0x1u32);
pub const RT5663_HP_SIG_SRC1_REG: u32 = (0x2u32);
pub const RT5663_HP_SIG_SRC1_SILENCE: u32 = (0x3u32);

/* RT5663 specific register */
pub const RT5663_HP_OUT_EN: u32 = 0x0002u32;
pub const RT5663_HP_LCH_DRE: u32 = 0x0005u32;
pub const RT5663_HP_RCH_DRE: u32 = 0x0006u32;
pub const RT5663_CALIB_BST: u32 = 0x000au32;
pub const RT5663_RECMIX: u32 = 0x0010u32;
pub const RT5663_SIL_DET_CTL: u32 = 0x0015u32;
pub const RT5663_PWR_SAV_SILDET: u32 = 0x0016u32;
pub const RT5663_SIDETONE_CTL: u32 = 0x0018u32;
pub const RT5663_STO1_DAC_DIG_VOL: u32 = 0x0019u32;
pub const RT5663_STO1_ADC_DIG_VOL: u32 = 0x001cu32;
pub const RT5663_STO1_BOOST: u32 = 0x001fu32;
pub const RT5663_HP_IMP_GAIN_1: u32 = 0x0022u32;
pub const RT5663_HP_IMP_GAIN_2: u32 = 0x0023u32;
pub const RT5663_STO1_ADC_MIXER: u32 = 0x0026u32;
pub const RT5663_AD_DA_MIXER: u32 = 0x0029u32;
pub const RT5663_STO_DAC_MIXER: u32 = 0x002au32;
pub const RT5663_DIG_SIDE_MIXER: u32 = 0x002cu32;
pub const RT5663_BYPASS_STO_DAC: u32 = 0x002du32;
pub const RT5663_CALIB_REC_MIX: u32 = 0x0040u32;
pub const RT5663_PWR_DIG_1: u32 = 0x0061u32;
pub const RT5663_PWR_DIG_2: u32 = 0x0062u32;
pub const RT5663_PWR_ANLG_1: u32 = 0x0063u32;
pub const RT5663_PWR_ANLG_2: u32 = 0x0064u32;
pub const RT5663_PWR_ANLG_3: u32 = 0x0065u32;
pub const RT5663_PWR_MIXER: u32 = 0x0066u32;
pub const RT5663_SIG_CLK_DET: u32 = 0x006bu32;
pub const RT5663_PRE_DIV_GATING_1: u32 = 0x006eu32;
pub const RT5663_PRE_DIV_GATING_2: u32 = 0x006fu32;
pub const RT5663_I2S1_SDP: u32 = 0x0070u32;
pub const RT5663_ADDA_CLK_1: u32 = 0x0073u32;
pub const RT5663_ADDA_RST: u32 = 0x0074u32;
pub const RT5663_FRAC_DIV_1: u32 = 0x0075u32;
pub const RT5663_FRAC_DIV_2: u32 = 0x0076u32;
pub const RT5663_TDM_1: u32 = 0x0077u32;
pub const RT5663_TDM_2: u32 = 0x0078u32;
pub const RT5663_TDM_3: u32 = 0x0079u32;
pub const RT5663_TDM_4: u32 = 0x007au32;
pub const RT5663_TDM_5: u32 = 0x007bu32;
pub const RT5663_TDM_6: u32 = 0x007cu32;
pub const RT5663_TDM_7: u32 = 0x007du32;
pub const RT5663_TDM_8: u32 = 0x007eu32;
pub const RT5663_TDM_9: u32 = 0x007fu32;
pub const RT5663_GLB_CLK: u32 = 0x0080u32;
pub const RT5663_PLL_1: u32 = 0x0081u32;
pub const RT5663_PLL_2: u32 = 0x0082u32;
pub const RT5663_ASRC_1: u32 = 0x0083u32;
pub const RT5663_ASRC_2: u32 = 0x0084u32;
pub const RT5663_ASRC_4: u32 = 0x0086u32;
pub const RT5663_DUMMY_REG: u32 = 0x0087u32;
pub const RT5663_ASRC_8: u32 = 0x008au32;
pub const RT5663_ASRC_9: u32 = 0x008bu32;
pub const RT5663_ASRC_11: u32 = 0x008cu32;
pub const RT5663_DEPOP_1: u32 = 0x008eu32;
pub const RT5663_DEPOP_2: u32 = 0x008fu32;
pub const RT5663_DEPOP_3: u32 = 0x0090u32;
pub const RT5663_HP_CHARGE_PUMP_1: u32 = 0x0091u32;
pub const RT5663_HP_CHARGE_PUMP_2: u32 = 0x0092u32;
pub const RT5663_MICBIAS_1: u32 = 0x0093u32;
pub const RT5663_RC_CLK: u32 = 0x0094u32;
pub const RT5663_ASRC_11_2: u32 = 0x0097u32;
pub const RT5663_DUMMY_REG_2: u32 = 0x0098u32;
pub const RT5663_REC_PATH_GAIN: u32 = 0x009au32;
pub const RT5663_AUTO_1MRC_CLK: u32 = 0x009fu32;
pub const RT5663_ADC_EQ_1: u32 = 0x00aeu32;
pub const RT5663_ADC_EQ_2: u32 = 0x00afu32;
pub const RT5663_IRQ_1: u32 = 0x00b6u32;
pub const RT5663_IRQ_2: u32 = 0x00b7u32;
pub const RT5663_IRQ_3: u32 = 0x00b8u32;
pub const RT5663_IRQ_4: u32 = 0x00bau32;
pub const RT5663_IRQ_5: u32 = 0x00bbu32;
pub const RT5663_INT_ST_1: u32 = 0x00beu32;
pub const RT5663_INT_ST_2: u32 = 0x00bfu32;
pub const RT5663_GPIO_1: u32 = 0x00c0u32;
pub const RT5663_GPIO_2: u32 = 0x00c1u32;
pub const RT5663_GPIO_STA1: u32 = 0x00c5u32;
pub const RT5663_SIN_GEN_1: u32 = 0x00cbu32;
pub const RT5663_SIN_GEN_2: u32 = 0x00ccu32;
pub const RT5663_SIN_GEN_3: u32 = 0x00cdu32;
pub const RT5663_SOF_VOL_ZC1: u32 = 0x00d9u32;
pub const RT5663_IL_CMD_1: u32 = 0x00dbu32;
pub const RT5663_IL_CMD_2: u32 = 0x00dcu32;
pub const RT5663_IL_CMD_3: u32 = 0x00ddu32;
pub const RT5663_IL_CMD_4: u32 = 0x00deu32;
pub const RT5663_IL_CMD_5: u32 = 0x00dfu32;
pub const RT5663_IL_CMD_6: u32 = 0x00e0u32;
pub const RT5663_IL_CMD_7: u32 = 0x00e1u32;
pub const RT5663_IL_CMD_8: u32 = 0x00e2u32;
pub const RT5663_IL_CMD_PWRSAV1: u32 = 0x00e4u32;
pub const RT5663_IL_CMD_PWRSAV2: u32 = 0x00e5u32;
pub const RT5663_EM_JACK_TYPE_1: u32 = 0x00e6u32;
pub const RT5663_EM_JACK_TYPE_2: u32 = 0x00e7u32;
pub const RT5663_EM_JACK_TYPE_3: u32 = 0x00e8u32;
pub const RT5663_EM_JACK_TYPE_4: u32 = 0x00e9u32;
pub const RT5663_EM_JACK_TYPE_5: u32 = 0x00eau32;
pub const RT5663_EM_JACK_TYPE_6: u32 = 0x00ebu32;
pub const RT5663_STO1_HPF_ADJ1: u32 = 0x00ecu32;
pub const RT5663_STO1_HPF_ADJ2: u32 = 0x00edu32;
pub const RT5663_FAST_OFF_MICBIAS: u32 = 0x00f4u32;
pub const RT5663_JD_CTRL1: u32 = 0x00f6u32;
pub const RT5663_JD_CTRL2: u32 = 0x00f8u32;
pub const RT5663_DIG_MISC: u32 = 0x00fau32;
pub const RT5663_DIG_VOL_ZCD: u32 = 0x0100u32;
pub const RT5663_ANA_BIAS_CUR_1: u32 = 0x0108u32;
pub const RT5663_ANA_BIAS_CUR_2: u32 = 0x0109u32;
pub const RT5663_ANA_BIAS_CUR_3: u32 = 0x010au32;
pub const RT5663_ANA_BIAS_CUR_4: u32 = 0x010bu32;
pub const RT5663_ANA_BIAS_CUR_5: u32 = 0x010cu32;
pub const RT5663_ANA_BIAS_CUR_6: u32 = 0x010du32;
pub const RT5663_BIAS_CUR_5: u32 = 0x010eu32;
pub const RT5663_BIAS_CUR_6: u32 = 0x010fu32;
pub const RT5663_BIAS_CUR_7: u32 = 0x0110u32;
pub const RT5663_BIAS_CUR_8: u32 = 0x0111u32;
pub const RT5663_DACREF_LDO: u32 = 0x0112u32;
pub const RT5663_DUMMY_REG_3: u32 = 0x0113u32;
pub const RT5663_BIAS_CUR_9: u32 = 0x0114u32;
pub const RT5663_DUMMY_REG_4: u32 = 0x0116u32;
pub const RT5663_VREFADJ_OP: u32 = 0x0117u32;
pub const RT5663_VREF_RECMIX: u32 = 0x0118u32;
pub const RT5663_CHARGE_PUMP_1: u32 = 0x0125u32;
pub const RT5663_CHARGE_PUMP_1_2: u32 = 0x0126u32;
pub const RT5663_CHARGE_PUMP_1_3: u32 = 0x0127u32;
pub const RT5663_CHARGE_PUMP_2: u32 = 0x0128u32;
pub const RT5663_DIG_IN_PIN1: u32 = 0x0132u32;
pub const RT5663_PAD_DRV_CTL: u32 = 0x0137u32;
pub const RT5663_PLL_INT_REG: u32 = 0x0139u32;
pub const RT5663_CHOP_DAC_L: u32 = 0x013au32;
pub const RT5663_CHOP_ADC: u32 = 0x013bu32;
pub const RT5663_CALIB_ADC: u32 = 0x013cu32;
pub const RT5663_CHOP_DAC_R: u32 = 0x013du32;
pub const RT5663_DUMMY_CTL_DACLR: u32 = 0x013eu32;
pub const RT5663_DUMMY_REG_5: u32 = 0x0140u32;
pub const RT5663_SOFT_RAMP: u32 = 0x0141u32;
pub const RT5663_TEST_MODE_1: u32 = 0x0144u32;
pub const RT5663_TEST_MODE_2: u32 = 0x0145u32;
pub const RT5663_TEST_MODE_3: u32 = 0x0146u32;
pub const RT5663_TEST_MODE_4: u32 = 0x0147u32;
pub const RT5663_TEST_MODE_5: u32 = 0x0148u32;
pub const RT5663_STO_DRE_1: u32 = 0x0160u32;
pub const RT5663_STO_DRE_2: u32 = 0x0161u32;
pub const RT5663_STO_DRE_3: u32 = 0x0162u32;
pub const RT5663_STO_DRE_4: u32 = 0x0163u32;
pub const RT5663_STO_DRE_5: u32 = 0x0164u32;
pub const RT5663_STO_DRE_6: u32 = 0x0165u32;
pub const RT5663_STO_DRE_7: u32 = 0x0166u32;
pub const RT5663_STO_DRE_8: u32 = 0x0167u32;
pub const RT5663_STO_DRE_9: u32 = 0x0168u32;
pub const RT5663_STO_DRE_10: u32 = 0x0169u32;
pub const RT5663_MIC_DECRO_1: u32 = 0x0180u32;
pub const RT5663_MIC_DECRO_2: u32 = 0x0181u32;
pub const RT5663_MIC_DECRO_3: u32 = 0x0182u32;
pub const RT5663_MIC_DECRO_4: u32 = 0x0183u32;
pub const RT5663_MIC_DECRO_5: u32 = 0x0184u32;
pub const RT5663_MIC_DECRO_6: u32 = 0x0185u32;
pub const RT5663_HP_DECRO_1: u32 = 0x01b0u32;
pub const RT5663_HP_DECRO_2: u32 = 0x01b1u32;
pub const RT5663_HP_DECRO_3: u32 = 0x01b2u32;
pub const RT5663_HP_DECRO_4: u32 = 0x01b3u32;
pub const RT5663_HP_DECOUP: u32 = 0x01b4u32;
pub const RT5663_HP_IMP_SEN_MAP8: u32 = 0x01b5u32;
pub const RT5663_HP_IMP_SEN_MAP9: u32 = 0x01b6u32;
pub const RT5663_HP_IMP_SEN_MAP10: u32 = 0x01b7u32;
pub const RT5663_HP_IMP_SEN_MAP11: u32 = 0x01b8u32;
pub const RT5663_HP_IMP_SEN_1: u32 = 0x01c0u32;
pub const RT5663_HP_IMP_SEN_2: u32 = 0x01c1u32;
pub const RT5663_HP_IMP_SEN_3: u32 = 0x01c2u32;
pub const RT5663_HP_IMP_SEN_4: u32 = 0x01c3u32;
pub const RT5663_HP_IMP_SEN_5: u32 = 0x01c4u32;
pub const RT5663_HP_IMP_SEN_6: u32 = 0x01c5u32;
pub const RT5663_HP_IMP_SEN_7: u32 = 0x01c6u32;
pub const RT5663_HP_IMP_SEN_8: u32 = 0x01c7u32;
pub const RT5663_HP_IMP_SEN_9: u32 = 0x01c8u32;
pub const RT5663_HP_IMP_SEN_10: u32 = 0x01c9u32;
pub const RT5663_HP_IMP_SEN_11: u32 = 0x01cau32;
pub const RT5663_HP_IMP_SEN_12: u32 = 0x01cbu32;
pub const RT5663_HP_IMP_SEN_13: u32 = 0x01ccu32;
pub const RT5663_HP_IMP_SEN_14: u32 = 0x01cdu32;
pub const RT5663_HP_IMP_SEN_15: u32 = 0x01ceu32;
pub const RT5663_HP_IMP_SEN_16: u32 = 0x01cfu32;
pub const RT5663_HP_IMP_SEN_17: u32 = 0x01d0u32;
pub const RT5663_HP_IMP_SEN_18: u32 = 0x01d1u32;
pub const RT5663_HP_IMP_SEN_19: u32 = 0x01d2u32;
pub const RT5663_HP_IMPSEN_DIG5: u32 = 0x01d3u32;
pub const RT5663_HP_IMPSEN_MAP1: u32 = 0x01d4u32;
pub const RT5663_HP_IMPSEN_MAP2: u32 = 0x01d5u32;
pub const RT5663_HP_IMPSEN_MAP3: u32 = 0x01d6u32;
pub const RT5663_HP_IMPSEN_MAP4: u32 = 0x01d7u32;
pub const RT5663_HP_IMPSEN_MAP5: u32 = 0x01d8u32;
pub const RT5663_HP_IMPSEN_MAP7: u32 = 0x01d9u32;
pub const RT5663_HP_LOGIC_1: u32 = 0x01dau32;
pub const RT5663_HP_LOGIC_2: u32 = 0x01dbu32;
pub const RT5663_HP_CALIB_1: u32 = 0x01ddu32;
pub const RT5663_HP_CALIB_1_1: u32 = 0x01deu32;
pub const RT5663_HP_CALIB_2: u32 = 0x01dfu32;
pub const RT5663_HP_CALIB_3: u32 = 0x01e0u32;
pub const RT5663_HP_CALIB_4: u32 = 0x01e1u32;
pub const RT5663_HP_CALIB_5: u32 = 0x01e2u32;
pub const RT5663_HP_CALIB_5_1: u32 = 0x01e3u32;
pub const RT5663_HP_CALIB_6: u32 = 0x01e4u32;
pub const RT5663_HP_CALIB_7: u32 = 0x01e5u32;
pub const RT5663_HP_CALIB_9: u32 = 0x01e6u32;
pub const RT5663_HP_CALIB_10: u32 = 0x01e7u32;
pub const RT5663_HP_CALIB_11: u32 = 0x01e8u32;
pub const RT5663_HP_CALIB_ST1: u32 = 0x01eau32;
pub const RT5663_HP_CALIB_ST2: u32 = 0x01ebu32;
pub const RT5663_HP_CALIB_ST3: u32 = 0x01ecu32;
pub const RT5663_HP_CALIB_ST4: u32 = 0x01edu32;
pub const RT5663_HP_CALIB_ST5: u32 = 0x01eeu32;
pub const RT5663_HP_CALIB_ST6: u32 = 0x01efu32;
pub const RT5663_HP_CALIB_ST7: u32 = 0x01f0u32;
pub const RT5663_HP_CALIB_ST8: u32 = 0x01f1u32;
pub const RT5663_HP_CALIB_ST9: u32 = 0x01f2u32;
pub const RT5663_HP_AMP_DET: u32 = 0x0200u32;
pub const RT5663_DUMMY_REG_6: u32 = 0x0201u32;
pub const RT5663_HP_BIAS: u32 = 0x0202u32;
pub const RT5663_CBJ_1: u32 = 0x0250u32;
pub const RT5663_CBJ_2: u32 = 0x0251u32;
pub const RT5663_CBJ_3: u32 = 0x0252u32;
pub const RT5663_DUMMY_1: u32 = 0x02fau32;
pub const RT5663_DUMMY_2: u32 = 0x02fbu32;
pub const RT5663_DUMMY_3: u32 = 0x02fcu32;
pub const RT5663_ANA_JD: u32 = 0x0300u32;
pub const RT5663_ADC_LCH_LPF1_A1: u32 = 0x03d0u32;
pub const RT5663_ADC_RCH_LPF1_A1: u32 = 0x03d1u32;
pub const RT5663_ADC_LCH_LPF1_H0: u32 = 0x03d2u32;
pub const RT5663_ADC_RCH_LPF1_H0: u32 = 0x03d3u32;
pub const RT5663_ADC_LCH_BPF1_A1: u32 = 0x03d4u32;
pub const RT5663_ADC_RCH_BPF1_A1: u32 = 0x03d5u32;
pub const RT5663_ADC_LCH_BPF1_A2: u32 = 0x03d6u32;
pub const RT5663_ADC_RCH_BPF1_A2: u32 = 0x03d7u32;
pub const RT5663_ADC_LCH_BPF1_H0: u32 = 0x03d8u32;
pub const RT5663_ADC_RCH_BPF1_H0: u32 = 0x03d9u32;
pub const RT5663_ADC_LCH_BPF2_A1: u32 = 0x03dau32;
pub const RT5663_ADC_RCH_BPF2_A1: u32 = 0x03dbu32;
pub const RT5663_ADC_LCH_BPF2_A2: u32 = 0x03dcu32;
pub const RT5663_ADC_RCH_BPF2_A2: u32 = 0x03ddu32;
pub const RT5663_ADC_LCH_BPF2_H0: u32 = 0x03deu32;
pub const RT5663_ADC_RCH_BPF2_H0: u32 = 0x03dfu32;
pub const RT5663_ADC_LCH_BPF3_A1: u32 = 0x03e0u32;
pub const RT5663_ADC_RCH_BPF3_A1: u32 = 0x03e1u32;
pub const RT5663_ADC_LCH_BPF3_A2: u32 = 0x03e2u32;
pub const RT5663_ADC_RCH_BPF3_A2: u32 = 0x03e3u32;
pub const RT5663_ADC_LCH_BPF3_H0: u32 = 0x03e4u32;
pub const RT5663_ADC_RCH_BPF3_H0: u32 = 0x03e5u32;
pub const RT5663_ADC_LCH_BPF4_A1: u32 = 0x03e6u32;
pub const RT5663_ADC_RCH_BPF4_A1: u32 = 0x03e7u32;
pub const RT5663_ADC_LCH_BPF4_A2: u32 = 0x03e8u32;
pub const RT5663_ADC_RCH_BPF4_A2: u32 = 0x03e9u32;
pub const RT5663_ADC_LCH_BPF4_H0: u32 = 0x03eau32;
pub const RT5663_ADC_RCH_BPF4_H0: u32 = 0x03ebu32;
pub const RT5663_ADC_LCH_HPF1_A1: u32 = 0x03ecu32;
pub const RT5663_ADC_RCH_HPF1_A1: u32 = 0x03edu32;
pub const RT5663_ADC_LCH_HPF1_H0: u32 = 0x03eeu32;
pub const RT5663_ADC_RCH_HPF1_H0: u32 = 0x03efu32;
pub const RT5663_ADC_EQ_PRE_VOL_L: u32 = 0x03f0u32;
pub const RT5663_ADC_EQ_PRE_VOL_R: u32 = 0x03f1u32;
pub const RT5663_ADC_EQ_POST_VOL_L: u32 = 0x03f2u32;
pub const RT5663_ADC_EQ_POST_VOL_R: u32 = 0x03f3u32;

/* RECMIX Control (0x0010) */
pub const RT5663_RECMIX1_BST1_MASK: u32 = (0x1u32);
pub const RT5663_RECMIX1_BST1_SHIFT: u32 = 0u32;
pub const RT5663_RECMIX1_BST1_ON: u32 = (0x0u32);
pub const RT5663_RECMIX1_BST1_OFF: u32 = (0x1u32);

/* Bypass Stereo1 DAC Mixer Control (0x002d) */
pub const RT5663_DACL1_SRC_MASK: u32 = (0x1u32 << 3u32);
pub const RT5663_DACL1_SRC_SHIFT: u32 = 3u32;
pub const RT5663_DACR1_SRC_MASK: u32 = (0x1u32 << 2u32);
pub const RT5663_DACR1_SRC_SHIFT: u32 = 2u32;

/* TDM control 2 (0x0078) */
pub const RT5663_DATA_SWAP_ADCDAT1_MASK: u32 = (0x3u32 << 14u32);
pub const RT5663_DATA_SWAP_ADCDAT1_SHIFT: u32 = 14u32;
pub const RT5663_DATA_SWAP_ADCDAT1_LR: u32 = (0x0u32 << 14u32);
pub const RT5663_DATA_SWAP_ADCDAT1_RL: u32 = (0x1u32 << 14u32);
pub const RT5663_DATA_SWAP_ADCDAT1_LL: u32 = (0x2u32 << 14u32);
pub const RT5663_DATA_SWAP_ADCDAT1_RR: u32 = (0x3u32 << 14u32);

/* TDM control 5 (0x007b) */
pub const RT5663_TDM_LENGTN_MASK: u32 = (0x3u32);
pub const RT5663_TDM_LENGTN_SHIFT: u32 = 0u32;
pub const RT5663_TDM_LENGTN_16: u32 = (0x0u32);
pub const RT5663_TDM_LENGTN_20: u32 = (0x1u32);
pub const RT5663_TDM_LENGTN_24: u32 = (0x2u32);
pub const RT5663_TDM_LENGTN_32: u32 = (0x3u32);

/* PLL tracking mode 1 (0x0083) */
pub const RT5663_I2S1_ASRC_MASK: u32 = (0x1u32 << 11u32);
pub const RT5663_I2S1_ASRC_SHIFT: u32 = 11u32;
pub const RT5663_DAC_STO1_ASRC_MASK: u32 = (0x1u32 << 10u32);
pub const RT5663_DAC_STO1_ASRC_SHIFT: u32 = 10u32;
pub const RT5663_ADC_STO1_ASRC_MASK: u32 = (0x1u32 << 3u32);
pub const RT5663_ADC_STO1_ASRC_SHIFT: u32 = 3u32;

/* PLL tracking mode 2 (0x0084)*/
pub const RT5663_DA_STO1_TRACK_MASK: u32 = (0x7u32 << 12u32);
pub const RT5663_DA_STO1_TRACK_SHIFT: u32 = 12u32;
pub const RT5663_DA_STO1_TRACK_SYSCLK: u32 = (0x0u32 << 12u32);
pub const RT5663_DA_STO1_TRACK_I2S1: u32 = (0x1u32 << 12u32);
pub const RT5663_AD_STO1_TRACK_MASK: u32 = (0x7u32);
pub const RT5663_AD_STO1_TRACK_SHIFT: u32 = 0u32;
pub const RT5663_AD_STO1_TRACK_SYSCLK: u32 = (0x0u32);
pub const RT5663_AD_STO1_TRACK_I2S1: u32 = (0x1u32);

/* HPOUT Charge pump control 1 (0x0091) */
pub const RT5663_SI_HP_MASK: u32 = (0x1u32 << 12u32);
pub const RT5663_SI_HP_SHIFT: u32 = 12u32;
pub const RT5663_SI_HP_EN: u32 = (0x1u32 << 12u32);
pub const RT5663_SI_HP_DIS: u32 = (0x0u32 << 12u32);

/* GPIO Control 2 (0x00b6) */
pub const RT5663_GP1_PIN_CONF_MASK: u32 = (0x1u32 << 2u32);
pub const RT5663_GP1_PIN_CONF_SHIFT: u32 = 2u32;
pub const RT5663_GP1_PIN_CONF_OUTPUT: u32 = (0x1u32 << 2u32);
pub const RT5663_GP1_PIN_CONF_INPUT: u32 = (0x0u32 << 2u32);

/* GPIO Control 2 (0x00b7) */
pub const RT5663_EN_IRQ_INLINE_MASK: u32 = (0x1u32 << 3u32);
pub const RT5663_EN_IRQ_INLINE_SHIFT: u32 = 3u32;
pub const RT5663_EN_IRQ_INLINE_NOR: u32 = (0x1u32 << 3u32);
pub const RT5663_EN_IRQ_INLINE_BYP: u32 = (0x0u32 << 3u32);

/* GPIO Control 1 (0x00c0) */
pub const RT5663_GPIO1_TYPE_MASK: u32 = (0x1u32 << 15u32);
pub const RT5663_GPIO1_TYPE_SHIFT: u32 = 15u32;
pub const RT5663_GPIO1_TYPE_EN: u32 = (0x1u32 << 15u32);
pub const RT5663_GPIO1_TYPE_DIS: u32 = (0x0u32 << 15u32);

/* IRQ Control 1 (0x00c1) */
pub const RT5663_EN_IRQ_JD1_MASK: u32 = (0x1u32 << 6u32);
pub const RT5663_EN_IRQ_JD1_SHIFT: u32 = 6u32;
pub const RT5663_EN_IRQ_JD1_EN: u32 = (0x1u32 << 6u32);
pub const RT5663_EN_IRQ_JD1_DIS: u32 = (0x0u32 << 6u32);
pub const RT5663_SEL_GPIO1_MASK: u32 = (0x1u32 << 2u32);
pub const RT5663_SEL_GPIO1_SHIFT: u32 = 6u32;
pub const RT5663_SEL_GPIO1_EN: u32 = (0x1u32 << 2u32);
pub const RT5663_SEL_GPIO1_DIS: u32 = (0x0u32 << 2u32);

/* Inline Command Function 2 (0x00dc) */
pub const RT5663_PWR_MIC_DET_MASK: u32 = (0x1u32);
pub const RT5663_PWR_MIC_DET_SHIFT: u32 = 0u32;
pub const RT5663_PWR_MIC_DET_ON: u32 = (0x1u32);
pub const RT5663_PWR_MIC_DET_OFF: u32 = (0x0u32);

/* Embeeded Jack and Type Detection Control 1 (0x00e6)*/
pub const RT5663_CBJ_DET_MASK: u32 = (0x1u32 << 15u32);
pub const RT5663_CBJ_DET_SHIFT: u32 = 15u32;
pub const RT5663_CBJ_DET_DIS: u32 = (0x0u32 << 15u32);
pub const RT5663_CBJ_DET_EN: u32 = (0x1u32 << 15u32);
pub const RT5663_EXT_JD_MASK: u32 = (0x1u32 << 11u32);
pub const RT5663_EXT_JD_SHIFT: u32 = 11u32;
pub const RT5663_EXT_JD_EN: u32 = (0x1u32 << 11u32);
pub const RT5663_EXT_JD_DIS: u32 = (0x0u32 << 11u32);
pub const RT5663_POL_EXT_JD_MASK: u32 = (0x1u32 << 10u32);
pub const RT5663_POL_EXT_JD_SHIFT: u32 = 10u32;
pub const RT5663_POL_EXT_JD_EN: u32 = (0x1u32 << 10u32);
pub const RT5663_POL_EXT_JD_DIS: u32 = (0x0u32 << 10u32);
pub const RT5663_EM_JD_MASK: u32 = (0x1u32 << 7u32);
pub const RT5663_EM_JD_SHIFT: u32 = 7u32;
pub const RT5663_EM_JD_NOR: u32 = (0x1u32 << 7u32);
pub const RT5663_EM_JD_RST: u32 = (0x0u32 << 7u32);

/* DACREF LDO Control (0x0112)*/
pub const RT5663_PWR_LDO_DACREFL_MASK: u32 = (0x1u32 << 9u32);
pub const RT5663_PWR_LDO_DACREFL_SHIFT: u32 = 9u32;
pub const RT5663_PWR_LDO_DACREFR_MASK: u32 = (0x1u32 << 1u32);
pub const RT5663_PWR_LDO_DACREFR_SHIFT: u32 = 1u32;

/* Stereo Dynamic Range Enhancement Control 9 (0x0168, 0x0169)*/
pub const RT5663_DRE_GAIN_HP_MASK: u32 = (0x1fu32);
pub const RT5663_DRE_GAIN_HP_SHIFT: u32 = 0u32;

/* Combo Jack Control (0x0250) */
pub const RT5663_INBUF_CBJ_BST1_MASK: u32 = (0x1u32 << 11u32);
pub const RT5663_INBUF_CBJ_BST1_SHIFT: u32 = 11u32;
pub const RT5663_INBUF_CBJ_BST1_ON: u32 = (0x1u32 << 11u32);
pub const RT5663_INBUF_CBJ_BST1_OFF: u32 = (0x0u32 << 11u32);
pub const RT5663_CBJ_SENSE_BST1_MASK: u32 = (0x1u32 << 10u32);
pub const RT5663_CBJ_SENSE_BST1_SHIFT: u32 = 10u32;
pub const RT5663_CBJ_SENSE_BST1_L: u32 = (0x1u32 << 10u32);
pub const RT5663_CBJ_SENSE_BST1_R: u32 = (0x0u32 << 10u32);

/* Combo Jack Control (0x0251) */
pub const RT5663_GAIN_BST1_MASK: u32 = (0xfu32);
pub const RT5663_GAIN_BST1_SHIFT: u32 = 0u32;

/* Dummy register 1 (0x02fa) */
pub const RT5663_EMB_CLK_MASK: u32 = (0x1u32 << 9u32);
pub const RT5663_EMB_CLK_SHIFT: u32 = 9u32;
pub const RT5663_EMB_CLK_EN: u32 = (0x1u32 << 9u32);
pub const RT5663_EMB_CLK_DIS: u32 = (0x0u32 << 9u32);
pub const RT5663_HPA_CPL_BIAS_MASK: u32 = (0x7u32 << 6u32);
pub const RT5663_HPA_CPL_BIAS_SHIFT: u32 = 6u32;
pub const RT5663_HPA_CPL_BIAS_0_5: u32 = (0x0u32 << 6u32);
pub const RT5663_HPA_CPL_BIAS_1: u32 = (0x1u32 << 6u32);
pub const RT5663_HPA_CPL_BIAS_2: u32 = (0x2u32 << 6u32);
pub const RT5663_HPA_CPL_BIAS_3: u32 = (0x3u32 << 6u32);
pub const RT5663_HPA_CPL_BIAS_4_1: u32 = (0x4u32 << 6u32);
pub const RT5663_HPA_CPL_BIAS_4_2: u32 = (0x5u32 << 6u32);
pub const RT5663_HPA_CPL_BIAS_6: u32 = (0x6u32 << 6u32);
pub const RT5663_HPA_CPL_BIAS_8: u32 = (0x7u32 << 6u32);
pub const RT5663_HPA_CPR_BIAS_MASK: u32 = (0x7u32 << 3u32);
pub const RT5663_HPA_CPR_BIAS_SHIFT: u32 = 3u32;
pub const RT5663_HPA_CPR_BIAS_0_5: u32 = (0x0u32 << 3u32);
pub const RT5663_HPA_CPR_BIAS_1: u32 = (0x1u32 << 3u32);
pub const RT5663_HPA_CPR_BIAS_2: u32 = (0x2u32 << 3u32);
pub const RT5663_HPA_CPR_BIAS_3: u32 = (0x3u32 << 3u32);
pub const RT5663_HPA_CPR_BIAS_4_1: u32 = (0x4u32 << 3u32);
pub const RT5663_HPA_CPR_BIAS_4_2: u32 = (0x5u32 << 3u32);
pub const RT5663_HPA_CPR_BIAS_6: u32 = (0x6u32 << 3u32);
pub const RT5663_HPA_CPR_BIAS_8: u32 = (0x7u32 << 3u32);
pub const RT5663_DUMMY_BIAS_MASK: u32 = (0x7u32);
pub const RT5663_DUMMY_BIAS_SHIFT: u32 = 0u32;
pub const RT5663_DUMMY_BIAS_0_5: u32 = (0x0u32);
pub const RT5663_DUMMY_BIAS_1: u32 = (0x1u32);
pub const RT5663_DUMMY_BIAS_2: u32 = (0x2u32);
pub const RT5663_DUMMY_BIAS_3: u32 = (0x3u32);
pub const RT5663_DUMMY_BIAS_4_1: u32 = (0x4u32);
pub const RT5663_DUMMY_BIAS_4_2: u32 = (0x5u32);
pub const RT5663_DUMMY_BIAS_6: u32 = (0x6u32);
pub const RT5663_DUMMY_BIAS_8: u32 = (0x7u32);


/* System Clock Source */
pub const RT5663_SCLK_S_MCLK: u32 = 0u32;
pub const RT5663_SCLK_S_PLL1: u32 = 1u32;
pub const RT5663_SCLK_S_RCCLK: u32 = 2u32;


/* PLL1 Source */
pub const RT5663_PLL1_S_MCLK: u32 = 0u32;
pub const RT5663_PLL1_S_BCLK1: u32 = 1u32;


pub const RT5663_AIF: u32 = 0u32;
pub const RT5663_AIFS: u32 = 1u32;


/* asrc clock source */
pub const RT5663_CLK_SEL_SYS: u32 = 0x0u32;
pub const RT5663_CLK_SEL_I2S1_ASRC: u32 = 0x1u32;


/* filter mask */
pub const RT5663_DA_STEREO_FILTER: u32 = 0x1u32;
pub const RT5663_AD_STEREO_FILTER: u32 = 0x2u32;


unsafe extern "C" {
    pub fn rt5663_sel_asrc_clk_src(
        component: *mut snd_soc_component,
        filter_mask: ::core::ffi::c_uint,
        clk_src: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
