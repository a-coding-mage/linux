/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rt5668_header.rs -- Rust translation of rt5668.h
 *
 * Original C header included <sound/rt5668.h>.
 */

/* SPDX-License-Identifier: GPL-2.0-only */
/*
// Untranslated header line:  * rt5668.h  --  RT5668/RT5658 ALSA SoC audio driver
// Untranslated header line:  *
// Untranslated header line:  * Copyright 2018 Realtek Microelectronics
// Untranslated header line:  * Author: Bard Liao <bardliao@realtek.com>
// Untranslated header line:  */



pub const DEVICE_ID: u32 = 0x6530;

/* Info */
pub const RT5668_RESET: u32 = 0x0000;
pub const RT5668_VERSION_ID: u32 = 0x00fd;
pub const RT5668_VENDOR_ID: u32 = 0x00fe;
pub const RT5668_DEVICE_ID: u32 = 0x00ff;
/*  I/O - Output */
pub const RT5668_HP_CTRL_1: u32 = 0x0002;
pub const RT5668_HP_CTRL_2: u32 = 0x0003;
pub const RT5668_HPL_GAIN: u32 = 0x0005;
pub const RT5668_HPR_GAIN: u32 = 0x0006;

pub const RT5668_I2C_CTRL: u32 = 0x0008;

/* I/O - Input */
pub const RT5668_CBJ_BST_CTRL: u32 = 0x000b;
pub const RT5668_CBJ_CTRL_1: u32 = 0x0010;
pub const RT5668_CBJ_CTRL_2: u32 = 0x0011;
pub const RT5668_CBJ_CTRL_3: u32 = 0x0012;
pub const RT5668_CBJ_CTRL_4: u32 = 0x0013;
pub const RT5668_CBJ_CTRL_5: u32 = 0x0014;
pub const RT5668_CBJ_CTRL_6: u32 = 0x0015;
pub const RT5668_CBJ_CTRL_7: u32 = 0x0016;
/* I/O - ADC/DAC/DMIC */
pub const RT5668_DAC1_DIG_VOL: u32 = 0x0019;
pub const RT5668_STO1_ADC_DIG_VOL: u32 = 0x001c;
pub const RT5668_STO1_ADC_BOOST: u32 = 0x001f;
pub const RT5668_HP_IMP_GAIN_1: u32 = 0x0022;
pub const RT5668_HP_IMP_GAIN_2: u32 = 0x0023;
/* Mixer - D-D */
pub const RT5668_SIDETONE_CTRL: u32 = 0x0024;
pub const RT5668_STO1_ADC_MIXER: u32 = 0x0026;
pub const RT5668_AD_DA_MIXER: u32 = 0x0029;
pub const RT5668_STO1_DAC_MIXER: u32 = 0x002a;
pub const RT5668_A_DAC1_MUX: u32 = 0x002b;
pub const RT5668_DIG_INF2_DATA: u32 = 0x0030;
/* Mixer - ADC */
pub const RT5668_REC_MIXER: u32 = 0x003c;
pub const RT5668_CAL_REC: u32 = 0x0044;
pub const RT5668_ALC_BACK_GAIN: u32 = 0x0049;
/* Power */
pub const RT5668_PWR_DIG_1: u32 = 0x0061;
pub const RT5668_PWR_DIG_2: u32 = 0x0062;
pub const RT5668_PWR_ANLG_1: u32 = 0x0063;
pub const RT5668_PWR_ANLG_2: u32 = 0x0064;
pub const RT5668_PWR_ANLG_3: u32 = 0x0065;
pub const RT5668_PWR_MIXER: u32 = 0x0066;
pub const RT5668_PWR_VOL: u32 = 0x0067;
/* Clock Detect */
pub const RT5668_CLK_DET: u32 = 0x006b;
/* Filter Auto Reset */
pub const RT5668_RESET_LPF_CTRL: u32 = 0x006c;
pub const RT5668_RESET_HPF_CTRL: u32 = 0x006d;
/* DMIC */
pub const RT5668_DMIC_CTRL_1: u32 = 0x006e;
/* Format - ADC/DAC */
pub const RT5668_I2S1_SDP: u32 = 0x0070;
pub const RT5668_I2S2_SDP: u32 = 0x0071;
pub const RT5668_ADDA_CLK_1: u32 = 0x0073;
pub const RT5668_ADDA_CLK_2: u32 = 0x0074;
pub const RT5668_I2S1_F_DIV_CTRL_1: u32 = 0x0075;
pub const RT5668_I2S1_F_DIV_CTRL_2: u32 = 0x0076;
/* Format - TDM Control */
pub const RT5668_TDM_CTRL: u32 = 0x0079;
pub const RT5668_TDM_ADDA_CTRL_1: u32 = 0x007a;
pub const RT5668_TDM_ADDA_CTRL_2: u32 = 0x007b;
pub const RT5668_DATA_SEL_CTRL_1: u32 = 0x007c;
pub const RT5668_TDM_TCON_CTRL: u32 = 0x007e;
/* Function - Analog */
pub const RT5668_GLB_CLK: u32 = 0x0080;
pub const RT5668_PLL_CTRL_1: u32 = 0x0081;
pub const RT5668_PLL_CTRL_2: u32 = 0x0082;
pub const RT5668_PLL_TRACK_1: u32 = 0x0083;
pub const RT5668_PLL_TRACK_2: u32 = 0x0084;
pub const RT5668_PLL_TRACK_3: u32 = 0x0085;
pub const RT5668_PLL_TRACK_4: u32 = 0x0086;
pub const RT5668_PLL_TRACK_5: u32 = 0x0087;
pub const RT5668_PLL_TRACK_6: u32 = 0x0088;
pub const RT5668_PLL_TRACK_11: u32 = 0x008c;
pub const RT5668_SDW_REF_CLK: u32 = 0x008d;
pub const RT5668_DEPOP_1: u32 = 0x008e;
pub const RT5668_DEPOP_2: u32 = 0x008f;
pub const RT5668_HP_CHARGE_PUMP_1: u32 = 0x0091;
pub const RT5668_HP_CHARGE_PUMP_2: u32 = 0x0092;
pub const RT5668_MICBIAS_1: u32 = 0x0093;
pub const RT5668_MICBIAS_2: u32 = 0x0094;
pub const RT5668_PLL_TRACK_12: u32 = 0x0098;
pub const RT5668_PLL_TRACK_14: u32 = 0x009a;
pub const RT5668_PLL2_CTRL_1: u32 = 0x009b;
pub const RT5668_PLL2_CTRL_2: u32 = 0x009c;
pub const RT5668_PLL2_CTRL_3: u32 = 0x009d;
pub const RT5668_PLL2_CTRL_4: u32 = 0x009e;
pub const RT5668_RC_CLK_CTRL: u32 = 0x009f;
pub const RT5668_I2S_M_CLK_CTRL_1: u32 = 0x00a0;
pub const RT5668_I2S2_F_DIV_CTRL_1: u32 = 0x00a3;
pub const RT5668_I2S2_F_DIV_CTRL_2: u32 = 0x00a4;
/* Function - Digital */
pub const RT5668_EQ_CTRL_1: u32 = 0x00ae;
pub const RT5668_EQ_CTRL_2: u32 = 0x00af;
pub const RT5668_IRQ_CTRL_1: u32 = 0x00b6;
pub const RT5668_IRQ_CTRL_2: u32 = 0x00b7;
pub const RT5668_IRQ_CTRL_3: u32 = 0x00b8;
pub const RT5668_IRQ_CTRL_4: u32 = 0x00b9;
pub const RT5668_INT_ST_1: u32 = 0x00be;
pub const RT5668_GPIO_CTRL_1: u32 = 0x00c0;
pub const RT5668_GPIO_CTRL_2: u32 = 0x00c1;
pub const RT5668_GPIO_CTRL_3: u32 = 0x00c2;
pub const RT5668_HP_AMP_DET_CTRL_1: u32 = 0x00d0;
pub const RT5668_HP_AMP_DET_CTRL_2: u32 = 0x00d1;
pub const RT5668_MID_HP_AMP_DET: u32 = 0x00d2;
pub const RT5668_LOW_HP_AMP_DET: u32 = 0x00d3;
pub const RT5668_DELAY_BUF_CTRL: u32 = 0x00d4;
pub const RT5668_SV_ZCD_1: u32 = 0x00d9;
pub const RT5668_SV_ZCD_2: u32 = 0x00da;
pub const RT5668_IL_CMD_1: u32 = 0x00db;
pub const RT5668_IL_CMD_2: u32 = 0x00dc;
pub const RT5668_IL_CMD_3: u32 = 0x00dd;
pub const RT5668_IL_CMD_4: u32 = 0x00de;
pub const RT5668_IL_CMD_5: u32 = 0x00df;
pub const RT5668_IL_CMD_6: u32 = 0x00e0;
pub const RT5668_4BTN_IL_CMD_1: u32 = 0x00e2;
pub const RT5668_4BTN_IL_CMD_2: u32 = 0x00e3;
pub const RT5668_4BTN_IL_CMD_3: u32 = 0x00e4;
pub const RT5668_4BTN_IL_CMD_4: u32 = 0x00e5;
pub const RT5668_4BTN_IL_CMD_5: u32 = 0x00e6;
pub const RT5668_4BTN_IL_CMD_6: u32 = 0x00e7;
pub const RT5668_4BTN_IL_CMD_7: u32 = 0x00e8;

pub const RT5668_ADC_STO1_HP_CTRL_1: u32 = 0x00ea;
pub const RT5668_ADC_STO1_HP_CTRL_2: u32 = 0x00eb;
pub const RT5668_AJD1_CTRL: u32 = 0x00f0;
pub const RT5668_JD1_THD: u32 = 0x00f1;
pub const RT5668_JD2_THD: u32 = 0x00f2;
pub const RT5668_JD_CTRL_1: u32 = 0x00f6;
/* General Control */
pub const RT5668_DUMMY_1: u32 = 0x00fa;
pub const RT5668_DUMMY_2: u32 = 0x00fb;
pub const RT5668_DUMMY_3: u32 = 0x00fc;

