/* SPDX-License-Identifier: GPL-2.0 */
/*
 * rt1011.h -- RT1011 ALSA SoC amplifier component driver header
 *
 * Copyright(c) 2019 Realtek Semiconductor Corp.
 */


pub const RT1011_DEVICE_ID_NUM: u32 = 0x1011;

pub const RT1011_RESET: u32 = 0x0000;
pub const RT1011_CLK_1: u32 = 0x0002;
pub const RT1011_CLK_2: u32 = 0x0004;
pub const RT1011_CLK_3: u32 = 0x0006;
pub const RT1011_CLK_4: u32 = 0x0008;
pub const RT1011_PLL_1: u32 = 0x000a;
pub const RT1011_PLL_2: u32 = 0x000c;
pub const RT1011_SRC_1: u32 = 0x000e;
pub const RT1011_SRC_2: u32 = 0x0010;
pub const RT1011_SRC_3: u32 = 0x0012;
pub const RT1011_CLK_DET: u32 = 0x0020;
pub const RT1011_SIL_DET: u32 = 0x0022;
pub const RT1011_PRIV_INDEX: u32 = 0x006a;
pub const RT1011_PRIV_DATA: u32 = 0x006c;
pub const RT1011_CUSTOMER_ID: u32 = 0x0076;
pub const RT1011_FM_VER: u32 = 0x0078;
pub const RT1011_VERSION_ID: u32 = 0x007a;
pub const RT1011_VENDOR_ID: u32 = 0x007c;
pub const RT1011_DEVICE_ID: u32 = 0x007d;
pub const RT1011_DUM_RW_0: u32 = 0x00f0;
pub const RT1011_DUM_YUN: u32 = 0x00f2;
pub const RT1011_DUM_RW_1: u32 = 0x00f3;
pub const RT1011_DUM_RO: u32 = 0x00f4;
pub const RT1011_MAN_I2C_DEV: u32 = 0x0100;
pub const RT1011_DAC_SET_1: u32 = 0x0102;
pub const RT1011_DAC_SET_2: u32 = 0x0104;
pub const RT1011_DAC_SET_3: u32 = 0x0106;
pub const RT1011_ADC_SET: u32 = 0x0107;
pub const RT1011_ADC_SET_1: u32 = 0x0108;
pub const RT1011_ADC_SET_2: u32 = 0x010a;
pub const RT1011_ADC_SET_3: u32 = 0x010c;
pub const RT1011_ADC_SET_4: u32 = 0x010e;
pub const RT1011_ADC_SET_5: u32 = 0x0110;
pub const RT1011_TDM_TOTAL_SET: u32 = 0x0111;
pub const RT1011_TDM1_SET_TCON: u32 = 0x0112;
pub const RT1011_TDM1_SET_1: u32 = 0x0114;
pub const RT1011_TDM1_SET_2: u32 = 0x0116;
pub const RT1011_TDM1_SET_3: u32 = 0x0118;
pub const RT1011_TDM1_SET_4: u32 = 0x011a;
pub const RT1011_TDM1_SET_5: u32 = 0x011c;
pub const RT1011_TDM2_SET_1: u32 = 0x011e;
pub const RT1011_TDM2_SET_2: u32 = 0x0120;
pub const RT1011_TDM2_SET_3: u32 = 0x0122;
pub const RT1011_TDM2_SET_4: u32 = 0x0124;
pub const RT1011_TDM2_SET_5: u32 = 0x0126;
pub const RT1011_PWM_CAL: u32 = 0x0200;
pub const RT1011_MIXER_1: u32 = 0x0300;
pub const RT1011_MIXER_2: u32 = 0x0302;
pub const RT1011_ADRC_LIMIT: u32 = 0x0310;
pub const RT1011_A_PRO: u32 = 0x0311;
pub const RT1011_A_TIMING_1: u32 = 0x0313;
pub const RT1011_A_TIMING_2: u32 = 0x0314;
pub const RT1011_A_TEMP_SEN: u32 = 0x0316;
pub const RT1011_SPK_VOL_DET_1: u32 = 0x0319;
pub const RT1011_SPK_VOL_DET_2: u32 = 0x031a;
pub const RT1011_SPK_VOL_TEST_OUT: u32 = 0x031b;
pub const RT1011_VBAT_VOL_DET_1: u32 = 0x031c;
pub const RT1011_VBAT_VOL_DET_2: u32 = 0x031d;
pub const RT1011_VBAT_TEST_OUT_1: u32 = 0x031e;
pub const RT1011_VBAT_TEST_OUT_2: u32 = 0x031f;
pub const RT1011_VBAT_PROTECTION: u32 = 0x0320;
pub const RT1011_VBAT_DET: u32 = 0x0321;
pub const RT1011_POWER_1: u32 = 0x0322;
pub const RT1011_POWER_2: u32 = 0x0324;
pub const RT1011_POWER_3: u32 = 0x0326;
pub const RT1011_POWER_4: u32 = 0x0328;
pub const RT1011_POWER_5: u32 = 0x0329;
pub const RT1011_POWER_6: u32 = 0x032a;
pub const RT1011_POWER_7: u32 = 0x032b;
pub const RT1011_POWER_8: u32 = 0x032c;
pub const RT1011_POWER_9: u32 = 0x032d;
pub const RT1011_CLASS_D_POS: u32 = 0x032e;
pub const RT1011_BOOST_CON_1: u32 = 0x0330;
pub const RT1011_BOOST_CON_2: u32 = 0x0332;
pub const RT1011_ANALOG_CTRL: u32 = 0x0334;
pub const RT1011_POWER_SEQ: u32 = 0x0340;
pub const RT1011_SHORT_CIRCUIT_DET_1: u32 = 0x0508;
pub const RT1011_SHORT_CIRCUIT_DET_2: u32 = 0x050a;
pub const RT1011_SPK_TEMP_PROTECT_0: u32 = 0x050c;
pub const RT1011_SPK_TEMP_PROTECT_1: u32 = 0x050d;
pub const RT1011_SPK_TEMP_PROTECT_2: u32 = 0x050e;
pub const RT1011_SPK_TEMP_PROTECT_3: u32 = 0x050f;
pub const RT1011_SPK_TEMP_PROTECT_4: u32 = 0x0510;
pub const RT1011_SPK_TEMP_PROTECT_5: u32 = 0x0511;
pub const RT1011_SPK_TEMP_PROTECT_6: u32 = 0x0512;
pub const RT1011_SPK_TEMP_PROTECT_7: u32 = 0x0516;
pub const RT1011_SPK_TEMP_PROTECT_8: u32 = 0x0517;
pub const RT1011_SPK_TEMP_PROTECT_9: u32 = 0x0518;
pub const RT1011_SPK_PRO_DC_DET_1: u32 = 0x0519;
pub const RT1011_SPK_PRO_DC_DET_2: u32 = 0x051a;
pub const RT1011_SPK_PRO_DC_DET_3: u32 = 0x051b;
pub const RT1011_SPK_PRO_DC_DET_4: u32 = 0x051c;
pub const RT1011_SPK_PRO_DC_DET_5: u32 = 0x051d;
pub const RT1011_SPK_PRO_DC_DET_6: u32 = 0x051e;
pub const RT1011_SPK_PRO_DC_DET_7: u32 = 0x051f;
pub const RT1011_SPK_PRO_DC_DET_8: u32 = 0x0520;
pub const RT1011_SPL_1: u32 = 0x0521;
pub const RT1011_SPL_2: u32 = 0x0522;
pub const RT1011_SPL_3: u32 = 0x0524;
pub const RT1011_SPL_4: u32 = 0x0526;
pub const RT1011_THER_FOLD_BACK_1: u32 = 0x0528;
pub const RT1011_THER_FOLD_BACK_2: u32 = 0x052a;
pub const RT1011_EXCUR_PROTECT_1: u32 = 0x0530;
pub const RT1011_EXCUR_PROTECT_2: u32 = 0x0532;
pub const RT1011_EXCUR_PROTECT_3: u32 = 0x0534;
pub const RT1011_EXCUR_PROTECT_4: u32 = 0x0535;
pub const RT1011_BAT_GAIN_1: u32 = 0x0536;
pub const RT1011_BAT_GAIN_2: u32 = 0x0538;
pub const RT1011_BAT_GAIN_3: u32 = 0x053a;
pub const RT1011_BAT_GAIN_4: u32 = 0x053c;
pub const RT1011_BAT_GAIN_5: u32 = 0x053d;
pub const RT1011_BAT_GAIN_6: u32 = 0x053e;
pub const RT1011_BAT_GAIN_7: u32 = 0x053f;
pub const RT1011_BAT_GAIN_8: u32 = 0x0540;
pub const RT1011_BAT_GAIN_9: u32 = 0x0541;
pub const RT1011_BAT_GAIN_10: u32 = 0x0542;
pub const RT1011_BAT_GAIN_11: u32 = 0x0543;
pub const RT1011_BAT_RT_THMAX_1: u32 = 0x0544;
pub const RT1011_BAT_RT_THMAX_2: u32 = 0x0545;
pub const RT1011_BAT_RT_THMAX_3: u32 = 0x0546;
pub const RT1011_BAT_RT_THMAX_4: u32 = 0x0547;
pub const RT1011_BAT_RT_THMAX_5: u32 = 0x0548;
pub const RT1011_BAT_RT_THMAX_6: u32 = 0x0549;
pub const RT1011_BAT_RT_THMAX_7: u32 = 0x054a;
pub const RT1011_BAT_RT_THMAX_8: u32 = 0x054b;
pub const RT1011_BAT_RT_THMAX_9: u32 = 0x054c;
pub const RT1011_BAT_RT_THMAX_10: u32 = 0x054d;
pub const RT1011_BAT_RT_THMAX_11: u32 = 0x054e;
pub const RT1011_BAT_RT_THMAX_12: u32 = 0x054f;
pub const RT1011_SPREAD_SPECTURM: u32 = 0x0568;
pub const RT1011_PRO_GAIN_MODE: u32 = 0x056a;
pub const RT1011_RT_DRC_CROSS: u32 = 0x0600;
pub const RT1011_RT_DRC_HB_1: u32 = 0x0611;
pub const RT1011_RT_DRC_HB_2: u32 = 0x0612;
pub const RT1011_RT_DRC_HB_3: u32 = 0x0613;
pub const RT1011_RT_DRC_HB_4: u32 = 0x0614;
pub const RT1011_RT_DRC_HB_5: u32 = 0x0615;
pub const RT1011_RT_DRC_HB_6: u32 = 0x0616;
pub const RT1011_RT_DRC_HB_7: u32 = 0x0617;
pub const RT1011_RT_DRC_HB_8: u32 = 0x0618;
pub const RT1011_RT_DRC_BB_1: u32 = 0x0621;
pub const RT1011_RT_DRC_BB_2: u32 = 0x0622;
pub const RT1011_RT_DRC_BB_3: u32 = 0x0623;
pub const RT1011_RT_DRC_BB_4: u32 = 0x0624;
pub const RT1011_RT_DRC_BB_5: u32 = 0x0625;
pub const RT1011_RT_DRC_BB_6: u32 = 0x0626;
pub const RT1011_RT_DRC_BB_7: u32 = 0x0627;
pub const RT1011_RT_DRC_BB_8: u32 = 0x0628;
pub const RT1011_RT_DRC_POS_1: u32 = 0x0631;
pub const RT1011_RT_DRC_POS_2: u32 = 0x0632;
pub const RT1011_RT_DRC_POS_3: u32 = 0x0633;
pub const RT1011_RT_DRC_POS_4: u32 = 0x0634;
pub const RT1011_RT_DRC_POS_5: u32 = 0x0635;
pub const RT1011_RT_DRC_POS_6: u32 = 0x0636;
pub const RT1011_RT_DRC_POS_7: u32 = 0x0637;
pub const RT1011_RT_DRC_POS_8: u32 = 0x0638;
pub const RT1011_CROSS_BQ_SET_1: u32 = 0x0702;
pub const RT1011_CROSS_BQ_SET_2: u32 = 0x0704;
pub const RT1011_BQ_SET_0: u32 = 0x0706;
pub const RT1011_BQ_SET_1: u32 = 0x0708;
pub const RT1011_BQ_SET_2: u32 = 0x070a;
pub const RT1011_BQ_PRE_GAIN_28_16: u32 = 0x0710;
pub const RT1011_BQ_PRE_GAIN_15_0: u32 = 0x0711;
pub const RT1011_BQ_POST_GAIN_28_16: u32 = 0x0712;
pub const RT1011_BQ_POST_GAIN_15_0: u32 = 0x0713;

