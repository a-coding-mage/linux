/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rt5659_header.rs -- Rust translation of rt5659.h
 *
 * Original C header included <sound/rt5659.h>; external types from that
 * dependency are referenced by name below.
 */

use core::ffi::c_int;

/*
 * rt5659.h  --  RT5659/RT5658 ALSA SoC audio driver
 *
 * Copyright 2015 Realtek Microelectronics
 * Author: Bard Liao <bardliao@realtek.com>
 */



pub const DEVICE_ID: u32 = 0x6311;

/* Info */
pub const RT5659_RESET: u32 = 0x0000;
pub const RT5659_VENDOR_ID: u32 = 0x00fd;
pub const RT5659_VENDOR_ID_1: u32 = 0x00fe;
pub const RT5659_DEVICE_ID: u32 = 0x00ff;
/*  I/O - Output */
pub const RT5659_SPO_VOL: u32 = 0x0001;
pub const RT5659_HP_VOL: u32 = 0x0002;
pub const RT5659_LOUT: u32 = 0x0003;
pub const RT5659_MONO_OUT: u32 = 0x0004;
pub const RT5659_HPL_GAIN: u32 = 0x0005;
pub const RT5659_HPR_GAIN: u32 = 0x0006;
pub const RT5659_MONO_GAIN: u32 = 0x0007;
pub const RT5659_SPDIF_CTRL_1: u32 = 0x0008;
pub const RT5659_SPDIF_CTRL_2: u32 = 0x0009;
/* I/O - Input */
pub const RT5659_CAL_BST_CTRL: u32 = 0x000a;
pub const RT5659_IN1_IN2: u32 = 0x000c;
pub const RT5659_IN3_IN4: u32 = 0x000d;
pub const RT5659_INL1_INR1_VOL: u32 = 0x000f;
/* I/O - Speaker */
pub const RT5659_EJD_CTRL_1: u32 = 0x0010;
pub const RT5659_EJD_CTRL_2: u32 = 0x0011;
pub const RT5659_EJD_CTRL_3: u32 = 0x0012;
pub const RT5659_SILENCE_CTRL: u32 = 0x0015;
pub const RT5659_PSV_CTRL: u32 = 0x0016;
/* I/O - Sidetone */
pub const RT5659_SIDETONE_CTRL: u32 = 0x0018;
/* I/O - ADC/DAC/DMIC */
pub const RT5659_DAC1_DIG_VOL: u32 = 0x0019;
pub const RT5659_DAC2_DIG_VOL: u32 = 0x001a;
pub const RT5659_DAC_CTRL: u32 = 0x001b;
pub const RT5659_STO1_ADC_DIG_VOL: u32 = 0x001c;
pub const RT5659_MONO_ADC_DIG_VOL: u32 = 0x001d;
pub const RT5659_STO2_ADC_DIG_VOL: u32 = 0x001e;
pub const RT5659_STO1_BOOST: u32 = 0x001f;
pub const RT5659_MONO_BOOST: u32 = 0x0020;
pub const RT5659_STO2_BOOST: u32 = 0x0021;
pub const RT5659_HP_IMP_GAIN_1: u32 = 0x0022;
pub const RT5659_HP_IMP_GAIN_2: u32 = 0x0023;
/* Mixer - D-D */
pub const RT5659_STO1_ADC_MIXER: u32 = 0x0026;
pub const RT5659_MONO_ADC_MIXER: u32 = 0x0027;
pub const RT5659_AD_DA_MIXER: u32 = 0x0029;
pub const RT5659_STO_DAC_MIXER: u32 = 0x002a;
pub const RT5659_MONO_DAC_MIXER: u32 = 0x002b;
pub const RT5659_DIG_MIXER: u32 = 0x002c;
pub const RT5659_A_DAC_MUX: u32 = 0x002d;
pub const RT5659_DIG_INF23_DATA: u32 = 0x002f;
/* Mixer - PDM */
pub const RT5659_PDM_OUT_CTRL: u32 = 0x0031;
pub const RT5659_PDM_DATA_CTRL_1: u32 = 0x0032;
pub const RT5659_PDM_DATA_CTRL_2: u32 = 0x0033;
pub const RT5659_PDM_DATA_CTRL_3: u32 = 0x0034;
pub const RT5659_PDM_DATA_CTRL_4: u32 = 0x0035;
pub const RT5659_SPDIF_CTRL: u32 = 0x0036;

/* Mixer - ADC */
pub const RT5659_REC1_GAIN: u32 = 0x003a;
pub const RT5659_REC1_L1_MIXER: u32 = 0x003b;
pub const RT5659_REC1_L2_MIXER: u32 = 0x003c;
pub const RT5659_REC1_R1_MIXER: u32 = 0x003d;
pub const RT5659_REC1_R2_MIXER: u32 = 0x003e;
pub const RT5659_CAL_REC: u32 = 0x0040;
pub const RT5659_REC2_L1_MIXER: u32 = 0x009b;
pub const RT5659_REC2_L2_MIXER: u32 = 0x009c;
pub const RT5659_REC2_R1_MIXER: u32 = 0x009d;
pub const RT5659_REC2_R2_MIXER: u32 = 0x009e;
pub const RT5659_RC_CLK_CTRL: u32 = 0x009f;
/* Mixer - DAC */
pub const RT5659_SPK_L_MIXER: u32 = 0x0046;
pub const RT5659_SPK_R_MIXER: u32 = 0x0047;
pub const RT5659_SPO_AMP_GAIN: u32 = 0x0048;
pub const RT5659_ALC_BACK_GAIN: u32 = 0x0049;
pub const RT5659_MONOMIX_GAIN: u32 = 0x004a;
pub const RT5659_MONOMIX_IN_GAIN: u32 = 0x004b;
pub const RT5659_OUT_L_GAIN: u32 = 0x004d;
pub const RT5659_OUT_L_MIXER: u32 = 0x004e;
pub const RT5659_OUT_R_GAIN: u32 = 0x004f;
pub const RT5659_OUT_R_MIXER: u32 = 0x0050;
pub const RT5659_LOUT_MIXER: u32 = 0x0052;

pub const RT5659_HAPTIC_GEN_CTRL_1: u32 = 0x0053;
pub const RT5659_HAPTIC_GEN_CTRL_2: u32 = 0x0054;
pub const RT5659_HAPTIC_GEN_CTRL_3: u32 = 0x0055;
pub const RT5659_HAPTIC_GEN_CTRL_4: u32 = 0x0056;
pub const RT5659_HAPTIC_GEN_CTRL_5: u32 = 0x0057;
pub const RT5659_HAPTIC_GEN_CTRL_6: u32 = 0x0058;
pub const RT5659_HAPTIC_GEN_CTRL_7: u32 = 0x0059;
pub const RT5659_HAPTIC_GEN_CTRL_8: u32 = 0x005a;
pub const RT5659_HAPTIC_GEN_CTRL_9: u32 = 0x005b;
pub const RT5659_HAPTIC_GEN_CTRL_10: u32 = 0x005c;
pub const RT5659_HAPTIC_GEN_CTRL_11: u32 = 0x005d;
pub const RT5659_HAPTIC_LPF_CTRL_1: u32 = 0x005e;
pub const RT5659_HAPTIC_LPF_CTRL_2: u32 = 0x005f;
pub const RT5659_HAPTIC_LPF_CTRL_3: u32 = 0x0060;
/* Power */
pub const RT5659_PWR_DIG_1: u32 = 0x0061;
pub const RT5659_PWR_DIG_2: u32 = 0x0062;
pub const RT5659_PWR_ANLG_1: u32 = 0x0063;
pub const RT5659_PWR_ANLG_2: u32 = 0x0064;
pub const RT5659_PWR_ANLG_3: u32 = 0x0065;
pub const RT5659_PWR_MIXER: u32 = 0x0066;
pub const RT5659_PWR_VOL: u32 = 0x0067;
/* Private Register Control */
pub const RT5659_PRIV_INDEX: u32 = 0x006a;
pub const RT5659_CLK_DET: u32 = 0x006b;
pub const RT5659_PRIV_DATA: u32 = 0x006c;
/* System Clock Pre Divider Gating Control */
pub const RT5659_PRE_DIV_1: u32 = 0x006e;
pub const RT5659_PRE_DIV_2: u32 = 0x006f;
/* Format - ADC/DAC */
pub const RT5659_I2S1_SDP: u32 = 0x0070;
pub const RT5659_I2S2_SDP: u32 = 0x0071;
pub const RT5659_I2S3_SDP: u32 = 0x0072;
pub const RT5659_ADDA_CLK_1: u32 = 0x0073;
pub const RT5659_ADDA_CLK_2: u32 = 0x0074;
pub const RT5659_DMIC_CTRL_1: u32 = 0x0075;
pub const RT5659_DMIC_CTRL_2: u32 = 0x0076;
/* Format - TDM Control */
pub const RT5659_TDM_CTRL_1: u32 = 0x0077;
pub const RT5659_TDM_CTRL_2: u32 = 0x0078;
pub const RT5659_TDM_CTRL_3: u32 = 0x0079;
pub const RT5659_TDM_CTRL_4: u32 = 0x007a;
pub const RT5659_TDM_CTRL_5: u32 = 0x007b;

/* Function - Analog */
pub const RT5659_GLB_CLK: u32 = 0x0080;
pub const RT5659_PLL_CTRL_1: u32 = 0x0081;
pub const RT5659_PLL_CTRL_2: u32 = 0x0082;
pub const RT5659_ASRC_1: u32 = 0x0083;
pub const RT5659_ASRC_2: u32 = 0x0084;
pub const RT5659_ASRC_3: u32 = 0x0085;
pub const RT5659_ASRC_4: u32 = 0x0086;
pub const RT5659_ASRC_5: u32 = 0x0087;
pub const RT5659_ASRC_6: u32 = 0x0088;
pub const RT5659_ASRC_7: u32 = 0x0089;
pub const RT5659_ASRC_8: u32 = 0x008a;
pub const RT5659_ASRC_9: u32 = 0x008b;
pub const RT5659_ASRC_10: u32 = 0x008c;
pub const RT5659_DEPOP_1: u32 = 0x008e;
pub const RT5659_DEPOP_2: u32 = 0x008f;
pub const RT5659_DEPOP_3: u32 = 0x0090;
pub const RT5659_HP_CHARGE_PUMP_1: u32 = 0x0091;
pub const RT5659_HP_CHARGE_PUMP_2: u32 = 0x0092;
pub const RT5659_MICBIAS_1: u32 = 0x0093;
pub const RT5659_MICBIAS_2: u32 = 0x0094;
pub const RT5659_ASRC_11: u32 = 0x0097;
pub const RT5659_ASRC_12: u32 = 0x0098;
pub const RT5659_ASRC_13: u32 = 0x0099;
pub const RT5659_REC_M1_M2_GAIN_CTRL: u32 = 0x009a;
pub const RT5659_CLASSD_CTRL_1: u32 = 0x00a0;
pub const RT5659_CLASSD_CTRL_2: u32 = 0x00a1;

/* Function - Digital */
pub const RT5659_ADC_EQ_CTRL_1: u32 = 0x00ae;
pub const RT5659_ADC_EQ_CTRL_2: u32 = 0x00af;
pub const RT5659_DAC_EQ_CTRL_1: u32 = 0x00b0;
pub const RT5659_DAC_EQ_CTRL_2: u32 = 0x00b1;
pub const RT5659_DAC_EQ_CTRL_3: u32 = 0x00b2;

pub const RT5659_IRQ_CTRL_1: u32 = 0x00b6;
pub const RT5659_IRQ_CTRL_2: u32 = 0x00b7;
pub const RT5659_IRQ_CTRL_3: u32 = 0x00b8;
pub const RT5659_IRQ_CTRL_4: u32 = 0x00ba;
pub const RT5659_IRQ_CTRL_5: u32 = 0x00bb;
pub const RT5659_IRQ_CTRL_6: u32 = 0x00bc;
pub const RT5659_INT_ST_1: u32 = 0x00be;
pub const RT5659_INT_ST_2: u32 = 0x00bf;
pub const RT5659_GPIO_CTRL_1: u32 = 0x00c0;
pub const RT5659_GPIO_CTRL_2: u32 = 0x00c1;
pub const RT5659_GPIO_CTRL_3: u32 = 0x00c2;
pub const RT5659_GPIO_CTRL_4: u32 = 0x00c3;
pub const RT5659_GPIO_CTRL_5: u32 = 0x00c4;
pub const RT5659_GPIO_STA: u32 = 0x00c5;
pub const RT5659_SINE_GEN_CTRL_1: u32 = 0x00cb;
pub const RT5659_SINE_GEN_CTRL_2: u32 = 0x00cc;
pub const RT5659_SINE_GEN_CTRL_3: u32 = 0x00cd;
pub const RT5659_HP_AMP_DET_CTRL_1: u32 = 0x00d6;
pub const RT5659_HP_AMP_DET_CTRL_2: u32 = 0x00d7;
pub const RT5659_SV_ZCD_1: u32 = 0x00d9;
pub const RT5659_SV_ZCD_2: u32 = 0x00da;
pub const RT5659_IL_CMD_1: u32 = 0x00db;
pub const RT5659_IL_CMD_2: u32 = 0x00dc;
pub const RT5659_IL_CMD_3: u32 = 0x00dd;
pub const RT5659_IL_CMD_4: u32 = 0x00de;
pub const RT5659_4BTN_IL_CMD_1: u32 = 0x00df;
pub const RT5659_4BTN_IL_CMD_2: u32 = 0x00e0;
pub const RT5659_4BTN_IL_CMD_3: u32 = 0x00e1;
pub const RT5659_PSV_IL_CMD_1: u32 = 0x00e4;
pub const RT5659_PSV_IL_CMD_2: u32 = 0x00e5;

pub const RT5659_ADC_STO1_HP_CTRL_1: u32 = 0x00ea;
pub const RT5659_ADC_STO1_HP_CTRL_2: u32 = 0x00eb;
pub const RT5659_ADC_MONO_HP_CTRL_1: u32 = 0x00ec;
pub const RT5659_ADC_MONO_HP_CTRL_2: u32 = 0x00ed;
pub const RT5659_AJD1_CTRL: u32 = 0x00f0;
pub const RT5659_AJD2_AJD3_CTRL: u32 = 0x00f1;
pub const RT5659_JD1_THD: u32 = 0x00f2;
pub const RT5659_JD2_THD: u32 = 0x00f3;
pub const RT5659_JD3_THD: u32 = 0x00f4;
pub const RT5659_JD_CTRL_1: u32 = 0x00f6;
pub const RT5659_JD_CTRL_2: u32 = 0x00f7;
pub const RT5659_JD_CTRL_3: u32 = 0x00f8;
pub const RT5659_JD_CTRL_4: u32 = 0x00f9;
/* General Control */
pub const RT5659_DIG_MISC: u32 = 0x00fa;
pub const RT5659_DUMMY_2: u32 = 0x00fb;
pub const RT5659_DUMMY_3: u32 = 0x00fc;