pub const RT5668_DAC_ADC_DIG_VOL1: u32 = 0x0100;
pub const RT5668_BIAS_CUR_CTRL_2: u32 = 0x010b;
pub const RT5668_BIAS_CUR_CTRL_3: u32 = 0x010c;
pub const RT5668_BIAS_CUR_CTRL_4: u32 = 0x010d;
pub const RT5668_BIAS_CUR_CTRL_5: u32 = 0x010e;
pub const RT5668_BIAS_CUR_CTRL_6: u32 = 0x010f;
pub const RT5668_BIAS_CUR_CTRL_7: u32 = 0x0110;
pub const RT5668_BIAS_CUR_CTRL_8: u32 = 0x0111;
pub const RT5668_BIAS_CUR_CTRL_9: u32 = 0x0112;
pub const RT5668_BIAS_CUR_CTRL_10: u32 = 0x0113;
pub const RT5668_VREF_REC_OP_FB_CAP_CTRL: u32 = 0x0117;
pub const RT5668_CHARGE_PUMP_1: u32 = 0x0125;
pub const RT5668_DIG_IN_CTRL_1: u32 = 0x0132;
pub const RT5668_PAD_DRIVING_CTRL: u32 = 0x0136;
pub const RT5668_SOFT_RAMP_DEPOP: u32 = 0x0138;
pub const RT5668_CHOP_DAC: u32 = 0x013a;
pub const RT5668_CHOP_ADC: u32 = 0x013b;
pub const RT5668_CALIB_ADC_CTRL: u32 = 0x013c;
pub const RT5668_VOL_TEST: u32 = 0x013f;
pub const RT5668_SPKVDD_DET_STA: u32 = 0x0142;
pub const RT5668_TEST_MODE_CTRL_1: u32 = 0x0145;
pub const RT5668_TEST_MODE_CTRL_2: u32 = 0x0146;
pub const RT5668_TEST_MODE_CTRL_3: u32 = 0x0147;
pub const RT5668_TEST_MODE_CTRL_4: u32 = 0x0148;
pub const RT5668_TEST_MODE_CTRL_5: u32 = 0x0149;
pub const RT5668_PLL1_INTERNAL: u32 = 0x0150;
pub const RT5668_PLL2_INTERNAL: u32 = 0x0151;
pub const RT5668_STO_NG2_CTRL_1: u32 = 0x0160;
pub const RT5668_STO_NG2_CTRL_2: u32 = 0x0161;
pub const RT5668_STO_NG2_CTRL_3: u32 = 0x0162;
pub const RT5668_STO_NG2_CTRL_4: u32 = 0x0163;
pub const RT5668_STO_NG2_CTRL_5: u32 = 0x0164;
pub const RT5668_STO_NG2_CTRL_6: u32 = 0x0165;
pub const RT5668_STO_NG2_CTRL_7: u32 = 0x0166;
pub const RT5668_STO_NG2_CTRL_8: u32 = 0x0167;
pub const RT5668_STO_NG2_CTRL_9: u32 = 0x0168;
pub const RT5668_STO_NG2_CTRL_10: u32 = 0x0169;
pub const RT5668_STO1_DAC_SIL_DET: u32 = 0x0190;
pub const RT5668_SIL_PSV_CTRL1: u32 = 0x0194;
pub const RT5668_SIL_PSV_CTRL2: u32 = 0x0195;
pub const RT5668_SIL_PSV_CTRL3: u32 = 0x0197;
pub const RT5668_SIL_PSV_CTRL4: u32 = 0x0198;
pub const RT5668_SIL_PSV_CTRL5: u32 = 0x0199;
pub const RT5668_HP_IMP_SENS_CTRL_01: u32 = 0x01af;
pub const RT5668_HP_IMP_SENS_CTRL_02: u32 = 0x01b0;
pub const RT5668_HP_IMP_SENS_CTRL_03: u32 = 0x01b1;
pub const RT5668_HP_IMP_SENS_CTRL_04: u32 = 0x01b2;
pub const RT5668_HP_IMP_SENS_CTRL_05: u32 = 0x01b3;
pub const RT5668_HP_IMP_SENS_CTRL_06: u32 = 0x01b4;
pub const RT5668_HP_IMP_SENS_CTRL_07: u32 = 0x01b5;
pub const RT5668_HP_IMP_SENS_CTRL_08: u32 = 0x01b6;
pub const RT5668_HP_IMP_SENS_CTRL_09: u32 = 0x01b7;
pub const RT5668_HP_IMP_SENS_CTRL_10: u32 = 0x01b8;
pub const RT5668_HP_IMP_SENS_CTRL_11: u32 = 0x01b9;
pub const RT5668_HP_IMP_SENS_CTRL_12: u32 = 0x01ba;
pub const RT5668_HP_IMP_SENS_CTRL_13: u32 = 0x01bb;
pub const RT5668_HP_IMP_SENS_CTRL_14: u32 = 0x01bc;
pub const RT5668_HP_IMP_SENS_CTRL_15: u32 = 0x01bd;
pub const RT5668_HP_IMP_SENS_CTRL_16: u32 = 0x01be;
pub const RT5668_HP_IMP_SENS_CTRL_17: u32 = 0x01bf;
pub const RT5668_HP_IMP_SENS_CTRL_18: u32 = 0x01c0;
pub const RT5668_HP_IMP_SENS_CTRL_19: u32 = 0x01c1;
pub const RT5668_HP_IMP_SENS_CTRL_20: u32 = 0x01c2;
pub const RT5668_HP_IMP_SENS_CTRL_21: u32 = 0x01c3;
pub const RT5668_HP_IMP_SENS_CTRL_22: u32 = 0x01c4;
pub const RT5668_HP_IMP_SENS_CTRL_23: u32 = 0x01c5;
pub const RT5668_HP_IMP_SENS_CTRL_24: u32 = 0x01c6;
pub const RT5668_HP_IMP_SENS_CTRL_25: u32 = 0x01c7;
pub const RT5668_HP_IMP_SENS_CTRL_26: u32 = 0x01c8;
pub const RT5668_HP_IMP_SENS_CTRL_27: u32 = 0x01c9;
pub const RT5668_HP_IMP_SENS_CTRL_28: u32 = 0x01ca;
pub const RT5668_HP_IMP_SENS_CTRL_29: u32 = 0x01cb;
pub const RT5668_HP_IMP_SENS_CTRL_30: u32 = 0x01cc;
pub const RT5668_HP_IMP_SENS_CTRL_31: u32 = 0x01cd;
pub const RT5668_HP_IMP_SENS_CTRL_32: u32 = 0x01ce;
pub const RT5668_HP_IMP_SENS_CTRL_33: u32 = 0x01cf;
pub const RT5668_HP_IMP_SENS_CTRL_34: u32 = 0x01d0;
pub const RT5668_HP_IMP_SENS_CTRL_35: u32 = 0x01d1;
pub const RT5668_HP_IMP_SENS_CTRL_36: u32 = 0x01d2;
pub const RT5668_HP_IMP_SENS_CTRL_37: u32 = 0x01d3;
pub const RT5668_HP_IMP_SENS_CTRL_38: u32 = 0x01d4;
pub const RT5668_HP_IMP_SENS_CTRL_39: u32 = 0x01d5;
pub const RT5668_HP_IMP_SENS_CTRL_40: u32 = 0x01d6;
pub const RT5668_HP_IMP_SENS_CTRL_41: u32 = 0x01d7;
pub const RT5668_HP_IMP_SENS_CTRL_42: u32 = 0x01d8;
pub const RT5668_HP_IMP_SENS_CTRL_43: u32 = 0x01d9;
pub const RT5668_HP_LOGIC_CTRL_1: u32 = 0x01da;
pub const RT5668_HP_LOGIC_CTRL_2: u32 = 0x01db;
pub const RT5668_HP_LOGIC_CTRL_3: u32 = 0x01dc;
pub const RT5668_HP_CALIB_CTRL_1: u32 = 0x01de;
pub const RT5668_HP_CALIB_CTRL_2: u32 = 0x01df;
pub const RT5668_HP_CALIB_CTRL_3: u32 = 0x01e0;
pub const RT5668_HP_CALIB_CTRL_4: u32 = 0x01e1;
pub const RT5668_HP_CALIB_CTRL_5: u32 = 0x01e2;
pub const RT5668_HP_CALIB_CTRL_6: u32 = 0x01e3;
pub const RT5668_HP_CALIB_CTRL_7: u32 = 0x01e4;
pub const RT5668_HP_CALIB_CTRL_9: u32 = 0x01e6;
pub const RT5668_HP_CALIB_CTRL_10: u32 = 0x01e7;
pub const RT5668_HP_CALIB_CTRL_11: u32 = 0x01e8;
pub const RT5668_HP_CALIB_STA_1: u32 = 0x01ea;
pub const RT5668_HP_CALIB_STA_2: u32 = 0x01eb;
pub const RT5668_HP_CALIB_STA_3: u32 = 0x01ec;
pub const RT5668_HP_CALIB_STA_4: u32 = 0x01ed;
pub const RT5668_HP_CALIB_STA_5: u32 = 0x01ee;
pub const RT5668_HP_CALIB_STA_6: u32 = 0x01ef;
pub const RT5668_HP_CALIB_STA_7: u32 = 0x01f0;
pub const RT5668_HP_CALIB_STA_8: u32 = 0x01f1;
pub const RT5668_HP_CALIB_STA_9: u32 = 0x01f2;
pub const RT5668_HP_CALIB_STA_10: u32 = 0x01f3;
pub const RT5668_HP_CALIB_STA_11: u32 = 0x01f4;
pub const RT5668_SAR_IL_CMD_1: u32 = 0x0210;
pub const RT5668_SAR_IL_CMD_2: u32 = 0x0211;
pub const RT5668_SAR_IL_CMD_3: u32 = 0x0212;
pub const RT5668_SAR_IL_CMD_4: u32 = 0x0213;
pub const RT5668_SAR_IL_CMD_5: u32 = 0x0214;
pub const RT5668_SAR_IL_CMD_6: u32 = 0x0215;
pub const RT5668_SAR_IL_CMD_7: u32 = 0x0216;
pub const RT5668_SAR_IL_CMD_8: u32 = 0x0217;
pub const RT5668_SAR_IL_CMD_9: u32 = 0x0218;
pub const RT5668_SAR_IL_CMD_10: u32 = 0x0219;
pub const RT5668_SAR_IL_CMD_11: u32 = 0x021a;
pub const RT5668_SAR_IL_CMD_12: u32 = 0x021b;
pub const RT5668_SAR_IL_CMD_13: u32 = 0x021c;
pub const RT5668_EFUSE_CTRL_1: u32 = 0x0250;
pub const RT5668_EFUSE_CTRL_2: u32 = 0x0251;
pub const RT5668_EFUSE_CTRL_3: u32 = 0x0252;
pub const RT5668_EFUSE_CTRL_4: u32 = 0x0253;
pub const RT5668_EFUSE_CTRL_5: u32 = 0x0254;
pub const RT5668_EFUSE_CTRL_6: u32 = 0x0255;
pub const RT5668_EFUSE_CTRL_7: u32 = 0x0256;
pub const RT5668_EFUSE_CTRL_8: u32 = 0x0257;
pub const RT5668_EFUSE_CTRL_9: u32 = 0x0258;
pub const RT5668_EFUSE_CTRL_10: u32 = 0x0259;
pub const RT5668_EFUSE_CTRL_11: u32 = 0x025a;
pub const RT5668_JD_TOP_VC_VTRL: u32 = 0x0270;
pub const RT5668_DRC1_CTRL_0: u32 = 0x02ff;
pub const RT5668_DRC1_CTRL_1: u32 = 0x0300;
pub const RT5668_DRC1_CTRL_2: u32 = 0x0301;
pub const RT5668_DRC1_CTRL_3: u32 = 0x0302;
pub const RT5668_DRC1_CTRL_4: u32 = 0x0303;
pub const RT5668_DRC1_CTRL_5: u32 = 0x0304;
pub const RT5668_DRC1_CTRL_6: u32 = 0x0305;
pub const RT5668_DRC1_HARD_LMT_CTRL_1: u32 = 0x0306;
pub const RT5668_DRC1_HARD_LMT_CTRL_2: u32 = 0x0307;
pub const RT5668_DRC1_PRIV_1: u32 = 0x0310;
pub const RT5668_DRC1_PRIV_2: u32 = 0x0311;
pub const RT5668_DRC1_PRIV_3: u32 = 0x0312;
pub const RT5668_DRC1_PRIV_4: u32 = 0x0313;
pub const RT5668_DRC1_PRIV_5: u32 = 0x0314;
pub const RT5668_DRC1_PRIV_6: u32 = 0x0315;
pub const RT5668_DRC1_PRIV_7: u32 = 0x0316;
pub const RT5668_DRC1_PRIV_8: u32 = 0x0317;
pub const RT5668_EQ_AUTO_RCV_CTRL1: u32 = 0x03c0;
pub const RT5668_EQ_AUTO_RCV_CTRL2: u32 = 0x03c1;
pub const RT5668_EQ_AUTO_RCV_CTRL3: u32 = 0x03c2;
pub const RT5668_EQ_AUTO_RCV_CTRL4: u32 = 0x03c3;
pub const RT5668_EQ_AUTO_RCV_CTRL5: u32 = 0x03c4;
pub const RT5668_EQ_AUTO_RCV_CTRL6: u32 = 0x03c5;
pub const RT5668_EQ_AUTO_RCV_CTRL7: u32 = 0x03c6;
pub const RT5668_EQ_AUTO_RCV_CTRL8: u32 = 0x03c7;
pub const RT5668_EQ_AUTO_RCV_CTRL9: u32 = 0x03c8;
pub const RT5668_EQ_AUTO_RCV_CTRL10: u32 = 0x03c9;
pub const RT5668_EQ_AUTO_RCV_CTRL11: u32 = 0x03ca;
pub const RT5668_EQ_AUTO_RCV_CTRL12: u32 = 0x03cb;
pub const RT5668_EQ_AUTO_RCV_CTRL13: u32 = 0x03cc;
pub const RT5668_ADC_L_EQ_LPF1_A1: u32 = 0x03d0;
pub const RT5668_R_EQ_LPF1_A1: u32 = 0x03d1;
pub const RT5668_L_EQ_LPF1_H0: u32 = 0x03d2;
pub const RT5668_R_EQ_LPF1_H0: u32 = 0x03d3;
pub const RT5668_L_EQ_BPF1_A1: u32 = 0x03d4;
pub const RT5668_R_EQ_BPF1_A1: u32 = 0x03d5;
pub const RT5668_L_EQ_BPF1_A2: u32 = 0x03d6;
pub const RT5668_R_EQ_BPF1_A2: u32 = 0x03d7;
pub const RT5668_L_EQ_BPF1_H0: u32 = 0x03d8;
pub const RT5668_R_EQ_BPF1_H0: u32 = 0x03d9;
pub const RT5668_L_EQ_BPF2_A1: u32 = 0x03da;
pub const RT5668_R_EQ_BPF2_A1: u32 = 0x03db;
pub const RT5668_L_EQ_BPF2_A2: u32 = 0x03dc;
pub const RT5668_R_EQ_BPF2_A2: u32 = 0x03dd;
pub const RT5668_L_EQ_BPF2_H0: u32 = 0x03de;
pub const RT5668_R_EQ_BPF2_H0: u32 = 0x03df;
pub const RT5668_L_EQ_BPF3_A1: u32 = 0x03e0;
pub const RT5668_R_EQ_BPF3_A1: u32 = 0x03e1;
pub const RT5668_L_EQ_BPF3_A2: u32 = 0x03e2;
pub const RT5668_R_EQ_BPF3_A2: u32 = 0x03e3;
pub const RT5668_L_EQ_BPF3_H0: u32 = 0x03e4;
pub const RT5668_R_EQ_BPF3_H0: u32 = 0x03e5;
pub const RT5668_L_EQ_BPF4_A1: u32 = 0x03e6;
pub const RT5668_R_EQ_BPF4_A1: u32 = 0x03e7;
pub const RT5668_L_EQ_BPF4_A2: u32 = 0x03e8;
pub const RT5668_R_EQ_BPF4_A2: u32 = 0x03e9;
pub const RT5668_L_EQ_BPF4_H0: u32 = 0x03ea;
pub const RT5668_R_EQ_BPF4_H0: u32 = 0x03eb;
pub const RT5668_L_EQ_HPF1_A1: u32 = 0x03ec;
pub const RT5668_R_EQ_HPF1_A1: u32 = 0x03ed;
pub const RT5668_L_EQ_HPF1_H0: u32 = 0x03ee;
pub const RT5668_R_EQ_HPF1_H0: u32 = 0x03ef;
pub const RT5668_L_EQ_PRE_VOL: u32 = 0x03f0;
pub const RT5668_R_EQ_PRE_VOL: u32 = 0x03f1;
pub const RT5668_L_EQ_POST_VOL: u32 = 0x03f2;
pub const RT5668_R_EQ_POST_VOL: u32 = 0x03f3;
pub const RT5668_I2C_MODE: u32 = 0xffff;