pub const RT1011_BQ_H0_28_16: u32 = 0x0720;
pub const RT1011_BQ_A2_15_0: u32 = 0x0729;
pub const RT1011_BQ_1_H0_28_16: u32 = 0x0730;
pub const RT1011_BQ_1_A2_15_0: u32 = 0x0739;
pub const RT1011_BQ_2_H0_28_16: u32 = 0x0740;
pub const RT1011_BQ_2_A2_15_0: u32 = 0x0749;
pub const RT1011_BQ_3_H0_28_16: u32 = 0x0750;
pub const RT1011_BQ_3_A2_15_0: u32 = 0x0759;
pub const RT1011_BQ_4_H0_28_16: u32 = 0x0760;
pub const RT1011_BQ_4_A2_15_0: u32 = 0x0769;
pub const RT1011_BQ_5_H0_28_16: u32 = 0x0770;
pub const RT1011_BQ_5_A2_15_0: u32 = 0x0779;
pub const RT1011_BQ_6_H0_28_16: u32 = 0x0780;
pub const RT1011_BQ_6_A2_15_0: u32 = 0x0789;
pub const RT1011_BQ_7_H0_28_16: u32 = 0x0790;
pub const RT1011_BQ_7_A2_15_0: u32 = 0x0799;
pub const RT1011_BQ_8_H0_28_16: u32 = 0x07a0;
pub const RT1011_BQ_8_A2_15_0: u32 = 0x07a9;
pub const RT1011_BQ_9_H0_28_16: u32 = 0x07b0;
pub const RT1011_BQ_9_A2_15_0: u32 = 0x07b9;
pub const RT1011_BQ_10_H0_28_16: u32 = 0x07c0;
pub const RT1011_BQ_10_A2_15_0: u32 = 0x07c9;
pub const RT1011_TEST_PAD_STATUS: u32 = 0x1000;
pub const RT1011_SYSTEM_RESET_1: u32 = 0x1007;
pub const RT1011_SYSTEM_RESET_2: u32 = 0x1008;
pub const RT1011_SYSTEM_RESET_3: u32 = 0x1009;
pub const RT1011_ADCDAT_OUT_SOURCE: u32 = 0x100D;
pub const RT1011_PLL_INTERNAL_SET: u32 = 0x1010;
pub const RT1011_TEST_OUT_1: u32 = 0x1020;
pub const RT1011_TEST_OUT_3: u32 = 0x1024;
pub const RT1011_DC_CALIB_CLASSD_1: u32 = 0x1200;
pub const RT1011_DC_CALIB_CLASSD_2: u32 = 0x1202;
pub const RT1011_DC_CALIB_CLASSD_3: u32 = 0x1204;
pub const RT1011_DC_CALIB_CLASSD_5: u32 = 0x1208;
pub const RT1011_DC_CALIB_CLASSD_6: u32 = 0x120a;
pub const RT1011_DC_CALIB_CLASSD_7: u32 = 0x120c;
pub const RT1011_DC_CALIB_CLASSD_8: u32 = 0x120e;
pub const RT1011_DC_CALIB_CLASSD_10: u32 = 0x1212;
pub const RT1011_CLASSD_INTERNAL_SET_1: u32 = 0x1300;
pub const RT1011_CLASSD_INTERNAL_SET_3: u32 = 0x1304;
pub const RT1011_CLASSD_INTERNAL_SET_8: u32 = 0x130c;
pub const RT1011_VREF_LV_1: u32 = 0x131a;
pub const RT1011_SMART_BOOST_TIMING_1: u32 = 0x1322;
pub const RT1011_SMART_BOOST_TIMING_36: u32 = 0x1349;
pub const RT1011_SINE_GEN_REG_1: u32 = 0x1500;
pub const RT1011_SINE_GEN_REG_2: u32 = 0x1502;
pub const RT1011_SINE_GEN_REG_3: u32 = 0x1504;
pub const RT1011_STP_INITIAL_RS_TEMP: u32 = 0x1510;
pub const RT1011_STP_CALIB_RS_TEMP: u32 = 0x152a;
pub const RT1011_INIT_RECIPROCAL_REG_24_16: u32 = 0x1538;
pub const RT1011_INIT_RECIPROCAL_REG_15_0: u32 = 0x1539;
pub const RT1011_STP_INITIAL_RESISTANCE_TEMP: u32 = 0x153c;
pub const RT1011_STP_ALPHA_RECIPROCAL_MSB: u32 = 0x153e;
pub const RT1011_SPK_RESISTANCE_1: u32 = 0x1544;
pub const RT1011_SPK_RESISTANCE_2: u32 = 0x1546;
pub const RT1011_SPK_THERMAL: u32 = 0x1548;
pub const RT1011_STP_OTP_TH: u32 = 0x1552;
pub const RT1011_ALC_BK_GAIN_O: u32 = 0x1554;
pub const RT1011_ALC_BK_GAIN_O_PRE: u32 = 0x1556;
pub const RT1011_SPK_DC_O_23_16: u32 = 0x155a;
pub const RT1011_SPK_DC_O_15_0: u32 = 0x155c;
pub const RT1011_INIT_RECIPROCAL_SYN_24_16: u32 = 0x1560;
pub const RT1011_INIT_RECIPROCAL_SYN_15_0: u32 = 0x1562;
pub const RT1011_STP_BQ_1_A1_L_28_16: u32 = 0x1570;
pub const RT1011_STP_BQ_1_H0_R_15_0: u32 = 0x1583;
pub const RT1011_STP_BQ_2_A1_L_28_16: u32 = 0x1590;
pub const RT1011_SPK_EXCURSION_23_16: u32 = 0x15be;
pub const RT1011_SPK_EXCURSION_15_0: u32 = 0x15bf;
pub const RT1011_SEP_MAIN_OUT_23_16: u32 = 0x15c0;
pub const RT1011_SEP_MAIN_OUT_15_0: u32 = 0x15c1;
pub const RT1011_SEP_RE_REG_15_0: u32 = 0x15f9;
pub const RT1011_DRC_CF_PARAMS_1: u32 = 0x1600;
pub const RT1011_DRC_CF_PARAMS_12: u32 = 0x160b;
pub const RT1011_ALC_DRC_HB_INTERNAL_1: u32 = 0x1611;
pub const RT1011_ALC_DRC_HB_INTERNAL_5: u32 = 0x1615;
pub const RT1011_ALC_DRC_HB_INTERNAL_6: u32 = 0x1616;
pub const RT1011_ALC_DRC_HB_INTERNAL_7: u32 = 0x1617;
pub const RT1011_ALC_DRC_BB_INTERNAL_1: u32 = 0x1621;
pub const RT1011_ALC_DRC_BB_INTERNAL_5: u32 = 0x1625;
pub const RT1011_ALC_DRC_BB_INTERNAL_6: u32 = 0x1626;
pub const RT1011_ALC_DRC_BB_INTERNAL_7: u32 = 0x1627;
pub const RT1011_ALC_DRC_POS_INTERNAL_1: u32 = 0x1631;
pub const RT1011_ALC_DRC_POS_INTERNAL_5: u32 = 0x1635;
pub const RT1011_ALC_DRC_POS_INTERNAL_6: u32 = 0x1636;
pub const RT1011_ALC_DRC_POS_INTERNAL_7: u32 = 0x1637;
pub const RT1011_ALC_DRC_POS_INTERNAL_8: u32 = 0x1638;
pub const RT1011_ALC_DRC_POS_INTERNAL_9: u32 = 0x163a;
pub const RT1011_ALC_DRC_POS_INTERNAL_10: u32 = 0x163c;
pub const RT1011_ALC_DRC_POS_INTERNAL_11: u32 = 0x163e;
pub const RT1011_BQ_1_PARAMS_CHECK_5: u32 = 0x1648;
pub const RT1011_BQ_2_PARAMS_CHECK_1: u32 = 0x1650;
pub const RT1011_BQ_2_PARAMS_CHECK_5: u32 = 0x1658;
pub const RT1011_BQ_3_PARAMS_CHECK_1: u32 = 0x1660;
pub const RT1011_BQ_3_PARAMS_CHECK_5: u32 = 0x1668;
pub const RT1011_BQ_4_PARAMS_CHECK_1: u32 = 0x1670;
pub const RT1011_BQ_4_PARAMS_CHECK_5: u32 = 0x1678;
pub const RT1011_BQ_5_PARAMS_CHECK_1: u32 = 0x1680;
pub const RT1011_BQ_5_PARAMS_CHECK_5: u32 = 0x1688;
pub const RT1011_BQ_6_PARAMS_CHECK_1: u32 = 0x1690;
pub const RT1011_BQ_6_PARAMS_CHECK_5: u32 = 0x1698;
pub const RT1011_BQ_7_PARAMS_CHECK_1: u32 = 0x1700;
pub const RT1011_BQ_7_PARAMS_CHECK_5: u32 = 0x1708;
pub const RT1011_BQ_8_PARAMS_CHECK_1: u32 = 0x1710;
pub const RT1011_BQ_8_PARAMS_CHECK_5: u32 = 0x1718;
pub const RT1011_BQ_9_PARAMS_CHECK_1: u32 = 0x1720;
pub const RT1011_BQ_9_PARAMS_CHECK_5: u32 = 0x1728;
pub const RT1011_BQ_10_PARAMS_CHECK_1: u32 = 0x1730;
pub const RT1011_BQ_10_PARAMS_CHECK_5: u32 = 0x1738;
pub const RT1011_IRQ_1: u32 = 0x173a;
pub const RT1011_PART_NUMBER_EFUSE: u32 = 0x173e;
pub const RT1011_EFUSE_CONTROL_1: u32 = 0x17bb;
pub const RT1011_EFUSE_CONTROL_2: u32 = 0x17bd;
pub const RT1011_EFUSE_MATCH_DONE: u32 = 0x17cb;
pub const RT1011_EFUSE_ADC_OFFSET_18_16: u32 = 0x17e5;
pub const RT1011_EFUSE_ADC_OFFSET_15_0: u32 = 0x17e7;
pub const RT1011_EFUSE_DAC_OFFSET_G0_20_16: u32 = 0x17e9;
pub const RT1011_EFUSE_DAC_OFFSET_G0_15_0: u32 = 0x17eb;
pub const RT1011_EFUSE_DAC_OFFSET_G1_20_16: u32 = 0x17ed;
pub const RT1011_EFUSE_DAC_OFFSET_G1_15_0: u32 = 0x17ef;
pub const RT1011_EFUSE_READ_R0_3_15_0: u32 = 0x1803;
pub const RT1011_MAX_REG: u32 = 0x1803;
pub const RT1011_REG_DISP_LEN: u32 = 23;