pub const RT5659_DAC_ADC_DIG_VOL: u32 = 0x0100;
pub const RT5659_BIAS_CUR_CTRL_1: u32 = 0x010a;
pub const RT5659_BIAS_CUR_CTRL_2: u32 = 0x010b;
pub const RT5659_BIAS_CUR_CTRL_3: u32 = 0x010c;
pub const RT5659_BIAS_CUR_CTRL_4: u32 = 0x010d;
pub const RT5659_BIAS_CUR_CTRL_5: u32 = 0x010e;
pub const RT5659_BIAS_CUR_CTRL_6: u32 = 0x010f;
pub const RT5659_BIAS_CUR_CTRL_7: u32 = 0x0110;
pub const RT5659_BIAS_CUR_CTRL_8: u32 = 0x0111;
pub const RT5659_BIAS_CUR_CTRL_9: u32 = 0x0112;
pub const RT5659_BIAS_CUR_CTRL_10: u32 = 0x0113;
pub const RT5659_MEMORY_TEST: u32 = 0x0116;
pub const RT5659_VREF_REC_OP_FB_CAP_CTRL: u32 = 0x0117;
pub const RT5659_CLASSD_0: u32 = 0x011a;
pub const RT5659_CLASSD_1: u32 = 0x011b;
pub const RT5659_CLASSD_2: u32 = 0x011c;
pub const RT5659_CLASSD_3: u32 = 0x011d;
pub const RT5659_CLASSD_4: u32 = 0x011e;
pub const RT5659_CLASSD_5: u32 = 0x011f;
pub const RT5659_CLASSD_6: u32 = 0x0120;
pub const RT5659_CLASSD_7: u32 = 0x0121;
pub const RT5659_CLASSD_8: u32 = 0x0122;
pub const RT5659_CLASSD_9: u32 = 0x0123;
pub const RT5659_CLASSD_10: u32 = 0x0124;
pub const RT5659_CHARGE_PUMP_1: u32 = 0x0125;
pub const RT5659_CHARGE_PUMP_2: u32 = 0x0126;
pub const RT5659_DIG_IN_CTRL_1: u32 = 0x0132;
pub const RT5659_DIG_IN_CTRL_2: u32 = 0x0133;
pub const RT5659_PAD_DRIVING_CTRL: u32 = 0x0137;
pub const RT5659_SOFT_RAMP_DEPOP: u32 = 0x0138;
pub const RT5659_PLL: u32 = 0x0139;
pub const RT5659_CHOP_DAC: u32 = 0x013a;
pub const RT5659_CHOP_ADC: u32 = 0x013b;
pub const RT5659_CALIB_ADC_CTRL: u32 = 0x013c;
pub const RT5659_SOFT_RAMP_DEPOP_DAC_CLK_CTRL: u32 = 0x013e;
pub const RT5659_VOL_TEST: u32 = 0x013f;
pub const RT5659_TEST_MODE_CTRL_1: u32 = 0x0145;
pub const RT5659_TEST_MODE_CTRL_2: u32 = 0x0146;
pub const RT5659_TEST_MODE_CTRL_3: u32 = 0x0147;
pub const RT5659_TEST_MODE_CTRL_4: u32 = 0x0148;
pub const RT5659_BASSBACK_CTRL: u32 = 0x0150;
pub const RT5659_MP3_PLUS_CTRL_1: u32 = 0x0151;
pub const RT5659_MP3_PLUS_CTRL_2: u32 = 0x0152;
pub const RT5659_MP3_HPF_A1: u32 = 0x0153;
pub const RT5659_MP3_HPF_A2: u32 = 0x0154;
pub const RT5659_MP3_HPF_H0: u32 = 0x0155;
pub const RT5659_MP3_LPF_H0: u32 = 0x0156;
pub const RT5659_3D_SPK_CTRL: u32 = 0x0157;
pub const RT5659_3D_SPK_COEF_1: u32 = 0x0158;
pub const RT5659_3D_SPK_COEF_2: u32 = 0x0159;
pub const RT5659_3D_SPK_COEF_3: u32 = 0x015a;
pub const RT5659_3D_SPK_COEF_4: u32 = 0x015b;
pub const RT5659_3D_SPK_COEF_5: u32 = 0x015c;
pub const RT5659_3D_SPK_COEF_6: u32 = 0x015d;
pub const RT5659_3D_SPK_COEF_7: u32 = 0x015e;
pub const RT5659_STO_NG2_CTRL_1: u32 = 0x0160;
pub const RT5659_STO_NG2_CTRL_2: u32 = 0x0161;
pub const RT5659_STO_NG2_CTRL_3: u32 = 0x0162;
pub const RT5659_STO_NG2_CTRL_4: u32 = 0x0163;
pub const RT5659_STO_NG2_CTRL_5: u32 = 0x0164;
pub const RT5659_STO_NG2_CTRL_6: u32 = 0x0165;
pub const RT5659_STO_NG2_CTRL_7: u32 = 0x0166;
pub const RT5659_STO_NG2_CTRL_8: u32 = 0x0167;
pub const RT5659_MONO_NG2_CTRL_1: u32 = 0x0170;
pub const RT5659_MONO_NG2_CTRL_2: u32 = 0x0171;
pub const RT5659_MONO_NG2_CTRL_3: u32 = 0x0172;
pub const RT5659_MONO_NG2_CTRL_4: u32 = 0x0173;
pub const RT5659_MONO_NG2_CTRL_5: u32 = 0x0174;
pub const RT5659_MONO_NG2_CTRL_6: u32 = 0x0175;
pub const RT5659_MID_HP_AMP_DET: u32 = 0x0190;
pub const RT5659_LOW_HP_AMP_DET: u32 = 0x0191;
pub const RT5659_LDO_CTRL: u32 = 0x0192;
pub const RT5659_HP_DECROSS_CTRL_1: u32 = 0x01b0;
pub const RT5659_HP_DECROSS_CTRL_2: u32 = 0x01b1;
pub const RT5659_HP_DECROSS_CTRL_3: u32 = 0x01b2;
pub const RT5659_HP_DECROSS_CTRL_4: u32 = 0x01b3;
pub const RT5659_HP_IMP_SENS_CTRL_1: u32 = 0x01c0;
pub const RT5659_HP_IMP_SENS_CTRL_2: u32 = 0x01c1;
pub const RT5659_HP_IMP_SENS_CTRL_3: u32 = 0x01c2;
pub const RT5659_HP_IMP_SENS_CTRL_4: u32 = 0x01c3;
pub const RT5659_HP_IMP_SENS_MAP_1: u32 = 0x01c7;
pub const RT5659_HP_IMP_SENS_MAP_2: u32 = 0x01c8;
pub const RT5659_HP_IMP_SENS_MAP_3: u32 = 0x01c9;
pub const RT5659_HP_IMP_SENS_MAP_4: u32 = 0x01ca;
pub const RT5659_HP_IMP_SENS_MAP_5: u32 = 0x01cb;
pub const RT5659_HP_IMP_SENS_MAP_6: u32 = 0x01cc;
pub const RT5659_HP_IMP_SENS_MAP_7: u32 = 0x01cd;
pub const RT5659_HP_IMP_SENS_MAP_8: u32 = 0x01ce;
pub const RT5659_HP_LOGIC_CTRL_1: u32 = 0x01da;
pub const RT5659_HP_LOGIC_CTRL_2: u32 = 0x01db;
pub const RT5659_HP_CALIB_CTRL_1: u32 = 0x01de;
pub const RT5659_HP_CALIB_CTRL_2: u32 = 0x01df;
pub const RT5659_HP_CALIB_CTRL_3: u32 = 0x01e0;
pub const RT5659_HP_CALIB_CTRL_4: u32 = 0x01e1;
pub const RT5659_HP_CALIB_CTRL_5: u32 = 0x01e2;
pub const RT5659_HP_CALIB_CTRL_6: u32 = 0x01e3;
pub const RT5659_HP_CALIB_CTRL_7: u32 = 0x01e4;
pub const RT5659_HP_CALIB_CTRL_9: u32 = 0x01e6;
pub const RT5659_HP_CALIB_CTRL_10: u32 = 0x01e7;
pub const RT5659_HP_CALIB_CTRL_11: u32 = 0x01e8;
pub const RT5659_HP_CALIB_STA_1: u32 = 0x01ea;
pub const RT5659_HP_CALIB_STA_2: u32 = 0x01eb;
pub const RT5659_HP_CALIB_STA_3: u32 = 0x01ec;
pub const RT5659_HP_CALIB_STA_4: u32 = 0x01ed;
pub const RT5659_HP_CALIB_STA_5: u32 = 0x01ee;
pub const RT5659_HP_CALIB_STA_6: u32 = 0x01ef;
pub const RT5659_HP_CALIB_STA_7: u32 = 0x01f0;
pub const RT5659_HP_CALIB_STA_8: u32 = 0x01f1;
pub const RT5659_HP_CALIB_STA_9: u32 = 0x01f2;
pub const RT5659_MONO_AMP_CALIB_CTRL_1: u32 = 0x01f6;
pub const RT5659_MONO_AMP_CALIB_CTRL_2: u32 = 0x01f7;
pub const RT5659_MONO_AMP_CALIB_CTRL_3: u32 = 0x01f8;
pub const RT5659_MONO_AMP_CALIB_CTRL_4: u32 = 0x01f9;
pub const RT5659_MONO_AMP_CALIB_CTRL_5: u32 = 0x01fa;
pub const RT5659_MONO_AMP_CALIB_STA_1: u32 = 0x01fb;
pub const RT5659_MONO_AMP_CALIB_STA_2: u32 = 0x01fc;
pub const RT5659_MONO_AMP_CALIB_STA_3: u32 = 0x01fd;
pub const RT5659_MONO_AMP_CALIB_STA_4: u32 = 0x01fe;
pub const RT5659_SPK_PWR_LMT_CTRL_1: u32 = 0x0200;
pub const RT5659_SPK_PWR_LMT_CTRL_2: u32 = 0x0201;
pub const RT5659_SPK_PWR_LMT_CTRL_3: u32 = 0x0202;
pub const RT5659_SPK_PWR_LMT_STA_1: u32 = 0x0203;
pub const RT5659_SPK_PWR_LMT_STA_2: u32 = 0x0204;
pub const RT5659_SPK_PWR_LMT_STA_3: u32 = 0x0205;
pub const RT5659_SPK_PWR_LMT_STA_4: u32 = 0x0206;
pub const RT5659_SPK_PWR_LMT_STA_5: u32 = 0x0207;
pub const RT5659_SPK_PWR_LMT_STA_6: u32 = 0x0208;
pub const RT5659_FLEX_SPK_BST_CTRL_1: u32 = 0x0256;
pub const RT5659_FLEX_SPK_BST_CTRL_2: u32 = 0x0257;
pub const RT5659_FLEX_SPK_BST_CTRL_3: u32 = 0x0258;
pub const RT5659_FLEX_SPK_BST_CTRL_4: u32 = 0x0259;
pub const RT5659_SPK_EX_LMT_CTRL_1: u32 = 0x025a;
pub const RT5659_SPK_EX_LMT_CTRL_2: u32 = 0x025b;
pub const RT5659_SPK_EX_LMT_CTRL_3: u32 = 0x025c;
pub const RT5659_SPK_EX_LMT_CTRL_4: u32 = 0x025d;
pub const RT5659_SPK_EX_LMT_CTRL_5: u32 = 0x025e;
pub const RT5659_SPK_EX_LMT_CTRL_6: u32 = 0x025f;
pub const RT5659_SPK_EX_LMT_CTRL_7: u32 = 0x0260;
pub const RT5659_ADJ_HPF_CTRL_1: u32 = 0x0261;
pub const RT5659_ADJ_HPF_CTRL_2: u32 = 0x0262;
pub const RT5659_SPK_DC_CAILB_CTRL_1: u32 = 0x0265;
pub const RT5659_SPK_DC_CAILB_CTRL_2: u32 = 0x0266;
pub const RT5659_SPK_DC_CAILB_CTRL_3: u32 = 0x0267;
pub const RT5659_SPK_DC_CAILB_CTRL_4: u32 = 0x0268;
pub const RT5659_SPK_DC_CAILB_CTRL_5: u32 = 0x0269;
pub const RT5659_SPK_DC_CAILB_STA_1: u32 = 0x026a;
pub const RT5659_SPK_DC_CAILB_STA_2: u32 = 0x026b;
pub const RT5659_SPK_DC_CAILB_STA_3: u32 = 0x026c;
pub const RT5659_SPK_DC_CAILB_STA_4: u32 = 0x026d;
pub const RT5659_SPK_DC_CAILB_STA_5: u32 = 0x026e;
pub const RT5659_SPK_DC_CAILB_STA_6: u32 = 0x026f;
pub const RT5659_SPK_DC_CAILB_STA_7: u32 = 0x0270;
pub const RT5659_SPK_DC_CAILB_STA_8: u32 = 0x0271;
pub const RT5659_SPK_DC_CAILB_STA_9: u32 = 0x0272;
pub const RT5659_SPK_DC_CAILB_STA_10: u32 = 0x0273;
pub const RT5659_SPK_VDD_STA_1: u32 = 0x0280;
pub const RT5659_SPK_VDD_STA_2: u32 = 0x0281;
pub const RT5659_SPK_DC_DET_CTRL_1: u32 = 0x0282;
pub const RT5659_SPK_DC_DET_CTRL_2: u32 = 0x0283;
pub const RT5659_SPK_DC_DET_CTRL_3: u32 = 0x0284;
pub const RT5659_PURE_DC_DET_CTRL_1: u32 = 0x0290;
pub const RT5659_PURE_DC_DET_CTRL_2: u32 = 0x0291;
pub const RT5659_DUMMY_4: u32 = 0x02fa;
pub const RT5659_DUMMY_5: u32 = 0x02fb;
pub const RT5659_DUMMY_6: u32 = 0x02fc;
pub const RT5659_DRC1_CTRL_1: u32 = 0x0300;
pub const RT5659_DRC1_CTRL_2: u32 = 0x0301;
pub const RT5659_DRC1_CTRL_3: u32 = 0x0302;
pub const RT5659_DRC1_CTRL_4: u32 = 0x0303;
pub const RT5659_DRC1_CTRL_5: u32 = 0x0304;
pub const RT5659_DRC1_CTRL_6: u32 = 0x0305;
pub const RT5659_DRC1_HARD_LMT_CTRL_1: u32 = 0x0306;
pub const RT5659_DRC1_HARD_LMT_CTRL_2: u32 = 0x0307;
pub const RT5659_DRC2_CTRL_1: u32 = 0x0308;
pub const RT5659_DRC2_CTRL_2: u32 = 0x0309;
pub const RT5659_DRC2_CTRL_3: u32 = 0x030a;
pub const RT5659_DRC2_CTRL_4: u32 = 0x030b;
pub const RT5659_DRC2_CTRL_5: u32 = 0x030c;
pub const RT5659_DRC2_CTRL_6: u32 = 0x030d;
pub const RT5659_DRC2_HARD_LMT_CTRL_1: u32 = 0x030e;
pub const RT5659_DRC2_HARD_LMT_CTRL_2: u32 = 0x030f;
pub const RT5659_DRC1_PRIV_1: u32 = 0x0310;
pub const RT5659_DRC1_PRIV_2: u32 = 0x0311;
pub const RT5659_DRC1_PRIV_3: u32 = 0x0312;
pub const RT5659_DRC1_PRIV_4: u32 = 0x0313;
pub const RT5659_DRC1_PRIV_5: u32 = 0x0314;
pub const RT5659_DRC1_PRIV_6: u32 = 0x0315;
pub const RT5659_DRC1_PRIV_7: u32 = 0x0316;
pub const RT5659_DRC2_PRIV_1: u32 = 0x0317;
pub const RT5659_DRC2_PRIV_2: u32 = 0x0318;
pub const RT5659_DRC2_PRIV_3: u32 = 0x0319;
pub const RT5659_DRC2_PRIV_4: u32 = 0x031a;
pub const RT5659_DRC2_PRIV_5: u32 = 0x031b;
pub const RT5659_DRC2_PRIV_6: u32 = 0x031c;
pub const RT5659_DRC2_PRIV_7: u32 = 0x031d;
pub const RT5659_MULTI_DRC_CTRL: u32 = 0x0320;
pub const RT5659_CROSS_OVER_1: u32 = 0x0321;
pub const RT5659_CROSS_OVER_2: u32 = 0x0322;
pub const RT5659_CROSS_OVER_3: u32 = 0x0323;
pub const RT5659_CROSS_OVER_4: u32 = 0x0324;
pub const RT5659_CROSS_OVER_5: u32 = 0x0325;
pub const RT5659_CROSS_OVER_6: u32 = 0x0326;
pub const RT5659_CROSS_OVER_7: u32 = 0x0327;
pub const RT5659_CROSS_OVER_8: u32 = 0x0328;
pub const RT5659_CROSS_OVER_9: u32 = 0x0329;
pub const RT5659_CROSS_OVER_10: u32 = 0x032a;
pub const RT5659_ALC_PGA_CTRL_1: u32 = 0x0330;
pub const RT5659_ALC_PGA_CTRL_2: u32 = 0x0331;
pub const RT5659_ALC_PGA_CTRL_3: u32 = 0x0332;
pub const RT5659_ALC_PGA_CTRL_4: u32 = 0x0333;
pub const RT5659_ALC_PGA_CTRL_5: u32 = 0x0334;
pub const RT5659_ALC_PGA_CTRL_6: u32 = 0x0335;
pub const RT5659_ALC_PGA_CTRL_7: u32 = 0x0336;
pub const RT5659_ALC_PGA_CTRL_8: u32 = 0x0337;
pub const RT5659_ALC_PGA_STA_1: u32 = 0x0338;
pub const RT5659_ALC_PGA_STA_2: u32 = 0x0339;
pub const RT5659_ALC_PGA_STA_3: u32 = 0x033a;
pub const RT5659_DAC_L_EQ_PRE_VOL: u32 = 0x0340;
pub const RT5659_DAC_R_EQ_PRE_VOL: u32 = 0x0341;
pub const RT5659_DAC_L_EQ_POST_VOL: u32 = 0x0342;
pub const RT5659_DAC_R_EQ_POST_VOL: u32 = 0x0343;
pub const RT5659_DAC_L_EQ_LPF1_A1: u32 = 0x0344;
pub const RT5659_DAC_L_EQ_LPF1_H0: u32 = 0x0345;
pub const RT5659_DAC_R_EQ_LPF1_A1: u32 = 0x0346;
pub const RT5659_DAC_R_EQ_LPF1_H0: u32 = 0x0347;
pub const RT5659_DAC_L_EQ_BPF2_A1: u32 = 0x0348;
pub const RT5659_DAC_L_EQ_BPF2_A2: u32 = 0x0349;
pub const RT5659_DAC_L_EQ_BPF2_H0: u32 = 0x034a;
pub const RT5659_DAC_R_EQ_BPF2_A1: u32 = 0x034b;
pub const RT5659_DAC_R_EQ_BPF2_A2: u32 = 0x034c;
pub const RT5659_DAC_R_EQ_BPF2_H0: u32 = 0x034d;
pub const RT5659_DAC_L_EQ_BPF3_A1: u32 = 0x034e;
pub const RT5659_DAC_L_EQ_BPF3_A2: u32 = 0x034f;
pub const RT5659_DAC_L_EQ_BPF3_H0: u32 = 0x0350;
pub const RT5659_DAC_R_EQ_BPF3_A1: u32 = 0x0351;
pub const RT5659_DAC_R_EQ_BPF3_A2: u32 = 0x0352;
pub const RT5659_DAC_R_EQ_BPF3_H0: u32 = 0x0353;
pub const RT5659_DAC_L_EQ_BPF4_A1: u32 = 0x0354;
pub const RT5659_DAC_L_EQ_BPF4_A2: u32 = 0x0355;
pub const RT5659_DAC_L_EQ_BPF4_H0: u32 = 0x0356;
pub const RT5659_DAC_R_EQ_BPF4_A1: u32 = 0x0357;
pub const RT5659_DAC_R_EQ_BPF4_A2: u32 = 0x0358;
pub const RT5659_DAC_R_EQ_BPF4_H0: u32 = 0x0359;
pub const RT5659_DAC_L_EQ_HPF1_A1: u32 = 0x035a;
pub const RT5659_DAC_L_EQ_HPF1_H0: u32 = 0x035b;
pub const RT5659_DAC_R_EQ_HPF1_A1: u32 = 0x035c;
pub const RT5659_DAC_R_EQ_HPF1_H0: u32 = 0x035d;
pub const RT5659_DAC_L_EQ_HPF2_A1: u32 = 0x035e;
pub const RT5659_DAC_L_EQ_HPF2_A2: u32 = 0x035f;
pub const RT5659_DAC_L_EQ_HPF2_H0: u32 = 0x0360;
pub const RT5659_DAC_R_EQ_HPF2_A1: u32 = 0x0361;
pub const RT5659_DAC_R_EQ_HPF2_A2: u32 = 0x0362;
pub const RT5659_DAC_R_EQ_HPF2_H0: u32 = 0x0363;
pub const RT5659_DAC_L_BI_EQ_BPF1_H0_1: u32 = 0x0364;
pub const RT5659_DAC_L_BI_EQ_BPF1_H0_2: u32 = 0x0365;
pub const RT5659_DAC_L_BI_EQ_BPF1_B1_1: u32 = 0x0366;
pub const RT5659_DAC_L_BI_EQ_BPF1_B1_2: u32 = 0x0367;
pub const RT5659_DAC_L_BI_EQ_BPF1_B2_1: u32 = 0x0368;
pub const RT5659_DAC_L_BI_EQ_BPF1_B2_2: u32 = 0x0369;
pub const RT5659_DAC_L_BI_EQ_BPF1_A1_1: u32 = 0x036a;
pub const RT5659_DAC_L_BI_EQ_BPF1_A1_2: u32 = 0x036b;
pub const RT5659_DAC_L_BI_EQ_BPF1_A2_1: u32 = 0x036c;
pub const RT5659_DAC_L_BI_EQ_BPF1_A2_2: u32 = 0x036d;
pub const RT5659_DAC_R_BI_EQ_BPF1_H0_1: u32 = 0x036e;
pub const RT5659_DAC_R_BI_EQ_BPF1_H0_2: u32 = 0x036f;
pub const RT5659_DAC_R_BI_EQ_BPF1_B1_1: u32 = 0x0370;
pub const RT5659_DAC_R_BI_EQ_BPF1_B1_2: u32 = 0x0371;
pub const RT5659_DAC_R_BI_EQ_BPF1_B2_1: u32 = 0x0372;
pub const RT5659_DAC_R_BI_EQ_BPF1_B2_2: u32 = 0x0373;
pub const RT5659_DAC_R_BI_EQ_BPF1_A1_1: u32 = 0x0374;
pub const RT5659_DAC_R_BI_EQ_BPF1_A1_2: u32 = 0x0375;
pub const RT5659_DAC_R_BI_EQ_BPF1_A2_1: u32 = 0x0376;
pub const RT5659_DAC_R_BI_EQ_BPF1_A2_2: u32 = 0x0377;
pub const RT5659_ADC_L_EQ_LPF1_A1: u32 = 0x03d0;
pub const RT5659_ADC_R_EQ_LPF1_A1: u32 = 0x03d1;
pub const RT5659_ADC_L_EQ_LPF1_H0: u32 = 0x03d2;
pub const RT5659_ADC_R_EQ_LPF1_H0: u32 = 0x03d3;
pub const RT5659_ADC_L_EQ_BPF1_A1: u32 = 0x03d4;
pub const RT5659_ADC_R_EQ_BPF1_A1: u32 = 0x03d5;
pub const RT5659_ADC_L_EQ_BPF1_A2: u32 = 0x03d6;
pub const RT5659_ADC_R_EQ_BPF1_A2: u32 = 0x03d7;
pub const RT5659_ADC_L_EQ_BPF1_H0: u32 = 0x03d8;
pub const RT5659_ADC_R_EQ_BPF1_H0: u32 = 0x03d9;
pub const RT5659_ADC_L_EQ_BPF2_A1: u32 = 0x03da;
pub const RT5659_ADC_R_EQ_BPF2_A1: u32 = 0x03db;
pub const RT5659_ADC_L_EQ_BPF2_A2: u32 = 0x03dc;
pub const RT5659_ADC_R_EQ_BPF2_A2: u32 = 0x03dd;
pub const RT5659_ADC_L_EQ_BPF2_H0: u32 = 0x03de;
pub const RT5659_ADC_R_EQ_BPF2_H0: u32 = 0x03df;
pub const RT5659_ADC_L_EQ_BPF3_A1: u32 = 0x03e0;
pub const RT5659_ADC_R_EQ_BPF3_A1: u32 = 0x03e1;
pub const RT5659_ADC_L_EQ_BPF3_A2: u32 = 0x03e2;
pub const RT5659_ADC_R_EQ_BPF3_A2: u32 = 0x03e3;
pub const RT5659_ADC_L_EQ_BPF3_H0: u32 = 0x03e4;
pub const RT5659_ADC_R_EQ_BPF3_H0: u32 = 0x03e5;
pub const RT5659_ADC_L_EQ_BPF4_A1: u32 = 0x03e6;
pub const RT5659_ADC_R_EQ_BPF4_A1: u32 = 0x03e7;
pub const RT5659_ADC_L_EQ_BPF4_A2: u32 = 0x03e8;
pub const RT5659_ADC_R_EQ_BPF4_A2: u32 = 0x03e9;
pub const RT5659_ADC_L_EQ_BPF4_H0: u32 = 0x03ea;
pub const RT5659_ADC_R_EQ_BPF4_H0: u32 = 0x03eb;
pub const RT5659_ADC_L_EQ_HPF1_A1: u32 = 0x03ec;
pub const RT5659_ADC_R_EQ_HPF1_A1: u32 = 0x03ed;
pub const RT5659_ADC_L_EQ_HPF1_H0: u32 = 0x03ee;
pub const RT5659_ADC_R_EQ_HPF1_H0: u32 = 0x03ef;
pub const RT5659_ADC_L_EQ_PRE_VOL: u32 = 0x03f0;
pub const RT5659_ADC_R_EQ_PRE_VOL: u32 = 0x03f1;
pub const RT5659_ADC_L_EQ_POST_VOL: u32 = 0x03f2;
pub const RT5659_ADC_R_EQ_POST_VOL: u32 = 0x03f3;



