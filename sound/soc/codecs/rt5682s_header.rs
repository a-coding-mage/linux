// SPDX-License-Identifier: GPL-2.0-only
//
//// rt5682s.h  --  RT5682I-VS ALSA SoC audio driver
////
//// Copyright 2021 Realtek Microelectronics
//// Author: Derek Fang <derek.fang@realtek.com>
// 

// Depends on <sound/rt5682s.h>
// Depends on <linux/regulator/consumer.h>
// Depends on <linux/gpio/consumer.h>
// Depends on <linux/clk.h>
// Depends on <linux/clkdev.h>
// Depends on <linux/clk-provider.h>


// Info 
pub const RT5682S_RESET: u32 = 0x0000;
pub const RT5682S_VERSION_ID: u32 = 0x00fd;
pub const RT5682S_VENDOR_ID: u32 = 0x00fe;
pub const RT5682S_DEVICE_ID: u32 = 0x00ff;
//  I/O - Output 
pub const RT5682S_HP_CTRL_1: u32 = 0x0002;
pub const RT5682S_HP_CTRL_2: u32 = 0x0003;
pub const RT5682S_HPL_GAIN: u32 = 0x0005;
pub const RT5682S_HPR_GAIN: u32 = 0x0006;

pub const RT5682S_I2C_CTRL: u32 = 0x0008;

// I/O - Input 
pub const RT5682S_CBJ_BST_CTRL: u32 = 0x000b;
pub const RT5682S_CBJ_DET_CTRL: u32 = 0x000f;
pub const RT5682S_CBJ_CTRL_1: u32 = 0x0010;
pub const RT5682S_CBJ_CTRL_2: u32 = 0x0011;
pub const RT5682S_CBJ_CTRL_3: u32 = 0x0012;
pub const RT5682S_CBJ_CTRL_4: u32 = 0x0013;
pub const RT5682S_CBJ_CTRL_5: u32 = 0x0014;
pub const RT5682S_CBJ_CTRL_6: u32 = 0x0015;
pub const RT5682S_CBJ_CTRL_7: u32 = 0x0016;
pub const RT5682S_CBJ_CTRL_8: u32 = 0x0017;
// I/O - ADC/DAC/DMIC 
pub const RT5682S_DAC1_DIG_VOL: u32 = 0x0019;
pub const RT5682S_STO1_ADC_DIG_VOL: u32 = 0x001c;
pub const RT5682S_STO1_ADC_BOOST: u32 = 0x001f;
pub const RT5682S_HP_IMP_GAIN_1: u32 = 0x0022;
pub const RT5682S_HP_IMP_GAIN_2: u32 = 0x0023;
// Mixer - D-D 
pub const RT5682S_SIDETONE_CTRL: u32 = 0x0024;
pub const RT5682S_STO1_ADC_MIXER: u32 = 0x0026;
pub const RT5682S_AD_DA_MIXER: u32 = 0x0029;
pub const RT5682S_STO1_DAC_MIXER: u32 = 0x002a;
pub const RT5682S_A_DAC1_MUX: u32 = 0x002b;
pub const RT5682S_DIG_INF2_DATA: u32 = 0x0030;
// Mixer - ADC 
pub const RT5682S_REC_MIXER: u32 = 0x003c;
pub const RT5682S_CAL_REC: u32 = 0x0044;
// HP Analog Offset Control 
pub const RT5682S_HP_ANA_OST_CTRL_1: u32 = 0x004b;
pub const RT5682S_HP_ANA_OST_CTRL_2: u32 = 0x004c;
pub const RT5682S_HP_ANA_OST_CTRL_3: u32 = 0x004d;
// Power 
pub const RT5682S_PWR_DIG_1: u32 = 0x0061;
pub const RT5682S_PWR_DIG_2: u32 = 0x0062;
pub const RT5682S_PWR_ANLG_1: u32 = 0x0063;
pub const RT5682S_PWR_ANLG_2: u32 = 0x0064;
pub const RT5682S_PWR_ANLG_3: u32 = 0x0065;
pub const RT5682S_PWR_MIXER: u32 = 0x0066;

pub const RT5682S_MB_CTRL: u32 = 0x0067;
pub const RT5682S_CLK_GATE_TCON_1: u32 = 0x0068;
pub const RT5682S_CLK_GATE_TCON_2: u32 = 0x0069;
pub const RT5682S_CLK_GATE_TCON_3: u32 = 0x006a;
// Clock Detect 
pub const RT5682S_CLK_DET: u32 = 0x006b;
// Filter Auto Reset 
pub const RT5682S_RESET_LPF_CTRL: u32 = 0x006c;
pub const RT5682S_RESET_HPF_CTRL: u32 = 0x006d;
// DMIC 
pub const RT5682S_DMIC_CTRL_1: u32 = 0x006e;
pub const RT5682S_LPF_AD_DMIC: u32 = 0x006f;
// Format - ADC/DAC 
pub const RT5682S_I2S1_SDP: u32 = 0x0070;
pub const RT5682S_I2S2_SDP: u32 = 0x0071;
pub const RT5682S_ADDA_CLK_1: u32 = 0x0073;
pub const RT5682S_ADDA_CLK_2: u32 = 0x0074;
pub const RT5682S_I2S1_F_DIV_CTRL_1: u32 = 0x0075;
pub const RT5682S_I2S1_F_DIV_CTRL_2: u32 = 0x0076;
// Format - TDM Control 
pub const RT5682S_TDM_CTRL: u32 = 0x0079;
pub const RT5682S_TDM_ADDA_CTRL_1: u32 = 0x007a;
pub const RT5682S_TDM_ADDA_CTRL_2: u32 = 0x007b;
pub const RT5682S_DATA_SEL_CTRL_1: u32 = 0x007c;
pub const RT5682S_TDM_TCON_CTRL_1: u32 = 0x007e;
pub const RT5682S_TDM_TCON_CTRL_2: u32 = 0x007f;
// Function - Analog 
pub const RT5682S_GLB_CLK: u32 = 0x0080;
pub const RT5682S_PLL_TRACK_1: u32 = 0x0083;
pub const RT5682S_PLL_TRACK_2: u32 = 0x0084;
pub const RT5682S_PLL_TRACK_3: u32 = 0x0085;
pub const RT5682S_PLL_TRACK_4: u32 = 0x0086;
pub const RT5682S_PLL_TRACK_5: u32 = 0x0087;
pub const RT5682S_PLL_TRACK_6: u32 = 0x0088;
pub const RT5682S_PLL_TRACK_11: u32 = 0x008c;
pub const RT5682S_DEPOP_1: u32 = 0x008e;
pub const RT5682S_HP_CHARGE_PUMP_1: u32 = 0x008f;
pub const RT5682S_HP_CHARGE_PUMP_2: u32 = 0x0091;
pub const RT5682S_HP_CHARGE_PUMP_3: u32 = 0x0092;
pub const RT5682S_MICBIAS_1: u32 = 0x0093;
pub const RT5682S_MICBIAS_2: u32 = 0x0094;
pub const RT5682S_MICBIAS_3: u32 = 0x0095;

pub const RT5682S_PLL_TRACK_12: u32 = 0x0096;
pub const RT5682S_PLL_TRACK_14: u32 = 0x0097;
pub const RT5682S_PLL_CTRL_1: u32 = 0x0098;
pub const RT5682S_PLL_CTRL_2: u32 = 0x0099;
pub const RT5682S_PLL_CTRL_3: u32 = 0x009a;
pub const RT5682S_PLL_CTRL_4: u32 = 0x009b;
pub const RT5682S_PLL_CTRL_5: u32 = 0x009c;
pub const RT5682S_PLL_CTRL_6: u32 = 0x009d;
pub const RT5682S_PLL_CTRL_7: u32 = 0x009e;

pub const RT5682S_RC_CLK_CTRL: u32 = 0x009f;
pub const RT5682S_I2S2_M_CLK_CTRL_1: u32 = 0x00a0;
pub const RT5682S_I2S2_F_DIV_CTRL_1: u32 = 0x00a3;
pub const RT5682S_I2S2_F_DIV_CTRL_2: u32 = 0x00a4;

pub const RT5682S_IRQ_CTRL_1: u32 = 0x00b6;
pub const RT5682S_IRQ_CTRL_2: u32 = 0x00b7;
pub const RT5682S_IRQ_CTRL_3: u32 = 0x00b8;
pub const RT5682S_IRQ_CTRL_4: u32 = 0x00b9;
pub const RT5682S_INT_ST_1: u32 = 0x00be;
pub const RT5682S_GPIO_CTRL_1: u32 = 0x00c0;
pub const RT5682S_GPIO_CTRL_2: u32 = 0x00c1;
pub const RT5682S_GPIO_ST: u32 = 0x00c2;
pub const RT5682S_HP_AMP_DET_CTRL_1: u32 = 0x00d0;
pub const RT5682S_MID_HP_AMP_DET: u32 = 0x00d2;
pub const RT5682S_LOW_HP_AMP_DET: u32 = 0x00d3;
pub const RT5682S_DELAY_BUF_CTRL: u32 = 0x00d4;
pub const RT5682S_SV_ZCD_1: u32 = 0x00d9;
pub const RT5682S_SV_ZCD_2: u32 = 0x00da;
pub const RT5682S_IL_CMD_1: u32 = 0x00db;
pub const RT5682S_IL_CMD_2: u32 = 0x00dc;
pub const RT5682S_IL_CMD_3: u32 = 0x00dd;
pub const RT5682S_IL_CMD_4: u32 = 0x00de;
pub const RT5682S_IL_CMD_5: u32 = 0x00df;
pub const RT5682S_IL_CMD_6: u32 = 0x00e0;
pub const RT5682S_4BTN_IL_CMD_1: u32 = 0x00e2;
pub const RT5682S_4BTN_IL_CMD_2: u32 = 0x00e3;
pub const RT5682S_4BTN_IL_CMD_3: u32 = 0x00e4;
pub const RT5682S_4BTN_IL_CMD_4: u32 = 0x00e5;
pub const RT5682S_4BTN_IL_CMD_5: u32 = 0x00e6;
pub const RT5682S_4BTN_IL_CMD_6: u32 = 0x00e7;
pub const RT5682S_4BTN_IL_CMD_7: u32 = 0x00e8;

pub const RT5682S_ADC_STO1_HP_CTRL_1: u32 = 0x00ea;
pub const RT5682S_ADC_STO1_HP_CTRL_2: u32 = 0x00eb;
pub const RT5682S_AJD1_CTRL: u32 = 0x00f0;
pub const RT5682S_JD_CTRL_1: u32 = 0x00f6;
// General Control 
pub const RT5682S_DUMMY_1: u32 = 0x00fa;
pub const RT5682S_DUMMY_2: u32 = 0x00fb;
pub const RT5682S_DUMMY_3: u32 = 0x00fc;