/* CLOCK-2 (0x0004) */
pub const RT1011_FS_SYS_PRE_MASK: u32 = (0x3 << 14);
pub const RT1011_FS_SYS_PRE_SFT: u32 = 14;
pub const RT1011_FS_SYS_PRE_MCLK: u32 = (0x0 << 14);
pub const RT1011_FS_SYS_PRE_BCLK: u32 = (0x1 << 14);
pub const RT1011_FS_SYS_PRE_PLL1: u32 = (0x2 << 14);
pub const RT1011_FS_SYS_PRE_RCCLK: u32 = (0x3 << 14);
pub const RT1011_PLL1_SRC_MASK: u32 = (0x1 << 13);
pub const RT1011_PLL1_SRC_SFT: u32 = 13;
pub const RT1011_PLL1_SRC_PLL2: u32 = (0x0 << 13);
pub const RT1011_PLL1_SRC_BCLK: u32 = (0x1 << 13);
pub const RT1011_PLL2_SRC_MASK: u32 = (0x1 << 12);
pub const RT1011_PLL2_SRC_SFT: u32 = 12;
pub const RT1011_PLL2_SRC_MCLK: u32 = (0x0 << 12);
pub const RT1011_PLL2_SRC_RCCLK: u32 = (0x1 << 12);
pub const RT1011_PLL2_SRC_DIV_MASK: u32 = (0x3 << 10);
pub const RT1011_PLL2_SRC_DIV_SFT: u32 = 10;
pub const RT1011_SRCIN_DIV_MASK: u32 = (0x3 << 8);
pub const RT1011_SRCIN_DIV_SFT: u32 = 8;
pub const RT1011_FS_SYS_DIV_MASK: u32 = (0x7 << 4);
pub const RT1011_FS_SYS_DIV_SFT: u32 = 4;

