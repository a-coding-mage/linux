/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rt5640.h  --  RT5640 ALSA SoC audio driver
 *
 * Copyright 2011 Realtek Microelectronics
 * Author: Johnny Hsu <johnnyhsu@realtek.com>
 */

// Dependencies from the original C header: <linux/clk.h>, <linux/gpio/consumer.h>, <linux/workqueue.h>, <dt-bindings/sound/rt5640.h>.
pub type bool_ = bool;

#[repr(C)]
pub struct snd_soc_component { _private: [u8; 0] }
#[repr(C)]
pub struct regmap { _private: [u8; 0] }
#[repr(C)]
pub struct clk { _private: [u8; 0] }
#[repr(C)]
pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)]
pub struct delayed_work { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_jack { _private: [u8; 0] }

/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rt5640.h  --  RT5640 ALSA SoC audio driver
 *
 * Copyright 2011 Realtek Microelectronics
 * Author: Johnny Hsu <johnnyhsu@realtek.com>
 */



/* Info */
pub const RT5640_RESET: u32 = 0x00u32;
pub const RT5640_VENDOR_ID: u32 = 0xfdu32;
pub const RT5640_VENDOR_ID1: u32 = 0xfeu32;
pub const RT5640_VENDOR_ID2: u32 = 0xffu32;
/*  I/O - Output */
pub const RT5640_SPK_VOL: u32 = 0x01u32;
pub const RT5640_HP_VOL: u32 = 0x02u32;
pub const RT5640_OUTPUT: u32 = 0x03u32;
pub const RT5640_MONO_OUT: u32 = 0x04u32;
/* I/O - Input */
pub const RT5640_IN1_IN2: u32 = 0x0du32;
pub const RT5640_IN3_IN4: u32 = 0x0eu32;
pub const RT5640_INL_INR_VOL: u32 = 0x0fu32;
/* I/O - ADC/DAC/DMIC */
pub const RT5640_DAC1_DIG_VOL: u32 = 0x19u32;
pub const RT5640_DAC2_DIG_VOL: u32 = 0x1au32;
pub const RT5640_DAC2_CTRL: u32 = 0x1bu32;
pub const RT5640_ADC_DIG_VOL: u32 = 0x1cu32;
pub const RT5640_ADC_DATA: u32 = 0x1du32;
pub const RT5640_ADC_BST_VOL: u32 = 0x1eu32;
/* Mixer - D-D */
pub const RT5640_STO_ADC_MIXER: u32 = 0x27u32;
pub const RT5640_MONO_ADC_MIXER: u32 = 0x28u32;
pub const RT5640_AD_DA_MIXER: u32 = 0x29u32;
pub const RT5640_STO_DAC_MIXER: u32 = 0x2au32;
pub const RT5640_MONO_DAC_MIXER: u32 = 0x2bu32;
pub const RT5640_DIG_MIXER: u32 = 0x2cu32;
pub const RT5640_DSP_PATH1: u32 = 0x2du32;
pub const RT5640_DSP_PATH2: u32 = 0x2eu32;
pub const RT5640_DIG_INF_DATA: u32 = 0x2fu32;
/* Mixer - ADC */
pub const RT5640_REC_L1_MIXER: u32 = 0x3bu32;
pub const RT5640_REC_L2_MIXER: u32 = 0x3cu32;
pub const RT5640_REC_R1_MIXER: u32 = 0x3du32;
pub const RT5640_REC_R2_MIXER: u32 = 0x3eu32;
/* Mixer - DAC */
pub const RT5640_HPO_MIXER: u32 = 0x45u32;
pub const RT5640_SPK_L_MIXER: u32 = 0x46u32;
pub const RT5640_SPK_R_MIXER: u32 = 0x47u32;
pub const RT5640_SPO_L_MIXER: u32 = 0x48u32;
pub const RT5640_SPO_R_MIXER: u32 = 0x49u32;
pub const RT5640_SPO_CLSD_RATIO: u32 = 0x4au32;
pub const RT5640_MONO_MIXER: u32 = 0x4cu32;
pub const RT5640_OUT_L1_MIXER: u32 = 0x4du32;
pub const RT5640_OUT_L2_MIXER: u32 = 0x4eu32;
pub const RT5640_OUT_L3_MIXER: u32 = 0x4fu32;
pub const RT5640_OUT_R1_MIXER: u32 = 0x50u32;
pub const RT5640_OUT_R2_MIXER: u32 = 0x51u32;
pub const RT5640_OUT_R3_MIXER: u32 = 0x52u32;
pub const RT5640_LOUT_MIXER: u32 = 0x53u32;
/* Power */
pub const RT5640_PWR_DIG1: u32 = 0x61u32;
pub const RT5640_PWR_DIG2: u32 = 0x62u32;
pub const RT5640_PWR_ANLG1: u32 = 0x63u32;
pub const RT5640_PWR_ANLG2: u32 = 0x64u32;
pub const RT5640_PWR_MIXER: u32 = 0x65u32;
pub const RT5640_PWR_VOL: u32 = 0x66u32;
/* Private Register Control */
pub const RT5640_PRIV_INDEX: u32 = 0x6au32;
pub const RT5640_PRIV_DATA: u32 = 0x6cu32;
/* Format - ADC/DAC */
pub const RT5640_I2S1_SDP: u32 = 0x70u32;
pub const RT5640_I2S2_SDP: u32 = 0x71u32;
pub const RT5640_ADDA_CLK1: u32 = 0x73u32;
pub const RT5640_ADDA_CLK2: u32 = 0x74u32;
pub const RT5640_DMIC: u32 = 0x75u32;
/* Function - Analog */
pub const RT5640_GLB_CLK: u32 = 0x80u32;
pub const RT5640_PLL_CTRL1: u32 = 0x81u32;
pub const RT5640_PLL_CTRL2: u32 = 0x82u32;
pub const RT5640_ASRC_1: u32 = 0x83u32;
pub const RT5640_ASRC_2: u32 = 0x84u32;
pub const RT5640_ASRC_3: u32 = 0x85u32;
pub const RT5640_ASRC_4: u32 = 0x89u32;
pub const RT5640_ASRC_5: u32 = 0x8au32;
pub const RT5640_HP_OVCD: u32 = 0x8bu32;
pub const RT5640_CLS_D_OVCD: u32 = 0x8cu32;
pub const RT5640_CLS_D_OUT: u32 = 0x8du32;
pub const RT5640_DEPOP_M1: u32 = 0x8eu32;
pub const RT5640_DEPOP_M2: u32 = 0x8fu32;
pub const RT5640_DEPOP_M3: u32 = 0x90u32;
pub const RT5640_CHARGE_PUMP: u32 = 0x91u32;
pub const RT5640_PV_DET_SPK_G: u32 = 0x92u32;
pub const RT5640_MICBIAS: u32 = 0x93u32;
/* Function - Digital */
pub const RT5640_EQ_CTRL1: u32 = 0xb0u32;
pub const RT5640_EQ_CTRL2: u32 = 0xb1u32;
pub const RT5640_WIND_FILTER: u32 = 0xb2u32;
pub const RT5640_DRC_AGC_1: u32 = 0xb4u32;
pub const RT5640_DRC_AGC_2: u32 = 0xb5u32;
pub const RT5640_DRC_AGC_3: u32 = 0xb6u32;
pub const RT5640_SVOL_ZC: u32 = 0xb7u32;
pub const RT5640_ANC_CTRL1: u32 = 0xb8u32;
pub const RT5640_ANC_CTRL2: u32 = 0xb9u32;
pub const RT5640_ANC_CTRL3: u32 = 0xbau32;
pub const RT5640_JD_CTRL: u32 = 0xbbu32;
pub const RT5640_ANC_JD: u32 = 0xbcu32;
pub const RT5640_IRQ_CTRL1: u32 = 0xbdu32;
pub const RT5640_IRQ_CTRL2: u32 = 0xbeu32;
pub const RT5640_INT_IRQ_ST: u32 = 0xbfu32;
pub const RT5640_GPIO_CTRL1: u32 = 0xc0u32;
pub const RT5640_GPIO_CTRL2: u32 = 0xc1u32;
pub const RT5640_GPIO_CTRL3: u32 = 0xc2u32;
pub const RT5640_DSP_CTRL1: u32 = 0xc4u32;
pub const RT5640_DSP_CTRL2: u32 = 0xc5u32;
pub const RT5640_DSP_CTRL3: u32 = 0xc6u32;
pub const RT5640_DSP_CTRL4: u32 = 0xc7u32;
pub const RT5640_PGM_REG_ARR1: u32 = 0xc8u32;
pub const RT5640_PGM_REG_ARR2: u32 = 0xc9u32;
pub const RT5640_PGM_REG_ARR3: u32 = 0xcau32;
pub const RT5640_PGM_REG_ARR4: u32 = 0xcbu32;
pub const RT5640_PGM_REG_ARR5: u32 = 0xccu32;
pub const RT5640_SCB_FUNC: u32 = 0xcdu32;
pub const RT5640_SCB_CTRL: u32 = 0xceu32;
pub const RT5640_BASE_BACK: u32 = 0xcfu32;
pub const RT5640_MP3_PLUS1: u32 = 0xd0u32;
pub const RT5640_MP3_PLUS2: u32 = 0xd1u32;
pub const RT5640_3D_HP: u32 = 0xd2u32;
pub const RT5640_ADJ_HPF: u32 = 0xd3u32;
pub const RT5640_HP_CALIB_AMP_DET: u32 = 0xd6u32;
pub const RT5640_HP_CALIB2: u32 = 0xd7u32;
pub const RT5640_SV_ZCD1: u32 = 0xd9u32;
pub const RT5640_SV_ZCD2: u32 = 0xdau32;
/* Dummy Register */
pub const RT5640_GCTL1: u32 = 0xfau32;
pub const RT5640_GCTL2: u32 = 0xfbu32;
pub const RT5640_DUMMY3: u32 = 0xfcu32;


/* Index of Codec Private Register definition */
pub const RT5640_BIAS_CUR4: u32 = 0x15u32;
pub const RT5640_CHPUMP_INT_REG1: u32 = 0x24u32;
pub const RT5640_MAMP_INT_REG2: u32 = 0x37u32;
pub const RT5640_3D_SPK: u32 = 0x63u32;
pub const RT5640_WND_1: u32 = 0x6cu32;
pub const RT5640_WND_2: u32 = 0x6du32;
pub const RT5640_WND_3: u32 = 0x6eu32;
pub const RT5640_WND_4: u32 = 0x6fu32;
pub const RT5640_WND_5: u32 = 0x70u32;
pub const RT5640_WND_8: u32 = 0x73u32;
pub const RT5640_DIP_SPK_INF: u32 = 0x75u32;
pub const RT5640_HP_DCC_INT1: u32 = 0x77u32;
pub const RT5640_EQ_BW_LOP: u32 = 0xa0u32;
pub const RT5640_EQ_GN_LOP: u32 = 0xa1u32;
pub const RT5640_EQ_FC_BP1: u32 = 0xa2u32;
pub const RT5640_EQ_BW_BP1: u32 = 0xa3u32;
pub const RT5640_EQ_GN_BP1: u32 = 0xa4u32;
pub const RT5640_EQ_FC_BP2: u32 = 0xa5u32;
pub const RT5640_EQ_BW_BP2: u32 = 0xa6u32;
pub const RT5640_EQ_GN_BP2: u32 = 0xa7u32;
pub const RT5640_EQ_FC_BP3: u32 = 0xa8u32;
pub const RT5640_EQ_BW_BP3: u32 = 0xa9u32;
pub const RT5640_EQ_GN_BP3: u32 = 0xaau32;
pub const RT5640_EQ_FC_BP4: u32 = 0xabu32;
pub const RT5640_EQ_BW_BP4: u32 = 0xacu32;
pub const RT5640_EQ_GN_BP4: u32 = 0xadu32;
pub const RT5640_EQ_FC_HIP1: u32 = 0xaeu32;
pub const RT5640_EQ_GN_HIP1: u32 = 0xafu32;
pub const RT5640_EQ_FC_HIP2: u32 = 0xb0u32;
pub const RT5640_EQ_BW_HIP2: u32 = 0xb1u32;
pub const RT5640_EQ_GN_HIP2: u32 = 0xb2u32;
pub const RT5640_EQ_PRE_VOL: u32 = 0xb3u32;
pub const RT5640_EQ_PST_VOL: u32 = 0xb4u32;

/* global definition */
pub const RT5640_L_MUTE: u32 = (0x1u32 << 15u32);
pub const RT5640_L_MUTE_SFT: u32 = 15u32;
pub const RT5640_VOL_L_MUTE: u32 = (0x1u32 << 14u32);
pub const RT5640_VOL_L_SFT: u32 = 14u32;
pub const RT5640_R_MUTE: u32 = (0x1u32 << 7u32);
pub const RT5640_R_MUTE_SFT: u32 = 7u32;
pub const RT5640_VOL_R_MUTE: u32 = (0x1u32 << 6u32);
pub const RT5640_VOL_R_SFT: u32 = 6u32;
pub const RT5640_L_VOL_MASK: u32 = (0x3fu32 << 8u32);
pub const RT5640_L_VOL_SFT: u32 = 8u32;
pub const RT5640_R_VOL_MASK: u32 = 0x3fu32;
pub const RT5640_R_VOL_SFT: u32 = 0u32;

/* SW Reset & Device ID (0x00) */
pub const RT5640_ID_MASK: u32 = (0x3u32 << 1u32);
pub const RT5640_ID_5639: u32 = (0x0u32 << 1u32);
pub const RT5640_ID_5640: u32 = (0x2u32 << 1u32);
pub const RT5640_ID_5642: u32 = (0x3u32 << 1u32);


/* IN1 and IN2 Control (0x0d) */
/* IN3 and IN4 Control (0x0e) */
pub const RT5640_BST_SFT1: u32 = 12u32;
pub const RT5640_BST_SFT2: u32 = 8u32;
pub const RT5640_IN_DF1: u32 = (0x1u32 << 7u32);
pub const RT5640_IN_SFT1: u32 = 7u32;
pub const RT5640_IN_DF2: u32 = (0x1u32 << 6u32);
pub const RT5640_IN_SFT2: u32 = 6u32;

/* INL and INR Volume Control (0x0f) */
pub const RT5640_INL_SEL_MASK: u32 = (0x1u32 << 15u32);
pub const RT5640_INL_SEL_SFT: u32 = 15u32;
pub const RT5640_INL_SEL_IN4P: u32 = (0x0u32 << 15u32);
pub const RT5640_INL_SEL_MONOP: u32 = (0x1u32 << 15u32);
pub const RT5640_INL_VOL_MASK: u32 = (0x1fu32 << 8u32);
pub const RT5640_INL_VOL_SFT: u32 = 8u32;
pub const RT5640_INR_SEL_MASK: u32 = (0x1u32 << 7u32);
pub const RT5640_INR_SEL_SFT: u32 = 7u32;
pub const RT5640_INR_SEL_IN4N: u32 = (0x0u32 << 7u32);
pub const RT5640_INR_SEL_MONON: u32 = (0x1u32 << 7u32);
pub const RT5640_INR_VOL_MASK: u32 = 0x1fu32;
pub const RT5640_INR_VOL_SFT: u32 = 0u32;

/* DAC1 Digital Volume (0x19) */
pub const RT5640_DAC_L1_VOL_MASK: u32 = (0xffu32 << 8u32);
pub const RT5640_DAC_L1_VOL_SFT: u32 = 8u32;
pub const RT5640_DAC_R1_VOL_MASK: u32 = 0xffu32;
pub const RT5640_DAC_R1_VOL_SFT: u32 = 0u32;

/* DAC2 Digital Volume (0x1a) */
pub const RT5640_DAC_L2_VOL_MASK: u32 = (0xffu32 << 8u32);
pub const RT5640_DAC_L2_VOL_SFT: u32 = 8u32;
pub const RT5640_DAC_R2_VOL_MASK: u32 = 0xffu32;
pub const RT5640_DAC_R2_VOL_SFT: u32 = 0u32;

/* DAC2 Control (0x1b) */
pub const RT5640_M_DAC_L2_VOL: u32 = (0x1u32 << 13u32);
pub const RT5640_M_DAC_L2_VOL_SFT: u32 = 13u32;
pub const RT5640_M_DAC_R2_VOL: u32 = (0x1u32 << 12u32);
pub const RT5640_M_DAC_R2_VOL_SFT: u32 = 12u32;

/* ADC Digital Volume Control (0x1c) */
pub const RT5640_ADC_L_VOL_MASK: u32 = (0x7fu32 << 8u32);
pub const RT5640_ADC_L_VOL_SFT: u32 = 8u32;
pub const RT5640_ADC_R_VOL_MASK: u32 = 0x7fu32;
pub const RT5640_ADC_R_VOL_SFT: u32 = 0u32;

/* Mono ADC Digital Volume Control (0x1d) */
pub const RT5640_MONO_ADC_L_VOL_MASK: u32 = (0x7fu32 << 8u32);
pub const RT5640_MONO_ADC_L_VOL_SFT: u32 = 8u32;
pub const RT5640_MONO_ADC_R_VOL_MASK: u32 = 0x7fu32;
pub const RT5640_MONO_ADC_R_VOL_SFT: u32 = 0u32;

/* ADC Boost Volume Control (0x1e) */
pub const RT5640_ADC_L_BST_MASK: u32 = (0x3u32 << 14u32);
pub const RT5640_ADC_L_BST_SFT: u32 = 14u32;
pub const RT5640_ADC_R_BST_MASK: u32 = (0x3u32 << 12u32);
pub const RT5640_ADC_R_BST_SFT: u32 = 12u32;
pub const RT5640_ADC_COMP_MASK: u32 = (0x3u32 << 10u32);
pub const RT5640_ADC_COMP_SFT: u32 = 10u32;

/* Stereo ADC Mixer Control (0x27) */
pub const RT5640_M_ADC_L1: u32 = (0x1u32 << 14u32);
pub const RT5640_M_ADC_L1_SFT: u32 = 14u32;
pub const RT5640_M_ADC_L2: u32 = (0x1u32 << 13u32);
pub const RT5640_M_ADC_L2_SFT: u32 = 13u32;
pub const RT5640_ADC_1_SRC_MASK: u32 = (0x1u32 << 12u32);
pub const RT5640_ADC_1_SRC_SFT: u32 = 12u32;
pub const RT5640_ADC_1_SRC_ADC: u32 = (0x1u32 << 12u32);
pub const RT5640_ADC_1_SRC_DACMIX: u32 = (0x0u32 << 12u32);
pub const RT5640_ADC_2_SRC_MASK: u32 = (0x3u32 << 10u32);
pub const RT5640_ADC_2_SRC_SFT: u32 = 10u32;
pub const RT5640_ADC_2_SRC_DMIC1: u32 = (0x0u32 << 10u32);
pub const RT5640_ADC_2_SRC_DMIC2: u32 = (0x1u32 << 10u32);
pub const RT5640_ADC_2_SRC_DACMIX: u32 = (0x2u32 << 10u32);
pub const RT5640_M_ADC_R1: u32 = (0x1u32 << 6u32);
pub const RT5640_M_ADC_R1_SFT: u32 = 6u32;
pub const RT5640_M_ADC_R2: u32 = (0x1u32 << 5u32);
pub const RT5640_M_ADC_R2_SFT: u32 = 5u32;

/* Mono ADC Mixer Control (0x28) */
pub const RT5640_M_MONO_ADC_L1: u32 = (0x1u32 << 14u32);
pub const RT5640_M_MONO_ADC_L1_SFT: u32 = 14u32;
pub const RT5640_M_MONO_ADC_L2: u32 = (0x1u32 << 13u32);
pub const RT5640_M_MONO_ADC_L2_SFT: u32 = 13u32;
pub const RT5640_MONO_ADC_L1_SRC_MASK: u32 = (0x1u32 << 12u32);
pub const RT5640_MONO_ADC_L1_SRC_SFT: u32 = 12u32;
pub const RT5640_MONO_ADC_L1_SRC_DACMIXL: u32 = (0x0u32 << 12u32);
pub const RT5640_MONO_ADC_L1_SRC_ADCL: u32 = (0x1u32 << 12u32);
pub const RT5640_MONO_ADC_L2_SRC_MASK: u32 = (0x3u32 << 10u32);
pub const RT5640_MONO_ADC_L2_SRC_SFT: u32 = 10u32;
pub const RT5640_MONO_ADC_L2_SRC_DMIC_L1: u32 = (0x0u32 << 10u32);
pub const RT5640_MONO_ADC_L2_SRC_DMIC_L2: u32 = (0x1u32 << 10u32);
pub const RT5640_MONO_ADC_L2_SRC_DACMIXL: u32 = (0x2u32 << 10u32);
pub const RT5640_M_MONO_ADC_R1: u32 = (0x1u32 << 6u32);
pub const RT5640_M_MONO_ADC_R1_SFT: u32 = 6u32;
pub const RT5640_M_MONO_ADC_R2: u32 = (0x1u32 << 5u32);
pub const RT5640_M_MONO_ADC_R2_SFT: u32 = 5u32;
pub const RT5640_MONO_ADC_R1_SRC_MASK: u32 = (0x1u32 << 4u32);
pub const RT5640_MONO_ADC_R1_SRC_SFT: u32 = 4u32;
pub const RT5640_MONO_ADC_R1_SRC_ADCR: u32 = (0x1u32 << 4u32);
pub const RT5640_MONO_ADC_R1_SRC_DACMIXR: u32 = (0x0u32 << 4u32);
pub const RT5640_MONO_ADC_R2_SRC_MASK: u32 = (0x3u32 << 2u32);
pub const RT5640_MONO_ADC_R2_SRC_SFT: u32 = 2u32;
pub const RT5640_MONO_ADC_R2_SRC_DMIC_R1: u32 = (0x0u32 << 2u32);
pub const RT5640_MONO_ADC_R2_SRC_DMIC_R2: u32 = (0x1u32 << 2u32);
pub const RT5640_MONO_ADC_R2_SRC_DACMIXR: u32 = (0x2u32 << 2u32);

/* ADC Mixer to DAC Mixer Control (0x29) */
pub const RT5640_M_ADCMIX_L: u32 = (0x1u32 << 15u32);
pub const RT5640_M_ADCMIX_L_SFT: u32 = 15u32;
pub const RT5640_M_IF1_DAC_L: u32 = (0x1u32 << 14u32);
pub const RT5640_M_IF1_DAC_L_SFT: u32 = 14u32;
pub const RT5640_M_ADCMIX_R: u32 = (0x1u32 << 7u32);
pub const RT5640_M_ADCMIX_R_SFT: u32 = 7u32;
pub const RT5640_M_IF1_DAC_R: u32 = (0x1u32 << 6u32);
pub const RT5640_M_IF1_DAC_R_SFT: u32 = 6u32;

/* Stereo DAC Mixer Control (0x2a) */
pub const RT5640_M_DAC_L1: u32 = (0x1u32 << 14u32);
pub const RT5640_M_DAC_L1_SFT: u32 = 14u32;
pub const RT5640_DAC_L1_STO_L_VOL_MASK: u32 = (0x1u32 << 13u32);
pub const RT5640_DAC_L1_STO_L_VOL_SFT: u32 = 13u32;
pub const RT5640_M_DAC_L2: u32 = (0x1u32 << 12u32);
pub const RT5640_M_DAC_L2_SFT: u32 = 12u32;
pub const RT5640_DAC_L2_STO_L_VOL_MASK: u32 = (0x1u32 << 11u32);
pub const RT5640_DAC_L2_STO_L_VOL_SFT: u32 = 11u32;
pub const RT5640_M_ANC_DAC_L: u32 = (0x1u32 << 10u32);
pub const RT5640_M_ANC_DAC_L_SFT: u32 = 10u32;
pub const RT5640_M_DAC_R1: u32 = (0x1u32 << 6u32);
pub const RT5640_M_DAC_R1_SFT: u32 = 6u32;
pub const RT5640_DAC_R1_STO_R_VOL_MASK: u32 = (0x1u32 << 5u32);
pub const RT5640_DAC_R1_STO_R_VOL_SFT: u32 = 5u32;
pub const RT5640_M_DAC_R2: u32 = (0x1u32 << 4u32);
pub const RT5640_M_DAC_R2_SFT: u32 = 4u32;
pub const RT5640_DAC_R2_STO_R_VOL_MASK: u32 = (0x1u32 << 3u32);
pub const RT5640_DAC_R2_STO_R_VOL_SFT: u32 = 3u32;
pub const RT5640_M_ANC_DAC_R: u32 = (0x1u32 << 2u32);
pub const RT5640_M_ANC_DAC_R_SFT: u32 = 2u32;

/* Mono DAC Mixer Control (0x2b) */
pub const RT5640_M_DAC_L1_MONO_L: u32 = (0x1u32 << 14u32);
pub const RT5640_M_DAC_L1_MONO_L_SFT: u32 = 14u32;
pub const RT5640_DAC_L1_MONO_L_VOL_MASK: u32 = (0x1u32 << 13u32);
pub const RT5640_DAC_L1_MONO_L_VOL_SFT: u32 = 13u32;
pub const RT5640_M_DAC_L2_MONO_L: u32 = (0x1u32 << 12u32);
pub const RT5640_M_DAC_L2_MONO_L_SFT: u32 = 12u32;
pub const RT5640_DAC_L2_MONO_L_VOL_MASK: u32 = (0x1u32 << 11u32);
pub const RT5640_DAC_L2_MONO_L_VOL_SFT: u32 = 11u32;
pub const RT5640_M_DAC_R2_MONO_L: u32 = (0x1u32 << 10u32);
pub const RT5640_M_DAC_R2_MONO_L_SFT: u32 = 10u32;
pub const RT5640_DAC_R2_MONO_L_VOL_MASK: u32 = (0x1u32 << 9u32);
pub const RT5640_DAC_R2_MONO_L_VOL_SFT: u32 = 9u32;
pub const RT5640_M_DAC_R1_MONO_R: u32 = (0x1u32 << 6u32);
pub const RT5640_M_DAC_R1_MONO_R_SFT: u32 = 6u32;
pub const RT5640_DAC_R1_MONO_R_VOL_MASK: u32 = (0x1u32 << 5u32);
pub const RT5640_DAC_R1_MONO_R_VOL_SFT: u32 = 5u32;
pub const RT5640_M_DAC_R2_MONO_R: u32 = (0x1u32 << 4u32);
pub const RT5640_M_DAC_R2_MONO_R_SFT: u32 = 4u32;
pub const RT5640_DAC_R2_MONO_R_VOL_MASK: u32 = (0x1u32 << 3u32);
pub const RT5640_DAC_R2_MONO_R_VOL_SFT: u32 = 3u32;
pub const RT5640_M_DAC_L2_MONO_R: u32 = (0x1u32 << 2u32);
pub const RT5640_M_DAC_L2_MONO_R_SFT: u32 = 2u32;
pub const RT5640_DAC_L2_MONO_R_VOL_MASK: u32 = (0x1u32 << 1u32);
pub const RT5640_DAC_L2_MONO_R_VOL_SFT: u32 = 1u32;

/* Digital Mixer Control (0x2c) */
pub const RT5640_M_STO_L_DAC_L: u32 = (0x1u32 << 15u32);
pub const RT5640_M_STO_L_DAC_L_SFT: u32 = 15u32;
pub const RT5640_STO_L_DAC_L_VOL_MASK: u32 = (0x1u32 << 14u32);
pub const RT5640_STO_L_DAC_L_VOL_SFT: u32 = 14u32;
pub const RT5640_M_DAC_L2_DAC_L: u32 = (0x1u32 << 13u32);
pub const RT5640_M_DAC_L2_DAC_L_SFT: u32 = 13u32;
pub const RT5640_DAC_L2_DAC_L_VOL_MASK: u32 = (0x1u32 << 12u32);
pub const RT5640_DAC_L2_DAC_L_VOL_SFT: u32 = 12u32;
pub const RT5640_M_STO_R_DAC_R: u32 = (0x1u32 << 11u32);
pub const RT5640_M_STO_R_DAC_R_SFT: u32 = 11u32;
pub const RT5640_STO_R_DAC_R_VOL_MASK: u32 = (0x1u32 << 10u32);
pub const RT5640_STO_R_DAC_R_VOL_SFT: u32 = 10u32;
pub const RT5640_M_DAC_R2_DAC_R: u32 = (0x1u32 << 9u32);
pub const RT5640_M_DAC_R2_DAC_R_SFT: u32 = 9u32;
pub const RT5640_DAC_R2_DAC_R_VOL_MASK: u32 = (0x1u32 << 8u32);
pub const RT5640_DAC_R2_DAC_R_VOL_SFT: u32 = 8u32;

/* DSP Path Control 1 (0x2d) */
pub const RT5640_RXDP_SRC_MASK: u32 = (0x1u32 << 15u32);
pub const RT5640_RXDP_SRC_SFT: u32 = 15u32;
pub const RT5640_RXDP_SRC_NOR: u32 = (0x0u32 << 15u32);
pub const RT5640_RXDP_SRC_DIV3: u32 = (0x1u32 << 15u32);
pub const RT5640_TXDP_SRC_MASK: u32 = (0x1u32 << 14u32);
pub const RT5640_TXDP_SRC_SFT: u32 = 14u32;
pub const RT5640_TXDP_SRC_NOR: u32 = (0x0u32 << 14u32);
pub const RT5640_TXDP_SRC_DIV3: u32 = (0x1u32 << 14u32);

/* DSP Path Control 2 (0x2e) */
pub const RT5640_DAC_L2_SEL_MASK: u32 = (0x3u32 << 14u32);
pub const RT5640_DAC_L2_SEL_SFT: u32 = 14u32;
pub const RT5640_DAC_L2_SEL_IF2: u32 = (0x0u32 << 14u32);
pub const RT5640_DAC_L2_SEL_IF3: u32 = (0x1u32 << 14u32);
pub const RT5640_DAC_L2_SEL_TXDC: u32 = (0x2u32 << 14u32);
pub const RT5640_DAC_L2_SEL_BASS: u32 = (0x3u32 << 14u32);
pub const RT5640_DAC_R2_SEL_MASK: u32 = (0x3u32 << 12u32);
pub const RT5640_DAC_R2_SEL_SFT: u32 = 12u32;
pub const RT5640_DAC_R2_SEL_IF2: u32 = (0x0u32 << 12u32);
pub const RT5640_DAC_R2_SEL_IF3: u32 = (0x1u32 << 12u32);
pub const RT5640_DAC_R2_SEL_TXDC: u32 = (0x2u32 << 12u32);
pub const RT5640_IF2_ADC_L_SEL_MASK: u32 = (0x1u32 << 11u32);
pub const RT5640_IF2_ADC_L_SEL_SFT: u32 = 11u32;
pub const RT5640_IF2_ADC_L_SEL_TXDP: u32 = (0x0u32 << 11u32);
pub const RT5640_IF2_ADC_L_SEL_PASS: u32 = (0x1u32 << 11u32);
pub const RT5640_IF2_ADC_R_SEL_MASK: u32 = (0x1u32 << 10u32);
pub const RT5640_IF2_ADC_R_SEL_SFT: u32 = 10u32;
pub const RT5640_IF2_ADC_R_SEL_TXDP: u32 = (0x0u32 << 10u32);
pub const RT5640_IF2_ADC_R_SEL_PASS: u32 = (0x1u32 << 10u32);
pub const RT5640_RXDC_SEL_MASK: u32 = (0x3u32 << 8u32);
pub const RT5640_RXDC_SEL_SFT: u32 = 8u32;
pub const RT5640_RXDC_SEL_NOR: u32 = (0x0u32 << 8u32);
pub const RT5640_RXDC_SEL_L2R: u32 = (0x1u32 << 8u32);
pub const RT5640_RXDC_SEL_R2L: u32 = (0x2u32 << 8u32);
pub const RT5640_RXDC_SEL_SWAP: u32 = (0x3u32 << 8u32);
pub const RT5640_RXDP_SEL_MASK: u32 = (0x3u32 << 6u32);
pub const RT5640_RXDP_SEL_SFT: u32 = 6u32;
pub const RT5640_RXDP_SEL_NOR: u32 = (0x0u32 << 6u32);
pub const RT5640_RXDP_SEL_L2R: u32 = (0x1u32 << 6u32);
pub const RT5640_RXDP_SEL_R2L: u32 = (0x2u32 << 6u32);
pub const RT5640_RXDP_SEL_SWAP: u32 = (0x3u32 << 6u32);
pub const RT5640_TXDC_SEL_MASK: u32 = (0x3u32 << 4u32);
pub const RT5640_TXDC_SEL_SFT: u32 = 4u32;
pub const RT5640_TXDC_SEL_NOR: u32 = (0x0u32 << 4u32);
pub const RT5640_TXDC_SEL_L2R: u32 = (0x1u32 << 4u32);
pub const RT5640_TXDC_SEL_R2L: u32 = (0x2u32 << 4u32);
pub const RT5640_TXDC_SEL_SWAP: u32 = (0x3u32 << 4u32);
pub const RT5640_TXDP_SEL_MASK: u32 = (0x3u32 << 2u32);
pub const RT5640_TXDP_SEL_SFT: u32 = 2u32;
pub const RT5640_TXDP_SEL_NOR: u32 = (0x0u32 << 2u32);
pub const RT5640_TXDP_SEL_L2R: u32 = (0x1u32 << 2u32);
pub const RT5640_TXDP_SEL_R2L: u32 = (0x2u32 << 2u32);
pub const RT5640_TRXDP_SEL_SWAP: u32 = (0x3u32 << 2u32);

/* Digital Interface Data Control (0x2f) */
pub const RT5640_IF1_DAC_SEL_MASK: u32 = (0x3u32 << 14u32);
pub const RT5640_IF1_DAC_SEL_SFT: u32 = 14u32;
pub const RT5640_IF1_DAC_SEL_NOR: u32 = (0x0u32 << 14u32);
pub const RT5640_IF1_DAC_SEL_SWAP: u32 = (0x1u32 << 14u32);
pub const RT5640_IF1_DAC_SEL_L2R: u32 = (0x2u32 << 14u32);
pub const RT5640_IF1_DAC_SEL_R2L: u32 = (0x3u32 << 14u32);
pub const RT5640_IF1_ADC_SEL_MASK: u32 = (0x3u32 << 12u32);
pub const RT5640_IF1_ADC_SEL_SFT: u32 = 12u32;
pub const RT5640_IF1_ADC_SEL_NOR: u32 = (0x0u32 << 12u32);
pub const RT5640_IF1_ADC_SEL_SWAP: u32 = (0x1u32 << 12u32);
pub const RT5640_IF1_ADC_SEL_L2R: u32 = (0x2u32 << 12u32);
pub const RT5640_IF1_ADC_SEL_R2L: u32 = (0x3u32 << 12u32);
pub const RT5640_IF2_DAC_SEL_MASK: u32 = (0x3u32 << 10u32);
pub const RT5640_IF2_DAC_SEL_SFT: u32 = 10u32;
pub const RT5640_IF2_DAC_SEL_NOR: u32 = (0x0u32 << 10u32);
pub const RT5640_IF2_DAC_SEL_SWAP: u32 = (0x1u32 << 10u32);
pub const RT5640_IF2_DAC_SEL_L2R: u32 = (0x2u32 << 10u32);
pub const RT5640_IF2_DAC_SEL_R2L: u32 = (0x3u32 << 10u32);
pub const RT5640_IF2_ADC_SEL_MASK: u32 = (0x3u32 << 8u32);
pub const RT5640_IF2_ADC_SEL_SFT: u32 = 8u32;
pub const RT5640_IF2_ADC_SEL_NOR: u32 = (0x0u32 << 8u32);
pub const RT5640_IF2_ADC_SEL_SWAP: u32 = (0x1u32 << 8u32);
pub const RT5640_IF2_ADC_SEL_L2R: u32 = (0x2u32 << 8u32);
pub const RT5640_IF2_ADC_SEL_R2L: u32 = (0x3u32 << 8u32);
pub const RT5640_IF3_DAC_SEL_MASK: u32 = (0x3u32 << 6u32);
pub const RT5640_IF3_DAC_SEL_SFT: u32 = 6u32;
pub const RT5640_IF3_DAC_SEL_NOR: u32 = (0x0u32 << 6u32);
pub const RT5640_IF3_DAC_SEL_SWAP: u32 = (0x1u32 << 6u32);
pub const RT5640_IF3_DAC_SEL_L2R: u32 = (0x2u32 << 6u32);
pub const RT5640_IF3_DAC_SEL_R2L: u32 = (0x3u32 << 6u32);
pub const RT5640_IF3_ADC_SEL_MASK: u32 = (0x3u32 << 4u32);
pub const RT5640_IF3_ADC_SEL_SFT: u32 = 4u32;
pub const RT5640_IF3_ADC_SEL_NOR: u32 = (0x0u32 << 4u32);
pub const RT5640_IF3_ADC_SEL_SWAP: u32 = (0x1u32 << 4u32);
pub const RT5640_IF3_ADC_SEL_L2R: u32 = (0x2u32 << 4u32);
pub const RT5640_IF3_ADC_SEL_R2L: u32 = (0x3u32 << 4u32);

/* REC Left Mixer Control 1 (0x3b) */
pub const RT5640_G_HP_L_RM_L_MASK: u32 = (0x7u32 << 13u32);
pub const RT5640_G_HP_L_RM_L_SFT: u32 = 13u32;
pub const RT5640_G_IN_L_RM_L_MASK: u32 = (0x7u32 << 10u32);
pub const RT5640_G_IN_L_RM_L_SFT: u32 = 10u32;
pub const RT5640_G_BST4_RM_L_MASK: u32 = (0x7u32 << 7u32);
pub const RT5640_G_BST4_RM_L_SFT: u32 = 7u32;
pub const RT5640_G_BST3_RM_L_MASK: u32 = (0x7u32 << 4u32);
pub const RT5640_G_BST3_RM_L_SFT: u32 = 4u32;
pub const RT5640_G_BST2_RM_L_MASK: u32 = (0x7u32 << 1u32);
pub const RT5640_G_BST2_RM_L_SFT: u32 = 1u32;

/* REC Left Mixer Control 2 (0x3c) */
pub const RT5640_G_BST1_RM_L_MASK: u32 = (0x7u32 << 13u32);
pub const RT5640_G_BST1_RM_L_SFT: u32 = 13u32;
pub const RT5640_G_OM_L_RM_L_MASK: u32 = (0x7u32 << 10u32);
pub const RT5640_G_OM_L_RM_L_SFT: u32 = 10u32;
pub const RT5640_M_HP_L_RM_L: u32 = (0x1u32 << 6u32);
pub const RT5640_M_HP_L_RM_L_SFT: u32 = 6u32;
pub const RT5640_M_IN_L_RM_L: u32 = (0x1u32 << 5u32);
pub const RT5640_M_IN_L_RM_L_SFT: u32 = 5u32;
pub const RT5640_M_BST4_RM_L: u32 = (0x1u32 << 4u32);
pub const RT5640_M_BST4_RM_L_SFT: u32 = 4u32;
pub const RT5640_M_BST3_RM_L: u32 = (0x1u32 << 3u32);
pub const RT5640_M_BST3_RM_L_SFT: u32 = 3u32;
pub const RT5640_M_BST2_RM_L: u32 = (0x1u32 << 2u32);
pub const RT5640_M_BST2_RM_L_SFT: u32 = 2u32;
pub const RT5640_M_BST1_RM_L: u32 = (0x1u32 << 1u32);
pub const RT5640_M_BST1_RM_L_SFT: u32 = 1u32;
pub const RT5640_M_OM_L_RM_L: u32 = 0x1u32;
pub const RT5640_M_OM_L_RM_L_SFT: u32 = 0u32;

/* REC Right Mixer Control 1 (0x3d) */
pub const RT5640_G_HP_R_RM_R_MASK: u32 = (0x7u32 << 13u32);
pub const RT5640_G_HP_R_RM_R_SFT: u32 = 13u32;
pub const RT5640_G_IN_R_RM_R_MASK: u32 = (0x7u32 << 10u32);
pub const RT5640_G_IN_R_RM_R_SFT: u32 = 10u32;
pub const RT5640_G_BST4_RM_R_MASK: u32 = (0x7u32 << 7u32);
pub const RT5640_G_BST4_RM_R_SFT: u32 = 7u32;
pub const RT5640_G_BST3_RM_R_MASK: u32 = (0x7u32 << 4u32);
pub const RT5640_G_BST3_RM_R_SFT: u32 = 4u32;
pub const RT5640_G_BST2_RM_R_MASK: u32 = (0x7u32 << 1u32);
pub const RT5640_G_BST2_RM_R_SFT: u32 = 1u32;

/* REC Right Mixer Control 2 (0x3e) */
pub const RT5640_G_BST1_RM_R_MASK: u32 = (0x7u32 << 13u32);
pub const RT5640_G_BST1_RM_R_SFT: u32 = 13u32;
pub const RT5640_G_OM_R_RM_R_MASK: u32 = (0x7u32 << 10u32);
pub const RT5640_G_OM_R_RM_R_SFT: u32 = 10u32;
pub const RT5640_M_HP_R_RM_R: u32 = (0x1u32 << 6u32);
pub const RT5640_M_HP_R_RM_R_SFT: u32 = 6u32;
pub const RT5640_M_IN_R_RM_R: u32 = (0x1u32 << 5u32);
pub const RT5640_M_IN_R_RM_R_SFT: u32 = 5u32;
pub const RT5640_M_BST4_RM_R: u32 = (0x1u32 << 4u32);
pub const RT5640_M_BST4_RM_R_SFT: u32 = 4u32;
pub const RT5640_M_BST3_RM_R: u32 = (0x1u32 << 3u32);
pub const RT5640_M_BST3_RM_R_SFT: u32 = 3u32;
pub const RT5640_M_BST2_RM_R: u32 = (0x1u32 << 2u32);
pub const RT5640_M_BST2_RM_R_SFT: u32 = 2u32;
pub const RT5640_M_BST1_RM_R: u32 = (0x1u32 << 1u32);
pub const RT5640_M_BST1_RM_R_SFT: u32 = 1u32;
pub const RT5640_M_OM_R_RM_R: u32 = 0x1u32;
pub const RT5640_M_OM_R_RM_R_SFT: u32 = 0u32;

/* HPMIX Control (0x45) */
pub const RT5640_M_DAC2_HM: u32 = (0x1u32 << 15u32);
pub const RT5640_M_DAC2_HM_SFT: u32 = 15u32;
pub const RT5640_M_DAC1_HM: u32 = (0x1u32 << 14u32);
pub const RT5640_M_DAC1_HM_SFT: u32 = 14u32;
pub const RT5640_M_HPVOL_HM: u32 = (0x1u32 << 13u32);
pub const RT5640_M_HPVOL_HM_SFT: u32 = 13u32;
pub const RT5640_G_HPOMIX_MASK: u32 = (0x1u32 << 12u32);
pub const RT5640_G_HPOMIX_SFT: u32 = 12u32;

/* SPK Left Mixer Control (0x46) */
pub const RT5640_G_RM_L_SM_L_MASK: u32 = (0x3u32 << 14u32);
pub const RT5640_G_RM_L_SM_L_SFT: u32 = 14u32;
pub const RT5640_G_IN_L_SM_L_MASK: u32 = (0x3u32 << 12u32);
pub const RT5640_G_IN_L_SM_L_SFT: u32 = 12u32;
pub const RT5640_G_DAC_L1_SM_L_MASK: u32 = (0x3u32 << 10u32);
pub const RT5640_G_DAC_L1_SM_L_SFT: u32 = 10u32;
pub const RT5640_G_DAC_L2_SM_L_MASK: u32 = (0x3u32 << 8u32);
pub const RT5640_G_DAC_L2_SM_L_SFT: u32 = 8u32;
pub const RT5640_G_OM_L_SM_L_MASK: u32 = (0x3u32 << 6u32);
pub const RT5640_G_OM_L_SM_L_SFT: u32 = 6u32;
pub const RT5640_M_RM_L_SM_L: u32 = (0x1u32 << 5u32);
pub const RT5640_M_RM_L_SM_L_SFT: u32 = 5u32;
pub const RT5640_M_IN_L_SM_L: u32 = (0x1u32 << 4u32);
pub const RT5640_M_IN_L_SM_L_SFT: u32 = 4u32;
pub const RT5640_M_DAC_L1_SM_L: u32 = (0x1u32 << 3u32);
pub const RT5640_M_DAC_L1_SM_L_SFT: u32 = 3u32;
pub const RT5640_M_DAC_L2_SM_L: u32 = (0x1u32 << 2u32);
pub const RT5640_M_DAC_L2_SM_L_SFT: u32 = 2u32;
pub const RT5640_M_OM_L_SM_L: u32 = (0x1u32 << 1u32);
pub const RT5640_M_OM_L_SM_L_SFT: u32 = 1u32;

/* SPK Right Mixer Control (0x47) */
pub const RT5640_G_RM_R_SM_R_MASK: u32 = (0x3u32 << 14u32);
pub const RT5640_G_RM_R_SM_R_SFT: u32 = 14u32;
pub const RT5640_G_IN_R_SM_R_MASK: u32 = (0x3u32 << 12u32);
pub const RT5640_G_IN_R_SM_R_SFT: u32 = 12u32;
pub const RT5640_G_DAC_R1_SM_R_MASK: u32 = (0x3u32 << 10u32);
pub const RT5640_G_DAC_R1_SM_R_SFT: u32 = 10u32;
pub const RT5640_G_DAC_R2_SM_R_MASK: u32 = (0x3u32 << 8u32);
pub const RT5640_G_DAC_R2_SM_R_SFT: u32 = 8u32;
pub const RT5640_G_OM_R_SM_R_MASK: u32 = (0x3u32 << 6u32);
pub const RT5640_G_OM_R_SM_R_SFT: u32 = 6u32;
pub const RT5640_M_RM_R_SM_R: u32 = (0x1u32 << 5u32);
pub const RT5640_M_RM_R_SM_R_SFT: u32 = 5u32;
pub const RT5640_M_IN_R_SM_R: u32 = (0x1u32 << 4u32);
pub const RT5640_M_IN_R_SM_R_SFT: u32 = 4u32;
pub const RT5640_M_DAC_R1_SM_R: u32 = (0x1u32 << 3u32);
pub const RT5640_M_DAC_R1_SM_R_SFT: u32 = 3u32;
pub const RT5640_M_DAC_R2_SM_R: u32 = (0x1u32 << 2u32);
pub const RT5640_M_DAC_R2_SM_R_SFT: u32 = 2u32;
pub const RT5640_M_OM_R_SM_R: u32 = (0x1u32 << 1u32);
pub const RT5640_M_OM_R_SM_R_SFT: u32 = 1u32;

/* SPOLMIX Control (0x48) */
pub const RT5640_M_DAC_R1_SPM_L: u32 = (0x1u32 << 15u32);
pub const RT5640_M_DAC_R1_SPM_L_SFT: u32 = 15u32;
pub const RT5640_M_DAC_L1_SPM_L: u32 = (0x1u32 << 14u32);
pub const RT5640_M_DAC_L1_SPM_L_SFT: u32 = 14u32;
pub const RT5640_M_SV_R_SPM_L: u32 = (0x1u32 << 13u32);
pub const RT5640_M_SV_R_SPM_L_SFT: u32 = 13u32;
pub const RT5640_M_SV_L_SPM_L: u32 = (0x1u32 << 12u32);
pub const RT5640_M_SV_L_SPM_L_SFT: u32 = 12u32;
pub const RT5640_M_BST1_SPM_L: u32 = (0x1u32 << 11u32);
pub const RT5640_M_BST1_SPM_L_SFT: u32 = 11u32;

/* SPORMIX Control (0x49) */
pub const RT5640_M_DAC_R1_SPM_R: u32 = (0x1u32 << 13u32);
pub const RT5640_M_DAC_R1_SPM_R_SFT: u32 = 13u32;
pub const RT5640_M_SV_R_SPM_R: u32 = (0x1u32 << 12u32);
pub const RT5640_M_SV_R_SPM_R_SFT: u32 = 12u32;
pub const RT5640_M_BST1_SPM_R: u32 = (0x1u32 << 11u32);
pub const RT5640_M_BST1_SPM_R_SFT: u32 = 11u32;

/* SPOLMIX / SPORMIX Ratio Control (0x4a) */
pub const RT5640_SPO_CLSD_RATIO_MASK: u32 = 0x7u32;
pub const RT5640_SPO_CLSD_RATIO_SFT: u32 = 0u32;

/* Mono Output Mixer Control (0x4c) */
pub const RT5640_M_DAC_R2_MM: u32 = (0x1u32 << 15u32);
pub const RT5640_M_DAC_R2_MM_SFT: u32 = 15u32;
pub const RT5640_M_DAC_L2_MM: u32 = (0x1u32 << 14u32);
pub const RT5640_M_DAC_L2_MM_SFT: u32 = 14u32;
pub const RT5640_M_OV_R_MM: u32 = (0x1u32 << 13u32);
pub const RT5640_M_OV_R_MM_SFT: u32 = 13u32;
pub const RT5640_M_OV_L_MM: u32 = (0x1u32 << 12u32);
pub const RT5640_M_OV_L_MM_SFT: u32 = 12u32;
pub const RT5640_M_BST1_MM: u32 = (0x1u32 << 11u32);
pub const RT5640_M_BST1_MM_SFT: u32 = 11u32;
pub const RT5640_G_MONOMIX_MASK: u32 = (0x1u32 << 10u32);
pub const RT5640_G_MONOMIX_SFT: u32 = 10u32;

/* Output Left Mixer Control 1 (0x4d) */
pub const RT5640_G_BST3_OM_L_MASK: u32 = (0x7u32 << 13u32);
pub const RT5640_G_BST3_OM_L_SFT: u32 = 13u32;
pub const RT5640_G_BST2_OM_L_MASK: u32 = (0x7u32 << 10u32);
pub const RT5640_G_BST2_OM_L_SFT: u32 = 10u32;
pub const RT5640_G_BST1_OM_L_MASK: u32 = (0x7u32 << 7u32);
pub const RT5640_G_BST1_OM_L_SFT: u32 = 7u32;
pub const RT5640_G_IN_L_OM_L_MASK: u32 = (0x7u32 << 4u32);
pub const RT5640_G_IN_L_OM_L_SFT: u32 = 4u32;
pub const RT5640_G_RM_L_OM_L_MASK: u32 = (0x7u32 << 1u32);
pub const RT5640_G_RM_L_OM_L_SFT: u32 = 1u32;

/* Output Left Mixer Control 2 (0x4e) */
pub const RT5640_G_DAC_R2_OM_L_MASK: u32 = (0x7u32 << 13u32);
pub const RT5640_G_DAC_R2_OM_L_SFT: u32 = 13u32;
pub const RT5640_G_DAC_L2_OM_L_MASK: u32 = (0x7u32 << 10u32);
pub const RT5640_G_DAC_L2_OM_L_SFT: u32 = 10u32;
pub const RT5640_G_DAC_L1_OM_L_MASK: u32 = (0x7u32 << 7u32);
pub const RT5640_G_DAC_L1_OM_L_SFT: u32 = 7u32;

/* Output Left Mixer Control 3 (0x4f) */
pub const RT5640_M_SM_L_OM_L: u32 = (0x1u32 << 8u32);
pub const RT5640_M_SM_L_OM_L_SFT: u32 = 8u32;
pub const RT5640_M_BST3_OM_L: u32 = (0x1u32 << 7u32);
pub const RT5640_M_BST3_OM_L_SFT: u32 = 7u32;
pub const RT5640_M_BST2_OM_L: u32 = (0x1u32 << 6u32);
pub const RT5640_M_BST2_OM_L_SFT: u32 = 6u32;
pub const RT5640_M_BST1_OM_L: u32 = (0x1u32 << 5u32);
pub const RT5640_M_BST1_OM_L_SFT: u32 = 5u32;
pub const RT5640_M_IN_L_OM_L: u32 = (0x1u32 << 4u32);
pub const RT5640_M_IN_L_OM_L_SFT: u32 = 4u32;
pub const RT5640_M_RM_L_OM_L: u32 = (0x1u32 << 3u32);
pub const RT5640_M_RM_L_OM_L_SFT: u32 = 3u32;
pub const RT5640_M_DAC_R2_OM_L: u32 = (0x1u32 << 2u32);
pub const RT5640_M_DAC_R2_OM_L_SFT: u32 = 2u32;
pub const RT5640_M_DAC_L2_OM_L: u32 = (0x1u32 << 1u32);
pub const RT5640_M_DAC_L2_OM_L_SFT: u32 = 1u32;
pub const RT5640_M_DAC_L1_OM_L: u32 = 0x1u32;
pub const RT5640_M_DAC_L1_OM_L_SFT: u32 = 0u32;

/* Output Right Mixer Control 1 (0x50) */
pub const RT5640_G_BST4_OM_R_MASK: u32 = (0x7u32 << 13u32);
pub const RT5640_G_BST4_OM_R_SFT: u32 = 13u32;
pub const RT5640_G_BST2_OM_R_MASK: u32 = (0x7u32 << 10u32);
pub const RT5640_G_BST2_OM_R_SFT: u32 = 10u32;
pub const RT5640_G_BST1_OM_R_MASK: u32 = (0x7u32 << 7u32);
pub const RT5640_G_BST1_OM_R_SFT: u32 = 7u32;
pub const RT5640_G_IN_R_OM_R_MASK: u32 = (0x7u32 << 4u32);
pub const RT5640_G_IN_R_OM_R_SFT: u32 = 4u32;
pub const RT5640_G_RM_R_OM_R_MASK: u32 = (0x7u32 << 1u32);
pub const RT5640_G_RM_R_OM_R_SFT: u32 = 1u32;

/* Output Right Mixer Control 2 (0x51) */
pub const RT5640_G_DAC_L2_OM_R_MASK: u32 = (0x7u32 << 13u32);
pub const RT5640_G_DAC_L2_OM_R_SFT: u32 = 13u32;
pub const RT5640_G_DAC_R2_OM_R_MASK: u32 = (0x7u32 << 10u32);
pub const RT5640_G_DAC_R2_OM_R_SFT: u32 = 10u32;
pub const RT5640_G_DAC_R1_OM_R_MASK: u32 = (0x7u32 << 7u32);
pub const RT5640_G_DAC_R1_OM_R_SFT: u32 = 7u32;

/* Output Right Mixer Control 3 (0x52) */
pub const RT5640_M_SM_L_OM_R: u32 = (0x1u32 << 8u32);
pub const RT5640_M_SM_L_OM_R_SFT: u32 = 8u32;
pub const RT5640_M_BST4_OM_R: u32 = (0x1u32 << 7u32);
pub const RT5640_M_BST4_OM_R_SFT: u32 = 7u32;
pub const RT5640_M_BST2_OM_R: u32 = (0x1u32 << 6u32);
pub const RT5640_M_BST2_OM_R_SFT: u32 = 6u32;
pub const RT5640_M_BST1_OM_R: u32 = (0x1u32 << 5u32);
pub const RT5640_M_BST1_OM_R_SFT: u32 = 5u32;
pub const RT5640_M_IN_R_OM_R: u32 = (0x1u32 << 4u32);
pub const RT5640_M_IN_R_OM_R_SFT: u32 = 4u32;
pub const RT5640_M_RM_R_OM_R: u32 = (0x1u32 << 3u32);
pub const RT5640_M_RM_R_OM_R_SFT: u32 = 3u32;
pub const RT5640_M_DAC_L2_OM_R: u32 = (0x1u32 << 2u32);
pub const RT5640_M_DAC_L2_OM_R_SFT: u32 = 2u32;
pub const RT5640_M_DAC_R2_OM_R: u32 = (0x1u32 << 1u32);
pub const RT5640_M_DAC_R2_OM_R_SFT: u32 = 1u32;
pub const RT5640_M_DAC_R1_OM_R: u32 = 0x1u32;
pub const RT5640_M_DAC_R1_OM_R_SFT: u32 = 0u32;

/* LOUT Mixer Control (0x53) */
pub const RT5640_M_DAC_L1_LM: u32 = (0x1u32 << 15u32);
pub const RT5640_M_DAC_L1_LM_SFT: u32 = 15u32;
pub const RT5640_M_DAC_R1_LM: u32 = (0x1u32 << 14u32);
pub const RT5640_M_DAC_R1_LM_SFT: u32 = 14u32;
pub const RT5640_M_OV_L_LM: u32 = (0x1u32 << 13u32);
pub const RT5640_M_OV_L_LM_SFT: u32 = 13u32;
pub const RT5640_M_OV_R_LM: u32 = (0x1u32 << 12u32);
pub const RT5640_M_OV_R_LM_SFT: u32 = 12u32;
pub const RT5640_G_LOUTMIX_MASK: u32 = (0x1u32 << 11u32);
pub const RT5640_G_LOUTMIX_SFT: u32 = 11u32;

/* Power Management for Digital 1 (0x61) */
pub const RT5640_PWR_I2S1: u32 = (0x1u32 << 15u32);
pub const RT5640_PWR_I2S1_BIT: u32 = 15u32;
pub const RT5640_PWR_I2S2: u32 = (0x1u32 << 14u32);
pub const RT5640_PWR_I2S2_BIT: u32 = 14u32;
pub const RT5640_PWR_DAC_L1: u32 = (0x1u32 << 12u32);
pub const RT5640_PWR_DAC_L1_BIT: u32 = 12u32;
pub const RT5640_PWR_DAC_R1: u32 = (0x1u32 << 11u32);
pub const RT5640_PWR_DAC_R1_BIT: u32 = 11u32;
pub const RT5640_PWR_DAC_L2: u32 = (0x1u32 << 7u32);
pub const RT5640_PWR_DAC_L2_BIT: u32 = 7u32;
pub const RT5640_PWR_DAC_R2: u32 = (0x1u32 << 6u32);
pub const RT5640_PWR_DAC_R2_BIT: u32 = 6u32;
pub const RT5640_PWR_ADC_L: u32 = (0x1u32 << 2u32);
pub const RT5640_PWR_ADC_L_BIT: u32 = 2u32;
pub const RT5640_PWR_ADC_R: u32 = (0x1u32 << 1u32);
pub const RT5640_PWR_ADC_R_BIT: u32 = 1u32;
pub const RT5640_PWR_CLS_D: u32 = 0x1u32;
pub const RT5640_PWR_CLS_D_BIT: u32 = 0u32;

/* Power Management for Digital 2 (0x62) */
pub const RT5640_PWR_ADC_SF: u32 = (0x1u32 << 15u32);
pub const RT5640_PWR_ADC_SF_BIT: u32 = 15u32;
pub const RT5640_PWR_ADC_MF_L: u32 = (0x1u32 << 14u32);
pub const RT5640_PWR_ADC_MF_L_BIT: u32 = 14u32;
pub const RT5640_PWR_ADC_MF_R: u32 = (0x1u32 << 13u32);
pub const RT5640_PWR_ADC_MF_R_BIT: u32 = 13u32;
pub const RT5640_PWR_I2S_DSP: u32 = (0x1u32 << 12u32);
pub const RT5640_PWR_I2S_DSP_BIT: u32 = 12u32;

/* Power Management for Analog 1 (0x63) */
pub const RT5640_PWR_VREF1: u32 = (0x1u32 << 15u32);
pub const RT5640_PWR_VREF1_BIT: u32 = 15u32;
pub const RT5640_PWR_FV1: u32 = (0x1u32 << 14u32);
pub const RT5640_PWR_FV1_BIT: u32 = 14u32;
pub const RT5640_PWR_MB: u32 = (0x1u32 << 13u32);
pub const RT5640_PWR_MB_BIT: u32 = 13u32;
pub const RT5640_PWR_LM: u32 = (0x1u32 << 12u32);
pub const RT5640_PWR_LM_BIT: u32 = 12u32;
pub const RT5640_PWR_BG: u32 = (0x1u32 << 11u32);
pub const RT5640_PWR_BG_BIT: u32 = 11u32;
pub const RT5640_PWR_MM: u32 = (0x1u32 << 10u32);
pub const RT5640_PWR_MM_BIT: u32 = 10u32;
pub const RT5640_PWR_MA: u32 = (0x1u32 << 8u32);
pub const RT5640_PWR_MA_BIT: u32 = 8u32;
pub const RT5640_PWR_HP_L: u32 = (0x1u32 << 7u32);
pub const RT5640_PWR_HP_L_BIT: u32 = 7u32;
pub const RT5640_PWR_HP_R: u32 = (0x1u32 << 6u32);
pub const RT5640_PWR_HP_R_BIT: u32 = 6u32;
pub const RT5640_PWR_HA: u32 = (0x1u32 << 5u32);
pub const RT5640_PWR_HA_BIT: u32 = 5u32;
pub const RT5640_PWR_VREF2: u32 = (0x1u32 << 4u32);
pub const RT5640_PWR_VREF2_BIT: u32 = 4u32;
pub const RT5640_PWR_FV2: u32 = (0x1u32 << 3u32);
pub const RT5640_PWR_FV2_BIT: u32 = 3u32;
pub const RT5640_PWR_LDO2: u32 = (0x1u32 << 2u32);
pub const RT5640_PWR_LDO2_BIT: u32 = 2u32;

/* Power Management for Analog 2 (0x64) */
pub const RT5640_PWR_BST1: u32 = (0x1u32 << 15u32);
pub const RT5640_PWR_BST1_BIT: u32 = 15u32;
pub const RT5640_PWR_BST2: u32 = (0x1u32 << 14u32);
pub const RT5640_PWR_BST2_BIT: u32 = 14u32;
pub const RT5640_PWR_BST3: u32 = (0x1u32 << 13u32);
pub const RT5640_PWR_BST3_BIT: u32 = 13u32;
pub const RT5640_PWR_BST4: u32 = (0x1u32 << 12u32);
pub const RT5640_PWR_BST4_BIT: u32 = 12u32;
pub const RT5640_PWR_MB1: u32 = (0x1u32 << 11u32);
pub const RT5640_PWR_MB1_BIT: u32 = 11u32;
pub const RT5640_PWR_PLL: u32 = (0x1u32 << 9u32);
pub const RT5640_PWR_PLL_BIT: u32 = 9u32;

/* Power Management for Mixer (0x65) */
pub const RT5640_PWR_OM_L: u32 = (0x1u32 << 15u32);
pub const RT5640_PWR_OM_L_BIT: u32 = 15u32;
pub const RT5640_PWR_OM_R: u32 = (0x1u32 << 14u32);
pub const RT5640_PWR_OM_R_BIT: u32 = 14u32;
pub const RT5640_PWR_SM_L: u32 = (0x1u32 << 13u32);
pub const RT5640_PWR_SM_L_BIT: u32 = 13u32;
pub const RT5640_PWR_SM_R: u32 = (0x1u32 << 12u32);
pub const RT5640_PWR_SM_R_BIT: u32 = 12u32;
pub const RT5640_PWR_RM_L: u32 = (0x1u32 << 11u32);
pub const RT5640_PWR_RM_L_BIT: u32 = 11u32;
pub const RT5640_PWR_RM_R: u32 = (0x1u32 << 10u32);
pub const RT5640_PWR_RM_R_BIT: u32 = 10u32;

/* Power Management for Volume (0x66) */
pub const RT5640_PWR_SV_L: u32 = (0x1u32 << 15u32);
pub const RT5640_PWR_SV_L_BIT: u32 = 15u32;
pub const RT5640_PWR_SV_R: u32 = (0x1u32 << 14u32);
pub const RT5640_PWR_SV_R_BIT: u32 = 14u32;
pub const RT5640_PWR_OV_L: u32 = (0x1u32 << 13u32);
pub const RT5640_PWR_OV_L_BIT: u32 = 13u32;
pub const RT5640_PWR_OV_R: u32 = (0x1u32 << 12u32);
pub const RT5640_PWR_OV_R_BIT: u32 = 12u32;
pub const RT5640_PWR_HV_L: u32 = (0x1u32 << 11u32);
pub const RT5640_PWR_HV_L_BIT: u32 = 11u32;
pub const RT5640_PWR_HV_R: u32 = (0x1u32 << 10u32);
pub const RT5640_PWR_HV_R_BIT: u32 = 10u32;
pub const RT5640_PWR_IN_L: u32 = (0x1u32 << 9u32);
pub const RT5640_PWR_IN_L_BIT: u32 = 9u32;
pub const RT5640_PWR_IN_R: u32 = (0x1u32 << 8u32);
pub const RT5640_PWR_IN_R_BIT: u32 = 8u32;

/* I2S1/2/3 Audio Serial Data Port Control (0x70 0x71 0x72) */
pub const RT5640_I2S_MS_MASK: u32 = (0x1u32 << 15u32);
pub const RT5640_I2S_MS_SFT: u32 = 15u32;
pub const RT5640_I2S_MS_M: u32 = (0x0u32 << 15u32);
pub const RT5640_I2S_MS_S: u32 = (0x1u32 << 15u32);
pub const RT5640_I2S_IF_MASK: u32 = (0x7u32 << 12u32);
pub const RT5640_I2S_IF_SFT: u32 = 12u32;
pub const RT5640_I2S_O_CP_MASK: u32 = (0x3u32 << 10u32);
pub const RT5640_I2S_O_CP_SFT: u32 = 10u32;
pub const RT5640_I2S_O_CP_OFF: u32 = (0x0u32 << 10u32);
pub const RT5640_I2S_O_CP_U_LAW: u32 = (0x1u32 << 10u32);
pub const RT5640_I2S_O_CP_A_LAW: u32 = (0x2u32 << 10u32);
pub const RT5640_I2S_I_CP_MASK: u32 = (0x3u32 << 8u32);
pub const RT5640_I2S_I_CP_SFT: u32 = 8u32;
pub const RT5640_I2S_I_CP_OFF: u32 = (0x0u32 << 8u32);
pub const RT5640_I2S_I_CP_U_LAW: u32 = (0x1u32 << 8u32);
pub const RT5640_I2S_I_CP_A_LAW: u32 = (0x2u32 << 8u32);
pub const RT5640_I2S_BP_MASK: u32 = (0x1u32 << 7u32);
pub const RT5640_I2S_BP_SFT: u32 = 7u32;
pub const RT5640_I2S_BP_NOR: u32 = (0x0u32 << 7u32);
pub const RT5640_I2S_BP_INV: u32 = (0x1u32 << 7u32);
pub const RT5640_I2S_DL_MASK: u32 = (0x3u32 << 2u32);
pub const RT5640_I2S_DL_SFT: u32 = 2u32;
pub const RT5640_I2S_DL_16: u32 = (0x0u32 << 2u32);
pub const RT5640_I2S_DL_20: u32 = (0x1u32 << 2u32);
pub const RT5640_I2S_DL_24: u32 = (0x2u32 << 2u32);
pub const RT5640_I2S_DL_8: u32 = (0x3u32 << 2u32);
pub const RT5640_I2S_DF_MASK: u32 = 0x3u32;
pub const RT5640_I2S_DF_SFT: u32 = 0u32;
pub const RT5640_I2S_DF_I2S: u32 = 0x0u32;
pub const RT5640_I2S_DF_LEFT: u32 = 0x1u32;
pub const RT5640_I2S_DF_PCM_A: u32 = 0x2u32;
pub const RT5640_I2S_DF_PCM_B: u32 = 0x3u32;

/* I2S2 Audio Serial Data Port Control (0x71) */
pub const RT5640_I2S2_SDI_MASK: u32 = (0x1u32 << 6u32);
pub const RT5640_I2S2_SDI_SFT: u32 = 6u32;
pub const RT5640_I2S2_SDI_I2S1: u32 = (0x0u32 << 6u32);
pub const RT5640_I2S2_SDI_I2S2: u32 = (0x1u32 << 6u32);

/* ADC/DAC Clock Control 1 (0x73) */
pub const RT5640_I2S_BCLK_MS1_MASK: u32 = (0x1u32 << 15u32);
pub const RT5640_I2S_BCLK_MS1_SFT: u32 = 15u32;
pub const RT5640_I2S_BCLK_MS1_32: u32 = (0x0u32 << 15u32);
pub const RT5640_I2S_BCLK_MS1_64: u32 = (0x1u32 << 15u32);
pub const RT5640_I2S_PD1_MASK: u32 = (0x7u32 << 12u32);
pub const RT5640_I2S_PD1_SFT: u32 = 12u32;
pub const RT5640_I2S_PD1_1: u32 = (0x0u32 << 12u32);
pub const RT5640_I2S_PD1_2: u32 = (0x1u32 << 12u32);
pub const RT5640_I2S_PD1_3: u32 = (0x2u32 << 12u32);
pub const RT5640_I2S_PD1_4: u32 = (0x3u32 << 12u32);
pub const RT5640_I2S_PD1_6: u32 = (0x4u32 << 12u32);
pub const RT5640_I2S_PD1_8: u32 = (0x5u32 << 12u32);
pub const RT5640_I2S_PD1_12: u32 = (0x6u32 << 12u32);
pub const RT5640_I2S_PD1_16: u32 = (0x7u32 << 12u32);
pub const RT5640_I2S_BCLK_MS2_MASK: u32 = (0x1u32 << 11u32);
pub const RT5640_I2S_BCLK_MS2_SFT: u32 = 11u32;
pub const RT5640_I2S_BCLK_MS2_32: u32 = (0x0u32 << 11u32);
pub const RT5640_I2S_BCLK_MS2_64: u32 = (0x1u32 << 11u32);
pub const RT5640_I2S_PD2_MASK: u32 = (0x7u32 << 8u32);
pub const RT5640_I2S_PD2_SFT: u32 = 8u32;
pub const RT5640_I2S_PD2_1: u32 = (0x0u32 << 8u32);
pub const RT5640_I2S_PD2_2: u32 = (0x1u32 << 8u32);
pub const RT5640_I2S_PD2_3: u32 = (0x2u32 << 8u32);
pub const RT5640_I2S_PD2_4: u32 = (0x3u32 << 8u32);
pub const RT5640_I2S_PD2_6: u32 = (0x4u32 << 8u32);
pub const RT5640_I2S_PD2_8: u32 = (0x5u32 << 8u32);
pub const RT5640_I2S_PD2_12: u32 = (0x6u32 << 8u32);
pub const RT5640_I2S_PD2_16: u32 = (0x7u32 << 8u32);
pub const RT5640_I2S_BCLK_MS3_MASK: u32 = (0x1u32 << 7u32);
pub const RT5640_I2S_BCLK_MS3_SFT: u32 = 7u32;
pub const RT5640_I2S_BCLK_MS3_32: u32 = (0x0u32 << 7u32);
pub const RT5640_I2S_BCLK_MS3_64: u32 = (0x1u32 << 7u32);
pub const RT5640_I2S_PD3_MASK: u32 = (0x7u32 << 4u32);
pub const RT5640_I2S_PD3_SFT: u32 = 4u32;
pub const RT5640_I2S_PD3_1: u32 = (0x0u32 << 4u32);
pub const RT5640_I2S_PD3_2: u32 = (0x1u32 << 4u32);
pub const RT5640_I2S_PD3_3: u32 = (0x2u32 << 4u32);
pub const RT5640_I2S_PD3_4: u32 = (0x3u32 << 4u32);
pub const RT5640_I2S_PD3_6: u32 = (0x4u32 << 4u32);
pub const RT5640_I2S_PD3_8: u32 = (0x5u32 << 4u32);
pub const RT5640_I2S_PD3_12: u32 = (0x6u32 << 4u32);
pub const RT5640_I2S_PD3_16: u32 = (0x7u32 << 4u32);
pub const RT5640_DAC_OSR_MASK: u32 = (0x3u32 << 2u32);
pub const RT5640_DAC_OSR_SFT: u32 = 2u32;
pub const RT5640_DAC_OSR_128: u32 = (0x0u32 << 2u32);
pub const RT5640_DAC_OSR_64: u32 = (0x1u32 << 2u32);
pub const RT5640_DAC_OSR_32: u32 = (0x2u32 << 2u32);
pub const RT5640_DAC_OSR_16: u32 = (0x3u32 << 2u32);
pub const RT5640_ADC_OSR_MASK: u32 = 0x3u32;
pub const RT5640_ADC_OSR_SFT: u32 = 0u32;
pub const RT5640_ADC_OSR_128: u32 = 0x0u32;
pub const RT5640_ADC_OSR_64: u32 = 0x1u32;
pub const RT5640_ADC_OSR_32: u32 = 0x2u32;
pub const RT5640_ADC_OSR_16: u32 = 0x3u32;

/* ADC/DAC Clock Control 2 (0x74) */
pub const RT5640_DAC_L_OSR_MASK: u32 = (0x3u32 << 14u32);
pub const RT5640_DAC_L_OSR_SFT: u32 = 14u32;
pub const RT5640_DAC_L_OSR_128: u32 = (0x0u32 << 14u32);
pub const RT5640_DAC_L_OSR_64: u32 = (0x1u32 << 14u32);
pub const RT5640_DAC_L_OSR_32: u32 = (0x2u32 << 14u32);
pub const RT5640_DAC_L_OSR_16: u32 = (0x3u32 << 14u32);
pub const RT5640_ADC_R_OSR_MASK: u32 = (0x3u32 << 12u32);
pub const RT5640_ADC_R_OSR_SFT: u32 = 12u32;
pub const RT5640_ADC_R_OSR_128: u32 = (0x0u32 << 12u32);
pub const RT5640_ADC_R_OSR_64: u32 = (0x1u32 << 12u32);
pub const RT5640_ADC_R_OSR_32: u32 = (0x2u32 << 12u32);
pub const RT5640_ADC_R_OSR_16: u32 = (0x3u32 << 12u32);
pub const RT5640_DAHPF_EN: u32 = (0x1u32 << 11u32);
pub const RT5640_DAHPF_EN_SFT: u32 = 11u32;
pub const RT5640_ADHPF_EN: u32 = (0x1u32 << 10u32);
pub const RT5640_ADHPF_EN_SFT: u32 = 10u32;

/* Digital Microphone Control (0x75) */
pub const RT5640_DMIC_1_EN_MASK: u32 = (0x1u32 << 15u32);
pub const RT5640_DMIC_1_EN_SFT: u32 = 15u32;
pub const RT5640_DMIC_1_DIS: u32 = (0x0u32 << 15u32);
pub const RT5640_DMIC_1_EN: u32 = (0x1u32 << 15u32);
pub const RT5640_DMIC_2_EN_MASK: u32 = (0x1u32 << 14u32);
pub const RT5640_DMIC_2_EN_SFT: u32 = 14u32;
pub const RT5640_DMIC_2_DIS: u32 = (0x0u32 << 14u32);
pub const RT5640_DMIC_2_EN: u32 = (0x1u32 << 14u32);
pub const RT5640_DMIC_1L_LH_MASK: u32 = (0x1u32 << 13u32);
pub const RT5640_DMIC_1L_LH_SFT: u32 = 13u32;
pub const RT5640_DMIC_1L_LH_FALLING: u32 = (0x0u32 << 13u32);
pub const RT5640_DMIC_1L_LH_RISING: u32 = (0x1u32 << 13u32);
pub const RT5640_DMIC_1R_LH_MASK: u32 = (0x1u32 << 12u32);
pub const RT5640_DMIC_1R_LH_SFT: u32 = 12u32;
pub const RT5640_DMIC_1R_LH_FALLING: u32 = (0x0u32 << 12u32);
pub const RT5640_DMIC_1R_LH_RISING: u32 = (0x1u32 << 12u32);
pub const RT5640_DMIC_1_DP_MASK: u32 = (0x1u32 << 11u32);
pub const RT5640_DMIC_1_DP_SFT: u32 = 11u32;
pub const RT5640_DMIC_1_DP_GPIO3: u32 = (0x0u32 << 11u32);
pub const RT5640_DMIC_1_DP_IN1P: u32 = (0x1u32 << 11u32);
pub const RT5640_DMIC_2_DP_MASK: u32 = (0x1u32 << 10u32);
pub const RT5640_DMIC_2_DP_SFT: u32 = 10u32;
pub const RT5640_DMIC_2_DP_GPIO4: u32 = (0x0u32 << 10u32);
pub const RT5640_DMIC_2_DP_IN1N: u32 = (0x1u32 << 10u32);
pub const RT5640_DMIC_2L_LH_MASK: u32 = (0x1u32 << 9u32);
pub const RT5640_DMIC_2L_LH_SFT: u32 = 9u32;
pub const RT5640_DMIC_2L_LH_FALLING: u32 = (0x0u32 << 9u32);
pub const RT5640_DMIC_2L_LH_RISING: u32 = (0x1u32 << 9u32);
pub const RT5640_DMIC_2R_LH_MASK: u32 = (0x1u32 << 8u32);
pub const RT5640_DMIC_2R_LH_SFT: u32 = 8u32;
pub const RT5640_DMIC_2R_LH_FALLING: u32 = (0x0u32 << 8u32);
pub const RT5640_DMIC_2R_LH_RISING: u32 = (0x1u32 << 8u32);
pub const RT5640_DMIC_CLK_MASK: u32 = (0x7u32 << 5u32);
pub const RT5640_DMIC_CLK_SFT: u32 = 5u32;

/* Global Clock Control (0x80) */
pub const RT5640_SCLK_SRC_MASK: u32 = (0x3u32 << 14u32);
pub const RT5640_SCLK_SRC_SFT: u32 = 14u32;
pub const RT5640_SCLK_SRC_MCLK: u32 = (0x0u32 << 14u32);
pub const RT5640_SCLK_SRC_PLL1: u32 = (0x1u32 << 14u32);
pub const RT5640_SCLK_SRC_RCCLK: u32 = (0x2u32 << 14u32);
pub const RT5640_PLL1_SRC_MASK: u32 = (0x3u32 << 12u32);
pub const RT5640_PLL1_SRC_SFT: u32 = 12u32;
pub const RT5640_PLL1_SRC_MCLK: u32 = (0x0u32 << 12u32);
pub const RT5640_PLL1_SRC_BCLK1: u32 = (0x1u32 << 12u32);
pub const RT5640_PLL1_SRC_BCLK2: u32 = (0x2u32 << 12u32);
pub const RT5640_PLL1_SRC_BCLK3: u32 = (0x3u32 << 12u32);
pub const RT5640_PLL1_PD_MASK: u32 = (0x1u32 << 3u32);
pub const RT5640_PLL1_PD_SFT: u32 = 3u32;
pub const RT5640_PLL1_PD_1: u32 = (0x0u32 << 3u32);
pub const RT5640_PLL1_PD_2: u32 = (0x1u32 << 3u32);

pub const RT5640_PLL_INP_MAX: u32 = 40000000u32;
pub const RT5640_PLL_INP_MIN: u32 = 256000u32;
/* PLL M/N/K Code Control 1 (0x81) */
pub const RT5640_PLL_N_MAX: u32 = 0x1ffu32;
pub const RT5640_PLL_N_MASK: u32 = (RT5640_PLL_N_MAX << 7u32);
pub const RT5640_PLL_N_SFT: u32 = 7u32;
pub const RT5640_PLL_K_MAX: u32 = 0x1fu32;
pub const RT5640_PLL_K_MASK: u32 = (RT5640_PLL_K_MAX);
pub const RT5640_PLL_K_SFT: u32 = 0u32;

/* PLL M/N/K Code Control 2 (0x82) */
pub const RT5640_PLL_M_MAX: u32 = 0xfu32;
pub const RT5640_PLL_M_MASK: u32 = (RT5640_PLL_M_MAX << 12u32);
pub const RT5640_PLL_M_SFT: u32 = 12u32;
pub const RT5640_PLL_M_BP: u32 = (0x1u32 << 11u32);
pub const RT5640_PLL_M_BP_SFT: u32 = 11u32;

/* ASRC Control 1 (0x83) */
pub const RT5640_STO_T_MASK: u32 = (0x1u32 << 15u32);
pub const RT5640_STO_T_SFT: u32 = 15u32;
pub const RT5640_STO_T_SCLK: u32 = (0x0u32 << 15u32);
pub const RT5640_STO_T_LRCK1: u32 = (0x1u32 << 15u32);
pub const RT5640_M1_T_MASK: u32 = (0x1u32 << 14u32);
pub const RT5640_M1_T_SFT: u32 = 14u32;
pub const RT5640_M1_T_I2S2: u32 = (0x0u32 << 14u32);
pub const RT5640_M1_T_I2S2_D3: u32 = (0x1u32 << 14u32);
pub const RT5640_I2S2_F_MASK: u32 = (0x1u32 << 12u32);
pub const RT5640_I2S2_F_SFT: u32 = 12u32;
pub const RT5640_I2S2_F_I2S2_D2: u32 = (0x0u32 << 12u32);
pub const RT5640_I2S2_F_I2S1_TCLK: u32 = (0x1u32 << 12u32);
pub const RT5640_DMIC_1_M_MASK: u32 = (0x1u32 << 9u32);
pub const RT5640_DMIC_1_M_SFT: u32 = 9u32;
pub const RT5640_DMIC_1_M_NOR: u32 = (0x0u32 << 9u32);
pub const RT5640_DMIC_1_M_ASYN: u32 = (0x1u32 << 9u32);
pub const RT5640_DMIC_2_M_MASK: u32 = (0x1u32 << 8u32);
pub const RT5640_DMIC_2_M_SFT: u32 = 8u32;
pub const RT5640_DMIC_2_M_NOR: u32 = (0x0u32 << 8u32);
pub const RT5640_DMIC_2_M_ASYN: u32 = (0x1u32 << 8u32);

/* ASRC clock source selection (0x84) */
pub const RT5640_CLK_SEL_SYS: u32 = 0x0u32;
pub const RT5640_CLK_SEL_ASRC: u32 = 0x1u32;

/* ASRC Control 2 (0x84) */
pub const RT5640_MDA_L_M_MASK: u32 = (0x1u32 << 15u32);
pub const RT5640_MDA_L_M_SFT: u32 = 15u32;
pub const RT5640_MDA_L_M_NOR: u32 = (0x0u32 << 15u32);
pub const RT5640_MDA_L_M_ASYN: u32 = (0x1u32 << 15u32);
pub const RT5640_MDA_R_M_MASK: u32 = (0x1u32 << 14u32);
pub const RT5640_MDA_R_M_SFT: u32 = 14u32;
pub const RT5640_MDA_R_M_NOR: u32 = (0x0u32 << 14u32);
pub const RT5640_MDA_R_M_ASYN: u32 = (0x1u32 << 14u32);
pub const RT5640_MAD_L_M_MASK: u32 = (0x1u32 << 13u32);
pub const RT5640_MAD_L_M_SFT: u32 = 13u32;
pub const RT5640_MAD_L_M_NOR: u32 = (0x0u32 << 13u32);
pub const RT5640_MAD_L_M_ASYN: u32 = (0x1u32 << 13u32);
pub const RT5640_MAD_R_M_MASK: u32 = (0x1u32 << 12u32);
pub const RT5640_MAD_R_M_SFT: u32 = 12u32;
pub const RT5640_MAD_R_M_NOR: u32 = (0x0u32 << 12u32);
pub const RT5640_MAD_R_M_ASYN: u32 = (0x1u32 << 12u32);
pub const RT5640_ADC_M_MASK: u32 = (0x1u32 << 11u32);
pub const RT5640_ADC_M_SFT: u32 = 11u32;
pub const RT5640_ADC_M_NOR: u32 = (0x0u32 << 11u32);
pub const RT5640_ADC_M_ASYN: u32 = (0x1u32 << 11u32);
pub const RT5640_STO_DAC_M_MASK: u32 = (0x1u32 << 5u32);
pub const RT5640_STO_DAC_M_SFT: u32 = 5u32;
pub const RT5640_STO_DAC_M_NOR: u32 = (0x0u32 << 5u32);
pub const RT5640_STO_DAC_M_ASYN: u32 = (0x1u32 << 5u32);
pub const RT5640_I2S1_R_D_MASK: u32 = (0x1u32 << 4u32);
pub const RT5640_I2S1_R_D_SFT: u32 = 4u32;
pub const RT5640_I2S1_R_D_DIS: u32 = (0x0u32 << 4u32);
pub const RT5640_I2S1_R_D_EN: u32 = (0x1u32 << 4u32);
pub const RT5640_I2S2_R_D_MASK: u32 = (0x1u32 << 3u32);
pub const RT5640_I2S2_R_D_SFT: u32 = 3u32;
pub const RT5640_I2S2_R_D_DIS: u32 = (0x0u32 << 3u32);
pub const RT5640_I2S2_R_D_EN: u32 = (0x1u32 << 3u32);
pub const RT5640_PRE_SCLK_MASK: u32 = 0x3u32;
pub const RT5640_PRE_SCLK_SFT: u32 = 0u32;
pub const RT5640_PRE_SCLK_512: u32 = 0x0u32;
pub const RT5640_PRE_SCLK_1024: u32 = 0x1u32;
pub const RT5640_PRE_SCLK_2048: u32 = 0x2u32;

/* ASRC Control 3 (0x85) */
pub const RT5640_I2S1_RATE_MASK: u32 = (0xfu32 << 12u32);
pub const RT5640_I2S1_RATE_SFT: u32 = 12u32;
pub const RT5640_I2S2_RATE_MASK: u32 = (0xfu32 << 8u32);
pub const RT5640_I2S2_RATE_SFT: u32 = 8u32;

/* ASRC Control 4 (0x89) */
pub const RT5640_I2S1_PD_MASK: u32 = (0x7u32 << 12u32);
pub const RT5640_I2S1_PD_SFT: u32 = 12u32;
pub const RT5640_I2S2_PD_MASK: u32 = (0x7u32 << 8u32);
pub const RT5640_I2S2_PD_SFT: u32 = 8u32;

/* HPOUT Over Current Detection (0x8b) */
pub const RT5640_HP_OVCD_MASK: u32 = (0x1u32 << 10u32);
pub const RT5640_HP_OVCD_SFT: u32 = 10u32;
pub const RT5640_HP_OVCD_DIS: u32 = (0x0u32 << 10u32);
pub const RT5640_HP_OVCD_EN: u32 = (0x1u32 << 10u32);
pub const RT5640_HP_OC_TH_MASK: u32 = (0x3u32 << 8u32);
pub const RT5640_HP_OC_TH_SFT: u32 = 8u32;
pub const RT5640_HP_OC_TH_90: u32 = (0x0u32 << 8u32);
pub const RT5640_HP_OC_TH_105: u32 = (0x1u32 << 8u32);
pub const RT5640_HP_OC_TH_120: u32 = (0x2u32 << 8u32);
pub const RT5640_HP_OC_TH_135: u32 = (0x3u32 << 8u32);

/* Class D Over Current Control (0x8c) */
pub const RT5640_CLSD_OC_MASK: u32 = (0x1u32 << 9u32);
pub const RT5640_CLSD_OC_SFT: u32 = 9u32;
pub const RT5640_CLSD_OC_PU: u32 = (0x0u32 << 9u32);
pub const RT5640_CLSD_OC_PD: u32 = (0x1u32 << 9u32);
pub const RT5640_AUTO_PD_MASK: u32 = (0x1u32 << 8u32);
pub const RT5640_AUTO_PD_SFT: u32 = 8u32;
pub const RT5640_AUTO_PD_DIS: u32 = (0x0u32 << 8u32);
pub const RT5640_AUTO_PD_EN: u32 = (0x1u32 << 8u32);
pub const RT5640_CLSD_OC_TH_MASK: u32 = 0x3fu32;
pub const RT5640_CLSD_OC_TH_SFT: u32 = 0u32;

/* Class D Output Control (0x8d) */
pub const RT5640_CLSD_RATIO_MASK: u32 = (0xfu32 << 12u32);
pub const RT5640_CLSD_RATIO_SFT: u32 = 12u32;
pub const RT5640_CLSD_OM_MASK: u32 = (0x1u32 << 11u32);
pub const RT5640_CLSD_OM_SFT: u32 = 11u32;
pub const RT5640_CLSD_OM_MONO: u32 = (0x0u32 << 11u32);
pub const RT5640_CLSD_OM_STO: u32 = (0x1u32 << 11u32);
pub const RT5640_CLSD_SCH_MASK: u32 = (0x1u32 << 10u32);
pub const RT5640_CLSD_SCH_SFT: u32 = 10u32;
pub const RT5640_CLSD_SCH_L: u32 = (0x0u32 << 10u32);
pub const RT5640_CLSD_SCH_S: u32 = (0x1u32 << 10u32);

/* Depop Mode Control 1 (0x8e) */
pub const RT5640_SMT_TRIG_MASK: u32 = (0x1u32 << 15u32);
pub const RT5640_SMT_TRIG_SFT: u32 = 15u32;
pub const RT5640_SMT_TRIG_DIS: u32 = (0x0u32 << 15u32);
pub const RT5640_SMT_TRIG_EN: u32 = (0x1u32 << 15u32);
pub const RT5640_HP_L_SMT_MASK: u32 = (0x1u32 << 9u32);
pub const RT5640_HP_L_SMT_SFT: u32 = 9u32;
pub const RT5640_HP_L_SMT_DIS: u32 = (0x0u32 << 9u32);
pub const RT5640_HP_L_SMT_EN: u32 = (0x1u32 << 9u32);
pub const RT5640_HP_R_SMT_MASK: u32 = (0x1u32 << 8u32);
pub const RT5640_HP_R_SMT_SFT: u32 = 8u32;
pub const RT5640_HP_R_SMT_DIS: u32 = (0x0u32 << 8u32);
pub const RT5640_HP_R_SMT_EN: u32 = (0x1u32 << 8u32);
pub const RT5640_HP_CD_PD_MASK: u32 = (0x1u32 << 7u32);
pub const RT5640_HP_CD_PD_SFT: u32 = 7u32;
pub const RT5640_HP_CD_PD_DIS: u32 = (0x0u32 << 7u32);
pub const RT5640_HP_CD_PD_EN: u32 = (0x1u32 << 7u32);
pub const RT5640_RSTN_MASK: u32 = (0x1u32 << 6u32);
pub const RT5640_RSTN_SFT: u32 = 6u32;
pub const RT5640_RSTN_DIS: u32 = (0x0u32 << 6u32);
pub const RT5640_RSTN_EN: u32 = (0x1u32 << 6u32);
pub const RT5640_RSTP_MASK: u32 = (0x1u32 << 5u32);
pub const RT5640_RSTP_SFT: u32 = 5u32;
pub const RT5640_RSTP_DIS: u32 = (0x0u32 << 5u32);
pub const RT5640_RSTP_EN: u32 = (0x1u32 << 5u32);
pub const RT5640_HP_CO_MASK: u32 = (0x1u32 << 4u32);
pub const RT5640_HP_CO_SFT: u32 = 4u32;
pub const RT5640_HP_CO_DIS: u32 = (0x0u32 << 4u32);
pub const RT5640_HP_CO_EN: u32 = (0x1u32 << 4u32);
pub const RT5640_HP_CP_MASK: u32 = (0x1u32 << 3u32);
pub const RT5640_HP_CP_SFT: u32 = 3u32;
pub const RT5640_HP_CP_PD: u32 = (0x0u32 << 3u32);
pub const RT5640_HP_CP_PU: u32 = (0x1u32 << 3u32);
pub const RT5640_HP_SG_MASK: u32 = (0x1u32 << 2u32);
pub const RT5640_HP_SG_SFT: u32 = 2u32;
pub const RT5640_HP_SG_DIS: u32 = (0x0u32 << 2u32);
pub const RT5640_HP_SG_EN: u32 = (0x1u32 << 2u32);
pub const RT5640_HP_DP_MASK: u32 = (0x1u32 << 1u32);
pub const RT5640_HP_DP_SFT: u32 = 1u32;
pub const RT5640_HP_DP_PD: u32 = (0x0u32 << 1u32);
pub const RT5640_HP_DP_PU: u32 = (0x1u32 << 1u32);
pub const RT5640_HP_CB_MASK: u32 = 0x1u32;
pub const RT5640_HP_CB_SFT: u32 = 0u32;
pub const RT5640_HP_CB_PD: u32 = 0x0u32;
pub const RT5640_HP_CB_PU: u32 = 0x1u32;

/* Depop Mode Control 2 (0x8f) */
pub const RT5640_DEPOP_MASK: u32 = (0x1u32 << 13u32);
pub const RT5640_DEPOP_SFT: u32 = 13u32;
pub const RT5640_DEPOP_AUTO: u32 = (0x0u32 << 13u32);
pub const RT5640_DEPOP_MAN: u32 = (0x1u32 << 13u32);
pub const RT5640_RAMP_MASK: u32 = (0x1u32 << 12u32);
pub const RT5640_RAMP_SFT: u32 = 12u32;
pub const RT5640_RAMP_DIS: u32 = (0x0u32 << 12u32);
pub const RT5640_RAMP_EN: u32 = (0x1u32 << 12u32);
pub const RT5640_BPS_MASK: u32 = (0x1u32 << 11u32);
pub const RT5640_BPS_SFT: u32 = 11u32;
pub const RT5640_BPS_DIS: u32 = (0x0u32 << 11u32);
pub const RT5640_BPS_EN: u32 = (0x1u32 << 11u32);
pub const RT5640_FAST_UPDN_MASK: u32 = (0x1u32 << 10u32);
pub const RT5640_FAST_UPDN_SFT: u32 = 10u32;
pub const RT5640_FAST_UPDN_DIS: u32 = (0x0u32 << 10u32);
pub const RT5640_FAST_UPDN_EN: u32 = (0x1u32 << 10u32);
pub const RT5640_MRES_MASK: u32 = (0x3u32 << 8u32);
pub const RT5640_MRES_SFT: u32 = 8u32;
pub const RT5640_MRES_15MO: u32 = (0x0u32 << 8u32);
pub const RT5640_MRES_25MO: u32 = (0x1u32 << 8u32);
pub const RT5640_MRES_35MO: u32 = (0x2u32 << 8u32);
pub const RT5640_MRES_45MO: u32 = (0x3u32 << 8u32);
pub const RT5640_VLO_MASK: u32 = (0x1u32 << 7u32);
pub const RT5640_VLO_SFT: u32 = 7u32;
pub const RT5640_VLO_3V: u32 = (0x0u32 << 7u32);
pub const RT5640_VLO_32V: u32 = (0x1u32 << 7u32);
pub const RT5640_DIG_DP_MASK: u32 = (0x1u32 << 6u32);
pub const RT5640_DIG_DP_SFT: u32 = 6u32;
pub const RT5640_DIG_DP_DIS: u32 = (0x0u32 << 6u32);
pub const RT5640_DIG_DP_EN: u32 = (0x1u32 << 6u32);
pub const RT5640_DP_TH_MASK: u32 = (0x3u32 << 4u32);
pub const RT5640_DP_TH_SFT: u32 = 4u32;

/* Depop Mode Control 3 (0x90) */
pub const RT5640_CP_SYS_MASK: u32 = (0x7u32 << 12u32);
pub const RT5640_CP_SYS_SFT: u32 = 12u32;
pub const RT5640_CP_FQ1_MASK: u32 = (0x7u32 << 8u32);
pub const RT5640_CP_FQ1_SFT: u32 = 8u32;
pub const RT5640_CP_FQ2_MASK: u32 = (0x7u32 << 4u32);
pub const RT5640_CP_FQ2_SFT: u32 = 4u32;
pub const RT5640_CP_FQ3_MASK: u32 = 0x7u32;
pub const RT5640_CP_FQ3_SFT: u32 = 0u32;
pub const RT5640_CP_FQ_1_5_KHZ: u32 = 0u32;
pub const RT5640_CP_FQ_3_KHZ: u32 = 1u32;
pub const RT5640_CP_FQ_6_KHZ: u32 = 2u32;
pub const RT5640_CP_FQ_12_KHZ: u32 = 3u32;
pub const RT5640_CP_FQ_24_KHZ: u32 = 4u32;
pub const RT5640_CP_FQ_48_KHZ: u32 = 5u32;
pub const RT5640_CP_FQ_96_KHZ: u32 = 6u32;
pub const RT5640_CP_FQ_192_KHZ: u32 = 7u32;

/* HPOUT charge pump (0x91) */
pub const RT5640_OSW_L_MASK: u32 = (0x1u32 << 11u32);
pub const RT5640_OSW_L_SFT: u32 = 11u32;
pub const RT5640_OSW_L_DIS: u32 = (0x0u32 << 11u32);
pub const RT5640_OSW_L_EN: u32 = (0x1u32 << 11u32);
pub const RT5640_OSW_R_MASK: u32 = (0x1u32 << 10u32);
pub const RT5640_OSW_R_SFT: u32 = 10u32;
pub const RT5640_OSW_R_DIS: u32 = (0x0u32 << 10u32);
pub const RT5640_OSW_R_EN: u32 = (0x1u32 << 10u32);
pub const RT5640_PM_HP_MASK: u32 = (0x3u32 << 8u32);
pub const RT5640_PM_HP_SFT: u32 = 8u32;
pub const RT5640_PM_HP_LV: u32 = (0x0u32 << 8u32);
pub const RT5640_PM_HP_MV: u32 = (0x1u32 << 8u32);
pub const RT5640_PM_HP_HV: u32 = (0x2u32 << 8u32);
pub const RT5640_IB_HP_MASK: u32 = (0x3u32 << 6u32);
pub const RT5640_IB_HP_SFT: u32 = 6u32;
pub const RT5640_IB_HP_125IL: u32 = (0x0u32 << 6u32);
pub const RT5640_IB_HP_25IL: u32 = (0x1u32 << 6u32);
pub const RT5640_IB_HP_5IL: u32 = (0x2u32 << 6u32);
pub const RT5640_IB_HP_1IL: u32 = (0x3u32 << 6u32);

/* PV detection and SPK gain control (0x92) */
pub const RT5640_PVDD_DET_MASK: u32 = (0x1u32 << 15u32);
pub const RT5640_PVDD_DET_SFT: u32 = 15u32;
pub const RT5640_PVDD_DET_DIS: u32 = (0x0u32 << 15u32);
pub const RT5640_PVDD_DET_EN: u32 = (0x1u32 << 15u32);
pub const RT5640_SPK_AG_MASK: u32 = (0x1u32 << 14u32);
pub const RT5640_SPK_AG_SFT: u32 = 14u32;
pub const RT5640_SPK_AG_DIS: u32 = (0x0u32 << 14u32);
pub const RT5640_SPK_AG_EN: u32 = (0x1u32 << 14u32);

/* Micbias Control (0x93) */
pub const RT5640_MIC1_BS_MASK: u32 = (0x1u32 << 15u32);
pub const RT5640_MIC1_BS_SFT: u32 = 15u32;
pub const RT5640_MIC1_BS_9AV: u32 = (0x0u32 << 15u32);
pub const RT5640_MIC1_BS_75AV: u32 = (0x1u32 << 15u32);
pub const RT5640_MIC2_BS_MASK: u32 = (0x1u32 << 14u32);
pub const RT5640_MIC2_BS_SFT: u32 = 14u32;
pub const RT5640_MIC2_BS_9AV: u32 = (0x0u32 << 14u32);
pub const RT5640_MIC2_BS_75AV: u32 = (0x1u32 << 14u32);
pub const RT5640_MIC1_CLK_MASK: u32 = (0x1u32 << 13u32);
pub const RT5640_MIC1_CLK_SFT: u32 = 13u32;
pub const RT5640_MIC1_CLK_DIS: u32 = (0x0u32 << 13u32);
pub const RT5640_MIC1_CLK_EN: u32 = (0x1u32 << 13u32);
pub const RT5640_MIC2_CLK_MASK: u32 = (0x1u32 << 12u32);
pub const RT5640_MIC2_CLK_SFT: u32 = 12u32;
pub const RT5640_MIC2_CLK_DIS: u32 = (0x0u32 << 12u32);
pub const RT5640_MIC2_CLK_EN: u32 = (0x1u32 << 12u32);
pub const RT5640_MIC1_OVCD_MASK: u32 = (0x1u32 << 11u32);
pub const RT5640_MIC1_OVCD_SFT: u32 = 11u32;
pub const RT5640_MIC1_OVCD_DIS: u32 = (0x0u32 << 11u32);
pub const RT5640_MIC1_OVCD_EN: u32 = (0x1u32 << 11u32);
pub const RT5640_MIC1_OVTH_MASK: u32 = (0x3u32 << 9u32);
pub const RT5640_MIC1_OVTH_SFT: u32 = 9u32;
pub const RT5640_MIC1_OVTH_600UA: u32 = (0x0u32 << 9u32);
pub const RT5640_MIC1_OVTH_1500UA: u32 = (0x1u32 << 9u32);
pub const RT5640_MIC1_OVTH_2000UA: u32 = (0x2u32 << 9u32);
pub const RT5640_MIC2_OVCD_MASK: u32 = (0x1u32 << 8u32);
pub const RT5640_MIC2_OVCD_SFT: u32 = 8u32;
pub const RT5640_MIC2_OVCD_DIS: u32 = (0x0u32 << 8u32);
pub const RT5640_MIC2_OVCD_EN: u32 = (0x1u32 << 8u32);
pub const RT5640_MIC2_OVTH_MASK: u32 = (0x3u32 << 6u32);
pub const RT5640_MIC2_OVTH_SFT: u32 = 6u32;
pub const RT5640_MIC2_OVTH_600UA: u32 = (0x0u32 << 6u32);
pub const RT5640_MIC2_OVTH_1500UA: u32 = (0x1u32 << 6u32);
pub const RT5640_MIC2_OVTH_2000UA: u32 = (0x2u32 << 6u32);
pub const RT5640_PWR_MB_MASK: u32 = (0x1u32 << 5u32);
pub const RT5640_PWR_MB_SFT: u32 = 5u32;
pub const RT5640_PWR_MB_PD: u32 = (0x0u32 << 5u32);
pub const RT5640_PWR_MB_PU: u32 = (0x1u32 << 5u32);
pub const RT5640_PWR_CLK25M_MASK: u32 = (0x1u32 << 4u32);
pub const RT5640_PWR_CLK25M_SFT: u32 = 4u32;
pub const RT5640_PWR_CLK25M_PD: u32 = (0x0u32 << 4u32);
pub const RT5640_PWR_CLK25M_PU: u32 = (0x1u32 << 4u32);

/* EQ Control 1 (0xb0) */
pub const RT5640_EQ_SRC_MASK: u32 = (0x1u32 << 15u32);
pub const RT5640_EQ_SRC_SFT: u32 = 15u32;
pub const RT5640_EQ_SRC_DAC: u32 = (0x0u32 << 15u32);
pub const RT5640_EQ_SRC_ADC: u32 = (0x1u32 << 15u32);
pub const RT5640_EQ_UPD: u32 = (0x1u32 << 14u32);
pub const RT5640_EQ_UPD_BIT: u32 = 14u32;
pub const RT5640_EQ_CD_MASK: u32 = (0x1u32 << 13u32);
pub const RT5640_EQ_CD_SFT: u32 = 13u32;
pub const RT5640_EQ_CD_DIS: u32 = (0x0u32 << 13u32);
pub const RT5640_EQ_CD_EN: u32 = (0x1u32 << 13u32);
pub const RT5640_EQ_DITH_MASK: u32 = (0x3u32 << 8u32);
pub const RT5640_EQ_DITH_SFT: u32 = 8u32;
pub const RT5640_EQ_DITH_NOR: u32 = (0x0u32 << 8u32);
pub const RT5640_EQ_DITH_LSB: u32 = (0x1u32 << 8u32);
pub const RT5640_EQ_DITH_LSB_1: u32 = (0x2u32 << 8u32);
pub const RT5640_EQ_DITH_LSB_2: u32 = (0x3u32 << 8u32);

/* EQ Control 2 (0xb1) */
pub const RT5640_EQ_HPF1_M_MASK: u32 = (0x1u32 << 8u32);
pub const RT5640_EQ_HPF1_M_SFT: u32 = 8u32;
pub const RT5640_EQ_HPF1_M_HI: u32 = (0x0u32 << 8u32);
pub const RT5640_EQ_HPF1_M_1ST: u32 = (0x1u32 << 8u32);
pub const RT5640_EQ_LPF1_M_MASK: u32 = (0x1u32 << 7u32);
pub const RT5640_EQ_LPF1_M_SFT: u32 = 7u32;
pub const RT5640_EQ_LPF1_M_LO: u32 = (0x0u32 << 7u32);
pub const RT5640_EQ_LPF1_M_1ST: u32 = (0x1u32 << 7u32);
pub const RT5640_EQ_HPF2_MASK: u32 = (0x1u32 << 6u32);
pub const RT5640_EQ_HPF2_SFT: u32 = 6u32;
pub const RT5640_EQ_HPF2_DIS: u32 = (0x0u32 << 6u32);
pub const RT5640_EQ_HPF2_EN: u32 = (0x1u32 << 6u32);
pub const RT5640_EQ_HPF1_MASK: u32 = (0x1u32 << 5u32);
pub const RT5640_EQ_HPF1_SFT: u32 = 5u32;
pub const RT5640_EQ_HPF1_DIS: u32 = (0x0u32 << 5u32);
pub const RT5640_EQ_HPF1_EN: u32 = (0x1u32 << 5u32);
pub const RT5640_EQ_BPF4_MASK: u32 = (0x1u32 << 4u32);
pub const RT5640_EQ_BPF4_SFT: u32 = 4u32;
pub const RT5640_EQ_BPF4_DIS: u32 = (0x0u32 << 4u32);
pub const RT5640_EQ_BPF4_EN: u32 = (0x1u32 << 4u32);
pub const RT5640_EQ_BPF3_MASK: u32 = (0x1u32 << 3u32);
pub const RT5640_EQ_BPF3_SFT: u32 = 3u32;
pub const RT5640_EQ_BPF3_DIS: u32 = (0x0u32 << 3u32);
pub const RT5640_EQ_BPF3_EN: u32 = (0x1u32 << 3u32);
pub const RT5640_EQ_BPF2_MASK: u32 = (0x1u32 << 2u32);
pub const RT5640_EQ_BPF2_SFT: u32 = 2u32;
pub const RT5640_EQ_BPF2_DIS: u32 = (0x0u32 << 2u32);
pub const RT5640_EQ_BPF2_EN: u32 = (0x1u32 << 2u32);
pub const RT5640_EQ_BPF1_MASK: u32 = (0x1u32 << 1u32);
pub const RT5640_EQ_BPF1_SFT: u32 = 1u32;
pub const RT5640_EQ_BPF1_DIS: u32 = (0x0u32 << 1u32);
pub const RT5640_EQ_BPF1_EN: u32 = (0x1u32 << 1u32);
pub const RT5640_EQ_LPF_MASK: u32 = 0x1u32;
pub const RT5640_EQ_LPF_SFT: u32 = 0u32;
pub const RT5640_EQ_LPF_DIS: u32 = 0x0u32;
pub const RT5640_EQ_LPF_EN: u32 = 0x1u32;

/* Memory Test (0xb2) */
pub const RT5640_MT_MASK: u32 = (0x1u32 << 15u32);
pub const RT5640_MT_SFT: u32 = 15u32;
pub const RT5640_MT_DIS: u32 = (0x0u32 << 15u32);
pub const RT5640_MT_EN: u32 = (0x1u32 << 15u32);

/* DRC/AGC Control 1 (0xb4) */
pub const RT5640_DRC_AGC_P_MASK: u32 = (0x1u32 << 15u32);
pub const RT5640_DRC_AGC_P_SFT: u32 = 15u32;
pub const RT5640_DRC_AGC_P_DAC: u32 = (0x0u32 << 15u32);
pub const RT5640_DRC_AGC_P_ADC: u32 = (0x1u32 << 15u32);
pub const RT5640_DRC_AGC_MASK: u32 = (0x1u32 << 14u32);
pub const RT5640_DRC_AGC_SFT: u32 = 14u32;
pub const RT5640_DRC_AGC_DIS: u32 = (0x0u32 << 14u32);
pub const RT5640_DRC_AGC_EN: u32 = (0x1u32 << 14u32);
pub const RT5640_DRC_AGC_UPD: u32 = (0x1u32 << 13u32);
pub const RT5640_DRC_AGC_UPD_BIT: u32 = 13u32;
pub const RT5640_DRC_AGC_AR_MASK: u32 = (0x1fu32 << 8u32);
pub const RT5640_DRC_AGC_AR_SFT: u32 = 8u32;
pub const RT5640_DRC_AGC_R_MASK: u32 = (0x7u32 << 5u32);
pub const RT5640_DRC_AGC_R_SFT: u32 = 5u32;
pub const RT5640_DRC_AGC_R_48K: u32 = (0x1u32 << 5u32);
pub const RT5640_DRC_AGC_R_96K: u32 = (0x2u32 << 5u32);
pub const RT5640_DRC_AGC_R_192K: u32 = (0x3u32 << 5u32);
pub const RT5640_DRC_AGC_R_441K: u32 = (0x5u32 << 5u32);
pub const RT5640_DRC_AGC_R_882K: u32 = (0x6u32 << 5u32);
pub const RT5640_DRC_AGC_R_1764K: u32 = (0x7u32 << 5u32);
pub const RT5640_DRC_AGC_RC_MASK: u32 = 0x1fu32;
pub const RT5640_DRC_AGC_RC_SFT: u32 = 0u32;

/* DRC/AGC Control 2 (0xb5) */
pub const RT5640_DRC_AGC_POB_MASK: u32 = (0x3fu32 << 8u32);
pub const RT5640_DRC_AGC_POB_SFT: u32 = 8u32;
pub const RT5640_DRC_AGC_CP_MASK: u32 = (0x1u32 << 7u32);
pub const RT5640_DRC_AGC_CP_SFT: u32 = 7u32;
pub const RT5640_DRC_AGC_CP_DIS: u32 = (0x0u32 << 7u32);
pub const RT5640_DRC_AGC_CP_EN: u32 = (0x1u32 << 7u32);
pub const RT5640_DRC_AGC_CPR_MASK: u32 = (0x3u32 << 5u32);
pub const RT5640_DRC_AGC_CPR_SFT: u32 = 5u32;
pub const RT5640_DRC_AGC_CPR_1_1: u32 = (0x0u32 << 5u32);
pub const RT5640_DRC_AGC_CPR_1_2: u32 = (0x1u32 << 5u32);
pub const RT5640_DRC_AGC_CPR_1_3: u32 = (0x2u32 << 5u32);
pub const RT5640_DRC_AGC_CPR_1_4: u32 = (0x3u32 << 5u32);
pub const RT5640_DRC_AGC_PRB_MASK: u32 = 0x1fu32;
pub const RT5640_DRC_AGC_PRB_SFT: u32 = 0u32;

/* DRC/AGC Control 3 (0xb6) */
pub const RT5640_DRC_AGC_NGB_MASK: u32 = (0xfu32 << 12u32);
pub const RT5640_DRC_AGC_NGB_SFT: u32 = 12u32;
pub const RT5640_DRC_AGC_TAR_MASK: u32 = (0x1fu32 << 7u32);
pub const RT5640_DRC_AGC_TAR_SFT: u32 = 7u32;
pub const RT5640_DRC_AGC_NG_MASK: u32 = (0x1u32 << 6u32);
pub const RT5640_DRC_AGC_NG_SFT: u32 = 6u32;
pub const RT5640_DRC_AGC_NG_DIS: u32 = (0x0u32 << 6u32);
pub const RT5640_DRC_AGC_NG_EN: u32 = (0x1u32 << 6u32);
pub const RT5640_DRC_AGC_NGH_MASK: u32 = (0x1u32 << 5u32);
pub const RT5640_DRC_AGC_NGH_SFT: u32 = 5u32;
pub const RT5640_DRC_AGC_NGH_DIS: u32 = (0x0u32 << 5u32);
pub const RT5640_DRC_AGC_NGH_EN: u32 = (0x1u32 << 5u32);
pub const RT5640_DRC_AGC_NGT_MASK: u32 = 0x1fu32;
pub const RT5640_DRC_AGC_NGT_SFT: u32 = 0u32;

/* ANC Control 1 (0xb8) */
pub const RT5640_ANC_M_MASK: u32 = (0x1u32 << 15u32);
pub const RT5640_ANC_M_SFT: u32 = 15u32;
pub const RT5640_ANC_M_NOR: u32 = (0x0u32 << 15u32);
pub const RT5640_ANC_M_REV: u32 = (0x1u32 << 15u32);
pub const RT5640_ANC_MASK: u32 = (0x1u32 << 14u32);
pub const RT5640_ANC_SFT: u32 = 14u32;
pub const RT5640_ANC_DIS: u32 = (0x0u32 << 14u32);
pub const RT5640_ANC_EN: u32 = (0x1u32 << 14u32);
pub const RT5640_ANC_MD_MASK: u32 = (0x3u32 << 12u32);
pub const RT5640_ANC_MD_SFT: u32 = 12u32;
pub const RT5640_ANC_MD_DIS: u32 = (0x0u32 << 12u32);
pub const RT5640_ANC_MD_67MS: u32 = (0x1u32 << 12u32);
pub const RT5640_ANC_MD_267MS: u32 = (0x2u32 << 12u32);
pub const RT5640_ANC_MD_1067MS: u32 = (0x3u32 << 12u32);
pub const RT5640_ANC_SN_MASK: u32 = (0x1u32 << 11u32);
pub const RT5640_ANC_SN_SFT: u32 = 11u32;
pub const RT5640_ANC_SN_DIS: u32 = (0x0u32 << 11u32);
pub const RT5640_ANC_SN_EN: u32 = (0x1u32 << 11u32);
pub const RT5640_ANC_CLK_MASK: u32 = (0x1u32 << 10u32);
pub const RT5640_ANC_CLK_SFT: u32 = 10u32;
pub const RT5640_ANC_CLK_ANC: u32 = (0x0u32 << 10u32);
pub const RT5640_ANC_CLK_REG: u32 = (0x1u32 << 10u32);
pub const RT5640_ANC_ZCD_MASK: u32 = (0x3u32 << 8u32);
pub const RT5640_ANC_ZCD_SFT: u32 = 8u32;
pub const RT5640_ANC_ZCD_DIS: u32 = (0x0u32 << 8u32);
pub const RT5640_ANC_ZCD_T1: u32 = (0x1u32 << 8u32);
pub const RT5640_ANC_ZCD_T2: u32 = (0x2u32 << 8u32);
pub const RT5640_ANC_ZCD_WT: u32 = (0x3u32 << 8u32);
pub const RT5640_ANC_CS_MASK: u32 = (0x1u32 << 7u32);
pub const RT5640_ANC_CS_SFT: u32 = 7u32;
pub const RT5640_ANC_CS_DIS: u32 = (0x0u32 << 7u32);
pub const RT5640_ANC_CS_EN: u32 = (0x1u32 << 7u32);
pub const RT5640_ANC_SW_MASK: u32 = (0x1u32 << 6u32);
pub const RT5640_ANC_SW_SFT: u32 = 6u32;
pub const RT5640_ANC_SW_NOR: u32 = (0x0u32 << 6u32);
pub const RT5640_ANC_SW_AUTO: u32 = (0x1u32 << 6u32);
pub const RT5640_ANC_CO_L_MASK: u32 = 0x3fu32;
pub const RT5640_ANC_CO_L_SFT: u32 = 0u32;

/* ANC Control 2 (0xb6) */
pub const RT5640_ANC_FG_R_MASK: u32 = (0xfu32 << 12u32);
pub const RT5640_ANC_FG_R_SFT: u32 = 12u32;
pub const RT5640_ANC_FG_L_MASK: u32 = (0xfu32 << 8u32);
pub const RT5640_ANC_FG_L_SFT: u32 = 8u32;
pub const RT5640_ANC_CG_R_MASK: u32 = (0xfu32 << 4u32);
pub const RT5640_ANC_CG_R_SFT: u32 = 4u32;
pub const RT5640_ANC_CG_L_MASK: u32 = 0xfu32;
pub const RT5640_ANC_CG_L_SFT: u32 = 0u32;

/* ANC Control 3 (0xb6) */
pub const RT5640_ANC_CD_MASK: u32 = (0x1u32 << 6u32);
pub const RT5640_ANC_CD_SFT: u32 = 6u32;
pub const RT5640_ANC_CD_BOTH: u32 = (0x0u32 << 6u32);
pub const RT5640_ANC_CD_IND: u32 = (0x1u32 << 6u32);
pub const RT5640_ANC_CO_R_MASK: u32 = 0x3fu32;
pub const RT5640_ANC_CO_R_SFT: u32 = 0u32;

/* Jack Detect Control (0xbb) */
pub const RT5640_JD_MASK: u32 = (0x7u32 << 13u32);
pub const RT5640_JD_SFT: u32 = 13u32;
pub const RT5640_JD_DIS: u32 = (0x0u32 << 13u32);
pub const RT5640_JD_GPIO1: u32 = (0x1u32 << 13u32);
pub const RT5640_JD_JD1_IN4P: u32 = (0x2u32 << 13u32);
pub const RT5640_JD_JD2_IN4N: u32 = (0x3u32 << 13u32);
pub const RT5640_JD_GPIO2: u32 = (0x4u32 << 13u32);
pub const RT5640_JD_GPIO3: u32 = (0x5u32 << 13u32);
pub const RT5640_JD_GPIO4: u32 = (0x6u32 << 13u32);
pub const RT5640_JD_HP_MASK: u32 = (0x1u32 << 11u32);
pub const RT5640_JD_HP_SFT: u32 = 11u32;
pub const RT5640_JD_HP_DIS: u32 = (0x0u32 << 11u32);
pub const RT5640_JD_HP_EN: u32 = (0x1u32 << 11u32);
pub const RT5640_JD_HP_TRG_MASK: u32 = (0x1u32 << 10u32);
pub const RT5640_JD_HP_TRG_SFT: u32 = 10u32;
pub const RT5640_JD_HP_TRG_LO: u32 = (0x0u32 << 10u32);
pub const RT5640_JD_HP_TRG_HI: u32 = (0x1u32 << 10u32);
pub const RT5640_JD_SPL_MASK: u32 = (0x1u32 << 9u32);
pub const RT5640_JD_SPL_SFT: u32 = 9u32;
pub const RT5640_JD_SPL_DIS: u32 = (0x0u32 << 9u32);
pub const RT5640_JD_SPL_EN: u32 = (0x1u32 << 9u32);
pub const RT5640_JD_SPL_TRG_MASK: u32 = (0x1u32 << 8u32);
pub const RT5640_JD_SPL_TRG_SFT: u32 = 8u32;
pub const RT5640_JD_SPL_TRG_LO: u32 = (0x0u32 << 8u32);
pub const RT5640_JD_SPL_TRG_HI: u32 = (0x1u32 << 8u32);
pub const RT5640_JD_SPR_MASK: u32 = (0x1u32 << 7u32);
pub const RT5640_JD_SPR_SFT: u32 = 7u32;
pub const RT5640_JD_SPR_DIS: u32 = (0x0u32 << 7u32);
pub const RT5640_JD_SPR_EN: u32 = (0x1u32 << 7u32);
pub const RT5640_JD_SPR_TRG_MASK: u32 = (0x1u32 << 6u32);
pub const RT5640_JD_SPR_TRG_SFT: u32 = 6u32;
pub const RT5640_JD_SPR_TRG_LO: u32 = (0x0u32 << 6u32);
pub const RT5640_JD_SPR_TRG_HI: u32 = (0x1u32 << 6u32);
pub const RT5640_JD_MO_MASK: u32 = (0x1u32 << 5u32);
pub const RT5640_JD_MO_SFT: u32 = 5u32;
pub const RT5640_JD_MO_DIS: u32 = (0x0u32 << 5u32);
pub const RT5640_JD_MO_EN: u32 = (0x1u32 << 5u32);
pub const RT5640_JD_MO_TRG_MASK: u32 = (0x1u32 << 4u32);
pub const RT5640_JD_MO_TRG_SFT: u32 = 4u32;
pub const RT5640_JD_MO_TRG_LO: u32 = (0x0u32 << 4u32);
pub const RT5640_JD_MO_TRG_HI: u32 = (0x1u32 << 4u32);
pub const RT5640_JD_LO_MASK: u32 = (0x1u32 << 3u32);
pub const RT5640_JD_LO_SFT: u32 = 3u32;
pub const RT5640_JD_LO_DIS: u32 = (0x0u32 << 3u32);
pub const RT5640_JD_LO_EN: u32 = (0x1u32 << 3u32);
pub const RT5640_JD_LO_TRG_MASK: u32 = (0x1u32 << 2u32);
pub const RT5640_JD_LO_TRG_SFT: u32 = 2u32;
pub const RT5640_JD_LO_TRG_LO: u32 = (0x0u32 << 2u32);
pub const RT5640_JD_LO_TRG_HI: u32 = (0x1u32 << 2u32);
pub const RT5640_JD1_IN4P_MASK: u32 = (0x1u32 << 1u32);
pub const RT5640_JD1_IN4P_SFT: u32 = 1u32;
pub const RT5640_JD1_IN4P_DIS: u32 = (0x0u32 << 1u32);
pub const RT5640_JD1_IN4P_EN: u32 = (0x1u32 << 1u32);
pub const RT5640_JD2_IN4N_MASK: u32 = 0x1u32;
pub const RT5640_JD2_IN4N_SFT: u32 = 0u32;
pub const RT5640_JD2_IN4N_DIS: u32 = 0x0u32;
pub const RT5640_JD2_IN4N_EN: u32 = 0x1u32;

/* Jack detect for ANC (0xbc) */
pub const RT5640_ANC_DET_MASK: u32 = (0x3u32 << 4u32);
pub const RT5640_ANC_DET_SFT: u32 = 4u32;
pub const RT5640_ANC_DET_DIS: u32 = (0x0u32 << 4u32);
pub const RT5640_ANC_DET_MB1: u32 = (0x1u32 << 4u32);
pub const RT5640_ANC_DET_MB2: u32 = (0x2u32 << 4u32);
pub const RT5640_ANC_DET_JD: u32 = (0x3u32 << 4u32);
pub const RT5640_AD_TRG_MASK: u32 = (0x1u32 << 3u32);
pub const RT5640_AD_TRG_SFT: u32 = 3u32;
pub const RT5640_AD_TRG_LO: u32 = (0x0u32 << 3u32);
pub const RT5640_AD_TRG_HI: u32 = (0x1u32 << 3u32);
pub const RT5640_ANCM_DET_MASK: u32 = (0x3u32 << 4u32);
pub const RT5640_ANCM_DET_SFT: u32 = 4u32;
pub const RT5640_ANCM_DET_DIS: u32 = (0x0u32 << 4u32);
pub const RT5640_ANCM_DET_MB1: u32 = (0x1u32 << 4u32);
pub const RT5640_ANCM_DET_MB2: u32 = (0x2u32 << 4u32);
pub const RT5640_ANCM_DET_JD: u32 = (0x3u32 << 4u32);
pub const RT5640_AMD_TRG_MASK: u32 = (0x1u32 << 3u32);
pub const RT5640_AMD_TRG_SFT: u32 = 3u32;
pub const RT5640_AMD_TRG_LO: u32 = (0x0u32 << 3u32);
pub const RT5640_AMD_TRG_HI: u32 = (0x1u32 << 3u32);

/* IRQ Control 1 (0xbd) */
pub const RT5640_IRQ_JD_MASK: u32 = (0x1u32 << 15u32);
pub const RT5640_IRQ_JD_SFT: u32 = 15u32;
pub const RT5640_IRQ_JD_BP: u32 = (0x0u32 << 15u32);
pub const RT5640_IRQ_JD_NOR: u32 = (0x1u32 << 15u32);
pub const RT5640_IRQ_OT_MASK: u32 = (0x1u32 << 14u32);
pub const RT5640_IRQ_OT_SFT: u32 = 14u32;
pub const RT5640_IRQ_OT_BP: u32 = (0x0u32 << 14u32);
pub const RT5640_IRQ_OT_NOR: u32 = (0x1u32 << 14u32);
pub const RT5640_JD_STKY_MASK: u32 = (0x1u32 << 13u32);
pub const RT5640_JD_STKY_SFT: u32 = 13u32;
pub const RT5640_JD_STKY_DIS: u32 = (0x0u32 << 13u32);
pub const RT5640_JD_STKY_EN: u32 = (0x1u32 << 13u32);
pub const RT5640_OT_STKY_MASK: u32 = (0x1u32 << 12u32);
pub const RT5640_OT_STKY_SFT: u32 = 12u32;
pub const RT5640_OT_STKY_DIS: u32 = (0x0u32 << 12u32);
pub const RT5640_OT_STKY_EN: u32 = (0x1u32 << 12u32);
pub const RT5640_JD_P_MASK: u32 = (0x1u32 << 11u32);
pub const RT5640_JD_P_SFT: u32 = 11u32;
pub const RT5640_JD_P_NOR: u32 = (0x0u32 << 11u32);
pub const RT5640_JD_P_INV: u32 = (0x1u32 << 11u32);
pub const RT5640_OT_P_MASK: u32 = (0x1u32 << 10u32);
pub const RT5640_OT_P_SFT: u32 = 10u32;
pub const RT5640_OT_P_NOR: u32 = (0x0u32 << 10u32);
pub const RT5640_OT_P_INV: u32 = (0x1u32 << 10u32);

/* IRQ Control 2 (0xbe) */
pub const RT5640_IRQ_MB1_OC_MASK: u32 = (0x1u32 << 15u32);
pub const RT5640_IRQ_MB1_OC_SFT: u32 = 15u32;
pub const RT5640_IRQ_MB1_OC_BP: u32 = (0x0u32 << 15u32);
pub const RT5640_IRQ_MB1_OC_NOR: u32 = (0x1u32 << 15u32);
pub const RT5640_IRQ_MB2_OC_MASK: u32 = (0x1u32 << 14u32);
pub const RT5640_IRQ_MB2_OC_SFT: u32 = 14u32;
pub const RT5640_IRQ_MB2_OC_BP: u32 = (0x0u32 << 14u32);
pub const RT5640_IRQ_MB2_OC_NOR: u32 = (0x1u32 << 14u32);
pub const RT5640_MB1_OC_STKY_MASK: u32 = (0x1u32 << 11u32);
pub const RT5640_MB1_OC_STKY_SFT: u32 = 11u32;
pub const RT5640_MB1_OC_STKY_DIS: u32 = (0x0u32 << 11u32);
pub const RT5640_MB1_OC_STKY_EN: u32 = (0x1u32 << 11u32);
pub const RT5640_MB2_OC_STKY_MASK: u32 = (0x1u32 << 10u32);
pub const RT5640_MB2_OC_STKY_SFT: u32 = 10u32;
pub const RT5640_MB2_OC_STKY_DIS: u32 = (0x0u32 << 10u32);
pub const RT5640_MB2_OC_STKY_EN: u32 = (0x1u32 << 10u32);
pub const RT5640_MB1_OC_P_MASK: u32 = (0x1u32 << 7u32);
pub const RT5640_MB1_OC_P_SFT: u32 = 7u32;
pub const RT5640_MB1_OC_P_NOR: u32 = (0x0u32 << 7u32);
pub const RT5640_MB1_OC_P_INV: u32 = (0x1u32 << 7u32);
pub const RT5640_MB2_OC_P_MASK: u32 = (0x1u32 << 6u32);
pub const RT5640_MB2_OC_P_SFT: u32 = 6u32;
pub const RT5640_MB2_OC_P_NOR: u32 = (0x0u32 << 6u32);
pub const RT5640_MB2_OC_P_INV: u32 = (0x1u32 << 6u32);
pub const RT5640_MB1_OC_STATUS: u32 = (0x1u32 << 3u32);
pub const RT5640_MB1_OC_STATUS_SFT: u32 = 3u32;
pub const RT5640_MB2_OC_STATUS: u32 = (0x1u32 << 2u32);
pub const RT5640_MB2_OC_STATUS_SFT: u32 = 2u32;

/* GPIO and Internal Status (0xbf) */
pub const RT5640_GPIO1_STATUS: u32 = (0x1u32 << 8u32);
pub const RT5640_GPIO2_STATUS: u32 = (0x1u32 << 7u32);
pub const RT5640_JD_STATUS: u32 = (0x1u32 << 4u32);
pub const RT5640_OVT_STATUS: u32 = (0x1u32 << 3u32);
pub const RT5640_CLS_D_OVCD_STATUS: u32 = (0x1u32 << 0u32);

/* GPIO Control 1 (0xc0) */
pub const RT5640_GP1_PIN_MASK: u32 = (0x1u32 << 15u32);
pub const RT5640_GP1_PIN_SFT: u32 = 15u32;
pub const RT5640_GP1_PIN_GPIO1: u32 = (0x0u32 << 15u32);
pub const RT5640_GP1_PIN_IRQ: u32 = (0x1u32 << 15u32);
pub const RT5640_GP2_PIN_MASK: u32 = (0x1u32 << 14u32);
pub const RT5640_GP2_PIN_SFT: u32 = 14u32;
pub const RT5640_GP2_PIN_GPIO2: u32 = (0x0u32 << 14u32);
pub const RT5640_GP2_PIN_DMIC1_SCL: u32 = (0x1u32 << 14u32);
pub const RT5640_GP3_PIN_MASK: u32 = (0x3u32 << 12u32);
pub const RT5640_GP3_PIN_SFT: u32 = 12u32;
pub const RT5640_GP3_PIN_GPIO3: u32 = (0x0u32 << 12u32);
pub const RT5640_GP3_PIN_DMIC1_SDA: u32 = (0x1u32 << 12u32);
pub const RT5640_GP3_PIN_IRQ: u32 = (0x2u32 << 12u32);
pub const RT5640_GP4_PIN_MASK: u32 = (0x1u32 << 11u32);
pub const RT5640_GP4_PIN_SFT: u32 = 11u32;
pub const RT5640_GP4_PIN_GPIO4: u32 = (0x0u32 << 11u32);
pub const RT5640_GP4_PIN_DMIC2_SDA: u32 = (0x1u32 << 11u32);
pub const RT5640_DP_SIG_MASK: u32 = (0x1u32 << 10u32);
pub const RT5640_DP_SIG_SFT: u32 = 10u32;
pub const RT5640_DP_SIG_TEST: u32 = (0x0u32 << 10u32);
pub const RT5640_DP_SIG_AP: u32 = (0x1u32 << 10u32);
pub const RT5640_GPIO_M_MASK: u32 = (0x1u32 << 9u32);
pub const RT5640_GPIO_M_SFT: u32 = 9u32;
pub const RT5640_GPIO_M_FLT: u32 = (0x0u32 << 9u32);
pub const RT5640_GPIO_M_PH: u32 = (0x1u32 << 9u32);

/* GPIO Control 3 (0xc2) */
pub const RT5640_GP4_PF_MASK: u32 = (0x1u32 << 11u32);
pub const RT5640_GP4_PF_SFT: u32 = 11u32;
pub const RT5640_GP4_PF_IN: u32 = (0x0u32 << 11u32);
pub const RT5640_GP4_PF_OUT: u32 = (0x1u32 << 11u32);
pub const RT5640_GP4_OUT_MASK: u32 = (0x1u32 << 10u32);
pub const RT5640_GP4_OUT_SFT: u32 = 10u32;
pub const RT5640_GP4_OUT_LO: u32 = (0x0u32 << 10u32);
pub const RT5640_GP4_OUT_HI: u32 = (0x1u32 << 10u32);
pub const RT5640_GP4_P_MASK: u32 = (0x1u32 << 9u32);
pub const RT5640_GP4_P_SFT: u32 = 9u32;
pub const RT5640_GP4_P_NOR: u32 = (0x0u32 << 9u32);
pub const RT5640_GP4_P_INV: u32 = (0x1u32 << 9u32);
pub const RT5640_GP3_PF_MASK: u32 = (0x1u32 << 8u32);
pub const RT5640_GP3_PF_SFT: u32 = 8u32;
pub const RT5640_GP3_PF_IN: u32 = (0x0u32 << 8u32);
pub const RT5640_GP3_PF_OUT: u32 = (0x1u32 << 8u32);
pub const RT5640_GP3_OUT_MASK: u32 = (0x1u32 << 7u32);
pub const RT5640_GP3_OUT_SFT: u32 = 7u32;
pub const RT5640_GP3_OUT_LO: u32 = (0x0u32 << 7u32);
pub const RT5640_GP3_OUT_HI: u32 = (0x1u32 << 7u32);
pub const RT5640_GP3_P_MASK: u32 = (0x1u32 << 6u32);
pub const RT5640_GP3_P_SFT: u32 = 6u32;
pub const RT5640_GP3_P_NOR: u32 = (0x0u32 << 6u32);
pub const RT5640_GP3_P_INV: u32 = (0x1u32 << 6u32);
pub const RT5640_GP2_PF_MASK: u32 = (0x1u32 << 5u32);
pub const RT5640_GP2_PF_SFT: u32 = 5u32;
pub const RT5640_GP2_PF_IN: u32 = (0x0u32 << 5u32);
pub const RT5640_GP2_PF_OUT: u32 = (0x1u32 << 5u32);
pub const RT5640_GP2_OUT_MASK: u32 = (0x1u32 << 4u32);
pub const RT5640_GP2_OUT_SFT: u32 = 4u32;
pub const RT5640_GP2_OUT_LO: u32 = (0x0u32 << 4u32);
pub const RT5640_GP2_OUT_HI: u32 = (0x1u32 << 4u32);
pub const RT5640_GP2_P_MASK: u32 = (0x1u32 << 3u32);
pub const RT5640_GP2_P_SFT: u32 = 3u32;
pub const RT5640_GP2_P_NOR: u32 = (0x0u32 << 3u32);
pub const RT5640_GP2_P_INV: u32 = (0x1u32 << 3u32);
pub const RT5640_GP1_PF_MASK: u32 = (0x1u32 << 2u32);
pub const RT5640_GP1_PF_SFT: u32 = 2u32;
pub const RT5640_GP1_PF_IN: u32 = (0x0u32 << 2u32);
pub const RT5640_GP1_PF_OUT: u32 = (0x1u32 << 2u32);
pub const RT5640_GP1_OUT_MASK: u32 = (0x1u32 << 1u32);
pub const RT5640_GP1_OUT_SFT: u32 = 1u32;
pub const RT5640_GP1_OUT_LO: u32 = (0x0u32 << 1u32);
pub const RT5640_GP1_OUT_HI: u32 = (0x1u32 << 1u32);
pub const RT5640_GP1_P_MASK: u32 = 0x1u32;
pub const RT5640_GP1_P_SFT: u32 = 0u32;
pub const RT5640_GP1_P_NOR: u32 = 0x0u32;
pub const RT5640_GP1_P_INV: u32 = 0x1u32;

/* FM34-500 Register Control 1 (0xc4) */
pub const RT5640_DSP_ADD_SFT: u32 = 0u32;

/* FM34-500 Register Control 2 (0xc5) */
pub const RT5640_DSP_DAT_SFT: u32 = 0u32;

/* FM34-500 Register Control 3 (0xc6) */
pub const RT5640_DSP_BUSY_MASK: u32 = (0x1u32 << 15u32);
pub const RT5640_DSP_BUSY_BIT: u32 = 15u32;
pub const RT5640_DSP_DS_MASK: u32 = (0x1u32 << 14u32);
pub const RT5640_DSP_DS_SFT: u32 = 14u32;
pub const RT5640_DSP_DS_FM3010: u32 = (0x1u32 << 14u32);
pub const RT5640_DSP_DS_TEMP: u32 = (0x1u32 << 14u32);
pub const RT5640_DSP_CLK_MASK: u32 = (0x3u32 << 12u32);
pub const RT5640_DSP_CLK_SFT: u32 = 12u32;
pub const RT5640_DSP_CLK_384K: u32 = (0x0u32 << 12u32);
pub const RT5640_DSP_CLK_192K: u32 = (0x1u32 << 12u32);
pub const RT5640_DSP_CLK_96K: u32 = (0x2u32 << 12u32);
pub const RT5640_DSP_CLK_64K: u32 = (0x3u32 << 12u32);
pub const RT5640_DSP_PD_PIN_MASK: u32 = (0x1u32 << 11u32);
pub const RT5640_DSP_PD_PIN_SFT: u32 = 11u32;
pub const RT5640_DSP_PD_PIN_LO: u32 = (0x0u32 << 11u32);
pub const RT5640_DSP_PD_PIN_HI: u32 = (0x1u32 << 11u32);
pub const RT5640_DSP_RST_PIN_MASK: u32 = (0x1u32 << 10u32);
pub const RT5640_DSP_RST_PIN_SFT: u32 = 10u32;
pub const RT5640_DSP_RST_PIN_LO: u32 = (0x0u32 << 10u32);
pub const RT5640_DSP_RST_PIN_HI: u32 = (0x1u32 << 10u32);
pub const RT5640_DSP_R_EN: u32 = (0x1u32 << 9u32);
pub const RT5640_DSP_R_EN_BIT: u32 = 9u32;
pub const RT5640_DSP_W_EN: u32 = (0x1u32 << 8u32);
pub const RT5640_DSP_W_EN_BIT: u32 = 8u32;
pub const RT5640_DSP_CMD_MASK: u32 = 0xffu32;
pub const RT5640_DSP_CMD_SFT: u32 = 0u32;
pub const RT5640_DSP_CMD_MW: u32 = 0x3Bu32;
pub const RT5640_DSP_CMD_MR: u32 = 0x37u32;
pub const RT5640_DSP_CMD_RR: u32 = 0x60u32;
pub const RT5640_DSP_CMD_RW: u32 = 0x68u32;

/* Programmable Register Array Control 1 (0xc8) */
pub const RT5640_REG_SEQ_MASK: u32 = (0xfu32 << 12u32);
pub const RT5640_REG_SEQ_SFT: u32 = 12u32;
pub const RT5640_SEQ1_ST_MASK: u32 = (0x1u32 << 11u32);
pub const RT5640_SEQ1_ST_SFT: u32 = 11u32;
pub const RT5640_SEQ1_ST_RUN: u32 = (0x0u32 << 11u32);
pub const RT5640_SEQ1_ST_FIN: u32 = (0x1u32 << 11u32);
pub const RT5640_SEQ2_ST_MASK: u32 = (0x1u32 << 10u32);
pub const RT5640_SEQ2_ST_SFT: u32 = 10u32;
pub const RT5640_SEQ2_ST_RUN: u32 = (0x0u32 << 10u32);
pub const RT5640_SEQ2_ST_FIN: u32 = (0x1u32 << 10u32);
pub const RT5640_REG_LV_MASK: u32 = (0x1u32 << 9u32);
pub const RT5640_REG_LV_SFT: u32 = 9u32;
pub const RT5640_REG_LV_MX: u32 = (0x0u32 << 9u32);
pub const RT5640_REG_LV_PR: u32 = (0x1u32 << 9u32);
pub const RT5640_SEQ_2_PT_MASK: u32 = (0x1u32 << 8u32);
pub const RT5640_SEQ_2_PT_BIT: u32 = 8u32;
pub const RT5640_REG_IDX_MASK: u32 = 0xffu32;
pub const RT5640_REG_IDX_SFT: u32 = 0u32;

/* Programmable Register Array Control 2 (0xc9) */
pub const RT5640_REG_DAT_MASK: u32 = 0xffffu32;
pub const RT5640_REG_DAT_SFT: u32 = 0u32;

/* Programmable Register Array Control 3 (0xca) */
pub const RT5640_SEQ_DLY_MASK: u32 = (0xffu32 << 8u32);
pub const RT5640_SEQ_DLY_SFT: u32 = 8u32;
pub const RT5640_PROG_MASK: u32 = (0x1u32 << 7u32);
pub const RT5640_PROG_SFT: u32 = 7u32;
pub const RT5640_PROG_DIS: u32 = (0x0u32 << 7u32);
pub const RT5640_PROG_EN: u32 = (0x1u32 << 7u32);
pub const RT5640_SEQ1_PT_RUN: u32 = (0x1u32 << 6u32);
pub const RT5640_SEQ1_PT_RUN_BIT: u32 = 6u32;
pub const RT5640_SEQ2_PT_RUN: u32 = (0x1u32 << 5u32);
pub const RT5640_SEQ2_PT_RUN_BIT: u32 = 5u32;

/* Programmable Register Array Control 4 (0xcb) */
pub const RT5640_SEQ1_START_MASK: u32 = (0xfu32 << 8u32);
pub const RT5640_SEQ1_START_SFT: u32 = 8u32;
pub const RT5640_SEQ1_END_MASK: u32 = 0xfu32;
pub const RT5640_SEQ1_END_SFT: u32 = 0u32;

/* Programmable Register Array Control 5 (0xcc) */
pub const RT5640_SEQ2_START_MASK: u32 = (0xfu32 << 8u32);
pub const RT5640_SEQ2_START_SFT: u32 = 8u32;
pub const RT5640_SEQ2_END_MASK: u32 = 0xfu32;
pub const RT5640_SEQ2_END_SFT: u32 = 0u32;

/* Scramble Function (0xcd) */
pub const RT5640_SCB_KEY_MASK: u32 = 0xffu32;
pub const RT5640_SCB_KEY_SFT: u32 = 0u32;

/* Scramble Control (0xce) */
pub const RT5640_SCB_SWAP_MASK: u32 = (0x1u32 << 15u32);
pub const RT5640_SCB_SWAP_SFT: u32 = 15u32;
pub const RT5640_SCB_SWAP_DIS: u32 = (0x0u32 << 15u32);
pub const RT5640_SCB_SWAP_EN: u32 = (0x1u32 << 15u32);
pub const RT5640_SCB_MASK: u32 = (0x1u32 << 14u32);
pub const RT5640_SCB_SFT: u32 = 14u32;
pub const RT5640_SCB_DIS: u32 = (0x0u32 << 14u32);
pub const RT5640_SCB_EN: u32 = (0x1u32 << 14u32);

/* Baseback Control (0xcf) */
pub const RT5640_BB_MASK: u32 = (0x1u32 << 15u32);
pub const RT5640_BB_SFT: u32 = 15u32;
pub const RT5640_BB_DIS: u32 = (0x0u32 << 15u32);
pub const RT5640_BB_EN: u32 = (0x1u32 << 15u32);
pub const RT5640_BB_CT_MASK: u32 = (0x7u32 << 12u32);
pub const RT5640_BB_CT_SFT: u32 = 12u32;
pub const RT5640_BB_CT_A: u32 = (0x0u32 << 12u32);
pub const RT5640_BB_CT_B: u32 = (0x1u32 << 12u32);
pub const RT5640_BB_CT_C: u32 = (0x2u32 << 12u32);
pub const RT5640_BB_CT_D: u32 = (0x3u32 << 12u32);
pub const RT5640_M_BB_L_MASK: u32 = (0x1u32 << 9u32);
pub const RT5640_M_BB_L_SFT: u32 = 9u32;
pub const RT5640_M_BB_R_MASK: u32 = (0x1u32 << 8u32);
pub const RT5640_M_BB_R_SFT: u32 = 8u32;
pub const RT5640_M_BB_HPF_L_MASK: u32 = (0x1u32 << 7u32);
pub const RT5640_M_BB_HPF_L_SFT: u32 = 7u32;
pub const RT5640_M_BB_HPF_R_MASK: u32 = (0x1u32 << 6u32);
pub const RT5640_M_BB_HPF_R_SFT: u32 = 6u32;
pub const RT5640_G_BB_BST_MASK: u32 = 0x3fu32;
pub const RT5640_G_BB_BST_SFT: u32 = 0u32;

/* MP3 Plus Control 1 (0xd0) */
pub const RT5640_M_MP3_L_MASK: u32 = (0x1u32 << 15u32);
pub const RT5640_M_MP3_L_SFT: u32 = 15u32;
pub const RT5640_M_MP3_R_MASK: u32 = (0x1u32 << 14u32);
pub const RT5640_M_MP3_R_SFT: u32 = 14u32;
pub const RT5640_M_MP3_MASK: u32 = (0x1u32 << 13u32);
pub const RT5640_M_MP3_SFT: u32 = 13u32;
pub const RT5640_M_MP3_DIS: u32 = (0x0u32 << 13u32);
pub const RT5640_M_MP3_EN: u32 = (0x1u32 << 13u32);
pub const RT5640_EG_MP3_MASK: u32 = (0x1fu32 << 8u32);
pub const RT5640_EG_MP3_SFT: u32 = 8u32;
pub const RT5640_MP3_HLP_MASK: u32 = (0x1u32 << 7u32);
pub const RT5640_MP3_HLP_SFT: u32 = 7u32;
pub const RT5640_MP3_HLP_DIS: u32 = (0x0u32 << 7u32);
pub const RT5640_MP3_HLP_EN: u32 = (0x1u32 << 7u32);
pub const RT5640_M_MP3_ORG_L_MASK: u32 = (0x1u32 << 6u32);
pub const RT5640_M_MP3_ORG_L_SFT: u32 = 6u32;
pub const RT5640_M_MP3_ORG_R_MASK: u32 = (0x1u32 << 5u32);
pub const RT5640_M_MP3_ORG_R_SFT: u32 = 5u32;

/* MP3 Plus Control 2 (0xd1) */
pub const RT5640_MP3_WT_MASK: u32 = (0x1u32 << 13u32);
pub const RT5640_MP3_WT_SFT: u32 = 13u32;
pub const RT5640_MP3_WT_1_4: u32 = (0x0u32 << 13u32);
pub const RT5640_MP3_WT_1_2: u32 = (0x1u32 << 13u32);
pub const RT5640_OG_MP3_MASK: u32 = (0x1fu32 << 8u32);
pub const RT5640_OG_MP3_SFT: u32 = 8u32;
pub const RT5640_HG_MP3_MASK: u32 = 0x3fu32;
pub const RT5640_HG_MP3_SFT: u32 = 0u32;

/* 3D HP Control 1 (0xd2) */
pub const RT5640_3D_CF_MASK: u32 = (0x1u32 << 15u32);
pub const RT5640_3D_CF_SFT: u32 = 15u32;
pub const RT5640_3D_CF_DIS: u32 = (0x0u32 << 15u32);
pub const RT5640_3D_CF_EN: u32 = (0x1u32 << 15u32);
pub const RT5640_3D_HP_MASK: u32 = (0x1u32 << 14u32);
pub const RT5640_3D_HP_SFT: u32 = 14u32;
pub const RT5640_3D_HP_DIS: u32 = (0x0u32 << 14u32);
pub const RT5640_3D_HP_EN: u32 = (0x1u32 << 14u32);
pub const RT5640_3D_BT_MASK: u32 = (0x1u32 << 13u32);
pub const RT5640_3D_BT_SFT: u32 = 13u32;
pub const RT5640_3D_BT_DIS: u32 = (0x0u32 << 13u32);
pub const RT5640_3D_BT_EN: u32 = (0x1u32 << 13u32);
pub const RT5640_3D_1F_MIX_MASK: u32 = (0x3u32 << 11u32);
pub const RT5640_3D_1F_MIX_SFT: u32 = 11u32;
pub const RT5640_3D_HP_M_MASK: u32 = (0x1u32 << 10u32);
pub const RT5640_3D_HP_M_SFT: u32 = 10u32;
pub const RT5640_3D_HP_M_SUR: u32 = (0x0u32 << 10u32);
pub const RT5640_3D_HP_M_FRO: u32 = (0x1u32 << 10u32);
pub const RT5640_M_3D_HRTF_MASK: u32 = (0x1u32 << 9u32);
pub const RT5640_M_3D_HRTF_SFT: u32 = 9u32;
pub const RT5640_M_3D_D2H_MASK: u32 = (0x1u32 << 8u32);
pub const RT5640_M_3D_D2H_SFT: u32 = 8u32;
pub const RT5640_M_3D_D2R_MASK: u32 = (0x1u32 << 7u32);
pub const RT5640_M_3D_D2R_SFT: u32 = 7u32;
pub const RT5640_M_3D_REVB_MASK: u32 = (0x1u32 << 6u32);
pub const RT5640_M_3D_REVB_SFT: u32 = 6u32;

/* Adjustable high pass filter control 1 (0xd3) */
pub const RT5640_2ND_HPF_MASK: u32 = (0x1u32 << 15u32);
pub const RT5640_2ND_HPF_SFT: u32 = 15u32;
pub const RT5640_2ND_HPF_DIS: u32 = (0x0u32 << 15u32);
pub const RT5640_2ND_HPF_EN: u32 = (0x1u32 << 15u32);
pub const RT5640_HPF_CF_L_MASK: u32 = (0x7u32 << 12u32);
pub const RT5640_HPF_CF_L_SFT: u32 = 12u32;
pub const RT5640_1ST_HPF_MASK: u32 = (0x1u32 << 11u32);
pub const RT5640_1ST_HPF_SFT: u32 = 11u32;
pub const RT5640_1ST_HPF_DIS: u32 = (0x0u32 << 11u32);
pub const RT5640_1ST_HPF_EN: u32 = (0x1u32 << 11u32);
pub const RT5640_HPF_CF_R_MASK: u32 = (0x7u32 << 8u32);
pub const RT5640_HPF_CF_R_SFT: u32 = 8u32;
pub const RT5640_ZD_T_MASK: u32 = (0x3u32 << 6u32);
pub const RT5640_ZD_T_SFT: u32 = 6u32;
pub const RT5640_ZD_F_MASK: u32 = (0x3u32 << 4u32);
pub const RT5640_ZD_F_SFT: u32 = 4u32;
pub const RT5640_ZD_F_IM: u32 = (0x0u32 << 4u32);
pub const RT5640_ZD_F_ZC_IM: u32 = (0x1u32 << 4u32);
pub const RT5640_ZD_F_ZC_IOD: u32 = (0x2u32 << 4u32);
pub const RT5640_ZD_F_UN: u32 = (0x3u32 << 4u32);

/* HP calibration control and Amp detection (0xd6) */
pub const RT5640_SI_DAC_MASK: u32 = (0x1u32 << 11u32);
pub const RT5640_SI_DAC_SFT: u32 = 11u32;
pub const RT5640_SI_DAC_AUTO: u32 = (0x0u32 << 11u32);
pub const RT5640_SI_DAC_TEST: u32 = (0x1u32 << 11u32);
pub const RT5640_DC_CAL_M_MASK: u32 = (0x1u32 << 10u32);
pub const RT5640_DC_CAL_M_SFT: u32 = 10u32;
pub const RT5640_DC_CAL_M_CAL: u32 = (0x0u32 << 10u32);
pub const RT5640_DC_CAL_M_NOR: u32 = (0x1u32 << 10u32);
pub const RT5640_DC_CAL_MASK: u32 = (0x1u32 << 9u32);
pub const RT5640_DC_CAL_SFT: u32 = 9u32;
pub const RT5640_DC_CAL_DIS: u32 = (0x0u32 << 9u32);
pub const RT5640_DC_CAL_EN: u32 = (0x1u32 << 9u32);
pub const RT5640_HPD_RCV_MASK: u32 = (0x7u32 << 6u32);
pub const RT5640_HPD_RCV_SFT: u32 = 6u32;
pub const RT5640_HPD_PS_MASK: u32 = (0x1u32 << 5u32);
pub const RT5640_HPD_PS_SFT: u32 = 5u32;
pub const RT5640_HPD_PS_DIS: u32 = (0x0u32 << 5u32);
pub const RT5640_HPD_PS_EN: u32 = (0x1u32 << 5u32);
pub const RT5640_CAL_M_MASK: u32 = (0x1u32 << 4u32);
pub const RT5640_CAL_M_SFT: u32 = 4u32;
pub const RT5640_CAL_M_DEP: u32 = (0x0u32 << 4u32);
pub const RT5640_CAL_M_CAL: u32 = (0x1u32 << 4u32);
pub const RT5640_CAL_MASK: u32 = (0x1u32 << 3u32);
pub const RT5640_CAL_SFT: u32 = 3u32;
pub const RT5640_CAL_DIS: u32 = (0x0u32 << 3u32);
pub const RT5640_CAL_EN: u32 = (0x1u32 << 3u32);
pub const RT5640_CAL_TEST_MASK: u32 = (0x1u32 << 2u32);
pub const RT5640_CAL_TEST_SFT: u32 = 2u32;
pub const RT5640_CAL_TEST_DIS: u32 = (0x0u32 << 2u32);
pub const RT5640_CAL_TEST_EN: u32 = (0x1u32 << 2u32);
pub const RT5640_CAL_P_MASK: u32 = 0x3u32;
pub const RT5640_CAL_P_SFT: u32 = 0u32;
pub const RT5640_CAL_P_NONE: u32 = 0x0u32;
pub const RT5640_CAL_P_CAL: u32 = 0x1u32;
pub const RT5640_CAL_P_DAC_CAL: u32 = 0x2u32;

/* Soft volume and zero cross control 1 (0xd9) */
pub const RT5640_SV_MASK: u32 = (0x1u32 << 15u32);
pub const RT5640_SV_SFT: u32 = 15u32;
pub const RT5640_SV_DIS: u32 = (0x0u32 << 15u32);
pub const RT5640_SV_EN: u32 = (0x1u32 << 15u32);
pub const RT5640_SPO_SV_MASK: u32 = (0x1u32 << 14u32);
pub const RT5640_SPO_SV_SFT: u32 = 14u32;
pub const RT5640_SPO_SV_DIS: u32 = (0x0u32 << 14u32);
pub const RT5640_SPO_SV_EN: u32 = (0x1u32 << 14u32);
pub const RT5640_OUT_SV_MASK: u32 = (0x1u32 << 13u32);
pub const RT5640_OUT_SV_SFT: u32 = 13u32;
pub const RT5640_OUT_SV_DIS: u32 = (0x0u32 << 13u32);
pub const RT5640_OUT_SV_EN: u32 = (0x1u32 << 13u32);
pub const RT5640_HP_SV_MASK: u32 = (0x1u32 << 12u32);
pub const RT5640_HP_SV_SFT: u32 = 12u32;
pub const RT5640_HP_SV_DIS: u32 = (0x0u32 << 12u32);
pub const RT5640_HP_SV_EN: u32 = (0x1u32 << 12u32);
pub const RT5640_ZCD_DIG_MASK: u32 = (0x1u32 << 11u32);
pub const RT5640_ZCD_DIG_SFT: u32 = 11u32;
pub const RT5640_ZCD_DIG_DIS: u32 = (0x0u32 << 11u32);
pub const RT5640_ZCD_DIG_EN: u32 = (0x1u32 << 11u32);
pub const RT5640_ZCD_MASK: u32 = (0x1u32 << 10u32);
pub const RT5640_ZCD_SFT: u32 = 10u32;
pub const RT5640_ZCD_PD: u32 = (0x0u32 << 10u32);
pub const RT5640_ZCD_PU: u32 = (0x1u32 << 10u32);
pub const RT5640_M_ZCD_MASK: u32 = (0x3fu32 << 4u32);
pub const RT5640_M_ZCD_SFT: u32 = 4u32;
pub const RT5640_M_ZCD_RM_L: u32 = (0x1u32 << 9u32);
pub const RT5640_M_ZCD_RM_R: u32 = (0x1u32 << 8u32);
pub const RT5640_M_ZCD_SM_L: u32 = (0x1u32 << 7u32);
pub const RT5640_M_ZCD_SM_R: u32 = (0x1u32 << 6u32);
pub const RT5640_M_ZCD_OM_L: u32 = (0x1u32 << 5u32);
pub const RT5640_M_ZCD_OM_R: u32 = (0x1u32 << 4u32);
pub const RT5640_SV_DLY_MASK: u32 = 0xfu32;
pub const RT5640_SV_DLY_SFT: u32 = 0u32;

/* Soft volume and zero cross control 2 (0xda) */
pub const RT5640_ZCD_HP_MASK: u32 = (0x1u32 << 15u32);
pub const RT5640_ZCD_HP_SFT: u32 = 15u32;
pub const RT5640_ZCD_HP_DIS: u32 = (0x0u32 << 15u32);
pub const RT5640_ZCD_HP_EN: u32 = (0x1u32 << 15u32);

/* General Control 1 (0xfa) */
pub const RT5640_EN_LOUT_DF: u32 = (0x1u32 << 14u32);
pub const RT5640_EN_LOUT_DF_SFT: u32 = 14u32;
pub const RT5640_M_MONO_ADC_L: u32 = (0x1u32 << 13u32);
pub const RT5640_M_MONO_ADC_L_SFT: u32 = 13u32;
pub const RT5640_M_MONO_ADC_R: u32 = (0x1u32 << 12u32);
pub const RT5640_M_MONO_ADC_R_SFT: u32 = 12u32;
pub const RT5640_MCLK_DET: u32 = (0x1u32 << 11u32);

/* General Control 2 (0xfb) */
pub const RT5640_IRQ_JD2_MASK: u32 = (0x1u32 << 12u32);
pub const RT5640_IRQ_JD2_SFT: u32 = 12u32;
pub const RT5640_IRQ_JD2_BP: u32 = (0x0u32 << 12u32);
pub const RT5640_IRQ_JD2_NOR: u32 = (0x1u32 << 12u32);
pub const RT5640_JD2_P_MASK: u32 = (0x1u32 << 10u32);
pub const RT5640_JD2_P_SFT: u32 = 10u32;
pub const RT5640_JD2_P_NOR: u32 = (0x0u32 << 10u32);
pub const RT5640_JD2_P_INV: u32 = (0x1u32 << 10u32);
pub const RT5640_JD2_MASK: u32 = (0x1u32 << 8u32);
pub const RT5640_JD2_SFT: u32 = 8u32;
pub const RT5640_JD2_DIS: u32 = (0x0u32 << 8u32);
pub const RT5640_JD2_EN: u32 = (0x1u32 << 8u32);

/* Codec Private Register definition */

/* MIC Over current threshold scale factor (0x15) */
pub const RT5640_MIC_OVCD_SF_MASK: u32 = (0x3u32 << 8u32);
pub const RT5640_MIC_OVCD_SF_SFT: u32 = 8u32;
pub const RT5640_MIC_OVCD_SF_0P5: u32 = (0x0u32 << 8u32);
pub const RT5640_MIC_OVCD_SF_0P75: u32 = (0x1u32 << 8u32);
pub const RT5640_MIC_OVCD_SF_1P0: u32 = (0x2u32 << 8u32);
pub const RT5640_MIC_OVCD_SF_1P5: u32 = (0x3u32 << 8u32);

/* 3D Speaker Control (0x63) */
pub const RT5640_3D_SPK_MASK: u32 = (0x1u32 << 15u32);
pub const RT5640_3D_SPK_SFT: u32 = 15u32;
pub const RT5640_3D_SPK_DIS: u32 = (0x0u32 << 15u32);
pub const RT5640_3D_SPK_EN: u32 = (0x1u32 << 15u32);
pub const RT5640_3D_SPK_M_MASK: u32 = (0x3u32 << 13u32);
pub const RT5640_3D_SPK_M_SFT: u32 = 13u32;
pub const RT5640_3D_SPK_CG_MASK: u32 = (0x1fu32 << 8u32);
pub const RT5640_3D_SPK_CG_SFT: u32 = 8u32;
pub const RT5640_3D_SPK_SG_MASK: u32 = 0x1fu32;
pub const RT5640_3D_SPK_SG_SFT: u32 = 0u32;

/* Wind Noise Detection Control 1 (0x6c) */
pub const RT5640_WND_MASK: u32 = (0x1u32 << 15u32);
pub const RT5640_WND_SFT: u32 = 15u32;
pub const RT5640_WND_DIS: u32 = (0x0u32 << 15u32);
pub const RT5640_WND_EN: u32 = (0x1u32 << 15u32);

/* Wind Noise Detection Control 2 (0x6d) */
pub const RT5640_WND_FC_NW_MASK: u32 = (0x3fu32 << 10u32);
pub const RT5640_WND_FC_NW_SFT: u32 = 10u32;
pub const RT5640_WND_FC_WK_MASK: u32 = (0x3fu32 << 4u32);
pub const RT5640_WND_FC_WK_SFT: u32 = 4u32;

/* Wind Noise Detection Control 3 (0x6e) */
pub const RT5640_HPF_FC_MASK: u32 = (0x3fu32 << 6u32);
pub const RT5640_HPF_FC_SFT: u32 = 6u32;
pub const RT5640_WND_FC_ST_MASK: u32 = 0x3fu32;
pub const RT5640_WND_FC_ST_SFT: u32 = 0u32;

/* Wind Noise Detection Control 4 (0x6f) */
pub const RT5640_WND_TH_LO_MASK: u32 = 0x3ffu32;
pub const RT5640_WND_TH_LO_SFT: u32 = 0u32;

/* Wind Noise Detection Control 5 (0x70) */
pub const RT5640_WND_TH_HI_MASK: u32 = 0x3ffu32;
pub const RT5640_WND_TH_HI_SFT: u32 = 0u32;

/* Wind Noise Detection Control 8 (0x73) */
pub const RT5640_WND_WIND_MASK: u32 = (0x1u32 << 13u32);
pub const RT5640_WND_WIND_SFT: u32 = 13u32;
pub const RT5640_WND_STRONG_MASK: u32 = (0x1u32 << 12u32);
pub const RT5640_WND_STRONG_SFT: u32 = 12u32;
pub const RT5640_NO_WIND: i32 = 0;
pub const RT5640_BREEZE: i32 = 1;
pub const RT5640_STORM: i32 = 2;


/* Dipole Speaker Interface (0x75) */
pub const RT5640_DP_ATT_MASK: u32 = (0x3u32 << 14u32);
pub const RT5640_DP_ATT_SFT: u32 = 14u32;
pub const RT5640_DP_SPK_MASK: u32 = (0x1u32 << 10u32);
pub const RT5640_DP_SPK_SFT: u32 = 10u32;
pub const RT5640_DP_SPK_DIS: u32 = (0x0u32 << 10u32);
pub const RT5640_DP_SPK_EN: u32 = (0x1u32 << 10u32);

/* EQ Pre Volume Control (0xb3) */
pub const RT5640_EQ_PRE_VOL_MASK: u32 = 0xffffu32;
pub const RT5640_EQ_PRE_VOL_SFT: u32 = 0u32;

/* EQ Post Volume Control (0xb4) */
pub const RT5640_EQ_PST_VOL_MASK: u32 = 0xffffu32;
pub const RT5640_EQ_PST_VOL_SFT: u32 = 0u32;

pub const RT5640_NO_JACK: u32 = (1u32 << 0u32);
pub const RT5640_HEADSET_DET: u32 = (1u32 << 1u32);
pub const RT5640_HEADPHO_DET: u32 = (1u32 << 2u32);

/* System Clock Source */
pub const RT5640_SCLK_S_MCLK: u32 = 0u32;
pub const RT5640_SCLK_S_PLL1: u32 = 1u32;
pub const RT5640_SCLK_S_PLL1_TK: u32 = 2u32;
pub const RT5640_SCLK_S_RCCLK: u32 = 3u32;

/* PLL1 Source */
pub const RT5640_PLL1_S_MCLK: u32 = 0u32;
pub const RT5640_PLL1_S_BCLK1: u32 = 1u32;
pub const RT5640_PLL1_S_BCLK2: u32 = 2u32;
pub const RT5640_PLL1_S_BCLK3: u32 = 3u32;


pub const RT5640_AIF1: i32 = 0;
pub const RT5640_AIF2: i32 = 1;
pub const RT5640_AIF3: i32 = 2;
pub const RT5640_AIFS: i32 = 3;


pub const RT5640_U_IF1: i32 = 0x1i32;
pub const RT5640_U_IF2: i32 = 0x2i32;
pub const RT5640_U_IF3: i32 = 0x4i32;


pub const RT5640_IF_123: i32 = 0;
pub const RT5640_IF_132: i32 = 1;
pub const RT5640_IF_312: i32 = 2;
pub const RT5640_IF_321: i32 = 3;
pub const RT5640_IF_231: i32 = 4;
pub const RT5640_IF_213: i32 = 5;
pub const RT5640_IF_113: i32 = 6;
pub const RT5640_IF_223: i32 = 7;
pub const RT5640_IF_ALL: i32 = 8;


pub const RT5640_DMIC_DIS: i32 = 0;
pub const RT5640_DMIC1: i32 = 1;
pub const RT5640_DMIC2: i32 = 2;


/* filter mask */
pub const RT5640_DA_STEREO_FILTER: i32 = 0x1i32;
pub const RT5640_DA_MONO_L_FILTER: i32 = ((0x1i32i32) << 1);
pub const RT5640_DA_MONO_R_FILTER: i32 = ((0x1i32i32) << 2);
pub const RT5640_AD_STEREO_FILTER: i32 = ((0x1i32i32) << 3);
pub const RT5640_AD_MONO_L_FILTER: i32 = ((0x1i32i32) << 4);
pub const RT5640_AD_MONO_R_FILTER: i32 = ((0x1i32i32) << 5);


#[repr(C)]
pub struct rt5640_priv {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub mclk: *mut clk,