pub const RT5682S_DAC_ADC_DIG_VOL1: u32 = 0x0100;
pub const RT5682S_BIAS_CUR_CTRL_2: u32 = 0x010b;
pub const RT5682S_BIAS_CUR_CTRL_3: u32 = 0x010c;
pub const RT5682S_BIAS_CUR_CTRL_4: u32 = 0x010d;
pub const RT5682S_BIAS_CUR_CTRL_5: u32 = 0x010e;
pub const RT5682S_BIAS_CUR_CTRL_6: u32 = 0x010f;
pub const RT5682S_BIAS_CUR_CTRL_7: u32 = 0x0110;
pub const RT5682S_BIAS_CUR_CTRL_8: u32 = 0x0111;
pub const RT5682S_BIAS_CUR_CTRL_9: u32 = 0x0112;
pub const RT5682S_BIAS_CUR_CTRL_10: u32 = 0x0113;
pub const RT5682S_VREF_REC_OP_FB_CAP_CTRL_1: u32 = 0x0117;
pub const RT5682S_VREF_REC_OP_FB_CAP_CTRL_2: u32 = 0x0118;
pub const RT5682S_CHARGE_PUMP_1: u32 = 0x0125;
pub const RT5682S_DIG_IN_CTRL_1: u32 = 0x0132;
pub const RT5682S_PAD_DRIVING_CTRL: u32 = 0x0136;
pub const RT5682S_CHOP_DAC_1: u32 = 0x0139;
pub const RT5682S_CHOP_DAC_2: u32 = 0x013a;
pub const RT5682S_CHOP_ADC: u32 = 0x013b;
pub const RT5682S_CALIB_ADC_CTRL: u32 = 0x013c;
pub const RT5682S_VOL_TEST: u32 = 0x013f;
pub const RT5682S_SPKVDD_DET_ST: u32 = 0x0142;
pub const RT5682S_TEST_MODE_CTRL_1: u32 = 0x0145;
pub const RT5682S_TEST_MODE_CTRL_2: u32 = 0x0146;
pub const RT5682S_TEST_MODE_CTRL_3: u32 = 0x0147;
pub const RT5682S_TEST_MODE_CTRL_4: u32 = 0x0148;
pub const RT5682S_PLL_INTERNAL_1: u32 = 0x0156;
pub const RT5682S_PLL_INTERNAL_2: u32 = 0x0157;
pub const RT5682S_PLL_INTERNAL_3: u32 = 0x0158;
pub const RT5682S_PLL_INTERNAL_4: u32 = 0x0159;
pub const RT5682S_STO_NG2_CTRL_1: u32 = 0x0160;
pub const RT5682S_STO_NG2_CTRL_2: u32 = 0x0161;
pub const RT5682S_STO_NG2_CTRL_3: u32 = 0x0162;
pub const RT5682S_STO_NG2_CTRL_4: u32 = 0x0163;
pub const RT5682S_STO_NG2_CTRL_5: u32 = 0x0164;
pub const RT5682S_STO_NG2_CTRL_6: u32 = 0x0165;
pub const RT5682S_STO_NG2_CTRL_7: u32 = 0x0166;
pub const RT5682S_STO_NG2_CTRL_8: u32 = 0x0167;
pub const RT5682S_STO_NG2_CTRL_9: u32 = 0x0168;
pub const RT5682S_STO_NG2_CTRL_10: u32 = 0x0169;
pub const RT5682S_STO1_DAC_SIL_DET: u32 = 0x0190;
pub const RT5682S_SIL_PSV_CTRL1: u32 = 0x0194;
pub const RT5682S_SIL_PSV_CTRL2: u32 = 0x0195;
pub const RT5682S_SIL_PSV_CTRL3: u32 = 0x0197;
pub const RT5682S_SIL_PSV_CTRL4: u32 = 0x0198;
pub const RT5682S_SIL_PSV_CTRL5: u32 = 0x0199;
pub const RT5682S_HP_IMP_SENS_CTRL_1: u32 = 0x01ac;
pub const RT5682S_HP_IMP_SENS_CTRL_2: u32 = 0x01ad;
pub const RT5682S_HP_IMP_SENS_CTRL_3: u32 = 0x01ae;
pub const RT5682S_HP_IMP_SENS_CTRL_4: u32 = 0x01af;
pub const RT5682S_HP_IMP_SENS_CTRL_5: u32 = 0x01b0;
pub const RT5682S_HP_IMP_SENS_CTRL_6: u32 = 0x01b1;
pub const RT5682S_HP_IMP_SENS_CTRL_7: u32 = 0x01b2;
pub const RT5682S_HP_IMP_SENS_CTRL_8: u32 = 0x01b3;
pub const RT5682S_HP_IMP_SENS_CTRL_9: u32 = 0x01b4;
pub const RT5682S_HP_IMP_SENS_CTRL_10: u32 = 0x01b5;
pub const RT5682S_HP_IMP_SENS_CTRL_11: u32 = 0x01b6;
pub const RT5682S_HP_IMP_SENS_CTRL_12: u32 = 0x01b7;
pub const RT5682S_HP_IMP_SENS_CTRL_13: u32 = 0x01b8;
pub const RT5682S_HP_IMP_SENS_CTRL_14: u32 = 0x01b9;
pub const RT5682S_HP_IMP_SENS_CTRL_15: u32 = 0x01ba;
pub const RT5682S_HP_IMP_SENS_CTRL_16: u32 = 0x01bb;
pub const RT5682S_HP_IMP_SENS_CTRL_17: u32 = 0x01bc;
pub const RT5682S_HP_IMP_SENS_CTRL_18: u32 = 0x01bd;
pub const RT5682S_HP_IMP_SENS_CTRL_19: u32 = 0x01be;
pub const RT5682S_HP_IMP_SENS_CTRL_20: u32 = 0x01bf;
pub const RT5682S_HP_IMP_SENS_CTRL_21: u32 = 0x01c0;
pub const RT5682S_HP_IMP_SENS_CTRL_22: u32 = 0x01c1;
pub const RT5682S_HP_IMP_SENS_CTRL_23: u32 = 0x01c2;
pub const RT5682S_HP_IMP_SENS_CTRL_24: u32 = 0x01c3;
pub const RT5682S_HP_IMP_SENS_CTRL_25: u32 = 0x01c4;
pub const RT5682S_HP_IMP_SENS_CTRL_26: u32 = 0x01c5;
pub const RT5682S_HP_IMP_SENS_CTRL_27: u32 = 0x01c6;
pub const RT5682S_HP_IMP_SENS_CTRL_28: u32 = 0x01c7;
pub const RT5682S_HP_IMP_SENS_CTRL_29: u32 = 0x01c8;
pub const RT5682S_HP_IMP_SENS_CTRL_30: u32 = 0x01c9;
pub const RT5682S_HP_IMP_SENS_CTRL_31: u32 = 0x01ca;
pub const RT5682S_HP_IMP_SENS_CTRL_32: u32 = 0x01cb;
pub const RT5682S_HP_IMP_SENS_CTRL_33: u32 = 0x01cc;
pub const RT5682S_HP_IMP_SENS_CTRL_34: u32 = 0x01cd;
pub const RT5682S_HP_IMP_SENS_CTRL_35: u32 = 0x01ce;
pub const RT5682S_HP_IMP_SENS_CTRL_36: u32 = 0x01cf;
pub const RT5682S_HP_IMP_SENS_CTRL_37: u32 = 0x01d0;
pub const RT5682S_HP_IMP_SENS_CTRL_38: u32 = 0x01d1;
pub const RT5682S_HP_IMP_SENS_CTRL_39: u32 = 0x01d2;
pub const RT5682S_HP_IMP_SENS_CTRL_40: u32 = 0x01d3;
pub const RT5682S_HP_IMP_SENS_CTRL_41: u32 = 0x01d4;
pub const RT5682S_HP_IMP_SENS_CTRL_42: u32 = 0x01d5;
pub const RT5682S_HP_IMP_SENS_CTRL_43: u32 = 0x01d6;
pub const RT5682S_HP_IMP_SENS_CTRL_44: u32 = 0x01d7;
pub const RT5682S_HP_IMP_SENS_CTRL_45: u32 = 0x01d8;
pub const RT5682S_HP_IMP_SENS_CTRL_46: u32 = 0x01d9;
pub const RT5682S_HP_LOGIC_CTRL_1: u32 = 0x01da;
pub const RT5682S_HP_LOGIC_CTRL_2: u32 = 0x01db;
pub const RT5682S_HP_LOGIC_CTRL_3: u32 = 0x01dc;
pub const RT5682S_HP_CALIB_CTRL_1: u32 = 0x01de;
pub const RT5682S_HP_CALIB_CTRL_2: u32 = 0x01df;
pub const RT5682S_HP_CALIB_CTRL_3: u32 = 0x01e0;
pub const RT5682S_HP_CALIB_CTRL_4: u32 = 0x01e1;
pub const RT5682S_HP_CALIB_CTRL_5: u32 = 0x01e2;
pub const RT5682S_HP_CALIB_CTRL_6: u32 = 0x01e3;
pub const RT5682S_HP_CALIB_CTRL_7: u32 = 0x01e4;
pub const RT5682S_HP_CALIB_CTRL_8: u32 = 0x01e5;
pub const RT5682S_HP_CALIB_CTRL_9: u32 = 0x01e6;
pub const RT5682S_HP_CALIB_CTRL_10: u32 = 0x01e7;
pub const RT5682S_HP_CALIB_CTRL_11: u32 = 0x01e8;
pub const RT5682S_HP_CALIB_ST_1: u32 = 0x01ea;
pub const RT5682S_HP_CALIB_ST_2: u32 = 0x01eb;
pub const RT5682S_HP_CALIB_ST_3: u32 = 0x01ec;
pub const RT5682S_HP_CALIB_ST_4: u32 = 0x01ed;
pub const RT5682S_HP_CALIB_ST_5: u32 = 0x01ee;
pub const RT5682S_HP_CALIB_ST_6: u32 = 0x01ef;
pub const RT5682S_HP_CALIB_ST_7: u32 = 0x01f0;
pub const RT5682S_HP_CALIB_ST_8: u32 = 0x01f1;
pub const RT5682S_HP_CALIB_ST_9: u32 = 0x01f2;
pub const RT5682S_HP_CALIB_ST_10: u32 = 0x01f3;
pub const RT5682S_HP_CALIB_ST_11: u32 = 0x01f4;
pub const RT5682S_SAR_IL_CMD_1: u32 = 0x0210;
pub const RT5682S_SAR_IL_CMD_2: u32 = 0x0211;
pub const RT5682S_SAR_IL_CMD_3: u32 = 0x0212;
pub const RT5682S_SAR_IL_CMD_4: u32 = 0x0213;
pub const RT5682S_SAR_IL_CMD_5: u32 = 0x0214;
pub const RT5682S_SAR_IL_CMD_6: u32 = 0x0215;
pub const RT5682S_SAR_IL_CMD_7: u32 = 0x0216;
pub const RT5682S_SAR_IL_CMD_8: u32 = 0x0217;
pub const RT5682S_SAR_IL_CMD_9: u32 = 0x0218;
pub const RT5682S_SAR_IL_CMD_10: u32 = 0x0219;
pub const RT5682S_SAR_IL_CMD_11: u32 = 0x021a;
pub const RT5682S_SAR_IL_CMD_12: u32 = 0x021b;
pub const RT5682S_SAR_IL_CMD_13: u32 = 0x021c;
pub const RT5682S_SAR_IL_CMD_14: u32 = 0x021d;
pub const RT5682S_DUMMY_4: u32 = 0x02fa;
pub const RT5682S_DUMMY_5: u32 = 0x02fb;
pub const RT5682S_DUMMY_6: u32 = 0x02fc;
pub const RT5682S_VERSION_ID_HIDE: u32 = 0x03fe;
pub const RT5682S_VERSION_ID_CUS: u32 = 0x03ff;
pub const RT5682S_SCAN_CTL: u32 = 0x0500;
pub const RT5682S_HP_AMP_DET: u32 = 0x0600;
pub const RT5682S_BIAS_CUR_CTRL_11: u32 = 0x0610;
pub const RT5682S_BIAS_CUR_CTRL_12: u32 = 0x0611;
pub const RT5682S_BIAS_CUR_CTRL_13: u32 = 0x0620;
pub const RT5682S_BIAS_CUR_CTRL_14: u32 = 0x0621;
pub const RT5682S_BIAS_CUR_CTRL_15: u32 = 0x0630;
pub const RT5682S_BIAS_CUR_CTRL_16: u32 = 0x0631;
pub const RT5682S_BIAS_CUR_CTRL_17: u32 = 0x0640;
pub const RT5682S_BIAS_CUR_CTRL_18: u32 = 0x0641;
pub const RT5682S_I2C_TRANS_CTRL: u32 = 0x07fa;
pub const RT5682S_DUMMY_7: u32 = 0x08fa;
pub const RT5682S_DUMMY_8: u32 = 0x08fb;
pub const RT5682S_DMIC_FLOAT_DET: u32 = 0x0d00;
pub const RT5682S_HA_CMP_OP_1: u32 = 0x1100;
pub const RT5682S_HA_CMP_OP_2: u32 = 0x1101;
pub const RT5682S_HA_CMP_OP_3: u32 = 0x1102;
pub const RT5682S_HA_CMP_OP_4: u32 = 0x1103;
pub const RT5682S_HA_CMP_OP_5: u32 = 0x1104;
pub const RT5682S_HA_CMP_OP_6: u32 = 0x1105;
pub const RT5682S_HA_CMP_OP_7: u32 = 0x1106;
pub const RT5682S_HA_CMP_OP_8: u32 = 0x1107;
pub const RT5682S_HA_CMP_OP_9: u32 = 0x1108;
pub const RT5682S_HA_CMP_OP_10: u32 = 0x1109;
pub const RT5682S_HA_CMP_OP_11: u32 = 0x110a;
pub const RT5682S_HA_CMP_OP_12: u32 = 0x110b;
pub const RT5682S_HA_CMP_OP_13: u32 = 0x110c;
pub const RT5682S_HA_CMP_OP_14: u32 = 0x1111;
pub const RT5682S_HA_CMP_OP_15: u32 = 0x1112;
pub const RT5682S_HA_CMP_OP_16: u32 = 0x1113;
pub const RT5682S_HA_CMP_OP_17: u32 = 0x1114;
pub const RT5682S_HA_CMP_OP_18: u32 = 0x1115;
pub const RT5682S_HA_CMP_OP_19: u32 = 0x1116;
pub const RT5682S_HA_CMP_OP_20: u32 = 0x1117;
pub const RT5682S_HA_CMP_OP_21: u32 = 0x1118;
pub const RT5682S_HA_CMP_OP_22: u32 = 0x1119;
pub const RT5682S_HA_CMP_OP_23: u32 = 0x111a;
pub const RT5682S_HA_CMP_OP_24: u32 = 0x111b;
pub const RT5682S_HA_CMP_OP_25: u32 = 0x111c;
pub const RT5682S_NEW_CBJ_DET_CTL_1: u32 = 0x1401;
pub const RT5682S_NEW_CBJ_DET_CTL_2: u32 = 0x1402;
pub const RT5682S_NEW_CBJ_DET_CTL_3: u32 = 0x1403;
pub const RT5682S_NEW_CBJ_DET_CTL_4: u32 = 0x1404;
pub const RT5682S_NEW_CBJ_DET_CTL_5: u32 = 0x1406;
pub const RT5682S_NEW_CBJ_DET_CTL_6: u32 = 0x1407;
pub const RT5682S_NEW_CBJ_DET_CTL_7: u32 = 0x1408;
pub const RT5682S_NEW_CBJ_DET_CTL_8: u32 = 0x1409;
pub const RT5682S_NEW_CBJ_DET_CTL_9: u32 = 0x140a;
pub const RT5682S_NEW_CBJ_DET_CTL_10: u32 = 0x140b;
pub const RT5682S_NEW_CBJ_DET_CTL_11: u32 = 0x140c;
pub const RT5682S_NEW_CBJ_DET_CTL_12: u32 = 0x140d;
pub const RT5682S_NEW_CBJ_DET_CTL_13: u32 = 0x140e;
pub const RT5682S_NEW_CBJ_DET_CTL_14: u32 = 0x140f;
pub const RT5682S_NEW_CBJ_DET_CTL_15: u32 = 0x1410;
pub const RT5682S_NEW_CBJ_DET_CTL_16: u32 = 0x1411;
pub const RT5682S_DA_FILTER_1: u32 = 0x1801;
pub const RT5682S_DA_FILTER_2: u32 = 0x1802;
pub const RT5682S_DA_FILTER_3: u32 = 0x1803;
pub const RT5682S_DA_FILTER_4: u32 = 0x1804;
pub const RT5682S_DA_FILTER_5: u32 = 0x1805;
pub const RT5682S_CLK_SW_TEST_1: u32 = 0x2c00;
pub const RT5682S_CLK_SW_TEST_2: u32 = 0x3400;
pub const RT5682S_CLK_SW_TEST_3: u32 = 0x3404;
pub const RT5682S_CLK_SW_TEST_4: u32 = 0x3405;
pub const RT5682S_CLK_SW_TEST_5: u32 = 0x3406;
pub const RT5682S_CLK_SW_TEST_6: u32 = 0x3407;
pub const RT5682S_CLK_SW_TEST_7: u32 = 0x3408;
pub const RT5682S_CLK_SW_TEST_8: u32 = 0x3409;
pub const RT5682S_CLK_SW_TEST_9: u32 = 0x340a;
pub const RT5682S_CLK_SW_TEST_10: u32 = 0x340b;
pub const RT5682S_CLK_SW_TEST_11: u32 = 0x340c;
pub const RT5682S_CLK_SW_TEST_12: u32 = 0x340d;
pub const RT5682S_CLK_SW_TEST_13: u32 = 0x340e;
pub const RT5682S_CLK_SW_TEST_14: u32 = 0x340f;
pub const RT5682S_EFUSE_MANU_WRITE_1: u32 = 0x3410;
pub const RT5682S_EFUSE_MANU_WRITE_2: u32 = 0x3411;
pub const RT5682S_EFUSE_MANU_WRITE_3: u32 = 0x3412;
pub const RT5682S_EFUSE_MANU_WRITE_4: u32 = 0x3413;
pub const RT5682S_EFUSE_MANU_WRITE_5: u32 = 0x3414;
pub const RT5682S_EFUSE_MANU_WRITE_6: u32 = 0x3415;
pub const RT5682S_EFUSE_READ_1: u32 = 0x3424;
pub const RT5682S_EFUSE_READ_2: u32 = 0x3425;
pub const RT5682S_EFUSE_READ_3: u32 = 0x3426;
pub const RT5682S_EFUSE_READ_4: u32 = 0x3427;
pub const RT5682S_EFUSE_READ_5: u32 = 0x3428;
pub const RT5682S_EFUSE_READ_6: u32 = 0x3429;
pub const RT5682S_EFUSE_READ_7: u32 = 0x342a;
pub const RT5682S_EFUSE_READ_8: u32 = 0x342b;
pub const RT5682S_EFUSE_READ_9: u32 = 0x342c;
pub const RT5682S_EFUSE_READ_10: u32 = 0x342d;
pub const RT5682S_EFUSE_READ_11: u32 = 0x342e;
pub const RT5682S_EFUSE_READ_12: u32 = 0x342f;
pub const RT5682S_EFUSE_READ_13: u32 = 0x3430;
pub const RT5682S_EFUSE_READ_14: u32 = 0x3431;
pub const RT5682S_EFUSE_READ_15: u32 = 0x3432;
pub const RT5682S_EFUSE_READ_16: u32 = 0x3433;
pub const RT5682S_EFUSE_READ_17: u32 = 0x3434;
pub const RT5682S_EFUSE_READ_18: u32 = 0x3435;
pub const RT5682S_EFUSE_TIMING_CTL_1: u32 = 0x3440;
pub const RT5682S_EFUSE_TIMING_CTL_2: u32 = 0x3441;
pub const RT5682S_PILOT_DIG_CTL_1: u32 = 0x3500;
pub const RT5682S_PILOT_DIG_CTL_2: u32 = 0x3501;
pub const RT5682S_HP_AMP_DET_CTL_1: u32 = 0x3b00;
pub const RT5682S_HP_AMP_DET_CTL_2: u32 = 0x3b01;
pub const RT5682S_HP_AMP_DET_CTL_3: u32 = 0x3b02;
pub const RT5682S_HP_AMP_DET_CTL_4: u32 = 0x3b03;

