/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mt8189-reg.h  --  Mediatek 8189 audio driver reg definition
 *
 * Copyright (c) 2025 MediaTek Inc.
 * Author: Darren Ye <darren.ye@mediatek.com>
 */

/* Header guard removed in Rust translation. */

/* reg bit enum */
pub const MT8189_MEMIF_PBUF_SIZE_32_BYTES: u32 = 0;
pub const MT8189_MEMIF_PBUF_SIZE_64_BYTES: u32 = 1;
pub const MT8189_MEMIF_PBUF_SIZE_128_BYTES: u32 = 2;
pub const MT8189_MEMIF_PBUF_SIZE_256_BYTES: u32 = 3;
pub const MT8189_MEMIF_PBUF_SIZE_NUM: u32 = 4;

/*


/*  /* reg bit enum */ */

/*****************************************************************************
/* AUDIO_TOP_CON0 */
pub const PDN_MTKAIFV4_SFT: u32 = 25;
pub const PDN_MTKAIFV4_MASK: u32 = 0x1;
pub const PDN_MTKAIFV4_MASK_SFT: u32 = 0x1 << 25;
pub const PDN_FM_I2S_SFT: u32 = 24;
pub const PDN_FM_I2S_MASK: u32 = 0x1;
pub const PDN_FM_I2S_MASK_SFT: u32 = 0x1 << 24;
pub const PDN_HW_GAIN01_SFT: u32 = 21;
pub const PDN_HW_GAIN01_MASK: u32 = 0x1;
pub const PDN_HW_GAIN01_MASK_SFT: u32 = 0x1 << 21;
pub const PDN_HW_GAIN23_SFT: u32 = 20;
pub const PDN_HW_GAIN23_MASK: u32 = 0x1;
pub const PDN_HW_GAIN23_MASK_SFT: u32 = 0x1 << 20;
pub const PDN_STF_SFT: u32 = 19;
pub const PDN_STF_MASK: u32 = 0x1;
pub const PDN_STF_MASK_SFT: u32 = 0x1 << 19;
pub const PDN_CM0_SFT: u32 = 18;
pub const PDN_CM0_MASK: u32 = 0x1;
pub const PDN_CM0_MASK_SFT: u32 = 0x1 << 18;
pub const PDN_CM1_SFT: u32 = 17;
pub const PDN_CM1_MASK: u32 = 0x1;
pub const PDN_CM1_MASK_SFT: u32 = 0x1 << 17;
pub const PDN_PCM0_SFT: u32 = 14;
pub const PDN_PCM0_MASK: u32 = 0x1;
pub const PDN_PCM0_MASK_SFT: u32 = 0x1 << 14;
pub const PDN_DL0_NLE_SFT: u32 = 11;
pub const PDN_DL0_NLE_MASK: u32 = 0x1;
pub const PDN_DL0_NLE_MASK_SFT: u32 = 0x1 << 11;
pub const PDN_DL0_PREDIS_SFT: u32 = 10;
pub const PDN_DL0_PREDIS_MASK: u32 = 0x1;
pub const PDN_DL0_PREDIS_MASK_SFT: u32 = 0x1 << 10;
pub const PDN_DL0_DAC_SFT: u32 = 9;
pub const PDN_DL0_DAC_MASK: u32 = 0x1;
pub const PDN_DL0_DAC_MASK_SFT: u32 = 0x1 << 9;
pub const PDN_DL0_DAC_HIRES_SFT: u32 = 8;
pub const PDN_DL0_DAC_HIRES_MASK: u32 = 0x1;
pub const PDN_DL0_DAC_HIRES_MASK_SFT: u32 = 0x1 << 8;
pub const PDN_DL0_DAC_TML_SFT: u32 = 7;
pub const PDN_DL0_DAC_TML_MASK: u32 = 0x1;
pub const PDN_DL0_DAC_TML_MASK_SFT: u32 = 0x1 << 7;

/* AUDIO_TOP_CON1 */
pub const PDN_UL0_ADC_SFT: u32 = 23;
pub const PDN_UL0_ADC_MASK: u32 = 0x1;
pub const PDN_UL0_ADC_MASK_SFT: u32 = 0x1 << 23;
pub const PDN_UL0_TML_SFT: u32 = 22;
pub const PDN_UL0_TML_MASK: u32 = 0x1;
pub const PDN_UL0_TML_MASK_SFT: u32 = 0x1 << 22;
pub const PDN_UL0_ADC_HIRES_SFT: u32 = 21;
pub const PDN_UL0_ADC_HIRES_MASK: u32 = 0x1;
pub const PDN_UL0_ADC_HIRES_MASK_SFT: u32 = 0x1 << 21;
pub const PDN_UL0_ADC_HIRES_TML_SFT: u32 = 20;
pub const PDN_UL0_ADC_HIRES_TML_MASK: u32 = 0x1;
pub const PDN_UL0_ADC_HIRES_TML_MASK_SFT: u32 = 0x1 << 20;
pub const PDN_UL1_ADC_SFT: u32 = 19;
pub const PDN_UL1_ADC_MASK: u32 = 0x1;
pub const PDN_UL1_ADC_MASK_SFT: u32 = 0x1 << 19;
pub const PDN_UL1_TML_SFT: u32 = 18;
pub const PDN_UL1_TML_MASK: u32 = 0x1;
pub const PDN_UL1_TML_MASK_SFT: u32 = 0x1 << 18;
pub const PDN_UL1_ADC_HIRES_SFT: u32 = 17;
pub const PDN_UL1_ADC_HIRES_MASK: u32 = 0x1;
pub const PDN_UL1_ADC_HIRES_MASK_SFT: u32 = 0x1 << 17;
pub const PDN_UL1_ADC_HIRES_TML_SFT: u32 = 16;
pub const PDN_UL1_ADC_HIRES_TML_MASK: u32 = 0x1;
pub const PDN_UL1_ADC_HIRES_TML_MASK_SFT: u32 = 0x1 << 16;
pub const PDN_DMIC0_ADC_SFT: u32 = 7;
pub const PDN_DMIC0_ADC_MASK: u32 = 0x1;
pub const PDN_DMIC0_ADC_MASK_SFT: u32 = 0x1 << 7;
pub const PDN_DMIC1_ADC_SFT: u32 = 3;
pub const PDN_DMIC1_ADC_MASK: u32 = 0x1;
pub const PDN_DMIC1_ADC_MASK_SFT: u32 = 0x1 << 3;

/* AUDIO_TOP_CON2 */
pub const PDN_TDM_OUT_SFT: u32 = 24;
pub const PDN_TDM_OUT_MASK: u32 = 0x1;
pub const PDN_TDM_OUT_MASK_SFT: u32 = 0x1 << 24;
pub const PDN_ETDM_OUT0_SFT: u32 = 21;
pub const PDN_ETDM_OUT0_MASK: u32 = 0x1;
pub const PDN_ETDM_OUT0_MASK_SFT: u32 = 0x1 << 21;
pub const PDN_ETDM_OUT1_SFT: u32 = 20;
pub const PDN_ETDM_OUT1_MASK: u32 = 0x1;
pub const PDN_ETDM_OUT1_MASK_SFT: u32 = 0x1 << 20;
pub const PDN_ETDM_OUT4_SFT: u32 = 17;
pub const PDN_ETDM_OUT4_MASK: u32 = 0x1;
pub const PDN_ETDM_OUT4_MASK_SFT: u32 = 0x1 << 17;
pub const PDN_ETDM_IN0_SFT: u32 = 13;
pub const PDN_ETDM_IN0_MASK: u32 = 0x1;
pub const PDN_ETDM_IN0_MASK_SFT: u32 = 0x1 << 13;
pub const PDN_ETDM_IN1_SFT: u32 = 12;
pub const PDN_ETDM_IN1_MASK: u32 = 0x1;
pub const PDN_ETDM_IN1_MASK_SFT: u32 = 0x1 << 12;

/* AUDIO_TOP_CON3 */
pub const PDN_CONNSYS_I2S_ASRC_SFT: u32 = 25;
pub const PDN_CONNSYS_I2S_ASRC_MASK: u32 = 0x1;
pub const PDN_CONNSYS_I2S_ASRC_MASK_SFT: u32 = 0x1 << 25;
pub const PDN_GENERAL0_ASRC_SFT: u32 = 24;
pub const PDN_GENERAL0_ASRC_MASK: u32 = 0x1;
pub const PDN_GENERAL0_ASRC_MASK_SFT: u32 = 0x1 << 24;
pub const PDN_GENERAL1_ASRC_SFT: u32 = 23;
pub const PDN_GENERAL1_ASRC_MASK: u32 = 0x1;
pub const PDN_GENERAL1_ASRC_MASK_SFT: u32 = 0x1 << 23;
pub const PDN_GENERAL2_ASRC_SFT: u32 = 22;
pub const PDN_GENERAL2_ASRC_MASK: u32 = 0x1;
pub const PDN_GENERAL2_ASRC_MASK_SFT: u32 = 0x1 << 22;
pub const PDN_GENERAL3_ASRC_SFT: u32 = 21;
pub const PDN_GENERAL3_ASRC_MASK: u32 = 0x1;
pub const PDN_GENERAL3_ASRC_MASK_SFT: u32 = 0x1 << 21;
pub const PDN_GENERAL4_ASRC_SFT: u32 = 20;
pub const PDN_GENERAL4_ASRC_MASK: u32 = 0x1;
pub const PDN_GENERAL4_ASRC_MASK_SFT: u32 = 0x1 << 20;

/* AUDIO_TOP_CON4 */
pub const PDN_APLL_TUNER1_SFT: u32 = 13;
pub const PDN_APLL_TUNER1_MASK: u32 = 0x1;
pub const PDN_APLL_TUNER1_MASK_SFT: u32 = 0x1 << 13;
pub const PDN_APLL_TUNER2_SFT: u32 = 12;
pub const PDN_APLL_TUNER2_MASK: u32 = 0x1;
pub const PDN_APLL_TUNER2_MASK_SFT: u32 = 0x1 << 12;
pub const CG_H208M_CK_SFT: u32 = 4;
pub const CG_H208M_CK_MASK: u32 = 0x1;
pub const CG_H208M_CK_MASK_SFT: u32 = 0x1 << 4;
pub const CG_APLL2_CK_SFT: u32 = 3;
pub const CG_APLL2_CK_MASK: u32 = 0x1;
pub const CG_APLL2_CK_MASK_SFT: u32 = 0x1 << 3;
pub const CG_APLL1_CK_SFT: u32 = 2;
pub const CG_APLL1_CK_MASK: u32 = 0x1;
pub const CG_APLL1_CK_MASK_SFT: u32 = 0x1 << 2;
pub const CG_AUDIO_F26M_CK_SFT: u32 = 1;
pub const CG_AUDIO_F26M_CK_MASK: u32 = 0x1;
pub const CG_AUDIO_F26M_CK_MASK_SFT: u32 = 0x1 << 1;
pub const CG_AUDIO_HOPPING_CK_SFT: u32 = 0;
pub const CG_AUDIO_HOPPING_CK_MASK: u32 = 0x1;
pub const CG_AUDIO_HOPPING_CK_MASK_SFT: u32 = 0x1 << 0;

/* AUDIO_ENGEN_CON0 */
/* AUDIO_ENGEN_CON0_USER1 */
/* AUDIO_ENGEN_CON0_USER2 */
pub const MULTI_USER_BYPASS_SFT: u32 = 17;
pub const MULTI_USER_BYPASS_MASK: u32 = 0x1;
pub const MULTI_USER_BYPASS_MASK_SFT: u32 = 0x1 << 17;
pub const MULTI_USER_RST_SFT: u32 = 16;
pub const MULTI_USER_RST_MASK: u32 = 0x1;
pub const MULTI_USER_RST_MASK_SFT: u32 = 0x1 << 16;
pub const AUDIO_F26M_EN_RST_SFT: u32 = 8;
pub const AUDIO_F26M_EN_RST_MASK: u32 = 0x1;
pub const AUDIO_F26M_EN_RST_MASK_SFT: u32 = 0x1 << 8;
pub const AUDIO_APLL2_EN_ON_SFT: u32 = 3;
pub const AUDIO_APLL2_EN_ON_MASK: u32 = 0x1;
pub const AUDIO_APLL2_EN_ON_MASK_SFT: u32 = 0x1 << 3;
pub const AUDIO_APLL1_EN_ON_SFT: u32 = 2;
pub const AUDIO_APLL1_EN_ON_MASK: u32 = 0x1;
pub const AUDIO_APLL1_EN_ON_MASK_SFT: u32 = 0x1 << 2;
pub const AUDIO_F3P25M_EN_ON_SFT: u32 = 1;
pub const AUDIO_F3P25M_EN_ON_MASK: u32 = 0x1;
pub const AUDIO_F3P25M_EN_ON_MASK_SFT: u32 = 0x1 << 1;
pub const AUDIO_26M_EN_ON_SFT: u32 = 0;
pub const AUDIO_26M_EN_ON_MASK: u32 = 0x1;
pub const AUDIO_26M_EN_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_SINEGEN_CON0 */
pub const DAC_EN_SFT: u32 = 26;
pub const DAC_EN_MASK: u32 = 0x1;
pub const DAC_EN_MASK_SFT: u32 = 0x1 << 26;
pub const TIE_SW_CH2_SFT: u32 = 25;
pub const TIE_SW_CH2_MASK: u32 = 0x1;
pub const TIE_SW_CH2_MASK_SFT: u32 = 0x1 << 25;
pub const TIE_SW_CH1_SFT: u32 = 24;
pub const TIE_SW_CH1_MASK: u32 = 0x1;
pub const TIE_SW_CH1_MASK_SFT: u32 = 0x1 << 24;
pub const AMP_DIV_CH2_SFT: u32 = 20;
pub const AMP_DIV_CH2_MASK: u32 = 0xf;
pub const AMP_DIV_CH2_MASK_SFT: u32 = 0xf << 20;
pub const FREQ_DIV_CH2_SFT: u32 = 12;
pub const FREQ_DIV_CH2_MASK: u32 = 0x1f;
pub const FREQ_DIV_CH2_MASK_SFT: u32 = 0x1f << 12;
pub const AMP_DIV_CH1_SFT: u32 = 8;
pub const AMP_DIV_CH1_MASK: u32 = 0xf;
pub const AMP_DIV_CH1_MASK_SFT: u32 = 0xf << 8;
pub const FREQ_DIV_CH1_SFT: u32 = 0;
pub const FREQ_DIV_CH1_MASK: u32 = 0x1f;
pub const FREQ_DIV_CH1_MASK_SFT: u32 = 0x1f << 0;

/* AFE_SINEGEN_CON1 */
pub const SINE_DOMAIN_SFT: u32 = 20;
pub const SINE_DOMAIN_MASK: u32 = 0x7;
pub const SINE_DOMAIN_MASK_SFT: u32 = 0x7 << 20;
pub const SINE_MODE_SFT: u32 = 12;
pub const SINE_MODE_MASK: u32 = 0x1f;
pub const SINE_MODE_MASK_SFT: u32 = 0x1f << 12;
pub const INNER_LOOP_BACKI_SEL_SFT: u32 = 8;
pub const INNER_LOOP_BACKI_SEL_MASK: u32 = 0x1;
pub const INNER_LOOP_BACKI_SEL_MASK_SFT: u32 = 0x1 << 8;
pub const INNER_LOOP_BACK_MODE_SFT: u32 = 0;
pub const INNER_LOOP_BACK_MODE_MASK: u32 = 0xff;
pub const INNER_LOOP_BACK_MODE_MASK_SFT: u32 = 0xff << 0;

/* AFE_SINEGEN_CON2 */
pub const TIE_CH1_CONSTANT_SFT: u32 = 0;
pub const TIE_CH1_CONSTANT_MASK: u32 = 0xffffffff;
pub const TIE_CH1_CONSTANT_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_SINEGEN_CON3 */
pub const TIE_CH2_CONSTANT_SFT: u32 = 0;
pub const TIE_CH2_CONSTANT_MASK: u32 = 0xffffffff;
pub const TIE_CH2_CONSTANT_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_APLL1_TUNER_CFG */
pub const UPPER_BOUND_SFT: u32 = 8;
pub const UPPER_BOUND_MASK: u32 = 0xff;
pub const UPPER_BOUND_MASK_SFT: u32 = 0xff << 8;
pub const APLL_DIV_SFT: u32 = 4;
pub const APLL_DIV_MASK: u32 = 0xf;
pub const APLL_DIV_MASK_SFT: u32 = 0xf << 4;
pub const XTAL_EN_128FS_SEL_SFT: u32 = 1;
pub const XTAL_EN_128FS_SEL_MASK: u32 = 0x3;
pub const XTAL_EN_128FS_SEL_MASK_SFT: u32 = 0x3 << 1;
pub const FREQ_TUNER_EN_SFT: u32 = 0;
pub const FREQ_TUNER_EN_MASK: u32 = 0x1;
pub const FREQ_TUNER_EN_MASK_SFT: u32 = 0x1 << 0;

/* AFE_APLL1_TUNER_MON0 */
pub const TUNER_MON_SFT: u32 = 0;
pub const TUNER_MON_MASK: u32 = 0xffffffff;
pub const TUNER_MON_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_APLL2_TUNER_CFG */
/* duplicate C macro: UPPER_BOUND_SFT = 8 */
/* duplicate C macro: UPPER_BOUND_MASK = 0xff */
/* duplicate C macro: UPPER_BOUND_MASK_SFT = 0xff << 8 */
/* duplicate C macro: APLL_DIV_SFT = 4 */
/* duplicate C macro: APLL_DIV_MASK = 0xf */
/* duplicate C macro: APLL_DIV_MASK_SFT = 0xf << 4 */
/* duplicate C macro: XTAL_EN_128FS_SEL_SFT = 1 */
/* duplicate C macro: XTAL_EN_128FS_SEL_MASK = 0x3 */
/* duplicate C macro: XTAL_EN_128FS_SEL_MASK_SFT = 0x3 << 1 */
/* duplicate C macro: FREQ_TUNER_EN_SFT = 0 */
/* duplicate C macro: FREQ_TUNER_EN_MASK = 0x1 */
/* duplicate C macro: FREQ_TUNER_EN_MASK_SFT = 0x1 << 0 */

/* AFE_APLL2_TUNER_MON0 */
/* duplicate C macro: TUNER_MON_SFT = 0 */
/* duplicate C macro: TUNER_MON_MASK = 0xffffffff */
/* duplicate C macro: TUNER_MON_MASK_SFT = 0xffffffff << 0 */

/* AUDIO_TOP_RG0 */
pub const RESERVE_RG_SFT: u32 = 0;
pub const RESERVE_RG_MASK: u32 = 0xffffffff;
pub const RESERVE_RG_MASK_SFT: u32 = 0xffffffff << 0;

/* AUDIO_TOP_RG1 */
/* duplicate C macro: RESERVE_RG_SFT = 0 */
/* duplicate C macro: RESERVE_RG_MASK = 0xffffffff */
/* duplicate C macro: RESERVE_RG_MASK_SFT = 0xffffffff << 0 */

/* AUDIO_TOP_RG2 */
/* duplicate C macro: RESERVE_RG_SFT = 0 */
/* duplicate C macro: RESERVE_RG_MASK = 0xffffffff */
/* duplicate C macro: RESERVE_RG_MASK_SFT = 0xffffffff << 0 */

/* AUDIO_TOP_RG3 */
/* duplicate C macro: RESERVE_RG_SFT = 0 */
/* duplicate C macro: RESERVE_RG_MASK = 0xffffffff */
/* duplicate C macro: RESERVE_RG_MASK_SFT = 0xffffffff << 0 */

/* AUDIO_TOP_RG4 */
/* duplicate C macro: RESERVE_RG_SFT = 0 */
/* duplicate C macro: RESERVE_RG_MASK = 0xffffffff */
/* duplicate C macro: RESERVE_RG_MASK_SFT = 0xffffffff << 0 */

/* AFE_SPM_CONTROL_REQ */
pub const AFE_DDREN_REQ_SFT: u32 = 4;
pub const AFE_DDREN_REQ_MASK: u32 = 0x1;
pub const AFE_DDREN_REQ_MASK_SFT: u32 = 0x1 << 4;
pub const AFE_INFRA_REQ_SFT: u32 = 3;
pub const AFE_INFRA_REQ_MASK: u32 = 0x1;
pub const AFE_INFRA_REQ_MASK_SFT: u32 = 0x1 << 3;
pub const AFE_VRF18_REQ_SFT: u32 = 2;
pub const AFE_VRF18_REQ_MASK: u32 = 0x1;
pub const AFE_VRF18_REQ_MASK_SFT: u32 = 0x1 << 2;
pub const AFE_APSRC_REQ_SFT: u32 = 1;
pub const AFE_APSRC_REQ_MASK: u32 = 0x1;
pub const AFE_APSRC_REQ_MASK_SFT: u32 = 0x1 << 1;
pub const AFE_SRCCLKENA_REQ_SFT: u32 = 0;
pub const AFE_SRCCLKENA_REQ_MASK: u32 = 0x1;
pub const AFE_SRCCLKENA_REQ_MASK_SFT: u32 = 0x1 << 0;

/* AFE_SPM_CONTROL_ACK */
pub const SPM_RESOURCE_CONTROL_ACK_SFT: u32 = 0;
pub const SPM_RESOURCE_CONTROL_ACK_MASK: u32 = 0xffffffff;
pub const SPM_RESOURCE_CONTROL_ACK_MASK_SFT: u32 = 0xffffffff << 0;

/* AUD_TOP_CFG_VCORE_RG */
pub const AUD_TOP_CFG_SFT: u32 = 0;
pub const AUD_TOP_CFG_MASK: u32 = 0xffffffff;
pub const AUD_TOP_CFG_MASK_SFT: u32 = 0xffffffff << 0;

/* AUDIO_TOP_IP_VERSION */
pub const AUDIO_TOP_IP_VERSION_SFT: u32 = 0;
pub const AUDIO_TOP_IP_VERSION_MASK: u32 = 0xffffffff;
pub const AUDIO_TOP_IP_VERSION_MASK_SFT: u32 = 0xffffffff << 0;

/* AUDIO_ENGEN_CON0_MON */
pub const AUDIO_ENGEN_MON_SFT: u32 = 0;
pub const AUDIO_ENGEN_MON_MASK: u32 = 0xffffffff;
pub const AUDIO_ENGEN_MON_MASK_SFT: u32 = 0xffffffff << 0;

/* AUD_TOP_CFG_VLP_RG */
/* duplicate C macro: AUD_TOP_CFG_SFT = 0 */
/* duplicate C macro: AUD_TOP_CFG_MASK = 0xffffffff */
/* duplicate C macro: AUD_TOP_CFG_MASK_SFT = 0xffffffff << 0 */

/* AUD_TOP_MON_RG */
pub const AUD_TOP_MON_SFT: u32 = 0;
pub const AUD_TOP_MON_MASK: u32 = 0xffffffff;
pub const AUD_TOP_MON_MASK_SFT: u32 = 0xffffffff << 0;

/* AUDIO_USE_DEFAULT_DELSEL0 */
pub const USE_DEFAULT_DELSEL_RG_SFT: u32 = 0;
pub const USE_DEFAULT_DELSEL_RG_MASK: u32 = 0xffffffff;
pub const USE_DEFAULT_DELSEL_RG_MASK_SFT: u32 = 0xffffffff << 0;

/* AUDIO_USE_DEFAULT_DELSEL1 */
/* duplicate C macro: USE_DEFAULT_DELSEL_RG_SFT = 0 */
/* duplicate C macro: USE_DEFAULT_DELSEL_RG_MASK = 0xffffffff */
/* duplicate C macro: USE_DEFAULT_DELSEL_RG_MASK_SFT = 0xffffffff << 0 */

/* AUDIO_USE_DEFAULT_DELSEL2 */
/* duplicate C macro: USE_DEFAULT_DELSEL_RG_SFT = 0 */
/* duplicate C macro: USE_DEFAULT_DELSEL_RG_MASK = 0xffffffff */
/* duplicate C macro: USE_DEFAULT_DELSEL_RG_MASK_SFT = 0xffffffff << 0 */

/* AFE_CONNSYS_I2S_IPM_VER_MON */
pub const RG_CONNSYS_I2S_IPM_VER_MON_SFT: u32 = 0;
pub const RG_CONNSYS_I2S_IPM_VER_MON_MASK: u32 = 0xffffffff;
pub const RG_CONNSYS_I2S_IPM_VER_MON_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_CONNSYS_I2S_MON_SEL */
pub const RG_CONNSYS_I2S_MON_SEL_SFT: u32 = 0;
pub const RG_CONNSYS_I2S_MON_SEL_MASK: u32 = 0xff;
pub const RG_CONNSYS_I2S_MON_SEL_MASK_SFT: u32 = 0xff << 0;

/* AFE_CONNSYS_I2S_MON */
pub const RG_CONNSYS_I2S_MON_SFT: u32 = 0;
pub const RG_CONNSYS_I2S_MON_MASK: u32 = 0xffffffff;
pub const RG_CONNSYS_I2S_MON_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_CONNSYS_I2S_CON */
pub const I2S_SOFT_RST_SFT: u32 = 31;
pub const I2S_SOFT_RST_MASK: u32 = 0x1;
pub const I2S_SOFT_RST_MASK_SFT: u32 = 0x1 << 31;
pub const BCK_NEG_EG_LATCH_SFT: u32 = 30;
pub const BCK_NEG_EG_LATCH_MASK: u32 = 0x1;
pub const BCK_NEG_EG_LATCH_MASK_SFT: u32 = 0x1 << 30;
pub const BCK_INV_SFT: u32 = 29;
pub const BCK_INV_MASK: u32 = 0x1;
pub const BCK_INV_MASK_SFT: u32 = 0x1 << 29;
pub const I2SIN_PAD_SEL_SFT: u32 = 28;
pub const I2SIN_PAD_SEL_MASK: u32 = 0x1;
pub const I2SIN_PAD_SEL_MASK_SFT: u32 = 0x1 << 28;
pub const I2S_LOOPBACK_SFT: u32 = 20;
pub const I2S_LOOPBACK_MASK: u32 = 0x1;
pub const I2S_LOOPBACK_MASK_SFT: u32 = 0x1 << 20;
pub const I2S_HDEN_SFT: u32 = 12;
pub const I2S_HDEN_MASK: u32 = 0x1;
pub const I2S_HDEN_MASK_SFT: u32 = 0x1 << 12;
pub const I2S_MODE_SFT: u32 = 8;
pub const I2S_MODE_MASK: u32 = 0xf;
pub const I2S_MODE_MASK_SFT: u32 = 0xf << 8;
pub const I2S_BYPSRC_SFT: u32 = 6;
pub const I2S_BYPSRC_MASK: u32 = 0x1;
pub const I2S_BYPSRC_MASK_SFT: u32 = 0x1 << 6;
pub const INV_LRCK_SFT: u32 = 5;
pub const INV_LRCK_MASK: u32 = 0x1;
pub const INV_LRCK_MASK_SFT: u32 = 0x1 << 5;
pub const I2S_FMT_SFT: u32 = 3;
pub const I2S_FMT_MASK: u32 = 0x1;
pub const I2S_FMT_MASK_SFT: u32 = 0x1 << 3;
pub const I2S_SRC_SFT: u32 = 2;
pub const I2S_SRC_MASK: u32 = 0x1;
pub const I2S_SRC_MASK_SFT: u32 = 0x1 << 2;
pub const I2S_WLEN_SFT: u32 = 1;
pub const I2S_WLEN_MASK: u32 = 0x1;
pub const I2S_WLEN_MASK_SFT: u32 = 0x1 << 1;
pub const I2S_EN_SFT: u32 = 0;
pub const I2S_EN_MASK: u32 = 0x1;
pub const I2S_EN_MASK_SFT: u32 = 0x1 << 0;

/* AFE_PCM0_INTF_CON0 */
pub const PCM0_HDEN_SFT: u32 = 26;
pub const PCM0_HDEN_MASK: u32 = 0x1;
pub const PCM0_HDEN_MASK_SFT: u32 = 0x1 << 26;
pub const PCM0_SYNC_DELSEL_SFT: u32 = 25;
pub const PCM0_SYNC_DELSEL_MASK: u32 = 0x1;
pub const PCM0_SYNC_DELSEL_MASK_SFT: u32 = 0x1 << 25;
pub const PCM0_TX_LR_SWAP_SFT: u32 = 24;
pub const PCM0_TX_LR_SWAP_MASK: u32 = 0x1;
pub const PCM0_TX_LR_SWAP_MASK_SFT: u32 = 0x1 << 24;
pub const PCM0_SYNC_OUT_INV_SFT: u32 = 23;
pub const PCM0_SYNC_OUT_INV_MASK: u32 = 0x1;
pub const PCM0_SYNC_OUT_INV_MASK_SFT: u32 = 0x1 << 23;
pub const PCM0_BCLK_OUT_INV_SFT: u32 = 22;
pub const PCM0_BCLK_OUT_INV_MASK: u32 = 0x1;
pub const PCM0_BCLK_OUT_INV_MASK_SFT: u32 = 0x1 << 22;
pub const PCM0_SYNC_IN_INV_SFT: u32 = 21;
pub const PCM0_SYNC_IN_INV_MASK: u32 = 0x1;
pub const PCM0_SYNC_IN_INV_MASK_SFT: u32 = 0x1 << 21;
pub const PCM0_BCLK_IN_INV_SFT: u32 = 20;
pub const PCM0_BCLK_IN_INV_MASK: u32 = 0x1;
pub const PCM0_BCLK_IN_INV_MASK_SFT: u32 = 0x1 << 20;
pub const PCM0_TX_LCH_RPT_SFT: u32 = 19;
pub const PCM0_TX_LCH_RPT_MASK: u32 = 0x1;
pub const PCM0_TX_LCH_RPT_MASK_SFT: u32 = 0x1 << 19;
pub const PCM0_VBT_16K_MODE_SFT: u32 = 18;
pub const PCM0_VBT_16K_MODE_MASK: u32 = 0x1;
pub const PCM0_VBT_16K_MODE_MASK_SFT: u32 = 0x1 << 18;
pub const PCM0_BIT_LENGTH_SFT: u32 = 16;
pub const PCM0_BIT_LENGTH_MASK: u32 = 0x3;
pub const PCM0_BIT_LENGTH_MASK_SFT: u32 = 0x3 << 16;
pub const PCM0_WLEN_SFT: u32 = 14;
pub const PCM0_WLEN_MASK: u32 = 0x3;
pub const PCM0_WLEN_MASK_SFT: u32 = 0x3 << 14;
pub const PCM0_SYNC_LENGTH_SFT: u32 = 9;
pub const PCM0_SYNC_LENGTH_MASK: u32 = 0x1f;
pub const PCM0_SYNC_LENGTH_MASK_SFT: u32 = 0x1f << 9;
pub const PCM0_SYNC_TYPE_SFT: u32 = 8;
pub const PCM0_SYNC_TYPE_MASK: u32 = 0x1;
pub const PCM0_SYNC_TYPE_MASK_SFT: u32 = 0x1 << 8;
pub const PCM0_BYP_ASRC_SFT: u32 = 7;
pub const PCM0_BYP_ASRC_MASK: u32 = 0x1;
pub const PCM0_BYP_ASRC_MASK_SFT: u32 = 0x1 << 7;
pub const PCM0_SLAVE_SFT: u32 = 6;
pub const PCM0_SLAVE_MASK: u32 = 0x1;
pub const PCM0_SLAVE_MASK_SFT: u32 = 0x1 << 6;
pub const PCM0_MODE_SFT: u32 = 3;
pub const PCM0_MODE_MASK: u32 = 0x7;
pub const PCM0_MODE_MASK_SFT: u32 = 0x7 << 3;
pub const PCM0_FMT_SFT: u32 = 1;
pub const PCM0_FMT_MASK: u32 = 0x3;
pub const PCM0_FMT_MASK_SFT: u32 = 0x3 << 1;
pub const PCM0_EN_SFT: u32 = 0;
pub const PCM0_EN_MASK: u32 = 0x1;
pub const PCM0_EN_MASK_SFT: u32 = 0x1 << 0;

/* AFE_PCM0_INTF_CON1 */
pub const PCM0_TX_RX_LOOPBACK_SFT: u32 = 31;
pub const PCM0_TX_RX_LOOPBACK_MASK: u32 = 0x1;
pub const PCM0_TX_RX_LOOPBACK_MASK_SFT: u32 = 0x1 << 31;
pub const PCM0_BUFFER_LOOPBACK_SFT: u32 = 30;
pub const PCM0_BUFFER_LOOPBACK_MASK: u32 = 0x1;
pub const PCM0_BUFFER_LOOPBACK_MASK_SFT: u32 = 0x1 << 30;
pub const PCM0_PARALLEL_LOOPBACK_SFT: u32 = 29;
pub const PCM0_PARALLEL_LOOPBACK_MASK: u32 = 0x1;
pub const PCM0_PARALLEL_LOOPBACK_MASK_SFT: u32 = 0x1 << 29;
pub const PCM0_SERIAL_LOOPBACK_SFT: u32 = 28;
pub const PCM0_SERIAL_LOOPBACK_MASK: u32 = 0x1;
pub const PCM0_SERIAL_LOOPBACK_MASK_SFT: u32 = 0x1 << 28;
pub const PCM0_DAI_LOOPBACK_SFT: u32 = 27;
pub const PCM0_DAI_LOOPBACK_MASK: u32 = 0x1;
pub const PCM0_DAI_LOOPBACK_MASK_SFT: u32 = 0x1 << 27;
pub const PCM0_I2S_LOOPBACK_SFT: u32 = 26;
pub const PCM0_I2S_LOOPBACK_MASK: u32 = 0x1;
pub const PCM0_I2S_LOOPBACK_MASK_SFT: u32 = 0x1 << 26;
pub const PCM0_1X_EN_DOMAIN_SFT: u32 = 23;
pub const PCM0_1X_EN_DOMAIN_MASK: u32 = 0x7;
pub const PCM0_1X_EN_DOMAIN_MASK_SFT: u32 = 0x7 << 23;
pub const PCM0_1X_EN_MODE_SFT: u32 = 18;
pub const PCM0_1X_EN_MODE_MASK: u32 = 0x1f;
pub const PCM0_1X_EN_MODE_MASK_SFT: u32 = 0x1f << 18;
pub const PCM0_TX3_RCH_DBG_MODE_SFT: u32 = 17;
pub const PCM0_TX3_RCH_DBG_MODE_MASK: u32 = 0x1;
pub const PCM0_TX3_RCH_DBG_MODE_MASK_SFT: u32 = 0x1 << 17;
pub const PCM0_PCM1_LOOPBACK_SFT: u32 = 16;
pub const PCM0_PCM1_LOOPBACK_MASK: u32 = 0x1;
pub const PCM0_PCM1_LOOPBACK_MASK_SFT: u32 = 0x1 << 16;
pub const PCM0_LOOPBACK_CH_SEL_SFT: u32 = 12;
pub const PCM0_LOOPBACK_CH_SEL_MASK: u32 = 0x3;
pub const PCM0_LOOPBACK_CH_SEL_MASK_SFT: u32 = 0x3 << 12;
pub const PCM0_BT_MODE_SFT: u32 = 11;
pub const PCM0_BT_MODE_MASK: u32 = 0x1;
pub const PCM0_BT_MODE_MASK_SFT: u32 = 0x1 << 11;
pub const PCM0_EXT_MODEM_SFT: u32 = 10;
pub const PCM0_EXT_MODEM_MASK: u32 = 0x1;
pub const PCM0_EXT_MODEM_MASK_SFT: u32 = 0x1 << 10;
pub const PCM0_USE_MD3_SFT: u32 = 9;
pub const PCM0_USE_MD3_MASK: u32 = 0x1;
pub const PCM0_USE_MD3_MASK_SFT: u32 = 0x1 << 9;
pub const PCM0_FIX_VALUE_SEL_SFT: u32 = 8;
pub const PCM0_FIX_VALUE_SEL_MASK: u32 = 0x1;
pub const PCM0_FIX_VALUE_SEL_MASK_SFT: u32 = 0x1 << 8;
pub const PCM0_TX_FIX_VALUE_SFT: u32 = 0;
pub const PCM0_TX_FIX_VALUE_MASK: u32 = 0xff;
pub const PCM0_TX_FIX_VALUE_MASK_SFT: u32 = 0xff << 0;

/* AFE_PCM_INTF_MON */
pub const PCM0_TX_FIFO_OV_SFT: u32 = 5;
pub const PCM0_TX_FIFO_OV_MASK: u32 = 0x1;
pub const PCM0_TX_FIFO_OV_MASK_SFT: u32 = 0x1 << 5;
pub const PCM0_RX_FIFO_OV_SFT: u32 = 4;
pub const PCM0_RX_FIFO_OV_MASK: u32 = 0x1;
pub const PCM0_RX_FIFO_OV_MASK_SFT: u32 = 0x1 << 4;
pub const PCM1_TX_FIFO_OV_SFT: u32 = 3;
pub const PCM1_TX_FIFO_OV_MASK: u32 = 0x1;
pub const PCM1_TX_FIFO_OV_MASK_SFT: u32 = 0x1 << 3;
pub const PCM1_RX_FIFO_OV_SFT: u32 = 2;
pub const PCM1_RX_FIFO_OV_MASK: u32 = 0x1;
pub const PCM1_RX_FIFO_OV_MASK_SFT: u32 = 0x1 << 2;
pub const PCM0_SYNC_GLITCH_SFT: u32 = 1;
pub const PCM0_SYNC_GLITCH_MASK: u32 = 0x1;
pub const PCM0_SYNC_GLITCH_MASK_SFT: u32 = 0x1 << 1;
pub const PCM1_SYNC_GLITCH_SFT: u32 = 0;
pub const PCM1_SYNC_GLITCH_MASK: u32 = 0x1;
pub const PCM1_SYNC_GLITCH_MASK_SFT: u32 = 0x1 << 0;

/* AFE_PCM_TOP_IP_VERSION */
pub const AFE_PCM_TOP_IP_VERSION_SFT: u32 = 0;
pub const AFE_PCM_TOP_IP_VERSION_MASK: u32 = 0xffffffff;
pub const AFE_PCM_TOP_IP_VERSION_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_IRQ_MCU_EN */
pub const AFE_IRQ_MCU_EN_SFT: u32 = 0;
pub const AFE_IRQ_MCU_EN_MASK: u32 = 0xffffffff;
pub const AFE_IRQ_MCU_EN_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_IRQ_MCU_DSP_EN */
pub const AFE_IRQ_DSP_EN_SFT: u32 = 0;
pub const AFE_IRQ_DSP_EN_MASK: u32 = 0xffffffff;
pub const AFE_IRQ_DSP_EN_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_IRQ_MCU_DSP2_EN */
pub const AFE_IRQ_DSP2_EN_SFT: u32 = 0;
pub const AFE_IRQ_DSP2_EN_MASK: u32 = 0xffffffff;
pub const AFE_IRQ_DSP2_EN_MASK_SFT: u32 = 0xffffffff << 0;

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
pub const AFE_CUSTOM_IRQ_MCU_EN_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_CUSTOM_IRQ_MCU_DSP_EN */
pub const AFE_CUSTOM_IRQ_DSP_EN_SFT: u32 = 0;
pub const AFE_CUSTOM_IRQ_DSP_EN_MASK: u32 = 0xffffffff;
pub const AFE_CUSTOM_IRQ_DSP_EN_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_CUSTOM_IRQ_MCU_DSP2_EN */
pub const AFE_CUSTOM_IRQ_DSP2_EN_SFT: u32 = 0;
pub const AFE_CUSTOM_IRQ_DSP2_EN_MASK: u32 = 0xffffffff;
pub const AFE_CUSTOM_IRQ_DSP2_EN_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_CUSTOM_IRQ_MCU_SCP_EN */
pub const AFE_CUSTOM_IRQ_SCP_EN_SFT: u32 = 0;
pub const AFE_CUSTOM_IRQ_SCP_EN_MASK: u32 = 0xffffffff;
pub const AFE_CUSTOM_IRQ_SCP_EN_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_IRQ_MCU_STATUS */
pub const IRQ26_MCU_SFT: u32 = 26;
pub const IRQ26_MCU_MASK: u32 = 0x1;
pub const IRQ26_MCU_MASK_SFT: u32 = 0x1 << 26;
pub const IRQ25_MCU_SFT: u32 = 25;
pub const IRQ25_MCU_MASK: u32 = 0x1;
pub const IRQ25_MCU_MASK_SFT: u32 = 0x1 << 25;
pub const IRQ24_MCU_SFT: u32 = 24;
pub const IRQ24_MCU_MASK: u32 = 0x1;
pub const IRQ24_MCU_MASK_SFT: u32 = 0x1 << 24;
pub const IRQ23_MCU_SFT: u32 = 23;
pub const IRQ23_MCU_MASK: u32 = 0x1;
pub const IRQ23_MCU_MASK_SFT: u32 = 0x1 << 23;
pub const IRQ22_MCU_SFT: u32 = 22;
pub const IRQ22_MCU_MASK: u32 = 0x1;
pub const IRQ22_MCU_MASK_SFT: u32 = 0x1 << 22;
pub const IRQ21_MCU_SFT: u32 = 21;
pub const IRQ21_MCU_MASK: u32 = 0x1;
pub const IRQ21_MCU_MASK_SFT: u32 = 0x1 << 21;
pub const IRQ20_MCU_SFT: u32 = 20;
pub const IRQ20_MCU_MASK: u32 = 0x1;
pub const IRQ20_MCU_MASK_SFT: u32 = 0x1 << 20;
pub const IRQ19_MCU_SFT: u32 = 19;
pub const IRQ19_MCU_MASK: u32 = 0x1;
pub const IRQ19_MCU_MASK_SFT: u32 = 0x1 << 19;
pub const IRQ18_MCU_SFT: u32 = 18;
pub const IRQ18_MCU_MASK: u32 = 0x1;
pub const IRQ18_MCU_MASK_SFT: u32 = 0x1 << 18;
pub const IRQ17_MCU_SFT: u32 = 17;
pub const IRQ17_MCU_MASK: u32 = 0x1;
pub const IRQ17_MCU_MASK_SFT: u32 = 0x1 << 17;
pub const IRQ16_MCU_SFT: u32 = 16;
pub const IRQ16_MCU_MASK: u32 = 0x1;
pub const IRQ16_MCU_MASK_SFT: u32 = 0x1 << 16;
pub const IRQ15_MCU_SFT: u32 = 15;
pub const IRQ15_MCU_MASK: u32 = 0x1;
pub const IRQ15_MCU_MASK_SFT: u32 = 0x1 << 15;
pub const IRQ14_MCU_SFT: u32 = 14;
pub const IRQ14_MCU_MASK: u32 = 0x1;
pub const IRQ14_MCU_MASK_SFT: u32 = 0x1 << 14;
pub const IRQ13_MCU_SFT: u32 = 13;
pub const IRQ13_MCU_MASK: u32 = 0x1;
pub const IRQ13_MCU_MASK_SFT: u32 = 0x1 << 13;
pub const IRQ12_MCU_SFT: u32 = 12;
pub const IRQ12_MCU_MASK: u32 = 0x1;
pub const IRQ12_MCU_MASK_SFT: u32 = 0x1 << 12;
pub const IRQ11_MCU_SFT: u32 = 11;
pub const IRQ11_MCU_MASK: u32 = 0x1;
pub const IRQ11_MCU_MASK_SFT: u32 = 0x1 << 11;
pub const IRQ10_MCU_SFT: u32 = 10;
pub const IRQ10_MCU_MASK: u32 = 0x1;
pub const IRQ10_MCU_MASK_SFT: u32 = 0x1 << 10;
pub const IRQ9_MCU_SFT: u32 = 9;
pub const IRQ9_MCU_MASK: u32 = 0x1;
pub const IRQ9_MCU_MASK_SFT: u32 = 0x1 << 9;
pub const IRQ8_MCU_SFT: u32 = 8;
pub const IRQ8_MCU_MASK: u32 = 0x1;
pub const IRQ8_MCU_MASK_SFT: u32 = 0x1 << 8;
pub const IRQ7_MCU_SFT: u32 = 7;
pub const IRQ7_MCU_MASK: u32 = 0x1;
pub const IRQ7_MCU_MASK_SFT: u32 = 0x1 << 7;
pub const IRQ6_MCU_SFT: u32 = 6;
pub const IRQ6_MCU_MASK: u32 = 0x1;
pub const IRQ6_MCU_MASK_SFT: u32 = 0x1 << 6;
pub const IRQ5_MCU_SFT: u32 = 5;
pub const IRQ5_MCU_MASK: u32 = 0x1;
pub const IRQ5_MCU_MASK_SFT: u32 = 0x1 << 5;
pub const IRQ4_MCU_SFT: u32 = 4;
pub const IRQ4_MCU_MASK: u32 = 0x1;
pub const IRQ4_MCU_MASK_SFT: u32 = 0x1 << 4;
pub const IRQ3_MCU_SFT: u32 = 3;
pub const IRQ3_MCU_MASK: u32 = 0x1;
pub const IRQ3_MCU_MASK_SFT: u32 = 0x1 << 3;
pub const IRQ2_MCU_SFT: u32 = 2;
pub const IRQ2_MCU_MASK: u32 = 0x1;
pub const IRQ2_MCU_MASK_SFT: u32 = 0x1 << 2;
pub const IRQ1_MCU_SFT: u32 = 1;
pub const IRQ1_MCU_MASK: u32 = 0x1;
pub const IRQ1_MCU_MASK_SFT: u32 = 0x1 << 1;
pub const IRQ0_MCU_SFT: u32 = 0;
pub const IRQ0_MCU_MASK: u32 = 0x1;
pub const IRQ0_MCU_MASK_SFT: u32 = 0x1 << 0;

/* AFE_CUSTOM_IRQ_MCU_STATUS */
pub const CUSTOM_IRQ21_MCU_SFT: u32 = 21;
pub const CUSTOM_IRQ21_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ21_MCU_MASK_SFT: u32 = 0x1 << 21;
pub const CUSTOM_IRQ20_MCU_SFT: u32 = 20;
pub const CUSTOM_IRQ20_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ20_MCU_MASK_SFT: u32 = 0x1 << 20;
pub const CUSTOM_IRQ19_MCU_SFT: u32 = 19;
pub const CUSTOM_IRQ19_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ19_MCU_MASK_SFT: u32 = 0x1 << 19;
pub const CUSTOM_IRQ18_MCU_SFT: u32 = 18;
pub const CUSTOM_IRQ18_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ18_MCU_MASK_SFT: u32 = 0x1 << 18;
pub const CUSTOM_IRQ17_MCU_SFT: u32 = 17;
pub const CUSTOM_IRQ17_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ17_MCU_MASK_SFT: u32 = 0x1 << 17;
pub const CUSTOM_IRQ16_MCU_SFT: u32 = 16;
pub const CUSTOM_IRQ16_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ16_MCU_MASK_SFT: u32 = 0x1 << 16;
pub const CUSTOM_IRQ9_MCU_SFT: u32 = 9;
pub const CUSTOM_IRQ9_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ9_MCU_MASK_SFT: u32 = 0x1 << 9;
pub const CUSTOM_IRQ8_MCU_SFT: u32 = 8;
pub const CUSTOM_IRQ8_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ8_MCU_MASK_SFT: u32 = 0x1 << 8;
pub const CUSTOM_IRQ7_MCU_SFT: u32 = 7;
pub const CUSTOM_IRQ7_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ7_MCU_MASK_SFT: u32 = 0x1 << 7;
pub const CUSTOM_IRQ6_MCU_SFT: u32 = 6;
pub const CUSTOM_IRQ6_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ6_MCU_MASK_SFT: u32 = 0x1 << 6;
pub const CUSTOM_IRQ5_MCU_SFT: u32 = 5;
pub const CUSTOM_IRQ5_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ5_MCU_MASK_SFT: u32 = 0x1 << 5;
pub const CUSTOM_IRQ4_MCU_SFT: u32 = 4;
pub const CUSTOM_IRQ4_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ4_MCU_MASK_SFT: u32 = 0x1 << 4;
pub const CUSTOM_IRQ3_MCU_SFT: u32 = 3;
pub const CUSTOM_IRQ3_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ3_MCU_MASK_SFT: u32 = 0x1 << 3;
pub const CUSTOM_IRQ2_MCU_SFT: u32 = 2;
pub const CUSTOM_IRQ2_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ2_MCU_MASK_SFT: u32 = 0x1 << 2;
pub const CUSTOM_IRQ1_MCU_SFT: u32 = 1;
pub const CUSTOM_IRQ1_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ1_MCU_MASK_SFT: u32 = 0x1 << 1;
pub const CUSTOM_IRQ0_MCU_SFT: u32 = 0;
pub const CUSTOM_IRQ0_MCU_MASK: u32 = 0x1;
pub const CUSTOM_IRQ0_MCU_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ_MCU_CFG */
pub const AFE_IRQ_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ0_MCU_CFG0 */
pub const AFE_IRQ0_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ0_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ0_MCU_DOMAIN_MASK_SFT: u32 = 0x7 << 9;
pub const AFE_IRQ0_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ0_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ0_MCU_FS_MASK_SFT: u32 = 0x1f << 4;
pub const AFE_IRQ0_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ0_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ0_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ0_MCU_CFG1 */
pub const AFE_IRQ0_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ0_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ0_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ0_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ0_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ0_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ0_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ0_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ0_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ1_MCU_CFG0 */
pub const AFE_IRQ1_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ1_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ1_MCU_DOMAIN_MASK_SFT: u32 = 0x7 << 9;
pub const AFE_IRQ1_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ1_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ1_MCU_FS_MASK_SFT: u32 = 0x1f << 4;
pub const AFE_IRQ1_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ1_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ1_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ1_MCU_CFG1 */
pub const AFE_IRQ1_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ1_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ1_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ1_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ1_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ1_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ1_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ1_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ1_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ2_MCU_CFG0 */
pub const AFE_IRQ2_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ2_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ2_MCU_DOMAIN_MASK_SFT: u32 = 0x7 << 9;
pub const AFE_IRQ2_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ2_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ2_MCU_FS_MASK_SFT: u32 = 0x1f << 4;
pub const AFE_IRQ2_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ2_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ2_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ2_MCU_CFG1 */
pub const AFE_IRQ2_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ2_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ2_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ2_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ2_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ2_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ2_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ2_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ2_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ3_MCU_CFG0 */
pub const AFE_IRQ3_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ3_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ3_MCU_DOMAIN_MASK_SFT: u32 = 0x7 << 9;
pub const AFE_IRQ3_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ3_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ3_MCU_FS_MASK_SFT: u32 = 0x1f << 4;
pub const AFE_IRQ3_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ3_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ3_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ3_MCU_CFG1 */
pub const AFE_IRQ3_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ3_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ3_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ3_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ3_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ3_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ3_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ3_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ3_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ4_MCU_CFG0 */
pub const AFE_IRQ4_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ4_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ4_MCU_DOMAIN_MASK_SFT: u32 = 0x7 << 9;
pub const AFE_IRQ4_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ4_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ4_MCU_FS_MASK_SFT: u32 = 0x1f << 4;
pub const AFE_IRQ4_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ4_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ4_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ4_MCU_CFG1 */
pub const AFE_IRQ4_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ4_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ4_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ4_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ4_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ4_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ4_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ4_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ4_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ5_MCU_CFG0 */
pub const AFE_IRQ5_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ5_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ5_MCU_DOMAIN_MASK_SFT: u32 = 0x7 << 9;
pub const AFE_IRQ5_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ5_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ5_MCU_FS_MASK_SFT: u32 = 0x1f << 4;
pub const AFE_IRQ5_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ5_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ5_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ5_MCU_CFG1 */
pub const AFE_IRQ5_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ5_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ5_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ5_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ5_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ5_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ5_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ5_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ5_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ6_MCU_CFG0 */
pub const AFE_IRQ6_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ6_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ6_MCU_DOMAIN_MASK_SFT: u32 = 0x7 << 9;
pub const AFE_IRQ6_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ6_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ6_MCU_FS_MASK_SFT: u32 = 0x1f << 4;
pub const AFE_IRQ6_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ6_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ6_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ6_MCU_CFG1 */
pub const AFE_IRQ6_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ6_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ6_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ6_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ6_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ6_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ6_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ6_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ6_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ7_MCU_CFG0 */
pub const AFE_IRQ7_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ7_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ7_MCU_DOMAIN_MASK_SFT: u32 = 0x7 << 9;
pub const AFE_IRQ7_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ7_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ7_MCU_FS_MASK_SFT: u32 = 0x1f << 4;
pub const AFE_IRQ7_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ7_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ7_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ7_MCU_CFG1 */
pub const AFE_IRQ7_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ7_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ7_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ7_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ7_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ7_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ7_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ7_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ7_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ8_MCU_CFG0 */
pub const AFE_IRQ8_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ8_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ8_MCU_DOMAIN_MASK_SFT: u32 = 0x7 << 9;
pub const AFE_IRQ8_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ8_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ8_MCU_FS_MASK_SFT: u32 = 0x1f << 4;
pub const AFE_IRQ8_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ8_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ8_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ8_MCU_CFG1 */
pub const AFE_IRQ8_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ8_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ8_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ8_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ8_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ8_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ8_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ8_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ8_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ9_MCU_CFG0 */
pub const AFE_IRQ9_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ9_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ9_MCU_DOMAIN_MASK_SFT: u32 = 0x7 << 9;
pub const AFE_IRQ9_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ9_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ9_MCU_FS_MASK_SFT: u32 = 0x1f << 4;
pub const AFE_IRQ9_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ9_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ9_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ9_MCU_CFG1 */
pub const AFE_IRQ9_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ9_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ9_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ9_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ9_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ9_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ9_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ9_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ9_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ10_MCU_CFG0 */
pub const AFE_IRQ10_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ10_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ10_MCU_DOMAIN_MASK_SFT: u32 = 0x7 << 9;
pub const AFE_IRQ10_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ10_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ10_MCU_FS_MASK_SFT: u32 = 0x1f << 4;
pub const AFE_IRQ10_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ10_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ10_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ10_MCU_CFG1 */
pub const AFE_IRQ10_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ10_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ10_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ10_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ10_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ10_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ10_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ10_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ10_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ11_MCU_CFG0 */
pub const AFE_IRQ11_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ11_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ11_MCU_DOMAIN_MASK_SFT: u32 = 0x7 << 9;
pub const AFE_IRQ11_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ11_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ11_MCU_FS_MASK_SFT: u32 = 0x1f << 4;
pub const AFE_IRQ11_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ11_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ11_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ11_MCU_CFG1 */
pub const AFE_IRQ11_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ11_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ11_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ11_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ11_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ11_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ11_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ11_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ11_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ12_MCU_CFG0 */
pub const AFE_IRQ12_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ12_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ12_MCU_DOMAIN_MASK_SFT: u32 = 0x7 << 9;
pub const AFE_IRQ12_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ12_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ12_MCU_FS_MASK_SFT: u32 = 0x1f << 4;
pub const AFE_IRQ12_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ12_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ12_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ12_MCU_CFG1 */
pub const AFE_IRQ12_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ12_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ12_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ12_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ12_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ12_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ12_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ12_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ12_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ13_MCU_CFG0 */
pub const AFE_IRQ13_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ13_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ13_MCU_DOMAIN_MASK_SFT: u32 = 0x7 << 9;
pub const AFE_IRQ13_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ13_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ13_MCU_FS_MASK_SFT: u32 = 0x1f << 4;
pub const AFE_IRQ13_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ13_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ13_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ13_MCU_CFG1 */
pub const AFE_IRQ13_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ13_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ13_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ13_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ13_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ13_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ13_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ13_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ13_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ14_MCU_CFG0 */
pub const AFE_IRQ14_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ14_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ14_MCU_DOMAIN_MASK_SFT: u32 = 0x7 << 9;
pub const AFE_IRQ14_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ14_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ14_MCU_FS_MASK_SFT: u32 = 0x1f << 4;
pub const AFE_IRQ14_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ14_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ14_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ14_MCU_CFG1 */
pub const AFE_IRQ14_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ14_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ14_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ14_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ14_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ14_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ14_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ14_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ14_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ15_MCU_CFG0 */
pub const AFE_IRQ15_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ15_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ15_MCU_DOMAIN_MASK_SFT: u32 = 0x7 << 9;
pub const AFE_IRQ15_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ15_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ15_MCU_FS_MASK_SFT: u32 = 0x1f << 4;
pub const AFE_IRQ15_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ15_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ15_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ15_MCU_CFG1 */
pub const AFE_IRQ15_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ15_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ15_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ15_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ15_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ15_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ15_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ15_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ15_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ16_MCU_CFG0 */
pub const AFE_IRQ16_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ16_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ16_MCU_DOMAIN_MASK_SFT: u32 = 0x7 << 9;
pub const AFE_IRQ16_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ16_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ16_MCU_FS_MASK_SFT: u32 = 0x1f << 4;
pub const AFE_IRQ16_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ16_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ16_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ16_MCU_CFG1 */
pub const AFE_IRQ16_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ16_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ16_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ16_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ16_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ16_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ16_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ16_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ16_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ17_MCU_CFG0 */
pub const AFE_IRQ17_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ17_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ17_MCU_DOMAIN_MASK_SFT: u32 = 0x7 << 9;
pub const AFE_IRQ17_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ17_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ17_MCU_FS_MASK_SFT: u32 = 0x1f << 4;
pub const AFE_IRQ17_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ17_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ17_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ17_MCU_CFG1 */
pub const AFE_IRQ17_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ17_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ17_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ17_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ17_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ17_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ17_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ17_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ17_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ18_MCU_CFG0 */
pub const AFE_IRQ18_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ18_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ18_MCU_DOMAIN_MASK_SFT: u32 = 0x7 << 9;
pub const AFE_IRQ18_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ18_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ18_MCU_FS_MASK_SFT: u32 = 0x1f << 4;
pub const AFE_IRQ18_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ18_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ18_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ18_MCU_CFG1 */
pub const AFE_IRQ18_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ18_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ18_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ18_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ18_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ18_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ18_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ18_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ18_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ19_MCU_CFG0 */
pub const AFE_IRQ19_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ19_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ19_MCU_DOMAIN_MASK_SFT: u32 = 0x7 << 9;
pub const AFE_IRQ19_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ19_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ19_MCU_FS_MASK_SFT: u32 = 0x1f << 4;
pub const AFE_IRQ19_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ19_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ19_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ19_MCU_CFG1 */
pub const AFE_IRQ19_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ19_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ19_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ19_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ19_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ19_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ19_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ19_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ19_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ20_MCU_CFG0 */
pub const AFE_IRQ20_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ20_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ20_MCU_DOMAIN_MASK_SFT: u32 = 0x7 << 9;
pub const AFE_IRQ20_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ20_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ20_MCU_FS_MASK_SFT: u32 = 0x1f << 4;
pub const AFE_IRQ20_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ20_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ20_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ20_MCU_CFG1 */
pub const AFE_IRQ20_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ20_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ20_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ20_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ20_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ20_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ20_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ20_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ20_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ21_MCU_CFG0 */
pub const AFE_IRQ21_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ21_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ21_MCU_DOMAIN_MASK_SFT: u32 = 0x7 << 9;
pub const AFE_IRQ21_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ21_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ21_MCU_FS_MASK_SFT: u32 = 0x1f << 4;
pub const AFE_IRQ21_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ21_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ21_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ21_MCU_CFG1 */
pub const AFE_IRQ21_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ21_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ21_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ21_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ21_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ21_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ21_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ21_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ21_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ22_MCU_CFG0 */
pub const AFE_IRQ22_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ22_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ22_MCU_DOMAIN_MASK_SFT: u32 = 0x7 << 9;
pub const AFE_IRQ22_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ22_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ22_MCU_FS_MASK_SFT: u32 = 0x1f << 4;
pub const AFE_IRQ22_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ22_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ22_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ22_MCU_CFG1 */
pub const AFE_IRQ22_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ22_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ22_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ22_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ22_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ22_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ22_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ22_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ22_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ23_MCU_CFG0 */
pub const AFE_IRQ23_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ23_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ23_MCU_DOMAIN_MASK_SFT: u32 = 0x7 << 9;
pub const AFE_IRQ23_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ23_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ23_MCU_FS_MASK_SFT: u32 = 0x1f << 4;
pub const AFE_IRQ23_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ23_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ23_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ23_MCU_CFG1 */
pub const AFE_IRQ23_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ23_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ23_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ23_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ23_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ23_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ23_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ23_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ23_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ24_MCU_CFG0 */
pub const AFE_IRQ24_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ24_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ24_MCU_DOMAIN_MASK_SFT: u32 = 0x7 << 9;
pub const AFE_IRQ24_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ24_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ24_MCU_FS_MASK_SFT: u32 = 0x1f << 4;
pub const AFE_IRQ24_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ24_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ24_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ24_MCU_CFG1 */
pub const AFE_IRQ24_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ24_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ24_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ24_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ24_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ24_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ24_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ24_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ24_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ25_MCU_CFG0 */
pub const AFE_IRQ25_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ25_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ25_MCU_DOMAIN_MASK_SFT: u32 = 0x7 << 9;
pub const AFE_IRQ25_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ25_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ25_MCU_FS_MASK_SFT: u32 = 0x1f << 4;
pub const AFE_IRQ25_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ25_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ25_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ25_MCU_CFG1 */
pub const AFE_IRQ25_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ25_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ25_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ25_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ25_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ25_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ25_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ25_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ25_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ26_MCU_CFG0 */
pub const AFE_IRQ26_MCU_DOMAIN_SFT: u32 = 9;
pub const AFE_IRQ26_MCU_DOMAIN_MASK: u32 = 0x7;
pub const AFE_IRQ26_MCU_DOMAIN_MASK_SFT: u32 = 0x7 << 9;
pub const AFE_IRQ26_MCU_FS_SFT: u32 = 4;
pub const AFE_IRQ26_MCU_FS_MASK: u32 = 0x1f;
pub const AFE_IRQ26_MCU_FS_MASK_SFT: u32 = 0x1f << 4;
pub const AFE_IRQ26_MCU_ON_SFT: u32 = 0;
pub const AFE_IRQ26_MCU_ON_MASK: u32 = 0x1;
pub const AFE_IRQ26_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ26_MCU_CFG1 */
pub const AFE_IRQ26_CLR_CFG_SFT: u32 = 31;
pub const AFE_IRQ26_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ26_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_IRQ26_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_IRQ26_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_IRQ26_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_IRQ26_MCU_CNT_SFT: u32 = 0;
pub const AFE_IRQ26_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_IRQ26_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_CUSTOM_IRQ0_MCU_CFG0 */
pub const AFE_CUSTOM_IRQ0_MCU_ON_SFT: u32 = 0;
pub const AFE_CUSTOM_IRQ0_MCU_ON_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ0_MCU_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_CUSTOM_IRQ0_CNT_MON */
pub const AFE_CUSTOM_IRQ0_CNT_MON_SFT: u32 = 0;
pub const AFE_CUSTOM_IRQ0_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_CUSTOM_IRQ0_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_CUSTOM_IRQ0_MCU_CFG1 */
pub const AFE_CUSTOM_IRQ0_CLR_CFG_SFT: u32 = 31;
pub const AFE_CUSTOM_IRQ0_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ0_CLR_CFG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_CUSTOM_IRQ0_MISS_FLAG_CLR_CFG_SFT: u32 = 30;
pub const AFE_CUSTOM_IRQ0_MISS_FLAG_CLR_CFG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ0_MISS_FLAG_CLR_CFG_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_CUSTOM_IRQ0_MCU_CNT_SFT: u32 = 0;
pub const AFE_CUSTOM_IRQ0_MCU_CNT_MASK: u32 = 0xffffff;
pub const AFE_CUSTOM_IRQ0_MCU_CNT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ_MCU_MON0 */
pub const AFE_IRQ26_MISS_FLAG_SFT: u32 = 26;
pub const AFE_IRQ26_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ26_MISS_FLAG_MASK_SFT: u32 = 0x1 << 26;
pub const AFE_IRQ25_MISS_FLAG_SFT: u32 = 25;
pub const AFE_IRQ25_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ25_MISS_FLAG_MASK_SFT: u32 = 0x1 << 25;
pub const AFE_IRQ24_MISS_FLAG_SFT: u32 = 24;
pub const AFE_IRQ24_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ24_MISS_FLAG_MASK_SFT: u32 = 0x1 << 24;
pub const AFE_IRQ23_MISS_FLAG_SFT: u32 = 23;
pub const AFE_IRQ23_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ23_MISS_FLAG_MASK_SFT: u32 = 0x1 << 23;
pub const AFE_IRQ22_MISS_FLAG_SFT: u32 = 22;
pub const AFE_IRQ22_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ22_MISS_FLAG_MASK_SFT: u32 = 0x1 << 22;
pub const AFE_IRQ21_MISS_FLAG_SFT: u32 = 21;
pub const AFE_IRQ21_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ21_MISS_FLAG_MASK_SFT: u32 = 0x1 << 21;
pub const AFE_IRQ20_MISS_FLAG_SFT: u32 = 20;
pub const AFE_IRQ20_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ20_MISS_FLAG_MASK_SFT: u32 = 0x1 << 20;
pub const AFE_IRQ19_MISS_FLAG_SFT: u32 = 19;
pub const AFE_IRQ19_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ19_MISS_FLAG_MASK_SFT: u32 = 0x1 << 19;
pub const AFE_IRQ18_MISS_FLAG_SFT: u32 = 18;
pub const AFE_IRQ18_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ18_MISS_FLAG_MASK_SFT: u32 = 0x1 << 18;
pub const AFE_IRQ17_MISS_FLAG_SFT: u32 = 17;
pub const AFE_IRQ17_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ17_MISS_FLAG_MASK_SFT: u32 = 0x1 << 17;
pub const AFE_IRQ16_MISS_FLAG_SFT: u32 = 16;
pub const AFE_IRQ16_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ16_MISS_FLAG_MASK_SFT: u32 = 0x1 << 16;
pub const AFE_IRQ15_MISS_FLAG_SFT: u32 = 15;
pub const AFE_IRQ15_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ15_MISS_FLAG_MASK_SFT: u32 = 0x1 << 15;
pub const AFE_IRQ14_MISS_FLAG_SFT: u32 = 14;
pub const AFE_IRQ14_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ14_MISS_FLAG_MASK_SFT: u32 = 0x1 << 14;
pub const AFE_IRQ13_MISS_FLAG_SFT: u32 = 13;
pub const AFE_IRQ13_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ13_MISS_FLAG_MASK_SFT: u32 = 0x1 << 13;
pub const AFE_IRQ12_MISS_FLAG_SFT: u32 = 12;
pub const AFE_IRQ12_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ12_MISS_FLAG_MASK_SFT: u32 = 0x1 << 12;
pub const AFE_IRQ11_MISS_FLAG_SFT: u32 = 11;
pub const AFE_IRQ11_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ11_MISS_FLAG_MASK_SFT: u32 = 0x1 << 11;
pub const AFE_IRQ10_MISS_FLAG_SFT: u32 = 10;
pub const AFE_IRQ10_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ10_MISS_FLAG_MASK_SFT: u32 = 0x1 << 10;
pub const AFE_IRQ9_MISS_FLAG_SFT: u32 = 9;
pub const AFE_IRQ9_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ9_MISS_FLAG_MASK_SFT: u32 = 0x1 << 9;
pub const AFE_IRQ8_MISS_FLAG_SFT: u32 = 8;
pub const AFE_IRQ8_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ8_MISS_FLAG_MASK_SFT: u32 = 0x1 << 8;
pub const AFE_IRQ7_MISS_FLAG_SFT: u32 = 7;
pub const AFE_IRQ7_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ7_MISS_FLAG_MASK_SFT: u32 = 0x1 << 7;
pub const AFE_IRQ6_MISS_FLAG_SFT: u32 = 6;
pub const AFE_IRQ6_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ6_MISS_FLAG_MASK_SFT: u32 = 0x1 << 6;
pub const AFE_IRQ5_MISS_FLAG_SFT: u32 = 5;
pub const AFE_IRQ5_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ5_MISS_FLAG_MASK_SFT: u32 = 0x1 << 5;
pub const AFE_IRQ4_MISS_FLAG_SFT: u32 = 4;
pub const AFE_IRQ4_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ4_MISS_FLAG_MASK_SFT: u32 = 0x1 << 4;
pub const AFE_IRQ3_MISS_FLAG_SFT: u32 = 3;
pub const AFE_IRQ3_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ3_MISS_FLAG_MASK_SFT: u32 = 0x1 << 3;
pub const AFE_IRQ2_MISS_FLAG_SFT: u32 = 2;
pub const AFE_IRQ2_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ2_MISS_FLAG_MASK_SFT: u32 = 0x1 << 2;
pub const AFE_IRQ1_MISS_FLAG_SFT: u32 = 1;
pub const AFE_IRQ1_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ1_MISS_FLAG_MASK_SFT: u32 = 0x1 << 1;
pub const AFE_IRQ0_MISS_FLAG_SFT: u32 = 0;
pub const AFE_IRQ0_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_IRQ0_MISS_FLAG_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ_MCU_MON1 */
pub const AFE_CUSTOM_IRQ21_MISS_FLAG_SFT: u32 = 21;
pub const AFE_CUSTOM_IRQ21_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ21_MISS_FLAG_MASK_SFT: u32 = 0x1 << 21;
pub const AFE_CUSTOM_IRQ20_MISS_FLAG_SFT: u32 = 20;
pub const AFE_CUSTOM_IRQ20_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ20_MISS_FLAG_MASK_SFT: u32 = 0x1 << 20;
pub const AFE_CUSTOM_IRQ19_MISS_FLAG_SFT: u32 = 19;
pub const AFE_CUSTOM_IRQ19_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ19_MISS_FLAG_MASK_SFT: u32 = 0x1 << 19;
pub const AFE_CUSTOM_IRQ18_MISS_FLAG_SFT: u32 = 18;
pub const AFE_CUSTOM_IRQ18_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ18_MISS_FLAG_MASK_SFT: u32 = 0x1 << 18;
pub const AFE_CUSTOM_IRQ17_MISS_FLAG_SFT: u32 = 17;
pub const AFE_CUSTOM_IRQ17_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ17_MISS_FLAG_MASK_SFT: u32 = 0x1 << 17;
pub const AFE_CUSTOM_IRQ16_MISS_FLAG_SFT: u32 = 16;
pub const AFE_CUSTOM_IRQ16_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ16_MISS_FLAG_MASK_SFT: u32 = 0x1 << 16;
pub const AFE_CUSTOM_IRQ9_MISS_FLAG_SFT: u32 = 9;
pub const AFE_CUSTOM_IRQ9_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ9_MISS_FLAG_MASK_SFT: u32 = 0x1 << 9;
pub const AFE_CUSTOM_IRQ8_MISS_FLAG_SFT: u32 = 8;
pub const AFE_CUSTOM_IRQ8_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ8_MISS_FLAG_MASK_SFT: u32 = 0x1 << 8;
pub const AFE_CUSTOM_IRQ7_MISS_FLAG_SFT: u32 = 7;
pub const AFE_CUSTOM_IRQ7_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ7_MISS_FLAG_MASK_SFT: u32 = 0x1 << 7;
pub const AFE_CUSTOM_IRQ6_MISS_FLAG_SFT: u32 = 6;
pub const AFE_CUSTOM_IRQ6_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ6_MISS_FLAG_MASK_SFT: u32 = 0x1 << 6;
pub const AFE_CUSTOM_IRQ5_MISS_FLAG_SFT: u32 = 5;
pub const AFE_CUSTOM_IRQ5_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ5_MISS_FLAG_MASK_SFT: u32 = 0x1 << 5;
pub const AFE_CUSTOM_IRQ4_MISS_FLAG_SFT: u32 = 4;
pub const AFE_CUSTOM_IRQ4_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ4_MISS_FLAG_MASK_SFT: u32 = 0x1 << 4;
pub const AFE_CUSTOM_IRQ3_MISS_FLAG_SFT: u32 = 3;
pub const AFE_CUSTOM_IRQ3_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ3_MISS_FLAG_MASK_SFT: u32 = 0x1 << 3;
pub const AFE_CUSTOM_IRQ2_MISS_FLAG_SFT: u32 = 2;
pub const AFE_CUSTOM_IRQ2_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ2_MISS_FLAG_MASK_SFT: u32 = 0x1 << 2;
pub const AFE_CUSTOM_IRQ1_MISS_FLAG_SFT: u32 = 1;
pub const AFE_CUSTOM_IRQ1_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ1_MISS_FLAG_MASK_SFT: u32 = 0x1 << 1;
pub const AFE_CUSTOM_IRQ0_MISS_FLAG_SFT: u32 = 0;
pub const AFE_CUSTOM_IRQ0_MISS_FLAG_MASK: u32 = 0x1;
pub const AFE_CUSTOM_IRQ0_MISS_FLAG_MASK_SFT: u32 = 0x1 << 0;

/* AFE_IRQ_MCU_MON2 */
pub const AFE_IRQ_B_R_CNT_SFT: u32 = 8;
pub const AFE_IRQ_B_R_CNT_MASK: u32 = 0xff;
pub const AFE_IRQ_B_R_CNT_MASK_SFT: u32 = 0xff << 8;
pub const AFE_IRQ_B_F_CNT_SFT: u32 = 0;
pub const AFE_IRQ_B_F_CNT_MASK: u32 = 0xff;
pub const AFE_IRQ_B_F_CNT_MASK_SFT: u32 = 0xff << 0;

/* AFE_IRQ0_CNT_MON */
pub const AFE_IRQ0_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ0_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ0_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ1_CNT_MON */
pub const AFE_IRQ1_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ1_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ1_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ2_CNT_MON */
pub const AFE_IRQ2_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ2_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ2_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ3_CNT_MON */
pub const AFE_IRQ3_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ3_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ3_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ4_CNT_MON */
pub const AFE_IRQ4_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ4_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ4_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ5_CNT_MON */
pub const AFE_IRQ5_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ5_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ5_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ6_CNT_MON */
pub const AFE_IRQ6_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ6_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ6_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ7_CNT_MON */
pub const AFE_IRQ7_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ7_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ7_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ8_CNT_MON */
pub const AFE_IRQ8_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ8_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ8_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ9_CNT_MON */
pub const AFE_IRQ9_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ9_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ9_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ10_CNT_MON */
pub const AFE_IRQ10_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ10_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ10_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ11_CNT_MON */
pub const AFE_IRQ11_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ11_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ11_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ12_CNT_MON */
pub const AFE_IRQ12_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ12_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ12_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ13_CNT_MON */
pub const AFE_IRQ13_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ13_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ13_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ14_CNT_MON */
pub const AFE_IRQ14_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ14_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ14_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ15_CNT_MON */
pub const AFE_IRQ15_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ15_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ15_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ16_CNT_MON */
pub const AFE_IRQ16_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ16_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ16_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ17_CNT_MON */
pub const AFE_IRQ17_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ17_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ17_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ18_CNT_MON */
pub const AFE_IRQ18_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ18_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ18_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ19_CNT_MON */
pub const AFE_IRQ19_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ19_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ19_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ20_CNT_MON */
pub const AFE_IRQ20_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ20_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ20_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ21_CNT_MON */
pub const AFE_IRQ21_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ21_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ21_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ22_CNT_MON */
pub const AFE_IRQ22_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ22_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ22_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ23_CNT_MON */
pub const AFE_IRQ23_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ23_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ23_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ24_CNT_MON */
pub const AFE_IRQ24_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ24_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ24_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ25_CNT_MON */
pub const AFE_IRQ25_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ25_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ25_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_IRQ26_CNT_MON */
pub const AFE_IRQ26_CNT_MON_SFT: u32 = 0;
pub const AFE_IRQ26_CNT_MON_MASK: u32 = 0xffffff;
pub const AFE_IRQ26_CNT_MON_MASK_SFT: u32 = 0xffffff << 0;

/*  /* AFE_GAIN0_CON0 */ */
/*  /* AFE_GAIN1_CON0 */ */
/*  /* AFE_GAIN2_CON0 */ */
/*  /* AFE_GAIN3_CON0 */ */
pub const GAIN_TARGET_SYNC_ON_SFT: u32 = 24;
pub const GAIN_TARGET_SYNC_ON_MASK: u32 = 0x1;
pub const GAIN_TARGET_SYNC_ON_MASK_SFT: u32 = 0x1 << 24;
pub const GAIN_TIMEOUT_SFT: u32 = 18;
pub const GAIN_TIMEOUT_MASK: u32 = 0x3f;
pub const GAIN_TIMEOUT_MASK_SFT: u32 = 0x3f << 18;
pub const GAIN_TRIG_SFT: u32 = 17;
pub const GAIN_TRIG_MASK: u32 = 0x1;
pub const GAIN_TRIG_MASK_SFT: u32 = 0x1 << 17;
pub const GAIN_ON_SFT: u32 = 16;
pub const GAIN_ON_MASK: u32 = 0x1;
pub const GAIN_ON_MASK_SFT: u32 = 0x1 << 16;
pub const GAIN_SAMPLE_PER_STEP_SFT: u32 = 8;
pub const GAIN_SAMPLE_PER_STEP_MASK: u32 = 0xff;
pub const GAIN_SAMPLE_PER_STEP_MASK_SFT: u32 = 0xff << 8;
pub const GAIN_SEL_DOMAIN_SFT: u32 = 5;
pub const GAIN_SEL_DOMAIN_MASK: u32 = 0x7;
pub const GAIN_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 5;
pub const GAIN_SEL_FS_SFT: u32 = 0;
pub const GAIN_SEL_FS_MASK: u32 = 0x1f;
pub const GAIN_SEL_FS_MASK_SFT: u32 = 0x1f << 0;

/*  /* AFE_GAIN0_CON1_R */ */
/*  /* AFE_GAIN1_CON1_R */ */
/*  /* AFE_GAIN2_CON1_R */ */
/*  /* AFE_GAIN3_CON1_R */ */
pub const GAIN_TARGET_R_SFT: u32 = 0;
pub const GAIN_TARGET_R_MASK: u32 = 0xffffffff;
pub const GAIN_TARGET_R_MASK_SFT: u32 = 0xffffffff << 0;

/*  /* AFE_GAIN0_CON1_L */ */
/*  /* AFE_GAIN1_CON1_L */ */
/*  /* AFE_GAIN2_CON1_L */ */
/*  /* AFE_GAIN3_CON1_L */ */
pub const GAIN_TARGET_L_SFT: u32 = 0;
pub const GAIN_TARGET_L_MASK: u32 = 0xffffffff;
pub const GAIN_TARGET_L_MASK_SFT: u32 = 0xffffffff << 0;

/*  /* AFE_GAIN0_CON2 */ */
/*  /* AFE_GAIN1_CON2 */ */
/*  /* AFE_GAIN2_CON2 */ */
/*  /* AFE_GAIN3_CON2 */ */
pub const GAIN_DOWN_STEP_SFT: u32 = 0;
pub const GAIN_DOWN_STEP_MASK: u32 = 0x3fffff;
pub const GAIN_DOWN_STEP_MASK_SFT: u32 = 0x3fffff << 0;

/*  /* AFE_GAIN0_CON3 */ */
/*  /* AFE_GAIN1_CON3 */ */
/*  /* AFE_GAIN2_CON3 */ */
/*  /* AFE_GAIN3_CON3 */ */
pub const GAIN_UP_STEP_SFT: u32 = 0;
pub const GAIN_UP_STEP_MASK: u32 = 0x3fffff;
pub const GAIN_UP_STEP_MASK_SFT: u32 = 0x3fffff << 0;

/*  /* AFE_GAIN0_CUR_R */ */
/*  /* AFE_GAIN1_CUR_R */ */
/*  /* AFE_GAIN2_CUR_R */ */
/*  /* AFE_GAIN3_CUR_R */ */
pub const AFE_GAIN_CUR_R_SFT: u32 = 0;
pub const AFE_GAIN_CUR_R_MASK: u32 = 0xffffffff;
pub const AFE_GAIN_CUR_R_MASK_SFT: u32 = 0xffffffff << 0;

/*  /* AFE_GAIN0_CUR_L */ */
/*  /* AFE_GAIN1_CUR_L */ */
/*  /* AFE_GAIN2_CUR_L */ */
/*  /* AFE_GAIN3_CUR_L */ */
pub const AFE_GAIN_CUR_L_SFT: u32 = 0;
pub const AFE_GAIN_CUR_L_MASK: u32 = 0xffffffff;
pub const AFE_GAIN_CUR_L_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_IPM_VER_MON */
pub const RG_DL_IPM_VER_MON_SFT: u32 = 0;
pub const RG_DL_IPM_VER_MON_MASK: u32 = 0xffffffff;
pub const RG_DL_IPM_VER_MON_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_SRC_CON0 */
pub const AFE_DL_INPUT_MODE_CTL_SFT: u32 = 24;
pub const AFE_DL_INPUT_MODE_CTL_MASK: u32 = 0x1f;
pub const AFE_DL_INPUT_MODE_CTL_MASK_SFT: u32 = 0x1f << 24;
pub const AFE_DL_CH1_SATURATION_EN_CTL_SFT: u32 = 21;
pub const AFE_DL_CH1_SATURATION_EN_CTL_MASK: u32 = 0x1;
pub const AFE_DL_CH1_SATURATION_EN_CTL_MASK_SFT: u32 = 0x1 << 21;
pub const AFE_DL_CH2_SATURATION_EN_CTL_SFT: u32 = 20;
pub const AFE_DL_CH2_SATURATION_EN_CTL_MASK: u32 = 0x1;
pub const AFE_DL_CH2_SATURATION_EN_CTL_MASK_SFT: u32 = 0x1 << 20;
pub const AFE_DL_OUTPUT_SEL_CTL_SFT: u32 = 18;
pub const AFE_DL_OUTPUT_SEL_CTL_MASK: u32 = 0x3;
pub const AFE_DL_OUTPUT_SEL_CTL_MASK_SFT: u32 = 0x3 << 18;
pub const AFE_DL_FADEIN_0START_EN_SFT: u32 = 16;
pub const AFE_DL_FADEIN_0START_EN_MASK: u32 = 0x3;
pub const AFE_DL_FADEIN_0START_EN_MASK_SFT: u32 = 0x3 << 16;
pub const AFE_DL_DISABLE_HW_CG_CTL_SFT: u32 = 15;
pub const AFE_DL_DISABLE_HW_CG_CTL_MASK: u32 = 0x1;
pub const AFE_DL_DISABLE_HW_CG_CTL_MASK_SFT: u32 = 0x1 << 15;
pub const AFE_DL_MUTE_CH1_OFF_CTL_PRE_SFT: u32 = 12;
pub const AFE_DL_MUTE_CH1_OFF_CTL_PRE_MASK: u32 = 0x1;
pub const AFE_DL_MUTE_CH1_OFF_CTL_PRE_MASK_SFT: u32 = 0x1 << 12;
pub const AFE_DL_MUTE_CH2_OFF_CTL_PRE_SFT: u32 = 11;
pub const AFE_DL_MUTE_CH2_OFF_CTL_PRE_MASK: u32 = 0x1;
pub const AFE_DL_MUTE_CH2_OFF_CTL_PRE_MASK_SFT: u32 = 0x1 << 11;
pub const AFE_DL_ARAMPSP_CTL_PRE_SFT: u32 = 9;
pub const AFE_DL_ARAMPSP_CTL_PRE_MASK: u32 = 0x3;
pub const AFE_DL_ARAMPSP_CTL_PRE_MASK_SFT: u32 = 0x3 << 9;
pub const AFE_DL_VOICE_MODE_CTL_PRE_SFT: u32 = 5;
pub const AFE_DL_VOICE_MODE_CTL_PRE_MASK: u32 = 0x1;
pub const AFE_DL_VOICE_MODE_CTL_PRE_MASK_SFT: u32 = 0x1 << 5;
pub const AFE_DL_MUTE_CH1_ON_CTL_PRE_SFT: u32 = 4;
pub const AFE_DL_MUTE_CH1_ON_CTL_PRE_MASK: u32 = 0x1;
pub const AFE_DL_MUTE_CH1_ON_CTL_PRE_MASK_SFT: u32 = 0x1 << 4;
pub const AFE_DL_MUTE_CH2_ON_CTL_PRE_SFT: u32 = 3;
pub const AFE_DL_MUTE_CH2_ON_CTL_PRE_MASK: u32 = 0x1;
pub const AFE_DL_MUTE_CH2_ON_CTL_PRE_MASK_SFT: u32 = 0x1 << 3;
pub const AFE_DL_GAIN_ON_CTL_PRE_SFT: u32 = 1;
pub const AFE_DL_GAIN_ON_CTL_PRE_MASK: u32 = 0x1;
pub const AFE_DL_GAIN_ON_CTL_PRE_MASK_SFT: u32 = 0x1 << 1;
pub const AFE_DL_SRC_ON_TMP_CTL_PRE_SFT: u32 = 0;
pub const AFE_DL_SRC_ON_TMP_CTL_PRE_MASK: u32 = 0x1;
pub const AFE_DL_SRC_ON_TMP_CTL_PRE_MASK_SFT: u32 = 0x1 << 0;

/* AFE_ADDA_DL_SRC_CON1 */
pub const AFE_DL_GAIN1_CTL_PRE_SFT: u32 = 16;
pub const AFE_DL_GAIN1_CTL_PRE_MASK: u32 = 0xffff;
pub const AFE_DL_GAIN1_CTL_PRE_MASK_SFT: u32 = 0xffff << 16;
pub const AFE_DL_GAIN2_CTL_PRE_SFT: u32 = 0;
pub const AFE_DL_GAIN2_CTL_PRE_MASK: u32 = 0xffff;
pub const AFE_DL_GAIN2_CTL_PRE_MASK_SFT: u32 = 0xffff << 0;

/* AFE_ADDA_DL_SRC_DEBUG_MON0 */
pub const AFE_DL_SLT_CNT_FLAG_CTL_SFT: u32 = 15;
pub const AFE_DL_SLT_CNT_FLAG_CTL_MASK: u32 = 0x1;
pub const AFE_DL_SLT_CNT_FLAG_CTL_MASK_SFT: u32 = 0x1 << 15;
pub const AFE_DL_INI_SRAM_FINISH_CTL_SFT: u32 = 12;
pub const AFE_DL_INI_SRAM_FINISH_CTL_MASK: u32 = 0x1;
pub const AFE_DL_INI_SRAM_FINISH_CTL_MASK_SFT: u32 = 0x1 << 12;
pub const AFE_DL_SLT_COUNTER_CTL_SFT: u32 = 0;
pub const AFE_DL_SLT_COUNTER_CTL_MASK: u32 = 0xfff;
pub const AFE_DL_SLT_COUNTER_CTL_MASK_SFT: u32 = 0xfff << 0;

/* AFE_ADDA_DL_PREDIS_CON0 */
pub const AFE_DL_PREDIS_ON_CH1_CTL_SFT: u32 = 31;
pub const AFE_DL_PREDIS_ON_CH1_CTL_MASK: u32 = 0x1;
pub const AFE_DL_PREDIS_ON_CH1_CTL_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_DL_PREDIS_A2_CH1_CTL_SFT: u32 = 16;
pub const AFE_DL_PREDIS_A2_CH1_CTL_MASK: u32 = 0xfff;
pub const AFE_DL_PREDIS_A2_CH1_CTL_MASK_SFT: u32 = 0xfff << 16;
pub const AFE_DL_PREDIS_A3_CH1_CTL_SFT: u32 = 0;
pub const AFE_DL_PREDIS_A3_CH1_CTL_MASK: u32 = 0xfff;
pub const AFE_DL_PREDIS_A3_CH1_CTL_MASK_SFT: u32 = 0xfff << 0;

/* AFE_ADDA_DL_PREDIS_CON1 */
pub const AFE_DL_PREDIS_ON_CH2_CTL_SFT: u32 = 31;
pub const AFE_DL_PREDIS_ON_CH2_CTL_MASK: u32 = 0x1;
pub const AFE_DL_PREDIS_ON_CH2_CTL_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_DL_PREDIS_A2_CH2_CTL_SFT: u32 = 16;
pub const AFE_DL_PREDIS_A2_CH2_CTL_MASK: u32 = 0xfff;
pub const AFE_DL_PREDIS_A2_CH2_CTL_MASK_SFT: u32 = 0xfff << 16;
pub const AFE_DL_PREDIS_A3_CH2_CTL_SFT: u32 = 0;
pub const AFE_DL_PREDIS_A3_CH2_CTL_MASK: u32 = 0xfff;
pub const AFE_DL_PREDIS_A3_CH2_CTL_MASK_SFT: u32 = 0xfff << 0;

/* AFE_ADDA_DL_PREDIS_CON2 */
pub const AFE_DL_PREDIS_A4_CH1_CTL_SFT: u32 = 16;
pub const AFE_DL_PREDIS_A4_CH1_CTL_MASK: u32 = 0xfff;
pub const AFE_DL_PREDIS_A4_CH1_CTL_MASK_SFT: u32 = 0xfff << 16;
pub const AFE_DL_PREDIS_A5_CH1_CTL_SFT: u32 = 0;
pub const AFE_DL_PREDIS_A5_CH1_CTL_MASK: u32 = 0xfff;
pub const AFE_DL_PREDIS_A5_CH1_CTL_MASK_SFT: u32 = 0xfff << 0;

/* AFE_ADDA_DL_PREDIS_CON3 */
pub const AFE_DL_PREDIS_A4_CH2_CTL_SFT: u32 = 16;
pub const AFE_DL_PREDIS_A4_CH2_CTL_MASK: u32 = 0xfff;
pub const AFE_DL_PREDIS_A4_CH2_CTL_MASK_SFT: u32 = 0xfff << 16;
pub const AFE_DL_PREDIS_A5_CH2_CTL_SFT: u32 = 0;
pub const AFE_DL_PREDIS_A5_CH2_CTL_MASK: u32 = 0xfff;
pub const AFE_DL_PREDIS_A5_CH2_CTL_MASK_SFT: u32 = 0xfff << 0;

/* AFE_ADDA_DL_SDM_DCCOMP_CON */
pub const AFE_DL_USE_NEW_2ND_12BIT_SDM_SFT: u32 = 31;
pub const AFE_DL_USE_NEW_2ND_12BIT_SDM_MASK: u32 = 0x1;
pub const AFE_DL_USE_NEW_2ND_12BIT_SDM_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_DL_USE_NEW_2ND_SDM_SFT: u32 = 30;
pub const AFE_DL_USE_NEW_2ND_SDM_MASK: u32 = 0x1;
pub const AFE_DL_USE_NEW_2ND_SDM_MASK_SFT: u32 = 0x1 << 30;
pub const AFE_DL_USE_3RD_SDM_SFT: u32 = 28;
pub const AFE_DL_USE_3RD_SDM_MASK: u32 = 0x1;
pub const AFE_DL_USE_3RD_SDM_MASK_SFT: u32 = 0x1 << 28;
pub const AFE_DL_DCM_AUTO_IDLE_EN_SFT: u32 = 14;
pub const AFE_DL_DCM_AUTO_IDLE_EN_MASK: u32 = 0x1;
pub const AFE_DL_DCM_AUTO_IDLE_EN_MASK_SFT: u32 = 0x1 << 14;
pub const AFE_DL_SRC_DCM_EN_SFT: u32 = 13;
pub const AFE_DL_SRC_DCM_EN_MASK: u32 = 0x1;
pub const AFE_DL_SRC_DCM_EN_MASK_SFT: u32 = 0x1 << 13;
pub const AFE_DL_POST_SRC_DCM_EN_SFT: u32 = 12;
pub const AFE_DL_POST_SRC_DCM_EN_MASK: u32 = 0x1;
pub const AFE_DL_POST_SRC_DCM_EN_MASK_SFT: u32 = 0x1 << 12;
pub const AFE_DL_DCCOMP_SYNC_TOGGLE_SFT: u32 = 11;
pub const AFE_DL_DCCOMP_SYNC_TOGGLE_MASK: u32 = 0x1;
pub const AFE_DL_DCCOMP_SYNC_TOGGLE_MASK_SFT: u32 = 0x1 << 11;
pub const AFE_DL_AUD_SDM_MONO_SFT: u32 = 9;
pub const AFE_DL_AUD_SDM_MONO_MASK: u32 = 0x1;
pub const AFE_DL_AUD_SDM_MONO_MASK_SFT: u32 = 0x1 << 9;
pub const AFE_DL_AUD_DC_COMP_EN_SFT: u32 = 8;
pub const AFE_DL_AUD_DC_COMP_EN_MASK: u32 = 0x1;
pub const AFE_DL_AUD_DC_COMP_EN_MASK_SFT: u32 = 0x1 << 8;
pub const AFE_DL_ATTGAIN_CTL_SFT: u32 = 0;
pub const AFE_DL_ATTGAIN_CTL_MASK: u32 = 0x3f;
pub const AFE_DL_ATTGAIN_CTL_MASK_SFT: u32 = 0x3f << 0;

/* AFE_ADDA_DL_SDM_TEST */
pub const AFE_DL_TRI_AMP_DIV_SFT: u32 = 12;
pub const AFE_DL_TRI_AMP_DIV_MASK: u32 = 0x7;
pub const AFE_DL_TRI_AMP_DIV_MASK_SFT: u32 = 0x7 << 12;
pub const AFE_DL_TRI_FREQ_DIV_SFT: u32 = 4;
pub const AFE_DL_TRI_FREQ_DIV_MASK: u32 = 0x3f;
pub const AFE_DL_TRI_FREQ_DIV_MASK_SFT: u32 = 0x3f << 4;
pub const AFE_DL_RG_DL_LEFT_SAT_RSTN_SFT: u32 = 3;
pub const AFE_DL_RG_DL_LEFT_SAT_RSTN_MASK: u32 = 0x1;
pub const AFE_DL_RG_DL_LEFT_SAT_RSTN_MASK_SFT: u32 = 0x1 << 3;
pub const AFE_DL_RG_DL_RIGHT_SAT_RSTN_SFT: u32 = 2;
pub const AFE_DL_RG_DL_RIGHT_SAT_RSTN_MASK: u32 = 0x1;
pub const AFE_DL_RG_DL_RIGHT_SAT_RSTN_MASK_SFT: u32 = 0x1 << 2;
pub const AFE_DL_TRI_MUTE_SW_SFT: u32 = 1;
pub const AFE_DL_TRI_MUTE_SW_MASK: u32 = 0x1;
pub const AFE_DL_TRI_MUTE_SW_MASK_SFT: u32 = 0x1 << 1;
pub const AFE_DL_TRI_DAC_EN_SFT: u32 = 0;
pub const AFE_DL_TRI_DAC_EN_MASK: u32 = 0x1;
pub const AFE_DL_TRI_DAC_EN_MASK_SFT: u32 = 0x1 << 0;

/* AFE_ADDA_DL_DC_COMP_CFG0 */
pub const AFE_DL_AUD_DC_COMP_LCH_H_SFT: u32 = 16;
pub const AFE_DL_AUD_DC_COMP_LCH_H_MASK: u32 = 0xffff;
pub const AFE_DL_AUD_DC_COMP_LCH_H_MASK_SFT: u32 = 0xffff << 16;
pub const AFE_DL_AUD_DC_COMP_LCH_L_SFT: u32 = 0;
pub const AFE_DL_AUD_DC_COMP_LCH_L_MASK: u32 = 0xffff;
pub const AFE_DL_AUD_DC_COMP_LCH_L_MASK_SFT: u32 = 0xffff << 0;

/* AFE_ADDA_DL_DC_COMP_CFG1 */
pub const AFE_DL_AUD_DC_COMP_RCH_H_SFT: u32 = 16;
pub const AFE_DL_AUD_DC_COMP_RCH_H_MASK: u32 = 0xffff;
pub const AFE_DL_AUD_DC_COMP_RCH_H_MASK_SFT: u32 = 0xffff << 16;
pub const AFE_DL_AUD_DC_COMP_RCH_L_SFT: u32 = 0;
pub const AFE_DL_AUD_DC_COMP_RCH_L_MASK: u32 = 0xffff;
pub const AFE_DL_AUD_DC_COMP_RCH_L_MASK_SFT: u32 = 0xffff << 0;

/* AFE_ADDA_DL_SDM_OUT_MON */
pub const AFE_DL_SDM_DITHER_MON_SFT: u32 = 28;
pub const AFE_DL_SDM_DITHER_MON_MASK: u32 = 0x3;
pub const AFE_DL_SDM_DITHER_MON_MASK_SFT: u32 = 0x3 << 28;
pub const AFE_DL_BF_SDM_LEFT_SAT_SFT: u32 = 21;
pub const AFE_DL_BF_SDM_LEFT_SAT_MASK: u32 = 0x1;
pub const AFE_DL_BF_SDM_LEFT_SAT_MASK_SFT: u32 = 0x1 << 21;
pub const AFE_DL_BF_SDM_RIGHT_SAT_SFT: u32 = 20;
pub const AFE_DL_BF_SDM_RIGHT_SAT_MASK: u32 = 0x1;
pub const AFE_DL_BF_SDM_RIGHT_SAT_MASK_SFT: u32 = 0x1 << 20;
pub const AFE_DL_3RD_SDM_AUTO_RESET_R_SFT: u32 = 19;
pub const AFE_DL_3RD_SDM_AUTO_RESET_R_MASK: u32 = 0x1;
pub const AFE_DL_3RD_SDM_AUTO_RESET_R_MASK_SFT: u32 = 0x1 << 19;
pub const AFE_DL_3RD_SDM_AUTO_RESET_L_SFT: u32 = 18;
pub const AFE_DL_3RD_SDM_AUTO_RESET_L_MASK: u32 = 0x1;
pub const AFE_DL_3RD_SDM_AUTO_RESET_L_MASK_SFT: u32 = 0x1 << 18;
pub const AFE_DL_2ND_SDM_AUTO_RESET_R_SFT: u32 = 17;
pub const AFE_DL_2ND_SDM_AUTO_RESET_R_MASK: u32 = 0x1;
pub const AFE_DL_2ND_SDM_AUTO_RESET_R_MASK_SFT: u32 = 0x1 << 17;
pub const AFE_DL_2ND_SDM_AUTO_RESET_L_SFT: u32 = 16;
pub const AFE_DL_2ND_SDM_AUTO_RESET_L_MASK: u32 = 0x1;
pub const AFE_DL_2ND_SDM_AUTO_RESET_L_MASK_SFT: u32 = 0x1 << 16;
pub const AFE_DL_AUD_SDM_OUT_L_SFT: u32 = 8;
pub const AFE_DL_AUD_SDM_OUT_L_MASK: u32 = 0xff;
pub const AFE_DL_AUD_SDM_OUT_L_MASK_SFT: u32 = 0xff << 8;
pub const AFE_DL_AUD_SDM_OUT_R_SFT: u32 = 0;
pub const AFE_DL_AUD_SDM_OUT_R_MASK: u32 = 0xff;
pub const AFE_DL_AUD_SDM_OUT_R_MASK_SFT: u32 = 0xff << 0;

/* AFE_ADDA_DL_SRC_LCH_MON */
pub const AFE_DL_ASDM_LEFT_SFT: u32 = 0;
pub const AFE_DL_ASDM_LEFT_MASK: u32 = 0xffffff;
pub const AFE_DL_ASDM_LEFT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_ADDA_DL_SRC_RCH_MON */
pub const AFE_DL_ASDM_RIGHT_SFT: u32 = 0;
pub const AFE_DL_ASDM_RIGHT_MASK: u32 = 0xffffff;
pub const AFE_DL_ASDM_RIGHT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_ADDA_DL_SRC_DEBUG */
pub const AFE_DL_SLT_CNT_FLAG_RESET_CTL_SFT: u32 = 12;
pub const AFE_DL_SLT_CNT_FLAG_RESET_CTL_MASK: u32 = 0x1;
pub const AFE_DL_SLT_CNT_FLAG_RESET_CTL_MASK_SFT: u32 = 0x1 << 12;
pub const AFE_DL_SLT_CNT_THD_CTL_SFT: u32 = 0;
pub const AFE_DL_SLT_CNT_THD_CTL_MASK: u32 = 0xfff;
pub const AFE_DL_SLT_CNT_THD_CTL_MASK_SFT: u32 = 0xfff << 0;

/* AFE_ADDA_DL_SDM_DITHER_CON */
pub const AFE_DL_SDM_DITHER_64TAP_EN_SFT: u32 = 20;
pub const AFE_DL_SDM_DITHER_64TAP_EN_MASK: u32 = 0x1;
pub const AFE_DL_SDM_DITHER_64TAP_EN_MASK_SFT: u32 = 0x1 << 20;
pub const AFE_DL_SDM_DITHER_EN_SFT: u32 = 16;
pub const AFE_DL_SDM_DITHER_EN_MASK: u32 = 0x1;
pub const AFE_DL_SDM_DITHER_EN_MASK_SFT: u32 = 0x1 << 16;
pub const AFE_DL_SDM_DITHER_GAIN_SFT: u32 = 0;
pub const AFE_DL_SDM_DITHER_GAIN_MASK: u32 = 0xff;
pub const AFE_DL_SDM_DITHER_GAIN_MASK_SFT: u32 = 0xff << 0;

/* AFE_ADDA_DL_SDM_AUTO_RESET_CON */
pub const AFE_DL_SDM_AUTO_RESET_TEST_ON_SFT: u32 = 31;
pub const AFE_DL_SDM_AUTO_RESET_TEST_ON_MASK: u32 = 0x1;
pub const AFE_DL_SDM_AUTO_RESET_TEST_ON_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_DL_SDM_AUTO_RESET_SOURCE_SEL_SFT: u32 = 24;
pub const AFE_DL_SDM_AUTO_RESET_SOURCE_SEL_MASK: u32 = 0x1;
pub const AFE_DL_SDM_AUTO_RESET_SOURCE_SEL_MASK_SFT: u32 = 0x1 << 24;
pub const AFE_DL_SDM_AUTO_RESET_COUNT_TH_SFT: u32 = 0;
pub const AFE_DL_SDM_AUTO_RESET_COUNT_TH_MASK: u32 = 0xffffff;
pub const AFE_DL_SDM_AUTO_RESET_COUNT_TH_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_CONFIG */
pub const AFE_DL_HBF1_SW_CONFIG_SFT: u32 = 31;
pub const AFE_DL_HBF1_SW_CONFIG_MASK: u32 = 0x1;
pub const AFE_DL_HBF1_SW_CONFIG_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_DL_HBF1_TAPNUM_CONFIG_SFT: u32 = 16;
pub const AFE_DL_HBF1_TAPNUM_CONFIG_MASK: u32 = 0x7f;
pub const AFE_DL_HBF1_TAPNUM_CONFIG_MASK_SFT: u32 = 0x7f << 16;
pub const AFE_DL_SCF1_SW_CONFIG_SFT: u32 = 8;
pub const AFE_DL_SCF1_SW_CONFIG_MASK: u32 = 0x1;
pub const AFE_DL_SCF1_SW_CONFIG_MASK_SFT: u32 = 0x1 << 8;
pub const AFE_DL_SCF1_TAPNUM_CONFIG_SFT: u32 = 0;
pub const AFE_DL_SCF1_TAPNUM_CONFIG_MASK: u32 = 0xff;
pub const AFE_DL_SCF1_TAPNUM_CONFIG_MASK_SFT: u32 = 0xff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP1_TAP2_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP1_TAP2_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP1_TAP2_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP1_TAP2_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP3_TAP4_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP3_TAP4_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP3_TAP4_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP3_TAP4_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP5_TAP6_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP5_TAP6_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP5_TAP6_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP5_TAP6_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP7_TAP8_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP7_TAP8_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP7_TAP8_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP7_TAP8_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP9_TAP10_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP9_TAP10_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP9_TAP10_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP9_TAP10_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP11_TAP12_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP11_TAP12_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP11_TAP12_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP11_TAP12_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP13_TAP14_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP13_TAP14_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP13_TAP14_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP13_TAP14_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP15_TAP16_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP15_TAP16_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP15_TAP16_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP15_TAP16_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP17_TAP18_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP17_TAP18_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP17_TAP18_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP17_TAP18_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP19_TAP20_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP19_TAP20_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP19_TAP20_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP19_TAP20_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP21_TAP22_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP21_TAP22_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP21_TAP22_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP21_TAP22_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP23_TAP24_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP23_TAP24_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP23_TAP24_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP23_TAP24_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP25_TAP26_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP25_TAP26_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP25_TAP26_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP25_TAP26_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP27_TAP28_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP27_TAP28_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP27_TAP28_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP27_TAP28_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP29_TAP30_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP29_TAP30_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP29_TAP30_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP29_TAP30_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP31_TAP32_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP31_TAP32_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP31_TAP32_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP31_TAP32_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP33_TAP34_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP33_TAP34_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP33_TAP34_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP33_TAP34_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP35_TAP36_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP35_TAP36_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP35_TAP36_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP35_TAP36_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP37_TAP38_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP37_TAP38_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP37_TAP38_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP37_TAP38_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP39_TAP40_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP39_TAP40_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP39_TAP40_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP39_TAP40_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP41_TAP42_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP41_TAP42_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP41_TAP42_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP41_TAP42_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP43_TAP44_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP43_TAP44_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP43_TAP44_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP43_TAP44_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP45_TAP46_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP45_TAP46_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP45_TAP46_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP45_TAP46_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP47_TAP48_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP47_TAP48_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP47_TAP48_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP47_TAP48_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP49_TAP50_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP49_TAP50_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP49_TAP50_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP49_TAP50_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP51_TAP52_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP51_TAP52_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP51_TAP52_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP51_TAP52_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP53_TAP54_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP53_TAP54_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP53_TAP54_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP53_TAP54_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_DL_HBF1_SCF1_TAP55_TAP56_CONFIG */
pub const AFE_DL_HBF1_SCF1_TAP55_TAP56_CONFIG_SFT: u32 = 0;
pub const AFE_DL_HBF1_SCF1_TAP55_TAP56_CONFIG_MASK: u32 = 0xffffffff;
pub const AFE_DL_HBF1_SCF1_TAP55_TAP56_CONFIG_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL_NLE_R_CFG0 */
pub const RG_NLE_R_GAIN_DIG_TAR_SFT: u32 = 24;
pub const RG_NLE_R_GAIN_DIG_TAR_MASK: u32 = 0x3f;
pub const RG_NLE_R_GAIN_DIG_TAR_MASK_SFT: u32 = 0x3f << 24;
pub const RG_NLE_R_GAIN_ANA_TAR_SFT: u32 = 16;
pub const RG_NLE_R_GAIN_ANA_TAR_MASK: u32 = 0x3f;
pub const RG_NLE_R_GAIN_ANA_TAR_MASK_SFT: u32 = 0x3f << 16;
pub const RG_NLE_R_NO_ZCE_SFT: u32 = 15;
pub const RG_NLE_R_NO_ZCE_MASK: u32 = 0x1;
pub const RG_NLE_R_NO_ZCE_MASK_SFT: u32 = 0x1 << 15;
pub const RG_NLE_R_HP_MODE_SFT: u32 = 14;
pub const RG_NLE_R_HP_MODE_MASK: u32 = 0x1;
pub const RG_NLE_R_HP_MODE_MASK_SFT: u32 = 0x1 << 14;
pub const RG_NLE_R_GAIN_STEP_SFT: u32 = 8;
pub const RG_NLE_R_GAIN_STEP_MASK: u32 = 0x7;
pub const RG_NLE_R_GAIN_STEP_MASK_SFT: u32 = 0x7 << 8;
pub const RG_NLE_R_TOGGLE_NUM_SFT: u32 = 0;
pub const RG_NLE_R_TOGGLE_NUM_MASK: u32 = 0x3f;
pub const RG_NLE_R_TOGGLE_NUM_MASK_SFT: u32 = 0x3f << 0;

/* AFE_DL_NLE_R_CFG1 */
pub const RG_NLE_R_INITIATE_SFT: u32 = 24;
pub const RG_NLE_R_INITIATE_MASK: u32 = 0x1;
pub const RG_NLE_R_INITIATE_MASK_SFT: u32 = 0x1 << 24;
pub const RG_NLE_R_READY_SFT: u32 = 16;
pub const RG_NLE_R_READY_MASK: u32 = 0x1;
pub const RG_NLE_R_READY_MASK_SFT: u32 = 0x1 << 16;
pub const RG_NLE_R_TIMEOUT_SCALE_SFT: u32 = 12;
pub const RG_NLE_R_TIMEOUT_SCALE_MASK: u32 = 0x7;
pub const RG_NLE_R_TIMEOUT_SCALE_MASK_SFT: u32 = 0x7 << 12;
pub const RG_NLE_R_ANC_ON_SFT: u32 = 11;
pub const RG_NLE_R_ANC_ON_MASK: u32 = 0x1;
pub const RG_NLE_R_ANC_ON_MASK_SFT: u32 = 0x1 << 11;
pub const RG_NLE_R_GTIME_SFT: u32 = 8;
pub const RG_NLE_R_GTIME_MASK: u32 = 0x7;
pub const RG_NLE_R_GTIME_MASK_SFT: u32 = 0x7 << 8;
pub const RG_NLE_R_ON_SFT: u32 = 7;
pub const RG_NLE_R_ON_MASK: u32 = 0x1;
pub const RG_NLE_R_ON_MASK_SFT: u32 = 0x1 << 7;
pub const RG_PDN_NLE_CTL_SFT: u32 = 6;
pub const RG_PDN_NLE_CTL_MASK: u32 = 0x1;
pub const RG_PDN_NLE_CTL_MASK_SFT: u32 = 0x1 << 6;
pub const RG_NLE_R_DELAY_ANA_SFT: u32 = 0;
pub const RG_NLE_R_DELAY_ANA_MASK: u32 = 0x3f;
pub const RG_NLE_R_DELAY_ANA_MASK_SFT: u32 = 0x3f << 0;

/* AFE_DL_NLE_L_CFG0 */
pub const RG_NLE_L_GAIN_DIG_TAR_SFT: u32 = 24;
pub const RG_NLE_L_GAIN_DIG_TAR_MASK: u32 = 0x3f;
pub const RG_NLE_L_GAIN_DIG_TAR_MASK_SFT: u32 = 0x3f << 24;
pub const RG_NLE_L_GAIN_ANA_TAR_SFT: u32 = 16;
pub const RG_NLE_L_GAIN_ANA_TAR_MASK: u32 = 0x3f;
pub const RG_NLE_L_GAIN_ANA_TAR_MASK_SFT: u32 = 0x3f << 16;
pub const RG_NLE_L_NO_ZCE_SFT: u32 = 15;
pub const RG_NLE_L_NO_ZCE_MASK: u32 = 0x1;
pub const RG_NLE_L_NO_ZCE_MASK_SFT: u32 = 0x1 << 15;
pub const RG_NLE_L_HP_MODE_SFT: u32 = 14;
pub const RG_NLE_L_HP_MODE_MASK: u32 = 0x1;
pub const RG_NLE_L_HP_MODE_MASK_SFT: u32 = 0x1 << 14;
pub const RG_NLE_L_GAIN_STEP_SFT: u32 = 8;
pub const RG_NLE_L_GAIN_STEP_MASK: u32 = 0x7;
pub const RG_NLE_L_GAIN_STEP_MASK_SFT: u32 = 0x7 << 8;
pub const RG_NLE_L_TOGGLE_NUM_SFT: u32 = 0;
pub const RG_NLE_L_TOGGLE_NUM_MASK: u32 = 0x3f;
pub const RG_NLE_L_TOGGLE_NUM_MASK_SFT: u32 = 0x3f << 0;

/* AFE_DL_NLE_L_CFG1 */
pub const RG_NLE_L_INITIATE_SFT: u32 = 24;
pub const RG_NLE_L_INITIATE_MASK: u32 = 0x1;
pub const RG_NLE_L_INITIATE_MASK_SFT: u32 = 0x1 << 24;
pub const RG_NLE_L_READY_SFT: u32 = 16;
pub const RG_NLE_L_READY_MASK: u32 = 0x1;
pub const RG_NLE_L_READY_MASK_SFT: u32 = 0x1 << 16;
pub const RG_NLE_L_TIMEOUT_SCALE_SFT: u32 = 12;
pub const RG_NLE_L_TIMEOUT_SCALE_MASK: u32 = 0x7;
pub const RG_NLE_L_TIMEOUT_SCALE_MASK_SFT: u32 = 0x7 << 12;
pub const RG_NLE_L_ANC_ON_SFT: u32 = 11;
pub const RG_NLE_L_ANC_ON_MASK: u32 = 0x1;
pub const RG_NLE_L_ANC_ON_MASK_SFT: u32 = 0x1 << 11;
pub const RG_NLE_L_GTIME_SFT: u32 = 8;
pub const RG_NLE_L_GTIME_MASK: u32 = 0x7;
pub const RG_NLE_L_GTIME_MASK_SFT: u32 = 0x7 << 8;
pub const RG_NLE_L_ON_SFT: u32 = 7;
pub const RG_NLE_L_ON_MASK: u32 = 0x1;
pub const RG_NLE_L_ON_MASK_SFT: u32 = 0x1 << 7;
/* duplicate C macro: RG_PDN_NLE_CTL_SFT = 6 */
/* duplicate C macro: RG_PDN_NLE_CTL_MASK = 0x1 */
/* duplicate C macro: RG_PDN_NLE_CTL_MASK_SFT = 0x1 << 6 */
pub const RG_NLE_L_DELAY_ANA_SFT: u32 = 0;
pub const RG_NLE_L_DELAY_ANA_MASK: u32 = 0x3f;
pub const RG_NLE_L_DELAY_ANA_MASK_SFT: u32 = 0x3f << 0;

/* AFE_DL_NLE_R_MON0 */
pub const NLE_R_GAIN_DIG_CUR_SFT: u32 = 24;
pub const NLE_R_GAIN_DIG_CUR_MASK: u32 = 0x3f;
pub const NLE_R_GAIN_DIG_CUR_MASK_SFT: u32 = 0x3f << 24;
pub const NLE_R_ANC_MASK_SFT: u32 = 23;
pub const NLE_R_ANC_MASK_MASK: u32 = 0x1;
pub const NLE_R_ANC_MASK_MASK_SFT: u32 = 0x1 << 23;
pub const NLE_R_GAIN_ANA_CUR_SFT: u32 = 16;
pub const NLE_R_GAIN_ANA_CUR_MASK: u32 = 0x3f;
pub const NLE_R_GAIN_ANA_CUR_MASK_SFT: u32 = 0x3f << 16;
pub const NLE_R_GAIN_DIG_TAR_CUR_SFT: u32 = 8;
pub const NLE_R_GAIN_DIG_TAR_CUR_MASK: u32 = 0x3f;
pub const NLE_R_GAIN_DIG_TAR_CUR_MASK_SFT: u32 = 0x3f << 8;
pub const NLE_R_GAIN_ANA_TAR_CUR_SFT: u32 = 0;
pub const NLE_R_GAIN_ANA_TAR_CUR_MASK: u32 = 0x3f;
pub const NLE_R_GAIN_ANA_TAR_CUR_MASK_SFT: u32 = 0x3f << 0;

/* AFE_DL_NLE_R_MON1 */
pub const NLE_R_STATE_CUR_SFT: u32 = 28;
pub const NLE_R_STATE_CUR_MASK: u32 = 0x7;
pub const NLE_R_STATE_CUR_MASK_SFT: u32 = 0x7 << 28;
pub const NLE_R_GAIN_STEP_CUR_SFT: u32 = 24;
pub const NLE_R_GAIN_STEP_CUR_MASK: u32 = 0xf;
pub const NLE_R_GAIN_STEP_CUR_MASK_SFT: u32 = 0xf << 24;
pub const NLE_R_TOGGLE_NUM_CUR_SFT: u32 = 16;
pub const NLE_R_TOGGLE_NUM_CUR_MASK: u32 = 0x3f;
pub const NLE_R_TOGGLE_NUM_CUR_MASK_SFT: u32 = 0x3f << 16;
pub const NLE_R_DIG_GAIN_TARGETED_SFT: u32 = 15;
pub const NLE_R_DIG_GAIN_TARGETED_MASK: u32 = 0x1;
pub const NLE_R_DIG_GAIN_TARGETED_MASK_SFT: u32 = 0x1 << 15;
pub const NLE_R_DIG_GAIN_INCREASE_SFT: u32 = 14;
pub const NLE_R_DIG_GAIN_INCREASE_MASK: u32 = 0x1;
pub const NLE_R_DIG_GAIN_INCREASE_MASK_SFT: u32 = 0x1 << 14;
pub const NLE_R_DIG_GAIN_DECREASE_SFT: u32 = 13;
pub const NLE_R_DIG_GAIN_DECREASE_MASK: u32 = 0x1;
pub const NLE_R_DIG_GAIN_DECREASE_MASK_SFT: u32 = 0x1 << 13;
pub const NLE_R_ANA_GAIN_TARGETED_SFT: u32 = 12;
pub const NLE_R_ANA_GAIN_TARGETED_MASK: u32 = 0x1;
pub const NLE_R_ANA_GAIN_TARGETED_MASK_SFT: u32 = 0x1 << 12;
pub const NLE_R_ANA_GAIN_INCREASE_SFT: u32 = 11;
pub const NLE_R_ANA_GAIN_INCREASE_MASK: u32 = 0x1;
pub const NLE_R_ANA_GAIN_INCREASE_MASK_SFT: u32 = 0x1 << 11;
pub const NLE_R_ANA_GAIN_DECREASE_SFT: u32 = 10;
pub const NLE_R_ANA_GAIN_DECREASE_MASK: u32 = 0x1;
pub const NLE_R_ANA_GAIN_DECREASE_MASK_SFT: u32 = 0x1 << 10;
pub const NLE_R_TIME_COUNTER_CUR_SFT: u32 = 0;
pub const NLE_R_TIME_COUNTER_CUR_MASK: u32 = 0x1ff;
pub const NLE_R_TIME_COUNTER_CUR_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL_NLE_R_MON2 */
pub const NLE_R_ANA_GAIN_SFT: u32 = 8;
pub const NLE_R_ANA_GAIN_MASK: u32 = 0x1f;
pub const NLE_R_ANA_GAIN_MASK_SFT: u32 = 0x1f << 8;
pub const NLE_MOSI2_ANA_GAIN_SFT: u32 = 0;
pub const NLE_MOSI2_ANA_GAIN_MASK: u32 = 0x7f;
pub const NLE_MOSI2_ANA_GAIN_MASK_SFT: u32 = 0x7f << 0;

/* AFE_DL_NLE_L_MON0 */
pub const NLE_L_GAIN_DIG_CUR_SFT: u32 = 24;
pub const NLE_L_GAIN_DIG_CUR_MASK: u32 = 0x3f;
pub const NLE_L_GAIN_DIG_CUR_MASK_SFT: u32 = 0x3f << 24;
pub const NLE_L_ANC_MASK_SFT: u32 = 23;
pub const NLE_L_ANC_MASK_MASK: u32 = 0x1;
pub const NLE_L_ANC_MASK_MASK_SFT: u32 = 0x1 << 23;
pub const NLE_L_GAIN_ANA_CUR_SFT: u32 = 16;
pub const NLE_L_GAIN_ANA_CUR_MASK: u32 = 0x3f;
pub const NLE_L_GAIN_ANA_CUR_MASK_SFT: u32 = 0x3f << 16;
pub const NLE_L_GAIN_DIG_TAR_CUR_SFT: u32 = 8;
pub const NLE_L_GAIN_DIG_TAR_CUR_MASK: u32 = 0x3f;
pub const NLE_L_GAIN_DIG_TAR_CUR_MASK_SFT: u32 = 0x3f << 8;
pub const NLE_L_GAIN_ANA_TAR_CUR_SFT: u32 = 0;
pub const NLE_L_GAIN_ANA_TAR_CUR_MASK: u32 = 0x3f;
pub const NLE_L_GAIN_ANA_TAR_CUR_MASK_SFT: u32 = 0x3f << 0;

/* AFE_DL_NLE_L_MON1 */
pub const NLE_L_STATE_CUR_SFT: u32 = 28;
pub const NLE_L_STATE_CUR_MASK: u32 = 0x7;
pub const NLE_L_STATE_CUR_MASK_SFT: u32 = 0x7 << 28;
pub const NLE_L_GAIN_STEP_CUR_SFT: u32 = 24;
pub const NLE_L_GAIN_STEP_CUR_MASK: u32 = 0xf;
pub const NLE_L_GAIN_STEP_CUR_MASK_SFT: u32 = 0xf << 24;
pub const NLE_L_TOGGLE_NUM_CUR_SFT: u32 = 16;
pub const NLE_L_TOGGLE_NUM_CUR_MASK: u32 = 0x3f;
pub const NLE_L_TOGGLE_NUM_CUR_MASK_SFT: u32 = 0x3f << 16;
pub const NLE_L_DIG_GAIN_TARGETED_SFT: u32 = 15;
pub const NLE_L_DIG_GAIN_TARGETED_MASK: u32 = 0x1;
pub const NLE_L_DIG_GAIN_TARGETED_MASK_SFT: u32 = 0x1 << 15;
pub const NLE_L_DIG_GAIN_INCREASE_SFT: u32 = 14;
pub const NLE_L_DIG_GAIN_INCREASE_MASK: u32 = 0x1;
pub const NLE_L_DIG_GAIN_INCREASE_MASK_SFT: u32 = 0x1 << 14;
pub const NLE_L_DIG_GAIN_DECREASE_SFT: u32 = 13;
pub const NLE_L_DIG_GAIN_DECREASE_MASK: u32 = 0x1;
pub const NLE_L_DIG_GAIN_DECREASE_MASK_SFT: u32 = 0x1 << 13;
pub const NLE_L_ANA_GAIN_TARGETED_SFT: u32 = 12;
pub const NLE_L_ANA_GAIN_TARGETED_MASK: u32 = 0x1;
pub const NLE_L_ANA_GAIN_TARGETED_MASK_SFT: u32 = 0x1 << 12;
pub const NLE_L_ANA_GAIN_INCREASE_SFT: u32 = 11;
pub const NLE_L_ANA_GAIN_INCREASE_MASK: u32 = 0x1;
pub const NLE_L_ANA_GAIN_INCREASE_MASK_SFT: u32 = 0x1 << 11;
pub const NLE_L_ANA_GAIN_DECREASE_SFT: u32 = 10;
pub const NLE_L_ANA_GAIN_DECREASE_MASK: u32 = 0x1;
pub const NLE_L_ANA_GAIN_DECREASE_MASK_SFT: u32 = 0x1 << 10;
pub const NLE_L_TIME_COUNTER_CUR_SFT: u32 = 0;
pub const NLE_L_TIME_COUNTER_CUR_MASK: u32 = 0x1ff;
pub const NLE_L_TIME_COUNTER_CUR_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL_NLE_L_MON2 */
pub const NLE_L_ANA_GAIN_SFT: u32 = 8;
pub const NLE_L_ANA_GAIN_MASK: u32 = 0x1f;
pub const NLE_L_ANA_GAIN_MASK_SFT: u32 = 0x1f << 8;
pub const NLE_MOSI1_ANA_GAIN_SFT: u32 = 0;
pub const NLE_MOSI1_ANA_GAIN_MASK: u32 = 0x7f;
pub const NLE_MOSI1_ANA_GAIN_MASK_SFT: u32 = 0x7f << 0;

/* AFE_DL_NLE_GAIN_CFG0 */
pub const MISO2_SEL_SFT: u32 = 4;
pub const MISO2_SEL_MASK: u32 = 0x3;
pub const MISO2_SEL_MASK_SFT: u32 = 0x3 << 4;
pub const MISO1_SEL_SFT: u32 = 0;
pub const MISO1_SEL_MASK: u32 = 0x3;
pub const MISO1_SEL_MASK_SFT: u32 = 0x3 << 0;

/* AFE_DEM_IDWA_CON0 */
pub const RG_IDWA_SDM_MAV_EN_SFT: u32 = 31;
pub const RG_IDWA_SDM_MAV_EN_MASK: u32 = 0x1;
pub const RG_IDWA_SDM_MAV_EN_MASK_SFT: u32 = 0x1 << 31;
pub const RG_IDWA_SDM_ADITHON_SFT: u32 = 30;
pub const RG_IDWA_SDM_ADITHON_MASK: u32 = 0x1;
pub const RG_IDWA_SDM_ADITHON_MASK_SFT: u32 = 0x1 << 30;
pub const RG_IDWA_SDM_ADITHVAL_SFT: u32 = 28;
pub const RG_IDWA_SDM_ADITHVAL_MASK: u32 = 0x3;
pub const RG_IDWA_SDM_ADITHVAL_MASK_SFT: u32 = 0x3 << 28;
pub const RG_IDWA_SDM_LOOPBACK_SFT: u32 = 27;
pub const RG_IDWA_SDM_LOOPBACK_MASK: u32 = 0x1;
pub const RG_IDWA_SDM_LOOPBACK_MASK_SFT: u32 = 0x1 << 27;
pub const RG_IDWA_SEL_SFT: u32 = 26;
pub const RG_IDWA_SEL_MASK: u32 = 0x1;
pub const RG_IDWA_SEL_MASK_SFT: u32 = 0x1 << 26;
pub const RG_IDWA_ON_SFT: u32 = 25;
pub const RG_IDWA_ON_MASK: u32 = 0x1;
pub const RG_IDWA_ON_MASK_SFT: u32 = 0x1 << 25;
pub const RG_DEM_IN_LR_SWAP_SFT: u32 = 24;
pub const RG_DEM_IN_LR_SWAP_MASK: u32 = 0x1;
pub const RG_DEM_IN_LR_SWAP_MASK_SFT: u32 = 0x1 << 24;
pub const RG_DEM_IN_L_INV_SFT: u32 = 23;
pub const RG_DEM_IN_L_INV_MASK: u32 = 0x1;
pub const RG_DEM_IN_L_INV_MASK_SFT: u32 = 0x1 << 23;
pub const RG_DEM_IN_R_EQ_L_SFT: u32 = 22;
pub const RG_DEM_IN_R_EQ_L_MASK: u32 = 0x1;
pub const RG_DEM_IN_R_EQ_L_MASK_SFT: u32 = 0x1 << 22;
pub const RG_DEM_IN_L_MUTE_SFT: u32 = 21;
pub const RG_DEM_IN_L_MUTE_MASK: u32 = 0x1;
pub const RG_DEM_IN_L_MUTE_MASK_SFT: u32 = 0x1 << 21;
pub const RG_DEM_IN_R_MUTE_SFT: u32 = 20;
pub const RG_DEM_IN_R_MUTE_MASK: u32 = 0x1;
pub const RG_DEM_IN_R_MUTE_MASK_SFT: u32 = 0x1 << 20;
pub const RG_DEM_IN_SOURCE_SFT: u32 = 19;
pub const RG_DEM_IN_SOURCE_MASK: u32 = 0x1;
pub const RG_DEM_IN_SOURCE_MASK_SFT: u32 = 0x1 << 19;
pub const RG_DEM_SPLITTER_TRUNC_RND_SFT: u32 = 18;
pub const RG_DEM_SPLITTER_TRUNC_RND_MASK: u32 = 0x1;
pub const RG_DEM_SPLITTER_TRUNC_RND_MASK_SFT: u32 = 0x1 << 18;
pub const RG_DEM_SCRAMBLER_CG_EN_SFT: u32 = 17;
pub const RG_DEM_SCRAMBLER_CG_EN_MASK: u32 = 0x1;
pub const RG_DEM_SCRAMBLER_CG_EN_MASK_SFT: u32 = 0x1 << 17;
pub const RG_DEM_SCRAMBLER_EN_SFT: u32 = 16;
pub const RG_DEM_SCRAMBLER_EN_MASK: u32 = 0x1;
pub const RG_DEM_SCRAMBLER_EN_MASK_SFT: u32 = 0x1 << 16;
pub const RG_DEM_AUD_SDM_7BIT_SEL_SFT: u32 = 15;
pub const RG_DEM_AUD_SDM_7BIT_SEL_MASK: u32 = 0x1;
pub const RG_DEM_AUD_SDM_7BIT_SEL_MASK_SFT: u32 = 0x1 << 15;
pub const RG_DEM_ZERO_PAD_DISABLE_SFT: u32 = 14;
pub const RG_DEM_ZERO_PAD_DISABLE_MASK: u32 = 0x1;
pub const RG_DEM_ZERO_PAD_DISABLE_MASK_SFT: u32 = 0x1 << 14;
pub const RG_DEM_SPLITTER_TEST_EN_SFT: u32 = 13;
pub const RG_DEM_SPLITTER_TEST_EN_MASK: u32 = 0x1;
pub const RG_DEM_SPLITTER_TEST_EN_MASK_SFT: u32 = 0x1 << 13;
pub const RG_DEM_IDAC_TEST_EN_SFT: u32 = 12;
pub const RG_DEM_IDAC_TEST_EN_MASK: u32 = 0x1;
pub const RG_DEM_IDAC_TEST_EN_MASK_SFT: u32 = 0x1 << 12;
pub const RG_DEM_SPLIT_SCRAM_ON_SFT: u32 = 11;
pub const RG_DEM_SPLIT_SCRAM_ON_MASK: u32 = 0x1;
pub const RG_DEM_SPLIT_SCRAM_ON_MASK_SFT: u32 = 0x1 << 11;
pub const RG_DEM_RAND_EN_SFT: u32 = 10;
pub const RG_DEM_RAND_EN_MASK: u32 = 0x1;
pub const RG_DEM_RAND_EN_MASK_SFT: u32 = 0x1 << 10;
pub const RG_DEM_SPLITTER2_DITHER_EN_SFT: u32 = 9;
pub const RG_DEM_SPLITTER2_DITHER_EN_MASK: u32 = 0x1;
pub const RG_DEM_SPLITTER2_DITHER_EN_MASK_SFT: u32 = 0x1 << 9;
pub const RG_DEM_SPLITTER1_DITHER_EN_SFT: u32 = 8;
pub const RG_DEM_SPLITTER1_DITHER_EN_MASK: u32 = 0x1;
pub const RG_DEM_SPLITTER1_DITHER_EN_MASK_SFT: u32 = 0x1 << 8;
pub const RG_DEM_SPLITTER2_DITHER_GAIN_SFT: u32 = 4;
pub const RG_DEM_SPLITTER2_DITHER_GAIN_MASK: u32 = 0xf;
pub const RG_DEM_SPLITTER2_DITHER_GAIN_MASK_SFT: u32 = 0xf << 4;
pub const RG_DEM_SPLITTER1_DITHER_GAIN_SFT: u32 = 0;
pub const RG_DEM_SPLITTER1_DITHER_GAIN_MASK: u32 = 0xf;
pub const RG_DEM_SPLITTER1_DITHER_GAIN_MASK_SFT: u32 = 0xf << 0;

/* DEM_RECONSTRUCT_MON */
pub const DEM_RECONSTRUCT_L_MON_SFT: u32 = 8;
pub const DEM_RECONSTRUCT_L_MON_MASK: u32 = 0xff;
pub const DEM_RECONSTRUCT_L_MON_MASK_SFT: u32 = 0xff << 8;
pub const DEM_RECONSTRUCT_R_MON_SFT: u32 = 0;
pub const DEM_RECONSTRUCT_R_MON_MASK: u32 = 0xff;
pub const DEM_RECONSTRUCT_R_MON_MASK_SFT: u32 = 0xff << 0;

/* AFE_STF_CON0 */
pub const SLT_CNT_FLAG_RESET_SFT: u32 = 28;
pub const SLT_CNT_FLAG_RESET_MASK: u32 = 0x1;
pub const SLT_CNT_FLAG_RESET_MASK_SFT: u32 = 0x1 << 28;
pub const SLT_CNT_THD_SFT: u32 = 16;
pub const SLT_CNT_THD_MASK: u32 = 0xfff;
pub const SLT_CNT_THD_MASK_SFT: u32 = 0xfff << 16;
pub const SIDE_TONE_HALF_TAP_NUM_SFT: u32 = 4;
pub const SIDE_TONE_HALF_TAP_NUM_MASK: u32 = 0x7f;
pub const SIDE_TONE_HALF_TAP_NUM_MASK_SFT: u32 = 0x7f << 4;
pub const SIDE_TONE_ODD_MODE_SFT: u32 = 1;
pub const SIDE_TONE_ODD_MODE_MASK: u32 = 0x1;
pub const SIDE_TONE_ODD_MODE_MASK_SFT: u32 = 0x1 << 1;
pub const SIDE_TONE_ON_SFT: u32 = 0;
pub const SIDE_TONE_ON_MASK: u32 = 0x1;
pub const SIDE_TONE_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_STF_CON1 */
pub const SIDE_TONE_IN_EN_SEL_DOMAIN_SFT: u32 = 5;
pub const SIDE_TONE_IN_EN_SEL_DOMAIN_MASK: u32 = 0x7;
pub const SIDE_TONE_IN_EN_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 5;
pub const SIDE_TONE_IN_EN_SEL_FS_SFT: u32 = 0;
pub const SIDE_TONE_IN_EN_SEL_FS_MASK: u32 = 0x1f;
pub const SIDE_TONE_IN_EN_SEL_FS_MASK_SFT: u32 = 0x1f << 0;

/* AFE_STF_COEFF */
pub const SIDE_TONE_COEFFICIENT_R_W_SEL_SFT: u32 = 24;
pub const SIDE_TONE_COEFFICIENT_R_W_SEL_MASK: u32 = 0x1;
pub const SIDE_TONE_COEFFICIENT_R_W_SEL_MASK_SFT: u32 = 0x1 << 24;
pub const SIDE_TONE_COEFFICIENT_ADDR_SFT: u32 = 16;
pub const SIDE_TONE_COEFFICIENT_ADDR_MASK: u32 = 0x1f;
pub const SIDE_TONE_COEFFICIENT_ADDR_MASK_SFT: u32 = 0x1f << 16;
pub const SIDE_TONE_COEFFICIENT_SFT: u32 = 0;
pub const SIDE_TONE_COEFFICIENT_MASK: u32 = 0xffff;
pub const SIDE_TONE_COEFFICIENT_MASK_SFT: u32 = 0xffff << 0;

/* AFE_STF_GAIN */
pub const SIDE_TONE_POSITIVE_GAIN_SFT: u32 = 16;
pub const SIDE_TONE_POSITIVE_GAIN_MASK: u32 = 0x7;
pub const SIDE_TONE_POSITIVE_GAIN_MASK_SFT: u32 = 0x7 << 16;
pub const SIDE_TONE_GAIN_SFT: u32 = 0;
pub const SIDE_TONE_GAIN_MASK: u32 = 0xffff;
pub const SIDE_TONE_GAIN_MASK_SFT: u32 = 0xffff << 0;

/* AFE_STF_MON */
pub const SIDE_TONE_R_RDY_SFT: u32 = 30;
pub const SIDE_TONE_R_RDY_MASK: u32 = 0x1;
pub const SIDE_TONE_R_RDY_MASK_SFT: u32 = 0x1 << 30;
pub const SIDE_TONE_W_RDY_SFT: u32 = 29;
pub const SIDE_TONE_W_RDY_MASK: u32 = 0x1;
pub const SIDE_TONE_W_RDY_MASK_SFT: u32 = 0x1 << 29;
pub const SLT_CNT_FLAG_SFT: u32 = 28;
pub const SLT_CNT_FLAG_MASK: u32 = 0x1;
pub const SLT_CNT_FLAG_MASK_SFT: u32 = 0x1 << 28;
pub const SLT_CNT_SFT: u32 = 16;
pub const SLT_CNT_MASK: u32 = 0xfff;
pub const SLT_CNT_MASK_SFT: u32 = 0xfff << 16;
pub const SIDE_TONE_COEFF_SFT: u32 = 0;
pub const SIDE_TONE_COEFF_MASK: u32 = 0xffff;
pub const SIDE_TONE_COEFF_MASK_SFT: u32 = 0xffff << 0;

/* AFE_STF_IP_VERSION */
pub const SIDE_TONE_IP_VERSION_SFT: u32 = 0;
pub const SIDE_TONE_IP_VERSION_MASK: u32 = 0xffffffff;
pub const SIDE_TONE_IP_VERSION_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_CM_REG */
pub const AFE_CM_UPDATE_CNT_SFT: u32 = 16;
pub const AFE_CM_UPDATE_CNT_MASK: u32 = 0x7fff;
pub const AFE_CM_UPDATE_CNT_MASK_SFT: u32 = 0x7fff << 16;
pub const AFE_CM_1X_EN_SEL_FS_SFT: u32 = 8;
pub const AFE_CM_1X_EN_SEL_FS_MASK: u32 = 0x1f;
pub const AFE_CM_1X_EN_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const AFE_CM_CH_NUM_SFT: u32 = 2;
pub const AFE_CM_CH_NUM_MASK: u32 = 0x1f;
pub const AFE_CM_CH_NUM_MASK_SFT: u32 = 0x1f << 2;
pub const AFE_CM_BYTE_SWAP_SFT: u32 = 1;
pub const AFE_CM_BYTE_SWAP_MASK: u32 = 0x1;
pub const AFE_CM_BYTE_SWAP_MASK_SFT: u32 = 0x1 << 1;
pub const AFE_CM_BYPASS_MODE_SFT: u32 = 31;
pub const AFE_CM_BYPASS_MODE_MASK: u32 = 0x1;
pub const AFE_CM_BYPASS_MODE_MASK_SFT: u32 = 0x1 << 31;

/* AFE_CM0_CON0 */
pub const AFE_CM0_BYPASS_MODE_SFT: u32 = 31;
pub const AFE_CM0_BYPASS_MODE_MASK: u32 = 0x1;
pub const AFE_CM0_BYPASS_MODE_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_CM0_UPDATE_CNT_SFT: u32 = 16;
pub const AFE_CM0_UPDATE_CNT_MASK: u32 = 0x7fff;
pub const AFE_CM0_UPDATE_CNT_MASK_SFT: u32 = 0x7fff << 16;
pub const AFE_CM0_1X_EN_SEL_DOMAIN_SFT: u32 = 13;
pub const AFE_CM0_1X_EN_SEL_DOMAIN_MASK: u32 = 0x7;
pub const AFE_CM0_1X_EN_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const AFE_CM0_1X_EN_SEL_FS_SFT: u32 = 8;
pub const AFE_CM0_1X_EN_SEL_FS_MASK: u32 = 0x1f;
pub const AFE_CM0_1X_EN_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const AFE_CM0_OUTPUT_MUX_SFT: u32 = 7;
pub const AFE_CM0_OUTPUT_MUX_MASK: u32 = 0x1;
pub const AFE_CM0_OUTPUT_MUX_MASK_SFT: u32 = 0x1 << 7;
pub const AFE_CM0_CH_NUM_SFT: u32 = 2;
pub const AFE_CM0_CH_NUM_MASK: u32 = 0x1f;
pub const AFE_CM0_CH_NUM_MASK_SFT: u32 = 0x1f << 2;
pub const AFE_CM0_BYTE_SWAP_SFT: u32 = 1;
pub const AFE_CM0_BYTE_SWAP_MASK: u32 = 0x1;
pub const AFE_CM0_BYTE_SWAP_MASK_SFT: u32 = 0x1 << 1;
pub const AFE_CM0_ON_SFT: u32 = 0;
pub const AFE_CM0_ON_MASK: u32 = 0x1;
pub const AFE_CM0_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_CM0_MON */
pub const AFE_CM0_BYPASS_MODE_MON_SFT: u32 = 31;
pub const AFE_CM0_BYPASS_MODE_MON_MASK: u32 = 0x1;
pub const AFE_CM0_BYPASS_MODE_MON_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_CM0_OUTPUT_CNT_MON_SFT: u32 = 16;
pub const AFE_CM0_OUTPUT_CNT_MON_MASK: u32 = 0x7fff;
pub const AFE_CM0_OUTPUT_CNT_MON_MASK_SFT: u32 = 0x7fff << 16;
pub const AFE_CM0_CUR_CHSET_MON_SFT: u32 = 5;
pub const AFE_CM0_CUR_CHSET_MON_MASK: u32 = 0xf;
pub const AFE_CM0_CUR_CHSET_MON_MASK_SFT: u32 = 0xf << 5;
pub const AFE_CM0_ODD_FLAG_MON_SFT: u32 = 4;
pub const AFE_CM0_ODD_FLAG_MON_MASK: u32 = 0x1;
pub const AFE_CM0_ODD_FLAG_MON_MASK_SFT: u32 = 0x1 << 4;
pub const AFE_CM0_BYTE_SWAP_MON_SFT: u32 = 1;
pub const AFE_CM0_BYTE_SWAP_MON_MASK: u32 = 0x1;
pub const AFE_CM0_BYTE_SWAP_MON_MASK_SFT: u32 = 0x1 << 1;
pub const AFE_CM0_ON_MON_SFT: u32 = 0;
pub const AFE_CM0_ON_MON_MASK: u32 = 0x1;
pub const AFE_CM0_ON_MON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_CM0_IP_VERSION */
pub const AFE_CM0_IP_VERSION_SFT: u32 = 0;
pub const AFE_CM0_IP_VERSION_MASK: u32 = 0xffffffff;
pub const AFE_CM0_IP_VERSION_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_CM1_CON0 */
pub const AFE_CM1_BYPASS_MODE_SFT: u32 = 31;
pub const AFE_CM1_BYPASS_MODE_MASK: u32 = 0x1;
pub const AFE_CM1_BYPASS_MODE_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_CM1_UPDATE_CNT_SFT: u32 = 16;
pub const AFE_CM1_UPDATE_CNT_MASK: u32 = 0x7fff;
pub const AFE_CM1_UPDATE_CNT_MASK_SFT: u32 = 0x7fff << 16;
pub const AFE_CM1_1X_EN_SEL_DOMAIN_SFT: u32 = 13;
pub const AFE_CM1_1X_EN_SEL_DOMAIN_MASK: u32 = 0x7;
pub const AFE_CM1_1X_EN_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const AFE_CM1_1X_EN_SEL_FS_SFT: u32 = 8;
pub const AFE_CM1_1X_EN_SEL_FS_MASK: u32 = 0x1f;
pub const AFE_CM1_1X_EN_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const AFE_CM1_OUTPUT_MUX_SFT: u32 = 7;
pub const AFE_CM1_OUTPUT_MUX_MASK: u32 = 0x1;
pub const AFE_CM1_OUTPUT_MUX_MASK_SFT: u32 = 0x1 << 7;
pub const AFE_CM1_CH_NUM_SFT: u32 = 2;
pub const AFE_CM1_CH_NUM_MASK: u32 = 0x1f;
pub const AFE_CM1_CH_NUM_MASK_SFT: u32 = 0x1f << 2;
pub const AFE_CM1_BYTE_SWAP_SFT: u32 = 1;
pub const AFE_CM1_BYTE_SWAP_MASK: u32 = 0x1;
pub const AFE_CM1_BYTE_SWAP_MASK_SFT: u32 = 0x1 << 1;
pub const AFE_CM1_ON_SFT: u32 = 0;
pub const AFE_CM1_ON_MASK: u32 = 0x1;
pub const AFE_CM1_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_CM1_MON */
pub const AFE_CM1_BYPASS_MODE_MON_SFT: u32 = 31;
pub const AFE_CM1_BYPASS_MODE_MON_MASK: u32 = 0x1;
pub const AFE_CM1_BYPASS_MODE_MON_MASK_SFT: u32 = 0x1 << 31;
pub const AFE_CM1_OUTPUT_CNT_MON_SFT: u32 = 16;
pub const AFE_CM1_OUTPUT_CNT_MON_MASK: u32 = 0x7fff;
pub const AFE_CM1_OUTPUT_CNT_MON_MASK_SFT: u32 = 0x7fff << 16;
pub const AFE_CM1_CUR_CHSET_MON_SFT: u32 = 5;
pub const AFE_CM1_CUR_CHSET_MON_MASK: u32 = 0xf;
pub const AFE_CM1_CUR_CHSET_MON_MASK_SFT: u32 = 0xf << 5;
pub const AFE_CM1_ODD_FLAG_MON_SFT: u32 = 4;
pub const AFE_CM1_ODD_FLAG_MON_MASK: u32 = 0x1;
pub const AFE_CM1_ODD_FLAG_MON_MASK_SFT: u32 = 0x1 << 4;
pub const AFE_CM1_BYTE_SWAP_MON_SFT: u32 = 1;
pub const AFE_CM1_BYTE_SWAP_MON_MASK: u32 = 0x1;
pub const AFE_CM1_BYTE_SWAP_MON_MASK_SFT: u32 = 0x1 << 1;
pub const AFE_CM1_ON_MON_SFT: u32 = 0;
pub const AFE_CM1_ON_MON_MASK: u32 = 0x1;
pub const AFE_CM1_ON_MON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_CM1_IP_VERSION */
pub const AFE_CM1_IP_VERSION_SFT: u32 = 0;
pub const AFE_CM1_IP_VERSION_MASK: u32 = 0xffffffff;
pub const AFE_CM1_IP_VERSION_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_UL0_SRC_CON0 */
pub const ULCF_CFG_EN_CTL_SFT: u32 = 31;
pub const ULCF_CFG_EN_CTL_MASK: u32 = 0x1;
pub const ULCF_CFG_EN_CTL_MASK_SFT: u32 = 0x1 << 31;
pub const UL_DMIC_PHASE_SEL_CH1_SFT: u32 = 27;
pub const UL_DMIC_PHASE_SEL_CH1_MASK: u32 = 0x7;
pub const UL_DMIC_PHASE_SEL_CH1_MASK_SFT: u32 = 0x7 << 27;
pub const UL_DMIC_PHASE_SEL_CH2_SFT: u32 = 24;
pub const UL_DMIC_PHASE_SEL_CH2_MASK: u32 = 0x7;
pub const UL_DMIC_PHASE_SEL_CH2_MASK_SFT: u32 = 0x7 << 24;
pub const UL_DMIC_TWO_WIRE_CTL_SFT: u32 = 23;
pub const UL_DMIC_TWO_WIRE_CTL_MASK: u32 = 0x1;
pub const UL_DMIC_TWO_WIRE_CTL_MASK_SFT: u32 = 0x1 << 23;
pub const UL_MODE_3P25M_CH2_CTL_SFT: u32 = 22;
pub const UL_MODE_3P25M_CH2_CTL_MASK: u32 = 0x1;
pub const UL_MODE_3P25M_CH2_CTL_MASK_SFT: u32 = 0x1 << 22;
pub const UL_MODE_3P25M_CH1_CTL_SFT: u32 = 21;
pub const UL_MODE_3P25M_CH1_CTL_MASK: u32 = 0x1;
pub const UL_MODE_3P25M_CH1_CTL_MASK_SFT: u32 = 0x1 << 21;
pub const UL_VOICE_MODE_CH1_CH2_CTL_SFT: u32 = 17;
pub const UL_VOICE_MODE_CH1_CH2_CTL_MASK: u32 = 0x7;
pub const UL_VOICE_MODE_CH1_CH2_CTL_MASK_SFT: u32 = 0x7 << 17;
pub const UL_AP_DMIC_ON_SFT: u32 = 16;
pub const UL_AP_DMIC_ON_MASK: u32 = 0x1;
pub const UL_AP_DMIC_ON_MASK_SFT: u32 = 0x1 << 16;
pub const DMIC_LOW_POWER_MODE_CTL_SFT: u32 = 14;
pub const DMIC_LOW_POWER_MODE_CTL_MASK: u32 = 0x3;
pub const DMIC_LOW_POWER_MODE_CTL_MASK_SFT: u32 = 0x3 << 14;
pub const UL_DISABLE_HW_CG_CTL_SFT: u32 = 12;
pub const UL_DISABLE_HW_CG_CTL_MASK: u32 = 0x1;
pub const UL_DISABLE_HW_CG_CTL_MASK_SFT: u32 = 0x1 << 12;
pub const AMIC_26M_SEL_CTL_SFT: u32 = 11;
pub const AMIC_26M_SEL_CTL_MASK: u32 = 0x1;
pub const AMIC_26M_SEL_CTL_MASK_SFT: u32 = 0x1 << 11;
pub const UL_IIR_ON_TMP_CTL_SFT: u32 = 10;
pub const UL_IIR_ON_TMP_CTL_MASK: u32 = 0x1;
pub const UL_IIR_ON_TMP_CTL_MASK_SFT: u32 = 0x1 << 10;
pub const UL_IIRMODE_CTL_SFT: u32 = 7;
pub const UL_IIRMODE_CTL_MASK: u32 = 0x7;
pub const UL_IIRMODE_CTL_MASK_SFT: u32 = 0x7 << 7;
pub const DIGMIC_4P33M_SEL_SFT: u32 = 6;
pub const DIGMIC_4P33M_SEL_MASK: u32 = 0x1;
pub const DIGMIC_4P33M_SEL_MASK_SFT: u32 = 0x1 << 6;
pub const DIGMIC_3P25M_1P625M_SEL_CTL_SFT: u32 = 5;
pub const DIGMIC_3P25M_1P625M_SEL_CTL_MASK: u32 = 0x1;
pub const DIGMIC_3P25M_1P625M_SEL_CTL_MASK_SFT: u32 = 0x1 << 5;
pub const AMIC_6P5M_SEL_CTL_SFT: u32 = 4;
pub const AMIC_6P5M_SEL_CTL_MASK: u32 = 0x1;
pub const AMIC_6P5M_SEL_CTL_MASK_SFT: u32 = 0x1 << 4;
pub const AMIC_1P625M_SEL_CTL_SFT: u32 = 3;
pub const AMIC_1P625M_SEL_CTL_MASK: u32 = 0x1;
pub const AMIC_1P625M_SEL_CTL_MASK_SFT: u32 = 0x1 << 3;
pub const UL_LOOP_BACK_MODE_CTL_SFT: u32 = 2;
pub const UL_LOOP_BACK_MODE_CTL_MASK: u32 = 0x1;
pub const UL_LOOP_BACK_MODE_CTL_MASK_SFT: u32 = 0x1 << 2;
pub const UL_SDM_3_LEVEL_CTL_SFT: u32 = 1;
pub const UL_SDM_3_LEVEL_CTL_MASK: u32 = 0x1;
pub const UL_SDM_3_LEVEL_CTL_MASK_SFT: u32 = 0x1 << 1;
pub const UL_SRC_ON_TMP_CTL_SFT: u32 = 0;
pub const UL_SRC_ON_TMP_CTL_MASK: u32 = 0x1;
pub const UL_SRC_ON_TMP_CTL_MASK_SFT: u32 = 0x1 << 0;

/* AFE_ADDA_UL0_SRC_CON1 */
pub const ADDA_UL_GAIN_VALUE_SFT: u32 = 16;
pub const ADDA_UL_GAIN_VALUE_MASK: u32 = 0xffff;
pub const ADDA_UL_GAIN_VALUE_MASK_SFT: u32 = 0xffff << 16;
pub const ADDA_UL_POSTIVEGAIN_SFT: u32 = 12;
pub const ADDA_UL_POSTIVEGAIN_MASK: u32 = 0x7;
pub const ADDA_UL_POSTIVEGAIN_MASK_SFT: u32 = 0x7 << 12;
pub const ADDA_UL_ODDTAP_MODE_SFT: u32 = 11;
pub const ADDA_UL_ODDTAP_MODE_MASK: u32 = 0x1;
pub const ADDA_UL_ODDTAP_MODE_MASK_SFT: u32 = 0x1 << 11;
pub const ADDA_UL_HALF_TAP_NUM_SFT: u32 = 5;
pub const ADDA_UL_HALF_TAP_NUM_MASK: u32 = 0x3f;
pub const ADDA_UL_HALF_TAP_NUM_MASK_SFT: u32 = 0x3f << 5;
pub const FIFO_SOFT_RST_SFT: u32 = 4;
pub const FIFO_SOFT_RST_MASK: u32 = 0x1;
pub const FIFO_SOFT_RST_MASK_SFT: u32 = 0x1 << 4;
pub const FIFO_SOFT_RST_EN_SFT: u32 = 3;
pub const FIFO_SOFT_RST_EN_MASK: u32 = 0x1;
pub const FIFO_SOFT_RST_EN_MASK_SFT: u32 = 0x1 << 3;
pub const LR_SWAP_SFT: u32 = 2;
pub const LR_SWAP_MASK: u32 = 0x1;
pub const LR_SWAP_MASK_SFT: u32 = 0x1 << 2;
pub const GAIN_MODE_SFT: u32 = 0;
pub const GAIN_MODE_MASK: u32 = 0x3;
pub const GAIN_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_ADDA_UL0_SRC_CON2 */
pub const C_DAC_EN_CTL_SFT: u32 = 27;
pub const C_DAC_EN_CTL_MASK: u32 = 0x1;
pub const C_DAC_EN_CTL_MASK_SFT: u32 = 0x1 << 27;
pub const C_MUTE_SW_CTL_SFT: u32 = 26;
pub const C_MUTE_SW_CTL_MASK: u32 = 0x1;
pub const C_MUTE_SW_CTL_MASK_SFT: u32 = 0x1 << 26;
pub const C_AMP_DIV_CH2_CTL_SFT: u32 = 21;
pub const C_AMP_DIV_CH2_CTL_MASK: u32 = 0x7;
pub const C_AMP_DIV_CH2_CTL_MASK_SFT: u32 = 0x7 << 21;
pub const C_FREQ_DIV_CH2_CTL_SFT: u32 = 16;
pub const C_FREQ_DIV_CH2_CTL_MASK: u32 = 0x1f;
pub const C_FREQ_DIV_CH2_CTL_MASK_SFT: u32 = 0x1f << 16;
pub const C_SINE_MODE_CH2_CTL_SFT: u32 = 12;
pub const C_SINE_MODE_CH2_CTL_MASK: u32 = 0xf;
pub const C_SINE_MODE_CH2_CTL_MASK_SFT: u32 = 0xf << 12;
pub const C_AMP_DIV_CH1_CTL_SFT: u32 = 9;
pub const C_AMP_DIV_CH1_CTL_MASK: u32 = 0x7;
pub const C_AMP_DIV_CH1_CTL_MASK_SFT: u32 = 0x7 << 9;
pub const C_FREQ_DIV_CH1_CTL_SFT: u32 = 4;
pub const C_FREQ_DIV_CH1_CTL_MASK: u32 = 0x1f;
pub const C_FREQ_DIV_CH1_CTL_MASK_SFT: u32 = 0x1f << 4;
pub const C_SINE_MODE_CH1_CTL_SFT: u32 = 0;
pub const C_SINE_MODE_CH1_CTL_MASK: u32 = 0xf;
pub const C_SINE_MODE_CH1_CTL_MASK_SFT: u32 = 0xf << 0;

/* AFE_ADDA_UL0_SRC_DEBUG */
pub const UL_SLT_CNT_FLAG_RESET_CTL_SFT: u32 = 16;
pub const UL_SLT_CNT_FLAG_RESET_CTL_MASK: u32 = 0x1;
pub const UL_SLT_CNT_FLAG_RESET_CTL_MASK_SFT: u32 = 0x1 << 16;
pub const FIFO_DIGMIC_TESTIN_SFT: u32 = 12;
pub const FIFO_DIGMIC_TESTIN_MASK: u32 = 0x3;
pub const FIFO_DIGMIC_TESTIN_MASK_SFT: u32 = 0x3 << 12;
pub const FIFO_DIGMIC_WDATA_TESTEN_SFT: u32 = 11;
pub const FIFO_DIGMIC_WDATA_TESTEN_MASK: u32 = 0x1;
pub const FIFO_DIGMIC_WDATA_TESTEN_MASK_SFT: u32 = 0x1 << 11;
pub const SLT_CNT_THD_CTL_SFT: u32 = 0;
pub const SLT_CNT_THD_CTL_MASK: u32 = 0x7ff;
pub const SLT_CNT_THD_CTL_MASK_SFT: u32 = 0x7ff << 0;

/* AFE_ADDA_UL0_SRC_DEBUG_MON0 */
pub const SLT_CNT_FLAG_CTL_SFT: u32 = 16;
pub const SLT_CNT_FLAG_CTL_MASK: u32 = 0x1;
pub const SLT_CNT_FLAG_CTL_MASK_SFT: u32 = 0x1 << 16;
pub const SLT_COUNTER_CTL_SFT: u32 = 0;
pub const SLT_COUNTER_CTL_MASK: u32 = 0x7ff;
pub const SLT_COUNTER_CTL_MASK_SFT: u32 = 0x7ff << 0;

/* AFE_ADDA_UL0_SRC_MON1 */
pub const UL_VOICE_MODE_CTL_SFT: u32 = 29;
pub const UL_VOICE_MODE_CTL_MASK: u32 = 0x7;
pub const UL_VOICE_MODE_CTL_MASK_SFT: u32 = 0x7 << 29;
pub const DATA_COMB_IN_CH2_SFT: u32 = 24;
pub const DATA_COMB_IN_CH2_MASK: u32 = 0x1f;
pub const DATA_COMB_IN_CH2_MASK_SFT: u32 = 0x1f << 24;
pub const DATA_COMB_OUT_CH2_SFT: u32 = 0;
pub const DATA_COMB_OUT_CH2_MASK: u32 = 0xffffff;
pub const DATA_COMB_OUT_CH2_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_ADDA_UL0_IIR_COEF_02_01 */
pub const ADDA_IIR_COEF_02_01_SFT: u32 = 0;
pub const ADDA_IIR_COEF_02_01_MASK: u32 = 0xffffffff;
pub const ADDA_IIR_COEF_02_01_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_UL0_IIR_COEF_04_03 */
pub const ADDA_IIR_COEF_04_03_SFT: u32 = 0;
pub const ADDA_IIR_COEF_04_03_MASK: u32 = 0xffffffff;
pub const ADDA_IIR_COEF_04_03_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_UL0_IIR_COEF_06_05 */
pub const ADDA_IIR_COEF_06_05_SFT: u32 = 0;
pub const ADDA_IIR_COEF_06_05_MASK: u32 = 0xffffffff;
pub const ADDA_IIR_COEF_06_05_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_UL0_IIR_COEF_08_07 */
pub const ADDA_IIR_COEF_08_07_SFT: u32 = 0;
pub const ADDA_IIR_COEF_08_07_MASK: u32 = 0xffffffff;
pub const ADDA_IIR_COEF_08_07_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_UL0_IIR_COEF_10_09 */
pub const ADDA_IIR_COEF_10_09_SFT: u32 = 0;
pub const ADDA_IIR_COEF_10_09_MASK: u32 = 0xffffffff;
pub const ADDA_IIR_COEF_10_09_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_UL0_ULCF_CFG_02_01 */
pub const ADDA_ULCF_CFG_02_01_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_02_01_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_02_01_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_UL0_ULCF_CFG_04_03 */
pub const ADDA_ULCF_CFG_04_03_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_04_03_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_04_03_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_UL0_ULCF_CFG_06_05 */
pub const ADDA_ULCF_CFG_06_05_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_06_05_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_06_05_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_UL0_ULCF_CFG_08_07 */
pub const ADDA_ULCF_CFG_08_07_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_08_07_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_08_07_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_UL0_ULCF_CFG_10_09 */
pub const ADDA_ULCF_CFG_10_09_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_10_09_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_10_09_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_UL0_ULCF_CFG_12_11 */
pub const ADDA_ULCF_CFG_12_11_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_12_11_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_12_11_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_UL0_ULCF_CFG_14_13 */
pub const ADDA_ULCF_CFG_14_13_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_14_13_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_14_13_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_UL0_ULCF_CFG_16_15 */
pub const ADDA_ULCF_CFG_16_15_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_16_15_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_16_15_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_UL0_ULCF_CFG_18_17 */
pub const ADDA_ULCF_CFG_18_17_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_18_17_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_18_17_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_UL0_ULCF_CFG_20_19 */
pub const ADDA_ULCF_CFG_20_19_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_20_19_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_20_19_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_UL0_ULCF_CFG_22_21 */
pub const ADDA_ULCF_CFG_22_21_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_22_21_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_22_21_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_UL0_ULCF_CFG_24_23 */
pub const ADDA_ULCF_CFG_24_23_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_24_23_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_24_23_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_UL0_ULCF_CFG_26_25 */
pub const ADDA_ULCF_CFG_26_25_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_26_25_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_26_25_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_UL0_ULCF_CFG_28_27 */
pub const ADDA_ULCF_CFG_28_27_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_28_27_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_28_27_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_UL0_ULCF_CFG_30_29 */
pub const ADDA_ULCF_CFG_30_29_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_30_29_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_30_29_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_UL0_ULCF_CFG_32_31 */
pub const ADDA_ULCF_CFG_32_31_SFT: u32 = 0;
pub const ADDA_ULCF_CFG_32_31_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_CFG_32_31_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_UL0_IP_VERSION */
pub const ADDA_ULCF_IP_VERSION_SFT: u32 = 0;
pub const ADDA_ULCF_IP_VERSION_MASK: u32 = 0xffffffff;
pub const ADDA_ULCF_IP_VERSION_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_UL1_SRC_CON0 */
/* duplicate C macro: ULCF_CFG_EN_CTL_SFT = 31 */
/* duplicate C macro: ULCF_CFG_EN_CTL_MASK = 0x1 */
/* duplicate C macro: ULCF_CFG_EN_CTL_MASK_SFT = 0x1 << 31 */
/* duplicate C macro: UL_DMIC_PHASE_SEL_CH1_SFT = 27 */
/* duplicate C macro: UL_DMIC_PHASE_SEL_CH1_MASK = 0x7 */
/* duplicate C macro: UL_DMIC_PHASE_SEL_CH1_MASK_SFT = 0x7 << 27 */
/* duplicate C macro: UL_DMIC_PHASE_SEL_CH2_SFT = 24 */
/* duplicate C macro: UL_DMIC_PHASE_SEL_CH2_MASK = 0x7 */
/* duplicate C macro: UL_DMIC_PHASE_SEL_CH2_MASK_SFT = 0x7 << 24 */
/* duplicate C macro: UL_DMIC_TWO_WIRE_CTL_SFT = 23 */
/* duplicate C macro: UL_DMIC_TWO_WIRE_CTL_MASK = 0x1 */
/* duplicate C macro: UL_DMIC_TWO_WIRE_CTL_MASK_SFT = 0x1 << 23 */
/* duplicate C macro: UL_MODE_3P25M_CH2_CTL_SFT = 22 */
/* duplicate C macro: UL_MODE_3P25M_CH2_CTL_MASK = 0x1 */
/* duplicate C macro: UL_MODE_3P25M_CH2_CTL_MASK_SFT = 0x1 << 22 */
/* duplicate C macro: UL_MODE_3P25M_CH1_CTL_SFT = 21 */
/* duplicate C macro: UL_MODE_3P25M_CH1_CTL_MASK = 0x1 */
/* duplicate C macro: UL_MODE_3P25M_CH1_CTL_MASK_SFT = 0x1 << 21 */
/* duplicate C macro: UL_VOICE_MODE_CH1_CH2_CTL_SFT = 17 */
/* duplicate C macro: UL_VOICE_MODE_CH1_CH2_CTL_MASK = 0x7 */
/* duplicate C macro: UL_VOICE_MODE_CH1_CH2_CTL_MASK_SFT = 0x7 << 17 */
/* duplicate C macro: UL_AP_DMIC_ON_SFT = 16 */
/* duplicate C macro: UL_AP_DMIC_ON_MASK = 0x1 */
/* duplicate C macro: UL_AP_DMIC_ON_MASK_SFT = 0x1 << 16 */
/* duplicate C macro: DMIC_LOW_POWER_MODE_CTL_SFT = 14 */
/* duplicate C macro: DMIC_LOW_POWER_MODE_CTL_MASK = 0x3 */
/* duplicate C macro: DMIC_LOW_POWER_MODE_CTL_MASK_SFT = 0x3 << 14 */
/* duplicate C macro: UL_DISABLE_HW_CG_CTL_SFT = 12 */
/* duplicate C macro: UL_DISABLE_HW_CG_CTL_MASK = 0x1 */
/* duplicate C macro: UL_DISABLE_HW_CG_CTL_MASK_SFT = 0x1 << 12 */
/* duplicate C macro: AMIC_26M_SEL_CTL_SFT = 11 */
/* duplicate C macro: AMIC_26M_SEL_CTL_MASK = 0x1 */
/* duplicate C macro: AMIC_26M_SEL_CTL_MASK_SFT = 0x1 << 11 */
/* duplicate C macro: UL_IIR_ON_TMP_CTL_SFT = 10 */
/* duplicate C macro: UL_IIR_ON_TMP_CTL_MASK = 0x1 */
/* duplicate C macro: UL_IIR_ON_TMP_CTL_MASK_SFT = 0x1 << 10 */
/* duplicate C macro: UL_IIRMODE_CTL_SFT = 7 */
/* duplicate C macro: UL_IIRMODE_CTL_MASK = 0x7 */
/* duplicate C macro: UL_IIRMODE_CTL_MASK_SFT = 0x7 << 7 */
/* duplicate C macro: DIGMIC_4P33M_SEL_SFT = 6 */
/* duplicate C macro: DIGMIC_4P33M_SEL_MASK = 0x1 */
/* duplicate C macro: DIGMIC_4P33M_SEL_MASK_SFT = 0x1 << 6 */
/* duplicate C macro: DIGMIC_3P25M_1P625M_SEL_CTL_SFT = 5 */
/* duplicate C macro: DIGMIC_3P25M_1P625M_SEL_CTL_MASK = 0x1 */
/* duplicate C macro: DIGMIC_3P25M_1P625M_SEL_CTL_MASK_SFT = 0x1 << 5 */
/* duplicate C macro: AMIC_6P5M_SEL_CTL_SFT = 4 */
/* duplicate C macro: AMIC_6P5M_SEL_CTL_MASK = 0x1 */
/* duplicate C macro: AMIC_6P5M_SEL_CTL_MASK_SFT = 0x1 << 4 */
/* duplicate C macro: AMIC_1P625M_SEL_CTL_SFT = 3 */
/* duplicate C macro: AMIC_1P625M_SEL_CTL_MASK = 0x1 */
/* duplicate C macro: AMIC_1P625M_SEL_CTL_MASK_SFT = 0x1 << 3 */
/* duplicate C macro: UL_LOOP_BACK_MODE_CTL_SFT = 2 */
/* duplicate C macro: UL_LOOP_BACK_MODE_CTL_MASK = 0x1 */
/* duplicate C macro: UL_LOOP_BACK_MODE_CTL_MASK_SFT = 0x1 << 2 */
/* duplicate C macro: UL_SDM_3_LEVEL_CTL_SFT = 1 */
/* duplicate C macro: UL_SDM_3_LEVEL_CTL_MASK = 0x1 */
/* duplicate C macro: UL_SDM_3_LEVEL_CTL_MASK_SFT = 0x1 << 1 */
/* duplicate C macro: UL_SRC_ON_TMP_CTL_SFT = 0 */
/* duplicate C macro: UL_SRC_ON_TMP_CTL_MASK = 0x1 */
/* duplicate C macro: UL_SRC_ON_TMP_CTL_MASK_SFT = 0x1 << 0 */

/* AFE_ADDA_UL1_SRC_CON1 */
/* duplicate C macro: ADDA_UL_GAIN_VALUE_SFT = 16 */
/* duplicate C macro: ADDA_UL_GAIN_VALUE_MASK = 0xffff */
/* duplicate C macro: ADDA_UL_GAIN_VALUE_MASK_SFT = 0xffff << 16 */
/* duplicate C macro: ADDA_UL_POSTIVEGAIN_SFT = 12 */
/* duplicate C macro: ADDA_UL_POSTIVEGAIN_MASK = 0x7 */
/* duplicate C macro: ADDA_UL_POSTIVEGAIN_MASK_SFT = 0x7 << 12 */
/* duplicate C macro: ADDA_UL_ODDTAP_MODE_SFT = 11 */
/* duplicate C macro: ADDA_UL_ODDTAP_MODE_MASK = 0x1 */
/* duplicate C macro: ADDA_UL_ODDTAP_MODE_MASK_SFT = 0x1 << 11 */
/* duplicate C macro: ADDA_UL_HALF_TAP_NUM_SFT = 5 */
/* duplicate C macro: ADDA_UL_HALF_TAP_NUM_MASK = 0x3f */
/* duplicate C macro: ADDA_UL_HALF_TAP_NUM_MASK_SFT = 0x3f << 5 */
/* duplicate C macro: FIFO_SOFT_RST_SFT = 4 */
/* duplicate C macro: FIFO_SOFT_RST_MASK = 0x1 */
/* duplicate C macro: FIFO_SOFT_RST_MASK_SFT = 0x1 << 4 */
/* duplicate C macro: FIFO_SOFT_RST_EN_SFT = 3 */
/* duplicate C macro: FIFO_SOFT_RST_EN_MASK = 0x1 */
/* duplicate C macro: FIFO_SOFT_RST_EN_MASK_SFT = 0x1 << 3 */
/* duplicate C macro: LR_SWAP_SFT = 2 */
/* duplicate C macro: LR_SWAP_MASK = 0x1 */
/* duplicate C macro: LR_SWAP_MASK_SFT = 0x1 << 2 */
/* duplicate C macro: GAIN_MODE_SFT = 0 */
/* duplicate C macro: GAIN_MODE_MASK = 0x3 */
/* duplicate C macro: GAIN_MODE_MASK_SFT = 0x3 << 0 */

/* AFE_ADDA_UL1_SRC_CON2 */
/* duplicate C macro: C_DAC_EN_CTL_SFT = 27 */
/* duplicate C macro: C_DAC_EN_CTL_MASK = 0x1 */
/* duplicate C macro: C_DAC_EN_CTL_MASK_SFT = 0x1 << 27 */
/* duplicate C macro: C_MUTE_SW_CTL_SFT = 26 */
/* duplicate C macro: C_MUTE_SW_CTL_MASK = 0x1 */
/* duplicate C macro: C_MUTE_SW_CTL_MASK_SFT = 0x1 << 26 */
/* duplicate C macro: C_AMP_DIV_CH2_CTL_SFT = 21 */
/* duplicate C macro: C_AMP_DIV_CH2_CTL_MASK = 0x7 */
/* duplicate C macro: C_AMP_DIV_CH2_CTL_MASK_SFT = 0x7 << 21 */
/* duplicate C macro: C_FREQ_DIV_CH2_CTL_SFT = 16 */
/* duplicate C macro: C_FREQ_DIV_CH2_CTL_MASK = 0x1f */
/* duplicate C macro: C_FREQ_DIV_CH2_CTL_MASK_SFT = 0x1f << 16 */
/* duplicate C macro: C_SINE_MODE_CH2_CTL_SFT = 12 */
/* duplicate C macro: C_SINE_MODE_CH2_CTL_MASK = 0xf */
/* duplicate C macro: C_SINE_MODE_CH2_CTL_MASK_SFT = 0xf << 12 */
/* duplicate C macro: C_AMP_DIV_CH1_CTL_SFT = 9 */
/* duplicate C macro: C_AMP_DIV_CH1_CTL_MASK = 0x7 */
/* duplicate C macro: C_AMP_DIV_CH1_CTL_MASK_SFT = 0x7 << 9 */
/* duplicate C macro: C_FREQ_DIV_CH1_CTL_SFT = 4 */
/* duplicate C macro: C_FREQ_DIV_CH1_CTL_MASK = 0x1f */
/* duplicate C macro: C_FREQ_DIV_CH1_CTL_MASK_SFT = 0x1f << 4 */
/* duplicate C macro: C_SINE_MODE_CH1_CTL_SFT = 0 */
/* duplicate C macro: C_SINE_MODE_CH1_CTL_MASK = 0xf */
/* duplicate C macro: C_SINE_MODE_CH1_CTL_MASK_SFT = 0xf << 0 */

/* AFE_ADDA_UL1_SRC_DEBUG */
/* duplicate C macro: UL_SLT_CNT_FLAG_RESET_CTL_SFT = 16 */
/* duplicate C macro: UL_SLT_CNT_FLAG_RESET_CTL_MASK = 0x1 */
/* duplicate C macro: UL_SLT_CNT_FLAG_RESET_CTL_MASK_SFT = 0x1 << 16 */
/* duplicate C macro: FIFO_DIGMIC_TESTIN_SFT = 12 */
/* duplicate C macro: FIFO_DIGMIC_TESTIN_MASK = 0x3 */
/* duplicate C macro: FIFO_DIGMIC_TESTIN_MASK_SFT = 0x3 << 12 */
/* duplicate C macro: FIFO_DIGMIC_WDATA_TESTEN_SFT = 11 */
/* duplicate C macro: FIFO_DIGMIC_WDATA_TESTEN_MASK = 0x1 */
/* duplicate C macro: FIFO_DIGMIC_WDATA_TESTEN_MASK_SFT = 0x1 << 11 */
/* duplicate C macro: SLT_CNT_THD_CTL_SFT = 0 */
/* duplicate C macro: SLT_CNT_THD_CTL_MASK = 0x7ff */
/* duplicate C macro: SLT_CNT_THD_CTL_MASK_SFT = 0x7ff << 0 */

/* AFE_ADDA_UL1_SRC_DEBUG_MON0 */
/* duplicate C macro: SLT_CNT_FLAG_CTL_SFT = 16 */
/* duplicate C macro: SLT_CNT_FLAG_CTL_MASK = 0x1 */
/* duplicate C macro: SLT_CNT_FLAG_CTL_MASK_SFT = 0x1 << 16 */
/* duplicate C macro: SLT_COUNTER_CTL_SFT = 0 */
/* duplicate C macro: SLT_COUNTER_CTL_MASK = 0x7ff */
/* duplicate C macro: SLT_COUNTER_CTL_MASK_SFT = 0x7ff << 0 */

/* AFE_ADDA_UL1_SRC_MON1 */
/* duplicate C macro: UL_VOICE_MODE_CTL_SFT = 29 */
/* duplicate C macro: UL_VOICE_MODE_CTL_MASK = 0x7 */
/* duplicate C macro: UL_VOICE_MODE_CTL_MASK_SFT = 0x7 << 29 */
/* duplicate C macro: DATA_COMB_IN_CH2_SFT = 24 */
/* duplicate C macro: DATA_COMB_IN_CH2_MASK = 0x1f */
/* duplicate C macro: DATA_COMB_IN_CH2_MASK_SFT = 0x1f << 24 */
/* duplicate C macro: DATA_COMB_OUT_CH2_SFT = 0 */
/* duplicate C macro: DATA_COMB_OUT_CH2_MASK = 0xffffff */
/* duplicate C macro: DATA_COMB_OUT_CH2_MASK_SFT = 0xffffff << 0 */

/* AFE_ADDA_UL1_IIR_COEF_02_01 */
/* duplicate C macro: ADDA_IIR_COEF_02_01_SFT = 0 */
/* duplicate C macro: ADDA_IIR_COEF_02_01_MASK = 0xffffffff */
/* duplicate C macro: ADDA_IIR_COEF_02_01_MASK_SFT = 0xffffffff << 0 */

/* AFE_ADDA_UL1_IIR_COEF_04_03 */
/* duplicate C macro: ADDA_IIR_COEF_04_03_SFT = 0 */
/* duplicate C macro: ADDA_IIR_COEF_04_03_MASK = 0xffffffff */
/* duplicate C macro: ADDA_IIR_COEF_04_03_MASK_SFT = 0xffffffff << 0 */

/* AFE_ADDA_UL1_IIR_COEF_06_05 */
/* duplicate C macro: ADDA_IIR_COEF_06_05_SFT = 0 */
/* duplicate C macro: ADDA_IIR_COEF_06_05_MASK = 0xffffffff */
/* duplicate C macro: ADDA_IIR_COEF_06_05_MASK_SFT = 0xffffffff << 0 */

/* AFE_ADDA_UL1_IIR_COEF_08_07 */
/* duplicate C macro: ADDA_IIR_COEF_08_07_SFT = 0 */
/* duplicate C macro: ADDA_IIR_COEF_08_07_MASK = 0xffffffff */
/* duplicate C macro: ADDA_IIR_COEF_08_07_MASK_SFT = 0xffffffff << 0 */

/* AFE_ADDA_UL1_IIR_COEF_10_09 */
/* duplicate C macro: ADDA_IIR_COEF_10_09_SFT = 0 */
/* duplicate C macro: ADDA_IIR_COEF_10_09_MASK = 0xffffffff */
/* duplicate C macro: ADDA_IIR_COEF_10_09_MASK_SFT = 0xffffffff << 0 */

/* AFE_ADDA_UL1_ULCF_CFG_02_01 */
/* duplicate C macro: ADDA_ULCF_CFG_02_01_SFT = 0 */
/* duplicate C macro: ADDA_ULCF_CFG_02_01_MASK = 0xffffffff */
/* duplicate C macro: ADDA_ULCF_CFG_02_01_MASK_SFT = 0xffffffff << 0 */

/* AFE_ADDA_UL1_ULCF_CFG_04_03 */
/* duplicate C macro: ADDA_ULCF_CFG_04_03_SFT = 0 */
/* duplicate C macro: ADDA_ULCF_CFG_04_03_MASK = 0xffffffff */
/* duplicate C macro: ADDA_ULCF_CFG_04_03_MASK_SFT = 0xffffffff << 0 */

/* AFE_ADDA_UL1_ULCF_CFG_06_05 */
/* duplicate C macro: ADDA_ULCF_CFG_06_05_SFT = 0 */
/* duplicate C macro: ADDA_ULCF_CFG_06_05_MASK = 0xffffffff */
/* duplicate C macro: ADDA_ULCF_CFG_06_05_MASK_SFT = 0xffffffff << 0 */

/* AFE_ADDA_UL1_ULCF_CFG_08_07 */
/* duplicate C macro: ADDA_ULCF_CFG_08_07_SFT = 0 */
/* duplicate C macro: ADDA_ULCF_CFG_08_07_MASK = 0xffffffff */
/* duplicate C macro: ADDA_ULCF_CFG_08_07_MASK_SFT = 0xffffffff << 0 */

/* AFE_ADDA_UL1_ULCF_CFG_10_09 */
/* duplicate C macro: ADDA_ULCF_CFG_10_09_SFT = 0 */
/* duplicate C macro: ADDA_ULCF_CFG_10_09_MASK = 0xffffffff */
/* duplicate C macro: ADDA_ULCF_CFG_10_09_MASK_SFT = 0xffffffff << 0 */

/* AFE_ADDA_UL1_ULCF_CFG_12_11 */
/* duplicate C macro: ADDA_ULCF_CFG_12_11_SFT = 0 */
/* duplicate C macro: ADDA_ULCF_CFG_12_11_MASK = 0xffffffff */
/* duplicate C macro: ADDA_ULCF_CFG_12_11_MASK_SFT = 0xffffffff << 0 */

/* AFE_ADDA_UL1_ULCF_CFG_14_13 */
/* duplicate C macro: ADDA_ULCF_CFG_14_13_SFT = 0 */
/* duplicate C macro: ADDA_ULCF_CFG_14_13_MASK = 0xffffffff */
/* duplicate C macro: ADDA_ULCF_CFG_14_13_MASK_SFT = 0xffffffff << 0 */

/* AFE_ADDA_UL1_ULCF_CFG_16_15 */
/* duplicate C macro: ADDA_ULCF_CFG_16_15_SFT = 0 */
/* duplicate C macro: ADDA_ULCF_CFG_16_15_MASK = 0xffffffff */
/* duplicate C macro: ADDA_ULCF_CFG_16_15_MASK_SFT = 0xffffffff << 0 */

/* AFE_ADDA_UL1_ULCF_CFG_18_17 */
/* duplicate C macro: ADDA_ULCF_CFG_18_17_SFT = 0 */
/* duplicate C macro: ADDA_ULCF_CFG_18_17_MASK = 0xffffffff */
/* duplicate C macro: ADDA_ULCF_CFG_18_17_MASK_SFT = 0xffffffff << 0 */

/* AFE_ADDA_UL1_ULCF_CFG_20_19 */
/* duplicate C macro: ADDA_ULCF_CFG_20_19_SFT = 0 */
/* duplicate C macro: ADDA_ULCF_CFG_20_19_MASK = 0xffffffff */
/* duplicate C macro: ADDA_ULCF_CFG_20_19_MASK_SFT = 0xffffffff << 0 */

/* AFE_ADDA_UL1_ULCF_CFG_22_21 */
/* duplicate C macro: ADDA_ULCF_CFG_22_21_SFT = 0 */
/* duplicate C macro: ADDA_ULCF_CFG_22_21_MASK = 0xffffffff */
/* duplicate C macro: ADDA_ULCF_CFG_22_21_MASK_SFT = 0xffffffff << 0 */

/* AFE_ADDA_UL1_ULCF_CFG_24_23 */
/* duplicate C macro: ADDA_ULCF_CFG_24_23_SFT = 0 */
/* duplicate C macro: ADDA_ULCF_CFG_24_23_MASK = 0xffffffff */
/* duplicate C macro: ADDA_ULCF_CFG_24_23_MASK_SFT = 0xffffffff << 0 */

/* AFE_ADDA_UL1_ULCF_CFG_26_25 */
/* duplicate C macro: ADDA_ULCF_CFG_26_25_SFT = 0 */
/* duplicate C macro: ADDA_ULCF_CFG_26_25_MASK = 0xffffffff */
/* duplicate C macro: ADDA_ULCF_CFG_26_25_MASK_SFT = 0xffffffff << 0 */

/* AFE_ADDA_UL1_ULCF_CFG_28_27 */
/* duplicate C macro: ADDA_ULCF_CFG_28_27_SFT = 0 */
/* duplicate C macro: ADDA_ULCF_CFG_28_27_MASK = 0xffffffff */
/* duplicate C macro: ADDA_ULCF_CFG_28_27_MASK_SFT = 0xffffffff << 0 */

/* AFE_ADDA_UL1_ULCF_CFG_30_29 */
/* duplicate C macro: ADDA_ULCF_CFG_30_29_SFT = 0 */
/* duplicate C macro: ADDA_ULCF_CFG_30_29_MASK = 0xffffffff */
/* duplicate C macro: ADDA_ULCF_CFG_30_29_MASK_SFT = 0xffffffff << 0 */

/* AFE_ADDA_UL1_ULCF_CFG_32_31 */
/* duplicate C macro: ADDA_ULCF_CFG_32_31_SFT = 0 */
/* duplicate C macro: ADDA_ULCF_CFG_32_31_MASK = 0xffffffff */
/* duplicate C macro: ADDA_ULCF_CFG_32_31_MASK_SFT = 0xffffffff << 0 */

/* AFE_ADDA_UL1_IP_VERSION */
/* duplicate C macro: ADDA_ULCF_IP_VERSION_SFT = 0 */
/* duplicate C macro: ADDA_ULCF_IP_VERSION_MASK = 0xffffffff */
/* duplicate C macro: ADDA_ULCF_IP_VERSION_MASK_SFT = 0xffffffff << 0 */

/* AFE_ADDA_PROXIMITY_CON0 */
pub const PROXIMITY_CH1_ON_SFT: u32 = 12;
pub const PROXIMITY_CH1_ON_MASK: u32 = 0x1;
pub const PROXIMITY_CH1_ON_MASK_SFT: u32 = 0x1 << 12;
pub const PROXIMITY_CH1_SEL_SFT: u32 = 8;
pub const PROXIMITY_CH1_SEL_MASK: u32 = 0xf;
pub const PROXIMITY_CH1_SEL_MASK_SFT: u32 = 0xf << 8;
pub const PROXIMITY_CH2_ON_SFT: u32 = 4;
pub const PROXIMITY_CH2_ON_MASK: u32 = 0x1;
pub const PROXIMITY_CH2_ON_MASK_SFT: u32 = 0x1 << 4;
pub const PROXIMITY_CH2_SEL_SFT: u32 = 0;
pub const PROXIMITY_CH2_SEL_MASK: u32 = 0xf;
pub const PROXIMITY_CH2_SEL_MASK_SFT: u32 = 0xf << 0;

/* AFE_ADDA_ULSRC_PHASE_CON0 */
pub const DMIC1_PHASE_FCLK_SEL_SFT: u32 = 30;
pub const DMIC1_PHASE_FCLK_SEL_MASK: u32 = 0x3;
pub const DMIC1_PHASE_FCLK_SEL_MASK_SFT: u32 = 0x3 << 30;
pub const DMIC0_PHASE_FCLK_SEL_SFT: u32 = 28;
pub const DMIC0_PHASE_FCLK_SEL_MASK: u32 = 0x3;
pub const DMIC0_PHASE_FCLK_SEL_MASK_SFT: u32 = 0x3 << 28;
pub const UL3_PHASE_FCLK_SEL_SFT: u32 = 26;
pub const UL3_PHASE_FCLK_SEL_MASK: u32 = 0x3;
pub const UL3_PHASE_FCLK_SEL_MASK_SFT: u32 = 0x3 << 26;
pub const UL2_PHASE_FCLK_SEL_SFT: u32 = 24;
pub const UL2_PHASE_FCLK_SEL_MASK: u32 = 0x3;
pub const UL2_PHASE_FCLK_SEL_MASK_SFT: u32 = 0x3 << 24;
pub const UL1_PHASE_FCLK_SEL_SFT: u32 = 22;
pub const UL1_PHASE_FCLK_SEL_MASK: u32 = 0x3;
pub const UL1_PHASE_FCLK_SEL_MASK_SFT: u32 = 0x3 << 22;
pub const UL0_PHASE_FCLK_SEL_SFT: u32 = 20;
pub const UL0_PHASE_FCLK_SEL_MASK: u32 = 0x3;
pub const UL0_PHASE_FCLK_SEL_MASK_SFT: u32 = 0x3 << 20;
pub const UL_PHASE_SYNC_FCLK_2_ON_SFT: u32 = 18;
pub const UL_PHASE_SYNC_FCLK_2_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_FCLK_2_ON_MASK_SFT: u32 = 0x1 << 18;
pub const UL_PHASE_SYNC_FCLK_1_ON_SFT: u32 = 17;
pub const UL_PHASE_SYNC_FCLK_1_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_FCLK_1_ON_MASK_SFT: u32 = 0x1 << 17;
pub const UL_PHASE_SYNC_FCLK_0_ON_SFT: u32 = 16;
pub const UL_PHASE_SYNC_FCLK_0_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_FCLK_0_ON_MASK_SFT: u32 = 0x1 << 16;
pub const DMIC1_PHASE_HCLK_SEL_SFT: u32 = 14;
pub const DMIC1_PHASE_HCLK_SEL_MASK: u32 = 0x3;
pub const DMIC1_PHASE_HCLK_SEL_MASK_SFT: u32 = 0x3 << 14;
pub const DMIC0_PHASE_HCLK_SEL_SFT: u32 = 12;
pub const DMIC0_PHASE_HCLK_SEL_MASK: u32 = 0x3;
pub const DMIC0_PHASE_HCLK_SEL_MASK_SFT: u32 = 0x3 << 12;
pub const UL3_PHASE_HCLK_SEL_SFT: u32 = 10;
pub const UL3_PHASE_HCLK_SEL_MASK: u32 = 0x3;
pub const UL3_PHASE_HCLK_SEL_MASK_SFT: u32 = 0x3 << 10;
pub const UL2_PHASE_HCLK_SEL_SFT: u32 = 8;
pub const UL2_PHASE_HCLK_SEL_MASK: u32 = 0x3;
pub const UL2_PHASE_HCLK_SEL_MASK_SFT: u32 = 0x3 << 8;
pub const UL1_PHASE_HCLK_SEL_SFT: u32 = 6;
pub const UL1_PHASE_HCLK_SEL_MASK: u32 = 0x3;
pub const UL1_PHASE_HCLK_SEL_MASK_SFT: u32 = 0x3 << 6;
pub const UL0_PHASE_HCLK_SEL_SFT: u32 = 4;
pub const UL0_PHASE_HCLK_SEL_MASK: u32 = 0x3;
pub const UL0_PHASE_HCLK_SEL_MASK_SFT: u32 = 0x3 << 4;
pub const UL_PHASE_SYNC_HCLK_2_ON_SFT: u32 = 2;
pub const UL_PHASE_SYNC_HCLK_2_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_HCLK_2_ON_MASK_SFT: u32 = 0x1 << 2;
pub const UL_PHASE_SYNC_HCLK_1_ON_SFT: u32 = 1;
pub const UL_PHASE_SYNC_HCLK_1_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_HCLK_1_ON_MASK_SFT: u32 = 0x1 << 1;
pub const UL_PHASE_SYNC_HCLK_0_ON_SFT: u32 = 0;
pub const UL_PHASE_SYNC_HCLK_0_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_HCLK_0_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_ADDA_ULSRC_PHASE_CON1 */
pub const DMIC_CLK_PHASE_SYNC_SET_SFT: u32 = 31;
pub const DMIC_CLK_PHASE_SYNC_SET_MASK: u32 = 0x1;
pub const DMIC_CLK_PHASE_SYNC_SET_MASK_SFT: u32 = 0x1 << 31;
pub const DMIC1_PHASE_SYNC_FCLK_SET_SFT: u32 = 11;
pub const DMIC1_PHASE_SYNC_FCLK_SET_MASK: u32 = 0x1;
pub const DMIC1_PHASE_SYNC_FCLK_SET_MASK_SFT: u32 = 0x1 << 11;
pub const DMIC1_PHASE_SYNC_HCLK_SET_SFT: u32 = 10;
pub const DMIC1_PHASE_SYNC_HCLK_SET_MASK: u32 = 0x1;
pub const DMIC1_PHASE_SYNC_HCLK_SET_MASK_SFT: u32 = 0x1 << 10;
pub const DMIC0_PHASE_SYNC_FCLK_SET_SFT: u32 = 9;
pub const DMIC0_PHASE_SYNC_FCLK_SET_MASK: u32 = 0x1;
pub const DMIC0_PHASE_SYNC_FCLK_SET_MASK_SFT: u32 = 0x1 << 9;
pub const DMIC0_PHASE_SYNC_HCLK_SET_SFT: u32 = 8;
pub const DMIC0_PHASE_SYNC_HCLK_SET_MASK: u32 = 0x1;
pub const DMIC0_PHASE_SYNC_HCLK_SET_MASK_SFT: u32 = 0x1 << 8;
pub const UL3_PHASE_SYNC_FCLK_SET_SFT: u32 = 7;
pub const UL3_PHASE_SYNC_FCLK_SET_MASK: u32 = 0x1;
pub const UL3_PHASE_SYNC_FCLK_SET_MASK_SFT: u32 = 0x1 << 7;
pub const UL3_PHASE_SYNC_HCLK_SET_SFT: u32 = 6;
pub const UL3_PHASE_SYNC_HCLK_SET_MASK: u32 = 0x1;
pub const UL3_PHASE_SYNC_HCLK_SET_MASK_SFT: u32 = 0x1 << 6;
pub const UL2_PHASE_SYNC_FCLK_SET_SFT: u32 = 5;
pub const UL2_PHASE_SYNC_FCLK_SET_MASK: u32 = 0x1;
pub const UL2_PHASE_SYNC_FCLK_SET_MASK_SFT: u32 = 0x1 << 5;
pub const UL2_PHASE_SYNC_HCLK_SET_SFT: u32 = 4;
pub const UL2_PHASE_SYNC_HCLK_SET_MASK: u32 = 0x1;
pub const UL2_PHASE_SYNC_HCLK_SET_MASK_SFT: u32 = 0x1 << 4;
pub const UL1_PHASE_SYNC_FCLK_SET_SFT: u32 = 3;
pub const UL1_PHASE_SYNC_FCLK_SET_MASK: u32 = 0x1;
pub const UL1_PHASE_SYNC_FCLK_SET_MASK_SFT: u32 = 0x1 << 3;
pub const UL1_PHASE_SYNC_HCLK_SET_SFT: u32 = 2;
pub const UL1_PHASE_SYNC_HCLK_SET_MASK: u32 = 0x1;
pub const UL1_PHASE_SYNC_HCLK_SET_MASK_SFT: u32 = 0x1 << 2;
pub const UL0_PHASE_SYNC_FCLK_SET_SFT: u32 = 1;
pub const UL0_PHASE_SYNC_FCLK_SET_MASK: u32 = 0x1;
pub const UL0_PHASE_SYNC_FCLK_SET_MASK_SFT: u32 = 0x1 << 1;
pub const UL0_PHASE_SYNC_HCLK_SET_SFT: u32 = 0;
pub const UL0_PHASE_SYNC_HCLK_SET_MASK: u32 = 0x1;
pub const UL0_PHASE_SYNC_HCLK_SET_MASK_SFT: u32 = 0x1 << 0;

/* AFE_ADDA_ULSRC_PHASE_CON2 */
pub const DMIC1_PHASE_SYNC_1X_EN_SEL_SFT: u32 = 26;
pub const DMIC1_PHASE_SYNC_1X_EN_SEL_MASK: u32 = 0x3;
pub const DMIC1_PHASE_SYNC_1X_EN_SEL_MASK_SFT: u32 = 0x3 << 26;
pub const DMIC0_PHASE_SYNC_1X_EN_SEL_SFT: u32 = 24;
pub const DMIC0_PHASE_SYNC_1X_EN_SEL_MASK: u32 = 0x3;
pub const DMIC0_PHASE_SYNC_1X_EN_SEL_MASK_SFT: u32 = 0x3 << 24;
pub const UL3_PHASE_SYNC_1X_EN_SEL_SFT: u32 = 22;
pub const UL3_PHASE_SYNC_1X_EN_SEL_MASK: u32 = 0x3;
pub const UL3_PHASE_SYNC_1X_EN_SEL_MASK_SFT: u32 = 0x3 << 22;
pub const UL2_PHASE_SYNC_1X_EN_SEL_SFT: u32 = 20;
pub const UL2_PHASE_SYNC_1X_EN_SEL_MASK: u32 = 0x3;
pub const UL2_PHASE_SYNC_1X_EN_SEL_MASK_SFT: u32 = 0x3 << 20;
pub const UL1_PHASE_SYNC_1X_EN_SEL_SFT: u32 = 18;
pub const UL1_PHASE_SYNC_1X_EN_SEL_MASK: u32 = 0x3;
pub const UL1_PHASE_SYNC_1X_EN_SEL_MASK_SFT: u32 = 0x3 << 18;
pub const UL0_PHASE_SYNC_1X_EN_SEL_SFT: u32 = 16;
pub const UL0_PHASE_SYNC_1X_EN_SEL_MASK: u32 = 0x3;
pub const UL0_PHASE_SYNC_1X_EN_SEL_MASK_SFT: u32 = 0x3 << 16;
pub const UL_PHASE_SYNC_FCLK_1X_EN_2_ON_SFT: u32 = 5;
pub const UL_PHASE_SYNC_FCLK_1X_EN_2_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_FCLK_1X_EN_2_ON_MASK_SFT: u32 = 0x1 << 5;
pub const UL_PHASE_SYNC_FCLK_1X_EN_1_ON_SFT: u32 = 4;
pub const UL_PHASE_SYNC_FCLK_1X_EN_1_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_FCLK_1X_EN_1_ON_MASK_SFT: u32 = 0x1 << 4;
pub const UL_PHASE_SYNC_FCLK_1X_EN_0_ON_SFT: u32 = 3;
pub const UL_PHASE_SYNC_FCLK_1X_EN_0_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_FCLK_1X_EN_0_ON_MASK_SFT: u32 = 0x1 << 3;
pub const UL_PHASE_SYNC_HCLK_1X_EN_2_ON_SFT: u32 = 2;
pub const UL_PHASE_SYNC_HCLK_1X_EN_2_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_HCLK_1X_EN_2_ON_MASK_SFT: u32 = 0x1 << 2;
pub const UL_PHASE_SYNC_HCLK_1X_EN_1_ON_SFT: u32 = 1;
pub const UL_PHASE_SYNC_HCLK_1X_EN_1_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_HCLK_1X_EN_1_ON_MASK_SFT: u32 = 0x1 << 1;
pub const UL_PHASE_SYNC_HCLK_1X_EN_0_ON_SFT: u32 = 0;
pub const UL_PHASE_SYNC_HCLK_1X_EN_0_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_HCLK_1X_EN_0_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_ADDA_ULSRC_PHASE_CON3 */
pub const DMIC1_PHASE_SYNC_SOFT_RST_SEL_SFT: u32 = 26;
pub const DMIC1_PHASE_SYNC_SOFT_RST_SEL_MASK: u32 = 0x3;
pub const DMIC1_PHASE_SYNC_SOFT_RST_SEL_MASK_SFT: u32 = 0x3 << 26;
pub const DMIC0_PHASE_SYNC_SOFT_RST_SEL_SFT: u32 = 24;
pub const DMIC0_PHASE_SYNC_SOFT_RST_SEL_MASK: u32 = 0x3;
pub const DMIC0_PHASE_SYNC_SOFT_RST_SEL_MASK_SFT: u32 = 0x3 << 24;
pub const UL3_PHASE_SYNC_SOFT_RST_SEL_SFT: u32 = 22;
pub const UL3_PHASE_SYNC_SOFT_RST_SEL_MASK: u32 = 0x3;
pub const UL3_PHASE_SYNC_SOFT_RST_SEL_MASK_SFT: u32 = 0x3 << 22;
pub const UL2_PHASE_SYNC_SOFT_RST_SEL_SFT: u32 = 20;
pub const UL2_PHASE_SYNC_SOFT_RST_SEL_MASK: u32 = 0x3;
pub const UL2_PHASE_SYNC_SOFT_RST_SEL_MASK_SFT: u32 = 0x3 << 20;
pub const UL1_PHASE_SYNC_SOFT_RST_SEL_SFT: u32 = 18;
pub const UL1_PHASE_SYNC_SOFT_RST_SEL_MASK: u32 = 0x3;
pub const UL1_PHASE_SYNC_SOFT_RST_SEL_MASK_SFT: u32 = 0x3 << 18;
pub const UL0_PHASE_SYNC_SOFT_RST_SEL_SFT: u32 = 16;
pub const UL0_PHASE_SYNC_SOFT_RST_SEL_MASK: u32 = 0x3;
pub const UL0_PHASE_SYNC_SOFT_RST_SEL_MASK_SFT: u32 = 0x3 << 16;
pub const DMIC1_PHASE_SYNC_CH1_FIFO_SEL_SFT: u32 = 13;
pub const DMIC1_PHASE_SYNC_CH1_FIFO_SEL_MASK: u32 = 0x1;
pub const DMIC1_PHASE_SYNC_CH1_FIFO_SEL_MASK_SFT: u32 = 0x1 << 13;
pub const DMIC0_PHASE_SYNC_CH1_FIFO_SEL_SFT: u32 = 12;
pub const DMIC0_PHASE_SYNC_CH1_FIFO_SEL_MASK: u32 = 0x1;
pub const DMIC0_PHASE_SYNC_CH1_FIFO_SEL_MASK_SFT: u32 = 0x1 << 12;
pub const UL3_PHASE_SYNC_CH1_FIFO_SEL_SFT: u32 = 11;
pub const UL3_PHASE_SYNC_CH1_FIFO_SEL_MASK: u32 = 0x1;
pub const UL3_PHASE_SYNC_CH1_FIFO_SEL_MASK_SFT: u32 = 0x1 << 11;
pub const UL2_PHASE_SYNC_CH1_FIFO_SEL_SFT: u32 = 10;
pub const UL2_PHASE_SYNC_CH1_FIFO_SEL_MASK: u32 = 0x1;
pub const UL2_PHASE_SYNC_CH1_FIFO_SEL_MASK_SFT: u32 = 0x1 << 10;
pub const UL1_PHASE_SYNC_CH1_FIFO_SEL_SFT: u32 = 9;
pub const UL1_PHASE_SYNC_CH1_FIFO_SEL_MASK: u32 = 0x1;
pub const UL1_PHASE_SYNC_CH1_FIFO_SEL_MASK_SFT: u32 = 0x1 << 9;
pub const UL0_PHASE_SYNC_CH1_FIFO_SEL_SFT: u32 = 8;
pub const UL0_PHASE_SYNC_CH1_FIFO_SEL_MASK: u32 = 0x1;
pub const UL0_PHASE_SYNC_CH1_FIFO_SEL_MASK_SFT: u32 = 0x1 << 8;
pub const UL_PHASE_SYNC_SOFT_RST_EN_2_ON_SFT: u32 = 5;
pub const UL_PHASE_SYNC_SOFT_RST_EN_2_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_SOFT_RST_EN_2_ON_MASK_SFT: u32 = 0x1 << 5;
pub const UL_PHASE_SYNC_SOFT_RST_EN_1_ON_SFT: u32 = 4;
pub const UL_PHASE_SYNC_SOFT_RST_EN_1_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_SOFT_RST_EN_1_ON_MASK_SFT: u32 = 0x1 << 4;
pub const UL_PHASE_SYNC_SOFT_RST_EN_0_ON_SFT: u32 = 3;
pub const UL_PHASE_SYNC_SOFT_RST_EN_0_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_SOFT_RST_EN_0_ON_MASK_SFT: u32 = 0x1 << 3;
pub const UL_PHASE_SYNC_SOFT_RST_2_ON_SFT: u32 = 2;
pub const UL_PHASE_SYNC_SOFT_RST_2_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_SOFT_RST_2_ON_MASK_SFT: u32 = 0x1 << 2;
pub const UL_PHASE_SYNC_SOFT_RST_1_ON_SFT: u32 = 1;
pub const UL_PHASE_SYNC_SOFT_RST_1_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_SOFT_RST_1_ON_MASK_SFT: u32 = 0x1 << 1;
pub const UL_PHASE_SYNC_SOFT_RST_0_ON_SFT: u32 = 0;
pub const UL_PHASE_SYNC_SOFT_RST_0_ON_MASK: u32 = 0x1;
pub const UL_PHASE_SYNC_SOFT_RST_0_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_MTKAIF_IPM_VER_MON */
pub const RG_MTKAIF_IPM_VER_MON_SFT: u32 = 0;
pub const RG_MTKAIF_IPM_VER_MON_MASK: u32 = 0xffffffff;
pub const RG_MTKAIF_IPM_VER_MON_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_MTKAIF_MON_SEL */
pub const RG_MTKAIF_MON_SEL_SFT: u32 = 0;
pub const RG_MTKAIF_MON_SEL_MASK: u32 = 0xff;
pub const RG_MTKAIF_MON_SEL_MASK_SFT: u32 = 0xff << 0;

/* AFE_MTKAIF_MON */
pub const RG_MTKAIF_MON_SFT: u32 = 0;
pub const RG_MTKAIF_MON_MASK: u32 = 0xffffffff;
pub const RG_MTKAIF_MON_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_MTKAIF0_CFG0 */
pub const RG_MTKAIF0_RXIF_CLKINV_SFT: u32 = 31;
pub const RG_MTKAIF0_RXIF_CLKINV_MASK: u32 = 0x1;
pub const RG_MTKAIF0_RXIF_CLKINV_MASK_SFT: u32 = 0x1 << 31;
pub const RG_MTKAIF0_RXIF_BYPASS_SRC_SFT: u32 = 17;
pub const RG_MTKAIF0_RXIF_BYPASS_SRC_MASK: u32 = 0x1;
pub const RG_MTKAIF0_RXIF_BYPASS_SRC_MASK_SFT: u32 = 0x1 << 17;
pub const RG_MTKAIF0_RXIF_PROTOCOL2_SFT: u32 = 16;
pub const RG_MTKAIF0_RXIF_PROTOCOL2_MASK: u32 = 0x1;
pub const RG_MTKAIF0_RXIF_PROTOCOL2_MASK_SFT: u32 = 0x1 << 16;
pub const RG_MTKAIF0_TXIF_NLE_DEBUG_SFT: u32 = 8;
pub const RG_MTKAIF0_TXIF_NLE_DEBUG_MASK: u32 = 0x1;
pub const RG_MTKAIF0_TXIF_NLE_DEBUG_MASK_SFT: u32 = 0x1 << 8;
pub const RG_MTKAIF0_TXIF_BYPASS_SRC_SFT: u32 = 5;
pub const RG_MTKAIF0_TXIF_BYPASS_SRC_MASK: u32 = 0x1;
pub const RG_MTKAIF0_TXIF_BYPASS_SRC_MASK_SFT: u32 = 0x1 << 5;
pub const RG_MTKAIF0_TXIF_PROTOCOL2_SFT: u32 = 4;
pub const RG_MTKAIF0_TXIF_PROTOCOL2_MASK: u32 = 0x1;
pub const RG_MTKAIF0_TXIF_PROTOCOL2_MASK_SFT: u32 = 0x1 << 4;
pub const RG_MTKAIF0_TXIF_8TO5_SFT: u32 = 2;
pub const RG_MTKAIF0_TXIF_8TO5_MASK: u32 = 0x1;
pub const RG_MTKAIF0_TXIF_8TO5_MASK_SFT: u32 = 0x1 << 2;
pub const RG_MTKAIF0_RXIF_8TO5_SFT: u32 = 1;
pub const RG_MTKAIF0_RXIF_8TO5_MASK: u32 = 0x1;
pub const RG_MTKAIF0_RXIF_8TO5_MASK_SFT: u32 = 0x1 << 1;
pub const RG_MTKAIF0_TX2RX_LOOPBACK1_SFT: u32 = 0;
pub const RG_MTKAIF0_TX2RX_LOOPBACK1_MASK: u32 = 0x1;
pub const RG_MTKAIF0_TX2RX_LOOPBACK1_MASK_SFT: u32 = 0x1 << 0;

/* AFE_MTKAIF0_TX_CFG0 */
pub const RG_MTKAIF0_TXIF_NLE_FIFO_SWAP_SFT: u32 = 23;
pub const RG_MTKAIF0_TXIF_NLE_FIFO_SWAP_MASK: u32 = 0x1;
pub const RG_MTKAIF0_TXIF_NLE_FIFO_SWAP_MASK_SFT: u32 = 0x1 << 23;
pub const RG_MTKAIF0_TXIF_NLE_FIFO_RSP_SFT: u32 = 20;
pub const RG_MTKAIF0_TXIF_NLE_FIFO_RSP_MASK: u32 = 0x7;
pub const RG_MTKAIF0_TXIF_NLE_FIFO_RSP_MASK_SFT: u32 = 0x7 << 20;
pub const RG_MTKAIF0_TXIF_FIFO_SWAP_SFT: u32 = 15;
pub const RG_MTKAIF0_TXIF_FIFO_SWAP_MASK: u32 = 0x1;
pub const RG_MTKAIF0_TXIF_FIFO_SWAP_MASK_SFT: u32 = 0x1 << 15;
pub const RG_MTKAIF0_TXIF_FIFO_RSP_SFT: u32 = 12;
pub const RG_MTKAIF0_TXIF_FIFO_RSP_MASK: u32 = 0x7;
pub const RG_MTKAIF0_TXIF_FIFO_RSP_MASK_SFT: u32 = 0x7 << 12;
pub const RG_MTKAIF0_TXIF_SYNC_WORD1_SFT: u32 = 4;
pub const RG_MTKAIF0_TXIF_SYNC_WORD1_MASK: u32 = 0x7;
pub const RG_MTKAIF0_TXIF_SYNC_WORD1_MASK_SFT: u32 = 0x7 << 4;
pub const RG_MTKAIF0_TXIF_SYNC_WORD0_SFT: u32 = 0;
pub const RG_MTKAIF0_TXIF_SYNC_WORD0_MASK: u32 = 0x7;
pub const RG_MTKAIF0_TXIF_SYNC_WORD0_MASK_SFT: u32 = 0x7 << 0;

/* AFE_MTKAIF0_RX_CFG0 */
pub const RG_MTKAIF0_RXIF_VOICE_MODE_SFT: u32 = 20;
pub const RG_MTKAIF0_RXIF_VOICE_MODE_MASK: u32 = 0xf;
pub const RG_MTKAIF0_RXIF_VOICE_MODE_MASK_SFT: u32 = 0xf << 20;
pub const RG_MTKAIF0_RXIF_DETECT_ON_SFT: u32 = 16;
pub const RG_MTKAIF0_RXIF_DETECT_ON_MASK: u32 = 0x1;
pub const RG_MTKAIF0_RXIF_DETECT_ON_MASK_SFT: u32 = 0x1 << 16;
pub const RG_MTKAIF0_RXIF_DATA_BIT_SFT: u32 = 8;
pub const RG_MTKAIF0_RXIF_DATA_BIT_MASK: u32 = 0x7;
pub const RG_MTKAIF0_RXIF_DATA_BIT_MASK_SFT: u32 = 0x7 << 8;
pub const RG_MTKAIF0_RXIF_FIFO_RSP_SFT: u32 = 4;
pub const RG_MTKAIF0_RXIF_FIFO_RSP_MASK: u32 = 0x7;
pub const RG_MTKAIF0_RXIF_FIFO_RSP_MASK_SFT: u32 = 0x7 << 4;
pub const RG_MTKAIF0_RXIF_DATA_MODE_SFT: u32 = 0;
pub const RG_MTKAIF0_RXIF_DATA_MODE_MASK: u32 = 0x1;
pub const RG_MTKAIF0_RXIF_DATA_MODE_MASK_SFT: u32 = 0x1 << 0;

/* AFE_MTKAIF0_RX_CFG1 */
pub const RG_MTKAIF0_RXIF_CLEAR_SYNC_FAIL_SFT: u32 = 28;
pub const RG_MTKAIF0_RXIF_CLEAR_SYNC_FAIL_MASK: u32 = 0x1;
pub const RG_MTKAIF0_RXIF_CLEAR_SYNC_FAIL_MASK_SFT: u32 = 0x1 << 28;
pub const RG_MTKAIF0_RXIF_SYNC_CNT_TABLE_SFT: u32 = 16;
pub const RG_MTKAIF0_RXIF_SYNC_CNT_TABLE_MASK: u32 = 0xfff;
pub const RG_MTKAIF0_RXIF_SYNC_CNT_TABLE_MASK_SFT: u32 = 0xfff << 16;
pub const RG_MTKAIF0_RXIF_SYNC_SEARCH_TABLE_SFT: u32 = 12;
pub const RG_MTKAIF0_RXIF_SYNC_SEARCH_TABLE_MASK: u32 = 0xf;
pub const RG_MTKAIF0_RXIF_SYNC_SEARCH_TABLE_MASK_SFT: u32 = 0xf << 12;
pub const RG_MTKAIF0_RXIF_INVALID_SYNC_CHECK_ROUND_SFT: u32 = 8;
pub const RG_MTKAIF0_RXIF_INVALID_SYNC_CHECK_ROUND_MASK: u32 = 0xf;
pub const RG_MTKAIF0_RXIF_INVALID_SYNC_CHECK_ROUND_MASK_SFT: u32 = 0xf << 8;
pub const RG_MTKAIF0_RXIF_SYNC_CHECK_ROUND_SFT: u32 = 4;
pub const RG_MTKAIF0_RXIF_SYNC_CHECK_ROUND_MASK: u32 = 0xf;
pub const RG_MTKAIF0_RXIF_SYNC_CHECK_ROUND_MASK_SFT: u32 = 0xf << 4;

/* AFE_MTKAIF0_RX_CFG2 */
pub const RG_MTKAIF0_RXIF_SYNC_WORD1_DISABLE_SFT: u32 = 27;
pub const RG_MTKAIF0_RXIF_SYNC_WORD1_DISABLE_MASK: u32 = 0x1;
pub const RG_MTKAIF0_RXIF_SYNC_WORD1_DISABLE_MASK_SFT: u32 = 0x1 << 27;
pub const RG_MTKAIF0_RXIF_SYNC_WORD1_SFT: u32 = 24;
pub const RG_MTKAIF0_RXIF_SYNC_WORD1_MASK: u32 = 0x7;
pub const RG_MTKAIF0_RXIF_SYNC_WORD1_MASK_SFT: u32 = 0x7 << 24;
pub const RG_MTKAIF0_RXIF_SYNC_WORD0_DISABLE_SFT: u32 = 23;
pub const RG_MTKAIF0_RXIF_SYNC_WORD0_DISABLE_MASK: u32 = 0x1;
pub const RG_MTKAIF0_RXIF_SYNC_WORD0_DISABLE_MASK_SFT: u32 = 0x1 << 23;
pub const RG_MTKAIF0_RXIF_SYNC_WORD0_SFT: u32 = 20;
pub const RG_MTKAIF0_RXIF_SYNC_WORD0_MASK: u32 = 0x7;
pub const RG_MTKAIF0_RXIF_SYNC_WORD0_MASK_SFT: u32 = 0x7 << 20;
pub const RG_MTKAIF0_RXIF_DELAY_CYCLE_SFT: u32 = 12;
pub const RG_MTKAIF0_RXIF_DELAY_CYCLE_MASK: u32 = 0xf;
pub const RG_MTKAIF0_RXIF_DELAY_CYCLE_MASK_SFT: u32 = 0xf << 12;
pub const RG_MTKAIF0_RXIF_DELAY_DATA_SFT: u32 = 8;
pub const RG_MTKAIF0_RXIF_DELAY_DATA_MASK: u32 = 0x1;
pub const RG_MTKAIF0_RXIF_DELAY_DATA_MASK_SFT: u32 = 0x1 << 8;

/* AFE_MTKAIF1_CFG0 */
pub const RG_MTKAIF1_RXIF_CLKINV_ADC_SFT: u32 = 31;
pub const RG_MTKAIF1_RXIF_CLKINV_ADC_MASK: u32 = 0x1;
pub const RG_MTKAIF1_RXIF_CLKINV_ADC_MASK_SFT: u32 = 0x1 << 31;
pub const RG_MTKAIF1_RXIF_BYPASS_SRC_SFT: u32 = 17;
pub const RG_MTKAIF1_RXIF_BYPASS_SRC_MASK: u32 = 0x1;
pub const RG_MTKAIF1_RXIF_BYPASS_SRC_MASK_SFT: u32 = 0x1 << 17;
pub const RG_MTKAIF1_RXIF_PROTOCOL2_SFT: u32 = 16;
pub const RG_MTKAIF1_RXIF_PROTOCOL2_MASK: u32 = 0x1;
pub const RG_MTKAIF1_RXIF_PROTOCOL2_MASK_SFT: u32 = 0x1 << 16;
pub const RG_MTKAIF1_TXIF_NLE_DEBUG_SFT: u32 = 8;
pub const RG_MTKAIF1_TXIF_NLE_DEBUG_MASK: u32 = 0x1;
pub const RG_MTKAIF1_TXIF_NLE_DEBUG_MASK_SFT: u32 = 0x1 << 8;
pub const RG_MTKAIF1_TXIF_BYPASS_SRC_SFT: u32 = 5;
pub const RG_MTKAIF1_TXIF_BYPASS_SRC_MASK: u32 = 0x1;
pub const RG_MTKAIF1_TXIF_BYPASS_SRC_MASK_SFT: u32 = 0x1 << 5;
pub const RG_MTKAIF1_TXIF_PROTOCOL2_SFT: u32 = 4;
pub const RG_MTKAIF1_TXIF_PROTOCOL2_MASK: u32 = 0x1;
pub const RG_MTKAIF1_TXIF_PROTOCOL2_MASK_SFT: u32 = 0x1 << 4;
pub const RG_MTKAIF1_TXIF_8TO5_SFT: u32 = 2;
pub const RG_MTKAIF1_TXIF_8TO5_MASK: u32 = 0x1;
pub const RG_MTKAIF1_TXIF_8TO5_MASK_SFT: u32 = 0x1 << 2;
pub const RG_MTKAIF1_RXIF_8TO5_SFT: u32 = 1;
pub const RG_MTKAIF1_RXIF_8TO5_MASK: u32 = 0x1;
pub const RG_MTKAIF1_RXIF_8TO5_MASK_SFT: u32 = 0x1 << 1;
pub const RG_MTKAIF1_IF_LOOPBACK1_SFT: u32 = 0;
pub const RG_MTKAIF1_IF_LOOPBACK1_MASK: u32 = 0x1;
pub const RG_MTKAIF1_IF_LOOPBACK1_MASK_SFT: u32 = 0x1 << 0;

/* AFE_MTKAIF1_TX_CFG0 */
pub const RG_MTKAIF1_TXIF_NLE_FIFO_SWAP_SFT: u32 = 23;
pub const RG_MTKAIF1_TXIF_NLE_FIFO_SWAP_MASK: u32 = 0x1;
pub const RG_MTKAIF1_TXIF_NLE_FIFO_SWAP_MASK_SFT: u32 = 0x1 << 23;
pub const RG_MTKAIF1_TXIF_NLE_FIFO_RSP_SFT: u32 = 20;
pub const RG_MTKAIF1_TXIF_NLE_FIFO_RSP_MASK: u32 = 0x7;
pub const RG_MTKAIF1_TXIF_NLE_FIFO_RSP_MASK_SFT: u32 = 0x7 << 20;
pub const RG_MTKAIF1_TXIF_FIFO_SWAP_SFT: u32 = 15;
pub const RG_MTKAIF1_TXIF_FIFO_SWAP_MASK: u32 = 0x1;
pub const RG_MTKAIF1_TXIF_FIFO_SWAP_MASK_SFT: u32 = 0x1 << 15;
pub const RG_MTKAIF1_TXIF_FIFO_RSP_SFT: u32 = 12;
pub const RG_MTKAIF1_TXIF_FIFO_RSP_MASK: u32 = 0x7;
pub const RG_MTKAIF1_TXIF_FIFO_RSP_MASK_SFT: u32 = 0x7 << 12;
pub const RG_MTKAIF1_TXIF_SYNC_WORD1_SFT: u32 = 4;
pub const RG_MTKAIF1_TXIF_SYNC_WORD1_MASK: u32 = 0x7;
pub const RG_MTKAIF1_TXIF_SYNC_WORD1_MASK_SFT: u32 = 0x7 << 4;
pub const RG_MTKAIF1_TXIF_SYNC_WORD0_SFT: u32 = 0;
pub const RG_MTKAIF1_TXIF_SYNC_WORD0_MASK: u32 = 0x7;
pub const RG_MTKAIF1_TXIF_SYNC_WORD0_MASK_SFT: u32 = 0x7 << 0;

/* AFE_MTKAIF1_RX_CFG0 */
pub const RG_MTKAIF1_RXIF_VOICE_MODE_SFT: u32 = 20;
pub const RG_MTKAIF1_RXIF_VOICE_MODE_MASK: u32 = 0xf;
pub const RG_MTKAIF1_RXIF_VOICE_MODE_MASK_SFT: u32 = 0xf << 20;
pub const RG_MTKAIF1_RXIF_DETECT_ON_SFT: u32 = 16;
pub const RG_MTKAIF1_RXIF_DETECT_ON_MASK: u32 = 0x1;
pub const RG_MTKAIF1_RXIF_DETECT_ON_MASK_SFT: u32 = 0x1 << 16;
pub const RG_MTKAIF1_RXIF_DATA_BIT_SFT: u32 = 8;
pub const RG_MTKAIF1_RXIF_DATA_BIT_MASK: u32 = 0x7;
pub const RG_MTKAIF1_RXIF_DATA_BIT_MASK_SFT: u32 = 0x7 << 8;
pub const RG_MTKAIF1_RXIF_FIFO_RSP_SFT: u32 = 4;
pub const RG_MTKAIF1_RXIF_FIFO_RSP_MASK: u32 = 0x7;
pub const RG_MTKAIF1_RXIF_FIFO_RSP_MASK_SFT: u32 = 0x7 << 4;
pub const RG_MTKAIF1_RXIF_DATA_MODE_SFT: u32 = 0;
pub const RG_MTKAIF1_RXIF_DATA_MODE_MASK: u32 = 0x1;
pub const RG_MTKAIF1_RXIF_DATA_MODE_MASK_SFT: u32 = 0x1 << 0;

/* AFE_MTKAIF1_RX_CFG1 */
pub const RG_MTKAIF1_RXIF_CLEAR_SYNC_FAIL_SFT: u32 = 28;
pub const RG_MTKAIF1_RXIF_CLEAR_SYNC_FAIL_MASK: u32 = 0x1;
pub const RG_MTKAIF1_RXIF_CLEAR_SYNC_FAIL_MASK_SFT: u32 = 0x1 << 28;
pub const RG_MTKAIF1_RXIF_SYNC_CNT_TABLE_SFT: u32 = 16;
pub const RG_MTKAIF1_RXIF_SYNC_CNT_TABLE_MASK: u32 = 0xfff;
pub const RG_MTKAIF1_RXIF_SYNC_CNT_TABLE_MASK_SFT: u32 = 0xfff << 16;
pub const RG_MTKAIF1_RXIF_SYNC_SEARCH_TABLE_SFT: u32 = 12;
pub const RG_MTKAIF1_RXIF_SYNC_SEARCH_TABLE_MASK: u32 = 0xf;
pub const RG_MTKAIF1_RXIF_SYNC_SEARCH_TABLE_MASK_SFT: u32 = 0xf << 12;
pub const RG_MTKAIF1_RXIF_INVALID_SYNC_CHECK_ROUND_SFT: u32 = 8;
pub const RG_MTKAIF1_RXIF_INVALID_SYNC_CHECK_ROUND_MASK: u32 = 0xf;
pub const RG_MTKAIF1_RXIF_INVALID_SYNC_CHECK_ROUND_MASK_SFT: u32 = 0xf << 8;
pub const RG_MTKAIF1_RXIF_SYNC_CHECK_ROUND_SFT: u32 = 4;
pub const RG_MTKAIF1_RXIF_SYNC_CHECK_ROUND_MASK: u32 = 0xf;
pub const RG_MTKAIF1_RXIF_SYNC_CHECK_ROUND_MASK_SFT: u32 = 0xf << 4;

/* AFE_MTKAIF1_RX_CFG2 */
pub const RG_MTKAIF1_RXIF_SYNC_WORD1_DISABLE_SFT: u32 = 27;
pub const RG_MTKAIF1_RXIF_SYNC_WORD1_DISABLE_MASK: u32 = 0x1;
pub const RG_MTKAIF1_RXIF_SYNC_WORD1_DISABLE_MASK_SFT: u32 = 0x1 << 27;
pub const RG_MTKAIF1_RXIF_SYNC_WORD1_SFT: u32 = 24;
pub const RG_MTKAIF1_RXIF_SYNC_WORD1_MASK: u32 = 0x7;
pub const RG_MTKAIF1_RXIF_SYNC_WORD1_MASK_SFT: u32 = 0x7 << 24;
pub const RG_MTKAIF1_RXIF_SYNC_WORD0_DISABLE_SFT: u32 = 23;
pub const RG_MTKAIF1_RXIF_SYNC_WORD0_DISABLE_MASK: u32 = 0x1;
pub const RG_MTKAIF1_RXIF_SYNC_WORD0_DISABLE_MASK_SFT: u32 = 0x1 << 23;
pub const RG_MTKAIF1_RXIF_SYNC_WORD0_SFT: u32 = 20;
pub const RG_MTKAIF1_RXIF_SYNC_WORD0_MASK: u32 = 0x7;
pub const RG_MTKAIF1_RXIF_SYNC_WORD0_MASK_SFT: u32 = 0x7 << 20;
pub const RG_MTKAIF1_RXIF_DELAY_CYCLE_SFT: u32 = 12;
pub const RG_MTKAIF1_RXIF_DELAY_CYCLE_MASK: u32 = 0xf;
pub const RG_MTKAIF1_RXIF_DELAY_CYCLE_MASK_SFT: u32 = 0xf << 12;
pub const RG_MTKAIF1_RXIF_DELAY_DATA_SFT: u32 = 8;
pub const RG_MTKAIF1_RXIF_DELAY_DATA_MASK: u32 = 0x1;
pub const RG_MTKAIF1_RXIF_DELAY_DATA_MASK_SFT: u32 = 0x1 << 8;

/* AFE_AUD_PAD_TOP_CFG0 */
pub const AUD_PAD_TOP_FIFO_RSP_SFT: u32 = 4;
pub const AUD_PAD_TOP_FIFO_RSP_MASK: u32 = 0xf;
pub const AUD_PAD_TOP_FIFO_RSP_MASK_SFT: u32 = 0xf << 4;
pub const RG_RX_PROTOCOL2_SFT: u32 = 3;
pub const RG_RX_PROTOCOL2_MASK: u32 = 0x1;
pub const RG_RX_PROTOCOL2_MASK_SFT: u32 = 0x1 << 3;
pub const RG_RX_FIFO_ON_SFT: u32 = 0;
pub const RG_RX_FIFO_ON_MASK: u32 = 0x1;
pub const RG_RX_FIFO_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_AUD_PAD_TOP_MON */
pub const AUD_PAD_TOP_MON_SFT: u32 = 0;
pub const AUD_PAD_TOP_MON_MASK: u32 = 0xffff;
pub const AUD_PAD_TOP_MON_MASK_SFT: u32 = 0xffff << 0;

/* AFE_ADDA_MTKAIFV4_TX_CFG0 */
pub const MTKAIFV4_TXIF_EN_SEL_SFT: u32 = 12;
pub const MTKAIFV4_TXIF_EN_SEL_MASK: u32 = 0x1;
pub const MTKAIFV4_TXIF_EN_SEL_MASK_SFT: u32 = 0x1 << 12;
pub const MTKAIFV4_TXIF_V4_SFT: u32 = 11;
pub const MTKAIFV4_TXIF_V4_MASK: u32 = 0x1;
pub const MTKAIFV4_TXIF_V4_MASK_SFT: u32 = 0x1 << 11;
pub const MTKAIFV4_ADDA6_OUT_EN_SEL_SFT: u32 = 10;
pub const MTKAIFV4_ADDA6_OUT_EN_SEL_MASK: u32 = 0x1;
pub const MTKAIFV4_ADDA6_OUT_EN_SEL_MASK_SFT: u32 = 0x1 << 10;
pub const MTKAIFV4_ADDA_OUT_EN_SEL_SFT: u32 = 9;
pub const MTKAIFV4_ADDA_OUT_EN_SEL_MASK: u32 = 0x1;
pub const MTKAIFV4_ADDA_OUT_EN_SEL_MASK_SFT: u32 = 0x1 << 9;
pub const MTKAIFV4_TXIF_INPUT_MODE_SFT: u32 = 4;
pub const MTKAIFV4_TXIF_INPUT_MODE_MASK: u32 = 0x1f;
pub const MTKAIFV4_TXIF_INPUT_MODE_MASK_SFT: u32 = 0x1f << 4;
pub const MTKAIFV4_TXIF_FOUR_CHANNEL_SFT: u32 = 1;
pub const MTKAIFV4_TXIF_FOUR_CHANNEL_MASK: u32 = 0x1;
pub const MTKAIFV4_TXIF_FOUR_CHANNEL_MASK_SFT: u32 = 0x1 << 1;
pub const MTKAIFV4_TXIF_AFE_ON_SFT: u32 = 0;
pub const MTKAIFV4_TXIF_AFE_ON_MASK: u32 = 0x1;
pub const MTKAIFV4_TXIF_AFE_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_ADDA6_MTKAIFV4_TX_CFG0 */
pub const ADDA6_MTKAIFV4_TXIF_EN_SEL_SFT: u32 = 12;
pub const ADDA6_MTKAIFV4_TXIF_EN_SEL_MASK: u32 = 0x1;
pub const ADDA6_MTKAIFV4_TXIF_EN_SEL_MASK_SFT: u32 = 0x1 << 12;
pub const ADDA6_MTKAIFV4_TXIF_INPUT_MODE_SFT: u32 = 4;
pub const ADDA6_MTKAIFV4_TXIF_INPUT_MODE_MASK: u32 = 0x1f;
pub const ADDA6_MTKAIFV4_TXIF_INPUT_MODE_MASK_SFT: u32 = 0x1f << 4;
pub const ADDA6_MTKAIFV4_TXIF_FOUR_CHANNEL_SFT: u32 = 1;
pub const ADDA6_MTKAIFV4_TXIF_FOUR_CHANNEL_MASK: u32 = 0x1;
pub const ADDA6_MTKAIFV4_TXIF_FOUR_CHANNEL_MASK_SFT: u32 = 0x1 << 1;
pub const ADDA6_MTKAIFV4_TXIF_AFE_ON_SFT: u32 = 0;
pub const ADDA6_MTKAIFV4_TXIF_AFE_ON_MASK: u32 = 0x1;
pub const ADDA6_MTKAIFV4_TXIF_AFE_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_ADDA_MTKAIFV4_RX_CFG0 */
pub const MTKAIFV4_RXIF_CLKINV_SFT: u32 = 31;
pub const MTKAIFV4_RXIF_CLKINV_MASK: u32 = 0x1;
pub const MTKAIFV4_RXIF_CLKINV_MASK_SFT: u32 = 0x1 << 31;
pub const MTKAIFV4_RXIF_LOOPBACK_MODE_SFT: u32 = 28;
pub const MTKAIFV4_RXIF_LOOPBACK_MODE_MASK: u32 = 0x1;
pub const MTKAIFV4_RXIF_LOOPBACK_MODE_MASK_SFT: u32 = 0x1 << 28;
pub const MTKAIFV4_UL_CH7CH8_IN_EN_SEL_SFT: u32 = 19;
pub const MTKAIFV4_UL_CH7CH8_IN_EN_SEL_MASK: u32 = 0x1;
pub const MTKAIFV4_UL_CH7CH8_IN_EN_SEL_MASK_SFT: u32 = 0x1 << 19;
pub const MTKAIFV4_UL_CH5CH6_IN_EN_SEL_SFT: u32 = 18;
pub const MTKAIFV4_UL_CH5CH6_IN_EN_SEL_MASK: u32 = 0x1;
pub const MTKAIFV4_UL_CH5CH6_IN_EN_SEL_MASK_SFT: u32 = 0x1 << 18;
pub const MTKAIFV4_UL_CH3CH4_IN_EN_SEL_SFT: u32 = 17;
pub const MTKAIFV4_UL_CH3CH4_IN_EN_SEL_MASK: u32 = 0x1;
pub const MTKAIFV4_UL_CH3CH4_IN_EN_SEL_MASK_SFT: u32 = 0x1 << 17;
pub const MTKAIFV4_UL_CH1CH2_IN_EN_SEL_SFT: u32 = 16;
pub const MTKAIFV4_UL_CH1CH2_IN_EN_SEL_MASK: u32 = 0x1;
pub const MTKAIFV4_UL_CH1CH2_IN_EN_SEL_MASK_SFT: u32 = 0x1 << 16;
pub const MTKAIFV4_RXIF_EN_SEL_SFT: u32 = 12;
pub const MTKAIFV4_RXIF_EN_SEL_MASK: u32 = 0x1;
pub const MTKAIFV4_RXIF_EN_SEL_MASK_SFT: u32 = 0x1 << 12;
pub const MTKAIFV4_RXIF_INPUT_MODE_SFT: u32 = 4;
pub const MTKAIFV4_RXIF_INPUT_MODE_MASK: u32 = 0x1f;
pub const MTKAIFV4_RXIF_INPUT_MODE_MASK_SFT: u32 = 0x1f << 4;
pub const MTKAIFV4_RXIF_FOUR_CHANNEL_SFT: u32 = 1;
pub const MTKAIFV4_RXIF_FOUR_CHANNEL_MASK: u32 = 0x1;
pub const MTKAIFV4_RXIF_FOUR_CHANNEL_MASK_SFT: u32 = 0x1 << 1;
pub const MTKAIFV4_RXIF_AFE_ON_SFT: u32 = 0;
pub const MTKAIFV4_RXIF_AFE_ON_MASK: u32 = 0x1;
pub const MTKAIFV4_RXIF_AFE_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_ADDA_MTKAIFV4_RX_CFG1 */
pub const MTKAIFV4_RXIF_SYNC_CNT_TABLE_SFT: u32 = 17;
pub const MTKAIFV4_RXIF_SYNC_CNT_TABLE_MASK: u32 = 0xfff;
pub const MTKAIFV4_RXIF_SYNC_CNT_TABLE_MASK_SFT: u32 = 0xfff << 17;
pub const MTKAIFV4_RXIF_SYNC_SEARCH_TABLE_SFT: u32 = 12;
pub const MTKAIFV4_RXIF_SYNC_SEARCH_TABLE_MASK: u32 = 0x1f;
pub const MTKAIFV4_RXIF_SYNC_SEARCH_TABLE_MASK_SFT: u32 = 0x1f << 12;
pub const MTKAIFV4_RXIF_INVAILD_SYNC_CHECK_ROUND_SFT: u32 = 8;
pub const MTKAIFV4_RXIF_INVAILD_SYNC_CHECK_ROUND_MASK: u32 = 0xf;
pub const MTKAIFV4_RXIF_INVAILD_SYNC_CHECK_ROUND_MASK_SFT: u32 = 0xf << 8;
pub const MTKAIFV4_RXIF_SYNC_CHECK_ROUND_SFT: u32 = 4;
pub const MTKAIFV4_RXIF_SYNC_CHECK_ROUND_MASK: u32 = 0xf;
pub const MTKAIFV4_RXIF_SYNC_CHECK_ROUND_MASK_SFT: u32 = 0xf << 4;
pub const MTKAIFV4_RXIF_FIFO_RSP_SFT: u32 = 1;
pub const MTKAIFV4_RXIF_FIFO_RSP_MASK: u32 = 0x7;
pub const MTKAIFV4_RXIF_FIFO_RSP_MASK_SFT: u32 = 0x7 << 1;
pub const MTKAIFV4_RXIF_SELF_DEFINE_TABLE_SFT: u32 = 0;
pub const MTKAIFV4_RXIF_SELF_DEFINE_TABLE_MASK: u32 = 0x1;
pub const MTKAIFV4_RXIF_SELF_DEFINE_TABLE_MASK_SFT: u32 = 0x1 << 0;

/* AFE_ADDA6_MTKAIFV4_RX_CFG0 */
pub const ADDA6_MTKAIFV4_RXIF_CLKINV_SFT: u32 = 31;
pub const ADDA6_MTKAIFV4_RXIF_CLKINV_MASK: u32 = 0x1;
pub const ADDA6_MTKAIFV4_RXIF_CLKINV_MASK_SFT: u32 = 0x1 << 31;
pub const ADDA6_MTKAIFV4_RXIF_LOOPBACK_MODE_SFT: u32 = 28;
pub const ADDA6_MTKAIFV4_RXIF_LOOPBACK_MODE_MASK: u32 = 0x1;
pub const ADDA6_MTKAIFV4_RXIF_LOOPBACK_MODE_MASK_SFT: u32 = 0x1 << 28;
pub const ADDA6_MTKAIFV4_RXIF_EN_SEL_SFT: u32 = 12;
pub const ADDA6_MTKAIFV4_RXIF_EN_SEL_MASK: u32 = 0x1;
pub const ADDA6_MTKAIFV4_RXIF_EN_SEL_MASK_SFT: u32 = 0x1 << 12;
pub const ADDA6_MTKAIFV4_RXIF_INPUT_MODE_SFT: u32 = 4;
pub const ADDA6_MTKAIFV4_RXIF_INPUT_MODE_MASK: u32 = 0x1f;
pub const ADDA6_MTKAIFV4_RXIF_INPUT_MODE_MASK_SFT: u32 = 0x1f << 4;
pub const ADDA6_MTKAIFV4_RXIF_FOUR_CHANNEL_SFT: u32 = 1;
pub const ADDA6_MTKAIFV4_RXIF_FOUR_CHANNEL_MASK: u32 = 0x1;
pub const ADDA6_MTKAIFV4_RXIF_FOUR_CHANNEL_MASK_SFT: u32 = 0x1 << 1;
pub const ADDA6_MTKAIFV4_RXIF_AFE_ON_SFT: u32 = 0;
pub const ADDA6_MTKAIFV4_RXIF_AFE_ON_MASK: u32 = 0x1;
pub const ADDA6_MTKAIFV4_RXIF_AFE_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_ADDA6_MTKAIFV4_RX_CFG1 */
pub const ADDA6_MTKAIFV4_RXIF_SYNC_CNT_TABLE_SFT: u32 = 17;
pub const ADDA6_MTKAIFV4_RXIF_SYNC_CNT_TABLE_MASK: u32 = 0xfff;
pub const ADDA6_MTKAIFV4_RXIF_SYNC_CNT_TABLE_MASK_SFT: u32 = 0xfff << 17;
pub const ADDA6_MTKAIFV4_RXIF_SYNC_SEARCH_TABLE_SFT: u32 = 12;
pub const ADDA6_MTKAIFV4_RXIF_SYNC_SEARCH_TABLE_MASK: u32 = 0x1f;
pub const ADDA6_MTKAIFV4_RXIF_SYNC_SEARCH_TABLE_MASK_SFT: u32 = 0x1f << 12;
pub const ADDA6_MTKAIFV4_RXIF_INVAILD_SYNC_CHECK_ROUND_SFT: u32 = 8;
pub const ADDA6_MTKAIFV4_RXIF_INVAILD_SYNC_CHECK_ROUND_MASK: u32 = 0xf;
pub const ADDA6_MTKAIFV4_RXIF_INVAILD_SYNC_CHECK_ROUND_MASK_SFT: u32 = 0xf << 8;
pub const ADDA6_MTKAIFV4_RXIF_SYNC_CHECK_ROUND_SFT: u32 = 4;
pub const ADDA6_MTKAIFV4_RXIF_SYNC_CHECK_ROUND_MASK: u32 = 0xf;
pub const ADDA6_MTKAIFV4_RXIF_SYNC_CHECK_ROUND_MASK_SFT: u32 = 0xf << 4;
pub const ADDA6_MTKAIFV4_RXIF_FIFO_RSP_SFT: u32 = 1;
pub const ADDA6_MTKAIFV4_RXIF_FIFO_RSP_MASK: u32 = 0x7;
pub const ADDA6_MTKAIFV4_RXIF_FIFO_RSP_MASK_SFT: u32 = 0x7 << 1;
pub const ADDA6_MTKAIFV4_RXIF_SELF_DEFINE_TABLE_SFT: u32 = 0;
pub const ADDA6_MTKAIFV4_RXIF_SELF_DEFINE_TABLE_MASK: u32 = 0x1;
pub const ADDA6_MTKAIFV4_RXIF_SELF_DEFINE_TABLE_MASK_SFT: u32 = 0x1 << 0;

/* AFE_ADDA_MTKAIFV4_TX_SYNCWORD_CFG */
pub const ADDA6_MTKAIFV4_TXIF_SYNCWORD_SFT: u32 = 16;
pub const ADDA6_MTKAIFV4_TXIF_SYNCWORD_MASK: u32 = 0xffff;
pub const ADDA6_MTKAIFV4_TXIF_SYNCWORD_MASK_SFT: u32 = 0xffff << 16;
pub const ADDA_MTKAIFV4_TXIF_SYNCWORD_SFT: u32 = 0;
pub const ADDA_MTKAIFV4_TXIF_SYNCWORD_MASK: u32 = 0xffff;
pub const ADDA_MTKAIFV4_TXIF_SYNCWORD_MASK_SFT: u32 = 0xffff << 0;

/* AFE_ADDA_MTKAIFV4_RX_SYNCWORD_CFG */
pub const ADDA6_MTKAIFV4_RXIF_SYNCWORD_SFT: u32 = 16;
pub const ADDA6_MTKAIFV4_RXIF_SYNCWORD_MASK: u32 = 0xffff;
pub const ADDA6_MTKAIFV4_RXIF_SYNCWORD_MASK_SFT: u32 = 0xffff << 16;
pub const ADDA_MTKAIFV4_RXIF_SYNCWORD_SFT: u32 = 0;
pub const ADDA_MTKAIFV4_RXIF_SYNCWORD_MASK: u32 = 0xffff;
pub const ADDA_MTKAIFV4_RXIF_SYNCWORD_MASK_SFT: u32 = 0xffff << 0;

/* AFE_ADDA_MTKAIFV4_MON0 */
pub const MTKAIFV4_TXIF_SDATA_OUT_SFT: u32 = 23;
pub const MTKAIFV4_TXIF_SDATA_OUT_MASK: u32 = 0x1;
pub const MTKAIFV4_TXIF_SDATA_OUT_MASK_SFT: u32 = 0x1 << 23;
pub const MTKAIFV4_RXIF_SDATA_IN_SFT: u32 = 22;
pub const MTKAIFV4_RXIF_SDATA_IN_MASK: u32 = 0x1;
pub const MTKAIFV4_RXIF_SDATA_IN_MASK_SFT: u32 = 0x1 << 22;
pub const MTKAIFV4_RXIF_SEARCH_FAIL_FLAG_SFT: u32 = 21;
pub const MTKAIFV4_RXIF_SEARCH_FAIL_FLAG_MASK: u32 = 0x1;
pub const MTKAIFV4_RXIF_SEARCH_FAIL_FLAG_MASK_SFT: u32 = 0x1 << 21;
pub const MTKAIFV4_RXIF_ADC_FIFO_STATUS_SFT: u32 = 0;
pub const MTKAIFV4_RXIF_ADC_FIFO_STATUS_MASK: u32 = 0xfff;
pub const MTKAIFV4_RXIF_ADC_FIFO_STATUS_MASK_SFT: u32 = 0xfff << 0;

/* AFE_ADDA_MTKAIFV4_MON1 */
pub const MTKAIFV4_RXIF_OUT_CH4_SFT: u32 = 24;
pub const MTKAIFV4_RXIF_OUT_CH4_MASK: u32 = 0xff;
pub const MTKAIFV4_RXIF_OUT_CH4_MASK_SFT: u32 = 0xff << 24;
pub const MTKAIFV4_RXIF_OUT_CH3_SFT: u32 = 16;
pub const MTKAIFV4_RXIF_OUT_CH3_MASK: u32 = 0xff;
pub const MTKAIFV4_RXIF_OUT_CH3_MASK_SFT: u32 = 0xff << 16;
pub const MTKAIFV4_RXIF_OUT_CH2_SFT: u32 = 8;
pub const MTKAIFV4_RXIF_OUT_CH2_MASK: u32 = 0xff;
pub const MTKAIFV4_RXIF_OUT_CH2_MASK_SFT: u32 = 0xff << 8;
pub const MTKAIFV4_RXIF_OUT_CH1_SFT: u32 = 0;
pub const MTKAIFV4_RXIF_OUT_CH1_MASK: u32 = 0xff;
pub const MTKAIFV4_RXIF_OUT_CH1_MASK_SFT: u32 = 0xff << 0;

/* AFE_ADDA6_MTKAIFV4_MON0 */
pub const ADDA6_MTKAIFV4_TXIF_SDATA_OUT_SFT: u32 = 23;
pub const ADDA6_MTKAIFV4_TXIF_SDATA_OUT_MASK: u32 = 0x1;
pub const ADDA6_MTKAIFV4_TXIF_SDATA_OUT_MASK_SFT: u32 = 0x1 << 23;
pub const ADDA6_MTKAIFV4_RXIF_SDATA_IN_SFT: u32 = 22;
pub const ADDA6_MTKAIFV4_RXIF_SDATA_IN_MASK: u32 = 0x1;
pub const ADDA6_MTKAIFV4_RXIF_SDATA_IN_MASK_SFT: u32 = 0x1 << 22;
pub const ADDA6_MTKAIFV4_RXIF_SEARCH_FAIL_FLAG_SFT: u32 = 21;
pub const ADDA6_MTKAIFV4_RXIF_SEARCH_FAIL_FLAG_MASK: u32 = 0x1;
pub const ADDA6_MTKAIFV4_RXIF_SEARCH_FAIL_FLAG_MASK_SFT: u32 = 0x1 << 21;
pub const ADDA6_MTKAIFV3P3_RXIF_ADC_FIFO_STATUS_SFT: u32 = 0;
pub const ADDA6_MTKAIFV3P3_RXIF_ADC_FIFO_STATUS_MASK: u32 = 0xfff;
pub const ADDA6_MTKAIFV3P3_RXIF_ADC_FIFO_STATUS_MASK_SFT: u32 = 0xfff << 0;

/* ETDM_IN0_CON0 */
/* ETDM_IN1_CON0 */
pub const REG_ETDM_IN_EN_SFT: u32 = 0;
pub const REG_ETDM_IN_EN_MASK: u32 = 0x1;
pub const REG_ETDM_IN_EN_MASK_SFT: u32 = 0x1 << 0;
pub const REG_SYNC_MODE_SFT: u32 = 1;
pub const REG_SYNC_MODE_MASK: u32 = 0x1;
pub const REG_SYNC_MODE_MASK_SFT: u32 = 0x1 << 1;
pub const REG_LSB_FIRST_SFT: u32 = 3;
pub const REG_LSB_FIRST_MASK: u32 = 0x1;
pub const REG_LSB_FIRST_MASK_SFT: u32 = 0x1 << 3;
pub const REG_SOFT_RST_SFT: u32 = 4;
pub const REG_SOFT_RST_MASK: u32 = 0x1;
pub const REG_SOFT_RST_MASK_SFT: u32 = 0x1 << 4;
pub const REG_SLAVE_MODE_SFT: u32 = 5;
pub const REG_SLAVE_MODE_MASK: u32 = 0x1;
pub const REG_SLAVE_MODE_MASK_SFT: u32 = 0x1 << 5;
pub const REG_FMT_SFT: u32 = 6;
pub const REG_FMT_MASK: u32 = 0x7;
pub const REG_FMT_MASK_SFT: u32 = 0x7 << 6;
pub const REG_LRCK_EDGE_SEL_SFT: u32 = 10;
pub const REG_LRCK_EDGE_SEL_MASK: u32 = 0x1;
pub const REG_LRCK_EDGE_SEL_MASK_SFT: u32 = 0x1 << 10;
pub const REG_BIT_LENGTH_SFT: u32 = 11;
pub const REG_BIT_LENGTH_MASK: u32 = 0x1f;
pub const REG_BIT_LENGTH_MASK_SFT: u32 = 0x1f << 11;
pub const REG_WORD_LENGTH_SFT: u32 = 16;
pub const REG_WORD_LENGTH_MASK: u32 = 0x1f;
pub const REG_WORD_LENGTH_MASK_SFT: u32 = 0x1f << 16;
pub const REG_CH_NUM_SFT: u32 = 23;
pub const REG_CH_NUM_MASK: u32 = 0x1f;
pub const REG_CH_NUM_MASK_SFT: u32 = 0x1f << 23;
pub const REG_RELATCH_1X_EN_DOMAIN_SEL_SFT: u32 = 28;
pub const REG_RELATCH_1X_EN_DOMAIN_SEL_MASK: u32 = 0x7;
pub const REG_RELATCH_1X_EN_DOMAIN_SEL_MASK_SFT: u32 = 0x7 << 28;
pub const REG_VALID_TOGETHER_SFT: u32 = 31;
pub const REG_VALID_TOGETHER_MASK: u32 = 0x1;
pub const REG_VALID_TOGETHER_MASK_SFT: u32 = 0x1 << 31;

/* ETDM_IN0_CON1 */
/* ETDM_IN1_CON1 */
pub const REG_INITIAL_COUNT_SFT: u32 = 0;
pub const REG_INITIAL_COUNT_MASK: u32 = 0x1f;
pub const REG_INITIAL_COUNT_MASK_SFT: u32 = 0x1f << 0;
pub const REG_INITIAL_POINT_SFT: u32 = 5;
pub const REG_INITIAL_POINT_MASK: u32 = 0x1f;
pub const REG_INITIAL_POINT_MASK_SFT: u32 = 0x1f << 5;
pub const REG_LRCK_AUTO_OFF_SFT: u32 = 10;
pub const REG_LRCK_AUTO_OFF_MASK: u32 = 0x1;
pub const REG_LRCK_AUTO_OFF_MASK_SFT: u32 = 0x1 << 10;
pub const REG_BCK_AUTO_OFF_SFT: u32 = 11;
pub const REG_BCK_AUTO_OFF_MASK: u32 = 0x1;
pub const REG_BCK_AUTO_OFF_MASK_SFT: u32 = 0x1 << 11;
pub const REG_INITIAL_LRCK_SFT: u32 = 13;
pub const REG_INITIAL_LRCK_MASK: u32 = 0x1;
pub const REG_INITIAL_LRCK_MASK_SFT: u32 = 0x1 << 13;
pub const REG_NO_ALIGN_1X_EN_SFT: u32 = 14;
pub const REG_NO_ALIGN_1X_EN_MASK: u32 = 0x1;
pub const REG_NO_ALIGN_1X_EN_MASK_SFT: u32 = 0x1 << 14;
pub const REG_LRCK_RESET_SFT: u32 = 15;
pub const REG_LRCK_RESET_MASK: u32 = 0x1;
pub const REG_LRCK_RESET_MASK_SFT: u32 = 0x1 << 15;
pub const PINMUX_MCLK_CTRL_OE_SFT: u32 = 16;
pub const PINMUX_MCLK_CTRL_OE_MASK: u32 = 0x1;
pub const PINMUX_MCLK_CTRL_OE_MASK_SFT: u32 = 0x1 << 16;
pub const REG_OUTPUT_CR_EN_SFT: u32 = 18;
pub const REG_OUTPUT_CR_EN_MASK: u32 = 0x1;
pub const REG_OUTPUT_CR_EN_MASK_SFT: u32 = 0x1 << 18;
pub const REG_LR_ALIGN_SFT: u32 = 19;
pub const REG_LR_ALIGN_MASK: u32 = 0x1;
pub const REG_LR_ALIGN_MASK_SFT: u32 = 0x1 << 19;
pub const REG_LRCK_WIDTH_SFT: u32 = 20;
pub const REG_LRCK_WIDTH_MASK: u32 = 0x3ff;
pub const REG_LRCK_WIDTH_MASK_SFT: u32 = 0x3ff << 20;
pub const REG_DIRECT_INPUT_MASTER_BCK_SFT: u32 = 30;
pub const REG_DIRECT_INPUT_MASTER_BCK_MASK: u32 = 0x1;
pub const REG_DIRECT_INPUT_MASTER_BCK_MASK_SFT: u32 = 0x1 << 30;
pub const REG_LRCK_AUTO_MODE_SFT: u32 = 31;
pub const REG_LRCK_AUTO_MODE_MASK: u32 = 0x1;
pub const REG_LRCK_AUTO_MODE_MASK_SFT: u32 = 0x1 << 31;

/* ETDM_IN0_CON2 */
/* ETDM_IN1_CON2 */
pub const REG_UPDATE_POINT_SFT: u32 = 0;
pub const REG_UPDATE_POINT_MASK: u32 = 0x1f;
pub const REG_UPDATE_POINT_MASK_SFT: u32 = 0x1f << 0;
pub const REG_UPDATE_GAP_SFT: u32 = 5;
pub const REG_UPDATE_GAP_MASK: u32 = 0x1f;
pub const REG_UPDATE_GAP_MASK_SFT: u32 = 0x1f << 5;
pub const REG_CLOCK_SOURCE_SEL_SFT: u32 = 10;
pub const REG_CLOCK_SOURCE_SEL_MASK: u32 = 0x7;
pub const REG_CLOCK_SOURCE_SEL_MASK_SFT: u32 = 0x7 << 10;
pub const REG_CK_EN_SEL_AUTO_SFT: u32 = 14;
pub const REG_CK_EN_SEL_AUTO_MASK: u32 = 0x1;
pub const REG_CK_EN_SEL_AUTO_MASK_SFT: u32 = 0x1 << 14;
pub const REG_MULTI_IP_TOTAL_CHNUM_SFT: u32 = 15;
pub const REG_MULTI_IP_TOTAL_CHNUM_MASK: u32 = 0x1f;
pub const REG_MULTI_IP_TOTAL_CHNUM_MASK_SFT: u32 = 0x1f << 15;
pub const REG_MASK_AUTO_SFT: u32 = 20;
pub const REG_MASK_AUTO_MASK: u32 = 0x1;
pub const REG_MASK_AUTO_MASK_SFT: u32 = 0x1 << 20;
pub const REG_MASK_NUM_SFT: u32 = 21;
pub const REG_MASK_NUM_MASK: u32 = 0x1f;
pub const REG_MASK_NUM_MASK_SFT: u32 = 0x1f << 21;
pub const REG_UPDATE_POINT_AUTO_SFT: u32 = 26;
pub const REG_UPDATE_POINT_AUTO_MASK: u32 = 0x1;
pub const REG_UPDATE_POINT_AUTO_MASK_SFT: u32 = 0x1 << 26;
pub const REG_SDATA_DELAY_0P5T_EN_SFT: u32 = 27;
pub const REG_SDATA_DELAY_0P5T_EN_MASK: u32 = 0x1;
pub const REG_SDATA_DELAY_0P5T_EN_MASK_SFT: u32 = 0x1 << 27;
pub const REG_SDATA_DELAY_BCK_INV_SFT: u32 = 28;
pub const REG_SDATA_DELAY_BCK_INV_MASK: u32 = 0x1;
pub const REG_SDATA_DELAY_BCK_INV_MASK_SFT: u32 = 0x1 << 28;
pub const REG_LRCK_DELAY_0P5T_EN_SFT: u32 = 29;
pub const REG_LRCK_DELAY_0P5T_EN_MASK: u32 = 0x1;
pub const REG_LRCK_DELAY_0P5T_EN_MASK_SFT: u32 = 0x1 << 29;
pub const REG_LRCK_DELAY_BCK_INV_SFT: u32 = 30;
pub const REG_LRCK_DELAY_BCK_INV_MASK: u32 = 0x1;
pub const REG_LRCK_DELAY_BCK_INV_MASK_SFT: u32 = 0x1 << 30;
pub const REG_MULTI_IP_MODE_SFT: u32 = 31;
pub const REG_MULTI_IP_MODE_MASK: u32 = 0x1;
pub const REG_MULTI_IP_MODE_MASK_SFT: u32 = 0x1 << 31;

/* ETDM_IN0_CON3 */
/* ETDM_IN1_CON3 */
pub const REG_DISABLE_OUT_SFT: u32 = 0;
pub const REG_DISABLE_OUT_MASK: u32 = 0xffff;
pub const REG_DISABLE_OUT_MASK_SFT: u32 = 0xffff << 0;
pub const REG_RJ_DATA_RIGHT_ALIGN_SFT: u32 = 16;
pub const REG_RJ_DATA_RIGHT_ALIGN_MASK: u32 = 0x1;
pub const REG_RJ_DATA_RIGHT_ALIGN_MASK_SFT: u32 = 0x1 << 16;
pub const REG_MONITOR_SEL_SFT: u32 = 17;
pub const REG_MONITOR_SEL_MASK: u32 = 0x3;
pub const REG_MONITOR_SEL_MASK_SFT: u32 = 0x3 << 17;
pub const REG_CNT_UPPER_LIMIT_SFT: u32 = 19;
pub const REG_CNT_UPPER_LIMIT_MASK: u32 = 0x3f;
pub const REG_CNT_UPPER_LIMIT_MASK_SFT: u32 = 0x3f << 19;
pub const REG_COMPACT_SAMPLE_END_DIS_SFT: u32 = 25;
pub const REG_COMPACT_SAMPLE_END_DIS_MASK: u32 = 0x1;
pub const REG_COMPACT_SAMPLE_END_DIS_MASK_SFT: u32 = 0x1 << 25;
pub const REG_FS_TIMING_SEL_SFT: u32 = 26;
pub const REG_FS_TIMING_SEL_MASK: u32 = 0x1f;
pub const REG_FS_TIMING_SEL_MASK_SFT: u32 = 0x1f << 26;
pub const REG_SAMPLE_END_MODE_SFT: u32 = 31;
pub const REG_SAMPLE_END_MODE_MASK: u32 = 0x1;
pub const REG_SAMPLE_END_MODE_MASK_SFT: u32 = 0x1 << 31;

/* ETDM_IN0_CON4 */
/* ETDM_IN1_CON4 */
pub const REG_ALWAYS_OPEN_1X_EN_SFT: u32 = 31;
pub const REG_ALWAYS_OPEN_1X_EN_MASK: u32 = 0x1;
pub const REG_ALWAYS_OPEN_1X_EN_MASK_SFT: u32 = 0x1 << 31;
pub const REG_WAIT_LAST_SAMPLE_SFT: u32 = 30;
pub const REG_WAIT_LAST_SAMPLE_MASK: u32 = 0x1;
pub const REG_WAIT_LAST_SAMPLE_MASK_SFT: u32 = 0x1 << 30;
pub const REG_SAMPLE_END_POINT_SFT: u32 = 25;
pub const REG_SAMPLE_END_POINT_MASK: u32 = 0x1f;
pub const REG_SAMPLE_END_POINT_MASK_SFT: u32 = 0x1f << 25;
pub const REG_RELATCH_1X_EN_SEL_SFT: u32 = 20;
pub const REG_RELATCH_1X_EN_SEL_MASK: u32 = 0x1f;
pub const REG_RELATCH_1X_EN_SEL_MASK_SFT: u32 = 0x1f << 20;
pub const REG_MASTER_WS_INV_SFT: u32 = 19;
pub const REG_MASTER_WS_INV_MASK: u32 = 0x1;
pub const REG_MASTER_WS_INV_MASK_SFT: u32 = 0x1 << 19;
pub const REG_MASTER_BCK_INV_SFT: u32 = 18;
pub const REG_MASTER_BCK_INV_MASK: u32 = 0x1;
pub const REG_MASTER_BCK_INV_MASK_SFT: u32 = 0x1 << 18;
pub const REG_SLAVE_LRCK_INV_SFT: u32 = 17;
pub const REG_SLAVE_LRCK_INV_MASK: u32 = 0x1;
pub const REG_SLAVE_LRCK_INV_MASK_SFT: u32 = 0x1 << 17;
pub const REG_SLAVE_BCK_INV_SFT: u32 = 16;
pub const REG_SLAVE_BCK_INV_MASK: u32 = 0x1;
pub const REG_SLAVE_BCK_INV_MASK_SFT: u32 = 0x1 << 16;
pub const REG_REPACK_CHNUM_SFT: u32 = 12;
pub const REG_REPACK_CHNUM_MASK: u32 = 0xf;
pub const REG_REPACK_CHNUM_MASK_SFT: u32 = 0xf << 12;
pub const REG_ASYNC_RESET_SFT: u32 = 11;
pub const REG_ASYNC_RESET_MASK: u32 = 0x1;
pub const REG_ASYNC_RESET_MASK_SFT: u32 = 0x1 << 11;
pub const REG_REPACK_WORD_LENGTH_SFT: u32 = 9;
pub const REG_REPACK_WORD_LENGTH_MASK: u32 = 0x3;
pub const REG_REPACK_WORD_LENGTH_MASK_SFT: u32 = 0x3 << 9;
pub const REG_REPACK_AUTO_MODE_SFT: u32 = 8;
pub const REG_REPACK_AUTO_MODE_MASK: u32 = 0x1;
pub const REG_REPACK_AUTO_MODE_MASK_SFT: u32 = 0x1 << 8;
pub const REG_REPACK_MODE_SFT: u32 = 0;
pub const REG_REPACK_MODE_MASK: u32 = 0x3f;
pub const REG_REPACK_MODE_MASK_SFT: u32 = 0x3f << 0;

/* ETDM_IN0_CON5 */
/* ETDM_IN1_CON5 */
pub const REG_LR_SWAP_SFT: u32 = 16;
pub const REG_LR_SWAP_MASK: u32 = 0xffff;
pub const REG_LR_SWAP_MASK_SFT: u32 = 0xffff << 16;
pub const REG_ODD_FLAG_EN_SFT: u32 = 0;
pub const REG_ODD_FLAG_EN_MASK: u32 = 0xffff;
pub const REG_ODD_FLAG_EN_MASK_SFT: u32 = 0xffff << 0;

/* ETDM_IN0_CON6 */
/* ETDM_IN1_CON6 */
pub const LCH_DATA_REG_SFT: u32 = 0;
pub const LCH_DATA_REG_MASK: u32 = 0xffffffff;
pub const LCH_DATA_REG_MASK_SFT: u32 = 0xffffffff << 0;

/* ETDM_IN0_CON7 */
/* ETDM_IN1_CON7 */
pub const RCH_DATA_REG_SFT: u32 = 0;
pub const RCH_DATA_REG_MASK: u32 = 0xffffffff;
pub const RCH_DATA_REG_MASK_SFT: u32 = 0xffffffff << 0;

/* ETDM_IN0_CON8 */
/* ETDM_IN1_CON8 */
pub const REG_AFIFO_THRESHOLD_SFT: u32 = 29;
pub const REG_AFIFO_THRESHOLD_MASK: u32 = 0x3;
pub const REG_AFIFO_THRESHOLD_MASK_SFT: u32 = 0x3 << 29;
pub const REG_CK_EN_SEL_MANUAL_SFT: u32 = 16;
pub const REG_CK_EN_SEL_MANUAL_MASK: u32 = 0x3ff;
pub const REG_CK_EN_SEL_MANUAL_MASK_SFT: u32 = 0x3ff << 16;
pub const REG_AFIFO_SW_RESET_SFT: u32 = 15;
pub const REG_AFIFO_SW_RESET_MASK: u32 = 0x1;
pub const REG_AFIFO_SW_RESET_MASK_SFT: u32 = 0x1 << 15;
pub const REG_AFIFO_RESET_SEL_SFT: u32 = 14;
pub const REG_AFIFO_RESET_SEL_MASK: u32 = 0x1;
pub const REG_AFIFO_RESET_SEL_MASK_SFT: u32 = 0x1 << 14;
pub const REG_AFIFO_AUTO_RESET_DIS_SFT: u32 = 9;
pub const REG_AFIFO_AUTO_RESET_DIS_MASK: u32 = 0x1;
pub const REG_AFIFO_AUTO_RESET_DIS_MASK_SFT: u32 = 0x1 << 9;
pub const REG_ETDM_USE_AFIFO_SFT: u32 = 8;
pub const REG_ETDM_USE_AFIFO_MASK: u32 = 0x1;
pub const REG_ETDM_USE_AFIFO_MASK_SFT: u32 = 0x1 << 8;
pub const REG_AFIFO_CLOCK_DOMAIN_SEL_SFT: u32 = 5;
pub const REG_AFIFO_CLOCK_DOMAIN_SEL_MASK: u32 = 0x7;
pub const REG_AFIFO_CLOCK_DOMAIN_SEL_MASK_SFT: u32 = 0x7 << 5;
pub const REG_AFIFO_MODE_SFT: u32 = 0;
pub const REG_AFIFO_MODE_MASK: u32 = 0x1f;
pub const REG_AFIFO_MODE_MASK_SFT: u32 = 0x1f << 0;

/* ETDM_IN0_CON9 */
/* ETDM_IN1_CON9 */
pub const REG_OUT2LATCH_TIME_SFT: u32 = 10;
pub const REG_OUT2LATCH_TIME_MASK: u32 = 0x1f;
pub const REG_OUT2LATCH_TIME_MASK_SFT: u32 = 0x1f << 10;
pub const REG_ALMOST_END_BIT_COUNT_SFT: u32 = 5;
pub const REG_ALMOST_END_BIT_COUNT_MASK: u32 = 0x1f;
pub const REG_ALMOST_END_BIT_COUNT_MASK_SFT: u32 = 0x1f << 5;
pub const REG_ALMOST_END_CH_COUNT_SFT: u32 = 0;
pub const REG_ALMOST_END_CH_COUNT_MASK: u32 = 0x1f;
pub const REG_ALMOST_END_CH_COUNT_MASK_SFT: u32 = 0x1f << 0;

/* ETDM_IN0_MON */
/* ETDM_IN1_MON */
pub const LRCK_INV_SFT: u32 = 30;
pub const LRCK_INV_MASK: u32 = 0x1;
pub const LRCK_INV_MASK_SFT: u32 = 0x1 << 30;
pub const EN_SYNC_OUT_SFT: u32 = 29;
pub const EN_SYNC_OUT_MASK: u32 = 0x1;
pub const EN_SYNC_OUT_MASK_SFT: u32 = 0x1 << 29;
pub const HOPPING_EN_SYNC_OUT_PRE_SFT: u32 = 28;
pub const HOPPING_EN_SYNC_OUT_PRE_MASK: u32 = 0x1;
pub const HOPPING_EN_SYNC_OUT_PRE_MASK_SFT: u32 = 0x1 << 28;
pub const WFULL_SFT: u32 = 27;
pub const WFULL_MASK: u32 = 0x1;
pub const WFULL_MASK_SFT: u32 = 0x1 << 27;
pub const REMPTY_SFT: u32 = 26;
pub const REMPTY_MASK: u32 = 0x1;
pub const REMPTY_MASK_SFT: u32 = 0x1 << 26;
pub const ETDM_2X_CK_EN_SFT: u32 = 25;
pub const ETDM_2X_CK_EN_MASK: u32 = 0x1;
pub const ETDM_2X_CK_EN_MASK_SFT: u32 = 0x1 << 25;
pub const ETDM_1X_CK_EN_SFT: u32 = 24;
pub const ETDM_1X_CK_EN_MASK: u32 = 0x1;
pub const ETDM_1X_CK_EN_MASK_SFT: u32 = 0x1 << 24;
pub const SDATA0_SFT: u32 = 23;
pub const SDATA0_MASK: u32 = 0x1;
pub const SDATA0_MASK_SFT: u32 = 0x1 << 23;
pub const CURRENT_STATUS_SFT: u32 = 21;
pub const CURRENT_STATUS_MASK: u32 = 0x3;
pub const CURRENT_STATUS_MASK_SFT: u32 = 0x3 << 21;
pub const BIT_POINT_SFT: u32 = 16;
pub const BIT_POINT_MASK: u32 = 0x1f;
pub const BIT_POINT_MASK_SFT: u32 = 0x1f << 16;
pub const BIT_CH_COUNT_SFT: u32 = 10;
pub const BIT_CH_COUNT_MASK: u32 = 0x3f;
pub const BIT_CH_COUNT_MASK_SFT: u32 = 0x3f << 10;
pub const BIT_COUNT_SFT: u32 = 5;
pub const BIT_COUNT_MASK: u32 = 0x1f;
pub const BIT_COUNT_MASK_SFT: u32 = 0x1f << 5;
pub const CH_COUNT_SFT: u32 = 0;
pub const CH_COUNT_MASK: u32 = 0x1f;
pub const CH_COUNT_MASK_SFT: u32 = 0x1f << 0;

/* ETDM_OUT0_CON0 */
/* ETDM_OUT1_CON0 */
/* ETDM_OUT4_CON0 */
pub const OUT_REG_ETDM_OUT_EN_SFT: u32 = 0;
pub const OUT_REG_ETDM_OUT_EN_MASK: u32 = 0x1;
pub const OUT_REG_ETDM_OUT_EN_MASK_SFT: u32 = 0x1 << 0;
pub const OUT_REG_SYNC_MODE_SFT: u32 = 1;
pub const OUT_REG_SYNC_MODE_MASK: u32 = 0x1;
pub const OUT_REG_SYNC_MODE_MASK_SFT: u32 = 0x1 << 1;
pub const OUT_REG_LSB_FIRST_SFT: u32 = 3;
pub const OUT_REG_LSB_FIRST_MASK: u32 = 0x1;
pub const OUT_REG_LSB_FIRST_MASK_SFT: u32 = 0x1 << 3;
pub const OUT_REG_SOFT_RST_SFT: u32 = 4;
pub const OUT_REG_SOFT_RST_MASK: u32 = 0x1;
pub const OUT_REG_SOFT_RST_MASK_SFT: u32 = 0x1 << 4;
pub const OUT_REG_SLAVE_MODE_SFT: u32 = 5;
pub const OUT_REG_SLAVE_MODE_MASK: u32 = 0x1;
pub const OUT_REG_SLAVE_MODE_MASK_SFT: u32 = 0x1 << 5;
pub const OUT_REG_FMT_SFT: u32 = 6;
pub const OUT_REG_FMT_MASK: u32 = 0x7;
pub const OUT_REG_FMT_MASK_SFT: u32 = 0x7 << 6;
pub const OUT_REG_LRCK_EDGE_SEL_SFT: u32 = 10;
pub const OUT_REG_LRCK_EDGE_SEL_MASK: u32 = 0x1;
pub const OUT_REG_LRCK_EDGE_SEL_MASK_SFT: u32 = 0x1 << 10;
pub const OUT_REG_BIT_LENGTH_SFT: u32 = 11;
pub const OUT_REG_BIT_LENGTH_MASK: u32 = 0x1f;
pub const OUT_REG_BIT_LENGTH_MASK_SFT: u32 = 0x1f << 11;
pub const OUT_REG_WORD_LENGTH_SFT: u32 = 16;
pub const OUT_REG_WORD_LENGTH_MASK: u32 = 0x1f;
pub const OUT_REG_WORD_LENGTH_MASK_SFT: u32 = 0x1f << 16;
pub const OUT_REG_CH_NUM_SFT: u32 = 23;
pub const OUT_REG_CH_NUM_MASK: u32 = 0x1f;
pub const OUT_REG_CH_NUM_MASK_SFT: u32 = 0x1f << 23;
pub const OUT_REG_RELATCH_DOMAIN_SEL_SFT: u32 = 28;
pub const OUT_REG_RELATCH_DOMAIN_SEL_MASK: u32 = 0x7;
pub const OUT_REG_RELATCH_DOMAIN_SEL_MASK_SFT: u32 = 0x7 << 28;
pub const OUT_REG_VALID_TOGETHER_SFT: u32 = 31;
pub const OUT_REG_VALID_TOGETHER_MASK: u32 = 0x1;
pub const OUT_REG_VALID_TOGETHER_MASK_SFT: u32 = 0x1 << 31;

/* ETDM_OUT0_CON1 */
/* ETDM_OUT1_CON1 */
/* ETDM_OUT4_CON1 */
pub const OUT_REG_INITIAL_COUNT_SFT: u32 = 0;
pub const OUT_REG_INITIAL_COUNT_MASK: u32 = 0x1f;
pub const OUT_REG_INITIAL_COUNT_MASK_SFT: u32 = 0x1f << 0;
pub const OUT_REG_INITIAL_POINT_SFT: u32 = 5;
pub const OUT_REG_INITIAL_POINT_MASK: u32 = 0x1f;
pub const OUT_REG_INITIAL_POINT_MASK_SFT: u32 = 0x1f << 5;
pub const OUT_REG_LRCK_AUTO_OFF_SFT: u32 = 10;
pub const OUT_REG_LRCK_AUTO_OFF_MASK: u32 = 0x1;
pub const OUT_REG_LRCK_AUTO_OFF_MASK_SFT: u32 = 0x1 << 10;
pub const OUT_REG_BCK_AUTO_OFF_SFT: u32 = 11;
pub const OUT_REG_BCK_AUTO_OFF_MASK: u32 = 0x1;
pub const OUT_REG_BCK_AUTO_OFF_MASK_SFT: u32 = 0x1 << 11;
pub const OUT_REG_INITIAL_LRCK_SFT: u32 = 13;
pub const OUT_REG_INITIAL_LRCK_MASK: u32 = 0x1;
pub const OUT_REG_INITIAL_LRCK_MASK_SFT: u32 = 0x1 << 13;
pub const OUT_REG_NO_ALIGN_1X_EN_SFT: u32 = 14;
pub const OUT_REG_NO_ALIGN_1X_EN_MASK: u32 = 0x1;
pub const OUT_REG_NO_ALIGN_1X_EN_MASK_SFT: u32 = 0x1 << 14;
pub const OUT_REG_LRCK_RESET_SFT: u32 = 15;
pub const OUT_REG_LRCK_RESET_MASK: u32 = 0x1;
pub const OUT_REG_LRCK_RESET_MASK_SFT: u32 = 0x1 << 15;
pub const OUT_PINMUX_MCLK_CTRL_OE_SFT: u32 = 16;
pub const OUT_PINMUX_MCLK_CTRL_OE_MASK: u32 = 0x1;
pub const OUT_PINMUX_MCLK_CTRL_OE_MASK_SFT: u32 = 0x1 << 16;
pub const OUT_REG_OUTPUT_CR_EN_SFT: u32 = 18;
pub const OUT_REG_OUTPUT_CR_EN_MASK: u32 = 0x1;
pub const OUT_REG_OUTPUT_CR_EN_MASK_SFT: u32 = 0x1 << 18;
pub const OUT_REG_LRCK_WIDTH_SFT: u32 = 19;
pub const OUT_REG_LRCK_WIDTH_MASK: u32 = 0x3ff;
pub const OUT_REG_LRCK_WIDTH_MASK_SFT: u32 = 0x3ff << 19;
pub const OUT_REG_LRCK_AUTO_MODE_SFT: u32 = 29;
pub const OUT_REG_LRCK_AUTO_MODE_MASK: u32 = 0x1;
pub const OUT_REG_LRCK_AUTO_MODE_MASK_SFT: u32 = 0x1 << 29;
pub const OUT_REG_DIRECT_INPUT_MASTER_BCK_SFT: u32 = 30;
pub const OUT_REG_DIRECT_INPUT_MASTER_BCK_MASK: u32 = 0x1;
pub const OUT_REG_DIRECT_INPUT_MASTER_BCK_MASK_SFT: u32 = 0x1 << 30;
pub const OUT_REG_16B_COMPACT_MODE_SFT: u32 = 31;
pub const OUT_REG_16B_COMPACT_MODE_MASK: u32 = 0x1;
pub const OUT_REG_16B_COMPACT_MODE_MASK_SFT: u32 = 0x1 << 31;

/* ETDM_OUT0_CON2 */
/* ETDM_OUT1_CON2 */
/* ETDM_OUT4_CON2 */
pub const OUT_REG_IN2LATCH_TIME_SFT: u32 = 0;
pub const OUT_REG_IN2LATCH_TIME_MASK: u32 = 0x1f;
pub const OUT_REG_IN2LATCH_TIME_MASK_SFT: u32 = 0x1f << 0;
pub const OUT_REG_MASK_NUM_SFT: u32 = 5;
pub const OUT_REG_MASK_NUM_MASK: u32 = 0x1f;
pub const OUT_REG_MASK_NUM_MASK_SFT: u32 = 0x1f << 5;
pub const OUT_REG_MASK_AUTO_SFT: u32 = 10;
pub const OUT_REG_MASK_AUTO_MASK: u32 = 0x1;
pub const OUT_REG_MASK_AUTO_MASK_SFT: u32 = 0x1 << 10;
pub const OUT_REG_SDATA_SHIFT_SFT: u32 = 11;
pub const OUT_REG_SDATA_SHIFT_MASK: u32 = 0x3;
pub const OUT_REG_SDATA_SHIFT_MASK_SFT: u32 = 0x3 << 11;
pub const OUT_REG_ALMOST_END_BIT_COUNT_SFT: u32 = 13;
pub const OUT_REG_ALMOST_END_BIT_COUNT_MASK: u32 = 0x1f;
pub const OUT_REG_ALMOST_END_BIT_COUNT_MASK_SFT: u32 = 0x1f << 13;
pub const OUT_REG_SDATA_CON_SFT: u32 = 18;
pub const OUT_REG_SDATA_CON_MASK: u32 = 0x3;
pub const OUT_REG_SDATA_CON_MASK_SFT: u32 = 0x3 << 18;
pub const OUT_REG_REDUNDANT_0_SFT: u32 = 20;
pub const OUT_REG_REDUNDANT_0_MASK: u32 = 0x1;
pub const OUT_REG_REDUNDANT_0_MASK_SFT: u32 = 0x1 << 20;
pub const OUT_REG_SDATA_AUTO_OFF_SFT: u32 = 21;
pub const OUT_REG_SDATA_AUTO_OFF_MASK: u32 = 0x1;
pub const OUT_REG_SDATA_AUTO_OFF_MASK_SFT: u32 = 0x1 << 21;
pub const OUT_REG_BCK_OFF_TIME_SFT: u32 = 22;
pub const OUT_REG_BCK_OFF_TIME_MASK: u32 = 0x3;
pub const OUT_REG_BCK_OFF_TIME_MASK_SFT: u32 = 0x3 << 22;
pub const OUT_REG_MONITOR_SEL_SFT: u32 = 24;
pub const OUT_REG_MONITOR_SEL_MASK: u32 = 0x3;
pub const OUT_REG_MONITOR_SEL_MASK_SFT: u32 = 0x3 << 24;
pub const OUT_REG_SHIFT_AUTO_SFT: u32 = 26;
pub const OUT_REG_SHIFT_AUTO_MASK: u32 = 0x1;
pub const OUT_REG_SHIFT_AUTO_MASK_SFT: u32 = 0x1 << 26;
pub const OUT_REG_SDATA_DELAY_0P5T_EN_SFT: u32 = 27;
pub const OUT_REG_SDATA_DELAY_0P5T_EN_MASK: u32 = 0x1;
pub const OUT_REG_SDATA_DELAY_0P5T_EN_MASK_SFT: u32 = 0x1 << 27;
pub const OUT_REG_SDATA_DELAY_BCK_INV_SFT: u32 = 28;
pub const OUT_REG_SDATA_DELAY_BCK_INV_MASK: u32 = 0x1;
pub const OUT_REG_SDATA_DELAY_BCK_INV_MASK_SFT: u32 = 0x1 << 28;
pub const OUT_REG_LRCK_DELAY_0P5T_EN_SFT: u32 = 29;
pub const OUT_REG_LRCK_DELAY_0P5T_EN_MASK: u32 = 0x1;
pub const OUT_REG_LRCK_DELAY_0P5T_EN_MASK_SFT: u32 = 0x1 << 29;
pub const OUT_REG_LRCK_DELAY_BCK_INV_SFT: u32 = 30;
pub const OUT_REG_LRCK_DELAY_BCK_INV_MASK: u32 = 0x1;
pub const OUT_REG_LRCK_DELAY_BCK_INV_MASK_SFT: u32 = 0x1 << 30;
pub const OUT_REG_OFF_CR_EN_SFT: u32 = 31;
pub const OUT_REG_OFF_CR_EN_MASK: u32 = 0x1;
pub const OUT_REG_OFF_CR_EN_MASK_SFT: u32 = 0x1 << 31;

/* ETDM_OUT0_CON3 */
/* ETDM_OUT1_CON3 */
/* ETDM_OUT4_CON3 */
pub const OUT_REG_START_CH_PAIR0_SFT: u32 = 0;
pub const OUT_REG_START_CH_PAIR0_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR0_MASK_SFT: u32 = 0xf << 0;
pub const OUT_REG_START_CH_PAIR1_SFT: u32 = 4;
pub const OUT_REG_START_CH_PAIR1_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR1_MASK_SFT: u32 = 0xf << 4;
pub const OUT_REG_START_CH_PAIR2_SFT: u32 = 8;
pub const OUT_REG_START_CH_PAIR2_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR2_MASK_SFT: u32 = 0xf << 8;
pub const OUT_REG_START_CH_PAIR3_SFT: u32 = 12;
pub const OUT_REG_START_CH_PAIR3_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR3_MASK_SFT: u32 = 0xf << 12;
pub const OUT_REG_START_CH_PAIR4_SFT: u32 = 16;
pub const OUT_REG_START_CH_PAIR4_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR4_MASK_SFT: u32 = 0xf << 16;
pub const OUT_REG_START_CH_PAIR5_SFT: u32 = 20;
pub const OUT_REG_START_CH_PAIR5_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR5_MASK_SFT: u32 = 0xf << 20;
pub const OUT_REG_START_CH_PAIR6_SFT: u32 = 24;
pub const OUT_REG_START_CH_PAIR6_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR6_MASK_SFT: u32 = 0xf << 24;
pub const OUT_REG_START_CH_PAIR7_SFT: u32 = 28;
pub const OUT_REG_START_CH_PAIR7_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR7_MASK_SFT: u32 = 0xf << 28;

/* ETDM_OUT0_CON4 */
/* ETDM_OUT1_CON4 */
/* ETDM_OUT4_CON4 */
pub const OUT_REG_FS_TIMING_SEL_SFT: u32 = 0;
pub const OUT_REG_FS_TIMING_SEL_MASK: u32 = 0x1f;
pub const OUT_REG_FS_TIMING_SEL_MASK_SFT: u32 = 0x1f << 0;
pub const OUT_REG_CLOCK_SOURCE_SEL_SFT: u32 = 6;
pub const OUT_REG_CLOCK_SOURCE_SEL_MASK: u32 = 0x7;
pub const OUT_REG_CLOCK_SOURCE_SEL_MASK_SFT: u32 = 0x7 << 6;
pub const OUT_REG_CK_EN_SEL_AUTO_SFT: u32 = 10;
pub const OUT_REG_CK_EN_SEL_AUTO_MASK: u32 = 0x1;
pub const OUT_REG_CK_EN_SEL_AUTO_MASK_SFT: u32 = 0x1 << 10;
pub const OUT_REG_ASYNC_RESET_SFT: u32 = 11;
pub const OUT_REG_ASYNC_RESET_MASK: u32 = 0x1;
pub const OUT_REG_ASYNC_RESET_MASK_SFT: u32 = 0x1 << 11;
pub const OUT_REG_CK_EN_SEL_MANUAL_SFT: u32 = 14;
pub const OUT_REG_CK_EN_SEL_MANUAL_MASK: u32 = 0x3ff;
pub const OUT_REG_CK_EN_SEL_MANUAL_MASK_SFT: u32 = 0x3ff << 14;
pub const OUT_REG_RELATCH_EN_SEL_SFT: u32 = 24;
pub const OUT_REG_RELATCH_EN_SEL_MASK: u32 = 0x1f;
pub const OUT_REG_RELATCH_EN_SEL_MASK_SFT: u32 = 0x1f << 24;
pub const OUT_REG_WAIT_LAST_SAMPLE_SFT: u32 = 30;
pub const OUT_REG_WAIT_LAST_SAMPLE_MASK: u32 = 0x1;
pub const OUT_REG_WAIT_LAST_SAMPLE_MASK_SFT: u32 = 0x1 << 30;
pub const OUT_REG_ALWAYS_OPEN_1X_EN_SFT: u32 = 31;
pub const OUT_REG_ALWAYS_OPEN_1X_EN_MASK: u32 = 0x1;
pub const OUT_REG_ALWAYS_OPEN_1X_EN_MASK_SFT: u32 = 0x1 << 31;

/* ETDM_OUT0_CON5 */
/* ETDM_OUT1_CON5 */
/* ETDM_OUT4_CON5 */
pub const OUT_REG_REPACK_BITNUM_SFT: u32 = 0;
pub const OUT_REG_REPACK_BITNUM_MASK: u32 = 0x3;
pub const OUT_REG_REPACK_BITNUM_MASK_SFT: u32 = 0x3 << 0;
pub const OUT_REG_REPACK_CHNUM_SFT: u32 = 2;
pub const OUT_REG_REPACK_CHNUM_MASK: u32 = 0xf;
pub const OUT_REG_REPACK_CHNUM_MASK_SFT: u32 = 0xf << 2;
pub const OUT_REG_SLAVE_BCK_INV_SFT: u32 = 7;
pub const OUT_REG_SLAVE_BCK_INV_MASK: u32 = 0x1;
pub const OUT_REG_SLAVE_BCK_INV_MASK_SFT: u32 = 0x1 << 7;
pub const OUT_REG_SLAVE_LRCK_INV_SFT: u32 = 8;
pub const OUT_REG_SLAVE_LRCK_INV_MASK: u32 = 0x1;
pub const OUT_REG_SLAVE_LRCK_INV_MASK_SFT: u32 = 0x1 << 8;
pub const OUT_REG_MASTER_BCK_INV_SFT: u32 = 9;
pub const OUT_REG_MASTER_BCK_INV_MASK: u32 = 0x1;
pub const OUT_REG_MASTER_BCK_INV_MASK_SFT: u32 = 0x1 << 9;
pub const OUT_REG_MASTER_WS_INV_SFT: u32 = 10;
pub const OUT_REG_MASTER_WS_INV_MASK: u32 = 0x1;
pub const OUT_REG_MASTER_WS_INV_MASK_SFT: u32 = 0x1 << 10;
pub const OUT_REG_REPACK_24B_MSB_ALIGN_SFT: u32 = 11;
pub const OUT_REG_REPACK_24B_MSB_ALIGN_MASK: u32 = 0x1;
pub const OUT_REG_REPACK_24B_MSB_ALIGN_MASK_SFT: u32 = 0x1 << 11;
pub const OUT_REG_LR_SWAP_SFT: u32 = 16;
pub const OUT_REG_LR_SWAP_MASK: u32 = 0xffff;
pub const OUT_REG_LR_SWAP_MASK_SFT: u32 = 0xffff << 16;

/* ETDM_OUT0_CON6 */
/* ETDM_OUT1_CON6 */
/* ETDM_OUT4_CON6 */
pub const OUT_LCH_DATA_REG_SFT: u32 = 0;
pub const OUT_LCH_DATA_REG_MASK: u32 = 0xffffffff;
pub const OUT_LCH_DATA_REG_MASK_SFT: u32 = 0xffffffff << 0;

/* ETDM_OUT0_CON7 */
/* ETDM_OUT1_CON7 */
/* ETDM_OUT4_CON7 */
pub const OUT_RCH_DATA_REG_SFT: u32 = 0;
pub const OUT_RCH_DATA_REG_MASK: u32 = 0xffffffff;
pub const OUT_RCH_DATA_REG_MASK_SFT: u32 = 0xffffffff << 0;

/* ETDM_OUT0_CON8 */
/* ETDM_OUT1_CON8 */
/* ETDM_OUT4_CON8 */
pub const OUT_REG_START_CH_PAIR8_SFT: u32 = 0;
pub const OUT_REG_START_CH_PAIR8_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR8_MASK_SFT: u32 = 0xf << 0;
pub const OUT_REG_START_CH_PAIR9_SFT: u32 = 4;
pub const OUT_REG_START_CH_PAIR9_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR9_MASK_SFT: u32 = 0xf << 4;
pub const OUT_REG_START_CH_PAIR10_SFT: u32 = 8;
pub const OUT_REG_START_CH_PAIR10_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR10_MASK_SFT: u32 = 0xf << 8;
pub const OUT_REG_START_CH_PAIR11_SFT: u32 = 12;
pub const OUT_REG_START_CH_PAIR11_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR11_MASK_SFT: u32 = 0xf << 12;
pub const OUT_REG_START_CH_PAIR12_SFT: u32 = 16;
pub const OUT_REG_START_CH_PAIR12_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR12_MASK_SFT: u32 = 0xf << 16;
pub const OUT_REG_START_CH_PAIR13_SFT: u32 = 20;
pub const OUT_REG_START_CH_PAIR13_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR13_MASK_SFT: u32 = 0xf << 20;
pub const OUT_REG_START_CH_PAIR14_SFT: u32 = 24;
pub const OUT_REG_START_CH_PAIR14_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR14_MASK_SFT: u32 = 0xf << 24;
pub const OUT_REG_START_CH_PAIR15_SFT: u32 = 28;
pub const OUT_REG_START_CH_PAIR15_MASK: u32 = 0xf;
pub const OUT_REG_START_CH_PAIR15_MASK_SFT: u32 = 0xf << 28;

/* ETDM_OUT0_CON9 */
/* ETDM_OUT1_CON9 */
/* ETDM_OUT4_CON9 */
pub const OUT_REG_AFIFO_THRESHOLD_SFT: u32 = 29;
pub const OUT_REG_AFIFO_THRESHOLD_MASK: u32 = 0x3;
pub const OUT_REG_AFIFO_THRESHOLD_MASK_SFT: u32 = 0x3 << 29;
pub const OUT_REG_AFIFO_SW_RESET_SFT: u32 = 15;
pub const OUT_REG_AFIFO_SW_RESET_MASK: u32 = 0x1;
pub const OUT_REG_AFIFO_SW_RESET_MASK_SFT: u32 = 0x1 << 15;
pub const OUT_REG_AFIFO_RESET_SEL_SFT: u32 = 14;
pub const OUT_REG_AFIFO_RESET_SEL_MASK: u32 = 0x1;
pub const OUT_REG_AFIFO_RESET_SEL_MASK_SFT: u32 = 0x1 << 14;
pub const OUT_REG_AFIFO_AUTO_RESET_DIS_SFT: u32 = 9;
pub const OUT_REG_AFIFO_AUTO_RESET_DIS_MASK: u32 = 0x1;
pub const OUT_REG_AFIFO_AUTO_RESET_DIS_MASK_SFT: u32 = 0x1 << 9;
pub const OUT_REG_ETDM_USE_AFIFO_SFT: u32 = 8;
pub const OUT_REG_ETDM_USE_AFIFO_MASK: u32 = 0x1;
pub const OUT_REG_ETDM_USE_AFIFO_MASK_SFT: u32 = 0x1 << 8;
pub const OUT_REG_AFIFO_CLOCK_DOMAIN_SEL_SFT: u32 = 5;
pub const OUT_REG_AFIFO_CLOCK_DOMAIN_SEL_MASK: u32 = 0x7;
pub const OUT_REG_AFIFO_CLOCK_DOMAIN_SEL_MASK_SFT: u32 = 0x7 << 5;
pub const OUT_REG_AFIFO_MODE_SFT: u32 = 0;
pub const OUT_REG_AFIFO_MODE_MASK: u32 = 0x1f;
pub const OUT_REG_AFIFO_MODE_MASK_SFT: u32 = 0x1f << 0;

/* ETDM_OUT0_MON */
/* ETDM_OUT1_MON */
/* ETDM_OUT4_MON */
/* duplicate C macro: LRCK_INV_SFT = 30 */
/* duplicate C macro: LRCK_INV_MASK = 0x1 */
/* duplicate C macro: LRCK_INV_MASK_SFT = 0x1 << 30 */
/* duplicate C macro: EN_SYNC_OUT_SFT = 29 */
/* duplicate C macro: EN_SYNC_OUT_MASK = 0x1 */
/* duplicate C macro: EN_SYNC_OUT_MASK_SFT = 0x1 << 29 */
/* duplicate C macro: HOPPING_EN_SYNC_OUT_PRE_SFT = 28 */
/* duplicate C macro: HOPPING_EN_SYNC_OUT_PRE_MASK = 0x1 */
/* duplicate C macro: HOPPING_EN_SYNC_OUT_PRE_MASK_SFT = 0x1 << 28 */
/* duplicate C macro: ETDM_2X_CK_EN_SFT = 25 */
/* duplicate C macro: ETDM_2X_CK_EN_MASK = 0x1 */
/* duplicate C macro: ETDM_2X_CK_EN_MASK_SFT = 0x1 << 25 */
/* duplicate C macro: ETDM_1X_CK_EN_SFT = 24 */
/* duplicate C macro: ETDM_1X_CK_EN_MASK = 0x1 */
/* duplicate C macro: ETDM_1X_CK_EN_MASK_SFT = 0x1 << 24 */
/* duplicate C macro: SDATA0_SFT = 23 */
/* duplicate C macro: SDATA0_MASK = 0x1 */
/* duplicate C macro: SDATA0_MASK_SFT = 0x1 << 23 */
/* duplicate C macro: CURRENT_STATUS_SFT = 21 */
/* duplicate C macro: CURRENT_STATUS_MASK = 0x3 */
/* duplicate C macro: CURRENT_STATUS_MASK_SFT = 0x3 << 21 */
/* duplicate C macro: BIT_POINT_SFT = 16 */
/* duplicate C macro: BIT_POINT_MASK = 0x1f */
/* duplicate C macro: BIT_POINT_MASK_SFT = 0x1f << 16 */
/* duplicate C macro: BIT_CH_COUNT_SFT = 10 */
/* duplicate C macro: BIT_CH_COUNT_MASK = 0x3f */
/* duplicate C macro: BIT_CH_COUNT_MASK_SFT = 0x3f << 10 */
/* duplicate C macro: BIT_COUNT_SFT = 5 */
/* duplicate C macro: BIT_COUNT_MASK = 0x1f */
/* duplicate C macro: BIT_COUNT_MASK_SFT = 0x1f << 5 */
/* duplicate C macro: CH_COUNT_SFT = 0 */
/* duplicate C macro: CH_COUNT_MASK = 0x1f */
/* duplicate C macro: CH_COUNT_MASK_SFT = 0x1f << 0 */

/* ETDM_0_3_COWORK_CON0 */
pub const ETDM_OUT0_DATA_SEL_SFT: u32 = 0;
pub const ETDM_OUT0_DATA_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT0_DATA_SEL_MASK_SFT: u32 = 0xf << 0;
pub const ETDM_OUT0_SYNC_SEL_SFT: u32 = 4;
pub const ETDM_OUT0_SYNC_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT0_SYNC_SEL_MASK_SFT: u32 = 0xf << 4;
pub const ETDM_OUT0_SLAVE_SEL_SFT: u32 = 8;
pub const ETDM_OUT0_SLAVE_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT0_SLAVE_SEL_MASK_SFT: u32 = 0xf << 8;
pub const ETDM_OUT1_DATA_SEL_SFT: u32 = 12;
pub const ETDM_OUT1_DATA_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT1_DATA_SEL_MASK_SFT: u32 = 0xf << 12;
pub const ETDM_OUT1_SYNC_SEL_SFT: u32 = 16;
pub const ETDM_OUT1_SYNC_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT1_SYNC_SEL_MASK_SFT: u32 = 0xf << 16;
pub const ETDM_OUT1_SLAVE_SEL_SFT: u32 = 20;
pub const ETDM_OUT1_SLAVE_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT1_SLAVE_SEL_MASK_SFT: u32 = 0xf << 20;
pub const ETDM_IN0_SLAVE_SEL_SFT: u32 = 24;
pub const ETDM_IN0_SLAVE_SEL_MASK: u32 = 0xf;
pub const ETDM_IN0_SLAVE_SEL_MASK_SFT: u32 = 0xf << 24;
pub const ETDM_IN0_SYNC_SEL_SFT: u32 = 28;
pub const ETDM_IN0_SYNC_SEL_MASK: u32 = 0xf;
pub const ETDM_IN0_SYNC_SEL_MASK_SFT: u32 = 0xf << 28;

/* ETDM_0_3_COWORK_CON1 */
pub const ETDM_IN0_SDATA0_SEL_SFT: u32 = 0;
pub const ETDM_IN0_SDATA0_SEL_MASK: u32 = 0xf;
pub const ETDM_IN0_SDATA0_SEL_MASK_SFT: u32 = 0xf << 0;
pub const ETDM_IN0_SDATA1_15_SEL_SFT: u32 = 4;
pub const ETDM_IN0_SDATA1_15_SEL_MASK: u32 = 0xf;
pub const ETDM_IN0_SDATA1_15_SEL_MASK_SFT: u32 = 0xf << 4;
pub const ETDM_IN1_SLAVE_SEL_SFT: u32 = 8;
pub const ETDM_IN1_SLAVE_SEL_MASK: u32 = 0xf;
pub const ETDM_IN1_SLAVE_SEL_MASK_SFT: u32 = 0xf << 8;
pub const ETDM_IN1_SYNC_SEL_SFT: u32 = 12;
pub const ETDM_IN1_SYNC_SEL_MASK: u32 = 0xf;
pub const ETDM_IN1_SYNC_SEL_MASK_SFT: u32 = 0xf << 12;
pub const ETDM_IN1_SDATA0_SEL_SFT: u32 = 16;
pub const ETDM_IN1_SDATA0_SEL_MASK: u32 = 0xf;
pub const ETDM_IN1_SDATA0_SEL_MASK_SFT: u32 = 0xf << 16;
pub const ETDM_IN1_SDATA1_15_SEL_SFT: u32 = 20;
pub const ETDM_IN1_SDATA1_15_SEL_MASK: u32 = 0xf;
pub const ETDM_IN1_SDATA1_15_SEL_MASK_SFT: u32 = 0xf << 20;

/* ETDM_4_7_COWORK_CON0 */
pub const ETDM_OUT4_DATA_SEL_SFT: u32 = 0;
pub const ETDM_OUT4_DATA_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT4_DATA_SEL_MASK_SFT: u32 = 0xf << 0;
pub const ETDM_OUT4_SYNC_SEL_SFT: u32 = 4;
pub const ETDM_OUT4_SYNC_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT4_SYNC_SEL_MASK_SFT: u32 = 0xf << 4;
pub const ETDM_OUT4_SLAVE_SEL_SFT: u32 = 8;
pub const ETDM_OUT4_SLAVE_SEL_MASK: u32 = 0xf;
pub const ETDM_OUT4_SLAVE_SEL_MASK_SFT: u32 = 0xf << 8;

/* AFE_DPTX_CON */
pub const DPTX_CHANNEL_ENABLE_SFT: u32 = 8;
pub const DPTX_CHANNEL_ENABLE_MASK: u32 = 0xff;
pub const DPTX_CHANNEL_ENABLE_MASK_SFT: u32 = 0xff << 8;
pub const DPTX_REGISTER_MONITOR_SELECT_SFT: u32 = 3;
pub const DPTX_REGISTER_MONITOR_SELECT_MASK: u32 = 0xf;
pub const DPTX_REGISTER_MONITOR_SELECT_MASK_SFT: u32 = 0xf << 3;
pub const DPTX_16BIT_SFT: u32 = 2;
pub const DPTX_16BIT_MASK: u32 = 0x1;
pub const DPTX_16BIT_MASK_SFT: u32 = 0x1 << 2;
pub const DPTX_CHANNEL_NUMBER_SFT: u32 = 1;
pub const DPTX_CHANNEL_NUMBER_MASK: u32 = 0x1;
pub const DPTX_CHANNEL_NUMBER_MASK_SFT: u32 = 0x1 << 1;
pub const DPTX_ON_SFT: u32 = 0;
pub const DPTX_ON_MASK: u32 = 0x1;
pub const DPTX_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_DPTX_MON */
pub const AFE_DPTX_MON0_SFT: u32 = 0;
pub const AFE_DPTX_MON0_MASK: u32 = 0xffffffff;
pub const AFE_DPTX_MON0_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_TDM_CON1 */
pub const TDM_EN_SFT: u32 = 0;
pub const TDM_EN_MASK: u32 = 0x1;
pub const TDM_EN_MASK_SFT: u32 = 0x1 << 0;
pub const BCK_INVERSE_SFT: u32 = 1;
pub const BCK_INVERSE_MASK: u32 = 0x1;
pub const BCK_INVERSE_MASK_SFT: u32 = 0x1 << 1;
pub const LRCK_INVERSE_SFT: u32 = 2;
pub const LRCK_INVERSE_MASK: u32 = 0x1;
pub const LRCK_INVERSE_MASK_SFT: u32 = 0x1 << 2;
pub const DELAY_DATA_SFT: u32 = 3;
pub const DELAY_DATA_MASK: u32 = 0x1;
pub const DELAY_DATA_MASK_SFT: u32 = 0x1 << 3;
pub const LEFT_ALIGN_SFT: u32 = 4;
pub const LEFT_ALIGN_MASK: u32 = 0x1;
pub const LEFT_ALIGN_MASK_SFT: u32 = 0x1 << 4;
pub const TDM_LRCK_D0P5T_SFT: u32 = 5;
pub const TDM_LRCK_D0P5T_MASK: u32 = 0x1;
pub const TDM_LRCK_D0P5T_MASK_SFT: u32 = 0x1 << 5;
pub const TDM_SDATA_D0P5T_SFT: u32 = 6;
pub const TDM_SDATA_D0P5T_MASK: u32 = 0x1;
pub const TDM_SDATA_D0P5T_MASK_SFT: u32 = 0x1 << 6;
pub const WLEN_SFT: u32 = 8;
pub const WLEN_MASK: u32 = 0x3;
pub const WLEN_MASK_SFT: u32 = 0x3 << 8;
pub const CHANNEL_NUM_SFT: u32 = 10;
pub const CHANNEL_NUM_MASK: u32 = 0x3;
pub const CHANNEL_NUM_MASK_SFT: u32 = 0x3 << 10;
pub const CHANNEL_BCK_CYCLES_SFT: u32 = 12;
pub const CHANNEL_BCK_CYCLES_MASK: u32 = 0x3;
pub const CHANNEL_BCK_CYCLES_MASK_SFT: u32 = 0x3 << 12;
pub const HDMI_CLK_INV_SEL_SFT: u32 = 15;
pub const HDMI_CLK_INV_SEL_MASK: u32 = 0x1;
pub const HDMI_CLK_INV_SEL_MASK_SFT: u32 = 0x1 << 15;
pub const DAC_BIT_NUM_SFT: u32 = 16;
pub const DAC_BIT_NUM_MASK: u32 = 0x1f;
pub const DAC_BIT_NUM_MASK_SFT: u32 = 0x1f << 16;
pub const LRCK_TDM_WIDTH_SFT: u32 = 24;
pub const LRCK_TDM_WIDTH_MASK: u32 = 0xff;
pub const LRCK_TDM_WIDTH_MASK_SFT: u32 = 0xff << 24;

/* AFE_TDM_CON2 */
pub const ST_CH_PAIR_SOUT0_SFT: u32 = 0;
pub const ST_CH_PAIR_SOUT0_MASK: u32 = 0x7;
pub const ST_CH_PAIR_SOUT0_MASK_SFT: u32 = 0x7 << 0;
pub const ST_CH_PAIR_SOUT1_SFT: u32 = 4;
pub const ST_CH_PAIR_SOUT1_MASK: u32 = 0x7;
pub const ST_CH_PAIR_SOUT1_MASK_SFT: u32 = 0x7 << 4;
pub const ST_CH_PAIR_SOUT2_SFT: u32 = 8;
pub const ST_CH_PAIR_SOUT2_MASK: u32 = 0x7;
pub const ST_CH_PAIR_SOUT2_MASK_SFT: u32 = 0x7 << 8;
pub const ST_CH_PAIR_SOUT3_SFT: u32 = 12;
pub const ST_CH_PAIR_SOUT3_MASK: u32 = 0x7;
pub const ST_CH_PAIR_SOUT3_MASK_SFT: u32 = 0x7 << 12;
pub const TDM_FIX_VALUE_SEL_SFT: u32 = 16;
pub const TDM_FIX_VALUE_SEL_MASK: u32 = 0x1;
pub const TDM_FIX_VALUE_SEL_MASK_SFT: u32 = 0x1 << 16;
pub const TDM_I2S_LOOPBACK_SFT: u32 = 20;
pub const TDM_I2S_LOOPBACK_MASK: u32 = 0x1;
pub const TDM_I2S_LOOPBACK_MASK_SFT: u32 = 0x1 << 20;
pub const TDM_I2S_LOOPBACK_CH_SFT: u32 = 21;
pub const TDM_I2S_LOOPBACK_CH_MASK: u32 = 0x3;
pub const TDM_I2S_LOOPBACK_CH_MASK_SFT: u32 = 0x3 << 21;
pub const TDM_USE_SINEGEN_INPUT_SFT: u32 = 23;
pub const TDM_USE_SINEGEN_INPUT_MASK: u32 = 0x1;
pub const TDM_USE_SINEGEN_INPUT_MASK_SFT: u32 = 0x1 << 23;
pub const TDM_FIX_VALUE_SFT: u32 = 24;
pub const TDM_FIX_VALUE_MASK: u32 = 0xff;
pub const TDM_FIX_VALUE_MASK_SFT: u32 = 0xff << 24;

/* AFE_TDM_CON3 */
pub const TDM_OUT_SEL_DOMAIN_SFT: u32 = 29;
pub const TDM_OUT_SEL_DOMAIN_MASK: u32 = 0x7;
pub const TDM_OUT_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 29;
pub const TDM_OUT_SEL_FS_SFT: u32 = 24;
pub const TDM_OUT_SEL_FS_MASK: u32 = 0x1f;
pub const TDM_OUT_SEL_FS_MASK_SFT: u32 = 0x1f << 24;
pub const TDM_OUT_MON_SEL_SFT: u32 = 3;
pub const TDM_OUT_MON_SEL_MASK: u32 = 0x1;
pub const TDM_OUT_MON_SEL_MASK_SFT: u32 = 0x1 << 3;
pub const RG_TDM_OUT_ASYNC_FIFO_SOFT_RST_EN_SFT: u32 = 2;
pub const RG_TDM_OUT_ASYNC_FIFO_SOFT_RST_EN_MASK: u32 = 0x1;
pub const RG_TDM_OUT_ASYNC_FIFO_SOFT_RST_EN_MASK_SFT: u32 = 0x1 << 2;
pub const RG_TDM_OUT_ASYNC_FIFO_SOFT_RST_SFT: u32 = 1;
pub const RG_TDM_OUT_ASYNC_FIFO_SOFT_RST_MASK: u32 = 0x1;
pub const RG_TDM_OUT_ASYNC_FIFO_SOFT_RST_MASK_SFT: u32 = 0x1 << 1;
pub const TDM_UPDATE_EN_SEL_SFT: u32 = 0;
pub const TDM_UPDATE_EN_SEL_MASK: u32 = 0x1;
pub const TDM_UPDATE_EN_SEL_MASK_SFT: u32 = 0x1 << 0;

/* AFE_TDM_OUT_MON */
pub const AFE_TDM_OUT_MON_SFT: u32 = 0;
pub const AFE_TDM_OUT_MON_MASK: u32 = 0xffffffff;
pub const AFE_TDM_OUT_MON_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_HDMI_CONN0 */
pub const HDMI_O_7_SFT: u32 = 21;
pub const HDMI_O_7_MASK: u32 = 0x7;
pub const HDMI_O_7_MASK_SFT: u32 = 0x7 << 21;
pub const HDMI_O_6_SFT: u32 = 18;
pub const HDMI_O_6_MASK: u32 = 0x7;
pub const HDMI_O_6_MASK_SFT: u32 = 0x7 << 18;
pub const HDMI_O_5_SFT: u32 = 15;
pub const HDMI_O_5_MASK: u32 = 0x7;
pub const HDMI_O_5_MASK_SFT: u32 = 0x7 << 15;
pub const HDMI_O_4_SFT: u32 = 12;
pub const HDMI_O_4_MASK: u32 = 0x7;
pub const HDMI_O_4_MASK_SFT: u32 = 0x7 << 12;
pub const HDMI_O_3_SFT: u32 = 9;
pub const HDMI_O_3_MASK: u32 = 0x7;
pub const HDMI_O_3_MASK_SFT: u32 = 0x7 << 9;
pub const HDMI_O_2_SFT: u32 = 6;
pub const HDMI_O_2_MASK: u32 = 0x7;
pub const HDMI_O_2_MASK_SFT: u32 = 0x7 << 6;
pub const HDMI_O_1_SFT: u32 = 3;
pub const HDMI_O_1_MASK: u32 = 0x7;
pub const HDMI_O_1_MASK_SFT: u32 = 0x7 << 3;
pub const HDMI_O_0_SFT: u32 = 0;
pub const HDMI_O_0_MASK: u32 = 0x7;
pub const HDMI_O_0_MASK_SFT: u32 = 0x7 << 0;

/* AFE_TDM_TOP_IP_VERSION */
pub const AFE_TDM_TOP_IP_VERSION_SFT: u32 = 0;
pub const AFE_TDM_TOP_IP_VERSION_MASK: u32 = 0xffffffff;
pub const AFE_TDM_TOP_IP_VERSION_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_HDMI_OUT_BASE_MSB */
pub const AFE_HDMI_OUT_BASE_MSB_SFT: u32 = 0;
pub const AFE_HDMI_OUT_BASE_MSB_MASK: u32 = 0x1ff;
pub const AFE_HDMI_OUT_BASE_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_HDMI_OUT_BASE */
pub const AFE_HDMI_OUT_BASE_SFT: u32 = 4;
pub const AFE_HDMI_OUT_BASE_MASK: u32 = 0xfffffff;
pub const AFE_HDMI_OUT_BASE_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_HDMI_OUT_CUR_MSB */
pub const AFE_HDMI_OUT_CUR_MSB_SFT: u32 = 0;
pub const AFE_HDMI_OUT_CUR_MSB_MASK: u32 = 0x1ff;
pub const AFE_HDMI_OUT_CUR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_HDMI_OUT_CUR */
pub const AFE_HDMI_OUT_CUR_SFT: u32 = 0;
pub const AFE_HDMI_OUT_CUR_MASK: u32 = 0xffffffff;
pub const AFE_HDMI_OUT_CUR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_HDMI_OUT_END_MSB */
pub const AFE_HDMI_OUT_END_MSB_SFT: u32 = 0;
pub const AFE_HDMI_OUT_END_MSB_MASK: u32 = 0x1ff;
pub const AFE_HDMI_OUT_END_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_HDMI_OUT_END */
pub const AFE_HDMI_OUT_END_SFT: u32 = 4;
pub const AFE_HDMI_OUT_END_MASK: u32 = 0xfffffff;
pub const AFE_HDMI_OUT_END_MASK_SFT: u32 = 0xfffffff << 4;
pub const AFE_HDMI_OUT_END_LSB_SFT: u32 = 0;
pub const AFE_HDMI_OUT_END_LSB_MASK: u32 = 0xf;
pub const AFE_HDMI_OUT_END_LSB_MASK_SFT: u32 = 0xf << 0;

/* AFE_HDMI_OUT_CON0 */
pub const HDMI_OUT_ON_SFT: u32 = 28;
pub const HDMI_OUT_ON_MASK: u32 = 0x1;
pub const HDMI_OUT_ON_MASK_SFT: u32 = 0x1 << 28;
pub const HDMI_CH_NUM_SFT: u32 = 24;
pub const HDMI_CH_NUM_MASK: u32 = 0xf;
pub const HDMI_CH_NUM_MASK_SFT: u32 = 0xf << 24;
pub const HDMI_OUT_ONE_HEART_SEL_SFT: u32 = 22;
pub const HDMI_OUT_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const HDMI_OUT_ONE_HEART_SEL_MASK_SFT: u32 = 0x3 << 22;
pub const HDMI_OUT_MINLEN_SFT: u32 = 20;
pub const HDMI_OUT_MINLEN_MASK: u32 = 0x3;
pub const HDMI_OUT_MINLEN_MASK_SFT: u32 = 0x3 << 20;
pub const HDMI_OUT_MAXLEN_SFT: u32 = 16;
pub const HDMI_OUT_MAXLEN_MASK: u32 = 0x3;
pub const HDMI_OUT_MAXLEN_MASK_SFT: u32 = 0x3 << 16;
pub const HDMI_OUT_SW_CLEAR_BUF_EMPTY_SFT: u32 = 15;
pub const HDMI_OUT_SW_CLEAR_BUF_EMPTY_MASK: u32 = 0x1;
pub const HDMI_OUT_SW_CLEAR_BUF_EMPTY_MASK_SFT: u32 = 0x1 << 15;
pub const HDMI_OUT_PBUF_SIZE_SFT: u32 = 12;
pub const HDMI_OUT_PBUF_SIZE_MASK: u32 = 0x3;
pub const HDMI_OUT_PBUF_SIZE_MASK_SFT: u32 = 0x3 << 12;
pub const HDMI_OUT_SW_CLEAR_HDMI_BUF_EMPTY_SFT: u32 = 7;
pub const HDMI_OUT_SW_CLEAR_HDMI_BUF_EMPTY_MASK: u32 = 0x1;
pub const HDMI_OUT_SW_CLEAR_HDMI_BUF_EMPTY_MASK_SFT: u32 = 0x1 << 7;
pub const HDMI_OUT_NORMAL_MODE_SFT: u32 = 5;
pub const HDMI_OUT_NORMAL_MODE_MASK: u32 = 0x1;
pub const HDMI_OUT_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 5;
pub const HDMI_OUT_HALIGN_SFT: u32 = 4;
pub const HDMI_OUT_HALIGN_MASK: u32 = 0x1;
pub const HDMI_OUT_HALIGN_MASK_SFT: u32 = 0x1 << 4;
pub const HDMI_OUT_HD_MODE_SFT: u32 = 0;
pub const HDMI_OUT_HD_MODE_MASK: u32 = 0x3;
pub const HDMI_OUT_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_CBIP_CFG0 */
pub const CBIP_TOP_SLV_MUX_WAY_EN_SFT: u32 = 16;
pub const CBIP_TOP_SLV_MUX_WAY_EN_MASK: u32 = 0xffff;
pub const CBIP_TOP_SLV_MUX_WAY_EN_MASK_SFT: u32 = 0xffff << 16;
pub const RESERVED_04_SFT: u32 = 15;
pub const RESERVED_04_MASK: u32 = 0x1;
pub const RESERVED_04_MASK_SFT: u32 = 0x1 << 15;
pub const CBIP_ASYNC_MST_RG_FIFO_THRE_SFT: u32 = 13;
pub const CBIP_ASYNC_MST_RG_FIFO_THRE_MASK: u32 = 0x3;
pub const CBIP_ASYNC_MST_RG_FIFO_THRE_MASK_SFT: u32 = 0x3 << 13;
pub const CBIP_ASYNC_MST_POSTWRITE_DIS_SFT: u32 = 12;
pub const CBIP_ASYNC_MST_POSTWRITE_DIS_MASK: u32 = 0x1;
pub const CBIP_ASYNC_MST_POSTWRITE_DIS_MASK_SFT: u32 = 0x1 << 12;
pub const RESERVED_03_SFT: u32 = 11;
pub const RESERVED_03_MASK: u32 = 0x1;
pub const RESERVED_03_MASK_SFT: u32 = 0x1 << 11;
pub const CBIP_ASYNC_SLV_RG_FIFO_THRE_SFT: u32 = 9;
pub const CBIP_ASYNC_SLV_RG_FIFO_THRE_MASK: u32 = 0x3;
pub const CBIP_ASYNC_SLV_RG_FIFO_THRE_MASK_SFT: u32 = 0x3 << 9;
pub const CBIP_ASYNC_SLV_POSTWRITE_DIS_SFT: u32 = 8;
pub const CBIP_ASYNC_SLV_POSTWRITE_DIS_MASK: u32 = 0x1;
pub const CBIP_ASYNC_SLV_POSTWRITE_DIS_MASK_SFT: u32 = 0x1 << 8;
pub const AUDIOSYS_BUSY_SFT: u32 = 7;
pub const AUDIOSYS_BUSY_MASK: u32 = 0x1;
pub const AUDIOSYS_BUSY_MASK_SFT: u32 = 0x1 << 7;
pub const CBIP_SLV_DECODER_ERR_FLAG_EN_SFT: u32 = 6;
pub const CBIP_SLV_DECODER_ERR_FLAG_EN_MASK: u32 = 0x1;
pub const CBIP_SLV_DECODER_ERR_FLAG_EN_MASK_SFT: u32 = 0x1 << 6;
pub const CBIP_SLV_DECODER_SLAVE_WAY_EN_SFT: u32 = 5;
pub const CBIP_SLV_DECODER_SLAVE_WAY_EN_MASK: u32 = 0x1;
pub const CBIP_SLV_DECODER_SLAVE_WAY_EN_MASK_SFT: u32 = 0x1 << 5;
pub const APB_R2T_SFT: u32 = 3;
pub const APB_R2T_MASK: u32 = 0x1;
pub const APB_R2T_MASK_SFT: u32 = 0x1 << 3;
pub const APB_W2T_SFT: u32 = 2;
pub const APB_W2T_MASK: u32 = 0x1;
pub const APB_W2T_MASK_SFT: u32 = 0x1 << 2;
pub const AHB_IDLE_EN_INT_SFT: u32 = 1;
pub const AHB_IDLE_EN_INT_MASK: u32 = 0x1;
pub const AHB_IDLE_EN_INT_MASK_SFT: u32 = 0x1 << 1;
pub const AHB_IDLE_EN_EXT_SFT: u32 = 0;
pub const AHB_IDLE_EN_EXT_MASK: u32 = 0x1;
pub const AHB_IDLE_EN_EXT_MASK_SFT: u32 = 0x1 << 0;

/* AFE_CBIP_SLV_DECODER_MON0 */
pub const CBIP_SLV_DECODER_ERR_DOMAIN_SFT: u32 = 4;
pub const CBIP_SLV_DECODER_ERR_DOMAIN_MASK: u32 = 0x1;
pub const CBIP_SLV_DECODER_ERR_DOMAIN_MASK_SFT: u32 = 0x1 << 4;
pub const CBIP_SLV_DECODER_ERR_ID_SFT: u32 = 3;
pub const CBIP_SLV_DECODER_ERR_ID_MASK: u32 = 0x1;
pub const CBIP_SLV_DECODER_ERR_ID_MASK_SFT: u32 = 0x1 << 3;
pub const CBIP_SLV_DECODER_ERR_RW_SFT: u32 = 2;
pub const CBIP_SLV_DECODER_ERR_RW_MASK: u32 = 0x1;
pub const CBIP_SLV_DECODER_ERR_RW_MASK_SFT: u32 = 0x1 << 2;
pub const CBIP_SLV_DECODER_ERR_DECERR_SFT: u32 = 1;
pub const CBIP_SLV_DECODER_ERR_DECERR_MASK: u32 = 0x1;
pub const CBIP_SLV_DECODER_ERR_DECERR_MASK_SFT: u32 = 0x1 << 1;
pub const CBIP_SLV_DECODER_CTRL_UPDATE_STATUS_SFT: u32 = 0;
pub const CBIP_SLV_DECODER_CTRL_UPDATE_STATUS_MASK: u32 = 0x1;
pub const CBIP_SLV_DECODER_CTRL_UPDATE_STATUS_MASK_SFT: u32 = 0x1 << 0;

/* AFE_CBIP_SLV_DECODER_MON1 */
pub const CBIP_SLV_DECODER_ERR_ADDR_SFT: u32 = 0;
pub const CBIP_SLV_DECODER_ERR_ADDR_MASK: u32 = 0xffffffff;
pub const CBIP_SLV_DECODER_ERR_ADDR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_CBIP_SLV_MUX_MON_CFG */
pub const CBIP_SLV_MUX_ERR_FLAG_EN_SFT: u32 = 3;
pub const CBIP_SLV_MUX_ERR_FLAG_EN_MASK: u32 = 0x1;
pub const CBIP_SLV_MUX_ERR_FLAG_EN_MASK_SFT: u32 = 0x1 << 3;
pub const CBIP_SLV_MUX_REG_SLAVE_WAY_EN_SFT: u32 = 2;
pub const CBIP_SLV_MUX_REG_SLAVE_WAY_EN_MASK: u32 = 0x1;
pub const CBIP_SLV_MUX_REG_SLAVE_WAY_EN_MASK_SFT: u32 = 0x1 << 2;
pub const CBIP_SLV_MUX_REG_LAYER_WAY_EN_SFT: u32 = 0;
pub const CBIP_SLV_MUX_REG_LAYER_WAY_EN_MASK: u32 = 0x3;
pub const CBIP_SLV_MUX_REG_LAYER_WAY_EN_MASK_SFT: u32 = 0x3 << 0;

/* AFE_CBIP_SLV_MUX_MON0 */
pub const CBIP_SLV_MUX_ERR_DOMAIN_SFT: u32 = 8;
pub const CBIP_SLV_MUX_ERR_DOMAIN_MASK: u32 = 0x1;
pub const CBIP_SLV_MUX_ERR_DOMAIN_MASK_SFT: u32 = 0x1 << 8;
pub const CBIP_SLV_MUX_ERR_ID_SFT: u32 = 7;
pub const CBIP_SLV_MUX_ERR_ID_MASK: u32 = 0x1;
pub const CBIP_SLV_MUX_ERR_ID_MASK_SFT: u32 = 0x1 << 7;
pub const CBIP_SLV_MUX_ERR_RD_SFT: u32 = 6;
pub const CBIP_SLV_MUX_ERR_RD_MASK: u32 = 0x1;
pub const CBIP_SLV_MUX_ERR_RD_MASK_SFT: u32 = 0x1 << 6;
pub const CBIP_SLV_MUX_ERR_WR_SFT: u32 = 5;
pub const CBIP_SLV_MUX_ERR_WR_MASK: u32 = 0x1;
pub const CBIP_SLV_MUX_ERR_WR_MASK_SFT: u32 = 0x1 << 5;
pub const CBIP_SLV_MUX_ERR_EN_SLV_SFT: u32 = 4;
pub const CBIP_SLV_MUX_ERR_EN_SLV_MASK: u32 = 0x1;
pub const CBIP_SLV_MUX_ERR_EN_SLV_MASK_SFT: u32 = 0x1 << 4;
pub const CBIP_SLV_MUX_ERR_EN_MST_SFT: u32 = 2;
pub const CBIP_SLV_MUX_ERR_EN_MST_MASK: u32 = 0x3;
pub const CBIP_SLV_MUX_ERR_EN_MST_MASK_SFT: u32 = 0x3 << 2;
pub const CBIP_SLV_MUX_CTRL_UPDATE_STATUS_SFT: u32 = 0;
pub const CBIP_SLV_MUX_CTRL_UPDATE_STATUS_MASK: u32 = 0x3;
pub const CBIP_SLV_MUX_CTRL_UPDATE_STATUS_MASK_SFT: u32 = 0x3 << 0;

/* AFE_CBIP_SLV_MUX_MON1 */
pub const CBIP_SLV_MUX_ERR_ADDR_SFT: u32 = 0;
pub const CBIP_SLV_MUX_ERR_ADDR_MASK: u32 = 0xffffffff;
pub const CBIP_SLV_MUX_ERR_ADDR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_MEMIF_CON0 */
pub const CPU_COMPACT_MODE_SFT: u32 = 2;
pub const CPU_COMPACT_MODE_MASK: u32 = 0x1;
pub const CPU_COMPACT_MODE_MASK_SFT: u32 = 0x1 << 2;
pub const CPU_HD_ALIGN_SFT: u32 = 1;
pub const CPU_HD_ALIGN_MASK: u32 = 0x1;
pub const CPU_HD_ALIGN_MASK_SFT: u32 = 0x1 << 1;
pub const SYSRAM_SIGN_SFT: u32 = 0;
pub const SYSRAM_SIGN_MASK: u32 = 0x1;
pub const SYSRAM_SIGN_MASK_SFT: u32 = 0x1 << 0;

/* AFE_MEMIF_ONE_HEART */
pub const DL_ONE_HEART_ON_2_SFT: u32 = 2;
pub const DL_ONE_HEART_ON_2_MASK: u32 = 0x1;
pub const DL_ONE_HEART_ON_2_MASK_SFT: u32 = 0x1 << 2;
pub const DL_ONE_HEART_ON_1_SFT: u32 = 1;
pub const DL_ONE_HEART_ON_1_MASK: u32 = 0x1;
pub const DL_ONE_HEART_ON_1_MASK_SFT: u32 = 0x1 << 1;
pub const DL_ONE_HEART_ON_0_SFT: u32 = 0;
pub const DL_ONE_HEART_ON_0_MASK: u32 = 0x1;
pub const DL_ONE_HEART_ON_0_MASK_SFT: u32 = 0x1 << 0;

/* AFE_DL0_BASE_MSB */
pub const DL0_BASE_ADDR_MSB_SFT: u32 = 0;
pub const DL0_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL0_BASE_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL0_BASE */
pub const DL0_BASE_ADDR_SFT: u32 = 4;
pub const DL0_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL0_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_DL0_CUR_MSB */
pub const DL0_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL0_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL0_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL0_CUR */
pub const DL0_CUR_PTR_SFT: u32 = 0;
pub const DL0_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL0_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL0_END_MSB */
pub const DL0_END_ADDR_MSB_SFT: u32 = 0;
pub const DL0_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL0_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL0_END */
pub const DL0_END_ADDR_SFT: u32 = 4;
pub const DL0_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL0_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_DL0_RCH_MON */
pub const DL0_RCH_DATA_SFT: u32 = 0;
pub const DL0_RCH_DATA_MASK: u32 = 0xffffffff;
pub const DL0_RCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL0_LCH_MON */
pub const DL0_LCH_DATA_SFT: u32 = 0;
pub const DL0_LCH_DATA_MASK: u32 = 0xffffffff;
pub const DL0_LCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL0_CON0 */
pub const DL0_ON_SFT: u32 = 28;
pub const DL0_ON_MASK: u32 = 0x1;
pub const DL0_ON_MASK_SFT: u32 = 0x1 << 28;
pub const DL0_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL0_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL0_ONE_HEART_SEL_MASK_SFT: u32 = 0x3 << 22;
pub const DL0_MINLEN_SFT: u32 = 20;
pub const DL0_MINLEN_MASK: u32 = 0x3;
pub const DL0_MINLEN_MASK_SFT: u32 = 0x3 << 20;
pub const DL0_MAXLEN_SFT: u32 = 16;
pub const DL0_MAXLEN_MASK: u32 = 0x3;
pub const DL0_MAXLEN_MASK_SFT: u32 = 0x3 << 16;
pub const DL0_SEL_DOMAIN_SFT: u32 = 13;
pub const DL0_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL0_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const DL0_SEL_FS_SFT: u32 = 8;
pub const DL0_SEL_FS_MASK: u32 = 0x1f;
pub const DL0_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const DL0_SW_CLEAR_BUF_EMPTY_SFT: u32 = 7;
pub const DL0_SW_CLEAR_BUF_EMPTY_MASK: u32 = 0x1;
pub const DL0_SW_CLEAR_BUF_EMPTY_MASK_SFT: u32 = 0x1 << 7;
pub const DL0_PBUF_SIZE_SFT: u32 = 5;
pub const DL0_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL0_PBUF_SIZE_MASK_SFT: u32 = 0x3 << 5;
pub const DL0_MONO_SFT: u32 = 4;
pub const DL0_MONO_MASK: u32 = 0x1;
pub const DL0_MONO_MASK_SFT: u32 = 0x1 << 4;
pub const DL0_NORMAL_MODE_SFT: u32 = 3;
pub const DL0_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL0_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 3;
pub const DL0_HALIGN_SFT: u32 = 2;
pub const DL0_HALIGN_MASK: u32 = 0x1;
pub const DL0_HALIGN_MASK_SFT: u32 = 0x1 << 2;
pub const DL0_HD_MODE_SFT: u32 = 0;
pub const DL0_HD_MODE_MASK: u32 = 0x3;
pub const DL0_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_DL0_MON0 */
pub const RESERVED_01_SFT: u32 = 20;
pub const RESERVED_01_MASK: u32 = 0xfff;
pub const RESERVED_01_MASK_SFT: u32 = 0xfff << 20;
pub const MEM_REQ_PENDING_SFT: u32 = 19;
pub const MEM_REQ_PENDING_MASK: u32 = 0x1;
pub const MEM_REQ_PENDING_MASK_SFT: u32 = 0x1 << 19;
pub const BUF_EMPTY_SFT: u32 = 18;
pub const BUF_EMPTY_MASK: u32 = 0x1;
pub const BUF_EMPTY_MASK_SFT: u32 = 0x1 << 18;
pub const ENABLE_SYNC_MEM_SFT: u32 = 17;
pub const ENABLE_SYNC_MEM_MASK: u32 = 0x1;
pub const ENABLE_SYNC_MEM_MASK_SFT: u32 = 0x1 << 17;
pub const ENABLE_SYNC_AGENT_SFT: u32 = 16;
pub const ENABLE_SYNC_AGENT_MASK: u32 = 0x1;
pub const ENABLE_SYNC_AGENT_MASK_SFT: u32 = 0x1 << 16;
pub const RESERVED_02_SFT: u32 = 6;
pub const RESERVED_02_MASK: u32 = 0x3ff;
pub const RESERVED_02_MASK_SFT: u32 = 0x3ff << 6;
pub const MEM_ADDR_DIFF_SFT: u32 = 0;
pub const MEM_ADDR_DIFF_MASK: u32 = 0x3f;
pub const MEM_ADDR_DIFF_MASK_SFT: u32 = 0x3f << 0;

/* AFE_DL1_BASE_MSB */
pub const DL1_BASE_ADDR_MSB_SFT: u32 = 0;
pub const DL1_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL1_BASE_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL1_BASE */
pub const DL1_BASE_ADDR_SFT: u32 = 4;
pub const DL1_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL1_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_DL1_CUR_MSB */
pub const DL1_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL1_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL1_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL1_CUR */
pub const DL1_CUR_PTR_SFT: u32 = 0;
pub const DL1_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL1_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL1_END_MSB */
pub const DL1_END_ADDR_MSB_SFT: u32 = 0;
pub const DL1_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL1_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL1_END */
pub const DL1_END_ADDR_SFT: u32 = 4;
pub const DL1_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL1_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_DL1_RCH_MON */
pub const DL1_RCH_DATA_SFT: u32 = 0;
pub const DL1_RCH_DATA_MASK: u32 = 0xffffffff;
pub const DL1_RCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL1_LCH_MON */
pub const DL1_LCH_DATA_SFT: u32 = 0;
pub const DL1_LCH_DATA_MASK: u32 = 0xffffffff;
pub const DL1_LCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL1_CON0 */
pub const DL1_ON_SFT: u32 = 28;
pub const DL1_ON_MASK: u32 = 0x1;
pub const DL1_ON_MASK_SFT: u32 = 0x1 << 28;
pub const DL1_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL1_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL1_ONE_HEART_SEL_MASK_SFT: u32 = 0x3 << 22;
pub const DL1_MINLEN_SFT: u32 = 20;
pub const DL1_MINLEN_MASK: u32 = 0x3;
pub const DL1_MINLEN_MASK_SFT: u32 = 0x3 << 20;
pub const DL1_MAXLEN_SFT: u32 = 16;
pub const DL1_MAXLEN_MASK: u32 = 0x3;
pub const DL1_MAXLEN_MASK_SFT: u32 = 0x3 << 16;
pub const DL1_SEL_DOMAIN_SFT: u32 = 13;
pub const DL1_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL1_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const DL1_SEL_FS_SFT: u32 = 8;
pub const DL1_SEL_FS_MASK: u32 = 0x1f;
pub const DL1_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const DL1_SW_CLEAR_BUF_EMPTY_SFT: u32 = 7;
pub const DL1_SW_CLEAR_BUF_EMPTY_MASK: u32 = 0x1;
pub const DL1_SW_CLEAR_BUF_EMPTY_MASK_SFT: u32 = 0x1 << 7;
pub const DL1_PBUF_SIZE_SFT: u32 = 5;
pub const DL1_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL1_PBUF_SIZE_MASK_SFT: u32 = 0x3 << 5;
pub const DL1_MONO_SFT: u32 = 4;
pub const DL1_MONO_MASK: u32 = 0x1;
pub const DL1_MONO_MASK_SFT: u32 = 0x1 << 4;
pub const DL1_NORMAL_MODE_SFT: u32 = 3;
pub const DL1_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL1_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 3;
pub const DL1_HALIGN_SFT: u32 = 2;
pub const DL1_HALIGN_MASK: u32 = 0x1;
pub const DL1_HALIGN_MASK_SFT: u32 = 0x1 << 2;
pub const DL1_HD_MODE_SFT: u32 = 0;
pub const DL1_HD_MODE_MASK: u32 = 0x3;
pub const DL1_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_DL1_MON0 */
/* duplicate C macro: RESERVED_01_SFT = 20 */
/* duplicate C macro: RESERVED_01_MASK = 0xfff */
/* duplicate C macro: RESERVED_01_MASK_SFT = 0xfff << 20 */
/* duplicate C macro: MEM_REQ_PENDING_SFT = 19 */
/* duplicate C macro: MEM_REQ_PENDING_MASK = 0x1 */
/* duplicate C macro: MEM_REQ_PENDING_MASK_SFT = 0x1 << 19 */
/* duplicate C macro: BUF_EMPTY_SFT = 18 */
/* duplicate C macro: BUF_EMPTY_MASK = 0x1 */
/* duplicate C macro: BUF_EMPTY_MASK_SFT = 0x1 << 18 */
/* duplicate C macro: ENABLE_SYNC_MEM_SFT = 17 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK_SFT = 0x1 << 17 */
/* duplicate C macro: ENABLE_SYNC_AGENT_SFT = 16 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK_SFT = 0x1 << 16 */
/* duplicate C macro: RESERVED_02_SFT = 6 */
/* duplicate C macro: RESERVED_02_MASK = 0x3ff */
/* duplicate C macro: RESERVED_02_MASK_SFT = 0x3ff << 6 */
/* duplicate C macro: MEM_ADDR_DIFF_SFT = 0 */
/* duplicate C macro: MEM_ADDR_DIFF_MASK = 0x3f */
/* duplicate C macro: MEM_ADDR_DIFF_MASK_SFT = 0x3f << 0 */

/* AFE_DL2_BASE_MSB */
pub const DL2_BASE__ADDR_MSB_SFT: u32 = 0;
pub const DL2_BASE__ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL2_BASE__ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL2_BASE */
pub const DL2_BASE_ADDR_SFT: u32 = 4;
pub const DL2_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL2_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_DL2_CUR_MSB */
pub const DL2_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL2_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL2_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL2_CUR */
pub const DL2_CUR_PTR_SFT: u32 = 0;
pub const DL2_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL2_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL2_END_MSB */
pub const DL2_END_ADDR_MSB_SFT: u32 = 0;
pub const DL2_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL2_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL2_END */
pub const DL2_END_ADDR_SFT: u32 = 4;
pub const DL2_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL2_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_DL2_RCH_MON */
pub const DL2_RCH_DATA_SFT: u32 = 0;
pub const DL2_RCH_DATA_MASK: u32 = 0xffffffff;
pub const DL2_RCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL2_LCH_MON */
pub const DL2_LCH_DATA_SFT: u32 = 0;
pub const DL2_LCH_DATA_MASK: u32 = 0xffffffff;
pub const DL2_LCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL2_CON0 */
pub const DL2_ON_SFT: u32 = 28;
pub const DL2_ON_MASK: u32 = 0x1;
pub const DL2_ON_MASK_SFT: u32 = 0x1 << 28;
pub const DL2_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL2_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL2_ONE_HEART_SEL_MASK_SFT: u32 = 0x3 << 22;
pub const DL2_MINLEN_SFT: u32 = 20;
pub const DL2_MINLEN_MASK: u32 = 0x3;
pub const DL2_MINLEN_MASK_SFT: u32 = 0x3 << 20;
pub const DL2_MAXLEN_SFT: u32 = 16;
pub const DL2_MAXLEN_MASK: u32 = 0x3;
pub const DL2_MAXLEN_MASK_SFT: u32 = 0x3 << 16;
pub const DL2_SEL_DOMAIN_SFT: u32 = 13;
pub const DL2_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL2_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const DL2_SEL_FS_SFT: u32 = 8;
pub const DL2_SEL_FS_MASK: u32 = 0x1f;
pub const DL2_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const DL2_SW_CLEAR_BUF_EMPTY_SFT: u32 = 7;
pub const DL2_SW_CLEAR_BUF_EMPTY_MASK: u32 = 0x1;
pub const DL2_SW_CLEAR_BUF_EMPTY_MASK_SFT: u32 = 0x1 << 7;
pub const DL2_PBUF_SIZE_SFT: u32 = 5;
pub const DL2_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL2_PBUF_SIZE_MASK_SFT: u32 = 0x3 << 5;
pub const DL2_MONO_SFT: u32 = 4;
pub const DL2_MONO_MASK: u32 = 0x1;
pub const DL2_MONO_MASK_SFT: u32 = 0x1 << 4;
pub const DL2_NORMAL_MODE_SFT: u32 = 3;
pub const DL2_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL2_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 3;
pub const DL2_HALIGN_SFT: u32 = 2;
pub const DL2_HALIGN_MASK: u32 = 0x1;
pub const DL2_HALIGN_MASK_SFT: u32 = 0x1 << 2;
pub const DL2_HD_MODE_SFT: u32 = 0;
pub const DL2_HD_MODE_MASK: u32 = 0x3;
pub const DL2_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_DL2_MON0 */
/* duplicate C macro: RESERVED_01_SFT = 20 */
/* duplicate C macro: RESERVED_01_MASK = 0xfff */
/* duplicate C macro: RESERVED_01_MASK_SFT = 0xfff << 20 */
/* duplicate C macro: MEM_REQ_PENDING_SFT = 19 */
/* duplicate C macro: MEM_REQ_PENDING_MASK = 0x1 */
/* duplicate C macro: MEM_REQ_PENDING_MASK_SFT = 0x1 << 19 */
/* duplicate C macro: BUF_EMPTY_SFT = 18 */
/* duplicate C macro: BUF_EMPTY_MASK = 0x1 */
/* duplicate C macro: BUF_EMPTY_MASK_SFT = 0x1 << 18 */
/* duplicate C macro: ENABLE_SYNC_MEM_SFT = 17 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK_SFT = 0x1 << 17 */
/* duplicate C macro: ENABLE_SYNC_AGENT_SFT = 16 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK_SFT = 0x1 << 16 */
/* duplicate C macro: RESERVED_02_SFT = 6 */
/* duplicate C macro: RESERVED_02_MASK = 0x3ff */
/* duplicate C macro: RESERVED_02_MASK_SFT = 0x3ff << 6 */
/* duplicate C macro: MEM_ADDR_DIFF_SFT = 0 */
/* duplicate C macro: MEM_ADDR_DIFF_MASK = 0x3f */
/* duplicate C macro: MEM_ADDR_DIFF_MASK_SFT = 0x3f << 0 */

/* AFE_DL3_BASE_MSB */
pub const DL3_BASE__ADDR_MSB_SFT: u32 = 0;
pub const DL3_BASE__ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL3_BASE__ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL3_BASE */
pub const DL3_BASE_ADDR_SFT: u32 = 4;
pub const DL3_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL3_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_DL3_CUR_MSB */
pub const DL3_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL3_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL3_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL3_CUR */
pub const DL3_CUR_PTR_SFT: u32 = 0;
pub const DL3_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL3_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL3_END_MSB */
pub const DL3_END_ADDR_MSB_SFT: u32 = 0;
pub const DL3_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL3_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL3_END */
pub const DL3_END_ADDR_SFT: u32 = 4;
pub const DL3_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL3_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_DL3_RCH_MON */
pub const DL3_RCH_DATA_SFT: u32 = 0;
pub const DL3_RCH_DATA_MASK: u32 = 0xffffffff;
pub const DL3_RCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL3_LCH_MON */
pub const DL3_LCH_DATA_SFT: u32 = 0;
pub const DL3_LCH_DATA_MASK: u32 = 0xffffffff;
pub const DL3_LCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL3_CON0 */
pub const DL3_ON_SFT: u32 = 28;
pub const DL3_ON_MASK: u32 = 0x1;
pub const DL3_ON_MASK_SFT: u32 = 0x1 << 28;
pub const DL3_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL3_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL3_ONE_HEART_SEL_MASK_SFT: u32 = 0x3 << 22;
pub const DL3_MINLEN_SFT: u32 = 20;
pub const DL3_MINLEN_MASK: u32 = 0x3;
pub const DL3_MINLEN_MASK_SFT: u32 = 0x3 << 20;
pub const DL3_MAXLEN_SFT: u32 = 16;
pub const DL3_MAXLEN_MASK: u32 = 0x3;
pub const DL3_MAXLEN_MASK_SFT: u32 = 0x3 << 16;
pub const DL3_SEL_DOMAIN_SFT: u32 = 13;
pub const DL3_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL3_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const DL3_SEL_FS_SFT: u32 = 8;
pub const DL3_SEL_FS_MASK: u32 = 0x1f;
pub const DL3_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const DL3_SW_CLEAR_BUF_EMPTY_SFT: u32 = 7;
pub const DL3_SW_CLEAR_BUF_EMPTY_MASK: u32 = 0x1;
pub const DL3_SW_CLEAR_BUF_EMPTY_MASK_SFT: u32 = 0x1 << 7;
pub const DL3_PBUF_SIZE_SFT: u32 = 5;
pub const DL3_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL3_PBUF_SIZE_MASK_SFT: u32 = 0x3 << 5;
pub const DL3_MONO_SFT: u32 = 4;
pub const DL3_MONO_MASK: u32 = 0x1;
pub const DL3_MONO_MASK_SFT: u32 = 0x1 << 4;
pub const DL3_NORMAL_MODE_SFT: u32 = 3;
pub const DL3_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL3_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 3;
pub const DL3_HALIGN_SFT: u32 = 2;
pub const DL3_HALIGN_MASK: u32 = 0x1;
pub const DL3_HALIGN_MASK_SFT: u32 = 0x1 << 2;
pub const DL3_HD_MODE_SFT: u32 = 0;
pub const DL3_HD_MODE_MASK: u32 = 0x3;
pub const DL3_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_DL3_MON0 */
/* duplicate C macro: RESERVED_01_SFT = 20 */
/* duplicate C macro: RESERVED_01_MASK = 0xfff */
/* duplicate C macro: RESERVED_01_MASK_SFT = 0xfff << 20 */
/* duplicate C macro: MEM_REQ_PENDING_SFT = 19 */
/* duplicate C macro: MEM_REQ_PENDING_MASK = 0x1 */
/* duplicate C macro: MEM_REQ_PENDING_MASK_SFT = 0x1 << 19 */
/* duplicate C macro: BUF_EMPTY_SFT = 18 */
/* duplicate C macro: BUF_EMPTY_MASK = 0x1 */
/* duplicate C macro: BUF_EMPTY_MASK_SFT = 0x1 << 18 */
/* duplicate C macro: ENABLE_SYNC_MEM_SFT = 17 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK_SFT = 0x1 << 17 */
/* duplicate C macro: ENABLE_SYNC_AGENT_SFT = 16 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK_SFT = 0x1 << 16 */
/* duplicate C macro: RESERVED_02_SFT = 6 */
/* duplicate C macro: RESERVED_02_MASK = 0x3ff */
/* duplicate C macro: RESERVED_02_MASK_SFT = 0x3ff << 6 */
/* duplicate C macro: MEM_ADDR_DIFF_SFT = 0 */
/* duplicate C macro: MEM_ADDR_DIFF_MASK = 0x3f */
/* duplicate C macro: MEM_ADDR_DIFF_MASK_SFT = 0x3f << 0 */

/* AFE_DL4_BASE_MSB */
pub const DL4_BASE__ADDR_MSB_SFT: u32 = 0;
pub const DL4_BASE__ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL4_BASE__ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL4_BASE */
pub const DL4_BASE_ADDR_SFT: u32 = 4;
pub const DL4_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL4_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_DL4_CUR_MSB */
pub const DL4_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL4_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL4_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL4_CUR */
pub const DL4_CUR_PTR_SFT: u32 = 0;
pub const DL4_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL4_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL4_END_MSB */
pub const DL4_END_ADDR_MSB_SFT: u32 = 0;
pub const DL4_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL4_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL4_END */
pub const DL4_END_ADDR_SFT: u32 = 4;
pub const DL4_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL4_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_DL4_RCH_MON */
pub const DL4_RCH_DATA_SFT: u32 = 0;
pub const DL4_RCH_DATA_MASK: u32 = 0xffffffff;
pub const DL4_RCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL4_LCH_MON */
pub const DL4_LCH_DATA_SFT: u32 = 0;
pub const DL4_LCH_DATA_MASK: u32 = 0xffffffff;
pub const DL4_LCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL4_CON0 */
pub const DL4_ON_SFT: u32 = 28;
pub const DL4_ON_MASK: u32 = 0x1;
pub const DL4_ON_MASK_SFT: u32 = 0x1 << 28;
pub const DL4_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL4_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL4_ONE_HEART_SEL_MASK_SFT: u32 = 0x3 << 22;
pub const DL4_MINLEN_SFT: u32 = 20;
pub const DL4_MINLEN_MASK: u32 = 0x3;
pub const DL4_MINLEN_MASK_SFT: u32 = 0x3 << 20;
pub const DL4_MAXLEN_SFT: u32 = 16;
pub const DL4_MAXLEN_MASK: u32 = 0x3;
pub const DL4_MAXLEN_MASK_SFT: u32 = 0x3 << 16;
pub const DL4_SEL_DOMAIN_SFT: u32 = 13;
pub const DL4_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL4_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const DL4_SEL_FS_SFT: u32 = 8;
pub const DL4_SEL_FS_MASK: u32 = 0x1f;
pub const DL4_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const DL4_SW_CLEAR_BUF_EMPTY_SFT: u32 = 7;
pub const DL4_SW_CLEAR_BUF_EMPTY_MASK: u32 = 0x1;
pub const DL4_SW_CLEAR_BUF_EMPTY_MASK_SFT: u32 = 0x1 << 7;
pub const DL4_PBUF_SIZE_SFT: u32 = 5;
pub const DL4_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL4_PBUF_SIZE_MASK_SFT: u32 = 0x3 << 5;
pub const DL4_MONO_SFT: u32 = 4;
pub const DL4_MONO_MASK: u32 = 0x1;
pub const DL4_MONO_MASK_SFT: u32 = 0x1 << 4;
pub const DL4_NORMAL_MODE_SFT: u32 = 3;
pub const DL4_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL4_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 3;
pub const DL4_HALIGN_SFT: u32 = 2;
pub const DL4_HALIGN_MASK: u32 = 0x1;
pub const DL4_HALIGN_MASK_SFT: u32 = 0x1 << 2;
pub const DL4_HD_MODE_SFT: u32 = 0;
pub const DL4_HD_MODE_MASK: u32 = 0x3;
pub const DL4_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_DL4_MON0 */
/* duplicate C macro: RESERVED_01_SFT = 20 */
/* duplicate C macro: RESERVED_01_MASK = 0xfff */
/* duplicate C macro: RESERVED_01_MASK_SFT = 0xfff << 20 */
/* duplicate C macro: MEM_REQ_PENDING_SFT = 19 */
/* duplicate C macro: MEM_REQ_PENDING_MASK = 0x1 */
/* duplicate C macro: MEM_REQ_PENDING_MASK_SFT = 0x1 << 19 */
/* duplicate C macro: BUF_EMPTY_SFT = 18 */
/* duplicate C macro: BUF_EMPTY_MASK = 0x1 */
/* duplicate C macro: BUF_EMPTY_MASK_SFT = 0x1 << 18 */
/* duplicate C macro: ENABLE_SYNC_MEM_SFT = 17 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK_SFT = 0x1 << 17 */
/* duplicate C macro: ENABLE_SYNC_AGENT_SFT = 16 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK_SFT = 0x1 << 16 */
/* duplicate C macro: RESERVED_02_SFT = 6 */
/* duplicate C macro: RESERVED_02_MASK = 0x3ff */
/* duplicate C macro: RESERVED_02_MASK_SFT = 0x3ff << 6 */
/* duplicate C macro: MEM_ADDR_DIFF_SFT = 0 */
/* duplicate C macro: MEM_ADDR_DIFF_MASK = 0x3f */
/* duplicate C macro: MEM_ADDR_DIFF_MASK_SFT = 0x3f << 0 */

/* AFE_DL5_BASE_MSB */
pub const DL5_BASE__ADDR_MSB_SFT: u32 = 0;
pub const DL5_BASE__ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL5_BASE__ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL5_BASE */
pub const DL5_BASE_ADDR_SFT: u32 = 4;
pub const DL5_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL5_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_DL5_CUR_MSB */
pub const DL5_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL5_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL5_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL5_CUR */
pub const DL5_CUR_PTR_SFT: u32 = 0;
pub const DL5_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL5_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL5_END_MSB */
pub const DL5_END_ADDR_MSB_SFT: u32 = 0;
pub const DL5_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL5_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL5_END */
pub const DL5_END_ADDR_SFT: u32 = 4;
pub const DL5_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL5_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_DL5_RCH_MON */
pub const DL5_RCH_DATA_SFT: u32 = 0;
pub const DL5_RCH_DATA_MASK: u32 = 0xffffffff;
pub const DL5_RCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL5_LCH_MON */
pub const DL5_LCH_DATA_SFT: u32 = 0;
pub const DL5_LCH_DATA_MASK: u32 = 0xffffffff;
pub const DL5_LCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL5_CON0 */
pub const DL5_ON_SFT: u32 = 28;
pub const DL5_ON_MASK: u32 = 0x1;
pub const DL5_ON_MASK_SFT: u32 = 0x1 << 28;
pub const DL5_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL5_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL5_ONE_HEART_SEL_MASK_SFT: u32 = 0x3 << 22;
pub const DL5_MINLEN_SFT: u32 = 20;
pub const DL5_MINLEN_MASK: u32 = 0x3;
pub const DL5_MINLEN_MASK_SFT: u32 = 0x3 << 20;
pub const DL5_MAXLEN_SFT: u32 = 16;
pub const DL5_MAXLEN_MASK: u32 = 0x3;
pub const DL5_MAXLEN_MASK_SFT: u32 = 0x3 << 16;
pub const DL5_SEL_DOMAIN_SFT: u32 = 13;
pub const DL5_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL5_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const DL5_SEL_FS_SFT: u32 = 8;
pub const DL5_SEL_FS_MASK: u32 = 0x1f;
pub const DL5_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const DL5_SW_CLEAR_BUF_EMPTY_SFT: u32 = 7;
pub const DL5_SW_CLEAR_BUF_EMPTY_MASK: u32 = 0x1;
pub const DL5_SW_CLEAR_BUF_EMPTY_MASK_SFT: u32 = 0x1 << 7;
pub const DL5_PBUF_SIZE_SFT: u32 = 5;
pub const DL5_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL5_PBUF_SIZE_MASK_SFT: u32 = 0x3 << 5;
pub const DL5_MONO_SFT: u32 = 4;
pub const DL5_MONO_MASK: u32 = 0x1;
pub const DL5_MONO_MASK_SFT: u32 = 0x1 << 4;
pub const DL5_NORMAL_MODE_SFT: u32 = 3;
pub const DL5_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL5_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 3;
pub const DL5_HALIGN_SFT: u32 = 2;
pub const DL5_HALIGN_MASK: u32 = 0x1;
pub const DL5_HALIGN_MASK_SFT: u32 = 0x1 << 2;
pub const DL5_HD_MODE_SFT: u32 = 0;
pub const DL5_HD_MODE_MASK: u32 = 0x3;
pub const DL5_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_DL5_MON0 */
/* duplicate C macro: RESERVED_01_SFT = 20 */
/* duplicate C macro: RESERVED_01_MASK = 0xfff */
/* duplicate C macro: RESERVED_01_MASK_SFT = 0xfff << 20 */
/* duplicate C macro: MEM_REQ_PENDING_SFT = 19 */
/* duplicate C macro: MEM_REQ_PENDING_MASK = 0x1 */
/* duplicate C macro: MEM_REQ_PENDING_MASK_SFT = 0x1 << 19 */
/* duplicate C macro: BUF_EMPTY_SFT = 18 */
/* duplicate C macro: BUF_EMPTY_MASK = 0x1 */
/* duplicate C macro: BUF_EMPTY_MASK_SFT = 0x1 << 18 */
/* duplicate C macro: ENABLE_SYNC_MEM_SFT = 17 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK_SFT = 0x1 << 17 */
/* duplicate C macro: ENABLE_SYNC_AGENT_SFT = 16 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK_SFT = 0x1 << 16 */
/* duplicate C macro: RESERVED_02_SFT = 6 */
/* duplicate C macro: RESERVED_02_MASK = 0x3ff */
/* duplicate C macro: RESERVED_02_MASK_SFT = 0x3ff << 6 */
/* duplicate C macro: MEM_ADDR_DIFF_SFT = 0 */
/* duplicate C macro: MEM_ADDR_DIFF_MASK = 0x3f */
/* duplicate C macro: MEM_ADDR_DIFF_MASK_SFT = 0x3f << 0 */

/* AFE_DL6_BASE_MSB */
pub const DL6_BASE__ADDR_MSB_SFT: u32 = 0;
pub const DL6_BASE__ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL6_BASE__ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL6_BASE */
pub const DL6_BASE_ADDR_SFT: u32 = 4;
pub const DL6_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL6_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_DL6_CUR_MSB */
pub const DL6_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL6_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL6_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL6_CUR */
pub const DL6_CUR_PTR_SFT: u32 = 0;
pub const DL6_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL6_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL6_END_MSB */
pub const DL6_END_ADDR_MSB_SFT: u32 = 0;
pub const DL6_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL6_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL6_END */
pub const DL6_END_ADDR_SFT: u32 = 4;
pub const DL6_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL6_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_DL6_RCH_MON */
pub const DL6_RCH_DATA_SFT: u32 = 0;
pub const DL6_RCH_DATA_MASK: u32 = 0xffffffff;
pub const DL6_RCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL6_LCH_MON */
pub const DL6_LCH_DATA_SFT: u32 = 0;
pub const DL6_LCH_DATA_MASK: u32 = 0xffffffff;
pub const DL6_LCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL6_CON0 */
pub const DL6_ON_SFT: u32 = 28;
pub const DL6_ON_MASK: u32 = 0x1;
pub const DL6_ON_MASK_SFT: u32 = 0x1 << 28;
pub const DL6_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL6_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL6_ONE_HEART_SEL_MASK_SFT: u32 = 0x3 << 22;
pub const DL6_MINLEN_SFT: u32 = 20;
pub const DL6_MINLEN_MASK: u32 = 0x3;
pub const DL6_MINLEN_MASK_SFT: u32 = 0x3 << 20;
pub const DL6_MAXLEN_SFT: u32 = 16;
pub const DL6_MAXLEN_MASK: u32 = 0x3;
pub const DL6_MAXLEN_MASK_SFT: u32 = 0x3 << 16;
pub const DL6_SEL_DOMAIN_SFT: u32 = 13;
pub const DL6_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL6_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const DL6_SEL_FS_SFT: u32 = 8;
pub const DL6_SEL_FS_MASK: u32 = 0x1f;
pub const DL6_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const DL6_SW_CLEAR_BUF_EMPTY_SFT: u32 = 7;
pub const DL6_SW_CLEAR_BUF_EMPTY_MASK: u32 = 0x1;
pub const DL6_SW_CLEAR_BUF_EMPTY_MASK_SFT: u32 = 0x1 << 7;
pub const DL6_PBUF_SIZE_SFT: u32 = 5;
pub const DL6_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL6_PBUF_SIZE_MASK_SFT: u32 = 0x3 << 5;
pub const DL6_MONO_SFT: u32 = 4;
pub const DL6_MONO_MASK: u32 = 0x1;
pub const DL6_MONO_MASK_SFT: u32 = 0x1 << 4;
pub const DL6_NORMAL_MODE_SFT: u32 = 3;
pub const DL6_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL6_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 3;
pub const DL6_HALIGN_SFT: u32 = 2;
pub const DL6_HALIGN_MASK: u32 = 0x1;
pub const DL6_HALIGN_MASK_SFT: u32 = 0x1 << 2;
pub const DL6_HD_MODE_SFT: u32 = 0;
pub const DL6_HD_MODE_MASK: u32 = 0x3;
pub const DL6_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_DL6_MON0 */
/* duplicate C macro: RESERVED_01_SFT = 20 */
/* duplicate C macro: RESERVED_01_MASK = 0xfff */
/* duplicate C macro: RESERVED_01_MASK_SFT = 0xfff << 20 */
/* duplicate C macro: MEM_REQ_PENDING_SFT = 19 */
/* duplicate C macro: MEM_REQ_PENDING_MASK = 0x1 */
/* duplicate C macro: MEM_REQ_PENDING_MASK_SFT = 0x1 << 19 */
/* duplicate C macro: BUF_EMPTY_SFT = 18 */
/* duplicate C macro: BUF_EMPTY_MASK = 0x1 */
/* duplicate C macro: BUF_EMPTY_MASK_SFT = 0x1 << 18 */
/* duplicate C macro: ENABLE_SYNC_MEM_SFT = 17 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK_SFT = 0x1 << 17 */
/* duplicate C macro: ENABLE_SYNC_AGENT_SFT = 16 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK_SFT = 0x1 << 16 */
/* duplicate C macro: RESERVED_02_SFT = 6 */
/* duplicate C macro: RESERVED_02_MASK = 0x3ff */
/* duplicate C macro: RESERVED_02_MASK_SFT = 0x3ff << 6 */
/* duplicate C macro: MEM_ADDR_DIFF_SFT = 0 */
/* duplicate C macro: MEM_ADDR_DIFF_MASK = 0x3f */
/* duplicate C macro: MEM_ADDR_DIFF_MASK_SFT = 0x3f << 0 */

/* AFE_DL7_BASE_MSB */
pub const DL7_BASE__ADDR_MSB_SFT: u32 = 0;
pub const DL7_BASE__ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL7_BASE__ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL7_BASE */
pub const DL7_BASE_ADDR_SFT: u32 = 4;
pub const DL7_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL7_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_DL7_CUR_MSB */
pub const DL7_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL7_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL7_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL7_CUR */
pub const DL7_CUR_PTR_SFT: u32 = 0;
pub const DL7_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL7_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL7_END_MSB */
pub const DL7_END_ADDR_MSB_SFT: u32 = 0;
pub const DL7_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL7_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL7_END */
pub const DL7_END_ADDR_SFT: u32 = 4;
pub const DL7_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL7_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_DL7_RCH_MON */
pub const DL7_RCH_DATA_SFT: u32 = 0;
pub const DL7_RCH_DATA_MASK: u32 = 0xffffffff;
pub const DL7_RCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL7_LCH_MON */
pub const DL7_LCH_DATA_SFT: u32 = 0;
pub const DL7_LCH_DATA_MASK: u32 = 0xffffffff;
pub const DL7_LCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL7_CON0 */
pub const DL7_ON_SFT: u32 = 28;
pub const DL7_ON_MASK: u32 = 0x1;
pub const DL7_ON_MASK_SFT: u32 = 0x1 << 28;
pub const DL7_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL7_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL7_ONE_HEART_SEL_MASK_SFT: u32 = 0x3 << 22;
pub const DL7_MINLEN_SFT: u32 = 20;
pub const DL7_MINLEN_MASK: u32 = 0x3;
pub const DL7_MINLEN_MASK_SFT: u32 = 0x3 << 20;
pub const DL7_MAXLEN_SFT: u32 = 16;
pub const DL7_MAXLEN_MASK: u32 = 0x3;
pub const DL7_MAXLEN_MASK_SFT: u32 = 0x3 << 16;
pub const DL7_SEL_DOMAIN_SFT: u32 = 13;
pub const DL7_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL7_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const DL7_SEL_FS_SFT: u32 = 8;
pub const DL7_SEL_FS_MASK: u32 = 0x1f;
pub const DL7_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const DL7_SW_CLEAR_BUF_EMPTY_SFT: u32 = 7;
pub const DL7_SW_CLEAR_BUF_EMPTY_MASK: u32 = 0x1;
pub const DL7_SW_CLEAR_BUF_EMPTY_MASK_SFT: u32 = 0x1 << 7;
pub const DL7_PBUF_SIZE_SFT: u32 = 5;
pub const DL7_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL7_PBUF_SIZE_MASK_SFT: u32 = 0x3 << 5;
pub const DL7_MONO_SFT: u32 = 4;
pub const DL7_MONO_MASK: u32 = 0x1;
pub const DL7_MONO_MASK_SFT: u32 = 0x1 << 4;
pub const DL7_NORMAL_MODE_SFT: u32 = 3;
pub const DL7_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL7_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 3;
pub const DL7_HALIGN_SFT: u32 = 2;
pub const DL7_HALIGN_MASK: u32 = 0x1;
pub const DL7_HALIGN_MASK_SFT: u32 = 0x1 << 2;
pub const DL7_HD_MODE_SFT: u32 = 0;
pub const DL7_HD_MODE_MASK: u32 = 0x3;
pub const DL7_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_DL7_MON0 */
/* duplicate C macro: RESERVED_01_SFT = 20 */
/* duplicate C macro: RESERVED_01_MASK = 0xfff */
/* duplicate C macro: RESERVED_01_MASK_SFT = 0xfff << 20 */
/* duplicate C macro: MEM_REQ_PENDING_SFT = 19 */
/* duplicate C macro: MEM_REQ_PENDING_MASK = 0x1 */
/* duplicate C macro: MEM_REQ_PENDING_MASK_SFT = 0x1 << 19 */
/* duplicate C macro: BUF_EMPTY_SFT = 18 */
/* duplicate C macro: BUF_EMPTY_MASK = 0x1 */
/* duplicate C macro: BUF_EMPTY_MASK_SFT = 0x1 << 18 */
/* duplicate C macro: ENABLE_SYNC_MEM_SFT = 17 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK_SFT = 0x1 << 17 */
/* duplicate C macro: ENABLE_SYNC_AGENT_SFT = 16 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK_SFT = 0x1 << 16 */
/* duplicate C macro: RESERVED_02_SFT = 6 */
/* duplicate C macro: RESERVED_02_MASK = 0x3ff */
/* duplicate C macro: RESERVED_02_MASK_SFT = 0x3ff << 6 */
/* duplicate C macro: MEM_ADDR_DIFF_SFT = 0 */
/* duplicate C macro: MEM_ADDR_DIFF_MASK = 0x3f */
/* duplicate C macro: MEM_ADDR_DIFF_MASK_SFT = 0x3f << 0 */

/* AFE_DL8_BASE_MSB */
pub const DL8_BASE__ADDR_MSB_SFT: u32 = 0;
pub const DL8_BASE__ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL8_BASE__ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL8_BASE */
pub const DL8_BASE_ADDR_SFT: u32 = 4;
pub const DL8_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL8_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_DL8_CUR_MSB */
pub const DL8_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL8_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL8_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL8_CUR */
pub const DL8_CUR_PTR_SFT: u32 = 0;
pub const DL8_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL8_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL8_END_MSB */
pub const DL8_END_ADDR_MSB_SFT: u32 = 0;
pub const DL8_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL8_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL8_END */
pub const DL8_END_ADDR_SFT: u32 = 4;
pub const DL8_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL8_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_DL8_RCH_MON */
pub const DL8_RCH_DATA_SFT: u32 = 0;
pub const DL8_RCH_DATA_MASK: u32 = 0xffffffff;
pub const DL8_RCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL8_LCH_MON */
pub const DL8_LCH_DATA_SFT: u32 = 0;
pub const DL8_LCH_DATA_MASK: u32 = 0xffffffff;
pub const DL8_LCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL8_CON0 */
pub const DL8_ON_SFT: u32 = 28;
pub const DL8_ON_MASK: u32 = 0x1;
pub const DL8_ON_MASK_SFT: u32 = 0x1 << 28;
pub const DL8_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL8_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL8_ONE_HEART_SEL_MASK_SFT: u32 = 0x3 << 22;
pub const DL8_MINLEN_SFT: u32 = 20;
pub const DL8_MINLEN_MASK: u32 = 0x3;
pub const DL8_MINLEN_MASK_SFT: u32 = 0x3 << 20;
pub const DL8_MAXLEN_SFT: u32 = 16;
pub const DL8_MAXLEN_MASK: u32 = 0x3;
pub const DL8_MAXLEN_MASK_SFT: u32 = 0x3 << 16;
pub const DL8_SEL_DOMAIN_SFT: u32 = 13;
pub const DL8_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL8_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const DL8_SEL_FS_SFT: u32 = 8;
pub const DL8_SEL_FS_MASK: u32 = 0x1f;
pub const DL8_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const DL8_SW_CLEAR_BUF_EMPTY_SFT: u32 = 7;
pub const DL8_SW_CLEAR_BUF_EMPTY_MASK: u32 = 0x1;
pub const DL8_SW_CLEAR_BUF_EMPTY_MASK_SFT: u32 = 0x1 << 7;
pub const DL8_PBUF_SIZE_SFT: u32 = 5;
pub const DL8_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL8_PBUF_SIZE_MASK_SFT: u32 = 0x3 << 5;
pub const DL8_MONO_SFT: u32 = 4;
pub const DL8_MONO_MASK: u32 = 0x1;
pub const DL8_MONO_MASK_SFT: u32 = 0x1 << 4;
pub const DL8_NORMAL_MODE_SFT: u32 = 3;
pub const DL8_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL8_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 3;
pub const DL8_HALIGN_SFT: u32 = 2;
pub const DL8_HALIGN_MASK: u32 = 0x1;
pub const DL8_HALIGN_MASK_SFT: u32 = 0x1 << 2;
pub const DL8_HD_MODE_SFT: u32 = 0;
pub const DL8_HD_MODE_MASK: u32 = 0x3;
pub const DL8_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_DL8_MON0 */
/* duplicate C macro: RESERVED_01_SFT = 20 */
/* duplicate C macro: RESERVED_01_MASK = 0xfff */
/* duplicate C macro: RESERVED_01_MASK_SFT = 0xfff << 20 */
/* duplicate C macro: MEM_REQ_PENDING_SFT = 19 */
/* duplicate C macro: MEM_REQ_PENDING_MASK = 0x1 */
/* duplicate C macro: MEM_REQ_PENDING_MASK_SFT = 0x1 << 19 */
/* duplicate C macro: BUF_EMPTY_SFT = 18 */
/* duplicate C macro: BUF_EMPTY_MASK = 0x1 */
/* duplicate C macro: BUF_EMPTY_MASK_SFT = 0x1 << 18 */
/* duplicate C macro: ENABLE_SYNC_MEM_SFT = 17 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK_SFT = 0x1 << 17 */
/* duplicate C macro: ENABLE_SYNC_AGENT_SFT = 16 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK_SFT = 0x1 << 16 */
/* duplicate C macro: RESERVED_02_SFT = 6 */
/* duplicate C macro: RESERVED_02_MASK = 0x3ff */
/* duplicate C macro: RESERVED_02_MASK_SFT = 0x3ff << 6 */
/* duplicate C macro: MEM_ADDR_DIFF_SFT = 0 */
/* duplicate C macro: MEM_ADDR_DIFF_MASK = 0x3f */
/* duplicate C macro: MEM_ADDR_DIFF_MASK_SFT = 0x3f << 0 */

/* AFE_DL_24CH_BASE_MSB */
pub const DL_24CH_BASE__ADDR_MSB_SFT: u32 = 0;
pub const DL_24CH_BASE__ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL_24CH_BASE__ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL_24CH_BASE */
pub const DL_24CH_BASE_ADDR_SFT: u32 = 4;
pub const DL_24CH_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL_24CH_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_DL_24CH_CUR_MSB */
pub const DL_24CH_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL_24CH_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL_24CH_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL_24CH_CUR */
pub const DL_24CH_CUR_PTR_SFT: u32 = 0;
pub const DL_24CH_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL_24CH_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL_24CH_END_MSB */
pub const DL_24CH_END_ADDR_MSB_SFT: u32 = 0;
pub const DL_24CH_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL_24CH_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL_24CH_END */
pub const DL_24CH_END_ADDR_SFT: u32 = 4;
pub const DL_24CH_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL_24CH_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_DL_24CH_CON0 */
pub const DL_24CH_ON_SFT: u32 = 31;
pub const DL_24CH_ON_MASK: u32 = 0x1;
pub const DL_24CH_ON_MASK_SFT: u32 = 0x1 << 31;
pub const DL_24CH_NUM_SFT: u32 = 24;
pub const DL_24CH_NUM_MASK: u32 = 0x3f;
pub const DL_24CH_NUM_MASK_SFT: u32 = 0x3f << 24;
pub const DL_24CH_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL_24CH_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL_24CH_ONE_HEART_SEL_MASK_SFT: u32 = 0x3 << 22;
pub const DL_24CH_MINLEN_SFT: u32 = 20;
pub const DL_24CH_MINLEN_MASK: u32 = 0x3;
pub const DL_24CH_MINLEN_MASK_SFT: u32 = 0x3 << 20;
pub const DL_24CH_MAXLEN_SFT: u32 = 16;
pub const DL_24CH_MAXLEN_MASK: u32 = 0x3;
pub const DL_24CH_MAXLEN_MASK_SFT: u32 = 0x3 << 16;
pub const DL_24CH_SEL_DOMAIN_SFT: u32 = 13;
pub const DL_24CH_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL_24CH_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const DL_24CH_SEL_FS_SFT: u32 = 8;
pub const DL_24CH_SEL_FS_MASK: u32 = 0x1f;
pub const DL_24CH_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const DL_24CH_BUF_EMPTY_CLR_SFT: u32 = 7;
pub const DL_24CH_BUF_EMPTY_CLR_MASK: u32 = 0x1;
pub const DL_24CH_BUF_EMPTY_CLR_MASK_SFT: u32 = 0x1 << 7;
pub const DL_24CH_PBUF_SIZE_SFT: u32 = 5;
pub const DL_24CH_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL_24CH_PBUF_SIZE_MASK_SFT: u32 = 0x3 << 5;
pub const DL_24CH_HANG_CLR_SFT: u32 = 4;
pub const DL_24CH_HANG_CLR_MASK: u32 = 0x1;
pub const DL_24CH_HANG_CLR_MASK_SFT: u32 = 0x1 << 4;
pub const DL_24CH_NORMAL_MODE_SFT: u32 = 3;
pub const DL_24CH_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL_24CH_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 3;
pub const DL_24CH_HALIGN_SFT: u32 = 2;
pub const DL_24CH_HALIGN_MASK: u32 = 0x1;
pub const DL_24CH_HALIGN_MASK_SFT: u32 = 0x1 << 2;
pub const DL_24CH_HD_MODE_SFT: u32 = 0;
pub const DL_24CH_HD_MODE_MASK: u32 = 0x3;
pub const DL_24CH_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_DL23_BASE_MSB */
pub const DL23_BASE__ADDR_MSB_SFT: u32 = 0;
pub const DL23_BASE__ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL23_BASE__ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL23_BASE */
pub const DL23_BASE_ADDR_SFT: u32 = 4;
pub const DL23_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL23_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_DL23_CUR_MSB */
pub const DL23_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL23_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL23_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL23_CUR */
pub const DL23_CUR_PTR_SFT: u32 = 0;
pub const DL23_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL23_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL23_END_MSB */
pub const DL23_END_ADDR_MSB_SFT: u32 = 0;
pub const DL23_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL23_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL23_END */
pub const DL23_END_ADDR_SFT: u32 = 4;
pub const DL23_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL23_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_DL23_RCH_MON */
pub const DL23_RCH_DATA_SFT: u32 = 0;
pub const DL23_RCH_DATA_MASK: u32 = 0xffffffff;
pub const DL23_RCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL23_LCH_MON */
pub const DL23_LCH_DATA_SFT: u32 = 0;
pub const DL23_LCH_DATA_MASK: u32 = 0xffffffff;
pub const DL23_LCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL23_CON0 */
pub const DL23_ON_SFT: u32 = 28;
pub const DL23_ON_MASK: u32 = 0x1;
pub const DL23_ON_MASK_SFT: u32 = 0x1 << 28;
pub const DL23_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL23_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL23_ONE_HEART_SEL_MASK_SFT: u32 = 0x3 << 22;
pub const DL23_MINLEN_SFT: u32 = 20;
pub const DL23_MINLEN_MASK: u32 = 0x3;
pub const DL23_MINLEN_MASK_SFT: u32 = 0x3 << 20;
pub const DL23_MAXLEN_SFT: u32 = 16;
pub const DL23_MAXLEN_MASK: u32 = 0x3;
pub const DL23_MAXLEN_MASK_SFT: u32 = 0x3 << 16;
pub const DL23_SEL_DOMAIN_SFT: u32 = 13;
pub const DL23_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL23_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const DL23_SEL_FS_SFT: u32 = 8;
pub const DL23_SEL_FS_MASK: u32 = 0x1f;
pub const DL23_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const DL23_SW_CLEAR_BUF_EMPTY_SFT: u32 = 7;
pub const DL23_SW_CLEAR_BUF_EMPTY_MASK: u32 = 0x1;
pub const DL23_SW_CLEAR_BUF_EMPTY_MASK_SFT: u32 = 0x1 << 7;
pub const DL23_PBUF_SIZE_SFT: u32 = 5;
pub const DL23_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL23_PBUF_SIZE_MASK_SFT: u32 = 0x3 << 5;
pub const DL23_MONO_SFT: u32 = 4;
pub const DL23_MONO_MASK: u32 = 0x1;
pub const DL23_MONO_MASK_SFT: u32 = 0x1 << 4;
pub const DL23_NORMAL_MODE_SFT: u32 = 3;
pub const DL23_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL23_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 3;
pub const DL23_HALIGN_SFT: u32 = 2;
pub const DL23_HALIGN_MASK: u32 = 0x1;
pub const DL23_HALIGN_MASK_SFT: u32 = 0x1 << 2;
pub const DL23_HD_MODE_SFT: u32 = 0;
pub const DL23_HD_MODE_MASK: u32 = 0x3;
pub const DL23_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_DL24_BASE_MSB */
pub const DL24_BASE__ADDR_MSB_SFT: u32 = 0;
pub const DL24_BASE__ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL24_BASE__ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL24_BASE */
pub const DL24_BASE_ADDR_SFT: u32 = 4;
pub const DL24_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL24_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_DL24_CUR_MSB */
pub const DL24_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL24_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL24_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL24_CUR */
pub const DL24_CUR_PTR_SFT: u32 = 0;
pub const DL24_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL24_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL24_END_MSB */
pub const DL24_END_ADDR_MSB_SFT: u32 = 0;
pub const DL24_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL24_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL24_END */
pub const DL24_END_ADDR_SFT: u32 = 4;
pub const DL24_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL24_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_DL24_RCH_MON */
pub const DL24_RCH_DATA_SFT: u32 = 0;
pub const DL24_RCH_DATA_MASK: u32 = 0xffffffff;
pub const DL24_RCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL24_LCH_MON */
pub const DL24_LCH_DATA_SFT: u32 = 0;
pub const DL24_LCH_DATA_MASK: u32 = 0xffffffff;
pub const DL24_LCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL24_CON0 */
pub const DL24_ON_SFT: u32 = 28;
pub const DL24_ON_MASK: u32 = 0x1;
pub const DL24_ON_MASK_SFT: u32 = 0x1 << 28;
pub const DL24_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL24_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL24_ONE_HEART_SEL_MASK_SFT: u32 = 0x3 << 22;
pub const DL24_MINLEN_SFT: u32 = 20;
pub const DL24_MINLEN_MASK: u32 = 0x3;
pub const DL24_MINLEN_MASK_SFT: u32 = 0x3 << 20;
pub const DL24_MAXLEN_SFT: u32 = 16;
pub const DL24_MAXLEN_MASK: u32 = 0x3;
pub const DL24_MAXLEN_MASK_SFT: u32 = 0x3 << 16;
pub const DL24_SEL_DOMAIN_SFT: u32 = 13;
pub const DL24_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL24_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const DL24_SEL_FS_SFT: u32 = 8;
pub const DL24_SEL_FS_MASK: u32 = 0x1f;
pub const DL24_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const DL24_SW_CLEAR_BUF_EMPTY_SFT: u32 = 7;
pub const DL24_SW_CLEAR_BUF_EMPTY_MASK: u32 = 0x1;
pub const DL24_SW_CLEAR_BUF_EMPTY_MASK_SFT: u32 = 0x1 << 7;
pub const DL24_PBUF_SIZE_SFT: u32 = 5;
pub const DL24_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL24_PBUF_SIZE_MASK_SFT: u32 = 0x3 << 5;
pub const DL24_MONO_SFT: u32 = 4;
pub const DL24_MONO_MASK: u32 = 0x1;
pub const DL24_MONO_MASK_SFT: u32 = 0x1 << 4;
pub const DL24_NORMAL_MODE_SFT: u32 = 3;
pub const DL24_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL24_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 3;
pub const DL24_HALIGN_SFT: u32 = 2;
pub const DL24_HALIGN_MASK: u32 = 0x1;
pub const DL24_HALIGN_MASK_SFT: u32 = 0x1 << 2;
pub const DL24_HD_MODE_SFT: u32 = 0;
pub const DL24_HD_MODE_MASK: u32 = 0x3;
pub const DL24_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_DL25_BASE_MSB */
pub const DL25_BASE__ADDR_MSB_SFT: u32 = 0;
pub const DL25_BASE__ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL25_BASE__ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL25_BASE */
pub const DL25_BASE_ADDR_SFT: u32 = 4;
pub const DL25_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const DL25_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_DL25_CUR_MSB */
pub const DL25_CUR_PTR_MSB_SFT: u32 = 0;
pub const DL25_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const DL25_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL25_CUR */
pub const DL25_CUR_PTR_SFT: u32 = 0;
pub const DL25_CUR_PTR_MASK: u32 = 0xffffffff;
pub const DL25_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL25_END_MSB */
pub const DL25_END_ADDR_MSB_SFT: u32 = 0;
pub const DL25_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const DL25_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_DL25_END */
pub const DL25_END_ADDR_SFT: u32 = 4;
pub const DL25_END_ADDR_MASK: u32 = 0xfffffff;
pub const DL25_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_DL25_RCH_MON */
pub const DL25_RCH_DATA_SFT: u32 = 0;
pub const DL25_RCH_DATA_MASK: u32 = 0xffffffff;
pub const DL25_RCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL25_LCH_MON */
pub const DL25_LCH_DATA_SFT: u32 = 0;
pub const DL25_LCH_DATA_MASK: u32 = 0xffffffff;
pub const DL25_LCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL25_CON0 */
pub const DL25_ON_SFT: u32 = 28;
pub const DL25_ON_MASK: u32 = 0x1;
pub const DL25_ON_MASK_SFT: u32 = 0x1 << 28;
pub const DL25_ONE_HEART_SEL_SFT: u32 = 22;
pub const DL25_ONE_HEART_SEL_MASK: u32 = 0x3;
pub const DL25_ONE_HEART_SEL_MASK_SFT: u32 = 0x3 << 22;
pub const DL25_MINLEN_SFT: u32 = 20;
pub const DL25_MINLEN_MASK: u32 = 0x3;
pub const DL25_MINLEN_MASK_SFT: u32 = 0x3 << 20;
pub const DL25_MAXLEN_SFT: u32 = 16;
pub const DL25_MAXLEN_MASK: u32 = 0x3;
pub const DL25_MAXLEN_MASK_SFT: u32 = 0x3 << 16;
pub const DL25_SEL_DOMAIN_SFT: u32 = 13;
pub const DL25_SEL_DOMAIN_MASK: u32 = 0x7;
pub const DL25_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const DL25_SEL_FS_SFT: u32 = 8;
pub const DL25_SEL_FS_MASK: u32 = 0x1f;
pub const DL25_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const DL25_SW_CLEAR_BUF_EMPTY_SFT: u32 = 7;
pub const DL25_SW_CLEAR_BUF_EMPTY_MASK: u32 = 0x1;
pub const DL25_SW_CLEAR_BUF_EMPTY_MASK_SFT: u32 = 0x1 << 7;
pub const DL25_PBUF_SIZE_SFT: u32 = 5;
pub const DL25_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL25_PBUF_SIZE_MASK_SFT: u32 = 0x3 << 5;
pub const DL25_MONO_SFT: u32 = 4;
pub const DL25_MONO_MASK: u32 = 0x1;
pub const DL25_MONO_MASK_SFT: u32 = 0x1 << 4;
pub const DL25_NORMAL_MODE_SFT: u32 = 3;
pub const DL25_NORMAL_MODE_MASK: u32 = 0x1;
pub const DL25_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 3;
pub const DL25_HALIGN_SFT: u32 = 2;
pub const DL25_HALIGN_MASK: u32 = 0x1;
pub const DL25_HALIGN_MASK_SFT: u32 = 0x1 << 2;
pub const DL25_HD_MODE_SFT: u32 = 0;
pub const DL25_HD_MODE_MASK: u32 = 0x3;
pub const DL25_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_VUL0_BASE_MSB */
pub const VUL0_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL0_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL0_BASE_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL0_BASE */
pub const VUL0_BASE_ADDR_SFT: u32 = 4;
pub const VUL0_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL0_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL0_CUR_MSB */
pub const VUL0_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL0_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL0_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL0_CUR */
pub const VUL0_CUR_PTR_SFT: u32 = 0;
pub const VUL0_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL0_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL0_END_MSB */
pub const VUL0_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL0_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL0_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL0_END */
pub const VUL0_END_ADDR_SFT: u32 = 4;
pub const VUL0_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL0_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL0_RCH_MON */
pub const VUL0_RCH_DATA_SFT: u32 = 0;
pub const VUL0_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL0_RCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL0_LCH_MON */
pub const VUL0_LCH_DATA_SFT: u32 = 0;
pub const VUL0_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL0_LCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL0_CON0 */
pub const VUL0_ON_SFT: u32 = 28;
pub const VUL0_ON_MASK: u32 = 0x1;
pub const VUL0_ON_MASK_SFT: u32 = 0x1 << 28;
pub const VUL0_MINLEN_SFT: u32 = 20;
pub const VUL0_MINLEN_MASK: u32 = 0x3;
pub const VUL0_MINLEN_MASK_SFT: u32 = 0x3 << 20;
pub const VUL0_MAXLEN_SFT: u32 = 16;
pub const VUL0_MAXLEN_MASK: u32 = 0x3;
pub const VUL0_MAXLEN_MASK_SFT: u32 = 0x3 << 16;
pub const VUL0_SEL_DOMAIN_SFT: u32 = 13;
pub const VUL0_SEL_DOMAIN_MASK: u32 = 0x7;
pub const VUL0_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const VUL0_SEL_FS_SFT: u32 = 8;
pub const VUL0_SEL_FS_MASK: u32 = 0x1f;
pub const VUL0_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const VUL0_SW_CLEAR_BUF_FULL_SFT: u32 = 7;
pub const VUL0_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL0_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = 0x1 << 7;
pub const VUL0_WR_SIGN_SFT: u32 = 6;
pub const VUL0_WR_SIGN_MASK: u32 = 0x1;
pub const VUL0_WR_SIGN_MASK_SFT: u32 = 0x1 << 6;
pub const VUL0_R_MONO_SFT: u32 = 5;
pub const VUL0_R_MONO_MASK: u32 = 0x1;
pub const VUL0_R_MONO_MASK_SFT: u32 = 0x1 << 5;
pub const VUL0_MONO_SFT: u32 = 4;
pub const VUL0_MONO_MASK: u32 = 0x1;
pub const VUL0_MONO_MASK_SFT: u32 = 0x1 << 4;
pub const VUL0_NORMAL_MODE_SFT: u32 = 3;
pub const VUL0_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL0_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 3;
pub const VUL0_HALIGN_SFT: u32 = 2;
pub const VUL0_HALIGN_MASK: u32 = 0x1;
pub const VUL0_HALIGN_MASK_SFT: u32 = 0x1 << 2;
pub const VUL0_HD_MODE_SFT: u32 = 0;
pub const VUL0_HD_MODE_MASK: u32 = 0x3;
pub const VUL0_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_VUL1_BASE_MSB */
pub const VUL1_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL1_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL1_BASE_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL1_BASE */
pub const VUL1_BASE_ADDR_SFT: u32 = 4;
pub const VUL1_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL1_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL1_CUR_MSB */
pub const VUL1_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL1_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL1_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL1_CUR */
pub const VUL1_CUR_PTR_SFT: u32 = 0;
pub const VUL1_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL1_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL1_END_MSB */
pub const VUL1_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL1_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL1_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL1_END */
pub const VUL1_END_ADDR_SFT: u32 = 4;
pub const VUL1_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL1_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL1_RCH_MON */
pub const VUL1_RCH_DATA_SFT: u32 = 0;
pub const VUL1_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL1_RCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL1_LCH_MON */
pub const VUL1_LCH_DATA_SFT: u32 = 0;
pub const VUL1_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL1_LCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL1_CON0 */
pub const VUL1_ON_SFT: u32 = 28;
pub const VUL1_ON_MASK: u32 = 0x1;
pub const VUL1_ON_MASK_SFT: u32 = 0x1 << 28;
pub const VUL1_MINLEN_SFT: u32 = 20;
pub const VUL1_MINLEN_MASK: u32 = 0x3;
pub const VUL1_MINLEN_MASK_SFT: u32 = 0x3 << 20;
pub const VUL1_MAXLEN_SFT: u32 = 16;
pub const VUL1_MAXLEN_MASK: u32 = 0x3;
pub const VUL1_MAXLEN_MASK_SFT: u32 = 0x3 << 16;
pub const VUL1_SEL_DOMAIN_SFT: u32 = 13;
pub const VUL1_SEL_DOMAIN_MASK: u32 = 0x7;
pub const VUL1_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const VUL1_SEL_FS_SFT: u32 = 8;
pub const VUL1_SEL_FS_MASK: u32 = 0x1f;
pub const VUL1_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const VUL1_SW_CLEAR_BUF_FULL_SFT: u32 = 7;
pub const VUL1_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL1_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = 0x1 << 7;
pub const VUL1_WR_SIGN_SFT: u32 = 6;
pub const VUL1_WR_SIGN_MASK: u32 = 0x1;
pub const VUL1_WR_SIGN_MASK_SFT: u32 = 0x1 << 6;
pub const VUL1_R_MONO_SFT: u32 = 5;
pub const VUL1_R_MONO_MASK: u32 = 0x1;
pub const VUL1_R_MONO_MASK_SFT: u32 = 0x1 << 5;
pub const VUL1_MONO_SFT: u32 = 4;
pub const VUL1_MONO_MASK: u32 = 0x1;
pub const VUL1_MONO_MASK_SFT: u32 = 0x1 << 4;
pub const VUL1_NORMAL_MODE_SFT: u32 = 3;
pub const VUL1_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL1_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 3;
pub const VUL1_HALIGN_SFT: u32 = 2;
pub const VUL1_HALIGN_MASK: u32 = 0x1;
pub const VUL1_HALIGN_MASK_SFT: u32 = 0x1 << 2;
pub const VUL1_HD_MODE_SFT: u32 = 0;
pub const VUL1_HD_MODE_MASK: u32 = 0x3;
pub const VUL1_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_VUL1_MON0 */
pub const MEM_HW_WEN_SFT: u32 = 20;
pub const MEM_HW_WEN_MASK: u32 = 0xf;
pub const MEM_HW_WEN_MASK_SFT: u32 = 0xf << 20;
/* duplicate C macro: MEM_REQ_PENDING_SFT = 19 */
/* duplicate C macro: MEM_REQ_PENDING_MASK = 0x1 */
/* duplicate C macro: MEM_REQ_PENDING_MASK_SFT = 0x1 << 19 */
pub const BUF_FULL_SFT: u32 = 18;
pub const BUF_FULL_MASK: u32 = 0x1;
pub const BUF_FULL_MASK_SFT: u32 = 0x1 << 18;
/* duplicate C macro: ENABLE_SYNC_MEM_SFT = 17 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK_SFT = 0x1 << 17 */
/* duplicate C macro: ENABLE_SYNC_AGENT_SFT = 16 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK_SFT = 0x1 << 16 */
/* duplicate C macro: RESERVED_02_SFT = 6 */
/* duplicate C macro: RESERVED_02_MASK = 0x3ff */
/* duplicate C macro: RESERVED_02_MASK_SFT = 0x3ff << 6 */
/* duplicate C macro: MEM_ADDR_DIFF_SFT = 0 */
/* duplicate C macro: MEM_ADDR_DIFF_MASK = 0x3f */
/* duplicate C macro: MEM_ADDR_DIFF_MASK_SFT = 0x3f << 0 */

/* AFE_VUL2_BASE_MSB */
pub const VUL2_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL2_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL2_BASE_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL2_BASE */
pub const VUL2_BASE_ADDR_SFT: u32 = 4;
pub const VUL2_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL2_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL2_CUR_MSB */
pub const VUL2_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL2_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL2_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL2_CUR */
pub const VUL2_CUR_PTR_SFT: u32 = 0;
pub const VUL2_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL2_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL2_END_MSB */
pub const VUL2_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL2_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL2_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL2_END */
pub const VUL2_END_ADDR_SFT: u32 = 4;
pub const VUL2_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL2_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL2_RCH_MON */
pub const VUL2_RCH_DATA_SFT: u32 = 0;
pub const VUL2_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL2_RCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL2_LCH_MON */
pub const VUL2_LCH_DATA_SFT: u32 = 0;
pub const VUL2_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL2_LCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL2_CON0 */
pub const VUL2_ON_SFT: u32 = 28;
pub const VUL2_ON_MASK: u32 = 0x1;
pub const VUL2_ON_MASK_SFT: u32 = 0x1 << 28;
pub const VUL2_MINLEN_SFT: u32 = 20;
pub const VUL2_MINLEN_MASK: u32 = 0x3;
pub const VUL2_MINLEN_MASK_SFT: u32 = 0x3 << 20;
pub const VUL2_MAXLEN_SFT: u32 = 16;
pub const VUL2_MAXLEN_MASK: u32 = 0x3;
pub const VUL2_MAXLEN_MASK_SFT: u32 = 0x3 << 16;
pub const VUL2_SEL_DOMAIN_SFT: u32 = 13;
pub const VUL2_SEL_DOMAIN_MASK: u32 = 0x7;
pub const VUL2_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const VUL2_SEL_FS_SFT: u32 = 8;
pub const VUL2_SEL_FS_MASK: u32 = 0x1f;
pub const VUL2_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const VUL2_SW_CLEAR_BUF_FULL_SFT: u32 = 7;
pub const VUL2_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL2_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = 0x1 << 7;
pub const VUL2_WR_SIGN_SFT: u32 = 6;
pub const VUL2_WR_SIGN_MASK: u32 = 0x1;
pub const VUL2_WR_SIGN_MASK_SFT: u32 = 0x1 << 6;
pub const VUL2_R_MONO_SFT: u32 = 5;
pub const VUL2_R_MONO_MASK: u32 = 0x1;
pub const VUL2_R_MONO_MASK_SFT: u32 = 0x1 << 5;
pub const VUL2_MONO_SFT: u32 = 4;
pub const VUL2_MONO_MASK: u32 = 0x1;
pub const VUL2_MONO_MASK_SFT: u32 = 0x1 << 4;
pub const VUL2_NORMAL_MODE_SFT: u32 = 3;
pub const VUL2_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL2_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 3;
pub const VUL2_HALIGN_SFT: u32 = 2;
pub const VUL2_HALIGN_MASK: u32 = 0x1;
pub const VUL2_HALIGN_MASK_SFT: u32 = 0x1 << 2;
pub const VUL2_HD_MODE_SFT: u32 = 0;
pub const VUL2_HD_MODE_MASK: u32 = 0x3;
pub const VUL2_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_VUL2_MON0 */
/* duplicate C macro: MEM_HW_WEN_SFT = 20 */
/* duplicate C macro: MEM_HW_WEN_MASK = 0xf */
/* duplicate C macro: MEM_HW_WEN_MASK_SFT = 0xf << 20 */
/* duplicate C macro: MEM_REQ_PENDING_SFT = 19 */
/* duplicate C macro: MEM_REQ_PENDING_MASK = 0x1 */
/* duplicate C macro: MEM_REQ_PENDING_MASK_SFT = 0x1 << 19 */
/* duplicate C macro: BUF_FULL_SFT = 18 */
/* duplicate C macro: BUF_FULL_MASK = 0x1 */
/* duplicate C macro: BUF_FULL_MASK_SFT = 0x1 << 18 */
/* duplicate C macro: ENABLE_SYNC_MEM_SFT = 17 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK_SFT = 0x1 << 17 */
/* duplicate C macro: ENABLE_SYNC_AGENT_SFT = 16 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK_SFT = 0x1 << 16 */
/* duplicate C macro: RESERVED_02_SFT = 6 */
/* duplicate C macro: RESERVED_02_MASK = 0x3ff */
/* duplicate C macro: RESERVED_02_MASK_SFT = 0x3ff << 6 */
/* duplicate C macro: MEM_ADDR_DIFF_SFT = 0 */
/* duplicate C macro: MEM_ADDR_DIFF_MASK = 0x3f */
/* duplicate C macro: MEM_ADDR_DIFF_MASK_SFT = 0x3f << 0 */

/* AFE_VUL3_BASE_MSB */
pub const VUL3_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL3_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL3_BASE_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL3_BASE */
pub const VUL3_BASE_ADDR_SFT: u32 = 4;
pub const VUL3_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL3_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL3_CUR_MSB */
pub const VUL3_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL3_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL3_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL3_CUR */
pub const VUL3_CUR_PTR_SFT: u32 = 0;
pub const VUL3_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL3_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL3_END_MSB */
pub const VUL3_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL3_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL3_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL3_END */
pub const VUL3_END_ADDR_SFT: u32 = 4;
pub const VUL3_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL3_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL3_RCH_MON */
pub const VUL3_RCH_DATA_SFT: u32 = 0;
pub const VUL3_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL3_RCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL3_LCH_MON */
pub const VUL3_LCH_DATA_SFT: u32 = 0;
pub const VUL3_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL3_LCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL3_CON0 */
pub const VUL3_ON_SFT: u32 = 28;
pub const VUL3_ON_MASK: u32 = 0x1;
pub const VUL3_ON_MASK_SFT: u32 = 0x1 << 28;
pub const VUL3_MINLEN_SFT: u32 = 20;
pub const VUL3_MINLEN_MASK: u32 = 0x3;
pub const VUL3_MINLEN_MASK_SFT: u32 = 0x3 << 20;
pub const VUL3_MAXLEN_SFT: u32 = 16;
pub const VUL3_MAXLEN_MASK: u32 = 0x3;
pub const VUL3_MAXLEN_MASK_SFT: u32 = 0x3 << 16;
pub const VUL3_SEL_DOMAIN_SFT: u32 = 13;
pub const VUL3_SEL_DOMAIN_MASK: u32 = 0x7;
pub const VUL3_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const VUL3_SEL_FS_SFT: u32 = 8;
pub const VUL3_SEL_FS_MASK: u32 = 0x1f;
pub const VUL3_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const VUL3_SW_CLEAR_BUF_FULL_SFT: u32 = 7;
pub const VUL3_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL3_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = 0x1 << 7;
pub const VUL3_WR_SIGN_SFT: u32 = 6;
pub const VUL3_WR_SIGN_MASK: u32 = 0x1;
pub const VUL3_WR_SIGN_MASK_SFT: u32 = 0x1 << 6;
pub const VUL3_R_MONO_SFT: u32 = 5;
pub const VUL3_R_MONO_MASK: u32 = 0x1;
pub const VUL3_R_MONO_MASK_SFT: u32 = 0x1 << 5;
pub const VUL3_MONO_SFT: u32 = 4;
pub const VUL3_MONO_MASK: u32 = 0x1;
pub const VUL3_MONO_MASK_SFT: u32 = 0x1 << 4;
pub const VUL3_NORMAL_MODE_SFT: u32 = 3;
pub const VUL3_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL3_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 3;
pub const VUL3_HALIGN_SFT: u32 = 2;
pub const VUL3_HALIGN_MASK: u32 = 0x1;
pub const VUL3_HALIGN_MASK_SFT: u32 = 0x1 << 2;
pub const VUL3_HD_MODE_SFT: u32 = 0;
pub const VUL3_HD_MODE_MASK: u32 = 0x3;
pub const VUL3_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_VUL3_MON0 */
/* duplicate C macro: MEM_HW_WEN_SFT = 20 */
/* duplicate C macro: MEM_HW_WEN_MASK = 0xf */
/* duplicate C macro: MEM_HW_WEN_MASK_SFT = 0xf << 20 */
/* duplicate C macro: MEM_REQ_PENDING_SFT = 19 */
/* duplicate C macro: MEM_REQ_PENDING_MASK = 0x1 */
/* duplicate C macro: MEM_REQ_PENDING_MASK_SFT = 0x1 << 19 */
/* duplicate C macro: BUF_FULL_SFT = 18 */
/* duplicate C macro: BUF_FULL_MASK = 0x1 */
/* duplicate C macro: BUF_FULL_MASK_SFT = 0x1 << 18 */
/* duplicate C macro: ENABLE_SYNC_MEM_SFT = 17 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK_SFT = 0x1 << 17 */
/* duplicate C macro: ENABLE_SYNC_AGENT_SFT = 16 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK_SFT = 0x1 << 16 */
/* duplicate C macro: RESERVED_02_SFT = 6 */
/* duplicate C macro: RESERVED_02_MASK = 0x3ff */
/* duplicate C macro: RESERVED_02_MASK_SFT = 0x3ff << 6 */
/* duplicate C macro: MEM_ADDR_DIFF_SFT = 0 */
/* duplicate C macro: MEM_ADDR_DIFF_MASK = 0x3f */
/* duplicate C macro: MEM_ADDR_DIFF_MASK_SFT = 0x3f << 0 */

/* AFE_VUL4_BASE_MSB */
pub const VUL4_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL4_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL4_BASE_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL4_BASE */
pub const VUL4_BASE_ADDR_SFT: u32 = 4;
pub const VUL4_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL4_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL4_CUR_MSB */
pub const VUL4_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL4_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL4_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL4_CUR */
pub const VUL4_CUR_PTR_SFT: u32 = 0;
pub const VUL4_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL4_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL4_END_MSB */
pub const VUL4_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL4_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL4_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL4_END */
pub const VUL4_END_ADDR_SFT: u32 = 4;
pub const VUL4_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL4_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL4_RCH_MON */
pub const VUL4_RCH_DATA_SFT: u32 = 0;
pub const VUL4_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL4_RCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL4_LCH_MON */
pub const VUL4_LCH_DATA_SFT: u32 = 0;
pub const VUL4_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL4_LCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL4_CON0 */
pub const VUL4_ON_SFT: u32 = 28;
pub const VUL4_ON_MASK: u32 = 0x1;
pub const VUL4_ON_MASK_SFT: u32 = 0x1 << 28;
pub const VUL4_MINLEN_SFT: u32 = 20;
pub const VUL4_MINLEN_MASK: u32 = 0x3;
pub const VUL4_MINLEN_MASK_SFT: u32 = 0x3 << 20;
pub const VUL4_MAXLEN_SFT: u32 = 16;
pub const VUL4_MAXLEN_MASK: u32 = 0x3;
pub const VUL4_MAXLEN_MASK_SFT: u32 = 0x3 << 16;
pub const VUL4_SEL_DOMAIN_SFT: u32 = 13;
pub const VUL4_SEL_DOMAIN_MASK: u32 = 0x7;
pub const VUL4_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const VUL4_SEL_FS_SFT: u32 = 8;
pub const VUL4_SEL_FS_MASK: u32 = 0x1f;
pub const VUL4_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const VUL4_SW_CLEAR_BUF_FULL_SFT: u32 = 7;
pub const VUL4_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL4_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = 0x1 << 7;
pub const VUL4_WR_SIGN_SFT: u32 = 6;
pub const VUL4_WR_SIGN_MASK: u32 = 0x1;
pub const VUL4_WR_SIGN_MASK_SFT: u32 = 0x1 << 6;
pub const VUL4_R_MONO_SFT: u32 = 5;
pub const VUL4_R_MONO_MASK: u32 = 0x1;
pub const VUL4_R_MONO_MASK_SFT: u32 = 0x1 << 5;
pub const VUL4_MONO_SFT: u32 = 4;
pub const VUL4_MONO_MASK: u32 = 0x1;
pub const VUL4_MONO_MASK_SFT: u32 = 0x1 << 4;
pub const VUL4_NORMAL_MODE_SFT: u32 = 3;
pub const VUL4_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL4_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 3;
pub const VUL4_HALIGN_SFT: u32 = 2;
pub const VUL4_HALIGN_MASK: u32 = 0x1;
pub const VUL4_HALIGN_MASK_SFT: u32 = 0x1 << 2;
pub const VUL4_HD_MODE_SFT: u32 = 0;
pub const VUL4_HD_MODE_MASK: u32 = 0x3;
pub const VUL4_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_VUL4_MON0 */
/* duplicate C macro: MEM_HW_WEN_SFT = 20 */
/* duplicate C macro: MEM_HW_WEN_MASK = 0xf */
/* duplicate C macro: MEM_HW_WEN_MASK_SFT = 0xf << 20 */
/* duplicate C macro: MEM_REQ_PENDING_SFT = 19 */
/* duplicate C macro: MEM_REQ_PENDING_MASK = 0x1 */
/* duplicate C macro: MEM_REQ_PENDING_MASK_SFT = 0x1 << 19 */
/* duplicate C macro: BUF_FULL_SFT = 18 */
/* duplicate C macro: BUF_FULL_MASK = 0x1 */
/* duplicate C macro: BUF_FULL_MASK_SFT = 0x1 << 18 */
/* duplicate C macro: ENABLE_SYNC_MEM_SFT = 17 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK_SFT = 0x1 << 17 */
/* duplicate C macro: ENABLE_SYNC_AGENT_SFT = 16 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK_SFT = 0x1 << 16 */
/* duplicate C macro: RESERVED_02_SFT = 6 */
/* duplicate C macro: RESERVED_02_MASK = 0x3ff */
/* duplicate C macro: RESERVED_02_MASK_SFT = 0x3ff << 6 */
/* duplicate C macro: MEM_ADDR_DIFF_SFT = 0 */
/* duplicate C macro: MEM_ADDR_DIFF_MASK = 0x3f */
/* duplicate C macro: MEM_ADDR_DIFF_MASK_SFT = 0x3f << 0 */

/* AFE_VUL5_BASE_MSB */
pub const VUL5_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL5_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL5_BASE_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL5_BASE */
pub const VUL5_BASE_ADDR_SFT: u32 = 4;
pub const VUL5_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL5_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL5_CUR_MSB */
pub const VUL5_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL5_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL5_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL5_CUR */
pub const VUL5_CUR_PTR_SFT: u32 = 0;
pub const VUL5_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL5_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL5_END_MSB */
pub const VUL5_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL5_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL5_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL5_END */
pub const VUL5_END_ADDR_SFT: u32 = 4;
pub const VUL5_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL5_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL5_RCH_MON */
pub const VUL5_RCH_DATA_SFT: u32 = 0;
pub const VUL5_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL5_RCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL5_LCH_MON */
pub const VUL5_LCH_DATA_SFT: u32 = 0;
pub const VUL5_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL5_LCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL5_CON0 */
pub const VUL5_ON_SFT: u32 = 28;
pub const VUL5_ON_MASK: u32 = 0x1;
pub const VUL5_ON_MASK_SFT: u32 = 0x1 << 28;
pub const VUL5_MINLEN_SFT: u32 = 20;
pub const VUL5_MINLEN_MASK: u32 = 0x3;
pub const VUL5_MINLEN_MASK_SFT: u32 = 0x3 << 20;
pub const VUL5_MAXLEN_SFT: u32 = 16;
pub const VUL5_MAXLEN_MASK: u32 = 0x3;
pub const VUL5_MAXLEN_MASK_SFT: u32 = 0x3 << 16;
pub const VUL5_SEL_DOMAIN_SFT: u32 = 13;
pub const VUL5_SEL_DOMAIN_MASK: u32 = 0x7;
pub const VUL5_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const VUL5_SEL_FS_SFT: u32 = 8;
pub const VUL5_SEL_FS_MASK: u32 = 0x1f;
pub const VUL5_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const VUL5_SW_CLEAR_BUF_FULL_SFT: u32 = 7;
pub const VUL5_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL5_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = 0x1 << 7;
pub const VUL5_WR_SIGN_SFT: u32 = 6;
pub const VUL5_WR_SIGN_MASK: u32 = 0x1;
pub const VUL5_WR_SIGN_MASK_SFT: u32 = 0x1 << 6;
pub const VUL5_R_MONO_SFT: u32 = 5;
pub const VUL5_R_MONO_MASK: u32 = 0x1;
pub const VUL5_R_MONO_MASK_SFT: u32 = 0x1 << 5;
pub const VUL5_MONO_SFT: u32 = 4;
pub const VUL5_MONO_MASK: u32 = 0x1;
pub const VUL5_MONO_MASK_SFT: u32 = 0x1 << 4;
pub const VUL5_NORMAL_MODE_SFT: u32 = 3;
pub const VUL5_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL5_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 3;
pub const VUL5_HALIGN_SFT: u32 = 2;
pub const VUL5_HALIGN_MASK: u32 = 0x1;
pub const VUL5_HALIGN_MASK_SFT: u32 = 0x1 << 2;
pub const VUL5_HD_MODE_SFT: u32 = 0;
pub const VUL5_HD_MODE_MASK: u32 = 0x3;
pub const VUL5_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_VUL5_MON0 */
/* duplicate C macro: MEM_HW_WEN_SFT = 20 */
/* duplicate C macro: MEM_HW_WEN_MASK = 0xf */
/* duplicate C macro: MEM_HW_WEN_MASK_SFT = 0xf << 20 */
/* duplicate C macro: MEM_REQ_PENDING_SFT = 19 */
/* duplicate C macro: MEM_REQ_PENDING_MASK = 0x1 */
/* duplicate C macro: MEM_REQ_PENDING_MASK_SFT = 0x1 << 19 */
/* duplicate C macro: BUF_FULL_SFT = 18 */
/* duplicate C macro: BUF_FULL_MASK = 0x1 */
/* duplicate C macro: BUF_FULL_MASK_SFT = 0x1 << 18 */
/* duplicate C macro: ENABLE_SYNC_MEM_SFT = 17 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK_SFT = 0x1 << 17 */
/* duplicate C macro: ENABLE_SYNC_AGENT_SFT = 16 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK_SFT = 0x1 << 16 */
/* duplicate C macro: RESERVED_02_SFT = 6 */
/* duplicate C macro: RESERVED_02_MASK = 0x3ff */
/* duplicate C macro: RESERVED_02_MASK_SFT = 0x3ff << 6 */
/* duplicate C macro: MEM_ADDR_DIFF_SFT = 0 */
/* duplicate C macro: MEM_ADDR_DIFF_MASK = 0x3f */
/* duplicate C macro: MEM_ADDR_DIFF_MASK_SFT = 0x3f << 0 */

/* AFE_VUL6_BASE_MSB */
pub const VUL6_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL6_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL6_BASE_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL6_BASE */
pub const VUL6_BASE_ADDR_SFT: u32 = 4;
pub const VUL6_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL6_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL6_CUR_MSB */
pub const VUL6_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL6_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL6_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL6_CUR */
pub const VUL6_CUR_PTR_SFT: u32 = 0;
pub const VUL6_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL6_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL6_END_MSB */
pub const VUL6_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL6_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL6_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL6_END */
pub const VUL6_END_ADDR_SFT: u32 = 4;
pub const VUL6_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL6_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL6_RCH_MON */
pub const VUL6_RCH_DATA_SFT: u32 = 0;
pub const VUL6_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL6_RCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL6_LCH_MON */
pub const VUL6_LCH_DATA_SFT: u32 = 0;
pub const VUL6_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL6_LCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL6_CON0 */
pub const VUL6_ON_SFT: u32 = 28;
pub const VUL6_ON_MASK: u32 = 0x1;
pub const VUL6_ON_MASK_SFT: u32 = 0x1 << 28;
pub const VUL6_MINLEN_SFT: u32 = 20;
pub const VUL6_MINLEN_MASK: u32 = 0x3;
pub const VUL6_MINLEN_MASK_SFT: u32 = 0x3 << 20;
pub const VUL6_MAXLEN_SFT: u32 = 16;
pub const VUL6_MAXLEN_MASK: u32 = 0x3;
pub const VUL6_MAXLEN_MASK_SFT: u32 = 0x3 << 16;
pub const VUL6_SEL_DOMAIN_SFT: u32 = 13;
pub const VUL6_SEL_DOMAIN_MASK: u32 = 0x7;
pub const VUL6_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const VUL6_SEL_FS_SFT: u32 = 8;
pub const VUL6_SEL_FS_MASK: u32 = 0x1f;
pub const VUL6_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const VUL6_SW_CLEAR_BUF_FULL_SFT: u32 = 7;
pub const VUL6_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL6_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = 0x1 << 7;
pub const VUL6_WR_SIGN_SFT: u32 = 6;
pub const VUL6_WR_SIGN_MASK: u32 = 0x1;
pub const VUL6_WR_SIGN_MASK_SFT: u32 = 0x1 << 6;
pub const VUL6_R_MONO_SFT: u32 = 5;
pub const VUL6_R_MONO_MASK: u32 = 0x1;
pub const VUL6_R_MONO_MASK_SFT: u32 = 0x1 << 5;
pub const VUL6_MONO_SFT: u32 = 4;
pub const VUL6_MONO_MASK: u32 = 0x1;
pub const VUL6_MONO_MASK_SFT: u32 = 0x1 << 4;
pub const VUL6_NORMAL_MODE_SFT: u32 = 3;
pub const VUL6_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL6_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 3;
pub const VUL6_HALIGN_SFT: u32 = 2;
pub const VUL6_HALIGN_MASK: u32 = 0x1;
pub const VUL6_HALIGN_MASK_SFT: u32 = 0x1 << 2;
pub const VUL6_HD_MODE_SFT: u32 = 0;
pub const VUL6_HD_MODE_MASK: u32 = 0x3;
pub const VUL6_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_VUL6_MON0 */
/* duplicate C macro: MEM_HW_WEN_SFT = 20 */
/* duplicate C macro: MEM_HW_WEN_MASK = 0xf */
/* duplicate C macro: MEM_HW_WEN_MASK_SFT = 0xf << 20 */
/* duplicate C macro: MEM_REQ_PENDING_SFT = 19 */
/* duplicate C macro: MEM_REQ_PENDING_MASK = 0x1 */
/* duplicate C macro: MEM_REQ_PENDING_MASK_SFT = 0x1 << 19 */
/* duplicate C macro: BUF_FULL_SFT = 18 */
/* duplicate C macro: BUF_FULL_MASK = 0x1 */
/* duplicate C macro: BUF_FULL_MASK_SFT = 0x1 << 18 */
/* duplicate C macro: ENABLE_SYNC_MEM_SFT = 17 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK_SFT = 0x1 << 17 */
/* duplicate C macro: ENABLE_SYNC_AGENT_SFT = 16 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK_SFT = 0x1 << 16 */
/* duplicate C macro: RESERVED_02_SFT = 6 */
/* duplicate C macro: RESERVED_02_MASK = 0x3ff */
/* duplicate C macro: RESERVED_02_MASK_SFT = 0x3ff << 6 */
/* duplicate C macro: MEM_ADDR_DIFF_SFT = 0 */
/* duplicate C macro: MEM_ADDR_DIFF_MASK = 0x3f */
/* duplicate C macro: MEM_ADDR_DIFF_MASK_SFT = 0x3f << 0 */

/* AFE_VUL7_BASE_MSB */
pub const VUL7_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL7_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL7_BASE_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL7_BASE */
pub const VUL7_BASE_ADDR_SFT: u32 = 4;
pub const VUL7_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL7_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL7_CUR_MSB */
pub const VUL7_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL7_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL7_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL7_CUR */
pub const VUL7_CUR_PTR_SFT: u32 = 0;
pub const VUL7_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL7_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL7_END_MSB */
pub const VUL7_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL7_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL7_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL7_END */
pub const VUL7_END_ADDR_SFT: u32 = 4;
pub const VUL7_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL7_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL7_RCH_MON */
pub const VUL7_RCH_DATA_SFT: u32 = 0;
pub const VUL7_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL7_RCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL7_LCH_MON */
pub const VUL7_LCH_DATA_SFT: u32 = 0;
pub const VUL7_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL7_LCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL7_CON0 */
pub const VUL7_ON_SFT: u32 = 28;
pub const VUL7_ON_MASK: u32 = 0x1;
pub const VUL7_ON_MASK_SFT: u32 = 0x1 << 28;
pub const VUL7_MINLEN_SFT: u32 = 20;
pub const VUL7_MINLEN_MASK: u32 = 0x3;
pub const VUL7_MINLEN_MASK_SFT: u32 = 0x3 << 20;
pub const VUL7_MAXLEN_SFT: u32 = 16;
pub const VUL7_MAXLEN_MASK: u32 = 0x3;
pub const VUL7_MAXLEN_MASK_SFT: u32 = 0x3 << 16;
pub const VUL7_SEL_DOMAIN_SFT: u32 = 13;
pub const VUL7_SEL_DOMAIN_MASK: u32 = 0x7;
pub const VUL7_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const VUL7_SEL_FS_SFT: u32 = 8;
pub const VUL7_SEL_FS_MASK: u32 = 0x1f;
pub const VUL7_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const VUL7_SW_CLEAR_BUF_FULL_SFT: u32 = 7;
pub const VUL7_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL7_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = 0x1 << 7;
pub const VUL7_WR_SIGN_SFT: u32 = 6;
pub const VUL7_WR_SIGN_MASK: u32 = 0x1;
pub const VUL7_WR_SIGN_MASK_SFT: u32 = 0x1 << 6;
pub const VUL7_R_MONO_SFT: u32 = 5;
pub const VUL7_R_MONO_MASK: u32 = 0x1;
pub const VUL7_R_MONO_MASK_SFT: u32 = 0x1 << 5;
pub const VUL7_MONO_SFT: u32 = 4;
pub const VUL7_MONO_MASK: u32 = 0x1;
pub const VUL7_MONO_MASK_SFT: u32 = 0x1 << 4;
pub const VUL7_NORMAL_MODE_SFT: u32 = 3;
pub const VUL7_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL7_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 3;
pub const VUL7_HALIGN_SFT: u32 = 2;
pub const VUL7_HALIGN_MASK: u32 = 0x1;
pub const VUL7_HALIGN_MASK_SFT: u32 = 0x1 << 2;
pub const VUL7_HD_MODE_SFT: u32 = 0;
pub const VUL7_HD_MODE_MASK: u32 = 0x3;
pub const VUL7_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_VUL7_MON0 */
/* duplicate C macro: MEM_HW_WEN_SFT = 20 */
/* duplicate C macro: MEM_HW_WEN_MASK = 0xf */
/* duplicate C macro: MEM_HW_WEN_MASK_SFT = 0xf << 20 */
/* duplicate C macro: MEM_REQ_PENDING_SFT = 19 */
/* duplicate C macro: MEM_REQ_PENDING_MASK = 0x1 */
/* duplicate C macro: MEM_REQ_PENDING_MASK_SFT = 0x1 << 19 */
/* duplicate C macro: BUF_FULL_SFT = 18 */
/* duplicate C macro: BUF_FULL_MASK = 0x1 */
/* duplicate C macro: BUF_FULL_MASK_SFT = 0x1 << 18 */
/* duplicate C macro: ENABLE_SYNC_MEM_SFT = 17 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK_SFT = 0x1 << 17 */
/* duplicate C macro: ENABLE_SYNC_AGENT_SFT = 16 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK_SFT = 0x1 << 16 */
/* duplicate C macro: RESERVED_02_SFT = 6 */
/* duplicate C macro: RESERVED_02_MASK = 0x3ff */
/* duplicate C macro: RESERVED_02_MASK_SFT = 0x3ff << 6 */
/* duplicate C macro: MEM_ADDR_DIFF_SFT = 0 */
/* duplicate C macro: MEM_ADDR_DIFF_MASK = 0x3f */
/* duplicate C macro: MEM_ADDR_DIFF_MASK_SFT = 0x3f << 0 */

/* AFE_VUL8_BASE_MSB */
pub const VUL8_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL8_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL8_BASE_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL8_BASE */
pub const VUL8_BASE_ADDR_SFT: u32 = 4;
pub const VUL8_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL8_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL8_CUR_MSB */
pub const VUL8_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL8_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL8_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL8_CUR */
pub const VUL8_CUR_PTR_SFT: u32 = 0;
pub const VUL8_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL8_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL8_END_MSB */
pub const VUL8_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL8_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL8_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL8_END */
pub const VUL8_END_ADDR_SFT: u32 = 4;
pub const VUL8_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL8_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL8_RCH_MON */
pub const VUL8_RCH_DATA_SFT: u32 = 0;
pub const VUL8_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL8_RCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL8_LCH_MON */
pub const VUL8_LCH_DATA_SFT: u32 = 0;
pub const VUL8_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL8_LCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL8_CON0 */
pub const VUL8_ON_SFT: u32 = 28;
pub const VUL8_ON_MASK: u32 = 0x1;
pub const VUL8_ON_MASK_SFT: u32 = 0x1 << 28;
pub const VUL8_MINLEN_SFT: u32 = 20;
pub const VUL8_MINLEN_MASK: u32 = 0x3;
pub const VUL8_MINLEN_MASK_SFT: u32 = 0x3 << 20;
pub const VUL8_MAXLEN_SFT: u32 = 16;
pub const VUL8_MAXLEN_MASK: u32 = 0x3;
pub const VUL8_MAXLEN_MASK_SFT: u32 = 0x3 << 16;
pub const VUL8_SEL_DOMAIN_SFT: u32 = 13;
pub const VUL8_SEL_DOMAIN_MASK: u32 = 0x7;
pub const VUL8_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const VUL8_SEL_FS_SFT: u32 = 8;
pub const VUL8_SEL_FS_MASK: u32 = 0x1f;
pub const VUL8_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const VUL8_SW_CLEAR_BUF_FULL_SFT: u32 = 7;
pub const VUL8_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL8_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = 0x1 << 7;
pub const VUL8_WR_SIGN_SFT: u32 = 6;
pub const VUL8_WR_SIGN_MASK: u32 = 0x1;
pub const VUL8_WR_SIGN_MASK_SFT: u32 = 0x1 << 6;
pub const VUL8_R_MONO_SFT: u32 = 5;
pub const VUL8_R_MONO_MASK: u32 = 0x1;
pub const VUL8_R_MONO_MASK_SFT: u32 = 0x1 << 5;
pub const VUL8_MONO_SFT: u32 = 4;
pub const VUL8_MONO_MASK: u32 = 0x1;
pub const VUL8_MONO_MASK_SFT: u32 = 0x1 << 4;
pub const VUL8_NORMAL_MODE_SFT: u32 = 3;
pub const VUL8_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL8_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 3;
pub const VUL8_HALIGN_SFT: u32 = 2;
pub const VUL8_HALIGN_MASK: u32 = 0x1;
pub const VUL8_HALIGN_MASK_SFT: u32 = 0x1 << 2;
pub const VUL8_HD_MODE_SFT: u32 = 0;
pub const VUL8_HD_MODE_MASK: u32 = 0x3;
pub const VUL8_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_VUL8_MON0 */
/* duplicate C macro: MEM_HW_WEN_SFT = 20 */
/* duplicate C macro: MEM_HW_WEN_MASK = 0xf */
/* duplicate C macro: MEM_HW_WEN_MASK_SFT = 0xf << 20 */
/* duplicate C macro: MEM_REQ_PENDING_SFT = 19 */
/* duplicate C macro: MEM_REQ_PENDING_MASK = 0x1 */
/* duplicate C macro: MEM_REQ_PENDING_MASK_SFT = 0x1 << 19 */
/* duplicate C macro: BUF_FULL_SFT = 18 */
/* duplicate C macro: BUF_FULL_MASK = 0x1 */
/* duplicate C macro: BUF_FULL_MASK_SFT = 0x1 << 18 */
/* duplicate C macro: ENABLE_SYNC_MEM_SFT = 17 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK_SFT = 0x1 << 17 */
/* duplicate C macro: ENABLE_SYNC_AGENT_SFT = 16 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK_SFT = 0x1 << 16 */
/* duplicate C macro: RESERVED_02_SFT = 6 */
/* duplicate C macro: RESERVED_02_MASK = 0x3ff */
/* duplicate C macro: RESERVED_02_MASK_SFT = 0x3ff << 6 */
/* duplicate C macro: MEM_ADDR_DIFF_SFT = 0 */
/* duplicate C macro: MEM_ADDR_DIFF_MASK = 0x3f */
/* duplicate C macro: MEM_ADDR_DIFF_MASK_SFT = 0x3f << 0 */

/* AFE_VUL9_BASE_MSB */
pub const VUL9_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL9_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL9_BASE_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL9_BASE */
pub const VUL9_BASE_ADDR_SFT: u32 = 4;
pub const VUL9_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL9_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL9_CUR_MSB */
pub const VUL9_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL9_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL9_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL9_CUR */
pub const VUL9_CUR_PTR_SFT: u32 = 0;
pub const VUL9_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL9_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL9_END_MSB */
pub const VUL9_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL9_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL9_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL9_END */
pub const VUL9_END_ADDR_SFT: u32 = 4;
pub const VUL9_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL9_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL9_RCH_MON */
pub const VUL9_RCH_DATA_SFT: u32 = 0;
pub const VUL9_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL9_RCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL9_LCH_MON */
pub const VUL9_LCH_DATA_SFT: u32 = 0;
pub const VUL9_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL9_LCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL9_CON0 */
pub const VUL9_ON_SFT: u32 = 28;
pub const VUL9_ON_MASK: u32 = 0x1;
pub const VUL9_ON_MASK_SFT: u32 = 0x1 << 28;
pub const VUL9_MINLEN_SFT: u32 = 20;
pub const VUL9_MINLEN_MASK: u32 = 0x3;
pub const VUL9_MINLEN_MASK_SFT: u32 = 0x3 << 20;
pub const VUL9_MAXLEN_SFT: u32 = 16;
pub const VUL9_MAXLEN_MASK: u32 = 0x3;
pub const VUL9_MAXLEN_MASK_SFT: u32 = 0x3 << 16;
pub const VUL9_SEL_DOMAIN_SFT: u32 = 13;
pub const VUL9_SEL_DOMAIN_MASK: u32 = 0x7;
pub const VUL9_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const VUL9_SEL_FS_SFT: u32 = 8;
pub const VUL9_SEL_FS_MASK: u32 = 0x1f;
pub const VUL9_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const VUL9_SW_CLEAR_BUF_FULL_SFT: u32 = 7;
pub const VUL9_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL9_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = 0x1 << 7;
pub const VUL9_WR_SIGN_SFT: u32 = 6;
pub const VUL9_WR_SIGN_MASK: u32 = 0x1;
pub const VUL9_WR_SIGN_MASK_SFT: u32 = 0x1 << 6;
pub const VUL9_R_MONO_SFT: u32 = 5;
pub const VUL9_R_MONO_MASK: u32 = 0x1;
pub const VUL9_R_MONO_MASK_SFT: u32 = 0x1 << 5;
pub const VUL9_MONO_SFT: u32 = 4;
pub const VUL9_MONO_MASK: u32 = 0x1;
pub const VUL9_MONO_MASK_SFT: u32 = 0x1 << 4;
pub const VUL9_NORMAL_MODE_SFT: u32 = 3;
pub const VUL9_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL9_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 3;
pub const VUL9_HALIGN_SFT: u32 = 2;
pub const VUL9_HALIGN_MASK: u32 = 0x1;
pub const VUL9_HALIGN_MASK_SFT: u32 = 0x1 << 2;
pub const VUL9_HD_MODE_SFT: u32 = 0;
pub const VUL9_HD_MODE_MASK: u32 = 0x3;
pub const VUL9_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_VUL9_MON0 */
/* duplicate C macro: MEM_HW_WEN_SFT = 20 */
/* duplicate C macro: MEM_HW_WEN_MASK = 0xf */
/* duplicate C macro: MEM_HW_WEN_MASK_SFT = 0xf << 20 */
/* duplicate C macro: MEM_REQ_PENDING_SFT = 19 */
/* duplicate C macro: MEM_REQ_PENDING_MASK = 0x1 */
/* duplicate C macro: MEM_REQ_PENDING_MASK_SFT = 0x1 << 19 */
/* duplicate C macro: BUF_FULL_SFT = 18 */
/* duplicate C macro: BUF_FULL_MASK = 0x1 */
/* duplicate C macro: BUF_FULL_MASK_SFT = 0x1 << 18 */
/* duplicate C macro: ENABLE_SYNC_MEM_SFT = 17 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK_SFT = 0x1 << 17 */
/* duplicate C macro: ENABLE_SYNC_AGENT_SFT = 16 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK_SFT = 0x1 << 16 */
/* duplicate C macro: RESERVED_02_SFT = 6 */
/* duplicate C macro: RESERVED_02_MASK = 0x3ff */
/* duplicate C macro: RESERVED_02_MASK_SFT = 0x3ff << 6 */
/* duplicate C macro: MEM_ADDR_DIFF_SFT = 0 */
/* duplicate C macro: MEM_ADDR_DIFF_MASK = 0x3f */
/* duplicate C macro: MEM_ADDR_DIFF_MASK_SFT = 0x3f << 0 */

/* AFE_VUL10_BASE_MSB */
pub const VUL10_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL10_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL10_BASE_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL10_BASE */
pub const VUL10_BASE_ADDR_SFT: u32 = 4;
pub const VUL10_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL10_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL10_CUR_MSB */
pub const VUL10_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL10_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL10_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL10_CUR */
pub const VUL10_CUR_PTR_SFT: u32 = 0;
pub const VUL10_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL10_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL10_END_MSB */
pub const VUL10_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL10_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL10_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL10_END */
pub const VUL10_END_ADDR_SFT: u32 = 4;
pub const VUL10_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL10_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL10_RCH_MON */
pub const VUL10_RCH_DATA_SFT: u32 = 0;
pub const VUL10_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL10_RCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL10_LCH_MON */
pub const VUL10_LCH_DATA_SFT: u32 = 0;
pub const VUL10_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL10_LCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL10_CON0 */
pub const VUL10_ON_SFT: u32 = 28;
pub const VUL10_ON_MASK: u32 = 0x1;
pub const VUL10_ON_MASK_SFT: u32 = 0x1 << 28;
pub const VUL10_MINLEN_SFT: u32 = 20;
pub const VUL10_MINLEN_MASK: u32 = 0x3;
pub const VUL10_MINLEN_MASK_SFT: u32 = 0x3 << 20;
pub const VUL10_MAXLEN_SFT: u32 = 16;
pub const VUL10_MAXLEN_MASK: u32 = 0x3;
pub const VUL10_MAXLEN_MASK_SFT: u32 = 0x3 << 16;
pub const VUL10_SEL_DOMAIN_SFT: u32 = 13;
pub const VUL10_SEL_DOMAIN_MASK: u32 = 0x7;
pub const VUL10_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const VUL10_SEL_FS_SFT: u32 = 8;
pub const VUL10_SEL_FS_MASK: u32 = 0x1f;
pub const VUL10_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const VUL10_SW_CLEAR_BUF_FULL_SFT: u32 = 7;
pub const VUL10_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL10_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = 0x1 << 7;
pub const VUL10_WR_SIGN_SFT: u32 = 6;
pub const VUL10_WR_SIGN_MASK: u32 = 0x1;
pub const VUL10_WR_SIGN_MASK_SFT: u32 = 0x1 << 6;
pub const VUL10_R_MONO_SFT: u32 = 5;
pub const VUL10_R_MONO_MASK: u32 = 0x1;
pub const VUL10_R_MONO_MASK_SFT: u32 = 0x1 << 5;
pub const VUL10_MONO_SFT: u32 = 4;
pub const VUL10_MONO_MASK: u32 = 0x1;
pub const VUL10_MONO_MASK_SFT: u32 = 0x1 << 4;
pub const VUL10_NORMAL_MODE_SFT: u32 = 3;
pub const VUL10_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL10_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 3;
pub const VUL10_HALIGN_SFT: u32 = 2;
pub const VUL10_HALIGN_MASK: u32 = 0x1;
pub const VUL10_HALIGN_MASK_SFT: u32 = 0x1 << 2;
pub const VUL10_HD_MODE_SFT: u32 = 0;
pub const VUL10_HD_MODE_MASK: u32 = 0x3;
pub const VUL10_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_VUL10_MON0 */
/* duplicate C macro: MEM_HW_WEN_SFT = 20 */
/* duplicate C macro: MEM_HW_WEN_MASK = 0xf */
/* duplicate C macro: MEM_HW_WEN_MASK_SFT = 0xf << 20 */
/* duplicate C macro: MEM_REQ_PENDING_SFT = 19 */
/* duplicate C macro: MEM_REQ_PENDING_MASK = 0x1 */
/* duplicate C macro: MEM_REQ_PENDING_MASK_SFT = 0x1 << 19 */
/* duplicate C macro: BUF_FULL_SFT = 18 */
/* duplicate C macro: BUF_FULL_MASK = 0x1 */
/* duplicate C macro: BUF_FULL_MASK_SFT = 0x1 << 18 */
/* duplicate C macro: ENABLE_SYNC_MEM_SFT = 17 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK_SFT = 0x1 << 17 */
/* duplicate C macro: ENABLE_SYNC_AGENT_SFT = 16 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK_SFT = 0x1 << 16 */
/* duplicate C macro: RESERVED_02_SFT = 6 */
/* duplicate C macro: RESERVED_02_MASK = 0x3ff */
/* duplicate C macro: RESERVED_02_MASK_SFT = 0x3ff << 6 */
/* duplicate C macro: MEM_ADDR_DIFF_SFT = 0 */
/* duplicate C macro: MEM_ADDR_DIFF_MASK = 0x3f */
/* duplicate C macro: MEM_ADDR_DIFF_MASK_SFT = 0x3f << 0 */

/* AFE_VUL24_BASE_MSB */
pub const VUL24_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL24_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL24_BASE_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL24_BASE */
pub const VUL24_BASE_ADDR_SFT: u32 = 4;
pub const VUL24_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL24_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL24_CUR_MSB */
pub const VUL24_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL24_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL24_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL24_CUR */
pub const VUL24_CUR_PTR_SFT: u32 = 0;
pub const VUL24_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL24_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL24_END_MSB */
pub const VUL24_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL24_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL24_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL24_END */
pub const VUL24_END_ADDR_SFT: u32 = 4;
pub const VUL24_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL24_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL24_CON0 */
pub const OUT_ON_USE_VUL24_SFT: u32 = 29;
pub const OUT_ON_USE_VUL24_MASK: u32 = 0x1;
pub const OUT_ON_USE_VUL24_MASK_SFT: u32 = 0x1 << 29;
pub const VUL24_ON_SFT: u32 = 28;
pub const VUL24_ON_MASK: u32 = 0x1;
pub const VUL24_ON_MASK_SFT: u32 = 0x1 << 28;
pub const VUL24_MINLEN_SFT: u32 = 20;
pub const VUL24_MINLEN_MASK: u32 = 0x3;
pub const VUL24_MINLEN_MASK_SFT: u32 = 0x3 << 20;
pub const VUL24_MAXLEN_SFT: u32 = 16;
pub const VUL24_MAXLEN_MASK: u32 = 0x3;
pub const VUL24_MAXLEN_MASK_SFT: u32 = 0x3 << 16;
pub const VUL24_SEL_DOMAIN_SFT: u32 = 13;
pub const VUL24_SEL_DOMAIN_MASK: u32 = 0x7;
pub const VUL24_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const VUL24_SEL_FS_SFT: u32 = 8;
pub const VUL24_SEL_FS_MASK: u32 = 0x1f;
pub const VUL24_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const VUL24_SW_CLEAR_BUF_FULL_SFT: u32 = 7;
pub const VUL24_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL24_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = 0x1 << 7;
pub const VUL24_WR_SIGN_SFT: u32 = 6;
pub const VUL24_WR_SIGN_MASK: u32 = 0x1;
pub const VUL24_WR_SIGN_MASK_SFT: u32 = 0x1 << 6;
pub const VUL24_R_MONO_SFT: u32 = 5;
pub const VUL24_R_MONO_MASK: u32 = 0x1;
pub const VUL24_R_MONO_MASK_SFT: u32 = 0x1 << 5;
pub const VUL24_MONO_SFT: u32 = 4;
pub const VUL24_MONO_MASK: u32 = 0x1;
pub const VUL24_MONO_MASK_SFT: u32 = 0x1 << 4;
pub const VUL24_NORMAL_MODE_SFT: u32 = 3;
pub const VUL24_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL24_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 3;
pub const VUL24_HALIGN_SFT: u32 = 2;
pub const VUL24_HALIGN_MASK: u32 = 0x1;
pub const VUL24_HALIGN_MASK_SFT: u32 = 0x1 << 2;
pub const VUL24_HD_MODE_SFT: u32 = 0;
pub const VUL24_HD_MODE_MASK: u32 = 0x3;
pub const VUL24_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_VUL24_MON0 */
/* duplicate C macro: MEM_HW_WEN_SFT = 20 */
/* duplicate C macro: MEM_HW_WEN_MASK = 0xf */
/* duplicate C macro: MEM_HW_WEN_MASK_SFT = 0xf << 20 */
/* duplicate C macro: MEM_REQ_PENDING_SFT = 19 */
/* duplicate C macro: MEM_REQ_PENDING_MASK = 0x1 */
/* duplicate C macro: MEM_REQ_PENDING_MASK_SFT = 0x1 << 19 */
/* duplicate C macro: BUF_FULL_SFT = 18 */
/* duplicate C macro: BUF_FULL_MASK = 0x1 */
/* duplicate C macro: BUF_FULL_MASK_SFT = 0x1 << 18 */
/* duplicate C macro: ENABLE_SYNC_MEM_SFT = 17 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK_SFT = 0x1 << 17 */
/* duplicate C macro: ENABLE_SYNC_AGENT_SFT = 16 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK_SFT = 0x1 << 16 */
/* duplicate C macro: RESERVED_02_SFT = 6 */
/* duplicate C macro: RESERVED_02_MASK = 0x3ff */
/* duplicate C macro: RESERVED_02_MASK_SFT = 0x3ff << 6 */
/* duplicate C macro: MEM_ADDR_DIFF_SFT = 0 */
/* duplicate C macro: MEM_ADDR_DIFF_MASK = 0x3f */
/* duplicate C macro: MEM_ADDR_DIFF_MASK_SFT = 0x3f << 0 */

/* AFE_VUL25_BASE_MSB */
pub const VUL25_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL25_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL25_BASE_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL25_BASE */
pub const VUL25_BASE_ADDR_SFT: u32 = 4;
pub const VUL25_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL25_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL25_CUR_MSB */
pub const VUL25_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL25_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL25_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL25_CUR */
pub const VUL25_CUR_PTR_SFT: u32 = 0;
pub const VUL25_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL25_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL25_END_MSB */
pub const VUL25_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL25_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL25_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL25_END */
pub const VUL25_END_ADDR_SFT: u32 = 4;
pub const VUL25_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL25_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL25_CON0 */
pub const OUT_ON_USE_VUL25_SFT: u32 = 29;
pub const OUT_ON_USE_VUL25_MASK: u32 = 0x1;
pub const OUT_ON_USE_VUL25_MASK_SFT: u32 = 0x1 << 29;
pub const VUL25_ON_SFT: u32 = 28;
pub const VUL25_ON_MASK: u32 = 0x1;
pub const VUL25_ON_MASK_SFT: u32 = 0x1 << 28;
pub const VUL25_MINLEN_SFT: u32 = 20;
pub const VUL25_MINLEN_MASK: u32 = 0x3;
pub const VUL25_MINLEN_MASK_SFT: u32 = 0x3 << 20;
pub const VUL25_MAXLEN_SFT: u32 = 16;
pub const VUL25_MAXLEN_MASK: u32 = 0x3;
pub const VUL25_MAXLEN_MASK_SFT: u32 = 0x3 << 16;
pub const VUL25_SEL_DOMAIN_SFT: u32 = 13;
pub const VUL25_SEL_DOMAIN_MASK: u32 = 0x7;
pub const VUL25_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 13;
pub const VUL25_SEL_FS_SFT: u32 = 8;
pub const VUL25_SEL_FS_MASK: u32 = 0x1f;
pub const VUL25_SEL_FS_MASK_SFT: u32 = 0x1f << 8;
pub const VUL25_SW_CLEAR_BUF_FULL_SFT: u32 = 7;
pub const VUL25_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL25_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = 0x1 << 7;
pub const VUL25_WR_SIGN_SFT: u32 = 6;
pub const VUL25_WR_SIGN_MASK: u32 = 0x1;
pub const VUL25_WR_SIGN_MASK_SFT: u32 = 0x1 << 6;
pub const VUL25_R_MONO_SFT: u32 = 5;
pub const VUL25_R_MONO_MASK: u32 = 0x1;
pub const VUL25_R_MONO_MASK_SFT: u32 = 0x1 << 5;
pub const VUL25_MONO_SFT: u32 = 4;
pub const VUL25_MONO_MASK: u32 = 0x1;
pub const VUL25_MONO_MASK_SFT: u32 = 0x1 << 4;
pub const VUL25_NORMAL_MODE_SFT: u32 = 3;
pub const VUL25_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL25_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 3;
pub const VUL25_HALIGN_SFT: u32 = 2;
pub const VUL25_HALIGN_MASK: u32 = 0x1;
pub const VUL25_HALIGN_MASK_SFT: u32 = 0x1 << 2;
pub const VUL25_HD_MODE_SFT: u32 = 0;
pub const VUL25_HD_MODE_MASK: u32 = 0x3;
pub const VUL25_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_VUL25_MON0 */
/* duplicate C macro: MEM_HW_WEN_SFT = 20 */
/* duplicate C macro: MEM_HW_WEN_MASK = 0xf */
/* duplicate C macro: MEM_HW_WEN_MASK_SFT = 0xf << 20 */
/* duplicate C macro: MEM_REQ_PENDING_SFT = 19 */
/* duplicate C macro: MEM_REQ_PENDING_MASK = 0x1 */
/* duplicate C macro: MEM_REQ_PENDING_MASK_SFT = 0x1 << 19 */
/* duplicate C macro: BUF_FULL_SFT = 18 */
/* duplicate C macro: BUF_FULL_MASK = 0x1 */
/* duplicate C macro: BUF_FULL_MASK_SFT = 0x1 << 18 */
/* duplicate C macro: ENABLE_SYNC_MEM_SFT = 17 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_MEM_MASK_SFT = 0x1 << 17 */
/* duplicate C macro: ENABLE_SYNC_AGENT_SFT = 16 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK = 0x1 */
/* duplicate C macro: ENABLE_SYNC_AGENT_MASK_SFT = 0x1 << 16 */
/* duplicate C macro: RESERVED_02_SFT = 6 */
/* duplicate C macro: RESERVED_02_MASK = 0x3ff */
/* duplicate C macro: RESERVED_02_MASK_SFT = 0x3ff << 6 */
/* duplicate C macro: MEM_ADDR_DIFF_SFT = 0 */
/* duplicate C macro: MEM_ADDR_DIFF_MASK = 0x3f */
/* duplicate C macro: MEM_ADDR_DIFF_MASK_SFT = 0x3f << 0 */

/* AFE_VUL_CM0_BASE_MSB */
pub const VUL_CM0_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL_CM0_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL_CM0_BASE_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL_CM0_BASE */
pub const VUL_CM0_BASE_ADDR_SFT: u32 = 4;
pub const VUL_CM0_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL_CM0_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL_CM0_CUR_MSB */
pub const VUL_CM0_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL_CM0_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL_CM0_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL_CM0_CUR */
pub const VUL_CM0_CUR_PTR_SFT: u32 = 0;
pub const VUL_CM0_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL_CM0_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL_CM0_END_MSB */
pub const VUL_CM0_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL_CM0_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL_CM0_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL_CM0_END */
pub const VUL_CM0_END_ADDR_SFT: u32 = 4;
pub const VUL_CM0_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL_CM0_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL_CM0_CON0 */
pub const VUL_CM0_ON_SFT: u32 = 28;
pub const VUL_CM0_ON_MASK: u32 = 0x1;
pub const VUL_CM0_ON_MASK_SFT: u32 = 0x1 << 28;
pub const VUL_CM0_REG_CH_SHIFT_MODE_SFT: u32 = 26;
pub const VUL_CM0_REG_CH_SHIFT_MODE_MASK: u32 = 0x1;
pub const VUL_CM0_REG_CH_SHIFT_MODE_MASK_SFT: u32 = 0x1 << 26;
pub const VUL_CM0_RG_FORCE_NO_MASK_EXTRA_SFT: u32 = 25;
pub const VUL_CM0_RG_FORCE_NO_MASK_EXTRA_MASK: u32 = 0x1;
pub const VUL_CM0_RG_FORCE_NO_MASK_EXTRA_MASK_SFT: u32 = 0x1 << 25;
pub const VUL_CM0_SW_CLEAR_BUF_FULL_SFT: u32 = 24;
pub const VUL_CM0_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL_CM0_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = 0x1 << 24;
pub const VUL_CM0_ULTRA_TH_SFT: u32 = 20;
pub const VUL_CM0_ULTRA_TH_MASK: u32 = 0xf;
pub const VUL_CM0_ULTRA_TH_MASK_SFT: u32 = 0xf << 20;
pub const VUL_CM0_NORMAL_MODE_SFT: u32 = 17;
pub const VUL_CM0_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL_CM0_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 17;
pub const VUL_CM0_ODD_USE_EVEN_SFT: u32 = 16;
pub const VUL_CM0_ODD_USE_EVEN_MASK: u32 = 0x1;
pub const VUL_CM0_ODD_USE_EVEN_MASK_SFT: u32 = 0x1 << 16;
pub const VUL_CM0_AXI_REQ_MAXLEN_SFT: u32 = 12;
pub const VUL_CM0_AXI_REQ_MAXLEN_MASK: u32 = 0x3;
pub const VUL_CM0_AXI_REQ_MAXLEN_MASK_SFT: u32 = 0x3 << 12;
pub const VUL_CM0_AXI_REQ_MINLEN_SFT: u32 = 8;
pub const VUL_CM0_AXI_REQ_MINLEN_MASK: u32 = 0x3;
pub const VUL_CM0_AXI_REQ_MINLEN_MASK_SFT: u32 = 0x3 << 8;
pub const VUL_CM0_HALIGN_SFT: u32 = 7;
pub const VUL_CM0_HALIGN_MASK: u32 = 0x1;
pub const VUL_CM0_HALIGN_MASK_SFT: u32 = 0x1 << 7;
pub const VUL_CM0_SIGN_EXT_SFT: u32 = 6;
pub const VUL_CM0_SIGN_EXT_MASK: u32 = 0x1;
pub const VUL_CM0_SIGN_EXT_MASK_SFT: u32 = 0x1 << 6;
pub const VUL_CM0_HD_MODE_SFT: u32 = 4;
pub const VUL_CM0_HD_MODE_MASK: u32 = 0x3;
pub const VUL_CM0_HD_MODE_MASK_SFT: u32 = 0x3 << 4;
pub const VUL_CM0_MAKE_EXTRA_UPDATE_SFT: u32 = 3;
pub const VUL_CM0_MAKE_EXTRA_UPDATE_MASK: u32 = 0x1;
pub const VUL_CM0_MAKE_EXTRA_UPDATE_MASK_SFT: u32 = 0x1 << 3;
pub const VUL_CM0_AGENT_FREE_RUN_SFT: u32 = 2;
pub const VUL_CM0_AGENT_FREE_RUN_MASK: u32 = 0x1;
pub const VUL_CM0_AGENT_FREE_RUN_MASK_SFT: u32 = 0x1 << 2;
pub const VUL_CM0_USE_INT_ODD_SFT: u32 = 1;
pub const VUL_CM0_USE_INT_ODD_MASK: u32 = 0x1;
pub const VUL_CM0_USE_INT_ODD_MASK_SFT: u32 = 0x1 << 1;
pub const VUL_CM0_INT_ODD_FLAG_SFT: u32 = 0;
pub const VUL_CM0_INT_ODD_FLAG_MASK: u32 = 0x1;
pub const VUL_CM0_INT_ODD_FLAG_MASK_SFT: u32 = 0x1 << 0;

/* AFE_VUL_CM1_BASE_MSB */
pub const VUL_CM1_BASE_ADDR_MSB_SFT: u32 = 0;
pub const VUL_CM1_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL_CM1_BASE_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL_CM1_BASE */
pub const VUL_CM1_BASE_ADDR_SFT: u32 = 4;
pub const VUL_CM1_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const VUL_CM1_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL_CM1_CUR_MSB */
pub const VUL_CM1_CUR_PTR_MSB_SFT: u32 = 0;
pub const VUL_CM1_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const VUL_CM1_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL_CM1_CUR */
pub const VUL_CM1_CUR_PTR_SFT: u32 = 0;
pub const VUL_CM1_CUR_PTR_MASK: u32 = 0xffffffff;
pub const VUL_CM1_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL_CM1_END_MSB */
pub const VUL_CM1_END_ADDR_MSB_SFT: u32 = 0;
pub const VUL_CM1_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const VUL_CM1_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_VUL_CM1_END */
pub const VUL_CM1_END_ADDR_SFT: u32 = 4;
pub const VUL_CM1_END_ADDR_MASK: u32 = 0xfffffff;
pub const VUL_CM1_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_VUL_CM1_CON0 */
pub const VUL_CM1_ON_SFT: u32 = 28;
pub const VUL_CM1_ON_MASK: u32 = 0x1;
pub const VUL_CM1_ON_MASK_SFT: u32 = 0x1 << 28;
pub const VUL_CM1_REG_CH_SHIFT_MODE_SFT: u32 = 26;
pub const VUL_CM1_REG_CH_SHIFT_MODE_MASK: u32 = 0x1;
pub const VUL_CM1_REG_CH_SHIFT_MODE_MASK_SFT: u32 = 0x1 << 26;
pub const VUL_CM1_RG_FORCE_NO_MASK_EXTRA_SFT: u32 = 25;
pub const VUL_CM1_RG_FORCE_NO_MASK_EXTRA_MASK: u32 = 0x1;
pub const VUL_CM1_RG_FORCE_NO_MASK_EXTRA_MASK_SFT: u32 = 0x1 << 25;
pub const VUL_CM1_SW_CLEAR_BUF_FULL_SFT: u32 = 24;
pub const VUL_CM1_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const VUL_CM1_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = 0x1 << 24;
pub const VUL_CM1_ULTRA_TH_SFT: u32 = 20;
pub const VUL_CM1_ULTRA_TH_MASK: u32 = 0xf;
pub const VUL_CM1_ULTRA_TH_MASK_SFT: u32 = 0xf << 20;
pub const VUL_CM1_NORMAL_MODE_SFT: u32 = 17;
pub const VUL_CM1_NORMAL_MODE_MASK: u32 = 0x1;
pub const VUL_CM1_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 17;
pub const VUL_CM1_ODD_USE_EVEN_SFT: u32 = 16;
pub const VUL_CM1_ODD_USE_EVEN_MASK: u32 = 0x1;
pub const VUL_CM1_ODD_USE_EVEN_MASK_SFT: u32 = 0x1 << 16;
pub const VUL_CM1_AXI_REQ_MAXLEN_SFT: u32 = 12;
pub const VUL_CM1_AXI_REQ_MAXLEN_MASK: u32 = 0x3;
pub const VUL_CM1_AXI_REQ_MAXLEN_MASK_SFT: u32 = 0x3 << 12;
pub const VUL_CM1_AXI_REQ_MINLEN_SFT: u32 = 8;
pub const VUL_CM1_AXI_REQ_MINLEN_MASK: u32 = 0x3;
pub const VUL_CM1_AXI_REQ_MINLEN_MASK_SFT: u32 = 0x3 << 8;
pub const VUL_CM1_HALIGN_SFT: u32 = 7;
pub const VUL_CM1_HALIGN_MASK: u32 = 0x1;
pub const VUL_CM1_HALIGN_MASK_SFT: u32 = 0x1 << 7;
pub const VUL_CM1_SIGN_EXT_SFT: u32 = 6;
pub const VUL_CM1_SIGN_EXT_MASK: u32 = 0x1;
pub const VUL_CM1_SIGN_EXT_MASK_SFT: u32 = 0x1 << 6;
pub const VUL_CM1_HD_MODE_SFT: u32 = 4;
pub const VUL_CM1_HD_MODE_MASK: u32 = 0x3;
pub const VUL_CM1_HD_MODE_MASK_SFT: u32 = 0x3 << 4;
pub const VUL_CM1_MAKE_EXTRA_UPDATE_SFT: u32 = 3;
pub const VUL_CM1_MAKE_EXTRA_UPDATE_MASK: u32 = 0x1;
pub const VUL_CM1_MAKE_EXTRA_UPDATE_MASK_SFT: u32 = 0x1 << 3;
pub const VUL_CM1_AGENT_FREE_RUN_SFT: u32 = 2;
pub const VUL_CM1_AGENT_FREE_RUN_MASK: u32 = 0x1;
pub const VUL_CM1_AGENT_FREE_RUN_MASK_SFT: u32 = 0x1 << 2;
pub const VUL_CM1_USE_INT_ODD_SFT: u32 = 1;
pub const VUL_CM1_USE_INT_ODD_MASK: u32 = 0x1;
pub const VUL_CM1_USE_INT_ODD_MASK_SFT: u32 = 0x1 << 1;
pub const VUL_CM1_INT_ODD_FLAG_SFT: u32 = 0;
pub const VUL_CM1_INT_ODD_FLAG_MASK: u32 = 0x1;
pub const VUL_CM1_INT_ODD_FLAG_MASK_SFT: u32 = 0x1 << 0;

/* AFE_ETDM_IN0_BASE_MSB */
pub const ETDM_IN0_BASE_ADDR_MSB_SFT: u32 = 0;
pub const ETDM_IN0_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const ETDM_IN0_BASE_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_ETDM_IN0_BASE */
pub const ETDM_IN0_BASE_ADDR_SFT: u32 = 4;
pub const ETDM_IN0_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const ETDM_IN0_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_ETDM_IN0_CUR_MSB */
pub const ETDM_IN0_CUR_PTR_MSB_SFT: u32 = 0;
pub const ETDM_IN0_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const ETDM_IN0_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_ETDM_IN0_CUR */
pub const ETDM_IN0_CUR_PTR_SFT: u32 = 0;
pub const ETDM_IN0_CUR_PTR_MASK: u32 = 0xffffffff;
pub const ETDM_IN0_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ETDM_IN0_END_MSB */
pub const ETDM_IN0_END_ADDR_MSB_SFT: u32 = 0;
pub const ETDM_IN0_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const ETDM_IN0_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_ETDM_IN0_END */
pub const ETDM_IN0_END_ADDR_SFT: u32 = 4;
pub const ETDM_IN0_END_ADDR_MASK: u32 = 0xfffffff;
pub const ETDM_IN0_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_ETDM_IN0_CON0 */
pub const ETDM_IN0_CH_NUM_SFT: u32 = 28;
pub const ETDM_IN0_CH_NUM_MASK: u32 = 0xf;
pub const ETDM_IN0_CH_NUM_MASK_SFT: u32 = 0xf << 28;
pub const ETDM_IN0_ON_SFT: u32 = 27;
pub const ETDM_IN0_ON_MASK: u32 = 0x1;
pub const ETDM_IN0_ON_MASK_SFT: u32 = 0x1 << 27;
pub const ETDM_IN0_REG_CH_SHIFT_MODE_SFT: u32 = 26;
pub const ETDM_IN0_REG_CH_SHIFT_MODE_MASK: u32 = 0x1;
pub const ETDM_IN0_REG_CH_SHIFT_MODE_MASK_SFT: u32 = 0x1 << 26;
pub const ETDM_IN0_RG_FORCE_NO_MASK_EXTRA_SFT: u32 = 25;
pub const ETDM_IN0_RG_FORCE_NO_MASK_EXTRA_MASK: u32 = 0x1;
pub const ETDM_IN0_RG_FORCE_NO_MASK_EXTRA_MASK_SFT: u32 = 0x1 << 25;
pub const ETDM_IN0_SW_CLEAR_BUF_FULL_SFT: u32 = 24;
pub const ETDM_IN0_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const ETDM_IN0_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = 0x1 << 24;
pub const ETDM_IN0_ULTRA_TH_SFT: u32 = 20;
pub const ETDM_IN0_ULTRA_TH_MASK: u32 = 0xf;
pub const ETDM_IN0_ULTRA_TH_MASK_SFT: u32 = 0xf << 20;
pub const ETDM_IN0_NORMAL_MODE_SFT: u32 = 17;
pub const ETDM_IN0_NORMAL_MODE_MASK: u32 = 0x1;
pub const ETDM_IN0_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 17;
pub const ETDM_IN0_ODD_USE_EVEN_SFT: u32 = 16;
pub const ETDM_IN0_ODD_USE_EVEN_MASK: u32 = 0x1;
pub const ETDM_IN0_ODD_USE_EVEN_MASK_SFT: u32 = 0x1 << 16;
pub const ETDM_IN0_AXI_REQ_MAXLEN_SFT: u32 = 12;
pub const ETDM_IN0_AXI_REQ_MAXLEN_MASK: u32 = 0x3;
pub const ETDM_IN0_AXI_REQ_MAXLEN_MASK_SFT: u32 = 0x3 << 12;
pub const ETDM_IN0_AXI_REQ_MINLEN_SFT: u32 = 8;
pub const ETDM_IN0_AXI_REQ_MINLEN_MASK: u32 = 0x3;
pub const ETDM_IN0_AXI_REQ_MINLEN_MASK_SFT: u32 = 0x3 << 8;
pub const ETDM_IN0_HALIGN_SFT: u32 = 7;
pub const ETDM_IN0_HALIGN_MASK: u32 = 0x1;
pub const ETDM_IN0_HALIGN_MASK_SFT: u32 = 0x1 << 7;
pub const ETDM_IN0_SIGN_EXT_SFT: u32 = 6;
pub const ETDM_IN0_SIGN_EXT_MASK: u32 = 0x1;
pub const ETDM_IN0_SIGN_EXT_MASK_SFT: u32 = 0x1 << 6;
pub const ETDM_IN0_HD_MODE_SFT: u32 = 4;
pub const ETDM_IN0_HD_MODE_MASK: u32 = 0x3;
pub const ETDM_IN0_HD_MODE_MASK_SFT: u32 = 0x3 << 4;
pub const ETDM_IN0_MAKE_EXTRA_UPDATE_SFT: u32 = 3;
pub const ETDM_IN0_MAKE_EXTRA_UPDATE_MASK: u32 = 0x1;
pub const ETDM_IN0_MAKE_EXTRA_UPDATE_MASK_SFT: u32 = 0x1 << 3;
pub const ETDM_IN0_AGENT_FREE_RUN_SFT: u32 = 2;
pub const ETDM_IN0_AGENT_FREE_RUN_MASK: u32 = 0x1;
pub const ETDM_IN0_AGENT_FREE_RUN_MASK_SFT: u32 = 0x1 << 2;
pub const ETDM_IN0_USE_INT_ODD_SFT: u32 = 1;
pub const ETDM_IN0_USE_INT_ODD_MASK: u32 = 0x1;
pub const ETDM_IN0_USE_INT_ODD_MASK_SFT: u32 = 0x1 << 1;
pub const ETDM_IN0_INT_ODD_FLAG_SFT: u32 = 0;
pub const ETDM_IN0_INT_ODD_FLAG_MASK: u32 = 0x1;
pub const ETDM_IN0_INT_ODD_FLAG_MASK_SFT: u32 = 0x1 << 0;

/* AFE_ETDM_IN1_BASE_MSB */
pub const ETDM_IN1_BASE_ADDR_MSB_SFT: u32 = 0;
pub const ETDM_IN1_BASE_ADDR_MSB_MASK: u32 = 0x1ff;
pub const ETDM_IN1_BASE_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_ETDM_IN1_BASE */
pub const ETDM_IN1_BASE_ADDR_SFT: u32 = 4;
pub const ETDM_IN1_BASE_ADDR_MASK: u32 = 0xfffffff;
pub const ETDM_IN1_BASE_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_ETDM_IN1_CUR_MSB */
pub const ETDM_IN1_CUR_PTR_MSB_SFT: u32 = 0;
pub const ETDM_IN1_CUR_PTR_MSB_MASK: u32 = 0x1ff;
pub const ETDM_IN1_CUR_PTR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_ETDM_IN1_CUR */
pub const ETDM_IN1_CUR_PTR_SFT: u32 = 0;
pub const ETDM_IN1_CUR_PTR_MASK: u32 = 0xffffffff;
pub const ETDM_IN1_CUR_PTR_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ETDM_IN1_END_MSB */
pub const ETDM_IN1_END_ADDR_MSB_SFT: u32 = 0;
pub const ETDM_IN1_END_ADDR_MSB_MASK: u32 = 0x1ff;
pub const ETDM_IN1_END_ADDR_MSB_MASK_SFT: u32 = 0x1ff << 0;

/* AFE_ETDM_IN1_END */
pub const ETDM_IN1_END_ADDR_SFT: u32 = 4;
pub const ETDM_IN1_END_ADDR_MASK: u32 = 0xfffffff;
pub const ETDM_IN1_END_ADDR_MASK_SFT: u32 = 0xfffffff << 4;

/* AFE_ETDM_IN1_CON0 */
pub const ETDM_IN1_CH_NUM_SFT: u32 = 28;
pub const ETDM_IN1_CH_NUM_MASK: u32 = 0xf;
pub const ETDM_IN1_CH_NUM_MASK_SFT: u32 = 0xf << 28;
pub const ETDM_IN1_ON_SFT: u32 = 27;
pub const ETDM_IN1_ON_MASK: u32 = 0x1;
pub const ETDM_IN1_ON_MASK_SFT: u32 = 0x1 << 27;
pub const ETDM_IN1_REG_CH_SHIFT_MODE_SFT: u32 = 26;
pub const ETDM_IN1_REG_CH_SHIFT_MODE_MASK: u32 = 0x1;
pub const ETDM_IN1_REG_CH_SHIFT_MODE_MASK_SFT: u32 = 0x1 << 26;
pub const ETDM_IN1_RG_FORCE_NO_MASK_EXTRA_SFT: u32 = 25;
pub const ETDM_IN1_RG_FORCE_NO_MASK_EXTRA_MASK: u32 = 0x1;
pub const ETDM_IN1_RG_FORCE_NO_MASK_EXTRA_MASK_SFT: u32 = 0x1 << 25;
pub const ETDM_IN1_SW_CLEAR_BUF_FULL_SFT: u32 = 24;
pub const ETDM_IN1_SW_CLEAR_BUF_FULL_MASK: u32 = 0x1;
pub const ETDM_IN1_SW_CLEAR_BUF_FULL_MASK_SFT: u32 = 0x1 << 24;
pub const ETDM_IN1_ULTRA_TH_SFT: u32 = 20;
pub const ETDM_IN1_ULTRA_TH_MASK: u32 = 0xf;
pub const ETDM_IN1_ULTRA_TH_MASK_SFT: u32 = 0xf << 20;
pub const ETDM_IN1_NORMAL_MODE_SFT: u32 = 17;
pub const ETDM_IN1_NORMAL_MODE_MASK: u32 = 0x1;
pub const ETDM_IN1_NORMAL_MODE_MASK_SFT: u32 = 0x1 << 17;
pub const ETDM_IN1_ODD_USE_EVEN_SFT: u32 = 16;
pub const ETDM_IN1_ODD_USE_EVEN_MASK: u32 = 0x1;
pub const ETDM_IN1_ODD_USE_EVEN_MASK_SFT: u32 = 0x1 << 16;
pub const ETDM_IN1_AXI_REQ_MAXLEN_SFT: u32 = 12;
pub const ETDM_IN1_AXI_REQ_MAXLEN_MASK: u32 = 0x3;
pub const ETDM_IN1_AXI_REQ_MAXLEN_MASK_SFT: u32 = 0x3 << 12;
pub const ETDM_IN1_AXI_REQ_MINLEN_SFT: u32 = 8;
pub const ETDM_IN1_AXI_REQ_MINLEN_MASK: u32 = 0x3;
pub const ETDM_IN1_AXI_REQ_MINLEN_MASK_SFT: u32 = 0x3 << 8;
pub const ETDM_IN1_HALIGN_SFT: u32 = 7;
pub const ETDM_IN1_HALIGN_MASK: u32 = 0x1;
pub const ETDM_IN1_HALIGN_MASK_SFT: u32 = 0x1 << 7;
pub const ETDM_IN1_SIGN_EXT_SFT: u32 = 6;
pub const ETDM_IN1_SIGN_EXT_MASK: u32 = 0x1;
pub const ETDM_IN1_SIGN_EXT_MASK_SFT: u32 = 0x1 << 6;
pub const ETDM_IN1_HD_MODE_SFT: u32 = 4;
pub const ETDM_IN1_HD_MODE_MASK: u32 = 0x3;
pub const ETDM_IN1_HD_MODE_MASK_SFT: u32 = 0x3 << 4;
pub const ETDM_IN1_MAKE_EXTRA_UPDATE_SFT: u32 = 3;
pub const ETDM_IN1_MAKE_EXTRA_UPDATE_MASK: u32 = 0x1;
pub const ETDM_IN1_MAKE_EXTRA_UPDATE_MASK_SFT: u32 = 0x1 << 3;
pub const ETDM_IN1_AGENT_FREE_RUN_SFT: u32 = 2;
pub const ETDM_IN1_AGENT_FREE_RUN_MASK: u32 = 0x1;
pub const ETDM_IN1_AGENT_FREE_RUN_MASK_SFT: u32 = 0x1 << 2;
pub const ETDM_IN1_USE_INT_ODD_SFT: u32 = 1;
pub const ETDM_IN1_USE_INT_ODD_MASK: u32 = 0x1;
pub const ETDM_IN1_USE_INT_ODD_MASK_SFT: u32 = 0x1 << 1;
pub const ETDM_IN1_INT_ODD_FLAG_SFT: u32 = 0;
pub const ETDM_IN1_INT_ODD_FLAG_MASK: u32 = 0x1;
pub const ETDM_IN1_INT_ODD_FLAG_MASK_SFT: u32 = 0x1 << 0;

/* AFE_VUL24_RCH_MON */
pub const VUL24_RCH_DATA_SFT: u32 = 0;
pub const VUL24_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL24_RCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL24_LCH_MON */
pub const VUL24_LCH_DATA_SFT: u32 = 0;
pub const VUL24_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL24_LCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL25_RCH_MON */
pub const VUL25_RCH_DATA_SFT: u32 = 0;
pub const VUL25_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL25_RCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL25_LCH_MON */
pub const VUL25_LCH_DATA_SFT: u32 = 0;
pub const VUL25_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL25_LCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL_CM0_RCH_MON */
pub const VUL_CM0_RCH_DATA_SFT: u32 = 0;
pub const VUL_CM0_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL_CM0_RCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL_CM0_LCH_MON */
pub const VUL_CM0_LCH_DATA_SFT: u32 = 0;
pub const VUL_CM0_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL_CM0_LCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL_CM1_RCH_MON */
pub const VUL_CM1_RCH_DATA_SFT: u32 = 0;
pub const VUL_CM1_RCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL_CM1_RCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_VUL_CM1_LCH_MON */
pub const VUL_CM1_LCH_DATA_SFT: u32 = 0;
pub const VUL_CM1_LCH_DATA_MASK: u32 = 0xffffffff;
pub const VUL_CM1_LCH_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL_24CH_CH0_MON */
pub const DL_24CH_CH0_DATA_SFT: u32 = 0;
pub const DL_24CH_CH0_DATA_MASK: u32 = 0xffffffff;
pub const DL_24CH_CH0_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL_24CH_CH1_MON */
pub const DL_24CH_CH1_DATA_SFT: u32 = 0;
pub const DL_24CH_CH1_DATA_MASK: u32 = 0xffffffff;
pub const DL_24CH_CH1_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL_24CH_CH2_MON */
pub const DL_24CH_CH2_DATA_SFT: u32 = 0;
pub const DL_24CH_CH2_DATA_MASK: u32 = 0xffffffff;
pub const DL_24CH_CH2_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL_24CH_CH3_MON */
pub const DL_24CH_CH3_DATA_SFT: u32 = 0;
pub const DL_24CH_CH3_DATA_MASK: u32 = 0xffffffff;
pub const DL_24CH_CH3_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL_24CH_CH4_MON */
pub const DL_24CH_CH4_DATA_SFT: u32 = 0;
pub const DL_24CH_CH4_DATA_MASK: u32 = 0xffffffff;
pub const DL_24CH_CH4_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL_24CH_CH5_MON */
pub const DL_24CH_CH5_DATA_SFT: u32 = 0;
pub const DL_24CH_CH5_DATA_MASK: u32 = 0xffffffff;
pub const DL_24CH_CH5_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL_24CH_CH6_MON */
pub const DL_24CH_CH6_DATA_SFT: u32 = 0;
pub const DL_24CH_CH6_DATA_MASK: u32 = 0xffffffff;
pub const DL_24CH_CH6_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DL_24CH_CH7_MON */
pub const DL_24CH_CH7_DATA_SFT: u32 = 0;
pub const DL_24CH_CH7_DATA_MASK: u32 = 0xffffffff;
pub const DL_24CH_CH7_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_SRAM_BOUND */
pub const SECURE_BIT_SFT: u32 = 19;
pub const SECURE_BIT_MASK: u32 = 0x1;
pub const SECURE_BIT_MASK_SFT: u32 = 0x1 << 19;
pub const SECURE_SRAM_BOUND_SFT: u32 = 0;
pub const SECURE_SRAM_BOUND_MASK: u32 = 0x7ffff;
pub const SECURE_SRAM_BOUND_MASK_SFT: u32 = 0x7ffff << 0;

/* AFE_SECURE_CON0 */
pub const READ_EN15_NS_SFT: u32 = 31;
pub const READ_EN15_NS_MASK: u32 = 0x1;
pub const READ_EN15_NS_MASK_SFT: u32 = 0x1 << 31;
pub const WRITE_EN15_NS_SFT: u32 = 30;
pub const WRITE_EN15_NS_MASK: u32 = 0x1;
pub const WRITE_EN15_NS_MASK_SFT: u32 = 0x1 << 30;
pub const READ_EN14_NS_SFT: u32 = 29;
pub const READ_EN14_NS_MASK: u32 = 0x1;
pub const READ_EN14_NS_MASK_SFT: u32 = 0x1 << 29;
pub const WRITE_EN14_NS_SFT: u32 = 28;
pub const WRITE_EN14_NS_MASK: u32 = 0x1;
pub const WRITE_EN14_NS_MASK_SFT: u32 = 0x1 << 28;
pub const READ_EN13_NS_SFT: u32 = 27;
pub const READ_EN13_NS_MASK: u32 = 0x1;
pub const READ_EN13_NS_MASK_SFT: u32 = 0x1 << 27;
pub const WRITE_EN13_NS_SFT: u32 = 26;
pub const WRITE_EN13_NS_MASK: u32 = 0x1;
pub const WRITE_EN13_NS_MASK_SFT: u32 = 0x1 << 26;
pub const READ_EN12_NS_SFT: u32 = 25;
pub const READ_EN12_NS_MASK: u32 = 0x1;
pub const READ_EN12_NS_MASK_SFT: u32 = 0x1 << 25;
pub const WRITE_EN12_NS_SFT: u32 = 24;
pub const WRITE_EN12_NS_MASK: u32 = 0x1;
pub const WRITE_EN12_NS_MASK_SFT: u32 = 0x1 << 24;
pub const READ_EN11_NS_SFT: u32 = 23;
pub const READ_EN11_NS_MASK: u32 = 0x1;
pub const READ_EN11_NS_MASK_SFT: u32 = 0x1 << 23;
pub const WRITE_EN11_NS_SFT: u32 = 22;
pub const WRITE_EN11_NS_MASK: u32 = 0x1;
pub const WRITE_EN11_NS_MASK_SFT: u32 = 0x1 << 22;
pub const READ_EN10_NS_SFT: u32 = 21;
pub const READ_EN10_NS_MASK: u32 = 0x1;
pub const READ_EN10_NS_MASK_SFT: u32 = 0x1 << 21;
pub const WRITE_EN10_NS_SFT: u32 = 20;
pub const WRITE_EN10_NS_MASK: u32 = 0x1;
pub const WRITE_EN10_NS_MASK_SFT: u32 = 0x1 << 20;
pub const READ_EN9_NS_SFT: u32 = 19;
pub const READ_EN9_NS_MASK: u32 = 0x1;
pub const READ_EN9_NS_MASK_SFT: u32 = 0x1 << 19;
pub const WRITE_EN9_NS_SFT: u32 = 18;
pub const WRITE_EN9_NS_MASK: u32 = 0x1;
pub const WRITE_EN9_NS_MASK_SFT: u32 = 0x1 << 18;
pub const READ_EN8_NS_SFT: u32 = 17;
pub const READ_EN8_NS_MASK: u32 = 0x1;
pub const READ_EN8_NS_MASK_SFT: u32 = 0x1 << 17;
pub const WRITE_EN8_NS_SFT: u32 = 16;
pub const WRITE_EN8_NS_MASK: u32 = 0x1;
pub const WRITE_EN8_NS_MASK_SFT: u32 = 0x1 << 16;
pub const READ_EN7_NS_SFT: u32 = 15;
pub const READ_EN7_NS_MASK: u32 = 0x1;
pub const READ_EN7_NS_MASK_SFT: u32 = 0x1 << 15;
pub const WRITE_EN7_NS_SFT: u32 = 14;
pub const WRITE_EN7_NS_MASK: u32 = 0x1;
pub const WRITE_EN7_NS_MASK_SFT: u32 = 0x1 << 14;
pub const READ_EN6_NS_SFT: u32 = 13;
pub const READ_EN6_NS_MASK: u32 = 0x1;
pub const READ_EN6_NS_MASK_SFT: u32 = 0x1 << 13;
pub const WRITE_EN6_NS_SFT: u32 = 12;
pub const WRITE_EN6_NS_MASK: u32 = 0x1;
pub const WRITE_EN6_NS_MASK_SFT: u32 = 0x1 << 12;
pub const READ_EN5_NS_SFT: u32 = 11;
pub const READ_EN5_NS_MASK: u32 = 0x1;
pub const READ_EN5_NS_MASK_SFT: u32 = 0x1 << 11;
pub const WRITE_EN5_NS_SFT: u32 = 10;
pub const WRITE_EN5_NS_MASK: u32 = 0x1;
pub const WRITE_EN5_NS_MASK_SFT: u32 = 0x1 << 10;
pub const READ_EN4_NS_SFT: u32 = 9;
pub const READ_EN4_NS_MASK: u32 = 0x1;
pub const READ_EN4_NS_MASK_SFT: u32 = 0x1 << 9;
pub const WRITE_EN4_NS_SFT: u32 = 8;
pub const WRITE_EN4_NS_MASK: u32 = 0x1;
pub const WRITE_EN4_NS_MASK_SFT: u32 = 0x1 << 8;
pub const READ_EN3_NS_SFT: u32 = 7;
pub const READ_EN3_NS_MASK: u32 = 0x1;
pub const READ_EN3_NS_MASK_SFT: u32 = 0x1 << 7;
pub const WRITE_EN3_NS_SFT: u32 = 6;
pub const WRITE_EN3_NS_MASK: u32 = 0x1;
pub const WRITE_EN3_NS_MASK_SFT: u32 = 0x1 << 6;
pub const READ_EN2_NS_SFT: u32 = 5;
pub const READ_EN2_NS_MASK: u32 = 0x1;
pub const READ_EN2_NS_MASK_SFT: u32 = 0x1 << 5;
pub const WRITE_EN2_NS_SFT: u32 = 4;
pub const WRITE_EN2_NS_MASK: u32 = 0x1;
pub const WRITE_EN2_NS_MASK_SFT: u32 = 0x1 << 4;
pub const READ_EN1_NS_SFT: u32 = 3;
pub const READ_EN1_NS_MASK: u32 = 0x1;
pub const READ_EN1_NS_MASK_SFT: u32 = 0x1 << 3;
pub const WRITE_EN1_NS_SFT: u32 = 2;
pub const WRITE_EN1_NS_MASK: u32 = 0x1;
pub const WRITE_EN1_NS_MASK_SFT: u32 = 0x1 << 2;
pub const READ_EN0_NS_SFT: u32 = 1;
pub const READ_EN0_NS_MASK: u32 = 0x1;
pub const READ_EN0_NS_MASK_SFT: u32 = 0x1 << 1;
pub const WRITE_EN0_NS_SFT: u32 = 0;
pub const WRITE_EN0_NS_MASK: u32 = 0x1;
pub const WRITE_EN0_NS_MASK_SFT: u32 = 0x1 << 0;

/* AFE_SECURE_CON1 */
pub const READ_EN15_S_SFT: u32 = 31;
pub const READ_EN15_S_MASK: u32 = 0x1;
pub const READ_EN15_S_MASK_SFT: u32 = 0x1 << 31;
pub const WRITE_EN15_S_SFT: u32 = 30;
pub const WRITE_EN15_S_MASK: u32 = 0x1;
pub const WRITE_EN15_S_MASK_SFT: u32 = 0x1 << 30;
pub const READ_EN14_S_SFT: u32 = 29;
pub const READ_EN14_S_MASK: u32 = 0x1;
pub const READ_EN14_S_MASK_SFT: u32 = 0x1 << 29;
pub const WRITE_EN14_S_SFT: u32 = 28;
pub const WRITE_EN14_S_MASK: u32 = 0x1;
pub const WRITE_EN14_S_MASK_SFT: u32 = 0x1 << 28;
pub const READ_EN13_S_SFT: u32 = 27;
pub const READ_EN13_S_MASK: u32 = 0x1;
pub const READ_EN13_S_MASK_SFT: u32 = 0x1 << 27;
pub const WRITE_EN13_S_SFT: u32 = 26;
pub const WRITE_EN13_S_MASK: u32 = 0x1;
pub const WRITE_EN13_S_MASK_SFT: u32 = 0x1 << 26;
pub const READ_EN12_S_SFT: u32 = 25;
pub const READ_EN12_S_MASK: u32 = 0x1;
pub const READ_EN12_S_MASK_SFT: u32 = 0x1 << 25;
pub const WRITE_EN12_S_SFT: u32 = 24;
pub const WRITE_EN12_S_MASK: u32 = 0x1;
pub const WRITE_EN12_S_MASK_SFT: u32 = 0x1 << 24;
pub const READ_EN11_S_SFT: u32 = 23;
pub const READ_EN11_S_MASK: u32 = 0x1;
pub const READ_EN11_S_MASK_SFT: u32 = 0x1 << 23;
pub const WRITE_EN11_S_SFT: u32 = 22;
pub const WRITE_EN11_S_MASK: u32 = 0x1;
pub const WRITE_EN11_S_MASK_SFT: u32 = 0x1 << 22;
pub const READ_EN10_S_SFT: u32 = 21;
pub const READ_EN10_S_MASK: u32 = 0x1;
pub const READ_EN10_S_MASK_SFT: u32 = 0x1 << 21;
pub const WRITE_EN10_S_SFT: u32 = 20;
pub const WRITE_EN10_S_MASK: u32 = 0x1;
pub const WRITE_EN10_S_MASK_SFT: u32 = 0x1 << 20;
pub const READ_EN9_S_SFT: u32 = 19;
pub const READ_EN9_S_MASK: u32 = 0x1;
pub const READ_EN9_S_MASK_SFT: u32 = 0x1 << 19;
pub const WRITE_EN9_S_SFT: u32 = 18;
pub const WRITE_EN9_S_MASK: u32 = 0x1;
pub const WRITE_EN9_S_MASK_SFT: u32 = 0x1 << 18;
pub const READ_EN8_S_SFT: u32 = 17;
pub const READ_EN8_S_MASK: u32 = 0x1;
pub const READ_EN8_S_MASK_SFT: u32 = 0x1 << 17;
pub const WRITE_EN8_S_SFT: u32 = 16;
pub const WRITE_EN8_S_MASK: u32 = 0x1;
pub const WRITE_EN8_S_MASK_SFT: u32 = 0x1 << 16;
pub const READ_EN7_S_SFT: u32 = 15;
pub const READ_EN7_S_MASK: u32 = 0x1;
pub const READ_EN7_S_MASK_SFT: u32 = 0x1 << 15;
pub const WRITE_EN7_S_SFT: u32 = 14;
pub const WRITE_EN7_S_MASK: u32 = 0x1;
pub const WRITE_EN7_S_MASK_SFT: u32 = 0x1 << 14;
pub const READ_EN6_S_SFT: u32 = 13;
pub const READ_EN6_S_MASK: u32 = 0x1;
pub const READ_EN6_S_MASK_SFT: u32 = 0x1 << 13;
pub const WRITE_EN6_S_SFT: u32 = 12;
pub const WRITE_EN6_S_MASK: u32 = 0x1;
pub const WRITE_EN6_S_MASK_SFT: u32 = 0x1 << 12;
pub const READ_EN5_S_SFT: u32 = 11;
pub const READ_EN5_S_MASK: u32 = 0x1;
pub const READ_EN5_S_MASK_SFT: u32 = 0x1 << 11;
pub const WRITE_EN5_S_SFT: u32 = 10;
pub const WRITE_EN5_S_MASK: u32 = 0x1;
pub const WRITE_EN5_S_MASK_SFT: u32 = 0x1 << 10;
pub const READ_EN4_S_SFT: u32 = 9;
pub const READ_EN4_S_MASK: u32 = 0x1;
pub const READ_EN4_S_MASK_SFT: u32 = 0x1 << 9;
pub const WRITE_EN4_S_SFT: u32 = 8;
pub const WRITE_EN4_S_MASK: u32 = 0x1;
pub const WRITE_EN4_S_MASK_SFT: u32 = 0x1 << 8;
pub const READ_EN3_S_SFT: u32 = 7;
pub const READ_EN3_S_MASK: u32 = 0x1;
pub const READ_EN3_S_MASK_SFT: u32 = 0x1 << 7;
pub const WRITE_EN3_S_SFT: u32 = 6;
pub const WRITE_EN3_S_MASK: u32 = 0x1;
pub const WRITE_EN3_S_MASK_SFT: u32 = 0x1 << 6;
pub const READ_EN2_S_SFT: u32 = 5;
pub const READ_EN2_S_MASK: u32 = 0x1;
pub const READ_EN2_S_MASK_SFT: u32 = 0x1 << 5;
pub const WRITE_EN2_S_SFT: u32 = 4;
pub const WRITE_EN2_S_MASK: u32 = 0x1;
pub const WRITE_EN2_S_MASK_SFT: u32 = 0x1 << 4;
pub const READ_EN1_S_SFT: u32 = 3;
pub const READ_EN1_S_MASK: u32 = 0x1;
pub const READ_EN1_S_MASK_SFT: u32 = 0x1 << 3;
pub const WRITE_EN1_S_SFT: u32 = 2;
pub const WRITE_EN1_S_MASK: u32 = 0x1;
pub const WRITE_EN1_S_MASK_SFT: u32 = 0x1 << 2;
pub const READ_EN0_S_SFT: u32 = 1;
pub const READ_EN0_S_MASK: u32 = 0x1;
pub const READ_EN0_S_MASK_SFT: u32 = 0x1 << 1;
pub const WRITE_EN0_S_SFT: u32 = 0;
pub const WRITE_EN0_S_MASK: u32 = 0x1;
pub const WRITE_EN0_S_MASK_SFT: u32 = 0x1 << 0;

/* AFE_SE_SECURE_CON0 */
pub const AFE_HDMI_SE_SECURE_BIT_SFT: u32 = 11;
pub const AFE_HDMI_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_HDMI_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 11;
pub const AFE_SPDIF2_OUT_SE_SECURE_BIT_SFT: u32 = 10;
pub const AFE_SPDIF2_OUT_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_SPDIF2_OUT_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 10;
pub const AFE_SPDIF_OUT_SE_SECURE_BIT_SFT: u32 = 9;
pub const AFE_SPDIF_OUT_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_SPDIF_OUT_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 9;
pub const AFE_DL8_SE_SECURE_BIT_SFT: u32 = 8;
pub const AFE_DL8_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL8_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 8;
pub const AFE_DL7_SE_SECURE_BIT_SFT: u32 = 7;
pub const AFE_DL7_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL7_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 7;
pub const AFE_DL6_SE_SECURE_BIT_SFT: u32 = 6;
pub const AFE_DL6_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL6_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 6;
pub const AFE_DL5_SE_SECURE_BIT_SFT: u32 = 5;
pub const AFE_DL5_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL5_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 5;
pub const AFE_DL4_SE_SECURE_BIT_SFT: u32 = 4;
pub const AFE_DL4_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL4_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 4;
pub const AFE_DL3_SE_SECURE_BIT_SFT: u32 = 3;
pub const AFE_DL3_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL3_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 3;
pub const AFE_DL2_SE_SECURE_BIT_SFT: u32 = 2;
pub const AFE_DL2_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL2_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 2;
pub const AFE_DL1_SE_SECURE_BIT_SFT: u32 = 1;
pub const AFE_DL1_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL1_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 1;
pub const AFE_DL0_SE_SECURE_BIT_SFT: u32 = 0;
pub const AFE_DL0_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL0_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 0;

/* AFE_SE_SECURE_CON1 */
pub const AFE_DL46_SE_SECURE_BIT_SFT: u32 = 26;
pub const AFE_DL46_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL46_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 26;
pub const AFE_DL45_SE_SECURE_BIT_SFT: u32 = 25;
pub const AFE_DL45_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL45_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 25;
pub const AFE_DL44_SE_SECURE_BIT_SFT: u32 = 24;
pub const AFE_DL44_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL44_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 24;
pub const AFE_DL43_SE_SECURE_BIT_SFT: u32 = 23;
pub const AFE_DL43_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL43_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 23;
pub const AFE_DL42_SE_SECURE_BIT_SFT: u32 = 22;
pub const AFE_DL42_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL42_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 22;
pub const AFE_DL41_SE_SECURE_BIT_SFT: u32 = 21;
pub const AFE_DL41_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL41_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 21;
pub const AFE_DL40_SE_SECURE_BIT_SFT: u32 = 20;
pub const AFE_DL40_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL40_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 20;
pub const AFE_DL39_SE_SECURE_BIT_SFT: u32 = 19;
pub const AFE_DL39_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL39_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 19;
pub const AFE_DL38_SE_SECURE_BIT_SFT: u32 = 18;
pub const AFE_DL38_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL38_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 18;
pub const AFE_DL37_SE_SECURE_BIT_SFT: u32 = 17;
pub const AFE_DL37_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL37_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 17;
pub const AFE_DL36_SE_SECURE_BIT_SFT: u32 = 16;
pub const AFE_DL36_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL36_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 16;
pub const AFE_DL35_SE_SECURE_BIT_SFT: u32 = 15;
pub const AFE_DL35_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL35_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 15;
pub const AFE_DL34_SE_SECURE_BIT_SFT: u32 = 14;
pub const AFE_DL34_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL34_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 14;
pub const AFE_DL33_SE_SECURE_BIT_SFT: u32 = 13;
pub const AFE_DL33_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL33_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 13;
pub const AFE_DL32_SE_SECURE_BIT_SFT: u32 = 12;
pub const AFE_DL32_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL32_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 12;
pub const AFE_DL31_SE_SECURE_BIT_SFT: u32 = 11;
pub const AFE_DL31_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL31_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 11;
pub const AFE_DL30_SE_SECURE_BIT_SFT: u32 = 10;
pub const AFE_DL30_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL30_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 10;
pub const AFE_DL29_SE_SECURE_BIT_SFT: u32 = 9;
pub const AFE_DL29_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL29_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 9;
pub const AFE_DL28_SE_SECURE_BIT_SFT: u32 = 8;
pub const AFE_DL28_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL28_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 8;
pub const AFE_DL27_SE_SECURE_BIT_SFT: u32 = 7;
pub const AFE_DL27_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL27_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 7;
pub const AFE_DL26_SE_SECURE_BIT_SFT: u32 = 6;
pub const AFE_DL26_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL26_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 6;
pub const AFE_DL25_SE_SECURE_BIT_SFT: u32 = 5;
pub const AFE_DL25_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL25_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 5;
pub const AFE_DL24_SE_SECURE_BIT_SFT: u32 = 4;
pub const AFE_DL24_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL24_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 4;
pub const AFE_DL23_SE_SECURE_BIT_SFT: u32 = 3;
pub const AFE_DL23_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL23_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 3;
pub const AFE_DL_48CH_SE_SECURE_BIT_SFT: u32 = 2;
pub const AFE_DL_48CH_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL_48CH_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 2;
pub const AFE_DL_24CH_SE_SECURE_BIT_SFT: u32 = 1;
pub const AFE_DL_24CH_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL_24CH_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 1;
pub const AFE_DL_4CH_SE_SECURE_BIT_SFT: u32 = 0;
pub const AFE_DL_4CH_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_DL_4CH_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 0;

/* AFE_SE_SECURE_CON2 */
pub const AFE_VUL38_SE_SECURE_BIT_SFT: u32 = 28;
pub const AFE_VUL38_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL38_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 28;
pub const AFE_VUL37_SE_SECURE_BIT_SFT: u32 = 27;
pub const AFE_VUL37_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL37_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 27;
pub const AFE_VUL36_SE_SECURE_BIT_SFT: u32 = 26;
pub const AFE_VUL36_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL36_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 26;
pub const AFE_VUL35_SE_SECURE_BIT_SFT: u32 = 25;
pub const AFE_VUL35_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL35_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 25;
pub const AFE_VUL34_SE_SECURE_BIT_SFT: u32 = 24;
pub const AFE_VUL34_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL34_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 24;
pub const AFE_VUL33_SE_SECURE_BIT_SFT: u32 = 23;
pub const AFE_VUL33_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL33_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 23;
pub const AFE_VUL32_SE_SECURE_BIT_SFT: u32 = 22;
pub const AFE_VUL32_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL32_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 22;
pub const AFE_VUL31_SE_SECURE_BIT_SFT: u32 = 21;
pub const AFE_VUL31_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL31_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 21;
pub const AFE_VUL30_SE_SECURE_BIT_SFT: u32 = 20;
pub const AFE_VUL30_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL30_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 20;
pub const AFE_VUL29_SE_SECURE_BIT_SFT: u32 = 19;
pub const AFE_VUL29_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL29_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 19;
pub const AFE_VUL28_SE_SECURE_BIT_SFT: u32 = 18;
pub const AFE_VUL28_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL28_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 18;
pub const AFE_VUL27_SE_SECURE_BIT_SFT: u32 = 17;
pub const AFE_VUL27_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL27_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 17;
pub const AFE_VUL26_SE_SECURE_BIT_SFT: u32 = 16;
pub const AFE_VUL26_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL26_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 16;
pub const AFE_VUL25_SE_SECURE_BIT_SFT: u32 = 15;
pub const AFE_VUL25_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL25_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 15;
pub const AFE_VUL24_SE_SECURE_BIT_SFT: u32 = 14;
pub const AFE_VUL24_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL24_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 14;
pub const AFE_VUL_CM2_SE_SECURE_BIT_SFT: u32 = 13;
pub const AFE_VUL_CM2_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL_CM2_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 13;
pub const AFE_VUL_CM1_SE_SECURE_BIT_SFT: u32 = 12;
pub const AFE_VUL_CM1_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL_CM1_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 12;
pub const AFE_VUL_CM0_SE_SECURE_BIT_SFT: u32 = 11;
pub const AFE_VUL_CM0_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL_CM0_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 11;
pub const AFE_VUL10_SE_SECURE_BIT_SFT: u32 = 10;
pub const AFE_VUL10_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL10_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 10;
pub const AFE_VUL9_SE_SECURE_BIT_SFT: u32 = 9;
pub const AFE_VUL9_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL9_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 9;
pub const AFE_VUL8_SE_SECURE_BIT_SFT: u32 = 8;
pub const AFE_VUL8_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL8_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 8;
pub const AFE_VUL7_SE_SECURE_BIT_SFT: u32 = 7;
pub const AFE_VUL7_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL7_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 7;
pub const AFE_VUL6_SE_SECURE_BIT_SFT: u32 = 6;
pub const AFE_VUL6_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL6_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 6;
pub const AFE_VUL5_SE_SECURE_BIT_SFT: u32 = 5;
pub const AFE_VUL5_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL5_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 5;
pub const AFE_VUL4_SE_SECURE_BIT_SFT: u32 = 4;
pub const AFE_VUL4_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL4_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 4;
pub const AFE_VUL3_SE_SECURE_BIT_SFT: u32 = 3;
pub const AFE_VUL3_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL3_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 3;
pub const AFE_VUL2_SE_SECURE_BIT_SFT: u32 = 2;
pub const AFE_VUL2_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL2_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 2;
pub const AFE_VUL1_SE_SECURE_BIT_SFT: u32 = 1;
pub const AFE_VUL1_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL1_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 1;
pub const AFE_VUL0_SE_SECURE_BIT_SFT: u32 = 0;
pub const AFE_VUL0_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_VUL0_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 0;

/* AFE_SE_SECURE_CON3 */
pub const AFE_SPDIFIN_SE_SECURE_BIT_SFT: u32 = 10;
pub const AFE_SPDIFIN_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_SPDIFIN_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 10;
pub const AFE_TDM_IN_SE_SECURE_BIT_SFT: u32 = 9;
pub const AFE_TDM_IN_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_TDM_IN_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 9;
pub const AFE_MPHONE_EARC_SE_SECURE_BIT_SFT: u32 = 8;
pub const AFE_MPHONE_EARC_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_MPHONE_EARC_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 8;
pub const AFE_MPHONE_SPDIF_SE_SECURE_BIT_SFT: u32 = 7;
pub const AFE_MPHONE_SPDIF_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_MPHONE_SPDIF_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 7;
pub const AFE_ETDM_IN1_SE_SECURE_BIT_SFT: u32 = 1;
pub const AFE_ETDM_IN1_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_ETDM_IN1_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 1;
pub const AFE_ETDM_IN0_SE_SECURE_BIT_SFT: u32 = 0;
pub const AFE_ETDM_IN0_SE_SECURE_BIT_MASK: u32 = 0x1;
pub const AFE_ETDM_IN0_SE_SECURE_BIT_MASK_SFT: u32 = 0x1 << 0;

/* AFE_SE_PROT_SIDEBAND0 */
pub const HDMI_HPROT_SFT: u32 = 11;
pub const HDMI_HPROT_MASK: u32 = 0x1;
pub const HDMI_HPROT_MASK_SFT: u32 = 0x1 << 11;
pub const SPDIF2_OUT_HPROT_SFT: u32 = 10;
pub const SPDIF2_OUT_HPROT_MASK: u32 = 0x1;
pub const SPDIF2_OUT_HPROT_MASK_SFT: u32 = 0x1 << 10;
pub const SPDIF_OUT_HPROT_SFT: u32 = 9;
pub const SPDIF_OUT_HPROT_MASK: u32 = 0x1;
pub const SPDIF_OUT_HPROT_MASK_SFT: u32 = 0x1 << 9;
pub const DL8_HPROT_SFT: u32 = 8;
pub const DL8_HPROT_MASK: u32 = 0x1;
pub const DL8_HPROT_MASK_SFT: u32 = 0x1 << 8;
pub const DL7_HPROT_SFT: u32 = 7;
pub const DL7_HPROT_MASK: u32 = 0x1;
pub const DL7_HPROT_MASK_SFT: u32 = 0x1 << 7;
pub const DL6_HPROT_SFT: u32 = 6;
pub const DL6_HPROT_MASK: u32 = 0x1;
pub const DL6_HPROT_MASK_SFT: u32 = 0x1 << 6;
pub const DL5_HPROT_SFT: u32 = 5;
pub const DL5_HPROT_MASK: u32 = 0x1;
pub const DL5_HPROT_MASK_SFT: u32 = 0x1 << 5;
pub const DL4_HPROT_SFT: u32 = 4;
pub const DL4_HPROT_MASK: u32 = 0x1;
pub const DL4_HPROT_MASK_SFT: u32 = 0x1 << 4;
pub const DL3_HPROT_SFT: u32 = 3;
pub const DL3_HPROT_MASK: u32 = 0x1;
pub const DL3_HPROT_MASK_SFT: u32 = 0x1 << 3;
pub const DL2_HPROT_SFT: u32 = 2;
pub const DL2_HPROT_MASK: u32 = 0x1;
pub const DL2_HPROT_MASK_SFT: u32 = 0x1 << 2;
pub const DL1_HPROT_SFT: u32 = 1;
pub const DL1_HPROT_MASK: u32 = 0x1;
pub const DL1_HPROT_MASK_SFT: u32 = 0x1 << 1;
pub const DL0_HPROT_SFT: u32 = 0;
pub const DL0_HPROT_MASK: u32 = 0x1;
pub const DL0_HPROT_MASK_SFT: u32 = 0x1 << 0;

/* AFE_SE_PROT_SIDEBAND1 */
pub const DL46_HPROT_SFT: u32 = 26;
pub const DL46_HPROT_MASK: u32 = 0x1;
pub const DL46_HPROT_MASK_SFT: u32 = 0x1 << 26;
pub const DL45_HPROT_SFT: u32 = 25;
pub const DL45_HPROT_MASK: u32 = 0x1;
pub const DL45_HPROT_MASK_SFT: u32 = 0x1 << 25;
pub const DL44_HPROT_SFT: u32 = 24;
pub const DL44_HPROT_MASK: u32 = 0x1;
pub const DL44_HPROT_MASK_SFT: u32 = 0x1 << 24;
pub const DL43_HPROT_SFT: u32 = 23;
pub const DL43_HPROT_MASK: u32 = 0x1;
pub const DL43_HPROT_MASK_SFT: u32 = 0x1 << 23;
pub const DL42_HPROT_SFT: u32 = 22;
pub const DL42_HPROT_MASK: u32 = 0x1;
pub const DL42_HPROT_MASK_SFT: u32 = 0x1 << 22;
pub const DL41_HPROT_SFT: u32 = 21;
pub const DL41_HPROT_MASK: u32 = 0x1;
pub const DL41_HPROT_MASK_SFT: u32 = 0x1 << 21;
pub const DL40_HPROT_SFT: u32 = 20;
pub const DL40_HPROT_MASK: u32 = 0x1;
pub const DL40_HPROT_MASK_SFT: u32 = 0x1 << 20;
pub const DL39_HPROT_SFT: u32 = 19;
pub const DL39_HPROT_MASK: u32 = 0x1;
pub const DL39_HPROT_MASK_SFT: u32 = 0x1 << 19;
pub const DL38_HPROT_SFT: u32 = 18;
pub const DL38_HPROT_MASK: u32 = 0x1;
pub const DL38_HPROT_MASK_SFT: u32 = 0x1 << 18;
pub const DL37_HPROT_SFT: u32 = 17;
pub const DL37_HPROT_MASK: u32 = 0x1;
pub const DL37_HPROT_MASK_SFT: u32 = 0x1 << 17;
pub const DL36_HPROT_SFT: u32 = 16;
pub const DL36_HPROT_MASK: u32 = 0x1;
pub const DL36_HPROT_MASK_SFT: u32 = 0x1 << 16;
pub const DL35_HPROT_SFT: u32 = 15;
pub const DL35_HPROT_MASK: u32 = 0x1;
pub const DL35_HPROT_MASK_SFT: u32 = 0x1 << 15;
pub const DL34_HPROT_SFT: u32 = 14;
pub const DL34_HPROT_MASK: u32 = 0x1;
pub const DL34_HPROT_MASK_SFT: u32 = 0x1 << 14;
pub const DL33_HPROT_SFT: u32 = 13;
pub const DL33_HPROT_MASK: u32 = 0x1;
pub const DL33_HPROT_MASK_SFT: u32 = 0x1 << 13;
pub const DL32_HPROT_SFT: u32 = 12;
pub const DL32_HPROT_MASK: u32 = 0x1;
pub const DL32_HPROT_MASK_SFT: u32 = 0x1 << 12;
pub const DL31_HPROT_SFT: u32 = 11;
pub const DL31_HPROT_MASK: u32 = 0x1;
pub const DL31_HPROT_MASK_SFT: u32 = 0x1 << 11;
pub const DL30_HPROT_SFT: u32 = 10;
pub const DL30_HPROT_MASK: u32 = 0x1;
pub const DL30_HPROT_MASK_SFT: u32 = 0x1 << 10;
pub const DL29_HPROT_SFT: u32 = 9;
pub const DL29_HPROT_MASK: u32 = 0x1;
pub const DL29_HPROT_MASK_SFT: u32 = 0x1 << 9;
pub const DL28_HPROT_SFT: u32 = 8;
pub const DL28_HPROT_MASK: u32 = 0x1;
pub const DL28_HPROT_MASK_SFT: u32 = 0x1 << 8;
pub const DL27_HPROT_SFT: u32 = 7;
pub const DL27_HPROT_MASK: u32 = 0x1;
pub const DL27_HPROT_MASK_SFT: u32 = 0x1 << 7;
pub const DL26_HPROT_SFT: u32 = 6;
pub const DL26_HPROT_MASK: u32 = 0x1;
pub const DL26_HPROT_MASK_SFT: u32 = 0x1 << 6;
pub const DL25_HPROT_SFT: u32 = 5;
pub const DL25_HPROT_MASK: u32 = 0x1;
pub const DL25_HPROT_MASK_SFT: u32 = 0x1 << 5;
pub const DL24_HPROT_SFT: u32 = 4;
pub const DL24_HPROT_MASK: u32 = 0x1;
pub const DL24_HPROT_MASK_SFT: u32 = 0x1 << 4;
pub const DL23_HPROT_SFT: u32 = 3;
pub const DL23_HPROT_MASK: u32 = 0x1;
pub const DL23_HPROT_MASK_SFT: u32 = 0x1 << 3;
pub const DL_48CH_PROT_SFT: u32 = 2;
pub const DL_48CH_PROT_MASK: u32 = 0x1;
pub const DL_48CH_PROT_MASK_SFT: u32 = 0x1 << 2;
pub const DL_24CH_PROT_SFT: u32 = 1;
pub const DL_24CH_PROT_MASK: u32 = 0x1;
pub const DL_24CH_PROT_MASK_SFT: u32 = 0x1 << 1;
pub const DL_4CH_PROT_SFT: u32 = 0;
pub const DL_4CH_PROT_MASK: u32 = 0x1;
pub const DL_4CH_PROT_MASK_SFT: u32 = 0x1 << 0;

/* AFE_SE_PROT_SIDEBAND2 */
pub const VUL38_HPROT_SFT: u32 = 28;
pub const VUL38_HPROT_MASK: u32 = 0x1;
pub const VUL38_HPROT_MASK_SFT: u32 = 0x1 << 28;
pub const VUL37_HPROT_SFT: u32 = 27;
pub const VUL37_HPROT_MASK: u32 = 0x1;
pub const VUL37_HPROT_MASK_SFT: u32 = 0x1 << 27;
pub const VUL36_HPROT_SFT: u32 = 26;
pub const VUL36_HPROT_MASK: u32 = 0x1;
pub const VUL36_HPROT_MASK_SFT: u32 = 0x1 << 26;
pub const VUL35_HPROT_SFT: u32 = 25;
pub const VUL35_HPROT_MASK: u32 = 0x1;
pub const VUL35_HPROT_MASK_SFT: u32 = 0x1 << 25;
pub const VUL34_HPROT_SFT: u32 = 24;
pub const VUL34_HPROT_MASK: u32 = 0x1;
pub const VUL34_HPROT_MASK_SFT: u32 = 0x1 << 24;
pub const VUL33_HPROT_SFT: u32 = 23;
pub const VUL33_HPROT_MASK: u32 = 0x1;
pub const VUL33_HPROT_MASK_SFT: u32 = 0x1 << 23;
pub const VUL32_HPROT_SFT: u32 = 22;
pub const VUL32_HPROT_MASK: u32 = 0x1;
pub const VUL32_HPROT_MASK_SFT: u32 = 0x1 << 22;
pub const VUL31_HPROT_SFT: u32 = 21;
pub const VUL31_HPROT_MASK: u32 = 0x1;
pub const VUL31_HPROT_MASK_SFT: u32 = 0x1 << 21;
pub const VUL30_HPROT_SFT: u32 = 20;
pub const VUL30_HPROT_MASK: u32 = 0x1;
pub const VUL30_HPROT_MASK_SFT: u32 = 0x1 << 20;
pub const VUL29_HPROT_SFT: u32 = 19;
pub const VUL29_HPROT_MASK: u32 = 0x1;
pub const VUL29_HPROT_MASK_SFT: u32 = 0x1 << 19;
pub const VUL28_HPROT_SFT: u32 = 18;
pub const VUL28_HPROT_MASK: u32 = 0x1;
pub const VUL28_HPROT_MASK_SFT: u32 = 0x1 << 18;
pub const VUL27_HPROT_SFT: u32 = 17;
pub const VUL27_HPROT_MASK: u32 = 0x1;
pub const VUL27_HPROT_MASK_SFT: u32 = 0x1 << 17;
pub const VUL26_HPROT_SFT: u32 = 16;
pub const VUL26_HPROT_MASK: u32 = 0x1;
pub const VUL26_HPROT_MASK_SFT: u32 = 0x1 << 16;
pub const VUL25_HPROT_SFT: u32 = 15;
pub const VUL25_HPROT_MASK: u32 = 0x1;
pub const VUL25_HPROT_MASK_SFT: u32 = 0x1 << 15;
pub const VUL24_HPROT_SFT: u32 = 14;
pub const VUL24_HPROT_MASK: u32 = 0x1;
pub const VUL24_HPROT_MASK_SFT: u32 = 0x1 << 14;
pub const VUL_CM2_HPROT_SFT: u32 = 13;
pub const VUL_CM2_HPROT_MASK: u32 = 0x1;
pub const VUL_CM2_HPROT_MASK_SFT: u32 = 0x1 << 13;
pub const VUL_CM1_HPROT_SFT: u32 = 12;
pub const VUL_CM1_HPROT_MASK: u32 = 0x1;
pub const VUL_CM1_HPROT_MASK_SFT: u32 = 0x1 << 12;
pub const VUL_CM0_HPROT_SFT: u32 = 11;
pub const VUL_CM0_HPROT_MASK: u32 = 0x1;
pub const VUL_CM0_HPROT_MASK_SFT: u32 = 0x1 << 11;
pub const VUL10_HPROT_SFT: u32 = 10;
pub const VUL10_HPROT_MASK: u32 = 0x1;
pub const VUL10_HPROT_MASK_SFT: u32 = 0x1 << 10;
pub const VUL9_HPROT_SFT: u32 = 9;
pub const VUL9_HPROT_MASK: u32 = 0x1;
pub const VUL9_HPROT_MASK_SFT: u32 = 0x1 << 9;
pub const VUL8_HPROT_SFT: u32 = 8;
pub const VUL8_HPROT_MASK: u32 = 0x1;
pub const VUL8_HPROT_MASK_SFT: u32 = 0x1 << 8;
pub const VUL7_HPROT_SFT: u32 = 7;
pub const VUL7_HPROT_MASK: u32 = 0x1;
pub const VUL7_HPROT_MASK_SFT: u32 = 0x1 << 7;
pub const VUL6_HPROT_SFT: u32 = 6;
pub const VUL6_HPROT_MASK: u32 = 0x1;
pub const VUL6_HPROT_MASK_SFT: u32 = 0x1 << 6;
pub const VUL5_HPROT_SFT: u32 = 5;
pub const VUL5_HPROT_MASK: u32 = 0x1;
pub const VUL5_HPROT_MASK_SFT: u32 = 0x1 << 5;
pub const VUL4_HPROT_SFT: u32 = 4;
pub const VUL4_HPROT_MASK: u32 = 0x1;
pub const VUL4_HPROT_MASK_SFT: u32 = 0x1 << 4;
pub const VUL3_HPROT_SFT: u32 = 3;
pub const VUL3_HPROT_MASK: u32 = 0x1;
pub const VUL3_HPROT_MASK_SFT: u32 = 0x1 << 3;
pub const VUL2_HPROT_SFT: u32 = 2;
pub const VUL2_HPROT_MASK: u32 = 0x1;
pub const VUL2_HPROT_MASK_SFT: u32 = 0x1 << 2;
pub const VUL1_HPROT_SFT: u32 = 1;
pub const VUL1_HPROT_MASK: u32 = 0x1;
pub const VUL1_HPROT_MASK_SFT: u32 = 0x1 << 1;
pub const VUL0_HPROT_SFT: u32 = 0;
pub const VUL0_HPROT_MASK: u32 = 0x1;
pub const VUL0_HPROT_MASK_SFT: u32 = 0x1 << 0;

/* AFE_SE_PROT_SIDEBAND3 */
pub const MPHONE_EARC_HPROT_SFT: u32 = 10;
pub const MPHONE_EARC_HPROT_MASK: u32 = 0x1;
pub const MPHONE_EARC_HPROT_MASK_SFT: u32 = 0x1 << 10;
pub const MPHONE_SPDIF_HPROT_SFT: u32 = 9;
pub const MPHONE_SPDIF_HPROT_MASK: u32 = 0x1;
pub const MPHONE_SPDIF_HPROT_MASK_SFT: u32 = 0x1 << 9;
pub const SPDIFIN_HPROT_SFT: u32 = 8;
pub const SPDIFIN_HPROT_MASK: u32 = 0x1;
pub const SPDIFIN_HPROT_MASK_SFT: u32 = 0x1 << 8;
pub const TDMIN_HPROT_SFT: u32 = 7;
pub const TDMIN_HPROT_MASK: u32 = 0x1;
pub const TDMIN_HPROT_MASK_SFT: u32 = 0x1 << 7;
pub const ETDM_IN1_HPROT_SFT: u32 = 1;
pub const ETDM_IN1_HPROT_MASK: u32 = 0x1;
pub const ETDM_IN1_HPROT_MASK_SFT: u32 = 0x1 << 1;
pub const ETDM_IN0_HPROT_SFT: u32 = 0;
pub const ETDM_IN0_HPROT_MASK: u32 = 0x1;
pub const ETDM_IN0_HPROT_MASK_SFT: u32 = 0x1 << 0;

/* AFE_SE_DOMAIN_SIDEBAND0 */
pub const DL7_HDOMAIN_SFT: u32 = 28;
pub const DL7_HDOMAIN_MASK: u32 = 0xf;
pub const DL7_HDOMAIN_MASK_SFT: u32 = 0xf << 28;
pub const DL6_HDOMAIN_SFT: u32 = 24;
pub const DL6_HDOMAIN_MASK: u32 = 0xf;
pub const DL6_HDOMAIN_MASK_SFT: u32 = 0xf << 24;
pub const DL5_HDOMAIN_SFT: u32 = 20;
pub const DL5_HDOMAIN_MASK: u32 = 0xf;
pub const DL5_HDOMAIN_MASK_SFT: u32 = 0xf << 20;
pub const DL4_HDOMAIN_SFT: u32 = 16;
pub const DL4_HDOMAIN_MASK: u32 = 0xf;
pub const DL4_HDOMAIN_MASK_SFT: u32 = 0xf << 16;
pub const DL3_HDOMAIN_SFT: u32 = 12;
pub const DL3_HDOMAIN_MASK: u32 = 0xf;
pub const DL3_HDOMAIN_MASK_SFT: u32 = 0xf << 12;
pub const DL2_HDOMAIN_SFT: u32 = 8;
pub const DL2_HDOMAIN_MASK: u32 = 0xf;
pub const DL2_HDOMAIN_MASK_SFT: u32 = 0xf << 8;
pub const DL1_HDOMAIN_SFT: u32 = 4;
pub const DL1_HDOMAIN_MASK: u32 = 0xf;
pub const DL1_HDOMAIN_MASK_SFT: u32 = 0xf << 4;
pub const DL0_HDOMAIN_SFT: u32 = 0;
pub const DL0_HDOMAIN_MASK: u32 = 0xf;
pub const DL0_HDOMAIN_MASK_SFT: u32 = 0xf << 0;

/* AFE_SE_DOMAIN_SIDEBAND1 */
pub const DL_48CH_HDOMAIN_SFT: u32 = 24;
pub const DL_48CH_HDOMAIN_MASK: u32 = 0xf;
pub const DL_48CH_HDOMAIN_MASK_SFT: u32 = 0xf << 24;
pub const DL_24CH_HDOMAIN_SFT: u32 = 20;
pub const DL_24CH_HDOMAIN_MASK: u32 = 0xf;
pub const DL_24CH_HDOMAIN_MASK_SFT: u32 = 0xf << 20;
pub const DL_4CH_HDOMAIN_SFT: u32 = 16;
pub const DL_4CH_HDOMAIN_MASK: u32 = 0xf;
pub const DL_4CH_HDOMAIN_MASK_SFT: u32 = 0xf << 16;
pub const HDMI_HDOMAIN_SFT: u32 = 12;
pub const HDMI_HDOMAIN_MASK: u32 = 0xf;
pub const HDMI_HDOMAIN_MASK_SFT: u32 = 0xf << 12;
pub const SPDIF2_OUT_HDOMAIN_SFT: u32 = 8;
pub const SPDIF2_OUT_HDOMAIN_MASK: u32 = 0xf;
pub const SPDIF2_OUT_HDOMAIN_MASK_SFT: u32 = 0xf << 8;
pub const SPDIF_OUT_HDOMAIN_SFT: u32 = 4;
pub const SPDIF_OUT_HDOMAIN_MASK: u32 = 0xf;
pub const SPDIF_OUT_HDOMAIN_MASK_SFT: u32 = 0xf << 4;
pub const DL8_HDOMAIN_SFT: u32 = 0;
pub const DL8_HDOMAIN_MASK: u32 = 0xf;
pub const DL8_HDOMAIN_MASK_SFT: u32 = 0xf << 0;

/* AFE_SE_DOMAIN_SIDEBAND2 */
pub const DL30_HDOMAIN_SFT: u32 = 28;
pub const DL30_HDOMAIN_MASK: u32 = 0xf;
pub const DL30_HDOMAIN_MASK_SFT: u32 = 0xf << 28;
pub const DL29_HDOMAIN_SFT: u32 = 24;
pub const DL29_HDOMAIN_MASK: u32 = 0xf;
pub const DL29_HDOMAIN_MASK_SFT: u32 = 0xf << 24;
pub const DL28_HDOMAIN_SFT: u32 = 20;
pub const DL28_HDOMAIN_MASK: u32 = 0xf;
pub const DL28_HDOMAIN_MASK_SFT: u32 = 0xf << 20;
pub const DL27_HDOMAIN_SFT: u32 = 16;
pub const DL27_HDOMAIN_MASK: u32 = 0xf;
pub const DL27_HDOMAIN_MASK_SFT: u32 = 0xf << 16;
pub const DL26_HDOMAIN_SFT: u32 = 12;
pub const DL26_HDOMAIN_MASK: u32 = 0xf;
pub const DL26_HDOMAIN_MASK_SFT: u32 = 0xf << 12;
pub const DL25_HDOMAIN_SFT: u32 = 8;
pub const DL25_HDOMAIN_MASK: u32 = 0xf;
pub const DL25_HDOMAIN_MASK_SFT: u32 = 0xf << 8;
pub const DL24_HDOMAIN_SFT: u32 = 4;
pub const DL24_HDOMAIN_MASK: u32 = 0xf;
pub const DL24_HDOMAIN_MASK_SFT: u32 = 0xf << 4;
pub const DL23_HDOMAIN_SFT: u32 = 0;
pub const DL23_HDOMAIN_MASK: u32 = 0xf;
pub const DL23_HDOMAIN_MASK_SFT: u32 = 0xf << 0;

/* AFE_SE_DOMAIN_SIDEBAND3 */
pub const DL38_HDOMAIN_SFT: u32 = 28;
pub const DL38_HDOMAIN_MASK: u32 = 0xf;
pub const DL38_HDOMAIN_MASK_SFT: u32 = 0xf << 28;
pub const DL37_HDOMAIN_SFT: u32 = 24;
pub const DL37_HDOMAIN_MASK: u32 = 0xf;
pub const DL37_HDOMAIN_MASK_SFT: u32 = 0xf << 24;
pub const DL36_HDOMAIN_SFT: u32 = 20;
pub const DL36_HDOMAIN_MASK: u32 = 0xf;
pub const DL36_HDOMAIN_MASK_SFT: u32 = 0xf << 20;
pub const DL35_HDOMAIN_SFT: u32 = 16;
pub const DL35_HDOMAIN_MASK: u32 = 0xf;
pub const DL35_HDOMAIN_MASK_SFT: u32 = 0xf << 16;
pub const DL34_HDOMAIN_SFT: u32 = 12;
pub const DL34_HDOMAIN_MASK: u32 = 0xf;
pub const DL34_HDOMAIN_MASK_SFT: u32 = 0xf << 12;
pub const DL33_HDOMAIN_SFT: u32 = 8;
pub const DL33_HDOMAIN_MASK: u32 = 0xf;
pub const DL33_HDOMAIN_MASK_SFT: u32 = 0xf << 8;
pub const DL32_HDOMAIN_SFT: u32 = 4;
pub const DL32_HDOMAIN_MASK: u32 = 0xf;
pub const DL32_HDOMAIN_MASK_SFT: u32 = 0xf << 4;
pub const DL31_HDOMAIN_SFT: u32 = 0;
pub const DL31_HDOMAIN_MASK: u32 = 0xf;
pub const DL31_HDOMAIN_MASK_SFT: u32 = 0xf << 0;

/* AFE_SE_DOMAIN_SIDEBAND4 */
pub const DL46_HDOMAIN_SFT: u32 = 28;
pub const DL46_HDOMAIN_MASK: u32 = 0xf;
pub const DL46_HDOMAIN_MASK_SFT: u32 = 0xf << 28;
pub const DL45_HDOMAIN_SFT: u32 = 24;
pub const DL45_HDOMAIN_MASK: u32 = 0xf;
pub const DL45_HDOMAIN_MASK_SFT: u32 = 0xf << 24;
pub const DL44_HDOMAIN_SFT: u32 = 20;
pub const DL44_HDOMAIN_MASK: u32 = 0xf;
pub const DL44_HDOMAIN_MASK_SFT: u32 = 0xf << 20;
pub const DL43_HDOMAIN_SFT: u32 = 16;
pub const DL43_HDOMAIN_MASK: u32 = 0xf;
pub const DL43_HDOMAIN_MASK_SFT: u32 = 0xf << 16;
pub const DL42_HDOMAIN_SFT: u32 = 12;
pub const DL42_HDOMAIN_MASK: u32 = 0xf;
pub const DL42_HDOMAIN_MASK_SFT: u32 = 0xf << 12;
pub const DL41_HDOMAIN_SFT: u32 = 8;
pub const DL41_HDOMAIN_MASK: u32 = 0xf;
pub const DL41_HDOMAIN_MASK_SFT: u32 = 0xf << 8;
pub const DL40_HDOMAIN_SFT: u32 = 4;
pub const DL40_HDOMAIN_MASK: u32 = 0xf;
pub const DL40_HDOMAIN_MASK_SFT: u32 = 0xf << 4;
pub const DL39_HDOMAIN_SFT: u32 = 0;
pub const DL39_HDOMAIN_MASK: u32 = 0xf;
pub const DL39_HDOMAIN_MASK_SFT: u32 = 0xf << 0;

/* AFE_SE_DOMAIN_SIDEBAND5 */
pub const VUL7_HDOMAIN_SFT: u32 = 28;
pub const VUL7_HDOMAIN_MASK: u32 = 0xf;
pub const VUL7_HDOMAIN_MASK_SFT: u32 = 0xf << 28;
pub const VUL6_HDOMAIN_SFT: u32 = 24;
pub const VUL6_HDOMAIN_MASK: u32 = 0xf;
pub const VUL6_HDOMAIN_MASK_SFT: u32 = 0xf << 24;
pub const VUL5_HDOMAIN_SFT: u32 = 20;
pub const VUL5_HDOMAIN_MASK: u32 = 0xf;
pub const VUL5_HDOMAIN_MASK_SFT: u32 = 0xf << 20;
pub const VUL4_HDOMAIN_SFT: u32 = 16;
pub const VUL4_HDOMAIN_MASK: u32 = 0xf;
pub const VUL4_HDOMAIN_MASK_SFT: u32 = 0xf << 16;
pub const VUL3_HDOMAIN_SFT: u32 = 12;
pub const VUL3_HDOMAIN_MASK: u32 = 0xf;
pub const VUL3_HDOMAIN_MASK_SFT: u32 = 0xf << 12;
pub const VUL2_HDOMAIN_SFT: u32 = 8;
pub const VUL2_HDOMAIN_MASK: u32 = 0xf;
pub const VUL2_HDOMAIN_MASK_SFT: u32 = 0xf << 8;
pub const VUL1_HDOMAIN_SFT: u32 = 4;
pub const VUL1_HDOMAIN_MASK: u32 = 0xf;
pub const VUL1_HDOMAIN_MASK_SFT: u32 = 0xf << 4;
pub const VUL0_HDOMAIN_SFT: u32 = 0;
pub const VUL0_HDOMAIN_MASK: u32 = 0xf;
pub const VUL0_HDOMAIN_MASK_SFT: u32 = 0xf << 0;

/* AFE_SE_DOMAIN_SIDEBAND6 */
pub const VU25_HDOMAIN_SFT: u32 = 28;
pub const VU25_HDOMAIN_MASK: u32 = 0xf;
pub const VU25_HDOMAIN_MASK_SFT: u32 = 0xf << 28;
pub const VUL24_HDOMAIN_SFT: u32 = 24;
pub const VUL24_HDOMAIN_MASK: u32 = 0xf;
pub const VUL24_HDOMAIN_MASK_SFT: u32 = 0xf << 24;
pub const VUL_CM2_HDOMAIN_SFT: u32 = 20;
pub const VUL_CM2_HDOMAIN_MASK: u32 = 0xf;
pub const VUL_CM2_HDOMAIN_MASK_SFT: u32 = 0xf << 20;
pub const VUL_CM1_HDOMAIN_SFT: u32 = 16;
pub const VUL_CM1_HDOMAIN_MASK: u32 = 0xf;
pub const VUL_CM1_HDOMAIN_MASK_SFT: u32 = 0xf << 16;
pub const VUL_CM0_HDOMAIN_SFT: u32 = 12;
pub const VUL_CM0_HDOMAIN_MASK: u32 = 0xf;
pub const VUL_CM0_HDOMAIN_MASK_SFT: u32 = 0xf << 12;
pub const VUL10_HDOMAIN_SFT: u32 = 8;
pub const VUL10_HDOMAIN_MASK: u32 = 0xf;
pub const VUL10_HDOMAIN_MASK_SFT: u32 = 0xf << 8;
pub const VUL9_HDOMAIN_SFT: u32 = 4;
pub const VUL9_HDOMAIN_MASK: u32 = 0xf;
pub const VUL9_HDOMAIN_MASK_SFT: u32 = 0xf << 4;
pub const VUL8_HDOMAIN_SFT: u32 = 0;
pub const VUL8_HDOMAIN_MASK: u32 = 0xf;
pub const VUL8_HDOMAIN_MASK_SFT: u32 = 0xf << 0;

/* AFE_SE_DOMAIN_SIDEBAND7 */
pub const VUL33_HDOMAIN_SFT: u32 = 28;
pub const VUL33_HDOMAIN_MASK: u32 = 0xf;
pub const VUL33_HDOMAIN_MASK_SFT: u32 = 0xf << 28;
pub const VUL32_HDOMAIN_SFT: u32 = 24;
pub const VUL32_HDOMAIN_MASK: u32 = 0xf;
pub const VUL32_HDOMAIN_MASK_SFT: u32 = 0xf << 24;
pub const VUL31_HDOMAIN_SFT: u32 = 20;
pub const VUL31_HDOMAIN_MASK: u32 = 0xf;
pub const VUL31_HDOMAIN_MASK_SFT: u32 = 0xf << 20;
pub const VUL30_HDOMAIN_SFT: u32 = 16;
pub const VUL30_HDOMAIN_MASK: u32 = 0xf;
pub const VUL30_HDOMAIN_MASK_SFT: u32 = 0xf << 16;
pub const VUL29_HDOMAIN_SFT: u32 = 12;
pub const VUL29_HDOMAIN_MASK: u32 = 0xf;
pub const VUL29_HDOMAIN_MASK_SFT: u32 = 0xf << 12;
pub const VUL28_HDOMAIN_SFT: u32 = 8;
pub const VUL28_HDOMAIN_MASK: u32 = 0xf;
pub const VUL28_HDOMAIN_MASK_SFT: u32 = 0xf << 8;
pub const VUL27_HDOMAIN_SFT: u32 = 4;
pub const VUL27_HDOMAIN_MASK: u32 = 0xf;
pub const VUL27_HDOMAIN_MASK_SFT: u32 = 0xf << 4;
pub const VUL26_HDOMAIN_SFT: u32 = 0;
pub const VUL26_HDOMAIN_MASK: u32 = 0xf;
pub const VUL26_HDOMAIN_MASK_SFT: u32 = 0xf << 0;

/* AFE_SE_DOMAIN_SIDEBAND8 */
pub const ETDM_IN1_HDOMAIN_SFT: u32 = 24;
pub const ETDM_IN1_HDOMAIN_MASK: u32 = 0xf;
pub const ETDM_IN1_HDOMAIN_MASK_SFT: u32 = 0xf << 24;
pub const ETDM_IN0_HDOMAIN_SFT: u32 = 20;
pub const ETDM_IN0_HDOMAIN_MASK: u32 = 0xf;
pub const ETDM_IN0_HDOMAIN_MASK_SFT: u32 = 0xf << 20;
pub const VUL38_HDOMAIN_SFT: u32 = 16;
pub const VUL38_HDOMAIN_MASK: u32 = 0xf;
pub const VUL38_HDOMAIN_MASK_SFT: u32 = 0xf << 16;
pub const VUL37_HDOMAIN_SFT: u32 = 12;
pub const VUL37_HDOMAIN_MASK: u32 = 0xf;
pub const VUL37_HDOMAIN_MASK_SFT: u32 = 0xf << 12;
pub const VUL36_HDOMAIN_SFT: u32 = 8;
pub const VUL36_HDOMAIN_MASK: u32 = 0xf;
pub const VUL36_HDOMAIN_MASK_SFT: u32 = 0xf << 8;
pub const VUL35_HDOMAIN_SFT: u32 = 4;
pub const VUL35_HDOMAIN_MASK: u32 = 0xf;
pub const VUL35_HDOMAIN_MASK_SFT: u32 = 0xf << 4;
pub const VUL34_HDOMAIN_SFT: u32 = 0;
pub const VUL34_HDOMAIN_MASK: u32 = 0xf;
pub const VUL34_HDOMAIN_MASK_SFT: u32 = 0xf << 0;

/* AFE_SE_DOMAIN_SIDEBAND9 */
pub const MPHONE_EARC_HDOMAIN_SFT: u32 = 28;
pub const MPHONE_EARC_HDOMAIN_MASK: u32 = 0xf;
pub const MPHONE_EARC_HDOMAIN_MASK_SFT: u32 = 0xf << 28;
pub const MPHONE_SPDIF_HDOMAIN_SFT: u32 = 24;
pub const MPHONE_SPDIF_HDOMAIN_MASK: u32 = 0xf;
pub const MPHONE_SPDIF_HDOMAIN_MASK_SFT: u32 = 0xf << 24;
pub const SPDIFIN_HDOMAIN_SFT: u32 = 20;
pub const SPDIFIN_HDOMAIN_MASK: u32 = 0xf;
pub const SPDIFIN_HDOMAIN_MASK_SFT: u32 = 0xf << 20;
pub const TDMIN_HDOMAIN_SFT: u32 = 16;
pub const TDMIN_HDOMAIN_MASK: u32 = 0xf;
pub const TDMIN_HDOMAIN_MASK_SFT: u32 = 0xf << 16;

/* AFE_PROT_SIDEBAND0_MON */
pub const AFE_DOMAIN_SIDEBAN0_MON_SFT: u32 = 0;
pub const AFE_DOMAIN_SIDEBAN0_MON_MASK: u32 = 0xffffffff;
pub const AFE_DOMAIN_SIDEBAN0_MON_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_PROT_SIDEBAND1_MON */
pub const AFE_DOMAIN_SIDEBAN1_MON_SFT: u32 = 0;
pub const AFE_DOMAIN_SIDEBAN1_MON_MASK: u32 = 0xffffffff;
pub const AFE_DOMAIN_SIDEBAN1_MON_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_PROT_SIDEBAND2_MON */
pub const AFE_DOMAIN_SIDEBAN2_MON_SFT: u32 = 0;
pub const AFE_DOMAIN_SIDEBAN2_MON_MASK: u32 = 0xffffffff;
pub const AFE_DOMAIN_SIDEBAN2_MON_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_PROT_SIDEBAND3_MON */
pub const AFE_DOMAIN_SIDEBAN3_MON_SFT: u32 = 0;
pub const AFE_DOMAIN_SIDEBAN3_MON_MASK: u32 = 0xffffffff;
pub const AFE_DOMAIN_SIDEBAN3_MON_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_DOMAIN_SIDEBAND0_MON */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN0_MON_SFT = 0 */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN0_MON_MASK = 0xffffffff */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN0_MON_MASK_SFT = 0xffffffff << 0 */

/* AFE_DOMAIN_SIDEBAND1_MON */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN1_MON_SFT = 0 */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN1_MON_MASK = 0xffffffff */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN1_MON_MASK_SFT = 0xffffffff << 0 */

/* AFE_DOMAIN_SIDEBAND2_MON */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN2_MON_SFT = 0 */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN2_MON_MASK = 0xffffffff */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN2_MON_MASK_SFT = 0xffffffff << 0 */

/* AFE_DOMAIN_SIDEBAND3_MON */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN3_MON_SFT = 0 */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN3_MON_MASK = 0xffffffff */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN3_MON_MASK_SFT = 0xffffffff << 0 */

/* AFE_DOMAIN_SIDEBAND4_MON */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN0_MON_SFT = 0 */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN0_MON_MASK = 0xffffffff */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN0_MON_MASK_SFT = 0xffffffff << 0 */

/* AFE_DOMAIN_SIDEBAND5_MON */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN1_MON_SFT = 0 */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN1_MON_MASK = 0xffffffff */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN1_MON_MASK_SFT = 0xffffffff << 0 */

/* AFE_DOMAIN_SIDEBAND6_MON */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN2_MON_SFT = 0 */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN2_MON_MASK = 0xffffffff */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN2_MON_MASK_SFT = 0xffffffff << 0 */

/* AFE_DOMAIN_SIDEBAND7_MON */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN3_MON_SFT = 0 */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN3_MON_MASK = 0xffffffff */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN3_MON_MASK_SFT = 0xffffffff << 0 */

/* AFE_DOMAIN_SIDEBAND8_MON */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN2_MON_SFT = 0 */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN2_MON_MASK = 0xffffffff */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN2_MON_MASK_SFT = 0xffffffff << 0 */

/* AFE_DOMAIN_SIDEBAND9_MON */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN3_MON_SFT = 0 */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN3_MON_MASK = 0xffffffff */
/* duplicate C macro: AFE_DOMAIN_SIDEBAN3_MON_MASK_SFT = 0xffffffff << 0 */

/* AFE_SECURE_CONN0 */
pub const AFE_SPDIFIN_LPBK_CON_MASK_S_SFT: u32 = 26;
pub const AFE_SPDIFIN_LPBK_CON_MASK_S_MASK: u32 = 0x3;
pub const AFE_SPDIFIN_LPBK_CON_MASK_S_MASK_SFT: u32 = 0x3 << 26;
pub const AFE_ADDA_DMIC1_SRC_CON0_MASK_S_SFT: u32 = 25;
pub const AFE_ADDA_DMIC1_SRC_CON0_MASK_S_MASK: u32 = 0x1;
pub const AFE_ADDA_DMIC1_SRC_CON0_MASK_S_MASK_SFT: u32 = 0x1 << 25;
pub const AFE_ADDA_DMIC0_SRC_CON0_MASK_S_SFT: u32 = 24;
pub const AFE_ADDA_DMIC0_SRC_CON0_MASK_S_MASK: u32 = 0x1;
pub const AFE_ADDA_DMIC0_SRC_CON0_MASK_S_MASK_SFT: u32 = 0x1 << 24;
pub const AFE_ADDA_UL3_SRC_CON0_MASK_S_SFT: u32 = 23;
pub const AFE_ADDA_UL3_SRC_CON0_MASK_S_MASK: u32 = 0x1;
pub const AFE_ADDA_UL3_SRC_CON0_MASK_S_MASK_SFT: u32 = 0x1 << 23;
pub const AFE_ADDA_UL2_SRC_CON0_MASK_S_SFT: u32 = 22;
pub const AFE_ADDA_UL2_SRC_CON0_MASK_S_MASK: u32 = 0x1;
pub const AFE_ADDA_UL2_SRC_CON0_MASK_S_MASK_SFT: u32 = 0x1 << 22;
pub const AFE_ADDA_UL1_SRC_CON0_MASK_S_SFT: u32 = 21;
pub const AFE_ADDA_UL1_SRC_CON0_MASK_S_MASK: u32 = 0x1;
pub const AFE_ADDA_UL1_SRC_CON0_MASK_S_MASK_SFT: u32 = 0x1 << 21;
pub const AFE_ADDA_UL0_SRC_CON0_MASK_S_SFT: u32 = 20;
pub const AFE_ADDA_UL0_SRC_CON0_MASK_S_MASK: u32 = 0x1;
pub const AFE_ADDA_UL0_SRC_CON0_MASK_S_MASK_SFT: u32 = 0x1 << 20;
pub const AFE_MRKAIF1_CFG0_MASK_S_SFT: u32 = 19;
pub const AFE_MRKAIF1_CFG0_MASK_S_MASK: u32 = 0x1;
pub const AFE_MRKAIF1_CFG0_MASK_S_MASK_SFT: u32 = 0x1 << 19;
pub const AFE_MRKAIF0_CFG0_MASK_S_SFT: u32 = 18;
pub const AFE_MRKAIF0_CFG0_MASK_S_MASK: u32 = 0x1;
pub const AFE_MRKAIF0_CFG0_MASK_S_MASK_SFT: u32 = 0x1 << 18;
pub const AFE_TDMIN_CON1_MASK_S_SFT: u32 = 17;
pub const AFE_TDMIN_CON1_MASK_S_MASK: u32 = 0x1;
pub const AFE_TDMIN_CON1_MASK_S_MASK_SFT: u32 = 0x1 << 17;
pub const AFE_TDM_CON2_MASK_S_SFT: u32 = 16;
pub const AFE_TDM_CON2_MASK_S_MASK: u32 = 0x1;
pub const AFE_TDM_CON2_MASK_S_MASK_SFT: u32 = 0x1 << 16;
pub const AFE_DAIBT_CON_MASK_S_SFT: u32 = 14;
pub const AFE_DAIBT_CON_MASK_S_MASK: u32 = 0x3;
pub const AFE_DAIBT_CON_MASK_S_MASK_SFT: u32 = 0x3 << 14;
pub const AFE_MRGIF_CON_MASK_S_SFT: u32 = 12;
pub const AFE_MRGIF_CON_MASK_S_MASK: u32 = 0x3;
pub const AFE_MRGIF_CON_MASK_S_MASK_SFT: u32 = 0x3 << 12;
pub const AFE_CONNSYS_I2S_CON_MASK_S_SFT: u32 = 11;
pub const AFE_CONNSYS_I2S_CON_MASK_S_MASK: u32 = 0x1;
pub const AFE_CONNSYS_I2S_CON_MASK_S_MASK_SFT: u32 = 0x1 << 11;
pub const AFE_PCM1_INFT_CON0_MASK_S_SFT: u32 = 6;
pub const AFE_PCM1_INFT_CON0_MASK_S_MASK: u32 = 0x1f;
pub const AFE_PCM1_INFT_CON0_MASK_S_MASK_SFT: u32 = 0x1f << 6;
pub const AFE_PCM0_INTF_CON1_MASK_S_SFT: u32 = 0;
pub const AFE_PCM0_INTF_CON1_MASK_S_MASK: u32 = 0x3f;
pub const AFE_PCM0_INTF_CON1_MASK_S_MASK_SFT: u32 = 0x3f << 0;

/* AFE_SECURE_CONN_ETDM1 */
pub const ETDM1_4_7_COWORK_CON1_MASK_S_0_SFT: u32 = 24;
pub const ETDM1_4_7_COWORK_CON1_MASK_S_0_MASK: u32 = 0xff;
pub const ETDM1_4_7_COWORK_CON1_MASK_S_0_MASK_SFT: u32 = 0xff << 24;
pub const ETDM1_4_7_COWORK_CON0_MASK_S_0_SFT: u32 = 20;
pub const ETDM1_4_7_COWORK_CON0_MASK_S_0_MASK: u32 = 0xf;
pub const ETDM1_4_7_COWORK_CON0_MASK_S_0_MASK_SFT: u32 = 0xf << 20;
pub const ETDM1_4_7_COWORK_CON0_MASK_S_1_SFT: u32 = 16;
pub const ETDM1_4_7_COWORK_CON0_MASK_S_1_MASK: u32 = 0xf;
pub const ETDM1_4_7_COWORK_CON0_MASK_S_1_MASK_SFT: u32 = 0xf << 16;
pub const ETDM1_0_3_COWORK_CON3_MASK_S_0_SFT: u32 = 8;
pub const ETDM1_0_3_COWORK_CON3_MASK_S_0_MASK: u32 = 0xff;
pub const ETDM1_0_3_COWORK_CON3_MASK_S_0_MASK_SFT: u32 = 0xff << 8;
pub const ETDM1_0_3_COWORK_CON3_MASK_S_1_SFT: u32 = 0;
pub const ETDM1_0_3_COWORK_CON3_MASK_S_1_MASK: u32 = 0xff;
pub const ETDM1_0_3_COWORK_CON3_MASK_S_1_MASK_SFT: u32 = 0xff << 0;

/* AFE_SECURE_CONN_ETDM2 */
pub const ETDM2_4_7_COWORK_CON3_MASK_S_0_SFT: u32 = 24;
pub const ETDM2_4_7_COWORK_CON3_MASK_S_0_MASK: u32 = 0xff;
pub const ETDM2_4_7_COWORK_CON3_MASK_S_0_MASK_SFT: u32 = 0xff << 24;
pub const ETDM2_4_7_COWORK_CON3_MASK_S_1_SFT: u32 = 16;
pub const ETDM2_4_7_COWORK_CON3_MASK_S_1_MASK: u32 = 0xff;
pub const ETDM2_4_7_COWORK_CON3_MASK_S_1_MASK_SFT: u32 = 0xff << 16;
pub const ETDM2_4_7_COWORK_CON2_MASK_S_0_SFT: u32 = 12;
pub const ETDM2_4_7_COWORK_CON2_MASK_S_0_MASK: u32 = 0xf;
pub const ETDM2_4_7_COWORK_CON2_MASK_S_0_MASK_SFT: u32 = 0xf << 12;
pub const ETDM2_4_7_COWORK_CON2_MASK_S_1_SFT: u32 = 8;
pub const ETDM2_4_7_COWORK_CON2_MASK_S_1_MASK: u32 = 0xf;
pub const ETDM2_4_7_COWORK_CON2_MASK_S_1_MASK_SFT: u32 = 0xf << 8;
pub const ETDM2_4_7_COWORK_CON1_MASK_S_0_SFT: u32 = 0;
pub const ETDM2_4_7_COWORK_CON1_MASK_S_0_MASK: u32 = 0xff;
pub const ETDM2_4_7_COWORK_CON1_MASK_S_0_MASK_SFT: u32 = 0xff << 0;

/* AFE_SECURE_SRAM_CON0 */
pub const SRAM_READ_EN15_NS_SFT: u32 = 31;
pub const SRAM_READ_EN15_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN15_NS_MASK_SFT: u32 = 0x1 << 31;
pub const SRAM_WRITE_EN15_NS_SFT: u32 = 30;
pub const SRAM_WRITE_EN15_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN15_NS_MASK_SFT: u32 = 0x1 << 30;
pub const SRAM_READ_EN14_NS_SFT: u32 = 29;
pub const SRAM_READ_EN14_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN14_NS_MASK_SFT: u32 = 0x1 << 29;
pub const SRAM_WRITE_EN14_NS_SFT: u32 = 28;
pub const SRAM_WRITE_EN14_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN14_NS_MASK_SFT: u32 = 0x1 << 28;
pub const SRAM_READ_EN13_NS_SFT: u32 = 27;
pub const SRAM_READ_EN13_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN13_NS_MASK_SFT: u32 = 0x1 << 27;
pub const SRAM_WRITE_EN13_NS_SFT: u32 = 26;
pub const SRAM_WRITE_EN13_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN13_NS_MASK_SFT: u32 = 0x1 << 26;
pub const SRAM_READ_EN12_NS_SFT: u32 = 25;
pub const SRAM_READ_EN12_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN12_NS_MASK_SFT: u32 = 0x1 << 25;
pub const SRAM_WRITE_EN12_NS_SFT: u32 = 24;
pub const SRAM_WRITE_EN12_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN12_NS_MASK_SFT: u32 = 0x1 << 24;
pub const SRAM_READ_EN11_NS_SFT: u32 = 23;
pub const SRAM_READ_EN11_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN11_NS_MASK_SFT: u32 = 0x1 << 23;
pub const SRAM_WRITE_EN11_NS_SFT: u32 = 22;
pub const SRAM_WRITE_EN11_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN11_NS_MASK_SFT: u32 = 0x1 << 22;
pub const SRAM_READ_EN10_NS_SFT: u32 = 21;
pub const SRAM_READ_EN10_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN10_NS_MASK_SFT: u32 = 0x1 << 21;
pub const SRAM_WRITE_EN10_NS_SFT: u32 = 20;
pub const SRAM_WRITE_EN10_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN10_NS_MASK_SFT: u32 = 0x1 << 20;
pub const SRAM_READ_EN9_NS_SFT: u32 = 19;
pub const SRAM_READ_EN9_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN9_NS_MASK_SFT: u32 = 0x1 << 19;
pub const SRAM_WRITE_EN9_NS_SFT: u32 = 18;
pub const SRAM_WRITE_EN9_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN9_NS_MASK_SFT: u32 = 0x1 << 18;
pub const SRAM_READ_EN8_NS_SFT: u32 = 17;
pub const SRAM_READ_EN8_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN8_NS_MASK_SFT: u32 = 0x1 << 17;
pub const SRAM_WRITE_EN8_NS_SFT: u32 = 16;
pub const SRAM_WRITE_EN8_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN8_NS_MASK_SFT: u32 = 0x1 << 16;
pub const SRAM_READ_EN7_NS_SFT: u32 = 15;
pub const SRAM_READ_EN7_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN7_NS_MASK_SFT: u32 = 0x1 << 15;
pub const SRAM_WRITE_EN7_NS_SFT: u32 = 14;
pub const SRAM_WRITE_EN7_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN7_NS_MASK_SFT: u32 = 0x1 << 14;
pub const SRAM_READ_EN6_NS_SFT: u32 = 13;
pub const SRAM_READ_EN6_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN6_NS_MASK_SFT: u32 = 0x1 << 13;
pub const SRAM_WRITE_EN6_NS_SFT: u32 = 12;
pub const SRAM_WRITE_EN6_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN6_NS_MASK_SFT: u32 = 0x1 << 12;
pub const SRAM_READ_EN5_NS_SFT: u32 = 11;
pub const SRAM_READ_EN5_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN5_NS_MASK_SFT: u32 = 0x1 << 11;
pub const SRAM_WRITE_EN5_NS_SFT: u32 = 10;
pub const SRAM_WRITE_EN5_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN5_NS_MASK_SFT: u32 = 0x1 << 10;
pub const SRAM_READ_EN4_NS_SFT: u32 = 9;
pub const SRAM_READ_EN4_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN4_NS_MASK_SFT: u32 = 0x1 << 9;
pub const SRAM_WRITE_EN4_NS_SFT: u32 = 8;
pub const SRAM_WRITE_EN4_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN4_NS_MASK_SFT: u32 = 0x1 << 8;
pub const SRAM_READ_EN3_NS_SFT: u32 = 7;
pub const SRAM_READ_EN3_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN3_NS_MASK_SFT: u32 = 0x1 << 7;
pub const SRAM_WRITE_EN3_NS_SFT: u32 = 6;
pub const SRAM_WRITE_EN3_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN3_NS_MASK_SFT: u32 = 0x1 << 6;
pub const SRAM_READ_EN2_NS_SFT: u32 = 5;
pub const SRAM_READ_EN2_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN2_NS_MASK_SFT: u32 = 0x1 << 5;
pub const SRAM_WRITE_EN2_NS_SFT: u32 = 4;
pub const SRAM_WRITE_EN2_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN2_NS_MASK_SFT: u32 = 0x1 << 4;
pub const SRAM_READ_EN1_NS_SFT: u32 = 3;
pub const SRAM_READ_EN1_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN1_NS_MASK_SFT: u32 = 0x1 << 3;
pub const SRAM_WRITE_EN1_NS_SFT: u32 = 2;
pub const SRAM_WRITE_EN1_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN1_NS_MASK_SFT: u32 = 0x1 << 2;
pub const SRAM_READ_EN0_NS_SFT: u32 = 1;
pub const SRAM_READ_EN0_NS_MASK: u32 = 0x1;
pub const SRAM_READ_EN0_NS_MASK_SFT: u32 = 0x1 << 1;
pub const SRAM_WRITE_EN0_NS_SFT: u32 = 0;
pub const SRAM_WRITE_EN0_NS_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN0_NS_MASK_SFT: u32 = 0x1 << 0;

/* AFE_SECURE_SRAM_CON1 */
pub const SRAM_READ_EN15_S_SFT: u32 = 31;
pub const SRAM_READ_EN15_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN15_S_MASK_SFT: u32 = 0x1 << 31;
pub const SRAM_WRITE_EN15_S_SFT: u32 = 30;
pub const SRAM_WRITE_EN15_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN15_S_MASK_SFT: u32 = 0x1 << 30;
pub const SRAM_READ_EN14_S_SFT: u32 = 29;
pub const SRAM_READ_EN14_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN14_S_MASK_SFT: u32 = 0x1 << 29;
pub const SRAM_WRITE_EN14_S_SFT: u32 = 28;
pub const SRAM_WRITE_EN14_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN14_S_MASK_SFT: u32 = 0x1 << 28;
pub const SRAM_READ_EN13_S_SFT: u32 = 27;
pub const SRAM_READ_EN13_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN13_S_MASK_SFT: u32 = 0x1 << 27;
pub const SRAM_WRITE_EN13_S_SFT: u32 = 26;
pub const SRAM_WRITE_EN13_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN13_S_MASK_SFT: u32 = 0x1 << 26;
pub const SRAM_READ_EN12_S_SFT: u32 = 25;
pub const SRAM_READ_EN12_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN12_S_MASK_SFT: u32 = 0x1 << 25;
pub const SRAM_WRITE_EN12_S_SFT: u32 = 24;
pub const SRAM_WRITE_EN12_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN12_S_MASK_SFT: u32 = 0x1 << 24;
pub const SRAM_READ_EN11_S_SFT: u32 = 23;
pub const SRAM_READ_EN11_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN11_S_MASK_SFT: u32 = 0x1 << 23;
pub const SRAM_WRITE_EN11_S_SFT: u32 = 22;
pub const SRAM_WRITE_EN11_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN11_S_MASK_SFT: u32 = 0x1 << 22;
pub const SRAM_READ_EN10_S_SFT: u32 = 21;
pub const SRAM_READ_EN10_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN10_S_MASK_SFT: u32 = 0x1 << 21;
pub const SRAM_WRITE_EN10_S_SFT: u32 = 20;
pub const SRAM_WRITE_EN10_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN10_S_MASK_SFT: u32 = 0x1 << 20;
pub const SRAM_READ_EN9_S_SFT: u32 = 19;
pub const SRAM_READ_EN9_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN9_S_MASK_SFT: u32 = 0x1 << 19;
pub const SRAM_WRITE_EN9_S_SFT: u32 = 18;
pub const SRAM_WRITE_EN9_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN9_S_MASK_SFT: u32 = 0x1 << 18;
pub const SRAM_READ_EN8_S_SFT: u32 = 17;
pub const SRAM_READ_EN8_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN8_S_MASK_SFT: u32 = 0x1 << 17;
pub const SRAM_WRITE_EN8_S_SFT: u32 = 16;
pub const SRAM_WRITE_EN8_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN8_S_MASK_SFT: u32 = 0x1 << 16;
pub const SRAM_READ_EN7_S_SFT: u32 = 15;
pub const SRAM_READ_EN7_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN7_S_MASK_SFT: u32 = 0x1 << 15;
pub const SRAM_WRITE_EN7_S_SFT: u32 = 14;
pub const SRAM_WRITE_EN7_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN7_S_MASK_SFT: u32 = 0x1 << 14;
pub const SRAM_READ_EN6_S_SFT: u32 = 13;
pub const SRAM_READ_EN6_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN6_S_MASK_SFT: u32 = 0x1 << 13;
pub const SRAM_WRITE_EN6_S_SFT: u32 = 12;
pub const SRAM_WRITE_EN6_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN6_S_MASK_SFT: u32 = 0x1 << 12;
pub const SRAM_READ_EN5_S_SFT: u32 = 11;
pub const SRAM_READ_EN5_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN5_S_MASK_SFT: u32 = 0x1 << 11;
pub const SRAM_WRITE_EN5_S_SFT: u32 = 10;
pub const SRAM_WRITE_EN5_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN5_S_MASK_SFT: u32 = 0x1 << 10;
pub const SRAM_READ_EN4_S_SFT: u32 = 9;
pub const SRAM_READ_EN4_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN4_S_MASK_SFT: u32 = 0x1 << 9;
pub const SRAM_WRITE_EN4_S_SFT: u32 = 8;
pub const SRAM_WRITE_EN4_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN4_S_MASK_SFT: u32 = 0x1 << 8;
pub const SRAM_READ_EN3_S_SFT: u32 = 7;
pub const SRAM_READ_EN3_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN3_S_MASK_SFT: u32 = 0x1 << 7;
pub const SRAM_WRITE_EN3_S_SFT: u32 = 6;
pub const SRAM_WRITE_EN3_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN3_S_MASK_SFT: u32 = 0x1 << 6;
pub const SRAM_READ_EN2_S_SFT: u32 = 5;
pub const SRAM_READ_EN2_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN2_S_MASK_SFT: u32 = 0x1 << 5;
pub const SRAM_WRITE_EN2_S_SFT: u32 = 4;
pub const SRAM_WRITE_EN2_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN2_S_MASK_SFT: u32 = 0x1 << 4;
pub const SRAM_READ_EN1_S_SFT: u32 = 3;
pub const SRAM_READ_EN1_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN1_S_MASK_SFT: u32 = 0x1 << 3;
pub const SRAM_WRITE_EN1_S_SFT: u32 = 2;
pub const SRAM_WRITE_EN1_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN1_S_MASK_SFT: u32 = 0x1 << 2;
pub const SRAM_READ_EN0_S_SFT: u32 = 1;
pub const SRAM_READ_EN0_S_MASK: u32 = 0x1;
pub const SRAM_READ_EN0_S_MASK_SFT: u32 = 0x1 << 1;
pub const SRAM_WRITE_EN0_S_SFT: u32 = 0;
pub const SRAM_WRITE_EN0_S_MASK: u32 = 0x1;
pub const SRAM_WRITE_EN0_S_MASK_SFT: u32 = 0x1 << 0;

/* AFE_SE_CONN_INPUT_MASK0 */
pub const SECURE_INTRCONN_I0_I31_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_I0_I31_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_I0_I31_S_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_SE_CONN_INPUT_MASK1 */
pub const SECURE_INTRCONN_I32_I63_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_I32_I63_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_I32_I63_S_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_SE_CONN_INPUT_MASK2 */
pub const SECURE_INTRCONN_I64_I95_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_I64_I95_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_I64_I95_S_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_SE_CONN_INPUT_MASK3 */
pub const SECURE_INTRCONN_I96_I127_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_I96_I127_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_I96_I127_S_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_SE_CONN_INPUT_MASK4 */
pub const SECURE_INTRCONN_I128_I159_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_I128_I159_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_I128_I159_S_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_SE_CONN_INPUT_MASK5 */
pub const SECURE_INTRCONN_I160_I191_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_I160_I191_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_I160_I191_S_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_SE_CONN_INPUT_MASK6 */
pub const SECURE_INTRCONN_I192_I223_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_I192_I223_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_I192_I223_S_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_SE_CONN_INPUT_MASK7 */
pub const SECURE_INTRCONN_I224_I256_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_I224_I256_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_I224_I256_S_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_NON_SE_CONN_INPUT_MASK0 */
pub const NORMAL_INTRCONN_I0_I31_S_SFT: u32 = 0;
pub const NORMAL_INTRCONN_I0_I31_S_MASK: u32 = 0xffffffff;
pub const NORMAL_INTRCONN_I0_I31_S_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_NON_SE_CONN_INPUT_MASK1 */
pub const NORMAL_INTRCONN_I32_I63_S_SFT: u32 = 0;
pub const NORMAL_INTRCONN_I32_I63_S_MASK: u32 = 0xffffffff;
pub const NORMAL_INTRCONN_I32_I63_S_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_NON_SE_CONN_INPUT_MASK2 */
pub const NORMAL_INTRCONN_I64_I95_S_SFT: u32 = 0;
pub const NORMAL_INTRCONN_I64_I95_S_MASK: u32 = 0xffffffff;
pub const NORMAL_INTRCONN_I64_I95_S_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_NON_SE_CONN_INPUT_MASK3 */
pub const NORMAL_INTRCONN_I96_I127_S_SFT: u32 = 0;
pub const NORMAL_INTRCONN_I96_I127_S_MASK: u32 = 0xffffffff;
pub const NORMAL_INTRCONN_I96_I127_S_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_NON_SE_CONN_INPUT_MASK4 */
pub const NORMAL_INTRCONN_I128_I159_S_SFT: u32 = 0;
pub const NORMAL_INTRCONN_I128_I159_S_MASK: u32 = 0xffffffff;
pub const NORMAL_INTRCONN_I128_I159_S_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_NON_SE_CONN_INPUT_MASK5 */
pub const NORMAL_INTRCONN_I160_I191_S_SFT: u32 = 0;
pub const NORMAL_INTRCONN_I160_I191_S_MASK: u32 = 0xffffffff;
pub const NORMAL_INTRCONN_I160_I191_S_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_NON_SE_CONN_INPUT_MASK6 */
pub const NORMAL_INTRCONN_I192_I223_S_SFT: u32 = 0;
pub const NORMAL_INTRCONN_I192_I223_S_MASK: u32 = 0xffffffff;
pub const NORMAL_INTRCONN_I192_I223_S_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_NON_SE_CONN_INPUT_MASK7 */
pub const NORMAL_INTRCONN_I224_I256_S_SFT: u32 = 0;
pub const NORMAL_INTRCONN_I224_I256_S_MASK: u32 = 0xffffffff;
pub const NORMAL_INTRCONN_I224_I256_S_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_SE_CONN_OUTPUT_SEL0 */
pub const SECURE_INTRCONN_O0_O31_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_O0_O31_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_O0_O31_S_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_SE_CONN_OUTPUT_SEL1 */
pub const SECURE_INTRCONN_O32_O63_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_O32_O63_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_O32_O63_S_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_SE_CONN_OUTPUT_SEL2 */
pub const SECURE_INTRCONN_O64_O95_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_O64_O95_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_O64_O95_S_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_SE_CONN_OUTPUT_SEL3 */
pub const SECURE_INTRCONN_O96_O127_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_O96_O127_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_O96_O127_S_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_SE_CONN_OUTPUT_SEL4 */
pub const SECURE_INTRCONN_O128_O159_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_O128_O159_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_O128_O159_S_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_SE_CONN_OUTPUT_SEL5 */
pub const SECURE_INTRCONN_O160_O191_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_O160_O191_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_O160_O191_S_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_SE_CONN_OUTPUT_SEL6 */
pub const SECURE_INTRCONN_O192_O223_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_O192_O223_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_O192_O223_S_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_SE_CONN_OUTPUT_SEL7 */
pub const SECURE_INTRCONN_O224_O256_S_SFT: u32 = 0;
pub const SECURE_INTRCONN_O224_O256_S_MASK: u32 = 0xffffffff;
pub const SECURE_INTRCONN_O224_O256_S_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_PCM0_INTF_CON1_MASK_MON */
pub const AFE_PCM0_INTF_CON1_MASK_MON_SFT: u32 = 0;
pub const AFE_PCM0_INTF_CON1_MASK_MON_MASK: u32 = 0xffffffff;
pub const AFE_PCM0_INTF_CON1_MASK_MON_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_PCM0_INTF_CON0_MASK_MON */
pub const AFE_PCM0_INTF_CON0_MASK_MON_SFT: u32 = 0;
pub const AFE_PCM0_INTF_CON0_MASK_MON_MASK: u32 = 0xffffffff;
pub const AFE_PCM0_INTF_CON0_MASK_MON_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_CONNSYS_I2S_CON_MASK_MON */
pub const AFE_CONNSYS_I2S_CON_MASK_MON_SFT: u32 = 0;
pub const AFE_CONNSYS_I2S_CON_MASK_MON_MASK: u32 = 0xffffffff;
pub const AFE_CONNSYS_I2S_CON_MASK_MON_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_MTKAIF0_CFG0_MASK_MON */
pub const AFE_MTKAIF0_CFG0_MASK_MON_SFT: u32 = 0;
pub const AFE_MTKAIF0_CFG0_MASK_MON_MASK: u32 = 0xffffffff;
pub const AFE_MTKAIF0_CFG0_MASK_MON_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_MTKAIF1_CFG0_MASK_MON */
pub const AFE_MTKAIF1_CFG0_MASK_MON_SFT: u32 = 0;
pub const AFE_MTKAIF1_CFG0_MASK_MON_MASK: u32 = 0xffffffff;
pub const AFE_MTKAIF1_CFG0_MASK_MON_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_UL0_SRC_CON0_MASK_MON */
pub const AFE_ADDA_UL0_SRC_CON0_MASK_MON_SFT: u32 = 0;
pub const AFE_ADDA_UL0_SRC_CON0_MASK_MON_MASK: u32 = 0xffffffff;
pub const AFE_ADDA_UL0_SRC_CON0_MASK_MON_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_ADDA_UL1_SRC_CON0_MASK_MON */
pub const AFE_ADDA_UL1_SRC_CON0_MASK_MON_SFT: u32 = 0;
pub const AFE_ADDA_UL1_SRC_CON0_MASK_MON_MASK: u32 = 0xffffffff;
pub const AFE_ADDA_UL1_SRC_CON0_MASK_MON_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_GASRC0_NEW_CON0 */
pub const ONE_HEART_SFT: u32 = 31;
pub const ONE_HEART_MASK: u32 = 0x1;
pub const ONE_HEART_MASK_SFT: u32 = 0x1 << 31;
pub const CHSET0_OFS_ONE_HEART_DISABLE_SFT: u32 = 30;
pub const CHSET0_OFS_ONE_HEART_DISABLE_MASK: u32 = 0x1;
pub const CHSET0_OFS_ONE_HEART_DISABLE_MASK_SFT: u32 = 0x1 << 30;
pub const USE_SHORT_DELAY_COEFF_SFT: u32 = 29;
pub const USE_SHORT_DELAY_COEFF_MASK: u32 = 0x1;
pub const USE_SHORT_DELAY_COEFF_MASK_SFT: u32 = 0x1 << 29;
pub const CHSET0_O16BIT_SFT: u32 = 19;
pub const CHSET0_O16BIT_MASK: u32 = 0x1;
pub const CHSET0_O16BIT_MASK_SFT: u32 = 0x1 << 19;
pub const CHSET0_CLR_IIR_HISTORY_SFT: u32 = 17;
pub const CHSET0_CLR_IIR_HISTORY_MASK: u32 = 0x1;
pub const CHSET0_CLR_IIR_HISTORY_MASK_SFT: u32 = 0x1 << 17;
pub const CHSET0_IS_MONO_SFT: u32 = 16;
pub const CHSET0_IS_MONO_MASK: u32 = 0x1;
pub const CHSET0_IS_MONO_MASK_SFT: u32 = 0x1 << 16;
pub const CHSET0_OFS_SEL_SFT: u32 = 14;
pub const CHSET0_OFS_SEL_MASK: u32 = 0x3;
pub const CHSET0_OFS_SEL_MASK_SFT: u32 = 0x3 << 14;
pub const CHSET0_IFS_SEL_SFT: u32 = 12;
pub const CHSET0_IFS_SEL_MASK: u32 = 0x3;
pub const CHSET0_IFS_SEL_MASK_SFT: u32 = 0x3 << 12;
pub const CHSET0_IIR_EN_SFT: u32 = 11;
pub const CHSET0_IIR_EN_MASK: u32 = 0x1;
pub const CHSET0_IIR_EN_MASK_SFT: u32 = 0x1 << 11;
pub const CHSET0_IIR_STAGE_SFT: u32 = 8;
pub const CHSET0_IIR_STAGE_MASK: u32 = 0x7;
pub const CHSET0_IIR_STAGE_MASK_SFT: u32 = 0x7 << 8;
pub const ASM_ON_MOD_SFT: u32 = 7;
pub const ASM_ON_MOD_MASK: u32 = 0x1;
pub const ASM_ON_MOD_MASK_SFT: u32 = 0x1 << 7;
pub const CHSET_STR_CLR_SFT: u32 = 4;
pub const CHSET_STR_CLR_MASK: u32 = 0x1;
pub const CHSET_STR_CLR_MASK_SFT: u32 = 0x1 << 4;
pub const CHSET_ON_SFT: u32 = 2;
pub const CHSET_ON_MASK: u32 = 0x1;
pub const CHSET_ON_MASK_SFT: u32 = 0x1 << 2;
pub const COEFF_SRAM_CTRL_SFT: u32 = 1;
pub const COEFF_SRAM_CTRL_MASK: u32 = 0x1;
pub const COEFF_SRAM_CTRL_MASK_SFT: u32 = 0x1 << 1;
pub const ASM_ON_SFT: u32 = 0;
pub const ASM_ON_MASK: u32 = 0x1;
pub const ASM_ON_MASK_SFT: u32 = 0x1 << 0;

/* AFE_GASRC0_NEW_CON1 */
pub const ASM_FREQ_0_SFT: u32 = 0;
pub const ASM_FREQ_0_MASK: u32 = 0xffffff;
pub const ASM_FREQ_0_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_GASRC0_NEW_CON2 */
pub const ASM_FREQ_1_SFT: u32 = 0;
pub const ASM_FREQ_1_MASK: u32 = 0xffffff;
pub const ASM_FREQ_1_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_GASRC0_NEW_CON3 */
pub const ASM_FREQ_2_SFT: u32 = 0;
pub const ASM_FREQ_2_MASK: u32 = 0xffffff;
pub const ASM_FREQ_2_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_GASRC0_NEW_CON4 */
pub const ASM_FREQ_3_SFT: u32 = 0;
pub const ASM_FREQ_3_MASK: u32 = 0xffffff;
pub const ASM_FREQ_3_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_GASRC0_NEW_CON5 */
pub const OUT_EN_SEL_DOMAIN_SFT: u32 = 29;
pub const OUT_EN_SEL_DOMAIN_MASK: u32 = 0x7;
pub const OUT_EN_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 29;
pub const OUT_EN_SEL_FS_SFT: u32 = 24;
pub const OUT_EN_SEL_FS_MASK: u32 = 0x1f;
pub const OUT_EN_SEL_FS_MASK_SFT: u32 = 0x1f << 24;
pub const IN_EN_SEL_DOMAIN_SFT: u32 = 21;
pub const IN_EN_SEL_DOMAIN_MASK: u32 = 0x7;
pub const IN_EN_SEL_DOMAIN_MASK_SFT: u32 = 0x7 << 21;
pub const IN_EN_SEL_FS_SFT: u32 = 16;
pub const IN_EN_SEL_FS_MASK: u32 = 0x1f;
pub const IN_EN_SEL_FS_MASK_SFT: u32 = 0x1f << 16;
pub const RESULT_SEL_SFT: u32 = 8;
pub const RESULT_SEL_MASK: u32 = 0x7;
pub const RESULT_SEL_MASK_SFT: u32 = 0x7 << 8;
pub const CALI_CK_SEL_SFT: u32 = 4;
pub const CALI_CK_SEL_MASK: u32 = 0x7;
pub const CALI_CK_SEL_MASK_SFT: u32 = 0x7 << 4;
pub const CALI_LRCK_SEL_SFT: u32 = 1;
pub const CALI_LRCK_SEL_MASK: u32 = 0x7;
pub const CALI_LRCK_SEL_MASK_SFT: u32 = 0x7 << 1;
pub const SOFT_RESET_SFT: u32 = 0;
pub const SOFT_RESET_MASK: u32 = 0x1;
pub const SOFT_RESET_MASK_SFT: u32 = 0x1 << 0;

/* AFE_GASRC0_NEW_CON6 */
pub const FREQ_CALI_CYCLE_SFT: u32 = 16;
pub const FREQ_CALI_CYCLE_MASK: u32 = 0xffff;
pub const FREQ_CALI_CYCLE_MASK_SFT: u32 = 0xffff << 16;
pub const FREQ_CALI_AUTORST_EN_SFT: u32 = 15;
pub const FREQ_CALI_AUTORST_EN_MASK: u32 = 0x1;
pub const FREQ_CALI_AUTORST_EN_MASK_SFT: u32 = 0x1 << 15;
pub const CALI_AUTORST_DETECT_SFT: u32 = 14;
pub const CALI_AUTORST_DETECT_MASK: u32 = 0x1;
pub const CALI_AUTORST_DETECT_MASK_SFT: u32 = 0x1 << 14;
pub const FREQ_CALC_RUNNING_SFT: u32 = 13;
pub const FREQ_CALC_RUNNING_MASK: u32 = 0x1;
pub const FREQ_CALC_RUNNING_MASK_SFT: u32 = 0x1 << 13;
pub const AUTO_TUNE_FREQ3_SFT: u32 = 12;
pub const AUTO_TUNE_FREQ3_MASK: u32 = 0x1;
pub const AUTO_TUNE_FREQ3_MASK_SFT: u32 = 0x1 << 12;
pub const COMP_FREQ_RES_EN_SFT: u32 = 11;
pub const COMP_FREQ_RES_EN_MASK: u32 = 0x1;
pub const COMP_FREQ_RES_EN_MASK_SFT: u32 = 0x1 << 11;
pub const FREQ_CALI_SEL_SFT: u32 = 8;
pub const FREQ_CALI_SEL_MASK: u32 = 0x3;
pub const FREQ_CALI_SEL_MASK_SFT: u32 = 0x3 << 8;
pub const FREQ_CALI_BP_DGL_SFT: u32 = 7;
pub const FREQ_CALI_BP_DGL_MASK: u32 = 0x1;
pub const FREQ_CALI_BP_DGL_MASK_SFT: u32 = 0x1 << 7;
pub const FREQ_CALI_MAX_GWIDTH_SFT: u32 = 4;
pub const FREQ_CALI_MAX_GWIDTH_MASK: u32 = 0x7;
pub const FREQ_CALI_MAX_GWIDTH_MASK_SFT: u32 = 0x7 << 4;
pub const AUTO_TUNE_FREQ2_SFT: u32 = 3;
pub const AUTO_TUNE_FREQ2_MASK: u32 = 0x1;
pub const AUTO_TUNE_FREQ2_MASK_SFT: u32 = 0x1 << 3;
pub const FREQ_CALI_AUTO_RESTART_SFT: u32 = 2;
pub const FREQ_CALI_AUTO_RESTART_MASK: u32 = 0x1;
pub const FREQ_CALI_AUTO_RESTART_MASK_SFT: u32 = 0x1 << 2;
pub const CALI_USE_FREQ_OUT_SFT: u32 = 1;
pub const CALI_USE_FREQ_OUT_MASK: u32 = 0x1;
pub const CALI_USE_FREQ_OUT_MASK_SFT: u32 = 0x1 << 1;
pub const CALI_EN_SFT: u32 = 0;
pub const CALI_EN_MASK: u32 = 0x1;
pub const CALI_EN_MASK_SFT: u32 = 0x1 << 0;

/* AFE_GASRC0_NEW_CON7 */
pub const FREQ_CALC_DENOMINATOR_SFT: u32 = 0;
pub const FREQ_CALC_DENOMINATOR_MASK: u32 = 0xffffff;
pub const FREQ_CALC_DENOMINATOR_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_GASRC0_NEW_CON8 */
pub const PRD_CALI_RESULT_RECORD_SFT: u32 = 0;
pub const PRD_CALI_RESULT_RECORD_MASK: u32 = 0xffffff;
pub const PRD_CALI_RESULT_RECORD_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_GASRC0_NEW_CON9 */
pub const FREQ_CALI_RESULT_SFT: u32 = 0;
pub const FREQ_CALI_RESULT_MASK: u32 = 0xffffff;
pub const FREQ_CALI_RESULT_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_GASRC0_NEW_CON10 */
pub const COEFF_SRAM_DATA_SFT: u32 = 0;
pub const COEFF_SRAM_DATA_MASK: u32 = 0xffffffff;
pub const COEFF_SRAM_DATA_MASK_SFT: u32 = 0xffffffff << 0;

/* AFE_GASRC0_NEW_CON11 */
pub const COEFF_SRAM_ADR_SFT: u32 = 0;
pub const COEFF_SRAM_ADR_MASK: u32 = 0x3f;
pub const COEFF_SRAM_ADR_MASK_SFT: u32 = 0x3f << 0;

/* AFE_GASRC0_NEW_CON12 */
pub const RING_DBG_RD_SFT: u32 = 0;
pub const RING_DBG_RD_MASK: u32 = 0x3ffffff;
pub const RING_DBG_RD_MASK_SFT: u32 = 0x3ffffff << 0;

/* AFE_GASRC0_NEW_CON13 */
pub const FREQ_CALI_AUTORST_TH_HIGH_SFT: u32 = 0;
pub const FREQ_CALI_AUTORST_TH_HIGH_MASK: u32 = 0xffffff;
pub const FREQ_CALI_AUTORST_TH_HIGH_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_GASRC0_NEW_CON14 */
pub const FREQ_CALI_AUTORST_TH_LOW_SFT: u32 = 0;
pub const FREQ_CALI_AUTORST_TH_LOW_MASK: u32 = 0xffffff;
pub const FREQ_CALI_AUTORST_TH_LOW_MASK_SFT: u32 = 0xffffff << 0;

/* AFE_GASRC0_NEW_IP_VERSION */
pub const IP_VERSION_SFT: u32 = 0;
pub const IP_VERSION_MASK: u32 = 0xffffffff;
pub const IP_VERSION_MASK_SFT: u32 = 0xffffffff << 0;

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
pub const AUDIO_PROJECT_MON: u32 = 0x80;
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
pub const AFE_PCM_TOP_IP_VERSION: u32 = 0xe8;
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
pub const AFE_GAIN_0_1_IP_VERSION: u32 = 0x474;
pub const AFE_GAIN_2_3_IP_VERSION: u32 = 0x478;
pub const AFE_ADDA_DL_IPM_VER_MON: u32 = 0x4c0;
pub const AFE_ADDA_DL_SRC_CON0: u32 = 0x4d0;
pub const AFE_ADDA_DL_SRC_CON1: u32 = 0x4d4;
pub const AFE_ADDA_DL_SRC_DEBUG_MON0: u32 = 0x4d8;
pub const AFE_ADDA_DL_PREDIS_CON0: u32 = 0x4dc;
pub const AFE_ADDA_DL_PREDIS_CON1: u32 = 0x4e0;
pub const AFE_ADDA_DL_PREDIS_CON2: u32 = 0x4e4;
pub const AFE_ADDA_DL_PREDIS_CON3: u32 = 0x4e8;
pub const AFE_ADDA_DL_SDM_DCCOMP_CON: u32 = 0x4ec;
pub const AFE_ADDA_DL_SDM_TEST: u32 = 0x4f0;
pub const AFE_ADDA_DL_DC_COMP_CFG0: u32 = 0x4f4;
pub const AFE_ADDA_DL_DC_COMP_CFG1: u32 = 0x4f8;
pub const AFE_ADDA_DL_SDM_OUT_MON: u32 = 0x4fc;
pub const AFE_ADDA_DL_SRC_LCH_MON: u32 = 0x500;
pub const AFE_ADDA_DL_SRC_RCH_MON: u32 = 0x504;
pub const AFE_ADDA_DL_SRC_DEBUG: u32 = 0x508;
pub const AFE_ADDA_DL_SDM_DITHER_CON: u32 = 0x50c;
pub const AFE_ADDA_DL_SDM_AUTO_RESET_CON: u32 = 0x510;
pub const AFE_ADDA_DL_HBF1_SCF1_CONFIG: u32 = 0x514;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP1_TAP2_CONFIG: u32 = 0x518;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP3_TAP4_CONFIG: u32 = 0x51c;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP5_TAP6_CONFIG: u32 = 0x520;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP7_TAP8_CONFIG: u32 = 0x524;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP9_TAP10_CONFIG: u32 = 0x528;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP11_TAP12_CONFIG: u32 = 0x52c;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP13_TAP14_CONFIG: u32 = 0x530;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP15_TAP16_CONFIG: u32 = 0x534;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP17_TAP18_CONFIG: u32 = 0x538;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP19_TAP20_CONFIG: u32 = 0x53c;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP21_TAP22_CONFIG: u32 = 0x540;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP23_TAP24_CONFIG: u32 = 0x544;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP25_TAP26_CONFIG: u32 = 0x548;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP27_TAP28_CONFIG: u32 = 0x54c;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP29_TAP30_CONFIG: u32 = 0x550;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP31_TAP32_CONFIG: u32 = 0x554;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP33_TAP34_CONFIG: u32 = 0x558;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP35_TAP36_CONFIG: u32 = 0x55c;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP37_TAP38_CONFIG: u32 = 0x560;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP39_TAP40_CONFIG: u32 = 0x564;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP41_TAP42_CONFIG: u32 = 0x568;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP43_TAP44_CONFIG: u32 = 0x56c;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP45_TAP46_CONFIG: u32 = 0x570;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP47_TAP48_CONFIG: u32 = 0x574;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP49_TAP50_CONFIG: u32 = 0x578;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP51_TAP52_CONFIG: u32 = 0x57c;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP53_TAP54_CONFIG: u32 = 0x580;
pub const AFE_ADDA_DL_HBF1_SCF1_TAP55_TAP56_CONFIG: u32 = 0x584;
pub const AFE_DEM_IDWA_CON0: u32 = 0xa1c;
pub const DEM_RECONSTRUCT_MON: u32 = 0xa20;
pub const AFE_CM0_CON0: u32 = 0xba0;
pub const AFE_CM0_MON: u32 = 0xba4;
pub const AFE_CM0_IP_VERSION: u32 = 0xba8;
pub const AFE_CM1_CON0: u32 = 0xbb0;
pub const AFE_CM1_MON: u32 = 0xbb4;
pub const AFE_CM1_IP_VERSION: u32 = 0xbb8;
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
pub const AFE_ADDA_DMIC0_SRC_CON0: u32 = 0xdd0;
pub const AFE_ADDA_DMIC0_SRC_CON1: u32 = 0xdd4;
pub const AFE_ADDA_DMIC0_SRC_CON2: u32 = 0xdd8;
pub const AFE_ADDA_DMIC0_SRC_DEBUG: u32 = 0xddc;
pub const AFE_ADDA_DMIC0_SRC_DEBUG_MON0: u32 = 0xde0;
pub const AFE_ADDA_DMIC0_SRC_MON0: u32 = 0xde4;
pub const AFE_ADDA_DMIC0_SRC_MON1: u32 = 0xde8;
pub const AFE_ADDA_DMIC0_IIR_COEF_02_01: u32 = 0xdec;
pub const AFE_ADDA_DMIC0_IIR_COEF_04_03: u32 = 0xdf0;
pub const AFE_ADDA_DMIC0_IIR_COEF_06_05: u32 = 0xdf4;
pub const AFE_ADDA_DMIC0_IIR_COEF_08_07: u32 = 0xdf8;
pub const AFE_ADDA_DMIC0_IIR_COEF_10_09: u32 = 0xdfc;
pub const AFE_ADDA_DMIC0_ULCF_CFG_02_01: u32 = 0xe00;
pub const AFE_ADDA_DMIC0_ULCF_CFG_04_03: u32 = 0xe04;
pub const AFE_ADDA_DMIC0_ULCF_CFG_06_05: u32 = 0xe08;
pub const AFE_ADDA_DMIC0_ULCF_CFG_08_07: u32 = 0xe0c;
pub const AFE_ADDA_DMIC0_ULCF_CFG_10_09: u32 = 0xe10;
pub const AFE_ADDA_DMIC0_ULCF_CFG_12_11: u32 = 0xe14;
pub const AFE_ADDA_DMIC0_ULCF_CFG_14_13: u32 = 0xe18;
pub const AFE_ADDA_DMIC0_ULCF_CFG_16_15: u32 = 0xe1c;
pub const AFE_ADDA_DMIC0_ULCF_CFG_18_17: u32 = 0xe20;
pub const AFE_ADDA_DMIC0_ULCF_CFG_20_19: u32 = 0xe24;
pub const AFE_ADDA_DMIC0_ULCF_CFG_22_21: u32 = 0xe28;
pub const AFE_ADDA_DMIC0_ULCF_CFG_24_23: u32 = 0xe2c;
pub const AFE_ADDA_DMIC0_ULCF_CFG_26_25: u32 = 0xe30;
pub const AFE_ADDA_DMIC0_ULCF_CFG_28_27: u32 = 0xe34;
pub const AFE_ADDA_DMIC0_ULCF_CFG_30_29: u32 = 0xe38;
pub const AFE_ADDA_DMIC0_ULCF_CFG_32_31: u32 = 0xe3c;
pub const AFE_ADDA_DMIC0_IP_VERSION: u32 = 0xe4c;
pub const AFE_ADDA_DMIC1_SRC_CON0: u32 = 0xe50;
pub const AFE_ADDA_DMIC1_SRC_CON1: u32 = 0xe54;
pub const AFE_ADDA_DMIC1_SRC_CON2: u32 = 0xe58;
pub const AFE_ADDA_DMIC1_SRC_DEBUG: u32 = 0xe5c;
pub const AFE_ADDA_DMIC1_SRC_DEBUG_MON0: u32 = 0xe60;
pub const AFE_ADDA_DMIC1_SRC_MON0: u32 = 0xe64;
pub const AFE_ADDA_DMIC1_SRC_MON1: u32 = 0xe68;
pub const AFE_ADDA_DMIC1_IIR_COEF_02_01: u32 = 0xe6c;
pub const AFE_ADDA_DMIC1_IIR_COEF_04_03: u32 = 0xe70;
pub const AFE_ADDA_DMIC1_IIR_COEF_06_05: u32 = 0xe74;
pub const AFE_ADDA_DMIC1_IIR_COEF_08_07: u32 = 0xe78;
pub const AFE_ADDA_DMIC1_IIR_COEF_10_09: u32 = 0xe7c;
pub const AFE_ADDA_DMIC1_ULCF_CFG_02_01: u32 = 0xe80;
pub const AFE_ADDA_DMIC1_ULCF_CFG_04_03: u32 = 0xe84;
pub const AFE_ADDA_DMIC1_ULCF_CFG_06_05: u32 = 0xe88;
pub const AFE_ADDA_DMIC1_ULCF_CFG_08_07: u32 = 0xe8c;
pub const AFE_ADDA_DMIC1_ULCF_CFG_10_09: u32 = 0xe90;
pub const AFE_ADDA_DMIC1_ULCF_CFG_12_11: u32 = 0xe94;
pub const AFE_ADDA_DMIC1_ULCF_CFG_14_13: u32 = 0xe98;
pub const AFE_ADDA_DMIC1_ULCF_CFG_16_15: u32 = 0xe9c;
pub const AFE_ADDA_DMIC1_ULCF_CFG_18_17: u32 = 0xea0;
pub const AFE_ADDA_DMIC1_ULCF_CFG_20_19: u32 = 0xea4;
pub const AFE_ADDA_DMIC1_ULCF_CFG_22_21: u32 = 0xea8;
pub const AFE_ADDA_DMIC1_ULCF_CFG_24_23: u32 = 0xeac;
pub const AFE_ADDA_DMIC1_ULCF_CFG_26_25: u32 = 0xeb0;
pub const AFE_ADDA_DMIC1_ULCF_CFG_28_27: u32 = 0xeb4;
pub const AFE_ADDA_DMIC1_ULCF_CFG_30_29: u32 = 0xeb8;
pub const AFE_ADDA_DMIC1_ULCF_CFG_32_31: u32 = 0xebc;
pub const AFE_ADDA_DMIC1_IP_VERSION: u32 = 0xecc;
pub const AFE_ADDA_ULSRC_PHASE_CLK_CON0: u32 = 0xf00;
pub const AFE_ADDA_ULSRC_PHASE_CLK_CON1: u32 = 0xf04;
pub const AFE_ADDA_ULSRC_PHASE_CLK_CON2: u32 = 0xf08;
pub const AFE_ADDA_ULSRC_PHASE_CLK_CON3: u32 = 0xf0c;
pub const AFE_ADDA_ULSRC_PHASE_CLK_CON4: u32 = 0xf10;
pub const AFE_ADDA_ULSRC_PHASE_ENGEN_CON0: u32 = 0xf14;
pub const AFE_ADDA_ULSRC_PHASE_ENGEN_CON1: u32 = 0xf18;
pub const AFE_ADDA_ULSRC_PHASE_RST_CON0: u32 = 0xf1c;
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
pub const ETDM_0_3_COWORK_CON0: u32 = 0x1680;
pub const ETDM_0_3_COWORK_CON1: u32 = 0x1684;
pub const ETDM_0_3_COWORK_CON2: u32 = 0x1688;
pub const ETDM_0_3_COWORK_CON3: u32 = 0x168c;
pub const ETDM_4_7_COWORK_CON0: u32 = 0x1690;
pub const ETDM_4_7_COWORK_CON1: u32 = 0x1694;
pub const ETDM_4_7_COWORK_CON2: u32 = 0x1698;
pub const ETDM_4_7_COWORK_CON3: u32 = 0x169c;
pub const ETDM_IP_VERSION: u32 = 0x1c4c;
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
pub const AFE_CONN004_6: u32 = 0x2118;
pub const AFE_CONN005_0: u32 = 0x2120;
pub const AFE_CONN005_1: u32 = 0x2124;
pub const AFE_CONN005_2: u32 = 0x2128;
pub const AFE_CONN005_4: u32 = 0x2130;
pub const AFE_CONN005_6: u32 = 0x2138;
pub const AFE_CONN006_0: u32 = 0x2140;
pub const AFE_CONN006_1: u32 = 0x2144;
pub const AFE_CONN006_2: u32 = 0x2148;
pub const AFE_CONN006_4: u32 = 0x2150;
pub const AFE_CONN006_6: u32 = 0x2158;
pub const AFE_CONN007_0: u32 = 0x2160;
pub const AFE_CONN007_1: u32 = 0x2164;
pub const AFE_CONN007_2: u32 = 0x2168;
pub const AFE_CONN007_4: u32 = 0x2170;
pub const AFE_CONN007_6: u32 = 0x2178;
pub const AFE_CONN008_0: u32 = 0x2180;
pub const AFE_CONN008_1: u32 = 0x2184;
pub const AFE_CONN008_2: u32 = 0x2188;
pub const AFE_CONN008_4: u32 = 0x2190;
pub const AFE_CONN008_6: u32 = 0x2198;
pub const AFE_CONN009_0: u32 = 0x21a0;
pub const AFE_CONN009_1: u32 = 0x21a4;
pub const AFE_CONN009_2: u32 = 0x21a8;
pub const AFE_CONN009_4: u32 = 0x21b0;
pub const AFE_CONN009_6: u32 = 0x21b8;
pub const AFE_CONN010_0: u32 = 0x21c0;
pub const AFE_CONN010_1: u32 = 0x21c4;
pub const AFE_CONN010_2: u32 = 0x21c8;
pub const AFE_CONN010_4: u32 = 0x21d0;
pub const AFE_CONN010_6: u32 = 0x21d8;
pub const AFE_CONN011_0: u32 = 0x21e0;
pub const AFE_CONN011_1: u32 = 0x21e4;
pub const AFE_CONN011_2: u32 = 0x21e8;
pub const AFE_CONN011_4: u32 = 0x21f0;
pub const AFE_CONN011_6: u32 = 0x21f8;
pub const AFE_CONN014_0: u32 = 0x2240;
pub const AFE_CONN014_1: u32 = 0x2244;
pub const AFE_CONN014_2: u32 = 0x2248;
pub const AFE_CONN014_4: u32 = 0x2250;
pub const AFE_CONN014_6: u32 = 0x2258;
pub const AFE_CONN015_0: u32 = 0x2260;
pub const AFE_CONN015_1: u32 = 0x2264;
pub const AFE_CONN015_2: u32 = 0x2268;
pub const AFE_CONN015_4: u32 = 0x2270;
pub const AFE_CONN015_6: u32 = 0x2278;
pub const AFE_CONN016_0: u32 = 0x2280;
pub const AFE_CONN016_1: u32 = 0x2284;
pub const AFE_CONN016_2: u32 = 0x2288;
pub const AFE_CONN016_4: u32 = 0x2290;
pub const AFE_CONN016_6: u32 = 0x2298;
pub const AFE_CONN017_0: u32 = 0x22a0;
pub const AFE_CONN017_1: u32 = 0x22a4;
pub const AFE_CONN017_2: u32 = 0x22a8;
pub const AFE_CONN017_4: u32 = 0x22b0;
pub const AFE_CONN017_6: u32 = 0x22b8;
pub const AFE_CONN018_0: u32 = 0x22c0;
pub const AFE_CONN018_1: u32 = 0x22c4;
pub const AFE_CONN018_2: u32 = 0x22c8;
pub const AFE_CONN018_4: u32 = 0x22d0;
pub const AFE_CONN018_6: u32 = 0x22d8;
pub const AFE_CONN019_0: u32 = 0x22e0;
pub const AFE_CONN019_1: u32 = 0x22e4;
pub const AFE_CONN019_2: u32 = 0x22e8;
pub const AFE_CONN019_4: u32 = 0x22f0;
pub const AFE_CONN019_6: u32 = 0x22f8;
pub const AFE_CONN020_0: u32 = 0x2300;
pub const AFE_CONN020_1: u32 = 0x2304;
pub const AFE_CONN020_2: u32 = 0x2308;
pub const AFE_CONN020_4: u32 = 0x2310;
pub const AFE_CONN020_6: u32 = 0x2318;
pub const AFE_CONN021_0: u32 = 0x2320;
pub const AFE_CONN021_1: u32 = 0x2324;
pub const AFE_CONN021_2: u32 = 0x2328;
pub const AFE_CONN021_4: u32 = 0x2330;
pub const AFE_CONN021_6: u32 = 0x2338;
pub const AFE_CONN022_0: u32 = 0x2340;
pub const AFE_CONN022_1: u32 = 0x2344;
pub const AFE_CONN022_2: u32 = 0x2348;
pub const AFE_CONN022_4: u32 = 0x2350;
pub const AFE_CONN022_6: u32 = 0x2358;
pub const AFE_CONN023_0: u32 = 0x2360;
pub const AFE_CONN023_1: u32 = 0x2364;
pub const AFE_CONN023_2: u32 = 0x2368;
pub const AFE_CONN023_4: u32 = 0x2370;
pub const AFE_CONN023_6: u32 = 0x2378;
pub const AFE_CONN024_0: u32 = 0x2380;
pub const AFE_CONN024_1: u32 = 0x2384;
pub const AFE_CONN024_2: u32 = 0x2388;
pub const AFE_CONN024_4: u32 = 0x2390;
pub const AFE_CONN024_6: u32 = 0x2398;
pub const AFE_CONN025_0: u32 = 0x23a0;
pub const AFE_CONN025_1: u32 = 0x23a4;
pub const AFE_CONN025_2: u32 = 0x23a8;
pub const AFE_CONN025_4: u32 = 0x23b0;
pub const AFE_CONN025_6: u32 = 0x23b8;
pub const AFE_CONN026_0: u32 = 0x23c0;
pub const AFE_CONN026_1: u32 = 0x23c4;
pub const AFE_CONN026_2: u32 = 0x23c8;
pub const AFE_CONN026_4: u32 = 0x23d0;
pub const AFE_CONN026_6: u32 = 0x23d8;
pub const AFE_CONN027_0: u32 = 0x23e0;
pub const AFE_CONN027_1: u32 = 0x23e4;
pub const AFE_CONN027_2: u32 = 0x23e8;
pub const AFE_CONN027_4: u32 = 0x23f0;
pub const AFE_CONN027_6: u32 = 0x23f8;
pub const AFE_CONN028_0: u32 = 0x2400;
pub const AFE_CONN028_1: u32 = 0x2404;
pub const AFE_CONN028_2: u32 = 0x2408;
pub const AFE_CONN028_4: u32 = 0x2410;
pub const AFE_CONN028_6: u32 = 0x2418;
pub const AFE_CONN029_0: u32 = 0x2420;
pub const AFE_CONN029_1: u32 = 0x2424;
pub const AFE_CONN029_2: u32 = 0x2428;
pub const AFE_CONN029_4: u32 = 0x2430;
pub const AFE_CONN029_6: u32 = 0x2438;
pub const AFE_CONN030_0: u32 = 0x2440;
pub const AFE_CONN030_1: u32 = 0x2444;
pub const AFE_CONN030_2: u32 = 0x2448;
pub const AFE_CONN030_4: u32 = 0x2450;
pub const AFE_CONN030_6: u32 = 0x2458;
pub const AFE_CONN031_0: u32 = 0x2460;
pub const AFE_CONN031_1: u32 = 0x2464;
pub const AFE_CONN031_2: u32 = 0x2468;
pub const AFE_CONN031_4: u32 = 0x2470;
pub const AFE_CONN031_6: u32 = 0x2478;
pub const AFE_CONN032_0: u32 = 0x2480;
pub const AFE_CONN032_1: u32 = 0x2484;
pub const AFE_CONN032_2: u32 = 0x2488;
pub const AFE_CONN032_4: u32 = 0x2490;
pub const AFE_CONN032_6: u32 = 0x2498;
pub const AFE_CONN033_0: u32 = 0x24a0;
pub const AFE_CONN033_1: u32 = 0x24a4;
pub const AFE_CONN033_2: u32 = 0x24a8;
pub const AFE_CONN033_4: u32 = 0x24b0;
pub const AFE_CONN033_6: u32 = 0x24b8;
pub const AFE_CONN034_0: u32 = 0x24c0;
pub const AFE_CONN034_1: u32 = 0x24c4;
pub const AFE_CONN034_2: u32 = 0x24c8;
pub const AFE_CONN034_4: u32 = 0x24d0;
pub const AFE_CONN034_6: u32 = 0x24d8;
pub const AFE_CONN035_0: u32 = 0x24e0;
pub const AFE_CONN035_1: u32 = 0x24e4;
pub const AFE_CONN035_2: u32 = 0x24e8;
pub const AFE_CONN035_4: u32 = 0x24f0;
pub const AFE_CONN035_6: u32 = 0x24f8;
pub const AFE_CONN036_0: u32 = 0x2500;
pub const AFE_CONN036_1: u32 = 0x2504;
pub const AFE_CONN036_2: u32 = 0x2508;
pub const AFE_CONN036_4: u32 = 0x2510;
pub const AFE_CONN036_6: u32 = 0x2518;
pub const AFE_CONN037_0: u32 = 0x2520;
pub const AFE_CONN037_1: u32 = 0x2524;
pub const AFE_CONN037_2: u32 = 0x2528;
pub const AFE_CONN037_4: u32 = 0x2530;
pub const AFE_CONN037_6: u32 = 0x2538;
pub const AFE_CONN038_0: u32 = 0x2540;
pub const AFE_CONN038_1: u32 = 0x2544;
pub const AFE_CONN038_2: u32 = 0x2548;
pub const AFE_CONN038_4: u32 = 0x2550;
pub const AFE_CONN038_6: u32 = 0x2558;
pub const AFE_CONN039_0: u32 = 0x2560;
pub const AFE_CONN039_1: u32 = 0x2564;
pub const AFE_CONN039_2: u32 = 0x2568;
pub const AFE_CONN039_4: u32 = 0x2570;
pub const AFE_CONN039_6: u32 = 0x2578;
pub const AFE_CONN040_0: u32 = 0x2580;
pub const AFE_CONN040_1: u32 = 0x2584;
pub const AFE_CONN040_2: u32 = 0x2588;
pub const AFE_CONN040_4: u32 = 0x2590;
pub const AFE_CONN040_6: u32 = 0x2598;
pub const AFE_CONN041_0: u32 = 0x25a0;
pub const AFE_CONN041_1: u32 = 0x25a4;
pub const AFE_CONN041_2: u32 = 0x25a8;
pub const AFE_CONN041_4: u32 = 0x25b0;
pub const AFE_CONN041_6: u32 = 0x25b8;
pub const AFE_CONN042_0: u32 = 0x25c0;
pub const AFE_CONN042_1: u32 = 0x25c4;
pub const AFE_CONN042_2: u32 = 0x25c8;
pub const AFE_CONN042_4: u32 = 0x25d0;
pub const AFE_CONN042_6: u32 = 0x25d8;
pub const AFE_CONN043_0: u32 = 0x25e0;
pub const AFE_CONN043_1: u32 = 0x25e4;
pub const AFE_CONN043_2: u32 = 0x25e8;
pub const AFE_CONN043_4: u32 = 0x25f0;
pub const AFE_CONN043_6: u32 = 0x25f8;
pub const AFE_CONN044_0: u32 = 0x2600;
pub const AFE_CONN044_1: u32 = 0x2604;
pub const AFE_CONN044_2: u32 = 0x2608;
pub const AFE_CONN044_4: u32 = 0x2610;
pub const AFE_CONN044_6: u32 = 0x2618;
pub const AFE_CONN045_0: u32 = 0x2620;
pub const AFE_CONN045_1: u32 = 0x2624;
pub const AFE_CONN045_2: u32 = 0x2628;
pub const AFE_CONN045_4: u32 = 0x2630;
pub const AFE_CONN045_6: u32 = 0x2638;
pub const AFE_CONN046_0: u32 = 0x2640;
pub const AFE_CONN046_1: u32 = 0x2644;
pub const AFE_CONN046_2: u32 = 0x2648;
pub const AFE_CONN046_4: u32 = 0x2650;
pub const AFE_CONN046_6: u32 = 0x2658;
pub const AFE_CONN047_0: u32 = 0x2660;
pub const AFE_CONN047_1: u32 = 0x2664;
pub const AFE_CONN047_2: u32 = 0x2668;
pub const AFE_CONN047_4: u32 = 0x2670;
pub const AFE_CONN047_6: u32 = 0x2678;
pub const AFE_CONN048_0: u32 = 0x2680;
pub const AFE_CONN048_1: u32 = 0x2684;
pub const AFE_CONN048_2: u32 = 0x2688;
pub const AFE_CONN048_4: u32 = 0x2690;
pub const AFE_CONN048_6: u32 = 0x2698;
pub const AFE_CONN049_0: u32 = 0x26a0;
pub const AFE_CONN049_1: u32 = 0x26a4;
pub const AFE_CONN049_2: u32 = 0x26a8;
pub const AFE_CONN049_4: u32 = 0x26b0;
pub const AFE_CONN049_6: u32 = 0x26b8;
pub const AFE_CONN050_0: u32 = 0x26c0;
pub const AFE_CONN050_1: u32 = 0x26c4;
pub const AFE_CONN050_2: u32 = 0x26c8;
pub const AFE_CONN050_4: u32 = 0x26d0;
pub const AFE_CONN050_6: u32 = 0x26d8;
pub const AFE_CONN051_0: u32 = 0x26e0;
pub const AFE_CONN051_1: u32 = 0x26e4;
pub const AFE_CONN051_2: u32 = 0x26e8;
pub const AFE_CONN051_4: u32 = 0x26f0;
pub const AFE_CONN051_6: u32 = 0x26f8;
pub const AFE_CONN052_0: u32 = 0x2700;
pub const AFE_CONN052_1: u32 = 0x2704;
pub const AFE_CONN052_2: u32 = 0x2708;
pub const AFE_CONN052_4: u32 = 0x2710;
pub const AFE_CONN052_6: u32 = 0x2718;
pub const AFE_CONN053_0: u32 = 0x2720;
pub const AFE_CONN053_1: u32 = 0x2724;
pub const AFE_CONN053_2: u32 = 0x2728;
pub const AFE_CONN053_4: u32 = 0x2730;
pub const AFE_CONN053_6: u32 = 0x2738;
pub const AFE_CONN054_0: u32 = 0x2740;
pub const AFE_CONN054_1: u32 = 0x2744;
pub const AFE_CONN054_2: u32 = 0x2748;
pub const AFE_CONN054_4: u32 = 0x2750;
pub const AFE_CONN054_6: u32 = 0x2758;
pub const AFE_CONN055_0: u32 = 0x2760;
pub const AFE_CONN055_1: u32 = 0x2764;
pub const AFE_CONN055_2: u32 = 0x2768;
pub const AFE_CONN055_4: u32 = 0x2770;
pub const AFE_CONN055_6: u32 = 0x2778;
pub const AFE_CONN056_0: u32 = 0x2780;
pub const AFE_CONN056_1: u32 = 0x2784;
pub const AFE_CONN056_2: u32 = 0x2788;
pub const AFE_CONN056_4: u32 = 0x2790;
pub const AFE_CONN056_6: u32 = 0x2798;
pub const AFE_CONN057_0: u32 = 0x27a0;
pub const AFE_CONN057_1: u32 = 0x27a4;
pub const AFE_CONN057_2: u32 = 0x27a8;
pub const AFE_CONN057_4: u32 = 0x27b0;
pub const AFE_CONN057_6: u32 = 0x27b8;
pub const AFE_CONN058_0: u32 = 0x27c0;
pub const AFE_CONN058_1: u32 = 0x27c4;
pub const AFE_CONN058_2: u32 = 0x27c8;
pub const AFE_CONN058_4: u32 = 0x27d0;
pub const AFE_CONN058_6: u32 = 0x27d8;
pub const AFE_CONN059_0: u32 = 0x27e0;
pub const AFE_CONN059_1: u32 = 0x27e4;
pub const AFE_CONN059_2: u32 = 0x27e8;
pub const AFE_CONN059_4: u32 = 0x27f0;
pub const AFE_CONN059_6: u32 = 0x27f8;
pub const AFE_CONN060_0: u32 = 0x2800;
pub const AFE_CONN060_1: u32 = 0x2804;
pub const AFE_CONN060_2: u32 = 0x2808;
pub const AFE_CONN060_4: u32 = 0x2810;
pub const AFE_CONN060_6: u32 = 0x2818;
pub const AFE_CONN061_0: u32 = 0x2820;
pub const AFE_CONN061_1: u32 = 0x2824;
pub const AFE_CONN061_2: u32 = 0x2828;
pub const AFE_CONN061_4: u32 = 0x2830;
pub const AFE_CONN061_6: u32 = 0x2838;
pub const AFE_CONN062_0: u32 = 0x2840;
pub const AFE_CONN062_1: u32 = 0x2844;
pub const AFE_CONN062_2: u32 = 0x2848;
pub const AFE_CONN062_4: u32 = 0x2850;
pub const AFE_CONN062_6: u32 = 0x2858;
pub const AFE_CONN063_0: u32 = 0x2860;
pub const AFE_CONN063_1: u32 = 0x2864;
pub const AFE_CONN063_2: u32 = 0x2868;
pub const AFE_CONN063_4: u32 = 0x2870;
pub const AFE_CONN063_6: u32 = 0x2878;
pub const AFE_CONN066_0: u32 = 0x28c0;
pub const AFE_CONN066_1: u32 = 0x28c4;
pub const AFE_CONN066_2: u32 = 0x28c8;
pub const AFE_CONN066_4: u32 = 0x28d0;
pub const AFE_CONN066_6: u32 = 0x28d8;
pub const AFE_CONN067_0: u32 = 0x28e0;
pub const AFE_CONN067_1: u32 = 0x28e4;
pub const AFE_CONN067_2: u32 = 0x28e8;
pub const AFE_CONN067_4: u32 = 0x28f0;
pub const AFE_CONN067_6: u32 = 0x28f8;
pub const AFE_CONN068_0: u32 = 0x2900;
pub const AFE_CONN068_1: u32 = 0x2904;
pub const AFE_CONN068_2: u32 = 0x2908;
pub const AFE_CONN068_4: u32 = 0x2910;
pub const AFE_CONN068_6: u32 = 0x2918;
pub const AFE_CONN069_0: u32 = 0x2920;
pub const AFE_CONN069_1: u32 = 0x2924;
pub const AFE_CONN069_2: u32 = 0x2928;
pub const AFE_CONN069_4: u32 = 0x2930;
pub const AFE_CONN069_6: u32 = 0x2938;
pub const AFE_CONN096_0: u32 = 0x2c80;
pub const AFE_CONN096_1: u32 = 0x2c84;
pub const AFE_CONN096_2: u32 = 0x2c88;
pub const AFE_CONN096_4: u32 = 0x2c90;
pub const AFE_CONN096_6: u32 = 0x2c98;
pub const AFE_CONN097_0: u32 = 0x2ca0;
pub const AFE_CONN097_1: u32 = 0x2ca4;
pub const AFE_CONN097_2: u32 = 0x2ca8;
pub const AFE_CONN097_4: u32 = 0x2cb0;
pub const AFE_CONN097_6: u32 = 0x2cb8;
pub const AFE_CONN098_0: u32 = 0x2cc0;
pub const AFE_CONN098_1: u32 = 0x2cc4;
pub const AFE_CONN098_2: u32 = 0x2cc8;
pub const AFE_CONN098_4: u32 = 0x2cd0;
pub const AFE_CONN098_6: u32 = 0x2cd8;
pub const AFE_CONN099_0: u32 = 0x2ce0;
pub const AFE_CONN099_1: u32 = 0x2ce4;
pub const AFE_CONN099_2: u32 = 0x2ce8;
pub const AFE_CONN099_4: u32 = 0x2cf0;
pub const AFE_CONN099_6: u32 = 0x2cf8;
pub const AFE_CONN100_0: u32 = 0x2d00;
pub const AFE_CONN100_1: u32 = 0x2d04;
pub const AFE_CONN100_2: u32 = 0x2d08;
pub const AFE_CONN100_4: u32 = 0x2d10;
pub const AFE_CONN100_6: u32 = 0x2d18;
pub const AFE_CONN108_0: u32 = 0x2e00;
pub const AFE_CONN108_1: u32 = 0x2e04;
pub const AFE_CONN108_2: u32 = 0x2e08;
pub const AFE_CONN108_4: u32 = 0x2e10;
pub const AFE_CONN108_6: u32 = 0x2e18;
pub const AFE_CONN109_0: u32 = 0x2e20;
pub const AFE_CONN109_1: u32 = 0x2e24;
pub const AFE_CONN109_2: u32 = 0x2e28;
pub const AFE_CONN109_4: u32 = 0x2e30;
pub const AFE_CONN109_6: u32 = 0x2e38;
pub const AFE_CONN110_0: u32 = 0x2e40;
pub const AFE_CONN110_1: u32 = 0x2e44;
pub const AFE_CONN110_2: u32 = 0x2e48;
pub const AFE_CONN110_4: u32 = 0x2e50;
pub const AFE_CONN110_6: u32 = 0x2e58;
pub const AFE_CONN111_0: u32 = 0x2e60;
pub const AFE_CONN111_1: u32 = 0x2e64;
pub const AFE_CONN111_2: u32 = 0x2e68;
pub const AFE_CONN111_4: u32 = 0x2e70;
pub const AFE_CONN111_6: u32 = 0x2e78;
pub const AFE_CONN116_0: u32 = 0x2f00;
pub const AFE_CONN116_1: u32 = 0x2f04;
pub const AFE_CONN116_2: u32 = 0x2f08;
pub const AFE_CONN116_4: u32 = 0x2f10;
pub const AFE_CONN116_6: u32 = 0x2f18;
pub const AFE_CONN117_0: u32 = 0x2f20;
pub const AFE_CONN117_1: u32 = 0x2f24;
pub const AFE_CONN117_2: u32 = 0x2f28;
pub const AFE_CONN117_4: u32 = 0x2f30;
pub const AFE_CONN117_6: u32 = 0x2f38;
pub const AFE_CONN118_0: u32 = 0x2f40;
pub const AFE_CONN118_1: u32 = 0x2f44;
pub const AFE_CONN118_2: u32 = 0x2f48;
pub const AFE_CONN118_4: u32 = 0x2f50;
pub const AFE_CONN118_6: u32 = 0x2f58;
pub const AFE_CONN119_0: u32 = 0x2f60;
pub const AFE_CONN119_1: u32 = 0x2f64;
pub const AFE_CONN119_2: u32 = 0x2f68;
pub const AFE_CONN119_4: u32 = 0x2f70;
pub const AFE_CONN119_6: u32 = 0x2f78;
pub const AFE_CONN120_0: u32 = 0x2f80;
pub const AFE_CONN120_1: u32 = 0x2f84;
pub const AFE_CONN120_2: u32 = 0x2f88;
pub const AFE_CONN120_4: u32 = 0x2f90;
pub const AFE_CONN120_6: u32 = 0x2f98;
pub const AFE_CONN121_0: u32 = 0x2fa0;
pub const AFE_CONN121_1: u32 = 0x2fa4;
pub const AFE_CONN121_2: u32 = 0x2fa8;
pub const AFE_CONN121_4: u32 = 0x2fb0;
pub const AFE_CONN121_6: u32 = 0x2fb8;
pub const AFE_CONN122_0: u32 = 0x2fc0;
pub const AFE_CONN122_1: u32 = 0x2fc4;
pub const AFE_CONN122_2: u32 = 0x2fc8;
pub const AFE_CONN122_4: u32 = 0x2fd0;
pub const AFE_CONN122_6: u32 = 0x2fd8;
pub const AFE_CONN123_0: u32 = 0x2fe0;
pub const AFE_CONN123_1: u32 = 0x2fe4;
pub const AFE_CONN123_2: u32 = 0x2fe8;
pub const AFE_CONN123_4: u32 = 0x2ff0;
pub const AFE_CONN123_6: u32 = 0x2ff8;
pub const AFE_CONN180_0: u32 = 0x3700;
pub const AFE_CONN180_1: u32 = 0x3704;
pub const AFE_CONN180_2: u32 = 0x3708;
pub const AFE_CONN180_4: u32 = 0x3710;
pub const AFE_CONN180_6: u32 = 0x3718;
pub const AFE_CONN181_0: u32 = 0x3720;
pub const AFE_CONN181_1: u32 = 0x3724;
pub const AFE_CONN181_2: u32 = 0x3728;
pub const AFE_CONN181_4: u32 = 0x3730;
pub const AFE_CONN181_6: u32 = 0x3738;
pub const AFE_CONN182_0: u32 = 0x3740;
pub const AFE_CONN182_1: u32 = 0x3744;
pub const AFE_CONN182_2: u32 = 0x3748;
pub const AFE_CONN182_4: u32 = 0x3750;
pub const AFE_CONN182_6: u32 = 0x3758;
pub const AFE_CONN183_0: u32 = 0x3760;
pub const AFE_CONN183_1: u32 = 0x3764;
pub const AFE_CONN183_2: u32 = 0x3768;
pub const AFE_CONN183_4: u32 = 0x3770;
pub const AFE_CONN183_6: u32 = 0x3778;
pub const AFE_CONN184_0: u32 = 0x3780;
pub const AFE_CONN184_1: u32 = 0x3784;
pub const AFE_CONN184_2: u32 = 0x3788;
pub const AFE_CONN184_4: u32 = 0x3790;
pub const AFE_CONN184_6: u32 = 0x3798;
pub const AFE_CONN185_0: u32 = 0x37a0;
pub const AFE_CONN185_1: u32 = 0x37a4;
pub const AFE_CONN185_2: u32 = 0x37a8;
pub const AFE_CONN185_4: u32 = 0x37b0;
pub const AFE_CONN185_6: u32 = 0x37b8;
pub const AFE_CONN186_0: u32 = 0x37c0;
pub const AFE_CONN186_1: u32 = 0x37c4;
pub const AFE_CONN186_2: u32 = 0x37c8;
pub const AFE_CONN186_4: u32 = 0x37d0;
pub const AFE_CONN186_6: u32 = 0x37d8;
pub const AFE_CONN187_0: u32 = 0x37e0;
pub const AFE_CONN187_1: u32 = 0x37e4;
pub const AFE_CONN187_2: u32 = 0x37e8;
pub const AFE_CONN187_4: u32 = 0x37f0;
pub const AFE_CONN187_6: u32 = 0x37f8;
pub const AFE_CONN188_0: u32 = 0x3800;
pub const AFE_CONN188_1: u32 = 0x3804;
pub const AFE_CONN188_2: u32 = 0x3808;
pub const AFE_CONN188_4: u32 = 0x3810;
pub const AFE_CONN188_6: u32 = 0x3818;
pub const AFE_CONN189_0: u32 = 0x3820;
pub const AFE_CONN189_1: u32 = 0x3824;
pub const AFE_CONN189_2: u32 = 0x3828;
pub const AFE_CONN189_4: u32 = 0x3830;
pub const AFE_CONN189_6: u32 = 0x3838;
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
pub const AFE_CONN_RS_5: u32 = 0x40b4;
pub const AFE_CONN_DI_0: u32 = 0x40c0;
pub const AFE_CONN_DI_1: u32 = 0x40c4;
pub const AFE_CONN_DI_2: u32 = 0x40c8;
pub const AFE_CONN_DI_3: u32 = 0x40cc;
pub const AFE_CONN_DI_5: u32 = 0x40d4;
pub const AFE_CONN_16BIT_0: u32 = 0x40e0;
pub const AFE_CONN_16BIT_1: u32 = 0x40e4;
pub const AFE_CONN_16BIT_2: u32 = 0x40e8;
pub const AFE_CONN_16BIT_3: u32 = 0x40ec;
pub const AFE_CONN_16BIT_5: u32 = 0x40f4;
pub const AFE_CONN_24BIT_0: u32 = 0x4100;
pub const AFE_CONN_24BIT_1: u32 = 0x4104;
pub const AFE_CONN_24BIT_2: u32 = 0x4108;
pub const AFE_CONN_24BIT_3: u32 = 0x410c;
pub const AFE_CONN_24BIT_5: u32 = 0x4114;
pub const AFE_CONN_TOP_IP_VERSION: u32 = 0x4120;
pub const AFE_CBIP_CFG0: u32 = 0x4380;
pub const AFE_CBIP_SLV_DECODER_MON0: u32 = 0x4384;
pub const AFE_CBIP_SLV_DECODER_MON1: u32 = 0x4388;
pub const AFE_CBIP_SLV_MUX_MON_CFG: u32 = 0x438c;
pub const AFE_CBIP_SLV_MUX_MON0: u32 = 0x4390;
pub const AFE_CBIP_SLV_MUX_MON1: u32 = 0x4394;
pub const AFE_MEMIF_IP_VERSION: u32 = 0x4398;
pub const AFE_MEMIF_CON0: u32 = 0x4400;
pub const AFE_MEMIF_RD_MON: u32 = 0x4408;
pub const AFE_MEMIF_WR_MON: u32 = 0x440c;
pub const AFE_MEMIF_CFG_MON0: u32 = 0x4410;
pub const AFE_BUS_CFG0: u32 = 0x4414;
pub const AFE_BUS_MON1: u32 = 0x4418;
pub const AFE_BUS_MON2: u32 = 0x441c;
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
pub const AFE_DL0_MEM_UP_MSB: u32 = 0x4468;
pub const AFE_DL0_MEM_UP: u32 = 0x446c;
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
pub const AFE_DL1_MEM_UP_MSB: u32 = 0x4498;
pub const AFE_DL1_MEM_UP: u32 = 0x449c;
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
pub const AFE_DL2_MEM_UP_MSB: u32 = 0x44c8;
pub const AFE_DL2_MEM_UP: u32 = 0x44cc;
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
pub const AFE_DL3_MEM_UP_MSB: u32 = 0x44f8;
pub const AFE_DL3_MEM_UP: u32 = 0x44fc;
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
pub const AFE_DL4_MEM_UP_MSB: u32 = 0x4528;
pub const AFE_DL4_MEM_UP: u32 = 0x452c;
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
pub const AFE_DL5_MEM_UP_MSB: u32 = 0x4558;
pub const AFE_DL5_MEM_UP: u32 = 0x455c;
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
pub const AFE_DL6_MEM_UP_MSB: u32 = 0x4588;
pub const AFE_DL6_MEM_UP: u32 = 0x458c;
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
pub const AFE_DL7_MEM_UP_MSB: u32 = 0x45b8;
pub const AFE_DL7_MEM_UP: u32 = 0x45bc;
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
pub const AFE_DL8_MEM_UP_MSB: u32 = 0x45e8;
pub const AFE_DL8_MEM_UP: u32 = 0x45ec;
pub const AFE_DL_24CH_BASE_MSB: u32 = 0x4620;
pub const AFE_DL_24CH_BASE: u32 = 0x4624;
pub const AFE_DL_24CH_CUR_MSB: u32 = 0x4628;
pub const AFE_DL_24CH_CUR: u32 = 0x462c;
pub const AFE_DL_24CH_END_MSB: u32 = 0x4630;
pub const AFE_DL_24CH_END: u32 = 0x4634;
pub const AFE_DL_24CH_CON0: u32 = 0x4640;
pub const AFE_DL_24CH_MON0: u32 = 0x4644;
pub const AFE_DL_24CH_MEM_UP_MSB: u32 = 0x4648;
pub const AFE_DL_24CH_MEM_UP: u32 = 0x464c;
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
pub const AFE_DL23_MEM_UP_MSB: u32 = 0x46a8;
pub const AFE_DL23_MEM_UP: u32 = 0x46ac;
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
pub const AFE_DL24_MEM_UP_MSB: u32 = 0x46d8;
pub const AFE_DL24_MEM_UP: u32 = 0x46dc;
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
pub const AFE_DL25_MEM_UP_MSB: u32 = 0x4708;
pub const AFE_DL25_MEM_UP: u32 = 0x470c;
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
pub const AFE_VUL_CM0_BASE_MSB: u32 = 0x51c0;
pub const AFE_VUL_CM0_BASE: u32 = 0x51c4;
pub const AFE_VUL_CM0_CUR_MSB: u32 = 0x51c8;
pub const AFE_VUL_CM0_CUR: u32 = 0x51cc;
pub const AFE_VUL_CM0_END_MSB: u32 = 0x51d0;
pub const AFE_VUL_CM0_END: u32 = 0x51d4;
pub const AFE_VUL_CM0_CON0: u32 = 0x51d8;
pub const AFE_VUL_CM0_MON0: u32 = 0x51dc;
pub const AFE_VUL_CM1_BASE_MSB: u32 = 0x51e0;
pub const AFE_VUL_CM1_BASE: u32 = 0x51e4;
pub const AFE_VUL_CM1_CUR_MSB: u32 = 0x51e8;
pub const AFE_VUL_CM1_CUR: u32 = 0x51ec;
pub const AFE_VUL_CM1_END_MSB: u32 = 0x51f0;
pub const AFE_VUL_CM1_END: u32 = 0x51f4;
pub const AFE_VUL_CM1_CON0: u32 = 0x51f8;
pub const AFE_VUL_CM1_MON0: u32 = 0x51fc;
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
pub const AFE_HDMI_OUT_BASE_MSB: u32 = 0x5360;
pub const AFE_HDMI_OUT_BASE: u32 = 0x5364;
pub const AFE_HDMI_OUT_CUR_MSB: u32 = 0x5368;
pub const AFE_HDMI_OUT_CUR: u32 = 0x536c;
pub const AFE_HDMI_OUT_END_MSB: u32 = 0x5370;
pub const AFE_HDMI_OUT_END: u32 = 0x5374;
pub const AFE_HDMI_OUT_CON0: u32 = 0x5378;
pub const AFE_HDMI_OUT_MON0: u32 = 0x537c;
pub const AFE_VUL24_RCH_MON: u32 = 0x53e0;
pub const AFE_VUL24_LCH_MON: u32 = 0x53e4;
pub const AFE_VUL25_RCH_MON: u32 = 0x53e8;
pub const AFE_VUL25_LCH_MON: u32 = 0x53ec;
pub const AFE_VUL_CM0_RCH_MON: u32 = 0x5458;
pub const AFE_VUL_CM0_LCH_MON: u32 = 0x545c;
pub const AFE_VUL_CM1_RCH_MON: u32 = 0x5460;
pub const AFE_VUL_CM1_LCH_MON: u32 = 0x5464;
pub const AFE_DL_24CH_CH0_MON: u32 = 0x5504;
pub const AFE_DL_24CH_CH1_MON: u32 = 0x5508;
pub const AFE_DL_24CH_CH2_MON: u32 = 0x550c;
pub const AFE_DL_24CH_CH3_MON: u32 = 0x5510;
pub const AFE_DL_24CH_CH4_MON: u32 = 0x5514;
pub const AFE_DL_24CH_CH5_MON: u32 = 0x5518;
pub const AFE_DL_24CH_CH6_MON: u32 = 0x551c;
pub const AFE_DL_24CH_CH7_MON: u32 = 0x5520;
pub const AFE_HDMI_OUT_MEM_UP_MSB: u32 = 0x55b0;
pub const AFE_HDMI_OUT_MEM_UP: u32 = 0x55b4;
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
pub const AFE_CONNSYS_I2S_CON_MASK_MON: u32 = 0x5738;
pub const AFE_TDM_CON2_MASK_MON: u32 = 0x5744;
pub const AFE_MTKAIF0_CFG0_MASK_MON: u32 = 0x574c;
pub const AFE_MTKAIF1_CFG0_MASK_MON: u32 = 0x5750;
pub const AFE_ADDA_UL0_SRC_CON0_MASK_MON: u32 = 0x5754;
pub const AFE_ADDA_DMIC0_SRC_CON0_MASK_MON: u32 = 0x5764;
pub const AFE_ADDA_DMIC1_SRC_CON0_MASK_MON: u32 = 0x5768;
pub const AFE_MON_SECURE_CON0: u32 = 0x5840;
pub const AFE_SECURE_CONN_ETDM3: u32 = 0x5850;
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
pub const AFE_SOUNDWIRE_ULSRC_PHASE_CLK_CON0: u32 = 0x9400;
pub const AFE_SOUNDWIRE_ULSRC_PHASE_CLK_CON1: u32 = 0x9404;
pub const AFE_SOUNDWIRE_ULSRC_PHASE_CLK_CON2: u32 = 0x9408;
pub const AFE_SOUNDWIRE_ULSRC_PHASE_CLK_CON3: u32 = 0x940c;
pub const AFE_SOUNDWIRE_ULSRC_PHASE_CLK_CON4: u32 = 0x9410;
pub const AFE_SOUNDWIRE_ULSRC_PHASE_ENGEN_CON0: u32 = 0x9414;
pub const AFE_SOUNDWIRE_ULSRC_PHASE_ENGEN_CON1: u32 = 0x9418;
pub const AFE_SOUNDWIRE_ULSRC_PHASE_RST_CON0: u32 = 0x941c;
pub const AFE_IRQ_MCU_EN: u32 = 0x9d00;
pub const AFE_IRQ_MCU_DSP_EN: u32 = 0x9d04;
pub const AFE_IRQ_MCU_DSP2_EN: u32 = 0x9d08;
pub const AFE_IRQ_MCU_SCP_EN: u32 = 0x9d0c;
pub const AFE_CUSTOM_IRQ_MCU_EN: u32 = 0x9d10;
pub const AFE_CUSTOM_IRQ_MCU_DSP_EN: u32 = 0x9d14;
pub const AFE_CUSTOM_IRQ_MCU_DSP2_EN: u32 = 0x9d18;
pub const AFE_CUSTOM_IRQ_MCU_SCP_EN: u32 = 0x9d1c;
pub const AFE_IRQ_MCU_STATUS: u32 = 0x9d20;
pub const AFE_CUSTOM_IRQ_MCU_STATUS: u32 = 0x9d24;
pub const AFE_IRQ0_MCU_CFG0: u32 = 0x9d40;
pub const AFE_IRQ0_MCU_CFG1: u32 = 0x9d44;
pub const AFE_IRQ1_MCU_CFG0: u32 = 0x9d48;
pub const AFE_IRQ1_MCU_CFG1: u32 = 0x9d4c;
pub const AFE_IRQ2_MCU_CFG0: u32 = 0x9d50;
pub const AFE_IRQ2_MCU_CFG1: u32 = 0x9d54;
pub const AFE_IRQ3_MCU_CFG0: u32 = 0x9d58;
pub const AFE_IRQ3_MCU_CFG1: u32 = 0x9d5c;
pub const AFE_IRQ4_MCU_CFG0: u32 = 0x9d60;
pub const AFE_IRQ4_MCU_CFG1: u32 = 0x9d64;
pub const AFE_IRQ5_MCU_CFG0: u32 = 0x9d68;
pub const AFE_IRQ5_MCU_CFG1: u32 = 0x9d6c;
pub const AFE_IRQ6_MCU_CFG0: u32 = 0x9d70;
pub const AFE_IRQ6_MCU_CFG1: u32 = 0x9d74;
pub const AFE_IRQ7_MCU_CFG0: u32 = 0x9d78;
pub const AFE_IRQ7_MCU_CFG1: u32 = 0x9d7c;
pub const AFE_IRQ8_MCU_CFG0: u32 = 0x9d80;
pub const AFE_IRQ8_MCU_CFG1: u32 = 0x9d84;
pub const AFE_IRQ9_MCU_CFG0: u32 = 0x9d88;
pub const AFE_IRQ9_MCU_CFG1: u32 = 0x9d8c;
pub const AFE_IRQ10_MCU_CFG0: u32 = 0x9d90;
pub const AFE_IRQ10_MCU_CFG1: u32 = 0x9d94;
pub const AFE_IRQ11_MCU_CFG0: u32 = 0x9d98;
pub const AFE_IRQ11_MCU_CFG1: u32 = 0x9d9c;
pub const AFE_IRQ12_MCU_CFG0: u32 = 0x9da0;
pub const AFE_IRQ12_MCU_CFG1: u32 = 0x9da4;
pub const AFE_IRQ13_MCU_CFG0: u32 = 0x9da8;
pub const AFE_IRQ13_MCU_CFG1: u32 = 0x9dac;
pub const AFE_IRQ14_MCU_CFG0: u32 = 0x9db0;
pub const AFE_IRQ14_MCU_CFG1: u32 = 0x9db4;
pub const AFE_IRQ15_MCU_CFG0: u32 = 0x9db8;
pub const AFE_IRQ15_MCU_CFG1: u32 = 0x9dbc;
pub const AFE_IRQ16_MCU_CFG0: u32 = 0x9dc0;
pub const AFE_IRQ16_MCU_CFG1: u32 = 0x9dc4;
pub const AFE_IRQ17_MCU_CFG0: u32 = 0x9dc8;
pub const AFE_IRQ17_MCU_CFG1: u32 = 0x9dcc;
pub const AFE_IRQ18_MCU_CFG0: u32 = 0x9dd0;
pub const AFE_IRQ18_MCU_CFG1: u32 = 0x9dd4;
pub const AFE_IRQ19_MCU_CFG0: u32 = 0x9dd8;
pub const AFE_IRQ19_MCU_CFG1: u32 = 0x9ddc;
pub const AFE_IRQ20_MCU_CFG0: u32 = 0x9de0;
pub const AFE_IRQ20_MCU_CFG1: u32 = 0x9de4;
pub const AFE_IRQ21_MCU_CFG0: u32 = 0x9de8;
pub const AFE_IRQ21_MCU_CFG1: u32 = 0x9dec;
pub const AFE_IRQ22_MCU_CFG0: u32 = 0x9df0;
pub const AFE_IRQ22_MCU_CFG1: u32 = 0x9df4;
pub const AFE_IRQ23_MCU_CFG0: u32 = 0x9df8;
pub const AFE_IRQ23_MCU_CFG1: u32 = 0x9dfc;
pub const AFE_IRQ24_MCU_CFG0: u32 = 0x9e00;
pub const AFE_IRQ24_MCU_CFG1: u32 = 0x9e04;
pub const AFE_IRQ25_MCU_CFG0: u32 = 0x9e08;
pub const AFE_IRQ25_MCU_CFG1: u32 = 0x9e0c;
pub const AFE_IRQ26_MCU_CFG0: u32 = 0x9e10;
pub const AFE_IRQ26_MCU_CFG1: u32 = 0x9e14;
pub const AFE_CUSTOM_IRQ0_MCU_CFG0: u32 = 0x9e68;
pub const AFE_CUSTOM_IRQ22_MCU_CFG0: u32 = 0x9ec8;
pub const AFE_CUSTOM_IRQ22_MCU_CFG1: u32 = 0x9ecc;
pub const AFE_CUSTOM_IRQ23_MCU_CFG0: u32 = 0x9ed0;
pub const AFE_CUSTOM_IRQ23_MCU_CFG1: u32 = 0x9ed4;
pub const AFE_IRQ0_CNT_MON: u32 = 0x9f10;
pub const AFE_IRQ1_CNT_MON: u32 = 0x9f14;
pub const AFE_IRQ2_CNT_MON: u32 = 0x9f18;
pub const AFE_IRQ3_CNT_MON: u32 = 0x9f1c;
pub const AFE_IRQ4_CNT_MON: u32 = 0x9f20;
pub const AFE_IRQ5_CNT_MON: u32 = 0x9f24;
pub const AFE_IRQ6_CNT_MON: u32 = 0x9f28;
pub const AFE_IRQ7_CNT_MON: u32 = 0x9f2c;
pub const AFE_IRQ8_CNT_MON: u32 = 0x9f30;
pub const AFE_IRQ9_CNT_MON: u32 = 0x9f34;
pub const AFE_IRQ10_CNT_MON: u32 = 0x9f38;
pub const AFE_IRQ11_CNT_MON: u32 = 0x9f3c;
pub const AFE_IRQ12_CNT_MON: u32 = 0x9f40;
pub const AFE_IRQ13_CNT_MON: u32 = 0x9f44;
pub const AFE_IRQ14_CNT_MON: u32 = 0x9f48;
pub const AFE_IRQ15_CNT_MON: u32 = 0x9f4c;
pub const AFE_IRQ16_CNT_MON: u32 = 0x9f50;
pub const AFE_IRQ17_CNT_MON: u32 = 0x9f54;
pub const AFE_IRQ18_CNT_MON: u32 = 0x9f58;
pub const AFE_IRQ19_CNT_MON: u32 = 0x9f5c;
pub const AFE_IRQ20_CNT_MON: u32 = 0x9f60;
pub const AFE_IRQ21_CNT_MON: u32 = 0x9f64;
pub const AFE_IRQ22_CNT_MON: u32 = 0x9f68;
pub const AFE_IRQ23_CNT_MON: u32 = 0x9f6c;
pub const AFE_IRQ24_CNT_MON: u32 = 0x9f70;
pub const AFE_IRQ25_CNT_MON: u32 = 0x9f74;
pub const AFE_IRQ26_CNT_MON: u32 = 0x9f78;
pub const AFE_CUSTOM_IRQ0_CNT_MON: u32 = 0x9f90;
pub const AFE_CUSTOM_IRQ0_MCU_CFG1: u32 = 0x9fdc;
pub const AFE_IRQ_MCU_DSP3_EN: u32 = 0xa000;
pub const AFE_CUSTOM_IRQ_MCU_DSP3_EN: u32 = 0xa004;
pub const AFE_CUSTOM2_IRQ_MCU_EN: u32 = 0xa008;
pub const AFE_CUSTOM2_IRQ_MCU_DSP_EN: u32 = 0xa00c;
pub const AFE_CUSTOM2_IRQ_MCU_DSP2_EN: u32 = 0xa010;
pub const AFE_CUSTOM2_IRQ_MCU_DSP3_EN: u32 = 0xa014;
pub const AFE_CUSTOM2_IRQ_MCU_SCP_EN: u32 = 0xa018;
pub const AFE_IRQ_MCU_MON3: u32 = 0xa01c;
pub const AFE_IRQ_MCU_MON0: u32 = 0xa024;
pub const AFE_IRQ_MCU_MON1: u32 = 0xa028;
pub const AFE_IRQ_MCU_MON2: u32 = 0xa02c;
pub const AFE_CUSTOM2_IRQ_MISS_FLAG_MCU_MON: u32 = 0xa034;
pub const AFE_CUSTOM2_IRQ_DELAY_EN: u32 = 0xa038;
pub const AFE_CUSTOM2_IRQ_MCU_STATUS: u32 = 0xa03c;
pub const AFE_CUSTOM2_IRQ0_MCU_CFG0: u32 = 0xa040;
pub const AFE_CUSTOM2_IRQ0_MCU_CFG1: u32 = 0xa044;
pub const AFE_CUSTOM2_IRQ0_CNT_MON: u32 = 0xa048;
pub const AFE_CUSTOM2_IRQ0_MCU_DELAY_CNT_CFG0: u32 = 0xa04c;
pub const AFE_CUSTOM2_IRQ1_MCU_CFG0: u32 = 0xa050;
pub const AFE_CUSTOM2_IRQ1_MCU_CFG1: u32 = 0xa054;
pub const AFE_CUSTOM2_IRQ1_CNT_MON: u32 = 0xa058;
pub const AFE_CUSTOM2_IRQ1_MCU_DELAY_CNT_CFG0: u32 = 0xa05c;
pub const AFE_CUSTOM2_IRQ2_MCU_CFG0: u32 = 0xa060;
pub const AFE_CUSTOM2_IRQ2_MCU_CFG1: u32 = 0xa064;
pub const AFE_CUSTOM2_IRQ2_CNT_MON: u32 = 0xa068;
pub const AFE_CUSTOM2_IRQ2_MCU_DELAY_CNT_CFG0: u32 = 0xa06c;
pub const AFE_CUSTOM2_IRQ3_MCU_CFG0: u32 = 0xa070;
pub const AFE_CUSTOM2_IRQ3_MCU_CFG1: u32 = 0xa074;
pub const AFE_CUSTOM2_IRQ3_CNT_MON: u32 = 0xa078;
pub const AFE_CUSTOM2_IRQ3_MCU_DELAY_CNT_CFG0: u32 = 0xa07c;
pub const AFE_CUSTOM2_IRQ4_MCU_CFG0: u32 = 0xa080;
pub const AFE_CUSTOM2_IRQ4_MCU_CFG1: u32 = 0xa084;
pub const AFE_CUSTOM2_IRQ4_CNT_MON: u32 = 0xa088;
pub const AFE_CUSTOM2_IRQ4_MCU_DELAY_CNT_CFG0: u32 = 0xa08c;
pub const AFE_CUSTOM2_IRQ5_MCU_CFG0: u32 = 0xa090;
pub const AFE_CUSTOM2_IRQ5_MCU_CFG1: u32 = 0xa094;
pub const AFE_CUSTOM2_IRQ5_CNT_MON: u32 = 0xa098;
pub const AFE_CUSTOM2_IRQ5_MCU_DELAY_CNT_CFG0: u32 = 0xa09c;
pub const AFE_CUSTOM2_IRQ6_MCU_CFG0: u32 = 0xa0a0;
pub const AFE_CUSTOM2_IRQ6_MCU_CFG1: u32 = 0xa0a4;
pub const AFE_CUSTOM2_IRQ6_CNT_MON: u32 = 0xa0a8;
pub const AFE_CUSTOM2_IRQ6_MCU_DELAY_CNT_CFG0: u32 = 0xa0ac;
pub const AFE_CUSTOM2_IRQ7_MCU_CFG0: u32 = 0xa0b0;
pub const AFE_CUSTOM2_IRQ7_MCU_CFG1: u32 = 0xa0b4;
pub const AFE_CUSTOM2_IRQ7_CNT_MON: u32 = 0xa0b8;
pub const AFE_CUSTOM2_IRQ7_MCU_DELAY_CNT_CFG0: u32 = 0xa0bc;
pub const AFE_CUSTOM2_IRQ8_MCU_CFG0: u32 = 0xa0c0;
pub const AFE_CUSTOM2_IRQ8_MCU_CFG1: u32 = 0xa0c4;
pub const AFE_CUSTOM2_IRQ8_CNT_MON: u32 = 0xa0c8;
pub const AFE_CUSTOM2_IRQ8_MCU_DELAY_CNT_CFG0: u32 = 0xa0cc;
pub const AFE_CUSTOM2_IRQ9_MCU_CFG0: u32 = 0xa0d0;
pub const AFE_CUSTOM2_IRQ9_MCU_CFG1: u32 = 0xa0d4;
pub const AFE_CUSTOM2_IRQ9_CNT_MON: u32 = 0xa0d8;
pub const AFE_CUSTOM2_IRQ9_MCU_DELAY_CNT_CFG0: u32 = 0xa0dc;
pub const AFE_CUSTOM2_IRQ10_MCU_CFG0: u32 = 0xa0e0;
pub const AFE_CUSTOM2_IRQ10_MCU_CFG1: u32 = 0xa0e4;
pub const AFE_CUSTOM2_IRQ10_CNT_MON: u32 = 0xa0e8;
pub const AFE_CUSTOM2_IRQ10_MCU_DELAY_CNT_CFG0: u32 = 0xa0ec;
pub const AFE_CUSTOM2_IRQ11_MCU_CFG0: u32 = 0xa0f0;
pub const AFE_CUSTOM2_IRQ11_MCU_CFG1: u32 = 0xa0f4;
pub const AFE_CUSTOM2_IRQ11_CNT_MON: u32 = 0xa0f8;
pub const AFE_CUSTOM2_IRQ11_MCU_DELAY_CNT_CFG0: u32 = 0xa0fc;
pub const AFE_CUSTOM2_IRQ12_MCU_CFG0: u32 = 0xa100;
pub const AFE_CUSTOM2_IRQ12_MCU_CFG1: u32 = 0xa104;
pub const AFE_CUSTOM2_IRQ12_CNT_MON: u32 = 0xa108;
pub const AFE_CUSTOM2_IRQ12_MCU_DELAY_CNT_CFG0: u32 = 0xa10c;
pub const AFE_CUSTOM2_IRQ30_MCU_CFG0: u32 = 0xa220;
pub const AFE_CUSTOM2_IRQ30_MCU_CFG1: u32 = 0xa224;
pub const AFE_CUSTOM2_IRQ30_CNT_MON: u32 = 0xa228;
pub const AFE_CUSTOM2_IRQ30_MCU_DELAY_CNT_CFG0: u32 = 0xa22c;
pub const AFE_CUSTOM2_IRQ31_MCU_CFG0: u32 = 0xa230;
pub const AFE_CUSTOM2_IRQ31_MCU_CFG1: u32 = 0xa234;
pub const AFE_CUSTOM2_IRQ31_CNT_MON: u32 = 0xa238;
pub const AFE_CUSTOM2_IRQ31_MCU_DELAY_CNT_CFG0: u32 = 0xa23c;
pub const AFE_CUSTOM3_IRQ8_MCU_CFG0: u32 = 0xa2c0;
pub const AFE_CUSTOM3_IRQ8_MCU_CFG1: u32 = 0xa2c4;
pub const AFE_CUSTOM3_IRQ8_CNT_MON: u32 = 0xa2c8;
pub const AFE_CUSTOM3_IRQ8_MCU_DELAY_CNT_CFG0: u32 = 0xa2cc;
pub const AFE_CUSTOM3_IRQ9_MCU_CFG0: u32 = 0xa2d0;
pub const AFE_CUSTOM3_IRQ9_MCU_CFG1: u32 = 0xa2d4;
pub const AFE_CUSTOM3_IRQ9_CNT_MON: u32 = 0xa2d8;
pub const AFE_CUSTOM3_IRQ9_MCU_DELAY_CNT_CFG0: u32 = 0xa2dc;
pub const AFE_CUSTOM3_IRQ_MISS_FLAG_MCU_MON: u32 = 0xa440;
pub const AFE_CUSTOM3_IRQ_DELAY_EN: u32 = 0xa444;
pub const AFE_CUSTOM3_IRQ_MCU_STATUS: u32 = 0xa448;
pub const AFE_CUSTOM3_IRQ_MCU_EN: u32 = 0xa44c;
pub const AFE_CUSTOM3_IRQ_MCU_DSP_EN: u32 = 0xa450;
pub const AFE_CUSTOM3_IRQ_MCU_DSP2_EN: u32 = 0xa454;
pub const AFE_CUSTOM3_IRQ_MCU_DSP3_EN: u32 = 0xa458;
pub const AFE_CUSTOM3_IRQ_MCU_DSP_WLA_EN: u32 = 0xa45c;
pub const AFE_CUSTOM3_IRQ_MCU_SCP_EN: u32 = 0xa460;
pub const AFE_CUSTOM2_IRQ_MCU_DSP_WLA_EN: u32 = 0xa464;
pub const AFE_IRQ_MCU_DSP_WLA_EN: u32 = 0xa468;
pub const AFE_COMMON2_IRQ_MCU_STATUS: u32 = 0xa46c;
pub const AFE_COMMON2_IRQ_MCU_EN: u32 = 0xa470;
pub const AFE_COMMON2_IRQ_MCU_DSP_EN: u32 = 0xa474;
pub const AFE_COMMON2_IRQ_MCU_DSP2_EN: u32 = 0xa478;
pub const AFE_COMMON2_IRQ_MCU_DSP3_EN: u32 = 0xa47c;
pub const AFE_COMMON2_IRQ_MCU_DSP_WLA_EN: u32 = 0xa480;
pub const AFE_COMMON2_IRQ_MCU_SCP_EN: u32 = 0xa484;
pub const AFE_CUSTOM_IRQ_MCU_DSP_WLA_EN: u32 = 0xa508;

pub const AFE_MAX_REGISTER: u32 = AFE_CUSTOM_IRQ_MCU_DSP_WLA_EN;

pub const AFE_IRQ_STATUS_BITS: u32 = 0x7FFFFFF;
pub const AFE_IRQ_CNT_SHIFT: u32 = 0;
pub const AFE_IRQ_CNT_MASK: u32 = 0xffffff;


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