/* PLL-1 (0x000a) */
pub const RT1011_PLL1_QM_MASK: u32 = (0xf << 12);
pub const RT1011_PLL1_QM_SFT: u32 = 12;
pub const RT1011_PLL1_BPM_MASK: u32 = (0x1 << 11);
pub const RT1011_PLL1_BPM_SFT: u32 = 11;
pub const RT1011_PLL1_BPM: u32 = (0x1 << 11);
pub const RT1011_PLL1_QN_MASK: u32 = (0x1ff << 0);
pub const RT1011_PLL1_QN_SFT: u32 = 0;

/* PLL-2 (0x000c) */
pub const RT1011_PLL2_BPK_MASK: u32 = (0x1 << 5);
pub const RT1011_PLL2_BPK_SFT: u32 = 5;
pub const RT1011_PLL2_BPK: u32 = (0x1 << 5);
pub const RT1011_PLL2_QK_MASK: u32 = (0x1f << 0);
pub const RT1011_PLL2_QK_SFT: u32 = 0;

/* Clock Detect (0x0020) */
pub const RT1011_EN_MCLK_DET_MASK: u32 = (0x1 << 15);
pub const RT1011_EN_MCLK_DET_SFT: u32 = 15;
pub const RT1011_EN_MCLK_DET: u32 = (0x1 << 15);