pub const RT5682S_MAX_REG: u32 = (RT5682S_HP_AMP_DET_CTL_4);

// global definition 
pub const RT5682S_L_MUTE: u32 = (0x1 << 15);
pub const RT5682S_L_MUTE_SFT: u32 = 15;
pub const RT5682S_R_MUTE: u32 = (0x1 << 7);
pub const RT5682S_R_MUTE_SFT: u32 = 7;
pub const RT5682S_L_VOL_SFT: u32 = 8;
pub const RT5682S_R_VOL_SFT: u32 = 0;
pub const RT5682S_CLK_SRC_MCLK: u32 = (0x0);
pub const RT5682S_CLK_SRC_PLL1: u32 = (0x1);
pub const RT5682S_CLK_SRC_PLL2: u32 = (0x2);
pub const RT5682S_CLK_SRC_RCCLK: u32 = (0x4); // 25M


// Headphone Amp Control 2 (0x0003) 
pub const RT5682S_HPO_L_PATH_MASK: u32 = (0x1 << 14);
pub const RT5682S_HPO_L_PATH_EN: u32 = (0x1 << 14);
pub const RT5682S_HPO_L_PATH_DIS: u32 = (0x0 << 14);
pub const RT5682S_HPO_R_PATH_MASK: u32 = (0x1 << 13);
pub const RT5682S_HPO_R_PATH_EN: u32 = (0x1 << 13);
pub const RT5682S_HPO_R_PATH_DIS: u32 = (0x0 << 13);
pub const RT5682S_HPO_SEL_IP_EN_SW: u32 = (0x1);
pub const RT5682S_HPO_IP_EN_GATING: u32 = (0x1);
pub const RT5682S_HPO_IP_NO_GATING: u32 = (0x0);

//Headphone Amp L/R Analog Gain and Digital NG2 Gain Control (0x0005 0x0006)
pub const RT5682S_G_HP: u32 = (0xf << 8);
pub const RT5682S_G_HP_SFT: u32 = 8;
pub const RT5682S_G_STO_DA_DMIX: u32 = (0xf);
pub const RT5682S_G_STO_DA_SFT: u32 = 0;

// Embeeded Jack and Type Detection Control 2 (0x0010) 
pub const RT5682S_EMB_JD_MASK: u32 = (0x1 << 15);
pub const RT5682S_EMB_JD_EN: u32 = (0x1 << 15);
pub const RT5682S_EMB_JD_EN_SFT: u32 = 15;
pub const RT5682S_EMB_JD_RST: u32 = (0x1 << 14);
pub const RT5682S_JD_MODE: u32 = (0x1 << 13);
pub const RT5682S_JD_MODE_SFT: u32 = 13;
pub const RT5682S_DET_TYPE: u32 = (0x1 << 12);
pub const RT5682S_DET_TYPE_SFT: u32 = 12;
pub const RT5682S_POLA_EXT_JD_MASK: u32 = (0x1 << 11);
pub const RT5682S_POLA_EXT_JD_LOW: u32 = (0x1 << 11);
pub const RT5682S_POLA_EXT_JD_HIGH: u32 = (0x0 << 11);
pub const RT5682S_SEL_FAST_OFF_MASK: u32 = (0x3 << 9);
pub const RT5682S_SEL_FAST_OFF_SFT: u32 = 9;
pub const RT5682S_POL_FAST_OFF_MASK: u32 = (0x1 << 8);
pub const RT5682S_POL_FAST_OFF_HIGH: u32 = (0x1 << 8);
pub const RT5682S_POL_FAST_OFF_LOW: u32 = (0x0 << 8);
pub const RT5682S_FAST_OFF_MASK: u32 = (0x1 << 7);
pub const RT5682S_FAST_OFF_EN: u32 = (0x1 << 7);
pub const RT5682S_FAST_OFF_DIS: u32 = (0x0 << 7);
pub const RT5682S_VREF_POW_MASK: u32 = (0x1 << 6);
pub const RT5682S_VREF_POW_FSM: u32 = (0x0 << 6);
pub const RT5682S_VREF_POW_REG: u32 = (0x1 << 6);
pub const RT5682S_MB1_PATH_BIT: u32 = 5;
pub const RT5682S_MB1_PATH_MASK: u32 = (0x1 << 5);
pub const RT5682S_CTRL_MB1_REG: u32 = (0x1 << 5);
pub const RT5682S_CTRL_MB1_FSM: u32 = (0x0 << 5);
pub const RT5682S_MB2_PATH_BIT: u32 = 4;
pub const RT5682S_MB2_PATH_MASK: u32 = (0x1 << 4);
pub const RT5682S_CTRL_MB2_REG: u32 = (0x1 << 4);
pub const RT5682S_CTRL_MB2_FSM: u32 = (0x0 << 4);
pub const RT5682S_TRIG_JD_MASK: u32 = (0x1 << 3);
pub const RT5682S_TRIG_JD_HIGH: u32 = (0x1 << 3);
pub const RT5682S_TRIG_JD_LOW: u32 = (0x0 << 3);
pub const RT5682S_MIC_CAP_MASK: u32 = (0x1 << 1);
pub const RT5682S_MIC_CAP_HS: u32 = (0x1 << 1);
pub const RT5682S_MIC_CAP_HP: u32 = (0x0 << 1);
pub const RT5682S_MIC_CAP_SRC_MASK: u32 = (0x1);
pub const RT5682S_MIC_CAP_SRC_REG: u32 = (0x1);
pub const RT5682S_MIC_CAP_SRC_ANA: u32 = (0x0);

// Embeeded Jack and Type Detection Control 3 (0x0011) 
pub const RT5682S_SEL_CBJ_TYPE_SLOW: u32 = (0x1 << 15);
pub const RT5682S_SEL_CBJ_TYPE_NORM: u32 = (0x0 << 15);
pub const RT5682S_SEL_CBJ_TYPE_MASK: u32 = (0x1 << 15);
pub const RT5682S_POW_BG_MB1_MASK: u32 = (0x1 << 13);
pub const RT5682S_POW_BG_MB1_REG: u32 = (0x1 << 13);
pub const RT5682S_POW_BG_MB1_FSM: u32 = (0x0 << 13);
pub const RT5682S_POW_BG_MB2_MASK: u32 = (0x1 << 12);
pub const RT5682S_POW_BG_MB2_REG: u32 = (0x1 << 12);
pub const RT5682S_POW_BG_MB2_FSM: u32 = (0x0 << 12);
pub const RT5682S_EXT_JD_SRC: u32 = (0x7 << 4);
pub const RT5682S_EXT_JD_SRC_SFT: u32 = 4;
pub const RT5682S_EXT_JD_SRC_GPIO_JD1: u32 = (0x0 << 4);
pub const RT5682S_EXT_JD_SRC_GPIO_JD2: u32 = (0x1 << 4);
pub const RT5682S_EXT_JD_SRC_JDH: u32 = (0x2 << 4);
pub const RT5682S_EXT_JD_SRC_JDL: u32 = (0x3 << 4);
pub const RT5682S_EXT_JD_SRC_MANUAL: u32 = (0x4 << 4);
pub const RT5682S_JACK_TYPE_MASK: u32 = (0x3);

// Combo Jack and Type Detection Control 4 (0x0012) 
pub const RT5682S_CBJ_IN_BUF_MASK: u32 = (0x1 << 7);
pub const RT5682S_CBJ_IN_BUF_EN: u32 = (0x1 << 7);
pub const RT5682S_CBJ_IN_BUF_DIS: u32 = (0x0 << 7);
pub const RT5682S_CBJ_IN_BUF_BIT: u32 = 7;

// Combo Jack and Type Detection Control 5 (0x0013) 
pub const RT5682S_SEL_SHT_MID_TON_MASK: u32 = (0x3 << 12);
pub const RT5682S_SEL_SHT_MID_TON_2: u32 = (0x0 << 12);
pub const RT5682S_SEL_SHT_MID_TON_3: u32 = (0x1 << 12);
pub const RT5682S_CBJ_JD_TEST_MASK: u32 = (0x1 << 6);
pub const RT5682S_CBJ_JD_TEST_NORM: u32 = (0x0 << 6);
pub const RT5682S_CBJ_JD_TEST_MODE: u32 = (0x1 << 6);

// Combo Jack and Type Detection Control 6 (0x0014) 
pub const RT5682S_JD_FAST_OFF_SRC_MASK: u32 = (0x7 << 8);
pub const RT5682S_JD_FAST_OFF_SRC_JDH: u32 = (0x6 << 8);
pub const RT5682S_JD_FAST_OFF_SRC_GPIO6: u32 = (0x5 << 8);
pub const RT5682S_JD_FAST_OFF_SRC_GPIO5: u32 = (0x4 << 8);
pub const RT5682S_JD_FAST_OFF_SRC_GPIO4: u32 = (0x3 << 8);
pub const RT5682S_JD_FAST_OFF_SRC_GPIO3: u32 = (0x2 << 8);
pub const RT5682S_JD_FAST_OFF_SRC_GPIO2: u32 = (0x1 << 8);
pub const RT5682S_JD_FAST_OFF_SRC_GPIO1: u32 = (0x0 << 8);

// DAC1 Digital Volume (0x0019) 
pub const RT5682S_DAC_L1_VOL_MASK: u32 = (0xff << 8);
pub const RT5682S_DAC_L1_VOL_SFT: u32 = 8;
pub const RT5682S_DAC_R1_VOL_MASK: u32 = (0xff);
pub const RT5682S_DAC_R1_VOL_SFT: u32 = 0;

// ADC Digital Volume Control (0x001c) 
pub const RT5682S_ADC_L_VOL_MASK: u32 = (0x7f << 8);
pub const RT5682S_ADC_L_VOL_SFT: u32 = 8;
pub const RT5682S_ADC_R_VOL_MASK: u32 = (0x7f);
pub const RT5682S_ADC_R_VOL_SFT: u32 = 0;

// Stereo1 ADC Boost Gain Control (0x001f) 
pub const RT5682S_STO1_ADC_L_BST_MASK: u32 = (0x3 << 14);
pub const RT5682S_STO1_ADC_L_BST_SFT: u32 = 14;
pub const RT5682S_STO1_ADC_R_BST_MASK: u32 = (0x3 << 12);
pub const RT5682S_STO1_ADC_R_BST_SFT: u32 = 12;