    pub ldo1_en: *mut gpio_desc, /* GPIO for LDO1_EN */
    pub irq: i32,
    pub jd_gpio_irq: i32,
    pub sysclk: i32,
    pub sysclk_src: i32,
    pub lrck: [i32; RT5640_AIFS as usize],
    pub bclk: [i32; RT5640_AIFS as usize],
    pub master: [i32; RT5640_AIFS as usize],

    pub pll_src: i32,
    pub pll_in: i32,
    pub pll_out: i32,

    pub hp_mute: bool,
    pub asrc_en: bool,
    pub irq_requested: bool,
    pub jd_gpio_irq_requested: bool,

    /* Jack and button detect data */
    pub ovcd_irq_enabled: bool,
    pub pressed: bool,
    pub press_reported: bool,
    pub press_count: i32,
    pub release_count: i32,
    pub poll_count: i32,
    pub bp_work: delayed_work,
    pub jack_work: delayed_work,
    pub jack: *mut snd_soc_jack,
    pub jd_gpio: *mut gpio_desc,
    pub jd_src: u32,
    pub jd_inverted: bool,
    pub ovcd_th: u32,
    pub ovcd_sf: u32,
    pub use_platform_clock: bool,
}






#[repr(C)]
pub struct rt5640_set_jack_data {
    pub codec_irq_override: i32,
    pub jd_gpio: *mut gpio_desc,
    pub use_platform_clock: bool,
}


unsafe extern "C" {
    pub fn rt5640_dmic_enable(component: *mut snd_soc_component, dmic1_data_pin: bool, dmic2_data_pin: bool) -> i32;
    pub fn rt5640_sel_asrc_clk_src(component: *mut snd_soc_component, filter_mask: u32, clk_src: u32) -> i32;


    pub fn rt5640_set_ovcd_params(component: *mut snd_soc_component);
    pub fn rt5640_enable_micbias1_for_ovcd(component: *mut snd_soc_component);
    pub fn rt5640_disable_micbias1_for_ovcd(component: *mut snd_soc_component);
    pub fn rt5640_detect_headset(component: *mut snd_soc_component, hp_det_gpio: *mut gpio_desc) -> i32;
}


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