/* DAC Setting-2 (0x0104) */
pub const RT1011_EN_CKGEN_DAC_MASK: u32 = (0x1 << 13);
pub const RT1011_EN_CKGEN_DAC_SFT: u32 = 13;
pub const RT1011_EN_CKGEN_DAC: u32 = (0x1 << 13);

/* DAC Setting-3 (0x0106) */
pub const RT1011_DA_MUTE_EN_MASK: u32 = (0x1 << 15);
pub const RT1011_DA_MUTE_EN_SFT: u32 = 15;

/* ADC Setting-5 (0x0110) */
pub const RT1011_AD_EN_CKGEN_ADC_MASK: u32 = (0x1 << 9);
pub const RT1011_AD_EN_CKGEN_ADC_SFT: u32 = 9;
pub const RT1011_AD_EN_CKGEN_ADC: u32 = (0x1 << 9);

/* TDM Total Setting (0x0111) */
pub const RT1011_I2S_TDM_MS_MASK: u32 = (0x1 << 14);
pub const RT1011_I2S_TDM_MS_SFT: u32 = 14;
pub const RT1011_I2S_TDM_MS_S: u32 = (0x0 << 14);
pub const RT1011_I2S_TDM_MS_M: u32 = (0x1 << 14);
pub const RT1011_I2S_TX_DL_MASK: u32 = (0x7 << 8);
pub const RT1011_I2S_TX_DL_SFT: u32 = 8;
pub const RT1011_I2S_TX_DL_16B: u32 = (0x0 << 8);
pub const RT1011_I2S_TX_DL_20B: u32 = (0x1 << 8);
pub const RT1011_I2S_TX_DL_24B: u32 = (0x2 << 8);
pub const RT1011_I2S_TX_DL_32B: u32 = (0x3 << 8);
pub const RT1011_I2S_TX_DL_8B: u32 = (0x4 << 8);
pub const RT1011_I2S_RX_DL_MASK: u32 = (0x7 << 5);
pub const RT1011_I2S_RX_DL_SFT: u32 = 5;
pub const RT1011_I2S_RX_DL_16B: u32 = (0x0 << 5);
pub const RT1011_I2S_RX_DL_20B: u32 = (0x1 << 5);
pub const RT1011_I2S_RX_DL_24B: u32 = (0x2 << 5);
pub const RT1011_I2S_RX_DL_32B: u32 = (0x3 << 5);
pub const RT1011_I2S_RX_DL_8B: u32 = (0x4 << 5);
pub const RT1011_ADCDAT1_PIN_CONFIG: u32 = (0x1 << 4);
pub const RT1011_ADCDAT1_OUTPUT: u32 = (0x0 << 4);
pub const RT1011_ADCDAT1_INPUT: u32 = (0x1 << 4);
pub const RT1011_ADCDAT2_PIN_CONFIG: u32 = (0x1 << 3);
pub const RT1011_ADCDAT2_OUTPUT: u32 = (0x0 << 3);
pub const RT1011_ADCDAT2_INPUT: u32 = (0x1 << 3);
pub const RT1011_I2S_TDM_DF_MASK: u32 = (0x7 << 0);
pub const RT1011_I2S_TDM_DF_SFT: u32 = 0;
pub const RT1011_I2S_TDM_DF_I2S: u32 = (0x0);
pub const RT1011_I2S_TDM_DF_LEFT: u32 = (0x1);
pub const RT1011_I2S_TDM_DF_PCM_A: u32 = (0x2);
pub const RT1011_I2S_TDM_DF_PCM_B: u32 = (0x3);
pub const RT1011_I2S_TDM_DF_PCM_A_N: u32 = (0x6);
pub const RT1011_I2S_TDM_DF_PCM_B_N: u32 = (0x7);

/* TDM_tcon Setting (0x0112) */
pub const RT1011_TCON_DF_MASK: u32 = (0x7 << 13);
pub const RT1011_TCON_DF_SFT: u32 = 13;
pub const RT1011_TCON_DF_I2S: u32 = (0x0 << 13);
pub const RT1011_TCON_DF_LEFT: u32 = (0x1 << 13);
pub const RT1011_TCON_DF_PCM_A: u32 = (0x2 << 13);
pub const RT1011_TCON_DF_PCM_B: u32 = (0x3 << 13);
pub const RT1011_TCON_DF_PCM_A_N: u32 = (0x6 << 13);
pub const RT1011_TCON_DF_PCM_B_N: u32 = (0x7 << 13);
pub const RT1011_TCON_BCLK_SEL_MASK: u32 = (0x3 << 10);
pub const RT1011_TCON_BCLK_SEL_SFT: u32 = 10;
pub const RT1011_TCON_BCLK_SEL_32FS: u32 = (0x0 << 10);
pub const RT1011_TCON_BCLK_SEL_64FS: u32 = (0x1 << 10);
pub const RT1011_TCON_BCLK_SEL_128FS: u32 = (0x2 << 10);
pub const RT1011_TCON_BCLK_SEL_256FS: u32 = (0x3 << 10);
pub const RT1011_TCON_CH_LEN_MASK: u32 = (0x3 << 5);
pub const RT1011_TCON_CH_LEN_SFT: u32 = 5;
pub const RT1011_TCON_CH_LEN_16B: u32 = (0x0 << 5);
pub const RT1011_TCON_CH_LEN_20B: u32 = (0x1 << 5);
pub const RT1011_TCON_CH_LEN_24B: u32 = (0x2 << 5);
pub const RT1011_TCON_CH_LEN_32B: u32 = (0x3 << 5);
pub const RT1011_TCON_BCLK_MST_MASK: u32 = (0x1 << 4);
pub const RT1011_TCON_BCLK_MST_SFT: u32 = 4;
pub const RT1011_TCON_BCLK_MST_INV: u32 = (0x1 << 4);