// Sidetone Control (0x0024) 
pub const RT5682S_ST_SRC_SEL: u32 = (0x1 << 8);
pub const RT5682S_ST_SRC_SFT: u32 = 8;
pub const RT5682S_ST_EN_MASK: u32 = (0x1 << 6);
pub const RT5682S_ST_DIS: u32 = (0x0 << 6);
pub const RT5682S_ST_EN: u32 = (0x1 << 6);
pub const RT5682S_ST_EN_SFT: u32 = 6;

// Stereo1 ADC Mixer Control (0x0026) 
pub const RT5682S_M_STO1_ADC_L1: u32 = (0x1 << 15);
pub const RT5682S_M_STO1_ADC_L1_SFT: u32 = 15;
pub const RT5682S_M_STO1_ADC_L2: u32 = (0x1 << 14);
pub const RT5682S_M_STO1_ADC_L2_SFT: u32 = 14;
pub const RT5682S_STO1_ADC1L_SRC_MASK: u32 = (0x1 << 13);
pub const RT5682S_STO1_ADC1L_SRC_SFT: u32 = 13;
pub const RT5682S_STO1_ADC1_SRC_ADC: u32 = (0x1 << 13);
pub const RT5682S_STO1_ADC1_SRC_DACMIX: u32 = (0x0 << 13);
pub const RT5682S_STO1_ADC2L_SRC_MASK: u32 = (0x1 << 12);
pub const RT5682S_STO1_ADC2L_SRC_SFT: u32 = 12;
pub const RT5682S_STO1_ADCL_SRC_MASK: u32 = (0x3 << 10);
pub const RT5682S_STO1_ADCL_SRC_SFT: u32 = 10;
pub const RT5682S_M_STO1_ADC_R1: u32 = (0x1 << 7);
pub const RT5682S_M_STO1_ADC_R1_SFT: u32 = 7;
pub const RT5682S_M_STO1_ADC_R2: u32 = (0x1 << 6);
pub const RT5682S_M_STO1_ADC_R2_SFT: u32 = 6;
pub const RT5682S_STO1_ADC1R_SRC_MASK: u32 = (0x1 << 5);
pub const RT5682S_STO1_ADC1R_SRC_SFT: u32 = 5;
pub const RT5682S_STO1_ADC2R_SRC_MASK: u32 = (0x1 << 4);
pub const RT5682S_STO1_ADC2R_SRC_SFT: u32 = 4;
pub const RT5682S_STO1_ADCR_SRC_MASK: u32 = (0x3 << 2);
pub const RT5682S_STO1_ADCR_SRC_SFT: u32 = 2;

// ADC Mixer to DAC Mixer Control (0x0029) 
pub const RT5682S_M_ADCMIX_L: u32 = (0x1 << 15);
pub const RT5682S_M_ADCMIX_L_SFT: u32 = 15;
pub const RT5682S_M_DAC1_L: u32 = (0x1 << 14);
pub const RT5682S_M_DAC1_L_SFT: u32 = 14;
pub const RT5682S_M_ADCMIX_R: u32 = (0x1 << 7);
pub const RT5682S_M_ADCMIX_R_SFT: u32 = 7;
pub const RT5682S_M_DAC1_R: u32 = (0x1 << 6);
pub const RT5682S_M_DAC1_R_SFT: u32 = 6;

// Stereo1 DAC Mixer Control (0x002a) 
pub const RT5682S_M_DAC_L1_STO_L: u32 = (0x1 << 15);
pub const RT5682S_M_DAC_L1_STO_L_SFT: u32 = 15;
pub const RT5682S_G_DAC_L1_STO_L_MASK: u32 = (0x1 << 14);
pub const RT5682S_G_DAC_L1_STO_L_SFT: u32 = 14;
pub const RT5682S_M_DAC_R1_STO_L: u32 = (0x1 << 13);
pub const RT5682S_M_DAC_R1_STO_L_SFT: u32 = 13;
pub const RT5682S_G_DAC_R1_STO_L_MASK: u32 = (0x1 << 12);
pub const RT5682S_G_DAC_R1_STO_L_SFT: u32 = 12;
pub const RT5682S_M_DAC_L1_STO_R: u32 = (0x1 << 7);
pub const RT5682S_M_DAC_L1_STO_R_SFT: u32 = 7;
pub const RT5682S_G_DAC_L1_STO_R_MASK: u32 = (0x1 << 6);
pub const RT5682S_G_DAC_L1_STO_R_SFT: u32 = 6;
pub const RT5682S_M_DAC_R1_STO_R: u32 = (0x1 << 5);
pub const RT5682S_M_DAC_R1_STO_R_SFT: u32 = 5;
pub const RT5682S_G_DAC_R1_STO_R_MASK: u32 = (0x1 << 4);
pub const RT5682S_G_DAC_R1_STO_R_SFT: u32 = 4;

// Analog DAC1 Input Source Control (0x002b) 
pub const RT5682S_M_ST_STO_L: u32 = (0x1 << 9);
pub const RT5682S_M_ST_STO_L_SFT: u32 = 9;
pub const RT5682S_M_ST_STO_R: u32 = (0x1 << 8);
pub const RT5682S_M_ST_STO_R_SFT: u32 = 8;
pub const RT5682S_DAC_L1_SRC_MASK: u32 = (0x1 << 4);
pub const RT5682S_A_DACL1_SFT: u32 = 4;
pub const RT5682S_DAC_R1_SRC_MASK: u32 = (0x1);
pub const RT5682S_A_DACR1_SFT: u32 = 0;

// Digital Interface Data Control (0x0030) 
pub const RT5682S_IF2_DAC_SEL_MASK: u32 = (0x3 << 2);
pub const RT5682S_IF2_DAC_SEL_SFT: u32 = 2;
pub const RT5682S_IF2_ADC_SEL_MASK: u32 = (0x3 << 0);
pub const RT5682S_IF2_ADC_SEL_SFT: u32 = 0;

// REC Left/Right Mixer Control 2 (0x003c) 
pub const RT5682S_BST_CBJ_MASK: u32 = (0x3f << 8);
pub const RT5682S_BST_CBJ_SFT: u32 = 8;
pub const RT5682S_M_CBJ_RM1_L: u32 = (0x1 << 7);
pub const RT5682S_M_CBJ_RM1_L_SFT: u32 = 7;
pub const RT5682S_M_CBJ_RM1_R: u32 = (0x1 << 6);
pub const RT5682S_M_CBJ_RM1_R_SFT: u32 = 6;

// REC Left/Right Mixer Calibration Control(0x0044) 
pub const RT5682S_PWR_RM1_R_BIT: u32 = 8;
pub const RT5682S_PWR_RM1_L_BIT: u32 = 0;

// Power Management for Digital 1 (0x0061) 
pub const RT5682S_PWR_I2S1: u32 = (0x1 << 15);
pub const RT5682S_PWR_I2S1_BIT: u32 = 15;
pub const RT5682S_PWR_I2S2: u32 = (0x1 << 14);
pub const RT5682S_PWR_I2S2_BIT: u32 = 14;
pub const RT5682S_PRE_CHR_DAC_L1: u32 = (0x1 << 13);
pub const RT5682S_PRE_CHR_DAC_L1_BIT: u32 = 13;
pub const RT5682S_PRE_CHR_DAC_R1: u32 = (0x1 << 12);
pub const RT5682S_PRE_CHR_DAC_R1_BIT: u32 = 12;
pub const RT5682S_PWR_DAC_L1: u32 = (0x1 << 11);
pub const RT5682S_PWR_DAC_L1_BIT: u32 = 11;
pub const RT5682S_PWR_DAC_R1: u32 = (0x1 << 10);
pub const RT5682S_PWR_DAC_R1_BIT: u32 = 10;
pub const RT5682S_PWR_LDO: u32 = (0x1 << 8);
pub const RT5682S_PWR_LDO_BIT: u32 = 8;
pub const RT5682S_PWR_D2S_L: u32 = (0x1 << 7);
pub const RT5682S_PWR_D2S_L_BIT: u32 = 7;
pub const RT5682S_PWR_D2S_R: u32 = (0x1 << 6);
pub const RT5682S_PWR_D2S_R_BIT: u32 = 6;
pub const RT5682S_PWR_ADC_L1: u32 = (0x1 << 4);
pub const RT5682S_PWR_ADC_L1_BIT: u32 = 4;
pub const RT5682S_PWR_ADC_R1: u32 = (0x1 << 3);
pub const RT5682S_PWR_ADC_R1_BIT: u32 = 3;
pub const RT5682S_EFUSE_SW_EN: u32 = (0x1 << 2);
pub const RT5682S_EFUSE_SW_DIS: u32 = (0x0 << 2);
pub const RT5682S_PWR_EFUSE: u32 = (0x1 << 1);
pub const RT5682S_PWR_EFUSE_BIT: u32 = 1;
pub const RT5682S_DIG_GATE_CTRL: u32 = (0x1 << 0);
pub const RT5682S_DIG_GATE_CTRL_SFT: u32 = 0;

// Power Management for Digital 2 (0x0062) 
pub const RT5682S_PWR_ADC_S1F: u32 = (0x1 << 15);
pub const RT5682S_PWR_ADC_S1F_BIT: u32 = 15;
pub const RT5682S_PWR_DAC_S1F: u32 = (0x1 << 10);
pub const RT5682S_PWR_DAC_S1F_BIT: u32 = 10;
pub const RT5682S_DLDO_I_LIMIT_MASK: u32 = (0x1 << 7);
pub const RT5682S_DLDO_I_LIMIT_EN: u32 = (0x1 << 7);
pub const RT5682S_DLDO_I_LIMIT_DIS: u32 = (0x0 << 7);
pub const RT5682S_DLDO_I_BIAS_SEL_4: u32 = (0x1 << 6);
pub const RT5682S_DLDO_I_BIAS_SEL_0: u32 = (0x0 << 6);
pub const RT5682S_DLDO_REG_TEST_1: u32 = (0x1 << 5);
pub const RT5682S_DLDO_REG_TEST_0: u32 = (0x0 << 5);
pub const RT5682S_DLDO_SRC_REG: u32 = (0x1 << 4);
pub const RT5682S_DLDO_SRC_EFUSE: u32 = (0x0 << 4);

// Power Management for Analog 1 (0x0063) 
pub const RT5682S_PWR_VREF1: u32 = (0x1 << 15);
pub const RT5682S_PWR_VREF1_BIT: u32 = 15;
pub const RT5682S_PWR_FV1: u32 = (0x1 << 14);
pub const RT5682S_PWR_FV1_BIT: u32 = 14;
pub const RT5682S_PWR_VREF2: u32 = (0x1 << 13);
pub const RT5682S_PWR_VREF2_BIT: u32 = 13;
pub const RT5682S_PWR_FV2: u32 = (0x1 << 12);
pub const RT5682S_PWR_FV2_BIT: u32 = 12;
pub const RT5682S_LDO1_DBG_MASK: u32 = (0x3 << 10);
pub const RT5682S_PWR_MB: u32 = (0x1 << 9);
pub const RT5682S_PWR_MB_BIT: u32 = 9;
pub const RT5682S_PWR_BG: u32 = (0x1 << 7);
pub const RT5682S_PWR_BG_BIT: u32 = 7;
pub const RT5682S_LDO1_BYPASS_MASK: u32 = (0x1 << 6);
pub const RT5682S_LDO1_BYPASS: u32 = (0x1 << 6);
pub const RT5682S_LDO1_NOT_BYPASS: u32 = (0x0 << 6);

// Power Management for Analog 2 (0x0064) 
pub const RT5682S_PWR_MCLK0_WD: u32 = (0x1 << 15);
pub const RT5682S_PWR_MCLK0_WD_BIT: u32 = 15;
pub const RT5682S_PWR_MCLK1_WD: u32 = (0x1 << 14);
pub const RT5682S_PWR_MCLK1_WD_BIT: u32 = 14;
pub const RT5682S_RST_MCLK0: u32 = (0x1 << 13);
pub const RT5682S_RST_MCLK0_BIT: u32 = 13;
pub const RT5682S_RST_MCLK1: u32 = (0x1 << 12);
pub const RT5682S_RST_MCLK1_BIT: u32 = 12;
pub const RT5682S_PWR_MB1: u32 = (0x1 << 11);
pub const RT5682S_PWR_MB1_PWR_DOWN: u32 = (0x0 << 11);
pub const RT5682S_PWR_MB1_BIT: u32 = 11;
pub const RT5682S_PWR_MB2: u32 = (0x1 << 10);
pub const RT5682S_PWR_MB2_PWR_DOWN: u32 = (0x0 << 10);
pub const RT5682S_PWR_MB2_BIT: u32 = 10;
pub const RT5682S_PWR_JD_MASK: u32 = (0x1 << 0);
pub const RT5682S_PWR_JD_ENABLE: u32 = (0x1 << 0);
pub const RT5682S_PWR_JD_DISABLE: u32 = (0x0 << 0);