/* global definition */
pub const RT5668_L_MUTE: u32 = (0x1 << 15);
pub const RT5668_L_MUTE_SFT: u32 = 15;
pub const RT5668_VOL_L_MUTE: u32 = (0x1 << 14);
pub const RT5668_VOL_L_SFT: u32 = 14;
pub const RT5668_R_MUTE: u32 = (0x1 << 7);
pub const RT5668_R_MUTE_SFT: u32 = 7;
pub const RT5668_VOL_R_MUTE: u32 = (0x1 << 6);
pub const RT5668_VOL_R_SFT: u32 = 6;
pub const RT5668_L_VOL_MASK: u32 = (0x3f << 8);
pub const RT5668_L_VOL_SFT: u32 = 8;
pub const RT5668_R_VOL_MASK: u32 = (0x3f);
pub const RT5668_R_VOL_SFT: u32 = 0;

/*Headphone Amp L/R Analog Gain and Digital NG2 Gain Control (0x0005 0x0006)*/
pub const RT5668_G_HP: u32 = (0xf << 8);
pub const RT5668_G_HP_SFT: u32 = 8;
pub const RT5668_G_STO_DA_DMIX: u32 = (0xf);
pub const RT5668_G_STO_DA_SFT: u32 = 0;

/* CBJ Control (0x000b) */
pub const RT5668_BST_CBJ_MASK: u32 = (0xf << 8);
pub const RT5668_BST_CBJ_SFT: u32 = 8;

/* Embeeded Jack and Type Detection Control 1 (0x0010) */
pub const RT5668_EMB_JD_EN: u32 = (0x1 << 15);
pub const RT5668_EMB_JD_EN_SFT: u32 = 15;
pub const RT5668_EMB_JD_RST: u32 = (0x1 << 14);
pub const RT5668_JD_MODE: u32 = (0x1 << 13);
pub const RT5668_JD_MODE_SFT: u32 = 13;
pub const RT5668_DET_TYPE: u32 = (0x1 << 12);
pub const RT5668_DET_TYPE_SFT: u32 = 12;
pub const RT5668_POLA_EXT_JD_MASK: u32 = (0x1 << 11);
pub const RT5668_POLA_EXT_JD_LOW: u32 = (0x1 << 11);
pub const RT5668_POLA_EXT_JD_HIGH: u32 = (0x0 << 11);
pub const RT5668_EXT_JD_DIG: u32 = (0x1 << 9);
pub const RT5668_POL_FAST_OFF_MASK: u32 = (0x1 << 8);
pub const RT5668_POL_FAST_OFF_HIGH: u32 = (0x1 << 8);
pub const RT5668_POL_FAST_OFF_LOW: u32 = (0x0 << 8);
pub const RT5668_FAST_OFF_MASK: u32 = (0x1 << 7);
pub const RT5668_FAST_OFF_EN: u32 = (0x1 << 7);
pub const RT5668_FAST_OFF_DIS: u32 = (0x0 << 7);
pub const RT5668_VREF_POW_MASK: u32 = (0x1 << 6);
pub const RT5668_VREF_POW_FSM: u32 = (0x0 << 6);
pub const RT5668_VREF_POW_REG: u32 = (0x1 << 6);
pub const RT5668_MB1_PATH_MASK: u32 = (0x1 << 5);
pub const RT5668_CTRL_MB1_REG: u32 = (0x1 << 5);
pub const RT5668_CTRL_MB1_FSM: u32 = (0x0 << 5);
pub const RT5668_MB2_PATH_MASK: u32 = (0x1 << 4);
pub const RT5668_CTRL_MB2_REG: u32 = (0x1 << 4);
pub const RT5668_CTRL_MB2_FSM: u32 = (0x0 << 4);
pub const RT5668_TRIG_JD_MASK: u32 = (0x1 << 3);
pub const RT5668_TRIG_JD_HIGH: u32 = (0x1 << 3);
pub const RT5668_TRIG_JD_LOW: u32 = (0x0 << 3);
pub const RT5668_MIC_CAP_MASK: u32 = (0x1 << 1);
pub const RT5668_MIC_CAP_HS: u32 = (0x1 << 1);
pub const RT5668_MIC_CAP_HP: u32 = (0x0 << 1);
pub const RT5668_MIC_CAP_SRC_MASK: u32 = (0x1);
pub const RT5668_MIC_CAP_SRC_REG: u32 = (0x1);
pub const RT5668_MIC_CAP_SRC_ANA: u32 = (0x0);

/* Embeeded Jack and Type Detection Control 2 (0x0011) */
pub const RT5668_EXT_JD_SRC: u32 = (0x7 << 4);
pub const RT5668_EXT_JD_SRC_SFT: u32 = 4;
pub const RT5668_EXT_JD_SRC_GPIO_JD1: u32 = (0x0 << 4);
pub const RT5668_EXT_JD_SRC_GPIO_JD2: u32 = (0x1 << 4);
pub const RT5668_EXT_JD_SRC_JDH: u32 = (0x2 << 4);
pub const RT5668_EXT_JD_SRC_JDL: u32 = (0x3 << 4);
pub const RT5668_EXT_JD_SRC_MANUAL: u32 = (0x4 << 4);
pub const RT5668_JACK_TYPE_MASK: u32 = (0x3);

/* Combo Jack and Type Detection Control 3 (0x0012) */
pub const RT5668_CBJ_IN_BUF_EN: u32 = (0x1 << 7);

/* Combo Jack and Type Detection Control 4 (0x0013) */
pub const RT5668_SEL_SHT_MID_TON_MASK: u32 = (0x3 << 12);
pub const RT5668_SEL_SHT_MID_TON_2: u32 = (0x0 << 12);
pub const RT5668_SEL_SHT_MID_TON_3: u32 = (0x1 << 12);
pub const RT5668_CBJ_JD_TEST_MASK: u32 = (0x1 << 6);
pub const RT5668_CBJ_JD_TEST_NORM: u32 = (0x0 << 6);
pub const RT5668_CBJ_JD_TEST_MODE: u32 = (0x1 << 6);

/* DAC1 Digital Volume (0x0019) */
pub const RT5668_DAC_L1_VOL_MASK: u32 = (0xff << 8);
pub const RT5668_DAC_L1_VOL_SFT: u32 = 8;
pub const RT5668_DAC_R1_VOL_MASK: u32 = (0xff);
pub const RT5668_DAC_R1_VOL_SFT: u32 = 0;

/* ADC Digital Volume Control (0x001c) */
pub const RT5668_ADC_L_VOL_MASK: u32 = (0x7f << 8);
pub const RT5668_ADC_L_VOL_SFT: u32 = 8;
pub const RT5668_ADC_R_VOL_MASK: u32 = (0x7f);
pub const RT5668_ADC_R_VOL_SFT: u32 = 0;

/* Stereo1 ADC Boost Gain Control (0x001f) */
pub const RT5668_STO1_ADC_L_BST_MASK: u32 = (0x3 << 14);
pub const RT5668_STO1_ADC_L_BST_SFT: u32 = 14;
pub const RT5668_STO1_ADC_R_BST_MASK: u32 = (0x3 << 12);
pub const RT5668_STO1_ADC_R_BST_SFT: u32 = 12;

/* Sidetone Control (0x0024) */
pub const RT5668_ST_SRC_SEL: u32 = (0x1 << 8);
pub const RT5668_ST_SRC_SFT: u32 = 8;
pub const RT5668_ST_EN_MASK: u32 = (0x1 << 6);
pub const RT5668_ST_DIS: u32 = (0x0 << 6);
pub const RT5668_ST_EN: u32 = (0x1 << 6);
pub const RT5668_ST_EN_SFT: u32 = 6;