/* global definition */
pub const RT5659_L_MUTE: u32 = (0x1 << 15);
pub const RT5659_L_MUTE_SFT: u32 = 15;
pub const RT5659_VOL_L_MUTE: u32 = (0x1 << 14);
pub const RT5659_VOL_L_SFT: u32 = 14;
pub const RT5659_R_MUTE: u32 = (0x1 << 7);
pub const RT5659_R_MUTE_SFT: u32 = 7;
pub const RT5659_VOL_R_MUTE: u32 = (0x1 << 6);
pub const RT5659_VOL_R_SFT: u32 = 6;
pub const RT5659_L_VOL_MASK: u32 = (0x3f << 8);
pub const RT5659_L_VOL_SFT: u32 = 8;
pub const RT5659_R_VOL_MASK: u32 = (0x3f);
pub const RT5659_R_VOL_SFT: u32 = 0;

/*Headphone Amp L/R Analog Gain and Digital NG2 Gain Control (0x0005 0x0006)*/
pub const RT5659_G_HP: u32 = (0x1f << 8);
pub const RT5659_G_HP_SFT: u32 = 8;
pub const RT5659_G_STO_DA_DMIX: u32 = (0x1f);
pub const RT5659_G_STO_DA_SFT: u32 = 0;

/* IN1/IN2 Control (0x000c) */
pub const RT5659_IN1_DF_MASK: u32 = (0x1 << 15);
pub const RT5659_IN1_DF: u32 = 15;
pub const RT5659_BST1_MASK: u32 = (0x7f << 8);
pub const RT5659_BST1_SFT: u32 = 8;
pub const RT5659_BST2_MASK: u32 = (0x7f);
pub const RT5659_BST2_SFT: u32 = 0;

/* IN3/IN4 Control (0x000d) */
pub const RT5659_IN3_DF_MASK: u32 = (0x1 << 15);
pub const RT5659_IN3_DF: u32 = 15;
pub const RT5659_BST3_MASK: u32 = (0x7f << 8);
pub const RT5659_BST3_SFT: u32 = 8;
pub const RT5659_IN4_DF_MASK: u32 = (0x1 << 7);
pub const RT5659_IN4_DF: u32 = 7;
pub const RT5659_BST4_MASK: u32 = (0x7f);
pub const RT5659_BST4_SFT: u32 = 0;

/* INL and INR Volume Control (0x000f) */
pub const RT5659_INL_VOL_MASK: u32 = (0x1f << 8);
pub const RT5659_INL_VOL_SFT: u32 = 8;
pub const RT5659_INR_VOL_MASK: u32 = (0x1f);
pub const RT5659_INR_VOL_SFT: u32 = 0;

/* Embeeded Jack and Type Detection Control 1 (0x0010) */
pub const RT5659_EMB_JD_EN: u32 = (0x1 << 15);
pub const RT5659_EMB_JD_EN_SFT: u32 = 15;
pub const RT5659_JD_MODE: u32 = (0x1 << 13);
pub const RT5659_JD_MODE_SFT: u32 = 13;
pub const RT5659_EXT_JD_EN: u32 = (0x1 << 11);
pub const RT5659_EXT_JD_EN_SFT: u32 = 11;
pub const RT5659_EXT_JD_DIG: u32 = (0x1 << 9);

/* Embeeded Jack and Type Detection Control 2 (0x0011) */
pub const RT5659_EXT_JD_SRC: u32 = (0x7 << 4);
pub const RT5659_EXT_JD_SRC_SFT: u32 = 4;
pub const RT5659_EXT_JD_SRC_GPIO_JD1: u32 = (0x0 << 4);
pub const RT5659_EXT_JD_SRC_GPIO_JD2: u32 = (0x1 << 4);
pub const RT5659_EXT_JD_SRC_JD1_1: u32 = (0x2 << 4);
pub const RT5659_EXT_JD_SRC_JD1_2: u32 = (0x3 << 4);
pub const RT5659_EXT_JD_SRC_JD2: u32 = (0x4 << 4);
pub const RT5659_EXT_JD_SRC_JD3: u32 = (0x5 << 4);
pub const RT5659_EXT_JD_SRC_MANUAL: u32 = (0x6 << 4);

/* Slience Detection Control (0x0015) */
pub const RT5659_SIL_DET_MASK: u32 = (0x1 << 15);
pub const RT5659_SIL_DET_DIS: u32 = (0x0 << 15);
pub const RT5659_SIL_DET_EN: u32 = (0x1 << 15);

/* Sidetone Control (0x0018) */
pub const RT5659_ST_SEL_MASK: u32 = (0x7 << 9);
pub const RT5659_ST_SEL_SFT: u32 = 9;
pub const RT5659_ST_EN: u32 = (0x1 << 6);
pub const RT5659_ST_EN_SFT: u32 = 6;

/* DAC1 Digital Volume (0x0019) */
pub const RT5659_DAC_L1_VOL_MASK: u32 = (0xff << 8);
pub const RT5659_DAC_L1_VOL_SFT: u32 = 8;
pub const RT5659_DAC_R1_VOL_MASK: u32 = (0xff);
pub const RT5659_DAC_R1_VOL_SFT: u32 = 0;

/* DAC2 Digital Volume (0x001a) */
pub const RT5659_DAC_L2_VOL_MASK: u32 = (0xff << 8);
pub const RT5659_DAC_L2_VOL_SFT: u32 = 8;
pub const RT5659_DAC_R2_VOL_MASK: u32 = (0xff);
pub const RT5659_DAC_R2_VOL_SFT: u32 = 0;

/* DAC2 Control (0x001b) */
pub const RT5659_M_DAC2_L_VOL: u32 = (0x1 << 13);
pub const RT5659_M_DAC2_L_VOL_SFT: u32 = 13;
pub const RT5659_M_DAC2_R_VOL: u32 = (0x1 << 12);
pub const RT5659_M_DAC2_R_VOL_SFT: u32 = 12;
pub const RT5659_DAC_L2_SEL_MASK: u32 = (0x7 << 4);
pub const RT5659_DAC_L2_SEL_SFT: u32 = 4;
pub const RT5659_DAC_R2_SEL_MASK: u32 = (0x7 << 0);
pub const RT5659_DAC_R2_SEL_SFT: u32 = 0;

/* ADC Digital Volume Control (0x001c) */
pub const RT5659_ADC_L_VOL_MASK: u32 = (0x7f << 8);
pub const RT5659_ADC_L_VOL_SFT: u32 = 8;
pub const RT5659_ADC_R_VOL_MASK: u32 = (0x7f);
pub const RT5659_ADC_R_VOL_SFT: u32 = 0;

/* Mono ADC Digital Volume Control (0x001d) */
pub const RT5659_MONO_ADC_L_VOL_MASK: u32 = (0x7f << 8);
pub const RT5659_MONO_ADC_L_VOL_SFT: u32 = 8;
pub const RT5659_MONO_ADC_R_VOL_MASK: u32 = (0x7f);
pub const RT5659_MONO_ADC_R_VOL_SFT: u32 = 0;

/* Stereo1 ADC Boost Gain Control (0x001f) */
pub const RT5659_STO1_ADC_L_BST_MASK: u32 = (0x3 << 14);
pub const RT5659_STO1_ADC_L_BST_SFT: u32 = 14;
pub const RT5659_STO1_ADC_R_BST_MASK: u32 = (0x3 << 12);
pub const RT5659_STO1_ADC_R_BST_SFT: u32 = 12;

/* Mono ADC Boost Gain Control (0x0020) */
pub const RT5659_MONO_ADC_L_BST_MASK: u32 = (0x3 << 14);
pub const RT5659_MONO_ADC_L_BST_SFT: u32 = 14;
pub const RT5659_MONO_ADC_R_BST_MASK: u32 = (0x3 << 12);
pub const RT5659_MONO_ADC_R_BST_SFT: u32 = 12;

/* Stereo1 ADC Boost Gain Control (0x001f) */
pub const RT5659_STO2_ADC_L_BST_MASK: u32 = (0x3 << 14);
pub const RT5659_STO2_ADC_L_BST_SFT: u32 = 14;
pub const RT5659_STO2_ADC_R_BST_MASK: u32 = (0x3 << 12);
pub const RT5659_STO2_ADC_R_BST_SFT: u32 = 12;