// Power Management for Analog 3 (0x0065) 
pub const RT5682S_PWR_LDO_PLLA: u32 = (0x1 << 15);
pub const RT5682S_PWR_LDO_PLLA_BIT: u32 = 15;
pub const RT5682S_PWR_LDO_PLLB: u32 = (0x1 << 14);
pub const RT5682S_PWR_LDO_PLLB_BIT: u32 = 14;
pub const RT5682S_PWR_BIAS_PLLA: u32 = (0x1 << 13);
pub const RT5682S_PWR_BIAS_PLLA_BIT: u32 = 13;
pub const RT5682S_PWR_BIAS_PLLB: u32 = (0x1 << 12);
pub const RT5682S_PWR_BIAS_PLLB_BIT: u32 = 12;
pub const RT5682S_PWR_CBJ: u32 = (0x1 << 9);
pub const RT5682S_PWR_CBJ_BIT: u32 = 9;
pub const RT5682S_RSTB_PLLB: u32 = (0x1 << 7);
pub const RT5682S_RSTB_PLLB_BIT: u32 = 7;
pub const RT5682S_RSTB_PLLA: u32 = (0x1 << 6);
pub const RT5682S_RSTB_PLLA_BIT: u32 = 6;
pub const RT5682S_PWR_PLLB: u32 = (0x1 << 5);
pub const RT5682S_PWR_PLLB_BIT: u32 = 5;
pub const RT5682S_PWR_PLLA: u32 = (0x1 << 4);
pub const RT5682S_PWR_PLLA_BIT: u32 = 4;
pub const RT5682S_PWR_LDO_MB2: u32 = (0x1 << 2);
pub const RT5682S_PWR_LDO_MB2_BIT: u32 = 2;
pub const RT5682S_PWR_LDO_MB1: u32 = (0x1 << 1);
pub const RT5682S_PWR_LDO_MB1_BIT: u32 = 1;
pub const RT5682S_PWR_BGLDO: u32 = (0x1 << 0);
pub const RT5682S_PWR_BGLDO_BIT: u32 = 0;

// Power Management for Mixer (0x0066) 
pub const RT5682S_PWR_CLK_COMP_8FS: u32 = (0x1 << 15);
pub const RT5682S_PWR_CLK_COMP_8FS_BIT: u32 = 15;
pub const RT5682S_DBG_BGLDO_MASK: u32 = (0x3 << 12);
pub const RT5682S_DBG_BGLDO_SFT: u32 = 12;
pub const RT5682S_DBG_BGLDO_MB1_MASK: u32 = (0x3 << 10);
pub const RT5682S_DBG_BGLDO_MB1_SFT: u32 = 10;
pub const RT5682S_DBG_BGLDO_MB2_MASK: u32 = (0x3 << 8);
pub const RT5682S_DBG_BGLDO_MB2_SFT: u32 = 8;
pub const RT5682S_DLDO_BGLDO_MASK: u32 = (0x3 << 6);
pub const RT5682S_DLDO_BGLDO_MB2_SFT: u32 = 6;
pub const RT5682S_PWR_STO1_DAC_L: u32 = (0x1 << 5);
pub const RT5682S_PWR_STO1_DAC_L_BIT: u32 = 5;
pub const RT5682S_PWR_STO1_DAC_R: u32 = (0x1 << 4);
pub const RT5682S_PWR_STO1_DAC_R_BIT: u32 = 4;
pub const RT5682S_DVO_BGLDO_MB1_MASK: u32 = (0x3 << 2);
pub const RT5682S_DVO_BGLDO_MB1_SFT: u32 = 2;
pub const RT5682S_DVO_BGLDO_MB2_MASK: u32 = (0x3 << 0);

// MCLK and System Clock Detection Control (0x006b) 
pub const RT5682S_SYS_CLK_DET: u32 = (0x1 << 15);
pub const RT5682S_SYS_CLK_DET_SFT: u32 = 15;
pub const RT5682S_PLL1_CLK_DET: u32 = (0x1 << 14);
pub const RT5682S_PLL1_CLK_DET_SFT: u32 = 14;

// Digital Microphone Control 1 (0x006e) 
pub const RT5682S_DMIC_1_EN_MASK: u32 = (0x1 << 15);
pub const RT5682S_DMIC_1_EN_SFT: u32 = 15;
pub const RT5682S_DMIC_1_DIS: u32 = (0x0 << 15);
pub const RT5682S_DMIC_1_EN: u32 = (0x1 << 15);
pub const RT5682S_FIFO_CLK_DIV_MASK: u32 = (0x7 << 12);
pub const RT5682S_FIFO_CLK_DIV_2: u32 = (0x1 << 12);
pub const RT5682S_DMIC_1_DP_MASK: u32 = (0x3 << 4);
pub const RT5682S_DMIC_1_DP_SFT: u32 = 4;
pub const RT5682S_DMIC_1_DP_GPIO2: u32 = (0x0 << 4);
pub const RT5682S_DMIC_1_DP_GPIO5: u32 = (0x1 << 4);
pub const RT5682S_DMIC_CLK_MASK: u32 = (0xf << 0);
pub const RT5682S_DMIC_CLK_SFT: u32 = 0;

// I2S1 Audio Serial Data Port Control (0x0070) 
pub const RT5682S_SEL_ADCDAT_MASK: u32 = (0x1 << 15);
pub const RT5682S_SEL_ADCDAT_OUT: u32 = (0x0 << 15);
pub const RT5682S_SEL_ADCDAT_IN: u32 = (0x1 << 15);
pub const RT5682S_SEL_ADCDAT_SFT: u32 = 15;
pub const RT5682S_I2S1_TX_CHL_MASK: u32 = (0x7 << 12);
pub const RT5682S_I2S1_TX_CHL_SFT: u32 = 12;
pub const RT5682S_I2S1_TX_CHL_16: u32 = (0x0 << 12);
pub const RT5682S_I2S1_TX_CHL_20: u32 = (0x1 << 12);
pub const RT5682S_I2S1_TX_CHL_24: u32 = (0x2 << 12);
pub const RT5682S_I2S1_TX_CHL_32: u32 = (0x3 << 12);
pub const RT5682S_I2S1_TX_CHL_8: u32 = (0x4 << 12);
pub const RT5682S_I2S1_RX_CHL_MASK: u32 = (0x7 << 8);
pub const RT5682S_I2S1_RX_CHL_SFT: u32 = 8;
pub const RT5682S_I2S1_RX_CHL_16: u32 = (0x0 << 8);
pub const RT5682S_I2S1_RX_CHL_20: u32 = (0x1 << 8);
pub const RT5682S_I2S1_RX_CHL_24: u32 = (0x2 << 8);
pub const RT5682S_I2S1_RX_CHL_32: u32 = (0x3 << 8);
pub const RT5682S_I2S1_RX_CHL_8: u32 = (0x4 << 8);
pub const RT5682S_I2S1_MONO_MASK: u32 = (0x1 << 7);
pub const RT5682S_I2S1_MONO_EN: u32 = (0x1 << 7);
pub const RT5682S_I2S1_MONO_DIS: u32 = (0x0 << 7);
pub const RT5682S_I2S1_DL_MASK: u32 = (0x7 << 4);
pub const RT5682S_I2S1_DL_SFT: u32 = 4;
pub const RT5682S_I2S1_DL_16: u32 = (0x0 << 4);
pub const RT5682S_I2S1_DL_20: u32 = (0x1 << 4);
pub const RT5682S_I2S1_DL_24: u32 = (0x2 << 4);
pub const RT5682S_I2S1_DL_32: u32 = (0x3 << 4);
pub const RT5682S_I2S1_DL_8: u32 = (0x4 << 4);

// I2S1/2 Audio Serial Data Port Control (0x0071) 
pub const RT5682S_I2S2_MS_MASK: u32 = (0x1 << 15);
pub const RT5682S_I2S2_MS_SFT: u32 = 15;
pub const RT5682S_I2S2_MS_M: u32 = (0x0 << 15);
pub const RT5682S_I2S2_MS_S: u32 = (0x1 << 15);
pub const RT5682S_I2S2_PIN_CFG_MASK: u32 = (0x1 << 14);
pub const RT5682S_I2S2_PIN_CFG_SFT: u32 = 14;
pub const RT5682S_I2S2_OUT_MASK: u32 = (0x1 << 9);
pub const RT5682S_I2S2_OUT_SFT: u32 = 9;
pub const RT5682S_I2S2_OUT_UM: u32 = (0x0 << 9);
pub const RT5682S_I2S2_OUT_M: u32 = (0x1 << 9);
pub const RT5682S_I2S_BP_MASK: u32 = (0x1 << 8);
pub const RT5682S_I2S_BP_SFT: u32 = 8;
pub const RT5682S_I2S_BP_NOR: u32 = (0x0 << 8);
pub const RT5682S_I2S_BP_INV: u32 = (0x1 << 8);
pub const RT5682S_I2S2_MONO_MASK: u32 = (0x1 << 7);
pub const RT5682S_I2S2_MONO_EN: u32 = (0x1 << 7);
pub const RT5682S_I2S2_MONO_DIS: u32 = (0x0 << 7);
pub const RT5682S_I2S2_DL_MASK: u32 = (0x7 << 4);
pub const RT5682S_I2S2_DL_SFT: u32 = 4;
pub const RT5682S_I2S2_DL_8: u32 = (0x0 << 4);
pub const RT5682S_I2S2_DL_16: u32 = (0x1 << 4);
pub const RT5682S_I2S2_DL_20: u32 = (0x2 << 4);
pub const RT5682S_I2S2_DL_24: u32 = (0x3 << 4);
pub const RT5682S_I2S2_DL_32: u32 = (0x4 << 4);
pub const RT5682S_I2S_DF_MASK: u32 = (0x7);
pub const RT5682S_I2S_DF_SFT: u32 = 0;
pub const RT5682S_I2S_DF_I2S: u32 = (0x0);
pub const RT5682S_I2S_DF_LEFT: u32 = (0x1);
pub const RT5682S_I2S_DF_PCM_A: u32 = (0x2);
pub const RT5682S_I2S_DF_PCM_B: u32 = (0x3);
pub const RT5682S_I2S_DF_PCM_A_N: u32 = (0x6);
pub const RT5682S_I2S_DF_PCM_B_N: u32 = (0x7);

// ADC/DAC Clock Control 1 (0x0073) 
pub const RT5682S_ADC_OSR_MASK: u32 = (0xf << 12);
pub const RT5682S_ADC_OSR_SFT: u32 = 12;
pub const RT5682S_ADC_OSR_D_1: u32 = (0x0 << 12);
pub const RT5682S_ADC_OSR_D_2: u32 = (0x1 << 12);
pub const RT5682S_ADC_OSR_D_4: u32 = (0x2 << 12);
pub const RT5682S_ADC_OSR_D_6: u32 = (0x3 << 12);
pub const RT5682S_ADC_OSR_D_8: u32 = (0x4 << 12);
pub const RT5682S_ADC_OSR_D_12: u32 = (0x5 << 12);
pub const RT5682S_ADC_OSR_D_16: u32 = (0x6 << 12);
pub const RT5682S_ADC_OSR_D_24: u32 = (0x7 << 12);
pub const RT5682S_ADC_OSR_D_32: u32 = (0x8 << 12);
pub const RT5682S_ADC_OSR_D_48: u32 = (0x9 << 12);
pub const RT5682S_I2S_M_D_MASK: u32 = (0xf << 8);
pub const RT5682S_I2S_M_D_SFT: u32 = 8;
pub const RT5682S_I2S_M_D_1: u32 = (0x0 << 8);
pub const RT5682S_I2S_M_D_2: u32 = (0x1 << 8);
pub const RT5682S_I2S_M_D_3: u32 = (0x2 << 8);
pub const RT5682S_I2S_M_D_4: u32 = (0x3 << 8);
pub const RT5682S_I2S_M_D_6: u32 = (0x4 << 8);
pub const RT5682S_I2S_M_D_8: u32 = (0x5 << 8);
pub const RT5682S_I2S_M_D_12: u32 = (0x6 << 8);
pub const RT5682S_I2S_M_D_16: u32 = (0x7 << 8);
pub const RT5682S_I2S_M_D_24: u32 = (0x8 << 8);
pub const RT5682S_I2S_M_D_32: u32 = (0x9 << 8);
pub const RT5682S_I2S_M_D_48: u32 = (0x10 << 8);
pub const RT5682S_I2S_M_CLK_SRC_MASK: u32 = (0x7 << 4);
pub const RT5682S_I2S_M_CLK_SRC_SFT: u32 = 4;
pub const RT5682S_DAC_OSR_MASK: u32 = (0xf << 0);
pub const RT5682S_DAC_OSR_SFT: u32 = 0;
pub const RT5682S_DAC_OSR_D_1: u32 = (0x0 << 0);
pub const RT5682S_DAC_OSR_D_2: u32 = (0x1 << 0);
pub const RT5682S_DAC_OSR_D_4: u32 = (0x2 << 0);
pub const RT5682S_DAC_OSR_D_6: u32 = (0x3 << 0);
pub const RT5682S_DAC_OSR_D_8: u32 = (0x4 << 0);
pub const RT5682S_DAC_OSR_D_12: u32 = (0x5 << 0);
pub const RT5682S_DAC_OSR_D_16: u32 = (0x6 << 0);
pub const RT5682S_DAC_OSR_D_24: u32 = (0x7 << 0);
pub const RT5682S_DAC_OSR_D_32: u32 = (0x8 << 0);
pub const RT5682S_DAC_OSR_D_48: u32 = (0x9 << 0);

// ADC/DAC Clock Control 2 (0x0074) 
pub const RT5682S_I2S2_BCLK_MS2_MASK: u32 = (0x1 << 11);
pub const RT5682S_I2S2_BCLK_MS2_SFT: u32 = 11;
pub const RT5682S_I2S2_BCLK_MS2_32: u32 = (0x0 << 11);
pub const RT5682S_I2S2_BCLK_MS2_64: u32 = (0x1 << 11);