/* Stereo1 ADC Mixer Control (0x0026) */
pub const RT5668_M_STO1_ADC_L1: u32 = (0x1 << 15);
pub const RT5668_M_STO1_ADC_L1_SFT: u32 = 15;
pub const RT5668_M_STO1_ADC_L2: u32 = (0x1 << 14);
pub const RT5668_M_STO1_ADC_L2_SFT: u32 = 14;
pub const RT5668_STO1_ADC1L_SRC_MASK: u32 = (0x1 << 13);
pub const RT5668_STO1_ADC1L_SRC_SFT: u32 = 13;
pub const RT5668_STO1_ADC1_SRC_ADC: u32 = (0x1 << 13);
pub const RT5668_STO1_ADC1_SRC_DACMIX: u32 = (0x0 << 13);
pub const RT5668_STO1_ADC2L_SRC_MASK: u32 = (0x1 << 12);
pub const RT5668_STO1_ADC2L_SRC_SFT: u32 = 12;
pub const RT5668_STO1_ADCL_SRC_MASK: u32 = (0x3 << 10);
pub const RT5668_STO1_ADCL_SRC_SFT: u32 = 10;
pub const RT5668_STO1_DD_L_SRC_MASK: u32 = (0x1 << 9);
pub const RT5668_STO1_DD_L_SRC_SFT: u32 = 9;
pub const RT5668_STO1_DMIC_SRC_MASK: u32 = (0x1 << 8);
pub const RT5668_STO1_DMIC_SRC_SFT: u32 = 8;
pub const RT5668_STO1_DMIC_SRC_DMIC2: u32 = (0x1 << 8);
pub const RT5668_STO1_DMIC_SRC_DMIC1: u32 = (0x0 << 8);
pub const RT5668_M_STO1_ADC_R1: u32 = (0x1 << 7);
pub const RT5668_M_STO1_ADC_R1_SFT: u32 = 7;
pub const RT5668_M_STO1_ADC_R2: u32 = (0x1 << 6);
pub const RT5668_M_STO1_ADC_R2_SFT: u32 = 6;
pub const RT5668_STO1_ADC1R_SRC_MASK: u32 = (0x1 << 5);
pub const RT5668_STO1_ADC1R_SRC_SFT: u32 = 5;
pub const RT5668_STO1_ADC2R_SRC_MASK: u32 = (0x1 << 4);
pub const RT5668_STO1_ADC2R_SRC_SFT: u32 = 4;
pub const RT5668_STO1_ADCR_SRC_MASK: u32 = (0x3 << 2);
pub const RT5668_STO1_ADCR_SRC_SFT: u32 = 2;

/* ADC Mixer to DAC Mixer Control (0x0029) */
pub const RT5668_M_ADCMIX_L: u32 = (0x1 << 15);
pub const RT5668_M_ADCMIX_L_SFT: u32 = 15;
pub const RT5668_M_DAC1_L: u32 = (0x1 << 14);
pub const RT5668_M_DAC1_L_SFT: u32 = 14;
pub const RT5668_DAC1_R_SEL_MASK: u32 = (0x1 << 10);
pub const RT5668_DAC1_R_SEL_SFT: u32 = 10;
pub const RT5668_DAC1_L_SEL_MASK: u32 = (0x1 << 8);
pub const RT5668_DAC1_L_SEL_SFT: u32 = 8;
pub const RT5668_M_ADCMIX_R: u32 = (0x1 << 7);
pub const RT5668_M_ADCMIX_R_SFT: u32 = 7;
pub const RT5668_M_DAC1_R: u32 = (0x1 << 6);
pub const RT5668_M_DAC1_R_SFT: u32 = 6;

/* Stereo1 DAC Mixer Control (0x002a) */
pub const RT5668_M_DAC_L1_STO_L: u32 = (0x1 << 15);
pub const RT5668_M_DAC_L1_STO_L_SFT: u32 = 15;
pub const RT5668_G_DAC_L1_STO_L_MASK: u32 = (0x1 << 14);
pub const RT5668_G_DAC_L1_STO_L_SFT: u32 = 14;
pub const RT5668_M_DAC_R1_STO_L: u32 = (0x1 << 13);
pub const RT5668_M_DAC_R1_STO_L_SFT: u32 = 13;
pub const RT5668_G_DAC_R1_STO_L_MASK: u32 = (0x1 << 12);
pub const RT5668_G_DAC_R1_STO_L_SFT: u32 = 12;
pub const RT5668_M_DAC_L1_STO_R: u32 = (0x1 << 7);
pub const RT5668_M_DAC_L1_STO_R_SFT: u32 = 7;
pub const RT5668_G_DAC_L1_STO_R_MASK: u32 = (0x1 << 6);
pub const RT5668_G_DAC_L1_STO_R_SFT: u32 = 6;
pub const RT5668_M_DAC_R1_STO_R: u32 = (0x1 << 5);
pub const RT5668_M_DAC_R1_STO_R_SFT: u32 = 5;
pub const RT5668_G_DAC_R1_STO_R_MASK: u32 = (0x1 << 4);
pub const RT5668_G_DAC_R1_STO_R_SFT: u32 = 4;

/* Analog DAC1 Input Source Control (0x002b) */
pub const RT5668_M_ST_STO_L: u32 = (0x1 << 9);
pub const RT5668_M_ST_STO_L_SFT: u32 = 9;
pub const RT5668_M_ST_STO_R: u32 = (0x1 << 8);
pub const RT5668_M_ST_STO_R_SFT: u32 = 8;
pub const RT5668_DAC_L1_SRC_MASK: u32 = (0x3 << 4);
pub const RT5668_A_DACL1_SFT: u32 = 4;
pub const RT5668_DAC_R1_SRC_MASK: u32 = (0x3);
pub const RT5668_A_DACR1_SFT: u32 = 0;

/* Digital Interface Data Control (0x0030) */
pub const RT5668_IF2_ADC_SEL_MASK: u32 = (0x3 << 0);
pub const RT5668_IF2_ADC_SEL_SFT: u32 = 0;

/* REC Left Mixer Control 2 (0x003c) */
pub const RT5668_G_CBJ_RM1_L: u32 = (0x7 << 10);
pub const RT5668_G_CBJ_RM1_L_SFT: u32 = 10;
pub const RT5668_M_CBJ_RM1_L: u32 = (0x1 << 7);
pub const RT5668_M_CBJ_RM1_L_SFT: u32 = 7;

/* Power Management for Digital 1 (0x0061) */
pub const RT5668_PWR_I2S1: u32 = (0x1 << 15);
pub const RT5668_PWR_I2S1_BIT: u32 = 15;
pub const RT5668_PWR_I2S2: u32 = (0x1 << 14);
pub const RT5668_PWR_I2S2_BIT: u32 = 14;
pub const RT5668_PWR_DAC_L1: u32 = (0x1 << 11);
pub const RT5668_PWR_DAC_L1_BIT: u32 = 11;
pub const RT5668_PWR_DAC_R1: u32 = (0x1 << 10);
pub const RT5668_PWR_DAC_R1_BIT: u32 = 10;
pub const RT5668_PWR_LDO: u32 = (0x1 << 8);
pub const RT5668_PWR_LDO_BIT: u32 = 8;
pub const RT5668_PWR_ADC_L1: u32 = (0x1 << 4);
pub const RT5668_PWR_ADC_L1_BIT: u32 = 4;
pub const RT5668_PWR_ADC_R1: u32 = (0x1 << 3);
pub const RT5668_PWR_ADC_R1_BIT: u32 = 3;
pub const RT5668_DIG_GATE_CTRL: u32 = (0x1 << 0);
pub const RT5668_DIG_GATE_CTRL_SFT: u32 = 0;


/* Power Management for Digital 2 (0x0062) */
pub const RT5668_PWR_ADC_S1F: u32 = (0x1 << 15);
pub const RT5668_PWR_ADC_S1F_BIT: u32 = 15;
pub const RT5668_PWR_DAC_S1F: u32 = (0x1 << 10);
pub const RT5668_PWR_DAC_S1F_BIT: u32 = 10;

/* Power Management for Analog 1 (0x0063) */
pub const RT5668_PWR_VREF1: u32 = (0x1 << 15);
pub const RT5668_PWR_VREF1_BIT: u32 = 15;
pub const RT5668_PWR_FV1: u32 = (0x1 << 14);
pub const RT5668_PWR_FV1_BIT: u32 = 14;
pub const RT5668_PWR_VREF2: u32 = (0x1 << 13);
pub const RT5668_PWR_VREF2_BIT: u32 = 13;
pub const RT5668_PWR_FV2: u32 = (0x1 << 12);
pub const RT5668_PWR_FV2_BIT: u32 = 12;
pub const RT5668_LDO1_DBG_MASK: u32 = (0x3 << 10);
pub const RT5668_PWR_MB: u32 = (0x1 << 9);
pub const RT5668_PWR_MB_BIT: u32 = 9;
pub const RT5668_PWR_BG: u32 = (0x1 << 7);
pub const RT5668_PWR_BG_BIT: u32 = 7;
pub const RT5668_LDO1_BYPASS_MASK: u32 = (0x1 << 6);
pub const RT5668_LDO1_BYPASS: u32 = (0x1 << 6);
pub const RT5668_LDO1_NOT_BYPASS: u32 = (0x0 << 6);
pub const RT5668_PWR_MA_BIT: u32 = 6;
pub const RT5668_LDO1_DVO_MASK: u32 = (0x3 << 4);
pub const RT5668_LDO1_DVO_09: u32 = (0x0 << 4);
pub const RT5668_LDO1_DVO_10: u32 = (0x1 << 4);
pub const RT5668_LDO1_DVO_12: u32 = (0x2 << 4);
pub const RT5668_LDO1_DVO_14: u32 = (0x3 << 4);
pub const RT5668_HP_DRIVER_MASK: u32 = (0x3 << 2);
pub const RT5668_HP_DRIVER_1X: u32 = (0x0 << 2);
pub const RT5668_HP_DRIVER_3X: u32 = (0x1 << 2);
pub const RT5668_HP_DRIVER_5X: u32 = (0x3 << 2);
pub const RT5668_PWR_HA_L: u32 = (0x1 << 1);
pub const RT5668_PWR_HA_L_BIT: u32 = 1;
pub const RT5668_PWR_HA_R: u32 = (0x1 << 0);
pub const RT5668_PWR_HA_R_BIT: u32 = 0;

/* Power Management for Analog 2 (0x0064) */
pub const RT5668_PWR_MB1: u32 = (0x1 << 11);
pub const RT5668_PWR_MB1_PWR_DOWN: u32 = (0x0 << 11);
pub const RT5668_PWR_MB1_BIT: u32 = 11;
pub const RT5668_PWR_MB2: u32 = (0x1 << 10);
pub const RT5668_PWR_MB2_PWR_DOWN: u32 = (0x0 << 10);
pub const RT5668_PWR_MB2_BIT: u32 = 10;
pub const RT5668_PWR_JDH: u32 = (0x1 << 3);
pub const RT5668_PWR_JDH_BIT: u32 = 3;
pub const RT5668_PWR_JDL: u32 = (0x1 << 2);
pub const RT5668_PWR_JDL_BIT: u32 = 2;
pub const RT5668_PWR_RM1_L: u32 = (0x1 << 1);
pub const RT5668_PWR_RM1_L_BIT: u32 = 1;

/* Power Management for Analog 3 (0x0065) */
pub const RT5668_PWR_CBJ: u32 = (0x1 << 9);
pub const RT5668_PWR_CBJ_BIT: u32 = 9;
pub const RT5668_PWR_PLL: u32 = (0x1 << 6);
pub const RT5668_PWR_PLL_BIT: u32 = 6;
pub const RT5668_PWR_PLL2B: u32 = (0x1 << 5);
pub const RT5668_PWR_PLL2B_BIT: u32 = 5;
pub const RT5668_PWR_PLL2F: u32 = (0x1 << 4);
pub const RT5668_PWR_PLL2F_BIT: u32 = 4;
pub const RT5668_PWR_LDO2: u32 = (0x1 << 2);
pub const RT5668_PWR_LDO2_BIT: u32 = 2;
pub const RT5668_PWR_DET_SPKVDD: u32 = (0x1 << 1);
pub const RT5668_PWR_DET_SPKVDD_BIT: u32 = 1;

/* Power Management for Mixer (0x0066) */
pub const RT5668_PWR_STO1_DAC_L: u32 = (0x1 << 5);
pub const RT5668_PWR_STO1_DAC_L_BIT: u32 = 5;
pub const RT5668_PWR_STO1_DAC_R: u32 = (0x1 << 4);
pub const RT5668_PWR_STO1_DAC_R_BIT: u32 = 4;

/* MCLK and System Clock Detection Control (0x006b) */
pub const RT5668_SYS_CLK_DET: u32 = (0x1 << 15);
pub const RT5668_SYS_CLK_DET_SFT: u32 = 15;
pub const RT5668_PLL1_CLK_DET: u32 = (0x1 << 14);
pub const RT5668_PLL1_CLK_DET_SFT: u32 = 14;
pub const RT5668_PLL2_CLK_DET: u32 = (0x1 << 13);
pub const RT5668_PLL2_CLK_DET_SFT: u32 = 13;
pub const RT5668_POW_CLK_DET2_SFT: u32 = 8;
pub const RT5668_POW_CLK_DET_SFT: u32 = 0;