/* TDM1 Setting-1 (0x0114) */
pub const RT1011_TDM_INV_BCLK_MASK: u32 = (0x1 << 15);
pub const RT1011_TDM_INV_BCLK_SFT: u32 = 15;
pub const RT1011_TDM_INV_BCLK: u32 = (0x1 << 15);
pub const RT1011_I2S_CH_TX_MASK: u32 = (0x3 << 10);
pub const RT1011_I2S_CH_TX_SFT: u32 = 10;
pub const RT1011_I2S_TX_2CH: u32 = (0x0 << 10);
pub const RT1011_I2S_TX_4CH: u32 = (0x1 << 10);
pub const RT1011_I2S_TX_6CH: u32 = (0x2 << 10);
pub const RT1011_I2S_TX_8CH: u32 = (0x3 << 10);
pub const RT1011_I2S_CH_RX_MASK: u32 = (0x3 << 8);
pub const RT1011_I2S_CH_RX_SFT: u32 = 8;
pub const RT1011_I2S_RX_2CH: u32 = (0x0 << 8);
pub const RT1011_I2S_RX_4CH: u32 = (0x1 << 8);
pub const RT1011_I2S_RX_6CH: u32 = (0x2 << 8);
pub const RT1011_I2S_RX_8CH: u32 = (0x3 << 8);
pub const RT1011_I2S_LR_CH_SEL_MASK: u32 = (0x1 << 7);
pub const RT1011_I2S_LR_CH_SEL_SFT: u32 = 7;
pub const RT1011_I2S_LEFT_CH_SEL: u32 = (0x0 << 7);
pub const RT1011_I2S_RIGHT_CH_SEL: u32 = (0x1 << 7);
pub const RT1011_I2S_CH_TX_LEN_MASK: u32 = (0x7 << 4);
pub const RT1011_I2S_CH_TX_LEN_SFT: u32 = 4;
pub const RT1011_I2S_CH_TX_LEN_16B: u32 = (0x0 << 4);
pub const RT1011_I2S_CH_TX_LEN_20B: u32 = (0x1 << 4);
pub const RT1011_I2S_CH_TX_LEN_24B: u32 = (0x2 << 4);
pub const RT1011_I2S_CH_TX_LEN_32B: u32 = (0x3 << 4);
pub const RT1011_I2S_CH_TX_LEN_8B: u32 = (0x4 << 4);
pub const RT1011_I2S_CH_RX_LEN_MASK: u32 = (0x7 << 0);
pub const RT1011_I2S_CH_RX_LEN_SFT: u32 = 0;
pub const RT1011_I2S_CH_RX_LEN_16B: u32 = (0x0 << 0);
pub const RT1011_I2S_CH_RX_LEN_20B: u32 = (0x1 << 0);
pub const RT1011_I2S_CH_RX_LEN_24B: u32 = (0x2 << 0);
pub const RT1011_I2S_CH_RX_LEN_32B: u32 = (0x3 << 0);
pub const RT1011_I2S_CH_RX_LEN_8B: u32 = (0x4 << 0);

/* TDM1 Setting-2 (0x0116) */
pub const RT1011_TDM_I2S_DOCK_ADCDAT_LEN_1_MASK: u32 = (0x7 << 13);
pub const RT1011_TDM_I2S_DOCK_ADCDAT_2CH: u32 = (0x1 << 13);
pub const RT1011_TDM_I2S_DOCK_ADCDAT_4CH: u32 = (0x3 << 13);
pub const RT1011_TDM_I2S_DOCK_ADCDAT_6CH: u32 = (0x5 << 13);
pub const RT1011_TDM_I2S_DOCK_ADCDAT_8CH: u32 = (0x7 << 13);
pub const RT1011_TDM_I2S_DOCK_EN_1_MASK: u32 = (0x1 << 3);
pub const RT1011_TDM_I2S_DOCK_EN_1_SFT: u32 = 3;
pub const RT1011_TDM_I2S_DOCK_EN_1: u32 = (0x1 << 3);
pub const RT1011_TDM_ADCDAT1_DATA_LOCATION: u32 = (0x7 << 0);

/* TDM1 Setting-3 (0x0118) */
pub const RT1011_TDM_I2S_RX_ADC1_1_MASK: u32 = (0x3 << 6);
pub const RT1011_TDM_I2S_RX_ADC2_1_MASK: u32 = (0x3 << 4);
pub const RT1011_TDM_I2S_RX_ADC3_1_MASK: u32 = (0x3 << 2);
pub const RT1011_TDM_I2S_RX_ADC4_1_MASK: u32 = (0x3 << 0);
pub const RT1011_TDM_I2S_RX_ADC1_1_LL: u32 = (0x2 << 6);
pub const RT1011_TDM_I2S_RX_ADC2_1_LL: u32 = (0x2 << 4);
pub const RT1011_TDM_I2S_RX_ADC3_1_LL: u32 = (0x2 << 2);
pub const RT1011_TDM_I2S_RX_ADC4_1_LL: u32 = (0x2 << 0);

/* TDM1 Setting-4 (0x011a) */
pub const RT1011_TDM_I2S_TX_L_DAC1_1_MASK: u32 = (0x7 << 12);
pub const RT1011_TDM_I2S_TX_R_DAC1_1_MASK: u32 = (0x7 << 8);
pub const RT1011_TDM_I2S_TX_L_DAC1_1_SFT: u32 = 12;
pub const RT1011_TDM_I2S_TX_R_DAC1_1_SFT: u32 = 8;

/* TDM2 Setting-2 (0x0120) */
pub const RT1011_TDM_I2S_DOCK_ADCDAT_LEN_2_MASK: u32 = (0x7 << 13);
pub const RT1011_TDM_I2S_DOCK_EN_2_MASK: u32 = (0x1 << 3);
pub const RT1011_TDM_I2S_DOCK_EN_2_SFT: u32 = 3;
pub const RT1011_TDM_I2S_DOCK_EN_2: u32 = (0x1 << 3);