// TDM control 1 (0x0079) 
pub const RT5682S_TDM_TX_CH_MASK: u32 = (0x3 << 12);
pub const RT5682S_TDM_TX_CH_2: u32 = (0x0 << 12);
pub const RT5682S_TDM_TX_CH_4: u32 = (0x1 << 12);
pub const RT5682S_TDM_TX_CH_6: u32 = (0x2 << 12);
pub const RT5682S_TDM_TX_CH_8: u32 = (0x3 << 12);
pub const RT5682S_TDM_RX_CH_MASK: u32 = (0x3 << 8);
pub const RT5682S_TDM_RX_CH_2: u32 = (0x0 << 8);
pub const RT5682S_TDM_RX_CH_4: u32 = (0x1 << 8);
pub const RT5682S_TDM_RX_CH_6: u32 = (0x2 << 8);
pub const RT5682S_TDM_RX_CH_8: u32 = (0x3 << 8);
pub const RT5682S_TDM_ADC_LCA_MASK: u32 = (0x7 << 4);
pub const RT5682S_TDM_ADC_LCA_SFT: u32 = 4;
pub const RT5682S_TDM_ADC_DL_MASK: u32 = (0x3 << 0);
pub const RT5682S_TDM_ADC_DL_SFT: u32 = 0;

// TDM control 2 (0x007a) 
pub const RT5682S_IF1_ADC1_SEL_SFT: u32 = 14;
pub const RT5682S_IF1_ADC2_SEL_SFT: u32 = 12;
pub const RT5682S_IF1_ADC3_SEL_SFT: u32 = 10;
pub const RT5682S_IF1_ADC4_SEL_SFT: u32 = 8;
pub const RT5682S_TDM_ADC_SEL_SFT: u32 = 3;

// TDM control 3 (0x007b) 
pub const RT5682S_TDM_EN: u32 = (0x1 << 7);

// TDM/I2S control (0x007e) 
pub const RT5682S_TDM_S_BP_MASK: u32 = (0x1 << 15);
pub const RT5682S_TDM_S_BP_SFT: u32 = 15;
pub const RT5682S_TDM_S_BP_NOR: u32 = (0x0 << 15);
pub const RT5682S_TDM_S_BP_INV: u32 = (0x1 << 15);
pub const RT5682S_TDM_S_LP_MASK: u32 = (0x1 << 14);
pub const RT5682S_TDM_S_LP_SFT: u32 = 14;
pub const RT5682S_TDM_S_LP_NOR: u32 = (0x0 << 14);
pub const RT5682S_TDM_S_LP_INV: u32 = (0x1 << 14);
pub const RT5682S_TDM_DF_MASK: u32 = (0x7 << 11);
pub const RT5682S_TDM_DF_SFT: u32 = 11;
pub const RT5682S_TDM_DF_I2S: u32 = (0x0 << 11);
pub const RT5682S_TDM_DF_LEFT: u32 = (0x1 << 11);
pub const RT5682S_TDM_DF_PCM_A: u32 = (0x2 << 11);
pub const RT5682S_TDM_DF_PCM_B: u32 = (0x3 << 11);
pub const RT5682S_TDM_DF_PCM_A_N: u32 = (0x6 << 11);
pub const RT5682S_TDM_DF_PCM_B_N: u32 = (0x7 << 11);
pub const RT5682S_TDM_BCLK_MS1_MASK: u32 = (0x3 << 8);
pub const RT5682S_TDM_BCLK_MS1_SFT: u32 = 8;
pub const RT5682S_TDM_BCLK_MS1_32: u32 = (0x0 << 8);
pub const RT5682S_TDM_BCLK_MS1_64: u32 = (0x1 << 8);
pub const RT5682S_TDM_BCLK_MS1_128: u32 = (0x2 << 8);
pub const RT5682S_TDM_BCLK_MS1_256: u32 = (0x3 << 8);
pub const RT5682S_TDM_BCLK_MS1_16: u32 = (0x4 << 8);
pub const RT5682S_TDM_CL_MASK: u32 = (0x3 << 4);
pub const RT5682S_TDM_CL_16: u32 = (0x0 << 4);
pub const RT5682S_TDM_CL_20: u32 = (0x1 << 4);
pub const RT5682S_TDM_CL_24: u32 = (0x2 << 4);
pub const RT5682S_TDM_CL_32: u32 = (0x3 << 4);
pub const RT5682S_TDM_M_BP_MASK: u32 = (0x1 << 2);
pub const RT5682S_TDM_M_BP_SFT: u32 = 2;
pub const RT5682S_TDM_M_BP_NOR: u32 = (0x0 << 2);
pub const RT5682S_TDM_M_BP_INV: u32 = (0x1 << 2);
pub const RT5682S_TDM_M_LP_MASK: u32 = (0x1 << 1);
pub const RT5682S_TDM_M_LP_SFT: u32 = 1;
pub const RT5682S_TDM_M_LP_NOR: u32 = (0x0 << 1);
pub const RT5682S_TDM_M_LP_INV: u32 = (0x1 << 1);
pub const RT5682S_TDM_MS_MASK: u32 = (0x1 << 0);
pub const RT5682S_TDM_MS_SFT: u32 = 0;
pub const RT5682S_TDM_MS_S: u32 = (0x0 << 0);
pub const RT5682S_TDM_MS_M: u32 = (0x1 << 0);

// Global Clock Control (0x0080) 
pub const RT5682S_SCLK_SRC_MASK: u32 = (0x7 << 13);
pub const RT5682S_SCLK_SRC_SFT: u32 = 13;
pub const RT5682S_PLL_SRC_MASK: u32 = (0x3 << 8);
pub const RT5682S_PLL_SRC_SFT: u32 = 8;
pub const RT5682S_PLL_SRC_MCLK: u32 = (0x0 << 8);
pub const RT5682S_PLL_SRC_BCLK1: u32 = (0x1 << 8);
pub const RT5682S_PLL_SRC_RC: u32 = (0x3 << 8);

// PLL tracking mode 1 (0x0083) 
pub const RT5682S_DA_ASRC_MASK: u32 = (0x1 << 13);
pub const RT5682S_DA_ASRC_SFT: u32 = 13;
pub const RT5682S_DAC_STO1_ASRC_MASK: u32 = (0x1 << 12);
pub const RT5682S_DAC_STO1_ASRC_SFT: u32 = 12;
pub const RT5682S_AD_ASRC_MASK: u32 = (0x1 << 8);
pub const RT5682S_AD_ASRC_SFT: u32 = 8;
pub const RT5682S_AD_ASRC_SEL_MASK: u32 = (0x1 << 4);
pub const RT5682S_AD_ASRC_SEL_SFT: u32 = 4;
pub const RT5682S_DMIC_ASRC_MASK: u32 = (0x1 << 3);
pub const RT5682S_DMIC_ASRC_SFT: u32 = 3;
pub const RT5682S_ADC_STO1_ASRC_MASK: u32 = (0x1 << 2);
pub const RT5682S_ADC_STO1_ASRC_SFT: u32 = 2;
pub const RT5682S_DA_ASRC_SEL_MASK: u32 = (0x1 << 0);
pub const RT5682S_DA_ASRC_SEL_SFT: u32 = 0;

// PLL tracking mode 2 3 (0x0084)(0x0085)
pub const RT5682S_FILTER_CLK_SEL_MASK: u32 = (0x7 << 12);
pub const RT5682S_FILTER_CLK_SEL_SFT: u32 = 12;
pub const RT5682S_FILTER_CLK_DIV_MASK: u32 = (0xf << 8);
pub const RT5682S_FILTER_CLK_DIV_SFT: u32 = 8;

// ASRC Control 4 (0x0086) 
pub const RT5682S_ASRCIN_FTK_N1_MASK: u32 = (0x3 << 14);
pub const RT5682S_ASRCIN_FTK_N1_SFT: u32 = 14;
pub const RT5682S_ASRCIN_FTK_N2_MASK: u32 = (0x3 << 12);
pub const RT5682S_ASRCIN_FTK_N2_SFT: u32 = 12;
pub const RT5682S_ASRCIN_FTK_M1_MASK: u32 = (0x7 << 8);
pub const RT5682S_ASRCIN_FTK_M1_SFT: u32 = 8;
pub const RT5682S_ASRCIN_FTK_M2_MASK: u32 = (0x7 << 4);
pub const RT5682S_ASRCIN_FTK_M2_SFT: u32 = 4;

// ASRC Control 11 (0x008c) 
pub const RT5682S_ASRCIN_AUTO_CLKOUT_MASK: u32 = (0x1 << 5);
pub const RT5682S_ASRCIN_AUTO_CLKOUT_EN: u32 = (0x1 << 5);
pub const RT5682S_ASRCIN_AUTO_CLKOUT_DIS: u32 = (0x0 << 5);
pub const RT5682S_ASRCIN_AUTO_RST_MASK: u32 = (0x1 << 4);
pub const RT5682S_ASRCIN_AUTO_RST_EN: u32 = (0x1 << 4);
pub const RT5682S_ASRCIN_AUTO_RST_DIS: u32 = (0x0 << 4);
pub const RT5682S_SEL_LRCK_DET_MASK: u32 = (0x3);
pub const RT5682S_SEL_LRCK_DET_DIV8: u32 = (0x3);
pub const RT5682S_SEL_LRCK_DET_DIV4: u32 = (0x2);
pub const RT5682S_SEL_LRCK_DET_DIV2: u32 = (0x1);
pub const RT5682S_SEL_LRCK_DET_DIV1: u32 = (0x0);

// Depop Mode Control 1 (0x008e) 
pub const RT5682S_OUT_HP_L_EN: u32 = (0x1 << 6);
pub const RT5682S_OUT_HP_R_EN: u32 = (0x1 << 5);
pub const RT5682S_LDO_PUMP_EN: u32 = (0x1 << 4);
pub const RT5682S_LDO_PUMP_EN_SFT: u32 = 4;
pub const RT5682S_PUMP_EN: u32 = (0x1 << 3);
pub const RT5682S_PUMP_EN_SFT: u32 = 3;
pub const RT5682S_CAPLESS_L_EN: u32 = (0x1 << 1);
pub const RT5682S_CAPLESS_L_EN_SFT: u32 = 1;
pub const RT5682S_CAPLESS_R_EN: u32 = (0x1 << 0);
pub const RT5682S_CAPLESS_R_EN_SFT: u32 = 0;

// Depop Mode Control 2 (0x8f) 
pub const RT5682S_RAMP_MASK: u32 = (0x1 << 12);
pub const RT5682S_RAMP_SFT: u32 = 12;
pub const RT5682S_RAMP_DIS: u32 = (0x0 << 12);
pub const RT5682S_RAMP_EN: u32 = (0x1 << 12);
pub const RT5682S_BPS_MASK: u32 = (0x1 << 11);
pub const RT5682S_BPS_SFT: u32 = 11;
pub const RT5682S_BPS_DIS: u32 = (0x0 << 11);
pub const RT5682S_BPS_EN: u32 = (0x1 << 11);
pub const RT5682S_FAST_UPDN_MASK: u32 = (0x1 << 10);
pub const RT5682S_FAST_UPDN_SFT: u32 = 10;
pub const RT5682S_FAST_UPDN_DIS: u32 = (0x0 << 10);
pub const RT5682S_FAST_UPDN_EN: u32 = (0x1 << 10);
pub const RT5682S_VLO_MASK: u32 = (0x1 << 7);
pub const RT5682S_VLO_SFT: u32 = 7;
pub const RT5682S_VLO_3V: u32 = (0x0 << 7);
pub const RT5682S_VLO_33V: u32 = (0x1 << 7);

// HPOUT charge pump 1 (0x0091) 
pub const RT5682S_OSW_L_MASK: u32 = (0x1 << 11);
pub const RT5682S_OSW_L_SFT: u32 = 11;
pub const RT5682S_OSW_L_DIS: u32 = (0x0 << 11);
pub const RT5682S_OSW_L_EN: u32 = (0x1 << 11);
pub const RT5682S_OSW_R_MASK: u32 = (0x1 << 10);
pub const RT5682S_OSW_R_SFT: u32 = 10;
pub const RT5682S_OSW_R_DIS: u32 = (0x0 << 10);
pub const RT5682S_OSW_R_EN: u32 = (0x1 << 10);
pub const RT5682S_PM_HP_MASK: u32 = (0x3 << 8);
pub const RT5682S_PM_HP_SFT: u32 = 8;
pub const RT5682S_PM_HP_LV: u32 = (0x0 << 8);
pub const RT5682S_PM_HP_MV: u32 = (0x1 << 8);
pub const RT5682S_PM_HP_HV: u32 = (0x2 << 8);

// Micbias Control1 (0x93) 
pub const RT5682S_MIC1_OV_MASK: u32 = (0x3 << 14);
pub const RT5682S_MIC1_OV_SFT: u32 = 14;
pub const RT5682S_MIC1_OV_2V7: u32 = (0x0 << 14);
pub const RT5682S_MIC1_OV_2V4: u32 = (0x1 << 14);
pub const RT5682S_MIC1_OV_2V25: u32 = (0x3 << 14);
pub const RT5682S_MIC1_OV_1V8: u32 = (0x4 << 14);
pub const RT5682S_MIC2_OV_MASK: u32 = (0x3 << 8);
pub const RT5682S_MIC2_OV_SFT: u32 = 8;
pub const RT5682S_MIC2_OV_2V7: u32 = (0x0 << 8);
pub const RT5682S_MIC2_OV_2V4: u32 = (0x1 << 8);
pub const RT5682S_MIC2_OV_2V25: u32 = (0x3 << 8);
pub const RT5682S_MIC2_OV_1V8: u32 = (0x4 << 8);

// Micbias Control2 (0x0094) 
pub const RT5682S_PWR_CLK25M_MASK: u32 = (0x1 << 9);
pub const RT5682S_PWR_CLK25M_SFT: u32 = 9;
pub const RT5682S_PWR_CLK25M_PD: u32 = (0x0 << 9);
pub const RT5682S_PWR_CLK25M_PU: u32 = (0x1 << 9);
pub const RT5682S_PWR_CLK1M_MASK: u32 = (0x1 << 8);
pub const RT5682S_PWR_CLK1M_SFT: u32 = 8;
pub const RT5682S_PWR_CLK1M_PD: u32 = (0x0 << 8);
pub const RT5682S_PWR_CLK1M_PU: u32 = (0x1 << 8);