/* Digital Microphone Control 1 (0x006e) */
pub const RT5668_DMIC_1_EN_MASK: u32 = (0x1 << 15);
pub const RT5668_DMIC_1_EN_SFT: u32 = 15;
pub const RT5668_DMIC_1_DIS: u32 = (0x0 << 15);
pub const RT5668_DMIC_1_EN: u32 = (0x1 << 15);
pub const RT5668_DMIC_1_DP_MASK: u32 = (0x3 << 4);
pub const RT5668_DMIC_1_DP_SFT: u32 = 4;
pub const RT5668_DMIC_1_DP_GPIO2: u32 = (0x0 << 4);
pub const RT5668_DMIC_1_DP_GPIO5: u32 = (0x1 << 4);
pub const RT5668_DMIC_CLK_MASK: u32 = (0xf << 0);
pub const RT5668_DMIC_CLK_SFT: u32 = 0;

/* I2S1 Audio Serial Data Port Control (0x0070) */
pub const RT5668_SEL_ADCDAT_MASK: u32 = (0x1 << 15);
pub const RT5668_SEL_ADCDAT_OUT: u32 = (0x0 << 15);
pub const RT5668_SEL_ADCDAT_IN: u32 = (0x1 << 15);
pub const RT5668_SEL_ADCDAT_SFT: u32 = 15;
pub const RT5668_I2S1_TX_CHL_MASK: u32 = (0x7 << 12);
pub const RT5668_I2S1_TX_CHL_SFT: u32 = 12;
pub const RT5668_I2S1_TX_CHL_16: u32 = (0x0 << 12);
pub const RT5668_I2S1_TX_CHL_20: u32 = (0x1 << 12);
pub const RT5668_I2S1_TX_CHL_24: u32 = (0x2 << 12);
pub const RT5668_I2S1_TX_CHL_32: u32 = (0x3 << 12);
pub const RT5668_I2S1_TX_CHL_8: u32 = (0x4 << 12);
pub const RT5668_I2S1_RX_CHL_MASK: u32 = (0x7 << 8);
pub const RT5668_I2S1_RX_CHL_SFT: u32 = 8;
pub const RT5668_I2S1_RX_CHL_16: u32 = (0x0 << 8);
pub const RT5668_I2S1_RX_CHL_20: u32 = (0x1 << 8);
pub const RT5668_I2S1_RX_CHL_24: u32 = (0x2 << 8);
pub const RT5668_I2S1_RX_CHL_32: u32 = (0x3 << 8);
pub const RT5668_I2S1_RX_CHL_8: u32 = (0x4 << 8);
pub const RT5668_I2S1_MONO_MASK: u32 = (0x1 << 7);
pub const RT5668_I2S1_MONO_EN: u32 = (0x1 << 7);
pub const RT5668_I2S1_MONO_DIS: u32 = (0x0 << 7);
pub const RT5668_I2S2_MONO_MASK: u32 = (0x1 << 6);
pub const RT5668_I2S2_MONO_EN: u32 = (0x1 << 6);
pub const RT5668_I2S2_MONO_DIS: u32 = (0x0 << 6);
pub const RT5668_I2S1_DL_MASK: u32 = (0x7 << 4);
pub const RT5668_I2S1_DL_SFT: u32 = 4;
pub const RT5668_I2S1_DL_16: u32 = (0x0 << 4);
pub const RT5668_I2S1_DL_20: u32 = (0x1 << 4);
pub const RT5668_I2S1_DL_24: u32 = (0x2 << 4);
pub const RT5668_I2S1_DL_32: u32 = (0x3 << 4);
pub const RT5668_I2S1_DL_8: u32 = (0x4 << 4);

/* I2S1/2 Audio Serial Data Port Control (0x0070)(0x0071) */
pub const RT5668_I2S2_MS_MASK: u32 = (0x1 << 15);
pub const RT5668_I2S2_MS_SFT: u32 = 15;
pub const RT5668_I2S2_MS_M: u32 = (0x0 << 15);
pub const RT5668_I2S2_MS_S: u32 = (0x1 << 15);
pub const RT5668_I2S2_PIN_CFG_MASK: u32 = (0x1 << 14);
pub const RT5668_I2S2_PIN_CFG_SFT: u32 = 14;
pub const RT5668_I2S2_CLK_SEL_MASK: u32 = (0x1 << 11);
pub const RT5668_I2S2_CLK_SEL_SFT: u32 = 11;
pub const RT5668_I2S2_OUT_MASK: u32 = (0x1 << 9);
pub const RT5668_I2S2_OUT_SFT: u32 = 9;
pub const RT5668_I2S2_OUT_UM: u32 = (0x0 << 9);
pub const RT5668_I2S2_OUT_M: u32 = (0x1 << 9);
pub const RT5668_I2S_BP_MASK: u32 = (0x1 << 8);
pub const RT5668_I2S_BP_SFT: u32 = 8;
pub const RT5668_I2S_BP_NOR: u32 = (0x0 << 8);
pub const RT5668_I2S_BP_INV: u32 = (0x1 << 8);
// Duplicate C macro redefinition: #define RT5668_I2S2_MONO_EN (0x1 << 6)
// Duplicate C macro redefinition: #define RT5668_I2S2_MONO_DIS (0x0 << 6)
pub const RT5668_I2S2_DL_MASK: u32 = (0x3 << 4);
pub const RT5668_I2S2_DL_SFT: u32 = 4;
pub const RT5668_I2S2_DL_16: u32 = (0x0 << 4);
pub const RT5668_I2S2_DL_20: u32 = (0x1 << 4);
pub const RT5668_I2S2_DL_24: u32 = (0x2 << 4);
pub const RT5668_I2S2_DL_8: u32 = (0x3 << 4);
pub const RT5668_I2S_DF_MASK: u32 = (0x7);
pub const RT5668_I2S_DF_SFT: u32 = 0;
pub const RT5668_I2S_DF_I2S: u32 = (0x0);
pub const RT5668_I2S_DF_LEFT: u32 = (0x1);
pub const RT5668_I2S_DF_PCM_A: u32 = (0x2);
pub const RT5668_I2S_DF_PCM_B: u32 = (0x3);
pub const RT5668_I2S_DF_PCM_A_N: u32 = (0x6);
pub const RT5668_I2S_DF_PCM_B_N: u32 = (0x7);

/* ADC/DAC Clock Control 1 (0x0073) */
pub const RT5668_ADC_OSR_MASK: u32 = (0xf << 12);
pub const RT5668_ADC_OSR_SFT: u32 = 12;
pub const RT5668_ADC_OSR_D_1: u32 = (0x0 << 12);
pub const RT5668_ADC_OSR_D_2: u32 = (0x1 << 12);
pub const RT5668_ADC_OSR_D_4: u32 = (0x2 << 12);
pub const RT5668_ADC_OSR_D_6: u32 = (0x3 << 12);
pub const RT5668_ADC_OSR_D_8: u32 = (0x4 << 12);
pub const RT5668_ADC_OSR_D_12: u32 = (0x5 << 12);
pub const RT5668_ADC_OSR_D_16: u32 = (0x6 << 12);
pub const RT5668_ADC_OSR_D_24: u32 = (0x7 << 12);
pub const RT5668_ADC_OSR_D_32: u32 = (0x8 << 12);
pub const RT5668_ADC_OSR_D_48: u32 = (0x9 << 12);
pub const RT5668_I2S_M_DIV_MASK: u32 = (0xf << 12);
pub const RT5668_I2S_M_DIV_SFT: u32 = 8;
pub const RT5668_I2S_M_D_1: u32 = (0x0 << 8);
pub const RT5668_I2S_M_D_2: u32 = (0x1 << 8);
pub const RT5668_I2S_M_D_3: u32 = (0x2 << 8);
pub const RT5668_I2S_M_D_4: u32 = (0x3 << 8);
pub const RT5668_I2S_M_D_6: u32 = (0x4 << 8);
pub const RT5668_I2S_M_D_8: u32 = (0x5 << 8);
pub const RT5668_I2S_M_D_12: u32 = (0x6 << 8);
pub const RT5668_I2S_M_D_16: u32 = (0x7 << 8);
pub const RT5668_I2S_M_D_24: u32 = (0x8 << 8);
pub const RT5668_I2S_M_D_32: u32 = (0x9 << 8);
pub const RT5668_I2S_M_D_48: u32 = (0x10 << 8);
pub const RT5668_I2S_CLK_SRC_MASK: u32 = (0x7 << 4);
pub const RT5668_I2S_CLK_SRC_SFT: u32 = 4;
pub const RT5668_I2S_CLK_SRC_MCLK: u32 = (0x0 << 4);
pub const RT5668_I2S_CLK_SRC_PLL1: u32 = (0x1 << 4);
pub const RT5668_I2S_CLK_SRC_PLL2: u32 = (0x2 << 4);
pub const RT5668_I2S_CLK_SRC_SDW: u32 = (0x3 << 4);
pub const RT5668_I2S_CLK_SRC_RCCLK: u32 = (0x4 << 4) /* 25M */;
pub const RT5668_DAC_OSR_MASK: u32 = (0xf << 0);
pub const RT5668_DAC_OSR_SFT: u32 = 0;
pub const RT5668_DAC_OSR_D_1: u32 = (0x0 << 0);
pub const RT5668_DAC_OSR_D_2: u32 = (0x1 << 0);
pub const RT5668_DAC_OSR_D_4: u32 = (0x2 << 0);
pub const RT5668_DAC_OSR_D_6: u32 = (0x3 << 0);
pub const RT5668_DAC_OSR_D_8: u32 = (0x4 << 0);
pub const RT5668_DAC_OSR_D_12: u32 = (0x5 << 0);
pub const RT5668_DAC_OSR_D_16: u32 = (0x6 << 0);
pub const RT5668_DAC_OSR_D_24: u32 = (0x7 << 0);
pub const RT5668_DAC_OSR_D_32: u32 = (0x8 << 0);
pub const RT5668_DAC_OSR_D_48: u32 = (0x9 << 0);

/* ADC/DAC Clock Control 2 (0x0074) */
pub const RT5668_I2S2_BCLK_MS2_MASK: u32 = (0x1 << 11);
pub const RT5668_I2S2_BCLK_MS2_SFT: u32 = 11;
pub const RT5668_I2S2_BCLK_MS2_32: u32 = (0x0 << 11);
pub const RT5668_I2S2_BCLK_MS2_64: u32 = (0x1 << 11);


/* TDM control 1 (0x0079) */
pub const RT5668_TDM_TX_CH_MASK: u32 = (0x3 << 12);
pub const RT5668_TDM_TX_CH_2: u32 = (0x0 << 12);
pub const RT5668_TDM_TX_CH_4: u32 = (0x1 << 12);
pub const RT5668_TDM_TX_CH_6: u32 = (0x2 << 12);
pub const RT5668_TDM_TX_CH_8: u32 = (0x3 << 12);
pub const RT5668_TDM_RX_CH_MASK: u32 = (0x3 << 8);
pub const RT5668_TDM_RX_CH_2: u32 = (0x0 << 8);
pub const RT5668_TDM_RX_CH_4: u32 = (0x1 << 8);
pub const RT5668_TDM_RX_CH_6: u32 = (0x2 << 8);
pub const RT5668_TDM_RX_CH_8: u32 = (0x3 << 8);
pub const RT5668_TDM_ADC_LCA_MASK: u32 = (0xf << 4);
pub const RT5668_TDM_ADC_LCA_SFT: u32 = 4;
pub const RT5668_TDM_ADC_DL_SFT: u32 = 0;

/* TDM control 3 (0x007a) */
pub const RT5668_IF1_ADC1_SEL_SFT: u32 = 14;
pub const RT5668_IF1_ADC2_SEL_SFT: u32 = 12;
pub const RT5668_IF1_ADC3_SEL_SFT: u32 = 10;
pub const RT5668_IF1_ADC4_SEL_SFT: u32 = 8;
pub const RT5668_TDM_ADC_SEL_SFT: u32 = 4;