/* Stereo ADC Mixer Control (0x0026) */
pub const RT5659_M_STO1_ADC_L1: u32 = (0x1 << 15);
pub const RT5659_M_STO1_ADC_L1_SFT: u32 = 15;
pub const RT5659_M_STO1_ADC_L2: u32 = (0x1 << 14);
pub const RT5659_M_STO1_ADC_L2_SFT: u32 = 14;
pub const RT5659_STO1_ADC1_SRC_MASK: u32 = (0x1 << 13);
pub const RT5659_STO1_ADC1_SRC_SFT: u32 = 13;
pub const RT5659_STO1_ADC1_SRC_ADC: u32 = (0x1 << 13);
pub const RT5659_STO1_ADC1_SRC_DACMIX: u32 = (0x0 << 13);
pub const RT5659_STO1_ADC_SRC_MASK: u32 = (0x1 << 12);
pub const RT5659_STO1_ADC_SRC_SFT: u32 = 12;
pub const RT5659_STO1_ADC_SRC_ADC1: u32 = (0x1 << 12);
pub const RT5659_STO1_ADC_SRC_ADC2: u32 = (0x0 << 12);
pub const RT5659_STO1_ADC2_SRC_MASK: u32 = (0x1 << 11);
pub const RT5659_STO1_ADC2_SRC_SFT: u32 = 11;
pub const RT5659_STO1_DMIC_SRC_MASK: u32 = (0x1 << 8);
pub const RT5659_STO1_DMIC_SRC_SFT: u32 = 8;
pub const RT5659_STO1_DMIC_SRC_DMIC2: u32 = (0x1 << 8);
pub const RT5659_STO1_DMIC_SRC_DMIC1: u32 = (0x0 << 8);
pub const RT5659_M_STO1_ADC_R1: u32 = (0x1 << 6);
pub const RT5659_M_STO1_ADC_R1_SFT: u32 = 6;
pub const RT5659_M_STO1_ADC_R2: u32 = (0x1 << 5);
pub const RT5659_M_STO1_ADC_R2_SFT: u32 = 5;

/* Mono1 ADC Mixer control (0x0027) */
pub const RT5659_M_MONO_ADC_L1: u32 = (0x1 << 15);
pub const RT5659_M_MONO_ADC_L1_SFT: u32 = 15;
pub const RT5659_M_MONO_ADC_L2: u32 = (0x1 << 14);
pub const RT5659_M_MONO_ADC_L2_SFT: u32 = 14;
pub const RT5659_MONO_ADC_L2_SRC_MASK: u32 = (0x1 << 12);
pub const RT5659_MONO_ADC_L2_SRC_SFT: u32 = 12;
pub const RT5659_MONO_ADC_L1_SRC_MASK: u32 = (0x1 << 11);
pub const RT5659_MONO_ADC_L1_SRC_SFT: u32 = 11;
pub const RT5659_MONO_ADC_L_SRC_MASK: u32 = (0x3 << 9);
pub const RT5659_MONO_ADC_L_SRC_SFT: u32 = 9;
pub const RT5659_MONO_DMIC_L_SRC_MASK: u32 = (0x1 << 8);
pub const RT5659_MONO_DMIC_L_SRC_SFT: u32 = 8;
pub const RT5659_M_MONO_ADC_R1: u32 = (0x1 << 7);
pub const RT5659_M_MONO_ADC_R1_SFT: u32 = 7;
pub const RT5659_M_MONO_ADC_R2: u32 = (0x1 << 6);
pub const RT5659_M_MONO_ADC_R2_SFT: u32 = 6;
pub const RT5659_STO2_ADC_SRC_MASK: u32 = (0x1 << 5);
pub const RT5659_STO2_ADC_SRC_SFT: u32 = 5;
pub const RT5659_MONO_ADC_R2_SRC_MASK: u32 = (0x1 << 4);
pub const RT5659_MONO_ADC_R2_SRC_SFT: u32 = 4;
pub const RT5659_MONO_ADC_R1_SRC_MASK: u32 = (0x1 << 3);
pub const RT5659_MONO_ADC_R1_SRC_SFT: u32 = 3;
pub const RT5659_MONO_ADC_R_SRC_MASK: u32 = (0x3 << 1);
pub const RT5659_MONO_ADC_R_SRC_SFT: u32 = 1;
pub const RT5659_MONO_DMIC_R_SRC_MASK: u32 = 0x1;
pub const RT5659_MONO_DMIC_R_SRC_SFT: u32 = 0;

/* ADC Mixer to DAC Mixer Control (0x0029) */
pub const RT5659_M_ADCMIX_L: u32 = (0x1 << 15);
pub const RT5659_M_ADCMIX_L_SFT: u32 = 15;
pub const RT5659_M_DAC1_L: u32 = (0x1 << 14);
pub const RT5659_M_DAC1_L_SFT: u32 = 14;
pub const RT5659_DAC1_R_SEL_MASK: u32 = (0x3 << 10);
pub const RT5659_DAC1_R_SEL_SFT: u32 = 10;
pub const RT5659_DAC1_R_SEL_IF1: u32 = (0x0 << 10);
pub const RT5659_DAC1_R_SEL_IF2: u32 = (0x1 << 10);
pub const RT5659_DAC1_R_SEL_IF3: u32 = (0x2 << 10);
pub const RT5659_DAC1_L_SEL_MASK: u32 = (0x3 << 8);
pub const RT5659_DAC1_L_SEL_SFT: u32 = 8;
pub const RT5659_DAC1_L_SEL_IF1: u32 = (0x0 << 8);
pub const RT5659_DAC1_L_SEL_IF2: u32 = (0x1 << 8);
pub const RT5659_DAC1_L_SEL_IF3: u32 = (0x2 << 8);
pub const RT5659_M_ADCMIX_R: u32 = (0x1 << 7);
pub const RT5659_M_ADCMIX_R_SFT: u32 = 7;
pub const RT5659_M_DAC1_R: u32 = (0x1 << 6);
pub const RT5659_M_DAC1_R_SFT: u32 = 6;

/* Stereo DAC Mixer Control (0x002a) */
pub const RT5659_M_DAC_L1_STO_L: u32 = (0x1 << 15);
pub const RT5659_M_DAC_L1_STO_L_SFT: u32 = 15;
pub const RT5659_G_DAC_L1_STO_L_MASK: u32 = (0x1 << 14);
pub const RT5659_G_DAC_L1_STO_L_SFT: u32 = 14;
pub const RT5659_M_DAC_R1_STO_L: u32 = (0x1 << 13);
pub const RT5659_M_DAC_R1_STO_L_SFT: u32 = 13;
pub const RT5659_G_DAC_R1_STO_L_MASK: u32 = (0x1 << 12);
pub const RT5659_G_DAC_R1_STO_L_SFT: u32 = 12;
pub const RT5659_M_DAC_L2_STO_L: u32 = (0x1 << 11);
pub const RT5659_M_DAC_L2_STO_L_SFT: u32 = 11;
pub const RT5659_G_DAC_L2_STO_L_MASK: u32 = (0x1 << 10);
pub const RT5659_G_DAC_L2_STO_L_SFT: u32 = 10;
pub const RT5659_M_DAC_R2_STO_L: u32 = (0x1 << 9);
pub const RT5659_M_DAC_R2_STO_L_SFT: u32 = 9;
pub const RT5659_G_DAC_R2_STO_L_MASK: u32 = (0x1 << 8);
pub const RT5659_G_DAC_R2_STO_L_SFT: u32 = 8;
pub const RT5659_M_DAC_L1_STO_R: u32 = (0x1 << 7);
pub const RT5659_M_DAC_L1_STO_R_SFT: u32 = 7;
pub const RT5659_G_DAC_L1_STO_R_MASK: u32 = (0x1 << 6);
pub const RT5659_G_DAC_L1_STO_R_SFT: u32 = 6;
pub const RT5659_M_DAC_R1_STO_R: u32 = (0x1 << 5);
pub const RT5659_M_DAC_R1_STO_R_SFT: u32 = 5;
pub const RT5659_G_DAC_R1_STO_R_MASK: u32 = (0x1 << 4);
pub const RT5659_G_DAC_R1_STO_R_SFT: u32 = 4;
pub const RT5659_M_DAC_L2_STO_R: u32 = (0x1 << 3);
pub const RT5659_M_DAC_L2_STO_R_SFT: u32 = 3;
pub const RT5659_G_DAC_L2_STO_R_MASK: u32 = (0x1 << 2);
pub const RT5659_G_DAC_L2_STO_R_SFT: u32 = 2;
pub const RT5659_M_DAC_R2_STO_R: u32 = (0x1 << 1);
pub const RT5659_M_DAC_R2_STO_R_SFT: u32 = 1;
pub const RT5659_G_DAC_R2_STO_R_MASK: u32 = (0x1);
pub const RT5659_G_DAC_R2_STO_R_SFT: u32 = 0;

/* Mono DAC Mixer Control (0x002b) */
pub const RT5659_M_DAC_L1_MONO_L: u32 = (0x1 << 15);
pub const RT5659_M_DAC_L1_MONO_L_SFT: u32 = 15;
pub const RT5659_G_DAC_L1_MONO_L_MASK: u32 = (0x1 << 14);
pub const RT5659_G_DAC_L1_MONO_L_SFT: u32 = 14;
pub const RT5659_M_DAC_R1_MONO_L: u32 = (0x1 << 13);
pub const RT5659_M_DAC_R1_MONO_L_SFT: u32 = 13;
pub const RT5659_G_DAC_R1_MONO_L_MASK: u32 = (0x1 << 12);
pub const RT5659_G_DAC_R1_MONO_L_SFT: u32 = 12;
pub const RT5659_M_DAC_L2_MONO_L: u32 = (0x1 << 11);
pub const RT5659_M_DAC_L2_MONO_L_SFT: u32 = 11;
pub const RT5659_G_DAC_L2_MONO_L_MASK: u32 = (0x1 << 10);
pub const RT5659_G_DAC_L2_MONO_L_SFT: u32 = 10;
pub const RT5659_M_DAC_R2_MONO_L: u32 = (0x1 << 9);
pub const RT5659_M_DAC_R2_MONO_L_SFT: u32 = 9;
pub const RT5659_G_DAC_R2_MONO_L_MASK: u32 = (0x1 << 8);
pub const RT5659_G_DAC_R2_MONO_L_SFT: u32 = 8;
pub const RT5659_M_DAC_L1_MONO_R: u32 = (0x1 << 7);
pub const RT5659_M_DAC_L1_MONO_R_SFT: u32 = 7;
pub const RT5659_G_DAC_L1_MONO_R_MASK: u32 = (0x1 << 6);
pub const RT5659_G_DAC_L1_MONO_R_SFT: u32 = 6;
pub const RT5659_M_DAC_R1_MONO_R: u32 = (0x1 << 5);
pub const RT5659_M_DAC_R1_MONO_R_SFT: u32 = 5;
pub const RT5659_G_DAC_R1_MONO_R_MASK: u32 = (0x1 << 4);
pub const RT5659_G_DAC_R1_MONO_R_SFT: u32 = 4;
pub const RT5659_M_DAC_L2_MONO_R: u32 = (0x1 << 3);
pub const RT5659_M_DAC_L2_MONO_R_SFT: u32 = 3;
pub const RT5659_G_DAC_L2_MONO_R_MASK: u32 = (0x1 << 2);
pub const RT5659_G_DAC_L2_MONO_R_SFT: u32 = 2;
pub const RT5659_M_DAC_R2_MONO_R: u32 = (0x1 << 1);
pub const RT5659_M_DAC_R2_MONO_R_SFT: u32 = 1;
pub const RT5659_G_DAC_R2_MONO_R_MASK: u32 = (0x1);
pub const RT5659_G_DAC_R2_MONO_R_SFT: u32 = 0;

/* Digital Mixer Control (0x002c) */
pub const RT5659_M_DAC_MIX_L: u32 = (0x1 << 7);
pub const RT5659_M_DAC_MIX_L_SFT: u32 = 7;
pub const RT5659_DAC_MIX_L_MASK: u32 = (0x1 << 6);
pub const RT5659_DAC_MIX_L_SFT: u32 = 6;
pub const RT5659_M_DAC_MIX_R: u32 = (0x1 << 5);
pub const RT5659_M_DAC_MIX_R_SFT: u32 = 5;
pub const RT5659_DAC_MIX_R_MASK: u32 = (0x1 << 4);
pub const RT5659_DAC_MIX_R_SFT: u32 = 4;

/* Analog DAC Input Source Control (0x002d) */
pub const RT5659_A_DACL1_SEL: u32 = (0x1 << 3);
pub const RT5659_A_DACL1_SFT: u32 = 3;
pub const RT5659_A_DACR1_SEL: u32 = (0x1 << 2);
pub const RT5659_A_DACR1_SFT: u32 = 2;
pub const RT5659_A_DACL2_SEL: u32 = (0x1 << 1);
pub const RT5659_A_DACL2_SFT: u32 = 1;
pub const RT5659_A_DACR2_SEL: u32 = (0x1 << 0);
pub const RT5659_A_DACR2_SFT: u32 = 0;

/* Digital Interface Data Control (0x002f) */
pub const RT5659_IF2_ADC3_IN_MASK: u32 = (0x3 << 14);
pub const RT5659_IF2_ADC3_IN_SFT: u32 = 14;
pub const RT5659_IF2_ADC_IN_MASK: u32 = (0x3 << 12);
pub const RT5659_IF2_ADC_IN_SFT: u32 = 12;
pub const RT5659_IF2_DAC_SEL_MASK: u32 = (0x3 << 10);
pub const RT5659_IF2_DAC_SEL_SFT: u32 = 10;
pub const RT5659_IF2_ADC_SEL_MASK: u32 = (0x3 << 8);
pub const RT5659_IF2_ADC_SEL_SFT: u32 = 8;
pub const RT5659_IF3_DAC_SEL_MASK: u32 = (0x3 << 6);
pub const RT5659_IF3_DAC_SEL_SFT: u32 = 6;
pub const RT5659_IF3_ADC_SEL_MASK: u32 = (0x3 << 4);
pub const RT5659_IF3_ADC_SEL_SFT: u32 = 4;
pub const RT5659_IF3_ADC_IN_MASK: u32 = (0x3 << 0);
pub const RT5659_IF3_ADC_IN_SFT: u32 = 0;

/* PDM Output Control (0x0031) */
pub const RT5659_PDM1_L_MASK: u32 = (0x1 << 15);
pub const RT5659_PDM1_L_SFT: u32 = 15;
pub const RT5659_M_PDM1_L: u32 = (0x1 << 14);
pub const RT5659_M_PDM1_L_SFT: u32 = 14;
pub const RT5659_PDM1_R_MASK: u32 = (0x1 << 13);
pub const RT5659_PDM1_R_SFT: u32 = 13;
pub const RT5659_M_PDM1_R: u32 = (0x1 << 12);
pub const RT5659_M_PDM1_R_SFT: u32 = 12;
pub const RT5659_PDM2_BUSY: u32 = (0x1 << 7);
pub const RT5659_PDM1_BUSY: u32 = (0x1 << 6);
pub const RT5659_PDM_PATTERN: u32 = (0x1 << 5);
pub const RT5659_PDM_GAIN: u32 = (0x1 << 4);
pub const RT5659_PDM_DIV_MASK: u32 = (0x3);

/*S/PDIF Output Control (0x0036) */
pub const RT5659_SPDIF_SEL_MASK: u32 = (0x3 << 0);
pub const RT5659_SPDIF_SEL_SFT: u32 = 0;

/* REC Left Mixer Control 2 (0x003c) */
pub const RT5659_M_BST1_RM1_L: u32 = (0x1 << 5);
pub const RT5659_M_BST1_RM1_L_SFT: u32 = 5;
pub const RT5659_M_BST2_RM1_L: u32 = (0x1 << 4);
pub const RT5659_M_BST2_RM1_L_SFT: u32 = 4;
pub const RT5659_M_BST3_RM1_L: u32 = (0x1 << 3);
pub const RT5659_M_BST3_RM1_L_SFT: u32 = 3;
pub const RT5659_M_BST4_RM1_L: u32 = (0x1 << 2);
pub const RT5659_M_BST4_RM1_L_SFT: u32 = 2;
pub const RT5659_M_INL_RM1_L: u32 = (0x1 << 1);
pub const RT5659_M_INL_RM1_L_SFT: u32 = 1;
pub const RT5659_M_SPKVOLL_RM1_L: u32 = (0x1);
pub const RT5659_M_SPKVOLL_RM1_L_SFT: u32 = 0;

