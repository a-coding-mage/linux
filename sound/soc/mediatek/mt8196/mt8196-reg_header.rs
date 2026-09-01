/* SPDX-License-Identifier: GPL-2.0 */
/*
//  * mt8196-reg.h  --  Mediatek 8196 audio driver reg definition
//  *
//  *  Copyright (c) 2025 MediaTek Inc.
//  *  Author: Darren Ye <darren.ye@mediatek.com>
//  */


 /* reg bit enum */
pub const MT8196_MEMIF_PBUF_SIZE_32_BYTES: u32 = 0;
pub const MT8196_MEMIF_PBUF_SIZE_64_BYTES: u32 = 1;
pub const MT8196_MEMIF_PBUF_SIZE_128_BYTES: u32 = 2;
pub const MT8196_MEMIF_PBUF_SIZE_256_BYTES: u32 = 3;
pub const MT8196_MEMIF_PBUF_SIZE_NUM: u32 = 4;


pub const MT8196_MEMIF_MAX_LEN_0_BYTES: u32 = 0;
pub const MT8196_MEMIF_MAX_LEN_16_BYTES: u32 = 1;
pub const MT8196_MEMIF_MAX_LEN_32_BYTES: u32 = 2;
pub const MT8196_MEMIF_MAX_LEN_64_BYTES: u32 = 3;


pub const MT8196_MEMIF_MIN_LEN_NOT_SUPPORT: u32 = 0;
pub const MT8196_MEMIF_MIN_LEN_16_BYTES: u32 = 1;
pub const MT8196_MEMIF_MIN_LEN_32_BYTES: u32 = 2;
pub const MT8196_MEMIF_MIN_LEN_64_BYTES: u32 = 3;


/*****************************************************************************
//  * R E G I S T E R  D E F I N I T I O N
//  *****************************************************************************/
/* AUDIO_TOP_CON0 */
pub const PDN_MTKAIFV4_SFT: u32 = 25;
pub const PDN_MTKAIFV4_MASK: u32 = 0x1;
pub const PDN_MTKAIFV4_MASK_SFT: u32 = (0x1 << 25);
pub const PDN_FM_I2S_SFT: u32 = 24;
pub const PDN_FM_I2S_MASK: u32 = 0x1;
pub const PDN_FM_I2S_MASK_SFT: u32 = (0x1 << 24);
pub const PDN_HW_GAIN01_SFT: u32 = 21;
pub const PDN_HW_GAIN01_MASK: u32 = 0x1;
pub const PDN_HW_GAIN01_MASK_SFT: u32 = (0x1 << 21);
pub const PDN_HW_GAIN23_SFT: u32 = 20;
pub const PDN_HW_GAIN23_MASK: u32 = 0x1;
pub const PDN_HW_GAIN23_MASK_SFT: u32 = (0x1 << 20);
pub const PDN_STF_SFT: u32 = 19;
pub const PDN_STF_MASK: u32 = 0x1;
pub const PDN_STF_MASK_SFT: u32 = (0x1 << 19);
pub const PDN_CM0_SFT: u32 = 18;
pub const PDN_CM0_MASK: u32 = 0x1;
pub const PDN_CM0_MASK_SFT: u32 = (0x1 << 18);
pub const PDN_CM1_SFT: u32 = 17;
pub const PDN_CM1_MASK: u32 = 0x1;
pub const PDN_CM1_MASK_SFT: u32 = (0x1 << 17);
pub const PDN_CM2_SFT: u32 = 16;
pub const PDN_CM2_MASK: u32 = 0x1;
pub const PDN_CM2_MASK_SFT: u32 = (0x1 << 16);
pub const PDN_PCM0_SFT: u32 = 14;
pub const PDN_PCM0_MASK: u32 = 0x1;
pub const PDN_PCM0_MASK_SFT: u32 = (0x1 << 14);
pub const PDN_PCM1_SFT: u32 = 13;
pub const PDN_PCM1_MASK: u32 = 0x1;
pub const PDN_PCM1_MASK_SFT: u32 = (0x1 << 13);

/* AUDIO_TOP_CON1 */
pub const PDN_UL0_ADC_SFT: u32 = 23;
pub const PDN_UL0_ADC_MASK: u32 = 0x1;
pub const PDN_UL0_ADC_MASK_SFT: u32 = (0x1 << 23);
pub const PDN_UL0_TML_SFT: u32 = 22;
pub const PDN_UL0_TML_MASK: u32 = 0x1;
pub const PDN_UL0_TML_MASK_SFT: u32 = (0x1 << 22);
pub const PDN_UL0_ADC_HIRES_SFT: u32 = 21;
pub const PDN_UL0_ADC_HIRES_MASK: u32 = 0x1;
pub const PDN_UL0_ADC_HIRES_MASK_SFT: u32 = (0x1 << 21);
pub const PDN_UL0_ADC_HIRES_TML_SFT: u32 = 20;
pub const PDN_UL0_ADC_HIRES_TML_MASK: u32 = 0x1;
pub const PDN_UL0_ADC_HIRES_TML_MASK_SFT: u32 = (0x1 << 20);
pub const PDN_UL1_ADC_SFT: u32 = 19;
pub const PDN_UL1_ADC_MASK: u32 = 0x1;
pub const PDN_UL1_ADC_MASK_SFT: u32 = (0x1 << 19);
pub const PDN_UL1_TML_SFT: u32 = 18;
pub const PDN_UL1_TML_MASK: u32 = 0x1;
pub const PDN_UL1_TML_MASK_SFT: u32 = (0x1 << 18);
pub const PDN_UL1_ADC_HIRES_SFT: u32 = 17;
pub const PDN_UL1_ADC_HIRES_MASK: u32 = 0x1;
pub const PDN_UL1_ADC_HIRES_MASK_SFT: u32 = (0x1 << 17);
pub const PDN_UL1_ADC_HIRES_TML_SFT: u32 = 16;
pub const PDN_UL1_ADC_HIRES_TML_MASK: u32 = 0x1;
pub const PDN_UL1_ADC_HIRES_TML_MASK_SFT: u32 = (0x1 << 16);
pub const PDN_UL2_ADC_SFT: u32 = 15;
pub const PDN_UL2_ADC_MASK: u32 = 0x1;
pub const PDN_UL2_ADC_MASK_SFT: u32 = (0x1 << 15);
pub const PDN_UL2_TML_SFT: u32 = 14;
pub const PDN_UL2_TML_MASK: u32 = 0x1;
pub const PDN_UL2_TML_MASK_SFT: u32 = (0x1 << 14);
pub const PDN_UL2_ADC_HIRES_SFT: u32 = 13;
pub const PDN_UL2_ADC_HIRES_MASK: u32 = 0x1;
pub const PDN_UL2_ADC_HIRES_MASK_SFT: u32 = (0x1 << 13);
pub const PDN_UL2_ADC_HIRES_TML_SFT: u32 = 12;
pub const PDN_UL2_ADC_HIRES_TML_MASK: u32 = 0x1;
pub const PDN_UL2_ADC_HIRES_TML_MASK_SFT: u32 = (0x1 << 12);

/* AUDIO_TOP_CON2 */
pub const PDN_TDM_OUT_SFT: u32 = 24;
pub const PDN_TDM_OUT_MASK: u32 = 0x1;
pub const PDN_TDM_OUT_MASK_SFT: u32 = (0x1 << 24);
pub const PDN_ETDM_OUT0_SFT: u32 = 21;
pub const PDN_ETDM_OUT0_MASK: u32 = 0x1;
pub const PDN_ETDM_OUT0_MASK_SFT: u32 = (0x1 << 21);
pub const PDN_ETDM_OUT1_SFT: u32 = 20;
pub const PDN_ETDM_OUT1_MASK: u32 = 0x1;
pub const PDN_ETDM_OUT1_MASK_SFT: u32 = (0x1 << 20);
pub const PDN_ETDM_OUT2_SFT: u32 = 19;
pub const PDN_ETDM_OUT2_MASK: u32 = 0x1;
pub const PDN_ETDM_OUT2_MASK_SFT: u32 = (0x1 << 19);
pub const PDN_ETDM_OUT3_SFT: u32 = 18;
pub const PDN_ETDM_OUT3_MASK: u32 = 0x1;
pub const PDN_ETDM_OUT3_MASK_SFT: u32 = (0x1 << 18);
pub const PDN_ETDM_OUT4_SFT: u32 = 17;
pub const PDN_ETDM_OUT4_MASK: u32 = 0x1;
pub const PDN_ETDM_OUT4_MASK_SFT: u32 = (0x1 << 17);
pub const PDN_ETDM_OUT5_SFT: u32 = 16;
pub const PDN_ETDM_OUT5_MASK: u32 = 0x1;
pub const PDN_ETDM_OUT5_MASK_SFT: u32 = (0x1 << 16);
pub const PDN_ETDM_OUT6_SFT: u32 = 15;
pub const PDN_ETDM_OUT6_MASK: u32 = 0x1;
pub const PDN_ETDM_OUT6_MASK_SFT: u32 = (0x1 << 15);
pub const PDN_ETDM_IN0_SFT: u32 = 13;
pub const PDN_ETDM_IN0_MASK: u32 = 0x1;
pub const PDN_ETDM_IN0_MASK_SFT: u32 = (0x1 << 13);
pub const PDN_ETDM_IN1_SFT: u32 = 12;
pub const PDN_ETDM_IN1_MASK: u32 = 0x1;
pub const PDN_ETDM_IN1_MASK_SFT: u32 = (0x1 << 12);
pub const PDN_ETDM_IN2_SFT: u32 = 11;
pub const PDN_ETDM_IN2_MASK: u32 = 0x1;
pub const PDN_ETDM_IN2_MASK_SFT: u32 = (0x1 << 11);
pub const PDN_ETDM_IN3_SFT: u32 = 10;
pub const PDN_ETDM_IN3_MASK: u32 = 0x1;
pub const PDN_ETDM_IN3_MASK_SFT: u32 = (0x1 << 10);
pub const PDN_ETDM_IN4_SFT: u32 = 9;
pub const PDN_ETDM_IN4_MASK: u32 = 0x1;
pub const PDN_ETDM_IN4_MASK_SFT: u32 = (0x1 << 9);
pub const PDN_ETDM_IN5_SFT: u32 = 8;
pub const PDN_ETDM_IN5_MASK: u32 = 0x1;
pub const PDN_ETDM_IN5_MASK_SFT: u32 = (0x1 << 8);
pub const PDN_ETDM_IN6_SFT: u32 = 7;
pub const PDN_ETDM_IN6_MASK: u32 = 0x1;
pub const PDN_ETDM_IN6_MASK_SFT: u32 = (0x1 << 7);

/* AUDIO_TOP_CON3 */
pub const PDN_CONNSYS_I2S_ASRC_SFT: u32 = 25;
pub const PDN_CONNSYS_I2S_ASRC_MASK: u32 = 0x1;
pub const PDN_CONNSYS_I2S_ASRC_MASK_SFT: u32 = (0x1 << 25);
pub const PDN_GENERAL0_ASRC_SFT: u32 = 24;
pub const PDN_GENERAL0_ASRC_MASK: u32 = 0x1;
pub const PDN_GENERAL0_ASRC_MASK_SFT: u32 = (0x1 << 24);
pub const PDN_GENERAL1_ASRC_SFT: u32 = 23;
pub const PDN_GENERAL1_ASRC_MASK: u32 = 0x1;
pub const PDN_GENERAL1_ASRC_MASK_SFT: u32 = (0x1 << 23);
pub const PDN_GENERAL2_ASRC_SFT: u32 = 22;
pub const PDN_GENERAL2_ASRC_MASK: u32 = 0x1;
pub const PDN_GENERAL2_ASRC_MASK_SFT: u32 = (0x1 << 22);
pub const PDN_GENERAL3_ASRC_SFT: u32 = 21;
pub const PDN_GENERAL3_ASRC_MASK: u32 = 0x1;
pub const PDN_GENERAL3_ASRC_MASK_SFT: u32 = (0x1 << 21);
pub const PDN_GENERAL4_ASRC_SFT: u32 = 20;
pub const PDN_GENERAL4_ASRC_MASK: u32 = 0x1;
pub const PDN_GENERAL4_ASRC_MASK_SFT: u32 = (0x1 << 20);
pub const PDN_GENERAL5_ASRC_SFT: u32 = 19;
pub const PDN_GENERAL5_ASRC_MASK: u32 = 0x1;
pub const PDN_GENERAL5_ASRC_MASK_SFT: u32 = (0x1 << 19);
pub const PDN_GENERAL6_ASRC_SFT: u32 = 18;
pub const PDN_GENERAL6_ASRC_MASK: u32 = 0x1;
pub const PDN_GENERAL6_ASRC_MASK_SFT: u32 = (0x1 << 18);
pub const PDN_GENERAL7_ASRC_SFT: u32 = 17;
pub const PDN_GENERAL7_ASRC_MASK: u32 = 0x1;
pub const PDN_GENERAL7_ASRC_MASK_SFT: u32 = (0x1 << 17);
pub const PDN_GENERAL8_ASRC_SFT: u32 = 16;
pub const PDN_GENERAL8_ASRC_MASK: u32 = 0x1;
pub const PDN_GENERAL8_ASRC_MASK_SFT: u32 = (0x1 << 16);
pub const PDN_GENERAL9_ASRC_SFT: u32 = 15;
pub const PDN_GENERAL9_ASRC_MASK: u32 = 0x1;
pub const PDN_GENERAL9_ASRC_MASK_SFT: u32 = (0x1 << 15);
pub const PDN_GENERAL10_ASRC_SFT: u32 = 14;
pub const PDN_GENERAL10_ASRC_MASK: u32 = 0x1;
pub const PDN_GENERAL10_ASRC_MASK_SFT: u32 = (0x1 << 14);
pub const PDN_GENERAL11_ASRC_SFT: u32 = 13;
pub const PDN_GENERAL11_ASRC_MASK: u32 = 0x1;
pub const PDN_GENERAL11_ASRC_MASK_SFT: u32 = (0x1 << 13);
pub const PDN_GENERAL12_ASRC_SFT: u32 = 12;
pub const PDN_GENERAL12_ASRC_MASK: u32 = 0x1;
pub const PDN_GENERAL12_ASRC_MASK_SFT: u32 = (0x1 << 12);
pub const PDN_GENERAL13_ASRC_SFT: u32 = 11;
pub const PDN_GENERAL13_ASRC_MASK: u32 = 0x1;
pub const PDN_GENERAL13_ASRC_MASK_SFT: u32 = (0x1 << 11);
pub const PDN_GENERAL14_ASRC_SFT: u32 = 10;
pub const PDN_GENERAL14_ASRC_MASK: u32 = 0x1;
pub const PDN_GENERAL14_ASRC_MASK_SFT: u32 = (0x1 << 10);
pub const PDN_GENERAL15_ASRC_SFT: u32 = 9;
pub const PDN_GENERAL15_ASRC_MASK: u32 = 0x1;
pub const PDN_GENERAL15_ASRC_MASK_SFT: u32 = (0x1 << 9);

/* AUDIO_TOP_CON4 */
pub const PDN_APLL_TUNER1_SFT: u32 = 13;
pub const PDN_APLL_TUNER1_MASK: u32 = 0x1;
pub const PDN_APLL_TUNER1_MASK_SFT: u32 = (0x1 << 13);
pub const PDN_APLL_TUNER2_SFT: u32 = 12;
pub const PDN_APLL_TUNER2_MASK: u32 = 0x1;
pub const PDN_APLL_TUNER2_MASK_SFT: u32 = (0x1 << 12);
pub const CG_H208M_CK_SFT: u32 = 4;
pub const CG_H208M_CK_MASK: u32 = 0x1;
pub const CG_H208M_CK_MASK_SFT: u32 = (0x1 << 4);
pub const CG_APLL2_CK_SFT: u32 = 3;
pub const CG_APLL2_CK_MASK: u32 = 0x1;
pub const CG_APLL2_CK_MASK_SFT: u32 = (0x1 << 3);
pub const CG_APLL1_CK_SFT: u32 = 2;
pub const CG_APLL1_CK_MASK: u32 = 0x1;
pub const CG_APLL1_CK_MASK_SFT: u32 = (0x1 << 2);
pub const CG_AUDIO_F26M_CK_SFT: u32 = 1;
pub const CG_AUDIO_F26M_CK_MASK: u32 = 0x1;
pub const CG_AUDIO_F26M_CK_MASK_SFT: u32 = (0x1 << 1);
pub const CG_AUDIO_HOPPING_CK_SFT: u32 = 0;
pub const CG_AUDIO_HOPPING_CK_MASK: u32 = 0x1;
pub const CG_AUDIO_HOPPING_CK_MASK_SFT: u32 = (0x1 << 0);

/* AUDIO_ENGEN_CON0 */
/* AUDIO_ENGEN_CON0_USER1 */
/* AUDIO_ENGEN_CON0_USER1 */
pub const MULTI_USER_BYPASS_SFT: u32 = 17;
pub const MULTI_USER_BYPASS_MASK: u32 = 0x1;
pub const MULTI_USER_BYPASS_MASK_SFT: u32 = (0x1 << 17);
pub const MULTI_USER_RST_SFT: u32 = 16;
pub const MULTI_USER_RST_MASK: u32 = 0x1;
pub const MULTI_USER_RST_MASK_SFT: u32 = (0x1 << 16);
pub const AUDIO_F26M_EN_RST_SFT: u32 = 8;
pub const AUDIO_F26M_EN_RST_MASK: u32 = 0x1;
pub const AUDIO_F26M_EN_RST_MASK_SFT: u32 = (0x1 << 8);
pub const AUDIO_APLL2_EN_ON_SFT: u32 = 3;
pub const AUDIO_APLL2_EN_ON_MASK: u32 = 0x1;
pub const AUDIO_APLL2_EN_ON_MASK_SFT: u32 = (0x1 << 3);
pub const AUDIO_APLL1_EN_ON_SFT: u32 = 2;
pub const AUDIO_APLL1_EN_ON_MASK: u32 = 0x1;
pub const AUDIO_APLL1_EN_ON_MASK_SFT: u32 = (0x1 << 2);
pub const AUDIO_F3P25M_EN_ON_SFT: u32 = 1;
pub const AUDIO_F3P25M_EN_ON_MASK: u32 = 0x1;
pub const AUDIO_F3P25M_EN_ON_MASK_SFT: u32 = (0x1 << 1);
pub const AUDIO_26M_EN_ON_SFT: u32 = 0;
pub const AUDIO_26M_EN_ON_MASK: u32 = 0x1;
pub const AUDIO_26M_EN_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_SINEGEN_CON0 */
pub const DAC_EN_SFT: u32 = 26;
pub const DAC_EN_MASK: u32 = 0x1;
pub const DAC_EN_MASK_SFT: u32 = (0x1 << 26);
pub const TIE_SW_CH2_SFT: u32 = 25;
pub const TIE_SW_CH2_MASK: u32 = 0x1;
pub const TIE_SW_CH2_MASK_SFT: u32 = (0x1 << 25);
pub const TIE_SW_CH1_SFT: u32 = 24;
pub const TIE_SW_CH1_MASK: u32 = 0x1;
pub const TIE_SW_CH1_MASK_SFT: u32 = (0x1 << 24);
pub const AMP_DIV_CH2_SFT: u32 = 20;
pub const AMP_DIV_CH2_MASK: u32 = 0xf;
pub const AMP_DIV_CH2_MASK_SFT: u32 = (0xf << 20);
pub const FREQ_DIV_CH2_SFT: u32 = 12;
pub const FREQ_DIV_CH2_MASK: u32 = 0x1f;
pub const FREQ_DIV_CH2_MASK_SFT: u32 = (0x1f << 12);
pub const AMP_DIV_CH1_SFT: u32 = 8;
pub const AMP_DIV_CH1_MASK: u32 = 0xf;
pub const AMP_DIV_CH1_MASK_SFT: u32 = (0xf << 8);
pub const FREQ_DIV_CH1_SFT: u32 = 0;
pub const FREQ_DIV_CH1_MASK: u32 = 0x1f;
pub const FREQ_DIV_CH1_MASK_SFT: u32 = (0x1f << 0);

/* AFE_SINEGEN_CON1 */
pub const SINE_DOMAIN_SFT: u32 = 20;
pub const SINE_DOMAIN_MASK: u32 = 0x7;
pub const SINE_DOMAIN_MASK_SFT: u32 = (0x7 << 20);
pub const SINE_MODE_SFT: u32 = 12;
pub const SINE_MODE_MASK: u32 = 0x1f;
pub const SINE_MODE_MASK_SFT: u32 = (0x1f << 12);
pub const INNER_LOOP_BACKI_SEL_SFT: u32 = 8;
pub const INNER_LOOP_BACKI_SEL_MASK: u32 = 0x1;
pub const INNER_LOOP_BACKI_SEL_MASK_SFT: u32 = (0x1 << 8);
pub const INNER_LOOP_BACK_MODE_SFT: u32 = 0;
pub const INNER_LOOP_BACK_MODE_MASK: u32 = 0xff;
pub const INNER_LOOP_BACK_MODE_MASK_SFT: u32 = (0xff << 0);

/* AFE_SINEGEN_CON2 */
pub const TIE_CH1_CONSTANT_SFT: u32 = 0;
pub const TIE_CH1_CONSTANT_MASK: u32 = 0xffffffff;
pub const TIE_CH1_CONSTANT_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_SINEGEN_CON3 */
pub const TIE_CH2_CONSTANT_SFT: u32 = 0;
pub const TIE_CH2_CONSTANT_MASK: u32 = 0xffffffff;
pub const TIE_CH2_CONSTANT_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_APLL1_TUNER_CFG */
/* AFE_APLL2_TUNER_CFG */
pub const UPPER_BOUND_SFT: u32 = 8;
pub const UPPER_BOUND_MASK: u32 = 0xff;
pub const UPPER_BOUND_MASK_SFT: u32 = (0xff << 8);
pub const APLL_DIV_SFT: u32 = 4;
pub const APLL_DIV_MASK: u32 = 0xf;
pub const APLL_DIV_MASK_SFT: u32 = (0xf << 4);
pub const XTAL_EN_128FS_SEL_SFT: u32 = 1;
pub const XTAL_EN_128FS_SEL_MASK: u32 = 0x3;
pub const XTAL_EN_128FS_SEL_MASK_SFT: u32 = (0x3 << 1);
pub const FREQ_TUNER_EN_SFT: u32 = 0;
pub const FREQ_TUNER_EN_MASK: u32 = 0x1;
pub const FREQ_TUNER_EN_MASK_SFT: u32 = (0x1 << 0);

/* AFE_APLL1_TUNER_MON0 */
/* AFE_APLL2_TUNER_MON0 */
pub const TUNER_MON_SFT: u32 = 0;
pub const TUNER_MON_MASK: u32 = 0xffffffff;
pub const TUNER_MON_MASK_SFT: u32 = (0xffffffff << 0);

/* AUDIO_TOP_RG0 */
/* AUDIO_TOP_RG1 */
/* AUDIO_TOP_RG2 */
/* AUDIO_TOP_RG3 */
/* AUDIO_TOP_RG4 */
pub const RESERVE_RG_SFT: u32 = 0;
pub const RESERVE_RG_MASK: u32 = 0xffffffff;
pub const RESERVE_RG_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_SPM_CONTROL_REQ */
pub const AFE_DDREN_REQ_SFT: u32 = 4;
pub const AFE_DDREN_REQ_MASK: u32 = 0x1;
pub const AFE_DDREN_REQ_MASK_SFT: u32 = (0x1 << 4);
pub const AFE_INFRA_REQ_SFT: u32 = 3;
pub const AFE_INFRA_REQ_MASK: u32 = 0x1;
pub const AFE_INFRA_REQ_MASK_SFT: u32 = (0x1 << 3);
pub const AFE_VRF18_REQ_SFT: u32 = 2;
pub const AFE_VRF18_REQ_MASK: u32 = 0x1;
pub const AFE_VRF18_REQ_MASK_SFT: u32 = (0x1 << 2);
pub const AFE_APSRC_REQ_SFT: u32 = 1;
pub const AFE_APSRC_REQ_MASK: u32 = 0x1;
pub const AFE_APSRC_REQ_MASK_SFT: u32 = (0x1 << 1);
pub const AFE_SRCCLKENA_REQ_SFT: u32 = 0;
pub const AFE_SRCCLKENA_REQ_MASK: u32 = 0x1;
pub const AFE_SRCCLKENA_REQ_MASK_SFT: u32 = (0x1 << 0);

/* AFE_SPM_CONTROL_ACK */
pub const SPM_RESOURCE_CONTROL_ACK_SFT: u32 = 0;
pub const SPM_RESOURCE_CONTROL_ACK_MASK: u32 = 0xffffffff;
pub const SPM_RESOURCE_CONTROL_ACK_MASK_SFT: u32 = (0xffffffff << 0);

/* AUD_TOP_CFG_VCORE_RG */
pub const AUD_TOP_CFG_SFT: u32 = 0;
pub const AUD_TOP_CFG_MASK: u32 = 0xffffffff;
pub const AUD_TOP_CFG_MASK_SFT: u32 = (0xffffffff << 0);

/* AUDIO_TOP_IP_VERSION */
pub const AUDIO_TOP_IP_VERSION_SFT: u32 = 0;
pub const AUDIO_TOP_IP_VERSION_MASK: u32 = 0xffffffff;
pub const AUDIO_TOP_IP_VERSION_MASK_SFT: u32 = (0xffffffff << 0);

/* AUDIO_ENGEN_CON0_MON */
pub const AUDIO_ENGEN_MON_SFT: u32 = 0;
pub const AUDIO_ENGEN_MON_MASK: u32 = 0xffffffff;
pub const AUDIO_ENGEN_MON_MASK_SFT: u32 = (0xffffffff << 0);

/* AUD_TOP_CFG_VLP_RG */
pub const I2SIN1_DAT_SEL_SFT: u32 = 31;
pub const I2SIN1_DAT_SEL_MASK: u32 = 0x1;
pub const I2SIN1_DAT_SEL_MASK_SFT: u32 = (0x1 << 31);
pub const FMI2S_IN_SEL_SFT: u32 = 30;
pub const FMI2S_IN_SEL_MASK: u32 = 0x1;
pub const FMI2S_IN_SEL_MASK_SFT: u32 = (0x1 << 30);
pub const RG_I2S4_IN_BCK_NEG_EG_LATCH_SFT: u32 = 21;
pub const RG_I2S4_IN_BCK_NEG_EG_LATCH_MASK: u32 = 0x1;
pub const RG_I2S4_IN_BCK_NEG_EG_LATCH_MASK_SFT: u32 = (0x1 << 21);
pub const RG_I2S4_OUT_BCK_NEG_EG_LATCH_SFT: u32 = 20;
pub const RG_I2S4_OUT_BCK_NEG_EG_LATCH_MASK: u32 = 0x1;
pub const RG_I2S4_OUT_BCK_NEG_EG_LATCH_MASK_SFT: u32 = (0x1 << 20);
pub const RG_I2S4_IN_SLV_LRCK_LATCH_EDGE_SFT: u32 = 19;
pub const RG_I2S4_IN_SLV_LRCK_LATCH_EDGE_MASK: u32 = 0x1;
pub const RG_I2S4_IN_SLV_LRCK_LATCH_EDGE_MASK_SFT: u32 = (0x1 << 19);
pub const RG_I2S4_IN_SLV_BCK_INV_SEL_SFT: u32 = 18;
pub const RG_I2S4_IN_SLV_BCK_INV_SEL_MASK: u32 = 0x1;
pub const RG_I2S4_IN_SLV_BCK_INV_SEL_MASK_SFT: u32 = (0x1 << 18);
pub const RG_I2S4_OUT_SLV_LRCK_LATCH_EDGE_SFT: u32 = 17;
pub const RG_I2S4_OUT_SLV_LRCK_LATCH_EDGE_MASK: u32 = 0x1;
pub const RG_I2S4_OUT_SLV_LRCK_LATCH_EDGE_MASK_SFT: u32 = (0x1 << 17);
pub const RG_I2S4_OUT_SLV_BCK_INV_SEL_SFT: u32 = 16;
pub const RG_I2S4_OUT_SLV_BCK_INV_SEL_MASK: u32 = 0x1;
pub const RG_I2S4_OUT_SLV_BCK_INV_SEL_MASK_SFT: u32 = (0x1 << 16);
pub const RG_I2S5_IN_BCK_NEG_EG_LATCH_SFT: u32 = 13;
pub const RG_I2S5_IN_BCK_NEG_EG_LATCH_MASK: u32 = 0x1;
pub const RG_I2S5_IN_BCK_NEG_EG_LATCH_MASK_SFT: u32 = (0x1 << 13);
pub const RG_I2S5_OUT_BCK_NEG_EG_LATCH_SFT: u32 = 12;
pub const RG_I2S5_OUT_BCK_NEG_EG_LATCH_MASK: u32 = 0x1;
pub const RG_I2S5_OUT_BCK_NEG_EG_LATCH_MASK_SFT: u32 = (0x1 << 12);
pub const RG_I2S5_IN_SLV_LRCK_LATCH_EDGE_SFT: u32 = 11;
pub const RG_I2S5_IN_SLV_LRCK_LATCH_EDGE_MASK: u32 = 0x1;
pub const RG_I2S5_IN_SLV_LRCK_LATCH_EDGE_MASK_SFT: u32 = (0x1 << 11);
pub const RG_I2S5_IN_SLV_BCK_INV_SEL_SFT: u32 = 10;
pub const RG_I2S5_IN_SLV_BCK_INV_SEL_MASK: u32 = 0x1;
pub const RG_I2S5_IN_SLV_BCK_INV_SEL_MASK_SFT: u32 = (0x1 << 10);
pub const RG_I2S5_OUT_SLV_LRCK_LATCH_EDGE_SFT: u32 = 9;
pub const RG_I2S5_OUT_SLV_LRCK_LATCH_EDGE_MASK: u32 = 0x1;
pub const RG_I2S5_OUT_SLV_LRCK_LATCH_EDGE_MASK_SFT: u32 = (0x1 << 9);
pub const RG_I2S5_OUT_SLV_BCK_INV_SEL_SFT: u32 = 8;
pub const RG_I2S5_OUT_SLV_BCK_INV_SEL_MASK: u32 = 0x1;
pub const RG_I2S5_OUT_SLV_BCK_INV_SEL_MASK_SFT: u32 = (0x1 << 8);
pub const RG_I2S4_PAD_TOP_CK_EN_SFT: u32 = 5;
pub const RG_I2S4_PAD_TOP_CK_EN_MASK: u32 = 0x1;
pub const RG_I2S4_PAD_TOP_CK_EN_MASK_SFT: u32 = (0x1 << 5);
pub const RG_I2S5_PAD_TOP_CK_EN_SFT: u32 = 4;
pub const RG_I2S5_PAD_TOP_CK_EN_MASK: u32 = 0x1;
pub const RG_I2S5_PAD_TOP_CK_EN_MASK_SFT: u32 = (0x1 << 4);
pub const RG_TEST_TYPE_SFT: u32 = 2;
pub const RG_TEST_TYPE_MASK: u32 = 0x1;
pub const RG_TEST_TYPE_MASK_SFT: u32 = (0x1 << 2);
pub const RG_SW_RESET_SFT: u32 = 1;
pub const RG_SW_RESET_MASK: u32 = 0x1;
pub const RG_SW_RESET_MASK_SFT: u32 = (0x1 << 1);
pub const RG_TEST_ON_SFT: u32 = 0;
pub const RG_TEST_ON_MASK: u32 = 0x1;
pub const RG_TEST_ON_MASK_SFT: u32 = (0x1 << 0);

/* AUD_TOP_MON_RG */
pub const AUD_TOP_MON_SFT: u32 = 0;
pub const AUD_TOP_MON_MASK: u32 = 0xffffffff;
pub const AUD_TOP_MON_MASK_SFT: u32 = (0xffffffff << 0);

/* AUDIO_USE_DEFAULT_DELSEL0 */
pub const USE_DEFAULT_DELSEL_RG_SFT: u32 = 0;
pub const USE_DEFAULT_DELSEL_RG_MASK: u32 = 0xffffffff;
pub const USE_DEFAULT_DELSEL_RG_MASK_SFT: u32 = (0xffffffff << 0);

/* AUDIO_USE_DEFAULT_DELSEL1 */
// duplicate C define preserved: #define USE_DEFAULT_DELSEL_RG_SFT 0
// duplicate C define preserved: #define USE_DEFAULT_DELSEL_RG_MASK 0xffffffff
// duplicate C define preserved: #define USE_DEFAULT_DELSEL_RG_MASK_SFT (0xffffffff << 0)

/* AUDIO_USE_DEFAULT_DELSEL2 */
// duplicate C define preserved: #define USE_DEFAULT_DELSEL_RG_SFT 0
// duplicate C define preserved: #define USE_DEFAULT_DELSEL_RG_MASK 0xffffffff
// duplicate C define preserved: #define USE_DEFAULT_DELSEL_RG_MASK_SFT (0xffffffff << 0)

/* AFE_CONNSYS_I2S_IPM_VER_MON */
pub const RG_CONNSYS_I2S_IPM_VER_MON_SFT: u32 = 0;
pub const RG_CONNSYS_I2S_IPM_VER_MON_MASK: u32 = 0xffffffff;
pub const RG_CONNSYS_I2S_IPM_VER_MON_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_CONNSYS_I2S_MON_SEL */
pub const RG_CONNSYS_I2S_MON_SEL_SFT: u32 = 0;
pub const RG_CONNSYS_I2S_MON_SEL_MASK: u32 = 0xff;
pub const RG_CONNSYS_I2S_MON_SEL_MASK_SFT: u32 = (0xff << 0);

/* AFE_CONNSYS_I2S_MON */
pub const RG_CONNSYS_I2S_MON_SFT: u32 = 0;
pub const RG_CONNSYS_I2S_MON_MASK: u32 = 0xffffffff;
pub const RG_CONNSYS_I2S_MON_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_CONNSYS_I2S_CON */
pub const I2S_SOFT_RST_SFT: u32 = 31;
pub const I2S_SOFT_RST_MASK: u32 = 0x1;
pub const I2S_SOFT_RST_MASK_SFT: u32 = (0x1 << 31);
pub const BCK_NEG_EG_LATCH_SFT: u32 = 30;
pub const BCK_NEG_EG_LATCH_MASK: u32 = 0x1;
pub const BCK_NEG_EG_LATCH_MASK_SFT: u32 = (0x1 << 30);
pub const BCK_INV_SFT: u32 = 29;
pub const BCK_INV_MASK: u32 = 0x1;
pub const BCK_INV_MASK_SFT: u32 = (0x1 << 29);
pub const I2SIN_PAD_SEL_SFT: u32 = 28;
pub const I2SIN_PAD_SEL_MASK: u32 = 0x1;
pub const I2SIN_PAD_SEL_MASK_SFT: u32 = (0x1 << 28);
pub const I2S_LOOPBACK_SFT: u32 = 20;
pub const I2S_LOOPBACK_MASK: u32 = 0x1;
pub const I2S_LOOPBACK_MASK_SFT: u32 = (0x1 << 20);
pub const I2S_HDEN_SFT: u32 = 12;
pub const I2S_HDEN_MASK: u32 = 0x1;
pub const I2S_HDEN_MASK_SFT: u32 = (0x1 << 12);
pub const I2S_MODE_SFT: u32 = 8;
pub const I2S_MODE_MASK: u32 = 0xf;
pub const I2S_MODE_MASK_SFT: u32 = (0xf << 8);
pub const I2S_BYPSRC_SFT: u32 = 6;
pub const I2S_BYPSRC_MASK: u32 = 0x1;
pub const I2S_BYPSRC_MASK_SFT: u32 = (0x1 << 6);
pub const INV_LRCK_SFT: u32 = 5;
pub const INV_LRCK_MASK: u32 = 0x1;
pub const INV_LRCK_MASK_SFT: u32 = (0x1 << 5);
pub const I2S_FMT_SFT: u32 = 3;
pub const I2S_FMT_MASK: u32 = 0x1;
pub const I2S_FMT_MASK_SFT: u32 = (0x1 << 3);
pub const I2S_SRC_SFT: u32 = 2;
pub const I2S_SRC_MASK: u32 = 0x1;
pub const I2S_SRC_MASK_SFT: u32 = (0x1 << 2);
pub const I2S_WLEN_SFT: u32 = 1;
pub const I2S_WLEN_MASK: u32 = 0x1;
pub const I2S_WLEN_MASK_SFT: u32 = (0x1 << 1);
pub const I2S_EN_SFT: u32 = 0;
pub const I2S_EN_MASK: u32 = 0x1;
pub const I2S_EN_MASK_SFT: u32 = (0x1 << 0);

/* AFE_PCM0_INTF_CON0 */
pub const PCM0_HDEN_SFT: u32 = 26;
pub const PCM0_HDEN_MASK: u32 = 0x1;
pub const PCM0_HDEN_MASK_SFT: u32 = (0x1 << 26);
pub const PCM0_SYNC_DELSEL_SFT: u32 = 25;
pub const PCM0_SYNC_DELSEL_MASK: u32 = 0x1;
pub const PCM0_SYNC_DELSEL_MASK_SFT: u32 = (0x1 << 25);
pub const PCM0_TX_LR_SWAP_SFT: u32 = 24;
pub const PCM0_TX_LR_SWAP_MASK: u32 = 0x1;
pub const PCM0_TX_LR_SWAP_MASK_SFT: u32 = (0x1 << 24);
pub const PCM0_SYNC_OUT_INV_SFT: u32 = 23;
pub const PCM0_SYNC_OUT_INV_MASK: u32 = 0x1;
pub const PCM0_SYNC_OUT_INV_MASK_SFT: u32 = (0x1 << 23);
pub const PCM0_BCLK_OUT_INV_SFT: u32 = 22;
pub const PCM0_BCLK_OUT_INV_MASK: u32 = 0x1;
pub const PCM0_BCLK_OUT_INV_MASK_SFT: u32 = (0x1 << 22);
pub const PCM0_SYNC_IN_INV_SFT: u32 = 21;
pub const PCM0_SYNC_IN_INV_MASK: u32 = 0x1;
pub const PCM0_SYNC_IN_INV_MASK_SFT: u32 = (0x1 << 21);
pub const PCM0_BCLK_IN_INV_SFT: u32 = 20;
pub const PCM0_BCLK_IN_INV_MASK: u32 = 0x1;
pub const PCM0_BCLK_IN_INV_MASK_SFT: u32 = (0x1 << 20);
pub const PCM0_TX_LCH_RPT_SFT: u32 = 19;
pub const PCM0_TX_LCH_RPT_MASK: u32 = 0x1;
pub const PCM0_TX_LCH_RPT_MASK_SFT: u32 = (0x1 << 19);
pub const PCM0_VBT_16K_MODE_SFT: u32 = 18;
pub const PCM0_VBT_16K_MODE_MASK: u32 = 0x1;
pub const PCM0_VBT_16K_MODE_MASK_SFT: u32 = (0x1 << 18);
pub const PCM0_BIT_LENGTH_SFT: u32 = 16;
pub const PCM0_BIT_LENGTH_MASK: u32 = 0x3;
pub const PCM0_BIT_LENGTH_MASK_SFT: u32 = (0x3 << 16);
pub const PCM0_WLEN_SFT: u32 = 14;
pub const PCM0_WLEN_MASK: u32 = 0x3;
pub const PCM0_WLEN_MASK_SFT: u32 = (0x3 << 14);
pub const PCM0_SYNC_LENGTH_SFT: u32 = 9;
pub const PCM0_SYNC_LENGTH_MASK: u32 = 0x1f;
pub const PCM0_SYNC_LENGTH_MASK_SFT: u32 = (0x1f << 9);
pub const PCM0_SYNC_TYPE_SFT: u32 = 8;
pub const PCM0_SYNC_TYPE_MASK: u32 = 0x1;
pub const PCM0_SYNC_TYPE_MASK_SFT: u32 = (0x1 << 8);
pub const PCM0_BYP_ASRC_SFT: u32 = 7;
pub const PCM0_BYP_ASRC_MASK: u32 = 0x1;
pub const PCM0_BYP_ASRC_MASK_SFT: u32 = (0x1 << 7);
pub const PCM0_SLAVE_SFT: u32 = 6;
pub const PCM0_SLAVE_MASK: u32 = 0x1;
pub const PCM0_SLAVE_MASK_SFT: u32 = (0x1 << 6);
pub const PCM0_MODE_SFT: u32 = 3;
pub const PCM0_MODE_MASK: u32 = 0x7;
pub const PCM0_MODE_MASK_SFT: u32 = (0x7 << 3);
pub const PCM0_FMT_SFT: u32 = 1;
pub const PCM0_FMT_MASK: u32 = 0x3;
pub const PCM0_FMT_MASK_SFT: u32 = (0x3 << 1);
pub const PCM0_EN_SFT: u32 = 0;
pub const PCM0_EN_MASK: u32 = 0x1;
pub const PCM0_EN_MASK_SFT: u32 = (0x1 << 0);

/* AFE_PCM0_INTF_CON1 */
pub const PCM0_TX_RX_LOOPBACK_SFT: u32 = 31;
pub const PCM0_TX_RX_LOOPBACK_MASK: u32 = 0x1;
pub const PCM0_TX_RX_LOOPBACK_MASK_SFT: u32 = (0x1 << 31);
pub const PCM0_BUFFER_LOOPBACK_SFT: u32 = 30;
pub const PCM0_BUFFER_LOOPBACK_MASK: u32 = 0x1;
pub const PCM0_BUFFER_LOOPBACK_MASK_SFT: u32 = (0x1 << 30);
pub const PCM0_PARALLEL_LOOPBACK_SFT: u32 = 29;
pub const PCM0_PARALLEL_LOOPBACK_MASK: u32 = 0x1;
pub const PCM0_PARALLEL_LOOPBACK_MASK_SFT: u32 = (0x1 << 29);
pub const PCM0_SERIAL_LOOPBACK_SFT: u32 = 28;
pub const PCM0_SERIAL_LOOPBACK_MASK: u32 = 0x1;
pub const PCM0_SERIAL_LOOPBACK_MASK_SFT: u32 = (0x1 << 28);
pub const PCM0_DAI_LOOPBACK_SFT: u32 = 27;
pub const PCM0_DAI_LOOPBACK_MASK: u32 = 0x1;
pub const PCM0_DAI_LOOPBACK_MASK_SFT: u32 = (0x1 << 27);
pub const PCM0_I2S_LOOPBACK_SFT: u32 = 26;
pub const PCM0_I2S_LOOPBACK_MASK: u32 = 0x1;
pub const PCM0_I2S_LOOPBACK_MASK_SFT: u32 = (0x1 << 26);
pub const PCM0_1X_EN_DOMAIN_SFT: u32 = 23;
pub const PCM0_1X_EN_DOMAIN_MASK: u32 = 0x7;
pub const PCM0_1X_EN_DOMAIN_MASK_SFT: u32 = (0x7 << 23);
pub const PCM0_1X_EN_MODE_SFT: u32 = 18;
pub const PCM0_1X_EN_MODE_MASK: u32 = 0x1f;
pub const PCM0_1X_EN_MODE_MASK_SFT: u32 = (0x1f << 18);
pub const PCM0_TX3_RCH_DBG_MODE_SFT: u32 = 17;
pub const PCM0_TX3_RCH_DBG_MODE_MASK: u32 = 0x1;
pub const PCM0_TX3_RCH_DBG_MODE_MASK_SFT: u32 = (0x1 << 17);
pub const PCM0_PCM1_LOOPBACK_SFT: u32 = 16;
pub const PCM0_PCM1_LOOPBACK_MASK: u32 = 0x1;
pub const PCM0_PCM1_LOOPBACK_MASK_SFT: u32 = (0x1 << 16);
pub const PCM0_LOOPBACK_CH_SEL_SFT: u32 = 12;
pub const PCM0_LOOPBACK_CH_SEL_MASK: u32 = 0x3;
pub const PCM0_LOOPBACK_CH_SEL_MASK_SFT: u32 = (0x3 << 12);
pub const PCM0_BT_MODE_SFT: u32 = 11;
pub const PCM0_BT_MODE_MASK: u32 = 0x1;
pub const PCM0_BT_MODE_MASK_SFT: u32 = (0x1 << 11);
pub const PCM0_EXT_MODEM_SFT: u32 = 10;
pub const PCM0_EXT_MODEM_MASK: u32 = 0x1;
pub const PCM0_EXT_MODEM_MASK_SFT: u32 = (0x1 << 10);
pub const PCM0_USE_MD3_SFT: u32 = 9;
pub const PCM0_USE_MD3_MASK: u32 = 0x1;
pub const PCM0_USE_MD3_MASK_SFT: u32 = (0x1 << 9);
pub const PCM0_FIX_VALUE_SEL_SFT: u32 = 8;
pub const PCM0_FIX_VALUE_SEL_MASK: u32 = 0x1;
pub const PCM0_FIX_VALUE_SEL_MASK_SFT: u32 = (0x1 << 8);
pub const PCM0_TX_FIX_VALUE_SFT: u32 = 0;
pub const PCM0_TX_FIX_VALUE_MASK: u32 = 0xff;
pub const PCM0_TX_FIX_VALUE_MASK_SFT: u32 = (0xff << 0);

/* AFE_PCM_INTF_MON */
pub const PCM0_TX_FIFO_OV_SFT: u32 = 5;
pub const PCM0_TX_FIFO_OV_MASK: u32 = 0x1;
pub const PCM0_TX_FIFO_OV_MASK_SFT: u32 = (0x1 << 5);
pub const PCM0_RX_FIFO_OV_SFT: u32 = 4;
pub const PCM0_RX_FIFO_OV_MASK: u32 = 0x1;
pub const PCM0_RX_FIFO_OV_MASK_SFT: u32 = (0x1 << 4);
pub const PCM1_TX_FIFO_OV_SFT: u32 = 3;
pub const PCM1_TX_FIFO_OV_MASK: u32 = 0x1;
pub const PCM1_TX_FIFO_OV_MASK_SFT: u32 = (0x1 << 3);
pub const PCM1_RX_FIFO_OV_SFT: u32 = 2;
pub const PCM1_RX_FIFO_OV_MASK: u32 = 0x1;
pub const PCM1_RX_FIFO_OV_MASK_SFT: u32 = (0x1 << 2);
pub const PCM0_SYNC_GLITCH_SFT: u32 = 1;
pub const PCM0_SYNC_GLITCH_MASK: u32 = 0x1;
pub const PCM0_SYNC_GLITCH_MASK_SFT: u32 = (0x1 << 1);
pub const PCM1_SYNC_GLITCH_SFT: u32 = 0;
pub const PCM1_SYNC_GLITCH_MASK: u32 = 0x1;
pub const PCM1_SYNC_GLITCH_MASK_SFT: u32 = (0x1 << 0);

/* AFE_PCM1_INTF_CON0 */
pub const PCM1_TX_FIX_VALUE_SFT: u32 = 24;
pub const PCM1_TX_FIX_VALUE_MASK: u32 = 0xff;
pub const PCM1_TX_FIX_VALUE_MASK_SFT: u32 = (0xff << 24);
pub const PCM1_FIX_VALUE_SEL_SFT: u32 = 23;
pub const PCM1_FIX_VALUE_SEL_MASK: u32 = 0x1;
pub const PCM1_FIX_VALUE_SEL_MASK_SFT: u32 = (0x1 << 23);
pub const PCM1_BUFFER_LOOPBACK_SFT: u32 = 22;
pub const PCM1_BUFFER_LOOPBACK_MASK: u32 = 0x1;
pub const PCM1_BUFFER_LOOPBACK_MASK_SFT: u32 = (0x1 << 22);
pub const PCM1_PARALLEL_LOOPBACK_SFT: u32 = 21;
pub const PCM1_PARALLEL_LOOPBACK_MASK: u32 = 0x1;
pub const PCM1_PARALLEL_LOOPBACK_MASK_SFT: u32 = (0x1 << 21);
pub const PCM1_SERIAL_LOOPBACK_SFT: u32 = 20;
pub const PCM1_SERIAL_LOOPBACK_MASK: u32 = 0x1;
pub const PCM1_SERIAL_LOOPBACK_MASK_SFT: u32 = (0x1 << 20);
pub const PCM1_DAI_PCM1_LOOPBACK_SFT: u32 = 19;
pub const PCM1_DAI_PCM1_LOOPBACK_MASK: u32 = 0x1;
pub const PCM1_DAI_PCM1_LOOPBACK_MASK_SFT: u32 = (0x1 << 19);
pub const PCM1_I2S_PCM1_LOOPBACK_SFT: u32 = 18;
pub const PCM1_I2S_PCM1_LOOPBACK_MASK: u32 = 0x1;
pub const PCM1_I2S_PCM1_LOOPBACK_MASK_SFT: u32 = (0x1 << 18);
pub const PCM1_SYNC_DELSEL_SFT: u32 = 17;
pub const PCM1_SYNC_DELSEL_MASK: u32 = 0x1;
pub const PCM1_SYNC_DELSEL_MASK_SFT: u32 = (0x1 << 17);
pub const PCM1_TX_LR_SWAP_SFT: u32 = 16;
pub const PCM1_TX_LR_SWAP_MASK: u32 = 0x1;
pub const PCM1_TX_LR_SWAP_MASK_SFT: u32 = (0x1 << 16);
pub const PCM1_SYNC_IN_INV_SFT: u32 = 15;
pub const PCM1_SYNC_IN_INV_MASK: u32 = 0x1;
pub const PCM1_SYNC_IN_INV_MASK_SFT: u32 = (0x1 << 15);
pub const PCM1_BCLK_IN_INV_SFT: u32 = 14;
pub const PCM1_BCLK_IN_INV_MASK: u32 = 0x1;
pub const PCM1_BCLK_IN_INV_MASK_SFT: u32 = (0x1 << 14);
pub const PCM1_TX_LCH_RPT_SFT: u32 = 13;
pub const PCM1_TX_LCH_RPT_MASK: u32 = 0x1;
pub const PCM1_TX_LCH_RPT_MASK_SFT: u32 = (0x1 << 13);
pub const PCM1_VBT_16K_MODE_SFT: u32 = 12;
pub const PCM1_VBT_16K_MODE_MASK: u32 = 0x1;
pub const PCM1_VBT_16K_MODE_MASK_SFT: u32 = (0x1 << 12);
pub const PCM1_LOOPBACK_CH_SEL_SFT: u32 = 10;
pub const PCM1_LOOPBACK_CH_SEL_MASK: u32 = 0x3;
pub const PCM1_LOOPBACK_CH_SEL_MASK_SFT: u32 = (0x3 << 10);
pub const PCM1_TX2_BT_MODE_SFT: u32 = 8;
pub const PCM1_TX2_BT_MODE_MASK: u32 = 0x1;
pub const PCM1_TX2_BT_MODE_MASK_SFT: u32 = (0x1 << 8);
pub const PCM1_BT_MODE_SFT: u32 = 7;
pub const PCM1_BT_MODE_MASK: u32 = 0x1;
pub const PCM1_BT_MODE_MASK_SFT: u32 = (0x1 << 7);
pub const PCM1_AFIFO_SFT: u32 = 6;
pub const PCM1_AFIFO_MASK: u32 = 0x1;
pub const PCM1_AFIFO_MASK_SFT: u32 = (0x1 << 6);
pub const PCM1_WLEN_SFT: u32 = 5;
pub const PCM1_WLEN_MASK: u32 = 0x1;
pub const PCM1_WLEN_MASK_SFT: u32 = (0x1 << 5);
pub const PCM1_MODE_SFT: u32 = 3;
pub const PCM1_MODE_MASK: u32 = 0x3;
pub const PCM1_MODE_MASK_SFT: u32 = (0x3 << 3);
pub const PCM1_FMT_SFT: u32 = 1;
pub const PCM1_FMT_MASK: u32 = 0x3;
pub const PCM1_FMT_MASK_SFT: u32 = (0x3 << 1);
pub const PCM1_EN_SFT: u32 = 0;
pub const PCM1_EN_MASK: u32 = 0x1;
pub const PCM1_EN_MASK_SFT: u32 = (0x1 << 0);

/* AFE_PCM1_INTF_CON1 */
pub const PCM1_1X_EN_DOMAIN_SFT: u32 = 23;
pub const PCM1_1X_EN_DOMAIN_MASK: u32 = 0x7;
pub const PCM1_1X_EN_DOMAIN_MASK_SFT: u32 = (0x7 << 23);
pub const PCM1_1X_EN_MODE_SFT: u32 = 18;
pub const PCM1_1X_EN_MODE_MASK: u32 = 0x1f;
pub const PCM1_1X_EN_MODE_MASK_SFT: u32 = (0x1f << 18);

/* AFE_PCM_TOP_IP_VERSION */
pub const AFE_PCM_TOP_IP_VERSION_SFT: u32 = 0;
pub const AFE_PCM_TOP_IP_VERSION_MASK: u32 = 0xffffffff;
pub const AFE_PCM_TOP_IP_VERSION_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_IRQ_MCU_EN */
pub const AFE_IRQ_MCU_EN_SFT: u32 = 0;
pub const AFE_IRQ_MCU_EN_MASK: u32 = 0xffffffff;
pub const AFE_IRQ_MCU_EN_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_IRQ_MCU_DSP_EN */
pub const AFE_IRQ_DSP_EN_SFT: u32 = 0;
pub const AFE_IRQ_DSP_EN_MASK: u32 = 0xffffffff;
pub const AFE_IRQ_DSP_EN_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_IRQ_MCU_DSP2_EN */
pub const AFE_IRQ_DSP2_EN_SFT: u32 = 0;
pub const AFE_IRQ_DSP2_EN_MASK: u32 = 0xffffffff;
pub const AFE_IRQ_DSP2_EN_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_IRQ_MCU_SCP_EN */
pub const IRQ31_MCU_SCP_EN_SFT: u32 = 31;
pub const IRQ30_MCU_SCP_EN_SFT: u32 = 30;
pub const IRQ29_MCU_SCP_EN_SFT: u32 = 29;
pub const IRQ28_MCU_SCP_EN_SFT: u32 = 28;
pub const IRQ27_MCU_SCP_EN_SFT: u32 = 27;
pub const IRQ26_MCU_SCP_EN_SFT: u32 = 26;
pub const IRQ25_MCU_SCP_EN_SFT: u32 = 25;
pub const IRQ24_MCU_SCP_EN_SFT: u32 = 24;
pub const IRQ23_MCU_SCP_EN_SFT: u32 = 23;
pub const IRQ22_MCU_SCP_EN_SFT: u32 = 22;
pub const IRQ21_MCU_SCP_EN_SFT: u32 = 21;
pub const IRQ20_MCU_SCP_EN_SFT: u32 = 20;
pub const IRQ19_MCU_SCP_EN_SFT: u32 = 19;
pub const IRQ18_MCU_SCP_EN_SFT: u32 = 18;
pub const IRQ17_MCU_SCP_EN_SFT: u32 = 17;
pub const IRQ16_MCU_SCP_EN_SFT: u32 = 16;
pub const IRQ15_MCU_SCP_EN_SFT: u32 = 15;
pub const IRQ14_MCU_SCP_EN_SFT: u32 = 14;
pub const IRQ13_MCU_SCP_EN_SFT: u32 = 13;
pub const IRQ12_MCU_SCP_EN_SFT: u32 = 12;
pub const IRQ11_MCU_SCP_EN_SFT: u32 = 11;
pub const IRQ10_MCU_SCP_EN_SFT: u32 = 10;
pub const IRQ9_MCU_SCP_EN_SFT: u32 = 9;
pub const IRQ8_MCU_SCP_EN_SFT: u32 = 8;
pub const IRQ7_MCU_SCP_EN_SFT: u32 = 7;
pub const IRQ6_MCU_SCP_EN_SFT: u32 = 6;
pub const IRQ5_MCU_SCP_EN_SFT: u32 = 5;
pub const IRQ4_MCU_SCP_EN_SFT: u32 = 4;
pub const IRQ3_MCU_SCP_EN_SFT: u32 = 3;
pub const IRQ2_MCU_SCP_EN_SFT: u32 = 2;
pub const IRQ1_MCU_SCP_EN_SFT: u32 = 1;
pub const IRQ0_MCU_SCP_EN_SFT: u32 = 0;

/* AFE_CUSTOM_IRQ_MCU_EN */
pub const AFE_CUSTOM_IRQ_MCU_EN_SFT: u32 = 0;
pub const AFE_CUSTOM_IRQ_MCU_EN_MASK: u32 = 0xffffffff;
pub const AFE_CUSTOM_IRQ_MCU_EN_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_CUSTOM_IRQ_MCU_DSP_EN */
pub const AFE_CUSTOM_IRQ_DSP_EN_SFT: u32 = 0;
pub const AFE_CUSTOM_IRQ_DSP_EN_MASK: u32 = 0xffffffff;
pub const AFE_CUSTOM_IRQ_DSP_EN_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_CUSTOM_IRQ_MCU_DSP2_EN */
pub const AFE_CUSTOM_IRQ_DSP2_EN_SFT: u32 = 0;
pub const AFE_CUSTOM_IRQ_DSP2_EN_MASK: u32 = 0xffffffff;
pub const AFE_CUSTOM_IRQ_DSP2_EN_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_CUSTOM_IRQ_MCU_SCP_EN */
pub const AFE_CUSTOM_IRQ_SCP_EN_SFT: u32 = 0;
pub const AFE_CUSTOM_IRQ_SCP_EN_MASK: u32 = 0xffffffff;
pub const AFE_CUSTOM_IRQ_SCP_EN_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_IRQ_MCU_STATUS */
pub const IRQ26_MCU_SFT: u32 = 26;
pub const IRQ26_MCU_MASK: u32 = 0x1;
pub const IRQ26_MCU_MASK_SFT: u32 = (0x1 << 26);
pub const IRQ25_MCU_SFT: u32 = 25;
pub const IRQ25_MCU_MASK: u32 = 0x1;
pub const IRQ25_MCU_MASK_SFT: u32 = (0x1 << 25);
pub const IRQ24_MCU_SFT: u32 = 24;
pub const IRQ24_MCU_MASK: u32 = 0x1;
pub const IRQ24_MCU_MASK_SFT: u32 = (0x1 << 24);
pub const IRQ23_MCU_SFT: u32 = 23;
pub const IRQ23_MCU_MASK: u32 = 0x1;
pub const IRQ23_MCU_MASK_SFT: u32 = (0x1 << 23);
pub const IRQ22_MCU_SFT: u32 = 22;
pub const IRQ22_MCU_MASK: u32 = 0x1;
pub const IRQ22_MCU_MASK_SFT: u32 = (0x1 << 22);
pub const IRQ21_MCU_SFT: u32 = 21;
pub const IRQ21_MCU_MASK: u32 = 0x1;
pub const IRQ21_MCU_MASK_SFT: u32 = (0x1 << 21);
pub const IRQ20_MCU_SFT: u32 = 20;
pub const IRQ20_MCU_MASK: u32 = 0x1;
pub const IRQ20_MCU_MASK_SFT: u32 = (0x1 << 20);
pub const IRQ19_MCU_SFT: u32 = 19;
pub const IRQ19_MCU_MASK: u32 = 0x1;
pub const IRQ19_MCU_MASK_SFT: u32 = (0x1 << 19);
pub const IRQ18_MCU_SFT: u32 = 18;
pub const IRQ18_MCU_MASK: u32 = 0x1;
pub const IRQ18_MCU_MASK_SFT: u32 = (0x1 << 18);
pub const IRQ17_MCU_SFT: u32 = 17;
pub const IRQ17_MCU_MASK: u32 = 0x1;
pub const IRQ17_MCU_MASK_SFT: u32 = (0x1 << 17);
pub const IRQ16_MCU_SFT: u32 = 16;
pub const IRQ16_MCU_MASK: u32 = 0x1;
pub const IRQ16_MCU_MASK_SFT: u32 = (0x1 << 16);
pub const IRQ15_MCU_SFT: u32 = 15;
pub const IRQ15_MCU_MASK: u32 = 0x1;
pub const IRQ15_MCU_MASK_SFT: u32 = (0x1 << 15);
pub const IRQ14_MCU_SFT: u32 = 14;
pub const IRQ14_MCU_MASK: u32 = 0x1;
pub const IRQ14_MCU_MASK_SFT: u32 = (0x1 << 14);
pub const IRQ13_MCU_SFT: u32 = 13;
pub const IRQ13_MCU_MASK: u32 = 0x1;
pub const IRQ13_MCU_MASK_SFT: u32 = (0x1 << 13);
pub const IRQ12_MCU_SFT: u32 = 12;
pub const IRQ12_MCU_MASK: u32 = 0x1;
pub const IRQ12_MCU_MASK_SFT: u32 = (0x1 << 12);
pub const IRQ11_MCU_SFT: u32 = 11;
pub const IRQ11_MCU_MASK: u32 = 0x1;
pub const IRQ11_MCU_MASK_SFT: u32 = (0x1 << 11);
pub const IRQ10_MCU_SFT: u32 = 10;
pub const IRQ10_MCU_MASK: u32 = 0x1;
pub const IRQ10_MCU_MASK_SFT: u32 = (0x1 << 10);
pub const IRQ9_MCU_SFT: u32 = 9;
pub const IRQ9_MCU_MASK: u32 = 0x1;
pub const IRQ9_MCU_MASK_SFT: u32 = (0x1 << 9);
pub const IRQ8_MCU_SFT: u32 = 8;
pub const IRQ8_MCU_MASK: u32 = 0x1;
pub const IRQ8_MCU_MASK_SFT: u32 = (0x1 << 8);
pub const IRQ7_MCU_SFT: u32 = 7;
pub const IRQ7_MCU_MASK: u32 = 0x1;
pub const IRQ7_MCU_MASK_SFT: u32 = (0x1 << 7);
pub const IRQ6_MCU_SFT: u32 = 6;
pub const IRQ6_MCU_MASK: u32 = 0x1;
pub const IRQ6_MCU_MASK_SFT: u32 = (0x1 << 6);
pub const IRQ5_MCU_SFT: u32 = 5;
pub const IRQ5_MCU_MASK: u32 = 0x1;
pub const IRQ5_MCU_MASK_SFT: u32 = (0x1 << 5);
pub const IRQ4_MCU_SFT: u32 = 4;
pub const IRQ4_MCU_MASK: u32 = 0x1;
pub const IRQ4_MCU_MASK_SFT: u32 = (0x1 << 4);
pub const IRQ3_MCU_SFT: u32 = 3;
pub const IRQ3_MCU_MASK: u32 = 0x1;
pub const IRQ3_MCU_MASK_SFT: u32 = (0x1 << 3);
pub const IRQ2_MCU_SFT: u32 = 2;
pub const IRQ2_MCU_MASK: u32 = 0x1;
pub const IRQ2_MCU_MASK_SFT: u32 = (0x1 << 2);
pub const IRQ1_MCU_SFT: u32 = 1;
pub const IRQ1_MCU_MASK: u32 = 0x1;
pub const IRQ1_MCU_MASK_SFT: u32 = (0x1 << 1);
pub const IRQ0_MCU_SFT: u32 = 0;
pub const IRQ0_MCU_MASK: u32 = 0x1;
pub const IRQ0_MCU_MASK_SFT: u32 = (0x1 << 0);

/* AFE_CUSTOM_IRQ_MCU_STATUS */
pub const CUSTOM_IRQ21_MCU_SFT: u32 = 21;
pub const CUSTOM_IRQ21_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ21_MCU_MASK_SFT: u32 = (0x1 << 21);
pub const CUSTOM_IRQ20_MCU_SFT: u32 = 20;
pub const CUSTOM_IRQ20_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ20_MCU_MASK_SFT: u32 = (0x1 << 20);
pub const CUSTOM_IRQ19_MCU_SFT: u32 = 19;
pub const CUSTOM_IRQ19_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ19_MCU_MASK_SFT: u32 = (0x1 << 19);
pub const CUSTOM_IRQ18_MCU_SFT: u32 = 18;
pub const CUSTOM_IRQ18_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ18_MCU_MASK_SFT: u32 = (0x1 << 18);
pub const CUSTOM_IRQ17_MCU_SFT: u32 = 17;
pub const CUSTOM_IRQ17_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ17_MCU_MASK_SFT: u32 = (0x1 << 17);
pub const CUSTOM_IRQ16_MCU_SFT: u32 = 16;
pub const CUSTOM_IRQ16_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ16_MCU_MASK_SFT: u32 = (0x1 << 16);
pub const CUSTOM_IRQ9_MCU_SFT: u32 = 9;
pub const CUSTOM_IRQ9_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ9_MCU_MASK_SFT: u32 = (0x1 << 9);
pub const CUSTOM_IRQ8_MCU_SFT: u32 = 8;
pub const CUSTOM_IRQ8_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ8_MCU_MASK_SFT: u32 = (0x1 << 8);
pub const CUSTOM_IRQ7_MCU_SFT: u32 = 7;
pub const CUSTOM_IRQ7_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ7_MCU_MASK_SFT: u32 = (0x1 << 7);
pub const CUSTOM_IRQ6_MCU_SFT: u32 = 6;
pub const CUSTOM_IRQ6_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ6_MCU_MASK_SFT: u32 = (0x1 << 6);
pub const CUSTOM_IRQ5_MCU_SFT: u32 = 5;
pub const CUSTOM_IRQ5_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ5_MCU_MASK_SFT: u32 = (0x1 << 5);
pub const CUSTOM_IRQ4_MCU_SFT: u32 = 4;
pub const CUSTOM_IRQ4_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ4_MCU_MASK_SFT: u32 = (0x1 << 4);
pub const CUSTOM_IRQ3_MCU_SFT: u32 = 3;
pub const CUSTOM_IRQ3_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ3_MCU_MASK_SFT: u32 = (0x1 << 3);
pub const CUSTOM_IRQ2_MCU_SFT: u32 = 2;
pub const CUSTOM_IRQ2_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ2_MCU_MASK_SFT: u32 = (0x1 << 2);
pub const CUSTOM_IRQ1_MCU_SFT: u32 = 1;
pub const CUSTOM_IRQ1_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ1_MCU_MASK_SFT: u32 = (0x1 << 1);
pub const CUSTOM_IRQ0_MCU_SFT: u32 = 0;
pub const CUSTOM_IRQ0_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ0_MCU_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ_MCU_CFG */
pub const AFE_IRQ_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ0_MCU_CFG0 */
pub const AFE_IRQ0_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ0_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ0_MCU_DOMAIN_MASK_SFT: u32 = (0x7 << 9);
pub const AFE_IRQ0_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ0_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ0_MCU_FS_MASK_SFT: u32 = (0x1f << 4);
pub const AFE_IRQ0_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ0_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ0_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ0_MCU_CFG1 */
pub const AFE_IRQ0_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ0_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ0_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ0_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ0_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ0_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ0_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ0_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ0_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ1_MCU_CFG0 */
pub const AFE_IRQ1_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ1_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ1_MCU_DOMAIN_MASK_SFT: u32 = (0x7 << 9);
pub const AFE_IRQ1_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ1_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ1_MCU_FS_MASK_SFT: u32 = (0x1f << 4);
pub const AFE_IRQ1_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ1_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ1_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ1_MCU_CFG1 */
pub const AFE_IRQ1_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ1_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ1_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ1_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ1_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ1_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ1_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ1_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ1_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ2_MCU_CFG0 */
pub const AFE_IRQ2_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ2_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ2_MCU_DOMAIN_MASK_SFT: u32 = (0x7 << 9);
pub const AFE_IRQ2_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ2_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ2_MCU_FS_MASK_SFT: u32 = (0x1f << 4);
pub const AFE_IRQ2_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ2_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ2_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ2_MCU_CFG1 */
pub const AFE_IRQ2_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ2_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ2_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ2_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ2_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ2_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ2_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ2_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ2_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ3_MCU_CFG0 */
pub const AFE_IRQ3_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ3_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ3_MCU_DOMAIN_MASK_SFT: u32 = (0x7 << 9);
pub const AFE_IRQ3_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ3_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ3_MCU_FS_MASK_SFT: u32 = (0x1f << 4);
pub const AFE_IRQ3_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ3_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ3_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ3_MCU_CFG1 */
pub const AFE_IRQ3_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ3_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ3_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ3_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ3_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ3_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ3_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ3_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ3_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ4_MCU_CFG0 */
pub const AFE_IRQ4_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ4_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ4_MCU_DOMAIN_MASK_SFT: u32 = (0x7 << 9);
pub const AFE_IRQ4_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ4_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ4_MCU_FS_MASK_SFT: u32 = (0x1f << 4);
pub const AFE_IRQ4_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ4_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ4_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ4_MCU_CFG1 */
pub const AFE_IRQ4_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ4_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ4_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ4_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ4_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ4_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ4_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ4_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ4_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ5_MCU_CFG0 */
pub const AFE_IRQ5_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ5_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ5_MCU_DOMAIN_MASK_SFT: u32 = (0x7 << 9);
pub const AFE_IRQ5_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ5_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ5_MCU_FS_MASK_SFT: u32 = (0x1f << 4);
pub const AFE_IRQ5_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ5_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ5_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ5_MCU_CFG1 */
pub const AFE_IRQ5_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ5_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ5_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ5_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ5_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ5_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ5_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ5_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ5_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ6_MCU_CFG0 */
pub const AFE_IRQ6_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ6_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ6_MCU_DOMAIN_MASK_SFT: u32 = (0x7 << 9);
pub const AFE_IRQ6_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ6_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ6_MCU_FS_MASK_SFT: u32 = (0x1f << 4);
pub const AFE_IRQ6_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ6_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ6_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ6_MCU_CFG1 */
pub const AFE_IRQ6_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ6_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ6_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ6_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ6_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ6_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ6_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ6_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ6_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ7_MCU_CFG0 */
pub const AFE_IRQ7_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ7_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ7_MCU_DOMAIN_MASK_SFT: u32 = (0x7 << 9);
pub const AFE_IRQ7_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ7_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ7_MCU_FS_MASK_SFT: u32 = (0x1f << 4);
pub const AFE_IRQ7_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ7_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ7_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ7_MCU_CFG1 */
pub const AFE_IRQ7_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ7_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ7_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ7_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ7_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ7_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ7_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ7_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ7_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ8_MCU_CFG0 */
pub const AFE_IRQ8_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ8_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ8_MCU_DOMAIN_MASK_SFT: u32 = (0x7 << 9);
pub const AFE_IRQ8_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ8_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ8_MCU_FS_MASK_SFT: u32 = (0x1f << 4);
pub const AFE_IRQ8_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ8_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ8_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ8_MCU_CFG1 */
pub const AFE_IRQ8_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ8_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ8_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ8_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ8_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ8_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ8_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ8_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ8_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ9_MCU_CFG0 */
pub const AFE_IRQ9_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ9_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ9_MCU_DOMAIN_MASK_SFT: u32 = (0x7 << 9);
pub const AFE_IRQ9_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ9_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ9_MCU_FS_MASK_SFT: u32 = (0x1f << 4);
pub const AFE_IRQ9_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ9_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ9_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ9_MCU_CFG1 */
pub const AFE_IRQ9_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ9_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ9_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ9_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ9_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ9_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ9_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ9_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ9_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ10_MCU_CFG0 */
pub const AFE_IRQ10_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ10_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ10_MCU_DOMAIN_MASK_SFT: u32 = (0x7 << 9);
pub const AFE_IRQ10_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ10_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ10_MCU_FS_MASK_SFT: u32 = (0x1f << 4);
pub const AFE_IRQ10_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ10_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ10_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ10_MCU_CFG1 */
pub const AFE_IRQ10_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ10_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ10_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ10_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ10_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ10_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ10_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ10_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ10_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ11_MCU_CFG0 */
pub const AFE_IRQ11_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ11_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ11_MCU_DOMAIN_MASK_SFT: u32 = (0x7 << 9);
pub const AFE_IRQ11_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ11_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ11_MCU_FS_MASK_SFT: u32 = (0x1f << 4);
pub const AFE_IRQ11_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ11_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ11_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ11_MCU_CFG1 */
pub const AFE_IRQ11_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ11_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ11_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ11_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ11_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ11_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ11_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ11_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ11_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ12_MCU_CFG0 */
pub const AFE_IRQ12_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ12_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ12_MCU_DOMAIN_MASK_SFT: u32 = (0x7 << 9);
pub const AFE_IRQ12_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ12_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ12_MCU_FS_MASK_SFT: u32 = (0x1f << 4);
pub const AFE_IRQ12_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ12_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ12_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ12_MCU_CFG1 */
pub const AFE_IRQ12_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ12_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ12_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ12_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ12_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ12_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ12_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ12_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ12_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ13_MCU_CFG0 */
pub const AFE_IRQ13_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ13_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ13_MCU_DOMAIN_MASK_SFT: u32 = (0x7 << 9);
pub const AFE_IRQ13_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ13_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ13_MCU_FS_MASK_SFT: u32 = (0x1f << 4);
pub const AFE_IRQ13_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ13_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ13_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ13_MCU_CFG1 */
pub const AFE_IRQ13_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ13_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ13_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ13_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ13_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ13_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ13_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ13_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ13_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ14_MCU_CFG0 */
pub const AFE_IRQ14_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ14_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ14_MCU_DOMAIN_MASK_SFT: u32 = (0x7 << 9);
pub const AFE_IRQ14_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ14_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ14_MCU_FS_MASK_SFT: u32 = (0x1f << 4);
pub const AFE_IRQ14_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ14_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ14_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ14_MCU_CFG1 */
pub const AFE_IRQ14_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ14_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ14_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ14_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ14_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ14_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ14_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ14_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ14_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ15_MCU_CFG0 */
pub const AFE_IRQ15_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ15_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ15_MCU_DOMAIN_MASK_SFT: u32 = (0x7 << 9);
pub const AFE_IRQ15_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ15_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ15_MCU_FS_MASK_SFT: u32 = (0x1f << 4);
pub const AFE_IRQ15_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ15_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ15_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ15_MCU_CFG1 */
pub const AFE_IRQ15_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ15_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ15_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ15_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ15_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ15_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ15_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ15_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ15_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ16_MCU_CFG0 */
pub const AFE_IRQ16_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ16_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ16_MCU_DOMAIN_MASK_SFT: u32 = (0x7 << 9);
pub const AFE_IRQ16_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ16_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ16_MCU_FS_MASK_SFT: u32 = (0x1f << 4);
pub const AFE_IRQ16_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ16_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ16_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ16_MCU_CFG1 */
pub const AFE_IRQ16_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ16_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ16_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ16_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ16_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ16_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ16_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ16_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ16_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ17_MCU_CFG0 */
pub const AFE_IRQ17_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ17_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ17_MCU_DOMAIN_MASK_SFT: u32 = (0x7 << 9);
pub const AFE_IRQ17_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ17_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ17_MCU_FS_MASK_SFT: u32 = (0x1f << 4);
pub const AFE_IRQ17_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ17_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ17_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ17_MCU_CFG1 */
pub const AFE_IRQ17_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ17_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ17_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ17_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ17_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ17_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ17_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ17_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ17_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ18_MCU_CFG0 */
pub const AFE_IRQ18_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ18_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ18_MCU_DOMAIN_MASK_SFT: u32 = (0x7 << 9);
pub const AFE_IRQ18_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ18_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ18_MCU_FS_MASK_SFT: u32 = (0x1f << 4);
pub const AFE_IRQ18_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ18_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ18_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ18_MCU_CFG1 */
pub const AFE_IRQ18_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ18_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ18_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ18_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ18_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ18_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ18_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ18_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ18_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ19_MCU_CFG0 */
pub const AFE_IRQ19_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ19_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ19_MCU_DOMAIN_MASK_SFT: u32 = (0x7 << 9);
pub const AFE_IRQ19_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ19_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ19_MCU_FS_MASK_SFT: u32 = (0x1f << 4);
pub const AFE_IRQ19_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ19_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ19_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ19_MCU_CFG1 */
pub const AFE_IRQ19_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ19_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ19_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ19_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ19_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ19_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ19_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ19_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ19_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ20_MCU_CFG0 */
pub const AFE_IRQ20_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ20_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ20_MCU_DOMAIN_MASK_SFT: u32 = (0x7 << 9);
pub const AFE_IRQ20_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ20_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ20_MCU_FS_MASK_SFT: u32 = (0x1f << 4);
pub const AFE_IRQ20_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ20_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ20_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ20_MCU_CFG1 */
pub const AFE_IRQ20_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ20_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ20_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ20_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ20_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ20_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ20_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ20_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ20_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ21_MCU_CFG0 */
pub const AFE_IRQ21_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ21_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ21_MCU_DOMAIN_MASK_SFT: u32 = (0x7 << 9);
pub const AFE_IRQ21_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ21_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ21_MCU_FS_MASK_SFT: u32 = (0x1f << 4);
pub const AFE_IRQ21_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ21_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ21_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ21_MCU_CFG1 */
pub const AFE_IRQ21_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ21_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ21_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ21_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ21_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ21_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ21_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ21_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ21_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ22_MCU_CFG0 */
pub const AFE_IRQ22_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ22_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ22_MCU_DOMAIN_MASK_SFT: u32 = (0x7 << 9);
pub const AFE_IRQ22_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ22_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ22_MCU_FS_MASK_SFT: u32 = (0x1f << 4);
pub const AFE_IRQ22_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ22_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ22_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ22_MCU_CFG1 */
pub const AFE_IRQ22_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ22_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ22_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ22_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ22_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ22_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ22_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ22_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ22_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ23_MCU_CFG0 */
pub const AFE_IRQ23_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ23_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ23_MCU_DOMAIN_MASK_SFT: u32 = (0x7 << 9);
pub const AFE_IRQ23_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ23_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ23_MCU_FS_MASK_SFT: u32 = (0x1f << 4);
pub const AFE_IRQ23_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ23_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ23_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ23_MCU_CFG1 */
pub const AFE_IRQ23_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ23_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ23_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ23_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ23_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ23_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ23_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ23_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ23_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ24_MCU_CFG0 */
pub const AFE_IRQ24_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ24_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ24_MCU_DOMAIN_MASK_SFT: u32 = (0x7 << 9);
pub const AFE_IRQ24_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ24_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ24_MCU_FS_MASK_SFT: u32 = (0x1f << 4);
pub const AFE_IRQ24_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ24_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ24_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ24_MCU_CFG1 */
pub const AFE_IRQ24_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ24_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ24_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ24_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ24_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ24_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ24_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ24_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ24_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ25_MCU_CFG0 */
pub const AFE_IRQ25_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ25_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ25_MCU_DOMAIN_MASK_SFT: u32 = (0x7 << 9);
pub const AFE_IRQ25_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ25_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ25_MCU_FS_MASK_SFT: u32 = (0x1f << 4);
pub const AFE_IRQ25_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ25_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ25_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ25_MCU_CFG1 */
pub const AFE_IRQ25_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ25_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ25_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ25_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ25_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ25_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ25_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ25_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ25_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ26_MCU_CFG0 */
pub const AFE_IRQ26_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ26_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ26_MCU_DOMAIN_MASK_SFT: u32 = (0x7 << 9);
pub const AFE_IRQ26_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ26_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ26_MCU_FS_MASK_SFT: u32 = (0x1f << 4);
pub const AFE_IRQ26_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ26_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ26_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ26_MCU_CFG1 */
pub const AFE_IRQ26_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ26_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ26_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_IRQ26_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ26_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ26_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_IRQ26_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ26_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ26_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_CUSTOM_IRQ0_MCU_CFG0 */
pub const AFE_CUSTOM_IRQ0_MCU_ON_SFT: u32 = 0;
pub const AFE_CUSTOM_IRQ0_MCU_ON_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ0_MCU_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ_MCU_MON0 */
pub const AFE_IRQ26_MISS_FLAG_SFT: u32 = 26;
pub const AFE_IRQ26_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ26_MISS_FLAG_MASK_SFT: u32 = (0x1 << 26);
pub const AFE_IRQ25_MISS_FLAG_SFT: u32 = 25;
pub const AFE_IRQ25_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ25_MISS_FLAG_MASK_SFT: u32 = (0x1 << 25);
pub const AFE_IRQ24_MISS_FLAG_SFT: u32 = 24;
pub const AFE_IRQ24_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ24_MISS_FLAG_MASK_SFT: u32 = (0x1 << 24);
pub const AFE_IRQ23_MISS_FLAG_SFT: u32 = 23;
pub const AFE_IRQ23_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ23_MISS_FLAG_MASK_SFT: u32 = (0x1 << 23);
pub const AFE_IRQ22_MISS_FLAG_SFT: u32 = 22;
pub const AFE_IRQ22_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ22_MISS_FLAG_MASK_SFT: u32 = (0x1 << 22);
pub const AFE_IRQ21_MISS_FLAG_SFT: u32 = 21;
pub const AFE_IRQ21_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ21_MISS_FLAG_MASK_SFT: u32 = (0x1 << 21);
pub const AFE_IRQ20_MISS_FLAG_SFT: u32 = 20;
pub const AFE_IRQ20_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ20_MISS_FLAG_MASK_SFT: u32 = (0x1 << 20);
pub const AFE_IRQ19_MISS_FLAG_SFT: u32 = 19;
pub const AFE_IRQ19_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ19_MISS_FLAG_MASK_SFT: u32 = (0x1 << 19);
pub const AFE_IRQ18_MISS_FLAG_SFT: u32 = 18;
pub const AFE_IRQ18_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ18_MISS_FLAG_MASK_SFT: u32 = (0x1 << 18);
pub const AFE_IRQ17_MISS_FLAG_SFT: u32 = 17;
pub const AFE_IRQ17_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ17_MISS_FLAG_MASK_SFT: u32 = (0x1 << 17);
pub const AFE_IRQ16_MISS_FLAG_SFT: u32 = 16;
pub const AFE_IRQ16_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ16_MISS_FLAG_MASK_SFT: u32 = (0x1 << 16);
pub const AFE_IRQ15_MISS_FLAG_SFT: u32 = 15;
pub const AFE_IRQ15_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ15_MISS_FLAG_MASK_SFT: u32 = (0x1 << 15);
pub const AFE_IRQ14_MISS_FLAG_SFT: u32 = 14;
pub const AFE_IRQ14_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ14_MISS_FLAG_MASK_SFT: u32 = (0x1 << 14);
pub const AFE_IRQ13_MISS_FLAG_SFT: u32 = 13;
pub const AFE_IRQ13_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ13_MISS_FLAG_MASK_SFT: u32 = (0x1 << 13);
pub const AFE_IRQ12_MISS_FLAG_SFT: u32 = 12;
pub const AFE_IRQ12_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ12_MISS_FLAG_MASK_SFT: u32 = (0x1 << 12);
pub const AFE_IRQ11_MISS_FLAG_SFT: u32 = 11;
pub const AFE_IRQ11_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ11_MISS_FLAG_MASK_SFT: u32 = (0x1 << 11);
pub const AFE_IRQ10_MISS_FLAG_SFT: u32 = 10;
pub const AFE_IRQ10_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ10_MISS_FLAG_MASK_SFT: u32 = (0x1 << 10);
pub const AFE_IRQ9_MISS_FLAG_SFT: u32 = 9;
pub const AFE_IRQ9_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ9_MISS_FLAG_MASK_SFT: u32 = (0x1 << 9);
pub const AFE_IRQ8_MISS_FLAG_SFT: u32 = 8;
pub const AFE_IRQ8_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ8_MISS_FLAG_MASK_SFT: u32 = (0x1 << 8);
pub const AFE_IRQ7_MISS_FLAG_SFT: u32 = 7;
pub const AFE_IRQ7_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ7_MISS_FLAG_MASK_SFT: u32 = (0x1 << 7);
pub const AFE_IRQ6_MISS_FLAG_SFT: u32 = 6;
pub const AFE_IRQ6_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ6_MISS_FLAG_MASK_SFT: u32 = (0x1 << 6);
pub const AFE_IRQ5_MISS_FLAG_SFT: u32 = 5;
pub const AFE_IRQ5_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ5_MISS_FLAG_MASK_SFT: u32 = (0x1 << 5);
pub const AFE_IRQ4_MISS_FLAG_SFT: u32 = 4;
pub const AFE_IRQ4_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ4_MISS_FLAG_MASK_SFT: u32 = (0x1 << 4);
pub const AFE_IRQ3_MISS_FLAG_SFT: u32 = 3;
pub const AFE_IRQ3_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ3_MISS_FLAG_MASK_SFT: u32 = (0x1 << 3);
pub const AFE_IRQ2_MISS_FLAG_SFT: u32 = 2;
pub const AFE_IRQ2_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ2_MISS_FLAG_MASK_SFT: u32 = (0x1 << 2);
pub const AFE_IRQ1_MISS_FLAG_SFT: u32 = 1;
pub const AFE_IRQ1_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ1_MISS_FLAG_MASK_SFT: u32 = (0x1 << 1);
pub const AFE_IRQ0_MISS_FLAG_SFT: u32 = 0;
pub const AFE_IRQ0_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ0_MISS_FLAG_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ_MCU_MON1 */
pub const AFE_CUSTOM_IRQ21_MISS_FLAG_SFT: u32 = 21;
pub const AFE_CUSTOM_IRQ21_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ21_MISS_FLAG_MASK_SFT: u32 = (0x1 << 21);
pub const AFE_CUSTOM_IRQ20_MISS_FLAG_SFT: u32 = 20;
pub const AFE_CUSTOM_IRQ20_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ20_MISS_FLAG_MASK_SFT: u32 = (0x1 << 20);
pub const AFE_CUSTOM_IRQ19_MISS_FLAG_SFT: u32 = 19;
pub const AFE_CUSTOM_IRQ19_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ19_MISS_FLAG_MASK_SFT: u32 = (0x1 << 19);
pub const AFE_CUSTOM_IRQ18_MISS_FLAG_SFT: u32 = 18;
pub const AFE_CUSTOM_IRQ18_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ18_MISS_FLAG_MASK_SFT: u32 = (0x1 << 18);
pub const AFE_CUSTOM_IRQ17_MISS_FLAG_SFT: u32 = 17;
pub const AFE_CUSTOM_IRQ17_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ17_MISS_FLAG_MASK_SFT: u32 = (0x1 << 17);
pub const AFE_CUSTOM_IRQ16_MISS_FLAG_SFT: u32 = 16;
pub const AFE_CUSTOM_IRQ16_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ16_MISS_FLAG_MASK_SFT: u32 = (0x1 << 16);
pub const AFE_CUSTOM_IRQ9_MISS_FLAG_SFT: u32 = 9;
pub const AFE_CUSTOM_IRQ9_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ9_MISS_FLAG_MASK_SFT: u32 = (0x1 << 9);
pub const AFE_CUSTOM_IRQ8_MISS_FLAG_SFT: u32 = 8;
pub const AFE_CUSTOM_IRQ8_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ8_MISS_FLAG_MASK_SFT: u32 = (0x1 << 8);
pub const AFE_CUSTOM_IRQ7_MISS_FLAG_SFT: u32 = 7;
pub const AFE_CUSTOM_IRQ7_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ7_MISS_FLAG_MASK_SFT: u32 = (0x1 << 7);
pub const AFE_CUSTOM_IRQ6_MISS_FLAG_SFT: u32 = 6;
pub const AFE_CUSTOM_IRQ6_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ6_MISS_FLAG_MASK_SFT: u32 = (0x1 << 6);
pub const AFE_CUSTOM_IRQ5_MISS_FLAG_SFT: u32 = 5;
pub const AFE_CUSTOM_IRQ5_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ5_MISS_FLAG_MASK_SFT: u32 = (0x1 << 5);
pub const AFE_CUSTOM_IRQ4_MISS_FLAG_SFT: u32 = 4;
pub const AFE_CUSTOM_IRQ4_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ4_MISS_FLAG_MASK_SFT: u32 = (0x1 << 4);
pub const AFE_CUSTOM_IRQ3_MISS_FLAG_SFT: u32 = 3;
pub const AFE_CUSTOM_IRQ3_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ3_MISS_FLAG_MASK_SFT: u32 = (0x1 << 3);
pub const AFE_CUSTOM_IRQ2_MISS_FLAG_SFT: u32 = 2;
pub const AFE_CUSTOM_IRQ2_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ2_MISS_FLAG_MASK_SFT: u32 = (0x1 << 2);
pub const AFE_CUSTOM_IRQ1_MISS_FLAG_SFT: u32 = 1;
pub const AFE_CUSTOM_IRQ1_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ1_MISS_FLAG_MASK_SFT: u32 = (0x1 << 1);
pub const AFE_CUSTOM_IRQ0_MISS_FLAG_SFT: u32 = 0;
pub const AFE_CUSTOM_IRQ0_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ0_MISS_FLAG_MASK_SFT: u32 = (0x1 << 0);

/* AFE_IRQ_MCU_MON2 */
pub const AFE_IRQ_B_R_CNT_SFT: u32 = 8;
pub const AFE_IRQ_B_R_CNT_MASK: u32 = 0xff;
pub const AFE_IRQ_B_R_CNT_MASK_SFT: u32 = (0xff << 8);
pub const AFE_IRQ_B_F_CNT_SFT: u32 = 0;
pub const AFE_IRQ_B_F_CNT_MASK: u32 = 0xff;
pub const AFE_IRQ_B_F_CNT_MASK_SFT: u32 = (0xff << 0);

/* AFE_IRQ0_CNT_MON */
pub const AFE_IRQ0_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ0_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ0_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ1_CNT_MON */
pub const AFE_IRQ1_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ1_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ1_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ2_CNT_MON */
pub const AFE_IRQ2_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ2_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ2_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ3_CNT_MON */
pub const AFE_IRQ3_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ3_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ3_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ4_CNT_MON */
pub const AFE_IRQ4_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ4_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ4_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ5_CNT_MON */
pub const AFE_IRQ5_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ5_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ5_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ6_CNT_MON */
pub const AFE_IRQ6_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ6_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ6_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ7_CNT_MON */
pub const AFE_IRQ7_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ7_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ7_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ8_CNT_MON */
pub const AFE_IRQ8_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ8_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ8_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ9_CNT_MON */
pub const AFE_IRQ9_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ9_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ9_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ10_CNT_MON */
pub const AFE_IRQ10_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ10_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ10_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ11_CNT_MON */
pub const AFE_IRQ11_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ11_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ11_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ12_CNT_MON */
pub const AFE_IRQ12_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ12_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ12_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ13_CNT_MON */
pub const AFE_IRQ13_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ13_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ13_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ14_CNT_MON */
pub const AFE_IRQ14_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ14_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ14_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ15_CNT_MON */
pub const AFE_IRQ15_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ15_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ15_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ16_CNT_MON */
pub const AFE_IRQ16_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ16_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ16_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ17_CNT_MON */
pub const AFE_IRQ17_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ17_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ17_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ18_CNT_MON */
pub const AFE_IRQ18_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ18_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ18_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ19_CNT_MON */
pub const AFE_IRQ19_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ19_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ19_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ20_CNT_MON */
pub const AFE_IRQ20_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ20_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ20_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ21_CNT_MON */
pub const AFE_IRQ21_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ21_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ21_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ22_CNT_MON */
pub const AFE_IRQ22_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ22_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ22_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ23_CNT_MON */
pub const AFE_IRQ23_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ23_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ23_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ24_CNT_MON */
pub const AFE_IRQ24_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ24_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ24_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ25_CNT_MON */
pub const AFE_IRQ25_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ25_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ25_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_IRQ26_CNT_MON */
pub const AFE_IRQ26_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ26_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ26_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_CUSTOM_IRQ0_CNT_MON */
pub const AFE_CUSTOM_IRQ0_CNT_MON_SFT: u32 = 0;
pub const AFE_CUSTOM_IRQ0_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_CUSTOM_IRQ0_CNT_MON_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_CUSTOM_IRQ0_MCU_CFG1 */
pub const AFE_CUSTOM_IRQ0_CLR_CFG_SFT: u32 = 31;
pub const AFE_CUSTOM_IRQ0_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ0_CLR_CFG_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_CUSTOM_IRQ0_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_CUSTOM_IRQ0_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ0_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = (0x1 << 30);
pub const AFE_CUSTOM_IRQ0_MCU_CNT_SFT: u32 = 0;
pub const AFE_CUSTOM_IRQ0_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_CUSTOM_IRQ0_MCU_CNT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_GAIN0_CON1_R */
/* AFE_GAIN1_CON1_R */
/* AFE_GAIN2_CON1_R */
/* AFE_GAIN3_CON1_R */
pub const GAIN_TARGET_R_SFT: u32 = 0;
pub const GAIN_TARGET_R_MASK: u32 = 0xffffffff;
pub const GAIN_TARGET_R_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_GAIN0_CON1_L */
/* AFE_GAIN1_CON1_L */
/* AFE_GAIN2_CON1_L */
/* AFE_GAIN3_CON1_L */
pub const GAIN_TARGET_L_SFT: u32 = 0;
pub const GAIN_TARGET_L_MASK: u32 = 0xffffffff;
pub const GAIN_TARGET_L_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_GAIN0_CON2 */
pub const GAIN0_DOWN_STEP_SFT: u32 = 0;
pub const GAIN0_DOWN_STEP_MASK: u32 = 0x3fffff;
pub const GAIN0_DOWN_STEP_MASK_SFT: u32 = (0x3fffff << 0);

/* AFE_GAIN0_CON3 */
pub const GAIN0_UP_STEP_SFT: u32 = 0;
pub const GAIN0_UP_STEP_MASK: u32 = 0x3fffff;
pub const GAIN0_UP_STEP_MASK_SFT: u32 = (0x3fffff << 0);

/* AFE_GAIN0_CUR_R */
/* AFE_GAIN1_CUR_R */
/* AFE_GAIN2_CUR_R */
/* AFE_GAIN3_CUR_R */
pub const AFE_GAIN_CUR_R_SFT: u32 = 0;
pub const AFE_GAIN_CUR_R_MASK: u32 = 0xffffffff;
pub const AFE_GAIN_CUR_R_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_GAIN0_CUR_L */
/* AFE_GAIN1_CUR_L */
/* AFE_GAIN2_CUR_L */
/* AFE_GAIN3_CUR_L */
pub const AFE_GAIN_CUR_L_SFT: u32 = 0;
pub const AFE_GAIN_CUR_L_MASK: u32 = 0xffffffff;
pub const AFE_GAIN_CUR_L_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_GAIN0_CON0 */
/* AFE_GAIN1_CON0 */
/* AFE_GAIN2_CON0 */
/* AFE_GAIN3_CON0 */
pub const GAIN_TARGET_SYNC_ON_SFT: u32 = 24;
pub const GAIN_TARGET_SYNC_ON_MASK: u32 = 0x1;
pub const GAIN_TARGET_SYNC_ON_MASK_SFT: u32 = (0x1 << 24);
pub const GAIN_TIMEOUT_SFT: u32 = 18;
pub const GAIN_TIMEOUT_MASK: u32 = 0x3f;
pub const GAIN_TIMEOUT_MASK_SFT: u32 = (0x3f << 18);
pub const GAIN_TRIG_SFT: u32 = 17;
pub const GAIN_TRIG_MASK: u32 = 0x1;
pub const GAIN_TRIG_MASK_SFT: u32 = (0x1 << 17);
pub const GAIN_ON_SFT: u32 = 16;
pub const GAIN_ON_MASK: u32 = 0x1;
pub const GAIN_ON_MASK_SFT: u32 = (0x1 << 16);
pub const GAIN_SAMPLE_PER_STEP_SFT: u32 = 8;
pub const GAIN_SAMPLE_PER_STEP_MASK: u32 = 0xff;
pub const GAIN_SAMPLE_PER_STEP_MASK_SFT: u32 = (0xff << 8);
pub const GAIN_SEL_DOMAIN_SFT: u32 = 5;
pub const GAIN_SEL_DOMAIN_MASK: u32 = 0x7;
pub const GAIN_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 5);
pub const GAIN_SEL_FS_SFT: u32 = 0;
pub const GAIN_SEL_FS_MASK: u32 = 0x1f;
pub const GAIN_SEL_FS_MASK_SFT: u32 = (0x1f << 0);

/* AFE_GAIN1_CON2 */
pub const GAIN1_DOWN_STEP_SFT: u32 = 0;
pub const GAIN1_DOWN_STEP_MASK: u32 = 0x3fffff;
pub const GAIN1_DOWN_STEP_MASK_SFT: u32 = (0x3fffff << 0);

/* AFE_GAIN1_CON3 */
pub const GAIN1_UP_STEP_SFT: u32 = 0;
pub const GAIN1_UP_STEP_MASK: u32 = 0x3fffff;
pub const GAIN1_UP_STEP_MASK_SFT: u32 = (0x3fffff << 0);

/* AFE_GAIN2_CON2 */
pub const GAIN2_DOWN_STEP_SFT: u32 = 0;
pub const GAIN2_DOWN_STEP_MASK: u32 = 0x3fffff;
pub const GAIN2_DOWN_STEP_MASK_SFT: u32 = (0x3fffff << 0);

/* AFE_GAIN2_CON3 */
pub const GAIN2_UP_STEP_SFT: u32 = 0;
pub const GAIN2_UP_STEP_MASK: u32 = 0x3fffff;
pub const GAIN2_UP_STEP_MASK_SFT: u32 = (0x3fffff << 0);

/* AFE_GAIN3_CON2 */
pub const GAIN3_DOWN_STEP_SFT: u32 = 0;
pub const GAIN3_DOWN_STEP_MASK: u32 = 0x3fffff;
pub const GAIN3_DOWN_STEP_MASK_SFT: u32 = (0x3fffff << 0);

/* AFE_GAIN3_CON3 */
pub const GAIN3_UP_STEP_SFT: u32 = 0;
pub const GAIN3_UP_STEP_MASK: u32 = 0x3fffff;
pub const GAIN3_UP_STEP_MASK_SFT: u32 = (0x3fffff << 0);

/* AFE_STF_CON0 */
pub const SLT_CNT_FLAG_RESET_SFT: u32 = 28;
pub const SLT_CNT_FLAG_RESET_MASK: u32 = 0x1;
pub const SLT_CNT_FLAG_RESET_MASK_SFT: u32 = (0x1 << 28);
pub const SLT_CNT_THD_SFT: u32 = 16;
pub const SLT_CNT_THD_MASK: u32 = 0xfff;
pub const SLT_CNT_THD_MASK_SFT: u32 = (0xfff << 16);
pub const SIDE_TONE_HALF_TAP_NUM_SFT: u32 = 4;
pub const SIDE_TONE_HALF_TAP_NUM_MASK: u32 = 0x7f;
pub const SIDE_TONE_HALF_TAP_NUM_MASK_SFT: u32 = (0x7f << 4);
pub const SIDE_TONE_ODD_MODE_SFT: u32 = 1;
pub const SIDE_TONE_ODD_MODE_MASK: u32 = 0x1;
pub const SIDE_TONE_ODD_MODE_MASK_SFT: u32 = (0x1 << 1);
pub const SIDE_TONE_ON_SFT: u32 = 0;
pub const SIDE_TONE_ON_MASK: u32 = 0x1;
pub const SIDE_TONE_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_STF_CON1 */
pub const SIDE_TONE_IN_EN_SEL_DOMAIN_SFT: u32 = 5;
pub const SIDE_TONE_IN_EN_SEL_DOMAIN_MASK: u32 = 0x7;
pub const SIDE_TONE_IN_EN_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 5);
pub const SIDE_TONE_IN_EN_SEL_FS_SFT: u32 = 0;
pub const SIDE_TONE_IN_EN_SEL_FS_MASK: u32 = 0x1f;
pub const SIDE_TONE_IN_EN_SEL_FS_MASK_SFT: u32 = (0x1f << 0);

/* AFE_STF_COEFF */
pub const SIDE_TONE_COEFFICIENT_R_W_SEL_SFT: u32 = 24;
pub const SIDE_TONE_COEFFICIENT_R_W_SEL_MASK: u32 = 0x1;
pub const SIDE_TONE_COEFFICIENT_R_W_SEL_MASK_SFT: u32 = (0x1 << 24);
pub const SIDE_TONE_COEFFICIENT_ADDR_SFT: u32 = 16;
pub const SIDE_TONE_COEFFICIENT_ADDR_MASK: u32 = 0x1f;
pub const SIDE_TONE_COEFFICIENT_ADDR_MASK_SFT: u32 = (0x1f << 16);
pub const SIDE_TONE_COEFFICIENT_SFT: u32 = 0;
pub const SIDE_TONE_COEFFICIENT_MASK: u32 = 0xffff;
pub const SIDE_TONE_COEFFICIENT_MASK_SFT: u32 = (0xffff << 0);

/* AFE_STF_GAIN */
pub const SIDE_TONE_POSITIVE_GAIN_SFT: u32 = 16;
pub const SIDE_TONE_POSITIVE_GAIN_MASK: u32 = 0x7;
pub const SIDE_TONE_POSITIVE_GAIN_MASK_SFT: u32 = (0x7 << 16);
pub const SIDE_TONE_GAIN_SFT: u32 = 0;
pub const SIDE_TONE_GAIN_MASK: u32 = 0xffff;
pub const SIDE_TONE_GAIN_MASK_SFT: u32 = (0xffff << 0);

/* AFE_STF_MON */
pub const SIDE_TONE_R_RDY_SFT: u32 = 30;
pub const SIDE_TONE_R_RDY_MASK: u32 = 0x1;
pub const SIDE_TONE_R_RDY_MASK_SFT: u32 = (0x1 << 30);
pub const SIDE_TONE_W_RDY_SFT: u32 = 29;
pub const SIDE_TONE_W_RDY_MASK: u32 = 0x1;
pub const SIDE_TONE_W_RDY_MASK_SFT: u32 = (0x1 << 29);
pub const SLT_CNT_FLAG_SFT: u32 = 28;
pub const SLT_CNT_FLAG_MASK: u32 = 0x1;
pub const SLT_CNT_FLAG_MASK_SFT: u32 = (0x1 << 28);
pub const SLT_CNT_SFT: u32 = 16;
pub const SLT_CNT_MASK: u32 = 0xfff;
pub const SLT_CNT_MASK_SFT: u32 = (0xfff << 16);
pub const SIDE_TONE_COEFF_SFT: u32 = 0;
pub const SIDE_TONE_COEFF_MASK: u32 = 0xffff;
pub const SIDE_TONE_COEFF_MASK_SFT: u32 = (0xffff << 0);

/* AFE_STF_IP_VERSION */
pub const SIDE_TONE_IP_VERSION_SFT: u32 = 0;
pub const SIDE_TONE_IP_VERSION_MASK: u32 = 0xffffffff;
pub const SIDE_TONE_IP_VERSION_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_CM_REG */
pub const AFE_CM_UPDATE_CNT_SFT: u32 = 16;
pub const AFE_CM_UPDATE_CNT_MASK: u32 = 0x7fff;
pub const AFE_CM_UPDATE_CNT_MASK_SFT: u32 = (0x7fff << 16);
pub const AFE_CM_1X_EN_SEL_FS_SFT: u32 = 8;
pub const AFE_CM_1X_EN_SEL_FS_MASK: u32 = 0x1f;
pub const AFE_CM_1X_EN_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const AFE_CM_CH_NUM_SFT: u32 = 2;
pub const AFE_CM_CH_NUM_MASK: u32 = 0x1f;
pub const AFE_CM_CH_NUM_MASK_SFT: u32 = (0x1f << 2);
pub const AFE_CM_BYTE_SWAP_SFT: u32 = 1;
pub const AFE_CM_BYTE_SWAP_MASK: u32 = 0x1;
pub const AFE_CM_BYTE_SWAP_MASK_SFT: u32 = (0x1 << 1);
pub const AFE_CM_BYPASS_MODE_SFT: u32 = 31;
pub const AFE_CM_BYPASS_MODE_MASK: u32 = 0x1;
pub const AFE_CM_BYPASS_MODE_MASK_SFT: u32 = (0x1 << 31);

/* AFE_CM0_CON0 */
pub const AFE_CM0_BYPASS_MODE_SFT: u32 = 31;
pub const AFE_CM0_BYPASS_MODE_MASK: u32 = 0x1;
pub const AFE_CM0_BYPASS_MODE_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_CM0_UPDATE_CNT_SFT: u32 = 16;
pub const AFE_CM0_UPDATE_CNT_MASK: u32 = 0x7fff;
pub const AFE_CM0_UPDATE_CNT_MASK_SFT: u32 = (0x7fff << 16);
pub const AFE_CM0_1X_EN_SEL_DOMAIN_SFT: u32 = 13;
pub const AFE_CM0_1X_EN_SEL_DOMAIN_MASK: u32 = 0x7;
pub const AFE_CM0_1X_EN_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const AFE_CM0_1X_EN_SEL_FS_SFT: u32 = 8;
pub const AFE_CM0_1X_EN_SEL_FS_MASK: u32 = 0x1f;
pub const AFE_CM0_1X_EN_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const AFE_CM0_OUTPUT_MUX_SFT: u32 = 7;
pub const AFE_CM0_OUTPUT_MUX_MASK: u32 = 0x1;
pub const AFE_CM0_OUTPUT_MUX_MASK_SFT: u32 = (0x1 << 7);
pub const AFE_CM0_CH_NUM_SFT: u32 = 2;
pub const AFE_CM0_CH_NUM_MASK: u32 = 0x1f;
pub const AFE_CM0_CH_NUM_MASK_SFT: u32 = (0x1f << 2);
pub const AFE_CM0_BYTE_SWAP_SFT: u32 = 1;
pub const AFE_CM0_BYTE_SWAP_MASK: u32 = 0x1;
pub const AFE_CM0_BYTE_SWAP_MASK_SFT: u32 = (0x1 << 1);
pub const AFE_CM0_ON_SFT: u32 = 0;
pub const AFE_CM0_ON_MASK: u32 = 0x1;
pub const AFE_CM0_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_CM0_MON */
pub const AFE_CM0_BYPASS_MODE_MON_SFT: u32 = 31;
pub const AFE_CM0_BYPASS_MODE_MON_MASK: u32 = 0x1;
pub const AFE_CM0_BYPASS_MODE_MON_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_CM0_OUTPUT_CNT_MON_SFT: u32 = 16;
pub const AFE_CM0_OUTPUT_CNT_MON_MASK: u32 = 0x7fff;
pub const AFE_CM0_OUTPUT_CNT_MON_MASK_SFT: u32 = (0x7fff << 16);
pub const AFE_CM0_CUR_CHSET_MON_SFT: u32 = 5;
pub const AFE_CM0_CUR_CHSET_MON_MASK: u32 = 0xf;
pub const AFE_CM0_CUR_CHSET_MON_MASK_SFT: u32 = (0xf << 5);
pub const AFE_CM0_ODD_FLAG_MON_SFT: u32 = 4;
pub const AFE_CM0_ODD_FLAG_MON_MASK: u32 = 0x1;
pub const AFE_CM0_ODD_FLAG_MON_MASK_SFT: u32 = (0x1 << 4);
pub const AFE_CM0_BYTE_SWAP_MON_SFT: u32 = 1;
pub const AFE_CM0_BYTE_SWAP_MON_MASK: u32 = 0x1;
pub const AFE_CM0_BYTE_SWAP_MON_MASK_SFT: u32 = (0x1 << 1);
pub const AFE_CM0_ON_MON_SFT: u32 = 0;
pub const AFE_CM0_ON_MON_MASK: u32 = 0x1;
pub const AFE_CM0_ON_MON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_CM0_IP_VERSION */
pub const AFE_CM0_IP_VERSION_SFT: u32 = 0;
pub const AFE_CM0_IP_VERSION_MASK: u32 = 0xffffffff;
pub const AFE_CM0_IP_VERSION_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_CM1_CON0 */
pub const AFE_CM1_BYPASS_MODE_SFT: u32 = 31;
pub const AFE_CM1_BYPASS_MODE_MASK: u32 = 0x1;
pub const AFE_CM1_BYPASS_MODE_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_CM1_UPDATE_CNT_SFT: u32 = 16;
pub const AFE_CM1_UPDATE_CNT_MASK: u32 = 0x7fff;
pub const AFE_CM1_UPDATE_CNT_MASK_SFT: u32 = (0x7fff << 16);
pub const AFE_CM1_1X_EN_SEL_DOMAIN_SFT: u32 = 13;
pub const AFE_CM1_1X_EN_SEL_DOMAIN_MASK: u32 = 0x7;
pub const AFE_CM1_1X_EN_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const AFE_CM1_1X_EN_SEL_FS_SFT: u32 = 8;
pub const AFE_CM1_1X_EN_SEL_FS_MASK: u32 = 0x1f;
pub const AFE_CM1_1X_EN_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const AFE_CM1_OUTPUT_MUX_SFT: u32 = 7;
pub const AFE_CM1_OUTPUT_MUX_MASK: u32 = 0x1;
pub const AFE_CM1_OUTPUT_MUX_MASK_SFT: u32 = (0x1 << 7);
pub const AFE_CM1_CH_NUM_SFT: u32 = 2;
pub const AFE_CM1_CH_NUM_MASK: u32 = 0x1f;
pub const AFE_CM1_CH_NUM_MASK_SFT: u32 = (0x1f << 2);
pub const AFE_CM1_BYTE_SWAP_SFT: u32 = 1;
pub const AFE_CM1_BYTE_SWAP_MASK: u32 = 0x1;
pub const AFE_CM1_BYTE_SWAP_MASK_SFT: u32 = (0x1 << 1);
pub const AFE_CM1_ON_SFT: u32 = 0;
pub const AFE_CM1_ON_MASK: u32 = 0x1;
pub const AFE_CM1_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_CM1_MON */
pub const AFE_CM1_BYPASS_MODE_MON_SFT: u32 = 31;
pub const AFE_CM1_BYPASS_MODE_MON_MASK: u32 = 0x1;
pub const AFE_CM1_BYPASS_MODE_MON_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_CM1_OUTPUT_CNT_MON_SFT: u32 = 16;
pub const AFE_CM1_OUTPUT_CNT_MON_MASK: u32 = 0x7fff;
pub const AFE_CM1_OUTPUT_CNT_MON_MASK_SFT: u32 = (0x7fff << 16);
pub const AFE_CM1_CUR_CHSET_MON_SFT: u32 = 5;
pub const AFE_CM1_CUR_CHSET_MON_MASK: u32 = 0xf;
pub const AFE_CM1_CUR_CHSET_MON_MASK_SFT: u32 = (0xf << 5);
pub const AFE_CM1_ODD_FLAG_MON_SFT: u32 = 4;
pub const AFE_CM1_ODD_FLAG_MON_MASK: u32 = 0x1;
pub const AFE_CM1_ODD_FLAG_MON_MASK_SFT: u32 = (0x1 << 4);
pub const AFE_CM1_BYTE_SWAP_MON_SFT: u32 = 1;
pub const AFE_CM1_BYTE_SWAP_MON_MASK: u32 = 0x1;
pub const AFE_CM1_BYTE_SWAP_MON_MASK_SFT: u32 = (0x1 << 1);
pub const AFE_CM1_ON_MON_SFT: u32 = 0;
pub const AFE_CM1_ON_MON_MASK: u32 = 0x1;
pub const AFE_CM1_ON_MON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_CM1_IP_VERSION */
pub const AFE_CM1_IP_VERSION_SFT: u32 = 0;
pub const AFE_CM1_IP_VERSION_MASK: u32 = 0xffffffff;
pub const AFE_CM1_IP_VERSION_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_CM2_CON0 */
pub const AFE_CM2_BYPASS_MODE_SFT: u32 = 31;
pub const AFE_CM2_BYPASS_MODE_MASK: u32 = 0x1;
pub const AFE_CM2_BYPASS_MODE_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_CM2_UPDATE_CNT_SFT: u32 = 16;
pub const AFE_CM2_UPDATE_CNT_MASK: u32 = 0x7fff;
pub const AFE_CM2_UPDATE_CNT_MASK_SFT: u32 = (0x7fff << 16);
pub const AFE_CM2_1X_EN_SEL_DOMAIN_SFT: u32 = 13;
pub const AFE_CM2_1X_EN_SEL_DOMAIN_MASK: u32 = 0x7;
pub const AFE_CM2_1X_EN_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const AFE_CM2_1X_EN_SEL_FS_SFT: u32 = 8;
pub const AFE_CM2_1X_EN_SEL_FS_MASK: u32 = 0x1f;
pub const AFE_CM2_1X_EN_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const AFE_CM2_OUTPUT_MUX_SFT: u32 = 7;
pub const AFE_CM2_OUTPUT_MUX_MASK: u32 = 0x1;
pub const AFE_CM2_OUTPUT_MUX_MASK_SFT: u32 = (0x1 << 7);
pub const AFE_CM2_CH_NUM_SFT: u32 = 2;
pub const AFE_CM2_CH_NUM_MASK: u32 = 0x1f;
pub const AFE_CM2_CH_NUM_MASK_SFT: u32 = (0x1f << 2);
pub const AFE_CM2_BYTE_SWAP_SFT: u32 = 1;
pub const AFE_CM2_BYTE_SWAP_MASK: u32 = 0x1;
pub const AFE_CM2_BYTE_SWAP_MASK_SFT: u32 = (0x1 << 1);
pub const AFE_CM2_ON_SFT: u32 = 0;
pub const AFE_CM2_ON_MASK: u32 = 0x1;
pub const AFE_CM2_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_CM2_MON */
pub const AFE_CM2_BYPASS_MODE_MON_SFT: u32 = 31;
pub const AFE_CM2_BYPASS_MODE_MON_MASK: u32 = 0x1;
pub const AFE_CM2_BYPASS_MODE_MON_MASK_SFT: u32 = (0x1 << 31);
pub const AFE_CM2_OUTPUT_CNT_MON_SFT: u32 = 16;
pub const AFE_CM2_OUTPUT_CNT_MON_MASK: u32 = 0x7fff;
pub const AFE_CM2_OUTPUT_CNT_MON_MASK_SFT: u32 = (0x7fff << 16);
pub const AFE_CM2_CUR_CHSET_MON_SFT: u32 = 5;
pub const AFE_CM2_CUR_CHSET_MON_MASK: u32 = 0xf;
pub const AFE_CM2_CUR_CHSET_MON_MASK_SFT: u32 = (0xf << 5);
pub const AFE_CM2_ODD_FLAG_MON_SFT: u32 = 4;
pub const AFE_CM2_ODD_FLAG_MON_MASK: u32 = 0x1;
pub const AFE_CM2_ODD_FLAG_MON_MASK_SFT: u32 = (0x1 << 4);
pub const AFE_CM2_BYTE_SWAP_MON_SFT: u32 = 1;
pub const AFE_CM2_BYTE_SWAP_MON_MASK: u32 = 0x1;
pub const AFE_CM2_BYTE_SWAP_MON_MASK_SFT: u32 = (0x1 << 1);
pub const AFE_CM2_ON_MON_SFT: u32 = 0;
pub const AFE_CM2_ON_MON_MASK: u32 = 0x1;
pub const AFE_CM2_ON_MON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_CM2_IP_VERSION */
pub const AFE_CM2_IP_VERSION_SFT: u32 = 0;
pub const AFE_CM2_IP_VERSION_MASK: u32 = 0xffffffff;
pub const AFE_CM2_IP_VERSION_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ADDA_UL0_SRC_CON0 */
pub const ULCF_CFG_EN_CTL_SFT: u32 = 31;
pub const ULCF_CFG_EN_CTL_MASK: u32 = 0x1;
pub const ULCF_CFG_EN_CTL_MASK_SFT: u32 = (0x1 << 31);
pub const UL_DMIC_PHASE_SEL_CH1_SFT: u32 = 27;
pub const UL_DMIC_PHASE_SEL_CH1_MASK: u32 = 0x7;
pub const UL_DMIC_PHASE_SEL_CH1_MASK_SFT: u32 = (0x7 << 27);
pub const UL_DMIC_PHASE_SEL_CH2_SFT: u32 = 24;
pub const UL_DMIC_PHASE_SEL_CH2_MASK: u32 = 0x7;
pub const UL_DMIC_PHASE_SEL_CH2_MASK_SFT: u32 = (0x7 << 24);
pub const UL_DMIC_TWO_WIRE_CTL_SFT: u32 = 23;
pub const UL_DMIC_TWO_WIRE_CTL_MASK: u32 = 0x1;
pub const UL_DMIC_TWO_WIRE_CTL_MASK_SFT: u32 = (0x1 << 23);
pub const UL_MODE_3P25M_CH2_CTL_SFT: u32 = 22;
pub const UL_MODE_3P25M_CH2_CTL_MASK: u32 = 0x1;
pub const UL_MODE_3P25M_CH2_CTL_MASK_SFT: u32 = (0x1 << 22);
pub const UL_MODE_3P25M_CH1_CTL_SFT: u32 = 21;
pub const UL_MODE_3P25M_CH1_CTL_MASK: u32 = 0x1;
pub const UL_MODE_3P25M_CH1_CTL_MASK_SFT: u32 = (0x1 << 21);
pub const UL_VOICE_MODE_CH1_CH2_CTL_SFT: u32 = 17;
pub const UL_VOICE_MODE_CH1_CH2_CTL_MASK: u32 = 0x7;
pub const UL_VOICE_MODE_CH1_CH2_CTL_MASK_SFT: u32 = (0x7 << 17);
pub const UL_AP_DMIC_ON_SFT: u32 = 16;
pub const UL_AP_DMIC_ON_MASK: u32 = 0x1;
pub const UL_AP_DMIC_ON_MASK_SFT: u32 = (0x1 << 16);
pub const DMIC_LOW_POWER_MODE_CTL_SFT: u32 = 14;
pub const DMIC_LOW_POWER_MODE_CTL_MASK: u32 = 0x3;
pub const DMIC_LOW_POWER_MODE_CTL_MASK_SFT: u32 = (0x3 << 14);
pub const UL_DISABLE_HW_CG_CTL_SFT: u32 = 12;
pub const UL_DISABLE_HW_CG_CTL_MASK: u32 = 0x1;
pub const UL_DISABLE_HW_CG_CTL_MASK_SFT: u32 = (0x1 << 12);
pub const AMIC_26M_SEL_CTL_SFT: u32 = 11;
pub const AMIC_26M_SEL_CTL_MASK: u32 = 0x1;
pub const AMIC_26M_SEL_CTL_MASK_SFT: u32 = (0x1 << 11);
pub const UL_IIR_ON_TMP_CTL_SFT: u32 = 10;
pub const UL_IIR_ON_TMP_CTL_MASK: u32 = 0x1;
pub const UL_IIR_ON_TMP_CTL_MASK_SFT: u32 = (0x1 << 10);
pub const UL_IIRMODE_CTL_SFT: u32 = 7;
pub const UL_IIRMODE_CTL_MASK: u32 = 0x7;
pub const UL_IIRMODE_CTL_MASK_SFT: u32 = (0x7 << 7);
pub const DIGMIC_4P33M_SEL_SFT: u32 = 6;
pub const DIGMIC_4P33M_SEL_MASK: u32 = 0x1;
pub const DIGMIC_4P33M_SEL_MASK_SFT: u32 = (0x1 << 6);
pub const DIGMIC_3P25M_1P625M_SEL_CTL_SFT: u32 = 5;
pub const DIGMIC_3P25M_1P625M_SEL_CTL_MASK: u32 = 0x1;
pub const DIGMIC_3P25M_1P625M_SEL_CTL_MASK_SFT: u32 = (0x1 << 5);
pub const AMIC_6P5M_SEL_CTL_SFT: u32 = 4;
pub const AMIC_6P5M_SEL_CTL_MASK: u32 = 0x1;
pub const AMIC_6P5M_SEL_CTL_MASK_SFT: u32 = (0x1 << 4);
pub const AMIC_1P625M_SEL_CTL_SFT: u32 = 3;
pub const AMIC_1P625M_SEL_CTL_MASK: u32 = 0x1;
pub const AMIC_1P625M_SEL_CTL_MASK_SFT: u32 = (0x1 << 3);
pub const UL_LOOP_BACK_MODE_CTL_SFT: u32 = 2;
pub const UL_LOOP_BACK_MODE_CTL_MASK: u32 = 0x1;
pub const UL_LOOP_BACK_MODE_CTL_MASK_SFT: u32 = (0x1 << 2);
pub const UL_SDM_3_LEVEL_CTL_SFT: u32 = 1;
pub const UL_SDM_3_LEVEL_CTL_MASK: u32 = 0x1;
pub const UL_SDM_3_LEVEL_CTL_MASK_SFT: u32 = (0x1 << 1);
pub const UL_SRC_ON_TMP_CTL_SFT: u32 = 0;
pub const UL_SRC_ON_TMP_CTL_MASK: u32 = 0x1;
pub const UL_SRC_ON_TMP_CTL_MASK_SFT: u32 = (0x1 << 0);

/* AFE_ADDA_UL0_SRC_CON1 */
pub const ADDA_UL_GAIN_VALUE_SFT: u32 = 16;
pub const ADDA_UL_GAIN_VALUE_MASK: u32 = 0xffff;
pub const ADDA_UL_GAIN_VALUE_MASK_SFT: u32 = (0xffff << 16);
pub const ADDA_UL_POSTIVEGAIN_SFT: u32 = 12;
pub const ADDA_UL_POSTIVEGAIN_MASK: u32 = 0x7;
pub const ADDA_UL_POSTIVEGAIN_MASK_SFT: u32 = (0x7 << 12);
pub const ADDA_UL_ODDTAP_MODE_SFT: u32 = 11;
pub const ADDA_UL_ODDTAP_MODE_MASK: u32 = 0x1;
pub const ADDA_UL_ODDTAP_MODE_MASK_SFT: u32 = (0x1 << 11);
pub const ADDA_UL_HALF_TAP_NUM_SFT: u32 = 5;
pub const ADDA_UL_HALF_TAP_NUM_MASK: u32 = 0x3f;
pub const ADDA_UL_HALF_TAP_NUM_MASK_SFT: u32 = (0x3f << 5);
pub const FIFO_SOFT_RST_SFT: u32 = 4;
pub const FIFO_SOFT_RST_MASK: u32 = 0x1;
pub const FIFO_SOFT_RST_MASK_SFT: u32 = (0x1 << 4);
pub const FIFO_SOFT_RST_EN_SFT: u32 = 3;
pub const FIFO_SOFT_RST_EN_MASK: u32 = 0x1;
pub const FIFO_SOFT_RST_EN_MASK_SFT: u32 = (0x1 << 3);
pub const LR_SWAP_SFT: u32 = 2;
pub const LR_SWAP_MASK: u32 = 0x1;
pub const LR_SWAP_MASK_SFT: u32 = (0x1 << 2);
pub const GAIN_MODE_SFT: u32 = 0;
pub const GAIN_MODE_MASK: u32 = 0x3;
pub const GAIN_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_ADDA_UL0_SRC_CON2 */
pub const C_DAC_EN_CTL_SFT: u32 = 27;
pub const C_DAC_EN_CTL_MASK: u32 = 0x1;
pub const C_DAC_EN_CTL_MASK_SFT: u32 = (0x1 << 27);
pub const C_MUTE_SW_CTL_SFT: u32 = 26;
pub const C_MUTE_SW_CTL_MASK: u32 = 0x1;
pub const C_MUTE_SW_CTL_MASK_SFT: u32 = (0x1 << 26);
pub const C_AMP_DIV_CH2_CTL_SFT: u32 = 21;
pub const C_AMP_DIV_CH2_CTL_MASK: u32 = 0x7;
pub const C_AMP_DIV_CH2_CTL_MASK_SFT: u32 = (0x7 << 21);
pub const C_FREQ_DIV_CH2_CTL_SFT: u32 = 16;
pub const C_FREQ_DIV_CH2_CTL_MASK: u32 = 0x1f;
pub const C_FREQ_DIV_CH2_CTL_MASK_SFT: u32 = (0x1f << 16);
pub const C_SINE_MODE_CH2_CTL_SFT: u32 = 12;
pub const C_SINE_MODE_CH2_CTL_MASK: u32 = 0xf;
pub const C_SINE_MODE_CH2_CTL_MASK_SFT: u32 = (0xf << 12);
pub const C_AMP_DIV_CH1_CTL_SFT: u32 = 9;
pub const C_AMP_DIV_CH1_CTL_MASK: u32 = 0x7;
pub const C_AMP_DIV_CH1_CTL_MASK_SFT: u32 = (0x7 << 9);
pub const C_FREQ_DIV_CH1_CTL_SFT: u32 = 4;
pub const C_FREQ_DIV_CH1_CTL_MASK: u32 = 0x1f;
pub const C_FREQ_DIV_CH1_CTL_MASK_SFT: u32 = (0x1f << 4);
pub const C_SINE_MODE_CH1_CTL_SFT: u32 = 0;
pub const C_SINE_MODE_CH1_CTL_MASK: u32 = 0xf;
pub const C_SINE_MODE_CH1_CTL_MASK_SFT: u32 = (0xf << 0);

/* AFE_ADDA_UL0_SRC_DEBUG */
pub const UL_SLT_CNT_FLAG_RESET_CTL_SFT: u32 = 16;
pub const UL_SLT_CNT_FLAG_RESET_CTL_MASK: u32 = 0x1;
pub const UL_SLT_CNT_FLAG_RESET_CTL_MASK_SFT: u32 = (0x1 << 16);
pub const FIFO_DIGMIC_TESTIN_SFT: u32 = 12;
pub const FIFO_DIGMIC_TESTIN_MASK: u32 = 0x3;
pub const FIFO_DIGMIC_TESTIN_MASK_SFT: u32 = (0x3 << 12);
pub const FIFO_DIGMIC_WDATA_TESTEN_SFT: u32 = 11;
pub const FIFO_DIGMIC_WDATA_TESTEN_MASK: u32 = 0x1;
pub const FIFO_DIGMIC_WDATA_TESTEN_MASK_SFT: u32 = (0x1 << 11);
pub const SLT_CNT_THD_CTL_SFT: u32 = 0;
pub const SLT_CNT_THD_CTL_MASK: u32 = 0x7ff;
pub const SLT_CNT_THD_CTL_MASK_SFT: u32 = (0x7ff << 0);

/* AFE_ADDA_UL0_SRC_DEBUG_MON0 */
pub const SLT_CNT_FLAG_CTL_SFT: u32 = 16;
pub const SLT_CNT_FLAG_CTL_MASK: u32 = 0x1;
pub const SLT_CNT_FLAG_CTL_MASK_SFT: u32 = (0x1 << 16);
pub const SLT_COUNTER_CTL_SFT: u32 = 0;
pub const SLT_COUNTER_CTL_MASK: u32 = 0x7ff;
pub const SLT_COUNTER_CTL_MASK_SFT: u32 = (0x7ff << 0);

/* AFE_ADDA_UL0_IIR_COEF_02_01 */
pub const ADDA_IIR_COEF_02_01_SFT: u32 = 0;
pub const ADDA_IIR_COEF_02_01_MASK: u32 = 0xffffffff;
pub const ADDA_IIR_COEF_02_01_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ADDA_UL0_IIR_COEF_04_03 */
pub const ADDA_IIR_COEF_04_03_SFT: u32 = 0;
pub const ADDA_IIR_COEF_04_03_MASK: u32 = 0xffffffff;
pub const ADDA_IIR_COEF_04_03_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ADDA_UL0_IIR_COEF_06_05 */
pub const ADDA_IIR_COEF_06_05_SFT: u32 = 0;
pub const ADDA_IIR_COEF_06_05_MASK: u32 = 0xffffffff;
pub const ADDA_IIR_COEF_06_05_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ADDA_UL0_IIR_COEF_08_07 */
pub const ADDA_IIR_COEF_08_07_SFT: u32 = 0;
pub const ADDA_IIR_COEF_08_07_MASK: u32 = 0xffffffff;
pub const ADDA_IIR_COEF_08_07_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ADDA_UL0_IIR_COEF_10_09 */
pub const ADDA_IIR_COEF_10_09_SFT: u32 = 0;
pub const ADDA_IIR_COEF_10_09_MASK: u32 = 0xffffffff;
pub const ADDA_IIR_COEF_10_09_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ADDA_UL0_ULCF_CFG_02_01 */
pub const ADDA_ULCF_CFG_02_01_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_02_01_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_02_01_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ADDA_UL0_ULCF_CFG_04_03 */
pub const ADDA_ULCF_CFG_04_03_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_04_03_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_04_03_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ADDA_UL0_ULCF_CFG_06_05 */
pub const ADDA_ULCF_CFG_06_05_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_06_05_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_06_05_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ADDA_UL0_ULCF_CFG_08_07 */
pub const ADDA_ULCF_CFG_08_07_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_08_07_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_08_07_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ADDA_UL0_ULCF_CFG_10_09 */
pub const ADDA_ULCF_CFG_10_09_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_10_09_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_10_09_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ADDA_UL0_ULCF_CFG_12_11 */
pub const ADDA_ULCF_CFG_12_11_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_12_11_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_12_11_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ADDA_UL0_ULCF_CFG_14_13 */
pub const ADDA_ULCF_CFG_14_13_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_14_13_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_14_13_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ADDA_UL0_ULCF_CFG_16_15 */
pub const ADDA_ULCF_CFG_16_15_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_16_15_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_16_15_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ADDA_UL0_ULCF_CFG_18_17 */
pub const ADDA_ULCF_CFG_18_17_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_18_17_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_18_17_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ADDA_UL0_ULCF_CFG_20_19 */
pub const ADDA_ULCF_CFG_20_19_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_20_19_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_20_19_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ADDA_UL0_ULCF_CFG_22_21 */
pub const ADDA_ULCF_CFG_22_21_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_22_21_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_22_21_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ADDA_UL0_ULCF_CFG_24_23 */
pub const ADDA_ULCF_CFG_24_23_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_24_23_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_24_23_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ADDA_UL0_ULCF_CFG_26_25 */
pub const ADDA_ULCF_CFG_26_25_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_26_25_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_26_25_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ADDA_UL0_ULCF_CFG_28_27 */
pub const ADDA_ULCF_CFG_28_27_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_28_27_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_28_27_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ADDA_UL0_ULCF_CFG_30_29 */
pub const ADDA_ULCF_CFG_30_29_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_30_29_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_30_29_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ADDA_UL0_ULCF_CFG_32_31 */
pub const ADDA_ULCF_CFG_32_31_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_32_31_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_32_31_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ADDA_UL0_IP_VERSION */
pub const ADDA_ULCF_IP_VERSION_SFT: u32 = 0;
pub const ADDA_ULCF_IP_VERSION_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_IP_VERSION_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ADDA_UL1_SRC_CON0 */
// duplicate C define preserved: #define ULCF_CFG_EN_CTL_SFT 31
// duplicate C define preserved: #define ULCF_CFG_EN_CTL_MASK 0x1
// duplicate C define preserved: #define ULCF_CFG_EN_CTL_MASK_SFT (0x1 << 31)
// duplicate C define preserved: #define UL_DMIC_PHASE_SEL_CH1_SFT 27
// duplicate C define preserved: #define UL_DMIC_PHASE_SEL_CH1_MASK 0x7
// duplicate C define preserved: #define UL_DMIC_PHASE_SEL_CH1_MASK_SFT (0x7 << 27)
// duplicate C define preserved: #define UL_DMIC_PHASE_SEL_CH2_SFT 24
// duplicate C define preserved: #define UL_DMIC_PHASE_SEL_CH2_MASK 0x7
// duplicate C define preserved: #define UL_DMIC_PHASE_SEL_CH2_MASK_SFT (0x7 << 24)
// duplicate C define preserved: #define UL_DMIC_TWO_WIRE_CTL_SFT 23
// duplicate C define preserved: #define UL_DMIC_TWO_WIRE_CTL_MASK 0x1
// duplicate C define preserved: #define UL_DMIC_TWO_WIRE_CTL_MASK_SFT (0x1 << 23)
// duplicate C define preserved: #define UL_MODE_3P25M_CH2_CTL_SFT 22
// duplicate C define preserved: #define UL_MODE_3P25M_CH2_CTL_MASK 0x1
// duplicate C define preserved: #define UL_MODE_3P25M_CH2_CTL_MASK_SFT (0x1 << 22)
// duplicate C define preserved: #define UL_MODE_3P25M_CH1_CTL_SFT 21
// duplicate C define preserved: #define UL_MODE_3P25M_CH1_CTL_MASK 0x1
// duplicate C define preserved: #define UL_MODE_3P25M_CH1_CTL_MASK_SFT (0x1 << 21)
// duplicate C define preserved: #define UL_VOICE_MODE_CH1_CH2_CTL_SFT 17
// duplicate C define preserved: #define UL_VOICE_MODE_CH1_CH2_CTL_MASK 0x7
// duplicate C define preserved: #define UL_VOICE_MODE_CH1_CH2_CTL_MASK_SFT (0x7 << 17)
// duplicate C define preserved: #define UL_AP_DMIC_ON_SFT 16
// duplicate C define preserved: #define UL_AP_DMIC_ON_MASK 0x1
// duplicate C define preserved: #define UL_AP_DMIC_ON_MASK_SFT (0x1 << 16)
// duplicate C define preserved: #define DMIC_LOW_POWER_MODE_CTL_SFT 14
// duplicate C define preserved: #define DMIC_LOW_POWER_MODE_CTL_MASK 0x3
// duplicate C define preserved: #define DMIC_LOW_POWER_MODE_CTL_MASK_SFT (0x3 << 14)
// duplicate C define preserved: #define UL_DISABLE_HW_CG_CTL_SFT 12
// duplicate C define preserved: #define UL_DISABLE_HW_CG_CTL_MASK 0x1
// duplicate C define preserved: #define UL_DISABLE_HW_CG_CTL_MASK_SFT (0x1 << 12)
// duplicate C define preserved: #define AMIC_26M_SEL_CTL_SFT 11
// duplicate C define preserved: #define AMIC_26M_SEL_CTL_MASK 0x1
// duplicate C define preserved: #define AMIC_26M_SEL_CTL_MASK_SFT (0x1 << 11)
// duplicate C define preserved: #define UL_IIR_ON_TMP_CTL_SFT 10
// duplicate C define preserved: #define UL_IIR_ON_TMP_CTL_MASK 0x1
// duplicate C define preserved: #define UL_IIR_ON_TMP_CTL_MASK_SFT (0x1 << 10)
// duplicate C define preserved: #define UL_IIRMODE_CTL_SFT 7
// duplicate C define preserved: #define UL_IIRMODE_CTL_MASK 0x7
// duplicate C define preserved: #define UL_IIRMODE_CTL_MASK_SFT (0x7 << 7)
// duplicate C define preserved: #define DIGMIC_4P33M_SEL_SFT 6
// duplicate C define preserved: #define DIGMIC_4P33M_SEL_MASK 0x1
// duplicate C define preserved: #define DIGMIC_4P33M_SEL_MASK_SFT (0x1 << 6)
// duplicate C define preserved: #define DIGMIC_3P25M_1P625M_SEL_CTL_SFT 5
// duplicate C define preserved: #define DIGMIC_3P25M_1P625M_SEL_CTL_MASK 0x1
// duplicate C define preserved: #define DIGMIC_3P25M_1P625M_SEL_CTL_MASK_SFT (0x1 << 5)
// duplicate C define preserved: #define AMIC_6P5M_SEL_CTL_SFT 4
// duplicate C define preserved: #define AMIC_6P5M_SEL_CTL_MASK 0x1
// duplicate C define preserved: #define AMIC_6P5M_SEL_CTL_MASK_SFT (0x1 << 4)
// duplicate C define preserved: #define AMIC_1P625M_SEL_CTL_SFT 3
// duplicate C define preserved: #define AMIC_1P625M_SEL_CTL_MASK 0x1
// duplicate C define preserved: #define AMIC_1P625M_SEL_CTL_MASK_SFT (0x1 << 3)
// duplicate C define preserved: #define UL_LOOP_BACK_MODE_CTL_SFT 2
// duplicate C define preserved: #define UL_LOOP_BACK_MODE_CTL_MASK 0x1
// duplicate C define preserved: #define UL_LOOP_BACK_MODE_CTL_MASK_SFT (0x1 << 2)
// duplicate C define preserved: #define UL_SDM_3_LEVEL_CTL_SFT 1
// duplicate C define preserved: #define UL_SDM_3_LEVEL_CTL_MASK 0x1
// duplicate C define preserved: #define UL_SDM_3_LEVEL_CTL_MASK_SFT (0x1 << 1)
// duplicate C define preserved: #define UL_SRC_ON_TMP_CTL_SFT 0
// duplicate C define preserved: #define UL_SRC_ON_TMP_CTL_MASK 0x1
// duplicate C define preserved: #define UL_SRC_ON_TMP_CTL_MASK_SFT (0x1 << 0)

/* AFE_ADDA_UL1_SRC_CON1 */
// duplicate C define preserved: #define ADDA_UL_GAIN_VALUE_SFT 16
// duplicate C define preserved: #define ADDA_UL_GAIN_VALUE_MASK 0xffff
// duplicate C define preserved: #define ADDA_UL_GAIN_VALUE_MASK_SFT (0xffff << 16)
// duplicate C define preserved: #define ADDA_UL_POSTIVEGAIN_SFT 12
// duplicate C define preserved: #define ADDA_UL_POSTIVEGAIN_MASK 0x7
// duplicate C define preserved: #define ADDA_UL_POSTIVEGAIN_MASK_SFT (0x7 << 12)
// duplicate C define preserved: #define ADDA_UL_ODDTAP_MODE_SFT 11
// duplicate C define preserved: #define ADDA_UL_ODDTAP_MODE_MASK 0x1
// duplicate C define preserved: #define ADDA_UL_ODDTAP_MODE_MASK_SFT (0x1 << 11)
// duplicate C define preserved: #define ADDA_UL_HALF_TAP_NUM_SFT 5
// duplicate C define preserved: #define ADDA_UL_HALF_TAP_NUM_MASK 0x3f
// duplicate C define preserved: #define ADDA_UL_HALF_TAP_NUM_MASK_SFT (0x3f << 5)
// duplicate C define preserved: #define FIFO_SOFT_RST_SFT 4
// duplicate C define preserved: #define FIFO_SOFT_RST_MASK 0x1
// duplicate C define preserved: #define FIFO_SOFT_RST_MASK_SFT (0x1 << 4)
// duplicate C define preserved: #define FIFO_SOFT_RST_EN_SFT 3
// duplicate C define preserved: #define FIFO_SOFT_RST_EN_MASK 0x1
// duplicate C define preserved: #define FIFO_SOFT_RST_EN_MASK_SFT (0x1 << 3)
// duplicate C define preserved: #define LR_SWAP_SFT 2
// duplicate C define preserved: #define LR_SWAP_MASK 0x1
// duplicate C define preserved: #define LR_SWAP_MASK_SFT (0x1 << 2)
// duplicate C define preserved: #define GAIN_MODE_SFT 0
// duplicate C define preserved: #define GAIN_MODE_MASK 0x3
// duplicate C define preserved: #define GAIN_MODE_MASK_SFT (0x3 << 0)

/* AFE_ADDA_UL1_SRC_CON2 */
// duplicate C define preserved: #define C_DAC_EN_CTL_SFT 27
// duplicate C define preserved: #define C_DAC_EN_CTL_MASK 0x1
// duplicate C define preserved: #define C_DAC_EN_CTL_MASK_SFT (0x1 << 27)
// duplicate C define preserved: #define C_MUTE_SW_CTL_SFT 26
// duplicate C define preserved: #define C_MUTE_SW_CTL_MASK 0x1
// duplicate C define preserved: #define C_MUTE_SW_CTL_MASK_SFT (0x1 << 26)
// duplicate C define preserved: #define C_AMP_DIV_CH2_CTL_SFT 21
// duplicate C define preserved: #define C_AMP_DIV_CH2_CTL_MASK 0x7
// duplicate C define preserved: #define C_AMP_DIV_CH2_CTL_MASK_SFT (0x7 << 21)
// duplicate C define preserved: #define C_FREQ_DIV_CH2_CTL_SFT 16
// duplicate C define preserved: #define C_FREQ_DIV_CH2_CTL_MASK 0x1f
// duplicate C define preserved: #define C_FREQ_DIV_CH2_CTL_MASK_SFT (0x1f << 16)
// duplicate C define preserved: #define C_SINE_MODE_CH2_CTL_SFT 12
// duplicate C define preserved: #define C_SINE_MODE_CH2_CTL_MASK 0xf
// duplicate C define preserved: #define C_SINE_MODE_CH2_CTL_MASK_SFT (0xf << 12)
// duplicate C define preserved: #define C_AMP_DIV_CH1_CTL_SFT 9
// duplicate C define preserved: #define C_AMP_DIV_CH1_CTL_MASK 0x7
// duplicate C define preserved: #define C_AMP_DIV_CH1_CTL_MASK_SFT (0x7 << 9)
// duplicate C define preserved: #define C_FREQ_DIV_CH1_CTL_SFT 4
// duplicate C define preserved: #define C_FREQ_DIV_CH1_CTL_MASK 0x1f
// duplicate C define preserved: #define C_FREQ_DIV_CH1_CTL_MASK_SFT (0x1f << 4)
// duplicate C define preserved: #define C_SINE_MODE_CH1_CTL_SFT 0
// duplicate C define preserved: #define C_SINE_MODE_CH1_CTL_MASK 0xf
// duplicate C define preserved: #define C_SINE_MODE_CH1_CTL_MASK_SFT (0xf << 0)

/* AFE_ADDA_UL1_SRC_DEBUG */
// duplicate C define preserved: #define UL_SLT_CNT_FLAG_RESET_CTL_SFT 16
// duplicate C define preserved: #define UL_SLT_CNT_FLAG_RESET_CTL_MASK 0x1
// duplicate C define preserved: #define UL_SLT_CNT_FLAG_RESET_CTL_MASK_SFT (0x1 << 16)
// duplicate C define preserved: #define FIFO_DIGMIC_TESTIN_SFT 12
// duplicate C define preserved: #define FIFO_DIGMIC_TESTIN_MASK 0x3
// duplicate C define preserved: #define FIFO_DIGMIC_TESTIN_MASK_SFT (0x3 << 12)
// duplicate C define preserved: #define FIFO_DIGMIC_WDATA_TESTEN_SFT 11
// duplicate C define preserved: #define FIFO_DIGMIC_WDATA_TESTEN_MASK 0x1
// duplicate C define preserved: #define FIFO_DIGMIC_WDATA_TESTEN_MASK_SFT (0x1 << 11)
// duplicate C define preserved: #define SLT_CNT_THD_CTL_SFT 0
// duplicate C define preserved: #define SLT_CNT_THD_CTL_MASK 0x7ff
// duplicate C define preserved: #define SLT_CNT_THD_CTL_MASK_SFT (0x7ff << 0)

/* AFE_ADDA_UL1_SRC_DEBUG_MON0 */
// duplicate C define preserved: #define SLT_CNT_FLAG_CTL_SFT 16
// duplicate C define preserved: #define SLT_CNT_FLAG_CTL_MASK 0x1
// duplicate C define preserved: #define SLT_CNT_FLAG_CTL_MASK_SFT (0x1 << 16)
// duplicate C define preserved: #define SLT_COUNTER_CTL_SFT 0
// duplicate C define preserved: #define SLT_COUNTER_CTL_MASK 0x7ff
// duplicate C define preserved: #define SLT_COUNTER_CTL_MASK_SFT (0x7ff << 0)

/* AFE_ADDA_UL1_IIR_COEF_02_01 */
// duplicate C define preserved: #define ADDA_IIR_COEF_02_01_SFT 0
// duplicate C define preserved: #define ADDA_IIR_COEF_02_01_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_IIR_COEF_02_01_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL1_IIR_COEF_04_03 */
// duplicate C define preserved: #define ADDA_IIR_COEF_04_03_SFT 0
// duplicate C define preserved: #define ADDA_IIR_COEF_04_03_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_IIR_COEF_04_03_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL1_IIR_COEF_06_05 */
// duplicate C define preserved: #define ADDA_IIR_COEF_06_05_SFT 0
// duplicate C define preserved: #define ADDA_IIR_COEF_06_05_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_IIR_COEF_06_05_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL1_IIR_COEF_08_07 */
// duplicate C define preserved: #define ADDA_IIR_COEF_08_07_SFT 0
// duplicate C define preserved: #define ADDA_IIR_COEF_08_07_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_IIR_COEF_08_07_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL1_IIR_COEF_10_09 */
// duplicate C define preserved: #define ADDA_IIR_COEF_10_09_SFT 0
// duplicate C define preserved: #define ADDA_IIR_COEF_10_09_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_IIR_COEF_10_09_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL1_ULCF_CFG_02_01 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_02_01_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_02_01_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_02_01_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL1_ULCF_CFG_04_03 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_04_03_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_04_03_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_04_03_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL1_ULCF_CFG_06_05 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_06_05_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_06_05_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_06_05_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL1_ULCF_CFG_08_07 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_08_07_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_08_07_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_08_07_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL1_ULCF_CFG_10_09 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_10_09_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_10_09_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_10_09_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL1_ULCF_CFG_12_11 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_12_11_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_12_11_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_12_11_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL1_ULCF_CFG_14_13 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_14_13_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_14_13_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_14_13_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL1_ULCF_CFG_16_15 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_16_15_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_16_15_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_16_15_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL1_ULCF_CFG_18_17 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_18_17_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_18_17_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_18_17_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL1_ULCF_CFG_20_19 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_20_19_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_20_19_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_20_19_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL1_ULCF_CFG_22_21 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_22_21_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_22_21_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_22_21_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL1_ULCF_CFG_24_23 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_24_23_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_24_23_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_24_23_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL1_ULCF_CFG_26_25 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_26_25_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_26_25_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_26_25_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL1_ULCF_CFG_28_27 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_28_27_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_28_27_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_28_27_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL1_ULCF_CFG_30_29 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_30_29_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_30_29_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_30_29_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL1_ULCF_CFG_32_31 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_32_31_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_32_31_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_32_31_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL1_IP_VERSION */
// duplicate C define preserved: #define ADDA_ULCF_IP_VERSION_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_IP_VERSION_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_IP_VERSION_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL2_SRC_CON0 */
// duplicate C define preserved: #define ULCF_CFG_EN_CTL_SFT 31
// duplicate C define preserved: #define ULCF_CFG_EN_CTL_MASK 0x1
// duplicate C define preserved: #define ULCF_CFG_EN_CTL_MASK_SFT (0x1 << 31)
// duplicate C define preserved: #define UL_DMIC_PHASE_SEL_CH1_SFT 27
// duplicate C define preserved: #define UL_DMIC_PHASE_SEL_CH1_MASK 0x7
// duplicate C define preserved: #define UL_DMIC_PHASE_SEL_CH1_MASK_SFT (0x7 << 27)
// duplicate C define preserved: #define UL_DMIC_PHASE_SEL_CH2_SFT 24
// duplicate C define preserved: #define UL_DMIC_PHASE_SEL_CH2_MASK 0x7
// duplicate C define preserved: #define UL_DMIC_PHASE_SEL_CH2_MASK_SFT (0x7 << 24)
// duplicate C define preserved: #define UL_DMIC_TWO_WIRE_CTL_SFT 23
// duplicate C define preserved: #define UL_DMIC_TWO_WIRE_CTL_MASK 0x1
// duplicate C define preserved: #define UL_DMIC_TWO_WIRE_CTL_MASK_SFT (0x1 << 23)
// duplicate C define preserved: #define UL_MODE_3P25M_CH2_CTL_SFT 22
// duplicate C define preserved: #define UL_MODE_3P25M_CH2_CTL_MASK 0x1
// duplicate C define preserved: #define UL_MODE_3P25M_CH2_CTL_MASK_SFT (0x1 << 22)
// duplicate C define preserved: #define UL_MODE_3P25M_CH1_CTL_SFT 21
// duplicate C define preserved: #define UL_MODE_3P25M_CH1_CTL_MASK 0x1
// duplicate C define preserved: #define UL_MODE_3P25M_CH1_CTL_MASK_SFT (0x1 << 21)
// duplicate C define preserved: #define UL_VOICE_MODE_CH1_CH2_CTL_SFT 17
// duplicate C define preserved: #define UL_VOICE_MODE_CH1_CH2_CTL_MASK 0x7
// duplicate C define preserved: #define UL_VOICE_MODE_CH1_CH2_CTL_MASK_SFT (0x7 << 17)
// duplicate C define preserved: #define UL_AP_DMIC_ON_SFT 16
// duplicate C define preserved: #define UL_AP_DMIC_ON_MASK 0x1
// duplicate C define preserved: #define UL_AP_DMIC_ON_MASK_SFT (0x1 << 16)
// duplicate C define preserved: #define DMIC_LOW_POWER_MODE_CTL_SFT 14
// duplicate C define preserved: #define DMIC_LOW_POWER_MODE_CTL_MASK 0x3
// duplicate C define preserved: #define DMIC_LOW_POWER_MODE_CTL_MASK_SFT (0x3 << 14)
// duplicate C define preserved: #define UL_DISABLE_HW_CG_CTL_SFT 12
// duplicate C define preserved: #define UL_DISABLE_HW_CG_CTL_MASK 0x1
// duplicate C define preserved: #define UL_DISABLE_HW_CG_CTL_MASK_SFT (0x1 << 12)
// duplicate C define preserved: #define AMIC_26M_SEL_CTL_SFT 11
// duplicate C define preserved: #define AMIC_26M_SEL_CTL_MASK 0x1
// duplicate C define preserved: #define AMIC_26M_SEL_CTL_MASK_SFT (0x1 << 11)
// duplicate C define preserved: #define UL_IIR_ON_TMP_CTL_SFT 10
// duplicate C define preserved: #define UL_IIR_ON_TMP_CTL_MASK 0x1
// duplicate C define preserved: #define UL_IIR_ON_TMP_CTL_MASK_SFT (0x1 << 10)
// duplicate C define preserved: #define UL_IIRMODE_CTL_SFT 7
// duplicate C define preserved: #define UL_IIRMODE_CTL_MASK 0x7
// duplicate C define preserved: #define UL_IIRMODE_CTL_MASK_SFT (0x7 << 7)
// duplicate C define preserved: #define DIGMIC_4P33M_SEL_SFT 6
// duplicate C define preserved: #define DIGMIC_4P33M_SEL_MASK 0x1
// duplicate C define preserved: #define DIGMIC_4P33M_SEL_MASK_SFT (0x1 << 6)
// duplicate C define preserved: #define DIGMIC_3P25M_1P625M_SEL_CTL_SFT 5
// duplicate C define preserved: #define DIGMIC_3P25M_1P625M_SEL_CTL_MASK 0x1
// duplicate C define preserved: #define DIGMIC_3P25M_1P625M_SEL_CTL_MASK_SFT (0x1 << 5)
// duplicate C define preserved: #define AMIC_6P5M_SEL_CTL_SFT 4
// duplicate C define preserved: #define AMIC_6P5M_SEL_CTL_MASK 0x1
// duplicate C define preserved: #define AMIC_6P5M_SEL_CTL_MASK_SFT (0x1 << 4)
// duplicate C define preserved: #define AMIC_1P625M_SEL_CTL_SFT 3
// duplicate C define preserved: #define AMIC_1P625M_SEL_CTL_MASK 0x1
// duplicate C define preserved: #define AMIC_1P625M_SEL_CTL_MASK_SFT (0x1 << 3)
// duplicate C define preserved: #define UL_LOOP_BACK_MODE_CTL_SFT 2
// duplicate C define preserved: #define UL_LOOP_BACK_MODE_CTL_MASK 0x1
// duplicate C define preserved: #define UL_LOOP_BACK_MODE_CTL_MASK_SFT (0x1 << 2)
// duplicate C define preserved: #define UL_SDM_3_LEVEL_CTL_SFT 1
// duplicate C define preserved: #define UL_SDM_3_LEVEL_CTL_MASK 0x1
// duplicate C define preserved: #define UL_SDM_3_LEVEL_CTL_MASK_SFT (0x1 << 1)
// duplicate C define preserved: #define UL_SRC_ON_TMP_CTL_SFT 0
// duplicate C define preserved: #define UL_SRC_ON_TMP_CTL_MASK 0x1
// duplicate C define preserved: #define UL_SRC_ON_TMP_CTL_MASK_SFT (0x1 << 0)

/* AFE_ADDA_UL2_SRC_CON1 */
// duplicate C define preserved: #define ADDA_UL_GAIN_VALUE_SFT 16
// duplicate C define preserved: #define ADDA_UL_GAIN_VALUE_MASK 0xffff
// duplicate C define preserved: #define ADDA_UL_GAIN_VALUE_MASK_SFT (0xffff << 16)
// duplicate C define preserved: #define ADDA_UL_POSTIVEGAIN_SFT 12
// duplicate C define preserved: #define ADDA_UL_POSTIVEGAIN_MASK 0x7
// duplicate C define preserved: #define ADDA_UL_POSTIVEGAIN_MASK_SFT (0x7 << 12)
// duplicate C define preserved: #define ADDA_UL_ODDTAP_MODE_SFT 11
// duplicate C define preserved: #define ADDA_UL_ODDTAP_MODE_MASK 0x1
// duplicate C define preserved: #define ADDA_UL_ODDTAP_MODE_MASK_SFT (0x1 << 11)
// duplicate C define preserved: #define ADDA_UL_HALF_TAP_NUM_SFT 5
// duplicate C define preserved: #define ADDA_UL_HALF_TAP_NUM_MASK 0x3f
// duplicate C define preserved: #define ADDA_UL_HALF_TAP_NUM_MASK_SFT (0x3f << 5)
// duplicate C define preserved: #define FIFO_SOFT_RST_SFT 4
// duplicate C define preserved: #define FIFO_SOFT_RST_MASK 0x1
// duplicate C define preserved: #define FIFO_SOFT_RST_MASK_SFT (0x1 << 4)
// duplicate C define preserved: #define FIFO_SOFT_RST_EN_SFT 3
// duplicate C define preserved: #define FIFO_SOFT_RST_EN_MASK 0x1
// duplicate C define preserved: #define FIFO_SOFT_RST_EN_MASK_SFT (0x1 << 3)
// duplicate C define preserved: #define LR_SWAP_SFT 2
// duplicate C define preserved: #define LR_SWAP_MASK 0x1
// duplicate C define preserved: #define LR_SWAP_MASK_SFT (0x1 << 2)
// duplicate C define preserved: #define GAIN_MODE_SFT 0
// duplicate C define preserved: #define GAIN_MODE_MASK 0x3
// duplicate C define preserved: #define GAIN_MODE_MASK_SFT (0x3 << 0)

/* AFE_ADDA_UL2_SRC_CON2 */
// duplicate C define preserved: #define C_DAC_EN_CTL_SFT 27
// duplicate C define preserved: #define C_DAC_EN_CTL_MASK 0x1
// duplicate C define preserved: #define C_DAC_EN_CTL_MASK_SFT (0x1 << 27)
// duplicate C define preserved: #define C_MUTE_SW_CTL_SFT 26
// duplicate C define preserved: #define C_MUTE_SW_CTL_MASK 0x1
// duplicate C define preserved: #define C_MUTE_SW_CTL_MASK_SFT (0x1 << 26)
// duplicate C define preserved: #define C_AMP_DIV_CH2_CTL_SFT 21
// duplicate C define preserved: #define C_AMP_DIV_CH2_CTL_MASK 0x7
// duplicate C define preserved: #define C_AMP_DIV_CH2_CTL_MASK_SFT (0x7 << 21)
// duplicate C define preserved: #define C_FREQ_DIV_CH2_CTL_SFT 16
// duplicate C define preserved: #define C_FREQ_DIV_CH2_CTL_MASK 0x1f
// duplicate C define preserved: #define C_FREQ_DIV_CH2_CTL_MASK_SFT (0x1f << 16)
// duplicate C define preserved: #define C_SINE_MODE_CH2_CTL_SFT 12
// duplicate C define preserved: #define C_SINE_MODE_CH2_CTL_MASK 0xf
// duplicate C define preserved: #define C_SINE_MODE_CH2_CTL_MASK_SFT (0xf << 12)
// duplicate C define preserved: #define C_AMP_DIV_CH1_CTL_SFT 9
// duplicate C define preserved: #define C_AMP_DIV_CH1_CTL_MASK 0x7
// duplicate C define preserved: #define C_AMP_DIV_CH1_CTL_MASK_SFT (0x7 << 9)
// duplicate C define preserved: #define C_FREQ_DIV_CH1_CTL_SFT 4
// duplicate C define preserved: #define C_FREQ_DIV_CH1_CTL_MASK 0x1f
// duplicate C define preserved: #define C_FREQ_DIV_CH1_CTL_MASK_SFT (0x1f << 4)
// duplicate C define preserved: #define C_SINE_MODE_CH1_CTL_SFT 0
// duplicate C define preserved: #define C_SINE_MODE_CH1_CTL_MASK 0xf
// duplicate C define preserved: #define C_SINE_MODE_CH1_CTL_MASK_SFT (0xf << 0)

/* AFE_ADDA_UL2_SRC_DEBUG */
// duplicate C define preserved: #define UL_SLT_CNT_FLAG_RESET_CTL_SFT 16
// duplicate C define preserved: #define UL_SLT_CNT_FLAG_RESET_CTL_MASK 0x1
// duplicate C define preserved: #define UL_SLT_CNT_FLAG_RESET_CTL_MASK_SFT (0x1 << 16)
// duplicate C define preserved: #define FIFO_DIGMIC_TESTIN_SFT 12
// duplicate C define preserved: #define FIFO_DIGMIC_TESTIN_MASK 0x3
// duplicate C define preserved: #define FIFO_DIGMIC_TESTIN_MASK_SFT (0x3 << 12)
// duplicate C define preserved: #define FIFO_DIGMIC_WDATA_TESTEN_SFT 11
// duplicate C define preserved: #define FIFO_DIGMIC_WDATA_TESTEN_MASK 0x1
// duplicate C define preserved: #define FIFO_DIGMIC_WDATA_TESTEN_MASK_SFT (0x1 << 11)
// duplicate C define preserved: #define SLT_CNT_THD_CTL_SFT 0
// duplicate C define preserved: #define SLT_CNT_THD_CTL_MASK 0x7ff
// duplicate C define preserved: #define SLT_CNT_THD_CTL_MASK_SFT (0x7ff << 0)

/* AFE_ADDA_UL2_SRC_DEBUG_MON0 */
// duplicate C define preserved: #define SLT_CNT_FLAG_CTL_SFT 16
// duplicate C define preserved: #define SLT_CNT_FLAG_CTL_MASK 0x1
// duplicate C define preserved: #define SLT_CNT_FLAG_CTL_MASK_SFT (0x1 << 16)
// duplicate C define preserved: #define SLT_COUNTER_CTL_SFT 0
// duplicate C define preserved: #define SLT_COUNTER_CTL_MASK 0x7ff
// duplicate C define preserved: #define SLT_COUNTER_CTL_MASK_SFT (0x7ff << 0)

/* AFE_ADDA_UL2_IIR_COEF_02_01 */
// duplicate C define preserved: #define ADDA_IIR_COEF_02_01_SFT 0
// duplicate C define preserved: #define ADDA_IIR_COEF_02_01_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_IIR_COEF_02_01_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL2_IIR_COEF_04_03 */
// duplicate C define preserved: #define ADDA_IIR_COEF_04_03_SFT 0
// duplicate C define preserved: #define ADDA_IIR_COEF_04_03_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_IIR_COEF_04_03_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL2_IIR_COEF_06_05 */
// duplicate C define preserved: #define ADDA_IIR_COEF_06_05_SFT 0
// duplicate C define preserved: #define ADDA_IIR_COEF_06_05_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_IIR_COEF_06_05_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL2_IIR_COEF_08_07 */
// duplicate C define preserved: #define ADDA_IIR_COEF_08_07_SFT 0
// duplicate C define preserved: #define ADDA_IIR_COEF_08_07_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_IIR_COEF_08_07_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL2_IIR_COEF_10_09 */
// duplicate C define preserved: #define ADDA_IIR_COEF_10_09_SFT 0
// duplicate C define preserved: #define ADDA_IIR_COEF_10_09_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_IIR_COEF_10_09_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL2_ULCF_CFG_02_01 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_02_01_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_02_01_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_02_01_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL2_ULCF_CFG_04_03 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_04_03_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_04_03_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_04_03_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL2_ULCF_CFG_06_05 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_06_05_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_06_05_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_06_05_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL2_ULCF_CFG_08_07 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_08_07_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_08_07_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_08_07_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL2_ULCF_CFG_10_09 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_10_09_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_10_09_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_10_09_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL2_ULCF_CFG_12_11 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_12_11_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_12_11_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_12_11_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL2_ULCF_CFG_14_13 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_14_13_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_14_13_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_14_13_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL2_ULCF_CFG_16_15 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_16_15_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_16_15_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_16_15_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL2_ULCF_CFG_18_17 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_18_17_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_18_17_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_18_17_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL2_ULCF_CFG_20_19 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_20_19_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_20_19_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_20_19_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL2_ULCF_CFG_22_21 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_22_21_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_22_21_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_22_21_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL2_ULCF_CFG_24_23 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_24_23_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_24_23_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_24_23_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL2_ULCF_CFG_26_25 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_26_25_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_26_25_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_26_25_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL2_ULCF_CFG_28_27 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_28_27_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_28_27_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_28_27_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL2_ULCF_CFG_30_29 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_30_29_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_30_29_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_30_29_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL2_ULCF_CFG_32_31 */
// duplicate C define preserved: #define ADDA_ULCF_CFG_32_31_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_CFG_32_31_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_CFG_32_31_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_UL2_IP_VERSION */
// duplicate C define preserved: #define ADDA_ULCF_IP_VERSION_SFT 0
// duplicate C define preserved: #define ADDA_ULCF_IP_VERSION_MASK 0xffffffff
// duplicate C define preserved: #define ADDA_ULCF_IP_VERSION_MASK_SFT (0xffffffff << 0)

/* AFE_ADDA_PROXIMITY_CON0 */
pub const PROXIMITY_CH1_ON_SFT: u32 = 12;
pub const PROXIMITY_CH1_ON_MASK: u32 = 0x1;
pub const PROXIMITY_CH1_ON_MASK_SFT: u32 = (0x1 << 12);
pub const PROXIMITY_CH1_SEL_SFT: u32 = 8;
pub const PROXIMITY_CH1_SEL_MASK: u32 = 0xf;
pub const PROXIMITY_CH1_SEL_MASK_SFT: u32 = (0xf << 8);
pub const PROXIMITY_CH2_ON_SFT: u32 = 4;
pub const PROXIMITY_CH2_ON_MASK: u32 = 0x1;
pub const PROXIMITY_CH2_ON_MASK_SFT: u32 = (0x1 << 4);
pub const PROXIMITY_CH2_SEL_SFT: u32 = 0;
pub const PROXIMITY_CH2_SEL_MASK: u32 = 0xf;
pub const PROXIMITY_CH2_SEL_MASK_SFT: u32 = (0xf << 0);

/* AFE_ADDA_ULSRC_PHASE_CON0 */
pub const DMIC1_PHASE_FCLK_SEL_SFT: u32 = 30;
pub const DMIC1_PHASE_FCLK_SEL_MASK: u32 = 0x3;
pub const DMIC1_PHASE_FCLK_SEL_MASK_SFT: u32 = (0x3 << 30);
pub const DMIC0_PHASE_FCLK_SEL_SFT: u32 = 28;
pub const DMIC0_PHASE_FCLK_SEL_MASK: u32 = 0x3;
pub const DMIC0_PHASE_FCLK_SEL_MASK_SFT: u32 = (0x3 << 28);
pub const UL3_PHASE_FCLK_SEL_SFT: u32 = 26;
pub const UL3_PHASE_FCLK_SEL_MASK: u32 = 0x3;
pub const UL3_PHASE_FCLK_SEL_MASK_SFT: u32 = (0x3 << 26);
pub const UL2_PHASE_FCLK_SEL_SFT: u32 = 24;
pub const UL2_PHASE_FCLK_SEL_MASK: u32 = 0x3;
pub const UL2_PHASE_FCLK_SEL_MASK_SFT: u32 = (0x3 << 24);
pub const UL1_PHASE_FCLK_SEL_SFT: u32 = 22;
pub const UL1_PHASE_FCLK_SEL_MASK: u32 = 0x3;
pub const UL1_PHASE_FCLK_SEL_MASK_SFT: u32 = (0x3 << 22);
pub const UL0_PHASE_FCLK_SEL_SFT: u32 = 20;
pub const UL0_PHASE_FCLK_SEL_MASK: u32 = 0x3;
pub const UL0_PHASE_FCLK_SEL_MASK_SFT: u32 = (0x3 << 20);
pub const UL_PHASE_SYNC_FCLK_2_ON_SFT: u32 = 18;
pub const UL_PHASE_SYNC_FCLK_2_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_FCLK_2_ON_MASK_SFT: u32 = (0x1 << 18);
pub const UL_PHASE_SYNC_FCLK_1_ON_SFT: u32 = 17;
pub const UL_PHASE_SYNC_FCLK_1_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_FCLK_1_ON_MASK_SFT: u32 = (0x1 << 17);
pub const UL_PHASE_SYNC_FCLK_0_ON_SFT: u32 = 16;
pub const UL_PHASE_SYNC_FCLK_0_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_FCLK_0_ON_MASK_SFT: u32 = (0x1 << 16);
pub const DMIC1_PHASE_HCLK_SEL_SFT: u32 = 14;
pub const DMIC1_PHASE_HCLK_SEL_MASK: u32 = 0x3;
pub const DMIC1_PHASE_HCLK_SEL_MASK_SFT: u32 = (0x3 << 14);
pub const DMIC0_PHASE_HCLK_SEL_SFT: u32 = 12;
pub const DMIC0_PHASE_HCLK_SEL_MASK: u32 = 0x3;
pub const DMIC0_PHASE_HCLK_SEL_MASK_SFT: u32 = (0x3 << 12);
pub const UL3_PHASE_HCLK_SEL_SFT: u32 = 10;
pub const UL3_PHASE_HCLK_SEL_MASK: u32 = 0x3;
pub const UL3_PHASE_HCLK_SEL_MASK_SFT: u32 = (0x3 << 10);
pub const UL2_PHASE_HCLK_SEL_SFT: u32 = 8;
pub const UL2_PHASE_HCLK_SEL_MASK: u32 = 0x3;
pub const UL2_PHASE_HCLK_SEL_MASK_SFT: u32 = (0x3 << 8);
pub const UL1_PHASE_HCLK_SEL_SFT: u32 = 6;
pub const UL1_PHASE_HCLK_SEL_MASK: u32 = 0x3;
pub const UL1_PHASE_HCLK_SEL_MASK_SFT: u32 = (0x3 << 6);
pub const UL0_PHASE_HCLK_SEL_SFT: u32 = 4;
pub const UL0_PHASE_HCLK_SEL_MASK: u32 = 0x3;
pub const UL0_PHASE_HCLK_SEL_MASK_SFT: u32 = (0x3 << 4);
pub const UL_PHASE_SYNC_HCLK_2_ON_SFT: u32 = 2;
pub const UL_PHASE_SYNC_HCLK_2_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_HCLK_2_ON_MASK_SFT: u32 = (0x1 << 2);
pub const UL_PHASE_SYNC_HCLK_1_ON_SFT: u32 = 1;
pub const UL_PHASE_SYNC_HCLK_1_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_HCLK_1_ON_MASK_SFT: u32 = (0x1 << 1);
pub const UL_PHASE_SYNC_HCLK_0_ON_SFT: u32 = 0;
pub const UL_PHASE_SYNC_HCLK_0_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_HCLK_0_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_ADDA_ULSRC_PHASE_CON1 */
pub const DMIC_CLK_PHASE_SYNC_SET_SFT: u32 = 31;
pub const DMIC_CLK_PHASE_SYNC_SET_MASK: u32 = 0x1;
pub const DMIC_CLK_PHASE_SYNC_SET_MASK_SFT: u32 = (0x1 << 31);
pub const DMIC1_PHASE_SYNC_FCLK_SET_SFT: u32 = 11;
pub const DMIC1_PHASE_SYNC_FCLK_SET_MASK: u32 = 0x1;
pub const DMIC1_PHASE_SYNC_FCLK_SET_MASK_SFT: u32 = (0x1 << 11);
pub const DMIC1_PHASE_SYNC_HCLK_SET_SFT: u32 = 10;
pub const DMIC1_PHASE_SYNC_HCLK_SET_MASK: u32 = 0x1;
pub const DMIC1_PHASE_SYNC_HCLK_SET_MASK_SFT: u32 = (0x1 << 10);
pub const DMIC0_PHASE_SYNC_FCLK_SET_SFT: u32 = 9;
pub const DMIC0_PHASE_SYNC_FCLK_SET_MASK: u32 = 0x1;
pub const DMIC0_PHASE_SYNC_FCLK_SET_MASK_SFT: u32 = (0x1 << 9);
pub const DMIC0_PHASE_SYNC_HCLK_SET_SFT: u32 = 8;
pub const DMIC0_PHASE_SYNC_HCLK_SET_MASK: u32 = 0x1;
pub const DMIC0_PHASE_SYNC_HCLK_SET_MASK_SFT: u32 = (0x1 << 8);
pub const UL3_PHASE_SYNC_FCLK_SET_SFT: u32 = 7;
pub const UL3_PHASE_SYNC_FCLK_SET_MASK: u32 = 0x1;
pub const UL3_PHASE_SYNC_FCLK_SET_MASK_SFT: u32 = (0x1 << 7);
pub const UL3_PHASE_SYNC_HCLK_SET_SFT: u32 = 6;
pub const UL3_PHASE_SYNC_HCLK_SET_MASK: u32 = 0x1;
pub const UL3_PHASE_SYNC_HCLK_SET_MASK_SFT: u32 = (0x1 << 6);
pub const UL2_PHASE_SYNC_FCLK_SET_SFT: u32 = 5;
pub const UL2_PHASE_SYNC_FCLK_SET_MASK: u32 = 0x1;
pub const UL2_PHASE_SYNC_FCLK_SET_MASK_SFT: u32 = (0x1 << 5);
pub const UL2_PHASE_SYNC_HCLK_SET_SFT: u32 = 4;
pub const UL2_PHASE_SYNC_HCLK_SET_MASK: u32 = 0x1;
pub const UL2_PHASE_SYNC_HCLK_SET_MASK_SFT: u32 = (0x1 << 4);
pub const UL1_PHASE_SYNC_FCLK_SET_SFT: u32 = 3;
pub const UL1_PHASE_SYNC_FCLK_SET_MASK: u32 = 0x1;
pub const UL1_PHASE_SYNC_FCLK_SET_MASK_SFT: u32 = (0x1 << 3);
pub const UL1_PHASE_SYNC_HCLK_SET_SFT: u32 = 2;
pub const UL1_PHASE_SYNC_HCLK_SET_MASK: u32 = 0x1;
pub const UL1_PHASE_SYNC_HCLK_SET_MASK_SFT: u32 = (0x1 << 2);
pub const UL0_PHASE_SYNC_FCLK_SET_SFT: u32 = 1;
pub const UL0_PHASE_SYNC_FCLK_SET_MASK: u32 = 0x1;
pub const UL0_PHASE_SYNC_FCLK_SET_MASK_SFT: u32 = (0x1 << 1);
pub const UL0_PHASE_SYNC_HCLK_SET_SFT: u32 = 0;
pub const UL0_PHASE_SYNC_HCLK_SET_MASK: u32 = 0x1;
pub const UL0_PHASE_SYNC_HCLK_SET_MASK_SFT: u32 = (0x1 << 0);

/* AFE_ADDA_ULSRC_PHASE_CON2 */
pub const DMIC1_PHASE_SYNC_1X_EN_SEL_SFT: u32 = 26;
pub const DMIC1_PHASE_SYNC_1X_EN_SEL_MASK: u32 = 0x3;
pub const DMIC1_PHASE_SYNC_1X_EN_SEL_MASK_SFT: u32 = (0x3 << 26);
pub const DMIC0_PHASE_SYNC_1X_EN_SEL_SFT: u32 = 24;
pub const DMIC0_PHASE_SYNC_1X_EN_SEL_MASK: u32 = 0x3;
pub const DMIC0_PHASE_SYNC_1X_EN_SEL_MASK_SFT: u32 = (0x3 << 24);
pub const UL3_PHASE_SYNC_1X_EN_SEL_SFT: u32 = 22;
pub const UL3_PHASE_SYNC_1X_EN_SEL_MASK: u32 = 0x3;
pub const UL3_PHASE_SYNC_1X_EN_SEL_MASK_SFT: u32 = (0x3 << 22);
pub const UL2_PHASE_SYNC_1X_EN_SEL_SFT: u32 = 20;
pub const UL2_PHASE_SYNC_1X_EN_SEL_MASK: u32 = 0x3;
pub const UL2_PHASE_SYNC_1X_EN_SEL_MASK_SFT: u32 = (0x3 << 20);
pub const UL1_PHASE_SYNC_1X_EN_SEL_SFT: u32 = 18;
pub const UL1_PHASE_SYNC_1X_EN_SEL_MASK: u32 = 0x3;
pub const UL1_PHASE_SYNC_1X_EN_SEL_MASK_SFT: u32 = (0x3 << 18);
pub const UL0_PHASE_SYNC_1X_EN_SEL_SFT: u32 = 16;
pub const UL0_PHASE_SYNC_1X_EN_SEL_MASK: u32 = 0x3;
pub const UL0_PHASE_SYNC_1X_EN_SEL_MASK_SFT: u32 = (0x3 << 16);
pub const UL_PHASE_SYNC_FCLK_1X_EN_2_ON_SFT: u32 = 5;
pub const UL_PHASE_SYNC_FCLK_1X_EN_2_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_FCLK_1X_EN_2_ON_MASK_SFT: u32 = (0x1 << 5);
pub const UL_PHASE_SYNC_FCLK_1X_EN_1_ON_SFT: u32 = 4;
pub const UL_PHASE_SYNC_FCLK_1X_EN_1_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_FCLK_1X_EN_1_ON_MASK_SFT: u32 = (0x1 << 4);
pub const UL_PHASE_SYNC_FCLK_1X_EN_0_ON_SFT: u32 = 3;
pub const UL_PHASE_SYNC_FCLK_1X_EN_0_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_FCLK_1X_EN_0_ON_MASK_SFT: u32 = (0x1 << 3);
pub const UL_PHASE_SYNC_HCLK_1X_EN_2_ON_SFT: u32 = 2;
pub const UL_PHASE_SYNC_HCLK_1X_EN_2_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_HCLK_1X_EN_2_ON_MASK_SFT: u32 = (0x1 << 2);
pub const UL_PHASE_SYNC_HCLK_1X_EN_1_ON_SFT: u32 = 1;
pub const UL_PHASE_SYNC_HCLK_1X_EN_1_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_HCLK_1X_EN_1_ON_MASK_SFT: u32 = (0x1 << 1);
pub const UL_PHASE_SYNC_HCLK_1X_EN_0_ON_SFT: u32 = 0;
pub const UL_PHASE_SYNC_HCLK_1X_EN_0_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_HCLK_1X_EN_0_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_ADDA_ULSRC_PHASE_CON3 */
pub const DMIC1_PHASE_SYNC_SOFT_RST_SEL_SFT: u32 = 26;
pub const DMIC1_PHASE_SYNC_SOFT_RST_SEL_MASK: u32 = 0x3;
pub const DMIC1_PHASE_SYNC_SOFT_RST_SEL_MASK_SFT: u32 = (0x3 << 26);
pub const DMIC0_PHASE_SYNC_SOFT_RST_SEL_SFT: u32 = 24;
pub const DMIC0_PHASE_SYNC_SOFT_RST_SEL_MASK: u32 = 0x3;
pub const DMIC0_PHASE_SYNC_SOFT_RST_SEL_MASK_SFT: u32 = (0x3 << 24);
pub const UL3_PHASE_SYNC_SOFT_RST_SEL_SFT: u32 = 22;
pub const UL3_PHASE_SYNC_SOFT_RST_SEL_MASK: u32 = 0x3;
pub const UL3_PHASE_SYNC_SOFT_RST_SEL_MASK_SFT: u32 = (0x3 << 22);
pub const UL2_PHASE_SYNC_SOFT_RST_SEL_SFT: u32 = 20;
pub const UL2_PHASE_SYNC_SOFT_RST_SEL_MASK: u32 = 0x3;
pub const UL2_PHASE_SYNC_SOFT_RST_SEL_MASK_SFT: u32 = (0x3 << 20);
pub const UL1_PHASE_SYNC_SOFT_RST_SEL_SFT: u32 = 18;
pub const UL1_PHASE_SYNC_SOFT_RST_SEL_MASK: u32 = 0x3;
pub const UL1_PHASE_SYNC_SOFT_RST_SEL_MASK_SFT: u32 = (0x3 << 18);
pub const UL0_PHASE_SYNC_SOFT_RST_SEL_SFT: u32 = 16;
pub const UL0_PHASE_SYNC_SOFT_RST_SEL_MASK: u32 = 0x3;
pub const UL0_PHASE_SYNC_SOFT_RST_SEL_MASK_SFT: u32 = (0x3 << 16);
pub const DMIC1_PHASE_SYNC_CH1_FIFO_SEL_SFT: u32 = 13;
pub const DMIC1_PHASE_SYNC_CH1_FIFO_SEL_MASK: u32 = 0x1;
pub const DMIC1_PHASE_SYNC_CH1_FIFO_SEL_MASK_SFT: u32 = (0x1 << 13);
pub const DMIC0_PHASE_SYNC_CH1_FIFO_SEL_SFT: u32 = 12;
pub const DMIC0_PHASE_SYNC_CH1_FIFO_SEL_MASK: u32 = 0x1;
pub const DMIC0_PHASE_SYNC_CH1_FIFO_SEL_MASK_SFT: u32 = (0x1 << 12);
pub const UL3_PHASE_SYNC_CH1_FIFO_SEL_SFT: u32 = 11;
pub const UL3_PHASE_SYNC_CH1_FIFO_SEL_MASK: u32 = 0x1;
pub const UL3_PHASE_SYNC_CH1_FIFO_SEL_MASK_SFT: u32 = (0x1 << 11);
pub const UL2_PHASE_SYNC_CH1_FIFO_SEL_SFT: u32 = 10;
pub const UL2_PHASE_SYNC_CH1_FIFO_SEL_MASK: u32 = 0x1;
pub const UL2_PHASE_SYNC_CH1_FIFO_SEL_MASK_SFT: u32 = (0x1 << 10);
pub const UL1_PHASE_SYNC_CH1_FIFO_SEL_SFT: u32 = 9;
pub const UL1_PHASE_SYNC_CH1_FIFO_SEL_MASK: u32 = 0x1;
pub const UL1_PHASE_SYNC_CH1_FIFO_SEL_MASK_SFT: u32 = (0x1 << 9);
pub const UL0_PHASE_SYNC_CH1_FIFO_SEL_SFT: u32 = 8;
pub const UL0_PHASE_SYNC_CH1_FIFO_SEL_MASK: u32 = 0x1;
pub const UL0_PHASE_SYNC_CH1_FIFO_SEL_MASK_SFT: u32 = (0x1 << 8);
pub const UL_PHASE_SYNC_SOFT_RST_EN_2_ON_SFT: u32 = 5;
pub const UL_PHASE_SYNC_SOFT_RST_EN_2_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_SOFT_RST_EN_2_ON_MASK_SFT: u32 = (0x1 << 5);
pub const UL_PHASE_SYNC_SOFT_RST_EN_1_ON_SFT: u32 = 4;
pub const UL_PHASE_SYNC_SOFT_RST_EN_1_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_SOFT_RST_EN_1_ON_MASK_SFT: u32 = (0x1 << 4);
pub const UL_PHASE_SYNC_SOFT_RST_EN_0_ON_SFT: u32 = 3;
pub const UL_PHASE_SYNC_SOFT_RST_EN_0_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_SOFT_RST_EN_0_ON_MASK_SFT: u32 = (0x1 << 3);
pub const UL_PHASE_SYNC_SOFT_RST_2_ON_SFT: u32 = 2;
pub const UL_PHASE_SYNC_SOFT_RST_2_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_SOFT_RST_2_ON_MASK_SFT: u32 = (0x1 << 2);
pub const UL_PHASE_SYNC_SOFT_RST_1_ON_SFT: u32 = 1;
pub const UL_PHASE_SYNC_SOFT_RST_1_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_SOFT_RST_1_ON_MASK_SFT: u32 = (0x1 << 1);
pub const UL_PHASE_SYNC_SOFT_RST_0_ON_SFT: u32 = 0;
pub const UL_PHASE_SYNC_SOFT_RST_0_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_SOFT_RST_0_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_MTKAIF_IPM_VER_MON */
pub const RG_MTKAIF_IPM_VER_MON_SFT: u32 = 0;
pub const RG_MTKAIF_IPM_VER_MON_MASK: u32 = 0xffffffff;
pub const RG_MTKAIF_IPM_VER_MON_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_MTKAIF_MON_SEL */
pub const RG_MTKAIF_MON_SEL_SFT: u32 = 0;
pub const RG_MTKAIF_MON_SEL_MASK: u32 = 0xff;
pub const RG_MTKAIF_MON_SEL_MASK_SFT: u32 = (0xff << 0);

/* AFE_MTKAIF_MON */
pub const RG_MTKAIF_MON_SFT: u32 = 0;
pub const RG_MTKAIF_MON_MASK: u32 = 0xffffffff;
pub const RG_MTKAIF_MON_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_MTKAIF0_CFG0 */
pub const RG_MTKAIF0_RXIF_CLKINV_SFT: u32 = 31;
pub const RG_MTKAIF0_RXIF_CLKINV_MASK: u32 = 0x1;
pub const RG_MTKAIF0_RXIF_CLKINV_MASK_SFT: u32 = (0x1 << 31);
pub const RG_MTKAIF0_RXIF_BYPASS_SRC_SFT: u32 = 17;
pub const RG_MTKAIF0_RXIF_BYPASS_SRC_MASK: u32 = 0x1;
pub const RG_MTKAIF0_RXIF_BYPASS_SRC_MASK_SFT: u32 = (0x1 << 17);
pub const RG_MTKAIF0_RXIF_PROTOCOL2_SFT: u32 = 16;
pub const RG_MTKAIF0_RXIF_PROTOCOL2_MASK: u32 = 0x1;
pub const RG_MTKAIF0_RXIF_PROTOCOL2_MASK_SFT: u32 = (0x1 << 16);
pub const RG_MTKAIF0_TXIF_NLE_DEBUG_SFT: u32 = 8;
pub const RG_MTKAIF0_TXIF_NLE_DEBUG_MASK: u32 = 0x1;
pub const RG_MTKAIF0_TXIF_NLE_DEBUG_MASK_SFT: u32 = (0x1 << 8);
pub const RG_MTKAIF0_TXIF_BYPASS_SRC_SFT: u32 = 5;
pub const RG_MTKAIF0_TXIF_BYPASS_SRC_MASK: u32 = 0x1;
pub const RG_MTKAIF0_TXIF_BYPASS_SRC_MASK_SFT: u32 = (0x1 << 5);
pub const RG_MTKAIF0_TXIF_PROTOCOL2_SFT: u32 = 4;
pub const RG_MTKAIF0_TXIF_PROTOCOL2_MASK: u32 = 0x1;
pub const RG_MTKAIF0_TXIF_PROTOCOL2_MASK_SFT: u32 = (0x1 << 4);
pub const RG_MTKAIF0_TXIF_8TO5_SFT: u32 = 2;
pub const RG_MTKAIF0_TXIF_8TO5_MASK: u32 = 0x1;
pub const RG_MTKAIF0_TXIF_8TO5_MASK_SFT: u32 = (0x1 << 2);
pub const RG_MTKAIF0_RXIF_8TO5_SFT: u32 = 1;
pub const RG_MTKAIF0_RXIF_8TO5_MASK: u32 = 0x1;
pub const RG_MTKAIF0_RXIF_8TO5_MASK_SFT: u32 = (0x1 << 1);
pub const RG_MTKAIF0_TX2RX_LOOPBACK1_SFT: u32 = 0;
pub const RG_MTKAIF0_TX2RX_LOOPBACK1_MASK: u32 = 0x1;
pub const RG_MTKAIF0_TX2RX_LOOPBACK1_MASK_SFT: u32 = (0x1 << 0);

/* AFE_MTKAIF0_TX_CFG0 */
pub const RG_MTKAIF0_TXIF_NLE_FIFO_SWAP_SFT: u32 = 23;
pub const RG_MTKAIF0_TXIF_NLE_FIFO_SWAP_MASK: u32 = 0x1;
pub const RG_MTKAIF0_TXIF_NLE_FIFO_SWAP_MASK_SFT: u32 = (0x1 << 23);
pub const RG_MTKAIF0_TXIF_NLE_FIFO_RSP_SFT: u32 = 20;
pub const RG_MTKAIF0_TXIF_NLE_FIFO_RSP_MASK: u32 = 0x7;
pub const RG_MTKAIF0_TXIF_NLE_FIFO_RSP_MASK_SFT: u32 = (0x7 << 20);
pub const RG_MTKAIF0_TXIF_FIFO_SWAP_SFT: u32 = 15;
pub const RG_MTKAIF0_TXIF_FIFO_SWAP_MASK: u32 = 0x1;
pub const RG_MTKAIF0_TXIF_FIFO_SWAP_MASK_SFT: u32 = (0x1 << 15);
pub const RG_MTKAIF0_TXIF_FIFO_RSP_SFT: u32 = 12;
pub const RG_MTKAIF0_TXIF_FIFO_RSP_MASK: u32 = 0x7;
pub const RG_MTKAIF0_TXIF_FIFO_RSP_MASK_SFT: u32 = (0x7 << 12);
pub const RG_MTKAIF0_TXIF_SYNC_WORD1_SFT: u32 = 4;
pub const RG_MTKAIF0_TXIF_SYNC_WORD1_MASK: u32 = 0x7;
pub const RG_MTKAIF0_TXIF_SYNC_WORD1_MASK_SFT: u32 = (0x7 << 4);
pub const RG_MTKAIF0_TXIF_SYNC_WORD0_SFT: u32 = 0;
pub const RG_MTKAIF0_TXIF_SYNC_WORD0_MASK: u32 = 0x7;
pub const RG_MTKAIF0_TXIF_SYNC_WORD0_MASK_SFT: u32 = (0x7 << 0);

/* AFE_MTKAIF0_RX_CFG0 */
pub const RG_MTKAIF0_RXIF_VOICE_MODE_SFT: u32 = 20;
pub const RG_MTKAIF0_RXIF_VOICE_MODE_MASK: u32 = 0xf;
pub const RG_MTKAIF0_RXIF_VOICE_MODE_MASK_SFT: u32 = (0xf << 20);
pub const RG_MTKAIF0_RXIF_DETECT_ON_SFT: u32 = 16;
pub const RG_MTKAIF0_RXIF_DETECT_ON_MASK: u32 = 0x1;
pub const RG_MTKAIF0_RXIF_DETECT_ON_MASK_SFT: u32 = (0x1 << 16);
pub const RG_MTKAIF0_RXIF_DATA_BIT_SFT: u32 = 8;
pub const RG_MTKAIF0_RXIF_DATA_BIT_MASK: u32 = 0x7;
pub const RG_MTKAIF0_RXIF_DATA_BIT_MASK_SFT: u32 = (0x7 << 8);
pub const RG_MTKAIF0_RXIF_FIFO_RSP_SFT: u32 = 4;
pub const RG_MTKAIF0_RXIF_FIFO_RSP_MASK: u32 = 0x7;
pub const RG_MTKAIF0_RXIF_FIFO_RSP_MASK_SFT: u32 = (0x7 << 4);
pub const RG_MTKAIF0_RXIF_DATA_MODE_SFT: u32 = 0;
pub const RG_MTKAIF0_RXIF_DATA_MODE_MASK: u32 = 0x1;
pub const RG_MTKAIF0_RXIF_DATA_MODE_MASK_SFT: u32 = (0x1 << 0);

/* AFE_MTKAIF0_RX_CFG1 */
pub const RG_MTKAIF0_RXIF_CLEAR_SYNC_FAIL_SFT: u32 = 28;
pub const RG_MTKAIF0_RXIF_CLEAR_SYNC_FAIL_MASK: u32 = 0x1;
pub const RG_MTKAIF0_RXIF_CLEAR_SYNC_FAIL_MASK_SFT: u32 = (0x1 << 28);
pub const RG_MTKAIF0_RXIF_SYNC_CNT_TABLE_SFT: u32 = 16;
pub const RG_MTKAIF0_RXIF_SYNC_CNT_TABLE_MASK: u32 = 0xfff;
pub const RG_MTKAIF0_RXIF_SYNC_CNT_TABLE_MASK_SFT: u32 = (0xfff << 16);
pub const RG_MTKAIF0_RXIF_SYNC_SEARCH_TABLE_SFT: u32 = 12;
pub const RG_MTKAIF0_RXIF_SYNC_SEARCH_TABLE_MASK: u32 = 0xf;
pub const RG_MTKAIF0_RXIF_SYNC_SEARCH_TABLE_MASK_SFT: u32 = (0xf << 12);
pub const RG_MTKAIF0_RXIF_INVALID_SYNC_CHECK_ROUND_SFT: u32 = 8;
pub const RG_MTKAIF0_RXIF_INVALID_SYNC_CHECK_ROUND_MASK: u32 = 0xf;
pub const RG_MTKAIF0_RXIF_INVALID_SYNC_CHECK_ROUND_MASK_SFT: u32 = (0xf << 8);
pub const RG_MTKAIF0_RXIF_SYNC_CHECK_ROUND_SFT: u32 = 4;
pub const RG_MTKAIF0_RXIF_SYNC_CHECK_ROUND_MASK: u32 = 0xf;
pub const RG_MTKAIF0_RXIF_SYNC_CHECK_ROUND_MASK_SFT: u32 = (0xf << 4);

/* AFE_MTKAIF0_RX_CFG2 */
pub const RG_MTKAIF0_RXIF_SYNC_WORD1_DISABLE_SFT: u32 = 27;
pub const RG_MTKAIF0_RXIF_SYNC_WORD1_DISABLE_MASK: u32 = 0x1;
pub const RG_MTKAIF0_RXIF_SYNC_WORD1_DISABLE_MASK_SFT: u32 = (0x1 << 27);
pub const RG_MTKAIF0_RXIF_SYNC_WORD1_SFT: u32 = 24;
pub const RG_MTKAIF0_RXIF_SYNC_WORD1_MASK: u32 = 0x7;
pub const RG_MTKAIF0_RXIF_SYNC_WORD1_MASK_SFT: u32 = (0x7 << 24);
pub const RG_MTKAIF0_RXIF_SYNC_WORD0_DISABLE_SFT: u32 = 23;
pub const RG_MTKAIF0_RXIF_SYNC_WORD0_DISABLE_MASK: u32 = 0x1;
pub const RG_MTKAIF0_RXIF_SYNC_WORD0_DISABLE_MASK_SFT: u32 = (0x1 << 23);
pub const RG_MTKAIF0_RXIF_SYNC_WORD0_SFT: u32 = 20;
pub const RG_MTKAIF0_RXIF_SYNC_WORD0_MASK: u32 = 0x7;
pub const RG_MTKAIF0_RXIF_SYNC_WORD0_MASK_SFT: u32 = (0x7 << 20);
pub const RG_MTKAIF0_RXIF_DELAY_CYCLE_SFT: u32 = 12;
pub const RG_MTKAIF0_RXIF_DELAY_CYCLE_MASK: u32 = 0xf;
pub const RG_MTKAIF0_RXIF_DELAY_CYCLE_MASK_SFT: u32 = (0xf << 12);
pub const RG_MTKAIF0_RXIF_DELAY_DATA_SFT: u32 = 8;
pub const RG_MTKAIF0_RXIF_DELAY_DATA_MASK: u32 = 0x1;
pub const RG_MTKAIF0_RXIF_DELAY_DATA_MASK_SFT: u32 = (0x1 << 8);

/* AFE_MTKAIF1_CFG0 */
pub const RG_MTKAIF1_RXIF_CLKINV_ADC_SFT: u32 = 31;
pub const RG_MTKAIF1_RXIF_CLKINV_ADC_MASK: u32 = 0x1;
pub const RG_MTKAIF1_RXIF_CLKINV_ADC_MASK_SFT: u32 = (0x1 << 31);
pub const RG_MTKAIF1_RXIF_BYPASS_SRC_SFT: u32 = 17;
pub const RG_MTKAIF1_RXIF_BYPASS_SRC_MASK: u32 = 0x1;
pub const RG_MTKAIF1_RXIF_BYPASS_SRC_MASK_SFT: u32 = (0x1 << 17);
pub const RG_MTKAIF1_RXIF_PROTOCOL2_SFT: u32 = 16;
pub const RG_MTKAIF1_RXIF_PROTOCOL2_MASK: u32 = 0x1;
pub const RG_MTKAIF1_RXIF_PROTOCOL2_MASK_SFT: u32 = (0x1 << 16);
pub const RG_MTKAIF1_TXIF_NLE_DEBUG_SFT: u32 = 8;
pub const RG_MTKAIF1_TXIF_NLE_DEBUG_MASK: u32 = 0x1;
pub const RG_MTKAIF1_TXIF_NLE_DEBUG_MASK_SFT: u32 = (0x1 << 8);
pub const RG_MTKAIF1_TXIF_BYPASS_SRC_SFT: u32 = 5;
pub const RG_MTKAIF1_TXIF_BYPASS_SRC_MASK: u32 = 0x1;
pub const RG_MTKAIF1_TXIF_BYPASS_SRC_MASK_SFT: u32 = (0x1 << 5);
pub const RG_MTKAIF1_TXIF_PROTOCOL2_SFT: u32 = 4;
pub const RG_MTKAIF1_TXIF_PROTOCOL2_MASK: u32 = 0x1;
pub const RG_MTKAIF1_TXIF_PROTOCOL2_MASK_SFT: u32 = (0x1 << 4);
pub const RG_MTKAIF1_TXIF_8TO5_SFT: u32 = 2;
pub const RG_MTKAIF1_TXIF_8TO5_MASK: u32 = 0x1;
pub const RG_MTKAIF1_TXIF_8TO5_MASK_SFT: u32 = (0x1 << 2);
pub const RG_MTKAIF1_RXIF_8TO5_SFT: u32 = 1;
pub const RG_MTKAIF1_RXIF_8TO5_MASK: u32 = 0x1;
pub const RG_MTKAIF1_RXIF_8TO5_MASK_SFT: u32 = (0x1 << 1);
pub const RG_MTKAIF1_IF_LOOPBACK1_SFT: u32 = 0;
pub const RG_MTKAIF1_IF_LOOPBACK1_MASK: u32 = 0x1;
pub const RG_MTKAIF1_IF_LOOPBACK1_MASK_SFT: u32 = (0x1 << 0);

/* AFE_MTKAIF1_TX_CFG0 */
pub const RG_MTKAIF1_TXIF_NLE_FIFO_SWAP_SFT: u32 = 23;
pub const RG_MTKAIF1_TXIF_NLE_FIFO_SWAP_MASK: u32 = 0x1;
pub const RG_MTKAIF1_TXIF_NLE_FIFO_SWAP_MASK_SFT: u32 = (0x1 << 23);
pub const RG_MTKAIF1_TXIF_NLE_FIFO_RSP_SFT: u32 = 20;
pub const RG_MTKAIF1_TXIF_NLE_FIFO_RSP_MASK: u32 = 0x7;
pub const RG_MTKAIF1_TXIF_NLE_FIFO_RSP_MASK_SFT: u32 = (0x7 << 20);
pub const RG_MTKAIF1_TXIF_FIFO_SWAP_SFT: u32 = 15;
pub const RG_MTKAIF1_TXIF_FIFO_SWAP_MASK: u32 = 0x1;
pub const RG_MTKAIF1_TXIF_FIFO_SWAP_MASK_SFT: u32 = (0x1 << 15);
pub const RG_MTKAIF1_TXIF_FIFO_RSP_SFT: u32 = 12;
pub const RG_MTKAIF1_TXIF_FIFO_RSP_MASK: u32 = 0x7;
pub const RG_MTKAIF1_TXIF_FIFO_RSP_MASK_SFT: u32 = (0x7 << 12);
pub const RG_MTKAIF1_TXIF_SYNC_WORD1_SFT: u32 = 4;
pub const RG_MTKAIF1_TXIF_SYNC_WORD1_MASK: u32 = 0x7;
pub const RG_MTKAIF1_TXIF_SYNC_WORD1_MASK_SFT: u32 = (0x7 << 4);
pub const RG_MTKAIF1_TXIF_SYNC_WORD0_SFT: u32 = 0;
pub const RG_MTKAIF1_TXIF_SYNC_WORD0_MASK: u32 = 0x7;
pub const RG_MTKAIF1_TXIF_SYNC_WORD0_MASK_SFT: u32 = (0x7 << 0);

/* AFE_MTKAIF1_RX_CFG0 */
pub const RG_MTKAIF1_RXIF_VOICE_MODE_SFT: u32 = 20;
pub const RG_MTKAIF1_RXIF_VOICE_MODE_MASK: u32 = 0xf;
pub const RG_MTKAIF1_RXIF_VOICE_MODE_MASK_SFT: u32 = (0xf << 20);
pub const RG_MTKAIF1_RXIF_DETECT_ON_SFT: u32 = 16;
pub const RG_MTKAIF1_RXIF_DETECT_ON_MASK: u32 = 0x1;
pub const RG_MTKAIF1_RXIF_DETECT_ON_MASK_SFT: u32 = (0x1 << 16);
pub const RG_MTKAIF1_RXIF_DATA_BIT_SFT: u32 = 8;
pub const RG_MTKAIF1_RXIF_DATA_BIT_MASK: u32 = 0x7;
pub const RG_MTKAIF1_RXIF_DATA_BIT_MASK_SFT: u32 = (0x7 << 8);
pub const RG_MTKAIF1_RXIF_FIFO_RSP_SFT: u32 = 4;
pub const RG_MTKAIF1_RXIF_FIFO_RSP_MASK: u32 = 0x7;
pub const RG_MTKAIF1_RXIF_FIFO_RSP_MASK_SFT: u32 = (0x7 << 4);
pub const RG_MTKAIF1_RXIF_DATA_MODE_SFT: u32 = 0;
pub const RG_MTKAIF1_RXIF_DATA_MODE_MASK: u32 = 0x1;
pub const RG_MTKAIF1_RXIF_DATA_MODE_MASK_SFT: u32 = (0x1 << 0);

/* AFE_MTKAIF1_RX_CFG1 */
pub const RG_MTKAIF1_RXIF_CLEAR_SYNC_FAIL_SFT: u32 = 28;
pub const RG_MTKAIF1_RXIF_CLEAR_SYNC_FAIL_MASK: u32 = 0x1;
pub const RG_MTKAIF1_RXIF_CLEAR_SYNC_FAIL_MASK_SFT: u32 = (0x1 << 28);
pub const RG_MTKAIF1_RXIF_SYNC_CNT_TABLE_SFT: u32 = 16;
pub const RG_MTKAIF1_RXIF_SYNC_CNT_TABLE_MASK: u32 = 0xfff;
pub const RG_MTKAIF1_RXIF_SYNC_CNT_TABLE_MASK_SFT: u32 = (0xfff << 16);
pub const RG_MTKAIF1_RXIF_SYNC_SEARCH_TABLE_SFT: u32 = 12;
pub const RG_MTKAIF1_RXIF_SYNC_SEARCH_TABLE_MASK: u32 = 0xf;
pub const RG_MTKAIF1_RXIF_SYNC_SEARCH_TABLE_MASK_SFT: u32 = (0xf << 12);
pub const RG_MTKAIF1_RXIF_INVALID_SYNC_CHECK_ROUND_SFT: u32 = 8;
pub const RG_MTKAIF1_RXIF_INVALID_SYNC_CHECK_ROUND_MASK: u32 = 0xf;
pub const RG_MTKAIF1_RXIF_INVALID_SYNC_CHECK_ROUND_MASK_SFT: u32 = (0xf << 8);
pub const RG_MTKAIF1_RXIF_SYNC_CHECK_ROUND_SFT: u32 = 4;
pub const RG_MTKAIF1_RXIF_SYNC_CHECK_ROUND_MASK: u32 = 0xf;
pub const RG_MTKAIF1_RXIF_SYNC_CHECK_ROUND_MASK_SFT: u32 = (0xf << 4);

/* AFE_MTKAIF1_RX_CFG2 */
pub const RG_MTKAIF1_RXIF_SYNC_WORD1_DISABLE_SFT: u32 = 27;
pub const RG_MTKAIF1_RXIF_SYNC_WORD1_DISABLE_MASK: u32 = 0x1;
pub const RG_MTKAIF1_RXIF_SYNC_WORD1_DISABLE_MASK_SFT: u32 = (0x1 << 27);
pub const RG_MTKAIF1_RXIF_SYNC_WORD1_SFT: u32 = 24;
pub const RG_MTKAIF1_RXIF_SYNC_WORD1_MASK: u32 = 0x7;
pub const RG_MTKAIF1_RXIF_SYNC_WORD1_MASK_SFT: u32 = (0x7 << 24);
pub const RG_MTKAIF1_RXIF_SYNC_WORD0_DISABLE_SFT: u32 = 23;
pub const RG_MTKAIF1_RXIF_SYNC_WORD0_DISABLE_MASK: u32 = 0x1;
pub const RG_MTKAIF1_RXIF_SYNC_WORD0_DISABLE_MASK_SFT: u32 = (0x1 << 23);
pub const RG_MTKAIF1_RXIF_SYNC_WORD0_SFT: u32 = 20;
pub const RG_MTKAIF1_RXIF_SYNC_WORD0_MASK: u32 = 0x7;
pub const RG_MTKAIF1_RXIF_SYNC_WORD0_MASK_SFT: u32 = (0x7 << 20);
pub const RG_MTKAIF1_RXIF_DELAY_CYCLE_SFT: u32 = 12;
pub const RG_MTKAIF1_RXIF_DELAY_CYCLE_MASK: u32 = 0xf;
pub const RG_MTKAIF1_RXIF_DELAY_CYCLE_MASK_SFT: u32 = (0xf << 12);
pub const RG_MTKAIF1_RXIF_DELAY_DATA_SFT: u32 = 8;
pub const RG_MTKAIF1_RXIF_DELAY_DATA_MASK: u32 = 0x1;
pub const RG_MTKAIF1_RXIF_DELAY_DATA_MASK_SFT: u32 = (0x1 << 8);

/* AFE_AUD_PAD_TOP_CFG0 */
pub const AUD_PAD_TOP_FIFO_RSP_SFT: u32 = 4;
pub const AUD_PAD_TOP_FIFO_RSP_MASK: u32 = 0xf;
pub const AUD_PAD_TOP_FIFO_RSP_MASK_SFT: u32 = (0xf << 4);
pub const RG_RX_PROTOCOL2_SFT: u32 = 3;
pub const RG_RX_PROTOCOL2_MASK: u32 = 0x1;
pub const RG_RX_PROTOCOL2_MASK_SFT: u32 = (0x1 << 3);
pub const RG_RX_FIFO_ON_SFT: u32 = 0;
pub const RG_RX_FIFO_ON_MASK: u32 = 0x1;
pub const RG_RX_FIFO_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_AUD_PAD_TOP_MON */
pub const AUD_PAD_TOP_MON_SFT: u32 = 0;
pub const AUD_PAD_TOP_MON_MASK: u32 = 0xffff;
pub const AUD_PAD_TOP_MON_MASK_SFT: u32 = (0xffff << 0);

/* AFE_ADDA_MTKAIFV4_TX_CFG0 */
pub const MTKAIFV4_TXIF_EN_SEL_SFT: u32 = 12;
pub const MTKAIFV4_TXIF_EN_SEL_MASK: u32 = 0x1;
pub const MTKAIFV4_TXIF_EN_SEL_MASK_SFT: u32 = (0x1 << 12);
pub const MTKAIFV4_TXIF_V4_SFT: u32 = 11;
pub const MTKAIFV4_TXIF_V4_MASK: u32 = 0x1;
pub const MTKAIFV4_TXIF_V4_MASK_SFT: u32 = (0x1 << 11);
pub const MTKAIFV4_ADDA6_OUT_EN_SEL_SFT: u32 = 10;
pub const MTKAIFV4_ADDA6_OUT_EN_SEL_MASK: u32 = 0x1;
pub const MTKAIFV4_ADDA6_OUT_EN_SEL_MASK_SFT: u32 = (0x1 << 10);
pub const MTKAIFV4_ADDA_OUT_EN_SEL_SFT: u32 = 9;
pub const MTKAIFV4_ADDA_OUT_EN_SEL_MASK: u32 = 0x1;
pub const MTKAIFV4_ADDA_OUT_EN_SEL_MASK_SFT: u32 = (0x1 << 9);
pub const MTKAIFV4_TXIF_INPUT_MODE_SFT: u32 = 4;
pub const MTKAIFV4_TXIF_INPUT_MODE_MASK: u32 = 0x1f;
pub const MTKAIFV4_TXIF_INPUT_MODE_MASK_SFT: u32 = (0x1f << 4);
pub const MTKAIFV4_TXIF_FOUR_CHANNEL_SFT: u32 = 1;
pub const MTKAIFV4_TXIF_FOUR_CHANNEL_MASK: u32 = 0x1;
pub const MTKAIFV4_TXIF_FOUR_CHANNEL_MASK_SFT: u32 = (0x1 << 1);
pub const MTKAIFV4_TXIF_AFE_ON_SFT: u32 = 0;
pub const MTKAIFV4_TXIF_AFE_ON_MASK: u32 = 0x1;
pub const MTKAIFV4_TXIF_AFE_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_ADDA6_MTKAIFV4_TX_CFG0 */
pub const ADDA6_MTKAIFV4_TXIF_EN_SEL_SFT: u32 = 12;
pub const ADDA6_MTKAIFV4_TXIF_EN_SEL_MASK: u32 = 0x1;
pub const ADDA6_MTKAIFV4_TXIF_EN_SEL_MASK_SFT: u32 = (0x1 << 12);
pub const ADDA6_MTKAIFV4_TXIF_INPUT_MODE_SFT: u32 = 4;
pub const ADDA6_MTKAIFV4_TXIF_INPUT_MODE_MASK: u32 = 0x1f;
pub const ADDA6_MTKAIFV4_TXIF_INPUT_MODE_MASK_SFT: u32 = (0x1f << 4);
pub const ADDA6_MTKAIFV4_TXIF_FOUR_CHANNEL_SFT: u32 = 1;
pub const ADDA6_MTKAIFV4_TXIF_FOUR_CHANNEL_MASK: u32 = 0x1;
pub const ADDA6_MTKAIFV4_TXIF_FOUR_CHANNEL_MASK_SFT: u32 = (0x1 << 1);
pub const ADDA6_MTKAIFV4_TXIF_AFE_ON_SFT: u32 = 0;
pub const ADDA6_MTKAIFV4_TXIF_AFE_ON_MASK: u32 = 0x1;
pub const ADDA6_MTKAIFV4_TXIF_AFE_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_ADDA_MTKAIFV4_RX_CFG0 */
pub const MTKAIFV4_RXIF_CLKINV_SFT: u32 = 31;
pub const MTKAIFV4_RXIF_CLKINV_MASK: u32 = 0x1;
pub const MTKAIFV4_RXIF_CLKINV_MASK_SFT: u32 = (0x1 << 31);
pub const MTKAIFV4_RXIF_LOOPBACK_MODE_SFT: u32 = 28;
pub const MTKAIFV4_RXIF_LOOPBACK_MODE_MASK: u32 = 0x1;
pub const MTKAIFV4_RXIF_LOOPBACK_MODE_MASK_SFT: u32 = (0x1 << 28);
pub const MTKAIFV4_UL_CH7CH8_IN_EN_SEL_SFT: u32 = 19;
pub const MTKAIFV4_UL_CH7CH8_IN_EN_SEL_MASK: u32 = 0x1;
pub const MTKAIFV4_UL_CH7CH8_IN_EN_SEL_MASK_SFT: u32 = (0x1 << 19);
pub const MTKAIFV4_UL_CH5CH6_IN_EN_SEL_SFT: u32 = 18;
pub const MTKAIFV4_UL_CH5CH6_IN_EN_SEL_MASK: u32 = 0x1;
pub const MTKAIFV4_UL_CH5CH6_IN_EN_SEL_MASK_SFT: u32 = (0x1 << 18);
pub const MTKAIFV4_UL_CH3CH4_IN_EN_SEL_SFT: u32 = 17;
pub const MTKAIFV4_UL_CH3CH4_IN_EN_SEL_MASK: u32 = 0x1;
pub const MTKAIFV4_UL_CH3CH4_IN_EN_SEL_MASK_SFT: u32 = (0x1 << 17);
pub const MTKAIFV4_UL_CH1CH2_IN_EN_SEL_SFT: u32 = 16;
pub const MTKAIFV4_UL_CH1CH2_IN_EN_SEL_MASK: u32 = 0x1;
pub const MTKAIFV4_UL_CH1CH2_IN_EN_SEL_MASK_SFT: u32 = (0x1 << 16);
pub const MTKAIFV4_RXIF_EN_SEL_SFT: u32 = 12;
pub const MTKAIFV4_RXIF_EN_SEL_MASK: u32 = 0x1;
pub const MTKAIFV4_RXIF_EN_SEL_MASK_SFT: u32 = (0x1 << 12);
pub const MTKAIFV4_RXIF_INPUT_MODE_SFT: u32 = 4;
pub const MTKAIFV4_RXIF_INPUT_MODE_MASK: u32 = 0x1f;
pub const MTKAIFV4_RXIF_INPUT_MODE_MASK_SFT: u32 = (0x1f << 4);
pub const MTKAIFV4_RXIF_FOUR_CHANNEL_SFT: u32 = 1;
pub const MTKAIFV4_RXIF_FOUR_CHANNEL_MASK: u32 = 0x1;
pub const MTKAIFV4_RXIF_FOUR_CHANNEL_MASK_SFT: u32 = (0x1 << 1);
pub const MTKAIFV4_RXIF_AFE_ON_SFT: u32 = 0;
pub const MTKAIFV4_RXIF_AFE_ON_MASK: u32 = 0x1;
pub const MTKAIFV4_RXIF_AFE_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_ADDA_MTKAIFV4_RX_CFG1 */
pub const MTKAIFV4_RXIF_SYNC_CNT_TABLE_SFT: u32 = 17;
pub const MTKAIFV4_RXIF_SYNC_CNT_TABLE_MASK: u32 = 0xfff;
pub const MTKAIFV4_RXIF_SYNC_CNT_TABLE_MASK_SFT: u32 = (0xfff << 17);
pub const MTKAIFV4_RXIF_SYNC_SEARCH_TABLE_SFT: u32 = 12;
pub const MTKAIFV4_RXIF_SYNC_SEARCH_TABLE_MASK: u32 = 0x1f;
pub const MTKAIFV4_RXIF_SYNC_SEARCH_TABLE_MASK_SFT: u32 = (0x1f << 12);
pub const MTKAIFV4_RXIF_INVAILD_SYNC_CHECK_ROUND_SFT: u32 = 8;
pub const MTKAIFV4_RXIF_INVAILD_SYNC_CHECK_ROUND_MASK: u32 = 0xf;
pub const MTKAIFV4_RXIF_INVAILD_SYNC_CHECK_ROUND_MASK_SFT: u32 = (0xf << 8);
pub const MTKAIFV4_RXIF_SYNC_CHECK_ROUND_SFT: u32 = 4;
pub const MTKAIFV4_RXIF_SYNC_CHECK_ROUND_MASK: u32 = 0xf;
pub const MTKAIFV4_RXIF_SYNC_CHECK_ROUND_MASK_SFT: u32 = (0xf << 4);
pub const MTKAIFV4_RXIF_FIFO_RSP_SFT: u32 = 1;
pub const MTKAIFV4_RXIF_FIFO_RSP_MASK: u32 = 0x7;
pub const MTKAIFV4_RXIF_FIFO_RSP_MASK_SFT: u32 = (0x7 << 1);
pub const MTKAIFV4_RXIF_SELF_DEFINE_TABLE_SFT: u32 = 0;
pub const MTKAIFV4_RXIF_SELF_DEFINE_TABLE_MASK: u32 = 0x1;
pub const MTKAIFV4_RXIF_SELF_DEFINE_TABLE_MASK_SFT: u32 = (0x1 << 0);

/* AFE_ADDA6_MTKAIFV4_RX_CFG0 */
pub const ADDA6_MTKAIFV4_RXIF_CLKINV_SFT: u32 = 31;
pub const ADDA6_MTKAIFV4_RXIF_CLKINV_MASK: u32 = 0x1;
pub const ADDA6_MTKAIFV4_RXIF_CLKINV_MASK_SFT: u32 = (0x1 << 31);
pub const ADDA6_MTKAIFV4_RXIF_LOOPBACK_MODE_SFT: u32 = 28;
pub const ADDA6_MTKAIFV4_RXIF_LOOPBACK_MODE_MASK: u32 = 0x1;
pub const ADDA6_MTKAIFV4_RXIF_LOOPBACK_MODE_MASK_SFT: u32 = (0x1 << 28);
pub const ADDA6_MTKAIFV4_RXIF_EN_SEL_SFT: u32 = 12;
pub const ADDA6_MTKAIFV4_RXIF_EN_SEL_MASK: u32 = 0x1;
pub const ADDA6_MTKAIFV4_RXIF_EN_SEL_MASK_SFT: u32 = (0x1 << 12);
pub const ADDA6_MTKAIFV4_RXIF_INPUT_MODE_SFT: u32 = 4;
pub const ADDA6_MTKAIFV4_RXIF_INPUT_MODE_MASK: u32 = 0x1f;
pub const ADDA6_MTKAIFV4_RXIF_INPUT_MODE_MASK_SFT: u32 = (0x1f << 4);
pub const ADDA6_MTKAIFV4_RXIF_FOUR_CHANNEL_SFT: u32 = 1;
pub const ADDA6_MTKAIFV4_RXIF_FOUR_CHANNEL_MASK: u32 = 0x1;
pub const ADDA6_MTKAIFV4_RXIF_FOUR_CHANNEL_MASK_SFT: u32 = (0x1 << 1);
pub const ADDA6_MTKAIFV4_RXIF_AFE_ON_SFT: u32 = 0;
pub const ADDA6_MTKAIFV4_RXIF_AFE_ON_MASK: u32 = 0x1;
pub const ADDA6_MTKAIFV4_RXIF_AFE_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_ADDA6_MTKAIFV4_RX_CFG1 */
pub const ADDA6_MTKAIFV4_RXIF_SYNC_CNT_TABLE_SFT: u32 = 17;
pub const ADDA6_MTKAIFV4_RXIF_SYNC_CNT_TABLE_MASK: u32 = 0xfff;
pub const ADDA6_MTKAIFV4_RXIF_SYNC_CNT_TABLE_MASK_SFT: u32 = (0xfff << 17);
pub const ADDA6_MTKAIFV4_RXIF_SYNC_SEARCH_TABLE_SFT: u32 = 12;
pub const ADDA6_MTKAIFV4_RXIF_SYNC_SEARCH_TABLE_MASK: u32 = 0x1f;
pub const ADDA6_MTKAIFV4_RXIF_SYNC_SEARCH_TABLE_MASK_SFT: u32 = (0x1f << 12);
pub const ADDA6_MTKAIFV4_RXIF_INVAILD_SYNC_CHECK_ROUND_SFT: u32 = 8;
pub const ADDA6_MTKAIFV4_RXIF_INVAILD_SYNC_CHECK_ROUND_MASK: u32 = 0xf;
pub const ADDA6_MTKAIFV4_RXIF_INVAILD_SYNC_CHECK_ROUND_MASK_SFT: u32 = (0xf << 8);
pub const ADDA6_MTKAIFV4_RXIF_SYNC_CHECK_ROUND_SFT: u32 = 4;
pub const ADDA6_MTKAIFV4_RXIF_SYNC_CHECK_ROUND_MASK: u32 = 0xf;
pub const ADDA6_MTKAIFV4_RXIF_SYNC_CHECK_ROUND_MASK_SFT: u32 = (0xf << 4);
pub const ADDA6_MTKAIFV4_RXIF_FIFO_RSP_SFT: u32 = 1;
pub const ADDA6_MTKAIFV4_RXIF_FIFO_RSP_MASK: u32 = 0x7;
pub const ADDA6_MTKAIFV4_RXIF_FIFO_RSP_MASK_SFT: u32 = (0x7 << 1);
pub const ADDA6_MTKAIFV4_RXIF_SELF_DEFINE_TABLE_SFT: u32 = 0;
pub const ADDA6_MTKAIFV4_RXIF_SELF_DEFINE_TABLE_MASK: u32 = 0x1;
pub const ADDA6_MTKAIFV4_RXIF_SELF_DEFINE_TABLE_MASK_SFT: u32 = (0x1 << 0);

/* AFE_ADDA_MTKAIFV4_TX_SYNCWORD_CFG */
pub const ADDA6_MTKAIFV4_TXIF_SYNCWORD_SFT: u32 = 16;
pub const ADDA6_MTKAIFV4_TXIF_SYNCWORD_MASK: u32 = 0xffff;
pub const ADDA6_MTKAIFV4_TXIF_SYNCWORD_MASK_SFT: u32 = (0xffff << 16);
pub const ADDA_MTKAIFV4_TXIF_SYNCWORD_SFT: u32 = 0;
pub const ADDA_MTKAIFV4_TXIF_SYNCWORD_MASK: u32 = 0xffff;
pub const ADDA_MTKAIFV4_TXIF_SYNCWORD_MASK_SFT: u32 = (0xffff << 0);

/* AFE_ADDA_MTKAIFV4_RX_SYNCWORD_CFG */
pub const ADDA6_MTKAIFV4_RXIF_SYNCWORD_SFT: u32 = 16;
pub const ADDA6_MTKAIFV4_RXIF_SYNCWORD_MASK: u32 = 0xffff;
pub const ADDA6_MTKAIFV4_RXIF_SYNCWORD_MASK_SFT: u32 = (0xffff << 16);
pub const ADDA_MTKAIFV4_RXIF_SYNCWORD_SFT: u32 = 0;
pub const ADDA_MTKAIFV4_RXIF_SYNCWORD_MASK: u32 = 0xffff;
pub const ADDA_MTKAIFV4_RXIF_SYNCWORD_MASK_SFT: u32 = (0xffff << 0);

/* AFE_ADDA_MTKAIFV4_MON0 */
pub const MTKAIFV4_TXIF_SDATA_OUT_SFT: u32 = 23;
pub const MTKAIFV4_TXIF_SDATA_OUT_MASK: u32 = 0x1;
pub const MTKAIFV4_TXIF_SDATA_OUT_MASK_SFT: u32 = (0x1 << 23);
pub const MTKAIFV4_RXIF_SDATA_IN_SFT: u32 = 22;
pub const MTKAIFV4_RXIF_SDATA_IN_MASK: u32 = 0x1;
pub const MTKAIFV4_RXIF_SDATA_IN_MASK_SFT: u32 = (0x1 << 22);
pub const MTKAIFV4_RXIF_SEARCH_FAIL_FLAG_SFT: u32 = 21;
pub const MTKAIFV4_RXIF_SEARCH_FAIL_FLAG_MASK: u32 = 0x1;
pub const MTKAIFV4_RXIF_SEARCH_FAIL_FLAG_MASK_SFT: u32 = (0x1 << 21);
pub const MTKAIFV4_RXIF_ADC_FIFO_STATUS_SFT: u32 = 0;
pub const MTKAIFV4_RXIF_ADC_FIFO_STATUS_MASK: u32 = 0xfff;
pub const MTKAIFV4_RXIF_ADC_FIFO_STATUS_MASK_SFT: u32 = (0xfff << 0);

/* AFE_ADDA_MTKAIFV4_MON1 */
pub const MTKAIFV4_RXIF_OUT_CH4_SFT: u32 = 24;
pub const MTKAIFV4_RXIF_OUT_CH4_MASK: u32 = 0xff;
pub const MTKAIFV4_RXIF_OUT_CH4_MASK_SFT: u32 = (0xff << 24);
pub const MTKAIFV4_RXIF_OUT_CH3_SFT: u32 = 16;
pub const MTKAIFV4_RXIF_OUT_CH3_MASK: u32 = 0xff;
pub const MTKAIFV4_RXIF_OUT_CH3_MASK_SFT: u32 = (0xff << 16);
pub const MTKAIFV4_RXIF_OUT_CH2_SFT: u32 = 8;
pub const MTKAIFV4_RXIF_OUT_CH2_MASK: u32 = 0xff;
pub const MTKAIFV4_RXIF_OUT_CH2_MASK_SFT: u32 = (0xff << 8);
pub const MTKAIFV4_RXIF_OUT_CH1_SFT: u32 = 0;
pub const MTKAIFV4_RXIF_OUT_CH1_MASK: u32 = 0xff;
pub const MTKAIFV4_RXIF_OUT_CH1_MASK_SFT: u32 = (0xff << 0);

/* AFE_ADDA6_MTKAIFV4_MON0 */
pub const ADDA6_MTKAIFV4_TXIF_SDATA_OUT_SFT: u32 = 23;
pub const ADDA6_MTKAIFV4_TXIF_SDATA_OUT_MASK: u32 = 0x1;
pub const ADDA6_MTKAIFV4_TXIF_SDATA_OUT_MASK_SFT: u32 = (0x1 << 23);
pub const ADDA6_MTKAIFV4_RXIF_SDATA_IN_SFT: u32 = 22;
pub const ADDA6_MTKAIFV4_RXIF_SDATA_IN_MASK: u32 = 0x1;
pub const ADDA6_MTKAIFV4_RXIF_SDATA_IN_MASK_SFT: u32 = (0x1 << 22);
pub const ADDA6_MTKAIFV4_RXIF_SEARCH_FAIL_FLAG_SFT: u32 = 21;
pub const ADDA6_MTKAIFV4_RXIF_SEARCH_FAIL_FLAG_MASK: u32 = 0x1;
pub const ADDA6_MTKAIFV4_RXIF_SEARCH_FAIL_FLAG_MASK_SFT: u32 = (0x1 << 21);
pub const ADDA6_MTKAIFV3P3_RXIF_ADC_FIFO_STATUS_SFT: u32 = 0;
pub const ADDA6_MTKAIFV3P3_RXIF_ADC_FIFO_STATUS_MASK: u32 = 0xfff;
pub const ADDA6_MTKAIFV3P3_RXIF_ADC_FIFO_STATUS_MASK_SFT: u32 = (0xfff << 0);

/* ETDM_IN0_CON0 */
pub const REG_ETDM_IN_EN_SFT: u32 = 0;
pub const REG_ETDM_IN_EN_MASK: u32 = 0x1;
pub const REG_ETDM_IN_EN_MASK_SFT: u32 = (0x1 << 0);
pub const REG_SYNC_MODE_SFT: u32 = 1;
pub const REG_SYNC_MODE_MASK: u32 = 0x1;
pub const REG_SYNC_MODE_MASK_SFT: u32 = (0x1 << 1);
pub const REG_LSB_FIRST_SFT: u32 = 3;
pub const REG_LSB_FIRST_MASK: u32 = 0x1;
pub const REG_LSB_FIRST_MASK_SFT: u32 = (0x1 << 3);
pub const REG_SOFT_RST_SFT: u32 = 4;
pub const REG_SOFT_RST_MASK: u32 = 0x1;
pub const REG_SOFT_RST_MASK_SFT: u32 = (0x1 << 4);
pub const REG_SLAVE_MODE_SFT: u32 = 5;
pub const REG_SLAVE_MODE_MASK: u32 = 0x1;
pub const REG_SLAVE_MODE_MASK_SFT: u32 = (0x1 << 5);
pub const REG_FMT_SFT: u32 = 6;
pub const REG_FMT_MASK: u32 = 0x7;
pub const REG_FMT_MASK_SFT: u32 = (0x7 << 6);
pub const REG_LRCK_EDGE_SEL_SFT: u32 = 10;
pub const REG_LRCK_EDGE_SEL_MASK: u32 = 0x1;
pub const REG_LRCK_EDGE_SEL_MASK_SFT: u32 = (0x1 << 10);
pub const REG_BIT_LENGTH_SFT: u32 = 11;
pub const REG_BIT_LENGTH_MASK: u32 = 0x1f;
pub const REG_BIT_LENGTH_MASK_SFT: u32 = (0x1f << 11);
pub const REG_WORD_LENGTH_SFT: u32 = 16;
pub const REG_WORD_LENGTH_MASK: u32 = 0x1f;
pub const REG_WORD_LENGTH_MASK_SFT: u32 = (0x1f << 16);
pub const REG_CH_NUM_SFT: u32 = 23;
pub const REG_CH_NUM_MASK: u32 = 0x1f;
pub const REG_CH_NUM_MASK_SFT: u32 = (0x1f << 23);
pub const REG_RELATCH_1X_EN_DOMAIN_SEL_SFT: u32 = 28;
pub const REG_RELATCH_1X_EN_DOMAIN_SEL_MASK: u32 = 0x7;
pub const REG_RELATCH_1X_EN_DOMAIN_SEL_MASK_SFT: u32 = (0x7 << 28);
pub const REG_VALID_TOGETHER_SFT: u32 = 31;
pub const REG_VALID_TOGETHER_MASK: u32 = 0x1;
pub const REG_VALID_TOGETHER_MASK_SFT: u32 = (0x1 << 31);

/* ETDM_IN0_CON1 */
/* ETDM_IN1_CON1 */
/* ETDM_IN2_CON1 */
/* ETDM_IN3_CON1 */
/* ETDM_IN4_CON1 */
/* ETDM_IN5_CON1 */
/* ETDM_IN6_CON1 */
pub const REG_INITIAL_COUNT_SFT: u32 = 0;
pub const REG_INITIAL_COUNT_MASK: u32 = 0x1f;
pub const REG_INITIAL_COUNT_MASK_SFT: u32 = (0x1f << 0);
pub const REG_INITIAL_POINT_SFT: u32 = 5;
pub const REG_INITIAL_POINT_MASK: u32 = 0x1f;
pub const REG_INITIAL_POINT_MASK_SFT: u32 = (0x1f << 5);
pub const REG_LRCK_AUTO_OFF_SFT: u32 = 10;
pub const REG_LRCK_AUTO_OFF_MASK: u32 = 0x1;
pub const REG_LRCK_AUTO_OFF_MASK_SFT: u32 = (0x1 << 10);
pub const REG_BCK_AUTO_OFF_SFT: u32 = 11;
pub const REG_BCK_AUTO_OFF_MASK: u32 = 0x1;
pub const REG_BCK_AUTO_OFF_MASK_SFT: u32 = (0x1 << 11);
pub const REG_INITIAL_LRCK_SFT: u32 = 13;
pub const REG_INITIAL_LRCK_MASK: u32 = 0x1;
pub const REG_INITIAL_LRCK_MASK_SFT: u32 = (0x1 << 13);
pub const REG_NO_ALIGN_1X_EN_SFT: u32 = 14;
pub const REG_NO_ALIGN_1X_EN_MASK: u32 = 0x1;
pub const REG_NO_ALIGN_1X_EN_MASK_SFT: u32 = (0x1 << 14);
pub const REG_LRCK_RESET_SFT: u32 = 15;
pub const REG_LRCK_RESET_MASK: u32 = 0x1;
pub const REG_LRCK_RESET_MASK_SFT: u32 = (0x1 << 15);
pub const PINMUX_MCLK_CTRL_OE_SFT: u32 = 16;
pub const PINMUX_MCLK_CTRL_OE_MASK: u32 = 0x1;
pub const PINMUX_MCLK_CTRL_OE_MASK_SFT: u32 = (0x1 << 16);
pub const REG_OUTPUT_CR_EN_SFT: u32 = 18;
pub const REG_OUTPUT_CR_EN_MASK: u32 = 0x1;
pub const REG_OUTPUT_CR_EN_MASK_SFT: u32 = (0x1 << 18);
pub const REG_LR_ALIGN_SFT: u32 = 19;
pub const REG_LR_ALIGN_MASK: u32 = 0x1;
pub const REG_LR_ALIGN_MASK_SFT: u32 = (0x1 << 19);
pub const REG_LRCK_WIDTH_SFT: u32 = 20;
pub const REG_LRCK_WIDTH_MASK: u32 = 0x3ff;
pub const REG_LRCK_WIDTH_MASK_SFT: u32 = (0x3ff << 20);
pub const REG_DIRECT_INPUT_MASTER_BCK_SFT: u32 = 30;
pub const REG_DIRECT_INPUT_MASTER_BCK_MASK: u32 = 0x1;
pub const REG_DIRECT_INPUT_MASTER_BCK_MASK_SFT: u32 = (0x1 << 30);
pub const REG_LRCK_AUTO_MODE_SFT: u32 = 31;
pub const REG_LRCK_AUTO_MODE_MASK: u32 = 0x1;
pub const REG_LRCK_AUTO_MODE_MASK_SFT: u32 = (0x1 << 31);

/* ETDM_IN0_CON2 */
/* ETDM_IN1_CON2 */
/* ETDM_IN2_CON2 */
/* ETDM_IN3_CON2 */
/* ETDM_IN4_CON2 */
/* ETDM_IN5_CON2 */
/* ETDM_IN6_CON2 */
pub const REG_UPDATE_POINT_SFT: u32 = 0;
pub const REG_UPDATE_POINT_MASK: u32 = 0x1f;
pub const REG_UPDATE_POINT_MASK_SFT: u32 = (0x1f << 0);
pub const REG_UPDATE_GAP_SFT: u32 = 5;
pub const REG_UPDATE_GAP_MASK: u32 = 0x1f;
pub const REG_UPDATE_GAP_MASK_SFT: u32 = (0x1f << 5);
pub const REG_CLOCK_SOURCE_SEL_SFT: u32 = 10;
pub const REG_CLOCK_SOURCE_SEL_MASK: u32 = 0x7;
pub const REG_CLOCK_SOURCE_SEL_MASK_SFT: u32 = (0x7 << 10);
pub const REG_CK_EN_SEL_AUTO_SFT: u32 = 14;
pub const REG_CK_EN_SEL_AUTO_MASK: u32 = 0x1;
pub const REG_CK_EN_SEL_AUTO_MASK_SFT: u32 = (0x1 << 14);
pub const REG_MULTI_IP_TOTAL_CHNUM_SFT: u32 = 15;
pub const REG_MULTI_IP_TOTAL_CHNUM_MASK: u32 = 0x1f;
pub const REG_MULTI_IP_TOTAL_CHNUM_MASK_SFT: u32 = (0x1f << 15);
pub const REG_MASK_AUTO_SFT: u32 = 20;
pub const REG_MASK_AUTO_MASK: u32 = 0x1;
pub const REG_MASK_AUTO_MASK_SFT: u32 = (0x1 << 20);
pub const REG_MASK_NUM_SFT: u32 = 21;
pub const REG_MASK_NUM_MASK: u32 = 0x1f;
pub const REG_MASK_NUM_MASK_SFT: u32 = (0x1f << 21);
pub const REG_UPDATE_POINT_AUTO_SFT: u32 = 26;
pub const REG_UPDATE_POINT_AUTO_MASK: u32 = 0x1;
pub const REG_UPDATE_POINT_AUTO_MASK_SFT: u32 = (0x1 << 26);
pub const REG_SDATA_DELAY_0P5T_EN_SFT: u32 = 27;
pub const REG_SDATA_DELAY_0P5T_EN_MASK: u32 = 0x1;
pub const REG_SDATA_DELAY_0P5T_EN_MASK_SFT: u32 = (0x1 << 27);
pub const REG_SDATA_DELAY_BCK_INV_SFT: u32 = 28;
pub const REG_SDATA_DELAY_BCK_INV_MASK: u32 = 0x1;
pub const REG_SDATA_DELAY_BCK_INV_MASK_SFT: u32 = (0x1 << 28);
pub const REG_LRCK_DELAY_0P5T_EN_SFT: u32 = 29;
pub const REG_LRCK_DELAY_0P5T_EN_MASK: u32 = 0x1;
pub const REG_LRCK_DELAY_0P5T_EN_MASK_SFT: u32 = (0x1 << 29);
pub const REG_LRCK_DELAY_BCK_INV_SFT: u32 = 30;
pub const REG_LRCK_DELAY_BCK_INV_MASK: u32 = 0x1;
pub const REG_LRCK_DELAY_BCK_INV_MASK_SFT: u32 = (0x1 << 30);
pub const REG_MULTI_IP_MODE_SFT: u32 = 31;
pub const REG_MULTI_IP_MODE_MASK: u32 = 0x1;
pub const REG_MULTI_IP_MODE_MASK_SFT: u32 = (0x1 << 31);

/* ETDM_IN0_CON3 */
/* ETDM_IN1_CON3 */
/* ETDM_IN2_CON3 */
/* ETDM_IN3_CON3 */
/* ETDM_IN4_CON3 */
/* ETDM_IN5_CON3 */
/* ETDM_IN6_CON3 */
pub const REG_DISABLE_OUT_SFT: u32 = 0;
pub const REG_DISABLE_OUT_MASK: u32 = 0xffff;
pub const REG_DISABLE_OUT_MASK_SFT: u32 = (0xffff << 0);
pub const REG_RJ_DATA_RIGHT_ALIGN_SFT: u32 = 16;
pub const REG_RJ_DATA_RIGHT_ALIGN_MASK: u32 = 0x1;
pub const REG_RJ_DATA_RIGHT_ALIGN_MASK_SFT: u32 = (0x1 << 16);
pub const REG_MONITOR_SEL_SFT: u32 = 17;
pub const REG_MONITOR_SEL_MASK: u32 = 0x3;
pub const REG_MONITOR_SEL_MASK_SFT: u32 = (0x3 << 17);
pub const REG_CNT_UPPER_LIMIT_SFT: u32 = 19;
pub const REG_CNT_UPPER_LIMIT_MASK: u32 = 0x3f;
pub const REG_CNT_UPPER_LIMIT_MASK_SFT: u32 = (0x3f << 19);
pub const REG_COMPACT_SAMPLE_END_DIS_SFT: u32 = 25;
pub const REG_COMPACT_SAMPLE_END_DIS_MASK: u32 = 0x1;
pub const REG_COMPACT_SAMPLE_END_DIS_MASK_SFT: u32 = (0x1 << 25);
pub const REG_FS_TIMING_SEL_SFT: u32 = 26;
pub const REG_FS_TIMING_SEL_MASK: u32 = 0x1f;
pub const REG_FS_TIMING_SEL_MASK_SFT: u32 = (0x1f << 26);
pub const REG_SAMPLE_END_MODE_SFT: u32 = 31;
pub const REG_SAMPLE_END_MODE_MASK: u32 = 0x1;
pub const REG_SAMPLE_END_MODE_MASK_SFT: u32 = (0x1 << 31);

/* ETDM_IN0_CON4 */
/* ETDM_IN1_CON4 */
/* ETDM_IN2_CON4 */
/* ETDM_IN3_CON4 */
/* ETDM_IN4_CON4 */
/* ETDM_IN5_CON4 */
/* ETDM_IN6_CON4 */
pub const REG_ALWAYS_OPEN_1X_EN_SFT: u32 = 31;
pub const REG_ALWAYS_OPEN_1X_EN_MASK: u32 = 0x1;
pub const REG_ALWAYS_OPEN_1X_EN_MASK_SFT: u32 = (0x1 << 31);
pub const REG_WAIT_LAST_SAMPLE_SFT: u32 = 30;
pub const REG_WAIT_LAST_SAMPLE_MASK: u32 = 0x1;
pub const REG_WAIT_LAST_SAMPLE_MASK_SFT: u32 = (0x1 << 30);
pub const REG_SAMPLE_END_POINT_SFT: u32 = 25;
pub const REG_SAMPLE_END_POINT_MASK: u32 = 0x1f;
pub const REG_SAMPLE_END_POINT_MASK_SFT: u32 = (0x1f << 25);
pub const REG_RELATCH_1X_EN_SEL_SFT: u32 = 20;
pub const REG_RELATCH_1X_EN_SEL_MASK: u32 = 0x1f;
pub const REG_RELATCH_1X_EN_SEL_MASK_SFT: u32 = (0x1f << 20);
pub const REG_MASTER_WS_INV_SFT: u32 = 19;
pub const REG_MASTER_WS_INV_MASK: u32 = 0x1;
pub const REG_MASTER_WS_INV_MASK_SFT: u32 = (0x1 << 19);
pub const REG_MASTER_BCK_INV_SFT: u32 = 18;
pub const REG_MASTER_BCK_INV_MASK: u32 = 0x1;
pub const REG_MASTER_BCK_INV_MASK_SFT: u32 = (0x1 << 18);
pub const REG_SLAVE_LRCK_INV_SFT: u32 = 17;
pub const REG_SLAVE_LRCK_INV_MASK: u32 = 0x1;
pub const REG_SLAVE_LRCK_INV_MASK_SFT: u32 = (0x1 << 17);
pub const REG_SLAVE_BCK_INV_SFT: u32 = 16;
pub const REG_SLAVE_BCK_INV_MASK: u32 = 0x1;
pub const REG_SLAVE_BCK_INV_MASK_SFT: u32 = (0x1 << 16);
pub const REG_REPACK_CHNUM_SFT: u32 = 12;
pub const REG_REPACK_CHNUM_MASK: u32 = 0xf;
pub const REG_REPACK_CHNUM_MASK_SFT: u32 = (0xf << 12);
pub const REG_ASYNC_RESET_SFT: u32 = 11;
pub const REG_ASYNC_RESET_MASK: u32 = 0x1;
pub const REG_ASYNC_RESET_MASK_SFT: u32 = (0x1 << 11);
pub const REG_REPACK_WORD_LENGTH_SFT: u32 = 9;
pub const REG_REPACK_WORD_LENGTH_MASK: u32 = 0x3;
pub const REG_REPACK_WORD_LENGTH_MASK_SFT: u32 = (0x3 << 9);
pub const REG_REPACK_AUTO_MODE_SFT: u32 = 8;
pub const REG_REPACK_AUTO_MODE_MASK: u32 = 0x1;
pub const REG_REPACK_AUTO_MODE_MASK_SFT: u32 = (0x1 << 8);
pub const REG_REPACK_MODE_SFT: u32 = 0;
pub const REG_REPACK_MODE_MASK: u32 = 0x3f;
pub const REG_REPACK_MODE_MASK_SFT: u32 = (0x3f << 0);

/* ETDM_IN0_CON5 */
/* ETDM_IN1_CON5 */
/* ETDM_IN2_CON5 */
/* ETDM_IN3_CON5 */
/* ETDM_IN4_CON5 */
/* ETDM_IN5_CON5 */
/* ETDM_IN6_CON5 */
pub const REG_LR_SWAP_SFT: u32 = 16;
pub const REG_LR_SWAP_MASK: u32 = 0xffff;
pub const REG_LR_SWAP_MASK_SFT: u32 = (0xffff << 16);
pub const REG_ODD_FLAG_EN_SFT: u32 = 0;
pub const REG_ODD_FLAG_EN_MASK: u32 = 0xffff;
pub const REG_ODD_FLAG_EN_MASK_SFT: u32 = (0xffff << 0);

/* ETDM_IN0_CON6 */
/* ETDM_IN1_CON6 */
/* ETDM_IN2_CON6 */
/* ETDM_IN3_CON6 */
/* ETDM_IN4_CON6 */
/* ETDM_IN5_CON6 */
/* ETDM_IN6_CON6 */
pub const LCH_DATA_REG_SFT: u32 = 0;
pub const LCH_DATA_REG_MASK: u32 = 0xffffffff;
pub const LCH_DATA_REG_MASK_SFT: u32 = (0xffffffff << 0);

/* ETDM_IN0_CON7 */
/* ETDM_IN1_CON7 */
/* ETDM_IN2_CON7 */
/* ETDM_IN3_CON7 */
/* ETDM_IN4_CON7 */
/* ETDM_IN5_CON7 */
/* ETDM_IN6_CON7 */
pub const RCH_DATA_REG_SFT: u32 = 0;
pub const RCH_DATA_REG_MASK: u32 = 0xffffffff;
pub const RCH_DATA_REG_MASK_SFT: u32 = (0xffffffff << 0);

/* ETDM_IN0_CON8 */
/* ETDM_IN1_CON8 */
/* ETDM_IN2_CON8 */
/* ETDM_IN3_CON8 */
/* ETDM_IN4_CON8 */
/* ETDM_IN5_CON8 */
/* ETDM_IN6_CON8 */
pub const REG_AFIFO_THRESHOLD_SFT: u32 = 29;
pub const REG_AFIFO_THRESHOLD_MASK: u32 = 0x3;
pub const REG_AFIFO_THRESHOLD_MASK_SFT: u32 = (0x3 << 29);
pub const REG_CK_EN_SEL_MANUAL_SFT: u32 = 16;
pub const REG_CK_EN_SEL_MANUAL_MASK: u32 = 0x3ff;
pub const REG_CK_EN_SEL_MANUAL_MASK_SFT: u32 = (0x3ff << 16);
pub const REG_AFIFO_SW_RESET_SFT: u32 = 15;
pub const REG_AFIFO_SW_RESET_MASK: u32 = 0x1;
pub const REG_AFIFO_SW_RESET_MASK_SFT: u32 = (0x1 << 15);
pub const REG_AFIFO_RESET_SEL_SFT: u32 = 14;
pub const REG_AFIFO_RESET_SEL_MASK: u32 = 0x1;
pub const REG_AFIFO_RESET_SEL_MASK_SFT: u32 = (0x1 << 14);
pub const REG_AFIFO_AUTO_RESET_DIS_SFT: u32 = 9;
pub const REG_AFIFO_AUTO_RESET_DIS_MASK: u32 = 0x1;
pub const REG_AFIFO_AUTO_RESET_DIS_MASK_SFT: u32 = (0x1 << 9);
pub const REG_ETDM_USE_AFIFO_SFT: u32 = 8;
pub const REG_ETDM_USE_AFIFO_MASK: u32 = 0x1;
pub const REG_ETDM_USE_AFIFO_MASK_SFT: u32 = (0x1 << 8);
pub const REG_AFIFO_CLOCK_DOMAIN_SEL_SFT: u32 = 5;
pub const REG_AFIFO_CLOCK_DOMAIN_SEL_MASK: u32 = 0x7;
pub const REG_AFIFO_CLOCK_DOMAIN_SEL_MASK_SFT: u32 = (0x7 << 5);
pub const REG_AFIFO_MODE_SFT: u32 = 0;
pub const REG_AFIFO_MODE_MASK: u32 = 0x1f;
pub const REG_AFIFO_MODE_MASK_SFT: u32 = (0x1f << 0);

/* ETDM_IN0_CON9 */
/* ETDM_IN1_CON9 */
/* ETDM_IN2_CON9 */
/* ETDM_IN3_CON9 */
/* ETDM_IN4_CON9 */
/* ETDM_IN5_CON9 */
/* ETDM_IN6_CON9 */
pub const REG_OUT2LATCH_TIME_SFT: u32 = 10;
pub const REG_OUT2LATCH_TIME_MASK: u32 = 0x1f;
pub const REG_OUT2LATCH_TIME_MASK_SFT: u32 = (0x1f << 10);
pub const REG_ALMOST_END_BIT_COUNT_SFT: u32 = 5;
pub const REG_ALMOST_END_BIT_COUNT_MASK: u32 = 0x1f;
pub const REG_ALMOST_END_BIT_COUNT_MASK_SFT: u32 = (0x1f << 5);
pub const REG_ALMOST_END_CH_COUNT_SFT: u32 = 0;
pub const REG_ALMOST_END_CH_COUNT_MASK: u32 = 0x1f;
pub const REG_ALMOST_END_CH_COUNT_MASK_SFT: u32 = (0x1f << 0);

/* ETDM_IN0_MON */
/* ETDM_IN1_MON */
/* ETDM_IN2_MON */
/* ETDM_IN3_MON */
/* ETDM_IN4_MON */
/* ETDM_IN5_MON */
/* ETDM_IN6_MON */
pub const LRCK_INV_SFT: u32 = 30;
pub const LRCK_INV_MASK: u32 = 0x1;
pub const LRCK_INV_MASK_SFT: u32 = (0x1 << 30);
pub const EN_SYNC_OUT_SFT: u32 = 29;
pub const EN_SYNC_OUT_MASK: u32 = 0x1;
pub const EN_SYNC_OUT_MASK_SFT: u32 = (0x1 << 29);
pub const HOPPING_EN_SYNC_OUT_PRE_SFT: u32 = 28;
pub const HOPPING_EN_SYNC_OUT_PRE_MASK: u32 = 0x1;
pub const HOPPING_EN_SYNC_OUT_PRE_MASK_SFT: u32 = (0x1 << 28);
pub const WFULL_SFT: u32 = 27;
pub const WFULL_MASK: u32 = 0x1;
pub const WFULL_MASK_SFT: u32 = (0x1 << 27);
pub const REMPTY_SFT: u32 = 26;
pub const REMPTY_MASK: u32 = 0x1;
pub const REMPTY_MASK_SFT: u32 = (0x1 << 26);
pub const ETDM_2X_CK_EN_SFT: u32 = 25;
pub const ETDM_2X_CK_EN_MASK: u32 = 0x1;
pub const ETDM_2X_CK_EN_MASK_SFT: u32 = (0x1 << 25);
pub const ETDM_1X_CK_EN_SFT: u32 = 24;
pub const ETDM_1X_CK_EN_MASK: u32 = 0x1;
pub const ETDM_1X_CK_EN_MASK_SFT: u32 = (0x1 << 24);
pub const SDATA0_SFT: u32 = 23;
pub const SDATA0_MASK: u32 = 0x1;
pub const SDATA0_MASK_SFT: u32 = (0x1 << 23);
pub const CURRENT_STATUS_SFT: u32 = 21;
pub const CURRENT_STATUS_MASK: u32 = 0x3;
pub const CURRENT_STATUS_MASK_SFT: u32 = (0x3 << 21);
pub const BIT_POINT_SFT: u32 = 16;
pub const BIT_POINT_MASK: u32 = 0x1f;
pub const BIT_POINT_MASK_SFT: u32 = (0x1f << 16);
pub const BIT_CH_COUNT_SFT: u32 = 10;
pub const BIT_CH_COUNT_MASK: u32 = 0x3f;
pub const BIT_CH_COUNT_MASK_SFT: u32 = (0x3f << 10);
pub const BIT_COUNT_SFT: u32 = 5;
pub const BIT_COUNT_MASK: u32 = 0x1f;
pub const BIT_COUNT_MASK_SFT: u32 = (0x1f << 5);
pub const CH_COUNT_SFT: u32 = 0;
pub const CH_COUNT_MASK: u32 = 0x1f;
pub const CH_COUNT_MASK_SFT: u32 = (0x1f << 0);

/* ETDM_OUT0_CON0 */
/* ETDM_OUT1_CON0 */
/* ETDM_OUT2_CON0 */
/* ETDM_OUT3_CON0 */
/* ETDM_OUT4_CON0 */
/* ETDM_OUT5_CON0 */
/* ETDM_OUT6_CON0 */
pub const OUT_REG_ETDM_OUT_EN_SFT: u32 = 0;
pub const OUT_REG_ETDM_OUT_EN_MASK: u32 = 0x1;
pub const OUT_REG_ETDM_OUT_EN_MASK_SFT: u32 = (0x1 << 0);
pub const OUT_REG_SYNC_MODE_SFT: u32 = 1;
pub const OUT_REG_SYNC_MODE_MASK: u32 = 0x1;
pub const OUT_REG_SYNC_MODE_MASK_SFT: u32 = (0x1 << 1);
pub const OUT_REG_LSB_FIRST_SFT: u32 = 3;
pub const OUT_REG_LSB_FIRST_MASK: u32 = 0x1;
pub const OUT_REG_LSB_FIRST_MASK_SFT: u32 = (0x1 << 3);
pub const OUT_REG_SOFT_RST_SFT: u32 = 4;
pub const OUT_REG_SOFT_RST_MASK: u32 = 0x1;
pub const OUT_REG_SOFT_RST_MASK_SFT: u32 = (0x1 << 4);
pub const OUT_REG_SLAVE_MODE_SFT: u32 = 5;
pub const OUT_REG_SLAVE_MODE_MASK: u32 = 0x1;
pub const OUT_REG_SLAVE_MODE_MASK_SFT: u32 = (0x1 << 5);
pub const OUT_REG_FMT_SFT: u32 = 6;
pub const OUT_REG_FMT_MASK: u32 = 0x7;
pub const OUT_REG_FMT_MASK_SFT: u32 = (0x7 << 6);
pub const OUT_REG_LRCK_EDGE_SEL_SFT: u32 = 10;
pub const OUT_REG_LRCK_EDGE_SEL_MASK: u32 = 0x1;
pub const OUT_REG_LRCK_EDGE_SEL_MASK_SFT: u32 = (0x1 << 10);
pub const OUT_REG_BIT_LENGTH_SFT: u32 = 11;
pub const OUT_REG_BIT_LENGTH_MASK: u32 = 0x1f;
pub const OUT_REG_BIT_LENGTH_MASK_SFT: u32 = (0x1f << 11);
pub const OUT_REG_WORD_LENGTH_SFT: u32 = 16;
pub const OUT_REG_WORD_LENGTH_MASK: u32 = 0x1f;
pub const OUT_REG_WORD_LENGTH_MASK_SFT: u32 = (0x1f << 16);
pub const OUT_REG_CH_NUM_SFT: u32 = 23;
pub const OUT_REG_CH_NUM_MASK: u32 = 0x1f;
pub const OUT_REG_CH_NUM_MASK_SFT: u32 = (0x1f << 23);
pub const OUT_REG_RELATCH_DOMAIN_SEL_SFT: u32 = 28;
pub const OUT_REG_RELATCH_DOMAIN_SEL_MASK: u32 = 0x7;
pub const OUT_REG_RELATCH_DOMAIN_SEL_MASK_SFT: u32 = (0x7 << 28);
pub const OUT_REG_VALID_TOGETHER_SFT: u32 = 31;
pub const OUT_REG_VALID_TOGETHER_MASK: u32 = 0x1;
pub const OUT_REG_VALID_TOGETHER_MASK_SFT: u32 = (0x1 << 31);

/* ETDM_OUT0_CON1 */
/* ETDM_OUT1_CON1 */
/* ETDM_OUT2_CON1 */
/* ETDM_OUT3_CON1 */
/* ETDM_OUT4_CON1 */
/* ETDM_OUT5_CON1 */
/* ETDM_OUT6_CON1 */
pub const OUT_REG_INITIAL_COUNT_SFT: u32 = 0;
pub const OUT_REG_INITIAL_COUNT_MASK: u32 = 0x1f;
pub const OUT_REG_INITIAL_COUNT_MASK_SFT: u32 = (0x1f << 0);
pub const OUT_REG_INITIAL_POINT_SFT: u32 = 5;
pub const OUT_REG_INITIAL_POINT_MASK: u32 = 0x1f;
pub const OUT_REG_INITIAL_POINT_MASK_SFT: u32 = (0x1f << 5);
pub const OUT_REG_LRCK_AUTO_OFF_SFT: u32 = 10;
pub const OUT_REG_LRCK_AUTO_OFF_MASK: u32 = 0x1;
pub const OUT_REG_LRCK_AUTO_OFF_MASK_SFT: u32 = (0x1 << 10);
pub const OUT_REG_BCK_AUTO_OFF_SFT: u32 = 11;
pub const OUT_REG_BCK_AUTO_OFF_MASK: u32 = 0x1;
pub const OUT_REG_BCK_AUTO_OFF_MASK_SFT: u32 = (0x1 << 11);
pub const OUT_REG_INITIAL_LRCK_SFT: u32 = 13;
pub const OUT_REG_INITIAL_LRCK_MASK: u32 = 0x1;
pub const OUT_REG_INITIAL_LRCK_MASK_SFT: u32 = (0x1 << 13);
pub const OUT_REG_NO_ALIGN_1X_EN_SFT: u32 = 14;
pub const OUT_REG_NO_ALIGN_1X_EN_MASK: u32 = 0x1;
pub const OUT_REG_NO_ALIGN_1X_EN_MASK_SFT: u32 = (0x1 << 14);
pub const OUT_REG_LRCK_RESET_SFT: u32 = 15;
pub const OUT_REG_LRCK_RESET_MASK: u32 = 0x1;
pub const OUT_REG_LRCK_RESET_MASK_SFT: u32 = (0x1 << 15);
pub const OUT_PINMUX_MCLK_CTRL_OE_SFT: u32 = 16;
pub const OUT_PINMUX_MCLK_CTRL_OE_MASK: u32 = 0x1;
pub const OUT_PINMUX_MCLK_CTRL_OE_MASK_SFT: u32 = (0x1 << 16);
pub const OUT_REG_OUTPUT_CR_EN_SFT: u32 = 18;
pub const OUT_REG_OUTPUT_CR_EN_MASK: u32 = 0x1;
pub const OUT_REG_OUTPUT_CR_EN_MASK_SFT: u32 = (0x1 << 18);
pub const OUT_REG_LRCK_WIDTH_SFT: u32 = 19;
pub const OUT_REG_LRCK_WIDTH_MASK: u32 = 0x3ff;
pub const OUT_REG_LRCK_WIDTH_MASK_SFT: u32 = (0x3ff << 19);
pub const OUT_REG_LRCK_AUTO_MODE_SFT: u32 = 29;
pub const OUT_REG_LRCK_AUTO_MODE_MASK: u32 = 0x1;
pub const OUT_REG_LRCK_AUTO_MODE_MASK_SFT: u32 = (0x1 << 29);
pub const OUT_REG_DIRECT_INPUT_MASTER_BCK_SFT: u32 = 30;
pub const OUT_REG_DIRECT_INPUT_MASTER_BCK_MASK: u32 = 0x1;
pub const OUT_REG_DIRECT_INPUT_MASTER_BCK_MASK_SFT: u32 = (0x1 << 30);
pub const OUT_REG_16B_COMPACT_MODE_SFT: u32 = 31;
pub const OUT_REG_16B_COMPACT_MODE_MASK: u32 = 0x1;
pub const OUT_REG_16B_COMPACT_MODE_MASK_SFT: u32 = (0x1 << 31);

/* ETDM_OUT0_CON2 */
/* ETDM_OUT1_CON2 */
/* ETDM_OUT2_CON2 */
/* ETDM_OUT3_CON2 */
/* ETDM_OUT4_CON2 */
/* ETDM_OUT5_CON2 */
/* ETDM_OUT6_CON2 */
pub const OUT_REG_IN2LATCH_TIME_SFT: u32 = 0;
pub const OUT_REG_IN2LATCH_TIME_MASK: u32 = 0x1f;
pub const OUT_REG_IN2LATCH_TIME_MASK_SFT: u32 = (0x1f << 0);
pub const OUT_REG_MASK_NUM_SFT: u32 = 5;
pub const OUT_REG_MASK_NUM_MASK: u32 = 0x1f;
pub const OUT_REG_MASK_NUM_MASK_SFT: u32 = (0x1f << 5);
pub const OUT_REG_MASK_AUTO_SFT: u32 = 10;
pub const OUT_REG_MASK_AUTO_MASK: u32 = 0x1;
pub const OUT_REG_MASK_AUTO_MASK_SFT: u32 = (0x1 << 10);
pub const OUT_REG_SDATA_SHIFT_SFT: u32 = 11;
pub const OUT_REG_SDATA_SHIFT_MASK: u32 = 0x3;
pub const OUT_REG_SDATA_SHIFT_MASK_SFT: u32 = (0x3 << 11);
pub const OUT_REG_ALMOST_END_BIT_COUNT_SFT: u32 = 13;
pub const OUT_REG_ALMOST_END_BIT_COUNT_MASK: u32 = 0x1f;
pub const OUT_REG_ALMOST_END_BIT_COUNT_MASK_SFT: u32 = (0x1f << 13);
pub const OUT_REG_SDATA_CON_SFT: u32 = 18;
pub const OUT_REG_SDATA_CON_MASK: u32 = 0x3;
pub const OUT_REG_SDATA_CON_MASK_SFT: u32 = (0x3 << 18);
pub const OUT_REG_REDUNDANT_0_SFT: u32 = 20;
pub const OUT_REG_REDUNDANT_0_MASK: u32 = 0x1;
pub const OUT_REG_REDUNDANT_0_MASK_SFT: u32 = (0x1 << 20);
pub const OUT_REG_SDATA_AUTO_OFF_SFT: u32 = 21;
pub const OUT_REG_SDATA_AUTO_OFF_MASK: u32 = 0x1;
pub const OUT_REG_SDATA_AUTO_OFF_MASK_SFT: u32 = (0x1 << 21);
pub const OUT_REG_BCK_OFF_TIME_SFT: u32 = 22;
pub const OUT_REG_BCK_OFF_TIME_MASK: u32 = 0x3;
pub const OUT_REG_BCK_OFF_TIME_MASK_SFT: u32 = (0x3 << 22);
pub const OUT_REG_MONITOR_SEL_SFT: u32 = 24;
pub const OUT_REG_MONITOR_SEL_MASK: u32 = 0x3;
pub const OUT_REG_MONITOR_SEL_MASK_SFT: u32 = (0x3 << 24);
pub const OUT_REG_SHIFT_AUTO_SFT: u32 = 26;
pub const OUT_REG_SHIFT_AUTO_MASK: u32 = 0x1;
pub const OUT_REG_SHIFT_AUTO_MASK_SFT: u32 = (0x1 << 26);
pub const OUT_REG_SDATA_DELAY_0P5T_EN_SFT: u32 = 27;
pub const OUT_REG_SDATA_DELAY_0P5T_EN_MASK: u32 = 0x1;
pub const OUT_REG_SDATA_DELAY_0P5T_EN_MASK_SFT: u32 = (0x1 << 27);
pub const OUT_REG_SDATA_DELAY_BCK_INV_SFT: u32 = 28;
pub const OUT_REG_SDATA_DELAY_BCK_INV_MASK: u32 = 0x1;
pub const OUT_REG_SDATA_DELAY_BCK_INV_MASK_SFT: u32 = (0x1 << 28);
pub const OUT_REG_LRCK_DELAY_0P5T_EN_SFT: u32 = 29;
pub const OUT_REG_LRCK_DELAY_0P5T_EN_MASK: u32 = 0x1;
pub const OUT_REG_LRCK_DELAY_0P5T_EN_MASK_SFT: u32 = (0x1 << 29);
pub const OUT_REG_LRCK_DELAY_BCK_INV_SFT: u32 = 30;
pub const OUT_REG_LRCK_DELAY_BCK_INV_MASK: u32 = 0x1;
pub const OUT_REG_LRCK_DELAY_BCK_INV_MASK_SFT: u32 = (0x1 << 30);
pub const OUT_REG_OFF_CR_EN_SFT: u32 = 31;
pub const OUT_REG_OFF_CR_EN_MASK: u32 = 0x1;
pub const OUT_REG_OFF_CR_EN_MASK_SFT: u32 = (0x1 << 31);

/* ETDM_OUT0_CON3 */
/* ETDM_OUT1_CON3 */
/* ETDM_OUT2_CON3 */
/* ETDM_OUT3_CON3 */
/* ETDM_OUT4_CON3 */
/* ETDM_OUT5_CON3 */
/* ETDM_OUT6_CON3 */
pub const OUT_REG_START_CH_PAIR0_SFT: u32 = 0;
pub const OUT_REG_START_CH_PAIR0_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR0_MASK_SFT: u32 = (0xf << 0);
pub const OUT_REG_START_CH_PAIR1_SFT: u32 = 4;
pub const OUT_REG_START_CH_PAIR1_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR1_MASK_SFT: u32 = (0xf << 4);
pub const OUT_REG_START_CH_PAIR2_SFT: u32 = 8;
pub const OUT_REG_START_CH_PAIR2_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR2_MASK_SFT: u32 = (0xf << 8);
pub const OUT_REG_START_CH_PAIR3_SFT: u32 = 12;
pub const OUT_REG_START_CH_PAIR3_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR3_MASK_SFT: u32 = (0xf << 12);
pub const OUT_REG_START_CH_PAIR4_SFT: u32 = 16;
pub const OUT_REG_START_CH_PAIR4_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR4_MASK_SFT: u32 = (0xf << 16);
pub const OUT_REG_START_CH_PAIR5_SFT: u32 = 20;
pub const OUT_REG_START_CH_PAIR5_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR5_MASK_SFT: u32 = (0xf << 20);
pub const OUT_REG_START_CH_PAIR6_SFT: u32 = 24;
pub const OUT_REG_START_CH_PAIR6_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR6_MASK_SFT: u32 = (0xf << 24);
pub const OUT_REG_START_CH_PAIR7_SFT: u32 = 28;
pub const OUT_REG_START_CH_PAIR7_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR7_MASK_SFT: u32 = (0xf << 28);

/* ETDM_OUT0_CON4 */
/* ETDM_OUT1_CON4 */
/* ETDM_OUT2_CON4 */
/* ETDM_OUT3_CON4 */
/* ETDM_OUT4_CON4 */
/* ETDM_OUT5_CON4 */
/* ETDM_OUT6_CON4 */
pub const OUT_REG_FS_TIMING_SEL_SFT: u32 = 0;
pub const OUT_REG_FS_TIMING_SEL_MASK: u32 = 0x1f;
pub const OUT_REG_FS_TIMING_SEL_MASK_SFT: u32 = (0x1f << 0);
pub const OUT_REG_CLOCK_SOURCE_SEL_SFT: u32 = 6;
pub const OUT_REG_CLOCK_SOURCE_SEL_MASK: u32 = 0x7;
pub const OUT_REG_CLOCK_SOURCE_SEL_MASK_SFT: u32 = (0x7 << 6);
pub const OUT_REG_CK_EN_SEL_AUTO_SFT: u32 = 10;
pub const OUT_REG_CK_EN_SEL_AUTO_MASK: u32 = 0x1;
pub const OUT_REG_CK_EN_SEL_AUTO_MASK_SFT: u32 = (0x1 << 10);
pub const OUT_REG_ASYNC_RESET_SFT: u32 = 11;
pub const OUT_REG_ASYNC_RESET_MASK: u32 = 0x1;
pub const OUT_REG_ASYNC_RESET_MASK_SFT: u32 = (0x1 << 11);
pub const OUT_REG_CK_EN_SEL_MANUAL_SFT: u32 = 14;
pub const OUT_REG_CK_EN_SEL_MANUAL_MASK: u32 = 0x3ff;
pub const OUT_REG_CK_EN_SEL_MANUAL_MASK_SFT: u32 = (0x3ff << 14);
pub const OUT_REG_RELATCH_EN_SEL_SFT: u32 = 24;
pub const OUT_REG_RELATCH_EN_SEL_MASK: u32 = 0x1f;
pub const OUT_REG_RELATCH_EN_SEL_MASK_SFT: u32 = (0x1f << 24);
pub const OUT_REG_WAIT_LAST_SAMPLE_SFT: u32 = 30;
pub const OUT_REG_WAIT_LAST_SAMPLE_MASK: u32 = 0x1;
pub const OUT_REG_WAIT_LAST_SAMPLE_MASK_SFT: u32 = (0x1 << 30);
pub const OUT_REG_ALWAYS_OPEN_1X_EN_SFT: u32 = 31;
pub const OUT_REG_ALWAYS_OPEN_1X_EN_MASK: u32 = 0x1;
pub const OUT_REG_ALWAYS_OPEN_1X_EN_MASK_SFT: u32 = (0x1 << 31);

/* ETDM_OUT0_CON5 */
/* ETDM_OUT1_CON5 */
/* ETDM_OUT2_CON5 */
/* ETDM_OUT3_CON5 */
/* ETDM_OUT4_CON5 */
/* ETDM_OUT5_CON5 */
/* ETDM_OUT6_CON5 */
pub const OUT_REG_REPACK_BITNUM_SFT: u32 = 0;
pub const OUT_REG_REPACK_BITNUM_MASK: u32 = 0x3;
pub const OUT_REG_REPACK_BITNUM_MASK_SFT: u32 = (0x3 << 0);
pub const OUT_REG_REPACK_CHNUM_SFT: u32 = 2;
pub const OUT_REG_REPACK_CHNUM_MASK: u32 = 0xf;
pub const OUT_REG_REPACK_CHNUM_MASK_SFT: u32 = (0xf << 2);
pub const OUT_REG_SLAVE_BCK_INV_SFT: u32 = 7;
pub const OUT_REG_SLAVE_BCK_INV_MASK: u32 = 0x1;
pub const OUT_REG_SLAVE_BCK_INV_MASK_SFT: u32 = (0x1 << 7);
pub const OUT_REG_SLAVE_LRCK_INV_SFT: u32 = 8;
pub const OUT_REG_SLAVE_LRCK_INV_MASK: u32 = 0x1;
pub const OUT_REG_SLAVE_LRCK_INV_MASK_SFT: u32 = (0x1 << 8);
pub const OUT_REG_MASTER_BCK_INV_SFT: u32 = 9;
pub const OUT_REG_MASTER_BCK_INV_MASK: u32 = 0x1;
pub const OUT_REG_MASTER_BCK_INV_MASK_SFT: u32 = (0x1 << 9);
pub const OUT_REG_MASTER_WS_INV_SFT: u32 = 10;
pub const OUT_REG_MASTER_WS_INV_MASK: u32 = 0x1;
pub const OUT_REG_MASTER_WS_INV_MASK_SFT: u32 = (0x1 << 10);
pub const OUT_REG_REPACK_24B_MSB_ALIGN_SFT: u32 = 11;
pub const OUT_REG_REPACK_24B_MSB_ALIGN_MASK: u32 = 0x1;
pub const OUT_REG_REPACK_24B_MSB_ALIGN_MASK_SFT: u32 = (0x1 << 11);
pub const OUT_REG_LR_SWAP_SFT: u32 = 16;
pub const OUT_REG_LR_SWAP_MASK: u32 = 0xffff;
pub const OUT_REG_LR_SWAP_MASK_SFT: u32 = (0xffff << 16);

/* ETDM_OUT0_CON6 */
/* ETDM_OUT1_CON6 */
/* ETDM_OUT2_CON6 */
/* ETDM_OUT3_CON6 */
/* ETDM_OUT4_CON6 */
/* ETDM_OUT5_CON6 */
/* ETDM_OUT6_CON6 */
pub const OUT_LCH_DATA_REG_SFT: u32 = 0;
pub const OUT_LCH_DATA_REG_MASK: u32 = 0xffffffff;
pub const OUT_LCH_DATA_REG_MASK_SFT: u32 = (0xffffffff << 0);

/* ETDM_OUT0_CON7 */
/* ETDM_OUT1_CON7 */
/* ETDM_OUT2_CON7 */
/* ETDM_OUT3_CON7 */
/* ETDM_OUT4_CON7 */
/* ETDM_OUT5_CON7 */
/* ETDM_OUT6_CON7 */
pub const OUT_RCH_DATA_REG_SFT: u32 = 0;
pub const OUT_RCH_DATA_REG_MASK: u32 = 0xffffffff;
pub const OUT_RCH_DATA_REG_MASK_SFT: u32 = (0xffffffff << 0);

/* ETDM_OUT0_CON8 */
/* ETDM_OUT1_CON8 */
/* ETDM_OUT2_CON8 */
/* ETDM_OUT3_CON8 */
/* ETDM_OUT4_CON8 */
/* ETDM_OUT5_CON8 */
/* ETDM_OUT6_CON8 */
pub const OUT_REG_START_CH_PAIR8_SFT: u32 = 0;
pub const OUT_REG_START_CH_PAIR8_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR8_MASK_SFT: u32 = (0xf << 0);
pub const OUT_REG_START_CH_PAIR9_SFT: u32 = 4;
pub const OUT_REG_START_CH_PAIR9_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR9_MASK_SFT: u32 = (0xf << 4);
pub const OUT_REG_START_CH_PAIR10_SFT: u32 = 8;
pub const OUT_REG_START_CH_PAIR10_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR10_MASK_SFT: u32 = (0xf << 8);
pub const OUT_REG_START_CH_PAIR11_SFT: u32 = 12;
pub const OUT_REG_START_CH_PAIR11_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR11_MASK_SFT: u32 = (0xf << 12);
pub const OUT_REG_START_CH_PAIR12_SFT: u32 = 16;
pub const OUT_REG_START_CH_PAIR12_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR12_MASK_SFT: u32 = (0xf << 16);
pub const OUT_REG_START_CH_PAIR13_SFT: u32 = 20;
pub const OUT_REG_START_CH_PAIR13_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR13_MASK_SFT: u32 = (0xf << 20);
pub const OUT_REG_START_CH_PAIR14_SFT: u32 = 24;
pub const OUT_REG_START_CH_PAIR14_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR14_MASK_SFT: u32 = (0xf << 24);
pub const OUT_REG_START_CH_PAIR15_SFT: u32 = 28;
pub const OUT_REG_START_CH_PAIR15_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR15_MASK_SFT: u32 = (0xf << 28);

/* ETDM_OUT0_CON9 */
/* ETDM_OUT1_CON9 */
/* ETDM_OUT2_CON9 */
/* ETDM_OUT3_CON9 */
/* ETDM_OUT4_CON9 */
/* ETDM_OUT5_CON9 */
/* ETDM_OUT6_CON9 */
pub const OUT_REG_AFIFO_THRESHOLD_SFT: u32 = 29;
pub const OUT_REG_AFIFO_THRESHOLD_MASK: u32 = 0x3;
pub const OUT_REG_AFIFO_THRESHOLD_MASK_SFT: u32 = (0x3 << 29);
pub const OUT_REG_AFIFO_SW_RESET_SFT: u32 = 15;
pub const OUT_REG_AFIFO_SW_RESET_MASK: u32 = 0x1;
pub const OUT_REG_AFIFO_SW_RESET_MASK_SFT: u32 = (0x1 << 15);
pub const OUT_REG_AFIFO_RESET_SEL_SFT: u32 = 14;
pub const OUT_REG_AFIFO_RESET_SEL_MASK: u32 = 0x1;
pub const OUT_REG_AFIFO_RESET_SEL_MASK_SFT: u32 = (0x1 << 14);
pub const OUT_REG_AFIFO_AUTO_RESET_DIS_SFT: u32 = 9;
pub const OUT_REG_AFIFO_AUTO_RESET_DIS_MASK: u32 = 0x1;
pub const OUT_REG_AFIFO_AUTO_RESET_DIS_MASK_SFT: u32 = (0x1 << 9);
pub const OUT_REG_ETDM_USE_AFIFO_SFT: u32 = 8;
pub const OUT_REG_ETDM_USE_AFIFO_MASK: u32 = 0x1;
pub const OUT_REG_ETDM_USE_AFIFO_MASK_SFT: u32 = (0x1 << 8);
pub const OUT_REG_AFIFO_CLOCK_DOMAIN_SEL_SFT: u32 = 5;
pub const OUT_REG_AFIFO_CLOCK_DOMAIN_SEL_MASK: u32 = 0x7;
pub const OUT_REG_AFIFO_CLOCK_DOMAIN_SEL_MASK_SFT: u32 = (0x7 << 5);
pub const OUT_REG_AFIFO_MODE_SFT: u32 = 0;
pub const OUT_REG_AFIFO_MODE_MASK: u32 = 0x1f;
pub const OUT_REG_AFIFO_MODE_MASK_SFT: u32 = (0x1f << 0);

/* ETDM_OUT0_MON */
/* ETDM_OUT1_MON */
/* ETDM_OUT2_MON */
/* ETDM_OUT3_MON */
/* ETDM_OUT4_MON */
/* ETDM_OUT5_MON */
/* ETDM_OUT6_MON */
// duplicate C define preserved: #define LRCK_INV_SFT 30
// duplicate C define preserved: #define LRCK_INV_MASK 0x1
// duplicate C define preserved: #define LRCK_INV_MASK_SFT (0x1 << 30)
// duplicate C define preserved: #define EN_SYNC_OUT_SFT 29
// duplicate C define preserved: #define EN_SYNC_OUT_MASK 0x1
// duplicate C define preserved: #define EN_SYNC_OUT_MASK_SFT (0x1 << 29)
// duplicate C define preserved: #define HOPPING_EN_SYNC_OUT_PRE_SFT 28
// duplicate C define preserved: #define HOPPING_EN_SYNC_OUT_PRE_MASK 0x1
// duplicate C define preserved: #define HOPPING_EN_SYNC_OUT_PRE_MASK_SFT (0x1 << 28)
// duplicate C define preserved: #define ETDM_2X_CK_EN_SFT 25
// duplicate C define preserved: #define ETDM_2X_CK_EN_MASK 0x1
// duplicate C define preserved: #define ETDM_2X_CK_EN_MASK_SFT (0x1 << 25)
// duplicate C define preserved: #define ETDM_1X_CK_EN_SFT 24
// duplicate C define preserved: #define ETDM_1X_CK_EN_MASK 0x1
// duplicate C define preserved: #define ETDM_1X_CK_EN_MASK_SFT (0x1 << 24)
// duplicate C define preserved: #define SDATA0_SFT 23
// duplicate C define preserved: #define SDATA0_MASK 0x1
// duplicate C define preserved: #define SDATA0_MASK_SFT (0x1 << 23)
// duplicate C define preserved: #define CURRENT_STATUS_SFT 21
// duplicate C define preserved: #define CURRENT_STATUS_MASK 0x3
// duplicate C define preserved: #define CURRENT_STATUS_MASK_SFT (0x3 << 21)
// duplicate C define preserved: #define BIT_POINT_SFT 16
// duplicate C define preserved: #define BIT_POINT_MASK 0x1f
// duplicate C define preserved: #define BIT_POINT_MASK_SFT (0x1f << 16)
// duplicate C define preserved: #define BIT_CH_COUNT_SFT 10
// duplicate C define preserved: #define BIT_CH_COUNT_MASK 0x3f
// duplicate C define preserved: #define BIT_CH_COUNT_MASK_SFT (0x3f << 10)
// duplicate C define preserved: #define BIT_COUNT_SFT 5
// duplicate C define preserved: #define BIT_COUNT_MASK 0x1f
// duplicate C define preserved: #define BIT_COUNT_MASK_SFT (0x1f << 5)
// duplicate C define preserved: #define CH_COUNT_SFT 0
// duplicate C define preserved: #define CH_COUNT_MASK 0x1f
// duplicate C define preserved: #define CH_COUNT_MASK_SFT (0x1f << 0)

/* ETDM_0_3_COWORK_CON0 */
pub const ETDM_OUT0_DATA_SEL_SFT: u32 = 0;
pub const ETDM_OUT0_DATA_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT0_DATA_SEL_MASK_SFT: u32 = (0xf << 0);
pub const ETDM_OUT0_SYNC_SEL_SFT: u32 = 4;
pub const ETDM_OUT0_SYNC_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT0_SYNC_SEL_MASK_SFT: u32 = (0xf << 4);
pub const ETDM_OUT0_SLAVE_SEL_SFT: u32 = 8;
pub const ETDM_OUT0_SLAVE_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT0_SLAVE_SEL_MASK_SFT: u32 = (0xf << 8);
pub const ETDM_OUT1_DATA_SEL_SFT: u32 = 12;
pub const ETDM_OUT1_DATA_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT1_DATA_SEL_MASK_SFT: u32 = (0xf << 12);
pub const ETDM_OUT1_SYNC_SEL_SFT: u32 = 16;
pub const ETDM_OUT1_SYNC_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT1_SYNC_SEL_MASK_SFT: u32 = (0xf << 16);
pub const ETDM_OUT1_SLAVE_SEL_SFT: u32 = 20;
pub const ETDM_OUT1_SLAVE_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT1_SLAVE_SEL_MASK_SFT: u32 = (0xf << 20);
pub const ETDM_IN0_SLAVE_SEL_SFT: u32 = 24;
pub const ETDM_IN0_SLAVE_SEL_MASK: u32 = 0xf;
pub const ETDM_IN0_SLAVE_SEL_MASK_SFT: u32 = (0xf << 24);
pub const ETDM_IN0_SYNC_SEL_SFT: u32 = 28;
pub const ETDM_IN0_SYNC_SEL_MASK: u32 = 0xf;
pub const ETDM_IN0_SYNC_SEL_MASK_SFT: u32 = (0xf << 28);

/* ETDM_0_3_COWORK_CON1 */
pub const ETDM_IN0_SDATA0_SEL_SFT: u32 = 0;
pub const ETDM_IN0_SDATA0_SEL_MASK: u32 = 0xf;
pub const ETDM_IN0_SDATA0_SEL_MASK_SFT: u32 = (0xf << 0);
pub const ETDM_IN0_SDATA1_15_SEL_SFT: u32 = 4;
pub const ETDM_IN0_SDATA1_15_SEL_MASK: u32 = 0xf;
pub const ETDM_IN0_SDATA1_15_SEL_MASK_SFT: u32 = (0xf << 4);
pub const ETDM_IN1_SLAVE_SEL_SFT: u32 = 8;
pub const ETDM_IN1_SLAVE_SEL_MASK: u32 = 0xf;
pub const ETDM_IN1_SLAVE_SEL_MASK_SFT: u32 = (0xf << 8);
pub const ETDM_IN1_SYNC_SEL_SFT: u32 = 12;
pub const ETDM_IN1_SYNC_SEL_MASK: u32 = 0xf;
pub const ETDM_IN1_SYNC_SEL_MASK_SFT: u32 = (0xf << 12);
pub const ETDM_IN1_SDATA0_SEL_SFT: u32 = 16;
pub const ETDM_IN1_SDATA0_SEL_MASK: u32 = 0xf;
pub const ETDM_IN1_SDATA0_SEL_MASK_SFT: u32 = (0xf << 16);
pub const ETDM_IN1_SDATA1_15_SEL_SFT: u32 = 20;
pub const ETDM_IN1_SDATA1_15_SEL_MASK: u32 = 0xf;
pub const ETDM_IN1_SDATA1_15_SEL_MASK_SFT: u32 = (0xf << 20);

/* ETDM_0_3_COWORK_CON2 */
pub const ETDM_OUT2_DATA_SEL_SFT: u32 = 0;
pub const ETDM_OUT2_DATA_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT2_DATA_SEL_MASK_SFT: u32 = (0xf << 0);
pub const ETDM_OUT2_SYNC_SEL_SFT: u32 = 4;
pub const ETDM_OUT2_SYNC_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT2_SYNC_SEL_MASK_SFT: u32 = (0xf << 4);
pub const ETDM_OUT2_SLAVE_SEL_SFT: u32 = 8;
pub const ETDM_OUT2_SLAVE_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT2_SLAVE_SEL_MASK_SFT: u32 = (0xf << 8);
pub const ETDM_OUT3_DATA_SEL_SFT: u32 = 12;
pub const ETDM_OUT3_DATA_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT3_DATA_SEL_MASK_SFT: u32 = (0xf << 12);
pub const ETDM_OUT3_SYNC_SEL_SFT: u32 = 16;
pub const ETDM_OUT3_SYNC_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT3_SYNC_SEL_MASK_SFT: u32 = (0xf << 16);
pub const ETDM_OUT3_SLAVE_SEL_SFT: u32 = 20;
pub const ETDM_OUT3_SLAVE_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT3_SLAVE_SEL_MASK_SFT: u32 = (0xf << 20);
pub const ETDM_IN2_SLAVE_SEL_SFT: u32 = 24;
pub const ETDM_IN2_SLAVE_SEL_MASK: u32 = 0xf;
pub const ETDM_IN2_SLAVE_SEL_MASK_SFT: u32 = (0xf << 24);
pub const ETDM_IN2_SYNC_SEL_SFT: u32 = 28;
pub const ETDM_IN2_SYNC_SEL_MASK: u32 = 0xf;
pub const ETDM_IN2_SYNC_SEL_MASK_SFT: u32 = (0xf << 28);

/* ETDM_0_3_COWORK_CON3 */
pub const ETDM_IN2_SDATA0_SEL_SFT: u32 = 0;
pub const ETDM_IN2_SDATA0_SEL_MASK: u32 = 0xf;
pub const ETDM_IN2_SDATA0_SEL_MASK_SFT: u32 = (0xf << 0);
pub const ETDM_IN2_SDATA1_15_SEL_SFT: u32 = 4;
pub const ETDM_IN2_SDATA1_15_SEL_MASK: u32 = 0xf;
pub const ETDM_IN2_SDATA1_15_SEL_MASK_SFT: u32 = (0xf << 4);
pub const ETDM_IN3_SLAVE_SEL_SFT: u32 = 8;
pub const ETDM_IN3_SLAVE_SEL_MASK: u32 = 0xf;
pub const ETDM_IN3_SLAVE_SEL_MASK_SFT: u32 = (0xf << 8);
pub const ETDM_IN3_SYNC_SEL_SFT: u32 = 12;
pub const ETDM_IN3_SYNC_SEL_MASK: u32 = 0xf;
pub const ETDM_IN3_SYNC_SEL_MASK_SFT: u32 = (0xf << 12);
pub const ETDM_IN3_SDATA0_SEL_SFT: u32 = 16;
pub const ETDM_IN3_SDATA0_SEL_MASK: u32 = 0xf;
pub const ETDM_IN3_SDATA0_SEL_MASK_SFT: u32 = (0xf << 16);
pub const ETDM_IN3_SDATA1_15_SEL_SFT: u32 = 20;
pub const ETDM_IN3_SDATA1_15_SEL_MASK: u32 = 0xf;
pub const ETDM_IN3_SDATA1_15_SEL_MASK_SFT: u32 = (0xf << 20);

/* ETDM_4_7_COWORK_CON0 */
pub const ETDM_OUT4_DATA_SEL_SFT: u32 = 0;
pub const ETDM_OUT4_DATA_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT4_DATA_SEL_MASK_SFT: u32 = (0xf << 0);
pub const ETDM_OUT4_SYNC_SEL_SFT: u32 = 4;
pub const ETDM_OUT4_SYNC_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT4_SYNC_SEL_MASK_SFT: u32 = (0xf << 4);
pub const ETDM_OUT4_SLAVE_SEL_SFT: u32 = 8;
pub const ETDM_OUT4_SLAVE_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT4_SLAVE_SEL_MASK_SFT: u32 = (0xf << 8);
pub const ETDM_OUT5_DATA_SEL_SFT: u32 = 12;
pub const ETDM_OUT5_DATA_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT5_DATA_SEL_MASK_SFT: u32 = (0xf << 12);
pub const ETDM_OUT5_SYNC_SEL_SFT: u32 = 16;
pub const ETDM_OUT5_SYNC_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT5_SYNC_SEL_MASK_SFT: u32 = (0xf << 16);
pub const ETDM_OUT5_SLAVE_SEL_SFT: u32 = 20;
pub const ETDM_OUT5_SLAVE_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT5_SLAVE_SEL_MASK_SFT: u32 = (0xf << 20);
pub const ETDM_IN4_SLAVE_SEL_SFT: u32 = 24;
pub const ETDM_IN4_SLAVE_SEL_MASK: u32 = 0xf;
pub const ETDM_IN4_SLAVE_SEL_MASK_SFT: u32 = (0xf << 24);
pub const ETDM_IN4_SYNC_SEL_SFT: u32 = 28;
pub const ETDM_IN4_SYNC_SEL_MASK: u32 = 0xf;
pub const ETDM_IN4_SYNC_SEL_MASK_SFT: u32 = (0xf << 28);

/* ETDM_4_7_COWORK_CON1 */
pub const ETDM_IN4_SDATA0_SEL_SFT: u32 = 0;
pub const ETDM_IN4_SDATA0_SEL_MASK: u32 = 0xf;
pub const ETDM_IN4_SDATA0_SEL_MASK_SFT: u32 = (0xf << 0);
pub const ETDM_IN4_SDATA1_15_SEL_SFT: u32 = 4;
pub const ETDM_IN4_SDATA1_15_SEL_MASK: u32 = 0xf;
pub const ETDM_IN4_SDATA1_15_SEL_MASK_SFT: u32 = (0xf << 4);
pub const ETDM_IN5_SLAVE_SEL_SFT: u32 = 8;
pub const ETDM_IN5_SLAVE_SEL_MASK: u32 = 0xf;
pub const ETDM_IN5_SLAVE_SEL_MASK_SFT: u32 = (0xf << 8);
pub const ETDM_IN5_SYNC_SEL_SFT: u32 = 12;
pub const ETDM_IN5_SYNC_SEL_MASK: u32 = 0xf;
pub const ETDM_IN5_SYNC_SEL_MASK_SFT: u32 = (0xf << 12);
pub const ETDM_IN5_SDATA0_SEL_SFT: u32 = 16;
pub const ETDM_IN5_SDATA0_SEL_MASK: u32 = 0xf;
pub const ETDM_IN5_SDATA0_SEL_MASK_SFT: u32 = (0xf << 16);
pub const ETDM_IN5_SDATA1_15_SEL_SFT: u32 = 20;
pub const ETDM_IN5_SDATA1_15_SEL_MASK: u32 = 0xf;
pub const ETDM_IN5_SDATA1_15_SEL_MASK_SFT: u32 = (0xf << 20);

/* ETDM_4_7_COWORK_CON2 */
pub const ETDM_OUT6_DATA_SEL_SFT: u32 = 0;
pub const ETDM_OUT6_DATA_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT6_DATA_SEL_MASK_SFT: u32 = (0xf << 0);
pub const ETDM_OUT6_SYNC_SEL_SFT: u32 = 4;
pub const ETDM_OUT6_SYNC_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT6_SYNC_SEL_MASK_SFT: u32 = (0xf << 4);
pub const ETDM_OUT6_SLAVE_SEL_SFT: u32 = 8;
pub const ETDM_OUT6_SLAVE_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT6_SLAVE_SEL_MASK_SFT: u32 = (0xf << 8);
pub const ETDM_OUT7_DATA_SEL_SFT: u32 = 12;
pub const ETDM_OUT7_DATA_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT7_DATA_SEL_MASK_SFT: u32 = (0xf << 12);
pub const ETDM_OUT7_SYNC_SEL_SFT: u32 = 16;
pub const ETDM_OUT7_SYNC_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT7_SYNC_SEL_MASK_SFT: u32 = (0xf << 16);
pub const ETDM_OUT7_SLAVE_SEL_SFT: u32 = 20;
pub const ETDM_OUT7_SLAVE_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT7_SLAVE_SEL_MASK_SFT: u32 = (0xf << 20);
pub const ETDM_IN6_SLAVE_SEL_SFT: u32 = 24;
pub const ETDM_IN6_SLAVE_SEL_MASK: u32 = 0xf;
pub const ETDM_IN6_SLAVE_SEL_MASK_SFT: u32 = (0xf << 24);
pub const ETDM_IN6_SYNC_SEL_SFT: u32 = 28;
pub const ETDM_IN6_SYNC_SEL_MASK: u32 = 0xf;
pub const ETDM_IN6_SYNC_SEL_MASK_SFT: u32 = (0xf << 28);

/* ETDM_4_7_COWORK_CON3 */
pub const ETDM_IN6_SDATA0_SEL_SFT: u32 = 0;
pub const ETDM_IN6_SDATA0_SEL_MASK: u32 = 0xf;
pub const ETDM_IN6_SDATA0_SEL_MASK_SFT: u32 = (0xf << 0);
pub const ETDM_IN6_SDATA1_15_SEL_SFT: u32 = 4;
pub const ETDM_IN6_SDATA1_15_SEL_MASK: u32 = 0xf;
pub const ETDM_IN6_SDATA1_15_SEL_MASK_SFT: u32 = (0xf << 4);
pub const ETDM_IN7_SLAVE_SEL_SFT: u32 = 8;
pub const ETDM_IN7_SLAVE_SEL_MASK: u32 = 0xf;
pub const ETDM_IN7_SLAVE_SEL_MASK_SFT: u32 = (0xf << 8);
pub const ETDM_IN7_SYNC_SEL_SFT: u32 = 12;
pub const ETDM_IN7_SYNC_SEL_MASK: u32 = 0xf;
pub const ETDM_IN7_SYNC_SEL_MASK_SFT: u32 = (0xf << 12);
pub const ETDM_IN7_SDATA0_SEL_SFT: u32 = 16;
pub const ETDM_IN7_SDATA0_SEL_MASK: u32 = 0xf;
pub const ETDM_IN7_SDATA0_SEL_MASK_SFT: u32 = (0xf << 16);
pub const ETDM_IN7_SDATA1_15_SEL_SFT: u32 = 20;
pub const ETDM_IN7_SDATA1_15_SEL_MASK: u32 = 0xf;
pub const ETDM_IN7_SDATA1_15_SEL_MASK_SFT: u32 = (0xf << 20);

/* AFE_DPTX_CON */
pub const DPTX_CHANNEL_ENABLE_SFT: u32 = 8;
pub const DPTX_CHANNEL_ENABLE_MASK: u32 = 0xff;
pub const DPTX_CHANNEL_ENABLE_MASK_SFT: u32 = (0xff << 8);
pub const DPTX_REGISTER_MONITOR_SELECT_SFT: u32 = 3;
pub const DPTX_REGISTER_MONITOR_SELECT_MASK: u32 = 0xf;
pub const DPTX_REGISTER_MONITOR_SELECT_MASK_SFT: u32 = (0xf << 3);
pub const DPTX_16BIT_SFT: u32 = 2;
pub const DPTX_16BIT_MASK: u32 = 0x1;
pub const DPTX_16BIT_MASK_SFT: u32 = (0x1 << 2);
pub const DPTX_CHANNEL_NUMBER_SFT: u32 = 1;
pub const DPTX_CHANNEL_NUMBER_MASK: u32 = 0x1;
pub const DPTX_CHANNEL_NUMBER_MASK_SFT: u32 = (0x1 << 1);
pub const DPTX_ON_SFT: u32 = 0;
pub const DPTX_ON_MASK: u32 = 0x1;
pub const DPTX_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_DPTX_MON */
pub const AFE_DPTX_MON0_SFT: u32 = 0;
pub const AFE_DPTX_MON0_MASK: u32 = 0xffffffff;
pub const AFE_DPTX_MON0_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_TDM_CON1 */
pub const TDM_EN_SFT: u32 = 0;
pub const TDM_EN_MASK: u32 = 0x1;
pub const TDM_EN_MASK_SFT: u32 = (0x1 << 0);
pub const BCK_INVERSE_SFT: u32 = 1;
pub const BCK_INVERSE_MASK: u32 = 0x1;
pub const BCK_INVERSE_MASK_SFT: u32 = (0x1 << 1);
pub const LRCK_INVERSE_SFT: u32 = 2;
pub const LRCK_INVERSE_MASK: u32 = 0x1;
pub const LRCK_INVERSE_MASK_SFT: u32 = (0x1 << 2);
pub const DELAY_DATA_SFT: u32 = 3;
pub const DELAY_DATA_MASK: u32 = 0x1;
pub const DELAY_DATA_MASK_SFT: u32 = (0x1 << 3);
pub const LEFT_ALIGN_SFT: u32 = 4;
pub const LEFT_ALIGN_MASK: u32 = 0x1;
pub const LEFT_ALIGN_MASK_SFT: u32 = (0x1 << 4);
pub const TDM_LRCK_D0P5T_SFT: u32 = 5;
pub const TDM_LRCK_D0P5T_MASK: u32 = 0x1;
pub const TDM_LRCK_D0P5T_MASK_SFT: u32 = (0x1 << 5);
pub const TDM_SDATA_D0P5T_SFT: u32 = 6;
pub const TDM_SDATA_D0P5T_MASK: u32 = 0x1;
pub const TDM_SDATA_D0P5T_MASK_SFT: u32 = (0x1 << 6);
pub const WLEN_SFT: u32 = 8;
pub const WLEN_MASK: u32 = 0x3;
pub const WLEN_MASK_SFT: u32 = (0x3 << 8);
pub const CHANNEL_NUM_SFT: u32 = 10;
pub const CHANNEL_NUM_MASK: u32 = 0x3;
pub const CHANNEL_NUM_MASK_SFT: u32 = (0x3 << 10);
pub const CHANNEL_BCK_CYCLES_SFT: u32 = 12;
pub const CHANNEL_BCK_CYCLES_MASK: u32 = 0x3;
pub const CHANNEL_BCK_CYCLES_MASK_SFT: u32 = (0x3 << 12);
pub const HDMI_CLK_INV_SEL_SFT: u32 = 15;
pub const HDMI_CLK_INV_SEL_MASK: u32 = 0x1;
pub const HDMI_CLK_INV_SEL_MASK_SFT: u32 = (0x1 << 15);
pub const DAC_BIT_NUM_SFT: u32 = 16;
pub const DAC_BIT_NUM_MASK: u32 = 0x1f;
pub const DAC_BIT_NUM_MASK_SFT: u32 = (0x1f << 16);
pub const LRCK_TDM_WIDTH_SFT: u32 = 24;
pub const LRCK_TDM_WIDTH_MASK: u32 = 0xff;
pub const LRCK_TDM_WIDTH_MASK_SFT: u32 = (0xff << 24);

/* AFE_TDM_CON2 */
pub const ST_CH_PAIR_SOUT0_SFT: u32 = 0;
pub const ST_CH_PAIR_SOUT0_MASK: u32 = 0x7;
pub const ST_CH_PAIR_SOUT0_MASK_SFT: u32 = (0x7 << 0);
pub const ST_CH_PAIR_SOUT1_SFT: u32 = 4;
pub const ST_CH_PAIR_SOUT1_MASK: u32 = 0x7;
pub const ST_CH_PAIR_SOUT1_MASK_SFT: u32 = (0x7 << 4);
pub const ST_CH_PAIR_SOUT2_SFT: u32 = 8;
pub const ST_CH_PAIR_SOUT2_MASK: u32 = 0x7;
pub const ST_CH_PAIR_SOUT2_MASK_SFT: u32 = (0x7 << 8);
pub const ST_CH_PAIR_SOUT3_SFT: u32 = 12;
pub const ST_CH_PAIR_SOUT3_MASK: u32 = 0x7;
pub const ST_CH_PAIR_SOUT3_MASK_SFT: u32 = (0x7 << 12);
pub const TDM_FIX_VALUE_SEL_SFT: u32 = 16;
pub const TDM_FIX_VALUE_SEL_MASK: u32 = 0x1;
pub const TDM_FIX_VALUE_SEL_MASK_SFT: u32 = (0x1 << 16);
pub const TDM_I2S_LOOPBACK_SFT: u32 = 20;
pub const TDM_I2S_LOOPBACK_MASK: u32 = 0x1;
pub const TDM_I2S_LOOPBACK_MASK_SFT: u32 = (0x1 << 20);
pub const TDM_I2S_LOOPBACK_CH_SFT: u32 = 21;
pub const TDM_I2S_LOOPBACK_CH_MASK: u32 = 0x3;
pub const TDM_I2S_LOOPBACK_CH_MASK_SFT: u32 = (0x3 << 21);
pub const TDM_USE_SINEGEN_INPUT_SFT: u32 = 23;
pub const TDM_USE_SINEGEN_INPUT_MASK: u32 = 0x1;
pub const TDM_USE_SINEGEN_INPUT_MASK_SFT: u32 = (0x1 << 23);
pub const TDM_FIX_VALUE_SFT: u32 = 24;
pub const TDM_FIX_VALUE_MASK: u32 = 0xff;
pub const TDM_FIX_VALUE_MASK_SFT: u32 = (0xff << 24);

/* AFE_TDM_CON3 */
pub const TDM_OUT_SEL_DOMAIN_SFT: u32 = 29;
pub const TDM_OUT_SEL_DOMAIN_MASK: u32 = 0x7;
pub const TDM_OUT_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 29);
pub const TDM_OUT_SEL_FS_SFT: u32 = 24;
pub const TDM_OUT_SEL_FS_MASK: u32 = 0x1f;
pub const TDM_OUT_SEL_FS_MASK_SFT: u32 = (0x1f << 24);
pub const TDM_OUT_MON_SEL_SFT: u32 = 3;
pub const TDM_OUT_MON_SEL_MASK: u32 = 0x1;
pub const TDM_OUT_MON_SEL_MASK_SFT: u32 = (0x1 << 3);
pub const RG_TDM_OUT_ASYNC_FIFO_SOFT_RST_EN_SFT: u32 = 2;
pub const RG_TDM_OUT_ASYNC_FIFO_SOFT_RST_EN_MASK: u32 = 0x1;
pub const RG_TDM_OUT_ASYNC_FIFO_SOFT_RST_EN_MASK_SFT: u32 = (0x1 << 2);
pub const RG_TDM_OUT_ASYNC_FIFO_SOFT_RST_SFT: u32 = 1;
pub const RG_TDM_OUT_ASYNC_FIFO_SOFT_RST_MASK: u32 = 0x1;
pub const RG_TDM_OUT_ASYNC_FIFO_SOFT_RST_MASK_SFT: u32 = (0x1 << 1);
pub const TDM_UPDATE_EN_SEL_SFT: u32 = 0;
pub const TDM_UPDATE_EN_SEL_MASK: u32 = 0x1;
pub const TDM_UPDATE_EN_SEL_MASK_SFT: u32 = (0x1 << 0);

/* AFE_TDM_OUT_MON */
pub const AFE_TDM_OUT_MON_SFT: u32 = 0;
pub const AFE_TDM_OUT_MON_MASK: u32 = 0xffffffff;
pub const AFE_TDM_OUT_MON_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_HDMI_CONN0 */
pub const HDMI_O_7_SFT: u32 = 21;
pub const HDMI_O_7_MASK: u32 = 0x7;
pub const HDMI_O_7_MASK_SFT: u32 = (0x7 << 21);
pub const HDMI_O_6_SFT: u32 = 18;
pub const HDMI_O_6_MASK: u32 = 0x7;
pub const HDMI_O_6_MASK_SFT: u32 = (0x7 << 18);
pub const HDMI_O_5_SFT: u32 = 15;
pub const HDMI_O_5_MASK: u32 = 0x7;
pub const HDMI_O_5_MASK_SFT: u32 = (0x7 << 15);
pub const HDMI_O_4_SFT: u32 = 12;
pub const HDMI_O_4_MASK: u32 = 0x7;
pub const HDMI_O_4_MASK_SFT: u32 = (0x7 << 12);
pub const HDMI_O_3_SFT: u32 = 9;
pub const HDMI_O_3_MASK: u32 = 0x7;
pub const HDMI_O_3_MASK_SFT: u32 = (0x7 << 9);
pub const HDMI_O_2_SFT: u32 = 6;
pub const HDMI_O_2_MASK: u32 = 0x7;
pub const HDMI_O_2_MASK_SFT: u32 = (0x7 << 6);
pub const HDMI_O_1_SFT: u32 = 3;
pub const HDMI_O_1_MASK: u32 = 0x7;
pub const HDMI_O_1_MASK_SFT: u32 = (0x7 << 3);
pub const HDMI_O_0_SFT: u32 = 0;
pub const HDMI_O_0_MASK: u32 = 0x7;
pub const HDMI_O_0_MASK_SFT: u32 = (0x7 << 0);

/* AFE_TDM_TOP_IP_VERSION */
pub const AFE_TDM_TOP_IP_VERSION_SFT: u32 = 0;
pub const AFE_TDM_TOP_IP_VERSION_MASK: u32 = 0xffffffff;
pub const AFE_TDM_TOP_IP_VERSION_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_CBIP_CFG0 */
pub const CBIP_TOP_SLV_MUX_WAY_EN_SFT: u32 = 16;
pub const CBIP_TOP_SLV_MUX_WAY_EN_MASK: u32 = 0xffff;
pub const CBIP_TOP_SLV_MUX_WAY_EN_MASK_SFT: u32 = (0xffff << 16);
pub const RESERVED_04_SFT: u32 = 15;
pub const RESERVED_04_MASK: u32 = 0x1;
pub const RESERVED_04_MASK_SFT: u32 = (0x1 << 15);
pub const CBIP_ASYNC_MST_RG_FIFO_THRE_SFT: u32 = 13;
pub const CBIP_ASYNC_MST_RG_FIFO_THRE_MASK: u32 = 0x3;
pub const CBIP_ASYNC_MST_RG_FIFO_THRE_MASK_SFT: u32 = (0x3 << 13);
pub const CBIP_ASYNC_MST_POSTWRITE_DIS_SFT: u32 = 12;
pub const CBIP_ASYNC_MST_POSTWRITE_DIS_MASK: u32 = 0x1;
pub const CBIP_ASYNC_MST_POSTWRITE_DIS_MASK_SFT: u32 = (0x1 << 12);
pub const RESERVED_03_SFT: u32 = 11;
pub const RESERVED_03_MASK: u32 = 0x1;
pub const RESERVED_03_MASK_SFT: u32 = (0x1 << 11);
pub const CBIP_ASYNC_SLV_RG_FIFO_THRE_SFT: u32 = 9;
pub const CBIP_ASYNC_SLV_RG_FIFO_THRE_MASK: u32 = 0x3;
pub const CBIP_ASYNC_SLV_RG_FIFO_THRE_MASK_SFT: u32 = (0x3 << 9);
pub const CBIP_ASYNC_SLV_POSTWRITE_DIS_SFT: u32 = 8;
pub const CBIP_ASYNC_SLV_POSTWRITE_DIS_MASK: u32 = 0x1;
pub const CBIP_ASYNC_SLV_POSTWRITE_DIS_MASK_SFT: u32 = (0x1 << 8);
pub const AUDIOSYS_BUSY_SFT: u32 = 7;
pub const AUDIOSYS_BUSY_MASK: u32 = 0x1;
pub const AUDIOSYS_BUSY_MASK_SFT: u32 = (0x1 << 7);
pub const CBIP_SLV_DECODER_ERR_FLAG_EN_SFT: u32 = 6;
pub const CBIP_SLV_DECODER_ERR_FLAG_EN_MASK: u32 = 0x1;
pub const CBIP_SLV_DECODER_ERR_FLAG_EN_MASK_SFT: u32 = (0x1 << 6);
pub const CBIP_SLV_DECODER_SLAVE_WAY_EN_SFT: u32 = 5;
pub const CBIP_SLV_DECODER_SLAVE_WAY_EN_MASK: u32 = 0x1;
pub const CBIP_SLV_DECODER_SLAVE_WAY_EN_MASK_SFT: u32 = (0x1 << 5);
pub const APB_R2T_SFT: u32 = 3;
pub const APB_R2T_MASK: u32 = 0x1;
pub const APB_R2T_MASK_SFT: u32 = (0x1 << 3);
pub const APB_W2T_SFT: u32 = 2;
pub const APB_W2T_MASK: u32 = 0x1;
pub const APB_W2T_MASK_SFT: u32 = (0x1 << 2);
pub const AHB_IDLE_EN_INT_SFT: u32 = 1;
pub const AHB_IDLE_EN_INT_MASK: u32 = 0x1;
pub const AHB_IDLE_EN_INT_MASK_SFT: u32 = (0x1 << 1);
pub const AHB_IDLE_EN_EXT_SFT: u32 = 0;
pub const AHB_IDLE_EN_EXT_MASK: u32 = 0x1;
pub const AHB_IDLE_EN_EXT_MASK_SFT: u32 = (0x1 << 0);

/* AFE_CBIP_SLV_DECODER_MON0 */
pub const CBIP_SLV_DECODER_ERR_DOMAIN_SFT: u32 = 4;
pub const CBIP_SLV_DECODER_ERR_DOMAIN_MASK: u32 = 0x1;
pub const CBIP_SLV_DECODER_ERR_DOMAIN_MASK_SFT: u32 = (0x1 << 4);
pub const CBIP_SLV_DECODER_ERR_ID_SFT: u32 = 3;
pub const CBIP_SLV_DECODER_ERR_ID_MASK: u32 = 0x1;
pub const CBIP_SLV_DECODER_ERR_ID_MASK_SFT: u32 = (0x1 << 3);
pub const CBIP_SLV_DECODER_ERR_RW_SFT: u32 = 2;
pub const CBIP_SLV_DECODER_ERR_RW_MASK: u32 = 0x1;
pub const CBIP_SLV_DECODER_ERR_RW_MASK_SFT: u32 = (0x1 << 2);
pub const CBIP_SLV_DECODER_ERR_DECERR_SFT: u32 = 1;
pub const CBIP_SLV_DECODER_ERR_DECERR_MASK: u32 = 0x1;
pub const CBIP_SLV_DECODER_ERR_DECERR_MASK_SFT: u32 = (0x1 << 1);
pub const CBIP_SLV_DECODER_CTRL_UPDATE_STATUS_SFT: u32 = 0;
pub const CBIP_SLV_DECODER_CTRL_UPDATE_STATUS_MASK: u32 = 0x1;
pub const CBIP_SLV_DECODER_CTRL_UPDATE_STATUS_MASK_SFT: u32 = (0x1 << 0);

/* AFE_CBIP_SLV_DECODER_MON1 */
pub const CBIP_SLV_DECODER_ERR_ADDR_SFT: u32 = 0;
pub const CBIP_SLV_DECODER_ERR_ADDR_MASK: u32 = 0xffffffff;
pub const CBIP_SLV_DECODER_ERR_ADDR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_CBIP_SLV_MUX_MON_CFG */
pub const CBIP_SLV_MUX_ERR_FLAG_EN_SFT: u32 = 3;
pub const CBIP_SLV_MUX_ERR_FLAG_EN_MASK: u32 = 0x1;
pub const CBIP_SLV_MUX_ERR_FLAG_EN_MASK_SFT: u32 = (0x1 << 3);
pub const CBIP_SLV_MUX_REG_SLAVE_WAY_EN_SFT: u32 = 2;
pub const CBIP_SLV_MUX_REG_SLAVE_WAY_EN_MASK: u32 = 0x1;
pub const CBIP_SLV_MUX_REG_SLAVE_WAY_EN_MASK_SFT: u32 = (0x1 << 2);
pub const CBIP_SLV_MUX_REG_LAYER_WAY_EN_SFT: u32 = 0;
pub const CBIP_SLV_MUX_REG_LAYER_WAY_EN_MASK: u32 = 0x3;
pub const CBIP_SLV_MUX_REG_LAYER_WAY_EN_MASK_SFT: u32 = (0x3 << 0);

/* AFE_CBIP_SLV_MUX_MON0 */
pub const CBIP_SLV_MUX_ERR_DOMAIN_SFT: u32 = 8;
pub const CBIP_SLV_MUX_ERR_DOMAIN_MASK: u32 = 0x1;
pub const CBIP_SLV_MUX_ERR_DOMAIN_MASK_SFT: u32 = (0x1 << 8);
pub const CBIP_SLV_MUX_ERR_ID_SFT: u32 = 7;
pub const CBIP_SLV_MUX_ERR_ID_MASK: u32 = 0x1;
pub const CBIP_SLV_MUX_ERR_ID_MASK_SFT: u32 = (0x1 << 7);
pub const CBIP_SLV_MUX_ERR_RD_SFT: u32 = 6;
pub const CBIP_SLV_MUX_ERR_RD_MASK: u32 = 0x1;
pub const CBIP_SLV_MUX_ERR_RD_MASK_SFT: u32 = (0x1 << 6);
pub const CBIP_SLV_MUX_ERR_WR_SFT: u32 = 5;
pub const CBIP_SLV_MUX_ERR_WR_MASK: u32 = 0x1;
pub const CBIP_SLV_MUX_ERR_WR_MASK_SFT: u32 = (0x1 << 5);
pub const CBIP_SLV_MUX_ERR_EN_SLV_SFT: u32 = 4;
pub const CBIP_SLV_MUX_ERR_EN_SLV_MASK: u32 = 0x1;
pub const CBIP_SLV_MUX_ERR_EN_SLV_MASK_SFT: u32 = (0x1 << 4);
pub const CBIP_SLV_MUX_ERR_EN_MST_SFT: u32 = 2;
pub const CBIP_SLV_MUX_ERR_EN_MST_MASK: u32 = 0x3;
pub const CBIP_SLV_MUX_ERR_EN_MST_MASK_SFT: u32 = (0x3 << 2);
pub const CBIP_SLV_MUX_CTRL_UPDATE_STATUS_SFT: u32 = 0;
pub const CBIP_SLV_MUX_CTRL_UPDATE_STATUS_MASK: u32 = 0x3;
pub const CBIP_SLV_MUX_CTRL_UPDATE_STATUS_MASK_SFT: u32 = (0x3 << 0);

/* AFE_CBIP_SLV_MUX_MON1 */
pub const CBIP_SLV_MUX_ERR_ADDR_SFT: u32 = 0;
pub const CBIP_SLV_MUX_ERR_ADDR_MASK: u32 = 0xffffffff;
pub const CBIP_SLV_MUX_ERR_ADDR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_MEMIF_CON0 */
pub const CPU_COMPACT_MODE_SFT: u32 = 2;
pub const CPU_COMPACT_MODE_MASK: u32 = 0x1;
pub const CPU_COMPACT_MODE_MASK_SFT: u32 = (0x1 << 2);
pub const CPU_HD_ALIGN_SFT: u32 = 1;
pub const CPU_HD_ALIGN_MASK: u32 = 0x1;
pub const CPU_HD_ALIGN_MASK_SFT: u32 = (0x1 << 1);
pub const SYSRAM_SIGN_SFT: u32 = 0;
pub const SYSRAM_SIGN_MASK: u32 = 0x1;
pub const SYSRAM_SIGN_MASK_SFT: u32 = (0x1 << 0);

/* AFE_MEMIF_ONE_HEART */
pub const DL_ONE_HEART_ON_2_SFT: u32 = 2;
pub const DL_ONE_HEART_ON_2_MASK: u32 = 0x1;
pub const DL_ONE_HEART_ON_2_MASK_SFT: u32 = (0x1 << 2);
pub const DL_ONE_HEART_ON_1_SFT: u32 = 1;
pub const DL_ONE_HEART_ON_1_MASK: u32 = 0x1;
pub const DL_ONE_HEART_ON_1_MASK_SFT: u32 = (0x1 << 1);
pub const DL_ONE_HEART_ON_0_SFT: u32 = 0;
pub const DL_ONE_HEART_ON_0_MASK: u32 = 0x1;
pub const DL_ONE_HEART_ON_0_MASK_SFT: u32 = (0x1 << 0);

/* AFE_DL0_BASE_MSB */
pub const DL0_BASE_ADDR_MSB_SFT: u32 = 0;
pub const DL0_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL0_BASE_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL0_BASE */
pub const DL0_BASE_ADDR_SFT: u32 = 4;
pub const DL0_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL0_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL0_CUR_MSB */
pub const DL0_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL0_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL0_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL0_CUR */
pub const DL0_CUR_PTR_SFT: u32 = 0;
pub const DL0_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL0_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL0_END_MSB */
pub const DL0_END_ADDR_MSB_SFT: u32 = 0;
pub const DL0_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL0_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL0_END */
pub const DL0_END_ADDR_SFT: u32 = 4;
pub const DL0_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL0_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL0_RCH_MON */
pub const DL0_RCH_DATA_SFT: u32 = 0;
pub const DL0_RCH_DATA_MASK: u32 = 0xffffffff;
pub const DL0_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL0_LCH_MON */
pub const DL0_LCH_DATA_SFT: u32 = 0;
pub const DL0_LCH_DATA_MASK: u32 = 0xffffffff;
pub const DL0_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL0_CON0 */
pub const DL0_ON_SFT: u32 = 28;
pub const DL0_ON_MASK: u32 = 0x1;
pub const DL0_ON_MASK_SFT: u32 = (0x1 << 28);
pub const DL0_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL0_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL0_ONE_HEART_SEL_MASK_SFT: u32 = (0x3 << 22);
pub const DL0_MINLEN_SFT: u32 = 20;
pub const DL0_MINLEN_MASK: u32 = 0x3;
pub const DL0_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const DL0_MAXLEN_SFT: u32 = 16;
pub const DL0_MAXLEN_MASK: u32 = 0x3;
pub const DL0_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const DL0_SEL_DOMAIN_SFT: u32 = 13;
pub const DL0_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL0_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const DL0_SEL_FS_SFT: u32 = 8;
pub const DL0_SEL_FS_MASK: u32 = 0x1f;
pub const DL0_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const DL0_SW_CLEAR_BUF_EMPTY_SFT: u32 = 7;
pub const DL0_SW_CLEAR_BUF_EMPTY_MASK: u32 = 0x1;
pub const DL0_SW_CLEAR_BUF_EMPTY_MASK_SFT: u32 = (0x1 << 7);
pub const DL0_PBUF_SIZE_SFT: u32 = 5;
pub const DL0_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL0_PBUF_SIZE_MASK_SFT: u32 = (0x3 << 5);
pub const DL0_MONO_SFT: u32 = 4;
pub const DL0_MONO_MASK: u32 = 0x1;
pub const DL0_MONO_MASK_SFT: u32 = (0x1 << 4);
pub const DL0_NORMAL_MODE_SFT: u32 = 3;
pub const DL0_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL0_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const DL0_HALIGN_SFT: u32 = 2;
pub const DL0_HALIGN_MASK: u32 = 0x1;
pub const DL0_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const DL0_HD_MODE_SFT: u32 = 0;
pub const DL0_HD_MODE_MASK: u32 = 0x3;
pub const DL0_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_DL1_BASE_MSB */
pub const DL1_BASE_ADDR_MSB_SFT: u32 = 0;
pub const DL1_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL1_BASE_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL1_BASE */
pub const DL1_BASE_ADDR_SFT: u32 = 4;
pub const DL1_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL1_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL1_CUR_MSB */
pub const DL1_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL1_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL1_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL1_CUR */
pub const DL1_CUR_PTR_SFT: u32 = 0;
pub const DL1_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL1_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL1_END_MSB */
pub const DL1_END_ADDR_MSB_SFT: u32 = 0;
pub const DL1_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL1_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL1_END */
pub const DL1_END_ADDR_SFT: u32 = 4;
pub const DL1_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL1_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL1_RCH_MON */
pub const DL1_RCH_DATA_SFT: u32 = 0;
pub const DL1_RCH_DATA_MASK: u32 = 0xffffffff;
pub const DL1_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL1_LCH_MON */
pub const DL1_LCH_DATA_SFT: u32 = 0;
pub const DL1_LCH_DATA_MASK: u32 = 0xffffffff;
pub const DL1_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL1_CON0 */
pub const DL1_ON_SFT: u32 = 28;
pub const DL1_ON_MASK: u32 = 0x1;
pub const DL1_ON_MASK_SFT: u32 = (0x1 << 28);
pub const DL1_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL1_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL1_ONE_HEART_SEL_MASK_SFT: u32 = (0x3 << 22);
pub const DL1_MINLEN_SFT: u32 = 20;
pub const DL1_MINLEN_MASK: u32 = 0x3;
pub const DL1_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const DL1_MAXLEN_SFT: u32 = 16;
pub const DL1_MAXLEN_MASK: u32 = 0x3;
pub const DL1_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const DL1_SEL_DOMAIN_SFT: u32 = 13;
pub const DL1_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL1_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const DL1_SEL_FS_SFT: u32 = 8;
pub const DL1_SEL_FS_MASK: u32 = 0x1f;
pub const DL1_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const DL1_SW_CLEAR_BUF_EMPTY_SFT: u32 = 7;
pub const DL1_SW_CLEAR_BUF_EMPTY_MASK: u32 = 0x1;
pub const DL1_SW_CLEAR_BUF_EMPTY_MASK_SFT: u32 = (0x1 << 7);
pub const DL1_PBUF_SIZE_SFT: u32 = 5;
pub const DL1_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL1_PBUF_SIZE_MASK_SFT: u32 = (0x3 << 5);
pub const DL1_MONO_SFT: u32 = 4;
pub const DL1_MONO_MASK: u32 = 0x1;
pub const DL1_MONO_MASK_SFT: u32 = (0x1 << 4);
pub const DL1_NORMAL_MODE_SFT: u32 = 3;
pub const DL1_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL1_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const DL1_HALIGN_SFT: u32 = 2;
pub const DL1_HALIGN_MASK: u32 = 0x1;
pub const DL1_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const DL1_HD_MODE_SFT: u32 = 0;
pub const DL1_HD_MODE_MASK: u32 = 0x3;
pub const DL1_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_DL2_BASE_MSB */
pub const DL2_BASE__ADDR_MSB_SFT: u32 = 0;
pub const DL2_BASE__ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL2_BASE__ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL2_BASE */
pub const DL2_BASE_ADDR_SFT: u32 = 4;
pub const DL2_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL2_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL2_CUR_MSB */
pub const DL2_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL2_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL2_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL2_CUR */
pub const DL2_CUR_PTR_SFT: u32 = 0;
pub const DL2_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL2_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL2_END_MSB */
pub const DL2_END_ADDR_MSB_SFT: u32 = 0;
pub const DL2_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL2_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL2_END */
pub const DL2_END_ADDR_SFT: u32 = 4;
pub const DL2_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL2_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL2_RCH_MON */
pub const DL2_RCH_DATA_SFT: u32 = 0;
pub const DL2_RCH_DATA_MASK: u32 = 0xffffffff;
pub const DL2_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL2_LCH_MON */
pub const DL2_LCH_DATA_SFT: u32 = 0;
pub const DL2_LCH_DATA_MASK: u32 = 0xffffffff;
pub const DL2_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL2_CON0 */
pub const DL2_ON_SFT: u32 = 28;
pub const DL2_ON_MASK: u32 = 0x1;
pub const DL2_ON_MASK_SFT: u32 = (0x1 << 28);
pub const DL2_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL2_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL2_ONE_HEART_SEL_MASK_SFT: u32 = (0x3 << 22);
pub const DL2_MINLEN_SFT: u32 = 20;
pub const DL2_MINLEN_MASK: u32 = 0x3;
pub const DL2_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const DL2_MAXLEN_SFT: u32 = 16;
pub const DL2_MAXLEN_MASK: u32 = 0x3;
pub const DL2_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const DL2_SEL_DOMAIN_SFT: u32 = 13;
pub const DL2_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL2_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const DL2_SEL_FS_SFT: u32 = 8;
pub const DL2_SEL_FS_MASK: u32 = 0x1f;
pub const DL2_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const DL2_SW_CLEAR_BUF_EMPTY_SFT: u32 = 7;
pub const DL2_SW_CLEAR_BUF_EMPTY_MASK: u32 = 0x1;
pub const DL2_SW_CLEAR_BUF_EMPTY_MASK_SFT: u32 = (0x1 << 7);
pub const DL2_PBUF_SIZE_SFT: u32 = 5;
pub const DL2_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL2_PBUF_SIZE_MASK_SFT: u32 = (0x3 << 5);
pub const DL2_MONO_SFT: u32 = 4;
pub const DL2_MONO_MASK: u32 = 0x1;
pub const DL2_MONO_MASK_SFT: u32 = (0x1 << 4);
pub const DL2_NORMAL_MODE_SFT: u32 = 3;
pub const DL2_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL2_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const DL2_HALIGN_SFT: u32 = 2;
pub const DL2_HALIGN_MASK: u32 = 0x1;
pub const DL2_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const DL2_HD_MODE_SFT: u32 = 0;
pub const DL2_HD_MODE_MASK: u32 = 0x3;
pub const DL2_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_DL3_BASE_MSB */
pub const DL3_BASE__ADDR_MSB_SFT: u32 = 0;
pub const DL3_BASE__ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL3_BASE__ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL3_BASE */
pub const DL3_BASE_ADDR_SFT: u32 = 4;
pub const DL3_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL3_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL3_CUR_MSB */
pub const DL3_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL3_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL3_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL3_CUR */
pub const DL3_CUR_PTR_SFT: u32 = 0;
pub const DL3_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL3_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL3_END_MSB */
pub const DL3_END_ADDR_MSB_SFT: u32 = 0;
pub const DL3_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL3_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL3_END */
pub const DL3_END_ADDR_SFT: u32 = 4;
pub const DL3_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL3_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL3_RCH_MON */
pub const DL3_RCH_DATA_SFT: u32 = 0;
pub const DL3_RCH_DATA_MASK: u32 = 0xffffffff;
pub const DL3_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL3_LCH_MON */
pub const DL3_LCH_DATA_SFT: u32 = 0;
pub const DL3_LCH_DATA_MASK: u32 = 0xffffffff;
pub const DL3_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL3_CON0 */
pub const DL3_ON_SFT: u32 = 28;
pub const DL3_ON_MASK: u32 = 0x1;
pub const DL3_ON_MASK_SFT: u32 = (0x1 << 28);
pub const DL3_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL3_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL3_ONE_HEART_SEL_MASK_SFT: u32 = (0x3 << 22);
pub const DL3_MINLEN_SFT: u32 = 20;
pub const DL3_MINLEN_MASK: u32 = 0x3;
pub const DL3_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const DL3_MAXLEN_SFT: u32 = 16;
pub const DL3_MAXLEN_MASK: u32 = 0x3;
pub const DL3_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const DL3_SEL_DOMAIN_SFT: u32 = 13;
pub const DL3_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL3_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const DL3_SEL_FS_SFT: u32 = 8;
pub const DL3_SEL_FS_MASK: u32 = 0x1f;
pub const DL3_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const DL3_SW_CLEAR_BUF_EMPTY_SFT: u32 = 7;
pub const DL3_SW_CLEAR_BUF_EMPTY_MASK: u32 = 0x1;
pub const DL3_SW_CLEAR_BUF_EMPTY_MASK_SFT: u32 = (0x1 << 7);
pub const DL3_PBUF_SIZE_SFT: u32 = 5;
pub const DL3_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL3_PBUF_SIZE_MASK_SFT: u32 = (0x3 << 5);
pub const DL3_MONO_SFT: u32 = 4;
pub const DL3_MONO_MASK: u32 = 0x1;
pub const DL3_MONO_MASK_SFT: u32 = (0x1 << 4);
pub const DL3_NORMAL_MODE_SFT: u32 = 3;
pub const DL3_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL3_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const DL3_HALIGN_SFT: u32 = 2;
pub const DL3_HALIGN_MASK: u32 = 0x1;
pub const DL3_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const DL3_HD_MODE_SFT: u32 = 0;
pub const DL3_HD_MODE_MASK: u32 = 0x3;
pub const DL3_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_DL4_BASE_MSB */
pub const DL4_BASE__ADDR_MSB_SFT: u32 = 0;
pub const DL4_BASE__ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL4_BASE__ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL4_BASE */
pub const DL4_BASE_ADDR_SFT: u32 = 4;
pub const DL4_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL4_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL4_CUR_MSB */
pub const DL4_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL4_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL4_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL4_CUR */
pub const DL4_CUR_PTR_SFT: u32 = 0;
pub const DL4_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL4_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL4_END_MSB */
pub const DL4_END_ADDR_MSB_SFT: u32 = 0;
pub const DL4_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL4_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL4_END */
pub const DL4_END_ADDR_SFT: u32 = 4;
pub const DL4_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL4_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL4_RCH_MON */
pub const DL4_RCH_DATA_SFT: u32 = 0;
pub const DL4_RCH_DATA_MASK: u32 = 0xffffffff;
pub const DL4_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL4_LCH_MON */
pub const DL4_LCH_DATA_SFT: u32 = 0;
pub const DL4_LCH_DATA_MASK: u32 = 0xffffffff;
pub const DL4_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL4_CON0 */
pub const DL4_ON_SFT: u32 = 28;
pub const DL4_ON_MASK: u32 = 0x1;
pub const DL4_ON_MASK_SFT: u32 = (0x1 << 28);
pub const DL4_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL4_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL4_ONE_HEART_SEL_MASK_SFT: u32 = (0x3 << 22);
pub const DL4_MINLEN_SFT: u32 = 20;
pub const DL4_MINLEN_MASK: u32 = 0x3;
pub const DL4_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const DL4_MAXLEN_SFT: u32 = 16;
pub const DL4_MAXLEN_MASK: u32 = 0x3;
pub const DL4_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const DL4_SEL_DOMAIN_SFT: u32 = 13;
pub const DL4_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL4_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const DL4_SEL_FS_SFT: u32 = 8;
pub const DL4_SEL_FS_MASK: u32 = 0x1f;
pub const DL4_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const DL4_SW_CLEAR_BUF_EMPTY_SFT: u32 = 7;
pub const DL4_SW_CLEAR_BUF_EMPTY_MASK: u32 = 0x1;
pub const DL4_SW_CLEAR_BUF_EMPTY_MASK_SFT: u32 = (0x1 << 7);
pub const DL4_PBUF_SIZE_SFT: u32 = 5;
pub const DL4_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL4_PBUF_SIZE_MASK_SFT: u32 = (0x3 << 5);
pub const DL4_MONO_SFT: u32 = 4;
pub const DL4_MONO_MASK: u32 = 0x1;
pub const DL4_MONO_MASK_SFT: u32 = (0x1 << 4);
pub const DL4_NORMAL_MODE_SFT: u32 = 3;
pub const DL4_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL4_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const DL4_HALIGN_SFT: u32 = 2;
pub const DL4_HALIGN_MASK: u32 = 0x1;
pub const DL4_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const DL4_HD_MODE_SFT: u32 = 0;
pub const DL4_HD_MODE_MASK: u32 = 0x3;
pub const DL4_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_DL5_BASE_MSB */
pub const DL5_BASE__ADDR_MSB_SFT: u32 = 0;
pub const DL5_BASE__ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL5_BASE__ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL5_BASE */
pub const DL5_BASE_ADDR_SFT: u32 = 4;
pub const DL5_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL5_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL5_CUR_MSB */
pub const DL5_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL5_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL5_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL5_CUR */
pub const DL5_CUR_PTR_SFT: u32 = 0;
pub const DL5_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL5_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL5_END_MSB */
pub const DL5_END_ADDR_MSB_SFT: u32 = 0;
pub const DL5_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL5_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL5_END */
pub const DL5_END_ADDR_SFT: u32 = 4;
pub const DL5_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL5_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL5_RCH_MON */
pub const DL5_RCH_DATA_SFT: u32 = 0;
pub const DL5_RCH_DATA_MASK: u32 = 0xffffffff;
pub const DL5_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL5_LCH_MON */
pub const DL5_LCH_DATA_SFT: u32 = 0;
pub const DL5_LCH_DATA_MASK: u32 = 0xffffffff;
pub const DL5_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL5_CON0 */
pub const DL5_ON_SFT: u32 = 28;
pub const DL5_ON_MASK: u32 = 0x1;
pub const DL5_ON_MASK_SFT: u32 = (0x1 << 28);
pub const DL5_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL5_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL5_ONE_HEART_SEL_MASK_SFT: u32 = (0x3 << 22);
pub const DL5_MINLEN_SFT: u32 = 20;
pub const DL5_MINLEN_MASK: u32 = 0x3;
pub const DL5_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const DL5_MAXLEN_SFT: u32 = 16;
pub const DL5_MAXLEN_MASK: u32 = 0x3;
pub const DL5_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const DL5_SEL_DOMAIN_SFT: u32 = 13;
pub const DL5_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL5_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const DL5_SEL_FS_SFT: u32 = 8;
pub const DL5_SEL_FS_MASK: u32 = 0x1f;
pub const DL5_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const DL5_SW_CLEAR_BUF_EMPTY_SFT: u32 = 7;
pub const DL5_SW_CLEAR_BUF_EMPTY_MASK: u32 = 0x1;
pub const DL5_SW_CLEAR_BUF_EMPTY_MASK_SFT: u32 = (0x1 << 7);
pub const DL5_PBUF_SIZE_SFT: u32 = 5;
pub const DL5_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL5_PBUF_SIZE_MASK_SFT: u32 = (0x3 << 5);
pub const DL5_MONO_SFT: u32 = 4;
pub const DL5_MONO_MASK: u32 = 0x1;
pub const DL5_MONO_MASK_SFT: u32 = (0x1 << 4);
pub const DL5_NORMAL_MODE_SFT: u32 = 3;
pub const DL5_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL5_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const DL5_HALIGN_SFT: u32 = 2;
pub const DL5_HALIGN_MASK: u32 = 0x1;
pub const DL5_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const DL5_HD_MODE_SFT: u32 = 0;
pub const DL5_HD_MODE_MASK: u32 = 0x3;
pub const DL5_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_DL6_BASE_MSB */
pub const DL6_BASE__ADDR_MSB_SFT: u32 = 0;
pub const DL6_BASE__ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL6_BASE__ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL6_BASE */
pub const DL6_BASE_ADDR_SFT: u32 = 4;
pub const DL6_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL6_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL6_CUR_MSB */
pub const DL6_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL6_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL6_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL6_CUR */
pub const DL6_CUR_PTR_SFT: u32 = 0;
pub const DL6_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL6_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL6_END_MSB */
pub const DL6_END_ADDR_MSB_SFT: u32 = 0;
pub const DL6_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL6_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL6_END */
pub const DL6_END_ADDR_SFT: u32 = 4;
pub const DL6_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL6_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL6_RCH_MON */
pub const DL6_RCH_DATA_SFT: u32 = 0;
pub const DL6_RCH_DATA_MASK: u32 = 0xffffffff;
pub const DL6_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL6_LCH_MON */
pub const DL6_LCH_DATA_SFT: u32 = 0;
pub const DL6_LCH_DATA_MASK: u32 = 0xffffffff;
pub const DL6_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL6_CON0 */
pub const DL6_ON_SFT: u32 = 28;
pub const DL6_ON_MASK: u32 = 0x1;
pub const DL6_ON_MASK_SFT: u32 = (0x1 << 28);
pub const DL6_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL6_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL6_ONE_HEART_SEL_MASK_SFT: u32 = (0x3 << 22);
pub const DL6_MINLEN_SFT: u32 = 20;
pub const DL6_MINLEN_MASK: u32 = 0x3;
pub const DL6_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const DL6_MAXLEN_SFT: u32 = 16;
pub const DL6_MAXLEN_MASK: u32 = 0x3;
pub const DL6_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const DL6_SEL_DOMAIN_SFT: u32 = 13;
pub const DL6_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL6_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const DL6_SEL_FS_SFT: u32 = 8;
pub const DL6_SEL_FS_MASK: u32 = 0x1f;
pub const DL6_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const DL6_SW_CLEAR_BUF_EMPTY_SFT: u32 = 7;
pub const DL6_SW_CLEAR_BUF_EMPTY_MASK: u32 = 0x1;
pub const DL6_SW_CLEAR_BUF_EMPTY_MASK_SFT: u32 = (0x1 << 7);
pub const DL6_PBUF_SIZE_SFT: u32 = 5;
pub const DL6_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL6_PBUF_SIZE_MASK_SFT: u32 = (0x3 << 5);
pub const DL6_MONO_SFT: u32 = 4;
pub const DL6_MONO_MASK: u32 = 0x1;
pub const DL6_MONO_MASK_SFT: u32 = (0x1 << 4);
pub const DL6_NORMAL_MODE_SFT: u32 = 3;
pub const DL6_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL6_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const DL6_HALIGN_SFT: u32 = 2;
pub const DL6_HALIGN_MASK: u32 = 0x1;
pub const DL6_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const DL6_HD_MODE_SFT: u32 = 0;
pub const DL6_HD_MODE_MASK: u32 = 0x3;
pub const DL6_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_DL7_BASE_MSB */
pub const DL7_BASE__ADDR_MSB_SFT: u32 = 0;
pub const DL7_BASE__ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL7_BASE__ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL7_BASE */
pub const DL7_BASE_ADDR_SFT: u32 = 4;
pub const DL7_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL7_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL7_CUR_MSB */
pub const DL7_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL7_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL7_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL7_CUR */
pub const DL7_CUR_PTR_SFT: u32 = 0;
pub const DL7_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL7_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL7_END_MSB */
pub const DL7_END_ADDR_MSB_SFT: u32 = 0;
pub const DL7_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL7_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL7_END */
pub const DL7_END_ADDR_SFT: u32 = 4;
pub const DL7_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL7_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL7_RCH_MON */
pub const DL7_RCH_DATA_SFT: u32 = 0;
pub const DL7_RCH_DATA_MASK: u32 = 0xffffffff;
pub const DL7_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL7_LCH_MON */
pub const DL7_LCH_DATA_SFT: u32 = 0;
pub const DL7_LCH_DATA_MASK: u32 = 0xffffffff;
pub const DL7_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL7_CON0 */
pub const DL7_ON_SFT: u32 = 28;
pub const DL7_ON_MASK: u32 = 0x1;
pub const DL7_ON_MASK_SFT: u32 = (0x1 << 28);
pub const DL7_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL7_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL7_ONE_HEART_SEL_MASK_SFT: u32 = (0x3 << 22);
pub const DL7_MINLEN_SFT: u32 = 20;
pub const DL7_MINLEN_MASK: u32 = 0x3;
pub const DL7_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const DL7_MAXLEN_SFT: u32 = 16;
pub const DL7_MAXLEN_MASK: u32 = 0x3;
pub const DL7_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const DL7_SEL_DOMAIN_SFT: u32 = 13;
pub const DL7_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL7_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const DL7_SEL_FS_SFT: u32 = 8;
pub const DL7_SEL_FS_MASK: u32 = 0x1f;
pub const DL7_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const DL7_SW_CLEAR_BUF_EMPTY_SFT: u32 = 7;
pub const DL7_SW_CLEAR_BUF_EMPTY_MASK: u32 = 0x1;
pub const DL7_SW_CLEAR_BUF_EMPTY_MASK_SFT: u32 = (0x1 << 7);
pub const DL7_PBUF_SIZE_SFT: u32 = 5;
pub const DL7_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL7_PBUF_SIZE_MASK_SFT: u32 = (0x3 << 5);
pub const DL7_MONO_SFT: u32 = 4;
pub const DL7_MONO_MASK: u32 = 0x1;
pub const DL7_MONO_MASK_SFT: u32 = (0x1 << 4);
pub const DL7_NORMAL_MODE_SFT: u32 = 3;
pub const DL7_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL7_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const DL7_HALIGN_SFT: u32 = 2;
pub const DL7_HALIGN_MASK: u32 = 0x1;
pub const DL7_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const DL7_HD_MODE_SFT: u32 = 0;
pub const DL7_HD_MODE_MASK: u32 = 0x3;
pub const DL7_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_DL8_BASE_MSB */
pub const DL8_BASE__ADDR_MSB_SFT: u32 = 0;
pub const DL8_BASE__ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL8_BASE__ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL8_BASE */
pub const DL8_BASE_ADDR_SFT: u32 = 4;
pub const DL8_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL8_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL8_CUR_MSB */
pub const DL8_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL8_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL8_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL8_CUR */
pub const DL8_CUR_PTR_SFT: u32 = 0;
pub const DL8_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL8_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL8_END_MSB */
pub const DL8_END_ADDR_MSB_SFT: u32 = 0;
pub const DL8_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL8_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL8_END */
pub const DL8_END_ADDR_SFT: u32 = 4;
pub const DL8_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL8_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL8_RCH_MON */
pub const DL8_RCH_DATA_SFT: u32 = 0;
pub const DL8_RCH_DATA_MASK: u32 = 0xffffffff;
pub const DL8_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL8_LCH_MON */
pub const DL8_LCH_DATA_SFT: u32 = 0;
pub const DL8_LCH_DATA_MASK: u32 = 0xffffffff;
pub const DL8_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL8_CON0 */
pub const DL8_ON_SFT: u32 = 28;
pub const DL8_ON_MASK: u32 = 0x1;
pub const DL8_ON_MASK_SFT: u32 = (0x1 << 28);
pub const DL8_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL8_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL8_ONE_HEART_SEL_MASK_SFT: u32 = (0x3 << 22);
pub const DL8_MINLEN_SFT: u32 = 20;
pub const DL8_MINLEN_MASK: u32 = 0x3;
pub const DL8_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const DL8_MAXLEN_SFT: u32 = 16;
pub const DL8_MAXLEN_MASK: u32 = 0x3;
pub const DL8_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const DL8_SEL_DOMAIN_SFT: u32 = 13;
pub const DL8_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL8_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const DL8_SEL_FS_SFT: u32 = 8;
pub const DL8_SEL_FS_MASK: u32 = 0x1f;
pub const DL8_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const DL8_SW_CLEAR_BUF_EMPTY_SFT: u32 = 7;
pub const DL8_SW_CLEAR_BUF_EMPTY_MASK: u32 = 0x1;
pub const DL8_SW_CLEAR_BUF_EMPTY_MASK_SFT: u32 = (0x1 << 7);
pub const DL8_PBUF_SIZE_SFT: u32 = 5;
pub const DL8_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL8_PBUF_SIZE_MASK_SFT: u32 = (0x3 << 5);
pub const DL8_MONO_SFT: u32 = 4;
pub const DL8_MONO_MASK: u32 = 0x1;
pub const DL8_MONO_MASK_SFT: u32 = (0x1 << 4);
pub const DL8_NORMAL_MODE_SFT: u32 = 3;
pub const DL8_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL8_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const DL8_HALIGN_SFT: u32 = 2;
pub const DL8_HALIGN_MASK: u32 = 0x1;
pub const DL8_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const DL8_HD_MODE_SFT: u32 = 0;
pub const DL8_HD_MODE_MASK: u32 = 0x3;
pub const DL8_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_DL_4CH_BASE_MSB */
pub const DL_4CH_BASE__ADDR_MSB_SFT: u32 = 0;
pub const DL_4CH_BASE__ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL_4CH_BASE__ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL_4CH_BASE */
pub const DL_4CH_BASE_ADDR_SFT: u32 = 4;
pub const DL_4CH_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL_4CH_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL_4CH_CUR_MSB */
pub const DL_4CH_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL_4CH_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL_4CH_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL_4CH_CUR */
pub const DL_4CH_CUR_PTR_SFT: u32 = 0;
pub const DL_4CH_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL_4CH_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL_4CH_END_MSB */
pub const DL_4CH_END_ADDR_MSB_SFT: u32 = 0;
pub const DL_4CH_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL_4CH_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL_4CH_END */
pub const DL_4CH_END_ADDR_SFT: u32 = 4;
pub const DL_4CH_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL_4CH_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL_4CH_CON0 */
pub const DL_4CH_ON_SFT: u32 = 31;
pub const DL_4CH_ON_MASK: u32 = 0x1;
pub const DL_4CH_ON_MASK_SFT: u32 = (0x1 << 31);
pub const DL_4CH_NUM_SFT: u32 = 24;
pub const DL_4CH_NUM_MASK: u32 = 0x1f;
pub const DL_4CH_NUM_MASK_SFT: u32 = (0x1f << 24);
pub const DL_4CH_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL_4CH_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL_4CH_ONE_HEART_SEL_MASK_SFT: u32 = (0x3 << 22);
pub const DL_4CH_MINLEN_SFT: u32 = 20;
pub const DL_4CH_MINLEN_MASK: u32 = 0x3;
pub const DL_4CH_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const DL_4CH_MAXLEN_SFT: u32 = 16;
pub const DL_4CH_MAXLEN_MASK: u32 = 0x3;
pub const DL_4CH_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const DL_4CH_SEL_DOMAIN_SFT: u32 = 13;
pub const DL_4CH_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL_4CH_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const DL_4CH_SEL_FS_SFT: u32 = 8;
pub const DL_4CH_SEL_FS_MASK: u32 = 0x1f;
pub const DL_4CH_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const DL_4CH_BUF_EMPTY_CLR_SFT: u32 = 7;
pub const DL_4CH_BUF_EMPTY_CLR_MASK: u32 = 0x1;
pub const DL_4CH_BUF_EMPTY_CLR_MASK_SFT: u32 = (0x1 << 7);
pub const DL_4CH_PBUF_SIZE_SFT: u32 = 5;
pub const DL_4CH_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL_4CH_PBUF_SIZE_MASK_SFT: u32 = (0x3 << 5);
pub const DL_4CH_HANG_CLR_SFT: u32 = 4;
pub const DL_4CH_HANG_CLR_MASK: u32 = 0x1;
pub const DL_4CH_HANG_CLR_MASK_SFT: u32 = (0x1 << 4);
pub const DL_4CH_NORMAL_MODE_SFT: u32 = 3;
pub const DL_4CH_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL_4CH_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const DL_4CH_HALIGN_SFT: u32 = 2;
pub const DL_4CH_HALIGN_MASK: u32 = 0x1;
pub const DL_4CH_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const DL_4CH_HD_MODE_SFT: u32 = 0;
pub const DL_4CH_HD_MODE_MASK: u32 = 0x3;
pub const DL_4CH_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_DL_24CH_BASE_MSB */
pub const DL_24CH_BASE__ADDR_MSB_SFT: u32 = 0;
pub const DL_24CH_BASE__ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL_24CH_BASE__ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL_24CH_BASE */
pub const DL_24CH_BASE_ADDR_SFT: u32 = 4;
pub const DL_24CH_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL_24CH_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL_24CH_CUR_MSB */
pub const DL_24CH_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL_24CH_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL_24CH_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL_24CH_CUR */
pub const DL_24CH_CUR_PTR_SFT: u32 = 0;
pub const DL_24CH_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL_24CH_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL_24CH_END_MSB */
pub const DL_24CH_END_ADDR_MSB_SFT: u32 = 0;
pub const DL_24CH_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL_24CH_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL_24CH_END */
pub const DL_24CH_END_ADDR_SFT: u32 = 4;
pub const DL_24CH_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL_24CH_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL_24CH_CON0 */
pub const DL_24CH_ON_SFT: u32 = 31;
pub const DL_24CH_ON_MASK: u32 = 0x1;
pub const DL_24CH_ON_MASK_SFT: u32 = (0x1 << 31);
pub const DL_24CH_NUM_SFT: u32 = 24;
pub const DL_24CH_NUM_MASK: u32 = 0x3f;
pub const DL_24CH_NUM_MASK_SFT: u32 = (0x3f << 24);
pub const DL_24CH_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL_24CH_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL_24CH_ONE_HEART_SEL_MASK_SFT: u32 = (0x3 << 22);
pub const DL_24CH_MINLEN_SFT: u32 = 20;
pub const DL_24CH_MINLEN_MASK: u32 = 0x3;
pub const DL_24CH_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const DL_24CH_MAXLEN_SFT: u32 = 16;
pub const DL_24CH_MAXLEN_MASK: u32 = 0x3;
pub const DL_24CH_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const DL_24CH_SEL_DOMAIN_SFT: u32 = 13;
pub const DL_24CH_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL_24CH_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const DL_24CH_SEL_FS_SFT: u32 = 8;
pub const DL_24CH_SEL_FS_MASK: u32 = 0x1f;
pub const DL_24CH_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const DL_24CH_BUF_EMPTY_CLR_SFT: u32 = 7;
pub const DL_24CH_BUF_EMPTY_CLR_MASK: u32 = 0x1;
pub const DL_24CH_BUF_EMPTY_CLR_MASK_SFT: u32 = (0x1 << 7);
pub const DL_24CH_PBUF_SIZE_SFT: u32 = 5;
pub const DL_24CH_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL_24CH_PBUF_SIZE_MASK_SFT: u32 = (0x3 << 5);
pub const DL_24CH_HANG_CLR_SFT: u32 = 4;
pub const DL_24CH_HANG_CLR_MASK: u32 = 0x1;
pub const DL_24CH_HANG_CLR_MASK_SFT: u32 = (0x1 << 4);
pub const DL_24CH_NORMAL_MODE_SFT: u32 = 3;
pub const DL_24CH_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL_24CH_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const DL_24CH_HALIGN_SFT: u32 = 2;
pub const DL_24CH_HALIGN_MASK: u32 = 0x1;
pub const DL_24CH_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const DL_24CH_HD_MODE_SFT: u32 = 0;
pub const DL_24CH_HD_MODE_MASK: u32 = 0x3;
pub const DL_24CH_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_DL23_BASE_MSB */
pub const DL23_BASE__ADDR_MSB_SFT: u32 = 0;
pub const DL23_BASE__ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL23_BASE__ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL23_BASE */
pub const DL23_BASE_ADDR_SFT: u32 = 4;
pub const DL23_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL23_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL23_CUR_MSB */
pub const DL23_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL23_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL23_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL23_CUR */
pub const DL23_CUR_PTR_SFT: u32 = 0;
pub const DL23_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL23_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL23_END_MSB */
pub const DL23_END_ADDR_MSB_SFT: u32 = 0;
pub const DL23_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL23_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL23_END */
pub const DL23_END_ADDR_SFT: u32 = 4;
pub const DL23_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL23_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL23_RCH_MON */
pub const DL23_RCH_DATA_SFT: u32 = 0;
pub const DL23_RCH_DATA_MASK: u32 = 0xffffffff;
pub const DL23_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL23_LCH_MON */
pub const DL23_LCH_DATA_SFT: u32 = 0;
pub const DL23_LCH_DATA_MASK: u32 = 0xffffffff;
pub const DL23_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL23_CON0 */
pub const DL23_ON_SFT: u32 = 28;
pub const DL23_ON_MASK: u32 = 0x1;
pub const DL23_ON_MASK_SFT: u32 = (0x1 << 28);
pub const DL23_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL23_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL23_ONE_HEART_SEL_MASK_SFT: u32 = (0x3 << 22);
pub const DL23_MINLEN_SFT: u32 = 20;
pub const DL23_MINLEN_MASK: u32 = 0x3;
pub const DL23_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const DL23_MAXLEN_SFT: u32 = 16;
pub const DL23_MAXLEN_MASK: u32 = 0x3;
pub const DL23_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const DL23_SEL_DOMAIN_SFT: u32 = 13;
pub const DL23_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL23_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const DL23_SEL_FS_SFT: u32 = 8;
pub const DL23_SEL_FS_MASK: u32 = 0x1f;
pub const DL23_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const DL23_SW_CLEAR_BUF_EMPTY_SFT: u32 = 7;
pub const DL23_SW_CLEAR_BUF_EMPTY_MASK: u32 = 0x1;
pub const DL23_SW_CLEAR_BUF_EMPTY_MASK_SFT: u32 = (0x1 << 7);
pub const DL23_PBUF_SIZE_SFT: u32 = 5;
pub const DL23_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL23_PBUF_SIZE_MASK_SFT: u32 = (0x3 << 5);
pub const DL23_MONO_SFT: u32 = 4;
pub const DL23_MONO_MASK: u32 = 0x1;
pub const DL23_MONO_MASK_SFT: u32 = (0x1 << 4);
pub const DL23_NORMAL_MODE_SFT: u32 = 3;
pub const DL23_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL23_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const DL23_HALIGN_SFT: u32 = 2;
pub const DL23_HALIGN_MASK: u32 = 0x1;
pub const DL23_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const DL23_HD_MODE_SFT: u32 = 0;
pub const DL23_HD_MODE_MASK: u32 = 0x3;
pub const DL23_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_DL24_BASE_MSB */
pub const DL24_BASE__ADDR_MSB_SFT: u32 = 0;
pub const DL24_BASE__ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL24_BASE__ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL24_BASE */
pub const DL24_BASE_ADDR_SFT: u32 = 4;
pub const DL24_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL24_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL24_CUR_MSB */
pub const DL24_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL24_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL24_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL24_CUR */
pub const DL24_CUR_PTR_SFT: u32 = 0;
pub const DL24_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL24_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL24_END_MSB */
pub const DL24_END_ADDR_MSB_SFT: u32 = 0;
pub const DL24_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL24_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL24_END */
pub const DL24_END_ADDR_SFT: u32 = 4;
pub const DL24_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL24_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL24_RCH_MON */
pub const DL24_RCH_DATA_SFT: u32 = 0;
pub const DL24_RCH_DATA_MASK: u32 = 0xffffffff;
pub const DL24_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL24_LCH_MON */
pub const DL24_LCH_DATA_SFT: u32 = 0;
pub const DL24_LCH_DATA_MASK: u32 = 0xffffffff;
pub const DL24_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL24_CON0 */
pub const DL24_ON_SFT: u32 = 28;
pub const DL24_ON_MASK: u32 = 0x1;
pub const DL24_ON_MASK_SFT: u32 = (0x1 << 28);
pub const DL24_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL24_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL24_ONE_HEART_SEL_MASK_SFT: u32 = (0x3 << 22);
pub const DL24_MINLEN_SFT: u32 = 20;
pub const DL24_MINLEN_MASK: u32 = 0x3;
pub const DL24_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const DL24_MAXLEN_SFT: u32 = 16;
pub const DL24_MAXLEN_MASK: u32 = 0x3;
pub const DL24_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const DL24_SEL_DOMAIN_SFT: u32 = 13;
pub const DL24_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL24_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const DL24_SEL_FS_SFT: u32 = 8;
pub const DL24_SEL_FS_MASK: u32 = 0x1f;
pub const DL24_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const DL24_SW_CLEAR_BUF_EMPTY_SFT: u32 = 7;
pub const DL24_SW_CLEAR_BUF_EMPTY_MASK: u32 = 0x1;
pub const DL24_SW_CLEAR_BUF_EMPTY_MASK_SFT: u32 = (0x1 << 7);
pub const DL24_PBUF_SIZE_SFT: u32 = 5;
pub const DL24_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL24_PBUF_SIZE_MASK_SFT: u32 = (0x3 << 5);
pub const DL24_MONO_SFT: u32 = 4;
pub const DL24_MONO_MASK: u32 = 0x1;
pub const DL24_MONO_MASK_SFT: u32 = (0x1 << 4);
pub const DL24_NORMAL_MODE_SFT: u32 = 3;
pub const DL24_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL24_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const DL24_HALIGN_SFT: u32 = 2;
pub const DL24_HALIGN_MASK: u32 = 0x1;
pub const DL24_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const DL24_HD_MODE_SFT: u32 = 0;
pub const DL24_HD_MODE_MASK: u32 = 0x3;
pub const DL24_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_DL25_BASE_MSB */
pub const DL25_BASE__ADDR_MSB_SFT: u32 = 0;
pub const DL25_BASE__ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL25_BASE__ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL25_BASE */
pub const DL25_BASE_ADDR_SFT: u32 = 4;
pub const DL25_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL25_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL25_CUR_MSB */
pub const DL25_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL25_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL25_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL25_CUR */
pub const DL25_CUR_PTR_SFT: u32 = 0;
pub const DL25_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL25_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL25_END_MSB */
pub const DL25_END_ADDR_MSB_SFT: u32 = 0;
pub const DL25_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL25_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL25_END */
pub const DL25_END_ADDR_SFT: u32 = 4;
pub const DL25_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL25_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL25_RCH_MON */
pub const DL25_RCH_DATA_SFT: u32 = 0;
pub const DL25_RCH_DATA_MASK: u32 = 0xffffffff;
pub const DL25_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL25_LCH_MON */
pub const DL25_LCH_DATA_SFT: u32 = 0;
pub const DL25_LCH_DATA_MASK: u32 = 0xffffffff;
pub const DL25_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL25_CON0 */
pub const DL25_ON_SFT: u32 = 28;
pub const DL25_ON_MASK: u32 = 0x1;
pub const DL25_ON_MASK_SFT: u32 = (0x1 << 28);
pub const DL25_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL25_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL25_ONE_HEART_SEL_MASK_SFT: u32 = (0x3 << 22);
pub const DL25_MINLEN_SFT: u32 = 20;
pub const DL25_MINLEN_MASK: u32 = 0x3;
pub const DL25_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const DL25_MAXLEN_SFT: u32 = 16;
pub const DL25_MAXLEN_MASK: u32 = 0x3;
pub const DL25_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const DL25_SEL_DOMAIN_SFT: u32 = 13;
pub const DL25_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL25_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const DL25_SEL_FS_SFT: u32 = 8;
pub const DL25_SEL_FS_MASK: u32 = 0x1f;
pub const DL25_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const DL25_SW_CLEAR_BUF_EMPTY_SFT: u32 = 7;
pub const DL25_SW_CLEAR_BUF_EMPTY_MASK: u32 = 0x1;
pub const DL25_SW_CLEAR_BUF_EMPTY_MASK_SFT: u32 = (0x1 << 7);
pub const DL25_PBUF_SIZE_SFT: u32 = 5;
pub const DL25_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL25_PBUF_SIZE_MASK_SFT: u32 = (0x3 << 5);
pub const DL25_MONO_SFT: u32 = 4;
pub const DL25_MONO_MASK: u32 = 0x1;
pub const DL25_MONO_MASK_SFT: u32 = (0x1 << 4);
pub const DL25_NORMAL_MODE_SFT: u32 = 3;
pub const DL25_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL25_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const DL25_HALIGN_SFT: u32 = 2;
pub const DL25_HALIGN_MASK: u32 = 0x1;
pub const DL25_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const DL25_HD_MODE_SFT: u32 = 0;
pub const DL25_HD_MODE_MASK: u32 = 0x3;
pub const DL25_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_DL26_BASE_MSB */
pub const DL26_BASE__ADDR_MSB_SFT: u32 = 0;
pub const DL26_BASE__ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL26_BASE__ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL26_BASE */
pub const DL26_BASE_ADDR_SFT: u32 = 4;
pub const DL26_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL26_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL26_CUR_MSB */
pub const DL26_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL26_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL26_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL26_CUR */
pub const DL26_CUR_PTR_SFT: u32 = 0;
pub const DL26_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL26_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL26_END_MSB */
pub const DL26_END_ADDR_MSB_SFT: u32 = 0;
pub const DL26_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL26_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_DL26_END */
pub const DL26_END_ADDR_SFT: u32 = 4;
pub const DL26_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL26_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_DL26_RCH_MON */
pub const DL26_RCH_DATA_SFT: u32 = 0;
pub const DL26_RCH_DATA_MASK: u32 = 0xffffffff;
pub const DL26_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL26_LCH_MON */
pub const DL26_LCH_DATA_SFT: u32 = 0;
pub const DL26_LCH_DATA_MASK: u32 = 0xffffffff;
pub const DL26_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL26_CON0 */
pub const DL26_ON_SFT: u32 = 28;
pub const DL26_ON_MASK: u32 = 0x1;
pub const DL26_ON_MASK_SFT: u32 = (0x1 << 28);
pub const DL26_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL26_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL26_ONE_HEART_SEL_MASK_SFT: u32 = (0x3 << 22);
pub const DL26_MINLEN_SFT: u32 = 20;
pub const DL26_MINLEN_MASK: u32 = 0x3;
pub const DL26_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const DL26_MAXLEN_SFT: u32 = 16;
pub const DL26_MAXLEN_MASK: u32 = 0x3;
pub const DL26_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const DL26_SEL_DOMAIN_SFT: u32 = 13;
pub const DL26_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL26_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const DL26_SEL_FS_SFT: u32 = 8;
pub const DL26_SEL_FS_MASK: u32 = 0x1f;
pub const DL26_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const DL26_SW_CLEAR_BUF_EMPTY_SFT: u32 = 7;
pub const DL26_SW_CLEAR_BUF_EMPTY_MASK: u32 = 0x1;
pub const DL26_SW_CLEAR_BUF_EMPTY_MASK_SFT: u32 = (0x1 << 7);
pub const DL26_PBUF_SIZE_SFT: u32 = 5;
pub const DL26_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL26_PBUF_SIZE_MASK_SFT: u32 = (0x3 << 5);
pub const DL26_MONO_SFT: u32 = 4;
pub const DL26_MONO_MASK: u32 = 0x1;
pub const DL26_MONO_MASK_SFT: u32 = (0x1 << 4);
pub const DL26_NORMAL_MODE_SFT: u32 = 3;
pub const DL26_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL26_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const DL26_HALIGN_SFT: u32 = 2;
pub const DL26_HALIGN_MASK: u32 = 0x1;
pub const DL26_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const DL26_HD_MODE_SFT: u32 = 0;
pub const DL26_HD_MODE_MASK: u32 = 0x3;
pub const DL26_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_VUL0_BASE_MSB */
pub const VUL0_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL0_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL0_BASE_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL0_BASE */
pub const VUL0_BASE_ADDR_SFT: u32 = 4;
pub const VUL0_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL0_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL0_CUR_MSB */
pub const VUL0_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL0_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL0_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL0_CUR */
pub const VUL0_CUR_PTR_SFT: u32 = 0;
pub const VUL0_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL0_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL0_END_MSB */
pub const VUL0_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL0_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL0_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL0_END */
pub const VUL0_END_ADDR_SFT: u32 = 4;
pub const VUL0_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL0_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL0_RCH_MON */
pub const VUL0_RCH_DATA_SFT: u32 = 0;
pub const VUL0_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL0_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL0_LCH_MON */
pub const VUL0_LCH_DATA_SFT: u32 = 0;
pub const VUL0_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL0_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL0_CON0 */
pub const VUL0_ON_SFT: u32 = 28;
pub const VUL0_ON_MASK: u32 = 0x1;
pub const VUL0_ON_MASK_SFT: u32 = (0x1 << 28);
pub const VUL0_MINLEN_SFT: u32 = 20;
pub const VUL0_MINLEN_MASK: u32 = 0x3;
pub const VUL0_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const VUL0_MAXLEN_SFT: u32 = 16;
pub const VUL0_MAXLEN_MASK: u32 = 0x3;
pub const VUL0_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const VUL0_SEL_DOMAIN_SFT: u32 = 13;
pub const VUL0_SEL_DOMAIN_MASK: u32 = 0x7;
pub const VUL0_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const VUL0_SEL_FS_SFT: u32 = 8;
pub const VUL0_SEL_FS_MASK: u32 = 0x1f;
pub const VUL0_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const VUL0_SW_CLEAR_BUF_FULL_SFT: u32 = 7;
pub const VUL0_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL0_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = (0x1 << 7);
pub const VUL0_WR_SIGN_SFT: u32 = 6;
pub const VUL0_WR_SIGN_MASK: u32 = 0x1;
pub const VUL0_WR_SIGN_MASK_SFT: u32 = (0x1 << 6);
pub const VUL0_R_MONO_SFT: u32 = 5;
pub const VUL0_R_MONO_MASK: u32 = 0x1;
pub const VUL0_R_MONO_MASK_SFT: u32 = (0x1 << 5);
pub const VUL0_MONO_SFT: u32 = 4;
pub const VUL0_MONO_MASK: u32 = 0x1;
pub const VUL0_MONO_MASK_SFT: u32 = (0x1 << 4);
pub const VUL0_NORMAL_MODE_SFT: u32 = 3;
pub const VUL0_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL0_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const VUL0_HALIGN_SFT: u32 = 2;
pub const VUL0_HALIGN_MASK: u32 = 0x1;
pub const VUL0_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const VUL0_HD_MODE_SFT: u32 = 0;
pub const VUL0_HD_MODE_MASK: u32 = 0x3;
pub const VUL0_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_VUL1_BASE_MSB */
pub const VUL1_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL1_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL1_BASE_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL1_BASE */
pub const VUL1_BASE_ADDR_SFT: u32 = 4;
pub const VUL1_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL1_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL1_CUR_MSB */
pub const VUL1_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL1_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL1_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL1_CUR */
pub const VUL1_CUR_PTR_SFT: u32 = 0;
pub const VUL1_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL1_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL1_END_MSB */
pub const VUL1_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL1_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL1_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL1_END */
pub const VUL1_END_ADDR_SFT: u32 = 4;
pub const VUL1_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL1_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL1_RCH_MON */
pub const VUL1_RCH_DATA_SFT: u32 = 0;
pub const VUL1_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL1_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL1_LCH_MON */
pub const VUL1_LCH_DATA_SFT: u32 = 0;
pub const VUL1_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL1_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL1_CON0 */
pub const VUL1_ON_SFT: u32 = 28;
pub const VUL1_ON_MASK: u32 = 0x1;
pub const VUL1_ON_MASK_SFT: u32 = (0x1 << 28);
pub const VUL1_MINLEN_SFT: u32 = 20;
pub const VUL1_MINLEN_MASK: u32 = 0x3;
pub const VUL1_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const VUL1_MAXLEN_SFT: u32 = 16;
pub const VUL1_MAXLEN_MASK: u32 = 0x3;
pub const VUL1_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const VUL1_SEL_DOMAIN_SFT: u32 = 13;
pub const VUL1_SEL_DOMAIN_MASK: u32 = 0x7;
pub const VUL1_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const VUL1_SEL_FS_SFT: u32 = 8;
pub const VUL1_SEL_FS_MASK: u32 = 0x1f;
pub const VUL1_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const VUL1_SW_CLEAR_BUF_FULL_SFT: u32 = 7;
pub const VUL1_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL1_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = (0x1 << 7);
pub const VUL1_WR_SIGN_SFT: u32 = 6;
pub const VUL1_WR_SIGN_MASK: u32 = 0x1;
pub const VUL1_WR_SIGN_MASK_SFT: u32 = (0x1 << 6);
pub const VUL1_R_MONO_SFT: u32 = 5;
pub const VUL1_R_MONO_MASK: u32 = 0x1;
pub const VUL1_R_MONO_MASK_SFT: u32 = (0x1 << 5);
pub const VUL1_MONO_SFT: u32 = 4;
pub const VUL1_MONO_MASK: u32 = 0x1;
pub const VUL1_MONO_MASK_SFT: u32 = (0x1 << 4);
pub const VUL1_NORMAL_MODE_SFT: u32 = 3;
pub const VUL1_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL1_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const VUL1_HALIGN_SFT: u32 = 2;
pub const VUL1_HALIGN_MASK: u32 = 0x1;
pub const VUL1_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const VUL1_HD_MODE_SFT: u32 = 0;
pub const VUL1_HD_MODE_MASK: u32 = 0x3;
pub const VUL1_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_VUL2_BASE_MSB */
pub const VUL2_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL2_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL2_BASE_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL2_BASE */
pub const VUL2_BASE_ADDR_SFT: u32 = 4;
pub const VUL2_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL2_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL2_CUR_MSB */
pub const VUL2_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL2_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL2_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL2_CUR */
pub const VUL2_CUR_PTR_SFT: u32 = 0;
pub const VUL2_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL2_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL2_END_MSB */
pub const VUL2_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL2_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL2_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL2_END */
pub const VUL2_END_ADDR_SFT: u32 = 4;
pub const VUL2_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL2_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL2_RCH_MON */
pub const VUL2_RCH_DATA_SFT: u32 = 0;
pub const VUL2_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL2_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL2_LCH_MON */
pub const VUL2_LCH_DATA_SFT: u32 = 0;
pub const VUL2_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL2_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL2_CON0 */
pub const VUL2_ON_SFT: u32 = 28;
pub const VUL2_ON_MASK: u32 = 0x1;
pub const VUL2_ON_MASK_SFT: u32 = (0x1 << 28);
pub const VUL2_MINLEN_SFT: u32 = 20;
pub const VUL2_MINLEN_MASK: u32 = 0x3;
pub const VUL2_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const VUL2_MAXLEN_SFT: u32 = 16;
pub const VUL2_MAXLEN_MASK: u32 = 0x3;
pub const VUL2_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const VUL2_SEL_DOMAIN_SFT: u32 = 13;
pub const VUL2_SEL_DOMAIN_MASK: u32 = 0x7;
pub const VUL2_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const VUL2_SEL_FS_SFT: u32 = 8;
pub const VUL2_SEL_FS_MASK: u32 = 0x1f;
pub const VUL2_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const VUL2_SW_CLEAR_BUF_FULL_SFT: u32 = 7;
pub const VUL2_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL2_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = (0x1 << 7);
pub const VUL2_WR_SIGN_SFT: u32 = 6;
pub const VUL2_WR_SIGN_MASK: u32 = 0x1;
pub const VUL2_WR_SIGN_MASK_SFT: u32 = (0x1 << 6);
pub const VUL2_R_MONO_SFT: u32 = 5;
pub const VUL2_R_MONO_MASK: u32 = 0x1;
pub const VUL2_R_MONO_MASK_SFT: u32 = (0x1 << 5);
pub const VUL2_MONO_SFT: u32 = 4;
pub const VUL2_MONO_MASK: u32 = 0x1;
pub const VUL2_MONO_MASK_SFT: u32 = (0x1 << 4);
pub const VUL2_NORMAL_MODE_SFT: u32 = 3;
pub const VUL2_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL2_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const VUL2_HALIGN_SFT: u32 = 2;
pub const VUL2_HALIGN_MASK: u32 = 0x1;
pub const VUL2_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const VUL2_HD_MODE_SFT: u32 = 0;
pub const VUL2_HD_MODE_MASK: u32 = 0x3;
pub const VUL2_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_VUL3_BASE_MSB */
pub const VUL3_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL3_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL3_BASE_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL3_BASE */
pub const VUL3_BASE_ADDR_SFT: u32 = 4;
pub const VUL3_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL3_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL3_CUR_MSB */
pub const VUL3_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL3_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL3_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL3_CUR */
pub const VUL3_CUR_PTR_SFT: u32 = 0;
pub const VUL3_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL3_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL3_END_MSB */
pub const VUL3_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL3_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL3_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL3_END */
pub const VUL3_END_ADDR_SFT: u32 = 4;
pub const VUL3_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL3_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL3_RCH_MON */
pub const VUL3_RCH_DATA_SFT: u32 = 0;
pub const VUL3_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL3_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL3_LCH_MON */
pub const VUL3_LCH_DATA_SFT: u32 = 0;
pub const VUL3_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL3_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL3_CON0 */
pub const VUL3_ON_SFT: u32 = 28;
pub const VUL3_ON_MASK: u32 = 0x1;
pub const VUL3_ON_MASK_SFT: u32 = (0x1 << 28);
pub const VUL3_MINLEN_SFT: u32 = 20;
pub const VUL3_MINLEN_MASK: u32 = 0x3;
pub const VUL3_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const VUL3_MAXLEN_SFT: u32 = 16;
pub const VUL3_MAXLEN_MASK: u32 = 0x3;
pub const VUL3_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const VUL3_SEL_DOMAIN_SFT: u32 = 13;
pub const VUL3_SEL_DOMAIN_MASK: u32 = 0x7;
pub const VUL3_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const VUL3_SEL_FS_SFT: u32 = 8;
pub const VUL3_SEL_FS_MASK: u32 = 0x1f;
pub const VUL3_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const VUL3_SW_CLEAR_BUF_FULL_SFT: u32 = 7;
pub const VUL3_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL3_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = (0x1 << 7);
pub const VUL3_WR_SIGN_SFT: u32 = 6;
pub const VUL3_WR_SIGN_MASK: u32 = 0x1;
pub const VUL3_WR_SIGN_MASK_SFT: u32 = (0x1 << 6);
pub const VUL3_R_MONO_SFT: u32 = 5;
pub const VUL3_R_MONO_MASK: u32 = 0x1;
pub const VUL3_R_MONO_MASK_SFT: u32 = (0x1 << 5);
pub const VUL3_MONO_SFT: u32 = 4;
pub const VUL3_MONO_MASK: u32 = 0x1;
pub const VUL3_MONO_MASK_SFT: u32 = (0x1 << 4);
pub const VUL3_NORMAL_MODE_SFT: u32 = 3;
pub const VUL3_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL3_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const VUL3_HALIGN_SFT: u32 = 2;
pub const VUL3_HALIGN_MASK: u32 = 0x1;
pub const VUL3_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const VUL3_HD_MODE_SFT: u32 = 0;
pub const VUL3_HD_MODE_MASK: u32 = 0x3;
pub const VUL3_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_VUL4_BASE_MSB */
pub const VUL4_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL4_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL4_BASE_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL4_BASE */
pub const VUL4_BASE_ADDR_SFT: u32 = 4;
pub const VUL4_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL4_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL4_CUR_MSB */
pub const VUL4_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL4_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL4_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL4_CUR */
pub const VUL4_CUR_PTR_SFT: u32 = 0;
pub const VUL4_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL4_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL4_END_MSB */
pub const VUL4_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL4_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL4_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL4_END */
pub const VUL4_END_ADDR_SFT: u32 = 4;
pub const VUL4_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL4_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL4_RCH_MON */
pub const VUL4_RCH_DATA_SFT: u32 = 0;
pub const VUL4_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL4_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL4_LCH_MON */
pub const VUL4_LCH_DATA_SFT: u32 = 0;
pub const VUL4_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL4_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL4_CON0 */
pub const VUL4_ON_SFT: u32 = 28;
pub const VUL4_ON_MASK: u32 = 0x1;
pub const VUL4_ON_MASK_SFT: u32 = (0x1 << 28);
pub const VUL4_MINLEN_SFT: u32 = 20;
pub const VUL4_MINLEN_MASK: u32 = 0x3;
pub const VUL4_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const VUL4_MAXLEN_SFT: u32 = 16;
pub const VUL4_MAXLEN_MASK: u32 = 0x3;
pub const VUL4_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const VUL4_SEL_DOMAIN_SFT: u32 = 13;
pub const VUL4_SEL_DOMAIN_MASK: u32 = 0x7;
pub const VUL4_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const VUL4_SEL_FS_SFT: u32 = 8;
pub const VUL4_SEL_FS_MASK: u32 = 0x1f;
pub const VUL4_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const VUL4_SW_CLEAR_BUF_FULL_SFT: u32 = 7;
pub const VUL4_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL4_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = (0x1 << 7);
pub const VUL4_WR_SIGN_SFT: u32 = 6;
pub const VUL4_WR_SIGN_MASK: u32 = 0x1;
pub const VUL4_WR_SIGN_MASK_SFT: u32 = (0x1 << 6);
pub const VUL4_R_MONO_SFT: u32 = 5;
pub const VUL4_R_MONO_MASK: u32 = 0x1;
pub const VUL4_R_MONO_MASK_SFT: u32 = (0x1 << 5);
pub const VUL4_MONO_SFT: u32 = 4;
pub const VUL4_MONO_MASK: u32 = 0x1;
pub const VUL4_MONO_MASK_SFT: u32 = (0x1 << 4);
pub const VUL4_NORMAL_MODE_SFT: u32 = 3;
pub const VUL4_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL4_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const VUL4_HALIGN_SFT: u32 = 2;
pub const VUL4_HALIGN_MASK: u32 = 0x1;
pub const VUL4_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const VUL4_HD_MODE_SFT: u32 = 0;
pub const VUL4_HD_MODE_MASK: u32 = 0x3;
pub const VUL4_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_VUL5_BASE_MSB */
pub const VUL5_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL5_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL5_BASE_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL5_BASE */
pub const VUL5_BASE_ADDR_SFT: u32 = 4;
pub const VUL5_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL5_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL5_CUR_MSB */
pub const VUL5_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL5_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL5_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL5_CUR */
pub const VUL5_CUR_PTR_SFT: u32 = 0;
pub const VUL5_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL5_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL5_END_MSB */
pub const VUL5_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL5_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL5_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL5_END */
pub const VUL5_END_ADDR_SFT: u32 = 4;
pub const VUL5_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL5_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL5_RCH_MON */
pub const VUL5_RCH_DATA_SFT: u32 = 0;
pub const VUL5_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL5_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL5_LCH_MON */
pub const VUL5_LCH_DATA_SFT: u32 = 0;
pub const VUL5_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL5_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL5_CON0 */
pub const VUL5_ON_SFT: u32 = 28;
pub const VUL5_ON_MASK: u32 = 0x1;
pub const VUL5_ON_MASK_SFT: u32 = (0x1 << 28);
pub const VUL5_MINLEN_SFT: u32 = 20;
pub const VUL5_MINLEN_MASK: u32 = 0x3;
pub const VUL5_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const VUL5_MAXLEN_SFT: u32 = 16;
pub const VUL5_MAXLEN_MASK: u32 = 0x3;
pub const VUL5_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const VUL5_SEL_DOMAIN_SFT: u32 = 13;
pub const VUL5_SEL_DOMAIN_MASK: u32 = 0x7;
pub const VUL5_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const VUL5_SEL_FS_SFT: u32 = 8;
pub const VUL5_SEL_FS_MASK: u32 = 0x1f;
pub const VUL5_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const VUL5_SW_CLEAR_BUF_FULL_SFT: u32 = 7;
pub const VUL5_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL5_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = (0x1 << 7);
pub const VUL5_WR_SIGN_SFT: u32 = 6;
pub const VUL5_WR_SIGN_MASK: u32 = 0x1;
pub const VUL5_WR_SIGN_MASK_SFT: u32 = (0x1 << 6);
pub const VUL5_R_MONO_SFT: u32 = 5;
pub const VUL5_R_MONO_MASK: u32 = 0x1;
pub const VUL5_R_MONO_MASK_SFT: u32 = (0x1 << 5);
pub const VUL5_MONO_SFT: u32 = 4;
pub const VUL5_MONO_MASK: u32 = 0x1;
pub const VUL5_MONO_MASK_SFT: u32 = (0x1 << 4);
pub const VUL5_NORMAL_MODE_SFT: u32 = 3;
pub const VUL5_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL5_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const VUL5_HALIGN_SFT: u32 = 2;
pub const VUL5_HALIGN_MASK: u32 = 0x1;
pub const VUL5_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const VUL5_HD_MODE_SFT: u32 = 0;
pub const VUL5_HD_MODE_MASK: u32 = 0x3;
pub const VUL5_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_VUL6_BASE_MSB */
pub const VUL6_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL6_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL6_BASE_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL6_BASE */
pub const VUL6_BASE_ADDR_SFT: u32 = 4;
pub const VUL6_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL6_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL6_CUR_MSB */
pub const VUL6_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL6_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL6_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL6_CUR */
pub const VUL6_CUR_PTR_SFT: u32 = 0;
pub const VUL6_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL6_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL6_END_MSB */
pub const VUL6_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL6_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL6_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL6_END */
pub const VUL6_END_ADDR_SFT: u32 = 4;
pub const VUL6_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL6_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL6_RCH_MON */
pub const VUL6_RCH_DATA_SFT: u32 = 0;
pub const VUL6_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL6_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL6_LCH_MON */
pub const VUL6_LCH_DATA_SFT: u32 = 0;
pub const VUL6_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL6_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL6_CON0 */
pub const VUL6_ON_SFT: u32 = 28;
pub const VUL6_ON_MASK: u32 = 0x1;
pub const VUL6_ON_MASK_SFT: u32 = (0x1 << 28);
pub const VUL6_MINLEN_SFT: u32 = 20;
pub const VUL6_MINLEN_MASK: u32 = 0x3;
pub const VUL6_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const VUL6_MAXLEN_SFT: u32 = 16;
pub const VUL6_MAXLEN_MASK: u32 = 0x3;
pub const VUL6_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const VUL6_SEL_DOMAIN_SFT: u32 = 13;
pub const VUL6_SEL_DOMAIN_MASK: u32 = 0x7;
pub const VUL6_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const VUL6_SEL_FS_SFT: u32 = 8;
pub const VUL6_SEL_FS_MASK: u32 = 0x1f;
pub const VUL6_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const VUL6_SW_CLEAR_BUF_FULL_SFT: u32 = 7;
pub const VUL6_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL6_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = (0x1 << 7);
pub const VUL6_WR_SIGN_SFT: u32 = 6;
pub const VUL6_WR_SIGN_MASK: u32 = 0x1;
pub const VUL6_WR_SIGN_MASK_SFT: u32 = (0x1 << 6);
pub const VUL6_R_MONO_SFT: u32 = 5;
pub const VUL6_R_MONO_MASK: u32 = 0x1;
pub const VUL6_R_MONO_MASK_SFT: u32 = (0x1 << 5);
pub const VUL6_MONO_SFT: u32 = 4;
pub const VUL6_MONO_MASK: u32 = 0x1;
pub const VUL6_MONO_MASK_SFT: u32 = (0x1 << 4);
pub const VUL6_NORMAL_MODE_SFT: u32 = 3;
pub const VUL6_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL6_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const VUL6_HALIGN_SFT: u32 = 2;
pub const VUL6_HALIGN_MASK: u32 = 0x1;
pub const VUL6_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const VUL6_HD_MODE_SFT: u32 = 0;
pub const VUL6_HD_MODE_MASK: u32 = 0x3;
pub const VUL6_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_VUL7_BASE_MSB */
pub const VUL7_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL7_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL7_BASE_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL7_BASE */
pub const VUL7_BASE_ADDR_SFT: u32 = 4;
pub const VUL7_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL7_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL7_CUR_MSB */
pub const VUL7_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL7_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL7_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL7_CUR */
pub const VUL7_CUR_PTR_SFT: u32 = 0;
pub const VUL7_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL7_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL7_END_MSB */
pub const VUL7_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL7_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL7_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL7_END */
pub const VUL7_END_ADDR_SFT: u32 = 4;
pub const VUL7_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL7_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL7_RCH_MON */
pub const VUL7_RCH_DATA_SFT: u32 = 0;
pub const VUL7_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL7_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL7_LCH_MON */
pub const VUL7_LCH_DATA_SFT: u32 = 0;
pub const VUL7_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL7_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL7_CON0 */
pub const VUL7_ON_SFT: u32 = 28;
pub const VUL7_ON_MASK: u32 = 0x1;
pub const VUL7_ON_MASK_SFT: u32 = (0x1 << 28);
pub const VUL7_MINLEN_SFT: u32 = 20;
pub const VUL7_MINLEN_MASK: u32 = 0x3;
pub const VUL7_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const VUL7_MAXLEN_SFT: u32 = 16;
pub const VUL7_MAXLEN_MASK: u32 = 0x3;
pub const VUL7_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const VUL7_SEL_DOMAIN_SFT: u32 = 13;
pub const VUL7_SEL_DOMAIN_MASK: u32 = 0x7;
pub const VUL7_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const VUL7_SEL_FS_SFT: u32 = 8;
pub const VUL7_SEL_FS_MASK: u32 = 0x1f;
pub const VUL7_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const VUL7_SW_CLEAR_BUF_FULL_SFT: u32 = 7;
pub const VUL7_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL7_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = (0x1 << 7);
pub const VUL7_WR_SIGN_SFT: u32 = 6;
pub const VUL7_WR_SIGN_MASK: u32 = 0x1;
pub const VUL7_WR_SIGN_MASK_SFT: u32 = (0x1 << 6);
pub const VUL7_R_MONO_SFT: u32 = 5;
pub const VUL7_R_MONO_MASK: u32 = 0x1;
pub const VUL7_R_MONO_MASK_SFT: u32 = (0x1 << 5);
pub const VUL7_MONO_SFT: u32 = 4;
pub const VUL7_MONO_MASK: u32 = 0x1;
pub const VUL7_MONO_MASK_SFT: u32 = (0x1 << 4);
pub const VUL7_NORMAL_MODE_SFT: u32 = 3;
pub const VUL7_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL7_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const VUL7_HALIGN_SFT: u32 = 2;
pub const VUL7_HALIGN_MASK: u32 = 0x1;
pub const VUL7_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const VUL7_HD_MODE_SFT: u32 = 0;
pub const VUL7_HD_MODE_MASK: u32 = 0x3;
pub const VUL7_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_VUL8_BASE_MSB */
pub const VUL8_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL8_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL8_BASE_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL8_BASE */
pub const VUL8_BASE_ADDR_SFT: u32 = 4;
pub const VUL8_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL8_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL8_CUR_MSB */
pub const VUL8_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL8_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL8_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL8_CUR */
pub const VUL8_CUR_PTR_SFT: u32 = 0;
pub const VUL8_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL8_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL8_END_MSB */
pub const VUL8_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL8_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL8_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL8_END */
pub const VUL8_END_ADDR_SFT: u32 = 4;
pub const VUL8_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL8_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL8_RCH_MON */
pub const VUL8_RCH_DATA_SFT: u32 = 0;
pub const VUL8_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL8_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL8_LCH_MON */
pub const VUL8_LCH_DATA_SFT: u32 = 0;
pub const VUL8_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL8_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL8_CON0 */
pub const VUL8_ON_SFT: u32 = 28;
pub const VUL8_ON_MASK: u32 = 0x1;
pub const VUL8_ON_MASK_SFT: u32 = (0x1 << 28);
pub const VUL8_MINLEN_SFT: u32 = 20;
pub const VUL8_MINLEN_MASK: u32 = 0x3;
pub const VUL8_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const VUL8_MAXLEN_SFT: u32 = 16;
pub const VUL8_MAXLEN_MASK: u32 = 0x3;
pub const VUL8_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const VUL8_SEL_DOMAIN_SFT: u32 = 13;
pub const VUL8_SEL_DOMAIN_MASK: u32 = 0x7;
pub const VUL8_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const VUL8_SEL_FS_SFT: u32 = 8;
pub const VUL8_SEL_FS_MASK: u32 = 0x1f;
pub const VUL8_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const VUL8_SW_CLEAR_BUF_FULL_SFT: u32 = 7;
pub const VUL8_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL8_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = (0x1 << 7);
pub const VUL8_WR_SIGN_SFT: u32 = 6;
pub const VUL8_WR_SIGN_MASK: u32 = 0x1;
pub const VUL8_WR_SIGN_MASK_SFT: u32 = (0x1 << 6);
pub const VUL8_R_MONO_SFT: u32 = 5;
pub const VUL8_R_MONO_MASK: u32 = 0x1;
pub const VUL8_R_MONO_MASK_SFT: u32 = (0x1 << 5);
pub const VUL8_MONO_SFT: u32 = 4;
pub const VUL8_MONO_MASK: u32 = 0x1;
pub const VUL8_MONO_MASK_SFT: u32 = (0x1 << 4);
pub const VUL8_NORMAL_MODE_SFT: u32 = 3;
pub const VUL8_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL8_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const VUL8_HALIGN_SFT: u32 = 2;
pub const VUL8_HALIGN_MASK: u32 = 0x1;
pub const VUL8_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const VUL8_HD_MODE_SFT: u32 = 0;
pub const VUL8_HD_MODE_MASK: u32 = 0x3;
pub const VUL8_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_VUL9_BASE_MSB */
pub const VUL9_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL9_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL9_BASE_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL9_BASE */
pub const VUL9_BASE_ADDR_SFT: u32 = 4;
pub const VUL9_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL9_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL9_CUR_MSB */
pub const VUL9_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL9_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL9_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL9_CUR */
pub const VUL9_CUR_PTR_SFT: u32 = 0;
pub const VUL9_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL9_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL9_END_MSB */
pub const VUL9_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL9_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL9_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL9_END */
pub const VUL9_END_ADDR_SFT: u32 = 4;
pub const VUL9_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL9_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL9_RCH_MON */
pub const VUL9_RCH_DATA_SFT: u32 = 0;
pub const VUL9_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL9_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL9_LCH_MON */
pub const VUL9_LCH_DATA_SFT: u32 = 0;
pub const VUL9_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL9_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL9_CON0 */
pub const VUL9_ON_SFT: u32 = 28;
pub const VUL9_ON_MASK: u32 = 0x1;
pub const VUL9_ON_MASK_SFT: u32 = (0x1 << 28);
pub const VUL9_MINLEN_SFT: u32 = 20;
pub const VUL9_MINLEN_MASK: u32 = 0x3;
pub const VUL9_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const VUL9_MAXLEN_SFT: u32 = 16;
pub const VUL9_MAXLEN_MASK: u32 = 0x3;
pub const VUL9_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const VUL9_SEL_DOMAIN_SFT: u32 = 13;
pub const VUL9_SEL_DOMAIN_MASK: u32 = 0x7;
pub const VUL9_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const VUL9_SEL_FS_SFT: u32 = 8;
pub const VUL9_SEL_FS_MASK: u32 = 0x1f;
pub const VUL9_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const VUL9_SW_CLEAR_BUF_FULL_SFT: u32 = 7;
pub const VUL9_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL9_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = (0x1 << 7);
pub const VUL9_WR_SIGN_SFT: u32 = 6;
pub const VUL9_WR_SIGN_MASK: u32 = 0x1;
pub const VUL9_WR_SIGN_MASK_SFT: u32 = (0x1 << 6);
pub const VUL9_R_MONO_SFT: u32 = 5;
pub const VUL9_R_MONO_MASK: u32 = 0x1;
pub const VUL9_R_MONO_MASK_SFT: u32 = (0x1 << 5);
pub const VUL9_MONO_SFT: u32 = 4;
pub const VUL9_MONO_MASK: u32 = 0x1;
pub const VUL9_MONO_MASK_SFT: u32 = (0x1 << 4);
pub const VUL9_NORMAL_MODE_SFT: u32 = 3;
pub const VUL9_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL9_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const VUL9_HALIGN_SFT: u32 = 2;
pub const VUL9_HALIGN_MASK: u32 = 0x1;
pub const VUL9_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const VUL9_HD_MODE_SFT: u32 = 0;
pub const VUL9_HD_MODE_MASK: u32 = 0x3;
pub const VUL9_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_VUL10_BASE_MSB */
pub const VUL10_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL10_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL10_BASE_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL10_BASE */
pub const VUL10_BASE_ADDR_SFT: u32 = 4;
pub const VUL10_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL10_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL10_CUR_MSB */
pub const VUL10_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL10_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL10_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL10_CUR */
pub const VUL10_CUR_PTR_SFT: u32 = 0;
pub const VUL10_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL10_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL10_END_MSB */
pub const VUL10_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL10_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL10_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL10_END */
pub const VUL10_END_ADDR_SFT: u32 = 4;
pub const VUL10_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL10_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL10_RCH_MON */
pub const VUL10_RCH_DATA_SFT: u32 = 0;
pub const VUL10_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL10_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL10_LCH_MON */
pub const VUL10_LCH_DATA_SFT: u32 = 0;
pub const VUL10_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL10_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL10_CON0 */
pub const VUL10_ON_SFT: u32 = 28;
pub const VUL10_ON_MASK: u32 = 0x1;
pub const VUL10_ON_MASK_SFT: u32 = (0x1 << 28);
pub const VUL10_MINLEN_SFT: u32 = 20;
pub const VUL10_MINLEN_MASK: u32 = 0x3;
pub const VUL10_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const VUL10_MAXLEN_SFT: u32 = 16;
pub const VUL10_MAXLEN_MASK: u32 = 0x3;
pub const VUL10_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const VUL10_SEL_DOMAIN_SFT: u32 = 13;
pub const VUL10_SEL_DOMAIN_MASK: u32 = 0x7;
pub const VUL10_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const VUL10_SEL_FS_SFT: u32 = 8;
pub const VUL10_SEL_FS_MASK: u32 = 0x1f;
pub const VUL10_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const VUL10_SW_CLEAR_BUF_FULL_SFT: u32 = 7;
pub const VUL10_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL10_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = (0x1 << 7);
pub const VUL10_WR_SIGN_SFT: u32 = 6;
pub const VUL10_WR_SIGN_MASK: u32 = 0x1;
pub const VUL10_WR_SIGN_MASK_SFT: u32 = (0x1 << 6);
pub const VUL10_R_MONO_SFT: u32 = 5;
pub const VUL10_R_MONO_MASK: u32 = 0x1;
pub const VUL10_R_MONO_MASK_SFT: u32 = (0x1 << 5);
pub const VUL10_MONO_SFT: u32 = 4;
pub const VUL10_MONO_MASK: u32 = 0x1;
pub const VUL10_MONO_MASK_SFT: u32 = (0x1 << 4);
pub const VUL10_NORMAL_MODE_SFT: u32 = 3;
pub const VUL10_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL10_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const VUL10_HALIGN_SFT: u32 = 2;
pub const VUL10_HALIGN_MASK: u32 = 0x1;
pub const VUL10_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const VUL10_HD_MODE_SFT: u32 = 0;
pub const VUL10_HD_MODE_MASK: u32 = 0x3;
pub const VUL10_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_VUL24_BASE_MSB */
pub const VUL24_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL24_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL24_BASE_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL24_BASE */
pub const VUL24_BASE_ADDR_SFT: u32 = 4;
pub const VUL24_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL24_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL24_CUR_MSB */
pub const VUL24_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL24_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL24_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL24_CUR */
pub const VUL24_CUR_PTR_SFT: u32 = 0;
pub const VUL24_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL24_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL24_END_MSB */
pub const VUL24_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL24_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL24_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL24_END */
pub const VUL24_END_ADDR_SFT: u32 = 4;
pub const VUL24_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL24_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL24_CON0 */
pub const OUT_ON_USE_VUL24_SFT: u32 = 29;
pub const OUT_ON_USE_VUL24_MASK: u32 = 0x1;
pub const OUT_ON_USE_VUL24_MASK_SFT: u32 = (0x1 << 29);
pub const VUL24_ON_SFT: u32 = 28;
pub const VUL24_ON_MASK: u32 = 0x1;
pub const VUL24_ON_MASK_SFT: u32 = (0x1 << 28);
pub const VUL24_MINLEN_SFT: u32 = 20;
pub const VUL24_MINLEN_MASK: u32 = 0x3;
pub const VUL24_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const VUL24_MAXLEN_SFT: u32 = 16;
pub const VUL24_MAXLEN_MASK: u32 = 0x3;
pub const VUL24_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const VUL24_SEL_DOMAIN_SFT: u32 = 13;
pub const VUL24_SEL_DOMAIN_MASK: u32 = 0x7;
pub const VUL24_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const VUL24_SEL_FS_SFT: u32 = 8;
pub const VUL24_SEL_FS_MASK: u32 = 0x1f;
pub const VUL24_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const VUL24_SW_CLEAR_BUF_FULL_SFT: u32 = 7;
pub const VUL24_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL24_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = (0x1 << 7);
pub const VUL24_WR_SIGN_SFT: u32 = 6;
pub const VUL24_WR_SIGN_MASK: u32 = 0x1;
pub const VUL24_WR_SIGN_MASK_SFT: u32 = (0x1 << 6);
pub const VUL24_R_MONO_SFT: u32 = 5;
pub const VUL24_R_MONO_MASK: u32 = 0x1;
pub const VUL24_R_MONO_MASK_SFT: u32 = (0x1 << 5);
pub const VUL24_MONO_SFT: u32 = 4;
pub const VUL24_MONO_MASK: u32 = 0x1;
pub const VUL24_MONO_MASK_SFT: u32 = (0x1 << 4);
pub const VUL24_NORMAL_MODE_SFT: u32 = 3;
pub const VUL24_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL24_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const VUL24_HALIGN_SFT: u32 = 2;
pub const VUL24_HALIGN_MASK: u32 = 0x1;
pub const VUL24_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const VUL24_HD_MODE_SFT: u32 = 0;
pub const VUL24_HD_MODE_MASK: u32 = 0x3;
pub const VUL24_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_VUL25_BASE_MSB */
pub const VUL25_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL25_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL25_BASE_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL25_BASE */
pub const VUL25_BASE_ADDR_SFT: u32 = 4;
pub const VUL25_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL25_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL25_CUR_MSB */
pub const VUL25_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL25_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL25_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL25_CUR */
pub const VUL25_CUR_PTR_SFT: u32 = 0;
pub const VUL25_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL25_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL25_END_MSB */
pub const VUL25_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL25_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL25_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL25_END */
pub const VUL25_END_ADDR_SFT: u32 = 4;
pub const VUL25_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL25_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL25_CON0 */
pub const OUT_ON_USE_VUL25_SFT: u32 = 29;
pub const OUT_ON_USE_VUL25_MASK: u32 = 0x1;
pub const OUT_ON_USE_VUL25_MASK_SFT: u32 = (0x1 << 29);
pub const VUL25_ON_SFT: u32 = 28;
pub const VUL25_ON_MASK: u32 = 0x1;
pub const VUL25_ON_MASK_SFT: u32 = (0x1 << 28);
pub const VUL25_MINLEN_SFT: u32 = 20;
pub const VUL25_MINLEN_MASK: u32 = 0x3;
pub const VUL25_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const VUL25_MAXLEN_SFT: u32 = 16;
pub const VUL25_MAXLEN_MASK: u32 = 0x3;
pub const VUL25_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const VUL25_SEL_DOMAIN_SFT: u32 = 13;
pub const VUL25_SEL_DOMAIN_MASK: u32 = 0x7;
pub const VUL25_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const VUL25_SEL_FS_SFT: u32 = 8;
pub const VUL25_SEL_FS_MASK: u32 = 0x1f;
pub const VUL25_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const VUL25_SW_CLEAR_BUF_FULL_SFT: u32 = 7;
pub const VUL25_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL25_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = (0x1 << 7);
pub const VUL25_WR_SIGN_SFT: u32 = 6;
pub const VUL25_WR_SIGN_MASK: u32 = 0x1;
pub const VUL25_WR_SIGN_MASK_SFT: u32 = (0x1 << 6);
pub const VUL25_R_MONO_SFT: u32 = 5;
pub const VUL25_R_MONO_MASK: u32 = 0x1;
pub const VUL25_R_MONO_MASK_SFT: u32 = (0x1 << 5);
pub const VUL25_MONO_SFT: u32 = 4;
pub const VUL25_MONO_MASK: u32 = 0x1;
pub const VUL25_MONO_MASK_SFT: u32 = (0x1 << 4);
pub const VUL25_NORMAL_MODE_SFT: u32 = 3;
pub const VUL25_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL25_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const VUL25_HALIGN_SFT: u32 = 2;
pub const VUL25_HALIGN_MASK: u32 = 0x1;
pub const VUL25_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const VUL25_HD_MODE_SFT: u32 = 0;
pub const VUL25_HD_MODE_MASK: u32 = 0x3;
pub const VUL25_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_VUL26_BASE_MSB */
pub const VUL26_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL26_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL26_BASE_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL26_BASE */
pub const VUL26_BASE_ADDR_SFT: u32 = 4;
pub const VUL26_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL26_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL26_CUR_MSB */
pub const VUL26_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL26_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL26_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL26_CUR */
pub const VUL26_CUR_PTR_SFT: u32 = 0;
pub const VUL26_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL26_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL26_END_MSB */
pub const VUL26_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL26_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL26_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL26_END */
pub const VUL26_END_ADDR_SFT: u32 = 4;
pub const VUL26_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL26_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL26_CON0 */
pub const OUT_ON_USE_VUL26_SFT: u32 = 29;
pub const OUT_ON_USE_VUL26_MASK: u32 = 0x1;
pub const OUT_ON_USE_VUL26_MASK_SFT: u32 = (0x1 << 29);
pub const VUL26_ON_SFT: u32 = 28;
pub const VUL26_ON_MASK: u32 = 0x1;
pub const VUL26_ON_MASK_SFT: u32 = (0x1 << 28);
pub const VUL26_MINLEN_SFT: u32 = 20;
pub const VUL26_MINLEN_MASK: u32 = 0x3;
pub const VUL26_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const VUL26_MAXLEN_SFT: u32 = 16;
pub const VUL26_MAXLEN_MASK: u32 = 0x3;
pub const VUL26_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const VUL26_SEL_DOMAIN_SFT: u32 = 13;
pub const VUL26_SEL_DOMAIN_MASK: u32 = 0x7;
pub const VUL26_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 13);
pub const VUL26_SEL_FS_SFT: u32 = 8;
pub const VUL26_SEL_FS_MASK: u32 = 0x1f;
pub const VUL26_SEL_FS_MASK_SFT: u32 = (0x1f << 8);
pub const VUL26_SW_CLEAR_BUF_FULL_SFT: u32 = 7;
pub const VUL26_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL26_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = (0x1 << 7);
pub const VUL26_WR_SIGN_SFT: u32 = 6;
pub const VUL26_WR_SIGN_MASK: u32 = 0x1;
pub const VUL26_WR_SIGN_MASK_SFT: u32 = (0x1 << 6);
pub const VUL26_R_MONO_SFT: u32 = 5;
pub const VUL26_R_MONO_MASK: u32 = 0x1;
pub const VUL26_R_MONO_MASK_SFT: u32 = (0x1 << 5);
pub const VUL26_MONO_SFT: u32 = 4;
pub const VUL26_MONO_MASK: u32 = 0x1;
pub const VUL26_MONO_MASK_SFT: u32 = (0x1 << 4);
pub const VUL26_NORMAL_MODE_SFT: u32 = 3;
pub const VUL26_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL26_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 3);
pub const VUL26_HALIGN_SFT: u32 = 2;
pub const VUL26_HALIGN_MASK: u32 = 0x1;
pub const VUL26_HALIGN_MASK_SFT: u32 = (0x1 << 2);
pub const VUL26_HD_MODE_SFT: u32 = 0;
pub const VUL26_HD_MODE_MASK: u32 = 0x3;
pub const VUL26_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_VUL_CM0_BASE_MSB */
pub const VUL_CM0_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL_CM0_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL_CM0_BASE_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL_CM0_BASE */
pub const VUL_CM0_BASE_ADDR_SFT: u32 = 4;
pub const VUL_CM0_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL_CM0_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL_CM0_CUR_MSB */
pub const VUL_CM0_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL_CM0_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL_CM0_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL_CM0_CUR */
pub const VUL_CM0_CUR_PTR_SFT: u32 = 0;
pub const VUL_CM0_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL_CM0_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL_CM0_END_MSB */
pub const VUL_CM0_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL_CM0_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL_CM0_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL_CM0_END */
pub const VUL_CM0_END_ADDR_SFT: u32 = 4;
pub const VUL_CM0_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL_CM0_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL_CM0_CON0 */
pub const VUL_CM0_ON_SFT: u32 = 28;
pub const VUL_CM0_ON_MASK: u32 = 0x1;
pub const VUL_CM0_ON_MASK_SFT: u32 = (0x1 << 28);
pub const VUL_CM0_REG_CH_SHIFT_MODE_SFT: u32 = 26;
pub const VUL_CM0_REG_CH_SHIFT_MODE_MASK: u32 = 0x1;
pub const VUL_CM0_REG_CH_SHIFT_MODE_MASK_SFT: u32 = (0x1 << 26);
pub const VUL_CM0_RG_FORCE_NO_MASK_EXTRA_SFT: u32 = 25;
pub const VUL_CM0_RG_FORCE_NO_MASK_EXTRA_MASK: u32 = 0x1;
pub const VUL_CM0_RG_FORCE_NO_MASK_EXTRA_MASK_SFT: u32 = (0x1 << 25);
pub const VUL_CM0_SW_CLEAR_BUF_FULL_SFT: u32 = 24;
pub const VUL_CM0_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL_CM0_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = (0x1 << 24);
pub const VUL_CM0_ULTRA_TH_SFT: u32 = 20;
pub const VUL_CM0_ULTRA_TH_MASK: u32 = 0xf;
pub const VUL_CM0_ULTRA_TH_MASK_SFT: u32 = (0xf << 20);
pub const VUL_CM0_NORMAL_MODE_SFT: u32 = 17;
pub const VUL_CM0_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL_CM0_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 17);
pub const VUL_CM0_ODD_USE_EVEN_SFT: u32 = 16;
pub const VUL_CM0_ODD_USE_EVEN_MASK: u32 = 0x1;
pub const VUL_CM0_ODD_USE_EVEN_MASK_SFT: u32 = (0x1 << 16);
pub const VUL_CM0_AXI_REQ_MAXLEN_SFT: u32 = 12;
pub const VUL_CM0_AXI_REQ_MAXLEN_MASK: u32 = 0x3;
pub const VUL_CM0_AXI_REQ_MAXLEN_MASK_SFT: u32 = (0x3 << 12);
pub const VUL_CM0_AXI_REQ_MINLEN_SFT: u32 = 8;
pub const VUL_CM0_AXI_REQ_MINLEN_MASK: u32 = 0x3;
pub const VUL_CM0_AXI_REQ_MINLEN_MASK_SFT: u32 = (0x3 << 8);
pub const VUL_CM0_HALIGN_SFT: u32 = 7;
pub const VUL_CM0_HALIGN_MASK: u32 = 0x1;
pub const VUL_CM0_HALIGN_MASK_SFT: u32 = (0x1 << 7);
pub const VUL_CM0_SIGN_EXT_SFT: u32 = 6;
pub const VUL_CM0_SIGN_EXT_MASK: u32 = 0x1;
pub const VUL_CM0_SIGN_EXT_MASK_SFT: u32 = (0x1 << 6);
pub const VUL_CM0_HD_MODE_SFT: u32 = 4;
pub const VUL_CM0_HD_MODE_MASK: u32 = 0x3;
pub const VUL_CM0_HD_MODE_MASK_SFT: u32 = (0x3 << 4);
pub const VUL_CM0_MAKE_EXTRA_UPDATE_SFT: u32 = 3;
pub const VUL_CM0_MAKE_EXTRA_UPDATE_MASK: u32 = 0x1;
pub const VUL_CM0_MAKE_EXTRA_UPDATE_MASK_SFT: u32 = (0x1 << 3);
pub const VUL_CM0_AGENT_FREE_RUN_SFT: u32 = 2;
pub const VUL_CM0_AGENT_FREE_RUN_MASK: u32 = 0x1;
pub const VUL_CM0_AGENT_FREE_RUN_MASK_SFT: u32 = (0x1 << 2);
pub const VUL_CM0_USE_INT_ODD_SFT: u32 = 1;
pub const VUL_CM0_USE_INT_ODD_MASK: u32 = 0x1;
pub const VUL_CM0_USE_INT_ODD_MASK_SFT: u32 = (0x1 << 1);
pub const VUL_CM0_INT_ODD_FLAG_SFT: u32 = 0;
pub const VUL_CM0_INT_ODD_FLAG_MASK: u32 = 0x1;
pub const VUL_CM0_INT_ODD_FLAG_MASK_SFT: u32 = (0x1 << 0);

/* AFE_VUL_CM1_BASE_MSB */
pub const VUL_CM1_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL_CM1_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL_CM1_BASE_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL_CM1_BASE */
pub const VUL_CM1_BASE_ADDR_SFT: u32 = 4;
pub const VUL_CM1_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL_CM1_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL_CM1_CUR_MSB */
pub const VUL_CM1_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL_CM1_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL_CM1_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL_CM1_CUR */
pub const VUL_CM1_CUR_PTR_SFT: u32 = 0;
pub const VUL_CM1_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL_CM1_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL_CM1_END_MSB */
pub const VUL_CM1_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL_CM1_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL_CM1_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL_CM1_END */
pub const VUL_CM1_END_ADDR_SFT: u32 = 4;
pub const VUL_CM1_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL_CM1_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL_CM1_CON0 */
pub const VUL_CM1_ON_SFT: u32 = 28;
pub const VUL_CM1_ON_MASK: u32 = 0x1;
pub const VUL_CM1_ON_MASK_SFT: u32 = (0x1 << 28);
pub const VUL_CM1_REG_CH_SHIFT_MODE_SFT: u32 = 26;
pub const VUL_CM1_REG_CH_SHIFT_MODE_MASK: u32 = 0x1;
pub const VUL_CM1_REG_CH_SHIFT_MODE_MASK_SFT: u32 = (0x1 << 26);
pub const VUL_CM1_RG_FORCE_NO_MASK_EXTRA_SFT: u32 = 25;
pub const VUL_CM1_RG_FORCE_NO_MASK_EXTRA_MASK: u32 = 0x1;
pub const VUL_CM1_RG_FORCE_NO_MASK_EXTRA_MASK_SFT: u32 = (0x1 << 25);
pub const VUL_CM1_SW_CLEAR_BUF_FULL_SFT: u32 = 24;
pub const VUL_CM1_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL_CM1_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = (0x1 << 24);
pub const VUL_CM1_ULTRA_TH_SFT: u32 = 20;
pub const VUL_CM1_ULTRA_TH_MASK: u32 = 0xf;
pub const VUL_CM1_ULTRA_TH_MASK_SFT: u32 = (0xf << 20);
pub const VUL_CM1_NORMAL_MODE_SFT: u32 = 17;
pub const VUL_CM1_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL_CM1_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 17);
pub const VUL_CM1_ODD_USE_EVEN_SFT: u32 = 16;
pub const VUL_CM1_ODD_USE_EVEN_MASK: u32 = 0x1;
pub const VUL_CM1_ODD_USE_EVEN_MASK_SFT: u32 = (0x1 << 16);
pub const VUL_CM1_AXI_REQ_MAXLEN_SFT: u32 = 12;
pub const VUL_CM1_AXI_REQ_MAXLEN_MASK: u32 = 0x3;
pub const VUL_CM1_AXI_REQ_MAXLEN_MASK_SFT: u32 = (0x3 << 12);
pub const VUL_CM1_AXI_REQ_MINLEN_SFT: u32 = 8;
pub const VUL_CM1_AXI_REQ_MINLEN_MASK: u32 = 0x3;
pub const VUL_CM1_AXI_REQ_MINLEN_MASK_SFT: u32 = (0x3 << 8);
pub const VUL_CM1_HALIGN_SFT: u32 = 7;
pub const VUL_CM1_HALIGN_MASK: u32 = 0x1;
pub const VUL_CM1_HALIGN_MASK_SFT: u32 = (0x1 << 7);
pub const VUL_CM1_SIGN_EXT_SFT: u32 = 6;
pub const VUL_CM1_SIGN_EXT_MASK: u32 = 0x1;
pub const VUL_CM1_SIGN_EXT_MASK_SFT: u32 = (0x1 << 6);
pub const VUL_CM1_HD_MODE_SFT: u32 = 4;
pub const VUL_CM1_HD_MODE_MASK: u32 = 0x3;
pub const VUL_CM1_HD_MODE_MASK_SFT: u32 = (0x3 << 4);
pub const VUL_CM1_MAKE_EXTRA_UPDATE_SFT: u32 = 3;
pub const VUL_CM1_MAKE_EXTRA_UPDATE_MASK: u32 = 0x1;
pub const VUL_CM1_MAKE_EXTRA_UPDATE_MASK_SFT: u32 = (0x1 << 3);
pub const VUL_CM1_AGENT_FREE_RUN_SFT: u32 = 2;
pub const VUL_CM1_AGENT_FREE_RUN_MASK: u32 = 0x1;
pub const VUL_CM1_AGENT_FREE_RUN_MASK_SFT: u32 = (0x1 << 2);
pub const VUL_CM1_USE_INT_ODD_SFT: u32 = 1;
pub const VUL_CM1_USE_INT_ODD_MASK: u32 = 0x1;
pub const VUL_CM1_USE_INT_ODD_MASK_SFT: u32 = (0x1 << 1);
pub const VUL_CM1_INT_ODD_FLAG_SFT: u32 = 0;
pub const VUL_CM1_INT_ODD_FLAG_MASK: u32 = 0x1;
pub const VUL_CM1_INT_ODD_FLAG_MASK_SFT: u32 = (0x1 << 0);

/* AFE_VUL_CM2_BASE_MSB */
pub const VUL_CM2_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL_CM2_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL_CM2_BASE_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL_CM2_BASE */
pub const VUL_CM2_BASE_ADDR_SFT: u32 = 4;
pub const VUL_CM2_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL_CM2_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL_CM2_CUR_MSB */
pub const VUL_CM2_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL_CM2_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL_CM2_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL_CM2_CUR */
pub const VUL_CM2_CUR_PTR_SFT: u32 = 0;
pub const VUL_CM2_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL_CM2_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL_CM2_END_MSB */
pub const VUL_CM2_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL_CM2_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL_CM2_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_VUL_CM2_END */
pub const VUL_CM2_END_ADDR_SFT: u32 = 4;
pub const VUL_CM2_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL_CM2_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_VUL_CM2_CON0 */
pub const VUL_CM2_ON_SFT: u32 = 28;
pub const VUL_CM2_ON_MASK: u32 = 0x1;
pub const VUL_CM2_ON_MASK_SFT: u32 = (0x1 << 28);
pub const VUL_CM2_REG_CH_SHIFT_MODE_SFT: u32 = 26;
pub const VUL_CM2_REG_CH_SHIFT_MODE_MASK: u32 = 0x1;
pub const VUL_CM2_REG_CH_SHIFT_MODE_MASK_SFT: u32 = (0x1 << 26);
pub const VUL_CM2_RG_FORCE_NO_MASK_EXTRA_SFT: u32 = 25;
pub const VUL_CM2_RG_FORCE_NO_MASK_EXTRA_MASK: u32 = 0x1;
pub const VUL_CM2_RG_FORCE_NO_MASK_EXTRA_MASK_SFT: u32 = (0x1 << 25);
pub const VUL_CM2_SW_CLEAR_BUF_FULL_SFT: u32 = 24;
pub const VUL_CM2_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL_CM2_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = (0x1 << 24);
pub const VUL_CM2_ULTRA_TH_SFT: u32 = 20;
pub const VUL_CM2_ULTRA_TH_MASK: u32 = 0xf;
pub const VUL_CM2_ULTRA_TH_MASK_SFT: u32 = (0xf << 20);
pub const VUL_CM2_NORMAL_MODE_SFT: u32 = 17;
pub const VUL_CM2_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL_CM2_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 17);
pub const VUL_CM2_ODD_USE_EVEN_SFT: u32 = 16;
pub const VUL_CM2_ODD_USE_EVEN_MASK: u32 = 0x1;
pub const VUL_CM2_ODD_USE_EVEN_MASK_SFT: u32 = (0x1 << 16);
pub const VUL_CM2_AXI_REQ_MAXLEN_SFT: u32 = 12;
pub const VUL_CM2_AXI_REQ_MAXLEN_MASK: u32 = 0x3;
pub const VUL_CM2_AXI_REQ_MAXLEN_MASK_SFT: u32 = (0x3 << 12);
pub const VUL_CM2_AXI_REQ_MINLEN_SFT: u32 = 8;
pub const VUL_CM2_AXI_REQ_MINLEN_MASK: u32 = 0x3;
pub const VUL_CM2_AXI_REQ_MINLEN_MASK_SFT: u32 = (0x3 << 8);
pub const VUL_CM2_HALIGN_SFT: u32 = 7;
pub const VUL_CM2_HALIGN_MASK: u32 = 0x1;
pub const VUL_CM2_HALIGN_MASK_SFT: u32 = (0x1 << 7);
pub const VUL_CM2_SIGN_EXT_SFT: u32 = 6;
pub const VUL_CM2_SIGN_EXT_MASK: u32 = 0x1;
pub const VUL_CM2_SIGN_EXT_MASK_SFT: u32 = (0x1 << 6);
pub const VUL_CM2_HD_MODE_SFT: u32 = 4;
pub const VUL_CM2_HD_MODE_MASK: u32 = 0x3;
pub const VUL_CM2_HD_MODE_MASK_SFT: u32 = (0x3 << 4);
pub const VUL_CM2_MAKE_EXTRA_UPDATE_SFT: u32 = 3;
pub const VUL_CM2_MAKE_EXTRA_UPDATE_MASK: u32 = 0x1;
pub const VUL_CM2_MAKE_EXTRA_UPDATE_MASK_SFT: u32 = (0x1 << 3);
pub const VUL_CM2_AGENT_FREE_RUN_SFT: u32 = 2;
pub const VUL_CM2_AGENT_FREE_RUN_MASK: u32 = 0x1;
pub const VUL_CM2_AGENT_FREE_RUN_MASK_SFT: u32 = (0x1 << 2);
pub const VUL_CM2_USE_INT_ODD_SFT: u32 = 1;
pub const VUL_CM2_USE_INT_ODD_MASK: u32 = 0x1;
pub const VUL_CM2_USE_INT_ODD_MASK_SFT: u32 = (0x1 << 1);
pub const VUL_CM2_INT_ODD_FLAG_SFT: u32 = 0;
pub const VUL_CM2_INT_ODD_FLAG_MASK: u32 = 0x1;
pub const VUL_CM2_INT_ODD_FLAG_MASK_SFT: u32 = (0x1 << 0);

/* AFE_ETDM_IN0_BASE_MSB */
pub const ETDM_IN0_BASE_ADDR_MSB_SFT: u32 = 0;
pub const ETDM_IN0_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const ETDM_IN0_BASE_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_ETDM_IN0_BASE */
pub const ETDM_IN0_BASE_ADDR_SFT: u32 = 4;
pub const ETDM_IN0_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const ETDM_IN0_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_ETDM_IN0_CUR_MSB */
pub const ETDM_IN0_CUR_PTR_MSB_SFT: u32 = 0;
pub const ETDM_IN0_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const ETDM_IN0_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_ETDM_IN0_CUR */
pub const ETDM_IN0_CUR_PTR_SFT: u32 = 0;
pub const ETDM_IN0_CUR_PTR_MASK: u32 = 0xffffffff;
pub const ETDM_IN0_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ETDM_IN0_END_MSB */
pub const ETDM_IN0_END_ADDR_MSB_SFT: u32 = 0;
pub const ETDM_IN0_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const ETDM_IN0_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_ETDM_IN0_END */
pub const ETDM_IN0_END_ADDR_SFT: u32 = 4;
pub const ETDM_IN0_END_ADDR_MASK: u32 = 0xfffffff;
pub const ETDM_IN0_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_ETDM_IN0_CON0 */
pub const ETDM_IN0_CH_NUM_SFT: u32 = 28;
pub const ETDM_IN0_CH_NUM_MASK: u32 = 0xf;
pub const ETDM_IN0_CH_NUM_MASK_SFT: u32 = (0xf << 28);
pub const ETDM_IN0_ON_SFT: u32 = 27;
pub const ETDM_IN0_ON_MASK: u32 = 0x1;
pub const ETDM_IN0_ON_MASK_SFT: u32 = (0x1 << 27);
pub const ETDM_IN0_REG_CH_SHIFT_MODE_SFT: u32 = 26;
pub const ETDM_IN0_REG_CH_SHIFT_MODE_MASK: u32 = 0x1;
pub const ETDM_IN0_REG_CH_SHIFT_MODE_MASK_SFT: u32 = (0x1 << 26);
pub const ETDM_IN0_RG_FORCE_NO_MASK_EXTRA_SFT: u32 = 25;
pub const ETDM_IN0_RG_FORCE_NO_MASK_EXTRA_MASK: u32 = 0x1;
pub const ETDM_IN0_RG_FORCE_NO_MASK_EXTRA_MASK_SFT: u32 = (0x1 << 25);
pub const ETDM_IN0_SW_CLEAR_BUF_FULL_SFT: u32 = 24;
pub const ETDM_IN0_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const ETDM_IN0_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = (0x1 << 24);
pub const ETDM_IN0_ULTRA_TH_SFT: u32 = 20;
pub const ETDM_IN0_ULTRA_TH_MASK: u32 = 0xf;
pub const ETDM_IN0_ULTRA_TH_MASK_SFT: u32 = (0xf << 20);
pub const ETDM_IN0_NORMAL_MODE_SFT: u32 = 17;
pub const ETDM_IN0_NORMAL_MODE_MASK: u32 = 0x1;
pub const ETDM_IN0_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 17);
pub const ETDM_IN0_ODD_USE_EVEN_SFT: u32 = 16;
pub const ETDM_IN0_ODD_USE_EVEN_MASK: u32 = 0x1;
pub const ETDM_IN0_ODD_USE_EVEN_MASK_SFT: u32 = (0x1 << 16);
pub const ETDM_IN0_AXI_REQ_MAXLEN_SFT: u32 = 12;
pub const ETDM_IN0_AXI_REQ_MAXLEN_MASK: u32 = 0x3;
pub const ETDM_IN0_AXI_REQ_MAXLEN_MASK_SFT: u32 = (0x3 << 12);
pub const ETDM_IN0_AXI_REQ_MINLEN_SFT: u32 = 8;
pub const ETDM_IN0_AXI_REQ_MINLEN_MASK: u32 = 0x3;
pub const ETDM_IN0_AXI_REQ_MINLEN_MASK_SFT: u32 = (0x3 << 8);
pub const ETDM_IN0_HALIGN_SFT: u32 = 7;
pub const ETDM_IN0_HALIGN_MASK: u32 = 0x1;
pub const ETDM_IN0_HALIGN_MASK_SFT: u32 = (0x1 << 7);
pub const ETDM_IN0_SIGN_EXT_SFT: u32 = 6;
pub const ETDM_IN0_SIGN_EXT_MASK: u32 = 0x1;
pub const ETDM_IN0_SIGN_EXT_MASK_SFT: u32 = (0x1 << 6);
pub const ETDM_IN0_HD_MODE_SFT: u32 = 4;
pub const ETDM_IN0_HD_MODE_MASK: u32 = 0x3;
pub const ETDM_IN0_HD_MODE_MASK_SFT: u32 = (0x3 << 4);
pub const ETDM_IN0_MAKE_EXTRA_UPDATE_SFT: u32 = 3;
pub const ETDM_IN0_MAKE_EXTRA_UPDATE_MASK: u32 = 0x1;
pub const ETDM_IN0_MAKE_EXTRA_UPDATE_MASK_SFT: u32 = (0x1 << 3);
pub const ETDM_IN0_AGENT_FREE_RUN_SFT: u32 = 2;
pub const ETDM_IN0_AGENT_FREE_RUN_MASK: u32 = 0x1;
pub const ETDM_IN0_AGENT_FREE_RUN_MASK_SFT: u32 = (0x1 << 2);
pub const ETDM_IN0_USE_INT_ODD_SFT: u32 = 1;
pub const ETDM_IN0_USE_INT_ODD_MASK: u32 = 0x1;
pub const ETDM_IN0_USE_INT_ODD_MASK_SFT: u32 = (0x1 << 1);
pub const ETDM_IN0_INT_ODD_FLAG_SFT: u32 = 0;
pub const ETDM_IN0_INT_ODD_FLAG_MASK: u32 = 0x1;
pub const ETDM_IN0_INT_ODD_FLAG_MASK_SFT: u32 = (0x1 << 0);

/* AFE_ETDM_IN1_BASE_MSB */
pub const ETDM_IN1_BASE_ADDR_MSB_SFT: u32 = 0;
pub const ETDM_IN1_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const ETDM_IN1_BASE_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_ETDM_IN1_BASE */
pub const ETDM_IN1_BASE_ADDR_SFT: u32 = 4;
pub const ETDM_IN1_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const ETDM_IN1_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_ETDM_IN1_CUR_MSB */
pub const ETDM_IN1_CUR_PTR_MSB_SFT: u32 = 0;
pub const ETDM_IN1_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const ETDM_IN1_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_ETDM_IN1_CUR */
pub const ETDM_IN1_CUR_PTR_SFT: u32 = 0;
pub const ETDM_IN1_CUR_PTR_MASK: u32 = 0xffffffff;
pub const ETDM_IN1_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ETDM_IN1_END_MSB */
pub const ETDM_IN1_END_ADDR_MSB_SFT: u32 = 0;
pub const ETDM_IN1_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const ETDM_IN1_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_ETDM_IN1_END */
pub const ETDM_IN1_END_ADDR_SFT: u32 = 4;
pub const ETDM_IN1_END_ADDR_MASK: u32 = 0xfffffff;
pub const ETDM_IN1_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_ETDM_IN1_CON0 */
pub const ETDM_IN1_CH_NUM_SFT: u32 = 28;
pub const ETDM_IN1_CH_NUM_MASK: u32 = 0xf;
pub const ETDM_IN1_CH_NUM_MASK_SFT: u32 = (0xf << 28);
pub const ETDM_IN1_ON_SFT: u32 = 27;
pub const ETDM_IN1_ON_MASK: u32 = 0x1;
pub const ETDM_IN1_ON_MASK_SFT: u32 = (0x1 << 27);
pub const ETDM_IN1_REG_CH_SHIFT_MODE_SFT: u32 = 26;
pub const ETDM_IN1_REG_CH_SHIFT_MODE_MASK: u32 = 0x1;
pub const ETDM_IN1_REG_CH_SHIFT_MODE_MASK_SFT: u32 = (0x1 << 26);
pub const ETDM_IN1_RG_FORCE_NO_MASK_EXTRA_SFT: u32 = 25;
pub const ETDM_IN1_RG_FORCE_NO_MASK_EXTRA_MASK: u32 = 0x1;
pub const ETDM_IN1_RG_FORCE_NO_MASK_EXTRA_MASK_SFT: u32 = (0x1 << 25);
pub const ETDM_IN1_SW_CLEAR_BUF_FULL_SFT: u32 = 24;
pub const ETDM_IN1_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const ETDM_IN1_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = (0x1 << 24);
pub const ETDM_IN1_ULTRA_TH_SFT: u32 = 20;
pub const ETDM_IN1_ULTRA_TH_MASK: u32 = 0xf;
pub const ETDM_IN1_ULTRA_TH_MASK_SFT: u32 = (0xf << 20);
pub const ETDM_IN1_NORMAL_MODE_SFT: u32 = 17;
pub const ETDM_IN1_NORMAL_MODE_MASK: u32 = 0x1;
pub const ETDM_IN1_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 17);
pub const ETDM_IN1_ODD_USE_EVEN_SFT: u32 = 16;
pub const ETDM_IN1_ODD_USE_EVEN_MASK: u32 = 0x1;
pub const ETDM_IN1_ODD_USE_EVEN_MASK_SFT: u32 = (0x1 << 16);
pub const ETDM_IN1_AXI_REQ_MAXLEN_SFT: u32 = 12;
pub const ETDM_IN1_AXI_REQ_MAXLEN_MASK: u32 = 0x3;
pub const ETDM_IN1_AXI_REQ_MAXLEN_MASK_SFT: u32 = (0x3 << 12);
pub const ETDM_IN1_AXI_REQ_MINLEN_SFT: u32 = 8;
pub const ETDM_IN1_AXI_REQ_MINLEN_MASK: u32 = 0x3;
pub const ETDM_IN1_AXI_REQ_MINLEN_MASK_SFT: u32 = (0x3 << 8);
pub const ETDM_IN1_HALIGN_SFT: u32 = 7;
pub const ETDM_IN1_HALIGN_MASK: u32 = 0x1;
pub const ETDM_IN1_HALIGN_MASK_SFT: u32 = (0x1 << 7);
pub const ETDM_IN1_SIGN_EXT_SFT: u32 = 6;
pub const ETDM_IN1_SIGN_EXT_MASK: u32 = 0x1;
pub const ETDM_IN1_SIGN_EXT_MASK_SFT: u32 = (0x1 << 6);
pub const ETDM_IN1_HD_MODE_SFT: u32 = 4;
pub const ETDM_IN1_HD_MODE_MASK: u32 = 0x3;
pub const ETDM_IN1_HD_MODE_MASK_SFT: u32 = (0x3 << 4);
pub const ETDM_IN1_MAKE_EXTRA_UPDATE_SFT: u32 = 3;
pub const ETDM_IN1_MAKE_EXTRA_UPDATE_MASK: u32 = 0x1;
pub const ETDM_IN1_MAKE_EXTRA_UPDATE_MASK_SFT: u32 = (0x1 << 3);
pub const ETDM_IN1_AGENT_FREE_RUN_SFT: u32 = 2;
pub const ETDM_IN1_AGENT_FREE_RUN_MASK: u32 = 0x1;
pub const ETDM_IN1_AGENT_FREE_RUN_MASK_SFT: u32 = (0x1 << 2);
pub const ETDM_IN1_USE_INT_ODD_SFT: u32 = 1;
pub const ETDM_IN1_USE_INT_ODD_MASK: u32 = 0x1;
pub const ETDM_IN1_USE_INT_ODD_MASK_SFT: u32 = (0x1 << 1);
pub const ETDM_IN1_INT_ODD_FLAG_SFT: u32 = 0;
pub const ETDM_IN1_INT_ODD_FLAG_MASK: u32 = 0x1;
pub const ETDM_IN1_INT_ODD_FLAG_MASK_SFT: u32 = (0x1 << 0);

/* AFE_ETDM_IN2_BASE_MSB */
pub const ETDM_IN2_BASE_ADDR_MSB_SFT: u32 = 0;
pub const ETDM_IN2_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const ETDM_IN2_BASE_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_ETDM_IN2_BASE */
pub const ETDM_IN2_BASE_ADDR_SFT: u32 = 4;
pub const ETDM_IN2_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const ETDM_IN2_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_ETDM_IN2_CUR_MSB */
pub const ETDM_IN2_CUR_PTR_MSB_SFT: u32 = 0;
pub const ETDM_IN2_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const ETDM_IN2_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_ETDM_IN2_CUR */
pub const ETDM_IN2_CUR_PTR_SFT: u32 = 0;
pub const ETDM_IN2_CUR_PTR_MASK: u32 = 0xffffffff;
pub const ETDM_IN2_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ETDM_IN2_END_MSB */
pub const ETDM_IN2_END_ADDR_MSB_SFT: u32 = 0;
pub const ETDM_IN2_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const ETDM_IN2_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_ETDM_IN2_END */
pub const ETDM_IN2_END_ADDR_SFT: u32 = 4;
pub const ETDM_IN2_END_ADDR_MASK: u32 = 0xfffffff;
pub const ETDM_IN2_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_ETDM_IN2_CON0 */
pub const ETDM_IN2_CH_NUM_SFT: u32 = 28;
pub const ETDM_IN2_CH_NUM_MASK: u32 = 0xf;
pub const ETDM_IN2_CH_NUM_MASK_SFT: u32 = (0xf << 28);
pub const ETDM_IN2_ON_SFT: u32 = 27;
pub const ETDM_IN2_ON_MASK: u32 = 0x1;
pub const ETDM_IN2_ON_MASK_SFT: u32 = (0x1 << 27);
pub const ETDM_IN2_REG_CH_SHIFT_MODE_SFT: u32 = 26;
pub const ETDM_IN2_REG_CH_SHIFT_MODE_MASK: u32 = 0x1;
pub const ETDM_IN2_REG_CH_SHIFT_MODE_MASK_SFT: u32 = (0x1 << 26);
pub const ETDM_IN2_RG_FORCE_NO_MASK_EXTRA_SFT: u32 = 25;
pub const ETDM_IN2_RG_FORCE_NO_MASK_EXTRA_MASK: u32 = 0x1;
pub const ETDM_IN2_RG_FORCE_NO_MASK_EXTRA_MASK_SFT: u32 = (0x1 << 25);
pub const ETDM_IN2_SW_CLEAR_BUF_FULL_SFT: u32 = 24;
pub const ETDM_IN2_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const ETDM_IN2_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = (0x1 << 24);
pub const ETDM_IN2_ULTRA_TH_SFT: u32 = 20;
pub const ETDM_IN2_ULTRA_TH_MASK: u32 = 0xf;
pub const ETDM_IN2_ULTRA_TH_MASK_SFT: u32 = (0xf << 20);
pub const ETDM_IN2_NORMAL_MODE_SFT: u32 = 17;
pub const ETDM_IN2_NORMAL_MODE_MASK: u32 = 0x1;
pub const ETDM_IN2_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 17);
pub const ETDM_IN2_ODD_USE_EVEN_SFT: u32 = 16;
pub const ETDM_IN2_ODD_USE_EVEN_MASK: u32 = 0x1;
pub const ETDM_IN2_ODD_USE_EVEN_MASK_SFT: u32 = (0x1 << 16);
pub const ETDM_IN2_AXI_REQ_MAXLEN_SFT: u32 = 12;
pub const ETDM_IN2_AXI_REQ_MAXLEN_MASK: u32 = 0x3;
pub const ETDM_IN2_AXI_REQ_MAXLEN_MASK_SFT: u32 = (0x3 << 12);
pub const ETDM_IN2_AXI_REQ_MINLEN_SFT: u32 = 8;
pub const ETDM_IN2_AXI_REQ_MINLEN_MASK: u32 = 0x3;
pub const ETDM_IN2_AXI_REQ_MINLEN_MASK_SFT: u32 = (0x3 << 8);
pub const ETDM_IN2_HALIGN_SFT: u32 = 7;
pub const ETDM_IN2_HALIGN_MASK: u32 = 0x1;
pub const ETDM_IN2_HALIGN_MASK_SFT: u32 = (0x1 << 7);
pub const ETDM_IN2_SIGN_EXT_SFT: u32 = 6;
pub const ETDM_IN2_SIGN_EXT_MASK: u32 = 0x1;
pub const ETDM_IN2_SIGN_EXT_MASK_SFT: u32 = (0x1 << 6);
pub const ETDM_IN2_HD_MODE_SFT: u32 = 4;
pub const ETDM_IN2_HD_MODE_MASK: u32 = 0x3;
pub const ETDM_IN2_HD_MODE_MASK_SFT: u32 = (0x3 << 4);
pub const ETDM_IN2_MAKE_EXTRA_UPDATE_SFT: u32 = 3;
pub const ETDM_IN2_MAKE_EXTRA_UPDATE_MASK: u32 = 0x1;
pub const ETDM_IN2_MAKE_EXTRA_UPDATE_MASK_SFT: u32 = (0x1 << 3);
pub const ETDM_IN2_AGENT_FREE_RUN_SFT: u32 = 2;
pub const ETDM_IN2_AGENT_FREE_RUN_MASK: u32 = 0x1;
pub const ETDM_IN2_AGENT_FREE_RUN_MASK_SFT: u32 = (0x1 << 2);
pub const ETDM_IN2_USE_INT_ODD_SFT: u32 = 1;
pub const ETDM_IN2_USE_INT_ODD_MASK: u32 = 0x1;
pub const ETDM_IN2_USE_INT_ODD_MASK_SFT: u32 = (0x1 << 1);
pub const ETDM_IN2_INT_ODD_FLAG_SFT: u32 = 0;
pub const ETDM_IN2_INT_ODD_FLAG_MASK: u32 = 0x1;
pub const ETDM_IN2_INT_ODD_FLAG_MASK_SFT: u32 = (0x1 << 0);

/* AFE_ETDM_IN3_BASE_MSB */
pub const ETDM_IN3_BASE_ADDR_MSB_SFT: u32 = 0;
pub const ETDM_IN3_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const ETDM_IN3_BASE_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_ETDM_IN3_BASE */
pub const ETDM_IN3_BASE_ADDR_SFT: u32 = 4;
pub const ETDM_IN3_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const ETDM_IN3_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_ETDM_IN3_CUR_MSB */
pub const ETDM_IN3_CUR_PTR_MSB_SFT: u32 = 0;
pub const ETDM_IN3_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const ETDM_IN3_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_ETDM_IN3_CUR */
pub const ETDM_IN3_CUR_PTR_SFT: u32 = 0;
pub const ETDM_IN3_CUR_PTR_MASK: u32 = 0xffffffff;
pub const ETDM_IN3_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ETDM_IN3_END_MSB */
pub const ETDM_IN3_END_ADDR_MSB_SFT: u32 = 0;
pub const ETDM_IN3_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const ETDM_IN3_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_ETDM_IN3_END */
pub const ETDM_IN3_END_ADDR_SFT: u32 = 4;
pub const ETDM_IN3_END_ADDR_MASK: u32 = 0xfffffff;
pub const ETDM_IN3_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_ETDM_IN3_CON0 */
pub const ETDM_IN3_CH_NUM_SFT: u32 = 28;
pub const ETDM_IN3_CH_NUM_MASK: u32 = 0xf;
pub const ETDM_IN3_CH_NUM_MASK_SFT: u32 = (0xf << 28);
pub const ETDM_IN3_ON_SFT: u32 = 27;
pub const ETDM_IN3_ON_MASK: u32 = 0x1;
pub const ETDM_IN3_ON_MASK_SFT: u32 = (0x1 << 27);
pub const ETDM_IN3_REG_CH_SHIFT_MODE_SFT: u32 = 26;
pub const ETDM_IN3_REG_CH_SHIFT_MODE_MASK: u32 = 0x1;
pub const ETDM_IN3_REG_CH_SHIFT_MODE_MASK_SFT: u32 = (0x1 << 26);
pub const ETDM_IN3_RG_FORCE_NO_MASK_EXTRA_SFT: u32 = 25;
pub const ETDM_IN3_RG_FORCE_NO_MASK_EXTRA_MASK: u32 = 0x1;
pub const ETDM_IN3_RG_FORCE_NO_MASK_EXTRA_MASK_SFT: u32 = (0x1 << 25);
pub const ETDM_IN3_SW_CLEAR_BUF_FULL_SFT: u32 = 24;
pub const ETDM_IN3_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const ETDM_IN3_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = (0x1 << 24);
pub const ETDM_IN3_ULTRA_TH_SFT: u32 = 20;
pub const ETDM_IN3_ULTRA_TH_MASK: u32 = 0xf;
pub const ETDM_IN3_ULTRA_TH_MASK_SFT: u32 = (0xf << 20);
pub const ETDM_IN3_NORMAL_MODE_SFT: u32 = 17;
pub const ETDM_IN3_NORMAL_MODE_MASK: u32 = 0x1;
pub const ETDM_IN3_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 17);
pub const ETDM_IN3_ODD_USE_EVEN_SFT: u32 = 16;
pub const ETDM_IN3_ODD_USE_EVEN_MASK: u32 = 0x1;
pub const ETDM_IN3_ODD_USE_EVEN_MASK_SFT: u32 = (0x1 << 16);
pub const ETDM_IN3_AXI_REQ_MAXLEN_SFT: u32 = 12;
pub const ETDM_IN3_AXI_REQ_MAXLEN_MASK: u32 = 0x3;
pub const ETDM_IN3_AXI_REQ_MAXLEN_MASK_SFT: u32 = (0x3 << 12);
pub const ETDM_IN3_AXI_REQ_MINLEN_SFT: u32 = 8;
pub const ETDM_IN3_AXI_REQ_MINLEN_MASK: u32 = 0x3;
pub const ETDM_IN3_AXI_REQ_MINLEN_MASK_SFT: u32 = (0x3 << 8);
pub const ETDM_IN3_HALIGN_SFT: u32 = 7;
pub const ETDM_IN3_HALIGN_MASK: u32 = 0x1;
pub const ETDM_IN3_HALIGN_MASK_SFT: u32 = (0x1 << 7);
pub const ETDM_IN3_SIGN_EXT_SFT: u32 = 6;
pub const ETDM_IN3_SIGN_EXT_MASK: u32 = 0x1;
pub const ETDM_IN3_SIGN_EXT_MASK_SFT: u32 = (0x1 << 6);
pub const ETDM_IN3_HD_MODE_SFT: u32 = 4;
pub const ETDM_IN3_HD_MODE_MASK: u32 = 0x3;
pub const ETDM_IN3_HD_MODE_MASK_SFT: u32 = (0x3 << 4);
pub const ETDM_IN3_MAKE_EXTRA_UPDATE_SFT: u32 = 3;
pub const ETDM_IN3_MAKE_EXTRA_UPDATE_MASK: u32 = 0x1;
pub const ETDM_IN3_MAKE_EXTRA_UPDATE_MASK_SFT: u32 = (0x1 << 3);
pub const ETDM_IN3_AGENT_FREE_RUN_SFT: u32 = 2;
pub const ETDM_IN3_AGENT_FREE_RUN_MASK: u32 = 0x1;
pub const ETDM_IN3_AGENT_FREE_RUN_MASK_SFT: u32 = (0x1 << 2);
pub const ETDM_IN3_USE_INT_ODD_SFT: u32 = 1;
pub const ETDM_IN3_USE_INT_ODD_MASK: u32 = 0x1;
pub const ETDM_IN3_USE_INT_ODD_MASK_SFT: u32 = (0x1 << 1);
pub const ETDM_IN3_INT_ODD_FLAG_SFT: u32 = 0;
pub const ETDM_IN3_INT_ODD_FLAG_MASK: u32 = 0x1;
pub const ETDM_IN3_INT_ODD_FLAG_MASK_SFT: u32 = (0x1 << 0);

/* AFE_ETDM_IN4_BASE_MSB */
pub const ETDM_IN4_BASE_ADDR_MSB_SFT: u32 = 0;
pub const ETDM_IN4_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const ETDM_IN4_BASE_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_ETDM_IN4_BASE */
pub const ETDM_IN4_BASE_ADDR_SFT: u32 = 4;
pub const ETDM_IN4_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const ETDM_IN4_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_ETDM_IN4_CUR_MSB */
pub const ETDM_IN4_CUR_PTR_MSB_SFT: u32 = 0;
pub const ETDM_IN4_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const ETDM_IN4_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_ETDM_IN4_CUR */
pub const ETDM_IN4_CUR_PTR_SFT: u32 = 0;
pub const ETDM_IN4_CUR_PTR_MASK: u32 = 0xffffffff;
pub const ETDM_IN4_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ETDM_IN4_END_MSB */
pub const ETDM_IN4_END_ADDR_MSB_SFT: u32 = 0;
pub const ETDM_IN4_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const ETDM_IN4_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_ETDM_IN4_END */
pub const ETDM_IN4_END_ADDR_SFT: u32 = 4;
pub const ETDM_IN4_END_ADDR_MASK: u32 = 0xfffffff;
pub const ETDM_IN4_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_ETDM_IN4_CON0 */
pub const ETDM_IN4_CH_NUM_SFT: u32 = 28;
pub const ETDM_IN4_CH_NUM_MASK: u32 = 0xf;
pub const ETDM_IN4_CH_NUM_MASK_SFT: u32 = (0xf << 28);
pub const ETDM_IN4_ON_SFT: u32 = 27;
pub const ETDM_IN4_ON_MASK: u32 = 0x1;
pub const ETDM_IN4_ON_MASK_SFT: u32 = (0x1 << 27);
pub const ETDM_IN4_REG_CH_SHIFT_MODE_SFT: u32 = 26;
pub const ETDM_IN4_REG_CH_SHIFT_MODE_MASK: u32 = 0x1;
pub const ETDM_IN4_REG_CH_SHIFT_MODE_MASK_SFT: u32 = (0x1 << 26);
pub const ETDM_IN4_RG_FORCE_NO_MASK_EXTRA_SFT: u32 = 25;
pub const ETDM_IN4_RG_FORCE_NO_MASK_EXTRA_MASK: u32 = 0x1;
pub const ETDM_IN4_RG_FORCE_NO_MASK_EXTRA_MASK_SFT: u32 = (0x1 << 25);
pub const ETDM_IN4_SW_CLEAR_BUF_FULL_SFT: u32 = 24;
pub const ETDM_IN4_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const ETDM_IN4_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = (0x1 << 24);
pub const ETDM_IN4_ULTRA_TH_SFT: u32 = 20;
pub const ETDM_IN4_ULTRA_TH_MASK: u32 = 0xf;
pub const ETDM_IN4_ULTRA_TH_MASK_SFT: u32 = (0xf << 20);
pub const ETDM_IN4_NORMAL_MODE_SFT: u32 = 17;
pub const ETDM_IN4_NORMAL_MODE_MASK: u32 = 0x1;
pub const ETDM_IN4_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 17);
pub const ETDM_IN4_ODD_USE_EVEN_SFT: u32 = 16;
pub const ETDM_IN4_ODD_USE_EVEN_MASK: u32 = 0x1;
pub const ETDM_IN4_ODD_USE_EVEN_MASK_SFT: u32 = (0x1 << 16);
pub const ETDM_IN4_AXI_REQ_MAXLEN_SFT: u32 = 12;
pub const ETDM_IN4_AXI_REQ_MAXLEN_MASK: u32 = 0x3;
pub const ETDM_IN4_AXI_REQ_MAXLEN_MASK_SFT: u32 = (0x3 << 12);
pub const ETDM_IN4_AXI_REQ_MINLEN_SFT: u32 = 8;
pub const ETDM_IN4_AXI_REQ_MINLEN_MASK: u32 = 0x3;
pub const ETDM_IN4_AXI_REQ_MINLEN_MASK_SFT: u32 = (0x3 << 8);
pub const ETDM_IN4_HALIGN_SFT: u32 = 7;
pub const ETDM_IN4_HALIGN_MASK: u32 = 0x1;
pub const ETDM_IN4_HALIGN_MASK_SFT: u32 = (0x1 << 7);
pub const ETDM_IN4_SIGN_EXT_SFT: u32 = 6;
pub const ETDM_IN4_SIGN_EXT_MASK: u32 = 0x1;
pub const ETDM_IN4_SIGN_EXT_MASK_SFT: u32 = (0x1 << 6);
pub const ETDM_IN4_HD_MODE_SFT: u32 = 4;
pub const ETDM_IN4_HD_MODE_MASK: u32 = 0x3;
pub const ETDM_IN4_HD_MODE_MASK_SFT: u32 = (0x3 << 4);
pub const ETDM_IN4_MAKE_EXTRA_UPDATE_SFT: u32 = 3;
pub const ETDM_IN4_MAKE_EXTRA_UPDATE_MASK: u32 = 0x1;
pub const ETDM_IN4_MAKE_EXTRA_UPDATE_MASK_SFT: u32 = (0x1 << 3);
pub const ETDM_IN4_AGENT_FREE_RUN_SFT: u32 = 2;
pub const ETDM_IN4_AGENT_FREE_RUN_MASK: u32 = 0x1;
pub const ETDM_IN4_AGENT_FREE_RUN_MASK_SFT: u32 = (0x1 << 2);
pub const ETDM_IN4_USE_INT_ODD_SFT: u32 = 1;
pub const ETDM_IN4_USE_INT_ODD_MASK: u32 = 0x1;
pub const ETDM_IN4_USE_INT_ODD_MASK_SFT: u32 = (0x1 << 1);
pub const ETDM_IN4_INT_ODD_FLAG_SFT: u32 = 0;
pub const ETDM_IN4_INT_ODD_FLAG_MASK: u32 = 0x1;
pub const ETDM_IN4_INT_ODD_FLAG_MASK_SFT: u32 = (0x1 << 0);

/* AFE_ETDM_IN5_BASE_MSB */
pub const ETDM_IN5_BASE_ADDR_MSB_SFT: u32 = 0;
pub const ETDM_IN5_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const ETDM_IN5_BASE_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_ETDM_IN5_BASE */
pub const ETDM_IN5_BASE_ADDR_SFT: u32 = 4;
pub const ETDM_IN5_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const ETDM_IN5_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_ETDM_IN5_CUR_MSB */
pub const ETDM_IN5_CUR_PTR_MSB_SFT: u32 = 0;
pub const ETDM_IN5_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const ETDM_IN5_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_ETDM_IN5_CUR */
pub const ETDM_IN5_CUR_PTR_SFT: u32 = 0;
pub const ETDM_IN5_CUR_PTR_MASK: u32 = 0xffffffff;
pub const ETDM_IN5_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ETDM_IN5_END_MSB */
pub const ETDM_IN5_END_ADDR_MSB_SFT: u32 = 0;
pub const ETDM_IN5_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const ETDM_IN5_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_ETDM_IN5_END */
pub const ETDM_IN5_END_ADDR_SFT: u32 = 4;
pub const ETDM_IN5_END_ADDR_MASK: u32 = 0xfffffff;
pub const ETDM_IN5_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_ETDM_IN5_CON0 */
pub const ETDM_IN5_CH_NUM_SFT: u32 = 28;
pub const ETDM_IN5_CH_NUM_MASK: u32 = 0xf;
pub const ETDM_IN5_CH_NUM_MASK_SFT: u32 = (0xf << 28);
pub const ETDM_IN5_ON_SFT: u32 = 27;
pub const ETDM_IN5_ON_MASK: u32 = 0x1;
pub const ETDM_IN5_ON_MASK_SFT: u32 = (0x1 << 27);
pub const ETDM_IN5_REG_CH_SHIFT_MODE_SFT: u32 = 26;
pub const ETDM_IN5_REG_CH_SHIFT_MODE_MASK: u32 = 0x1;
pub const ETDM_IN5_REG_CH_SHIFT_MODE_MASK_SFT: u32 = (0x1 << 26);
pub const ETDM_IN5_RG_FORCE_NO_MASK_EXTRA_SFT: u32 = 25;
pub const ETDM_IN5_RG_FORCE_NO_MASK_EXTRA_MASK: u32 = 0x1;
pub const ETDM_IN5_RG_FORCE_NO_MASK_EXTRA_MASK_SFT: u32 = (0x1 << 25);
pub const ETDM_IN5_SW_CLEAR_BUF_FULL_SFT: u32 = 24;
pub const ETDM_IN5_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const ETDM_IN5_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = (0x1 << 24);
pub const ETDM_IN5_ULTRA_TH_SFT: u32 = 20;
pub const ETDM_IN5_ULTRA_TH_MASK: u32 = 0xf;
pub const ETDM_IN5_ULTRA_TH_MASK_SFT: u32 = (0xf << 20);
pub const ETDM_IN5_NORMAL_MODE_SFT: u32 = 17;
pub const ETDM_IN5_NORMAL_MODE_MASK: u32 = 0x1;
pub const ETDM_IN5_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 17);
pub const ETDM_IN5_ODD_USE_EVEN_SFT: u32 = 16;
pub const ETDM_IN5_ODD_USE_EVEN_MASK: u32 = 0x1;
pub const ETDM_IN5_ODD_USE_EVEN_MASK_SFT: u32 = (0x1 << 16);
pub const ETDM_IN5_AXI_REQ_MAXLEN_SFT: u32 = 12;
pub const ETDM_IN5_AXI_REQ_MAXLEN_MASK: u32 = 0x3;
pub const ETDM_IN5_AXI_REQ_MAXLEN_MASK_SFT: u32 = (0x3 << 12);
pub const ETDM_IN5_AXI_REQ_MINLEN_SFT: u32 = 8;
pub const ETDM_IN5_AXI_REQ_MINLEN_MASK: u32 = 0x3;
pub const ETDM_IN5_AXI_REQ_MINLEN_MASK_SFT: u32 = (0x3 << 8);
pub const ETDM_IN5_HALIGN_SFT: u32 = 7;
pub const ETDM_IN5_HALIGN_MASK: u32 = 0x1;
pub const ETDM_IN5_HALIGN_MASK_SFT: u32 = (0x1 << 7);
pub const ETDM_IN5_SIGN_EXT_SFT: u32 = 6;
pub const ETDM_IN5_SIGN_EXT_MASK: u32 = 0x1;
pub const ETDM_IN5_SIGN_EXT_MASK_SFT: u32 = (0x1 << 6);
pub const ETDM_IN5_HD_MODE_SFT: u32 = 4;
pub const ETDM_IN5_HD_MODE_MASK: u32 = 0x3;
pub const ETDM_IN5_HD_MODE_MASK_SFT: u32 = (0x3 << 4);
pub const ETDM_IN5_MAKE_EXTRA_UPDATE_SFT: u32 = 3;
pub const ETDM_IN5_MAKE_EXTRA_UPDATE_MASK: u32 = 0x1;
pub const ETDM_IN5_MAKE_EXTRA_UPDATE_MASK_SFT: u32 = (0x1 << 3);
pub const ETDM_IN5_AGENT_FREE_RUN_SFT: u32 = 2;
pub const ETDM_IN5_AGENT_FREE_RUN_MASK: u32 = 0x1;
pub const ETDM_IN5_AGENT_FREE_RUN_MASK_SFT: u32 = (0x1 << 2);
pub const ETDM_IN5_USE_INT_ODD_SFT: u32 = 1;
pub const ETDM_IN5_USE_INT_ODD_MASK: u32 = 0x1;
pub const ETDM_IN5_USE_INT_ODD_MASK_SFT: u32 = (0x1 << 1);
pub const ETDM_IN5_INT_ODD_FLAG_SFT: u32 = 0;
pub const ETDM_IN5_INT_ODD_FLAG_MASK: u32 = 0x1;
pub const ETDM_IN5_INT_ODD_FLAG_MASK_SFT: u32 = (0x1 << 0);

/* AFE_ETDM_IN6_BASE_MSB */
pub const ETDM_IN6_BASE_ADDR_MSB_SFT: u32 = 0;
pub const ETDM_IN6_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const ETDM_IN6_BASE_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_ETDM_IN6_BASE */
pub const ETDM_IN6_BASE_ADDR_SFT: u32 = 4;
pub const ETDM_IN6_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const ETDM_IN6_BASE_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_ETDM_IN6_CUR_MSB */
pub const ETDM_IN6_CUR_PTR_MSB_SFT: u32 = 0;
pub const ETDM_IN6_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const ETDM_IN6_CUR_PTR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_ETDM_IN6_CUR */
pub const ETDM_IN6_CUR_PTR_SFT: u32 = 0;
pub const ETDM_IN6_CUR_PTR_MASK: u32 = 0xffffffff;
pub const ETDM_IN6_CUR_PTR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ETDM_IN6_END_MSB */
pub const ETDM_IN6_END_ADDR_MSB_SFT: u32 = 0;
pub const ETDM_IN6_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const ETDM_IN6_END_ADDR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_ETDM_IN6_END */
pub const ETDM_IN6_END_ADDR_SFT: u32 = 4;
pub const ETDM_IN6_END_ADDR_MASK: u32 = 0xfffffff;
pub const ETDM_IN6_END_ADDR_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_ETDM_IN6_CON0 */
pub const ETDM_IN6_CH_NUM_SFT: u32 = 28;
pub const ETDM_IN6_CH_NUM_MASK: u32 = 0xf;
pub const ETDM_IN6_CH_NUM_MASK_SFT: u32 = (0xf << 28);
pub const ETDM_IN6_ON_SFT: u32 = 27;
pub const ETDM_IN6_ON_MASK: u32 = 0x1;
pub const ETDM_IN6_ON_MASK_SFT: u32 = (0x1 << 27);
pub const ETDM_IN6_REG_CH_SHIFT_MODE_SFT: u32 = 26;
pub const ETDM_IN6_REG_CH_SHIFT_MODE_MASK: u32 = 0x1;
pub const ETDM_IN6_REG_CH_SHIFT_MODE_MASK_SFT: u32 = (0x1 << 26);
pub const ETDM_IN6_RG_FORCE_NO_MASK_EXTRA_SFT: u32 = 25;
pub const ETDM_IN6_RG_FORCE_NO_MASK_EXTRA_MASK: u32 = 0x1;
pub const ETDM_IN6_RG_FORCE_NO_MASK_EXTRA_MASK_SFT: u32 = (0x1 << 25);
pub const ETDM_IN6_SW_CLEAR_BUF_FULL_SFT: u32 = 24;
pub const ETDM_IN6_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const ETDM_IN6_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = (0x1 << 24);
pub const ETDM_IN6_ULTRA_TH_SFT: u32 = 20;
pub const ETDM_IN6_ULTRA_TH_MASK: u32 = 0xf;
pub const ETDM_IN6_ULTRA_TH_MASK_SFT: u32 = (0xf << 20);
pub const ETDM_IN6_NORMAL_MODE_SFT: u32 = 17;
pub const ETDM_IN6_NORMAL_MODE_MASK: u32 = 0x1;
pub const ETDM_IN6_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 17);
pub const ETDM_IN6_ODD_USE_EVEN_SFT: u32 = 16;
pub const ETDM_IN6_ODD_USE_EVEN_MASK: u32 = 0x1;
pub const ETDM_IN6_ODD_USE_EVEN_MASK_SFT: u32 = (0x1 << 16);
pub const ETDM_IN6_AXI_REQ_MAXLEN_SFT: u32 = 12;
pub const ETDM_IN6_AXI_REQ_MAXLEN_MASK: u32 = 0x3;
pub const ETDM_IN6_AXI_REQ_MAXLEN_MASK_SFT: u32 = (0x3 << 12);
pub const ETDM_IN6_AXI_REQ_MINLEN_SFT: u32 = 8;
pub const ETDM_IN6_AXI_REQ_MINLEN_MASK: u32 = 0x3;
pub const ETDM_IN6_AXI_REQ_MINLEN_MASK_SFT: u32 = (0x3 << 8);
pub const ETDM_IN6_HALIGN_SFT: u32 = 7;
pub const ETDM_IN6_HALIGN_MASK: u32 = 0x1;
pub const ETDM_IN6_HALIGN_MASK_SFT: u32 = (0x1 << 7);
pub const ETDM_IN6_SIGN_EXT_SFT: u32 = 6;
pub const ETDM_IN6_SIGN_EXT_MASK: u32 = 0x1;
pub const ETDM_IN6_SIGN_EXT_MASK_SFT: u32 = (0x1 << 6);
pub const ETDM_IN6_HD_MODE_SFT: u32 = 4;
pub const ETDM_IN6_HD_MODE_MASK: u32 = 0x3;
pub const ETDM_IN6_HD_MODE_MASK_SFT: u32 = (0x3 << 4);
pub const ETDM_IN6_MAKE_EXTRA_UPDATE_SFT: u32 = 3;
pub const ETDM_IN6_MAKE_EXTRA_UPDATE_MASK: u32 = 0x1;
pub const ETDM_IN6_MAKE_EXTRA_UPDATE_MASK_SFT: u32 = (0x1 << 3);
pub const ETDM_IN6_AGENT_FREE_RUN_SFT: u32 = 2;
pub const ETDM_IN6_AGENT_FREE_RUN_MASK: u32 = 0x1;
pub const ETDM_IN6_AGENT_FREE_RUN_MASK_SFT: u32 = (0x1 << 2);
pub const ETDM_IN6_USE_INT_ODD_SFT: u32 = 1;
pub const ETDM_IN6_USE_INT_ODD_MASK: u32 = 0x1;
pub const ETDM_IN6_USE_INT_ODD_MASK_SFT: u32 = (0x1 << 1);
pub const ETDM_IN6_INT_ODD_FLAG_SFT: u32 = 0;
pub const ETDM_IN6_INT_ODD_FLAG_MASK: u32 = 0x1;
pub const ETDM_IN6_INT_ODD_FLAG_MASK_SFT: u32 = (0x1 << 0);

/* AFE_HDMI_OUT_BASE_MSB */
pub const AFE_HDMI_OUT_BASE_MSB_SFT: u32 = 0;
pub const AFE_HDMI_OUT_BASE_MSB_MASK: u32 = 0x1ff;
pub const AFE_HDMI_OUT_BASE_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_HDMI_OUT_BASE */
pub const AFE_HDMI_OUT_BASE_SFT: u32 = 4;
pub const AFE_HDMI_OUT_BASE_MASK: u32 = 0xfffffff;
pub const AFE_HDMI_OUT_BASE_MASK_SFT: u32 = (0xfffffff << 4);

/* AFE_HDMI_OUT_CUR_MSB */
pub const AFE_HDMI_OUT_CUR_MSB_SFT: u32 = 0;
pub const AFE_HDMI_OUT_CUR_MSB_MASK: u32 = 0x1ff;
pub const AFE_HDMI_OUT_CUR_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_HDMI_OUT_CUR */
pub const AFE_HDMI_OUT_CUR_SFT: u32 = 0;
pub const AFE_HDMI_OUT_CUR_MASK: u32 = 0xffffffff;
pub const AFE_HDMI_OUT_CUR_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_HDMI_OUT_END_MSB */
pub const AFE_HDMI_OUT_END_MSB_SFT: u32 = 0;
pub const AFE_HDMI_OUT_END_MSB_MASK: u32 = 0x1ff;
pub const AFE_HDMI_OUT_END_MSB_MASK_SFT: u32 = (0x1ff << 0);

/* AFE_HDMI_OUT_END */
pub const AFE_HDMI_OUT_END_SFT: u32 = 4;
pub const AFE_HDMI_OUT_END_MASK: u32 = 0xfffffff;
pub const AFE_HDMI_OUT_END_MASK_SFT: u32 = (0xfffffff << 4);
pub const AFE_HDMI_OUT_END_LSB_SFT: u32 = 0;
pub const AFE_HDMI_OUT_END_LSB_MASK: u32 = 0xf;
pub const AFE_HDMI_OUT_END_LSB_MASK_SFT: u32 = (0xf << 0);

/* AFE_HDMI_OUT_CON0 */
pub const HDMI_OUT_ON_SFT: u32 = 28;
pub const HDMI_OUT_ON_MASK: u32 = 0x1;
pub const HDMI_OUT_ON_MASK_SFT: u32 = (0x1 << 28);
pub const HDMI_CH_NUM_SFT: u32 = 24;
pub const HDMI_CH_NUM_MASK: u32 = 0xf;
pub const HDMI_CH_NUM_MASK_SFT: u32 = (0xf << 24);
pub const HDMI_OUT_ONE_HEART_SEL_SFT: u32 = 22;
pub const HDMI_OUT_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const HDMI_OUT_ONE_HEART_SEL_MASK_SFT: u32 = (0x3 << 22);
pub const HDMI_OUT_MINLEN_SFT: u32 = 20;
pub const HDMI_OUT_MINLEN_MASK: u32 = 0x3;
pub const HDMI_OUT_MINLEN_MASK_SFT: u32 = (0x3 << 20);
pub const HDMI_OUT_MAXLEN_SFT: u32 = 16;
pub const HDMI_OUT_MAXLEN_MASK: u32 = 0x3;
pub const HDMI_OUT_MAXLEN_MASK_SFT: u32 = (0x3 << 16);
pub const HDMI_OUT_SW_CLEAR_BUF_EMPTY_SFT: u32 = 15;
pub const HDMI_OUT_SW_CLEAR_BUF_EMPTY_MASK: u32 = 0x1;
pub const HDMI_OUT_SW_CLEAR_BUF_EMPTY_MASK_SFT: u32 = (0x1 << 15);
pub const HDMI_OUT_PBUF_SIZE_SFT: u32 = 12;
pub const HDMI_OUT_PBUF_SIZE_MASK: u32 = 0x3;
pub const HDMI_OUT_PBUF_SIZE_MASK_SFT: u32 = (0x3 << 12);
pub const HDMI_OUT_SW_CLEAR_HDMI_BUF_EMPTY_SFT: u32 = 7;
pub const HDMI_OUT_SW_CLEAR_HDMI_BUF_EMPTY_MASK: u32 = 0x1;
pub const HDMI_OUT_SW_CLEAR_HDMI_BUF_EMPTY_MASK_SFT: u32 = (0x1 << 7);
pub const HDMI_OUT_NORMAL_MODE_SFT: u32 = 5;
pub const HDMI_OUT_NORMAL_MODE_MASK: u32 = 0x1;
pub const HDMI_OUT_NORMAL_MODE_MASK_SFT: u32 = (0x1 << 5);
pub const HDMI_OUT_HALIGN_SFT: u32 = 4;
pub const HDMI_OUT_HALIGN_MASK: u32 = 0x1;
pub const HDMI_OUT_HALIGN_MASK_SFT: u32 = (0x1 << 4);
pub const HDMI_OUT_HD_MODE_SFT: u32 = 0;
pub const HDMI_OUT_HD_MODE_MASK: u32 = 0x3;
pub const HDMI_OUT_HD_MODE_MASK_SFT: u32 = (0x3 << 0);

/* AFE_VUL24_RCH_MON */
pub const VUL24_RCH_DATA_SFT: u32 = 0;
pub const VUL24_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL24_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL24_LCH_MON */
pub const VUL24_LCH_DATA_SFT: u32 = 0;
pub const VUL24_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL24_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL25_RCH_MON */
pub const VUL25_RCH_DATA_SFT: u32 = 0;
pub const VUL25_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL25_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL25_LCH_MON */
pub const VUL25_LCH_DATA_SFT: u32 = 0;
pub const VUL25_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL25_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL26_RCH_MON */
pub const VUL26_RCH_DATA_SFT: u32 = 0;
pub const VUL26_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL26_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL26_LCH_MON */
pub const VUL26_LCH_DATA_SFT: u32 = 0;
pub const VUL26_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL26_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL_CM0_RCH_MON */
pub const VUL_CM0_RCH_DATA_SFT: u32 = 0;
pub const VUL_CM0_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL_CM0_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL_CM0_LCH_MON */
pub const VUL_CM0_LCH_DATA_SFT: u32 = 0;
pub const VUL_CM0_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL_CM0_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL_CM1_RCH_MON */
pub const VUL_CM1_RCH_DATA_SFT: u32 = 0;
pub const VUL_CM1_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL_CM1_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL_CM1_LCH_MON */
pub const VUL_CM1_LCH_DATA_SFT: u32 = 0;
pub const VUL_CM1_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL_CM1_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL_CM2_RCH_MON */
pub const VUL_CM2_RCH_DATA_SFT: u32 = 0;
pub const VUL_CM2_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL_CM2_RCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_VUL_CM2_LCH_MON */
pub const VUL_CM2_LCH_DATA_SFT: u32 = 0;
pub const VUL_CM2_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL_CM2_LCH_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL_4CH_CH0_MON */
pub const DL_4CH_CH0_DATA_SFT: u32 = 0;
pub const DL_4CH_CH0_DATA_MASK: u32 = 0xffffffff;
pub const DL_4CH_CH0_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL_4CH_CH1_MON */
pub const DL_4CH_CH1_DATA_SFT: u32 = 0;
pub const DL_4CH_CH1_DATA_MASK: u32 = 0xffffffff;
pub const DL_4CH_CH1_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL_4CH_CH2_MON */
pub const DL_4CH_CH2_DATA_SFT: u32 = 0;
pub const DL_4CH_CH2_DATA_MASK: u32 = 0xffffffff;
pub const DL_4CH_CH2_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL_4CH_CH3_MON */
pub const DL_4CH_CH3_DATA_SFT: u32 = 0;
pub const DL_4CH_CH3_DATA_MASK: u32 = 0xffffffff;
pub const DL_4CH_CH3_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL_24CH_CH0_MON */
pub const DL_24CH_CH0_DATA_SFT: u32 = 0;
pub const DL_24CH_CH0_DATA_MASK: u32 = 0xffffffff;
pub const DL_24CH_CH0_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL_24CH_CH1_MON */
pub const DL_24CH_CH1_DATA_SFT: u32 = 0;
pub const DL_24CH_CH1_DATA_MASK: u32 = 0xffffffff;
pub const DL_24CH_CH1_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL_24CH_CH2_MON */
pub const DL_24CH_CH2_DATA_SFT: u32 = 0;
pub const DL_24CH_CH2_DATA_MASK: u32 = 0xffffffff;
pub const DL_24CH_CH2_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL_24CH_CH3_MON */
pub const DL_24CH_CH3_DATA_SFT: u32 = 0;
pub const DL_24CH_CH3_DATA_MASK: u32 = 0xffffffff;
pub const DL_24CH_CH3_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL_24CH_CH4_MON */
pub const DL_24CH_CH4_DATA_SFT: u32 = 0;
pub const DL_24CH_CH4_DATA_MASK: u32 = 0xffffffff;
pub const DL_24CH_CH4_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL_24CH_CH5_MON */
pub const DL_24CH_CH5_DATA_SFT: u32 = 0;
pub const DL_24CH_CH5_DATA_MASK: u32 = 0xffffffff;
pub const DL_24CH_CH5_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL_24CH_CH6_MON */
pub const DL_24CH_CH6_DATA_SFT: u32 = 0;
pub const DL_24CH_CH6_DATA_MASK: u32 = 0xffffffff;
pub const DL_24CH_CH6_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL_24CH_CH7_MON */
pub const DL_24CH_CH7_DATA_SFT: u32 = 0;
pub const DL_24CH_CH7_DATA_MASK: u32 = 0xffffffff;
pub const DL_24CH_CH7_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL_24CH_CH8_MON */
pub const DL_24CH_CH8_DATA_SFT: u32 = 0;
pub const DL_24CH_CH8_DATA_MASK: u32 = 0xffffffff;
pub const DL_24CH_CH8_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL_24CH_CH9_MON */
pub const DL_24CH_CH9_DATA_SFT: u32 = 0;
pub const DL_24CH_CH9_DATA_MASK: u32 = 0xffffffff;
pub const DL_24CH_CH9_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL_24CH_CH10_MON */
pub const DL_24CH_CH10_DATA_SFT: u32 = 0;
pub const DL_24CH_CH10_DATA_MASK: u32 = 0xffffffff;
pub const DL_24CH_CH10_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL_24CH_CH11_MON */
pub const DL_24CH_CH11_DATA_SFT: u32 = 0;
pub const DL_24CH_CH11_DATA_MASK: u32 = 0xffffffff;
pub const DL_24CH_CH11_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL_24CH_CH12_MON */
pub const DL_24CH_CH12_DATA_SFT: u32 = 0;
pub const DL_24CH_CH12_DATA_MASK: u32 = 0xffffffff;
pub const DL_24CH_CH12_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL_24CH_CH13_MON */
pub const DL_24CH_CH13_DATA_SFT: u32 = 0;
pub const DL_24CH_CH13_DATA_MASK: u32 = 0xffffffff;
pub const DL_24CH_CH13_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL_24CH_CH14_MON */
pub const DL_24CH_CH14_DATA_SFT: u32 = 0;
pub const DL_24CH_CH14_DATA_MASK: u32 = 0xffffffff;
pub const DL_24CH_CH14_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DL_24CH_CH15_MON */
pub const DL_24CH_CH15_DATA_SFT: u32 = 0;
pub const DL_24CH_CH15_DATA_MASK: u32 = 0xffffffff;
pub const DL_24CH_CH15_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_SRAM_BOUND */
pub const SECURE_BIT_SFT: u32 = 19;
pub const SECURE_BIT_MASK: u32 = 0x1;
pub const SECURE_BIT_MASK_SFT: u32 = (0x1 << 19);
pub const SECURE_SRAM_BOUND_SFT: u32 = 0;
pub const SECURE_SRAM_BOUND_MASK: u32 = 0x7ffff;
pub const SECURE_SRAM_BOUND_MASK_SFT: u32 = (0x7ffff << 0);

/* AFE_SECURE_CON0 */
pub const READ_EN15_NS_SFT: u32 = 31;
pub const READ_EN15_NS_MASK: u32 = 0x1;
pub const READ_EN15_NS_MASK_SFT: u32 = (0x1 << 31);
pub const WRITE_EN15_NS_SFT: u32 = 30;
pub const WRITE_EN15_NS_MASK: u32 = 0x1;
pub const WRITE_EN15_NS_MASK_SFT: u32 = (0x1 << 30);
pub const READ_EN14_NS_SFT: u32 = 29;
pub const READ_EN14_NS_MASK: u32 = 0x1;
pub const READ_EN14_NS_MASK_SFT: u32 = (0x1 << 29);
pub const WRITE_EN14_NS_SFT: u32 = 28;
pub const WRITE_EN14_NS_MASK: u32 = 0x1;
pub const WRITE_EN14_NS_MASK_SFT: u32 = (0x1 << 28);
pub const READ_EN13_NS_SFT: u32 = 27;
pub const READ_EN13_NS_MASK: u32 = 0x1;
pub const READ_EN13_NS_MASK_SFT: u32 = (0x1 << 27);
pub const WRITE_EN13_NS_SFT: u32 = 26;
pub const WRITE_EN13_NS_MASK: u32 = 0x1;
pub const WRITE_EN13_NS_MASK_SFT: u32 = (0x1 << 26);
pub const READ_EN12_NS_SFT: u32 = 25;
pub const READ_EN12_NS_MASK: u32 = 0x1;
pub const READ_EN12_NS_MASK_SFT: u32 = (0x1 << 25);
pub const WRITE_EN12_NS_SFT: u32 = 24;
pub const WRITE_EN12_NS_MASK: u32 = 0x1;
pub const WRITE_EN12_NS_MASK_SFT: u32 = (0x1 << 24);
pub const READ_EN11_NS_SFT: u32 = 23;
pub const READ_EN11_NS_MASK: u32 = 0x1;
pub const READ_EN11_NS_MASK_SFT: u32 = (0x1 << 23);
pub const WRITE_EN11_NS_SFT: u32 = 22;
pub const WRITE_EN11_NS_MASK: u32 = 0x1;
pub const WRITE_EN11_NS_MASK_SFT: u32 = (0x1 << 22);
pub const READ_EN10_NS_SFT: u32 = 21;
pub const READ_EN10_NS_MASK: u32 = 0x1;
pub const READ_EN10_NS_MASK_SFT: u32 = (0x1 << 21);
pub const WRITE_EN10_NS_SFT: u32 = 20;
pub const WRITE_EN10_NS_MASK: u32 = 0x1;
pub const WRITE_EN10_NS_MASK_SFT: u32 = (0x1 << 20);
pub const READ_EN9_NS_SFT: u32 = 19;
pub const READ_EN9_NS_MASK: u32 = 0x1;
pub const READ_EN9_NS_MASK_SFT: u32 = (0x1 << 19);
pub const WRITE_EN9_NS_SFT: u32 = 18;
pub const WRITE_EN9_NS_MASK: u32 = 0x1;
pub const WRITE_EN9_NS_MASK_SFT: u32 = (0x1 << 18);
pub const READ_EN8_NS_SFT: u32 = 17;
pub const READ_EN8_NS_MASK: u32 = 0x1;
pub const READ_EN8_NS_MASK_SFT: u32 = (0x1 << 17);
pub const WRITE_EN8_NS_SFT: u32 = 16;
pub const WRITE_EN8_NS_MASK: u32 = 0x1;
pub const WRITE_EN8_NS_MASK_SFT: u32 = (0x1 << 16);
pub const READ_EN7_NS_SFT: u32 = 15;
pub const READ_EN7_NS_MASK: u32 = 0x1;
pub const READ_EN7_NS_MASK_SFT: u32 = (0x1 << 15);
pub const WRITE_EN7_NS_SFT: u32 = 14;
pub const WRITE_EN7_NS_MASK: u32 = 0x1;
pub const WRITE_EN7_NS_MASK_SFT: u32 = (0x1 << 14);
pub const READ_EN6_NS_SFT: u32 = 13;
pub const READ_EN6_NS_MASK: u32 = 0x1;
pub const READ_EN6_NS_MASK_SFT: u32 = (0x1 << 13);
pub const WRITE_EN6_NS_SFT: u32 = 12;
pub const WRITE_EN6_NS_MASK: u32 = 0x1;
pub const WRITE_EN6_NS_MASK_SFT: u32 = (0x1 << 12);
pub const READ_EN5_NS_SFT: u32 = 11;
pub const READ_EN5_NS_MASK: u32 = 0x1;
pub const READ_EN5_NS_MASK_SFT: u32 = (0x1 << 11);
pub const WRITE_EN5_NS_SFT: u32 = 10;
pub const WRITE_EN5_NS_MASK: u32 = 0x1;
pub const WRITE_EN5_NS_MASK_SFT: u32 = (0x1 << 10);
pub const READ_EN4_NS_SFT: u32 = 9;
pub const READ_EN4_NS_MASK: u32 = 0x1;
pub const READ_EN4_NS_MASK_SFT: u32 = (0x1 << 9);
pub const WRITE_EN4_NS_SFT: u32 = 8;
pub const WRITE_EN4_NS_MASK: u32 = 0x1;
pub const WRITE_EN4_NS_MASK_SFT: u32 = (0x1 << 8);
pub const READ_EN3_NS_SFT: u32 = 7;
pub const READ_EN3_NS_MASK: u32 = 0x1;
pub const READ_EN3_NS_MASK_SFT: u32 = (0x1 << 7);
pub const WRITE_EN3_NS_SFT: u32 = 6;
pub const WRITE_EN3_NS_MASK: u32 = 0x1;
pub const WRITE_EN3_NS_MASK_SFT: u32 = (0x1 << 6);
pub const READ_EN2_NS_SFT: u32 = 5;
pub const READ_EN2_NS_MASK: u32 = 0x1;
pub const READ_EN2_NS_MASK_SFT: u32 = (0x1 << 5);
pub const WRITE_EN2_NS_SFT: u32 = 4;
pub const WRITE_EN2_NS_MASK: u32 = 0x1;
pub const WRITE_EN2_NS_MASK_SFT: u32 = (0x1 << 4);
pub const READ_EN1_NS_SFT: u32 = 3;
pub const READ_EN1_NS_MASK: u32 = 0x1;
pub const READ_EN1_NS_MASK_SFT: u32 = (0x1 << 3);
pub const WRITE_EN1_NS_SFT: u32 = 2;
pub const WRITE_EN1_NS_MASK: u32 = 0x1;
pub const WRITE_EN1_NS_MASK_SFT: u32 = (0x1 << 2);
pub const READ_EN0_NS_SFT: u32 = 1;
pub const READ_EN0_NS_MASK: u32 = 0x1;
pub const READ_EN0_NS_MASK_SFT: u32 = (0x1 << 1);
pub const WRITE_EN0_NS_SFT: u32 = 0;
pub const WRITE_EN0_NS_MASK: u32 = 0x1;
pub const WRITE_EN0_NS_MASK_SFT: u32 = (0x1 << 0);

/* AFE_SECURE_CON1 */
pub const READ_EN15_S_SFT: u32 = 31;
pub const READ_EN15_S_MASK: u32 = 0x1;
pub const READ_EN15_S_MASK_SFT: u32 = (0x1 << 31);
pub const WRITE_EN15_S_SFT: u32 = 30;
pub const WRITE_EN15_S_MASK: u32 = 0x1;
pub const WRITE_EN15_S_MASK_SFT: u32 = (0x1 << 30);
pub const READ_EN14_S_SFT: u32 = 29;
pub const READ_EN14_S_MASK: u32 = 0x1;
pub const READ_EN14_S_MASK_SFT: u32 = (0x1 << 29);
pub const WRITE_EN14_S_SFT: u32 = 28;
pub const WRITE_EN14_S_MASK: u32 = 0x1;
pub const WRITE_EN14_S_MASK_SFT: u32 = (0x1 << 28);
pub const READ_EN13_S_SFT: u32 = 27;
pub const READ_EN13_S_MASK: u32 = 0x1;
pub const READ_EN13_S_MASK_SFT: u32 = (0x1 << 27);
pub const WRITE_EN13_S_SFT: u32 = 26;
pub const WRITE_EN13_S_MASK: u32 = 0x1;
pub const WRITE_EN13_S_MASK_SFT: u32 = (0x1 << 26);
pub const READ_EN12_S_SFT: u32 = 25;
pub const READ_EN12_S_MASK: u32 = 0x1;
pub const READ_EN12_S_MASK_SFT: u32 = (0x1 << 25);
pub const WRITE_EN12_S_SFT: u32 = 24;
pub const WRITE_EN12_S_MASK: u32 = 0x1;
pub const WRITE_EN12_S_MASK_SFT: u32 = (0x1 << 24);
pub const READ_EN11_S_SFT: u32 = 23;
pub const READ_EN11_S_MASK: u32 = 0x1;
pub const READ_EN11_S_MASK_SFT: u32 = (0x1 << 23);
pub const WRITE_EN11_S_SFT: u32 = 22;
pub const WRITE_EN11_S_MASK: u32 = 0x1;
pub const WRITE_EN11_S_MASK_SFT: u32 = (0x1 << 22);
pub const READ_EN10_S_SFT: u32 = 21;
pub const READ_EN10_S_MASK: u32 = 0x1;
pub const READ_EN10_S_MASK_SFT: u32 = (0x1 << 21);
pub const WRITE_EN10_S_SFT: u32 = 20;
pub const WRITE_EN10_S_MASK: u32 = 0x1;
pub const WRITE_EN10_S_MASK_SFT: u32 = (0x1 << 20);
pub const READ_EN9_S_SFT: u32 = 19;
pub const READ_EN9_S_MASK: u32 = 0x1;
pub const READ_EN9_S_MASK_SFT: u32 = (0x1 << 19);
pub const WRITE_EN9_S_SFT: u32 = 18;
pub const WRITE_EN9_S_MASK: u32 = 0x1;
pub const WRITE_EN9_S_MASK_SFT: u32 = (0x1 << 18);
pub const READ_EN8_S_SFT: u32 = 17;
pub const READ_EN8_S_MASK: u32 = 0x1;
pub const READ_EN8_S_MASK_SFT: u32 = (0x1 << 17);
pub const WRITE_EN8_S_SFT: u32 = 16;
pub const WRITE_EN8_S_MASK: u32 = 0x1;
pub const WRITE_EN8_S_MASK_SFT: u32 = (0x1 << 16);
pub const READ_EN7_S_SFT: u32 = 15;
pub const READ_EN7_S_MASK: u32 = 0x1;
pub const READ_EN7_S_MASK_SFT: u32 = (0x1 << 15);
pub const WRITE_EN7_S_SFT: u32 = 14;
pub const WRITE_EN7_S_MASK: u32 = 0x1;
pub const WRITE_EN7_S_MASK_SFT: u32 = (0x1 << 14);
pub const READ_EN6_S_SFT: u32 = 13;
pub const READ_EN6_S_MASK: u32 = 0x1;
pub const READ_EN6_S_MASK_SFT: u32 = (0x1 << 13);
pub const WRITE_EN6_S_SFT: u32 = 12;
pub const WRITE_EN6_S_MASK: u32 = 0x1;
pub const WRITE_EN6_S_MASK_SFT: u32 = (0x1 << 12);
pub const READ_EN5_S_SFT: u32 = 11;
pub const READ_EN5_S_MASK: u32 = 0x1;
pub const READ_EN5_S_MASK_SFT: u32 = (0x1 << 11);
pub const WRITE_EN5_S_SFT: u32 = 10;
pub const WRITE_EN5_S_MASK: u32 = 0x1;
pub const WRITE_EN5_S_MASK_SFT: u32 = (0x1 << 10);
pub const READ_EN4_S_SFT: u32 = 9;
pub const READ_EN4_S_MASK: u32 = 0x1;
pub const READ_EN4_S_MASK_SFT: u32 = (0x1 << 9);
pub const WRITE_EN4_S_SFT: u32 = 8;
pub const WRITE_EN4_S_MASK: u32 = 0x1;
pub const WRITE_EN4_S_MASK_SFT: u32 = (0x1 << 8);
pub const READ_EN3_S_SFT: u32 = 7;
pub const READ_EN3_S_MASK: u32 = 0x1;
pub const READ_EN3_S_MASK_SFT: u32 = (0x1 << 7);
pub const WRITE_EN3_S_SFT: u32 = 6;
pub const WRITE_EN3_S_MASK: u32 = 0x1;
pub const WRITE_EN3_S_MASK_SFT: u32 = (0x1 << 6);
pub const READ_EN2_S_SFT: u32 = 5;
pub const READ_EN2_S_MASK: u32 = 0x1;
pub const READ_EN2_S_MASK_SFT: u32 = (0x1 << 5);
pub const WRITE_EN2_S_SFT: u32 = 4;
pub const WRITE_EN2_S_MASK: u32 = 0x1;
pub const WRITE_EN2_S_MASK_SFT: u32 = (0x1 << 4);
pub const READ_EN1_S_SFT: u32 = 3;
pub const READ_EN1_S_MASK: u32 = 0x1;
pub const READ_EN1_S_MASK_SFT: u32 = (0x1 << 3);
pub const WRITE_EN1_S_SFT: u32 = 2;
pub const WRITE_EN1_S_MASK: u32 = 0x1;
pub const WRITE_EN1_S_MASK_SFT: u32 = (0x1 << 2);
pub const READ_EN0_S_SFT: u32 = 1;
pub const READ_EN0_S_MASK: u32 = 0x1;
pub const READ_EN0_S_MASK_SFT: u32 = (0x1 << 1);
pub const WRITE_EN0_S_SFT: u32 = 0;
pub const WRITE_EN0_S_MASK: u32 = 0x1;
pub const WRITE_EN0_S_MASK_SFT: u32 = (0x1 << 0);

/* AFE_SE_SECURE_CON0 */
pub const AFE_HDMI_SE_SECURE_BIT_SFT: u32 = 11;
pub const AFE_HDMI_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_HDMI_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 11);
pub const AFE_SPDIF2_OUT_SE_SECURE_BIT_SFT: u32 = 10;
pub const AFE_SPDIF2_OUT_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_SPDIF2_OUT_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 10);
pub const AFE_SPDIF_OUT_SE_SECURE_BIT_SFT: u32 = 9;
pub const AFE_SPDIF_OUT_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_SPDIF_OUT_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 9);
pub const AFE_DL8_SE_SECURE_BIT_SFT: u32 = 8;
pub const AFE_DL8_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL8_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 8);
pub const AFE_DL7_SE_SECURE_BIT_SFT: u32 = 7;
pub const AFE_DL7_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL7_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 7);
pub const AFE_DL6_SE_SECURE_BIT_SFT: u32 = 6;
pub const AFE_DL6_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL6_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 6);
pub const AFE_DL5_SE_SECURE_BIT_SFT: u32 = 5;
pub const AFE_DL5_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL5_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 5);
pub const AFE_DL4_SE_SECURE_BIT_SFT: u32 = 4;
pub const AFE_DL4_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL4_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 4);
pub const AFE_DL3_SE_SECURE_BIT_SFT: u32 = 3;
pub const AFE_DL3_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL3_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 3);
pub const AFE_DL2_SE_SECURE_BIT_SFT: u32 = 2;
pub const AFE_DL2_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL2_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 2);
pub const AFE_DL1_SE_SECURE_BIT_SFT: u32 = 1;
pub const AFE_DL1_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL1_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 1);
pub const AFE_DL0_SE_SECURE_BIT_SFT: u32 = 0;
pub const AFE_DL0_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL0_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 0);

/* AFE_SE_SECURE_CON1 */
pub const AFE_DL46_SE_SECURE_BIT_SFT: u32 = 26;
pub const AFE_DL46_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL46_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 26);
pub const AFE_DL45_SE_SECURE_BIT_SFT: u32 = 25;
pub const AFE_DL45_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL45_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 25);
pub const AFE_DL44_SE_SECURE_BIT_SFT: u32 = 24;
pub const AFE_DL44_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL44_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 24);
pub const AFE_DL43_SE_SECURE_BIT_SFT: u32 = 23;
pub const AFE_DL43_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL43_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 23);
pub const AFE_DL42_SE_SECURE_BIT_SFT: u32 = 22;
pub const AFE_DL42_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL42_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 22);
pub const AFE_DL41_SE_SECURE_BIT_SFT: u32 = 21;
pub const AFE_DL41_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL41_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 21);
pub const AFE_DL40_SE_SECURE_BIT_SFT: u32 = 20;
pub const AFE_DL40_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL40_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 20);
pub const AFE_DL39_SE_SECURE_BIT_SFT: u32 = 19;
pub const AFE_DL39_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL39_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 19);
pub const AFE_DL38_SE_SECURE_BIT_SFT: u32 = 18;
pub const AFE_DL38_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL38_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 18);
pub const AFE_DL37_SE_SECURE_BIT_SFT: u32 = 17;
pub const AFE_DL37_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL37_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 17);
pub const AFE_DL36_SE_SECURE_BIT_SFT: u32 = 16;
pub const AFE_DL36_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL36_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 16);
pub const AFE_DL35_SE_SECURE_BIT_SFT: u32 = 15;
pub const AFE_DL35_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL35_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 15);
pub const AFE_DL34_SE_SECURE_BIT_SFT: u32 = 14;
pub const AFE_DL34_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL34_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 14);
pub const AFE_DL33_SE_SECURE_BIT_SFT: u32 = 13;
pub const AFE_DL33_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL33_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 13);
pub const AFE_DL32_SE_SECURE_BIT_SFT: u32 = 12;
pub const AFE_DL32_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL32_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 12);
pub const AFE_DL31_SE_SECURE_BIT_SFT: u32 = 11;
pub const AFE_DL31_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL31_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 11);
pub const AFE_DL30_SE_SECURE_BIT_SFT: u32 = 10;
pub const AFE_DL30_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL30_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 10);
pub const AFE_DL29_SE_SECURE_BIT_SFT: u32 = 9;
pub const AFE_DL29_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL29_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 9);
pub const AFE_DL28_SE_SECURE_BIT_SFT: u32 = 8;
pub const AFE_DL28_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL28_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 8);
pub const AFE_DL27_SE_SECURE_BIT_SFT: u32 = 7;
pub const AFE_DL27_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL27_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 7);
pub const AFE_DL26_SE_SECURE_BIT_SFT: u32 = 6;
pub const AFE_DL26_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL26_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 6);
pub const AFE_DL25_SE_SECURE_BIT_SFT: u32 = 5;
pub const AFE_DL25_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL25_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 5);
pub const AFE_DL24_SE_SECURE_BIT_SFT: u32 = 4;
pub const AFE_DL24_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL24_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 4);
pub const AFE_DL23_SE_SECURE_BIT_SFT: u32 = 3;
pub const AFE_DL23_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL23_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 3);
pub const AFE_DL_48CH_SE_SECURE_BIT_SFT: u32 = 2;
pub const AFE_DL_48CH_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL_48CH_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 2);
pub const AFE_DL_24CH_SE_SECURE_BIT_SFT: u32 = 1;
pub const AFE_DL_24CH_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL_24CH_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 1);
pub const AFE_DL_4CH_SE_SECURE_BIT_SFT: u32 = 0;
pub const AFE_DL_4CH_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL_4CH_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 0);

/* AFE_SE_SECURE_CON2 */
pub const AFE_VUL38_SE_SECURE_BIT_SFT: u32 = 28;
pub const AFE_VUL38_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL38_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 28);
pub const AFE_VUL37_SE_SECURE_BIT_SFT: u32 = 27;
pub const AFE_VUL37_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL37_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 27);
pub const AFE_VUL36_SE_SECURE_BIT_SFT: u32 = 26;
pub const AFE_VUL36_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL36_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 26);
pub const AFE_VUL35_SE_SECURE_BIT_SFT: u32 = 25;
pub const AFE_VUL35_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL35_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 25);
pub const AFE_VUL34_SE_SECURE_BIT_SFT: u32 = 24;
pub const AFE_VUL34_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL34_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 24);
pub const AFE_VUL33_SE_SECURE_BIT_SFT: u32 = 23;
pub const AFE_VUL33_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL33_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 23);
pub const AFE_VUL32_SE_SECURE_BIT_SFT: u32 = 22;
pub const AFE_VUL32_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL32_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 22);
pub const AFE_VUL31_SE_SECURE_BIT_SFT: u32 = 21;
pub const AFE_VUL31_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL31_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 21);
pub const AFE_VUL30_SE_SECURE_BIT_SFT: u32 = 20;
pub const AFE_VUL30_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL30_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 20);
pub const AFE_VUL29_SE_SECURE_BIT_SFT: u32 = 19;
pub const AFE_VUL29_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL29_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 19);
pub const AFE_VUL28_SE_SECURE_BIT_SFT: u32 = 18;
pub const AFE_VUL28_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL28_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 18);
pub const AFE_VUL27_SE_SECURE_BIT_SFT: u32 = 17;
pub const AFE_VUL27_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL27_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 17);
pub const AFE_VUL26_SE_SECURE_BIT_SFT: u32 = 16;
pub const AFE_VUL26_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL26_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 16);
pub const AFE_VUL25_SE_SECURE_BIT_SFT: u32 = 15;
pub const AFE_VUL25_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL25_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 15);
pub const AFE_VUL24_SE_SECURE_BIT_SFT: u32 = 14;
pub const AFE_VUL24_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL24_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 14);
pub const AFE_VUL_CM2_SE_SECURE_BIT_SFT: u32 = 13;
pub const AFE_VUL_CM2_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL_CM2_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 13);
pub const AFE_VUL_CM1_SE_SECURE_BIT_SFT: u32 = 12;
pub const AFE_VUL_CM1_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL_CM1_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 12);
pub const AFE_VUL_CM0_SE_SECURE_BIT_SFT: u32 = 11;
pub const AFE_VUL_CM0_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL_CM0_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 11);
pub const AFE_VUL10_SE_SECURE_BIT_SFT: u32 = 10;
pub const AFE_VUL10_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL10_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 10);
pub const AFE_VUL9_SE_SECURE_BIT_SFT: u32 = 9;
pub const AFE_VUL9_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL9_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 9);
pub const AFE_VUL8_SE_SECURE_BIT_SFT: u32 = 8;
pub const AFE_VUL8_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL8_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 8);
pub const AFE_VUL7_SE_SECURE_BIT_SFT: u32 = 7;
pub const AFE_VUL7_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL7_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 7);
pub const AFE_VUL6_SE_SECURE_BIT_SFT: u32 = 6;
pub const AFE_VUL6_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL6_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 6);
pub const AFE_VUL5_SE_SECURE_BIT_SFT: u32 = 5;
pub const AFE_VUL5_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL5_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 5);
pub const AFE_VUL4_SE_SECURE_BIT_SFT: u32 = 4;
pub const AFE_VUL4_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL4_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 4);
pub const AFE_VUL3_SE_SECURE_BIT_SFT: u32 = 3;
pub const AFE_VUL3_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL3_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 3);
pub const AFE_VUL2_SE_SECURE_BIT_SFT: u32 = 2;
pub const AFE_VUL2_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL2_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 2);
pub const AFE_VUL1_SE_SECURE_BIT_SFT: u32 = 1;
pub const AFE_VUL1_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL1_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 1);
pub const AFE_VUL0_SE_SECURE_BIT_SFT: u32 = 0;
pub const AFE_VUL0_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL0_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 0);

/* AFE_SE_SECURE_CON3 */
pub const AFE_SPDIFIN_SE_SECURE_BIT_SFT: u32 = 10;
pub const AFE_SPDIFIN_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_SPDIFIN_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 10);
pub const AFE_TDM_IN_SE_SECURE_BIT_SFT: u32 = 9;
pub const AFE_TDM_IN_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_TDM_IN_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 9);
pub const AFE_MPHONE_EARC_SE_SECURE_BIT_SFT: u32 = 8;
pub const AFE_MPHONE_EARC_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_MPHONE_EARC_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 8);
pub const AFE_MPHONE_SPDIF_SE_SECURE_BIT_SFT: u32 = 7;
pub const AFE_MPHONE_SPDIF_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_MPHONE_SPDIF_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 7);
pub const AFE_ETDM_IN6_SE_SECURE_BIT_SFT: u32 = 6;
pub const AFE_ETDM_IN6_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_ETDM_IN6_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 6);
pub const AFE_ETDM_IN5_SE_SECURE_BIT_SFT: u32 = 5;
pub const AFE_ETDM_IN5_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_ETDM_IN5_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 5);
pub const AFE_ETDM_IN4_SE_SECURE_BIT_SFT: u32 = 4;
pub const AFE_ETDM_IN4_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_ETDM_IN4_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 4);
pub const AFE_ETDM_IN3_SE_SECURE_BIT_SFT: u32 = 3;
pub const AFE_ETDM_IN3_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_ETDM_IN3_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 3);
pub const AFE_ETDM_IN2_SE_SECURE_BIT_SFT: u32 = 2;
pub const AFE_ETDM_IN2_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_ETDM_IN2_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 2);
pub const AFE_ETDM_IN1_SE_SECURE_BIT_SFT: u32 = 1;
pub const AFE_ETDM_IN1_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_ETDM_IN1_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 1);
pub const AFE_ETDM_IN0_SE_SECURE_BIT_SFT: u32 = 0;
pub const AFE_ETDM_IN0_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_ETDM_IN0_SE_SECURE_BIT_MASK_SFT: u32 = (0x1 << 0);

/* AFE_SE_PROT_SIDEBAND0 */
pub const HDMI_HPROT_SFT: u32 = 11;
pub const HDMI_HPROT_MASK: u32 = 0x1;
pub const HDMI_HPROT_MASK_SFT: u32 = (0x1 << 11);
pub const SPDIF2_OUT_HPROT_SFT: u32 = 10;
pub const SPDIF2_OUT_HPROT_MASK: u32 = 0x1;
pub const SPDIF2_OUT_HPROT_MASK_SFT: u32 = (0x1 << 10);
pub const SPDIF_OUT_HPROT_SFT: u32 = 9;
pub const SPDIF_OUT_HPROT_MASK: u32 = 0x1;
pub const SPDIF_OUT_HPROT_MASK_SFT: u32 = (0x1 << 9);
pub const DL8_HPROT_SFT: u32 = 8;
pub const DL8_HPROT_MASK: u32 = 0x1;
pub const DL8_HPROT_MASK_SFT: u32 = (0x1 << 8);
pub const DL7_HPROT_SFT: u32 = 7;
pub const DL7_HPROT_MASK: u32 = 0x1;
pub const DL7_HPROT_MASK_SFT: u32 = (0x1 << 7);
pub const DL6_HPROT_SFT: u32 = 6;
pub const DL6_HPROT_MASK: u32 = 0x1;
pub const DL6_HPROT_MASK_SFT: u32 = (0x1 << 6);
pub const DL5_HPROT_SFT: u32 = 5;
pub const DL5_HPROT_MASK: u32 = 0x1;
pub const DL5_HPROT_MASK_SFT: u32 = (0x1 << 5);
pub const DL4_HPROT_SFT: u32 = 4;
pub const DL4_HPROT_MASK: u32 = 0x1;
pub const DL4_HPROT_MASK_SFT: u32 = (0x1 << 4);
pub const DL3_HPROT_SFT: u32 = 3;
pub const DL3_HPROT_MASK: u32 = 0x1;
pub const DL3_HPROT_MASK_SFT: u32 = (0x1 << 3);
pub const DL2_HPROT_SFT: u32 = 2;
pub const DL2_HPROT_MASK: u32 = 0x1;
pub const DL2_HPROT_MASK_SFT: u32 = (0x1 << 2);
pub const DL1_HPROT_SFT: u32 = 1;
pub const DL1_HPROT_MASK: u32 = 0x1;
pub const DL1_HPROT_MASK_SFT: u32 = (0x1 << 1);
pub const DL0_HPROT_SFT: u32 = 0;
pub const DL0_HPROT_MASK: u32 = 0x1;
pub const DL0_HPROT_MASK_SFT: u32 = (0x1 << 0);

/* AFE_SE_PROT_SIDEBAND1 */
pub const DL46_HPROT_SFT: u32 = 26;
pub const DL46_HPROT_MASK: u32 = 0x1;
pub const DL46_HPROT_MASK_SFT: u32 = (0x1 << 26);
pub const DL45_HPROT_SFT: u32 = 25;
pub const DL45_HPROT_MASK: u32 = 0x1;
pub const DL45_HPROT_MASK_SFT: u32 = (0x1 << 25);
pub const DL44_HPROT_SFT: u32 = 24;
pub const DL44_HPROT_MASK: u32 = 0x1;
pub const DL44_HPROT_MASK_SFT: u32 = (0x1 << 24);
pub const DL43_HPROT_SFT: u32 = 23;
pub const DL43_HPROT_MASK: u32 = 0x1;
pub const DL43_HPROT_MASK_SFT: u32 = (0x1 << 23);
pub const DL42_HPROT_SFT: u32 = 22;
pub const DL42_HPROT_MASK: u32 = 0x1;
pub const DL42_HPROT_MASK_SFT: u32 = (0x1 << 22);
pub const DL41_HPROT_SFT: u32 = 21;
pub const DL41_HPROT_MASK: u32 = 0x1;
pub const DL41_HPROT_MASK_SFT: u32 = (0x1 << 21);
pub const DL40_HPROT_SFT: u32 = 20;
pub const DL40_HPROT_MASK: u32 = 0x1;
pub const DL40_HPROT_MASK_SFT: u32 = (0x1 << 20);
pub const DL39_HPROT_SFT: u32 = 19;
pub const DL39_HPROT_MASK: u32 = 0x1;
pub const DL39_HPROT_MASK_SFT: u32 = (0x1 << 19);
pub const DL38_HPROT_SFT: u32 = 18;
pub const DL38_HPROT_MASK: u32 = 0x1;
pub const DL38_HPROT_MASK_SFT: u32 = (0x1 << 18);
pub const DL37_HPROT_SFT: u32 = 17;
pub const DL37_HPROT_MASK: u32 = 0x1;
pub const DL37_HPROT_MASK_SFT: u32 = (0x1 << 17);
pub const DL36_HPROT_SFT: u32 = 16;
pub const DL36_HPROT_MASK: u32 = 0x1;
pub const DL36_HPROT_MASK_SFT: u32 = (0x1 << 16);
pub const DL35_HPROT_SFT: u32 = 15;
pub const DL35_HPROT_MASK: u32 = 0x1;
pub const DL35_HPROT_MASK_SFT: u32 = (0x1 << 15);
pub const DL34_HPROT_SFT: u32 = 14;
pub const DL34_HPROT_MASK: u32 = 0x1;
pub const DL34_HPROT_MASK_SFT: u32 = (0x1 << 14);
pub const DL33_HPROT_SFT: u32 = 13;
pub const DL33_HPROT_MASK: u32 = 0x1;
pub const DL33_HPROT_MASK_SFT: u32 = (0x1 << 13);
pub const DL32_HPROT_SFT: u32 = 12;
pub const DL32_HPROT_MASK: u32 = 0x1;
pub const DL32_HPROT_MASK_SFT: u32 = (0x1 << 12);
pub const DL31_HPROT_SFT: u32 = 11;
pub const DL31_HPROT_MASK: u32 = 0x1;
pub const DL31_HPROT_MASK_SFT: u32 = (0x1 << 11);
pub const DL30_HPROT_SFT: u32 = 10;
pub const DL30_HPROT_MASK: u32 = 0x1;
pub const DL30_HPROT_MASK_SFT: u32 = (0x1 << 10);
pub const DL29_HPROT_SFT: u32 = 9;
pub const DL29_HPROT_MASK: u32 = 0x1;
pub const DL29_HPROT_MASK_SFT: u32 = (0x1 << 9);
pub const DL28_HPROT_SFT: u32 = 8;
pub const DL28_HPROT_MASK: u32 = 0x1;
pub const DL28_HPROT_MASK_SFT: u32 = (0x1 << 8);
pub const DL27_HPROT_SFT: u32 = 7;
pub const DL27_HPROT_MASK: u32 = 0x1;
pub const DL27_HPROT_MASK_SFT: u32 = (0x1 << 7);
pub const DL26_HPROT_SFT: u32 = 6;
pub const DL26_HPROT_MASK: u32 = 0x1;
pub const DL26_HPROT_MASK_SFT: u32 = (0x1 << 6);
pub const DL25_HPROT_SFT: u32 = 5;
pub const DL25_HPROT_MASK: u32 = 0x1;
pub const DL25_HPROT_MASK_SFT: u32 = (0x1 << 5);
pub const DL24_HPROT_SFT: u32 = 4;
pub const DL24_HPROT_MASK: u32 = 0x1;
pub const DL24_HPROT_MASK_SFT: u32 = (0x1 << 4);
pub const DL23_HPROT_SFT: u32 = 3;
pub const DL23_HPROT_MASK: u32 = 0x1;
pub const DL23_HPROT_MASK_SFT: u32 = (0x1 << 3);
pub const DL_48CH_PROT_SFT: u32 = 2;
pub const DL_48CH_PROT_MASK: u32 = 0x1;
pub const DL_48CH_PROT_MASK_SFT: u32 = (0x1 << 2);
pub const DL_24CH_PROT_SFT: u32 = 1;
pub const DL_24CH_PROT_MASK: u32 = 0x1;
pub const DL_24CH_PROT_MASK_SFT: u32 = (0x1 << 1);
pub const DL_4CH_PROT_SFT: u32 = 0;
pub const DL_4CH_PROT_MASK: u32 = 0x1;
pub const DL_4CH_PROT_MASK_SFT: u32 = (0x1 << 0);

/* AFE_SE_PROT_SIDEBAND2 */
pub const VUL38_HPROT_SFT: u32 = 28;
pub const VUL38_HPROT_MASK: u32 = 0x1;
pub const VUL38_HPROT_MASK_SFT: u32 = (0x1 << 28);
pub const VUL37_HPROT_SFT: u32 = 27;
pub const VUL37_HPROT_MASK: u32 = 0x1;
pub const VUL37_HPROT_MASK_SFT: u32 = (0x1 << 27);
pub const VUL36_HPROT_SFT: u32 = 26;
pub const VUL36_HPROT_MASK: u32 = 0x1;
pub const VUL36_HPROT_MASK_SFT: u32 = (0x1 << 26);
pub const VUL35_HPROT_SFT: u32 = 25;
pub const VUL35_HPROT_MASK: u32 = 0x1;
pub const VUL35_HPROT_MASK_SFT: u32 = (0x1 << 25);
pub const VUL34_HPROT_SFT: u32 = 24;
pub const VUL34_HPROT_MASK: u32 = 0x1;
pub const VUL34_HPROT_MASK_SFT: u32 = (0x1 << 24);
pub const VUL33_HPROT_SFT: u32 = 23;
pub const VUL33_HPROT_MASK: u32 = 0x1;
pub const VUL33_HPROT_MASK_SFT: u32 = (0x1 << 23);
pub const VUL32_HPROT_SFT: u32 = 22;
pub const VUL32_HPROT_MASK: u32 = 0x1;
pub const VUL32_HPROT_MASK_SFT: u32 = (0x1 << 22);
pub const VUL31_HPROT_SFT: u32 = 21;
pub const VUL31_HPROT_MASK: u32 = 0x1;
pub const VUL31_HPROT_MASK_SFT: u32 = (0x1 << 21);
pub const VUL30_HPROT_SFT: u32 = 20;
pub const VUL30_HPROT_MASK: u32 = 0x1;
pub const VUL30_HPROT_MASK_SFT: u32 = (0x1 << 20);
pub const VUL29_HPROT_SFT: u32 = 19;
pub const VUL29_HPROT_MASK: u32 = 0x1;
pub const VUL29_HPROT_MASK_SFT: u32 = (0x1 << 19);
pub const VUL28_HPROT_SFT: u32 = 18;
pub const VUL28_HPROT_MASK: u32 = 0x1;
pub const VUL28_HPROT_MASK_SFT: u32 = (0x1 << 18);
pub const VUL27_HPROT_SFT: u32 = 17;
pub const VUL27_HPROT_MASK: u32 = 0x1;
pub const VUL27_HPROT_MASK_SFT: u32 = (0x1 << 17);
pub const VUL26_HPROT_SFT: u32 = 16;
pub const VUL26_HPROT_MASK: u32 = 0x1;
pub const VUL26_HPROT_MASK_SFT: u32 = (0x1 << 16);
pub const VUL25_HPROT_SFT: u32 = 15;
pub const VUL25_HPROT_MASK: u32 = 0x1;
pub const VUL25_HPROT_MASK_SFT: u32 = (0x1 << 15);
pub const VUL24_HPROT_SFT: u32 = 14;
pub const VUL24_HPROT_MASK: u32 = 0x1;
pub const VUL24_HPROT_MASK_SFT: u32 = (0x1 << 14);
pub const VUL_CM2_HPROT_SFT: u32 = 13;
pub const VUL_CM2_HPROT_MASK: u32 = 0x1;
pub const VUL_CM2_HPROT_MASK_SFT: u32 = (0x1 << 13);
pub const VUL_CM1_HPROT_SFT: u32 = 12;
pub const VUL_CM1_HPROT_MASK: u32 = 0x1;
pub const VUL_CM1_HPROT_MASK_SFT: u32 = (0x1 << 12);
pub const VUL_CM0_HPROT_SFT: u32 = 11;
pub const VUL_CM0_HPROT_MASK: u32 = 0x1;
pub const VUL_CM0_HPROT_MASK_SFT: u32 = (0x1 << 11);
pub const VUL10_HPROT_SFT: u32 = 10;
pub const VUL10_HPROT_MASK: u32 = 0x1;
pub const VUL10_HPROT_MASK_SFT: u32 = (0x1 << 10);
pub const VUL9_HPROT_SFT: u32 = 9;
pub const VUL9_HPROT_MASK: u32 = 0x1;
pub const VUL9_HPROT_MASK_SFT: u32 = (0x1 << 9);
pub const VUL8_HPROT_SFT: u32 = 8;
pub const VUL8_HPROT_MASK: u32 = 0x1;
pub const VUL8_HPROT_MASK_SFT: u32 = (0x1 << 8);
pub const VUL7_HPROT_SFT: u32 = 7;
pub const VUL7_HPROT_MASK: u32 = 0x1;
pub const VUL7_HPROT_MASK_SFT: u32 = (0x1 << 7);
pub const VUL6_HPROT_SFT: u32 = 6;
pub const VUL6_HPROT_MASK: u32 = 0x1;
pub const VUL6_HPROT_MASK_SFT: u32 = (0x1 << 6);
pub const VUL5_HPROT_SFT: u32 = 5;
pub const VUL5_HPROT_MASK: u32 = 0x1;
pub const VUL5_HPROT_MASK_SFT: u32 = (0x1 << 5);
pub const VUL4_HPROT_SFT: u32 = 4;
pub const VUL4_HPROT_MASK: u32 = 0x1;
pub const VUL4_HPROT_MASK_SFT: u32 = (0x1 << 4);
pub const VUL3_HPROT_SFT: u32 = 3;
pub const VUL3_HPROT_MASK: u32 = 0x1;
pub const VUL3_HPROT_MASK_SFT: u32 = (0x1 << 3);
pub const VUL2_HPROT_SFT: u32 = 2;
pub const VUL2_HPROT_MASK: u32 = 0x1;
pub const VUL2_HPROT_MASK_SFT: u32 = (0x1 << 2);
pub const VUL1_HPROT_SFT: u32 = 1;
pub const VUL1_HPROT_MASK: u32 = 0x1;
pub const VUL1_HPROT_MASK_SFT: u32 = (0x1 << 1);
pub const VUL0_HPROT_SFT: u32 = 0;
pub const VUL0_HPROT_MASK: u32 = 0x1;
pub const VUL0_HPROT_MASK_SFT: u32 = (0x1 << 0);

/* AFE_SE_PROT_SIDEBAND3 */
pub const MPHONE_EARC_HPROT_SFT: u32 = 10;
pub const MPHONE_EARC_HPROT_MASK: u32 = 0x1;
pub const MPHONE_EARC_HPROT_MASK_SFT: u32 = (0x1 << 10);
pub const MPHONE_SPDIF_HPROT_SFT: u32 = 9;
pub const MPHONE_SPDIF_HPROT_MASK: u32 = 0x1;
pub const MPHONE_SPDIF_HPROT_MASK_SFT: u32 = (0x1 << 9);
pub const SPDIFIN_HPROT_SFT: u32 = 8;
pub const SPDIFIN_HPROT_MASK: u32 = 0x1;
pub const SPDIFIN_HPROT_MASK_SFT: u32 = (0x1 << 8);
pub const TDMIN_HPROT_SFT: u32 = 7;
pub const TDMIN_HPROT_MASK: u32 = 0x1;
pub const TDMIN_HPROT_MASK_SFT: u32 = (0x1 << 7);
pub const ETDM_IN6_HPROT_SFT: u32 = 6;
pub const ETDM_IN6_HPROT_MASK: u32 = 0x1;
pub const ETDM_IN6_HPROT_MASK_SFT: u32 = (0x1 << 6);
pub const ETDM_IN5_HPROT_SFT: u32 = 5;
pub const ETDM_IN5_HPROT_MASK: u32 = 0x1;
pub const ETDM_IN5_HPROT_MASK_SFT: u32 = (0x1 << 5);
pub const ETDM_IN4_HPROT_SFT: u32 = 4;
pub const ETDM_IN4_HPROT_MASK: u32 = 0x1;
pub const ETDM_IN4_HPROT_MASK_SFT: u32 = (0x1 << 4);
pub const ETDM_IN3_HPROT_SFT: u32 = 3;
pub const ETDM_IN3_HPROT_MASK: u32 = 0x1;
pub const ETDM_IN3_HPROT_MASK_SFT: u32 = (0x1 << 3);
pub const ETDM_IN2_HPROT_SFT: u32 = 2;
pub const ETDM_IN2_HPROT_MASK: u32 = 0x1;
pub const ETDM_IN2_HPROT_MASK_SFT: u32 = (0x1 << 2);
pub const ETDM_IN1_HPROT_SFT: u32 = 1;
pub const ETDM_IN1_HPROT_MASK: u32 = 0x1;
pub const ETDM_IN1_HPROT_MASK_SFT: u32 = (0x1 << 1);
pub const ETDM_IN0_HPROT_SFT: u32 = 0;
pub const ETDM_IN0_HPROT_MASK: u32 = 0x1;
pub const ETDM_IN0_HPROT_MASK_SFT: u32 = (0x1 << 0);

/* AFE_SE_DOMAIN_SIDEBAND0 */
pub const DL7_HDOMAIN_SFT: u32 = 28;
pub const DL7_HDOMAIN_MASK: u32 = 0xf;
pub const DL7_HDOMAIN_MASK_SFT: u32 = (0xf << 28);
pub const DL6_HDOMAIN_SFT: u32 = 24;
pub const DL6_HDOMAIN_MASK: u32 = 0xf;
pub const DL6_HDOMAIN_MASK_SFT: u32 = (0xf << 24);
pub const DL5_HDOMAIN_SFT: u32 = 20;
pub const DL5_HDOMAIN_MASK: u32 = 0xf;
pub const DL5_HDOMAIN_MASK_SFT: u32 = (0xf << 20);
pub const DL4_HDOMAIN_SFT: u32 = 16;
pub const DL4_HDOMAIN_MASK: u32 = 0xf;
pub const DL4_HDOMAIN_MASK_SFT: u32 = (0xf << 16);
pub const DL3_HDOMAIN_SFT: u32 = 12;
pub const DL3_HDOMAIN_MASK: u32 = 0xf;
pub const DL3_HDOMAIN_MASK_SFT: u32 = (0xf << 12);
pub const DL2_HDOMAIN_SFT: u32 = 8;
pub const DL2_HDOMAIN_MASK: u32 = 0xf;
pub const DL2_HDOMAIN_MASK_SFT: u32 = (0xf << 8);
pub const DL1_HDOMAIN_SFT: u32 = 4;
pub const DL1_HDOMAIN_MASK: u32 = 0xf;
pub const DL1_HDOMAIN_MASK_SFT: u32 = (0xf << 4);
pub const DL0_HDOMAIN_SFT: u32 = 0;
pub const DL0_HDOMAIN_MASK: u32 = 0xf;
pub const DL0_HDOMAIN_MASK_SFT: u32 = (0xf << 0);

/* AFE_SE_DOMAIN_SIDEBAND1 */
pub const DL_48CH_HDOMAIN_SFT: u32 = 24;
pub const DL_48CH_HDOMAIN_MASK: u32 = 0xf;
pub const DL_48CH_HDOMAIN_MASK_SFT: u32 = (0xf << 24);
pub const DL_24CH_HDOMAIN_SFT: u32 = 20;
pub const DL_24CH_HDOMAIN_MASK: u32 = 0xf;
pub const DL_24CH_HDOMAIN_MASK_SFT: u32 = (0xf << 20);
pub const DL_4CH_HDOMAIN_SFT: u32 = 16;
pub const DL_4CH_HDOMAIN_MASK: u32 = 0xf;
pub const DL_4CH_HDOMAIN_MASK_SFT: u32 = (0xf << 16);
pub const HDMI_HDOMAIN_SFT: u32 = 12;
pub const HDMI_HDOMAIN_MASK: u32 = 0xf;
pub const HDMI_HDOMAIN_MASK_SFT: u32 = (0xf << 12);
pub const SPDIF2_OUT_HDOMAIN_SFT: u32 = 8;
pub const SPDIF2_OUT_HDOMAIN_MASK: u32 = 0xf;
pub const SPDIF2_OUT_HDOMAIN_MASK_SFT: u32 = (0xf << 8);
pub const SPDIF_OUT_HDOMAIN_SFT: u32 = 4;
pub const SPDIF_OUT_HDOMAIN_MASK: u32 = 0xf;
pub const SPDIF_OUT_HDOMAIN_MASK_SFT: u32 = (0xf << 4);
pub const DL8_HDOMAIN_SFT: u32 = 0;
pub const DL8_HDOMAIN_MASK: u32 = 0xf;
pub const DL8_HDOMAIN_MASK_SFT: u32 = (0xf << 0);

/* AFE_SE_DOMAIN_SIDEBAND2 */
pub const DL30_HDOMAIN_SFT: u32 = 28;
pub const DL30_HDOMAIN_MASK: u32 = 0xf;
pub const DL30_HDOMAIN_MASK_SFT: u32 = (0xf << 28);
pub const DL29_HDOMAIN_SFT: u32 = 24;
pub const DL29_HDOMAIN_MASK: u32 = 0xf;
pub const DL29_HDOMAIN_MASK_SFT: u32 = (0xf << 24);
pub const DL28_HDOMAIN_SFT: u32 = 20;
pub const DL28_HDOMAIN_MASK: u32 = 0xf;
pub const DL28_HDOMAIN_MASK_SFT: u32 = (0xf << 20);
pub const DL27_HDOMAIN_SFT: u32 = 16;
pub const DL27_HDOMAIN_MASK: u32 = 0xf;
pub const DL27_HDOMAIN_MASK_SFT: u32 = (0xf << 16);
pub const DL26_HDOMAIN_SFT: u32 = 12;
pub const DL26_HDOMAIN_MASK: u32 = 0xf;
pub const DL26_HDOMAIN_MASK_SFT: u32 = (0xf << 12);
pub const DL25_HDOMAIN_SFT: u32 = 8;
pub const DL25_HDOMAIN_MASK: u32 = 0xf;
pub const DL25_HDOMAIN_MASK_SFT: u32 = (0xf << 8);
pub const DL24_HDOMAIN_SFT: u32 = 4;
pub const DL24_HDOMAIN_MASK: u32 = 0xf;
pub const DL24_HDOMAIN_MASK_SFT: u32 = (0xf << 4);
pub const DL23_HDOMAIN_SFT: u32 = 0;
pub const DL23_HDOMAIN_MASK: u32 = 0xf;
pub const DL23_HDOMAIN_MASK_SFT: u32 = (0xf << 0);

/* AFE_SE_DOMAIN_SIDEBAND3 */
pub const DL38_HDOMAIN_SFT: u32 = 28;
pub const DL38_HDOMAIN_MASK: u32 = 0xf;
pub const DL38_HDOMAIN_MASK_SFT: u32 = (0xf << 28);
pub const DL37_HDOMAIN_SFT: u32 = 24;
pub const DL37_HDOMAIN_MASK: u32 = 0xf;
pub const DL37_HDOMAIN_MASK_SFT: u32 = (0xf << 24);
pub const DL36_HDOMAIN_SFT: u32 = 20;
pub const DL36_HDOMAIN_MASK: u32 = 0xf;
pub const DL36_HDOMAIN_MASK_SFT: u32 = (0xf << 20);
pub const DL35_HDOMAIN_SFT: u32 = 16;
pub const DL35_HDOMAIN_MASK: u32 = 0xf;
pub const DL35_HDOMAIN_MASK_SFT: u32 = (0xf << 16);
pub const DL34_HDOMAIN_SFT: u32 = 12;
pub const DL34_HDOMAIN_MASK: u32 = 0xf;
pub const DL34_HDOMAIN_MASK_SFT: u32 = (0xf << 12);
pub const DL33_HDOMAIN_SFT: u32 = 8;
pub const DL33_HDOMAIN_MASK: u32 = 0xf;
pub const DL33_HDOMAIN_MASK_SFT: u32 = (0xf << 8);
pub const DL32_HDOMAIN_SFT: u32 = 4;
pub const DL32_HDOMAIN_MASK: u32 = 0xf;
pub const DL32_HDOMAIN_MASK_SFT: u32 = (0xf << 4);
pub const DL31_HDOMAIN_SFT: u32 = 0;
pub const DL31_HDOMAIN_MASK: u32 = 0xf;
pub const DL31_HDOMAIN_MASK_SFT: u32 = (0xf << 0);

/* AFE_SE_DOMAIN_SIDEBAND4 */
pub const DL46_HDOMAIN_SFT: u32 = 28;
pub const DL46_HDOMAIN_MASK: u32 = 0xf;
pub const DL46_HDOMAIN_MASK_SFT: u32 = (0xf << 28);
pub const DL45_HDOMAIN_SFT: u32 = 24;
pub const DL45_HDOMAIN_MASK: u32 = 0xf;
pub const DL45_HDOMAIN_MASK_SFT: u32 = (0xf << 24);
pub const DL44_HDOMAIN_SFT: u32 = 20;
pub const DL44_HDOMAIN_MASK: u32 = 0xf;
pub const DL44_HDOMAIN_MASK_SFT: u32 = (0xf << 20);
pub const DL43_HDOMAIN_SFT: u32 = 16;
pub const DL43_HDOMAIN_MASK: u32 = 0xf;
pub const DL43_HDOMAIN_MASK_SFT: u32 = (0xf << 16);
pub const DL42_HDOMAIN_SFT: u32 = 12;
pub const DL42_HDOMAIN_MASK: u32 = 0xf;
pub const DL42_HDOMAIN_MASK_SFT: u32 = (0xf << 12);
pub const DL41_HDOMAIN_SFT: u32 = 8;
pub const DL41_HDOMAIN_MASK: u32 = 0xf;
pub const DL41_HDOMAIN_MASK_SFT: u32 = (0xf << 8);
pub const DL40_HDOMAIN_SFT: u32 = 4;
pub const DL40_HDOMAIN_MASK: u32 = 0xf;
pub const DL40_HDOMAIN_MASK_SFT: u32 = (0xf << 4);
pub const DL39_HDOMAIN_SFT: u32 = 0;
pub const DL39_HDOMAIN_MASK: u32 = 0xf;
pub const DL39_HDOMAIN_MASK_SFT: u32 = (0xf << 0);

/* AFE_SE_DOMAIN_SIDEBAND5 */
pub const VUL7_HDOMAIN_SFT: u32 = 28;
pub const VUL7_HDOMAIN_MASK: u32 = 0xf;
pub const VUL7_HDOMAIN_MASK_SFT: u32 = (0xf << 28);
pub const VUL6_HDOMAIN_SFT: u32 = 24;
pub const VUL6_HDOMAIN_MASK: u32 = 0xf;
pub const VUL6_HDOMAIN_MASK_SFT: u32 = (0xf << 24);
pub const VUL5_HDOMAIN_SFT: u32 = 20;
pub const VUL5_HDOMAIN_MASK: u32 = 0xf;
pub const VUL5_HDOMAIN_MASK_SFT: u32 = (0xf << 20);
pub const VUL4_HDOMAIN_SFT: u32 = 16;
pub const VUL4_HDOMAIN_MASK: u32 = 0xf;
pub const VUL4_HDOMAIN_MASK_SFT: u32 = (0xf << 16);
pub const VUL3_HDOMAIN_SFT: u32 = 12;
pub const VUL3_HDOMAIN_MASK: u32 = 0xf;
pub const VUL3_HDOMAIN_MASK_SFT: u32 = (0xf << 12);
pub const VUL2_HDOMAIN_SFT: u32 = 8;
pub const VUL2_HDOMAIN_MASK: u32 = 0xf;
pub const VUL2_HDOMAIN_MASK_SFT: u32 = (0xf << 8);
pub const VUL1_HDOMAIN_SFT: u32 = 4;
pub const VUL1_HDOMAIN_MASK: u32 = 0xf;
pub const VUL1_HDOMAIN_MASK_SFT: u32 = (0xf << 4);
pub const VUL0_HDOMAIN_SFT: u32 = 0;
pub const VUL0_HDOMAIN_MASK: u32 = 0xf;
pub const VUL0_HDOMAIN_MASK_SFT: u32 = (0xf << 0);

/* AFE_SE_DOMAIN_SIDEBAND6 */
pub const VU25_HDOMAIN_SFT: u32 = 28;
pub const VU25_HDOMAIN_MASK: u32 = 0xf;
pub const VU25_HDOMAIN_MASK_SFT: u32 = (0xf << 28);
pub const VUL24_HDOMAIN_SFT: u32 = 24;
pub const VUL24_HDOMAIN_MASK: u32 = 0xf;
pub const VUL24_HDOMAIN_MASK_SFT: u32 = (0xf << 24);
pub const VUL_CM2_HDOMAIN_SFT: u32 = 20;
pub const VUL_CM2_HDOMAIN_MASK: u32 = 0xf;
pub const VUL_CM2_HDOMAIN_MASK_SFT: u32 = (0xf << 20);
pub const VUL_CM1_HDOMAIN_SFT: u32 = 16;
pub const VUL_CM1_HDOMAIN_MASK: u32 = 0xf;
pub const VUL_CM1_HDOMAIN_MASK_SFT: u32 = (0xf << 16);
pub const VUL_CM0_HDOMAIN_SFT: u32 = 12;
pub const VUL_CM0_HDOMAIN_MASK: u32 = 0xf;
pub const VUL_CM0_HDOMAIN_MASK_SFT: u32 = (0xf << 12);
pub const VUL10_HDOMAIN_SFT: u32 = 8;
pub const VUL10_HDOMAIN_MASK: u32 = 0xf;
pub const VUL10_HDOMAIN_MASK_SFT: u32 = (0xf << 8);
pub const VUL9_HDOMAIN_SFT: u32 = 4;
pub const VUL9_HDOMAIN_MASK: u32 = 0xf;
pub const VUL9_HDOMAIN_MASK_SFT: u32 = (0xf << 4);
pub const VUL8_HDOMAIN_SFT: u32 = 0;
pub const VUL8_HDOMAIN_MASK: u32 = 0xf;
pub const VUL8_HDOMAIN_MASK_SFT: u32 = (0xf << 0);

/* AFE_SE_DOMAIN_SIDEBAND7 */
pub const VUL33_HDOMAIN_SFT: u32 = 28;
pub const VUL33_HDOMAIN_MASK: u32 = 0xf;
pub const VUL33_HDOMAIN_MASK_SFT: u32 = (0xf << 28);
pub const VUL32_HDOMAIN_SFT: u32 = 24;
pub const VUL32_HDOMAIN_MASK: u32 = 0xf;
pub const VUL32_HDOMAIN_MASK_SFT: u32 = (0xf << 24);
pub const VUL31_HDOMAIN_SFT: u32 = 20;
pub const VUL31_HDOMAIN_MASK: u32 = 0xf;
pub const VUL31_HDOMAIN_MASK_SFT: u32 = (0xf << 20);
pub const VUL30_HDOMAIN_SFT: u32 = 16;
pub const VUL30_HDOMAIN_MASK: u32 = 0xf;
pub const VUL30_HDOMAIN_MASK_SFT: u32 = (0xf << 16);
pub const VUL29_HDOMAIN_SFT: u32 = 12;
pub const VUL29_HDOMAIN_MASK: u32 = 0xf;
pub const VUL29_HDOMAIN_MASK_SFT: u32 = (0xf << 12);
pub const VUL28_HDOMAIN_SFT: u32 = 8;
pub const VUL28_HDOMAIN_MASK: u32 = 0xf;
pub const VUL28_HDOMAIN_MASK_SFT: u32 = (0xf << 8);
pub const VUL27_HDOMAIN_SFT: u32 = 4;
pub const VUL27_HDOMAIN_MASK: u32 = 0xf;
pub const VUL27_HDOMAIN_MASK_SFT: u32 = (0xf << 4);
pub const VUL26_HDOMAIN_SFT: u32 = 0;
pub const VUL26_HDOMAIN_MASK: u32 = 0xf;
pub const VUL26_HDOMAIN_MASK_SFT: u32 = (0xf << 0);

/* AFE_SE_DOMAIN_SIDEBAND8 */
pub const ETDM_IN2_HDOMAIN_SFT: u32 = 28;
pub const ETDM_IN2_HDOMAIN_MASK: u32 = 0xf;
pub const ETDM_IN2_HDOMAIN_MASK_SFT: u32 = (0xf << 28);
pub const ETDM_IN1_HDOMAIN_SFT: u32 = 24;
pub const ETDM_IN1_HDOMAIN_MASK: u32 = 0xf;
pub const ETDM_IN1_HDOMAIN_MASK_SFT: u32 = (0xf << 24);
pub const ETDM_IN0_HDOMAIN_SFT: u32 = 20;
pub const ETDM_IN0_HDOMAIN_MASK: u32 = 0xf;
pub const ETDM_IN0_HDOMAIN_MASK_SFT: u32 = (0xf << 20);
pub const VUL38_HDOMAIN_SFT: u32 = 16;
pub const VUL38_HDOMAIN_MASK: u32 = 0xf;
pub const VUL38_HDOMAIN_MASK_SFT: u32 = (0xf << 16);
pub const VUL37_HDOMAIN_SFT: u32 = 12;
pub const VUL37_HDOMAIN_MASK: u32 = 0xf;
pub const VUL37_HDOMAIN_MASK_SFT: u32 = (0xf << 12);
pub const VUL36_HDOMAIN_SFT: u32 = 8;
pub const VUL36_HDOMAIN_MASK: u32 = 0xf;
pub const VUL36_HDOMAIN_MASK_SFT: u32 = (0xf << 8);
pub const VUL35_HDOMAIN_SFT: u32 = 4;
pub const VUL35_HDOMAIN_MASK: u32 = 0xf;
pub const VUL35_HDOMAIN_MASK_SFT: u32 = (0xf << 4);
pub const VUL34_HDOMAIN_SFT: u32 = 0;
pub const VUL34_HDOMAIN_MASK: u32 = 0xf;
pub const VUL34_HDOMAIN_MASK_SFT: u32 = (0xf << 0);

/* AFE_SE_DOMAIN_SIDEBAND9 */
pub const MPHONE_EARC_HDOMAIN_SFT: u32 = 28;
pub const MPHONE_EARC_HDOMAIN_MASK: u32 = 0xf;
pub const MPHONE_EARC_HDOMAIN_MASK_SFT: u32 = (0xf << 28);
pub const MPHONE_SPDIF_HDOMAIN_SFT: u32 = 24;
pub const MPHONE_SPDIF_HDOMAIN_MASK: u32 = 0xf;
pub const MPHONE_SPDIF_HDOMAIN_MASK_SFT: u32 = (0xf << 24);
pub const SPDIFIN_HDOMAIN_SFT: u32 = 20;
pub const SPDIFIN_HDOMAIN_MASK: u32 = 0xf;
pub const SPDIFIN_HDOMAIN_MASK_SFT: u32 = (0xf << 20);
pub const TDMIN_HDOMAIN_SFT: u32 = 16;
pub const TDMIN_HDOMAIN_MASK: u32 = 0xf;
pub const TDMIN_HDOMAIN_MASK_SFT: u32 = (0xf << 16);
pub const ETDM_IN6_HDOMAIN_SFT: u32 = 12;
pub const ETDM_IN6_HDOMAIN_MASK: u32 = 0xf;
pub const ETDM_IN6_HDOMAIN_MASK_SFT: u32 = (0xf << 12);
pub const ETDM_IN5_HDOMAIN_SFT: u32 = 8;
pub const ETDM_IN5_HDOMAIN_MASK: u32 = 0xf;
pub const ETDM_IN5_HDOMAIN_MASK_SFT: u32 = (0xf << 8);
pub const ETDM_IN4_HDOMAIN_SFT: u32 = 4;
pub const ETDM_IN4_HDOMAIN_MASK: u32 = 0xf;
pub const ETDM_IN4_HDOMAIN_MASK_SFT: u32 = (0xf << 4);
pub const ETDM_IN3_HDOMAIN_SFT: u32 = 0;
pub const ETDM_IN3_HDOMAIN_MASK: u32 = 0xf;
pub const ETDM_IN3_HDOMAIN_MASK_SFT: u32 = (0xf << 0);

/* AFE_PROT_SIDEBAND0_MON */
pub const AFE_DOMAIN_SIDEBAN0_MON_SFT: u32 = 0;
pub const AFE_DOMAIN_SIDEBAN0_MON_MASK: u32 = 0xffffffff;
pub const AFE_DOMAIN_SIDEBAN0_MON_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_PROT_SIDEBAND1_MON */
pub const AFE_DOMAIN_SIDEBAN1_MON_SFT: u32 = 0;
pub const AFE_DOMAIN_SIDEBAN1_MON_MASK: u32 = 0xffffffff;
pub const AFE_DOMAIN_SIDEBAN1_MON_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_PROT_SIDEBAND2_MON */
pub const AFE_DOMAIN_SIDEBAN2_MON_SFT: u32 = 0;
pub const AFE_DOMAIN_SIDEBAN2_MON_MASK: u32 = 0xffffffff;
pub const AFE_DOMAIN_SIDEBAN2_MON_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_PROT_SIDEBAND3_MON */
pub const AFE_DOMAIN_SIDEBAN3_MON_SFT: u32 = 0;
pub const AFE_DOMAIN_SIDEBAN3_MON_MASK: u32 = 0xffffffff;
pub const AFE_DOMAIN_SIDEBAN3_MON_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_DOMAIN_SIDEBAND0_MON */
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN0_MON_SFT 0
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN0_MON_MASK 0xffffffff
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN0_MON_MASK_SFT (0xffffffff << 0)

/* AFE_DOMAIN_SIDEBAND1_MON */
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN1_MON_SFT 0
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN1_MON_MASK 0xffffffff
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN1_MON_MASK_SFT (0xffffffff << 0)

/* AFE_DOMAIN_SIDEBAND2_MON */
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN2_MON_SFT 0
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN2_MON_MASK 0xffffffff
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN2_MON_MASK_SFT (0xffffffff << 0)

/* AFE_DOMAIN_SIDEBAND3_MON */
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN3_MON_SFT 0
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN3_MON_MASK 0xffffffff
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN3_MON_MASK_SFT (0xffffffff << 0)

/* AFE_DOMAIN_SIDEBAND4_MON */
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN0_MON_SFT 0
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN0_MON_MASK 0xffffffff
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN0_MON_MASK_SFT (0xffffffff << 0)

/* AFE_DOMAIN_SIDEBAND5_MON */
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN1_MON_SFT 0
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN1_MON_MASK 0xffffffff
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN1_MON_MASK_SFT (0xffffffff << 0)

/* AFE_DOMAIN_SIDEBAND6_MON */
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN2_MON_SFT 0
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN2_MON_MASK 0xffffffff
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN2_MON_MASK_SFT (0xffffffff << 0)

/* AFE_DOMAIN_SIDEBAND7_MON */
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN3_MON_SFT 0
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN3_MON_MASK 0xffffffff
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN3_MON_MASK_SFT (0xffffffff << 0)

/* AFE_DOMAIN_SIDEBAND8_MON */
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN2_MON_SFT 0
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN2_MON_MASK 0xffffffff
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN2_MON_MASK_SFT (0xffffffff << 0)

/* AFE_DOMAIN_SIDEBAND9_MON */
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN3_MON_SFT 0
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN3_MON_MASK 0xffffffff
// duplicate C define preserved: #define AFE_DOMAIN_SIDEBAN3_MON_MASK_SFT (0xffffffff << 0)

/* AFE_SECURE_CONN0 */
pub const AFE_SPDIFIN_LPBK_CON_MASK_S_SFT: u32 = 26;
pub const AFE_SPDIFIN_LPBK_CON_MASK_S_MASK: u32 = 0x3;
pub const AFE_SPDIFIN_LPBK_CON_MASK_S_MASK_SFT: u32 = (0x3 << 26);
pub const AFE_ADDA_DMIC1_SRC_CON0_MASK_S_SFT: u32 = 25;
pub const AFE_ADDA_DMIC1_SRC_CON0_MASK_S_MASK: u32 = 0x1;
pub const AFE_ADDA_DMIC1_SRC_CON0_MASK_S_MASK_SFT: u32 = (0x1 << 25);
pub const AFE_ADDA_DMIC0_SRC_CON0_MASK_S_SFT: u32 = 24;
pub const AFE_ADDA_DMIC0_SRC_CON0_MASK_S_MASK: u32 = 0x1;
pub const AFE_ADDA_DMIC0_SRC_CON0_MASK_S_MASK_SFT: u32 = (0x1 << 24);
pub const AFE_ADDA_UL3_SRC_CON0_MASK_S_SFT: u32 = 23;
pub const AFE_ADDA_UL3_SRC_CON0_MASK_S_MASK: u32 = 0x1;
pub const AFE_ADDA_UL3_SRC_CON0_MASK_S_MASK_SFT: u32 = (0x1 << 23);
pub const AFE_ADDA_UL2_SRC_CON0_MASK_S_SFT: u32 = 22;
pub const AFE_ADDA_UL2_SRC_CON0_MASK_S_MASK: u32 = 0x1;
pub const AFE_ADDA_UL2_SRC_CON0_MASK_S_MASK_SFT: u32 = (0x1 << 22);
pub const AFE_ADDA_UL1_SRC_CON0_MASK_S_SFT: u32 = 21;
pub const AFE_ADDA_UL1_SRC_CON0_MASK_S_MASK: u32 = 0x1;
pub const AFE_ADDA_UL1_SRC_CON0_MASK_S_MASK_SFT: u32 = (0x1 << 21);
pub const AFE_ADDA_UL0_SRC_CON0_MASK_S_SFT: u32 = 20;
pub const AFE_ADDA_UL0_SRC_CON0_MASK_S_MASK: u32 = 0x1;
pub const AFE_ADDA_UL0_SRC_CON0_MASK_S_MASK_SFT: u32 = (0x1 << 20);
pub const AFE_MRKAIF1_CFG0_MASK_S_SFT: u32 = 19;
pub const AFE_MRKAIF1_CFG0_MASK_S_MASK: u32 = 0x1;
pub const AFE_MRKAIF1_CFG0_MASK_S_MASK_SFT: u32 = (0x1 << 19);
pub const AFE_MRKAIF0_CFG0_MASK_S_SFT: u32 = 18;
pub const AFE_MRKAIF0_CFG0_MASK_S_MASK: u32 = 0x1;
pub const AFE_MRKAIF0_CFG0_MASK_S_MASK_SFT: u32 = (0x1 << 18);
pub const AFE_TDMIN_CON1_MASK_S_SFT: u32 = 17;
pub const AFE_TDMIN_CON1_MASK_S_MASK: u32 = 0x1;
pub const AFE_TDMIN_CON1_MASK_S_MASK_SFT: u32 = (0x1 << 17);
pub const AFE_TDM_CON2_MASK_S_SFT: u32 = 16;
pub const AFE_TDM_CON2_MASK_S_MASK: u32 = 0x1;
pub const AFE_TDM_CON2_MASK_S_MASK_SFT: u32 = (0x1 << 16);
pub const AFE_DAIBT_CON_MASK_S_SFT: u32 = 14;
pub const AFE_DAIBT_CON_MASK_S_MASK: u32 = 0x3;
pub const AFE_DAIBT_CON_MASK_S_MASK_SFT: u32 = (0x3 << 14);
pub const AFE_MRGIF_CON_MASK_S_SFT: u32 = 12;
pub const AFE_MRGIF_CON_MASK_S_MASK: u32 = 0x3;
pub const AFE_MRGIF_CON_MASK_S_MASK_SFT: u32 = (0x3 << 12);
pub const AFE_CONNSYS_I2S_CON_MASK_S_SFT: u32 = 11;
pub const AFE_CONNSYS_I2S_CON_MASK_S_MASK: u32 = 0x1;
pub const AFE_CONNSYS_I2S_CON_MASK_S_MASK_SFT: u32 = (0x1 << 11);
pub const AFE_PCM1_INFT_CON0_MASK_S_SFT: u32 = 6;
pub const AFE_PCM1_INFT_CON0_MASK_S_MASK: u32 = 0x1f;
pub const AFE_PCM1_INFT_CON0_MASK_S_MASK_SFT: u32 = (0x1f << 6);
pub const AFE_PCM0_INTF_CON1_MASK_S_SFT: u32 = 0;
pub const AFE_PCM0_INTF_CON1_MASK_S_MASK: u32 = 0x3f;
pub const AFE_PCM0_INTF_CON1_MASK_S_MASK_SFT: u32 = (0x3f << 0);

/* AFE_SECURE_CONN_ETDM0 */
pub const ETDM_0_3_COWORK_CON2_OUT3_DATA_SEL_SFT: u32 = 28;
pub const ETDM_0_3_COWORK_CON2_OUT3_DATA_SEL_MASK: u32 = 0xf;
pub const ETDM_0_3_COWORK_CON2_OUT3_DATA_SEL_MASK_SFT: u32 = (0xf << 28);
pub const ETDM_0_3_COWORK_CON2_OUT2_DATA_SEL_SFT: u32 = 24;
pub const ETDM_0_3_COWORK_CON2_OUT2_DATA_SEL_MASK: u32 = 0xf;
pub const ETDM_0_3_COWORK_CON2_OUT2_DATA_SEL_MASK_SFT: u32 = (0xf << 24);
pub const ETDM_0_3_COWORK_CON2_IN1_SDATA1_15_SEL_SFT: u32 = 20;
pub const ETDM_0_3_COWORK_CON2_IN1_SDATA1_15_SEL_MASK: u32 = 0xf;
pub const ETDM_0_3_COWORK_CON2_IN1_SDATA1_15_SEL_MASK_SFT: u32 = (0xf << 20);
pub const ETDM_0_3_COWORK_CON2_IN1_SDATA0_SEL_SFT: u32 = 16;
pub const ETDM_0_3_COWORK_CON2_IN1_SDATA0_SEL_MASK: u32 = 0xf;
pub const ETDM_0_3_COWORK_CON2_IN1_SDATA0_SEL_MASK_SFT: u32 = (0xf << 16);
pub const ETDM_0_3_COWORK_CON2_IN0_SDATA1_15_SEL_SFT: u32 = 12;
pub const ETDM_0_3_COWORK_CON2_IN0_SDATA1_15_SEL_MASK: u32 = 0xf;
pub const ETDM_0_3_COWORK_CON2_IN0_SDATA1_15_SEL_MASK_SFT: u32 = (0xf << 12);
pub const ETDM_0_3_COWORK_CON2_IN0_SDATA0_SEL_SFT: u32 = 8;
pub const ETDM_0_3_COWORK_CON2_IN0_SDATA0_SEL_MASK: u32 = 0xf;
pub const ETDM_0_3_COWORK_CON2_IN0_SDATA0_SEL_MASK_SFT: u32 = (0xf << 8);
pub const ETDM_0_3_COWORK_CON2_OUT1_DATA_SEL_SFT: u32 = 4;
pub const ETDM_0_3_COWORK_CON2_OUT1_DATA_SEL_MASK: u32 = 0xf;
pub const ETDM_0_3_COWORK_CON2_OUT1_DATA_SEL_MASK_SFT: u32 = (0xf << 4);
pub const ETDM_0_3_COWORK_CON2_OUT0_DATA_SEL_SFT: u32 = 0;
pub const ETDM_0_3_COWORK_CON2_OUT0_DATA_SEL_MASK: u32 = 0xf;
pub const ETDM_0_3_COWORK_CON2_OUT0_DATA_SEL_MASK_SFT: u32 = (0xf << 0);

/* AFE_SECURE_CONN_ETDM1 */
pub const ETDM_4_7_COWORK_CON1_IN4_SDATA1_15_SEL_SFT: u32 = 28;
pub const ETDM_4_7_COWORK_CON1_IN4_SDATA1_15_SEL_MASK: u32 = 0xf;
pub const ETDM_4_7_COWORK_CON1_IN4_SDATA1_15_SEL_MASK_SFT: u32 = (0xf << 28);
pub const ETDM_4_7_COWORK_CON1_IN4_SDATA0_SEL_SFT: u32 = 24;
pub const ETDM_4_7_COWORK_CON1_IN4_SDATA0_SEL_MASK: u32 = 0xf;
pub const ETDM_4_7_COWORK_CON1_IN4_SDATA0_SEL_MASK_SFT: u32 = (0xf << 24);
pub const ETDM_4_7_COWORK_CON1_OUT5_DATA_SEL_SFT: u32 = 20;
pub const ETDM_4_7_COWORK_CON1_OUT5_DATA_SEL_MASK: u32 = 0xf;
pub const ETDM_4_7_COWORK_CON1_OUT5_DATA_SEL_MASK_SFT: u32 = (0xf << 20);
pub const ETDM_4_7_COWORK_CON1_OUT4_DATA_SEL_SFT: u32 = 16;
pub const ETDM_4_7_COWORK_CON1_OUT4_DATA_SEL_MASK: u32 = 0xf;
pub const ETDM_4_7_COWORK_CON1_OUT4_DATA_SEL_MASK_SFT: u32 = (0xf << 16);
pub const ETDM_4_7_COWORK_CON1_IN3_SDATA1_15_SEL_SFT: u32 = 12;
pub const ETDM_4_7_COWORK_CON1_IN3_SDATA1_15_SEL_MASK: u32 = 0xf;
pub const ETDM_4_7_COWORK_CON1_IN3_SDATA1_15_SEL_MASK_SFT: u32 = (0xf << 12);
pub const ETDM_4_7_COWORK_CON1_IN3_SDATA0_SEL_SFT: u32 = 8;
pub const ETDM_4_7_COWORK_CON1_IN3_SDATA0_SEL_MASK: u32 = 0xf;
pub const ETDM_4_7_COWORK_CON1_IN3_SDATA0_SEL_MASK_SFT: u32 = (0xf << 8);
pub const ETDM_4_7_COWORK_CON1_IN2_SDATA1_15_SEL_SFT: u32 = 4;
pub const ETDM_4_7_COWORK_CON1_IN2_SDATA1_15_SEL_MASK: u32 = 0xf;
pub const ETDM_4_7_COWORK_CON1_IN2_SDATA1_15_SEL_MASK_SFT: u32 = (0xf << 4);
pub const ETDM_4_7_COWORK_CON1_IN2_SDATA0_SEL_SFT: u32 = 0;
pub const ETDM_4_7_COWORK_CON1_IN2_SDATA0_SEL_MASK: u32 = 0xf;
pub const ETDM_4_7_COWORK_CON1_IN2_SDATA0_SEL_MASK_SFT: u32 = (0xf << 0);

/* AFE_SECURE_CONN_ETDM2 */
pub const ETDM_4_7_COWORK_CON3_IN7_SDATA1_15_SEL_SFT: u32 = 28;
pub const ETDM_4_7_COWORK_CON3_IN7_SDATA1_15_SEL_MASK: u32 = 0xf;
pub const ETDM_4_7_COWORK_CON3_IN7_SDATA1_15_SEL_MASK_SFT: u32 = (0xf << 28);
pub const ETDM_4_7_COWORK_CON3_IN7_SDATA0_SEL_SFT: u32 = 24;
pub const ETDM_4_7_COWORK_CON3_IN7_SDATA0_SEL_MASK: u32 = 0xf;
pub const ETDM_4_7_COWORK_CON3_IN7_SDATA0_SEL_MASK_SFT: u32 = (0xf << 24);
pub const ETDM_4_7_COWORK_CON3_IN6_SDATA1_15_SEL_SFT: u32 = 20;
pub const ETDM_4_7_COWORK_CON3_IN6_SDATA1_15_SEL_MASK: u32 = 0xf;
pub const ETDM_4_7_COWORK_CON3_IN6_SDATA1_15_SEL_MASK_SFT: u32 = (0xf << 20);
pub const ETDM_4_7_COWORK_CON3_IN6_SDATA0_SEL_SFT: u32 = 16;
pub const ETDM_4_7_COWORK_CON3_IN6_SDATA0_SEL_MASK: u32 = 0xf;
pub const ETDM_4_7_COWORK_CON3_IN6_SDATA0_SEL_MASK_SFT: u32 = (0xf << 16);
pub const ETDM_4_7_COWORK_CON3_OUT7_DATA_SEL_SFT: u32 = 12;
pub const ETDM_4_7_COWORK_CON3_OUT7_DATA_SEL_MASK: u32 = 0xf;
pub const ETDM_4_7_COWORK_CON3_OUT7_DATA_SEL_MASK_SFT: u32 = (0xf << 12);
pub const ETDM_4_7_COWORK_CON3_OUT6_DATA_SEL_SFT: u32 = 8;
pub const ETDM_4_7_COWORK_CON3_OUT6_DATA_SEL_MASK: u32 = 0xf;
pub const ETDM_4_7_COWORK_CON3_OUT6_DATA_SEL_MASK_SFT: u32 = (0xf << 8);
pub const ETDM_4_7_COWORK_CON3_IN5_SDATA1_15_SEL_SFT: u32 = 4;
pub const ETDM_4_7_COWORK_CON3_IN5_SDATA1_15_SEL_MASK: u32 = 0xf;
pub const ETDM_4_7_COWORK_CON3_IN5_SDATA1_15_SEL_MASK_SFT: u32 = (0xf << 4);
pub const ETDM_4_7_COWORK_CON3_IN5_SDATA0_SEL_SFT: u32 = 0;
pub const ETDM_4_7_COWORK_CON3_IN5_SDATA0_SEL_MASK: u32 = 0xf;
pub const ETDM_4_7_COWORK_CON3_IN5_SDATA0_SEL_MASK_SFT: u32 = (0xf << 0);

/* AFE_SECURE_SRAM_CON0 */
pub const SRAM_READ_EN15_NS_SFT: u32 = 31;
pub const SRAM_READ_EN15_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN15_NS_MASK_SFT: u32 = (0x1 << 31);
pub const SRAM_WRITE_EN15_NS_SFT: u32 = 30;
pub const SRAM_WRITE_EN15_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN15_NS_MASK_SFT: u32 = (0x1 << 30);
pub const SRAM_READ_EN14_NS_SFT: u32 = 29;
pub const SRAM_READ_EN14_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN14_NS_MASK_SFT: u32 = (0x1 << 29);
pub const SRAM_WRITE_EN14_NS_SFT: u32 = 28;
pub const SRAM_WRITE_EN14_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN14_NS_MASK_SFT: u32 = (0x1 << 28);
pub const SRAM_READ_EN13_NS_SFT: u32 = 27;
pub const SRAM_READ_EN13_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN13_NS_MASK_SFT: u32 = (0x1 << 27);
pub const SRAM_WRITE_EN13_NS_SFT: u32 = 26;
pub const SRAM_WRITE_EN13_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN13_NS_MASK_SFT: u32 = (0x1 << 26);
pub const SRAM_READ_EN12_NS_SFT: u32 = 25;
pub const SRAM_READ_EN12_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN12_NS_MASK_SFT: u32 = (0x1 << 25);
pub const SRAM_WRITE_EN12_NS_SFT: u32 = 24;
pub const SRAM_WRITE_EN12_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN12_NS_MASK_SFT: u32 = (0x1 << 24);
pub const SRAM_READ_EN11_NS_SFT: u32 = 23;
pub const SRAM_READ_EN11_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN11_NS_MASK_SFT: u32 = (0x1 << 23);
pub const SRAM_WRITE_EN11_NS_SFT: u32 = 22;
pub const SRAM_WRITE_EN11_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN11_NS_MASK_SFT: u32 = (0x1 << 22);
pub const SRAM_READ_EN10_NS_SFT: u32 = 21;
pub const SRAM_READ_EN10_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN10_NS_MASK_SFT: u32 = (0x1 << 21);
pub const SRAM_WRITE_EN10_NS_SFT: u32 = 20;
pub const SRAM_WRITE_EN10_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN10_NS_MASK_SFT: u32 = (0x1 << 20);
pub const SRAM_READ_EN9_NS_SFT: u32 = 19;
pub const SRAM_READ_EN9_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN9_NS_MASK_SFT: u32 = (0x1 << 19);
pub const SRAM_WRITE_EN9_NS_SFT: u32 = 18;
pub const SRAM_WRITE_EN9_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN9_NS_MASK_SFT: u32 = (0x1 << 18);
pub const SRAM_READ_EN8_NS_SFT: u32 = 17;
pub const SRAM_READ_EN8_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN8_NS_MASK_SFT: u32 = (0x1 << 17);
pub const SRAM_WRITE_EN8_NS_SFT: u32 = 16;
pub const SRAM_WRITE_EN8_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN8_NS_MASK_SFT: u32 = (0x1 << 16);
pub const SRAM_READ_EN7_NS_SFT: u32 = 15;
pub const SRAM_READ_EN7_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN7_NS_MASK_SFT: u32 = (0x1 << 15);
pub const SRAM_WRITE_EN7_NS_SFT: u32 = 14;
pub const SRAM_WRITE_EN7_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN7_NS_MASK_SFT: u32 = (0x1 << 14);
pub const SRAM_READ_EN6_NS_SFT: u32 = 13;
pub const SRAM_READ_EN6_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN6_NS_MASK_SFT: u32 = (0x1 << 13);
pub const SRAM_WRITE_EN6_NS_SFT: u32 = 12;
pub const SRAM_WRITE_EN6_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN6_NS_MASK_SFT: u32 = (0x1 << 12);
pub const SRAM_READ_EN5_NS_SFT: u32 = 11;
pub const SRAM_READ_EN5_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN5_NS_MASK_SFT: u32 = (0x1 << 11);
pub const SRAM_WRITE_EN5_NS_SFT: u32 = 10;
pub const SRAM_WRITE_EN5_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN5_NS_MASK_SFT: u32 = (0x1 << 10);
pub const SRAM_READ_EN4_NS_SFT: u32 = 9;
pub const SRAM_READ_EN4_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN4_NS_MASK_SFT: u32 = (0x1 << 9);
pub const SRAM_WRITE_EN4_NS_SFT: u32 = 8;
pub const SRAM_WRITE_EN4_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN4_NS_MASK_SFT: u32 = (0x1 << 8);
pub const SRAM_READ_EN3_NS_SFT: u32 = 7;
pub const SRAM_READ_EN3_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN3_NS_MASK_SFT: u32 = (0x1 << 7);
pub const SRAM_WRITE_EN3_NS_SFT: u32 = 6;
pub const SRAM_WRITE_EN3_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN3_NS_MASK_SFT: u32 = (0x1 << 6);
pub const SRAM_READ_EN2_NS_SFT: u32 = 5;
pub const SRAM_READ_EN2_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN2_NS_MASK_SFT: u32 = (0x1 << 5);
pub const SRAM_WRITE_EN2_NS_SFT: u32 = 4;
pub const SRAM_WRITE_EN2_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN2_NS_MASK_SFT: u32 = (0x1 << 4);
pub const SRAM_READ_EN1_NS_SFT: u32 = 3;
pub const SRAM_READ_EN1_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN1_NS_MASK_SFT: u32 = (0x1 << 3);
pub const SRAM_WRITE_EN1_NS_SFT: u32 = 2;
pub const SRAM_WRITE_EN1_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN1_NS_MASK_SFT: u32 = (0x1 << 2);
pub const SRAM_READ_EN0_NS_SFT: u32 = 1;
pub const SRAM_READ_EN0_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN0_NS_MASK_SFT: u32 = (0x1 << 1);
pub const SRAM_WRITE_EN0_NS_SFT: u32 = 0;
pub const SRAM_WRITE_EN0_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN0_NS_MASK_SFT: u32 = (0x1 << 0);

/* AFE_SECURE_SRAM_CON1 */
pub const SRAM_READ_EN15_S_SFT: u32 = 31;
pub const SRAM_READ_EN15_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN15_S_MASK_SFT: u32 = (0x1 << 31);
pub const SRAM_WRITE_EN15_S_SFT: u32 = 30;
pub const SRAM_WRITE_EN15_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN15_S_MASK_SFT: u32 = (0x1 << 30);
pub const SRAM_READ_EN14_S_SFT: u32 = 29;
pub const SRAM_READ_EN14_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN14_S_MASK_SFT: u32 = (0x1 << 29);
pub const SRAM_WRITE_EN14_S_SFT: u32 = 28;
pub const SRAM_WRITE_EN14_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN14_S_MASK_SFT: u32 = (0x1 << 28);
pub const SRAM_READ_EN13_S_SFT: u32 = 27;
pub const SRAM_READ_EN13_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN13_S_MASK_SFT: u32 = (0x1 << 27);
pub const SRAM_WRITE_EN13_S_SFT: u32 = 26;
pub const SRAM_WRITE_EN13_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN13_S_MASK_SFT: u32 = (0x1 << 26);
pub const SRAM_READ_EN12_S_SFT: u32 = 25;
pub const SRAM_READ_EN12_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN12_S_MASK_SFT: u32 = (0x1 << 25);
pub const SRAM_WRITE_EN12_S_SFT: u32 = 24;
pub const SRAM_WRITE_EN12_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN12_S_MASK_SFT: u32 = (0x1 << 24);
pub const SRAM_READ_EN11_S_SFT: u32 = 23;
pub const SRAM_READ_EN11_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN11_S_MASK_SFT: u32 = (0x1 << 23);
pub const SRAM_WRITE_EN11_S_SFT: u32 = 22;
pub const SRAM_WRITE_EN11_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN11_S_MASK_SFT: u32 = (0x1 << 22);
pub const SRAM_READ_EN10_S_SFT: u32 = 21;
pub const SRAM_READ_EN10_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN10_S_MASK_SFT: u32 = (0x1 << 21);
pub const SRAM_WRITE_EN10_S_SFT: u32 = 20;
pub const SRAM_WRITE_EN10_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN10_S_MASK_SFT: u32 = (0x1 << 20);
pub const SRAM_READ_EN9_S_SFT: u32 = 19;
pub const SRAM_READ_EN9_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN9_S_MASK_SFT: u32 = (0x1 << 19);
pub const SRAM_WRITE_EN9_S_SFT: u32 = 18;
pub const SRAM_WRITE_EN9_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN9_S_MASK_SFT: u32 = (0x1 << 18);
pub const SRAM_READ_EN8_S_SFT: u32 = 17;
pub const SRAM_READ_EN8_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN8_S_MASK_SFT: u32 = (0x1 << 17);
pub const SRAM_WRITE_EN8_S_SFT: u32 = 16;
pub const SRAM_WRITE_EN8_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN8_S_MASK_SFT: u32 = (0x1 << 16);
pub const SRAM_READ_EN7_S_SFT: u32 = 15;
pub const SRAM_READ_EN7_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN7_S_MASK_SFT: u32 = (0x1 << 15);
pub const SRAM_WRITE_EN7_S_SFT: u32 = 14;
pub const SRAM_WRITE_EN7_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN7_S_MASK_SFT: u32 = (0x1 << 14);
pub const SRAM_READ_EN6_S_SFT: u32 = 13;
pub const SRAM_READ_EN6_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN6_S_MASK_SFT: u32 = (0x1 << 13);
pub const SRAM_WRITE_EN6_S_SFT: u32 = 12;
pub const SRAM_WRITE_EN6_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN6_S_MASK_SFT: u32 = (0x1 << 12);
pub const SRAM_READ_EN5_S_SFT: u32 = 11;
pub const SRAM_READ_EN5_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN5_S_MASK_SFT: u32 = (0x1 << 11);
pub const SRAM_WRITE_EN5_S_SFT: u32 = 10;
pub const SRAM_WRITE_EN5_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN5_S_MASK_SFT: u32 = (0x1 << 10);
pub const SRAM_READ_EN4_S_SFT: u32 = 9;
pub const SRAM_READ_EN4_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN4_S_MASK_SFT: u32 = (0x1 << 9);
pub const SRAM_WRITE_EN4_S_SFT: u32 = 8;
pub const SRAM_WRITE_EN4_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN4_S_MASK_SFT: u32 = (0x1 << 8);
pub const SRAM_READ_EN3_S_SFT: u32 = 7;
pub const SRAM_READ_EN3_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN3_S_MASK_SFT: u32 = (0x1 << 7);
pub const SRAM_WRITE_EN3_S_SFT: u32 = 6;
pub const SRAM_WRITE_EN3_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN3_S_MASK_SFT: u32 = (0x1 << 6);
pub const SRAM_READ_EN2_S_SFT: u32 = 5;
pub const SRAM_READ_EN2_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN2_S_MASK_SFT: u32 = (0x1 << 5);
pub const SRAM_WRITE_EN2_S_SFT: u32 = 4;
pub const SRAM_WRITE_EN2_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN2_S_MASK_SFT: u32 = (0x1 << 4);
pub const SRAM_READ_EN1_S_SFT: u32 = 3;
pub const SRAM_READ_EN1_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN1_S_MASK_SFT: u32 = (0x1 << 3);
pub const SRAM_WRITE_EN1_S_SFT: u32 = 2;
pub const SRAM_WRITE_EN1_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN1_S_MASK_SFT: u32 = (0x1 << 2);
pub const SRAM_READ_EN0_S_SFT: u32 = 1;
pub const SRAM_READ_EN0_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN0_S_MASK_SFT: u32 = (0x1 << 1);
pub const SRAM_WRITE_EN0_S_SFT: u32 = 0;
pub const SRAM_WRITE_EN0_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN0_S_MASK_SFT: u32 = (0x1 << 0);

/* AFE_SE_CONN_INPUT_MASK0 */
pub const SECURE_INTRCONN_I0_I31_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_I0_I31_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_I0_I31_S_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_SE_CONN_INPUT_MASK1 */
pub const SECURE_INTRCONN_I32_I63_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_I32_I63_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_I32_I63_S_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_SE_CONN_INPUT_MASK2 */
pub const SECURE_INTRCONN_I64_I95_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_I64_I95_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_I64_I95_S_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_SE_CONN_INPUT_MASK3 */
pub const SECURE_INTRCONN_I96_I127_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_I96_I127_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_I96_I127_S_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_SE_CONN_INPUT_MASK4 */
pub const SECURE_INTRCONN_I128_I159_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_I128_I159_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_I128_I159_S_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_SE_CONN_INPUT_MASK5 */
pub const SECURE_INTRCONN_I160_I191_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_I160_I191_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_I160_I191_S_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_SE_CONN_INPUT_MASK6 */
pub const SECURE_INTRCONN_I192_I223_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_I192_I223_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_I192_I223_S_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_SE_CONN_INPUT_MASK7 */
pub const SECURE_INTRCONN_I224_I256_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_I224_I256_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_I224_I256_S_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_NON_SE_CONN_INPUT_MASK0 */
pub const NORMAL_INTRCONN_I0_I31_S_SFT: u32 = 0;
pub const NORMAL_INTRCONN_I0_I31_S_MASK: u32 = 0xffffffff;
pub const NORMAL_INTRCONN_I0_I31_S_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_NON_SE_CONN_INPUT_MASK1 */
pub const NORMAL_INTRCONN_I32_I63_S_SFT: u32 = 0;
pub const NORMAL_INTRCONN_I32_I63_S_MASK: u32 = 0xffffffff;
pub const NORMAL_INTRCONN_I32_I63_S_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_NON_SE_CONN_INPUT_MASK2 */
pub const NORMAL_INTRCONN_I64_I95_S_SFT: u32 = 0;
pub const NORMAL_INTRCONN_I64_I95_S_MASK: u32 = 0xffffffff;
pub const NORMAL_INTRCONN_I64_I95_S_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_NON_SE_CONN_INPUT_MASK3 */
pub const NORMAL_INTRCONN_I96_I127_S_SFT: u32 = 0;
pub const NORMAL_INTRCONN_I96_I127_S_MASK: u32 = 0xffffffff;
pub const NORMAL_INTRCONN_I96_I127_S_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_NON_SE_CONN_INPUT_MASK4 */
pub const NORMAL_INTRCONN_I128_I159_S_SFT: u32 = 0;
pub const NORMAL_INTRCONN_I128_I159_S_MASK: u32 = 0xffffffff;
pub const NORMAL_INTRCONN_I128_I159_S_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_NON_SE_CONN_INPUT_MASK5 */
pub const NORMAL_INTRCONN_I160_I191_S_SFT: u32 = 0;
pub const NORMAL_INTRCONN_I160_I191_S_MASK: u32 = 0xffffffff;
pub const NORMAL_INTRCONN_I160_I191_S_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_NON_SE_CONN_INPUT_MASK6 */
pub const NORMAL_INTRCONN_I192_I223_S_SFT: u32 = 0;
pub const NORMAL_INTRCONN_I192_I223_S_MASK: u32 = 0xffffffff;
pub const NORMAL_INTRCONN_I192_I223_S_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_NON_SE_CONN_INPUT_MASK7 */
pub const NORMAL_INTRCONN_I224_I256_S_SFT: u32 = 0;
pub const NORMAL_INTRCONN_I224_I256_S_MASK: u32 = 0xffffffff;
pub const NORMAL_INTRCONN_I224_I256_S_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_SE_CONN_OUTPUT_SEL0 */
pub const SECURE_INTRCONN_O0_O31_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_O0_O31_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_O0_O31_S_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_SE_CONN_OUTPUT_SEL1 */
pub const SECURE_INTRCONN_O32_O63_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_O32_O63_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_O32_O63_S_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_SE_CONN_OUTPUT_SEL2 */
pub const SECURE_INTRCONN_O64_O95_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_O64_O95_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_O64_O95_S_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_SE_CONN_OUTPUT_SEL3 */
pub const SECURE_INTRCONN_O96_O127_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_O96_O127_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_O96_O127_S_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_SE_CONN_OUTPUT_SEL4 */
pub const SECURE_INTRCONN_O128_O159_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_O128_O159_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_O128_O159_S_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_SE_CONN_OUTPUT_SEL5 */
pub const SECURE_INTRCONN_O160_O191_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_O160_O191_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_O160_O191_S_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_SE_CONN_OUTPUT_SEL6 */
pub const SECURE_INTRCONN_O192_O223_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_O192_O223_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_O192_O223_S_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_SE_CONN_OUTPUT_SEL7 */
pub const SECURE_INTRCONN_O224_O256_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_O224_O256_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_O224_O256_S_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_PCM0_INTF_CON1_MASK_MON */
pub const AFE_PCM0_INTF_CON1_MASK_MON_SFT: u32 = 0;
pub const AFE_PCM0_INTF_CON1_MASK_MON_MASK: u32 = 0xffffffff;
pub const AFE_PCM0_INTF_CON1_MASK_MON_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_PCM0_INTF_CON0_MASK_MON */
pub const AFE_PCM0_INTF_CON0_MASK_MON_SFT: u32 = 0;
pub const AFE_PCM0_INTF_CON0_MASK_MON_MASK: u32 = 0xffffffff;
pub const AFE_PCM0_INTF_CON0_MASK_MON_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_CONNSYS_I2S_CON_MASK_MON */
pub const AFE_CONNSYS_I2S_CON_MASK_MON_SFT: u32 = 0;
pub const AFE_CONNSYS_I2S_CON_MASK_MON_MASK: u32 = 0xffffffff;
pub const AFE_CONNSYS_I2S_CON_MASK_MON_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_TDM_CON2_MASK_MON */
pub const AFE_TDM_CON2_MASK_MON_SFT: u32 = 0;
pub const AFE_TDM_CON2_MASK_MON_MASK: u32 = 0xffffffff;
pub const AFE_TDM_CON2_MASK_MON_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_MTKAIF0_CFG0_MASK_MON */
pub const AFE_MTKAIF0_CFG0_MASK_MON_SFT: u32 = 0;
pub const AFE_MTKAIF0_CFG0_MASK_MON_MASK: u32 = 0xffffffff;
pub const AFE_MTKAIF0_CFG0_MASK_MON_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_MTKAIF1_CFG0_MASK_MON */
pub const AFE_MTKAIF1_CFG0_MASK_MON_SFT: u32 = 0;
pub const AFE_MTKAIF1_CFG0_MASK_MON_MASK: u32 = 0xffffffff;
pub const AFE_MTKAIF1_CFG0_MASK_MON_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ADDA_UL0_SRC_CON0_MASK_MON */
pub const AFE_ADDA_UL0_SRC_CON0_MASK_MON_SFT: u32 = 0;
pub const AFE_ADDA_UL0_SRC_CON0_MASK_MON_MASK: u32 = 0xffffffff;
pub const AFE_ADDA_UL0_SRC_CON0_MASK_MON_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ADDA_UL1_SRC_CON0_MASK_MON */
pub const AFE_ADDA_UL1_SRC_CON0_MASK_MON_SFT: u32 = 0;
pub const AFE_ADDA_UL1_SRC_CON0_MASK_MON_MASK: u32 = 0xffffffff;
pub const AFE_ADDA_UL1_SRC_CON0_MASK_MON_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ADDA_UL2_SRC_CON0_MASK_MON */
pub const AFE_ADDA_UL2_SRC_CON0_MASK_MON_SFT: u32 = 0;
pub const AFE_ADDA_UL2_SRC_CON0_MASK_MON_MASK: u32 = 0xffffffff;
pub const AFE_ADDA_UL2_SRC_CON0_MASK_MON_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ASRC_NEW_CON0 */
pub const ONE_HEART_SFT: u32 = 31;
pub const ONE_HEART_MASK: u32 = 0x1;
pub const ONE_HEART_MASK_SFT: u32 = (0x1 << 31);
pub const CHSET0_OFS_ONE_HEART_DISABLE_SFT: u32 = 30;
pub const CHSET0_OFS_ONE_HEART_DISABLE_MASK: u32 = 0x1;
pub const CHSET0_OFS_ONE_HEART_DISABLE_MASK_SFT: u32 = (0x1 << 30);
pub const USE_SHORT_DELAY_COEFF_SFT: u32 = 29;
pub const USE_SHORT_DELAY_COEFF_MASK: u32 = 0x1;
pub const USE_SHORT_DELAY_COEFF_MASK_SFT: u32 = (0x1 << 29);
pub const CHSET0_O16BIT_SFT: u32 = 19;
pub const CHSET0_O16BIT_MASK: u32 = 0x1;
pub const CHSET0_O16BIT_MASK_SFT: u32 = (0x1 << 19);
pub const CHSET0_CLR_IIR_HISTORY_SFT: u32 = 17;
pub const CHSET0_CLR_IIR_HISTORY_MASK: u32 = 0x1;
pub const CHSET0_CLR_IIR_HISTORY_MASK_SFT: u32 = (0x1 << 17);
pub const CHSET0_IS_MONO_SFT: u32 = 16;
pub const CHSET0_IS_MONO_MASK: u32 = 0x1;
pub const CHSET0_IS_MONO_MASK_SFT: u32 = (0x1 << 16);
pub const CHSET0_OFS_SEL_SFT: u32 = 14;
pub const CHSET0_OFS_SEL_MASK: u32 = 0x3;
pub const CHSET0_OFS_SEL_MASK_SFT: u32 = (0x3 << 14);
pub const CHSET0_IFS_SEL_SFT: u32 = 12;
pub const CHSET0_IFS_SEL_MASK: u32 = 0x3;
pub const CHSET0_IFS_SEL_MASK_SFT: u32 = (0x3 << 12);
pub const CHSET0_IIR_EN_SFT: u32 = 11;
pub const CHSET0_IIR_EN_MASK: u32 = 0x1;
pub const CHSET0_IIR_EN_MASK_SFT: u32 = (0x1 << 11);
pub const CHSET0_IIR_STAGE_SFT: u32 = 8;
pub const CHSET0_IIR_STAGE_MASK: u32 = 0x7;
pub const CHSET0_IIR_STAGE_MASK_SFT: u32 = (0x7 << 8);
pub const ASM_ON_MOD_SFT: u32 = 7;
pub const ASM_ON_MOD_MASK: u32 = 0x1;
pub const ASM_ON_MOD_MASK_SFT: u32 = (0x1 << 7);
pub const CHSET_STR_CLR_SFT: u32 = 4;
pub const CHSET_STR_CLR_MASK: u32 = 0x1;
pub const CHSET_STR_CLR_MASK_SFT: u32 = (0x1 << 4);
pub const CHSET_ON_SFT: u32 = 2;
pub const CHSET_ON_MASK: u32 = 0x1;
pub const CHSET_ON_MASK_SFT: u32 = (0x1 << 2);
pub const COEFF_SRAM_CTRL_SFT: u32 = 1;
pub const COEFF_SRAM_CTRL_MASK: u32 = 0x1;
pub const COEFF_SRAM_CTRL_MASK_SFT: u32 = (0x1 << 1);
pub const ASM_ON_SFT: u32 = 0;
pub const ASM_ON_MASK: u32 = 0x1;
pub const ASM_ON_MASK_SFT: u32 = (0x1 << 0);

/* AFE_ASRC_NEW_CON1 */
pub const ASM_FREQ_0_SFT: u32 = 0;
pub const ASM_FREQ_0_MASK: u32 = 0xffffff;
pub const ASM_FREQ_0_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_ASRC_NEW_CON2 */
pub const ASM_FREQ_1_SFT: u32 = 0;
pub const ASM_FREQ_1_MASK: u32 = 0xffffff;
pub const ASM_FREQ_1_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_ASRC_NEW_CON3 */
pub const ASM_FREQ_2_SFT: u32 = 0;
pub const ASM_FREQ_2_MASK: u32 = 0xffffff;
pub const ASM_FREQ_2_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_ASRC_NEW_CON4 */
pub const ASM_FREQ_3_SFT: u32 = 0;
pub const ASM_FREQ_3_MASK: u32 = 0xffffff;
pub const ASM_FREQ_3_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_ASRC_NEW_CON5 */
pub const OUT_EN_SEL_DOMAIN_SFT: u32 = 29;
pub const OUT_EN_SEL_DOMAIN_MASK: u32 = 0x7;
pub const OUT_EN_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 29);
pub const OUT_EN_SEL_FS_SFT: u32 = 24;
pub const OUT_EN_SEL_FS_MASK: u32 = 0x1f;
pub const OUT_EN_SEL_FS_MASK_SFT: u32 = (0x1f << 24);
pub const IN_EN_SEL_DOMAIN_SFT: u32 = 21;
pub const IN_EN_SEL_DOMAIN_MASK: u32 = 0x7;
pub const IN_EN_SEL_DOMAIN_MASK_SFT: u32 = (0x7 << 21);
pub const IN_EN_SEL_FS_SFT: u32 = 16;
pub const IN_EN_SEL_FS_MASK: u32 = 0x1f;
pub const IN_EN_SEL_FS_MASK_SFT: u32 = (0x1f << 16);
pub const RESULT_SEL_SFT: u32 = 8;
pub const RESULT_SEL_MASK: u32 = 0x7;
pub const RESULT_SEL_MASK_SFT: u32 = (0x7 << 8);
pub const CALI_CK_SEL_SFT: u32 = 4;
pub const CALI_CK_SEL_MASK: u32 = 0x7;
pub const CALI_CK_SEL_MASK_SFT: u32 = (0x7 << 4);
pub const CALI_LRCK_SEL_SFT: u32 = 1;
pub const CALI_LRCK_SEL_MASK: u32 = 0x7;
pub const CALI_LRCK_SEL_MASK_SFT: u32 = (0x7 << 1);
pub const SOFT_RESET_SFT: u32 = 0;
pub const SOFT_RESET_MASK: u32 = 0x1;
pub const SOFT_RESET_MASK_SFT: u32 = (0x1 << 0);

/* AFE_ASRC_NEW_CON6 */
pub const FREQ_CALI_CYCLE_SFT: u32 = 16;
pub const FREQ_CALI_CYCLE_MASK: u32 = 0xffff;
pub const FREQ_CALI_CYCLE_MASK_SFT: u32 = (0xffff << 16);
pub const FREQ_CALI_AUTORST_EN_SFT: u32 = 15;
pub const FREQ_CALI_AUTORST_EN_MASK: u32 = 0x1;
pub const FREQ_CALI_AUTORST_EN_MASK_SFT: u32 = (0x1 << 15);
pub const CALI_AUTORST_DETECT_SFT: u32 = 14;
pub const CALI_AUTORST_DETECT_MASK: u32 = 0x1;
pub const CALI_AUTORST_DETECT_MASK_SFT: u32 = (0x1 << 14);
pub const FREQ_CALC_RUNNING_SFT: u32 = 13;
pub const FREQ_CALC_RUNNING_MASK: u32 = 0x1;
pub const FREQ_CALC_RUNNING_MASK_SFT: u32 = (0x1 << 13);
pub const AUTO_TUNE_FREQ3_SFT: u32 = 12;
pub const AUTO_TUNE_FREQ3_MASK: u32 = 0x1;
pub const AUTO_TUNE_FREQ3_MASK_SFT: u32 = (0x1 << 12);
pub const COMP_FREQ_RES_EN_SFT: u32 = 11;
pub const COMP_FREQ_RES_EN_MASK: u32 = 0x1;
pub const COMP_FREQ_RES_EN_MASK_SFT: u32 = (0x1 << 11);
pub const FREQ_CALI_SEL_SFT: u32 = 8;
pub const FREQ_CALI_SEL_MASK: u32 = 0x3;
pub const FREQ_CALI_SEL_MASK_SFT: u32 = (0x3 << 8);
pub const FREQ_CALI_BP_DGL_SFT: u32 = 7;
pub const FREQ_CALI_BP_DGL_MASK: u32 = 0x1;
pub const FREQ_CALI_BP_DGL_MASK_SFT: u32 = (0x1 << 7);
pub const FREQ_CALI_MAX_GWIDTH_SFT: u32 = 4;
pub const FREQ_CALI_MAX_GWIDTH_MASK: u32 = 0x7;
pub const FREQ_CALI_MAX_GWIDTH_MASK_SFT: u32 = (0x7 << 4);
pub const AUTO_TUNE_FREQ2_SFT: u32 = 3;
pub const AUTO_TUNE_FREQ2_MASK: u32 = 0x1;
pub const AUTO_TUNE_FREQ2_MASK_SFT: u32 = (0x1 << 3);
pub const FREQ_CALI_AUTO_RESTART_SFT: u32 = 2;
pub const FREQ_CALI_AUTO_RESTART_MASK: u32 = 0x1;
pub const FREQ_CALI_AUTO_RESTART_MASK_SFT: u32 = (0x1 << 2);
pub const CALI_USE_FREQ_OUT_SFT: u32 = 1;
pub const CALI_USE_FREQ_OUT_MASK: u32 = 0x1;
pub const CALI_USE_FREQ_OUT_MASK_SFT: u32 = (0x1 << 1);
pub const CALI_EN_SFT: u32 = 0;
pub const CALI_EN_MASK: u32 = 0x1;
pub const CALI_EN_MASK_SFT: u32 = (0x1 << 0);

/* AFE_ASRC_NEW_CON7 */
pub const FREQ_CALC_DENOMINATOR_SFT: u32 = 0;
pub const FREQ_CALC_DENOMINATOR_MASK: u32 = 0xffffff;
pub const FREQ_CALC_DENOMINATOR_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_ASRC_NEW_CON8 */
pub const PRD_CALI_RESULT_RECORD_SFT: u32 = 0;
pub const PRD_CALI_RESULT_RECORD_MASK: u32 = 0xffffff;
pub const PRD_CALI_RESULT_RECORD_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_ASRC_NEW_CON9 */
pub const FREQ_CALI_RESULT_SFT: u32 = 0;
pub const FREQ_CALI_RESULT_MASK: u32 = 0xffffff;
pub const FREQ_CALI_RESULT_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_ASRC_NEW_CON10 */
pub const COEFF_SRAM_DATA_SFT: u32 = 0;
pub const COEFF_SRAM_DATA_MASK: u32 = 0xffffffff;
pub const COEFF_SRAM_DATA_MASK_SFT: u32 = (0xffffffff << 0);

/* AFE_ASRC_NEW_CON11 */
pub const COEFF_SRAM_ADR_SFT: u32 = 0;
pub const COEFF_SRAM_ADR_MASK: u32 = 0x3f;
pub const COEFF_SRAM_ADR_MASK_SFT: u32 = (0x3f << 0);

/* AFE_ASRC_NEW_CON12 */
pub const RING_DBG_RD_SFT: u32 = 0;
pub const RING_DBG_RD_MASK: u32 = 0x3ffffff;
pub const RING_DBG_RD_MASK_SFT: u32 = (0x3ffffff << 0);

/* AFE_ASRC_NEW_CON13 */
pub const FREQ_CALI_AUTORST_TH_HIGH_SFT: u32 = 0;
pub const FREQ_CALI_AUTORST_TH_HIGH_MASK: u32 = 0xffffff;
pub const FREQ_CALI_AUTORST_TH_HIGH_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_ASRC_NEW_CON14 */
pub const FREQ_CALI_AUTORST_TH_LOW_SFT: u32 = 0;
pub const FREQ_CALI_AUTORST_TH_LOW_MASK: u32 = 0xffffff;
pub const FREQ_CALI_AUTORST_TH_LOW_MASK_SFT: u32 = (0xffffff << 0);

/* AFE_ASRC_NEW_IP_VERSION */
pub const IP_VERSION_SFT: u32 = 0;
pub const IP_VERSION_MASK: u32 = 0xffffffff;
pub const IP_VERSION_MASK_SFT: u32 = (0xffffffff << 0);

pub const AUDIO_TOP_CON0: u32 = 0x0;
pub const AUDIO_TOP_CON1: u32 = 0x4;
pub const AUDIO_TOP_CON2: u32 = 0x8;
pub const AUDIO_TOP_CON3: u32 = 0xc;
pub const AUDIO_TOP_CON4: u32 = 0x10;
pub const AUDIO_ENGEN_CON0: u32 = 0x14;
pub const AUDIO_ENGEN_CON0_USER1: u32 = 0x18;
pub const AUDIO_ENGEN_CON0_USER2: u32 = 0x1c;
pub const AFE_SINEGEN_CON0: u32 = 0x20;
pub const AFE_SINEGEN_CON1: u32 = 0x24;
pub const AFE_SINEGEN_CON2: u32 = 0x28;
pub const AFE_SINEGEN_CON3: u32 = 0x2c;
pub const AFE_APLL1_TUNER_CFG: u32 = 0x30;
pub const AFE_APLL1_TUNER_MON0: u32 = 0x34;
pub const AFE_APLL2_TUNER_CFG: u32 = 0x38;
pub const AFE_APLL2_TUNER_MON0: u32 = 0x3c;
pub const AUDIO_TOP_RG0: u32 = 0x4c;
pub const AUDIO_TOP_RG1: u32 = 0x50;
pub const AUDIO_TOP_RG2: u32 = 0x54;
pub const AUDIO_TOP_RG3: u32 = 0x58;
pub const AUDIO_TOP_RG4: u32 = 0x5c;
pub const AFE_SPM_CONTROL_REQ: u32 = 0x60;
pub const AFE_SPM_CONTROL_ACK: u32 = 0x64;
pub const AUD_TOP_CFG_VCORE_RG: u32 = 0x68;
pub const AUDIO_TOP_IP_VERSION: u32 = 0x6c;
pub const AUDIO_ENGEN_CON0_MON: u32 = 0x7c;
pub const AUD_TOP_CFG_VLP_RG: u32 = 0x98;
pub const AUD_TOP_MON_RG: u32 = 0x9c;
pub const AUDIO_USE_DEFAULT_DELSEL0: u32 = 0xa0;
pub const AUDIO_USE_DEFAULT_DELSEL1: u32 = 0xa4;
pub const AUDIO_USE_DEFAULT_DELSEL2: u32 = 0xa8;
pub const AFE_CONNSYS_I2S_IPM_VER_MON: u32 = 0xb0;
pub const AFE_CONNSYS_I2S_MON_SEL: u32 = 0xb4;
pub const AFE_CONNSYS_I2S_MON: u32 = 0xb8;
pub const AFE_CONNSYS_I2S_CON: u32 = 0xbc;
pub const AFE_PCM0_INTF_CON0: u32 = 0xc0;
pub const AFE_PCM0_INTF_CON1: u32 = 0xc4;
pub const AFE_PCM_INTF_MON: u32 = 0xc8;
pub const AFE_PCM1_INTF_CON0: u32 = 0xd0;
pub const AFE_PCM1_INTF_CON1: u32 = 0xd4;
pub const AFE_PCM_TOP_IP_VERSION: u32 = 0xe8;
pub const AFE_IRQ_MCU_EN: u32 = 0x100;
pub const AFE_IRQ_MCU_DSP_EN: u32 = 0x104;
pub const AFE_IRQ_MCU_DSP2_EN: u32 = 0x108;
pub const AFE_IRQ_MCU_SCP_EN: u32 = 0x10c;
pub const AFE_CUSTOM_IRQ_MCU_EN: u32 = 0x110;
pub const AFE_CUSTOM_IRQ_MCU_DSP_EN: u32 = 0x114;
pub const AFE_CUSTOM_IRQ_MCU_DSP2_EN: u32 = 0x118;
pub const AFE_CUSTOM_IRQ_MCU_SCP_EN: u32 = 0x11c;
pub const AFE_IRQ_MCU_STATUS: u32 = 0x120;
pub const AFE_CUSTOM_IRQ_MCU_STATUS: u32 = 0x124;
pub const AFE_IRQ0_MCU_CFG0: u32 = 0x140;
pub const AFE_IRQ0_MCU_CFG1: u32 = 0x144;
pub const AFE_IRQ1_MCU_CFG0: u32 = 0x148;
pub const AFE_IRQ1_MCU_CFG1: u32 = 0x14c;
pub const AFE_IRQ2_MCU_CFG0: u32 = 0x150;
pub const AFE_IRQ2_MCU_CFG1: u32 = 0x154;
pub const AFE_IRQ3_MCU_CFG0: u32 = 0x158;
pub const AFE_IRQ3_MCU_CFG1: u32 = 0x15c;
pub const AFE_IRQ4_MCU_CFG0: u32 = 0x160;
pub const AFE_IRQ4_MCU_CFG1: u32 = 0x164;
pub const AFE_IRQ5_MCU_CFG0: u32 = 0x168;
pub const AFE_IRQ5_MCU_CFG1: u32 = 0x16c;
pub const AFE_IRQ6_MCU_CFG0: u32 = 0x170;
pub const AFE_IRQ6_MCU_CFG1: u32 = 0x174;
pub const AFE_IRQ7_MCU_CFG0: u32 = 0x178;
pub const AFE_IRQ7_MCU_CFG1: u32 = 0x17c;
pub const AFE_IRQ8_MCU_CFG0: u32 = 0x180;
pub const AFE_IRQ8_MCU_CFG1: u32 = 0x184;
pub const AFE_IRQ9_MCU_CFG0: u32 = 0x188;
pub const AFE_IRQ9_MCU_CFG1: u32 = 0x18c;
pub const AFE_IRQ10_MCU_CFG0: u32 = 0x190;
pub const AFE_IRQ10_MCU_CFG1: u32 = 0x194;
pub const AFE_IRQ11_MCU_CFG0: u32 = 0x198;
pub const AFE_IRQ11_MCU_CFG1: u32 = 0x19c;
pub const AFE_IRQ12_MCU_CFG0: u32 = 0x1a0;
pub const AFE_IRQ12_MCU_CFG1: u32 = 0x1a4;
pub const AFE_IRQ13_MCU_CFG0: u32 = 0x1a8;
pub const AFE_IRQ13_MCU_CFG1: u32 = 0x1ac;
pub const AFE_IRQ14_MCU_CFG0: u32 = 0x1b0;
pub const AFE_IRQ14_MCU_CFG1: u32 = 0x1b4;
pub const AFE_IRQ15_MCU_CFG0: u32 = 0x1b8;
pub const AFE_IRQ15_MCU_CFG1: u32 = 0x1bc;
pub const AFE_IRQ16_MCU_CFG0: u32 = 0x1c0;
pub const AFE_IRQ16_MCU_CFG1: u32 = 0x1c4;
pub const AFE_IRQ17_MCU_CFG0: u32 = 0x1c8;
pub const AFE_IRQ17_MCU_CFG1: u32 = 0x1cc;
pub const AFE_IRQ18_MCU_CFG0: u32 = 0x1d0;
pub const AFE_IRQ18_MCU_CFG1: u32 = 0x1d4;
pub const AFE_IRQ19_MCU_CFG0: u32 = 0x1d8;
pub const AFE_IRQ19_MCU_CFG1: u32 = 0x1dc;
pub const AFE_IRQ20_MCU_CFG0: u32 = 0x1e0;
pub const AFE_IRQ20_MCU_CFG1: u32 = 0x1e4;
pub const AFE_IRQ21_MCU_CFG0: u32 = 0x1e8;
pub const AFE_IRQ21_MCU_CFG1: u32 = 0x1ec;
pub const AFE_IRQ22_MCU_CFG0: u32 = 0x1f0;
pub const AFE_IRQ22_MCU_CFG1: u32 = 0x1f4;
pub const AFE_IRQ23_MCU_CFG0: u32 = 0x1f8;
pub const AFE_IRQ23_MCU_CFG1: u32 = 0x1fc;
pub const AFE_IRQ24_MCU_CFG0: u32 = 0x200;
pub const AFE_IRQ24_MCU_CFG1: u32 = 0x204;
pub const AFE_IRQ25_MCU_CFG0: u32 = 0x208;
pub const AFE_IRQ25_MCU_CFG1: u32 = 0x20c;
pub const AFE_IRQ26_MCU_CFG0: u32 = 0x210;
pub const AFE_IRQ26_MCU_CFG1: u32 = 0x214;
pub const AFE_CUSTOM_IRQ0_MCU_CFG0: u32 = 0x268;
pub const AFE_IRQ_MCU_MON0: u32 = 0x300;
pub const AFE_IRQ_MCU_MON1: u32 = 0x304;
pub const AFE_IRQ_MCU_MON2: u32 = 0x308;
pub const AFE_IRQ0_CNT_MON: u32 = 0x310;
pub const AFE_IRQ1_CNT_MON: u32 = 0x314;
pub const AFE_IRQ2_CNT_MON: u32 = 0x318;
pub const AFE_IRQ3_CNT_MON: u32 = 0x31c;
pub const AFE_IRQ4_CNT_MON: u32 = 0x320;
pub const AFE_IRQ5_CNT_MON: u32 = 0x324;
pub const AFE_IRQ6_CNT_MON: u32 = 0x328;
pub const AFE_IRQ7_CNT_MON: u32 = 0x32c;
pub const AFE_IRQ8_CNT_MON: u32 = 0x330;
pub const AFE_IRQ9_CNT_MON: u32 = 0x334;
pub const AFE_IRQ10_CNT_MON: u32 = 0x338;
pub const AFE_IRQ11_CNT_MON: u32 = 0x33c;
pub const AFE_IRQ12_CNT_MON: u32 = 0x340;
pub const AFE_IRQ13_CNT_MON: u32 = 0x344;
pub const AFE_IRQ14_CNT_MON: u32 = 0x348;
pub const AFE_IRQ15_CNT_MON: u32 = 0x34c;
pub const AFE_IRQ16_CNT_MON: u32 = 0x350;
pub const AFE_IRQ17_CNT_MON: u32 = 0x354;
pub const AFE_IRQ18_CNT_MON: u32 = 0x358;
pub const AFE_IRQ19_CNT_MON: u32 = 0x35c;
pub const AFE_IRQ20_CNT_MON: u32 = 0x360;
pub const AFE_IRQ21_CNT_MON: u32 = 0x364;
pub const AFE_IRQ22_CNT_MON: u32 = 0x368;
pub const AFE_IRQ23_CNT_MON: u32 = 0x36c;
pub const AFE_IRQ24_CNT_MON: u32 = 0x370;
pub const AFE_IRQ25_CNT_MON: u32 = 0x374;
pub const AFE_IRQ26_CNT_MON: u32 = 0x378;
pub const AFE_CUSTOM_IRQ0_CNT_MON: u32 = 0x390;
pub const AFE_CUSTOM_IRQ0_MCU_CFG1: u32 = 0x3dc;
pub const AFE_GAIN0_CON0: u32 = 0x400;
pub const AFE_GAIN0_CON1_R: u32 = 0x404;
pub const AFE_GAIN0_CON1_L: u32 = 0x408;
pub const AFE_GAIN0_CON2: u32 = 0x40c;
pub const AFE_GAIN0_CON3: u32 = 0x410;
pub const AFE_GAIN0_CUR_R: u32 = 0x414;
pub const AFE_GAIN0_CUR_L: u32 = 0x418;
pub const AFE_GAIN1_CON0: u32 = 0x41c;
pub const AFE_GAIN1_CON1_R: u32 = 0x420;
pub const AFE_GAIN1_CON1_L: u32 = 0x424;
pub const AFE_GAIN1_CON2: u32 = 0x428;
pub const AFE_GAIN1_CON3: u32 = 0x42c;
pub const AFE_GAIN1_CUR_R: u32 = 0x430;
pub const AFE_GAIN1_CUR_L: u32 = 0x434;
pub const AFE_GAIN2_CON0: u32 = 0x438;
pub const AFE_GAIN2_CON1_R: u32 = 0x43c;
pub const AFE_GAIN2_CON1_L: u32 = 0x440;
pub const AFE_GAIN2_CON2: u32 = 0x444;
pub const AFE_GAIN2_CON3: u32 = 0x448;
pub const AFE_GAIN2_CUR_R: u32 = 0x44c;
pub const AFE_GAIN2_CUR_L: u32 = 0x450;
pub const AFE_GAIN3_CON0: u32 = 0x454;
pub const AFE_GAIN3_CON1_R: u32 = 0x458;
pub const AFE_GAIN3_CON1_L: u32 = 0x45c;
pub const AFE_GAIN3_CON2: u32 = 0x460;
pub const AFE_GAIN3_CON3: u32 = 0x464;
pub const AFE_GAIN3_CUR_R: u32 = 0x468;
pub const AFE_GAIN3_CUR_L: u32 = 0x46c;
pub const AFE_STF_CON0: u32 = 0xb80;
pub const AFE_STF_CON1: u32 = 0xb84;
pub const AFE_STF_COEFF: u32 = 0xb88;
pub const AFE_STF_GAIN: u32 = 0xb8c;
pub const AFE_STF_MON: u32 = 0xb90;
pub const AFE_STF_IP_VERSION: u32 = 0xb94;
pub const AFE_CM0_CON0: u32 = 0xba0;
pub const AFE_CM0_MON: u32 = 0xba4;
pub const AFE_CM0_IP_VERSION: u32 = 0xba8;
pub const AFE_CM1_CON0: u32 = 0xbb0;
pub const AFE_CM1_MON: u32 = 0xbb4;
pub const AFE_CM1_IP_VERSION: u32 = 0xbb8;
pub const AFE_CM2_CON0: u32 = 0xbc0;
pub const AFE_CM2_MON: u32 = 0xbc4;
pub const AFE_CM2_IP_VERSION: u32 = 0xbc8;
pub const AFE_ADDA_UL0_SRC_CON0: u32 = 0xbd0;
pub const AFE_ADDA_UL0_SRC_CON1: u32 = 0xbd4;
pub const AFE_ADDA_UL0_SRC_CON2: u32 = 0xbd8;
pub const AFE_ADDA_UL0_SRC_DEBUG: u32 = 0xbdc;
pub const AFE_ADDA_UL0_SRC_DEBUG_MON0: u32 = 0xbe0;
pub const AFE_ADDA_UL0_SRC_MON0: u32 = 0xbe4;
pub const AFE_ADDA_UL0_SRC_MON1: u32 = 0xbe8;
pub const AFE_ADDA_UL0_IIR_COEF_02_01: u32 = 0xbec;
pub const AFE_ADDA_UL0_IIR_COEF_04_03: u32 = 0xbf0;
pub const AFE_ADDA_UL0_IIR_COEF_06_05: u32 = 0xbf4;
pub const AFE_ADDA_UL0_IIR_COEF_08_07: u32 = 0xbf8;
pub const AFE_ADDA_UL0_IIR_COEF_10_09: u32 = 0xbfc;
pub const AFE_ADDA_UL0_ULCF_CFG_02_01: u32 = 0xc00;
pub const AFE_ADDA_UL0_ULCF_CFG_04_03: u32 = 0xc04;
pub const AFE_ADDA_UL0_ULCF_CFG_06_05: u32 = 0xc08;
pub const AFE_ADDA_UL0_ULCF_CFG_08_07: u32 = 0xc0c;
pub const AFE_ADDA_UL0_ULCF_CFG_10_09: u32 = 0xc10;
pub const AFE_ADDA_UL0_ULCF_CFG_12_11: u32 = 0xc14;
pub const AFE_ADDA_UL0_ULCF_CFG_14_13: u32 = 0xc18;
pub const AFE_ADDA_UL0_ULCF_CFG_16_15: u32 = 0xc1c;
pub const AFE_ADDA_UL0_ULCF_CFG_18_17: u32 = 0xc20;
pub const AFE_ADDA_UL0_ULCF_CFG_20_19: u32 = 0xc24;
pub const AFE_ADDA_UL0_ULCF_CFG_22_21: u32 = 0xc28;
pub const AFE_ADDA_UL0_ULCF_CFG_24_23: u32 = 0xc2c;
pub const AFE_ADDA_UL0_ULCF_CFG_26_25: u32 = 0xc30;
pub const AFE_ADDA_UL0_ULCF_CFG_28_27: u32 = 0xc34;
pub const AFE_ADDA_UL0_ULCF_CFG_30_29: u32 = 0xc38;
pub const AFE_ADDA_UL0_ULCF_CFG_32_31: u32 = 0xc3c;
pub const AFE_ADDA_UL0_IP_VERSION: u32 = 0xc4c;
pub const AFE_ADDA_UL1_SRC_CON0: u32 = 0xc50;
pub const AFE_ADDA_UL1_SRC_CON1: u32 = 0xc54;
pub const AFE_ADDA_UL1_SRC_CON2: u32 = 0xc58;
pub const AFE_ADDA_UL1_SRC_DEBUG: u32 = 0xc5c;
pub const AFE_ADDA_UL1_SRC_DEBUG_MON0: u32 = 0xc60;
pub const AFE_ADDA_UL1_SRC_MON0: u32 = 0xc64;
pub const AFE_ADDA_UL1_SRC_MON1: u32 = 0xc68;
pub const AFE_ADDA_UL1_IIR_COEF_02_01: u32 = 0xc6c;
pub const AFE_ADDA_UL1_IIR_COEF_04_03: u32 = 0xc70;
pub const AFE_ADDA_UL1_IIR_COEF_06_05: u32 = 0xc74;
pub const AFE_ADDA_UL1_IIR_COEF_08_07: u32 = 0xc78;
pub const AFE_ADDA_UL1_IIR_COEF_10_09: u32 = 0xc7c;
pub const AFE_ADDA_UL1_ULCF_CFG_02_01: u32 = 0xc80;
pub const AFE_ADDA_UL1_ULCF_CFG_04_03: u32 = 0xc84;
pub const AFE_ADDA_UL1_ULCF_CFG_06_05: u32 = 0xc88;
pub const AFE_ADDA_UL1_ULCF_CFG_08_07: u32 = 0xc8c;
pub const AFE_ADDA_UL1_ULCF_CFG_10_09: u32 = 0xc90;
pub const AFE_ADDA_UL1_ULCF_CFG_12_11: u32 = 0xc94;
pub const AFE_ADDA_UL1_ULCF_CFG_14_13: u32 = 0xc98;
pub const AFE_ADDA_UL1_ULCF_CFG_16_15: u32 = 0xc9c;
pub const AFE_ADDA_UL1_ULCF_CFG_18_17: u32 = 0xca0;
pub const AFE_ADDA_UL1_ULCF_CFG_20_19: u32 = 0xca4;
pub const AFE_ADDA_UL1_ULCF_CFG_22_21: u32 = 0xca8;
pub const AFE_ADDA_UL1_ULCF_CFG_24_23: u32 = 0xcac;
pub const AFE_ADDA_UL1_ULCF_CFG_26_25: u32 = 0xcb0;
pub const AFE_ADDA_UL1_ULCF_CFG_28_27: u32 = 0xcb4;
pub const AFE_ADDA_UL1_ULCF_CFG_30_29: u32 = 0xcb8;
pub const AFE_ADDA_UL1_ULCF_CFG_32_31: u32 = 0xcbc;
pub const AFE_ADDA_UL1_IP_VERSION: u32 = 0xccc;
pub const AFE_ADDA_UL2_SRC_CON0: u32 = 0xcd0;
pub const AFE_ADDA_UL2_SRC_CON1: u32 = 0xcd4;
pub const AFE_ADDA_UL2_SRC_CON2: u32 = 0xcd8;
pub const AFE_ADDA_UL2_SRC_DEBUG: u32 = 0xcdc;
pub const AFE_ADDA_UL2_SRC_DEBUG_MON0: u32 = 0xce0;
pub const AFE_ADDA_UL2_SRC_MON0: u32 = 0xce4;
pub const AFE_ADDA_UL2_SRC_MON1: u32 = 0xce8;
pub const AFE_ADDA_UL2_IIR_COEF_02_01: u32 = 0xcec;
pub const AFE_ADDA_UL2_IIR_COEF_04_03: u32 = 0xcf0;
pub const AFE_ADDA_UL2_IIR_COEF_06_05: u32 = 0xcf4;
pub const AFE_ADDA_UL2_IIR_COEF_08_07: u32 = 0xcf8;
pub const AFE_ADDA_UL2_IIR_COEF_10_09: u32 = 0xcfc;
pub const AFE_ADDA_UL2_ULCF_CFG_02_01: u32 = 0xd00;
pub const AFE_ADDA_UL2_ULCF_CFG_04_03: u32 = 0xd04;
pub const AFE_ADDA_UL2_ULCF_CFG_06_05: u32 = 0xd08;
pub const AFE_ADDA_UL2_ULCF_CFG_08_07: u32 = 0xd0c;
pub const AFE_ADDA_UL2_ULCF_CFG_10_09: u32 = 0xd10;
pub const AFE_ADDA_UL2_ULCF_CFG_12_11: u32 = 0xd14;
pub const AFE_ADDA_UL2_ULCF_CFG_14_13: u32 = 0xd18;
pub const AFE_ADDA_UL2_ULCF_CFG_16_15: u32 = 0xd1c;
pub const AFE_ADDA_UL2_ULCF_CFG_18_17: u32 = 0xd20;
pub const AFE_ADDA_UL2_ULCF_CFG_20_19: u32 = 0xd24;
pub const AFE_ADDA_UL2_ULCF_CFG_22_21: u32 = 0xd28;
pub const AFE_ADDA_UL2_ULCF_CFG_24_23: u32 = 0xd2c;
pub const AFE_ADDA_UL2_ULCF_CFG_26_25: u32 = 0xd30;
pub const AFE_ADDA_UL2_ULCF_CFG_28_27: u32 = 0xd34;
pub const AFE_ADDA_UL2_ULCF_CFG_30_29: u32 = 0xd38;
pub const AFE_ADDA_UL2_ULCF_CFG_32_31: u32 = 0xd3c;
pub const AFE_ADDA_UL2_IP_VERSION: u32 = 0xd4c;
pub const AFE_ADDA_PROXIMITY_CON0: u32 = 0xed0;
pub const AFE_ADDA_ULSRC_PHASE_CON0: u32 = 0xf00;
pub const AFE_ADDA_ULSRC_PHASE_CON1: u32 = 0xf04;
pub const AFE_ADDA_ULSRC_PHASE_CON2: u32 = 0xf08;
pub const AFE_ADDA_ULSRC_PHASE_CON3: u32 = 0xf0c;
pub const AFE_MTKAIF_IPM_VER_MON: u32 = 0x1180;
pub const AFE_MTKAIF_MON_SEL: u32 = 0x1184;
pub const AFE_MTKAIF_MON: u32 = 0x1188;
pub const AFE_MTKAIF0_CFG0: u32 = 0x1190;
pub const AFE_MTKAIF0_TX_CFG0: u32 = 0x1194;
pub const AFE_MTKAIF0_RX_CFG0: u32 = 0x1198;
pub const AFE_MTKAIF0_RX_CFG1: u32 = 0x119c;
pub const AFE_MTKAIF0_RX_CFG2: u32 = 0x11a0;
pub const AFE_MTKAIF1_CFG0: u32 = 0x11f0;
pub const AFE_MTKAIF1_TX_CFG0: u32 = 0x11f4;
pub const AFE_MTKAIF1_RX_CFG0: u32 = 0x11f8;
pub const AFE_MTKAIF1_RX_CFG1: u32 = 0x11fc;
pub const AFE_MTKAIF1_RX_CFG2: u32 = 0x1200;
pub const AFE_AUD_PAD_TOP_CFG0: u32 = 0x1204;
pub const AFE_AUD_PAD_TOP_MON: u32 = 0x1208;
pub const AFE_ADDA_MTKAIFV4_TX_CFG0: u32 = 0x1280;
pub const AFE_ADDA6_MTKAIFV4_TX_CFG0: u32 = 0x1284;
pub const AFE_ADDA_MTKAIFV4_RX_CFG0: u32 = 0x1288;
pub const AFE_ADDA_MTKAIFV4_RX_CFG1: u32 = 0x128c;
pub const AFE_ADDA6_MTKAIFV4_RX_CFG0: u32 = 0x1290;
pub const AFE_ADDA6_MTKAIFV4_RX_CFG1: u32 = 0x1294;
pub const AFE_ADDA_MTKAIFV4_TX_SYNCWORD_CFG: u32 = 0x1298;
pub const AFE_ADDA_MTKAIFV4_RX_SYNCWORD_CFG: u32 = 0x129c;
pub const AFE_ADDA_MTKAIFV4_MON0: u32 = 0x12a0;
pub const AFE_ADDA_MTKAIFV4_MON1: u32 = 0x12a4;
pub const AFE_ADDA6_MTKAIFV4_MON0: u32 = 0x12a8;
pub const ETDM_IN0_CON0: u32 = 0x1300;
pub const ETDM_IN0_CON1: u32 = 0x1304;
pub const ETDM_IN0_CON2: u32 = 0x1308;
pub const ETDM_IN0_CON3: u32 = 0x130c;
pub const ETDM_IN0_CON4: u32 = 0x1310;
pub const ETDM_IN0_CON5: u32 = 0x1314;
pub const ETDM_IN0_CON6: u32 = 0x1318;
pub const ETDM_IN0_CON7: u32 = 0x131c;
pub const ETDM_IN0_CON8: u32 = 0x1320;
pub const ETDM_IN0_CON9: u32 = 0x1324;
pub const ETDM_IN0_MON: u32 = 0x1328;
pub const ETDM_IN1_CON0: u32 = 0x1330;
pub const ETDM_IN1_CON1: u32 = 0x1334;
pub const ETDM_IN1_CON2: u32 = 0x1338;
pub const ETDM_IN1_CON3: u32 = 0x133c;
pub const ETDM_IN1_CON4: u32 = 0x1340;
pub const ETDM_IN1_CON5: u32 = 0x1344;
pub const ETDM_IN1_CON6: u32 = 0x1348;
pub const ETDM_IN1_CON7: u32 = 0x134c;
pub const ETDM_IN1_CON8: u32 = 0x1350;
pub const ETDM_IN1_CON9: u32 = 0x1354;
pub const ETDM_IN1_MON: u32 = 0x1358;
pub const ETDM_IN2_CON0: u32 = 0x1360;
pub const ETDM_IN2_CON1: u32 = 0x1364;
pub const ETDM_IN2_CON2: u32 = 0x1368;
pub const ETDM_IN2_CON3: u32 = 0x136c;
pub const ETDM_IN2_CON4: u32 = 0x1370;
pub const ETDM_IN2_CON5: u32 = 0x1374;
pub const ETDM_IN2_CON6: u32 = 0x1378;
pub const ETDM_IN2_CON7: u32 = 0x137c;
pub const ETDM_IN2_CON8: u32 = 0x1380;
pub const ETDM_IN2_CON9: u32 = 0x1384;
pub const ETDM_IN2_MON: u32 = 0x1388;
pub const ETDM_IN3_CON0: u32 = 0x1390;
pub const ETDM_IN3_CON1: u32 = 0x1394;
pub const ETDM_IN3_CON2: u32 = 0x1398;
pub const ETDM_IN3_CON3: u32 = 0x139c;
pub const ETDM_IN3_CON4: u32 = 0x13a0;
pub const ETDM_IN3_CON5: u32 = 0x13a4;
pub const ETDM_IN3_CON6: u32 = 0x13a8;
pub const ETDM_IN3_CON7: u32 = 0x13ac;
pub const ETDM_IN3_CON8: u32 = 0x13b0;
pub const ETDM_IN3_CON9: u32 = 0x13b4;
pub const ETDM_IN3_MON: u32 = 0x13b8;
pub const ETDM_IN4_CON0: u32 = 0x13c0;
pub const ETDM_IN4_CON1: u32 = 0x13c4;
pub const ETDM_IN4_CON2: u32 = 0x13c8;
pub const ETDM_IN4_CON3: u32 = 0x13cc;
pub const ETDM_IN4_CON4: u32 = 0x13d0;
pub const ETDM_IN4_CON5: u32 = 0x13d4;
pub const ETDM_IN4_CON6: u32 = 0x13d8;
pub const ETDM_IN4_CON7: u32 = 0x13dc;
pub const ETDM_IN4_CON8: u32 = 0x13e0;
pub const ETDM_IN4_CON9: u32 = 0x13e4;
pub const ETDM_IN4_MON: u32 = 0x13e8;
pub const ETDM_IN5_CON0: u32 = 0x13f0;
pub const ETDM_IN5_CON1: u32 = 0x13f4;
pub const ETDM_IN5_CON2: u32 = 0x13f8;
pub const ETDM_IN5_CON3: u32 = 0x13fc;
pub const ETDM_IN5_CON4: u32 = 0x1400;
pub const ETDM_IN5_CON5: u32 = 0x1404;
pub const ETDM_IN5_CON6: u32 = 0x1408;
pub const ETDM_IN5_CON7: u32 = 0x140c;
pub const ETDM_IN5_CON8: u32 = 0x1410;
pub const ETDM_IN5_CON9: u32 = 0x1414;
pub const ETDM_IN5_MON: u32 = 0x1418;
pub const ETDM_IN6_CON0: u32 = 0x1420;
pub const ETDM_IN6_CON1: u32 = 0x1424;
pub const ETDM_IN6_CON2: u32 = 0x1428;
pub const ETDM_IN6_CON3: u32 = 0x142c;
pub const ETDM_IN6_CON4: u32 = 0x1430;
pub const ETDM_IN6_CON5: u32 = 0x1434;
pub const ETDM_IN6_CON6: u32 = 0x1438;
pub const ETDM_IN6_CON7: u32 = 0x143c;
pub const ETDM_IN6_CON8: u32 = 0x1440;
pub const ETDM_IN6_CON9: u32 = 0x1444;
pub const ETDM_IN6_MON: u32 = 0x1448;
pub const ETDM_OUT0_CON0: u32 = 0x1480;
pub const ETDM_OUT0_CON1: u32 = 0x1484;
pub const ETDM_OUT0_CON2: u32 = 0x1488;
pub const ETDM_OUT0_CON3: u32 = 0x148c;
pub const ETDM_OUT0_CON4: u32 = 0x1490;
pub const ETDM_OUT0_CON5: u32 = 0x1494;
pub const ETDM_OUT0_CON6: u32 = 0x1498;
pub const ETDM_OUT0_CON7: u32 = 0x149c;
pub const ETDM_OUT0_CON8: u32 = 0x14a0;
pub const ETDM_OUT0_CON9: u32 = 0x14a4;
pub const ETDM_OUT0_MON: u32 = 0x14a8;
pub const ETDM_OUT1_CON0: u32 = 0x14c0;
pub const ETDM_OUT1_CON1: u32 = 0x14c4;
pub const ETDM_OUT1_CON2: u32 = 0x14c8;
pub const ETDM_OUT1_CON3: u32 = 0x14cc;
pub const ETDM_OUT1_CON4: u32 = 0x14d0;
pub const ETDM_OUT1_CON5: u32 = 0x14d4;
pub const ETDM_OUT1_CON6: u32 = 0x14d8;
pub const ETDM_OUT1_CON7: u32 = 0x14dc;
pub const ETDM_OUT1_CON8: u32 = 0x14e0;
pub const ETDM_OUT1_CON9: u32 = 0x14e4;
pub const ETDM_OUT1_MON: u32 = 0x14e8;
pub const ETDM_OUT2_CON0: u32 = 0x1500;
pub const ETDM_OUT2_CON1: u32 = 0x1504;
pub const ETDM_OUT2_CON2: u32 = 0x1508;
pub const ETDM_OUT2_CON3: u32 = 0x150c;
pub const ETDM_OUT2_CON4: u32 = 0x1510;
pub const ETDM_OUT2_CON5: u32 = 0x1514;
pub const ETDM_OUT2_CON6: u32 = 0x1518;
pub const ETDM_OUT2_CON7: u32 = 0x151c;
pub const ETDM_OUT2_CON8: u32 = 0x1520;
pub const ETDM_OUT2_CON9: u32 = 0x1524;
pub const ETDM_OUT2_MON: u32 = 0x1528;
pub const ETDM_OUT3_CON0: u32 = 0x1540;
pub const ETDM_OUT3_CON1: u32 = 0x1544;
pub const ETDM_OUT3_CON2: u32 = 0x1548;
pub const ETDM_OUT3_CON3: u32 = 0x154c;
pub const ETDM_OUT3_CON4: u32 = 0x1550;
pub const ETDM_OUT3_CON5: u32 = 0x1554;
pub const ETDM_OUT3_CON6: u32 = 0x1558;
pub const ETDM_OUT3_CON7: u32 = 0x155c;
pub const ETDM_OUT3_CON8: u32 = 0x1560;
pub const ETDM_OUT3_CON9: u32 = 0x1564;
pub const ETDM_OUT3_MON: u32 = 0x1568;
pub const ETDM_OUT4_CON0: u32 = 0x1580;
pub const ETDM_OUT4_CON1: u32 = 0x1584;
pub const ETDM_OUT4_CON2: u32 = 0x1588;
pub const ETDM_OUT4_CON3: u32 = 0x158c;
pub const ETDM_OUT4_CON4: u32 = 0x1590;
pub const ETDM_OUT4_CON5: u32 = 0x1594;
pub const ETDM_OUT4_CON6: u32 = 0x1598;
pub const ETDM_OUT4_CON7: u32 = 0x159c;
pub const ETDM_OUT4_CON8: u32 = 0x15a0;
pub const ETDM_OUT4_CON9: u32 = 0x15a4;
pub const ETDM_OUT4_MON: u32 = 0x15a8;
pub const ETDM_OUT5_CON0: u32 = 0x15c0;
pub const ETDM_OUT5_CON1: u32 = 0x15c4;
pub const ETDM_OUT5_CON2: u32 = 0x15c8;
pub const ETDM_OUT5_CON3: u32 = 0x15cc;
pub const ETDM_OUT5_CON4: u32 = 0x15d0;
pub const ETDM_OUT5_CON5: u32 = 0x15d4;
pub const ETDM_OUT5_CON6: u32 = 0x15d8;
pub const ETDM_OUT5_CON7: u32 = 0x15dc;
pub const ETDM_OUT5_CON8: u32 = 0x15e0;
pub const ETDM_OUT5_CON9: u32 = 0x15e4;
pub const ETDM_OUT5_MON: u32 = 0x15e8;
pub const ETDM_OUT6_CON0: u32 = 0x1600;
pub const ETDM_OUT6_CON1: u32 = 0x1604;
pub const ETDM_OUT6_CON2: u32 = 0x1608;
pub const ETDM_OUT6_CON3: u32 = 0x160c;
pub const ETDM_OUT6_CON4: u32 = 0x1610;
pub const ETDM_OUT6_CON5: u32 = 0x1614;
pub const ETDM_OUT6_CON6: u32 = 0x1618;
pub const ETDM_OUT6_CON7: u32 = 0x161c;
pub const ETDM_OUT6_CON8: u32 = 0x1620;
pub const ETDM_OUT6_CON9: u32 = 0x1624;
pub const ETDM_OUT6_MON: u32 = 0x1628;
pub const ETDM_0_3_COWORK_CON0: u32 = 0x1680;
pub const ETDM_0_3_COWORK_CON1: u32 = 0x1684;
pub const ETDM_0_3_COWORK_CON2: u32 = 0x1688;
pub const ETDM_0_3_COWORK_CON3: u32 = 0x168c;
pub const ETDM_4_7_COWORK_CON0: u32 = 0x1690;
pub const ETDM_4_7_COWORK_CON1: u32 = 0x1694;
pub const ETDM_4_7_COWORK_CON2: u32 = 0x1698;
pub const ETDM_4_7_COWORK_CON3: u32 = 0x169c;
pub const AFE_DPTX_CON: u32 = 0x2040;
pub const AFE_DPTX_MON: u32 = 0x2044;
pub const AFE_TDM_CON1: u32 = 0x2048;
pub const AFE_TDM_CON2: u32 = 0x204c;
pub const AFE_TDM_CON3: u32 = 0x2050;
pub const AFE_TDM_OUT_MON: u32 = 0x2054;
pub const AFE_HDMI_CONN0: u32 = 0x2078;
pub const AFE_TDM_TOP_IP_VERSION: u32 = 0x207c;
pub const AFE_CONN004_0: u32 = 0x2100;
pub const AFE_CONN004_1: u32 = 0x2104;
pub const AFE_CONN004_2: u32 = 0x2108;
pub const AFE_CONN004_4: u32 = 0x2110;
pub const AFE_CONN004_5: u32 = 0x2114;
pub const AFE_CONN004_6: u32 = 0x2118;
pub const AFE_CONN004_7: u32 = 0x211c;
pub const AFE_CONN005_0: u32 = 0x2120;
pub const AFE_CONN005_1: u32 = 0x2124;
pub const AFE_CONN005_2: u32 = 0x2128;
pub const AFE_CONN005_4: u32 = 0x2130;
pub const AFE_CONN005_5: u32 = 0x2134;
pub const AFE_CONN005_6: u32 = 0x2138;
pub const AFE_CONN005_7: u32 = 0x213c;
pub const AFE_CONN006_0: u32 = 0x2140;
pub const AFE_CONN006_1: u32 = 0x2144;
pub const AFE_CONN006_2: u32 = 0x2148;
pub const AFE_CONN006_4: u32 = 0x2150;
pub const AFE_CONN006_5: u32 = 0x2154;
pub const AFE_CONN006_6: u32 = 0x2158;
pub const AFE_CONN006_7: u32 = 0x215c;
pub const AFE_CONN007_0: u32 = 0x2160;
pub const AFE_CONN007_1: u32 = 0x2164;
pub const AFE_CONN007_2: u32 = 0x2168;
pub const AFE_CONN007_4: u32 = 0x2170;
pub const AFE_CONN007_5: u32 = 0x2174;
pub const AFE_CONN007_6: u32 = 0x2178;
pub const AFE_CONN007_7: u32 = 0x217c;
pub const AFE_CONN008_0: u32 = 0x2180;
pub const AFE_CONN008_1: u32 = 0x2184;
pub const AFE_CONN008_2: u32 = 0x2188;
pub const AFE_CONN008_4: u32 = 0x2190;
pub const AFE_CONN008_5: u32 = 0x2194;
pub const AFE_CONN008_6: u32 = 0x2198;
pub const AFE_CONN008_7: u32 = 0x219c;
pub const AFE_CONN009_0: u32 = 0x21a0;
pub const AFE_CONN009_1: u32 = 0x21a4;
pub const AFE_CONN009_2: u32 = 0x21a8;
pub const AFE_CONN009_4: u32 = 0x21b0;
pub const AFE_CONN009_5: u32 = 0x21b4;
pub const AFE_CONN009_6: u32 = 0x21b8;
pub const AFE_CONN009_7: u32 = 0x21bc;
pub const AFE_CONN010_0: u32 = 0x21c0;
pub const AFE_CONN010_1: u32 = 0x21c4;
pub const AFE_CONN010_2: u32 = 0x21c8;
pub const AFE_CONN010_4: u32 = 0x21d0;
pub const AFE_CONN010_5: u32 = 0x21d4;
pub const AFE_CONN010_6: u32 = 0x21d8;
pub const AFE_CONN010_7: u32 = 0x21dc;
pub const AFE_CONN011_0: u32 = 0x21e0;
pub const AFE_CONN011_1: u32 = 0x21e4;
pub const AFE_CONN011_2: u32 = 0x21e8;
pub const AFE_CONN011_4: u32 = 0x21f0;
pub const AFE_CONN011_5: u32 = 0x21f4;
pub const AFE_CONN011_6: u32 = 0x21f8;
pub const AFE_CONN011_7: u32 = 0x21fc;
pub const AFE_CONN012_0: u32 = 0x2200;
pub const AFE_CONN012_1: u32 = 0x2204;
pub const AFE_CONN012_2: u32 = 0x2208;
pub const AFE_CONN012_4: u32 = 0x2210;
pub const AFE_CONN012_5: u32 = 0x2214;
pub const AFE_CONN012_6: u32 = 0x2218;
pub const AFE_CONN012_7: u32 = 0x221c;
pub const AFE_CONN014_0: u32 = 0x2240;
pub const AFE_CONN014_1: u32 = 0x2244;
pub const AFE_CONN014_2: u32 = 0x2248;
pub const AFE_CONN014_4: u32 = 0x2250;
pub const AFE_CONN014_5: u32 = 0x2254;
pub const AFE_CONN014_6: u32 = 0x2258;
pub const AFE_CONN014_7: u32 = 0x225c;
pub const AFE_CONN015_0: u32 = 0x2260;
pub const AFE_CONN015_1: u32 = 0x2264;
pub const AFE_CONN015_2: u32 = 0x2268;
pub const AFE_CONN015_4: u32 = 0x2270;
pub const AFE_CONN015_5: u32 = 0x2274;
pub const AFE_CONN015_6: u32 = 0x2278;
pub const AFE_CONN015_7: u32 = 0x227c;
pub const AFE_CONN016_0: u32 = 0x2280;
pub const AFE_CONN016_1: u32 = 0x2284;
pub const AFE_CONN016_2: u32 = 0x2288;
pub const AFE_CONN016_4: u32 = 0x2290;
pub const AFE_CONN016_5: u32 = 0x2294;
pub const AFE_CONN016_6: u32 = 0x2298;
pub const AFE_CONN016_7: u32 = 0x229c;
pub const AFE_CONN017_0: u32 = 0x22a0;
pub const AFE_CONN017_1: u32 = 0x22a4;
pub const AFE_CONN017_2: u32 = 0x22a8;
pub const AFE_CONN017_4: u32 = 0x22b0;
pub const AFE_CONN017_5: u32 = 0x22b4;
pub const AFE_CONN017_6: u32 = 0x22b8;
pub const AFE_CONN017_7: u32 = 0x22bc;
pub const AFE_CONN018_0: u32 = 0x22c0;
pub const AFE_CONN018_1: u32 = 0x22c4;
pub const AFE_CONN018_2: u32 = 0x22c8;
pub const AFE_CONN018_4: u32 = 0x22d0;
pub const AFE_CONN018_5: u32 = 0x22d4;
pub const AFE_CONN018_6: u32 = 0x22d8;
pub const AFE_CONN018_7: u32 = 0x22dc;
pub const AFE_CONN019_0: u32 = 0x22e0;
pub const AFE_CONN019_1: u32 = 0x22e4;
pub const AFE_CONN019_2: u32 = 0x22e8;
pub const AFE_CONN019_4: u32 = 0x22f0;
pub const AFE_CONN019_5: u32 = 0x22f4;
pub const AFE_CONN019_6: u32 = 0x22f8;
pub const AFE_CONN019_7: u32 = 0x22fc;
pub const AFE_CONN020_0: u32 = 0x2300;
pub const AFE_CONN020_1: u32 = 0x2304;
pub const AFE_CONN020_2: u32 = 0x2308;
pub const AFE_CONN020_4: u32 = 0x2310;
pub const AFE_CONN020_5: u32 = 0x2314;
pub const AFE_CONN020_6: u32 = 0x2318;
pub const AFE_CONN020_7: u32 = 0x231c;
pub const AFE_CONN021_0: u32 = 0x2320;
pub const AFE_CONN021_1: u32 = 0x2324;
pub const AFE_CONN021_2: u32 = 0x2328;
pub const AFE_CONN021_4: u32 = 0x2330;
pub const AFE_CONN021_5: u32 = 0x2334;
pub const AFE_CONN021_6: u32 = 0x2338;
pub const AFE_CONN021_7: u32 = 0x233c;
pub const AFE_CONN022_0: u32 = 0x2340;
pub const AFE_CONN022_1: u32 = 0x2344;
pub const AFE_CONN022_2: u32 = 0x2348;
pub const AFE_CONN022_4: u32 = 0x2350;
pub const AFE_CONN022_5: u32 = 0x2354;
pub const AFE_CONN022_6: u32 = 0x2358;
pub const AFE_CONN022_7: u32 = 0x235c;
pub const AFE_CONN023_0: u32 = 0x2360;
pub const AFE_CONN023_1: u32 = 0x2364;
pub const AFE_CONN023_2: u32 = 0x2368;
pub const AFE_CONN023_4: u32 = 0x2370;
pub const AFE_CONN023_5: u32 = 0x2374;
pub const AFE_CONN023_6: u32 = 0x2378;
pub const AFE_CONN023_7: u32 = 0x237c;
pub const AFE_CONN024_0: u32 = 0x2380;
pub const AFE_CONN024_1: u32 = 0x2384;
pub const AFE_CONN024_2: u32 = 0x2388;
pub const AFE_CONN024_4: u32 = 0x2390;
pub const AFE_CONN024_5: u32 = 0x2394;
pub const AFE_CONN024_6: u32 = 0x2398;
pub const AFE_CONN024_7: u32 = 0x239c;
pub const AFE_CONN025_0: u32 = 0x23a0;
pub const AFE_CONN025_1: u32 = 0x23a4;
pub const AFE_CONN025_2: u32 = 0x23a8;
pub const AFE_CONN025_4: u32 = 0x23b0;
pub const AFE_CONN025_5: u32 = 0x23b4;
pub const AFE_CONN025_6: u32 = 0x23b8;
pub const AFE_CONN025_7: u32 = 0x23bc;
pub const AFE_CONN026_0: u32 = 0x23c0;
pub const AFE_CONN026_1: u32 = 0x23c4;
pub const AFE_CONN026_2: u32 = 0x23c8;
pub const AFE_CONN026_4: u32 = 0x23d0;
pub const AFE_CONN026_5: u32 = 0x23d4;
pub const AFE_CONN026_6: u32 = 0x23d8;
pub const AFE_CONN026_7: u32 = 0x23dc;
pub const AFE_CONN027_0: u32 = 0x23e0;
pub const AFE_CONN027_1: u32 = 0x23e4;
pub const AFE_CONN027_2: u32 = 0x23e8;
pub const AFE_CONN027_4: u32 = 0x23f0;
pub const AFE_CONN027_5: u32 = 0x23f4;
pub const AFE_CONN027_6: u32 = 0x23f8;
pub const AFE_CONN027_7: u32 = 0x23fc;
pub const AFE_CONN028_0: u32 = 0x2400;
pub const AFE_CONN028_1: u32 = 0x2404;
pub const AFE_CONN028_2: u32 = 0x2408;
pub const AFE_CONN028_4: u32 = 0x2410;
pub const AFE_CONN028_5: u32 = 0x2414;
pub const AFE_CONN028_6: u32 = 0x2418;
pub const AFE_CONN028_7: u32 = 0x241c;
pub const AFE_CONN029_0: u32 = 0x2420;
pub const AFE_CONN029_1: u32 = 0x2424;
pub const AFE_CONN029_2: u32 = 0x2428;
pub const AFE_CONN029_4: u32 = 0x2430;
pub const AFE_CONN029_5: u32 = 0x2434;
pub const AFE_CONN029_6: u32 = 0x2438;
pub const AFE_CONN029_7: u32 = 0x243c;
pub const AFE_CONN030_0: u32 = 0x2440;
pub const AFE_CONN030_1: u32 = 0x2444;
pub const AFE_CONN030_2: u32 = 0x2448;
pub const AFE_CONN030_4: u32 = 0x2450;
pub const AFE_CONN030_5: u32 = 0x2454;
pub const AFE_CONN030_6: u32 = 0x2458;
pub const AFE_CONN030_7: u32 = 0x245c;
pub const AFE_CONN031_0: u32 = 0x2460;
pub const AFE_CONN031_1: u32 = 0x2464;
pub const AFE_CONN031_2: u32 = 0x2468;
pub const AFE_CONN031_4: u32 = 0x2470;
pub const AFE_CONN031_5: u32 = 0x2474;
pub const AFE_CONN031_6: u32 = 0x2478;
pub const AFE_CONN031_7: u32 = 0x247c;
pub const AFE_CONN032_0: u32 = 0x2480;
pub const AFE_CONN032_1: u32 = 0x2484;
pub const AFE_CONN032_2: u32 = 0x2488;
pub const AFE_CONN032_4: u32 = 0x2490;
pub const AFE_CONN032_5: u32 = 0x2494;
pub const AFE_CONN032_6: u32 = 0x2498;
pub const AFE_CONN032_7: u32 = 0x249c;
pub const AFE_CONN033_0: u32 = 0x24a0;
pub const AFE_CONN033_1: u32 = 0x24a4;
pub const AFE_CONN033_2: u32 = 0x24a8;
pub const AFE_CONN033_4: u32 = 0x24b0;
pub const AFE_CONN033_5: u32 = 0x24b4;
pub const AFE_CONN033_6: u32 = 0x24b8;
pub const AFE_CONN033_7: u32 = 0x24bc;
pub const AFE_CONN034_0: u32 = 0x24c0;
pub const AFE_CONN034_1: u32 = 0x24c4;
pub const AFE_CONN034_2: u32 = 0x24c8;
pub const AFE_CONN034_4: u32 = 0x24d0;
pub const AFE_CONN034_5: u32 = 0x24d4;
pub const AFE_CONN034_6: u32 = 0x24d8;
pub const AFE_CONN034_7: u32 = 0x24dc;
pub const AFE_CONN035_0: u32 = 0x24e0;
pub const AFE_CONN035_1: u32 = 0x24e4;
pub const AFE_CONN035_2: u32 = 0x24e8;
pub const AFE_CONN035_4: u32 = 0x24f0;
pub const AFE_CONN035_5: u32 = 0x24f4;
pub const AFE_CONN035_6: u32 = 0x24f8;
pub const AFE_CONN035_7: u32 = 0x24fc;
pub const AFE_CONN036_0: u32 = 0x2500;
pub const AFE_CONN036_1: u32 = 0x2504;
pub const AFE_CONN036_2: u32 = 0x2508;
pub const AFE_CONN036_4: u32 = 0x2510;
pub const AFE_CONN036_5: u32 = 0x2514;
pub const AFE_CONN036_6: u32 = 0x2518;
pub const AFE_CONN036_7: u32 = 0x251c;
pub const AFE_CONN037_0: u32 = 0x2520;
pub const AFE_CONN037_1: u32 = 0x2524;
pub const AFE_CONN037_2: u32 = 0x2528;
pub const AFE_CONN037_4: u32 = 0x2530;
pub const AFE_CONN037_5: u32 = 0x2534;
pub const AFE_CONN037_6: u32 = 0x2538;
pub const AFE_CONN037_7: u32 = 0x253c;
pub const AFE_CONN038_0: u32 = 0x2540;
pub const AFE_CONN038_1: u32 = 0x2544;
pub const AFE_CONN038_2: u32 = 0x2548;
pub const AFE_CONN038_4: u32 = 0x2550;
pub const AFE_CONN038_5: u32 = 0x2554;
pub const AFE_CONN038_6: u32 = 0x2558;
pub const AFE_CONN038_7: u32 = 0x255c;
pub const AFE_CONN039_0: u32 = 0x2560;
pub const AFE_CONN039_1: u32 = 0x2564;
pub const AFE_CONN039_2: u32 = 0x2568;
pub const AFE_CONN039_4: u32 = 0x2570;
pub const AFE_CONN039_5: u32 = 0x2574;
pub const AFE_CONN039_6: u32 = 0x2578;
pub const AFE_CONN039_7: u32 = 0x257c;
pub const AFE_CONN040_0: u32 = 0x2580;
pub const AFE_CONN040_1: u32 = 0x2584;
pub const AFE_CONN040_2: u32 = 0x2588;
pub const AFE_CONN040_4: u32 = 0x2590;
pub const AFE_CONN040_5: u32 = 0x2594;
pub const AFE_CONN040_6: u32 = 0x2598;
pub const AFE_CONN040_7: u32 = 0x259c;
pub const AFE_CONN041_0: u32 = 0x25a0;
pub const AFE_CONN041_1: u32 = 0x25a4;
pub const AFE_CONN041_2: u32 = 0x25a8;
pub const AFE_CONN041_4: u32 = 0x25b0;
pub const AFE_CONN041_5: u32 = 0x25b4;
pub const AFE_CONN041_6: u32 = 0x25b8;
pub const AFE_CONN041_7: u32 = 0x25bc;
pub const AFE_CONN042_0: u32 = 0x25c0;
pub const AFE_CONN042_1: u32 = 0x25c4;
pub const AFE_CONN042_2: u32 = 0x25c8;
pub const AFE_CONN042_4: u32 = 0x25d0;
pub const AFE_CONN042_5: u32 = 0x25d4;
pub const AFE_CONN042_6: u32 = 0x25d8;
pub const AFE_CONN042_7: u32 = 0x25dc;
pub const AFE_CONN043_0: u32 = 0x25e0;
pub const AFE_CONN043_1: u32 = 0x25e4;
pub const AFE_CONN043_2: u32 = 0x25e8;
pub const AFE_CONN043_4: u32 = 0x25f0;
pub const AFE_CONN043_5: u32 = 0x25f4;
pub const AFE_CONN043_6: u32 = 0x25f8;
pub const AFE_CONN043_7: u32 = 0x25fc;
pub const AFE_CONN044_0: u32 = 0x2600;
pub const AFE_CONN044_1: u32 = 0x2604;
pub const AFE_CONN044_2: u32 = 0x2608;
pub const AFE_CONN044_4: u32 = 0x2610;
pub const AFE_CONN044_5: u32 = 0x2614;
pub const AFE_CONN044_6: u32 = 0x2618;
pub const AFE_CONN044_7: u32 = 0x261c;
pub const AFE_CONN045_0: u32 = 0x2620;
pub const AFE_CONN045_1: u32 = 0x2624;
pub const AFE_CONN045_2: u32 = 0x2628;
pub const AFE_CONN045_4: u32 = 0x2630;
pub const AFE_CONN045_5: u32 = 0x2634;
pub const AFE_CONN045_6: u32 = 0x2638;
pub const AFE_CONN045_7: u32 = 0x263c;
pub const AFE_CONN046_0: u32 = 0x2640;
pub const AFE_CONN046_1: u32 = 0x2644;
pub const AFE_CONN046_2: u32 = 0x2648;
pub const AFE_CONN046_4: u32 = 0x2650;
pub const AFE_CONN046_5: u32 = 0x2654;
pub const AFE_CONN046_6: u32 = 0x2658;
pub const AFE_CONN046_7: u32 = 0x265c;
pub const AFE_CONN047_0: u32 = 0x2660;
pub const AFE_CONN047_1: u32 = 0x2664;
pub const AFE_CONN047_2: u32 = 0x2668;
pub const AFE_CONN047_4: u32 = 0x2670;
pub const AFE_CONN047_5: u32 = 0x2674;
pub const AFE_CONN047_6: u32 = 0x2678;
pub const AFE_CONN047_7: u32 = 0x267c;
pub const AFE_CONN048_0: u32 = 0x2680;
pub const AFE_CONN048_1: u32 = 0x2684;
pub const AFE_CONN048_2: u32 = 0x2688;
pub const AFE_CONN048_4: u32 = 0x2690;
pub const AFE_CONN048_5: u32 = 0x2694;
pub const AFE_CONN048_6: u32 = 0x2698;
pub const AFE_CONN048_7: u32 = 0x269c;
pub const AFE_CONN049_0: u32 = 0x26a0;
pub const AFE_CONN049_1: u32 = 0x26a4;
pub const AFE_CONN049_2: u32 = 0x26a8;
pub const AFE_CONN049_4: u32 = 0x26b0;
pub const AFE_CONN049_5: u32 = 0x26b4;
pub const AFE_CONN049_6: u32 = 0x26b8;
pub const AFE_CONN049_7: u32 = 0x26bc;
pub const AFE_CONN050_0: u32 = 0x26c0;
pub const AFE_CONN050_1: u32 = 0x26c4;
pub const AFE_CONN050_2: u32 = 0x26c8;
pub const AFE_CONN050_4: u32 = 0x26d0;
pub const AFE_CONN050_5: u32 = 0x26d4;
pub const AFE_CONN050_6: u32 = 0x26d8;
pub const AFE_CONN050_7: u32 = 0x26dc;
pub const AFE_CONN051_0: u32 = 0x26e0;
pub const AFE_CONN051_1: u32 = 0x26e4;
pub const AFE_CONN051_2: u32 = 0x26e8;
pub const AFE_CONN051_4: u32 = 0x26f0;
pub const AFE_CONN051_5: u32 = 0x26f4;
pub const AFE_CONN051_6: u32 = 0x26f8;
pub const AFE_CONN051_7: u32 = 0x26fc;
pub const AFE_CONN052_0: u32 = 0x2700;
pub const AFE_CONN052_1: u32 = 0x2704;
pub const AFE_CONN052_2: u32 = 0x2708;
pub const AFE_CONN052_4: u32 = 0x2710;
pub const AFE_CONN052_5: u32 = 0x2714;
pub const AFE_CONN052_6: u32 = 0x2718;
pub const AFE_CONN052_7: u32 = 0x271c;
pub const AFE_CONN053_0: u32 = 0x2720;
pub const AFE_CONN053_1: u32 = 0x2724;
pub const AFE_CONN053_2: u32 = 0x2728;
pub const AFE_CONN053_4: u32 = 0x2730;
pub const AFE_CONN053_5: u32 = 0x2734;
pub const AFE_CONN053_6: u32 = 0x2738;
pub const AFE_CONN053_7: u32 = 0x273c;
pub const AFE_CONN054_0: u32 = 0x2740;
pub const AFE_CONN054_1: u32 = 0x2744;
pub const AFE_CONN054_2: u32 = 0x2748;
pub const AFE_CONN054_4: u32 = 0x2750;
pub const AFE_CONN054_5: u32 = 0x2754;
pub const AFE_CONN054_6: u32 = 0x2758;
pub const AFE_CONN054_7: u32 = 0x275c;
pub const AFE_CONN055_0: u32 = 0x2760;
pub const AFE_CONN055_1: u32 = 0x2764;
pub const AFE_CONN055_2: u32 = 0x2768;
pub const AFE_CONN055_4: u32 = 0x2770;
pub const AFE_CONN055_5: u32 = 0x2774;
pub const AFE_CONN055_6: u32 = 0x2778;
pub const AFE_CONN055_7: u32 = 0x277c;
pub const AFE_CONN056_0: u32 = 0x2780;
pub const AFE_CONN056_1: u32 = 0x2784;
pub const AFE_CONN056_2: u32 = 0x2788;
pub const AFE_CONN056_4: u32 = 0x2790;
pub const AFE_CONN056_5: u32 = 0x2794;
pub const AFE_CONN056_6: u32 = 0x2798;
pub const AFE_CONN056_7: u32 = 0x279c;
pub const AFE_CONN057_0: u32 = 0x27a0;
pub const AFE_CONN057_1: u32 = 0x27a4;
pub const AFE_CONN057_2: u32 = 0x27a8;
pub const AFE_CONN057_4: u32 = 0x27b0;
pub const AFE_CONN057_5: u32 = 0x27b4;
pub const AFE_CONN057_6: u32 = 0x27b8;
pub const AFE_CONN057_7: u32 = 0x27bc;
pub const AFE_CONN058_0: u32 = 0x27c0;
pub const AFE_CONN058_1: u32 = 0x27c4;
pub const AFE_CONN058_2: u32 = 0x27c8;
pub const AFE_CONN058_4: u32 = 0x27d0;
pub const AFE_CONN058_5: u32 = 0x27d4;
pub const AFE_CONN058_6: u32 = 0x27d8;
pub const AFE_CONN058_7: u32 = 0x27dc;
pub const AFE_CONN059_0: u32 = 0x27e0;
pub const AFE_CONN059_1: u32 = 0x27e4;
pub const AFE_CONN059_2: u32 = 0x27e8;
pub const AFE_CONN059_4: u32 = 0x27f0;
pub const AFE_CONN059_5: u32 = 0x27f4;
pub const AFE_CONN059_6: u32 = 0x27f8;
pub const AFE_CONN059_7: u32 = 0x27fc;
pub const AFE_CONN060_0: u32 = 0x2800;
pub const AFE_CONN060_1: u32 = 0x2804;
pub const AFE_CONN060_2: u32 = 0x2808;
pub const AFE_CONN060_4: u32 = 0x2810;
pub const AFE_CONN060_5: u32 = 0x2814;
pub const AFE_CONN060_6: u32 = 0x2818;
pub const AFE_CONN060_7: u32 = 0x281c;
pub const AFE_CONN061_0: u32 = 0x2820;
pub const AFE_CONN061_1: u32 = 0x2824;
pub const AFE_CONN061_2: u32 = 0x2828;
pub const AFE_CONN061_4: u32 = 0x2830;
pub const AFE_CONN061_5: u32 = 0x2834;
pub const AFE_CONN061_6: u32 = 0x2838;
pub const AFE_CONN061_7: u32 = 0x283c;
pub const AFE_CONN062_0: u32 = 0x2840;
pub const AFE_CONN062_1: u32 = 0x2844;
pub const AFE_CONN062_2: u32 = 0x2848;
pub const AFE_CONN062_4: u32 = 0x2850;
pub const AFE_CONN062_5: u32 = 0x2854;
pub const AFE_CONN062_6: u32 = 0x2858;
pub const AFE_CONN062_7: u32 = 0x285c;
pub const AFE_CONN063_0: u32 = 0x2860;
pub const AFE_CONN063_1: u32 = 0x2864;
pub const AFE_CONN063_2: u32 = 0x2868;
pub const AFE_CONN063_4: u32 = 0x2870;
pub const AFE_CONN063_5: u32 = 0x2874;
pub const AFE_CONN063_6: u32 = 0x2878;
pub const AFE_CONN063_7: u32 = 0x287c;
pub const AFE_CONN064_0: u32 = 0x2880;
pub const AFE_CONN064_1: u32 = 0x2884;
pub const AFE_CONN064_2: u32 = 0x2888;
pub const AFE_CONN064_4: u32 = 0x2890;
pub const AFE_CONN064_5: u32 = 0x2894;
pub const AFE_CONN064_6: u32 = 0x2898;
pub const AFE_CONN064_7: u32 = 0x289c;
pub const AFE_CONN065_0: u32 = 0x28a0;
pub const AFE_CONN065_1: u32 = 0x28a4;
pub const AFE_CONN065_2: u32 = 0x28a8;
pub const AFE_CONN065_4: u32 = 0x28b0;
pub const AFE_CONN065_5: u32 = 0x28b4;
pub const AFE_CONN065_6: u32 = 0x28b8;
pub const AFE_CONN065_7: u32 = 0x28bc;
pub const AFE_CONN066_0: u32 = 0x28c0;
pub const AFE_CONN066_1: u32 = 0x28c4;
pub const AFE_CONN066_2: u32 = 0x28c8;
pub const AFE_CONN066_4: u32 = 0x28d0;
pub const AFE_CONN066_5: u32 = 0x28d4;
pub const AFE_CONN066_6: u32 = 0x28d8;
pub const AFE_CONN066_7: u32 = 0x28dc;
pub const AFE_CONN067_0: u32 = 0x28e0;
pub const AFE_CONN067_1: u32 = 0x28e4;
pub const AFE_CONN067_2: u32 = 0x28e8;
pub const AFE_CONN067_4: u32 = 0x28f0;
pub const AFE_CONN067_5: u32 = 0x28f4;
pub const AFE_CONN067_6: u32 = 0x28f8;
pub const AFE_CONN067_7: u32 = 0x28fc;
pub const AFE_CONN068_0: u32 = 0x2900;
pub const AFE_CONN068_1: u32 = 0x2904;
pub const AFE_CONN068_2: u32 = 0x2908;
pub const AFE_CONN068_4: u32 = 0x2910;
pub const AFE_CONN068_5: u32 = 0x2914;
pub const AFE_CONN068_6: u32 = 0x2918;
pub const AFE_CONN068_7: u32 = 0x291c;
pub const AFE_CONN069_0: u32 = 0x2920;
pub const AFE_CONN069_1: u32 = 0x2924;
pub const AFE_CONN069_2: u32 = 0x2928;
pub const AFE_CONN069_4: u32 = 0x2930;
pub const AFE_CONN069_5: u32 = 0x2934;
pub const AFE_CONN069_6: u32 = 0x2938;
pub const AFE_CONN069_7: u32 = 0x293c;
pub const AFE_CONN070_0: u32 = 0x2940;
pub const AFE_CONN070_1: u32 = 0x2944;
pub const AFE_CONN070_2: u32 = 0x2948;
pub const AFE_CONN070_4: u32 = 0x2950;
pub const AFE_CONN070_5: u32 = 0x2954;
pub const AFE_CONN070_6: u32 = 0x2958;
pub const AFE_CONN070_7: u32 = 0x295c;
pub const AFE_CONN071_0: u32 = 0x2960;
pub const AFE_CONN071_1: u32 = 0x2964;
pub const AFE_CONN071_2: u32 = 0x2968;
pub const AFE_CONN071_4: u32 = 0x2970;
pub const AFE_CONN071_5: u32 = 0x2974;
pub const AFE_CONN071_6: u32 = 0x2978;
pub const AFE_CONN071_7: u32 = 0x297c;
pub const AFE_CONN072_0: u32 = 0x2980;
pub const AFE_CONN072_1: u32 = 0x2984;
pub const AFE_CONN072_2: u32 = 0x2988;
pub const AFE_CONN072_4: u32 = 0x2990;
pub const AFE_CONN072_5: u32 = 0x2994;
pub const AFE_CONN072_6: u32 = 0x2998;
pub const AFE_CONN072_7: u32 = 0x299c;
pub const AFE_CONN073_0: u32 = 0x29a0;
pub const AFE_CONN073_1: u32 = 0x29a4;
pub const AFE_CONN073_2: u32 = 0x29a8;
pub const AFE_CONN073_4: u32 = 0x29b0;
pub const AFE_CONN073_5: u32 = 0x29b4;
pub const AFE_CONN073_6: u32 = 0x29b8;
pub const AFE_CONN073_7: u32 = 0x29bc;
pub const AFE_CONN074_0: u32 = 0x29c0;
pub const AFE_CONN074_1: u32 = 0x29c4;
pub const AFE_CONN074_2: u32 = 0x29c8;
pub const AFE_CONN074_4: u32 = 0x29d0;
pub const AFE_CONN074_5: u32 = 0x29d4;
pub const AFE_CONN074_6: u32 = 0x29d8;
pub const AFE_CONN074_7: u32 = 0x29dc;
pub const AFE_CONN075_0: u32 = 0x29e0;
pub const AFE_CONN075_1: u32 = 0x29e4;
pub const AFE_CONN075_2: u32 = 0x29e8;
pub const AFE_CONN075_4: u32 = 0x29f0;
pub const AFE_CONN075_5: u32 = 0x29f4;
pub const AFE_CONN075_6: u32 = 0x29f8;
pub const AFE_CONN075_7: u32 = 0x29fc;
pub const AFE_CONN076_0: u32 = 0x2a00;
pub const AFE_CONN076_1: u32 = 0x2a04;
pub const AFE_CONN076_2: u32 = 0x2a08;
pub const AFE_CONN076_4: u32 = 0x2a10;
pub const AFE_CONN076_5: u32 = 0x2a14;
pub const AFE_CONN076_6: u32 = 0x2a18;
pub const AFE_CONN076_7: u32 = 0x2a1c;
pub const AFE_CONN077_0: u32 = 0x2a20;
pub const AFE_CONN077_1: u32 = 0x2a24;
pub const AFE_CONN077_2: u32 = 0x2a28;
pub const AFE_CONN077_4: u32 = 0x2a30;
pub const AFE_CONN077_5: u32 = 0x2a34;
pub const AFE_CONN077_6: u32 = 0x2a38;
pub const AFE_CONN077_7: u32 = 0x2a3c;
pub const AFE_CONN078_0: u32 = 0x2a40;
pub const AFE_CONN078_1: u32 = 0x2a44;
pub const AFE_CONN078_2: u32 = 0x2a48;
pub const AFE_CONN078_4: u32 = 0x2a50;
pub const AFE_CONN078_5: u32 = 0x2a54;
pub const AFE_CONN078_6: u32 = 0x2a58;
pub const AFE_CONN078_7: u32 = 0x2a5c;
pub const AFE_CONN079_0: u32 = 0x2a60;
pub const AFE_CONN079_1: u32 = 0x2a64;
pub const AFE_CONN079_2: u32 = 0x2a68;
pub const AFE_CONN079_4: u32 = 0x2a70;
pub const AFE_CONN079_5: u32 = 0x2a74;
pub const AFE_CONN079_6: u32 = 0x2a78;
pub const AFE_CONN079_7: u32 = 0x2a7c;
pub const AFE_CONN080_0: u32 = 0x2a80;
pub const AFE_CONN080_1: u32 = 0x2a84;
pub const AFE_CONN080_2: u32 = 0x2a88;
pub const AFE_CONN080_4: u32 = 0x2a90;
pub const AFE_CONN080_5: u32 = 0x2a94;
pub const AFE_CONN080_6: u32 = 0x2a98;
pub const AFE_CONN080_7: u32 = 0x2a9c;
pub const AFE_CONN081_0: u32 = 0x2aa0;
pub const AFE_CONN081_1: u32 = 0x2aa4;
pub const AFE_CONN081_2: u32 = 0x2aa8;
pub const AFE_CONN081_4: u32 = 0x2ab0;
pub const AFE_CONN081_5: u32 = 0x2ab4;
pub const AFE_CONN081_6: u32 = 0x2ab8;
pub const AFE_CONN081_7: u32 = 0x2abc;
pub const AFE_CONN082_0: u32 = 0x2ac0;
pub const AFE_CONN082_1: u32 = 0x2ac4;
pub const AFE_CONN082_2: u32 = 0x2ac8;
pub const AFE_CONN082_4: u32 = 0x2ad0;
pub const AFE_CONN082_5: u32 = 0x2ad4;
pub const AFE_CONN082_6: u32 = 0x2ad8;
pub const AFE_CONN082_7: u32 = 0x2adc;
pub const AFE_CONN083_0: u32 = 0x2ae0;
pub const AFE_CONN083_1: u32 = 0x2ae4;
pub const AFE_CONN083_2: u32 = 0x2ae8;
pub const AFE_CONN083_4: u32 = 0x2af0;
pub const AFE_CONN083_5: u32 = 0x2af4;
pub const AFE_CONN083_6: u32 = 0x2af8;
pub const AFE_CONN083_7: u32 = 0x2afc;
pub const AFE_CONN084_0: u32 = 0x2b00;
pub const AFE_CONN084_1: u32 = 0x2b04;
pub const AFE_CONN084_2: u32 = 0x2b08;
pub const AFE_CONN084_4: u32 = 0x2b10;
pub const AFE_CONN084_5: u32 = 0x2b14;
pub const AFE_CONN084_6: u32 = 0x2b18;
pub const AFE_CONN084_7: u32 = 0x2b1c;
pub const AFE_CONN085_0: u32 = 0x2b20;
pub const AFE_CONN085_1: u32 = 0x2b24;
pub const AFE_CONN085_2: u32 = 0x2b28;
pub const AFE_CONN085_4: u32 = 0x2b30;
pub const AFE_CONN085_5: u32 = 0x2b34;
pub const AFE_CONN085_6: u32 = 0x2b38;
pub const AFE_CONN085_7: u32 = 0x2b3c;
pub const AFE_CONN086_0: u32 = 0x2b40;
pub const AFE_CONN086_1: u32 = 0x2b44;
pub const AFE_CONN086_2: u32 = 0x2b48;
pub const AFE_CONN086_4: u32 = 0x2b50;
pub const AFE_CONN086_5: u32 = 0x2b54;
pub const AFE_CONN086_6: u32 = 0x2b58;
pub const AFE_CONN086_7: u32 = 0x2b5c;
pub const AFE_CONN087_0: u32 = 0x2b60;
pub const AFE_CONN087_1: u32 = 0x2b64;
pub const AFE_CONN087_2: u32 = 0x2b68;
pub const AFE_CONN087_4: u32 = 0x2b70;
pub const AFE_CONN087_5: u32 = 0x2b74;
pub const AFE_CONN087_6: u32 = 0x2b78;
pub const AFE_CONN087_7: u32 = 0x2b7c;
pub const AFE_CONN088_0: u32 = 0x2b80;
pub const AFE_CONN088_1: u32 = 0x2b84;
pub const AFE_CONN088_2: u32 = 0x2b88;
pub const AFE_CONN088_4: u32 = 0x2b90;
pub const AFE_CONN088_5: u32 = 0x2b94;
pub const AFE_CONN088_6: u32 = 0x2b98;
pub const AFE_CONN088_7: u32 = 0x2b9c;
pub const AFE_CONN089_0: u32 = 0x2ba0;
pub const AFE_CONN089_1: u32 = 0x2ba4;
pub const AFE_CONN089_2: u32 = 0x2ba8;
pub const AFE_CONN089_4: u32 = 0x2bb0;
pub const AFE_CONN089_5: u32 = 0x2bb4;
pub const AFE_CONN089_6: u32 = 0x2bb8;
pub const AFE_CONN089_7: u32 = 0x2bbc;
pub const AFE_CONN090_0: u32 = 0x2bc0;
pub const AFE_CONN090_1: u32 = 0x2bc4;
pub const AFE_CONN090_2: u32 = 0x2bc8;
pub const AFE_CONN090_4: u32 = 0x2bd0;
pub const AFE_CONN090_5: u32 = 0x2bd4;
pub const AFE_CONN090_6: u32 = 0x2bd8;
pub const AFE_CONN090_7: u32 = 0x2bdc;
pub const AFE_CONN091_0: u32 = 0x2be0;
pub const AFE_CONN091_1: u32 = 0x2be4;
pub const AFE_CONN091_2: u32 = 0x2be8;
pub const AFE_CONN091_4: u32 = 0x2bf0;
pub const AFE_CONN091_5: u32 = 0x2bf4;
pub const AFE_CONN091_6: u32 = 0x2bf8;
pub const AFE_CONN091_7: u32 = 0x2bfc;
pub const AFE_CONN092_0: u32 = 0x2c00;
pub const AFE_CONN092_1: u32 = 0x2c04;
pub const AFE_CONN092_2: u32 = 0x2c08;
pub const AFE_CONN092_4: u32 = 0x2c10;
pub const AFE_CONN092_5: u32 = 0x2c14;
pub const AFE_CONN092_6: u32 = 0x2c18;
pub const AFE_CONN092_7: u32 = 0x2c1c;
pub const AFE_CONN093_0: u32 = 0x2c20;
pub const AFE_CONN093_1: u32 = 0x2c24;
pub const AFE_CONN093_2: u32 = 0x2c28;
pub const AFE_CONN093_4: u32 = 0x2c30;
pub const AFE_CONN093_5: u32 = 0x2c34;
pub const AFE_CONN093_6: u32 = 0x2c38;
pub const AFE_CONN093_7: u32 = 0x2c3c;
pub const AFE_CONN094_0: u32 = 0x2c40;
pub const AFE_CONN094_1: u32 = 0x2c44;
pub const AFE_CONN094_2: u32 = 0x2c48;
pub const AFE_CONN094_4: u32 = 0x2c50;
pub const AFE_CONN094_5: u32 = 0x2c54;
pub const AFE_CONN094_6: u32 = 0x2c58;
pub const AFE_CONN094_7: u32 = 0x2c5c;
pub const AFE_CONN095_0: u32 = 0x2c60;
pub const AFE_CONN095_1: u32 = 0x2c64;
pub const AFE_CONN095_2: u32 = 0x2c68;
pub const AFE_CONN095_4: u32 = 0x2c70;
pub const AFE_CONN095_5: u32 = 0x2c74;
pub const AFE_CONN095_6: u32 = 0x2c78;
pub const AFE_CONN095_7: u32 = 0x2c7c;
pub const AFE_CONN096_0: u32 = 0x2c80;
pub const AFE_CONN096_1: u32 = 0x2c84;
pub const AFE_CONN096_2: u32 = 0x2c88;
pub const AFE_CONN096_4: u32 = 0x2c90;
pub const AFE_CONN096_5: u32 = 0x2c94;
pub const AFE_CONN096_6: u32 = 0x2c98;
pub const AFE_CONN096_7: u32 = 0x2c9c;
pub const AFE_CONN097_0: u32 = 0x2ca0;
pub const AFE_CONN097_1: u32 = 0x2ca4;
pub const AFE_CONN097_2: u32 = 0x2ca8;
pub const AFE_CONN097_4: u32 = 0x2cb0;
pub const AFE_CONN097_5: u32 = 0x2cb4;
pub const AFE_CONN097_6: u32 = 0x2cb8;
pub const AFE_CONN097_7: u32 = 0x2cbc;
pub const AFE_CONN098_0: u32 = 0x2cc0;
pub const AFE_CONN098_1: u32 = 0x2cc4;
pub const AFE_CONN098_2: u32 = 0x2cc8;
pub const AFE_CONN098_4: u32 = 0x2cd0;
pub const AFE_CONN098_5: u32 = 0x2cd4;
pub const AFE_CONN098_6: u32 = 0x2cd8;
pub const AFE_CONN098_7: u32 = 0x2cdc;
pub const AFE_CONN099_0: u32 = 0x2ce0;
pub const AFE_CONN099_1: u32 = 0x2ce4;
pub const AFE_CONN099_2: u32 = 0x2ce8;
pub const AFE_CONN099_4: u32 = 0x2cf0;
pub const AFE_CONN099_5: u32 = 0x2cf4;
pub const AFE_CONN099_6: u32 = 0x2cf8;
pub const AFE_CONN099_7: u32 = 0x2cfc;
pub const AFE_CONN100_0: u32 = 0x2d00;
pub const AFE_CONN100_1: u32 = 0x2d04;
pub const AFE_CONN100_2: u32 = 0x2d08;
pub const AFE_CONN100_4: u32 = 0x2d10;
pub const AFE_CONN100_5: u32 = 0x2d14;
pub const AFE_CONN100_6: u32 = 0x2d18;
pub const AFE_CONN100_7: u32 = 0x2d1c;
pub const AFE_CONN102_0: u32 = 0x2d40;
pub const AFE_CONN102_1: u32 = 0x2d44;
pub const AFE_CONN102_2: u32 = 0x2d48;
pub const AFE_CONN102_4: u32 = 0x2d50;
pub const AFE_CONN102_5: u32 = 0x2d54;
pub const AFE_CONN102_6: u32 = 0x2d58;
pub const AFE_CONN102_7: u32 = 0x2d5c;
pub const AFE_CONN103_0: u32 = 0x2d60;
pub const AFE_CONN103_1: u32 = 0x2d64;
pub const AFE_CONN103_2: u32 = 0x2d68;
pub const AFE_CONN103_4: u32 = 0x2d70;
pub const AFE_CONN103_5: u32 = 0x2d74;
pub const AFE_CONN103_6: u32 = 0x2d78;
pub const AFE_CONN103_7: u32 = 0x2d7c;
pub const AFE_CONN104_0: u32 = 0x2d80;
pub const AFE_CONN104_1: u32 = 0x2d84;
pub const AFE_CONN104_2: u32 = 0x2d88;
pub const AFE_CONN104_4: u32 = 0x2d90;
pub const AFE_CONN104_5: u32 = 0x2d94;
pub const AFE_CONN104_6: u32 = 0x2d98;
pub const AFE_CONN104_7: u32 = 0x2d9c;
pub const AFE_CONN105_0: u32 = 0x2da0;
pub const AFE_CONN105_1: u32 = 0x2da4;
pub const AFE_CONN105_2: u32 = 0x2da8;
pub const AFE_CONN105_4: u32 = 0x2db0;
pub const AFE_CONN105_5: u32 = 0x2db4;
pub const AFE_CONN105_6: u32 = 0x2db8;
pub const AFE_CONN105_7: u32 = 0x2dbc;
pub const AFE_CONN106_0: u32 = 0x2dc0;
pub const AFE_CONN106_1: u32 = 0x2dc4;
pub const AFE_CONN106_2: u32 = 0x2dc8;
pub const AFE_CONN106_4: u32 = 0x2dd0;
pub const AFE_CONN106_5: u32 = 0x2dd4;
pub const AFE_CONN106_6: u32 = 0x2dd8;
pub const AFE_CONN106_7: u32 = 0x2ddc;
pub const AFE_CONN108_0: u32 = 0x2e00;
pub const AFE_CONN108_1: u32 = 0x2e04;
pub const AFE_CONN108_2: u32 = 0x2e08;
pub const AFE_CONN108_4: u32 = 0x2e10;
pub const AFE_CONN108_5: u32 = 0x2e14;
pub const AFE_CONN108_6: u32 = 0x2e18;
pub const AFE_CONN108_7: u32 = 0x2e1c;
pub const AFE_CONN109_0: u32 = 0x2e20;
pub const AFE_CONN109_1: u32 = 0x2e24;
pub const AFE_CONN109_2: u32 = 0x2e28;
pub const AFE_CONN109_4: u32 = 0x2e30;
pub const AFE_CONN109_5: u32 = 0x2e34;
pub const AFE_CONN109_6: u32 = 0x2e38;
pub const AFE_CONN109_7: u32 = 0x2e3c;
pub const AFE_CONN110_0: u32 = 0x2e40;
pub const AFE_CONN110_1: u32 = 0x2e44;
pub const AFE_CONN110_2: u32 = 0x2e48;
pub const AFE_CONN110_4: u32 = 0x2e50;
pub const AFE_CONN110_5: u32 = 0x2e54;
pub const AFE_CONN110_6: u32 = 0x2e58;
pub const AFE_CONN110_7: u32 = 0x2e5c;
pub const AFE_CONN111_0: u32 = 0x2e60;
pub const AFE_CONN111_1: u32 = 0x2e64;
pub const AFE_CONN111_2: u32 = 0x2e68;
pub const AFE_CONN111_4: u32 = 0x2e70;
pub const AFE_CONN111_5: u32 = 0x2e74;
pub const AFE_CONN111_6: u32 = 0x2e78;
pub const AFE_CONN111_7: u32 = 0x2e7c;
pub const AFE_CONN112_0: u32 = 0x2e80;
pub const AFE_CONN112_1: u32 = 0x2e84;
pub const AFE_CONN112_2: u32 = 0x2e88;
pub const AFE_CONN112_4: u32 = 0x2e90;
pub const AFE_CONN112_5: u32 = 0x2e94;
pub const AFE_CONN112_6: u32 = 0x2e98;
pub const AFE_CONN112_7: u32 = 0x2e9c;
pub const AFE_CONN113_0: u32 = 0x2ea0;
pub const AFE_CONN113_1: u32 = 0x2ea4;
pub const AFE_CONN113_2: u32 = 0x2ea8;
pub const AFE_CONN113_4: u32 = 0x2eb0;
pub const AFE_CONN113_5: u32 = 0x2eb4;
pub const AFE_CONN113_6: u32 = 0x2eb8;
pub const AFE_CONN113_7: u32 = 0x2ebc;
pub const AFE_CONN114_0: u32 = 0x2ec0;
pub const AFE_CONN114_1: u32 = 0x2ec4;
pub const AFE_CONN114_2: u32 = 0x2ec8;
pub const AFE_CONN114_4: u32 = 0x2ed0;
pub const AFE_CONN114_5: u32 = 0x2ed4;
pub const AFE_CONN114_6: u32 = 0x2ed8;
pub const AFE_CONN114_7: u32 = 0x2edc;
pub const AFE_CONN115_0: u32 = 0x2ee0;
pub const AFE_CONN115_1: u32 = 0x2ee4;
pub const AFE_CONN115_2: u32 = 0x2ee8;
pub const AFE_CONN115_4: u32 = 0x2ef0;
pub const AFE_CONN115_5: u32 = 0x2ef4;
pub const AFE_CONN115_6: u32 = 0x2ef8;
pub const AFE_CONN115_7: u32 = 0x2efc;
pub const AFE_CONN116_0: u32 = 0x2f00;
pub const AFE_CONN116_1: u32 = 0x2f04;
pub const AFE_CONN116_2: u32 = 0x2f08;
pub const AFE_CONN116_4: u32 = 0x2f10;
pub const AFE_CONN116_5: u32 = 0x2f14;
pub const AFE_CONN116_6: u32 = 0x2f18;
pub const AFE_CONN116_7: u32 = 0x2f1c;
pub const AFE_CONN117_0: u32 = 0x2f20;
pub const AFE_CONN117_1: u32 = 0x2f24;
pub const AFE_CONN117_2: u32 = 0x2f28;
pub const AFE_CONN117_4: u32 = 0x2f30;
pub const AFE_CONN117_5: u32 = 0x2f34;
pub const AFE_CONN117_6: u32 = 0x2f38;
pub const AFE_CONN117_7: u32 = 0x2f3c;
pub const AFE_CONN118_0: u32 = 0x2f40;
pub const AFE_CONN118_1: u32 = 0x2f44;
pub const AFE_CONN118_2: u32 = 0x2f48;
pub const AFE_CONN118_4: u32 = 0x2f50;
pub const AFE_CONN118_5: u32 = 0x2f54;
pub const AFE_CONN118_6: u32 = 0x2f58;
pub const AFE_CONN118_7: u32 = 0x2f5c;
pub const AFE_CONN119_0: u32 = 0x2f60;
pub const AFE_CONN119_1: u32 = 0x2f64;
pub const AFE_CONN119_2: u32 = 0x2f68;
pub const AFE_CONN119_4: u32 = 0x2f70;
pub const AFE_CONN119_5: u32 = 0x2f74;
pub const AFE_CONN119_6: u32 = 0x2f78;
pub const AFE_CONN119_7: u32 = 0x2f7c;
pub const AFE_CONN120_0: u32 = 0x2f80;
pub const AFE_CONN120_1: u32 = 0x2f84;
pub const AFE_CONN120_2: u32 = 0x2f88;
pub const AFE_CONN120_4: u32 = 0x2f90;
pub const AFE_CONN120_5: u32 = 0x2f94;
pub const AFE_CONN120_6: u32 = 0x2f98;
pub const AFE_CONN120_7: u32 = 0x2f9c;
pub const AFE_CONN121_0: u32 = 0x2fa0;
pub const AFE_CONN121_1: u32 = 0x2fa4;
pub const AFE_CONN121_2: u32 = 0x2fa8;
pub const AFE_CONN121_4: u32 = 0x2fb0;
pub const AFE_CONN121_5: u32 = 0x2fb4;
pub const AFE_CONN121_6: u32 = 0x2fb8;
pub const AFE_CONN121_7: u32 = 0x2fbc;
pub const AFE_CONN122_0: u32 = 0x2fc0;
pub const AFE_CONN122_1: u32 = 0x2fc4;
pub const AFE_CONN122_2: u32 = 0x2fc8;
pub const AFE_CONN122_4: u32 = 0x2fd0;
pub const AFE_CONN122_5: u32 = 0x2fd4;
pub const AFE_CONN122_6: u32 = 0x2fd8;
pub const AFE_CONN122_7: u32 = 0x2fdc;
pub const AFE_CONN123_0: u32 = 0x2fe0;
pub const AFE_CONN123_1: u32 = 0x2fe4;
pub const AFE_CONN123_2: u32 = 0x2fe8;
pub const AFE_CONN123_4: u32 = 0x2ff0;
pub const AFE_CONN123_5: u32 = 0x2ff4;
pub const AFE_CONN123_6: u32 = 0x2ff8;
pub const AFE_CONN123_7: u32 = 0x2ffc;
pub const AFE_CONN124_0: u32 = 0x3000;
pub const AFE_CONN124_1: u32 = 0x3004;
pub const AFE_CONN124_2: u32 = 0x3008;
pub const AFE_CONN124_4: u32 = 0x3010;
pub const AFE_CONN124_5: u32 = 0x3014;
pub const AFE_CONN124_6: u32 = 0x3018;
pub const AFE_CONN124_7: u32 = 0x301c;
pub const AFE_CONN125_0: u32 = 0x3020;
pub const AFE_CONN125_1: u32 = 0x3024;
pub const AFE_CONN125_2: u32 = 0x3028;
pub const AFE_CONN125_4: u32 = 0x3030;
pub const AFE_CONN125_5: u32 = 0x3034;
pub const AFE_CONN125_6: u32 = 0x3038;
pub const AFE_CONN125_7: u32 = 0x303c;
pub const AFE_CONN126_0: u32 = 0x3040;
pub const AFE_CONN126_1: u32 = 0x3044;
pub const AFE_CONN126_2: u32 = 0x3048;
pub const AFE_CONN126_4: u32 = 0x3050;
pub const AFE_CONN126_5: u32 = 0x3054;
pub const AFE_CONN126_6: u32 = 0x3058;
pub const AFE_CONN126_7: u32 = 0x305c;
pub const AFE_CONN127_0: u32 = 0x3060;
pub const AFE_CONN127_1: u32 = 0x3064;
pub const AFE_CONN127_2: u32 = 0x3068;
pub const AFE_CONN127_4: u32 = 0x3070;
pub const AFE_CONN127_5: u32 = 0x3074;
pub const AFE_CONN127_6: u32 = 0x3078;
pub const AFE_CONN127_7: u32 = 0x307c;
pub const AFE_CONN128_0: u32 = 0x3080;
pub const AFE_CONN128_1: u32 = 0x3084;
pub const AFE_CONN128_2: u32 = 0x3088;
pub const AFE_CONN128_4: u32 = 0x3090;
pub const AFE_CONN128_5: u32 = 0x3094;
pub const AFE_CONN128_6: u32 = 0x3098;
pub const AFE_CONN128_7: u32 = 0x309c;
pub const AFE_CONN129_0: u32 = 0x30a0;
pub const AFE_CONN129_1: u32 = 0x30a4;
pub const AFE_CONN129_2: u32 = 0x30a8;
pub const AFE_CONN129_4: u32 = 0x30b0;
pub const AFE_CONN129_5: u32 = 0x30b4;
pub const AFE_CONN129_6: u32 = 0x30b8;
pub const AFE_CONN129_7: u32 = 0x30bc;
pub const AFE_CONN130_0: u32 = 0x30c0;
pub const AFE_CONN130_1: u32 = 0x30c4;
pub const AFE_CONN130_2: u32 = 0x30c8;
pub const AFE_CONN130_4: u32 = 0x30d0;
pub const AFE_CONN130_5: u32 = 0x30d4;
pub const AFE_CONN130_6: u32 = 0x30d8;
pub const AFE_CONN130_7: u32 = 0x30dc;
pub const AFE_CONN131_0: u32 = 0x30e0;
pub const AFE_CONN131_1: u32 = 0x30e4;
pub const AFE_CONN131_2: u32 = 0x30e8;
pub const AFE_CONN131_4: u32 = 0x30f0;
pub const AFE_CONN131_5: u32 = 0x30f4;
pub const AFE_CONN131_6: u32 = 0x30f8;
pub const AFE_CONN131_7: u32 = 0x30fc;
pub const AFE_CONN132_0: u32 = 0x3100;
pub const AFE_CONN132_1: u32 = 0x3104;
pub const AFE_CONN132_2: u32 = 0x3108;
pub const AFE_CONN132_4: u32 = 0x3110;
pub const AFE_CONN132_5: u32 = 0x3114;
pub const AFE_CONN132_6: u32 = 0x3118;
pub const AFE_CONN132_7: u32 = 0x311c;
pub const AFE_CONN133_0: u32 = 0x3120;
pub const AFE_CONN133_1: u32 = 0x3124;
pub const AFE_CONN133_2: u32 = 0x3128;
pub const AFE_CONN133_4: u32 = 0x3130;
pub const AFE_CONN133_5: u32 = 0x3134;
pub const AFE_CONN133_6: u32 = 0x3138;
pub const AFE_CONN133_7: u32 = 0x313c;
pub const AFE_CONN134_0: u32 = 0x3140;
pub const AFE_CONN134_1: u32 = 0x3144;
pub const AFE_CONN134_2: u32 = 0x3148;
pub const AFE_CONN134_4: u32 = 0x3150;
pub const AFE_CONN134_5: u32 = 0x3154;
pub const AFE_CONN134_6: u32 = 0x3158;
pub const AFE_CONN134_7: u32 = 0x315c;
pub const AFE_CONN135_0: u32 = 0x3160;
pub const AFE_CONN135_1: u32 = 0x3164;
pub const AFE_CONN135_2: u32 = 0x3168;
pub const AFE_CONN135_4: u32 = 0x3170;
pub const AFE_CONN135_5: u32 = 0x3174;
pub const AFE_CONN135_6: u32 = 0x3178;
pub const AFE_CONN135_7: u32 = 0x317c;
pub const AFE_CONN136_0: u32 = 0x3180;
pub const AFE_CONN136_1: u32 = 0x3184;
pub const AFE_CONN136_2: u32 = 0x3188;
pub const AFE_CONN136_4: u32 = 0x3190;
pub const AFE_CONN136_5: u32 = 0x3194;
pub const AFE_CONN136_6: u32 = 0x3198;
pub const AFE_CONN136_7: u32 = 0x319c;
pub const AFE_CONN137_0: u32 = 0x31a0;
pub const AFE_CONN137_1: u32 = 0x31a4;
pub const AFE_CONN137_2: u32 = 0x31a8;
pub const AFE_CONN137_4: u32 = 0x31b0;
pub const AFE_CONN137_5: u32 = 0x31b4;
pub const AFE_CONN137_6: u32 = 0x31b8;
pub const AFE_CONN137_7: u32 = 0x31bc;
pub const AFE_CONN138_0: u32 = 0x31c0;
pub const AFE_CONN138_1: u32 = 0x31c4;
pub const AFE_CONN138_2: u32 = 0x31c8;
pub const AFE_CONN138_4: u32 = 0x31d0;
pub const AFE_CONN138_5: u32 = 0x31d4;
pub const AFE_CONN138_6: u32 = 0x31d8;
pub const AFE_CONN138_7: u32 = 0x31dc;
pub const AFE_CONN139_0: u32 = 0x31e0;
pub const AFE_CONN139_1: u32 = 0x31e4;
pub const AFE_CONN139_2: u32 = 0x31e8;
pub const AFE_CONN139_4: u32 = 0x31f0;
pub const AFE_CONN139_5: u32 = 0x31f4;
pub const AFE_CONN139_6: u32 = 0x31f8;
pub const AFE_CONN139_7: u32 = 0x31fc;
pub const AFE_CONN148_0: u32 = 0x3300;
pub const AFE_CONN148_1: u32 = 0x3304;
pub const AFE_CONN148_2: u32 = 0x3308;
pub const AFE_CONN148_4: u32 = 0x3310;
pub const AFE_CONN148_5: u32 = 0x3314;
pub const AFE_CONN148_6: u32 = 0x3318;
pub const AFE_CONN148_7: u32 = 0x331c;
pub const AFE_CONN149_0: u32 = 0x3320;
pub const AFE_CONN149_1: u32 = 0x3324;
pub const AFE_CONN149_2: u32 = 0x3328;
pub const AFE_CONN149_4: u32 = 0x3330;
pub const AFE_CONN149_5: u32 = 0x3334;
pub const AFE_CONN149_6: u32 = 0x3338;
pub const AFE_CONN149_7: u32 = 0x333c;
pub const AFE_CONN180_0: u32 = 0x3700;
pub const AFE_CONN180_1: u32 = 0x3704;
pub const AFE_CONN180_2: u32 = 0x3708;
pub const AFE_CONN180_4: u32 = 0x3710;
pub const AFE_CONN180_5: u32 = 0x3714;
pub const AFE_CONN180_6: u32 = 0x3718;
pub const AFE_CONN180_7: u32 = 0x371c;
pub const AFE_CONN181_0: u32 = 0x3720;
pub const AFE_CONN181_1: u32 = 0x3724;
pub const AFE_CONN181_2: u32 = 0x3728;
pub const AFE_CONN181_4: u32 = 0x3730;
pub const AFE_CONN181_5: u32 = 0x3734;
pub const AFE_CONN181_6: u32 = 0x3738;
pub const AFE_CONN181_7: u32 = 0x373c;
pub const AFE_CONN182_0: u32 = 0x3740;
pub const AFE_CONN182_1: u32 = 0x3744;
pub const AFE_CONN182_2: u32 = 0x3748;
pub const AFE_CONN182_4: u32 = 0x3750;
pub const AFE_CONN182_5: u32 = 0x3754;
pub const AFE_CONN182_6: u32 = 0x3758;
pub const AFE_CONN182_7: u32 = 0x375c;
pub const AFE_CONN183_0: u32 = 0x3760;
pub const AFE_CONN183_1: u32 = 0x3764;
pub const AFE_CONN183_2: u32 = 0x3768;
pub const AFE_CONN183_4: u32 = 0x3770;
pub const AFE_CONN183_5: u32 = 0x3774;
pub const AFE_CONN183_6: u32 = 0x3778;
pub const AFE_CONN183_7: u32 = 0x377c;
pub const AFE_CONN184_0: u32 = 0x3780;
pub const AFE_CONN184_1: u32 = 0x3784;
pub const AFE_CONN184_2: u32 = 0x3788;
pub const AFE_CONN184_4: u32 = 0x3790;
pub const AFE_CONN184_5: u32 = 0x3794;
pub const AFE_CONN184_6: u32 = 0x3798;
pub const AFE_CONN184_7: u32 = 0x379c;
pub const AFE_CONN185_0: u32 = 0x37a0;
pub const AFE_CONN185_1: u32 = 0x37a4;
pub const AFE_CONN185_2: u32 = 0x37a8;
pub const AFE_CONN185_4: u32 = 0x37b0;
pub const AFE_CONN185_5: u32 = 0x37b4;
pub const AFE_CONN185_6: u32 = 0x37b8;
pub const AFE_CONN185_7: u32 = 0x37bc;
pub const AFE_CONN186_0: u32 = 0x37c0;
pub const AFE_CONN186_1: u32 = 0x37c4;
pub const AFE_CONN186_2: u32 = 0x37c8;
pub const AFE_CONN186_4: u32 = 0x37d0;
pub const AFE_CONN186_5: u32 = 0x37d4;
pub const AFE_CONN186_6: u32 = 0x37d8;
pub const AFE_CONN186_7: u32 = 0x37dc;
pub const AFE_CONN187_0: u32 = 0x37e0;
pub const AFE_CONN187_1: u32 = 0x37e4;
pub const AFE_CONN187_2: u32 = 0x37e8;
pub const AFE_CONN187_4: u32 = 0x37f0;
pub const AFE_CONN187_5: u32 = 0x37f4;
pub const AFE_CONN187_6: u32 = 0x37f8;
pub const AFE_CONN187_7: u32 = 0x37fc;
pub const AFE_CONN188_0: u32 = 0x3800;
pub const AFE_CONN188_1: u32 = 0x3804;
pub const AFE_CONN188_2: u32 = 0x3808;
pub const AFE_CONN188_4: u32 = 0x3810;
pub const AFE_CONN188_5: u32 = 0x3814;
pub const AFE_CONN188_6: u32 = 0x3818;
pub const AFE_CONN188_7: u32 = 0x381c;
pub const AFE_CONN189_0: u32 = 0x3820;
pub const AFE_CONN189_1: u32 = 0x3824;
pub const AFE_CONN189_2: u32 = 0x3828;
pub const AFE_CONN189_4: u32 = 0x3830;
pub const AFE_CONN189_5: u32 = 0x3834;
pub const AFE_CONN189_6: u32 = 0x3838;
pub const AFE_CONN189_7: u32 = 0x383c;
pub const AFE_CONN190_0: u32 = 0x3840;
pub const AFE_CONN190_1: u32 = 0x3844;
pub const AFE_CONN190_2: u32 = 0x3848;
pub const AFE_CONN190_4: u32 = 0x3850;
pub const AFE_CONN190_5: u32 = 0x3854;
pub const AFE_CONN190_6: u32 = 0x3858;
pub const AFE_CONN190_7: u32 = 0x385c;
pub const AFE_CONN191_0: u32 = 0x3860;
pub const AFE_CONN191_1: u32 = 0x3864;
pub const AFE_CONN191_2: u32 = 0x3868;
pub const AFE_CONN191_4: u32 = 0x3870;
pub const AFE_CONN191_5: u32 = 0x3874;
pub const AFE_CONN191_6: u32 = 0x3878;
pub const AFE_CONN191_7: u32 = 0x387c;
pub const AFE_CONN192_0: u32 = 0x3880;
pub const AFE_CONN192_1: u32 = 0x3884;
pub const AFE_CONN192_2: u32 = 0x3888;
pub const AFE_CONN192_4: u32 = 0x3890;
pub const AFE_CONN192_5: u32 = 0x3894;
pub const AFE_CONN192_6: u32 = 0x3898;
pub const AFE_CONN192_7: u32 = 0x389c;
pub const AFE_CONN193_0: u32 = 0x38a0;
pub const AFE_CONN193_1: u32 = 0x38a4;
pub const AFE_CONN193_2: u32 = 0x38a8;
pub const AFE_CONN193_4: u32 = 0x38b0;
pub const AFE_CONN193_5: u32 = 0x38b4;
pub const AFE_CONN193_6: u32 = 0x38b8;
pub const AFE_CONN193_7: u32 = 0x38bc;
pub const AFE_CONN194_0: u32 = 0x38c0;
pub const AFE_CONN194_1: u32 = 0x38c4;
pub const AFE_CONN194_2: u32 = 0x38c8;
pub const AFE_CONN194_4: u32 = 0x38d0;
pub const AFE_CONN194_5: u32 = 0x38d4;
pub const AFE_CONN194_6: u32 = 0x38d8;
pub const AFE_CONN194_7: u32 = 0x38dc;
pub const AFE_CONN195_0: u32 = 0x38e0;
pub const AFE_CONN195_1: u32 = 0x38e4;
pub const AFE_CONN195_2: u32 = 0x38e8;
pub const AFE_CONN195_4: u32 = 0x38f0;
pub const AFE_CONN195_5: u32 = 0x38f4;
pub const AFE_CONN195_6: u32 = 0x38f8;
pub const AFE_CONN195_7: u32 = 0x38fc;
pub const AFE_CONN196_0: u32 = 0x3900;
pub const AFE_CONN196_1: u32 = 0x3904;
pub const AFE_CONN196_2: u32 = 0x3908;
pub const AFE_CONN196_4: u32 = 0x3910;
pub const AFE_CONN196_5: u32 = 0x3914;
pub const AFE_CONN196_6: u32 = 0x3918;
pub const AFE_CONN196_7: u32 = 0x391c;
pub const AFE_CONN197_0: u32 = 0x3920;
pub const AFE_CONN197_1: u32 = 0x3924;
pub const AFE_CONN197_2: u32 = 0x3928;
pub const AFE_CONN197_4: u32 = 0x3930;
pub const AFE_CONN197_5: u32 = 0x3934;
pub const AFE_CONN197_6: u32 = 0x3938;
pub const AFE_CONN197_7: u32 = 0x393c;
pub const AFE_CONN198_0: u32 = 0x3940;
pub const AFE_CONN198_1: u32 = 0x3944;
pub const AFE_CONN198_2: u32 = 0x3948;
pub const AFE_CONN198_4: u32 = 0x3950;
pub const AFE_CONN198_5: u32 = 0x3954;
pub const AFE_CONN198_6: u32 = 0x3958;
pub const AFE_CONN198_7: u32 = 0x395c;
pub const AFE_CONN199_0: u32 = 0x3960;
pub const AFE_CONN199_1: u32 = 0x3964;
pub const AFE_CONN199_2: u32 = 0x3968;
pub const AFE_CONN199_4: u32 = 0x3970;
pub const AFE_CONN199_5: u32 = 0x3974;
pub const AFE_CONN199_6: u32 = 0x3978;
pub const AFE_CONN199_7: u32 = 0x397c;
pub const AFE_CONN200_0: u32 = 0x3980;
pub const AFE_CONN200_1: u32 = 0x3984;
pub const AFE_CONN200_2: u32 = 0x3988;
pub const AFE_CONN200_4: u32 = 0x3990;
pub const AFE_CONN200_5: u32 = 0x3994;
pub const AFE_CONN200_6: u32 = 0x3998;
pub const AFE_CONN200_7: u32 = 0x399c;
pub const AFE_CONN201_0: u32 = 0x39a0;
pub const AFE_CONN201_1: u32 = 0x39a4;
pub const AFE_CONN201_2: u32 = 0x39a8;
pub const AFE_CONN201_4: u32 = 0x39b0;
pub const AFE_CONN201_5: u32 = 0x39b4;
pub const AFE_CONN201_6: u32 = 0x39b8;
pub const AFE_CONN201_7: u32 = 0x39bc;
pub const AFE_CONN202_0: u32 = 0x39c0;
pub const AFE_CONN202_1: u32 = 0x39c4;
pub const AFE_CONN202_2: u32 = 0x39c8;
pub const AFE_CONN202_4: u32 = 0x39d0;
pub const AFE_CONN202_5: u32 = 0x39d4;
pub const AFE_CONN202_6: u32 = 0x39d8;
pub const AFE_CONN202_7: u32 = 0x39dc;
pub const AFE_CONN203_0: u32 = 0x39e0;
pub const AFE_CONN203_1: u32 = 0x39e4;
pub const AFE_CONN203_2: u32 = 0x39e8;
pub const AFE_CONN203_4: u32 = 0x39f0;
pub const AFE_CONN203_5: u32 = 0x39f4;
pub const AFE_CONN203_6: u32 = 0x39f8;
pub const AFE_CONN203_7: u32 = 0x39fc;
pub const AFE_CONN204_0: u32 = 0x3a00;
pub const AFE_CONN204_1: u32 = 0x3a04;
pub const AFE_CONN204_2: u32 = 0x3a08;
pub const AFE_CONN204_4: u32 = 0x3a10;
pub const AFE_CONN204_5: u32 = 0x3a14;
pub const AFE_CONN204_6: u32 = 0x3a18;
pub const AFE_CONN204_7: u32 = 0x3a1c;
pub const AFE_CONN205_0: u32 = 0x3a20;
pub const AFE_CONN205_1: u32 = 0x3a24;
pub const AFE_CONN205_2: u32 = 0x3a28;
pub const AFE_CONN205_4: u32 = 0x3a30;
pub const AFE_CONN205_5: u32 = 0x3a34;
pub const AFE_CONN205_6: u32 = 0x3a38;
pub const AFE_CONN205_7: u32 = 0x3a3c;
pub const AFE_CONN206_0: u32 = 0x3a40;
pub const AFE_CONN206_1: u32 = 0x3a44;
pub const AFE_CONN206_2: u32 = 0x3a48;
pub const AFE_CONN206_4: u32 = 0x3a50;
pub const AFE_CONN206_5: u32 = 0x3a54;
pub const AFE_CONN206_6: u32 = 0x3a58;
pub const AFE_CONN206_7: u32 = 0x3a5c;
pub const AFE_CONN207_0: u32 = 0x3a60;
pub const AFE_CONN207_1: u32 = 0x3a64;
pub const AFE_CONN207_2: u32 = 0x3a68;
pub const AFE_CONN207_4: u32 = 0x3a70;
pub const AFE_CONN207_5: u32 = 0x3a74;
pub const AFE_CONN207_6: u32 = 0x3a78;
pub const AFE_CONN207_7: u32 = 0x3a7c;
pub const AFE_CONN208_0: u32 = 0x3a80;
pub const AFE_CONN208_1: u32 = 0x3a84;
pub const AFE_CONN208_2: u32 = 0x3a88;
pub const AFE_CONN208_4: u32 = 0x3a90;
pub const AFE_CONN208_5: u32 = 0x3a94;
pub const AFE_CONN208_6: u32 = 0x3a98;
pub const AFE_CONN208_7: u32 = 0x3a9c;
pub const AFE_CONN209_0: u32 = 0x3aa0;
pub const AFE_CONN209_1: u32 = 0x3aa4;
pub const AFE_CONN209_2: u32 = 0x3aa8;
pub const AFE_CONN209_4: u32 = 0x3ab0;
pub const AFE_CONN209_5: u32 = 0x3ab4;
pub const AFE_CONN209_6: u32 = 0x3ab8;
pub const AFE_CONN209_7: u32 = 0x3abc;
pub const AFE_CONN210_0: u32 = 0x3ac0;
pub const AFE_CONN210_1: u32 = 0x3ac4;
pub const AFE_CONN210_2: u32 = 0x3ac8;
pub const AFE_CONN210_4: u32 = 0x3ad0;
pub const AFE_CONN210_5: u32 = 0x3ad4;
pub const AFE_CONN210_6: u32 = 0x3ad8;
pub const AFE_CONN210_7: u32 = 0x3adc;
pub const AFE_CONN211_0: u32 = 0x3ae0;
pub const AFE_CONN211_1: u32 = 0x3ae4;
pub const AFE_CONN211_2: u32 = 0x3ae8;
pub const AFE_CONN211_4: u32 = 0x3af0;
pub const AFE_CONN211_5: u32 = 0x3af4;
pub const AFE_CONN211_6: u32 = 0x3af8;
pub const AFE_CONN211_7: u32 = 0x3afc;
pub const AFE_CONN_MON_CFG: u32 = 0x4080;
pub const AFE_CONN_MON0: u32 = 0x4084;
pub const AFE_CONN_MON1: u32 = 0x4088;
pub const AFE_CONN_MON2: u32 = 0x408c;
pub const AFE_CONN_MON3: u32 = 0x4090;
pub const AFE_CONN_MON4: u32 = 0x4094;
pub const AFE_CONN_MON5: u32 = 0x4098;
pub const AFE_CONN_RS_0: u32 = 0x40a0;
pub const AFE_CONN_RS_1: u32 = 0x40a4;
pub const AFE_CONN_RS_2: u32 = 0x40a8;
pub const AFE_CONN_RS_3: u32 = 0x40ac;
pub const AFE_CONN_RS_4: u32 = 0x40b0;
pub const AFE_CONN_RS_5: u32 = 0x40b4;
pub const AFE_CONN_RS_6: u32 = 0x40b8;
pub const AFE_CONN_DI_0: u32 = 0x40c0;
pub const AFE_CONN_DI_1: u32 = 0x40c4;
pub const AFE_CONN_DI_2: u32 = 0x40c8;
pub const AFE_CONN_DI_3: u32 = 0x40cc;
pub const AFE_CONN_DI_4: u32 = 0x40d0;
pub const AFE_CONN_DI_5: u32 = 0x40d4;
pub const AFE_CONN_DI_6: u32 = 0x40d8;
pub const AFE_CONN_16BIT_0: u32 = 0x40e0;
pub const AFE_CONN_16BIT_1: u32 = 0x40e4;
pub const AFE_CONN_16BIT_2: u32 = 0x40e8;
pub const AFE_CONN_16BIT_3: u32 = 0x40ec;
pub const AFE_CONN_16BIT_4: u32 = 0x40f0;
pub const AFE_CONN_16BIT_5: u32 = 0x40f4;
pub const AFE_CONN_16BIT_6: u32 = 0x40f8;
pub const AFE_CONN_24BIT_0: u32 = 0x4100;
pub const AFE_CONN_24BIT_1: u32 = 0x4104;
pub const AFE_CONN_24BIT_2: u32 = 0x4108;
pub const AFE_CONN_24BIT_3: u32 = 0x410c;
pub const AFE_CONN_24BIT_4: u32 = 0x4110;
pub const AFE_CONN_24BIT_5: u32 = 0x4114;
pub const AFE_CONN_24BIT_6: u32 = 0x4118;
pub const AFE_CBIP_CFG0: u32 = 0x4380;
pub const AFE_CBIP_SLV_DECODER_MON0: u32 = 0x4384;
pub const AFE_CBIP_SLV_DECODER_MON1: u32 = 0x4388;
pub const AFE_CBIP_SLV_MUX_MON_CFG: u32 = 0x438c;
pub const AFE_CBIP_SLV_MUX_MON0: u32 = 0x4390;
pub const AFE_CBIP_SLV_MUX_MON1: u32 = 0x4394;
pub const AFE_MEMIF_CON0: u32 = 0x4400;
pub const AFE_MEMIF_ONE_HEART: u32 = 0x4420;
pub const AFE_DL0_BASE_MSB: u32 = 0x4440;
pub const AFE_DL0_BASE: u32 = 0x4444;
pub const AFE_DL0_CUR_MSB: u32 = 0x4448;
pub const AFE_DL0_CUR: u32 = 0x444c;
pub const AFE_DL0_END_MSB: u32 = 0x4450;
pub const AFE_DL0_END: u32 = 0x4454;
pub const AFE_DL0_RCH_MON: u32 = 0x4458;
pub const AFE_DL0_LCH_MON: u32 = 0x445c;
pub const AFE_DL0_CON0: u32 = 0x4460;
pub const AFE_DL0_MON0: u32 = 0x4464;
pub const AFE_DL1_BASE_MSB: u32 = 0x4470;
pub const AFE_DL1_BASE: u32 = 0x4474;
pub const AFE_DL1_CUR_MSB: u32 = 0x4478;
pub const AFE_DL1_CUR: u32 = 0x447c;
pub const AFE_DL1_END_MSB: u32 = 0x4480;
pub const AFE_DL1_END: u32 = 0x4484;
pub const AFE_DL1_RCH_MON: u32 = 0x4488;
pub const AFE_DL1_LCH_MON: u32 = 0x448c;
pub const AFE_DL1_CON0: u32 = 0x4490;
pub const AFE_DL1_MON0: u32 = 0x4494;
pub const AFE_DL2_BASE_MSB: u32 = 0x44a0;
pub const AFE_DL2_BASE: u32 = 0x44a4;
pub const AFE_DL2_CUR_MSB: u32 = 0x44a8;
pub const AFE_DL2_CUR: u32 = 0x44ac;
pub const AFE_DL2_END_MSB: u32 = 0x44b0;
pub const AFE_DL2_END: u32 = 0x44b4;
pub const AFE_DL2_RCH_MON: u32 = 0x44b8;
pub const AFE_DL2_LCH_MON: u32 = 0x44bc;
pub const AFE_DL2_CON0: u32 = 0x44c0;
pub const AFE_DL2_MON0: u32 = 0x44c4;
pub const AFE_DL3_BASE_MSB: u32 = 0x44d0;
pub const AFE_DL3_BASE: u32 = 0x44d4;
pub const AFE_DL3_CUR_MSB: u32 = 0x44d8;
pub const AFE_DL3_CUR: u32 = 0x44dc;
pub const AFE_DL3_END_MSB: u32 = 0x44e0;
pub const AFE_DL3_END: u32 = 0x44e4;
pub const AFE_DL3_RCH_MON: u32 = 0x44e8;
pub const AFE_DL3_LCH_MON: u32 = 0x44ec;
pub const AFE_DL3_CON0: u32 = 0x44f0;
pub const AFE_DL3_MON0: u32 = 0x44f4;
pub const AFE_DL4_BASE_MSB: u32 = 0x4500;
pub const AFE_DL4_BASE: u32 = 0x4504;
pub const AFE_DL4_CUR_MSB: u32 = 0x4508;
pub const AFE_DL4_CUR: u32 = 0x450c;
pub const AFE_DL4_END_MSB: u32 = 0x4510;
pub const AFE_DL4_END: u32 = 0x4514;
pub const AFE_DL4_RCH_MON: u32 = 0x4518;
pub const AFE_DL4_LCH_MON: u32 = 0x451c;
pub const AFE_DL4_CON0: u32 = 0x4520;
pub const AFE_DL4_MON0: u32 = 0x4524;
pub const AFE_DL5_BASE_MSB: u32 = 0x4530;
pub const AFE_DL5_BASE: u32 = 0x4534;
pub const AFE_DL5_CUR_MSB: u32 = 0x4538;
pub const AFE_DL5_CUR: u32 = 0x453c;
pub const AFE_DL5_END_MSB: u32 = 0x4540;
pub const AFE_DL5_END: u32 = 0x4544;
pub const AFE_DL5_RCH_MON: u32 = 0x4548;
pub const AFE_DL5_LCH_MON: u32 = 0x454c;
pub const AFE_DL5_CON0: u32 = 0x4550;
pub const AFE_DL5_MON0: u32 = 0x4554;
pub const AFE_DL6_BASE_MSB: u32 = 0x4560;
pub const AFE_DL6_BASE: u32 = 0x4564;
pub const AFE_DL6_CUR_MSB: u32 = 0x4568;
pub const AFE_DL6_CUR: u32 = 0x456c;
pub const AFE_DL6_END_MSB: u32 = 0x4570;
pub const AFE_DL6_END: u32 = 0x4574;
pub const AFE_DL6_RCH_MON: u32 = 0x4578;
pub const AFE_DL6_LCH_MON: u32 = 0x457c;
pub const AFE_DL6_CON0: u32 = 0x4580;
pub const AFE_DL6_MON0: u32 = 0x4584;
pub const AFE_DL7_BASE_MSB: u32 = 0x4590;
pub const AFE_DL7_BASE: u32 = 0x4594;
pub const AFE_DL7_CUR_MSB: u32 = 0x4598;
pub const AFE_DL7_CUR: u32 = 0x459c;
pub const AFE_DL7_END_MSB: u32 = 0x45a0;
pub const AFE_DL7_END: u32 = 0x45a4;
pub const AFE_DL7_RCH_MON: u32 = 0x45a8;
pub const AFE_DL7_LCH_MON: u32 = 0x45ac;
pub const AFE_DL7_CON0: u32 = 0x45b0;
pub const AFE_DL7_MON0: u32 = 0x45b4;
pub const AFE_DL8_BASE_MSB: u32 = 0x45c0;
pub const AFE_DL8_BASE: u32 = 0x45c4;
pub const AFE_DL8_CUR_MSB: u32 = 0x45c8;
pub const AFE_DL8_CUR: u32 = 0x45cc;
pub const AFE_DL8_END_MSB: u32 = 0x45d0;
pub const AFE_DL8_END: u32 = 0x45d4;
pub const AFE_DL8_RCH_MON: u32 = 0x45d8;
pub const AFE_DL8_LCH_MON: u32 = 0x45dc;
pub const AFE_DL8_CON0: u32 = 0x45e0;
pub const AFE_DL8_MON0: u32 = 0x45e4;
pub const AFE_DL_4CH_BASE_MSB: u32 = 0x45f0;
pub const AFE_DL_4CH_BASE: u32 = 0x45f4;
pub const AFE_DL_4CH_CUR_MSB: u32 = 0x45f8;
pub const AFE_DL_4CH_CUR: u32 = 0x45fc;
pub const AFE_DL_4CH_END_MSB: u32 = 0x4600;
pub const AFE_DL_4CH_END: u32 = 0x4604;
pub const AFE_DL_4CH_CON0: u32 = 0x4610;
pub const AFE_DL_4CH_MON0: u32 = 0x4618;
pub const AFE_DL_24CH_BASE_MSB: u32 = 0x4620;
pub const AFE_DL_24CH_BASE: u32 = 0x4624;
pub const AFE_DL_24CH_CUR_MSB: u32 = 0x4628;
pub const AFE_DL_24CH_CUR: u32 = 0x462c;
pub const AFE_DL_24CH_END_MSB: u32 = 0x4630;
pub const AFE_DL_24CH_END: u32 = 0x4634;
pub const AFE_DL_24CH_CON0: u32 = 0x4640;
pub const AFE_DL_24CH_MON0: u32 = 0x4648;
pub const AFE_DL23_BASE_MSB: u32 = 0x4680;
pub const AFE_DL23_BASE: u32 = 0x4684;
pub const AFE_DL23_CUR_MSB: u32 = 0x4688;
pub const AFE_DL23_CUR: u32 = 0x468c;
pub const AFE_DL23_END_MSB: u32 = 0x4690;
pub const AFE_DL23_END: u32 = 0x4694;
pub const AFE_DL23_RCH_MON: u32 = 0x4698;
pub const AFE_DL23_LCH_MON: u32 = 0x469c;
pub const AFE_DL23_CON0: u32 = 0x46a0;
pub const AFE_DL23_MON0: u32 = 0x46a4;
pub const AFE_DL24_BASE_MSB: u32 = 0x46b0;
pub const AFE_DL24_BASE: u32 = 0x46b4;
pub const AFE_DL24_CUR_MSB: u32 = 0x46b8;
pub const AFE_DL24_CUR: u32 = 0x46bc;
pub const AFE_DL24_END_MSB: u32 = 0x46c0;
pub const AFE_DL24_END: u32 = 0x46c4;
pub const AFE_DL24_RCH_MON: u32 = 0x46c8;
pub const AFE_DL24_LCH_MON: u32 = 0x46cc;
pub const AFE_DL24_CON0: u32 = 0x46d0;
pub const AFE_DL24_MON0: u32 = 0x46d4;
pub const AFE_DL25_BASE_MSB: u32 = 0x46e0;
pub const AFE_DL25_BASE: u32 = 0x46e4;
pub const AFE_DL25_CUR_MSB: u32 = 0x46e8;
pub const AFE_DL25_CUR: u32 = 0x46ec;
pub const AFE_DL25_END_MSB: u32 = 0x46f0;
pub const AFE_DL25_END: u32 = 0x46f4;
pub const AFE_DL25_RCH_MON: u32 = 0x46f8;
pub const AFE_DL25_LCH_MON: u32 = 0x46fc;
pub const AFE_DL25_CON0: u32 = 0x4700;
pub const AFE_DL25_MON0: u32 = 0x4704;
pub const AFE_DL26_BASE_MSB: u32 = 0x4710;
pub const AFE_DL26_BASE: u32 = 0x4714;
pub const AFE_DL26_CUR_MSB: u32 = 0x4718;
pub const AFE_DL26_CUR: u32 = 0x471c;
pub const AFE_DL26_END_MSB: u32 = 0x4720;
pub const AFE_DL26_END: u32 = 0x4724;
pub const AFE_DL26_RCH_MON: u32 = 0x4728;
pub const AFE_DL26_LCH_MON: u32 = 0x472c;
pub const AFE_DL26_CON0: u32 = 0x4730;
pub const AFE_DL26_MON0: u32 = 0x4734;
pub const AFE_VUL0_BASE_MSB: u32 = 0x4d60;
pub const AFE_VUL0_BASE: u32 = 0x4d64;
pub const AFE_VUL0_CUR_MSB: u32 = 0x4d68;
pub const AFE_VUL0_CUR: u32 = 0x4d6c;
pub const AFE_VUL0_END_MSB: u32 = 0x4d70;
pub const AFE_VUL0_END: u32 = 0x4d74;
pub const AFE_VUL0_RCH_MON: u32 = 0x4d78;
pub const AFE_VUL0_LCH_MON: u32 = 0x4d7c;
pub const AFE_VUL0_CON0: u32 = 0x4d80;
pub const AFE_VUL0_MON0: u32 = 0x4d84;
pub const AFE_VUL1_BASE_MSB: u32 = 0x4d90;
pub const AFE_VUL1_BASE: u32 = 0x4d94;
pub const AFE_VUL1_CUR_MSB: u32 = 0x4d98;
pub const AFE_VUL1_CUR: u32 = 0x4d9c;
pub const AFE_VUL1_END_MSB: u32 = 0x4da0;
pub const AFE_VUL1_END: u32 = 0x4da4;
pub const AFE_VUL1_RCH_MON: u32 = 0x4da8;
pub const AFE_VUL1_LCH_MON: u32 = 0x4dac;
pub const AFE_VUL1_CON0: u32 = 0x4db0;
pub const AFE_VUL1_MON0: u32 = 0x4db4;
pub const AFE_VUL2_BASE_MSB: u32 = 0x4dc0;
pub const AFE_VUL2_BASE: u32 = 0x4dc4;
pub const AFE_VUL2_CUR_MSB: u32 = 0x4dc8;
pub const AFE_VUL2_CUR: u32 = 0x4dcc;
pub const AFE_VUL2_END_MSB: u32 = 0x4dd0;
pub const AFE_VUL2_END: u32 = 0x4dd4;
pub const AFE_VUL2_RCH_MON: u32 = 0x4dd8;
pub const AFE_VUL2_LCH_MON: u32 = 0x4ddc;
pub const AFE_VUL2_CON0: u32 = 0x4de0;
pub const AFE_VUL2_MON0: u32 = 0x4de4;
pub const AFE_VUL3_BASE_MSB: u32 = 0x4df0;
pub const AFE_VUL3_BASE: u32 = 0x4df4;
pub const AFE_VUL3_CUR_MSB: u32 = 0x4df8;
pub const AFE_VUL3_CUR: u32 = 0x4dfc;
pub const AFE_VUL3_END_MSB: u32 = 0x4e00;
pub const AFE_VUL3_END: u32 = 0x4e04;
pub const AFE_VUL3_RCH_MON: u32 = 0x4e08;
pub const AFE_VUL3_LCH_MON: u32 = 0x4e0c;
pub const AFE_VUL3_CON0: u32 = 0x4e10;
pub const AFE_VUL3_MON0: u32 = 0x4e14;
pub const AFE_VUL4_BASE_MSB: u32 = 0x4e20;
pub const AFE_VUL4_BASE: u32 = 0x4e24;
pub const AFE_VUL4_CUR_MSB: u32 = 0x4e28;
pub const AFE_VUL4_CUR: u32 = 0x4e2c;
pub const AFE_VUL4_END_MSB: u32 = 0x4e30;
pub const AFE_VUL4_END: u32 = 0x4e34;
pub const AFE_VUL4_RCH_MON: u32 = 0x4e38;
pub const AFE_VUL4_LCH_MON: u32 = 0x4e3c;
pub const AFE_VUL4_CON0: u32 = 0x4e40;
pub const AFE_VUL4_MON0: u32 = 0x4e44;
pub const AFE_VUL5_BASE_MSB: u32 = 0x4e50;
pub const AFE_VUL5_BASE: u32 = 0x4e54;
pub const AFE_VUL5_CUR_MSB: u32 = 0x4e58;
pub const AFE_VUL5_CUR: u32 = 0x4e5c;
pub const AFE_VUL5_END_MSB: u32 = 0x4e60;
pub const AFE_VUL5_END: u32 = 0x4e64;
pub const AFE_VUL5_RCH_MON: u32 = 0x4e68;
pub const AFE_VUL5_LCH_MON: u32 = 0x4e6c;
pub const AFE_VUL5_CON0: u32 = 0x4e70;
pub const AFE_VUL5_MON0: u32 = 0x4e74;
pub const AFE_VUL6_BASE_MSB: u32 = 0x4e80;
pub const AFE_VUL6_BASE: u32 = 0x4e84;
pub const AFE_VUL6_CUR_MSB: u32 = 0x4e88;
pub const AFE_VUL6_CUR: u32 = 0x4e8c;
pub const AFE_VUL6_END_MSB: u32 = 0x4e90;
pub const AFE_VUL6_END: u32 = 0x4e94;
pub const AFE_VUL6_RCH_MON: u32 = 0x4e98;
pub const AFE_VUL6_LCH_MON: u32 = 0x4e9c;
pub const AFE_VUL6_CON0: u32 = 0x4ea0;
pub const AFE_VUL6_MON0: u32 = 0x4ea4;
pub const AFE_VUL7_BASE_MSB: u32 = 0x4eb0;
pub const AFE_VUL7_BASE: u32 = 0x4eb4;
pub const AFE_VUL7_CUR_MSB: u32 = 0x4eb8;
pub const AFE_VUL7_CUR: u32 = 0x4ebc;
pub const AFE_VUL7_END_MSB: u32 = 0x4ec0;
pub const AFE_VUL7_END: u32 = 0x4ec4;
pub const AFE_VUL7_RCH_MON: u32 = 0x4ec8;
pub const AFE_VUL7_LCH_MON: u32 = 0x4ecc;
pub const AFE_VUL7_CON0: u32 = 0x4ed0;
pub const AFE_VUL7_MON0: u32 = 0x4ed4;
pub const AFE_VUL8_BASE_MSB: u32 = 0x4ee0;
pub const AFE_VUL8_BASE: u32 = 0x4ee4;
pub const AFE_VUL8_CUR_MSB: u32 = 0x4ee8;
pub const AFE_VUL8_CUR: u32 = 0x4eec;
pub const AFE_VUL8_END_MSB: u32 = 0x4ef0;
pub const AFE_VUL8_END: u32 = 0x4ef4;
pub const AFE_VUL8_RCH_MON: u32 = 0x4ef8;
pub const AFE_VUL8_LCH_MON: u32 = 0x4efc;
pub const AFE_VUL8_CON0: u32 = 0x4f00;
pub const AFE_VUL8_MON0: u32 = 0x4f04;
pub const AFE_VUL9_BASE_MSB: u32 = 0x4f10;
pub const AFE_VUL9_BASE: u32 = 0x4f14;
pub const AFE_VUL9_CUR_MSB: u32 = 0x4f18;
pub const AFE_VUL9_CUR: u32 = 0x4f1c;
pub const AFE_VUL9_END_MSB: u32 = 0x4f20;
pub const AFE_VUL9_END: u32 = 0x4f24;
pub const AFE_VUL9_RCH_MON: u32 = 0x4f28;
pub const AFE_VUL9_LCH_MON: u32 = 0x4f2c;
pub const AFE_VUL9_CON0: u32 = 0x4f30;
pub const AFE_VUL9_MON0: u32 = 0x4f34;
pub const AFE_VUL10_BASE_MSB: u32 = 0x4f40;
pub const AFE_VUL10_BASE: u32 = 0x4f44;
pub const AFE_VUL10_CUR_MSB: u32 = 0x4f48;
pub const AFE_VUL10_CUR: u32 = 0x4f4c;
pub const AFE_VUL10_END_MSB: u32 = 0x4f50;
pub const AFE_VUL10_END: u32 = 0x4f54;
pub const AFE_VUL10_RCH_MON: u32 = 0x4f58;
pub const AFE_VUL10_LCH_MON: u32 = 0x4f5c;
pub const AFE_VUL10_CON0: u32 = 0x4f60;
pub const AFE_VUL10_MON0: u32 = 0x4f64;
pub const AFE_VUL24_BASE_MSB: u32 = 0x4fa0;
pub const AFE_VUL24_BASE: u32 = 0x4fa4;
pub const AFE_VUL24_CUR_MSB: u32 = 0x4fa8;
pub const AFE_VUL24_CUR: u32 = 0x4fac;
pub const AFE_VUL24_END_MSB: u32 = 0x4fb0;
pub const AFE_VUL24_END: u32 = 0x4fb4;
pub const AFE_VUL24_CON0: u32 = 0x4fb8;
pub const AFE_VUL24_MON0: u32 = 0x4fbc;
pub const AFE_VUL25_BASE_MSB: u32 = 0x4fc0;
pub const AFE_VUL25_BASE: u32 = 0x4fc4;
pub const AFE_VUL25_CUR_MSB: u32 = 0x4fc8;
pub const AFE_VUL25_CUR: u32 = 0x4fcc;
pub const AFE_VUL25_END_MSB: u32 = 0x4fd0;
pub const AFE_VUL25_END: u32 = 0x4fd4;
pub const AFE_VUL25_CON0: u32 = 0x4fd8;
pub const AFE_VUL25_MON0: u32 = 0x4fdc;
pub const AFE_VUL26_BASE_MSB: u32 = 0x4fe0;
pub const AFE_VUL26_BASE: u32 = 0x4fe4;
pub const AFE_VUL26_CUR_MSB: u32 = 0x4fe8;
pub const AFE_VUL26_CUR: u32 = 0x4fec;
pub const AFE_VUL26_END_MSB: u32 = 0x4ff0;
pub const AFE_VUL26_END: u32 = 0x4ff4;
pub const AFE_VUL26_CON0: u32 = 0x4ff8;
pub const AFE_VUL26_MON0: u32 = 0x4ffc;
pub const AFE_VUL_CM0_BASE_MSB: u32 = 0x51c0;
pub const AFE_VUL_CM0_BASE: u32 = 0x51c4;
pub const AFE_VUL_CM0_CUR_MSB: u32 = 0x51c8;
pub const AFE_VUL_CM0_CUR: u32 = 0x51cc;
pub const AFE_VUL_CM0_END_MSB: u32 = 0x51d0;
pub const AFE_VUL_CM0_END: u32 = 0x51d4;
pub const AFE_VUL_CM0_CON0: u32 = 0x51d8;
pub const AFE_VUL_CM1_BASE_MSB: u32 = 0x51e0;
pub const AFE_VUL_CM1_BASE: u32 = 0x51e4;
pub const AFE_VUL_CM1_CUR_MSB: u32 = 0x51e8;
pub const AFE_VUL_CM1_CUR: u32 = 0x51ec;
pub const AFE_VUL_CM1_END_MSB: u32 = 0x51f0;
pub const AFE_VUL_CM1_END: u32 = 0x51f4;
pub const AFE_VUL_CM1_CON0: u32 = 0x51f8;
pub const AFE_VUL_CM2_BASE_MSB: u32 = 0x5200;
pub const AFE_VUL_CM2_BASE: u32 = 0x5204;
pub const AFE_VUL_CM2_CUR_MSB: u32 = 0x5208;
pub const AFE_VUL_CM2_CUR: u32 = 0x520c;
pub const AFE_VUL_CM2_END_MSB: u32 = 0x5210;
pub const AFE_VUL_CM2_END: u32 = 0x5214;
pub const AFE_VUL_CM2_CON0: u32 = 0x5218;
pub const AFE_ETDM_IN0_BASE_MSB: u32 = 0x5220;
pub const AFE_ETDM_IN0_BASE: u32 = 0x5224;
pub const AFE_ETDM_IN0_CUR_MSB: u32 = 0x5228;
pub const AFE_ETDM_IN0_CUR: u32 = 0x522c;
pub const AFE_ETDM_IN0_END_MSB: u32 = 0x5230;
pub const AFE_ETDM_IN0_END: u32 = 0x5234;
pub const AFE_ETDM_IN0_CON0: u32 = 0x5238;
pub const AFE_ETDM_IN1_BASE_MSB: u32 = 0x5240;
pub const AFE_ETDM_IN1_BASE: u32 = 0x5244;
pub const AFE_ETDM_IN1_CUR_MSB: u32 = 0x5248;
pub const AFE_ETDM_IN1_CUR: u32 = 0x524c;
pub const AFE_ETDM_IN1_END_MSB: u32 = 0x5250;
pub const AFE_ETDM_IN1_END: u32 = 0x5254;
pub const AFE_ETDM_IN1_CON0: u32 = 0x5258;
pub const AFE_ETDM_IN2_BASE_MSB: u32 = 0x5260;
pub const AFE_ETDM_IN2_BASE: u32 = 0x5264;
pub const AFE_ETDM_IN2_CUR_MSB: u32 = 0x5268;
pub const AFE_ETDM_IN2_CUR: u32 = 0x526c;
pub const AFE_ETDM_IN2_END_MSB: u32 = 0x5270;
pub const AFE_ETDM_IN2_END: u32 = 0x5274;
pub const AFE_ETDM_IN2_CON0: u32 = 0x5278;
pub const AFE_ETDM_IN3_BASE_MSB: u32 = 0x5280;
pub const AFE_ETDM_IN3_BASE: u32 = 0x5284;
pub const AFE_ETDM_IN3_CUR_MSB: u32 = 0x5288;
pub const AFE_ETDM_IN3_CUR: u32 = 0x528c;
pub const AFE_ETDM_IN3_END_MSB: u32 = 0x5290;
pub const AFE_ETDM_IN3_END: u32 = 0x5294;
pub const AFE_ETDM_IN3_CON0: u32 = 0x5298;
pub const AFE_ETDM_IN4_BASE_MSB: u32 = 0x52a0;
pub const AFE_ETDM_IN4_BASE: u32 = 0x52a4;
pub const AFE_ETDM_IN4_CUR_MSB: u32 = 0x52a8;
pub const AFE_ETDM_IN4_CUR: u32 = 0x52ac;
pub const AFE_ETDM_IN4_END_MSB: u32 = 0x52b0;
pub const AFE_ETDM_IN4_END: u32 = 0x52b4;
pub const AFE_ETDM_IN4_CON0: u32 = 0x52b8;
pub const AFE_ETDM_IN5_BASE_MSB: u32 = 0x52c0;
pub const AFE_ETDM_IN5_BASE: u32 = 0x52c4;
pub const AFE_ETDM_IN5_CUR_MSB: u32 = 0x52c8;
pub const AFE_ETDM_IN5_CUR: u32 = 0x52cc;
pub const AFE_ETDM_IN5_END_MSB: u32 = 0x52d0;
pub const AFE_ETDM_IN5_END: u32 = 0x52d4;
pub const AFE_ETDM_IN5_CON0: u32 = 0x52d8;
pub const AFE_ETDM_IN6_BASE_MSB: u32 = 0x52e0;
pub const AFE_ETDM_IN6_BASE: u32 = 0x52e4;
pub const AFE_ETDM_IN6_CUR_MSB: u32 = 0x52e8;
pub const AFE_ETDM_IN6_CUR: u32 = 0x52ec;
pub const AFE_ETDM_IN6_END_MSB: u32 = 0x52f0;
pub const AFE_ETDM_IN6_END: u32 = 0x52f4;
pub const AFE_ETDM_IN6_CON0: u32 = 0x52f8;
pub const AFE_HDMI_OUT_BASE_MSB: u32 = 0x5360;
pub const AFE_HDMI_OUT_BASE: u32 = 0x5364;
pub const AFE_HDMI_OUT_CUR_MSB: u32 = 0x5368;
pub const AFE_HDMI_OUT_CUR: u32 = 0x536c;
pub const AFE_HDMI_OUT_END_MSB: u32 = 0x5370;
pub const AFE_HDMI_OUT_END: u32 = 0x5374;
pub const AFE_HDMI_OUT_CON0: u32 = 0x5378;
pub const AFE_VUL24_RCH_MON: u32 = 0x53e0;
pub const AFE_VUL24_LCH_MON: u32 = 0x53e4;
pub const AFE_VUL25_RCH_MON: u32 = 0x53e8;
pub const AFE_VUL25_LCH_MON: u32 = 0x53ec;
pub const AFE_VUL26_RCH_MON: u32 = 0x53f0;
pub const AFE_VUL26_LCH_MON: u32 = 0x53f4;
pub const AFE_VUL_CM0_RCH_MON: u32 = 0x5458;
pub const AFE_VUL_CM0_LCH_MON: u32 = 0x545c;
pub const AFE_VUL_CM1_RCH_MON: u32 = 0x5460;
pub const AFE_VUL_CM1_LCH_MON: u32 = 0x5464;
pub const AFE_VUL_CM2_RCH_MON: u32 = 0x5468;
pub const AFE_VUL_CM2_LCH_MON: u32 = 0x546c;
pub const AFE_DL_4CH_CH0_MON: u32 = 0x54f4;
pub const AFE_DL_4CH_CH1_MON: u32 = 0x54f8;
pub const AFE_DL_4CH_CH2_MON: u32 = 0x54fc;
pub const AFE_DL_4CH_CH3_MON: u32 = 0x5500;
pub const AFE_DL_24CH_CH0_MON: u32 = 0x5504;
pub const AFE_DL_24CH_CH1_MON: u32 = 0x5508;
pub const AFE_DL_24CH_CH2_MON: u32 = 0x550c;
pub const AFE_DL_24CH_CH3_MON: u32 = 0x5510;
pub const AFE_DL_24CH_CH4_MON: u32 = 0x5514;
pub const AFE_DL_24CH_CH5_MON: u32 = 0x5518;
pub const AFE_DL_24CH_CH6_MON: u32 = 0x551c;
pub const AFE_DL_24CH_CH7_MON: u32 = 0x5520;
pub const AFE_DL_24CH_CH8_MON: u32 = 0x5524;
pub const AFE_DL_24CH_CH9_MON: u32 = 0x5528;
pub const AFE_DL_24CH_CH10_MON: u32 = 0x552c;
pub const AFE_DL_24CH_CH11_MON: u32 = 0x5530;
pub const AFE_DL_24CH_CH12_MON: u32 = 0x5534;
pub const AFE_DL_24CH_CH13_MON: u32 = 0x5538;
pub const AFE_DL_24CH_CH14_MON: u32 = 0x553c;
pub const AFE_DL_24CH_CH15_MON: u32 = 0x5540;
pub const AFE_SRAM_BOUND: u32 = 0x5620;
pub const AFE_SECURE_CON0: u32 = 0x5624;
pub const AFE_SECURE_CON1: u32 = 0x5628;
pub const AFE_SE_SECURE_CON0: u32 = 0x5630;
pub const AFE_SE_SECURE_CON1: u32 = 0x5634;
pub const AFE_SE_SECURE_CON2: u32 = 0x5638;
pub const AFE_SE_SECURE_CON3: u32 = 0x563c;
pub const AFE_SE_PROT_SIDEBAND0: u32 = 0x5640;
pub const AFE_SE_PROT_SIDEBAND1: u32 = 0x5644;
pub const AFE_SE_PROT_SIDEBAND2: u32 = 0x5648;
pub const AFE_SE_PROT_SIDEBAND3: u32 = 0x564c;
pub const AFE_SE_DOMAIN_SIDEBAND0: u32 = 0x5650;
pub const AFE_SE_DOMAIN_SIDEBAND1: u32 = 0x5654;
pub const AFE_SE_DOMAIN_SIDEBAND2: u32 = 0x5658;
pub const AFE_SE_DOMAIN_SIDEBAND3: u32 = 0x565c;
pub const AFE_SE_DOMAIN_SIDEBAND4: u32 = 0x5660;
pub const AFE_SE_DOMAIN_SIDEBAND5: u32 = 0x5664;
pub const AFE_SE_DOMAIN_SIDEBAND6: u32 = 0x5668;
pub const AFE_SE_DOMAIN_SIDEBAND7: u32 = 0x566c;
pub const AFE_SE_DOMAIN_SIDEBAND8: u32 = 0x5670;
pub const AFE_SE_DOMAIN_SIDEBAND9: u32 = 0x5674;
pub const AFE_PROT_SIDEBAND0_MON: u32 = 0x5678;
pub const AFE_PROT_SIDEBAND1_MON: u32 = 0x567c;
pub const AFE_PROT_SIDEBAND2_MON: u32 = 0x5680;
pub const AFE_PROT_SIDEBAND3_MON: u32 = 0x5684;
pub const AFE_DOMAIN_SIDEBAND0_MON: u32 = 0x5688;
pub const AFE_DOMAIN_SIDEBAND1_MON: u32 = 0x568c;
pub const AFE_DOMAIN_SIDEBAND2_MON: u32 = 0x5690;
pub const AFE_DOMAIN_SIDEBAND3_MON: u32 = 0x5694;
pub const AFE_DOMAIN_SIDEBAND4_MON: u32 = 0x5698;
pub const AFE_DOMAIN_SIDEBAND5_MON: u32 = 0x569c;
pub const AFE_DOMAIN_SIDEBAND6_MON: u32 = 0x56a0;
pub const AFE_DOMAIN_SIDEBAND7_MON: u32 = 0x56a4;
pub const AFE_DOMAIN_SIDEBAND8_MON: u32 = 0x56a8;
pub const AFE_DOMAIN_SIDEBAND9_MON: u32 = 0x56ac;
pub const AFE_SECURE_CONN0: u32 = 0x56b0;
pub const AFE_SECURE_CONN_ETDM0: u32 = 0x56b4;
pub const AFE_SECURE_CONN_ETDM1: u32 = 0x56b8;
pub const AFE_SECURE_CONN_ETDM2: u32 = 0x56bc;
pub const AFE_SECURE_SRAM_CON0: u32 = 0x56c0;
pub const AFE_SECURE_SRAM_CON1: u32 = 0x56c4;
pub const AFE_SE_CONN_INPUT_MASK0: u32 = 0x56d0;
pub const AFE_SE_CONN_INPUT_MASK1: u32 = 0x56d4;
pub const AFE_SE_CONN_INPUT_MASK2: u32 = 0x56d8;
pub const AFE_SE_CONN_INPUT_MASK3: u32 = 0x56dc;
pub const AFE_SE_CONN_INPUT_MASK4: u32 = 0x56e0;
pub const AFE_SE_CONN_INPUT_MASK5: u32 = 0x56e4;
pub const AFE_SE_CONN_INPUT_MASK6: u32 = 0x56e8;
pub const AFE_SE_CONN_INPUT_MASK7: u32 = 0x56ec;
pub const AFE_NON_SE_CONN_INPUT_MASK0: u32 = 0x56f0;
pub const AFE_NON_SE_CONN_INPUT_MASK1: u32 = 0x56f4;
pub const AFE_NON_SE_CONN_INPUT_MASK2: u32 = 0x56f8;
pub const AFE_NON_SE_CONN_INPUT_MASK3: u32 = 0x56fc;
pub const AFE_NON_SE_CONN_INPUT_MASK4: u32 = 0x5700;
pub const AFE_NON_SE_CONN_INPUT_MASK5: u32 = 0x5704;
pub const AFE_NON_SE_CONN_INPUT_MASK6: u32 = 0x5708;
pub const AFE_NON_SE_CONN_INPUT_MASK7: u32 = 0x570c;
pub const AFE_SE_CONN_OUTPUT_SEL0: u32 = 0x5710;
pub const AFE_SE_CONN_OUTPUT_SEL1: u32 = 0x5714;
pub const AFE_SE_CONN_OUTPUT_SEL2: u32 = 0x5718;
pub const AFE_SE_CONN_OUTPUT_SEL3: u32 = 0x571c;
pub const AFE_SE_CONN_OUTPUT_SEL4: u32 = 0x5720;
pub const AFE_SE_CONN_OUTPUT_SEL5: u32 = 0x5724;
pub const AFE_SE_CONN_OUTPUT_SEL6: u32 = 0x5728;
pub const AFE_SE_CONN_OUTPUT_SEL7: u32 = 0x572c;
pub const AFE_PCM0_INTF_CON1_MASK_MON: u32 = 0x5730;
pub const AFE_PCM0_INTF_CON0_MASK_MON: u32 = 0x5734;
pub const AFE_CONNSYS_I2S_CON_MASK_MON: u32 = 0x5738;
pub const AFE_TDM_CON2_MASK_MON: u32 = 0x5744;
pub const AFE_MTKAIF0_CFG0_MASK_MON: u32 = 0x574c;
pub const AFE_MTKAIF1_CFG0_MASK_MON: u32 = 0x5750;
pub const AFE_ADDA_UL0_SRC_CON0_MASK_MON: u32 = 0x5754;
pub const AFE_ADDA_UL1_SRC_CON0_MASK_MON: u32 = 0x5758;
pub const AFE_ADDA_UL2_SRC_CON0_MASK_MON: u32 = 0x575c;
pub const AFE_ASRC_NEW_CON0: u32 = 0x7800;
pub const AFE_ASRC_NEW_CON1: u32 = 0x7804;
pub const AFE_ASRC_NEW_CON2: u32 = 0x7808;
pub const AFE_ASRC_NEW_CON3: u32 = 0x780c;
pub const AFE_ASRC_NEW_CON4: u32 = 0x7810;
pub const AFE_ASRC_NEW_CON5: u32 = 0x7814;
pub const AFE_ASRC_NEW_CON6: u32 = 0x7818;
pub const AFE_ASRC_NEW_CON7: u32 = 0x781c;
pub const AFE_ASRC_NEW_CON8: u32 = 0x7820;
pub const AFE_ASRC_NEW_CON9: u32 = 0x7824;
pub const AFE_ASRC_NEW_CON10: u32 = 0x7828;
pub const AFE_ASRC_NEW_CON11: u32 = 0x782c;
pub const AFE_ASRC_NEW_CON12: u32 = 0x7830;
pub const AFE_ASRC_NEW_CON13: u32 = 0x7834;
pub const AFE_ASRC_NEW_CON14: u32 = 0x7838;
pub const AFE_ASRC_NEW_IP_VERSION: u32 = 0x783c;
pub const AFE_GASRC0_NEW_CON0: u32 = 0x7840;
pub const AFE_GASRC0_NEW_CON1: u32 = 0x7844;
pub const AFE_GASRC0_NEW_CON2: u32 = 0x7848;
pub const AFE_GASRC0_NEW_CON3: u32 = 0x784c;
pub const AFE_GASRC0_NEW_CON4: u32 = 0x7850;
pub const AFE_GASRC0_NEW_CON5: u32 = 0x7854;
pub const AFE_GASRC0_NEW_CON6: u32 = 0x7858;
pub const AFE_GASRC0_NEW_CON7: u32 = 0x785c;
pub const AFE_GASRC0_NEW_CON8: u32 = 0x7860;
pub const AFE_GASRC0_NEW_CON9: u32 = 0x7864;
pub const AFE_GASRC0_NEW_CON10: u32 = 0x7868;
pub const AFE_GASRC0_NEW_CON11: u32 = 0x786c;
pub const AFE_GASRC0_NEW_CON12: u32 = 0x7870;
pub const AFE_GASRC0_NEW_CON13: u32 = 0x7874;
pub const AFE_GASRC0_NEW_CON14: u32 = 0x7878;
pub const AFE_GASRC0_NEW_IP_VERSION: u32 = 0x787c;
pub const AFE_GASRC1_NEW_CON0: u32 = 0x7880;
pub const AFE_GASRC1_NEW_CON1: u32 = 0x7884;
pub const AFE_GASRC1_NEW_CON2: u32 = 0x7888;
pub const AFE_GASRC1_NEW_CON3: u32 = 0x788c;
pub const AFE_GASRC1_NEW_CON4: u32 = 0x7890;
pub const AFE_GASRC1_NEW_CON5: u32 = 0x7894;
pub const AFE_GASRC1_NEW_CON6: u32 = 0x7898;
pub const AFE_GASRC1_NEW_CON7: u32 = 0x789c;
pub const AFE_GASRC1_NEW_CON8: u32 = 0x78a0;
pub const AFE_GASRC1_NEW_CON9: u32 = 0x78a4;
pub const AFE_GASRC1_NEW_CON10: u32 = 0x78a8;
pub const AFE_GASRC1_NEW_CON11: u32 = 0x78ac;
pub const AFE_GASRC1_NEW_CON12: u32 = 0x78b0;
pub const AFE_GASRC1_NEW_CON13: u32 = 0x78b4;
pub const AFE_GASRC1_NEW_CON14: u32 = 0x78b8;
pub const AFE_GASRC1_NEW_IP_VERSION: u32 = 0x78bc;
pub const AFE_GASRC2_NEW_CON0: u32 = 0x78c0;
pub const AFE_GASRC2_NEW_CON1: u32 = 0x78c4;
pub const AFE_GASRC2_NEW_CON2: u32 = 0x78c8;
pub const AFE_GASRC2_NEW_CON3: u32 = 0x78cc;
pub const AFE_GASRC2_NEW_CON4: u32 = 0x78d0;
pub const AFE_GASRC2_NEW_CON5: u32 = 0x78d4;
pub const AFE_GASRC2_NEW_CON6: u32 = 0x78d8;
pub const AFE_GASRC2_NEW_CON7: u32 = 0x78dc;
pub const AFE_GASRC2_NEW_CON8: u32 = 0x78e0;
pub const AFE_GASRC2_NEW_CON9: u32 = 0x78e4;
pub const AFE_GASRC2_NEW_CON10: u32 = 0x78e8;
pub const AFE_GASRC2_NEW_CON11: u32 = 0x78ec;
pub const AFE_GASRC2_NEW_CON12: u32 = 0x78f0;
pub const AFE_GASRC2_NEW_CON13: u32 = 0x78f4;
pub const AFE_GASRC2_NEW_CON14: u32 = 0x78f8;
pub const AFE_GASRC2_NEW_IP_VERSION: u32 = 0x78fc;
pub const AFE_GASRC3_NEW_CON0: u32 = 0x7900;
pub const AFE_GASRC3_NEW_CON1: u32 = 0x7904;
pub const AFE_GASRC3_NEW_CON2: u32 = 0x7908;
pub const AFE_GASRC3_NEW_CON3: u32 = 0x790c;
pub const AFE_GASRC3_NEW_CON4: u32 = 0x7910;
pub const AFE_GASRC3_NEW_CON5: u32 = 0x7914;
pub const AFE_GASRC3_NEW_CON6: u32 = 0x7918;
pub const AFE_GASRC3_NEW_CON7: u32 = 0x791c;
pub const AFE_GASRC3_NEW_CON8: u32 = 0x7920;
pub const AFE_GASRC3_NEW_CON9: u32 = 0x7924;
pub const AFE_GASRC3_NEW_CON10: u32 = 0x7928;
pub const AFE_GASRC3_NEW_CON11: u32 = 0x792c;
pub const AFE_GASRC3_NEW_CON12: u32 = 0x7930;
pub const AFE_GASRC3_NEW_CON13: u32 = 0x7934;
pub const AFE_GASRC3_NEW_CON14: u32 = 0x7938;
pub const AFE_GASRC3_NEW_IP_VERSION: u32 = 0x793c;
pub const AFE_GASRC4_NEW_CON0: u32 = 0x7940;
pub const AFE_GASRC4_NEW_CON1: u32 = 0x7944;
pub const AFE_GASRC4_NEW_CON2: u32 = 0x7948;
pub const AFE_GASRC4_NEW_CON3: u32 = 0x794c;
pub const AFE_GASRC4_NEW_CON4: u32 = 0x7950;
pub const AFE_GASRC4_NEW_CON5: u32 = 0x7954;
pub const AFE_GASRC4_NEW_CON6: u32 = 0x7958;
pub const AFE_GASRC4_NEW_CON7: u32 = 0x795c;
pub const AFE_GASRC4_NEW_CON8: u32 = 0x7960;
pub const AFE_GASRC4_NEW_CON9: u32 = 0x7964;
pub const AFE_GASRC4_NEW_CON10: u32 = 0x7968;
pub const AFE_GASRC4_NEW_CON11: u32 = 0x796c;
pub const AFE_GASRC4_NEW_CON12: u32 = 0x7970;
pub const AFE_GASRC4_NEW_CON13: u32 = 0x7974;
pub const AFE_GASRC4_NEW_CON14: u32 = 0x7978;
pub const AFE_GASRC4_NEW_IP_VERSION: u32 = 0x797c;
pub const AFE_GASRC5_NEW_CON0: u32 = 0x7980;
pub const AFE_GASRC5_NEW_CON1: u32 = 0x7984;
pub const AFE_GASRC5_NEW_CON2: u32 = 0x7988;
pub const AFE_GASRC5_NEW_CON3: u32 = 0x798c;
pub const AFE_GASRC5_NEW_CON4: u32 = 0x7990;
pub const AFE_GASRC5_NEW_CON5: u32 = 0x7994;
pub const AFE_GASRC5_NEW_CON6: u32 = 0x7998;
pub const AFE_GASRC5_NEW_CON7: u32 = 0x799c;
pub const AFE_GASRC5_NEW_CON8: u32 = 0x79a0;
pub const AFE_GASRC5_NEW_CON9: u32 = 0x79a4;
pub const AFE_GASRC5_NEW_CON10: u32 = 0x79a8;
pub const AFE_GASRC5_NEW_CON11: u32 = 0x79ac;
pub const AFE_GASRC5_NEW_CON12: u32 = 0x79b0;
pub const AFE_GASRC5_NEW_CON13: u32 = 0x79b4;
pub const AFE_GASRC5_NEW_CON14: u32 = 0x79b8;
pub const AFE_GASRC5_NEW_IP_VERSION: u32 = 0x79bc;
pub const AFE_GASRC6_NEW_CON0: u32 = 0x79c0;
pub const AFE_GASRC6_NEW_CON1: u32 = 0x79c4;
pub const AFE_GASRC6_NEW_CON2: u32 = 0x79c8;
pub const AFE_GASRC6_NEW_CON3: u32 = 0x79cc;
pub const AFE_GASRC6_NEW_CON4: u32 = 0x79d0;
pub const AFE_GASRC6_NEW_CON5: u32 = 0x79d4;
pub const AFE_GASRC6_NEW_CON6: u32 = 0x79d8;
pub const AFE_GASRC6_NEW_CON7: u32 = 0x79dc;
pub const AFE_GASRC6_NEW_CON8: u32 = 0x79e0;
pub const AFE_GASRC6_NEW_CON9: u32 = 0x79e4;
pub const AFE_GASRC6_NEW_CON10: u32 = 0x79e8;
pub const AFE_GASRC6_NEW_CON11: u32 = 0x79ec;
pub const AFE_GASRC6_NEW_CON12: u32 = 0x79f0;
pub const AFE_GASRC6_NEW_CON13: u32 = 0x79f4;
pub const AFE_GASRC6_NEW_CON14: u32 = 0x79f8;
pub const AFE_GASRC6_NEW_IP_VERSION: u32 = 0x79fc;
pub const AFE_GASRC7_NEW_CON0: u32 = 0x7a00;
pub const AFE_GASRC7_NEW_CON1: u32 = 0x7a04;
pub const AFE_GASRC7_NEW_CON2: u32 = 0x7a08;
pub const AFE_GASRC7_NEW_CON3: u32 = 0x7a0c;
pub const AFE_GASRC7_NEW_CON4: u32 = 0x7a10;
pub const AFE_GASRC7_NEW_CON5: u32 = 0x7a14;
pub const AFE_GASRC7_NEW_CON6: u32 = 0x7a18;
pub const AFE_GASRC7_NEW_CON7: u32 = 0x7a1c;
pub const AFE_GASRC7_NEW_CON8: u32 = 0x7a20;
pub const AFE_GASRC7_NEW_CON9: u32 = 0x7a24;
pub const AFE_GASRC7_NEW_CON10: u32 = 0x7a28;
pub const AFE_GASRC7_NEW_CON11: u32 = 0x7a2c;
pub const AFE_GASRC7_NEW_CON12: u32 = 0x7a30;
pub const AFE_GASRC7_NEW_CON13: u32 = 0x7a34;
pub const AFE_GASRC7_NEW_CON14: u32 = 0x7a38;
pub const AFE_GASRC7_NEW_IP_VERSION: u32 = 0x7a3c;
pub const AFE_GASRC8_NEW_CON0: u32 = 0x7a40;
pub const AFE_GASRC8_NEW_CON1: u32 = 0x7a44;
pub const AFE_GASRC8_NEW_CON2: u32 = 0x7a48;
pub const AFE_GASRC8_NEW_CON3: u32 = 0x7a4c;
pub const AFE_GASRC8_NEW_CON4: u32 = 0x7a50;
pub const AFE_GASRC8_NEW_CON5: u32 = 0x7a54;
pub const AFE_GASRC8_NEW_CON6: u32 = 0x7a58;
pub const AFE_GASRC8_NEW_CON7: u32 = 0x7a5c;
pub const AFE_GASRC8_NEW_CON8: u32 = 0x7a60;
pub const AFE_GASRC8_NEW_CON9: u32 = 0x7a64;
pub const AFE_GASRC8_NEW_CON10: u32 = 0x7a68;
pub const AFE_GASRC8_NEW_CON11: u32 = 0x7a6c;
pub const AFE_GASRC8_NEW_CON12: u32 = 0x7a70;
pub const AFE_GASRC8_NEW_CON13: u32 = 0x7a74;
pub const AFE_GASRC8_NEW_CON14: u32 = 0x7a78;
pub const AFE_GASRC8_NEW_IP_VERSION: u32 = 0x7a7c;
pub const AFE_GASRC9_NEW_CON0: u32 = 0x7a80;
pub const AFE_GASRC9_NEW_CON1: u32 = 0x7a84;
pub const AFE_GASRC9_NEW_CON2: u32 = 0x7a88;
pub const AFE_GASRC9_NEW_CON3: u32 = 0x7a8c;
pub const AFE_GASRC9_NEW_CON4: u32 = 0x7a90;
pub const AFE_GASRC9_NEW_CON5: u32 = 0x7a94;
pub const AFE_GASRC9_NEW_CON6: u32 = 0x7a98;
pub const AFE_GASRC9_NEW_CON7: u32 = 0x7a9c;
pub const AFE_GASRC9_NEW_CON8: u32 = 0x7aa0;
pub const AFE_GASRC9_NEW_CON9: u32 = 0x7aa4;
pub const AFE_GASRC9_NEW_CON10: u32 = 0x7aa8;
pub const AFE_GASRC9_NEW_CON11: u32 = 0x7aac;
pub const AFE_GASRC9_NEW_CON12: u32 = 0x7ab0;
pub const AFE_GASRC9_NEW_CON13: u32 = 0x7ab4;
pub const AFE_GASRC9_NEW_CON14: u32 = 0x7ab8;
pub const AFE_GASRC9_NEW_IP_VERSION: u32 = 0x7abc;
pub const AFE_GASRC10_NEW_CON0: u32 = 0x7ac0;
pub const AFE_GASRC10_NEW_CON1: u32 = 0x7ac4;
pub const AFE_GASRC10_NEW_CON2: u32 = 0x7ac8;
pub const AFE_GASRC10_NEW_CON3: u32 = 0x7acc;
pub const AFE_GASRC10_NEW_CON4: u32 = 0x7ad0;
pub const AFE_GASRC10_NEW_CON5: u32 = 0x7ad4;
pub const AFE_GASRC10_NEW_CON6: u32 = 0x7ad8;
pub const AFE_GASRC10_NEW_CON7: u32 = 0x7adc;
pub const AFE_GASRC10_NEW_CON8: u32 = 0x7ae0;
pub const AFE_GASRC10_NEW_CON9: u32 = 0x7ae4;
pub const AFE_GASRC10_NEW_CON10: u32 = 0x7ae8;
pub const AFE_GASRC10_NEW_CON11: u32 = 0x7aec;
pub const AFE_GASRC10_NEW_CON12: u32 = 0x7af0;
pub const AFE_GASRC10_NEW_CON13: u32 = 0x7af4;
pub const AFE_GASRC10_NEW_CON14: u32 = 0x7af8;
pub const AFE_GASRC10_NEW_IP_VERSION: u32 = 0x7afc;
pub const AFE_GASRC11_NEW_CON0: u32 = 0x7b00;
pub const AFE_GASRC11_NEW_CON1: u32 = 0x7b04;
pub const AFE_GASRC11_NEW_CON2: u32 = 0x7b08;
pub const AFE_GASRC11_NEW_CON3: u32 = 0x7b0c;
pub const AFE_GASRC11_NEW_CON4: u32 = 0x7b10;
pub const AFE_GASRC11_NEW_CON5: u32 = 0x7b14;
pub const AFE_GASRC11_NEW_CON6: u32 = 0x7b18;
pub const AFE_GASRC11_NEW_CON7: u32 = 0x7b1c;
pub const AFE_GASRC11_NEW_CON8: u32 = 0x7b20;
pub const AFE_GASRC11_NEW_CON9: u32 = 0x7b24;
pub const AFE_GASRC11_NEW_CON10: u32 = 0x7b28;
pub const AFE_GASRC11_NEW_CON11: u32 = 0x7b2c;
pub const AFE_GASRC11_NEW_CON12: u32 = 0x7b30;
pub const AFE_GASRC11_NEW_CON13: u32 = 0x7b34;
pub const AFE_GASRC11_NEW_CON14: u32 = 0x7b38;
pub const AFE_GASRC11_NEW_IP_VERSION: u32 = 0x7b3c;
pub const AFE_GASRC12_NEW_CON0: u32 = 0x7b40;
pub const AFE_GASRC12_NEW_CON1: u32 = 0x7b44;
pub const AFE_GASRC12_NEW_CON2: u32 = 0x7b48;
pub const AFE_GASRC12_NEW_CON3: u32 = 0x7b4c;
pub const AFE_GASRC12_NEW_CON4: u32 = 0x7b50;
pub const AFE_GASRC12_NEW_CON5: u32 = 0x7b54;
pub const AFE_GASRC12_NEW_CON6: u32 = 0x7b58;
pub const AFE_GASRC12_NEW_CON7: u32 = 0x7b5c;
pub const AFE_GASRC12_NEW_CON8: u32 = 0x7b60;
pub const AFE_GASRC12_NEW_CON9: u32 = 0x7b64;
pub const AFE_GASRC12_NEW_CON10: u32 = 0x7b68;
pub const AFE_GASRC12_NEW_CON11: u32 = 0x7b6c;
pub const AFE_GASRC12_NEW_CON12: u32 = 0x7b70;
pub const AFE_GASRC12_NEW_CON13: u32 = 0x7b74;
pub const AFE_GASRC12_NEW_CON14: u32 = 0x7b78;
pub const AFE_GASRC12_NEW_IP_VERSION: u32 = 0x7b7c;
pub const AFE_GASRC13_NEW_CON0: u32 = 0x7b80;
pub const AFE_GASRC13_NEW_CON1: u32 = 0x7b84;
pub const AFE_GASRC13_NEW_CON2: u32 = 0x7b88;
pub const AFE_GASRC13_NEW_CON3: u32 = 0x7b8c;
pub const AFE_GASRC13_NEW_CON4: u32 = 0x7b90;
pub const AFE_GASRC13_NEW_CON5: u32 = 0x7b94;
pub const AFE_GASRC13_NEW_CON6: u32 = 0x7b98;
pub const AFE_GASRC13_NEW_CON7: u32 = 0x7b9c;
pub const AFE_GASRC13_NEW_CON8: u32 = 0x7ba0;
pub const AFE_GASRC13_NEW_CON9: u32 = 0x7ba4;
pub const AFE_GASRC13_NEW_CON10: u32 = 0x7ba8;
pub const AFE_GASRC13_NEW_CON11: u32 = 0x7bac;
pub const AFE_GASRC13_NEW_CON12: u32 = 0x7bb0;
pub const AFE_GASRC13_NEW_CON13: u32 = 0x7bb4;
pub const AFE_GASRC13_NEW_CON14: u32 = 0x7bb8;
pub const AFE_GASRC13_NEW_IP_VERSION: u32 = 0x7bbc;
pub const AFE_GASRC14_NEW_CON0: u32 = 0x7bc0;
pub const AFE_GASRC14_NEW_CON1: u32 = 0x7bc4;
pub const AFE_GASRC14_NEW_CON2: u32 = 0x7bc8;
pub const AFE_GASRC14_NEW_CON3: u32 = 0x7bcc;
pub const AFE_GASRC14_NEW_CON4: u32 = 0x7bd0;
pub const AFE_GASRC14_NEW_CON5: u32 = 0x7bd4;
pub const AFE_GASRC14_NEW_CON6: u32 = 0x7bd8;
pub const AFE_GASRC14_NEW_CON7: u32 = 0x7bdc;
pub const AFE_GASRC14_NEW_CON8: u32 = 0x7be0;
pub const AFE_GASRC14_NEW_CON9: u32 = 0x7be4;
pub const AFE_GASRC14_NEW_CON10: u32 = 0x7be8;
pub const AFE_GASRC14_NEW_CON11: u32 = 0x7bec;
pub const AFE_GASRC14_NEW_CON12: u32 = 0x7bf0;
pub const AFE_GASRC14_NEW_CON13: u32 = 0x7bf4;
pub const AFE_GASRC14_NEW_CON14: u32 = 0x7bf8;
pub const AFE_GASRC14_NEW_IP_VERSION: u32 = 0x7bfc;
pub const AFE_GASRC15_NEW_CON0: u32 = 0x7c00;
pub const AFE_GASRC15_NEW_CON1: u32 = 0x7c04;
pub const AFE_GASRC15_NEW_CON2: u32 = 0x7c08;
pub const AFE_GASRC15_NEW_CON3: u32 = 0x7c0c;
pub const AFE_GASRC15_NEW_CON4: u32 = 0x7c10;
pub const AFE_GASRC15_NEW_CON5: u32 = 0x7c14;
pub const AFE_GASRC15_NEW_CON6: u32 = 0x7c18;
pub const AFE_GASRC15_NEW_CON7: u32 = 0x7c1c;
pub const AFE_GASRC15_NEW_CON8: u32 = 0x7c20;
pub const AFE_GASRC15_NEW_CON9: u32 = 0x7c24;
pub const AFE_GASRC15_NEW_CON10: u32 = 0x7c28;
pub const AFE_GASRC15_NEW_CON11: u32 = 0x7c2c;
pub const AFE_GASRC15_NEW_CON12: u32 = 0x7c30;
pub const AFE_GASRC15_NEW_CON13: u32 = 0x7c34;
pub const AFE_GASRC15_NEW_CON14: u32 = 0x7c38;
pub const AFE_GASRC15_NEW_IP_VERSION: u32 = 0x7c3c;

pub const AFE_MAX_REGISTER: u32 = AFE_GASRC15_NEW_IP_VERSION;

pub const AFE_IRQ_STATUS_BITS: u32 = 0x87FFFFFF;
pub const AFE_IRQ_CNT_SHIFT: u32 = 0;
pub const AFE_IRQ_CNT_MASK: u32 = 0xffffff;


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