/* TDM/I2S control (0x007e) */
pub const RT5668_TDM_S_BP_MASK: u32 = (0x1 << 15);
pub const RT5668_TDM_S_BP_SFT: u32 = 15;
pub const RT5668_TDM_S_BP_NOR: u32 = (0x0 << 15);
pub const RT5668_TDM_S_BP_INV: u32 = (0x1 << 15);
pub const RT5668_TDM_S_LP_MASK: u32 = (0x1 << 14);
pub const RT5668_TDM_S_LP_SFT: u32 = 14;
pub const RT5668_TDM_S_LP_NOR: u32 = (0x0 << 14);
pub const RT5668_TDM_S_LP_INV: u32 = (0x1 << 14);
pub const RT5668_TDM_DF_MASK: u32 = (0x7 << 11);
pub const RT5668_TDM_DF_SFT: u32 = 11;
pub const RT5668_TDM_DF_I2S: u32 = (0x0 << 11);
pub const RT5668_TDM_DF_LEFT: u32 = (0x1 << 11);
pub const RT5668_TDM_DF_PCM_A: u32 = (0x2 << 11);
pub const RT5668_TDM_DF_PCM_B: u32 = (0x3 << 11);
pub const RT5668_TDM_DF_PCM_A_N: u32 = (0x6 << 11);
pub const RT5668_TDM_DF_PCM_B_N: u32 = (0x7 << 11);
pub const RT5668_TDM_CL_MASK: u32 = (0x3 << 4);
pub const RT5668_TDM_CL_16: u32 = (0x0 << 4);
pub const RT5668_TDM_CL_20: u32 = (0x1 << 4);
pub const RT5668_TDM_CL_24: u32 = (0x2 << 4);
pub const RT5668_TDM_CL_32: u32 = (0x3 << 4);
pub const RT5668_TDM_M_BP_MASK: u32 = (0x1 << 2);
pub const RT5668_TDM_M_BP_SFT: u32 = 2;
pub const RT5668_TDM_M_BP_NOR: u32 = (0x0 << 2);
pub const RT5668_TDM_M_BP_INV: u32 = (0x1 << 2);
pub const RT5668_TDM_M_LP_MASK: u32 = (0x1 << 1);
pub const RT5668_TDM_M_LP_SFT: u32 = 1;
pub const RT5668_TDM_M_LP_NOR: u32 = (0x0 << 1);
pub const RT5668_TDM_M_LP_INV: u32 = (0x1 << 1);
pub const RT5668_TDM_MS_MASK: u32 = (0x1 << 0);
pub const RT5668_TDM_MS_SFT: u32 = 0;
pub const RT5668_TDM_MS_M: u32 = (0x0 << 0);
pub const RT5668_TDM_MS_S: u32 = (0x1 << 0);

/* Global Clock Control (0x0080) */
pub const RT5668_SCLK_SRC_MASK: u32 = (0x7 << 13);
pub const RT5668_SCLK_SRC_SFT: u32 = 13;
pub const RT5668_SCLK_SRC_MCLK: u32 = (0x0 << 13);
pub const RT5668_SCLK_SRC_PLL1: u32 = (0x1 << 13);
pub const RT5668_SCLK_SRC_PLL2: u32 = (0x2 << 13);
pub const RT5668_SCLK_SRC_SDW: u32 = (0x3 << 13);
pub const RT5668_SCLK_SRC_RCCLK: u32 = (0x4 << 13);
pub const RT5668_PLL1_SRC_MASK: u32 = (0x3 << 10);
pub const RT5668_PLL1_SRC_SFT: u32 = 10;
pub const RT5668_PLL1_SRC_MCLK: u32 = (0x0 << 10);
pub const RT5668_PLL1_SRC_BCLK1: u32 = (0x1 << 10);
pub const RT5668_PLL1_SRC_SDW: u32 = (0x2 << 10);
pub const RT5668_PLL1_SRC_RC: u32 = (0x3 << 10);
pub const RT5668_PLL2_SRC_MASK: u32 = (0x3 << 8);
pub const RT5668_PLL2_SRC_SFT: u32 = 8;
pub const RT5668_PLL2_SRC_MCLK: u32 = (0x0 << 8);
pub const RT5668_PLL2_SRC_BCLK1: u32 = (0x1 << 8);
pub const RT5668_PLL2_SRC_SDW: u32 = (0x2 << 8);
pub const RT5668_PLL2_SRC_RC: u32 = (0x3 << 8);



pub const RT5668_PLL_INP_MAX: u32 = 40000000;
pub const RT5668_PLL_INP_MIN: u32 = 256000;
/* PLL M/N/K Code Control 1 (0x0081) */
pub const RT5668_PLL_N_MAX: u32 = 0x001ff;
pub const RT5668_PLL_N_MASK: u32 = (RT5668_PLL_N_MAX << 7);
pub const RT5668_PLL_N_SFT: u32 = 7;
pub const RT5668_PLL_K_MAX: u32 = 0x001f;
pub const RT5668_PLL_K_MASK: u32 = (RT5668_PLL_K_MAX);
pub const RT5668_PLL_K_SFT: u32 = 0;

/* PLL M/N/K Code Control 2 (0x0082) */
pub const RT5668_PLL_M_MAX: u32 = 0x00f;
pub const RT5668_PLL_M_MASK: u32 = (RT5668_PLL_M_MAX << 12);
pub const RT5668_PLL_M_SFT: u32 = 12;
pub const RT5668_PLL_M_BP: u32 = (0x1 << 11);
pub const RT5668_PLL_M_BP_SFT: u32 = 11;
pub const RT5668_PLL_K_BP: u32 = (0x1 << 10);
pub const RT5668_PLL_K_BP_SFT: u32 = 10;

/* PLL tracking mode 1 (0x0083) */
pub const RT5668_DA_ASRC_MASK: u32 = (0x1 << 13);
pub const RT5668_DA_ASRC_SFT: u32 = 13;
pub const RT5668_DAC_STO1_ASRC_MASK: u32 = (0x1 << 12);
pub const RT5668_DAC_STO1_ASRC_SFT: u32 = 12;
pub const RT5668_AD_ASRC_MASK: u32 = (0x1 << 8);
pub const RT5668_AD_ASRC_SFT: u32 = 8;
pub const RT5668_AD_ASRC_SEL_MASK: u32 = (0x1 << 4);
pub const RT5668_AD_ASRC_SEL_SFT: u32 = 4;
pub const RT5668_DMIC_ASRC_MASK: u32 = (0x1 << 3);
pub const RT5668_DMIC_ASRC_SFT: u32 = 3;
pub const RT5668_ADC_STO1_ASRC_MASK: u32 = (0x1 << 2);
pub const RT5668_ADC_STO1_ASRC_SFT: u32 = 2;
pub const RT5668_DA_ASRC_SEL_MASK: u32 = (0x1 << 0);
pub const RT5668_DA_ASRC_SEL_SFT: u32 = 0;

/* PLL tracking mode 2 3 (0x0084)(0x0085)*/
pub const RT5668_FILTER_CLK_SEL_MASK: u32 = (0x7 << 12);
pub const RT5668_FILTER_CLK_SEL_SFT: u32 = 12;

/* ASRC Control 4 (0x0086) */
pub const RT5668_ASRCIN_FTK_N1_MASK: u32 = (0x3 << 14);
pub const RT5668_ASRCIN_FTK_N1_SFT: u32 = 14;
pub const RT5668_ASRCIN_FTK_N2_MASK: u32 = (0x3 << 12);
pub const RT5668_ASRCIN_FTK_N2_SFT: u32 = 12;
pub const RT5668_ASRCIN_FTK_M1_MASK: u32 = (0x7 << 8);
pub const RT5668_ASRCIN_FTK_M1_SFT: u32 = 8;
pub const RT5668_ASRCIN_FTK_M2_MASK: u32 = (0x7 << 4);
pub const RT5668_ASRCIN_FTK_M2_SFT: u32 = 4;

/* SoundWire reference clk (0x008d) */
pub const RT5668_PLL2_OUT_MASK: u32 = (0x1 << 8);
pub const RT5668_PLL2_OUT_98M: u32 = (0x0 << 8);
pub const RT5668_PLL2_OUT_49M: u32 = (0x1 << 8);
pub const RT5668_SDW_REF_2_MASK: u32 = (0xf << 4);
pub const RT5668_SDW_REF_2_SFT: u32 = 4;
pub const RT5668_SDW_REF_2_48K: u32 = (0x0 << 4);
pub const RT5668_SDW_REF_2_96K: u32 = (0x1 << 4);
pub const RT5668_SDW_REF_2_192K: u32 = (0x2 << 4);
pub const RT5668_SDW_REF_2_32K: u32 = (0x3 << 4);
pub const RT5668_SDW_REF_2_24K: u32 = (0x4 << 4);
pub const RT5668_SDW_REF_2_16K: u32 = (0x5 << 4);
pub const RT5668_SDW_REF_2_12K: u32 = (0x6 << 4);
pub const RT5668_SDW_REF_2_8K: u32 = (0x7 << 4);
pub const RT5668_SDW_REF_2_44K: u32 = (0x8 << 4);
pub const RT5668_SDW_REF_2_88K: u32 = (0x9 << 4);
pub const RT5668_SDW_REF_2_176K: u32 = (0xa << 4);
pub const RT5668_SDW_REF_2_353K: u32 = (0xb << 4);
pub const RT5668_SDW_REF_2_22K: u32 = (0xc << 4);
pub const RT5668_SDW_REF_2_384K: u32 = (0xd << 4);
pub const RT5668_SDW_REF_2_11K: u32 = (0xe << 4);
pub const RT5668_SDW_REF_1_MASK: u32 = (0xf << 0);
pub const RT5668_SDW_REF_1_SFT: u32 = 0;
pub const RT5668_SDW_REF_1_48K: u32 = (0x0 << 0);
pub const RT5668_SDW_REF_1_96K: u32 = (0x1 << 0);
pub const RT5668_SDW_REF_1_192K: u32 = (0x2 << 0);
pub const RT5668_SDW_REF_1_32K: u32 = (0x3 << 0);
pub const RT5668_SDW_REF_1_24K: u32 = (0x4 << 0);
pub const RT5668_SDW_REF_1_16K: u32 = (0x5 << 0);
pub const RT5668_SDW_REF_1_12K: u32 = (0x6 << 0);
pub const RT5668_SDW_REF_1_8K: u32 = (0x7 << 0);
pub const RT5668_SDW_REF_1_44K: u32 = (0x8 << 0);
pub const RT5668_SDW_REF_1_88K: u32 = (0x9 << 0);
pub const RT5668_SDW_REF_1_176K: u32 = (0xa << 0);
pub const RT5668_SDW_REF_1_353K: u32 = (0xb << 0);
pub const RT5668_SDW_REF_1_22K: u32 = (0xc << 0);
pub const RT5668_SDW_REF_1_384K: u32 = (0xd << 0);
pub const RT5668_SDW_REF_1_11K: u32 = (0xe << 0);

/* Depop Mode Control 1 (0x008e) */
pub const RT5668_PUMP_EN: u32 = (0x1 << 3);
pub const RT5668_PUMP_EN_SFT: u32 = 3;
pub const RT5668_CAPLESS_EN: u32 = (0x1 << 0);
pub const RT5668_CAPLESS_EN_SFT: u32 = 0;