// PLL M/N/K Code Control 1 (0x0098) 
pub const RT5682S_PLLA_N_MASK: u32 = (0x1ff << 0);

// PLL M/N/K Code Control 2 (0x0099) 
pub const RT5682S_PLLA_M_MASK: u32 = (0x1f << 8);
pub const RT5682S_PLLA_M_SFT: u32 = 8;
pub const RT5682S_PLLA_K_MASK: u32 = (0x1f << 0);

// PLL M/N/K Code Control 3 (0x009a) 
pub const RT5682S_PLLB_N_MASK: u32 = (0x3ff << 0);

// PLL M/N/K Code Control 4 (0x009b) 
pub const RT5682S_PLLB_M_MASK: u32 = (0x1f << 8);
pub const RT5682S_PLLB_M_SFT: u32 = 8;
pub const RT5682S_PLLB_K_MASK: u32 = (0x1f << 0);

// PLL M/N/K Code Control 6 (0x009d) 
pub const RT5682S_PLLB_SEL_PS_MASK: u32 = (0x1 << 13);
pub const RT5682S_PLLB_SEL_PS_SFT: u32 = 13;
pub const RT5682S_PLLB_BYP_PS_MASK: u32 = (0x1 << 12);
pub const RT5682S_PLLB_BYP_PS_SFT: u32 = 12;
pub const RT5682S_PLLB_M_BP_MASK: u32 = (0x1 << 11);
pub const RT5682S_PLLB_M_BP_SFT: u32 = 11;
pub const RT5682S_PLLB_K_BP_MASK: u32 = (0x1 << 10);
pub const RT5682S_PLLB_K_BP_SFT: u32 = 10;
pub const RT5682S_PLLA_M_BP_MASK: u32 = (0x1 << 7);
pub const RT5682S_PLLA_M_BP_SFT: u32 = 7;
pub const RT5682S_PLLA_K_BP_MASK: u32 = (0x1 << 6);
pub const RT5682S_PLLA_K_BP_SFT: u32 = 6;

// PLL M/N/K Code Control 7 (0x009e) 
pub const RT5682S_PLLB_SRC_MASK: u32 = (0x1);
pub const RT5682S_PLLB_SRC_DFIN: u32 = (0x1);
pub const RT5682S_PLLB_SRC_PLLA: u32 = (0x0);

// RC Clock Control (0x009f) 
pub const RT5682S_POW_IRQ: u32 = (0x1 << 15);
pub const RT5682S_POW_JDH: u32 = (0x1 << 14);

// I2S2 Master Mode Clock Control 1 (0x00a0) 
pub const RT5682S_I2S2_M_CLK_SRC_MASK: u32 = (0x7 << 4);
pub const RT5682S_I2S2_M_CLK_SRC_SFT: u32 = 4;
pub const RT5682S_I2S2_M_D_MASK: u32 = (0xf << 0);
pub const RT5682S_I2S2_M_D_1: u32 = (0x0);
pub const RT5682S_I2S2_M_D_2: u32 = (0x1);
pub const RT5682S_I2S2_M_D_3: u32 = (0x2);
pub const RT5682S_I2S2_M_D_4: u32 = (0x3);
pub const RT5682S_I2S2_M_D_6: u32 = (0x4);
pub const RT5682S_I2S2_M_D_8: u32 = (0x5);
pub const RT5682S_I2S2_M_D_12: u32 = (0x6);
pub const RT5682S_I2S2_M_D_16: u32 = (0x7);
pub const RT5682S_I2S2_M_D_24: u32 = (0x8);
pub const RT5682S_I2S2_M_D_32: u32 = (0x9);
pub const RT5682S_I2S2_M_D_48: u32 = (0xa);
pub const RT5682S_I2S2_M_D_SFT: u32 = 0;

// IRQ Control 1 (0x00b6) 
pub const RT5682S_JD1_PULSE_EN_MASK: u32 = (0x1 << 10);
pub const RT5682S_JD1_PULSE_EN_SFT: u32 = 10;
pub const RT5682S_JD1_PULSE_DIS: u32 = (0x0 << 10);
pub const RT5682S_JD1_PULSE_EN: u32 = (0x1 << 10);

// IRQ Control 2 (0x00b7) 
pub const RT5682S_JD1_EN_MASK: u32 = (0x1 << 15);
pub const RT5682S_JD1_EN_SFT: u32 = 15;
pub const RT5682S_JD1_DIS: u32 = (0x0 << 15);
pub const RT5682S_JD1_EN: u32 = (0x1 << 15);
pub const RT5682S_JD1_POL_MASK: u32 = (0x1 << 13);
pub const RT5682S_JD1_POL_NOR: u32 = (0x0 << 13);
pub const RT5682S_JD1_POL_INV: u32 = (0x1 << 13);
pub const RT5682S_JD1_IRQ_MASK: u32 = (0x1 << 10);
pub const RT5682S_JD1_IRQ_LEV: u32 = (0x0 << 10);
pub const RT5682S_JD1_IRQ_PUL: u32 = (0x1 << 10);

// IRQ Control 3 (0x00b8) 
pub const RT5682S_IL_IRQ_MASK: u32 = (0x1 << 7);
pub const RT5682S_IL_IRQ_DIS: u32 = (0x0 << 7);
pub const RT5682S_IL_IRQ_EN: u32 = (0x1 << 7);
pub const RT5682S_IL_IRQ_TYPE_MASK: u32 = (0x1 << 4);
pub const RT5682S_IL_IRQ_LEV: u32 = (0x0 << 4);
pub const RT5682S_IL_IRQ_PUL: u32 = (0x1 << 4);

// GPIO Control 1 (0x00c0) 
pub const RT5682S_GP1_PIN_MASK: u32 = (0x3 << 14);
pub const RT5682S_GP1_PIN_SFT: u32 = 14;
pub const RT5682S_GP1_PIN_GPIO1: u32 = (0x0 << 14);
pub const RT5682S_GP1_PIN_IRQ: u32 = (0x1 << 14);
pub const RT5682S_GP1_PIN_DMIC_CLK: u32 = (0x2 << 14);
pub const RT5682S_GP2_PIN_MASK: u32 = (0x3 << 12);
pub const RT5682S_GP2_PIN_SFT: u32 = 12;
pub const RT5682S_GP2_PIN_GPIO2: u32 = (0x0 << 12);
pub const RT5682S_GP2_PIN_LRCK2: u32 = (0x1 << 12);
pub const RT5682S_GP2_PIN_DMIC_SDA: u32 = (0x2 << 12);
pub const RT5682S_GP3_PIN_MASK: u32 = (0x3 << 10);
pub const RT5682S_GP3_PIN_SFT: u32 = 10;
pub const RT5682S_GP3_PIN_GPIO3: u32 = (0x0 << 10);
pub const RT5682S_GP3_PIN_BCLK2: u32 = (0x1 << 10);
pub const RT5682S_GP3_PIN_DMIC_CLK: u32 = (0x2 << 10);
pub const RT5682S_GP4_PIN_MASK: u32 = (0x3 << 8);
pub const RT5682S_GP4_PIN_SFT: u32 = 8;
pub const RT5682S_GP4_PIN_GPIO4: u32 = (0x0 << 8);
pub const RT5682S_GP4_PIN_ADCDAT1: u32 = (0x1 << 8);
pub const RT5682S_GP4_PIN_DMIC_CLK: u32 = (0x2 << 8);
pub const RT5682S_GP4_PIN_ADCDAT2: u32 = (0x3 << 8);
pub const RT5682S_GP5_PIN_MASK: u32 = (0x3 << 6);
pub const RT5682S_GP5_PIN_SFT: u32 = 6;
pub const RT5682S_GP5_PIN_GPIO5: u32 = (0x0 << 6);
pub const RT5682S_GP5_PIN_DACDAT1: u32 = (0x1 << 6);
pub const RT5682S_GP5_PIN_DMIC_SDA: u32 = (0x2 << 6);
pub const RT5682S_GP6_PIN_MASK: u32 = (0x1 << 5);
pub const RT5682S_GP6_PIN_SFT: u32 = 5;
pub const RT5682S_GP6_PIN_GPIO6: u32 = (0x0 << 5);
pub const RT5682S_GP6_PIN_LRCK1: u32 = (0x1 << 5);

// GPIO Control 2 (0x00c1)
pub const RT5682S_GP1_PF_MASK: u32 = (0x1 << 15);
pub const RT5682S_GP1_PF_IN: u32 = (0x0 << 15);
pub const RT5682S_GP1_PF_OUT: u32 = (0x1 << 15);
pub const RT5682S_GP1_OUT_MASK: u32 = (0x1 << 14);
pub const RT5682S_GP1_OUT_L: u32 = (0x0 << 14);
pub const RT5682S_GP1_OUT_H: u32 = (0x1 << 14);
pub const RT5682S_GP2_PF_MASK: u32 = (0x1 << 13);
pub const RT5682S_GP2_PF_IN: u32 = (0x0 << 13);
pub const RT5682S_GP2_PF_OUT: u32 = (0x1 << 13);
pub const RT5682S_GP2_OUT_MASK: u32 = (0x1 << 12);
pub const RT5682S_GP2_OUT_L: u32 = (0x0 << 12);
pub const RT5682S_GP2_OUT_H: u32 = (0x1 << 12);
pub const RT5682S_GP3_PF_MASK: u32 = (0x1 << 11);
pub const RT5682S_GP3_PF_IN: u32 = (0x0 << 11);
pub const RT5682S_GP3_PF_OUT: u32 = (0x1 << 11);
pub const RT5682S_GP3_OUT_MASK: u32 = (0x1 << 10);
pub const RT5682S_GP3_OUT_L: u32 = (0x0 << 10);
pub const RT5682S_GP3_OUT_H: u32 = (0x1 << 10);
pub const RT5682S_GP4_PF_MASK: u32 = (0x1 << 9);
pub const RT5682S_GP4_PF_IN: u32 = (0x0 << 9);
pub const RT5682S_GP4_PF_OUT: u32 = (0x1 << 9);
pub const RT5682S_GP4_OUT_MASK: u32 = (0x1 << 8);
pub const RT5682S_GP4_OUT_L: u32 = (0x0 << 8);
pub const RT5682S_GP4_OUT_H: u32 = (0x1 << 8);
pub const RT5682S_GP5_PF_MASK: u32 = (0x1 << 7);
pub const RT5682S_GP5_PF_IN: u32 = (0x0 << 7);
pub const RT5682S_GP5_PF_OUT: u32 = (0x1 << 7);
pub const RT5682S_GP5_OUT_MASK: u32 = (0x1 << 6);
pub const RT5682S_GP5_OUT_L: u32 = (0x0 << 6);
pub const RT5682S_GP5_OUT_H: u32 = (0x1 << 6);
pub const RT5682S_GP6_PF_MASK: u32 = (0x1 << 5);
pub const RT5682S_GP6_PF_IN: u32 = (0x0 << 5);
pub const RT5682S_GP6_PF_OUT: u32 = (0x1 << 5);
pub const RT5682S_GP6_OUT_MASK: u32 = (0x1 << 4);
pub const RT5682S_GP6_OUT_L: u32 = (0x0 << 4);
pub const RT5682S_GP6_OUT_H: u32 = (0x1 << 4);

// GPIO Status (0x00c2) 
pub const RT5682S_GP6_ST: u32 = (0x1 << 6);
pub const RT5682S_GP5_ST: u32 = (0x1 << 5);
pub const RT5682S_GP4_ST: u32 = (0x1 << 4);
pub const RT5682S_GP3_ST: u32 = (0x1 << 3);
pub const RT5682S_GP2_ST: u32 = (0x1 << 2);
pub const RT5682S_GP1_ST: u32 = (0x1 << 1);

// Soft volume and zero cross control 1 (0x00d9) 
pub const RT5682S_ZCD_MASK: u32 = (0x1 << 10);
pub const RT5682S_ZCD_SFT: u32 = 10;
pub const RT5682S_ZCD_PD: u32 = (0x0 << 10);
pub const RT5682S_ZCD_PU: u32 = (0x1 << 10);

// 4 Button Inline Command Control 2 (0x00e3) 
pub const RT5682S_4BTN_IL_MASK: u32 = (0x1 << 15);
pub const RT5682S_4BTN_IL_EN: u32 = (0x1 << 15);
pub const RT5682S_4BTN_IL_DIS: u32 = (0x0 << 15);
pub const RT5682S_4BTN_IL_RST_MASK: u32 = (0x1 << 14);
pub const RT5682S_4BTN_IL_NOR: u32 = (0x1 << 14);
pub const RT5682S_4BTN_IL_RST: u32 = (0x0 << 14);

// 4 Button Inline Command Control 3~6 (0x00e5~0x00e8) 
pub const RT5682S_4BTN_IL_HOLD_WIN_MASK: u32 = (0x7f << 8);
pub const RT5682S_4BTN_IL_HOLD_WIN_SFT: u32 = 8;
pub const RT5682S_4BTN_IL_CLICK_WIN_MASK: u32 = (0x7f);
pub const RT5682S_4BTN_IL_CLICK_WIN_SFT: u32 = 0;

// Analog JD Control (0x00f0) 
pub const RT5682S_JDH_RS_MASK: u32 = (0x1 << 4);
pub const RT5682S_JDH_NO_PLUG: u32 = (0x1 << 4);
pub const RT5682S_JDH_PLUG: u32 = (0x0 << 4);

// Bias current control 7  (0x0110) 
pub const RT5682S_LDO_DACREF_MASK: u32 = (0x3 << 4);
pub const RT5682S_LDO_DACREF_1_607V: u32 = (0x0 << 4);
pub const RT5682S_LDO_DACREF_1_5V: u32 = (0x1 << 4);
pub const RT5682S_LDO_DACREF_1_406V: u32 = (0x2 << 4);
pub const RT5682S_LDO_DACREF_1_731V: u32 = (0x3 << 4);

