/* SPDX-License-Identifier: GPL-2.0 */
pub const RT5631_RESET: u16 = 0x00;
pub const RT5631_SPK_OUT_VOL: u16 = 0x02;
pub const RT5631_HP_OUT_VOL: u16 = 0x04;
pub const RT5631_MONO_AXO_1_2_VOL: u16 = 0x06;
pub const RT5631_AUX_IN_VOL: u16 = 0x0a;
pub const RT5631_STEREO_DAC_VOL_1: u16 = 0x0c;
pub const RT5631_MIC_CTRL_1: u16 = 0x0e;
pub const RT5631_STEREO_DAC_VOL_2: u16 = 0x10;
pub const RT5631_ADC_CTRL_1: u16 = 0x12;
pub const RT5631_ADC_REC_MIXER: u16 = 0x14;
pub const RT5631_ADC_CTRL_2: u16 = 0x16;
pub const RT5631_VDAC_DIG_VOL: u16 = 0x18;
pub const RT5631_OUTMIXER_L_CTRL: u16 = 0x1a;
pub const RT5631_OUTMIXER_R_CTRL: u16 = 0x1c;
pub const RT5631_AXO1MIXER_CTRL: u16 = 0x1e;
pub const RT5631_AXO2MIXER_CTRL: u16 = 0x20;
pub const RT5631_MIC_CTRL_2: u16 = 0x22;
pub const RT5631_DIG_MIC_CTRL: u16 = 0x24;
pub const RT5631_MONO_INPUT_VOL: u16 = 0x26;
pub const RT5631_SPK_MIXER_CTRL: u16 = 0x28;
pub const RT5631_SPK_MONO_OUT_CTRL: u16 = 0x2a;
pub const RT5631_SPK_MONO_HP_OUT_CTRL: u16 = 0x2c;
pub const RT5631_SDP_CTRL: u16 = 0x34;
pub const RT5631_MONO_SDP_CTRL: u16 = 0x36;
pub const RT5631_STEREO_AD_DA_CLK_CTRL: u16 = 0x38;
pub const RT5631_PWR_MANAG_ADD1: u16 = 0x3a;
pub const RT5631_PWR_MANAG_ADD2: u16 = 0x3b;
pub const RT5631_PWR_MANAG_ADD3: u16 = 0x3c;
pub const RT5631_PWR_MANAG_ADD4: u16 = 0x3e;
pub const RT5631_GEN_PUR_CTRL_REG: u16 = 0x40;
pub const RT5631_GLOBAL_CLK_CTRL: u16 = 0x42;
pub const RT5631_PLL_CTRL: u16 = 0x44;
pub const RT5631_INT_ST_IRQ_CTRL_1: u16 = 0x48;
pub const RT5631_INT_ST_IRQ_CTRL_2: u16 = 0x4a;
pub const RT5631_GPIO_CTRL: u16 = 0x4c;
pub const RT5631_MISC_CTRL: u16 = 0x52;
pub const RT5631_DEPOP_FUN_CTRL_1: u16 = 0x54;
pub const RT5631_DEPOP_FUN_CTRL_2: u16 = 0x56;
pub const RT5631_JACK_DET_CTRL: u16 = 0x5a;
pub const RT5631_SOFT_VOL_CTRL: u16 = 0x5c;
pub const RT5631_ALC_CTRL_1: u16 = 0x64;
pub const RT5631_ALC_CTRL_2: u16 = 0x65;
pub const RT5631_ALC_CTRL_3: u16 = 0x66;
pub const RT5631_PSEUDO_SPATL_CTRL: u16 = 0x68;
pub const RT5631_INDEX_ADD: u16 = 0x6a;
pub const RT5631_INDEX_DATA: u16 = 0x6c;
pub const RT5631_EQ_CTRL: u16 = 0x6e;
pub const RT5631_VENDOR_ID: u16 = 0x7a;
pub const RT5631_VENDOR_ID1: u16 = 0x7c;
pub const RT5631_VENDOR_ID2: u16 = 0x7e;
/* Index of Codec Private Register definition */
pub const RT5631_EQ_BW_LOP: u16 = 0x00;
pub const RT5631_EQ_GAIN_LOP: u16 = 0x01;
pub const RT5631_EQ_FC_BP1: u16 = 0x02;
pub const RT5631_EQ_BW_BP1: u16 = 0x03;
pub const RT5631_EQ_GAIN_BP1: u16 = 0x04;
pub const RT5631_EQ_FC_BP2: u16 = 0x05;
pub const RT5631_EQ_BW_BP2: u16 = 0x06;
pub const RT5631_EQ_GAIN_BP2: u16 = 0x07;
pub const RT5631_EQ_FC_BP3: u16 = 0x08;
pub const RT5631_EQ_BW_BP3: u16 = 0x09;
pub const RT5631_EQ_GAIN_BP3: u16 = 0x0a;
pub const RT5631_EQ_BW_HIP: u16 = 0x0b;
pub const RT5631_EQ_GAIN_HIP: u16 = 0x0c;
pub const RT5631_EQ_HPF_A1: u16 = 0x0d;
pub const RT5631_EQ_HPF_A2: u16 = 0x0e;
pub const RT5631_EQ_HPF_GAIN: u16 = 0x0f;
pub const RT5631_EQ_PRE_VOL_CTRL: u16 = 0x11;
pub const RT5631_EQ_POST_VOL_CTRL: u16 = 0x12;
pub const RT5631_TEST_MODE_CTRL: u16 = 0x39;
pub const RT5631_CP_INTL_REG2: u16 = 0x45;
pub const RT5631_ADDA_MIXER_INTL_REG3: u16 = 0x52;
pub const RT5631_SPK_INTL_CTRL: u16 = 0x56;
/* global definition */
pub const RT5631_L_MUTE: u16 = 0x1 << 15;
pub const RT5631_L_MUTE_SHIFT: u16 = 15;
pub const RT5631_L_EN: u16 = 0x1 << 14;
pub const RT5631_L_EN_SHIFT: u16 = 14;
pub const RT5631_R_MUTE: u16 = 0x1 << 7;
pub const RT5631_R_MUTE_SHIFT: u16 = 7;
pub const RT5631_R_EN: u16 = 0x1 << 6;
pub const RT5631_R_EN_SHIFT: u16 = 6;
pub const RT5631_VOL_MASK: u16 = 0x1f;
pub const RT5631_L_VOL_SHIFT: u16 = 8;
pub const RT5631_R_VOL_SHIFT: u16 = 0;
/* Speaker Output Control(0x02) */
pub const RT5631_SPK_L_VOL_SEL_MASK: u16 = 0x1 << 14;
pub const RT5631_SPK_L_VOL_SEL_VMID: u16 = 0x0 << 14;
pub const RT5631_SPK_L_VOL_SEL_SPKMIX_L: u16 = 0x1 << 14;
pub const RT5631_SPK_R_VOL_SEL_MASK: u16 = 0x1 << 6;
pub const RT5631_SPK_R_VOL_SEL_VMID: u16 = 0x0 << 6;
pub const RT5631_SPK_R_VOL_SEL_SPKMIX_R: u16 = 0x1 << 6;
/* Headphone Output Control(0x04) */
pub const RT5631_HP_L_VOL_SEL_MASK: u16 = 0x1 << 14;
pub const RT5631_HP_L_VOL_SEL_VMID: u16 = 0x0 << 14;
pub const RT5631_HP_L_VOL_SEL_OUTMIX_L: u16 = 0x1 << 14;
pub const RT5631_HP_R_VOL_SEL_MASK: u16 = 0x1 << 6;
pub const RT5631_HP_R_VOL_SEL_VMID: u16 = 0x0 << 6;
pub const RT5631_HP_R_VOL_SEL_OUTMIX_R: u16 = 0x1 << 6;
/* Output Control for AUXOUT/MONO(0x06) */
pub const RT5631_AUXOUT_1_VOL_SEL_MASK: u16 = 0x1 << 14;
pub const RT5631_AUXOUT_1_VOL_SEL_VMID: u16 = 0x0 << 14;
pub const RT5631_AUXOUT_1_VOL_SEL_OUTMIX_L: u16 = 0x1 << 14;
pub const RT5631_MUTE_MONO: u16 = 0x1 << 13;
pub const RT5631_MUTE_MONO_SHIFT: u16 = 13;
pub const RT5631_AUXOUT_2_VOL_SEL_MASK: u16 = 0x1 << 6;
pub const RT5631_AUXOUT_2_VOL_SEL_VMID: u16 = 0x0 << 6;
pub const RT5631_AUXOUT_2_VOL_SEL_OUTMIX_R: u16 = 0x1 << 6;
/* Microphone Input Control 1(0x0E) */
pub const RT5631_MIC1_DIFF_INPUT_CTRL: u16 = 0x1 << 15;
pub const RT5631_MIC1_DIFF_INPUT_SHIFT: u16 = 15;
pub const RT5631_MIC2_DIFF_INPUT_CTRL: u16 = 0x1 << 7;
pub const RT5631_MIC2_DIFF_INPUT_SHIFT: u16 = 7;
/* Stereo DAC Digital Volume2(0x10) */
pub const RT5631_DAC_VOL_MASK: u16 = 0xff;
/* ADC Recording Mixer Control(0x14) */
pub const RT5631_M_OUTMIXER_L_TO_RECMIXER_L: u16 = 0x1 << 15;
pub const RT5631_M_OUTMIXL_RECMIXL_BIT: u16 = 15;
pub const RT5631_M_MIC1_TO_RECMIXER_L: u16 = 0x1 << 14;
pub const RT5631_M_MIC1_RECMIXL_BIT: u16 = 14;
pub const RT5631_M_AXIL_TO_RECMIXER_L: u16 = 0x1 << 13;
pub const RT5631_M_AXIL_RECMIXL_BIT: u16 = 13;
pub const RT5631_M_MONO_IN_TO_RECMIXER_L: u16 = 0x1 << 12;
pub const RT5631_M_MONO_IN_RECMIXL_BIT: u16 = 12;
pub const RT5631_M_OUTMIXER_R_TO_RECMIXER_R: u16 = 0x1 << 7;
pub const RT5631_M_OUTMIXR_RECMIXR_BIT: u16 = 7;
pub const RT5631_M_MIC2_TO_RECMIXER_R: u16 = 0x1 << 6;
pub const RT5631_M_MIC2_RECMIXR_BIT: u16 = 6;
pub const RT5631_M_AXIR_TO_RECMIXER_R: u16 = 0x1 << 5;
pub const RT5631_M_AXIR_RECMIXR_BIT: u16 = 5;
pub const RT5631_M_MONO_IN_TO_RECMIXER_R: u16 = 0x1 << 4;
pub const RT5631_M_MONO_IN_RECMIXR_BIT: u16 = 4;
/* Left Output Mixer Control(0x1A) */
pub const RT5631_M_RECMIXER_L_TO_OUTMIXER_L: u16 = 0x1 << 15;
pub const RT5631_M_RECMIXL_OUTMIXL_BIT: u16 = 15;
pub const RT5631_M_RECMIXER_R_TO_OUTMIXER_L: u16 = 0x1 << 14;
pub const RT5631_M_RECMIXR_OUTMIXL_BIT: u16 = 14;
pub const RT5631_M_DAC_L_TO_OUTMIXER_L: u16 = 0x1 << 13;
pub const RT5631_M_DACL_OUTMIXL_BIT: u16 = 13;
pub const RT5631_M_MIC1_TO_OUTMIXER_L: u16 = 0x1 << 12;
pub const RT5631_M_MIC1_OUTMIXL_BIT: u16 = 12;
pub const RT5631_M_MIC2_TO_OUTMIXER_L: u16 = 0x1 << 11;
pub const RT5631_M_MIC2_OUTMIXL_BIT: u16 = 11;
pub const RT5631_M_MONO_IN_P_TO_OUTMIXER_L: u16 = 0x1 << 10;
pub const RT5631_M_MONO_INP_OUTMIXL_BIT: u16 = 10;
pub const RT5631_M_AXIL_TO_OUTMIXER_L: u16 = 0x1 << 9;
pub const RT5631_M_AXIL_OUTMIXL_BIT: u16 = 9;
pub const RT5631_M_AXIR_TO_OUTMIXER_L: u16 = 0x1 << 8;
pub const RT5631_M_AXIR_OUTMIXL_BIT: u16 = 8;
pub const RT5631_M_VDAC_TO_OUTMIXER_L: u16 = 0x1 << 7;
pub const RT5631_M_VDAC_OUTMIXL_BIT: u16 = 7;
/* Right Output Mixer Control(0x1C) */
pub const RT5631_M_RECMIXER_L_TO_OUTMIXER_R: u16 = 0x1 << 15;
pub const RT5631_M_RECMIXL_OUTMIXR_BIT: u16 = 15;
pub const RT5631_M_RECMIXER_R_TO_OUTMIXER_R: u16 = 0x1 << 14;
pub const RT5631_M_RECMIXR_OUTMIXR_BIT: u16 = 14;
pub const RT5631_M_DAC_R_TO_OUTMIXER_R: u16 = 0x1 << 13;
pub const RT5631_M_DACR_OUTMIXR_BIT: u16 = 13;
pub const RT5631_M_MIC1_TO_OUTMIXER_R: u16 = 0x1 << 12;
pub const RT5631_M_MIC1_OUTMIXR_BIT: u16 = 12;
pub const RT5631_M_MIC2_TO_OUTMIXER_R: u16 = 0x1 << 11;
pub const RT5631_M_MIC2_OUTMIXR_BIT: u16 = 11;
pub const RT5631_M_MONO_IN_N_TO_OUTMIXER_R: u16 = 0x1 << 10;
pub const RT5631_M_MONO_INN_OUTMIXR_BIT: u16 = 10;
pub const RT5631_M_AXIL_TO_OUTMIXER_R: u16 = 0x1 << 9;
pub const RT5631_M_AXIL_OUTMIXR_BIT: u16 = 9;
pub const RT5631_M_AXIR_TO_OUTMIXER_R: u16 = 0x1 << 8;
pub const RT5631_M_AXIR_OUTMIXR_BIT: u16 = 8;
pub const RT5631_M_VDAC_TO_OUTMIXER_R: u16 = 0x1 << 7;
pub const RT5631_M_VDAC_OUTMIXR_BIT: u16 = 7;
/* Lout Mixer Control(0x1E) */
pub const RT5631_M_MIC1_TO_AXO1MIXER: u16 = 0x1 << 15;
pub const RT5631_M_MIC1_AXO1MIX_BIT: u16 = 15;
pub const RT5631_M_MIC2_TO_AXO1MIXER: u16 = 0x1 << 11;
pub const RT5631_M_MIC2_AXO1MIX_BIT: u16 = 11;
pub const RT5631_M_OUTMIXER_L_TO_AXO1MIXER: u16 = 0x1 << 7;
pub const RT5631_M_OUTMIXL_AXO1MIX_BIT: u16 = 7;
pub const RT5631_M_OUTMIXER_R_TO_AXO1MIXER: u16 = 0x1 << 6;
pub const RT5631_M_OUTMIXR_AXO1MIX_BIT: u16 = 6;
/* Rout Mixer Control(0x20) */
pub const RT5631_M_MIC1_TO_AXO2MIXER: u16 = 0x1 << 15;
pub const RT5631_M_MIC1_AXO2MIX_BIT: u16 = 15;
pub const RT5631_M_MIC2_TO_AXO2MIXER: u16 = 0x1 << 11;
pub const RT5631_M_MIC2_AXO2MIX_BIT: u16 = 11;
pub const RT5631_M_OUTMIXER_L_TO_AXO2MIXER: u16 = 0x1 << 7;
pub const RT5631_M_OUTMIXL_AXO2MIX_BIT: u16 = 7;
pub const RT5631_M_OUTMIXER_R_TO_AXO2MIXER: u16 = 0x1 << 6;
pub const RT5631_M_OUTMIXR_AXO2MIX_BIT: u16 = 6;
/* Micphone Input Control 2(0x22) */
pub const RT5631_MIC_BIAS_90_PRECNET_AVDD: u16 = 1;
pub const RT5631_MIC_BIAS_75_PRECNET_AVDD: u16 = 2;
pub const RT5631_MIC1_BOOST_CTRL_MASK: u16 = 0xf << 12;
pub const RT5631_MIC1_BOOST_CTRL_BYPASS: u16 = 0x0 << 12;
pub const RT5631_MIC1_BOOST_CTRL_20DB: u16 = 0x1 << 12;
pub const RT5631_MIC1_BOOST_CTRL_24DB: u16 = 0x2 << 12;
pub const RT5631_MIC1_BOOST_CTRL_30DB: u16 = 0x3 << 12;
pub const RT5631_MIC1_BOOST_CTRL_35DB: u16 = 0x4 << 12;
pub const RT5631_MIC1_BOOST_CTRL_40DB: u16 = 0x5 << 12;
pub const RT5631_MIC1_BOOST_CTRL_34DB: u16 = 0x6 << 12;
pub const RT5631_MIC1_BOOST_CTRL_50DB: u16 = 0x7 << 12;
pub const RT5631_MIC1_BOOST_CTRL_52DB: u16 = 0x8 << 12;
pub const RT5631_MIC1_BOOST_SHIFT: u16 = 12;
pub const RT5631_MIC2_BOOST_CTRL_MASK: u16 = 0xf << 8;
pub const RT5631_MIC2_BOOST_CTRL_BYPASS: u16 = 0x0 << 8;
pub const RT5631_MIC2_BOOST_CTRL_20DB: u16 = 0x1 << 8;
pub const RT5631_MIC2_BOOST_CTRL_24DB: u16 = 0x2 << 8;
pub const RT5631_MIC2_BOOST_CTRL_30DB: u16 = 0x3 << 8;
pub const RT5631_MIC2_BOOST_CTRL_35DB: u16 = 0x4 << 8;
pub const RT5631_MIC2_BOOST_CTRL_40DB: u16 = 0x5 << 8;
pub const RT5631_MIC2_BOOST_CTRL_34DB: u16 = 0x6 << 8;
pub const RT5631_MIC2_BOOST_CTRL_50DB: u16 = 0x7 << 8;
pub const RT5631_MIC2_BOOST_CTRL_52DB: u16 = 0x8 << 8;
pub const RT5631_MIC2_BOOST_SHIFT: u16 = 8;
pub const RT5631_MICBIAS1_VOLT_CTRL_MASK: u16 = 0x1 << 7;
pub const RT5631_MICBIAS1_VOLT_CTRL_90P: u16 = 0x0 << 7;
pub const RT5631_MICBIAS1_VOLT_CTRL_75P: u16 = 0x1 << 7;
pub const RT5631_MICBIAS1_S_C_DET_MASK: u16 = 0x1 << 6;
pub const RT5631_MICBIAS1_S_C_DET_DIS: u16 = 0x0 << 6;
pub const RT5631_MICBIAS1_S_C_DET_ENA: u16 = 0x1 << 6;
pub const RT5631_MICBIAS1_SHORT_CURR_DET_MASK: u16 = 0x3 << 4;
pub const RT5631_MICBIAS1_SHORT_CURR_DET_600UA: u16 = 0x0 << 4;
pub const RT5631_MICBIAS1_SHORT_CURR_DET_1500UA: u16 = 0x1 << 4;
pub const RT5631_MICBIAS1_SHORT_CURR_DET_2000UA: u16 = 0x2 << 4;
pub const RT5631_MICBIAS2_VOLT_CTRL_MASK: u16 = 0x1 << 3;
pub const RT5631_MICBIAS2_VOLT_CTRL_90P: u16 = 0x0 << 3;
pub const RT5631_MICBIAS2_VOLT_CTRL_75P: u16 = 0x1 << 3;
pub const RT5631_MICBIAS2_S_C_DET_MASK: u16 = 0x1 << 2;
pub const RT5631_MICBIAS2_S_C_DET_DIS: u16 = 0x0 << 2;
pub const RT5631_MICBIAS2_S_C_DET_ENA: u16 = 0x1 << 2;
pub const RT5631_MICBIAS2_SHORT_CURR_DET_MASK: u16 = 0x3;
pub const RT5631_MICBIAS2_SHORT_CURR_DET_600UA: u16 = 0x0;
pub const RT5631_MICBIAS2_SHORT_CURR_DET_1500UA: u16 = 0x1;
pub const RT5631_MICBIAS2_SHORT_CURR_DET_2000UA: u16 = 0x2;
/* Digital Microphone Control(0x24) */
pub const RT5631_DMIC_ENA_MASK: u16 = 0x1 << 15;
pub const RT5631_DMIC_ENA_SHIFT: u16 = 15;
/* DMIC_ENA: DMIC to ADC Digital filter */
pub const RT5631_DMIC_ENA: u16 = 0x1 << 15;
/* DMIC_DIS: ADC mixer to ADC Digital filter */
pub const RT5631_DMIC_DIS: u16 = 0x0 << 15;
pub const RT5631_DMIC_L_CH_MUTE: u16 = 0x1 << 13;
pub const RT5631_DMIC_L_CH_MUTE_SHIFT: u16 = 13;
pub const RT5631_DMIC_R_CH_MUTE: u16 = 0x1 << 12;
pub const RT5631_DMIC_R_CH_MUTE_SHIFT: u16 = 12;
pub const RT5631_DMIC_L_CH_LATCH_MASK: u16 = 0x1 << 9;
pub const RT5631_DMIC_L_CH_LATCH_RISING: u16 = 0x1 << 9;
pub const RT5631_DMIC_L_CH_LATCH_FALLING: u16 = 0x0 << 9;
pub const RT5631_DMIC_R_CH_LATCH_MASK: u16 = 0x1 << 8;
pub const RT5631_DMIC_R_CH_LATCH_RISING: u16 = 0x1 << 8;
pub const RT5631_DMIC_R_CH_LATCH_FALLING: u16 = 0x0 << 8;
pub const RT5631_DMIC_CLK_CTRL_MASK: u16 = 0x3 << 4;
pub const RT5631_DMIC_CLK_CTRL_TO_128FS: u16 = 0x0 << 4;
pub const RT5631_DMIC_CLK_CTRL_TO_64FS: u16 = 0x1 << 4;
pub const RT5631_DMIC_CLK_CTRL_TO_32FS: u16 = 0x2 << 4;
/* Microphone Input Volume(0x26) */
pub const RT5631_MONO_DIFF_INPUT_SHIFT: u16 = 15;
/* Speaker Mixer Control(0x28) */
pub const RT5631_M_RECMIXER_L_TO_SPKMIXER_L: u16 = 0x1 << 15;
pub const RT5631_M_RECMIXL_SPKMIXL_BIT: u16 = 15;
pub const RT5631_M_MIC1_P_TO_SPKMIXER_L: u16 = 0x1 << 14;
pub const RT5631_M_MIC1P_SPKMIXL_BIT: u16 = 14;
pub const RT5631_M_DAC_L_TO_SPKMIXER_L: u16 = 0x1 << 13;
pub const RT5631_M_DACL_SPKMIXL_BIT: u16 = 13;
pub const RT5631_M_OUTMIXER_L_TO_SPKMIXER_L: u16 = 0x1 << 12;
pub const RT5631_M_OUTMIXL_SPKMIXL_BIT: u16 = 12;
pub const RT5631_M_RECMIXER_R_TO_SPKMIXER_R: u16 = 0x1 << 7;
pub const RT5631_M_RECMIXR_SPKMIXR_BIT: u16 = 7;
pub const RT5631_M_MIC2_P_TO_SPKMIXER_R: u16 = 0x1 << 6;
pub const RT5631_M_MIC2P_SPKMIXR_BIT: u16 = 6;
pub const RT5631_M_DAC_R_TO_SPKMIXER_R: u16 = 0x1 << 5;
pub const RT5631_M_DACR_SPKMIXR_BIT: u16 = 5;
pub const RT5631_M_OUTMIXER_R_TO_SPKMIXER_R: u16 = 0x1 << 4;
pub const RT5631_M_OUTMIXR_SPKMIXR_BIT: u16 = 4;
/* Speaker/Mono Output Control(0x2A) */
pub const RT5631_M_SPKVOL_L_TO_SPOL_MIXER: u16 = 0x1 << 15;
pub const RT5631_M_SPKVOLL_SPOLMIX_BIT: u16 = 15;
pub const RT5631_M_SPKVOL_R_TO_SPOL_MIXER: u16 = 0x1 << 14;
pub const RT5631_M_SPKVOLR_SPOLMIX_BIT: u16 = 14;
pub const RT5631_M_SPKVOL_L_TO_SPOR_MIXER: u16 = 0x1 << 13;
pub const RT5631_M_SPKVOLL_SPORMIX_BIT: u16 = 13;
pub const RT5631_M_SPKVOL_R_TO_SPOR_MIXER: u16 = 0x1 << 12;
pub const RT5631_M_SPKVOLR_SPORMIX_BIT: u16 = 12;
pub const RT5631_M_OUTVOL_L_TO_MONOMIXER: u16 = 0x1 << 11;
pub const RT5631_M_OUTVOLL_MONOMIX_BIT: u16 = 11;
pub const RT5631_M_OUTVOL_R_TO_MONOMIXER: u16 = 0x1 << 10;
pub const RT5631_M_OUTVOLR_MONOMIX_BIT: u16 = 10;
/* Speaker/Mono/HP Output Control(0x2C) */
pub const RT5631_SPK_L_MUX_SEL_MASK: u16 = 0x3 << 14;
pub const RT5631_SPK_L_MUX_SEL_SPKMIXER_L: u16 = 0x0 << 14;
pub const RT5631_SPK_L_MUX_SEL_MONO_IN: u16 = 0x1 << 14;
pub const RT5631_SPK_L_MUX_SEL_DAC_L: u16 = 0x3 << 14;
pub const RT5631_SPK_L_MUX_SEL_SHIFT: u16 = 14;
pub const RT5631_SPK_R_MUX_SEL_MASK: u16 = 0x3 << 10;
pub const RT5631_SPK_R_MUX_SEL_SPKMIXER_R: u16 = 0x0 << 10;
pub const RT5631_SPK_R_MUX_SEL_MONO_IN: u16 = 0x1 << 10;
pub const RT5631_SPK_R_MUX_SEL_DAC_R: u16 = 0x3 << 10;
pub const RT5631_SPK_R_MUX_SEL_SHIFT: u16 = 10;
pub const RT5631_MONO_MUX_SEL_MASK: u16 = 0x3 << 6;
pub const RT5631_MONO_MUX_SEL_MONOMIXER: u16 = 0x0 << 6;
pub const RT5631_MONO_MUX_SEL_MONO_IN: u16 = 0x1 << 6;
pub const RT5631_MONO_MUX_SEL_SHIFT: u16 = 6;
pub const RT5631_HP_L_MUX_SEL_MASK: u16 = 0x1 << 3;
pub const RT5631_HP_L_MUX_SEL_HPVOL_L: u16 = 0x0 << 3;
pub const RT5631_HP_L_MUX_SEL_DAC_L: u16 = 0x1 << 3;
pub const RT5631_HP_L_MUX_SEL_SHIFT: u16 = 3;
pub const RT5631_HP_R_MUX_SEL_MASK: u16 = 0x1 << 2;
pub const RT5631_HP_R_MUX_SEL_HPVOL_R: u16 = 0x0 << 2;
pub const RT5631_HP_R_MUX_SEL_DAC_R: u16 = 0x1 << 2;
pub const RT5631_HP_R_MUX_SEL_SHIFT: u16 = 2;
/* Stereo I2S Serial Data Port Control(0x34) */
pub const RT5631_SDP_MODE_SEL_MASK: u16 = 0x1 << 15;
pub const RT5631_SDP_MODE_SEL_MASTER: u16 = 0x0 << 15;
pub const RT5631_SDP_MODE_SEL_SLAVE: u16 = 0x1 << 15;
pub const RT5631_SDP_ADC_CPS_SEL_MASK: u16 = 0x3 << 10;
pub const RT5631_SDP_ADC_CPS_SEL_OFF: u16 = 0x0 << 10;
pub const RT5631_SDP_ADC_CPS_SEL_U_LAW: u16 = 0x1 << 10;
pub const RT5631_SDP_ADC_CPS_SEL_A_LAW: u16 = 0x2 << 10;
pub const RT5631_SDP_DAC_CPS_SEL_MASK: u16 = 0x3 << 8;
pub const RT5631_SDP_DAC_CPS_SEL_OFF: u16 = 0x0 << 8;
pub const RT5631_SDP_DAC_CPS_SEL_U_LAW: u16 = 0x1 << 8;
pub const RT5631_SDP_DAC_CPS_SEL_A_LAW: u16 = 0x2 << 8;
/* 0:Normal 1:Invert */
pub const RT5631_SDP_I2S_BCLK_POL_CTRL: u16 = 0x1 << 7;
/* 0:Normal 1:Invert */
pub const RT5631_SDP_DAC_R_INV: u16 = 0x1 << 6;
/* 0:ADC data appear at left phase of LRCK
 * 1:ADC data appear at right phase of LRCK
 */