/* Depop Mode Control 2 (0x8f) */
pub const RT5668_RAMP_MASK: u32 = (0x1 << 12);
pub const RT5668_RAMP_SFT: u32 = 12;
pub const RT5668_RAMP_DIS: u32 = (0x0 << 12);
pub const RT5668_RAMP_EN: u32 = (0x1 << 12);
pub const RT5668_BPS_MASK: u32 = (0x1 << 11);
pub const RT5668_BPS_SFT: u32 = 11;
pub const RT5668_BPS_DIS: u32 = (0x0 << 11);
pub const RT5668_BPS_EN: u32 = (0x1 << 11);
pub const RT5668_FAST_UPDN_MASK: u32 = (0x1 << 10);
pub const RT5668_FAST_UPDN_SFT: u32 = 10;
pub const RT5668_FAST_UPDN_DIS: u32 = (0x0 << 10);
pub const RT5668_FAST_UPDN_EN: u32 = (0x1 << 10);
pub const RT5668_VLO_MASK: u32 = (0x1 << 7);
pub const RT5668_VLO_SFT: u32 = 7;
pub const RT5668_VLO_3V: u32 = (0x0 << 7);
pub const RT5668_VLO_33V: u32 = (0x1 << 7);

/* HPOUT charge pump 1 (0x0091) */
pub const RT5668_OSW_L_MASK: u32 = (0x1 << 11);
pub const RT5668_OSW_L_SFT: u32 = 11;
pub const RT5668_OSW_L_DIS: u32 = (0x0 << 11);
pub const RT5668_OSW_L_EN: u32 = (0x1 << 11);
pub const RT5668_OSW_R_MASK: u32 = (0x1 << 10);
pub const RT5668_OSW_R_SFT: u32 = 10;
pub const RT5668_OSW_R_DIS: u32 = (0x0 << 10);
pub const RT5668_OSW_R_EN: u32 = (0x1 << 10);
pub const RT5668_PM_HP_MASK: u32 = (0x3 << 8);
pub const RT5668_PM_HP_SFT: u32 = 8;
pub const RT5668_PM_HP_LV: u32 = (0x0 << 8);
pub const RT5668_PM_HP_MV: u32 = (0x1 << 8);
pub const RT5668_PM_HP_HV: u32 = (0x2 << 8);
pub const RT5668_IB_HP_MASK: u32 = (0x3 << 6);
pub const RT5668_IB_HP_SFT: u32 = 6;
pub const RT5668_IB_HP_125IL: u32 = (0x0 << 6);
pub const RT5668_IB_HP_25IL: u32 = (0x1 << 6);
pub const RT5668_IB_HP_5IL: u32 = (0x2 << 6);
pub const RT5668_IB_HP_1IL: u32 = (0x3 << 6);

/* Micbias Control1 (0x93) */
pub const RT5668_MIC1_OV_MASK: u32 = (0x3 << 14);
pub const RT5668_MIC1_OV_SFT: u32 = 14;
pub const RT5668_MIC1_OV_2V7: u32 = (0x0 << 14);
pub const RT5668_MIC1_OV_2V4: u32 = (0x1 << 14);
pub const RT5668_MIC1_OV_2V25: u32 = (0x3 << 14);
pub const RT5668_MIC1_OV_1V8: u32 = (0x4 << 14);
pub const RT5668_MIC1_CLK_MASK: u32 = (0x1 << 13);
pub const RT5668_MIC1_CLK_SFT: u32 = 13;
pub const RT5668_MIC1_CLK_DIS: u32 = (0x0 << 13);
pub const RT5668_MIC1_CLK_EN: u32 = (0x1 << 13);
pub const RT5668_MIC1_OVCD_MASK: u32 = (0x1 << 12);
pub const RT5668_MIC1_OVCD_SFT: u32 = 12;
pub const RT5668_MIC1_OVCD_DIS: u32 = (0x0 << 12);
pub const RT5668_MIC1_OVCD_EN: u32 = (0x1 << 12);
pub const RT5668_MIC1_OVTH_MASK: u32 = (0x3 << 10);
pub const RT5668_MIC1_OVTH_SFT: u32 = 10;
pub const RT5668_MIC1_OVTH_768UA: u32 = (0x0 << 10);
pub const RT5668_MIC1_OVTH_960UA: u32 = (0x1 << 10);
pub const RT5668_MIC1_OVTH_1152UA: u32 = (0x2 << 10);
pub const RT5668_MIC1_OVTH_1960UA: u32 = (0x3 << 10);
pub const RT5668_MIC2_OV_MASK: u32 = (0x3 << 8);
pub const RT5668_MIC2_OV_SFT: u32 = 8;
pub const RT5668_MIC2_OV_2V7: u32 = (0x0 << 8);
pub const RT5668_MIC2_OV_2V4: u32 = (0x1 << 8);
pub const RT5668_MIC2_OV_2V25: u32 = (0x3 << 8);
pub const RT5668_MIC2_OV_1V8: u32 = (0x4 << 8);
pub const RT5668_MIC2_CLK_MASK: u32 = (0x1 << 7);
pub const RT5668_MIC2_CLK_SFT: u32 = 7;
pub const RT5668_MIC2_CLK_DIS: u32 = (0x0 << 7);
pub const RT5668_MIC2_CLK_EN: u32 = (0x1 << 7);
pub const RT5668_MIC2_OVTH_MASK: u32 = (0x3 << 4);
pub const RT5668_MIC2_OVTH_SFT: u32 = 4;
pub const RT5668_MIC2_OVTH_768UA: u32 = (0x0 << 4);
pub const RT5668_MIC2_OVTH_960UA: u32 = (0x1 << 4);
pub const RT5668_MIC2_OVTH_1152UA: u32 = (0x2 << 4);
pub const RT5668_MIC2_OVTH_1960UA: u32 = (0x3 << 4);
pub const RT5668_PWR_MB_MASK: u32 = (0x1 << 3);
pub const RT5668_PWR_MB_SFT: u32 = 3;
pub const RT5668_PWR_MB_PD: u32 = (0x0 << 3);
pub const RT5668_PWR_MB_PU: u32 = (0x1 << 3);

/* Micbias Control2 (0x0094) */
pub const RT5668_PWR_CLK25M_MASK: u32 = (0x1 << 9);
pub const RT5668_PWR_CLK25M_SFT: u32 = 9;
pub const RT5668_PWR_CLK25M_PD: u32 = (0x0 << 9);
pub const RT5668_PWR_CLK25M_PU: u32 = (0x1 << 9);
pub const RT5668_PWR_CLK1M_MASK: u32 = (0x1 << 8);
pub const RT5668_PWR_CLK1M_SFT: u32 = 8;
pub const RT5668_PWR_CLK1M_PD: u32 = (0x0 << 8);
pub const RT5668_PWR_CLK1M_PU: u32 = (0x1 << 8);

/* RC Clock Control (0x009f) */
pub const RT5668_POW_IRQ: u32 = (0x1 << 15);
pub const RT5668_POW_JDH: u32 = (0x1 << 14);
pub const RT5668_POW_JDL: u32 = (0x1 << 13);
pub const RT5668_POW_ANA: u32 = (0x1 << 12);

/* I2S Master Mode Clock Control 1 (0x00a0) */
pub const RT5668_CLK_SRC_MCLK: u32 = (0x0);
pub const RT5668_CLK_SRC_PLL1: u32 = (0x1);
pub const RT5668_CLK_SRC_PLL2: u32 = (0x2);
pub const RT5668_CLK_SRC_SDW: u32 = (0x3);
pub const RT5668_CLK_SRC_RCCLK: u32 = (0x4);
pub const RT5668_I2S_PD_1: u32 = (0x0);
pub const RT5668_I2S_PD_2: u32 = (0x1);
pub const RT5668_I2S_PD_3: u32 = (0x2);
pub const RT5668_I2S_PD_4: u32 = (0x3);
pub const RT5668_I2S_PD_6: u32 = (0x4);
pub const RT5668_I2S_PD_8: u32 = (0x5);
pub const RT5668_I2S_PD_12: u32 = (0x6);
pub const RT5668_I2S_PD_16: u32 = (0x7);
pub const RT5668_I2S_PD_24: u32 = (0x8);
pub const RT5668_I2S_PD_32: u32 = (0x9);
pub const RT5668_I2S_PD_48: u32 = (0xa);
pub const RT5668_I2S2_SRC_MASK: u32 = (0x3 << 4);
pub const RT5668_I2S2_SRC_SFT: u32 = 4;
pub const RT5668_I2S2_M_PD_MASK: u32 = (0xf << 0);
pub const RT5668_I2S2_M_PD_SFT: u32 = 0;

/* IRQ Control 1 (0x00b6) */
pub const RT5668_JD1_PULSE_EN_MASK: u32 = (0x1 << 10);
pub const RT5668_JD1_PULSE_EN_SFT: u32 = 10;
pub const RT5668_JD1_PULSE_DIS: u32 = (0x0 << 10);
pub const RT5668_JD1_PULSE_EN: u32 = (0x1 << 10);

/* IRQ Control 2 (0x00b7) */
pub const RT5668_JD1_EN_MASK: u32 = (0x1 << 15);
pub const RT5668_JD1_EN_SFT: u32 = 15;
pub const RT5668_JD1_DIS: u32 = (0x0 << 15);
pub const RT5668_JD1_EN: u32 = (0x1 << 15);
pub const RT5668_JD1_POL_MASK: u32 = (0x1 << 13);
pub const RT5668_JD1_POL_NOR: u32 = (0x0 << 13);
pub const RT5668_JD1_POL_INV: u32 = (0x1 << 13);

/* IRQ Control 3 (0x00b8) */
pub const RT5668_IL_IRQ_MASK: u32 = (0x1 << 7);
pub const RT5668_IL_IRQ_DIS: u32 = (0x0 << 7);
pub const RT5668_IL_IRQ_EN: u32 = (0x1 << 7);

/* GPIO Control 1 (0x00c0) */
pub const RT5668_GP1_PIN_MASK: u32 = (0x3 << 14);
pub const RT5668_GP1_PIN_SFT: u32 = 14;
pub const RT5668_GP1_PIN_GPIO1: u32 = (0x0 << 14);
pub const RT5668_GP1_PIN_IRQ: u32 = (0x1 << 14);
pub const RT5668_GP1_PIN_DMIC_CLK: u32 = (0x2 << 14);
pub const RT5668_GP2_PIN_MASK: u32 = (0x3 << 12);
pub const RT5668_GP2_PIN_SFT: u32 = 12;
pub const RT5668_GP2_PIN_GPIO2: u32 = (0x0 << 12);
pub const RT5668_GP2_PIN_LRCK2: u32 = (0x1 << 12);
pub const RT5668_GP2_PIN_DMIC_SDA: u32 = (0x2 << 12);
pub const RT5668_GP3_PIN_MASK: u32 = (0x3 << 10);
pub const RT5668_GP3_PIN_SFT: u32 = 10;
pub const RT5668_GP3_PIN_GPIO3: u32 = (0x0 << 10);
pub const RT5668_GP3_PIN_BCLK2: u32 = (0x1 << 10);
pub const RT5668_GP3_PIN_DMIC_CLK: u32 = (0x2 << 10);
pub const RT5668_GP4_PIN_MASK: u32 = (0x3 << 8);
pub const RT5668_GP4_PIN_SFT: u32 = 8;
pub const RT5668_GP4_PIN_GPIO4: u32 = (0x0 << 8);
pub const RT5668_GP4_PIN_ADCDAT1: u32 = (0x1 << 8);
pub const RT5668_GP4_PIN_DMIC_CLK: u32 = (0x2 << 8);
pub const RT5668_GP4_PIN_ADCDAT2: u32 = (0x3 << 8);
pub const RT5668_GP5_PIN_MASK: u32 = (0x3 << 6);
pub const RT5668_GP5_PIN_SFT: u32 = 6;
pub const RT5668_GP5_PIN_GPIO5: u32 = (0x0 << 6);
pub const RT5668_GP5_PIN_DACDAT1: u32 = (0x1 << 6);
pub const RT5668_GP5_PIN_DMIC_SDA: u32 = (0x2 << 6);
pub const RT5668_GP6_PIN_MASK: u32 = (0x1 << 5);
pub const RT5668_GP6_PIN_SFT: u32 = 5;
pub const RT5668_GP6_PIN_GPIO6: u32 = (0x0 << 5);
pub const RT5668_GP6_PIN_LRCK1: u32 = (0x1 << 5);