/* REC Right Mixer Control 2 (0x003e) */
pub const RT5659_M_BST1_RM1_R: u32 = (0x1 << 5);
pub const RT5659_M_BST1_RM1_R_SFT: u32 = 5;
pub const RT5659_M_BST2_RM1_R: u32 = (0x1 << 4);
pub const RT5659_M_BST2_RM1_R_SFT: u32 = 4;
pub const RT5659_M_BST3_RM1_R: u32 = (0x1 << 3);
pub const RT5659_M_BST3_RM1_R_SFT: u32 = 3;
pub const RT5659_M_BST4_RM1_R: u32 = (0x1 << 2);
pub const RT5659_M_BST4_RM1_R_SFT: u32 = 2;
pub const RT5659_M_INR_RM1_R: u32 = (0x1 << 1);
pub const RT5659_M_INR_RM1_R_SFT: u32 = 1;
pub const RT5659_M_HPOVOLR_RM1_R: u32 = (0x1);
pub const RT5659_M_HPOVOLR_RM1_R_SFT: u32 = 0;

/* SPK Left Mixer Control (0x0046) */
pub const RT5659_M_BST3_SM_L: u32 = (0x1 << 4);
pub const RT5659_M_BST3_SM_L_SFT: u32 = 4;
pub const RT5659_M_IN_R_SM_L: u32 = (0x1 << 3);
pub const RT5659_M_IN_R_SM_L_SFT: u32 = 3;
pub const RT5659_M_IN_L_SM_L: u32 = (0x1 << 2);
pub const RT5659_M_IN_L_SM_L_SFT: u32 = 2;
pub const RT5659_M_BST1_SM_L: u32 = (0x1 << 1);
pub const RT5659_M_BST1_SM_L_SFT: u32 = 1;
pub const RT5659_M_DAC_L2_SM_L: u32 = (0x1);
pub const RT5659_M_DAC_L2_SM_L_SFT: u32 = 0;

/* SPK Right Mixer Control (0x0047) */
pub const RT5659_M_BST3_SM_R: u32 = (0x1 << 4);
pub const RT5659_M_BST3_SM_R_SFT: u32 = 4;
pub const RT5659_M_IN_R_SM_R: u32 = (0x1 << 3);
pub const RT5659_M_IN_R_SM_R_SFT: u32 = 3;
pub const RT5659_M_IN_L_SM_R: u32 = (0x1 << 2);
pub const RT5659_M_IN_L_SM_R_SFT: u32 = 2;
pub const RT5659_M_BST4_SM_R: u32 = (0x1 << 1);
pub const RT5659_M_BST4_SM_R_SFT: u32 = 1;
pub const RT5659_M_DAC_R2_SM_R: u32 = (0x1);
pub const RT5659_M_DAC_R2_SM_R_SFT: u32 = 0;

/* SPO Amp Input and Gain Control (0x0048) */
pub const RT5659_M_DAC_L2_SPKOMIX: u32 = (0x1 << 13);
pub const RT5659_M_DAC_L2_SPKOMIX_SFT: u32 = 13;
pub const RT5659_M_SPKVOLL_SPKOMIX: u32 = (0x1 << 12);
pub const RT5659_M_SPKVOLL_SPKOMIX_SFT: u32 = 12;
pub const RT5659_M_DAC_R2_SPKOMIX: u32 = (0x1 << 9);
pub const RT5659_M_DAC_R2_SPKOMIX_SFT: u32 = 9;
pub const RT5659_M_SPKVOLR_SPKOMIX: u32 = (0x1 << 8);
pub const RT5659_M_SPKVOLR_SPKOMIX_SFT: u32 = 8;

/* MONOMIX Input and Gain Control (0x004b) */
pub const RT5659_M_MONOVOL_MA: u32 = (0x1 << 9);
pub const RT5659_M_MONOVOL_MA_SFT: u32 = 9;
pub const RT5659_M_DAC_L2_MA: u32 = (0x1 << 8);
pub const RT5659_M_DAC_L2_MA_SFT: u32 = 8;
pub const RT5659_M_BST3_MM: u32 = (0x1 << 4);
pub const RT5659_M_BST3_MM_SFT: u32 = 4;
pub const RT5659_M_BST2_MM: u32 = (0x1 << 3);
pub const RT5659_M_BST2_MM_SFT: u32 = 3;
pub const RT5659_M_BST1_MM: u32 = (0x1 << 2);
pub const RT5659_M_BST1_MM_SFT: u32 = 2;
pub const RT5659_M_DAC_R2_MM: u32 = (0x1 << 1);
pub const RT5659_M_DAC_R2_MM_SFT: u32 = 1;
pub const RT5659_M_DAC_L2_MM: u32 = (0x1);
pub const RT5659_M_DAC_L2_MM_SFT: u32 = 0;

/* Output Left Mixer Control 1 (0x004d) */
pub const RT5659_G_BST3_OM_L_MASK: u32 = (0x7 << 12);
pub const RT5659_G_BST3_OM_L_SFT: u32 = 12;
pub const RT5659_G_BST2_OM_L_MASK: u32 = (0x7 << 9);
pub const RT5659_G_BST2_OM_L_SFT: u32 = 9;
pub const RT5659_G_BST1_OM_L_MASK: u32 = (0x7 << 6);
pub const RT5659_G_BST1_OM_L_SFT: u32 = 6;
pub const RT5659_G_IN_L_OM_L_MASK: u32 = (0x7 << 3);
pub const RT5659_G_IN_L_OM_L_SFT: u32 = 3;
pub const RT5659_G_DAC_L2_OM_L_MASK: u32 = (0x7 << 0);
pub const RT5659_G_DAC_L2_OM_L_SFT: u32 = 0;

/* Output Left Mixer Input Control (0x004e) */
pub const RT5659_M_BST3_OM_L: u32 = (0x1 << 4);
pub const RT5659_M_BST3_OM_L_SFT: u32 = 4;
pub const RT5659_M_BST2_OM_L: u32 = (0x1 << 3);
pub const RT5659_M_BST2_OM_L_SFT: u32 = 3;
pub const RT5659_M_BST1_OM_L: u32 = (0x1 << 2);
pub const RT5659_M_BST1_OM_L_SFT: u32 = 2;
pub const RT5659_M_IN_L_OM_L: u32 = (0x1 << 1);
pub const RT5659_M_IN_L_OM_L_SFT: u32 = 1;
pub const RT5659_M_DAC_L2_OM_L: u32 = (0x1);
pub const RT5659_M_DAC_L2_OM_L_SFT: u32 = 0;

/* Output Right Mixer Input Control (0x0050) */
pub const RT5659_M_BST4_OM_R: u32 = (0x1 << 4);
pub const RT5659_M_BST4_OM_R_SFT: u32 = 4;
pub const RT5659_M_BST3_OM_R: u32 = (0x1 << 3);
pub const RT5659_M_BST3_OM_R_SFT: u32 = 3;
pub const RT5659_M_BST2_OM_R: u32 = (0x1 << 2);
pub const RT5659_M_BST2_OM_R_SFT: u32 = 2;
pub const RT5659_M_IN_R_OM_R: u32 = (0x1 << 1);
pub const RT5659_M_IN_R_OM_R_SFT: u32 = 1;
pub const RT5659_M_DAC_R2_OM_R: u32 = (0x1);
pub const RT5659_M_DAC_R2_OM_R_SFT: u32 = 0;

/* LOUT Mixer Control (0x0052) */
pub const RT5659_M_DAC_L2_LM: u32 = (0x1 << 15);
pub const RT5659_M_DAC_L2_LM_SFT: u32 = 15;
pub const RT5659_M_DAC_R2_LM: u32 = (0x1 << 14);
pub const RT5659_M_DAC_R2_LM_SFT: u32 = 14;
pub const RT5659_M_OV_L_LM: u32 = (0x1 << 13);
pub const RT5659_M_OV_L_LM_SFT: u32 = 13;
pub const RT5659_M_OV_R_LM: u32 = (0x1 << 12);
pub const RT5659_M_OV_R_LM_SFT: u32 = 12;

/* Power Management for Digital 1 (0x0061) */
pub const RT5659_PWR_I2S1: u32 = (0x1 << 15);
pub const RT5659_PWR_I2S1_BIT: u32 = 15;
pub const RT5659_PWR_I2S2: u32 = (0x1 << 14);
pub const RT5659_PWR_I2S2_BIT: u32 = 14;
pub const RT5659_PWR_I2S3: u32 = (0x1 << 13);
pub const RT5659_PWR_I2S3_BIT: u32 = 13;
pub const RT5659_PWR_SPDIF: u32 = (0x1 << 12);
pub const RT5659_PWR_SPDIF_BIT: u32 = 12;
pub const RT5659_PWR_DAC_L1: u32 = (0x1 << 11);
pub const RT5659_PWR_DAC_L1_BIT: u32 = 11;
pub const RT5659_PWR_DAC_R1: u32 = (0x1 << 10);
pub const RT5659_PWR_DAC_R1_BIT: u32 = 10;
pub const RT5659_PWR_DAC_L2: u32 = (0x1 << 9);
pub const RT5659_PWR_DAC_L2_BIT: u32 = 9;
pub const RT5659_PWR_DAC_R2: u32 = (0x1 << 8);
pub const RT5659_PWR_DAC_R2_BIT: u32 = 8;
pub const RT5659_PWR_LDO: u32 = (0x1 << 7);
pub const RT5659_PWR_LDO_BIT: u32 = 7;
pub const RT5659_PWR_ADC_L1: u32 = (0x1 << 4);
pub const RT5659_PWR_ADC_L1_BIT: u32 = 4;
pub const RT5659_PWR_ADC_R1: u32 = (0x1 << 3);
pub const RT5659_PWR_ADC_R1_BIT: u32 = 3;
pub const RT5659_PWR_ADC_L2: u32 = (0x1 << 2);
pub const RT5659_PWR_ADC_L2_BIT: u32 = 2;
pub const RT5659_PWR_ADC_R2: u32 = (0x1 << 1);
pub const RT5659_PWR_ADC_R2_BIT: u32 = 1;
pub const RT5659_PWR_CLS_D: u32 = (0x1);
pub const RT5659_PWR_CLS_D_BIT: u32 = 0;

/* Power Management for Digital 2 (0x0062) */
pub const RT5659_PWR_ADC_S1F: u32 = (0x1 << 15);
pub const RT5659_PWR_ADC_S1F_BIT: u32 = 15;
pub const RT5659_PWR_ADC_S2F: u32 = (0x1 << 14);
pub const RT5659_PWR_ADC_S2F_BIT: u32 = 14;
pub const RT5659_PWR_ADC_MF_L: u32 = (0x1 << 13);
pub const RT5659_PWR_ADC_MF_L_BIT: u32 = 13;
pub const RT5659_PWR_ADC_MF_R: u32 = (0x1 << 12);
pub const RT5659_PWR_ADC_MF_R_BIT: u32 = 12;
pub const RT5659_PWR_DAC_S1F: u32 = (0x1 << 10);
pub const RT5659_PWR_DAC_S1F_BIT: u32 = 10;
pub const RT5659_PWR_DAC_MF_L: u32 = (0x1 << 9);
pub const RT5659_PWR_DAC_MF_L_BIT: u32 = 9;
pub const RT5659_PWR_DAC_MF_R: u32 = (0x1 << 8);
pub const RT5659_PWR_DAC_MF_R_BIT: u32 = 8;
pub const RT5659_PWR_PDM1: u32 = (0x1 << 7);
pub const RT5659_PWR_PDM1_BIT: u32 = 7;

/* Power Management for Analog 1 (0x0063) */
pub const RT5659_PWR_VREF1: u32 = (0x1 << 15);
pub const RT5659_PWR_VREF1_BIT: u32 = 15;
pub const RT5659_PWR_FV1: u32 = (0x1 << 14);
pub const RT5659_PWR_FV1_BIT: u32 = 14;
pub const RT5659_PWR_VREF2: u32 = (0x1 << 13);
pub const RT5659_PWR_VREF2_BIT: u32 = 13;
pub const RT5659_PWR_FV2: u32 = (0x1 << 12);
pub const RT5659_PWR_FV2_BIT: u32 = 12;
pub const RT5659_PWR_VREF3: u32 = (0x1 << 11);
pub const RT5659_PWR_VREF3_BIT: u32 = 11;
pub const RT5659_PWR_FV3: u32 = (0x1 << 10);
pub const RT5659_PWR_FV3_BIT: u32 = 10;
pub const RT5659_PWR_MB: u32 = (0x1 << 9);
pub const RT5659_PWR_MB_BIT: u32 = 9;
pub const RT5659_PWR_LM: u32 = (0x1 << 8);
pub const RT5659_PWR_LM_BIT: u32 = 8;
pub const RT5659_PWR_BG: u32 = (0x1 << 7);
pub const RT5659_PWR_BG_BIT: u32 = 7;
pub const RT5659_PWR_MA: u32 = (0x1 << 6);
pub const RT5659_PWR_MA_BIT: u32 = 6;
pub const RT5659_PWR_HA_L: u32 = (0x1 << 5);
pub const RT5659_PWR_HA_L_BIT: u32 = 5;
pub const RT5659_PWR_HA_R: u32 = (0x1 << 4);
pub const RT5659_PWR_HA_R_BIT: u32 = 4;

/* Power Management for Analog 2 (0x0064) */
pub const RT5659_PWR_BST1: u32 = (0x1 << 15);
pub const RT5659_PWR_BST1_BIT: u32 = 15;
pub const RT5659_PWR_BST2: u32 = (0x1 << 14);
pub const RT5659_PWR_BST2_BIT: u32 = 14;
pub const RT5659_PWR_BST3: u32 = (0x1 << 13);
pub const RT5659_PWR_BST3_BIT: u32 = 13;
pub const RT5659_PWR_BST4: u32 = (0x1 << 12);
pub const RT5659_PWR_BST4_BIT: u32 = 12;
pub const RT5659_PWR_MB1: u32 = (0x1 << 11);
pub const RT5659_PWR_MB1_BIT: u32 = 11;
pub const RT5659_PWR_MB2: u32 = (0x1 << 10);
pub const RT5659_PWR_MB2_BIT: u32 = 10;
pub const RT5659_PWR_MB3: u32 = (0x1 << 9);
pub const RT5659_PWR_MB3_BIT: u32 = 9;
pub const RT5659_PWR_BST1_P: u32 = (0x1 << 6);
pub const RT5659_PWR_BST1_P_BIT: u32 = 6;
pub const RT5659_PWR_BST2_P: u32 = (0x1 << 5);
pub const RT5659_PWR_BST2_P_BIT: u32 = 5;
pub const RT5659_PWR_BST3_P: u32 = (0x1 << 4);
pub const RT5659_PWR_BST3_P_BIT: u32 = 4;
pub const RT5659_PWR_BST4_P: u32 = (0x1 << 3);
pub const RT5659_PWR_BST4_P_BIT: u32 = 3;
pub const RT5659_PWR_JD1: u32 = (0x1 << 2);
pub const RT5659_PWR_JD1_BIT: u32 = 2;
pub const RT5659_PWR_JD2: u32 = (0x1 << 1);
pub const RT5659_PWR_JD2_BIT: u32 = 1;
pub const RT5659_PWR_JD3: u32 = (0x1);
pub const RT5659_PWR_JD3_BIT: u32 = 0;

/* Power Management for Analog 3 (0x0065) */
pub const RT5659_PWR_BST_L: u32 = (0x1 << 8);
pub const RT5659_PWR_BST_L_BIT: u32 = 8;
pub const RT5659_PWR_BST_R: u32 = (0x1 << 7);
pub const RT5659_PWR_BST_R_BIT: u32 = 7;
pub const RT5659_PWR_PLL: u32 = (0x1 << 6);
pub const RT5659_PWR_PLL_BIT: u32 = 6;
pub const RT5659_PWR_LDO5: u32 = (0x1 << 5);
pub const RT5659_PWR_LDO5_BIT: u32 = 5;
pub const RT5659_PWR_LDO4: u32 = (0x1 << 4);
pub const RT5659_PWR_LDO4_BIT: u32 = 4;
pub const RT5659_PWR_LDO3: u32 = (0x1 << 3);
pub const RT5659_PWR_LDO3_BIT: u32 = 3;
pub const RT5659_PWR_LDO2: u32 = (0x1 << 2);
pub const RT5659_PWR_LDO2_BIT: u32 = 2;
pub const RT5659_PWR_SVD: u32 = (0x1 << 1);
pub const RT5659_PWR_SVD_BIT: u32 = 1;