/* MIXER 1 (0x0300) */
pub const RT1011_MIXER_MUTE_MIX_I_MASK: u32 = (0x1 << 15);
pub const RT1011_MIXER_MUTE_MIX_I_SFT: u32 = 15;
pub const RT1011_MIXER_MUTE_MIX_I: u32 = (0x1 << 15);
pub const RT1011_MIXER_MUTE_SUM_I_MASK: u32 = (0x1 << 14);
pub const RT1011_MIXER_MUTE_SUM_I_SFT: u32 = 14;
pub const RT1011_MIXER_MUTE_SUM_I: u32 = (0x1 << 14);
pub const RT1011_MIXER_MUTE_MIX_V_MASK: u32 = (0x1 << 7);
pub const RT1011_MIXER_MUTE_MIX_V_SFT: u32 = 7;
pub const RT1011_MIXER_MUTE_MIX_V: u32 = (0x1 << 7);
pub const RT1011_MIXER_MUTE_SUM_V_MASK: u32 = (0x1 << 6);
pub const RT1011_MIXER_MUTE_SUM_V_SFT: u32 = 6;
pub const RT1011_MIXER_MUTE_SUM_V: u32 = (0x1 << 6);

/* Analog Temperature Sensor (0x0316) */
pub const RT1011_POW_TEMP_REG: u32 = (0x1 << 2);
pub const RT1011_POW_TEMP_REG_BIT: u32 = 2;

/* POWER-1 (0x0322) */
pub const RT1011_POW_LDO2: u32 = (0x1 << 15);
pub const RT1011_POW_LDO2_BIT: u32 = 15;
pub const RT1011_POW_DAC: u32 = (0x1 << 14);
pub const RT1011_POW_DAC_BIT: u32 = 14;
pub const RT1011_POW_CLK12M: u32 = (0x1 << 13);
pub const RT1011_POW_CLK12M_BIT: u32 = 13;
pub const RT1011_POW_TEMP: u32 = (0x1 << 12);
pub const RT1011_POW_TEMP_BIT: u32 = 12;
pub const RT1011_POW_ISENSE_SPK: u32 = (0x1 << 7);
pub const RT1011_POW_ISENSE_SPK_BIT: u32 = 7;
pub const RT1011_POW_LPF_SPK: u32 = (0x1 << 6);
pub const RT1011_POW_LPF_SPK_BIT: u32 = 6;
pub const RT1011_POW_VSENSE_SPK: u32 = (0x1 << 5);
pub const RT1011_POW_VSENSE_SPK_BIT: u32 = 5;
pub const RT1011_POW_TWO_BATTERY_SPK: u32 = (0x1 << 4);
pub const RT1011_POW_TWO_BATTERY_SPK_BIT: u32 = 4;

/* POWER-2 (0x0324) */
pub const RT1011_PLLEN: u32 = (0x1 << 2);
pub const RT1011_PLLEN_BIT: u32 = 2;
pub const RT1011_POW_BG: u32 = (0x1 << 1);
pub const RT1011_POW_BG_BIT: u32 = 1;
pub const RT1011_POW_BG_MBIAS_LV: u32 = (0x1 << 0);
pub const RT1011_POW_BG_MBIAS_LV_BIT: u32 = 0;

/* POWER-3 (0x0326) */
pub const RT1011_POW_DET_SPKVDD: u32 = (0x1 << 15);
pub const RT1011_POW_DET_SPKVDD_BIT: u32 = 15;
pub const RT1011_POW_DET_VBAT: u32 = (0x1 << 14);
pub const RT1011_POW_DET_VBAT_BIT: u32 = 14;
pub const RT1011_POW_FC: u32 = (0x1 << 13);
pub const RT1011_POW_FC_BIT: u32 = 13;
pub const RT1011_POW_MBIAS_LV: u32 = (0x1 << 12);
pub const RT1011_POW_MBIAS_LV_BIT: u32 = 12;
pub const RT1011_POW_ADC_I: u32 = (0x1 << 11);
pub const RT1011_POW_ADC_I_BIT: u32 = 11;
pub const RT1011_POW_ADC_V: u32 = (0x1 << 10);
pub const RT1011_POW_ADC_V_BIT: u32 = 10;
pub const RT1011_POW_ADC_T: u32 = (0x1 << 9);
pub const RT1011_POW_ADC_T_BIT: u32 = 9;
pub const RT1011_POWD_ADC_T: u32 = (0x1 << 8);
pub const RT1011_POWD_ADC_T_BIT: u32 = 8;
pub const RT1011_POW_MIX_I: u32 = (0x1 << 7);
pub const RT1011_POW_MIX_I_BIT: u32 = 7;
pub const RT1011_POW_MIX_V: u32 = (0x1 << 6);
pub const RT1011_POW_MIX_V_BIT: u32 = 6;
pub const RT1011_POW_SUM_I: u32 = (0x1 << 5);
pub const RT1011_POW_SUM_I_BIT: u32 = 5;
pub const RT1011_POW_SUM_V: u32 = (0x1 << 4);
pub const RT1011_POW_SUM_V_BIT: u32 = 4;
pub const RT1011_POW_MIX_T: u32 = (0x1 << 2);
pub const RT1011_POW_MIX_T_BIT: u32 = 2;
pub const RT1011_BYPASS_MIX_T: u32 = (0x1 << 1);
pub const RT1011_BYPASS_MIX_T_BIT: u32 = 1;
pub const RT1011_POW_VREF_LV: u32 = (0x1 << 0);
pub const RT1011_POW_VREF_LV_BIT: u32 = 0;

/* POWER-4 (0x0328) */
pub const RT1011_POW_EN_SWR: u32 = (0x1 << 12);
pub const RT1011_POW_EN_SWR_BIT: u32 = 12;
pub const RT1011_POW_EN_PASS_BGOK_SWR: u32 = (0x1 << 10);
pub const RT1011_POW_EN_PASS_BGOK_SWR_BIT: u32 = 10;
pub const RT1011_POW_EN_PASS_VPOK_SWR: u32 = (0x1 << 9);
pub const RT1011_POW_EN_PASS_VPOK_SWR_BIT: u32 = 9;

/* POWER-9 (0x032d) */
pub const RT1011_POW_SDB_REG_MASK: u32 = (0x1 << 9);
pub const RT1011_POW_SDB_REG_BIT: u32 = 9;
pub const RT1011_POW_SDB_REG: u32 = (0x1 << 9);
pub const RT1011_POW_SEL_SDB_MODE_MASK: u32 = (0x1 << 6);
pub const RT1011_POW_SEL_SDB_MODE_BIT: u32 = 6;
pub const RT1011_POW_SEL_SDB_MODE: u32 = (0x1 << 6);
pub const RT1011_POW_MNL_SDB_MASK: u32 = (0x1 << 5);
pub const RT1011_POW_MNL_SDB_BIT: u32 = 5;
pub const RT1011_POW_MNL_SDB: u32 = (0x1 << 5);