/* GPIO Control 2 (0x00c1)*/
pub const RT5668_GP1_PF_MASK: u32 = (0x1 << 15);
pub const RT5668_GP1_PF_IN: u32 = (0x0 << 15);
pub const RT5668_GP1_PF_OUT: u32 = (0x1 << 15);
pub const RT5668_GP1_OUT_MASK: u32 = (0x1 << 14);
pub const RT5668_GP1_OUT_L: u32 = (0x0 << 14);
pub const RT5668_GP1_OUT_H: u32 = (0x1 << 14);
pub const RT5668_GP2_PF_MASK: u32 = (0x1 << 13);
pub const RT5668_GP2_PF_IN: u32 = (0x0 << 13);
pub const RT5668_GP2_PF_OUT: u32 = (0x1 << 13);
pub const RT5668_GP2_OUT_MASK: u32 = (0x1 << 12);
pub const RT5668_GP2_OUT_L: u32 = (0x0 << 12);
pub const RT5668_GP2_OUT_H: u32 = (0x1 << 12);
pub const RT5668_GP3_PF_MASK: u32 = (0x1 << 11);
pub const RT5668_GP3_PF_IN: u32 = (0x0 << 11);
pub const RT5668_GP3_PF_OUT: u32 = (0x1 << 11);
pub const RT5668_GP3_OUT_MASK: u32 = (0x1 << 10);
pub const RT5668_GP3_OUT_L: u32 = (0x0 << 10);
pub const RT5668_GP3_OUT_H: u32 = (0x1 << 10);
pub const RT5668_GP4_PF_MASK: u32 = (0x1 << 9);
pub const RT5668_GP4_PF_IN: u32 = (0x0 << 9);
pub const RT5668_GP4_PF_OUT: u32 = (0x1 << 9);
pub const RT5668_GP4_OUT_MASK: u32 = (0x1 << 8);
pub const RT5668_GP4_OUT_L: u32 = (0x0 << 8);
pub const RT5668_GP4_OUT_H: u32 = (0x1 << 8);
pub const RT5668_GP5_PF_MASK: u32 = (0x1 << 7);
pub const RT5668_GP5_PF_IN: u32 = (0x0 << 7);
pub const RT5668_GP5_PF_OUT: u32 = (0x1 << 7);
pub const RT5668_GP5_OUT_MASK: u32 = (0x1 << 6);
pub const RT5668_GP5_OUT_L: u32 = (0x0 << 6);
pub const RT5668_GP5_OUT_H: u32 = (0x1 << 6);
pub const RT5668_GP6_PF_MASK: u32 = (0x1 << 5);
pub const RT5668_GP6_PF_IN: u32 = (0x0 << 5);
pub const RT5668_GP6_PF_OUT: u32 = (0x1 << 5);
pub const RT5668_GP6_OUT_MASK: u32 = (0x1 << 4);
pub const RT5668_GP6_OUT_L: u32 = (0x0 << 4);
pub const RT5668_GP6_OUT_H: u32 = (0x1 << 4);


/* GPIO Status (0x00c2) */
pub const RT5668_GP6_STA: u32 = (0x1 << 6);
pub const RT5668_GP5_STA: u32 = (0x1 << 5);
pub const RT5668_GP4_STA: u32 = (0x1 << 4);
pub const RT5668_GP3_STA: u32 = (0x1 << 3);
pub const RT5668_GP2_STA: u32 = (0x1 << 2);
pub const RT5668_GP1_STA: u32 = (0x1 << 1);

/* Soft volume and zero cross control 1 (0x00d9) */
pub const RT5668_SV_MASK: u32 = (0x1 << 15);
pub const RT5668_SV_SFT: u32 = 15;
pub const RT5668_SV_DIS: u32 = (0x0 << 15);
pub const RT5668_SV_EN: u32 = (0x1 << 15);
pub const RT5668_ZCD_MASK: u32 = (0x1 << 10);
pub const RT5668_ZCD_SFT: u32 = 10;
pub const RT5668_ZCD_PD: u32 = (0x0 << 10);
pub const RT5668_ZCD_PU: u32 = (0x1 << 10);
pub const RT5668_SV_DLY_MASK: u32 = (0xf);
pub const RT5668_SV_DLY_SFT: u32 = 0;

/* Soft volume and zero cross control 2 (0x00da) */
pub const RT5668_ZCD_BST1_CBJ_MASK: u32 = (0x1 << 7);
pub const RT5668_ZCD_BST1_CBJ_SFT: u32 = 7;
pub const RT5668_ZCD_BST1_CBJ_DIS: u32 = (0x0 << 7);
pub const RT5668_ZCD_BST1_CBJ_EN: u32 = (0x1 << 7);
pub const RT5668_ZCD_RECMIX_MASK: u32 = (0x1);
pub const RT5668_ZCD_RECMIX_SFT: u32 = 0;
pub const RT5668_ZCD_RECMIX_DIS: u32 = (0x0);
pub const RT5668_ZCD_RECMIX_EN: u32 = (0x1);

/* 4 Button Inline Command Control 2 (0x00e3) */
pub const RT5668_4BTN_IL_MASK: u32 = (0x1 << 15);
pub const RT5668_4BTN_IL_EN: u32 = (0x1 << 15);
pub const RT5668_4BTN_IL_DIS: u32 = (0x0 << 15);
pub const RT5668_4BTN_IL_RST_MASK: u32 = (0x1 << 14);
pub const RT5668_4BTN_IL_NOR: u32 = (0x1 << 14);
pub const RT5668_4BTN_IL_RST: u32 = (0x0 << 14);

/* Analog JD Control (0x00f0) */
pub const RT5668_JDH_RS_MASK: u32 = (0x1 << 4);
pub const RT5668_JDH_NO_PLUG: u32 = (0x1 << 4);
pub const RT5668_JDH_PLUG: u32 = (0x0 << 4);

/* Chopper and Clock control for DAC (0x013a)*/
pub const RT5668_CKXEN_DAC1_MASK: u32 = (0x1 << 13);
pub const RT5668_CKXEN_DAC1_SFT: u32 = 13;
pub const RT5668_CKGEN_DAC1_MASK: u32 = (0x1 << 12);
pub const RT5668_CKGEN_DAC1_SFT: u32 = 12;

/* Chopper and Clock control for ADC (0x013b)*/
pub const RT5668_CKXEN_ADC1_MASK: u32 = (0x1 << 13);
pub const RT5668_CKXEN_ADC1_SFT: u32 = 13;
pub const RT5668_CKGEN_ADC1_MASK: u32 = (0x1 << 12);
pub const RT5668_CKGEN_ADC1_SFT: u32 = 12;

/* Volume test (0x013f)*/
pub const RT5668_SEL_CLK_VOL_MASK: u32 = (0x1 << 15);
pub const RT5668_SEL_CLK_VOL_EN: u32 = (0x1 << 15);
pub const RT5668_SEL_CLK_VOL_DIS: u32 = (0x0 << 15);

/* Test Mode Control 1 (0x0145) */
pub const RT5668_AD2DA_LB_MASK: u32 = (0x1 << 10);
pub const RT5668_AD2DA_LB_SFT: u32 = 10;

/* Stereo Noise Gate Control 1 (0x0160) */
pub const RT5668_NG2_EN_MASK: u32 = (0x1 << 15);
pub const RT5668_NG2_EN: u32 = (0x1 << 15);
pub const RT5668_NG2_DIS: u32 = (0x0 << 15);

/* Stereo1 DAC Silence Detection Control (0x0190) */
pub const RT5668_DEB_STO_DAC_MASK: u32 = (0x7 << 4);
pub const RT5668_DEB_80_MS: u32 = (0x0 << 4);

/* SAR ADC Inline Command Control 1 (0x0210) */
pub const RT5668_SAR_BUTT_DET_MASK: u32 = (0x1 << 15);
pub const RT5668_SAR_BUTT_DET_EN: u32 = (0x1 << 15);
pub const RT5668_SAR_BUTT_DET_DIS: u32 = (0x0 << 15);
pub const RT5668_SAR_BUTDET_MODE_MASK: u32 = (0x1 << 14);
pub const RT5668_SAR_BUTDET_POW_SAV: u32 = (0x1 << 14);
pub const RT5668_SAR_BUTDET_POW_NORM: u32 = (0x0 << 14);
pub const RT5668_SAR_BUTDET_RST_MASK: u32 = (0x1 << 13);
pub const RT5668_SAR_BUTDET_RST_NORMAL: u32 = (0x1 << 13);
pub const RT5668_SAR_BUTDET_RST: u32 = (0x0 << 13);
pub const RT5668_SAR_POW_MASK: u32 = (0x1 << 12);
pub const RT5668_SAR_POW_EN: u32 = (0x1 << 12);
pub const RT5668_SAR_POW_DIS: u32 = (0x0 << 12);
pub const RT5668_SAR_RST_MASK: u32 = (0x1 << 11);
pub const RT5668_SAR_RST_NORMAL: u32 = (0x1 << 11);
pub const RT5668_SAR_RST: u32 = (0x0 << 11);
pub const RT5668_SAR_BYPASS_MASK: u32 = (0x1 << 10);
pub const RT5668_SAR_BYPASS_EN: u32 = (0x1 << 10);
pub const RT5668_SAR_BYPASS_DIS: u32 = (0x0 << 10);
pub const RT5668_SAR_SEL_MB1_MASK: u32 = (0x1 << 9);
pub const RT5668_SAR_SEL_MB1_SEL: u32 = (0x1 << 9);
pub const RT5668_SAR_SEL_MB1_NOSEL: u32 = (0x0 << 9);
pub const RT5668_SAR_SEL_MB2_MASK: u32 = (0x1 << 8);
pub const RT5668_SAR_SEL_MB2_SEL: u32 = (0x1 << 8);
pub const RT5668_SAR_SEL_MB2_NOSEL: u32 = (0x0 << 8);
pub const RT5668_SAR_SEL_MODE_MASK: u32 = (0x1 << 7);
pub const RT5668_SAR_SEL_MODE_CMP: u32 = (0x1 << 7);
pub const RT5668_SAR_SEL_MODE_ADC: u32 = (0x0 << 7);
pub const RT5668_SAR_SEL_MB1_MB2_MASK: u32 = (0x1 << 5);
pub const RT5668_SAR_SEL_MB1_MB2_AUTO: u32 = (0x1 << 5);
pub const RT5668_SAR_SEL_MB1_MB2_MANU: u32 = (0x0 << 5);
pub const RT5668_SAR_SEL_SIGNAL_MASK: u32 = (0x1 << 4);
pub const RT5668_SAR_SEL_SIGNAL_AUTO: u32 = (0x1 << 4);
pub const RT5668_SAR_SEL_SIGNAL_MANU: u32 = (0x0 << 4);

/* SAR ADC Inline Command Control 13 (0x021c) */
pub const RT5668_SAR_SOUR_MASK: u32 = (0x3f);
pub const RT5668_SAR_SOUR_BTN: u32 = (0x3f);
pub const RT5668_SAR_SOUR_TYPE: u32 = (0x0);


/* System Clock Source */
pub const RT5668_SCLK_S_MCLK: u32 = 0;
pub const RT5668_SCLK_S_PLL1: u32 = 1;
pub const RT5668_SCLK_S_PLL2: u32 = 2;
pub const RT5668_SCLK_S_RCCLK: u32 = 3;


/* PLL Source */
pub const RT5668_PLL1_S_MCLK: u32 = 0;
pub const RT5668_PLL1_S_BCLK1: u32 = 1;
pub const RT5668_PLL1_S_RCCLK: u32 = 2;


pub const RT5668_AIF1: u32 = 0;
pub const RT5668_AIF2: u32 = 1;
pub const RT5668_AIFS: u32 = 2;


/* filter mask */
pub const RT5668_DA_STEREO1_FILTER: u32 = 0x1;
pub const RT5668_AD_STEREO1_FILTER: u32 = (0x1 << 1);


pub const RT5668_CLK_SEL_SYS: u32 = 0;
pub const RT5668_CLK_SEL_I2S1_ASRC: u32 = 1;
pub const RT5668_CLK_SEL_I2S2_ASRC: u32 = 2;



// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