pub const RT5631_SDP_ADC_DATA_L_R_SWAP: u16 = 0x1 << 5;
/* 0:DAC data appear at left phase of LRCK
 * 1:DAC data appear at right phase of LRCK
 */
pub const RT5631_SDP_DAC_DATA_L_R_SWAP: u16 = 0x1 << 4;
/* Data Length Slection */
pub const RT5631_SDP_I2S_DL_MASK: u16 = 0x3 << 2;
pub const RT5631_SDP_I2S_DL_16: u16 = 0x0 << 2;
pub const RT5631_SDP_I2S_DL_20: u16 = 0x1 << 2;
pub const RT5631_SDP_I2S_DL_24: u16 = 0x2 << 2;
pub const RT5631_SDP_I2S_DL_8: u16 = 0x3 << 2;
/* PCM Data Format Selection */
pub const RT5631_SDP_I2S_DF_MASK: u16 = 0x3;
pub const RT5631_SDP_I2S_DF_I2S: u16 = 0x0;
pub const RT5631_SDP_I2S_DF_LEFT: u16 = 0x1;
pub const RT5631_SDP_I2S_DF_PCM_A: u16 = 0x2;
pub const RT5631_SDP_I2S_DF_PCM_B: u16 = 0x3;
/* Stereo AD/DA Clock Control(0x38h) */
pub const RT5631_I2S_PRE_DIV_MASK: u16 = 0x7 << 13;
pub const RT5631_I2S_PRE_DIV_1: u16 = 0x0 << 13;
pub const RT5631_I2S_PRE_DIV_2: u16 = 0x1 << 13;
pub const RT5631_I2S_PRE_DIV_4: u16 = 0x2 << 13;
pub const RT5631_I2S_PRE_DIV_8: u16 = 0x3 << 13;
pub const RT5631_I2S_PRE_DIV_16: u16 = 0x4 << 13;
pub const RT5631_I2S_PRE_DIV_32: u16 = 0x5 << 13;
/* CLOCK RELATIVE OF BCLK AND LCRK */
pub const RT5631_I2S_LRCK_SEL_N_BCLK_MASK: u16 = 0x1 << 12;
pub const RT5631_I2S_LRCK_SEL_64_BCLK: u16 = 0x0 << 12 /* 64FS */;
pub const RT5631_I2S_LRCK_SEL_32_BCLK: u16 = 0x1 << 12 /* 32FS */;
pub const RT5631_DAC_OSR_SEL_MASK: u16 = 0x3 << 10;
pub const RT5631_DAC_OSR_SEL_128FS: u16 = 0x3 << 10;
pub const RT5631_DAC_OSR_SEL_64FS: u16 = 0x3 << 10;
pub const RT5631_DAC_OSR_SEL_32FS: u16 = 0x3 << 10;
pub const RT5631_DAC_OSR_SEL_16FS: u16 = 0x3 << 10;
pub const RT5631_ADC_OSR_SEL_MASK: u16 = 0x3 << 8;
pub const RT5631_ADC_OSR_SEL_128FS: u16 = 0x3 << 8;
pub const RT5631_ADC_OSR_SEL_64FS: u16 = 0x3 << 8;
pub const RT5631_ADC_OSR_SEL_32FS: u16 = 0x3 << 8;
pub const RT5631_ADC_OSR_SEL_16FS: u16 = 0x3 << 8;
pub const RT5631_ADDA_FILTER_CLK_SEL_256FS: u16 = 0 << 7 /* 256FS */;
pub const RT5631_ADDA_FILTER_CLK_SEL_384FS: u16 = 1 << 7 /* 384FS */;
/* Power managment addition 1 (0x3A) */
pub const RT5631_PWR_MAIN_I2S_EN: u16 = 0x1 << 15;
pub const RT5631_PWR_MAIN_I2S_BIT: u16 = 15;
pub const RT5631_PWR_CLASS_D: u16 = 0x1 << 12;
pub const RT5631_PWR_CLASS_D_BIT: u16 = 12;
pub const RT5631_PWR_ADC_L_CLK: u16 = 0x1 << 11;
pub const RT5631_PWR_ADC_L_CLK_BIT: u16 = 11;
pub const RT5631_PWR_ADC_R_CLK: u16 = 0x1 << 10;
pub const RT5631_PWR_ADC_R_CLK_BIT: u16 = 10;
pub const RT5631_PWR_DAC_L_CLK: u16 = 0x1 << 9;
pub const RT5631_PWR_DAC_L_CLK_BIT: u16 = 9;
pub const RT5631_PWR_DAC_R_CLK: u16 = 0x1 << 8;
pub const RT5631_PWR_DAC_R_CLK_BIT: u16 = 8;
pub const RT5631_PWR_DAC_REF: u16 = 0x1 << 7;
pub const RT5631_PWR_DAC_REF_BIT: u16 = 7;
pub const RT5631_PWR_DAC_L_TO_MIXER: u16 = 0x1 << 6;
pub const RT5631_PWR_DAC_L_TO_MIXER_BIT: u16 = 6;
pub const RT5631_PWR_DAC_R_TO_MIXER: u16 = 0x1 << 5;
pub const RT5631_PWR_DAC_R_TO_MIXER_BIT: u16 = 5;
/* Power managment addition 2 (0x3B) */
pub const RT5631_PWR_OUTMIXER_L: u16 = 0x1 << 15;
pub const RT5631_PWR_OUTMIXER_L_BIT: u16 = 15;
pub const RT5631_PWR_OUTMIXER_R: u16 = 0x1 << 14;
pub const RT5631_PWR_OUTMIXER_R_BIT: u16 = 14;
pub const RT5631_PWR_SPKMIXER_L: u16 = 0x1 << 13;
pub const RT5631_PWR_SPKMIXER_L_BIT: u16 = 13;
pub const RT5631_PWR_SPKMIXER_R: u16 = 0x1 << 12;
pub const RT5631_PWR_SPKMIXER_R_BIT: u16 = 12;
pub const RT5631_PWR_RECMIXER_L: u16 = 0x1 << 11;
pub const RT5631_PWR_RECMIXER_L_BIT: u16 = 11;
pub const RT5631_PWR_RECMIXER_R: u16 = 0x1 << 10;
pub const RT5631_PWR_RECMIXER_R_BIT: u16 = 10;
pub const RT5631_PWR_MIC1_BOOT_GAIN: u16 = 0x1 << 5;
pub const RT5631_PWR_MIC1_BOOT_GAIN_BIT: u16 = 5;
pub const RT5631_PWR_MIC2_BOOT_GAIN: u16 = 0x1 << 4;
pub const RT5631_PWR_MIC2_BOOT_GAIN_BIT: u16 = 4;
pub const RT5631_PWR_MICBIAS1_VOL: u16 = 0x1 << 3;
pub const RT5631_PWR_MICBIAS1_VOL_BIT: u16 = 3;
pub const RT5631_PWR_MICBIAS2_VOL: u16 = 0x1 << 2;
pub const RT5631_PWR_MICBIAS2_VOL_BIT: u16 = 2;
pub const RT5631_PWR_PLL1: u16 = 0x1 << 1;
pub const RT5631_PWR_PLL1_BIT: u16 = 1;
pub const RT5631_PWR_PLL2: u16 = 0x1 << 0;
pub const RT5631_PWR_PLL2_BIT: u16 = 0;
/* Power managment addition 3(0x3C) */
pub const RT5631_PWR_VREF: u16 = 0x1 << 15;
pub const RT5631_PWR_VREF_BIT: u16 = 15;
pub const RT5631_PWR_FAST_VREF_CTRL: u16 = 0x1 << 14;
pub const RT5631_PWR_FAST_VREF_CTRL_BIT: u16 = 14;
pub const RT5631_PWR_MAIN_BIAS: u16 = 0x1 << 13;
pub const RT5631_PWR_MAIN_BIAS_BIT: u16 = 13;
pub const RT5631_PWR_AXO1MIXER: u16 = 0x1 << 11;
pub const RT5631_PWR_AXO1MIXER_BIT: u16 = 11;
pub const RT5631_PWR_AXO2MIXER: u16 = 0x1 << 10;
pub const RT5631_PWR_AXO2MIXER_BIT: u16 = 10;
pub const RT5631_PWR_MONOMIXER: u16 = 0x1 << 9;
pub const RT5631_PWR_MONOMIXER_BIT: u16 = 9;
pub const RT5631_PWR_MONO_DEPOP_DIS: u16 = 0x1 << 8;
pub const RT5631_PWR_MONO_DEPOP_DIS_BIT: u16 = 8;
pub const RT5631_PWR_MONO_AMP_EN: u16 = 0x1 << 7;
pub const RT5631_PWR_MONO_AMP_EN_BIT: u16 = 7;
pub const RT5631_PWR_CHARGE_PUMP: u16 = 0x1 << 4;
pub const RT5631_PWR_CHARGE_PUMP_BIT: u16 = 4;
pub const RT5631_PWR_HP_L_AMP: u16 = 0x1 << 3;
pub const RT5631_PWR_HP_L_AMP_BIT: u16 = 3;
pub const RT5631_PWR_HP_R_AMP: u16 = 0x1 << 2;
pub const RT5631_PWR_HP_R_AMP_BIT: u16 = 2;
pub const RT5631_PWR_HP_DEPOP_DIS: u16 = 0x1 << 1;
pub const RT5631_PWR_HP_DEPOP_DIS_BIT: u16 = 1;
pub const RT5631_PWR_HP_AMP_DRIVING: u16 = 0x1 << 0;
pub const RT5631_PWR_HP_AMP_DRIVING_BIT: u16 = 0;
/* Power managment addition 4(0x3E) */
pub const RT5631_PWR_SPK_L_VOL: u16 = 0x1 << 15;
pub const RT5631_PWR_SPK_L_VOL_BIT: u16 = 15;
pub const RT5631_PWR_SPK_R_VOL: u16 = 0x1 << 14;
pub const RT5631_PWR_SPK_R_VOL_BIT: u16 = 14;
pub const RT5631_PWR_LOUT_VOL: u16 = 0x1 << 13;
pub const RT5631_PWR_LOUT_VOL_BIT: u16 = 13;
pub const RT5631_PWR_ROUT_VOL: u16 = 0x1 << 12;
pub const RT5631_PWR_ROUT_VOL_BIT: u16 = 12;
pub const RT5631_PWR_HP_L_OUT_VOL: u16 = 0x1 << 11;
pub const RT5631_PWR_HP_L_OUT_VOL_BIT: u16 = 11;
pub const RT5631_PWR_HP_R_OUT_VOL: u16 = 0x1 << 10;
pub const RT5631_PWR_HP_R_OUT_VOL_BIT: u16 = 10;
pub const RT5631_PWR_AXIL_IN_VOL: u16 = 0x1 << 9;
pub const RT5631_PWR_AXIL_IN_VOL_BIT: u16 = 9;
pub const RT5631_PWR_AXIR_IN_VOL: u16 = 0x1 << 8;
pub const RT5631_PWR_AXIR_IN_VOL_BIT: u16 = 8;
pub const RT5631_PWR_MONO_IN_P_VOL: u16 = 0x1 << 7;
pub const RT5631_PWR_MONO_IN_P_VOL_BIT: u16 = 7;
pub const RT5631_PWR_MONO_IN_N_VOL: u16 = 0x1 << 6;
pub const RT5631_PWR_MONO_IN_N_VOL_BIT: u16 = 6;
/* General Purpose Control Register(0x40) */
pub const RT5631_SPK_AMP_AUTO_RATIO_EN: u16 = 0x1 << 15;
pub const RT5631_SPK_AMP_RATIO_CTRL_MASK: u16 = 0x7 << 12;
pub const RT5631_SPK_AMP_RATIO_CTRL_2_34: u16 = 0x0 << 12 /* 7.40DB */;
pub const RT5631_SPK_AMP_RATIO_CTRL_1_99: u16 = 0x1 << 12 /* 5.99DB */;
pub const RT5631_SPK_AMP_RATIO_CTRL_1_68: u16 = 0x2 << 12 /* 4.50DB */;
pub const RT5631_SPK_AMP_RATIO_CTRL_1_56: u16 = 0x3 << 12 /* 3.86DB */;
pub const RT5631_SPK_AMP_RATIO_CTRL_1_44: u16 = 0x4 << 12 /* 3.16DB */;
pub const RT5631_SPK_AMP_RATIO_CTRL_1_27: u16 = 0x5 << 12 /* 2.10DB */;
pub const RT5631_SPK_AMP_RATIO_CTRL_1_09: u16 = 0x6 << 12 /* 0.80DB */;
pub const RT5631_SPK_AMP_RATIO_CTRL_1_00: u16 = 0x7 << 12 /* 0.00DB */;
pub const RT5631_SPK_AMP_RATIO_CTRL_SHIFT: u16 = 12;
pub const RT5631_STEREO_DAC_HI_PASS_FILT_EN: u16 = 0x1 << 11;
pub const RT5631_STEREO_ADC_HI_PASS_FILT_EN: u16 = 0x1 << 10;
/* Select ADC Wind Filter Clock type */
pub const RT5631_ADC_WIND_FILT_MASK: u16 = 0x3 << 4;
pub const RT5631_ADC_WIND_FILT_8_16_32K: u16 = 0x0 << 4 /*8/16/32k*/;
pub const RT5631_ADC_WIND_FILT_11_22_44K: u16 = 0x1 << 4 /*11/22/44k*/;
pub const RT5631_ADC_WIND_FILT_12_24_48K: u16 = 0x2 << 4 /*12/24/48k*/;
pub const RT5631_ADC_WIND_FILT_EN: u16 = 0x1 << 3;
/* SelectADC Wind Filter Corner Frequency */
pub const RT5631_ADC_WIND_CNR_FREQ_MASK: u16 = 0x7 << 0;
pub const RT5631_ADC_WIND_CNR_FREQ_82_113_122: u16 = 0x0 << 0 /* 82/113/122 Hz */;
pub const RT5631_ADC_WIND_CNR_FREQ_102_141_153: u16 = 0x1 << 0 /* 102/141/153 Hz */;
pub const RT5631_ADC_WIND_CNR_FREQ_131_180_156: u16 = 0x2 << 0 /* 131/180/156 Hz */;
pub const RT5631_ADC_WIND_CNR_FREQ_163_225_245: u16 = 0x3 << 0 /* 163/225/245 Hz */;
pub const RT5631_ADC_WIND_CNR_FREQ_204_281_306: u16 = 0x4 << 0 /* 204/281/306 Hz */;
pub const RT5631_ADC_WIND_CNR_FREQ_261_360_392: u16 = 0x5 << 0 /* 261/360/392 Hz */;
pub const RT5631_ADC_WIND_CNR_FREQ_327_450_490: u16 = 0x6 << 0 /* 327/450/490 Hz */;
pub const RT5631_ADC_WIND_CNR_FREQ_408_563_612: u16 = 0x7 << 0 /* 408/563/612 Hz */;
/* Global Clock Control Register(0x42) */
pub const RT5631_SYSCLK_SOUR_SEL_MASK: u16 = 0x3 << 14;
pub const RT5631_SYSCLK_SOUR_SEL_MCLK: u16 = 0x0 << 14;
pub const RT5631_SYSCLK_SOUR_SEL_PLL: u16 = 0x1 << 14;
pub const RT5631_SYSCLK_SOUR_SEL_PLL_TCK: u16 = 0x2 << 14;
pub const RT5631_PLLCLK_SOUR_SEL_MASK: u16 = 0x3 << 12;
pub const RT5631_PLLCLK_SOUR_SEL_MCLK: u16 = 0x0 << 12;
pub const RT5631_PLLCLK_SOUR_SEL_BCLK: u16 = 0x1 << 12;
pub const RT5631_PLLCLK_SOUR_SEL_VBCLK: u16 = 0x2 << 12;
pub const RT5631_PLLCLK_PRE_DIV1: u16 = 0x0 << 11;
pub const RT5631_PLLCLK_PRE_DIV2: u16 = 0x1 << 11;
/* PLL Control(0x44) */
pub const fn RT5631_PLL_CTRL_M_VAL(m: u16) -> u16 { (m & 0x0f) }
pub const fn RT5631_PLL_CTRL_K_VAL(k: u16) -> u16 { ((k & 0x07) << 4) }
pub const fn RT5631_PLL_CTRL_N_VAL(n: u16) -> u16 { ((n & 0xff) << 8) }
/* Internal Status and IRQ Control2(0x4A) */
pub const RT5631_ADC_DATA_SEL_MASK: u16 = 0x3 << 14;
pub const RT5631_ADC_DATA_SEL_Disable: u16 = 0x0 << 14;
pub const RT5631_ADC_DATA_SEL_MIC1: u16 = 0x1 << 14;
pub const RT5631_ADC_DATA_SEL_MIC1_SHIFT: u16 = 14;
pub const RT5631_ADC_DATA_SEL_MIC2: u16 = 0x2 << 14;
pub const RT5631_ADC_DATA_SEL_MIC2_SHIFT: u16 = 15;
pub const RT5631_ADC_DATA_SEL_STO: u16 = 0x3 << 14;
pub const RT5631_ADC_DATA_SEL_SHIFT: u16 = 14;
/* GPIO Pin Configuration(0x4C) */
pub const RT5631_GPIO_PIN_FUN_SEL_MASK: u16 = 0x1 << 15;
pub const RT5631_GPIO_PIN_FUN_SEL_IRQ: u16 = 0x1 << 15;
pub const RT5631_GPIO_PIN_FUN_SEL_GPIO_DIMC: u16 = 0x0 << 15;
pub const RT5631_GPIO_DMIC_FUN_SEL_MASK: u16 = 0x1 << 3;
pub const RT5631_GPIO_DMIC_FUN_SEL_DIMC: u16 = 0x1 << 3;
pub const RT5631_GPIO_DMIC_FUN_SEL_GPIO: u16 = 0x0 << 3;
pub const RT5631_GPIO_PIN_CON_MASK: u16 = 0x1 << 2;
pub const RT5631_GPIO_PIN_SET_INPUT: u16 = 0x0 << 2;
pub const RT5631_GPIO_PIN_SET_OUTPUT: u16 = 0x1 << 2;
/* De-POP function Control 1(0x54) */
pub const RT5631_POW_ON_SOFT_GEN: u16 = 0x1 << 15;
pub const RT5631_EN_MUTE_UNMUTE_DEPOP: u16 = 0x1 << 14;
pub const RT5631_EN_DEPOP2_FOR_HP: u16 = 0x1 << 7;
/* Power Down HPAMP_L Starts Up Signal */
pub const RT5631_PD_HPAMP_L_ST_UP: u16 = 0x1 << 5;
/* Power Down HPAMP_R Starts Up Signal */
pub const RT5631_PD_HPAMP_R_ST_UP: u16 = 0x1 << 4;
/* Enable left HP mute/unmute depop */
pub const RT5631_EN_HP_L_M_UN_MUTE_DEPOP: u16 = 0x1 << 1;
/* Enable right HP mute/unmute depop */
pub const RT5631_EN_HP_R_M_UN_MUTE_DEPOP: u16 = 0x1 << 0;
/* De-POP Fnction Control(0x56) */
pub const RT5631_EN_ONE_BIT_DEPOP: u16 = 0x1 << 15;
pub const RT5631_EN_CAP_FREE_DEPOP: u16 = 0x1 << 14;
/* Jack Detect Control Register(0x5A) */
pub const RT5631_JD_USE_MASK: u16 = 0x3 << 14;
pub const RT5631_JD_USE_JD2: u16 = 0x3 << 14;
pub const RT5631_JD_USE_JD1: u16 = 0x2 << 14;
pub const RT5631_JD_USE_GPIO: u16 = 0x1 << 14;
pub const RT5631_JD_OFF: u16 = 0x0 << 14;
/* JD trigger enable for HP */
pub const RT5631_JD_HP_EN: u16 = 0x1 << 11;
pub const RT5631_JD_HP_TRI_MASK: u16 = 0x1 << 10;
pub const RT5631_JD_HP_TRI_HI: u16 = 0x1 << 10;
pub const RT5631_JD_HP_TRI_LO: u16 = 0x1 << 10;
/* JD trigger enable for speaker LP/LN */
pub const RT5631_JD_SPK_L_EN: u16 = 0x1 << 9;
pub const RT5631_JD_SPK_L_TRI_MASK: u16 = 0x1 << 8;
pub const RT5631_JD_SPK_L_TRI_HI: u16 = 0x1 << 8;
pub const RT5631_JD_SPK_L_TRI_LO: u16 = 0x0 << 8;
/* JD trigger enable for speaker RP/RN */
pub const RT5631_JD_SPK_R_EN: u16 = 0x1 << 7;
pub const RT5631_JD_SPK_R_TRI_MASK: u16 = 0x1 << 6;
pub const RT5631_JD_SPK_R_TRI_HI: u16 = 0x1 << 6;
pub const RT5631_JD_SPK_R_TRI_LO: u16 = 0x0 << 6;
/* JD trigger enable for monoout */
pub const RT5631_JD_MONO_EN: u16 = 0x1 << 5;
pub const RT5631_JD_MONO_TRI_MASK: u16 = 0x1 << 4;
pub const RT5631_JD_MONO_TRI_HI: u16 = 0x1 << 4;
pub const RT5631_JD_MONO_TRI_LO: u16 = 0x0 << 4;
/* JD trigger enable for Lout */
pub const RT5631_JD_AUX_1_EN: u16 = 0x1 << 3;
pub const RT5631_JD_AUX_1_MASK: u16 = 0x1 << 2;
pub const RT5631_JD_AUX_1_TRI_HI: u16 = 0x1 << 2;
pub const RT5631_JD_AUX_1_TRI_LO: u16 = 0x0 << 2;
/* JD trigger enable for Rout */
pub const RT5631_JD_AUX_2_EN: u16 = 0x1 << 1;
pub const RT5631_JD_AUX_2_MASK: u16 = 0x1 << 0;
pub const RT5631_JD_AUX_2_TRI_HI: u16 = 0x1 << 0;
pub const RT5631_JD_AUX_2_TRI_LO: u16 = 0x0 << 0;
/* ALC CONTROL 1(0x64) */
pub const RT5631_ALC_ATTACK_RATE_MASK: u16 = 0x1f << 8;
pub const RT5631_ALC_RECOVERY_RATE_MASK: u16 = 0x1f << 0;
/* ALC CONTROL 2(0x65) */
/* select Compensation gain for Noise gate function */
pub const RT5631_ALC_COM_NOISE_GATE_MASK: u16 = 0xf << 0;
/* ALC CONTROL 3(0x66) */
pub const RT5631_ALC_FUN_MASK: u16 = 0x3 << 14;
pub const RT5631_ALC_FUN_DIS: u16 = 0x0 << 14;
pub const RT5631_ALC_ENA_DAC_PATH: u16 = 0x1 << 14;
pub const RT5631_ALC_ENA_ADC_PATH: u16 = 0x3 << 14;
pub const RT5631_ALC_PARA_UPDATE: u16 = 0x1 << 13;
pub const RT5631_ALC_LIMIT_LEVEL_MASK: u16 = 0x1f << 8;
pub const RT5631_ALC_NOISE_GATE_FUN_MASK: u16 = 0x1 << 7;
pub const RT5631_ALC_NOISE_GATE_FUN_DIS: u16 = 0x0 << 7;
pub const RT5631_ALC_NOISE_GATE_FUN_ENA: u16 = 0x1 << 7;
/* ALC noise gate hold data function */
pub const RT5631_ALC_NOISE_GATE_H_D_MASK: u16 = 0x1 << 6;
pub const RT5631_ALC_NOISE_GATE_H_D_DIS: u16 = 0x0 << 6;
pub const RT5631_ALC_NOISE_GATE_H_D_ENA: u16 = 0x1 << 6;
/* Psedueo Stereo & Spatial Effect Block Control(0x68) */
pub const RT5631_SPATIAL_CTRL_EN: u16 = 0x1 << 15;
pub const RT5631_ALL_PASS_FILTER_EN: u16 = 0x1 << 14;
pub const RT5631_PSEUDO_STEREO_EN: u16 = 0x1 << 13;
pub const RT5631_STEREO_EXPENSION_EN: u16 = 0x1 << 12;
/* 3D gain parameter */
pub const RT5631_GAIN_3D_PARA_MASK: u16 = 0x3 << 6;
pub const RT5631_GAIN_3D_PARA_1_00: u16 = 0x0 << 6 /* 3D gain 1.0 */;
pub const RT5631_GAIN_3D_PARA_1_50: u16 = 0x1 << 6 /* 3D gain 1.5 */;
pub const RT5631_GAIN_3D_PARA_2_00: u16 = 0x2 << 6 /* 3D gain 2.0 */;
/* 3D ratio parameter */
pub const RT5631_RATIO_3D_MASK: u16 = 0x3 << 4;
pub const RT5631_RATIO_3D_0_0: u16 = 0x0 << 4 /* 3D ratio 0.0 */;
pub const RT5631_RATIO_3D_0_66: u16 = 0x1 << 4 /* 3D ratio 0.66 */;
pub const RT5631_RATIO_3D_1_0: u16 = 0x2 << 4 /* 3D ratio 1.0 */;
/* select samplerate for all pass filter */
pub const RT5631_APF_FUN_SLE_MASK: u16 = 0x3 << 0;
pub const RT5631_APF_FUN_SEL_48K: u16 = 0x3 << 0;
pub const RT5631_APF_FUN_SEL_44_1K: u16 = 0x2 << 0;
pub const RT5631_APF_FUN_SEL_32K: u16 = 0x1 << 0;
pub const RT5631_APF_FUN_DIS: u16 = 0x0 << 0;
/* EQ CONTROL 1(0x6E) */
pub const RT5631_HW_EQ_PATH_SEL_MASK: u16 = 0x1 << 15;
pub const RT5631_HW_EQ_PATH_SEL_DAC: u16 = 0x0 << 15;
pub const RT5631_HW_EQ_PATH_SEL_ADC: u16 = 0x1 << 15;
pub const RT5631_HW_EQ_UPDATE_CTRL: u16 = 0x1 << 14;
pub const RT5631_EN_HW_EQ_HPF2: u16 = 0x1 << 5;
pub const RT5631_EN_HW_EQ_HPF1: u16 = 0x1 << 4;
pub const RT5631_EN_HW_EQ_BP3: u16 = 0x1 << 3;
pub const RT5631_EN_HW_EQ_BP2: u16 = 0x1 << 2;
pub const RT5631_EN_HW_EQ_BP1: u16 = 0x1 << 1;
pub const RT5631_EN_HW_EQ_LPF: u16 = 0x1 << 0;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