/* SPK Protection-Temperature Protection (0x050c) */
pub const RT1011_STP_EN_MASK: u32 = (0x1 << 15);
pub const RT1011_STP_EN_BIT: u32 = 15;
pub const RT1011_STP_EN: u32 = (0x1 << 15);
pub const RT1011_STP_RS_CLB_EN_MASK: u32 = (0x1 << 14);
pub const RT1011_STP_RS_CLB_EN_BIT: u32 = 14;
pub const RT1011_STP_RS_CLB_EN: u32 = (0x1 << 14);

/* SPK Protection-Temperature Protection-4 (0x0510) */
pub const RT1011_STP_R0_SELECT_MASK: u32 = (0x3 << 6);
pub const RT1011_STP_R0_SELECT_EFUSE: u32 = (0x0 << 6);
pub const RT1011_STP_R0_SELECT_START_VAL: u32 = (0x1 << 6);
pub const RT1011_STP_R0_SELECT_REG: u32 = (0x2 << 6);
pub const RT1011_STP_R0_SELECT_FORCE_ZERO: u32 = (0x3 << 6);

/* SPK Protection-Temperature Protection-6 (0x0512) */
pub const RT1011_STP_R0_EN_MASK: u32 = (0x1 << 7);
pub const RT1011_STP_R0_EN_BIT: u32 = 7;
pub const RT1011_STP_R0_EN: u32 = (0x1 << 7);
pub const RT1011_STP_T0_EN_MASK: u32 = (0x1 << 6);
pub const RT1011_STP_T0_EN_BIT: u32 = 6;
pub const RT1011_STP_T0_EN: u32 = (0x1 << 6);

/* Cross Biquad Setting-1 (0x0702) */
pub const RT1011_MONO_LR_SEL_MASK: u32 = (0x3 << 5);
pub const RT1011_MONO_L_CHANNEL: u32 = (0x0 << 5);
pub const RT1011_MONO_R_CHANNEL: u32 = (0x1 << 5);
pub const RT1011_MONO_LR_MIX_CHANNEL: u32 = (0x2 << 5);

/* ClassD Internal Setting-1 (0x1300) */
pub const RT1011_DRIVER_READY_SPK: u32 = (0x1 << 12);
pub const RT1011_DRIVER_READY_SPK_BIT: u32 = 12;
pub const RT1011_RECV_MODE_SPK_MASK: u32 = (0x1 << 5);
pub const RT1011_SPK_MODE: u32 = (0x0 << 5);
pub const RT1011_RECV_MODE: u32 = (0x1 << 5);
pub const RT1011_RECV_MODE_SPK_BIT: u32 = 5;

/* ClassD Internal Setting-3 (0x1304) */
pub const RT1011_REG_GAIN_CLASSD_RI_SPK_MASK: u32 = (0x7 << 12);
pub const RT1011_REG_GAIN_CLASSD_RI_410K: u32 = (0x0 << 12);
pub const RT1011_REG_GAIN_CLASSD_RI_95K: u32 = (0x1 << 12);
pub const RT1011_REG_GAIN_CLASSD_RI_82P5K: u32 = (0x2 << 12);
pub const RT1011_REG_GAIN_CLASSD_RI_72P5K: u32 = (0x3 << 12);
pub const RT1011_REG_GAIN_CLASSD_RI_62P5K: u32 = (0x4 << 12);

/* ClassD Internal Setting-8 (0x130c) */
pub const RT1011_TM_PORPVDD_SPK: u32 = (0x1 << 1);
pub const RT1011_TM_PORPVDD_SPK_BIT: u32 = 1;

/* SPK Protection-Temperature Protection-SINE_GEN_REG-1 (0x1500) */
pub const RT1011_STP_SIN_GEN_EN_MASK: u32 = (0x1 << 13);
pub const RT1011_STP_SIN_GEN_EN: u32 = (0x1 << 13);
pub const RT1011_STP_SIN_GEN_EN_BIT: u32 = 13;


/* System Clock Source */

/* System Clock Source */
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Rt1011FsSysPreSource {
    RT1011_FS_SYS_PRE_S_MCLK = 0,
    RT1011_FS_SYS_PRE_S_BCLK = 1,
    RT1011_FS_SYS_PRE_S_PLL1 = 2,
    RT1011_FS_SYS_PRE_S_RCCLK = 3, /* 12M Hz */
}

/* PLL Source 1/2 */
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Rt1011PllSource {
    RT1011_PLL1_S_BCLK = 0,
    RT1011_PLL2_S_MCLK = 1,
    RT1011_PLL2_S_RCCLK = 2, /* 12M Hz */
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Rt1011Aif {
    RT1011_AIF1 = 0,
    RT1011_AIFS = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Rt1011I2sRef {
    RT1011_I2S_REF_NONE = 0,
    RT1011_I2S_REF_LEFT_CH = 1,
    RT1011_I2S_REF_RIGHT_CH = 2,
}

/* BiQual & DRC related settings */
pub const RT1011_BQ_DRC_NUM: u32 = 128;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rt1011_bq_drc_params {
    pub val: u16,
    pub reg: u16,
    /* Present only when CONFIG_64BIT is enabled in the C build. */
    #[cfg(target_pointer_width = "64")]
    pub reserved: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Rt1011Advmode {
    RT1011_ADVMODE_INITIAL_SET = 0,
    RT1011_ADVMODE_SEP_BQ_COEFF = 1,
    RT1011_ADVMODE_EQ_BQ_COEFF = 2,
    RT1011_ADVMODE_BQ_UI_COEFF = 3,
    RT1011_ADVMODE_SMARTBOOST_COEFF = 4,
    RT1011_ADVMODE_NUM = 5,
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rt1011_priv {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub cali_work: work_struct,
    pub bq_drc_params: *mut *mut rt1011_bq_drc_params,

    pub sysclk: ::core::ffi::c_int,
    pub sysclk_src: ::core::ffi::c_int,
    pub lrck: ::core::ffi::c_int,
    pub bclk: ::core::ffi::c_int,
    pub id: ::core::ffi::c_int,

    pub pll_src: ::core::ffi::c_int,
    pub pll_in: ::core::ffi::c_int,
    pub pll_out: ::core::ffi::c_int,

    pub bq_drc_set: ::core::ffi::c_int,
    pub r0_reg: ::core::ffi::c_uint,
    pub cali_done: ::core::ffi::c_uint,
    pub r0_calib: ::core::ffi::c_uint,
    pub temperature_calib: ::core::ffi::c_uint,
    pub recv_spk_mode: ::core::ffi::c_int,
    pub i2s_ref: ::core::ffi::c_int,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