/* Power Management for Mixer (0x0066) */
pub const RT5659_PWR_OM_L: u32 = (0x1 << 15);
pub const RT5659_PWR_OM_L_BIT: u32 = 15;
pub const RT5659_PWR_OM_R: u32 = (0x1 << 14);
pub const RT5659_PWR_OM_R_BIT: u32 = 14;
pub const RT5659_PWR_SM_L: u32 = (0x1 << 13);
pub const RT5659_PWR_SM_L_BIT: u32 = 13;
pub const RT5659_PWR_SM_R: u32 = (0x1 << 12);
pub const RT5659_PWR_SM_R_BIT: u32 = 12;
pub const RT5659_PWR_RM1_L: u32 = (0x1 << 11);
pub const RT5659_PWR_RM1_L_BIT: u32 = 11;
pub const RT5659_PWR_RM1_R: u32 = (0x1 << 10);
pub const RT5659_PWR_RM1_R_BIT: u32 = 10;
pub const RT5659_PWR_MM: u32 = (0x1 << 8);
pub const RT5659_PWR_MM_BIT: u32 = 8;
pub const RT5659_PWR_RM2_L: u32 = (0x1 << 3);
pub const RT5659_PWR_RM2_L_BIT: u32 = 3;
pub const RT5659_PWR_RM2_R: u32 = (0x1 << 2);
pub const RT5659_PWR_RM2_R_BIT: u32 = 2;

/* Power Management for Volume (0x0067) */
pub const RT5659_PWR_SV_L: u32 = (0x1 << 15);
pub const RT5659_PWR_SV_L_BIT: u32 = 15;
pub const RT5659_PWR_SV_R: u32 = (0x1 << 14);
pub const RT5659_PWR_SV_R_BIT: u32 = 14;
pub const RT5659_PWR_OV_L: u32 = (0x1 << 13);
pub const RT5659_PWR_OV_L_BIT: u32 = 13;
pub const RT5659_PWR_OV_R: u32 = (0x1 << 12);
pub const RT5659_PWR_OV_R_BIT: u32 = 12;
pub const RT5659_PWR_IN_L: u32 = (0x1 << 9);
pub const RT5659_PWR_IN_L_BIT: u32 = 9;
pub const RT5659_PWR_IN_R: u32 = (0x1 << 8);
pub const RT5659_PWR_IN_R_BIT: u32 = 8;
pub const RT5659_PWR_MV: u32 = (0x1 << 7);
pub const RT5659_PWR_MV_BIT: u32 = 7;
pub const RT5659_PWR_MIC_DET: u32 = (0x1 << 5);
pub const RT5659_PWR_MIC_DET_BIT: u32 = 5;

/* I2S1/2/3 Audio Serial Data Port Control (0x0070 0x0071 0x0072) */
pub const RT5659_I2S_MS_MASK: u32 = (0x1 << 15);
pub const RT5659_I2S_MS_SFT: u32 = 15;
pub const RT5659_I2S_MS_M: u32 = (0x0 << 15);
pub const RT5659_I2S_MS_S: u32 = (0x1 << 15);
pub const RT5659_I2S_O_CP_MASK: u32 = (0x3 << 12);
pub const RT5659_I2S_O_CP_SFT: u32 = 12;
pub const RT5659_I2S_O_CP_OFF: u32 = (0x0 << 12);
pub const RT5659_I2S_O_CP_U_LAW: u32 = (0x1 << 12);
pub const RT5659_I2S_O_CP_A_LAW: u32 = (0x2 << 12);
pub const RT5659_I2S_I_CP_MASK: u32 = (0x3 << 10);
pub const RT5659_I2S_I_CP_SFT: u32 = 10;
pub const RT5659_I2S_I_CP_OFF: u32 = (0x0 << 10);
pub const RT5659_I2S_I_CP_U_LAW: u32 = (0x1 << 10);
pub const RT5659_I2S_I_CP_A_LAW: u32 = (0x2 << 10);
pub const RT5659_I2S_BP_MASK: u32 = (0x1 << 8);
pub const RT5659_I2S_BP_SFT: u32 = 8;
pub const RT5659_I2S_BP_NOR: u32 = (0x0 << 8);
pub const RT5659_I2S_BP_INV: u32 = (0x1 << 8);
pub const RT5659_I2S_DL_MASK: u32 = (0x3 << 4);
pub const RT5659_I2S_DL_SFT: u32 = 4;
pub const RT5659_I2S_DL_16: u32 = (0x0 << 4);
pub const RT5659_I2S_DL_20: u32 = (0x1 << 4);
pub const RT5659_I2S_DL_24: u32 = (0x2 << 4);
pub const RT5659_I2S_DL_8: u32 = (0x3 << 4);
pub const RT5659_I2S_DF_MASK: u32 = (0x7);
pub const RT5659_I2S_DF_SFT: u32 = 0;
pub const RT5659_I2S_DF_I2S: u32 = (0x0);
pub const RT5659_I2S_DF_LEFT: u32 = (0x1);
pub const RT5659_I2S_DF_PCM_A: u32 = (0x2);
pub const RT5659_I2S_DF_PCM_B: u32 = (0x3);
pub const RT5659_I2S_DF_PCM_A_N: u32 = (0x6);
pub const RT5659_I2S_DF_PCM_B_N: u32 = (0x7);

/* ADC/DAC Clock Control 1 (0x0073) */
pub const RT5659_I2S_PD1_MASK: u32 = (0x7 << 12);
pub const RT5659_I2S_PD1_SFT: u32 = 12;
pub const RT5659_I2S_PD1_1: u32 = (0x0 << 12);
pub const RT5659_I2S_PD1_2: u32 = (0x1 << 12);
pub const RT5659_I2S_PD1_3: u32 = (0x2 << 12);
pub const RT5659_I2S_PD1_4: u32 = (0x3 << 12);
pub const RT5659_I2S_PD1_6: u32 = (0x4 << 12);
pub const RT5659_I2S_PD1_8: u32 = (0x5 << 12);
pub const RT5659_I2S_PD1_12: u32 = (0x6 << 12);
pub const RT5659_I2S_PD1_16: u32 = (0x7 << 12);
pub const RT5659_I2S_BCLK_MS2_MASK: u32 = (0x1 << 11);
pub const RT5659_I2S_BCLK_MS2_SFT: u32 = 11;
pub const RT5659_I2S_BCLK_MS2_32: u32 = (0x0 << 11);
pub const RT5659_I2S_BCLK_MS2_64: u32 = (0x1 << 11);
pub const RT5659_I2S_PD2_MASK: u32 = (0x7 << 8);
pub const RT5659_I2S_PD2_SFT: u32 = 8;
pub const RT5659_I2S_PD2_1: u32 = (0x0 << 8);
pub const RT5659_I2S_PD2_2: u32 = (0x1 << 8);
pub const RT5659_I2S_PD2_3: u32 = (0x2 << 8);
pub const RT5659_I2S_PD2_4: u32 = (0x3 << 8);
pub const RT5659_I2S_PD2_6: u32 = (0x4 << 8);
pub const RT5659_I2S_PD2_8: u32 = (0x5 << 8);
pub const RT5659_I2S_PD2_12: u32 = (0x6 << 8);
pub const RT5659_I2S_PD2_16: u32 = (0x7 << 8);
pub const RT5659_I2S_BCLK_MS3_MASK: u32 = (0x1 << 7);
pub const RT5659_I2S_BCLK_MS3_SFT: u32 = 7;
pub const RT5659_I2S_BCLK_MS3_32: u32 = (0x0 << 7);
pub const RT5659_I2S_BCLK_MS3_64: u32 = (0x1 << 7);
pub const RT5659_I2S_PD3_MASK: u32 = (0x7 << 4);
pub const RT5659_I2S_PD3_SFT: u32 = 4;
pub const RT5659_I2S_PD3_1: u32 = (0x0 << 4);
pub const RT5659_I2S_PD3_2: u32 = (0x1 << 4);
pub const RT5659_I2S_PD3_3: u32 = (0x2 << 4);
pub const RT5659_I2S_PD3_4: u32 = (0x3 << 4);
pub const RT5659_I2S_PD3_6: u32 = (0x4 << 4);
pub const RT5659_I2S_PD3_8: u32 = (0x5 << 4);
pub const RT5659_I2S_PD3_12: u32 = (0x6 << 4);
pub const RT5659_I2S_PD3_16: u32 = (0x7 << 4);
pub const RT5659_DAC_OSR_MASK: u32 = (0x3 << 2);
pub const RT5659_DAC_OSR_SFT: u32 = 2;
pub const RT5659_DAC_OSR_128: u32 = (0x0 << 2);
pub const RT5659_DAC_OSR_64: u32 = (0x1 << 2);
pub const RT5659_DAC_OSR_32: u32 = (0x2 << 2);
pub const RT5659_DAC_OSR_16: u32 = (0x3 << 2);
pub const RT5659_ADC_OSR_MASK: u32 = (0x3);
pub const RT5659_ADC_OSR_SFT: u32 = 0;
pub const RT5659_ADC_OSR_128: u32 = (0x0);
pub const RT5659_ADC_OSR_64: u32 = (0x1);
pub const RT5659_ADC_OSR_32: u32 = (0x2);
pub const RT5659_ADC_OSR_16: u32 = (0x3);

/* Digital Microphone Control (0x0075) */
pub const RT5659_DMIC_1_EN_MASK: u32 = (0x1 << 15);
pub const RT5659_DMIC_1_EN_SFT: u32 = 15;
pub const RT5659_DMIC_1_DIS: u32 = (0x0 << 15);
pub const RT5659_DMIC_1_EN: u32 = (0x1 << 15);
pub const RT5659_DMIC_2_EN_MASK: u32 = (0x1 << 14);
pub const RT5659_DMIC_2_EN_SFT: u32 = 14;
pub const RT5659_DMIC_2_DIS: u32 = (0x0 << 14);
pub const RT5659_DMIC_2_EN: u32 = (0x1 << 14);
pub const RT5659_DMIC_1L_LH_MASK: u32 = (0x1 << 13);
pub const RT5659_DMIC_1L_LH_SFT: u32 = 13;
pub const RT5659_DMIC_1L_LH_RISING: u32 = (0x0 << 13);
pub const RT5659_DMIC_1L_LH_FALLING: u32 = (0x1 << 13);
pub const RT5659_DMIC_1R_LH_MASK: u32 = (0x1 << 12);
pub const RT5659_DMIC_1R_LH_SFT: u32 = 12;
pub const RT5659_DMIC_1R_LH_RISING: u32 = (0x0 << 12);
pub const RT5659_DMIC_1R_LH_FALLING: u32 = (0x1 << 12);
pub const RT5659_DMIC_2_DP_MASK: u32 = (0x3 << 10);
pub const RT5659_DMIC_2_DP_SFT: u32 = 10;
pub const RT5659_DMIC_2_DP_GPIO6: u32 = (0x0 << 10);
pub const RT5659_DMIC_2_DP_GPIO10: u32 = (0x1 << 10);
pub const RT5659_DMIC_2_DP_GPIO12: u32 = (0x2 << 10);
pub const RT5659_DMIC_2_DP_IN2P: u32 = (0x3 << 10);
pub const RT5659_DMIC_CLK_MASK: u32 = (0x7 << 5);
pub const RT5659_DMIC_CLK_SFT: u32 = 5;
pub const RT5659_DMIC_1_DP_MASK: u32 = (0x3 << 0);
pub const RT5659_DMIC_1_DP_SFT: u32 = 0;
pub const RT5659_DMIC_1_DP_GPIO5: u32 = (0x0 << 0);
pub const RT5659_DMIC_1_DP_GPIO9: u32 = (0x1 << 0);
pub const RT5659_DMIC_1_DP_GPIO11: u32 = (0x2 << 0);
pub const RT5659_DMIC_1_DP_IN2N: u32 = (0x3 << 0);

/* TDM control 1 (0x0078)*/
pub const RT5659_DS_ADC_SLOT01_SFT: u32 = 14;
pub const RT5659_DS_ADC_SLOT23_SFT: u32 = 12;
pub const RT5659_DS_ADC_SLOT45_SFT: u32 = 10;
pub const RT5659_DS_ADC_SLOT67_SFT: u32 = 8;
pub const RT5659_ADCDAT_SRC_MASK: u32 = 0x1f;
pub const RT5659_ADCDAT_SRC_SFT: u32 = 0;

/* Global Clock Control (0x0080) */
pub const RT5659_SCLK_SRC_MASK: u32 = (0x3 << 14);
pub const RT5659_SCLK_SRC_SFT: u32 = 14;
pub const RT5659_SCLK_SRC_MCLK: u32 = (0x0 << 14);
pub const RT5659_SCLK_SRC_PLL1: u32 = (0x1 << 14);
pub const RT5659_SCLK_SRC_RCCLK: u32 = (0x2 << 14);
pub const RT5659_PLL1_SRC_MASK: u32 = (0x7 << 11);
pub const RT5659_PLL1_SRC_SFT: u32 = 11;
pub const RT5659_PLL1_SRC_MCLK: u32 = (0x0 << 11);
pub const RT5659_PLL1_SRC_BCLK1: u32 = (0x1 << 11);
pub const RT5659_PLL1_SRC_BCLK2: u32 = (0x2 << 11);
pub const RT5659_PLL1_SRC_BCLK3: u32 = (0x3 << 11);
pub const RT5659_PLL1_PD_MASK: u32 = (0x1 << 3);
pub const RT5659_PLL1_PD_SFT: u32 = 3;
pub const RT5659_PLL1_PD_1: u32 = (0x0 << 3);
pub const RT5659_PLL1_PD_2: u32 = (0x1 << 3);

pub const RT5659_PLL_INP_MAX: u32 = 40000000;
pub const RT5659_PLL_INP_MIN: u32 = 256000;
/* PLL M/N/K Code Control 1 (0x0081) */
pub const RT5659_PLL_N_MAX: u32 = 0x001ff;
pub const RT5659_PLL_N_MASK: u32 = (RT5659_PLL_N_MAX << 7);
pub const RT5659_PLL_N_SFT: u32 = 7;
pub const RT5659_PLL_K_MAX: u32 = 0x001f;
pub const RT5659_PLL_K_MASK: u32 = (RT5659_PLL_K_MAX);
pub const RT5659_PLL_K_SFT: u32 = 0;

/* PLL M/N/K Code Control 2 (0x0082) */
pub const RT5659_PLL_M_MAX: u32 = 0x00f;
pub const RT5659_PLL_M_MASK: u32 = (RT5659_PLL_M_MAX << 12);
pub const RT5659_PLL_M_SFT: u32 = 12;
pub const RT5659_PLL_M_BP: u32 = (0x1 << 11);
pub const RT5659_PLL_M_BP_SFT: u32 = 11;

/* PLL tracking mode 1 (0x0083) */
pub const RT5659_I2S3_ASRC_MASK: u32 = (0x1 << 13);
pub const RT5659_I2S3_ASRC_SFT: u32 = 13;
pub const RT5659_I2S2_ASRC_MASK: u32 = (0x1 << 12);
pub const RT5659_I2S2_ASRC_SFT: u32 = 12;
pub const RT5659_I2S1_ASRC_MASK: u32 = (0x1 << 11);
pub const RT5659_I2S1_ASRC_SFT: u32 = 11;
pub const RT5659_DAC_STO_ASRC_MASK: u32 = (0x1 << 10);
pub const RT5659_DAC_STO_ASRC_SFT: u32 = 10;
pub const RT5659_DAC_MONO_L_ASRC_MASK: u32 = (0x1 << 9);
pub const RT5659_DAC_MONO_L_ASRC_SFT: u32 = 9;
pub const RT5659_DAC_MONO_R_ASRC_MASK: u32 = (0x1 << 8);
pub const RT5659_DAC_MONO_R_ASRC_SFT: u32 = 8;
pub const RT5659_DMIC_STO1_ASRC_MASK: u32 = (0x1 << 7);
pub const RT5659_DMIC_STO1_ASRC_SFT: u32 = 7;
pub const RT5659_DMIC_MONO_L_ASRC_MASK: u32 = (0x1 << 5);
pub const RT5659_DMIC_MONO_L_ASRC_SFT: u32 = 5;
pub const RT5659_DMIC_MONO_R_ASRC_MASK: u32 = (0x1 << 4);
pub const RT5659_DMIC_MONO_R_ASRC_SFT: u32 = 4;
pub const RT5659_ADC_STO1_ASRC_MASK: u32 = (0x1 << 3);
pub const RT5659_ADC_STO1_ASRC_SFT: u32 = 3;
pub const RT5659_ADC_MONO_L_ASRC_MASK: u32 = (0x1 << 1);
pub const RT5659_ADC_MONO_L_ASRC_SFT: u32 = 1;
pub const RT5659_ADC_MONO_R_ASRC_MASK: u32 = (0x1);
pub const RT5659_ADC_MONO_R_ASRC_SFT: u32 = 0;