// Charge Pump Internal Register1 (0x0125) 
pub const RT5682S_CP_CLK_HP_MASK: u32 = (0x3 << 4);
pub const RT5682S_CP_CLK_HP_100KHZ: u32 = (0x0 << 4);
pub const RT5682S_CP_CLK_HP_200KHZ: u32 = (0x1 << 4);
pub const RT5682S_CP_CLK_HP_300KHZ: u32 = (0x2 << 4);
pub const RT5682S_CP_CLK_HP_600KHZ: u32 = (0x3 << 4);

// Pad Driving Control (0x0136) 
pub const RT5682S_PAD_DRV_GP1_MASK: u32 = (0x1 << 14);
pub const RT5682S_PAD_DRV_GP1_HIGH: u32 = (0x1 << 14);
pub const RT5682S_PAD_DRV_GP1_LOW: u32 = (0x0 << 14);
pub const RT5682S_PAD_DRV_GP2_MASK: u32 = (0x1 << 12);
pub const RT5682S_PAD_DRV_GP2_HIGH: u32 = (0x1 << 12);
pub const RT5682S_PAD_DRV_GP2_LOW: u32 = (0x0 << 12);
pub const RT5682S_PAD_DRV_GP3_MASK: u32 = (0x1 << 10);
pub const RT5682S_PAD_DRV_GP3_HIGH: u32 = (0x1 << 10);
pub const RT5682S_PAD_DRV_GP3_LOW: u32 = (0x0 << 10);
pub const RT5682S_PAD_DRV_GP4_MASK: u32 = (0x1 << 8);
pub const RT5682S_PAD_DRV_GP4_HIGH: u32 = (0x1 << 8);
pub const RT5682S_PAD_DRV_GP4_LOW: u32 = (0x0 << 8);
pub const RT5682S_PAD_DRV_GP5_MASK: u32 = (0x1 << 6);
pub const RT5682S_PAD_DRV_GP5_HIGH: u32 = (0x1 << 6);
pub const RT5682S_PAD_DRV_GP5_LOW: u32 = (0x0 << 6);
pub const RT5682S_PAD_DRV_GP6_MASK: u32 = (0x1 << 4);
pub const RT5682S_PAD_DRV_GP6_HIGH: u32 = (0x1 << 4);
pub const RT5682S_PAD_DRV_GP6_LOW: u32 = (0x0 << 4);

// Chopper and Clock control for DAC (0x013a)
pub const RT5682S_CKXEN_DAC1_MASK: u32 = (0x1 << 13);
pub const RT5682S_CKXEN_DAC1_SFT: u32 = 13;
pub const RT5682S_CKGEN_DAC1_MASK: u32 = (0x1 << 12);
pub const RT5682S_CKGEN_DAC1_SFT: u32 = 12;

// Chopper and Clock control for ADC (0x013b)
pub const RT5682S_CKXEN_ADC1_MASK: u32 = (0x1 << 13);
pub const RT5682S_CKXEN_ADC1_SFT: u32 = 13;
pub const RT5682S_CKGEN_ADC1_MASK: u32 = (0x1 << 12);
pub const RT5682S_CKGEN_ADC1_SFT: u32 = 12;

// Volume test (0x013f)
pub const RT5682S_SEL_CLK_VOL_MASK: u32 = (0x1 << 15);
pub const RT5682S_SEL_CLK_VOL_EN: u32 = (0x1 << 15);
pub const RT5682S_SEL_CLK_VOL_DIS: u32 = (0x0 << 15);

// Test Mode Control 1 (0x0145) 
pub const RT5682S_AD2DA_LB_MASK: u32 = (0x1 << 10);
pub const RT5682S_AD2DA_LB_SFT: u32 = 10;

// Stereo Noise Gate Control 1 (0x0160) 
pub const RT5682S_NG2_EN_MASK: u32 = (0x1 << 15);
pub const RT5682S_NG2_EN: u32 = (0x1 << 15);
pub const RT5682S_NG2_DIS: u32 = (0x0 << 15);

// Stereo1 DAC Silence Detection Control (0x0190) 
pub const RT5682S_DEB_STO_DAC_MASK: u32 = (0x7 << 4);
pub const RT5682S_DEB_80_MS: u32 = (0x0 << 4);

// HP Behavior Logic Control 2 (0x01db) 
pub const RT5682S_HP_SIG_SRC_MASK: u32 = (0x3);
pub const RT5682S_HP_SIG_SRC_1BIT_CTL: u32 = (0x3);
pub const RT5682S_HP_SIG_SRC_REG: u32 = (0x2);
pub const RT5682S_HP_SIG_SRC_IMPE_REG: u32 = (0x1);
pub const RT5682S_HP_SIG_SRC_DC_CALI: u32 = (0x0);

// SAR ADC Inline Command Control 1 (0x0210) 
pub const RT5682S_SAR_BUTDET_MASK: u32 = (0x1 << 15);
pub const RT5682S_SAR_BUTDET_EN: u32 = (0x1 << 15);
pub const RT5682S_SAR_BUTDET_DIS: u32 = (0x0 << 15);
pub const RT5682S_SAR_BUTDET_POW_MASK: u32 = (0x1 << 14);
pub const RT5682S_SAR_BUTDET_POW_SAV: u32 = (0x1 << 14);
pub const RT5682S_SAR_BUTDET_POW_NORM: u32 = (0x0 << 14);
pub const RT5682S_SAR_BUTDET_RST_MASK: u32 = (0x1 << 13);
pub const RT5682S_SAR_BUTDET_RST_NORM: u32 = (0x1 << 13);
pub const RT5682S_SAR_BUTDET_RST: u32 = (0x0 << 13);
pub const RT5682S_SAR_POW_MASK: u32 = (0x1 << 12);
pub const RT5682S_SAR_POW_EN: u32 = (0x1 << 12);
pub const RT5682S_SAR_POW_DIS: u32 = (0x0 << 12);
pub const RT5682S_SAR_RST_MASK: u32 = (0x1 << 11);
pub const RT5682S_SAR_RST_NORMAL: u32 = (0x1 << 11);
pub const RT5682S_SAR_RST: u32 = (0x0 << 11);
pub const RT5682S_SAR_BYPASS_MASK: u32 = (0x1 << 10);
pub const RT5682S_SAR_BYPASS_EN: u32 = (0x1 << 10);
pub const RT5682S_SAR_BYPASS_DIS: u32 = (0x0 << 10);
pub const RT5682S_SAR_SEL_MB1_2_MASK: u32 = (0x3 << 8);
pub const RT5682S_SAR_SEL_MB1_2_SFT: u32 = 8;
pub const RT5682S_SAR_SEL_MODE_MASK: u32 = (0x1 << 7);
pub const RT5682S_SAR_SEL_MODE_CMP: u32 = (0x1 << 7);
pub const RT5682S_SAR_SEL_MODE_ADC: u32 = (0x0 << 7);
pub const RT5682S_SAR_SEL_MB1_2_CTL_MASK: u32 = (0x1 << 5);
pub const RT5682S_SAR_SEL_MB1_2_AUTO: u32 = (0x1 << 5);
pub const RT5682S_SAR_SEL_MB1_2_MANU: u32 = (0x0 << 5);
pub const RT5682S_SAR_SEL_SIGNAL_MASK: u32 = (0x1 << 4);
pub const RT5682S_SAR_SEL_SIGNAL_AUTO: u32 = (0x1 << 4);
pub const RT5682S_SAR_SEL_SIGNAL_MANU: u32 = (0x0 << 4);

// SAR ADC Inline Command Control 2 (0x0211) 
pub const RT5682S_SAR_ADC_PSV_MASK: u32 = (0x1 << 4);
pub const RT5682S_SAR_ADC_PSV_ENTRY: u32 = (0x1 << 4);


// SAR ADC Inline Command Control 13 (0x021c) 
pub const RT5682S_SAR_SOUR_MASK: u32 = (0x3f);
pub const RT5682S_SAR_SOUR_BTN: u32 = (0x3f);
pub const RT5682S_SAR_SOUR_TYPE: u32 = (0x0);

// Headphone Amp Detection Control 1 (0x3b00) 
pub const RT5682S_CP_SW_SIZE_MASK: u32 = (0x7 << 4);
pub const RT5682S_CP_SW_SIZE_L: u32 = (0x4 << 4);
pub const RT5682S_CP_SW_SIZE_M: u32 = (0x2 << 4);
pub const RT5682S_CP_SW_SIZE_S: u32 = (0x1 << 4);

pub const RT5682S_STEREO_RATES: u32 = SNDRV_PCM_RATE_8000_192000 as u32;
pub const RT5682S_FORMATS: u32 = (SNDRV_PCM_FMTBIT_S16_LE as u32 | SNDRV_PCM_FMTBIT_S20_3LE as u32 |  SNDRV_PCM_FMTBIT_S24_LE as u32 | SNDRV_PCM_FMTBIT_S8 as u32);

// System Clock Source 
pub const RT5682S_SCLK_S_MCLK: u32 = 0;
pub const RT5682S_SCLK_S_PLL1: u32 = 1;
pub const RT5682S_SCLK_S_PLL2: u32 = 2;
pub const RT5682S_SCLK_S_RCCLK: u32 = 3;

// PLL Source 
pub const RT5682S_PLL_S_MCLK: u32 = 0;
pub const RT5682S_PLL_S_BCLK1: u32 = 1;
pub const RT5682S_PLL_S_BCLK2: u32 = 2;
pub const RT5682S_PLL_S_RCCLK: u32 = 3;

pub const RT5682S_PLL1: usize = 0;
pub const RT5682S_PLL2: usize = 1;
pub const RT5682S_PLLS: usize = 2;

pub const RT5682S_AIF1: usize = 0;
pub const RT5682S_AIF2: usize = 1;
pub const RT5682S_AIFS: usize = 2;

// filter mask 
pub const RT5682S_DA_STEREO1_FILTER: u32 = 0x1;
pub const RT5682S_AD_STEREO1_FILTER: u32 = 0x1 << 1;

pub const RT5682S_CLK_SEL_SYS: u32 = 0;
pub const RT5682S_CLK_SEL_I2S1_ASRC: u32 = 1;
pub const RT5682S_CLK_SEL_I2S2_ASRC: u32 = 2;

pub const USE_PLLA: u32 = 0;
pub const USE_PLLB: u32 = 1;
pub const USE_PLLAB: u32 = 2;

#[repr(C)]
pub struct pll_calc_map {
    pub freq_in: ::std::os::raw::c_uint,
    pub freq_out: ::std::os::raw::c_uint,
    pub m: ::std::os::raw::c_int,
    pub n: ::std::os::raw::c_int,
    pub k: ::std::os::raw::c_int,
    pub m_bp: bool,
    pub k_bp: bool,
    pub byp_ps: bool,
    pub sel_ps: bool,
}

pub const RT5682S_SUPPLY_AVDD: usize = 0;
pub const RT5682S_SUPPLY_MICVDD: usize = 1;
pub const RT5682S_SUPPLY_DBVDD: usize = 2;
pub const RT5682S_SUPPLY_LDO1_IN: usize = 3;
pub const RT5682S_NUM_SUPPLIES: usize = 4;

#[repr(C)]
pub struct snd_soc_component { _private: [u8; 0] }
#[repr(C)]
pub struct rt5682s_platform_data { _private: [u8; 0] }
#[repr(C)]
pub struct gpio_desc { _private: [u8; 0] }
#[repr(C)]
pub struct regmap { _private: [u8; 0] }
#[repr(C)]
pub struct snd_soc_jack { _private: [u8; 0] }
#[repr(C)]
pub struct regulator_bulk_data { _private: [u8; 0] }
#[repr(C)]
pub struct delayed_work { _private: [u8; 0] }
#[repr(C)]
pub struct mutex { _private: [u8; 0] }
#[repr(C)]
pub struct clk_hw { _private: [u8; 0] }
#[repr(C)]
pub struct clk { _private: [u8; 0] }

#[repr(C)]
pub struct rt5682s_priv {
    pub component: *mut snd_soc_component,
    pub pdata: rt5682s_platform_data,
    pub ldo1_en: *mut gpio_desc,
    pub regmap: *mut regmap,
    pub hs_jack: *mut snd_soc_jack,
    pub supplies: [regulator_bulk_data; RT5682S_NUM_SUPPLIES],
    pub jack_detect_work: delayed_work,
    pub jd_check_work: delayed_work,
    pub calibrate_mutex: mutex,
    pub sar_mutex: mutex,
    pub wclk_mutex: mutex,

    // Present in C only when CONFIG_COMMON_CLK is enabled.
    pub dai_clks_hw: [clk_hw; RT5682S_DAI_NUM_CLKS],
    pub mclk: *mut clk,

    pub sysclk: ::std::os::raw::c_int,
    pub sysclk_src: ::std::os::raw::c_int,
    pub lrck: [::std::os::raw::c_int; RT5682S_AIFS],
    pub bclk: [::std::os::raw::c_int; RT5682S_AIFS],
    pub master: [::std::os::raw::c_int; RT5682S_AIFS],

    pub pll_src: [::std::os::raw::c_int; RT5682S_PLLS],
    pub pll_in: [::std::os::raw::c_int; RT5682S_PLLS],
    pub pll_out: [::std::os::raw::c_int; RT5682S_PLLS],
    pub pll_comb: ::std::os::raw::c_int,

    pub jack_type: ::std::os::raw::c_int,
    pub irq: ::std::os::raw::c_uint,
    pub irq_work_delay_time: ::std::os::raw::c_int,
    pub wclk_enabled: ::std::os::raw::c_int,
}

unsafe extern "C" {
    pub fn rt5682s_sel_asrc_clk_src(
        component: *mut snd_soc_component,
        filter_mask: ::std::os::raw::c_uint,
        clk_src: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}



// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