/* PLL tracking mode 2 (0x0084)*/
pub const RT5659_DA_STO_T_MASK: u32 = (0x7 << 12);
pub const RT5659_DA_STO_T_SFT: u32 = 12;
pub const RT5659_DA_MONO_L_T_MASK: u32 = (0x7 << 8);
pub const RT5659_DA_MONO_L_T_SFT: u32 = 8;
pub const RT5659_DA_MONO_R_T_MASK: u32 = (0x7 << 4);
pub const RT5659_DA_MONO_R_T_SFT: u32 = 4;
pub const RT5659_AD_STO1_T_MASK: u32 = (0x7);
pub const RT5659_AD_STO1_T_SFT: u32 = 0;

/* PLL tracking mode 3 (0x0085)*/
pub const RT5659_AD_STO2_T_MASK: u32 = (0x7 << 8);
pub const RT5659_AD_STO2_T_SFT: u32 = 8;
pub const RT5659_AD_MONO_L_T_MASK: u32 = (0x7 << 4);
pub const RT5659_AD_MONO_L_T_SFT: u32 = 4;
pub const RT5659_AD_MONO_R_T_MASK: u32 = (0x7);
pub const RT5659_AD_MONO_R_T_SFT: u32 = 0;

/* ASRC Control 4 (0x0086) */
pub const RT5659_I2S1_RATE_MASK: u32 = (0xf << 12);
pub const RT5659_I2S1_RATE_SFT: u32 = 12;
pub const RT5659_I2S2_RATE_MASK: u32 = (0xf << 8);
pub const RT5659_I2S2_RATE_SFT: u32 = 8;
pub const RT5659_I2S3_RATE_MASK: u32 = (0xf << 4);
pub const RT5659_I2S3_RATE_SFT: u32 = 4;

/* Depop Mode Control 1 (0x8e) */
pub const RT5659_SMT_TRIG_MASK: u32 = (0x1 << 15);
pub const RT5659_SMT_TRIG_SFT: u32 = 15;
pub const RT5659_SMT_TRIG_DIS: u32 = (0x0 << 15);
pub const RT5659_SMT_TRIG_EN: u32 = (0x1 << 15);
pub const RT5659_HP_L_SMT_MASK: u32 = (0x1 << 9);
pub const RT5659_HP_L_SMT_SFT: u32 = 9;
pub const RT5659_HP_L_SMT_DIS: u32 = (0x0 << 9);
pub const RT5659_HP_L_SMT_EN: u32 = (0x1 << 9);
pub const RT5659_HP_R_SMT_MASK: u32 = (0x1 << 8);
pub const RT5659_HP_R_SMT_SFT: u32 = 8;
pub const RT5659_HP_R_SMT_DIS: u32 = (0x0 << 8);
pub const RT5659_HP_R_SMT_EN: u32 = (0x1 << 8);
pub const RT5659_HP_CD_PD_MASK: u32 = (0x1 << 7);
pub const RT5659_HP_CD_PD_SFT: u32 = 7;
pub const RT5659_HP_CD_PD_DIS: u32 = (0x0 << 7);
pub const RT5659_HP_CD_PD_EN: u32 = (0x1 << 7);
pub const RT5659_RSTN_MASK: u32 = (0x1 << 6);
pub const RT5659_RSTN_SFT: u32 = 6;
pub const RT5659_RSTN_DIS: u32 = (0x0 << 6);
pub const RT5659_RSTN_EN: u32 = (0x1 << 6);
pub const RT5659_RSTP_MASK: u32 = (0x1 << 5);
pub const RT5659_RSTP_SFT: u32 = 5;
pub const RT5659_RSTP_DIS: u32 = (0x0 << 5);
pub const RT5659_RSTP_EN: u32 = (0x1 << 5);
pub const RT5659_HP_CO_MASK: u32 = (0x1 << 4);
pub const RT5659_HP_CO_SFT: u32 = 4;
pub const RT5659_HP_CO_DIS: u32 = (0x0 << 4);
pub const RT5659_HP_CO_EN: u32 = (0x1 << 4);
pub const RT5659_HP_CP_MASK: u32 = (0x1 << 3);
pub const RT5659_HP_CP_SFT: u32 = 3;
pub const RT5659_HP_CP_PD: u32 = (0x0 << 3);
pub const RT5659_HP_CP_PU: u32 = (0x1 << 3);
pub const RT5659_HP_SG_MASK: u32 = (0x1 << 2);
pub const RT5659_HP_SG_SFT: u32 = 2;
pub const RT5659_HP_SG_DIS: u32 = (0x0 << 2);
pub const RT5659_HP_SG_EN: u32 = (0x1 << 2);
pub const RT5659_HP_DP_MASK: u32 = (0x1 << 1);
pub const RT5659_HP_DP_SFT: u32 = 1;
pub const RT5659_HP_DP_PD: u32 = (0x0 << 1);
pub const RT5659_HP_DP_PU: u32 = (0x1 << 1);
pub const RT5659_HP_CB_MASK: u32 = (0x1);
pub const RT5659_HP_CB_SFT: u32 = 0;
pub const RT5659_HP_CB_PD: u32 = (0x0);
pub const RT5659_HP_CB_PU: u32 = (0x1);

/* Depop Mode Control 2 (0x8f) */
pub const RT5659_DEPOP_MASK: u32 = (0x1 << 13);
pub const RT5659_DEPOP_SFT: u32 = 13;
pub const RT5659_DEPOP_AUTO: u32 = (0x0 << 13);
pub const RT5659_DEPOP_MAN: u32 = (0x1 << 13);
pub const RT5659_RAMP_MASK: u32 = (0x1 << 12);
pub const RT5659_RAMP_SFT: u32 = 12;
pub const RT5659_RAMP_DIS: u32 = (0x0 << 12);
pub const RT5659_RAMP_EN: u32 = (0x1 << 12);
pub const RT5659_BPS_MASK: u32 = (0x1 << 11);
pub const RT5659_BPS_SFT: u32 = 11;
pub const RT5659_BPS_DIS: u32 = (0x0 << 11);
pub const RT5659_BPS_EN: u32 = (0x1 << 11);
pub const RT5659_FAST_UPDN_MASK: u32 = (0x1 << 10);
pub const RT5659_FAST_UPDN_SFT: u32 = 10;
pub const RT5659_FAST_UPDN_DIS: u32 = (0x0 << 10);
pub const RT5659_FAST_UPDN_EN: u32 = (0x1 << 10);
pub const RT5659_MRES_MASK: u32 = (0x3 << 8);
pub const RT5659_MRES_SFT: u32 = 8;
pub const RT5659_MRES_15MO: u32 = (0x0 << 8);
pub const RT5659_MRES_25MO: u32 = (0x1 << 8);
pub const RT5659_MRES_35MO: u32 = (0x2 << 8);
pub const RT5659_MRES_45MO: u32 = (0x3 << 8);
pub const RT5659_VLO_MASK: u32 = (0x1 << 7);
pub const RT5659_VLO_SFT: u32 = 7;
pub const RT5659_VLO_3V: u32 = (0x0 << 7);
pub const RT5659_VLO_32V: u32 = (0x1 << 7);
pub const RT5659_DIG_DP_MASK: u32 = (0x1 << 6);
pub const RT5659_DIG_DP_SFT: u32 = 6;
pub const RT5659_DIG_DP_DIS: u32 = (0x0 << 6);
pub const RT5659_DIG_DP_EN: u32 = (0x1 << 6);
pub const RT5659_DP_TH_MASK: u32 = (0x3 << 4);
pub const RT5659_DP_TH_SFT: u32 = 4;

/* Depop Mode Control 3 (0x90) */
pub const RT5659_CP_SYS_MASK: u32 = (0x7 << 12);
pub const RT5659_CP_SYS_SFT: u32 = 12;
pub const RT5659_CP_FQ1_MASK: u32 = (0x7 << 8);
pub const RT5659_CP_FQ1_SFT: u32 = 8;
pub const RT5659_CP_FQ2_MASK: u32 = (0x7 << 4);
pub const RT5659_CP_FQ2_SFT: u32 = 4;
pub const RT5659_CP_FQ3_MASK: u32 = (0x7);
pub const RT5659_CP_FQ3_SFT: u32 = 0;
pub const RT5659_CP_FQ_1_5_KHZ: u32 = 0;
pub const RT5659_CP_FQ_3_KHZ: u32 = 1;
pub const RT5659_CP_FQ_6_KHZ: u32 = 2;
pub const RT5659_CP_FQ_12_KHZ: u32 = 3;
pub const RT5659_CP_FQ_24_KHZ: u32 = 4;
pub const RT5659_CP_FQ_48_KHZ: u32 = 5;
pub const RT5659_CP_FQ_96_KHZ: u32 = 6;
pub const RT5659_CP_FQ_192_KHZ: u32 = 7;

/* HPOUT charge pump 1 (0x0091) */
pub const RT5659_OSW_L_MASK: u32 = (0x1 << 11);
pub const RT5659_OSW_L_SFT: u32 = 11;
pub const RT5659_OSW_L_DIS: u32 = (0x0 << 11);
pub const RT5659_OSW_L_EN: u32 = (0x1 << 11);
pub const RT5659_OSW_R_MASK: u32 = (0x1 << 10);
pub const RT5659_OSW_R_SFT: u32 = 10;
pub const RT5659_OSW_R_DIS: u32 = (0x0 << 10);
pub const RT5659_OSW_R_EN: u32 = (0x1 << 10);
pub const RT5659_PM_HP_MASK: u32 = (0x3 << 8);
pub const RT5659_PM_HP_SFT: u32 = 8;
pub const RT5659_PM_HP_LV: u32 = (0x0 << 8);
pub const RT5659_PM_HP_MV: u32 = (0x1 << 8);
pub const RT5659_PM_HP_HV: u32 = (0x2 << 8);
pub const RT5659_IB_HP_MASK: u32 = (0x3 << 6);
pub const RT5659_IB_HP_SFT: u32 = 6;
pub const RT5659_IB_HP_125IL: u32 = (0x0 << 6);
pub const RT5659_IB_HP_25IL: u32 = (0x1 << 6);
pub const RT5659_IB_HP_5IL: u32 = (0x2 << 6);
pub const RT5659_IB_HP_1IL: u32 = (0x3 << 6);

/* PV detection and SPK gain control (0x92) */
pub const RT5659_PVDD_DET_MASK: u32 = (0x1 << 15);
pub const RT5659_PVDD_DET_SFT: u32 = 15;
pub const RT5659_PVDD_DET_DIS: u32 = (0x0 << 15);
pub const RT5659_PVDD_DET_EN: u32 = (0x1 << 15);
pub const RT5659_SPK_AG_MASK: u32 = (0x1 << 14);
pub const RT5659_SPK_AG_SFT: u32 = 14;
pub const RT5659_SPK_AG_DIS: u32 = (0x0 << 14);
pub const RT5659_SPK_AG_EN: u32 = (0x1 << 14);

/* Micbias Control (0x93) */
pub const RT5659_MIC1_BS_MASK: u32 = (0x1 << 15);
pub const RT5659_MIC1_BS_SFT: u32 = 15;
pub const RT5659_MIC1_BS_9AV: u32 = (0x0 << 15);
pub const RT5659_MIC1_BS_75AV: u32 = (0x1 << 15);
pub const RT5659_MIC2_BS_MASK: u32 = (0x1 << 14);
pub const RT5659_MIC2_BS_SFT: u32 = 14;
pub const RT5659_MIC2_BS_9AV: u32 = (0x0 << 14);
pub const RT5659_MIC2_BS_75AV: u32 = (0x1 << 14);
pub const RT5659_MIC1_CLK_MASK: u32 = (0x1 << 13);
pub const RT5659_MIC1_CLK_SFT: u32 = 13;
pub const RT5659_MIC1_CLK_DIS: u32 = (0x0 << 13);
pub const RT5659_MIC1_CLK_EN: u32 = (0x1 << 13);
pub const RT5659_MIC2_CLK_MASK: u32 = (0x1 << 12);
pub const RT5659_MIC2_CLK_SFT: u32 = 12;
pub const RT5659_MIC2_CLK_DIS: u32 = (0x0 << 12);
pub const RT5659_MIC2_CLK_EN: u32 = (0x1 << 12);
pub const RT5659_MIC1_OVCD_MASK: u32 = (0x1 << 11);
pub const RT5659_MIC1_OVCD_SFT: u32 = 11;
pub const RT5659_MIC1_OVCD_DIS: u32 = (0x0 << 11);
pub const RT5659_MIC1_OVCD_EN: u32 = (0x1 << 11);
pub const RT5659_MIC1_OVTH_MASK: u32 = (0x3 << 9);
pub const RT5659_MIC1_OVTH_SFT: u32 = 9;
pub const RT5659_MIC1_OVTH_600UA: u32 = (0x0 << 9);
pub const RT5659_MIC1_OVTH_1500UA: u32 = (0x1 << 9);
pub const RT5659_MIC1_OVTH_2000UA: u32 = (0x2 << 9);
pub const RT5659_MIC2_OVCD_MASK: u32 = (0x1 << 8);
pub const RT5659_MIC2_OVCD_SFT: u32 = 8;
pub const RT5659_MIC2_OVCD_DIS: u32 = (0x0 << 8);
pub const RT5659_MIC2_OVCD_EN: u32 = (0x1 << 8);
pub const RT5659_MIC2_OVTH_MASK: u32 = (0x3 << 6);
pub const RT5659_MIC2_OVTH_SFT: u32 = 6;
pub const RT5659_MIC2_OVTH_600UA: u32 = (0x0 << 6);
pub const RT5659_MIC2_OVTH_1500UA: u32 = (0x1 << 6);
pub const RT5659_MIC2_OVTH_2000UA: u32 = (0x2 << 6);
pub const RT5659_PWR_MB_MASK: u32 = (0x1 << 5);
pub const RT5659_PWR_MB_SFT: u32 = 5;
pub const RT5659_PWR_MB_PD: u32 = (0x0 << 5);
pub const RT5659_PWR_MB_PU: u32 = (0x1 << 5);
pub const RT5659_PWR_CLK25M_MASK: u32 = (0x1 << 4);
pub const RT5659_PWR_CLK25M_SFT: u32 = 4;
pub const RT5659_PWR_CLK25M_PD: u32 = (0x0 << 4);
pub const RT5659_PWR_CLK25M_PU: u32 = (0x1 << 4);

/* REC Mixer 2 Left Control 2 (0x009c) */
pub const RT5659_M_BST1_RM2_L: u32 = (0x1 << 5);
pub const RT5659_M_BST1_RM2_L_SFT: u32 = 5;
pub const RT5659_M_BST2_RM2_L: u32 = (0x1 << 4);
pub const RT5659_M_BST2_RM2_L_SFT: u32 = 4;
pub const RT5659_M_BST3_RM2_L: u32 = (0x1 << 3);
pub const RT5659_M_BST3_RM2_L_SFT: u32 = 3;
pub const RT5659_M_BST4_RM2_L: u32 = (0x1 << 2);
pub const RT5659_M_BST4_RM2_L_SFT: u32 = 2;
pub const RT5659_M_OUTVOLL_RM2_L: u32 = (0x1 << 1);
pub const RT5659_M_OUTVOLL_RM2_L_SFT: u32 = 1;
pub const RT5659_M_SPKVOL_RM2_L: u32 = (0x1);
pub const RT5659_M_SPKVOL_RM2_L_SFT: u32 = 0;

/* REC Mixer 2 Right Control 2 (0x009e) */
pub const RT5659_M_BST1_RM2_R: u32 = (0x1 << 5);
pub const RT5659_M_BST1_RM2_R_SFT: u32 = 5;
pub const RT5659_M_BST2_RM2_R: u32 = (0x1 << 4);
pub const RT5659_M_BST2_RM2_R_SFT: u32 = 4;
pub const RT5659_M_BST3_RM2_R: u32 = (0x1 << 3);
pub const RT5659_M_BST3_RM2_R_SFT: u32 = 3;
pub const RT5659_M_BST4_RM2_R: u32 = (0x1 << 2);
pub const RT5659_M_BST4_RM2_R_SFT: u32 = 2;
pub const RT5659_M_OUTVOLR_RM2_R: u32 = (0x1 << 1);
pub const RT5659_M_OUTVOLR_RM2_R_SFT: u32 = 1;
pub const RT5659_M_MONOVOL_RM2_R: u32 = (0x1);
pub const RT5659_M_MONOVOL_RM2_R_SFT: u32 = 0;

/* Class D Output Control (0x00a0) */
pub const RT5659_POW_CLSD_DB_MASK: u32 = (0x1 << 9);
pub const RT5659_POW_CLSD_DB_EN: u32 = (0x1 << 9);
pub const RT5659_POW_CLSD_DB_DIS: u32 = (0x0 << 9);

/* EQ Control 1 (0x00b0) */
pub const RT5659_EQ_SRC_DAC: u32 = (0x0 << 15);
pub const RT5659_EQ_SRC_ADC: u32 = (0x1 << 15);
pub const RT5659_EQ_UPD: u32 = (0x1 << 14);
pub const RT5659_EQ_UPD_BIT: u32 = 14;
pub const RT5659_EQ_CD_MASK: u32 = (0x1 << 13);
pub const RT5659_EQ_CD_SFT: u32 = 13;
pub const RT5659_EQ_CD_DIS: u32 = (0x0 << 13);
pub const RT5659_EQ_CD_EN: u32 = (0x1 << 13);
pub const RT5659_EQ_DITH_MASK: u32 = (0x3 << 8);
pub const RT5659_EQ_DITH_SFT: u32 = 8;
pub const RT5659_EQ_DITH_NOR: u32 = (0x0 << 8);
pub const RT5659_EQ_DITH_LSB: u32 = (0x1 << 8);
pub const RT5659_EQ_DITH_LSB_1: u32 = (0x2 << 8);
pub const RT5659_EQ_DITH_LSB_2: u32 = (0x3 << 8);

/* IRQ Control 1 (0x00b7) */
pub const RT5659_JD1_1_EN_MASK: u32 = (0x1 << 15);
pub const RT5659_JD1_1_EN_SFT: u32 = 15;
pub const RT5659_JD1_1_DIS: u32 = (0x0 << 15);
pub const RT5659_JD1_1_EN: u32 = (0x1 << 15);
pub const RT5659_JD1_2_EN_MASK: u32 = (0x1 << 12);
pub const RT5659_JD1_2_EN_SFT: u32 = 12;
pub const RT5659_JD1_2_DIS: u32 = (0x0 << 12);
pub const RT5659_JD1_2_EN: u32 = (0x1 << 12);
pub const RT5659_IL_IRQ_MASK: u32 = (0x1 << 3);
pub const RT5659_IL_IRQ_DIS: u32 = (0x0 << 3);
pub const RT5659_IL_IRQ_EN: u32 = (0x1 << 3);

/* IRQ Control 5 (0x00ba) */
pub const RT5659_IRQ_JD_EN: u32 = (0x1 << 3);
pub const RT5659_IRQ_JD_EN_SFT: u32 = 3;

/* GPIO Control 1 (0x00c0) */
pub const RT5659_GP1_PIN_MASK: u32 = (0x1 << 15);
pub const RT5659_GP1_PIN_SFT: u32 = 15;
pub const RT5659_GP1_PIN_GPIO1: u32 = (0x0 << 15);
pub const RT5659_GP1_PIN_IRQ: u32 = (0x1 << 15);
pub const RT5659_GP2_PIN_MASK: u32 = (0x1 << 14);
pub const RT5659_GP2_PIN_SFT: u32 = 14;
pub const RT5659_GP2_PIN_GPIO2: u32 = (0x0 << 14);
pub const RT5659_GP2_PIN_DMIC1_SCL: u32 = (0x1 << 14);
pub const RT5659_GP3_PIN_MASK: u32 = (0x1 << 13);
pub const RT5659_GP3_PIN_SFT: u32 = 13;
pub const RT5659_GP3_PIN_GPIO3: u32 = (0x0 << 13);
pub const RT5659_GP3_PIN_PDM_SCL: u32 = (0x1 << 13);
pub const RT5659_GP4_PIN_MASK: u32 = (0x1 << 12);
pub const RT5659_GP4_PIN_SFT: u32 = 12;
pub const RT5659_GP4_PIN_GPIO4: u32 = (0x0 << 12);
pub const RT5659_GP4_PIN_PDM_SDA: u32 = (0x1 << 12);
pub const RT5659_GP5_PIN_MASK: u32 = (0x1 << 11);
pub const RT5659_GP5_PIN_SFT: u32 = 11;
pub const RT5659_GP5_PIN_GPIO5: u32 = (0x0 << 11);
pub const RT5659_GP5_PIN_DMIC1_SDA: u32 = (0x1 << 11);
pub const RT5659_GP6_PIN_MASK: u32 = (0x1 << 10);
pub const RT5659_GP6_PIN_SFT: u32 = 10;
pub const RT5659_GP6_PIN_GPIO6: u32 = (0x0 << 10);
pub const RT5659_GP6_PIN_DMIC2_SDA: u32 = (0x1 << 10);
pub const RT5659_GP7_PIN_MASK: u32 = (0x1 << 9);
pub const RT5659_GP7_PIN_SFT: u32 = 9;
pub const RT5659_GP7_PIN_GPIO7: u32 = (0x0 << 9);
pub const RT5659_GP7_PIN_PDM_SCL: u32 = (0x1 << 9);
pub const RT5659_GP8_PIN_MASK: u32 = (0x1 << 8);
pub const RT5659_GP8_PIN_SFT: u32 = 8;
pub const RT5659_GP8_PIN_GPIO8: u32 = (0x0 << 8);
pub const RT5659_GP8_PIN_PDM_SDA: u32 = (0x1 << 8);
pub const RT5659_GP9_PIN_MASK: u32 = (0x1 << 7);
pub const RT5659_GP9_PIN_SFT: u32 = 7;
pub const RT5659_GP9_PIN_GPIO9: u32 = (0x0 << 7);
pub const RT5659_GP9_PIN_DMIC1_SDA: u32 = (0x1 << 7);
pub const RT5659_GP10_PIN_MASK: u32 = (0x1 << 6);
pub const RT5659_GP10_PIN_SFT: u32 = 6;
pub const RT5659_GP10_PIN_GPIO10: u32 = (0x0 << 6);
pub const RT5659_GP10_PIN_DMIC2_SDA: u32 = (0x1 << 6);
pub const RT5659_GP11_PIN_MASK: u32 = (0x1 << 5);
pub const RT5659_GP11_PIN_SFT: u32 = 5;
pub const RT5659_GP11_PIN_GPIO11: u32 = (0x0 << 5);
pub const RT5659_GP11_PIN_DMIC1_SDA: u32 = (0x1 << 5);
pub const RT5659_GP12_PIN_MASK: u32 = (0x1 << 4);
pub const RT5659_GP12_PIN_SFT: u32 = 4;
pub const RT5659_GP12_PIN_GPIO12: u32 = (0x0 << 4);
pub const RT5659_GP12_PIN_DMIC2_SDA: u32 = (0x1 << 4);
pub const RT5659_GP13_PIN_MASK: u32 = (0x3 << 2);
pub const RT5659_GP13_PIN_SFT: u32 = 2;
pub const RT5659_GP13_PIN_GPIO13: u32 = (0x0 << 2);
pub const RT5659_GP13_PIN_SPDIF_SDA: u32 = (0x1 << 2);
pub const RT5659_GP13_PIN_DMIC2_SCL: u32 = (0x2 << 2);
pub const RT5659_GP13_PIN_PDM_SCL: u32 = (0x3 << 2);
pub const RT5659_GP15_PIN_MASK: u32 = (0x3);
pub const RT5659_GP15_PIN_SFT: u32 = 0;
pub const RT5659_GP15_PIN_GPIO15: u32 = (0x0);
pub const RT5659_GP15_PIN_DMIC3_SCL: u32 = (0x1);
pub const RT5659_GP15_PIN_PDM_SDA: u32 = (0x2);

/* GPIO Control 2 (0x00c1)*/
pub const RT5659_GP1_PF_IN: u32 = (0x0 << 2);
pub const RT5659_GP1_PF_OUT: u32 = (0x1 << 2);
pub const RT5659_GP1_PF_MASK: u32 = (0x1 << 2);
pub const RT5659_GP1_PF_SFT: u32 = 2;

/* GPIO Control 3 (0x00c2) */
pub const RT5659_I2S2_PIN_MASK: u32 = (0x1 << 15);
pub const RT5659_I2S2_PIN_SFT: u32 = 15;
pub const RT5659_I2S2_PIN_I2S: u32 = (0x0 << 15);
pub const RT5659_I2S2_PIN_GPIO: u32 = (0x1 << 15);

/* Soft volume and zero cross control 1 (0x00d9) */
pub const RT5659_SV_MASK: u32 = (0x1 << 15);
pub const RT5659_SV_SFT: u32 = 15;
pub const RT5659_SV_DIS: u32 = (0x0 << 15);
pub const RT5659_SV_EN: u32 = (0x1 << 15);
pub const RT5659_OUT_SV_MASK: u32 = (0x1 << 13);
pub const RT5659_OUT_SV_SFT: u32 = 13;
pub const RT5659_OUT_SV_DIS: u32 = (0x0 << 13);
pub const RT5659_OUT_SV_EN: u32 = (0x1 << 13);
pub const RT5659_HP_SV_MASK: u32 = (0x1 << 12);
pub const RT5659_HP_SV_SFT: u32 = 12;
pub const RT5659_HP_SV_DIS: u32 = (0x0 << 12);
pub const RT5659_HP_SV_EN: u32 = (0x1 << 12);
pub const RT5659_ZCD_DIG_MASK: u32 = (0x1 << 11);
pub const RT5659_ZCD_DIG_SFT: u32 = 11;
pub const RT5659_ZCD_DIG_DIS: u32 = (0x0 << 11);
pub const RT5659_ZCD_DIG_EN: u32 = (0x1 << 11);
pub const RT5659_ZCD_MASK: u32 = (0x1 << 10);
pub const RT5659_ZCD_SFT: u32 = 10;
pub const RT5659_ZCD_PD: u32 = (0x0 << 10);
pub const RT5659_ZCD_PU: u32 = (0x1 << 10);
pub const RT5659_SV_DLY_MASK: u32 = (0xf);
pub const RT5659_SV_DLY_SFT: u32 = 0;

/* Soft volume and zero cross control 2 (0x00da) */
pub const RT5659_ZCD_HP_MASK: u32 = (0x1 << 15);
pub const RT5659_ZCD_HP_SFT: u32 = 15;
pub const RT5659_ZCD_HP_DIS: u32 = (0x0 << 15);
pub const RT5659_ZCD_HP_EN: u32 = (0x1 << 15);

/* 4 Button Inline Command Control 2 (0x00e0) */
pub const RT5659_4BTN_IL_MASK: u32 = (0x1 << 15);
pub const RT5659_4BTN_IL_EN: u32 = (0x1 << 15);
pub const RT5659_4BTN_IL_DIS: u32 = (0x0 << 15);

/* Analog JD Control 1 (0x00f0) */
pub const RT5659_JD1_MODE_MASK: u32 = (0x3 << 0);
pub const RT5659_JD1_MODE_0: u32 = (0x0 << 0);
pub const RT5659_JD1_MODE_1: u32 = (0x1 << 0);
pub const RT5659_JD1_MODE_2: u32 = (0x2 << 0);

/* Jack Detect Control 3 (0x00f8) */
pub const RT5659_JD_TRI_HPO_SEL_MASK: u32 = (0x7);
pub const RT5659_JD_TRI_HPO_SEL_SFT: u32 = (0);
pub const RT5659_JD_HPO_GPIO_JD1: u32 = (0x0);
pub const RT5659_JD_HPO_JD1_1: u32 = (0x1);
pub const RT5659_JD_HPO_JD1_2: u32 = (0x2);
pub const RT5659_JD_HPO_JD2: u32 = (0x3);
pub const RT5659_JD_HPO_GPIO_JD2: u32 = (0x4);
pub const RT5659_JD_HPO_JD3: u32 = (0x5);
pub const RT5659_JD_HPO_JD_D: u32 = (0x6);

/* Digital Misc Control (0x00fa) */
pub const RT5659_AM_MASK: u32 = (0x1 << 7);
pub const RT5659_AM_EN: u32 = (0x1 << 7);
pub const RT5659_AM_DIS: u32 = (0x1 << 7);
pub const RT5659_DIG_GATE_CTRL: u32 = 0x1;
pub const RT5659_DIG_GATE_CTRL_SFT: u32 = (0);

/* Chopper and Clock control for ADC (0x011c)*/
pub const RT5659_M_RF_DIG_MASK: u32 = (0x1 << 12);
pub const RT5659_M_RF_DIG_SFT: u32 = 12;
pub const RT5659_M_RI_DIG: u32 = (0x1 << 11);

/* Chopper and Clock control for DAC (0x013a)*/
pub const RT5659_CKXEN_DAC1_MASK: u32 = (0x1 << 13);
pub const RT5659_CKXEN_DAC1_SFT: u32 = 13;
pub const RT5659_CKGEN_DAC1_MASK: u32 = (0x1 << 12);
pub const RT5659_CKGEN_DAC1_SFT: u32 = 12;
pub const RT5659_CKXEN_DAC2_MASK: u32 = (0x1 << 5);
pub const RT5659_CKXEN_DAC2_SFT: u32 = 5;
pub const RT5659_CKGEN_DAC2_MASK: u32 = (0x1 << 4);
pub const RT5659_CKGEN_DAC2_SFT: u32 = 4;

/* Chopper and Clock control for ADC (0x013b)*/
pub const RT5659_CKXEN_ADC1_MASK: u32 = (0x1 << 13);
pub const RT5659_CKXEN_ADC1_SFT: u32 = 13;
pub const RT5659_CKGEN_ADC1_MASK: u32 = (0x1 << 12);
pub const RT5659_CKGEN_ADC1_SFT: u32 = 12;
pub const RT5659_CKXEN_ADC2_MASK: u32 = (0x1 << 5);
pub const RT5659_CKXEN_ADC2_SFT: u32 = 5;
pub const RT5659_CKGEN_ADC2_MASK: u32 = (0x1 << 4);
pub const RT5659_CKGEN_ADC2_SFT: u32 = 4;

/* Test Mode Control 1 (0x0145) */
pub const RT5659_AD2DA_LB_MASK: u32 = (0x1 << 9);
pub const RT5659_AD2DA_LB_SFT: u32 = 9;

/* Stereo Noise Gate Control 1 (0x0160) */
pub const RT5659_NG2_EN_MASK: u32 = (0x1 << 15);
pub const RT5659_NG2_EN: u32 = (0x1 << 15);
pub const RT5659_NG2_DIS: u32 = (0x0 << 15);

/* System Clock Source */
pub const RT5659_SCLK_S_MCLK: c_int = 0;
pub const RT5659_SCLK_S_PLL1: c_int = 1;
pub const RT5659_SCLK_S_RCCLK: c_int = 2;

/* PLL1 Source */
pub const RT5659_PLL1_S_MCLK: c_int = 0;
pub const RT5659_PLL1_S_BCLK1: c_int = 1;
pub const RT5659_PLL1_S_BCLK2: c_int = 2;
pub const RT5659_PLL1_S_BCLK3: c_int = 3;
pub const RT5659_PLL1_S_BCLK4: c_int = 4;

pub const RT5659_AIF1: c_int = 0;
pub const RT5659_AIF2: c_int = 1;
pub const RT5659_AIF3: c_int = 2;
pub const RT5659_AIF4: c_int = 3;
pub const RT5659_AIFS: c_int = 4;





#[repr(C)]
pub struct rt5659_pll_code {
    pub m_bp: bool,
    pub m_code: c_int,
    pub n_code: c_int,
    pub k_code: c_int,
}

#[repr(C)]
pub struct rt5659_priv {
    pub component: *mut snd_soc_component,
    pub pdata: rt5659_platform_data,
    pub regmap: *mut regmap,
    pub gpiod_ldo1_en: *mut gpio_desc,
    pub gpiod_reset: *mut gpio_desc,
    pub hs_jack: *mut snd_soc_jack,
    pub jack_detect_work: delayed_work,
    pub mclk: *mut clk,

    pub sysclk: c_int,
    pub sysclk_src: c_int,
    pub lrck: [c_int; RT5659_AIFS as usize],
    pub bclk: [c_int; RT5659_AIFS as usize],
    pub master: [c_int; RT5659_AIFS as usize],
    pub v_id: c_int,

    pub pll_src: c_int,
    pub pll_in: c_int,
    pub pll_out: c_int,

    pub jack_type: c_int,
    pub hda_hp_plugged: bool,
    pub hda_mic_plugged: bool,
}

unsafe extern "C" {
    pub fn rt5659_set_jack_detect(
        component: *mut snd_soc_component,
        hs_jack: *mut snd_soc_jack,
    ) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
