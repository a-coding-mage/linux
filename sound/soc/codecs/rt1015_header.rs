// SPDX-License-Identifier: GPL-2.0
//
// rt1015.h  --  RT1015 ALSA SoC audio amplifier driver
//
// Copyright 2019 Realtek Semiconductor Corp.
// Author: Jack Yu <jack.yu@realtek.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//

// C header dependency: <sound/rt1015.h>

pub const RT1015_DEVICE_ID_VAL: u32 = 0x1011;
pub const RT1015_DEVICE_ID_VAL2: u32 = 0x1015;

pub const RT1015_RESET: u32 = 0x0000;
pub const RT1015_CLK2: u32 = 0x0004;
pub const RT1015_CLK3: u32 = 0x0006;
pub const RT1015_PLL1: u32 = 0x000a;
pub const RT1015_PLL2: u32 = 0x000c;
pub const RT1015_DUM_RW1: u32 = 0x000e;
pub const RT1015_DUM_RW2: u32 = 0x0010;
pub const RT1015_DUM_RW3: u32 = 0x0012;
pub const RT1015_DUM_RW4: u32 = 0x0014;
pub const RT1015_DUM_RW5: u32 = 0x0016;
pub const RT1015_DUM_RW6: u32 = 0x0018;
pub const RT1015_CLK_DET: u32 = 0x0020;
pub const RT1015_SIL_DET: u32 = 0x0022;
pub const RT1015_CUSTOMER_ID: u32 = 0x0076;
pub const RT1015_PCODE_FWVER: u32 = 0x0078;
pub const RT1015_VER_ID: u32 = 0x007a;
pub const RT1015_VENDOR_ID: u32 = 0x007c;
pub const RT1015_DEVICE_ID: u32 = 0x007d;
pub const RT1015_PAD_DRV1: u32 = 0x00f0;
pub const RT1015_PAD_DRV2: u32 = 0x00f2;
pub const RT1015_GAT_BOOST: u32 = 0x00f3;
pub const RT1015_PRO_ALT: u32 = 0x00f4;
pub const RT1015_OSCK_STA: u32 = 0x00f6;
pub const RT1015_MAN_I2C: u32 = 0x0100;
pub const RT1015_DAC1: u32 = 0x0102;
pub const RT1015_DAC2: u32 = 0x0104;
pub const RT1015_DAC3: u32 = 0x0106;
pub const RT1015_ADC1: u32 = 0x010c;
pub const RT1015_ADC2: u32 = 0x010e;
pub const RT1015_TDM_MASTER: u32 = 0x0111;
pub const RT1015_TDM_TCON: u32 = 0x0112;
pub const RT1015_TDM1_1: u32 = 0x0114;
pub const RT1015_TDM1_2: u32 = 0x0116;
pub const RT1015_TDM1_3: u32 = 0x0118;
pub const RT1015_TDM1_4: u32 = 0x011a;
pub const RT1015_TDM1_5: u32 = 0x011c;
pub const RT1015_MIXER1: u32 = 0x0300;
pub const RT1015_MIXER2: u32 = 0x0302;
pub const RT1015_ANA_PROTECT1: u32 = 0x0311;
pub const RT1015_ANA_CTRL_SEQ1: u32 = 0x0313;
pub const RT1015_ANA_CTRL_SEQ2: u32 = 0x0314;
pub const RT1015_VBAT_DET_DEB: u32 = 0x031a;
pub const RT1015_VBAT_VOLT_DET1: u32 = 0x031c;
pub const RT1015_VBAT_VOLT_DET2: u32 = 0x031d;
pub const RT1015_VBAT_TEST_OUT1: u32 = 0x031e;
pub const RT1015_VBAT_TEST_OUT2: u32 = 0x031f;
pub const RT1015_VBAT_PROT_ATT: u32 = 0x0320;
pub const RT1015_VBAT_DET_CODE: u32 = 0x0321;
pub const RT1015_PWR1: u32 = 0x0322;
pub const RT1015_PWR4: u32 = 0x0328;
pub const RT1015_PWR5: u32 = 0x0329;
pub const RT1015_PWR6: u32 = 0x032a;
pub const RT1015_PWR7: u32 = 0x032b;
pub const RT1015_PWR8: u32 = 0x032c;
pub const RT1015_PWR9: u32 = 0x032d;
pub const RT1015_CLASSD_SEQ: u32 = 0x032e;
pub const RT1015_SMART_BST_CTRL1: u32 = 0x0330;
pub const RT1015_SMART_BST_CTRL2: u32 = 0x0332;
pub const RT1015_ANA_CTRL1: u32 = 0x0334;
pub const RT1015_ANA_CTRL2: u32 = 0x0336;
pub const RT1015_PWR_STATE_CTRL: u32 = 0x0338;
pub const RT1015_MONO_DYNA_CTRL: u32 = 0x04fa;
pub const RT1015_MONO_DYNA_CTRL1: u32 = 0x04fc;
pub const RT1015_MONO_DYNA_CTRL2: u32 = 0x04fe;
pub const RT1015_MONO_DYNA_CTRL3: u32 = 0x0500;
pub const RT1015_MONO_DYNA_CTRL4: u32 = 0x0502;
pub const RT1015_MONO_DYNA_CTRL5: u32 = 0x0504;
pub const RT1015_SPK_VOL: u32 = 0x0506;
pub const RT1015_SHORT_DETTOP1: u32 = 0x0508;
pub const RT1015_SHORT_DETTOP2: u32 = 0x050a;
pub const RT1015_SPK_DC_DETECT1: u32 = 0x0519;
pub const RT1015_SPK_DC_DETECT2: u32 = 0x051a;
pub const RT1015_SPK_DC_DETECT3: u32 = 0x051b;
pub const RT1015_SPK_DC_DETECT4: u32 = 0x051d;
pub const RT1015_SPK_DC_DETECT5: u32 = 0x051f;
pub const RT1015_BAT_RPO_STEP1: u32 = 0x0536;
pub const RT1015_BAT_RPO_STEP2: u32 = 0x0538;
pub const RT1015_BAT_RPO_STEP3: u32 = 0x053a;
pub const RT1015_BAT_RPO_STEP4: u32 = 0x053c;
pub const RT1015_BAT_RPO_STEP5: u32 = 0x053d;
pub const RT1015_BAT_RPO_STEP6: u32 = 0x053e;
pub const RT1015_BAT_RPO_STEP7: u32 = 0x053f;
pub const RT1015_BAT_RPO_STEP8: u32 = 0x0540;
pub const RT1015_BAT_RPO_STEP9: u32 = 0x0541;
pub const RT1015_BAT_RPO_STEP10: u32 = 0x0542;
pub const RT1015_BAT_RPO_STEP11: u32 = 0x0543;
pub const RT1015_BAT_RPO_STEP12: u32 = 0x0544;
pub const RT1015_SPREAD_SPEC1: u32 = 0x0568;
pub const RT1015_SPREAD_SPEC2: u32 = 0x056a;
pub const RT1015_PAD_STATUS: u32 = 0x1000;
pub const RT1015_PADS_PULLING_CTRL1: u32 = 0x1002;
pub const RT1015_PADS_DRIVING: u32 = 0x1006;
pub const RT1015_SYS_RST1: u32 = 0x1007;
pub const RT1015_SYS_RST2: u32 = 0x1009;
pub const RT1015_SYS_GATING1: u32 = 0x100a;
pub const RT1015_TEST_MODE1: u32 = 0x100c;
pub const RT1015_TEST_MODE2: u32 = 0x100d;
pub const RT1015_TIMING_CTRL1: u32 = 0x100e;
pub const RT1015_PLL_INT: u32 = 0x1010;
pub const RT1015_TEST_OUT1: u32 = 0x1020;
pub const RT1015_DC_CALIB_CLSD1: u32 = 0x1200;
pub const RT1015_DC_CALIB_CLSD2: u32 = 0x1202;
pub const RT1015_DC_CALIB_CLSD3: u32 = 0x1204;
pub const RT1015_DC_CALIB_CLSD4: u32 = 0x1206;
pub const RT1015_DC_CALIB_CLSD5: u32 = 0x1208;
pub const RT1015_DC_CALIB_CLSD6: u32 = 0x120a;
pub const RT1015_DC_CALIB_CLSD7: u32 = 0x120c;
pub const RT1015_DC_CALIB_CLSD8: u32 = 0x120e;
pub const RT1015_DC_CALIB_CLSD9: u32 = 0x1210;
pub const RT1015_DC_CALIB_CLSD10: u32 = 0x1212;
pub const RT1015_CLSD_INTERNAL1: u32 = 0x1300;
pub const RT1015_CLSD_INTERNAL2: u32 = 0x1302;
pub const RT1015_CLSD_INTERNAL3: u32 = 0x1304;
pub const RT1015_CLSD_INTERNAL4: u32 = 0x1305;
pub const RT1015_CLSD_INTERNAL5: u32 = 0x1306;
pub const RT1015_CLSD_INTERNAL6: u32 = 0x1308;
pub const RT1015_CLSD_INTERNAL7: u32 = 0x130a;
pub const RT1015_CLSD_INTERNAL8: u32 = 0x130c;
pub const RT1015_CLSD_INTERNAL9: u32 = 0x130e;
pub const RT1015_CLSD_OCP_CTRL: u32 = 0x130f;
pub const RT1015_VREF_LV: u32 = 0x1310;
pub const RT1015_MBIAS1: u32 = 0x1312;
pub const RT1015_MBIAS2: u32 = 0x1314;
pub const RT1015_MBIAS3: u32 = 0x1316;
pub const RT1015_MBIAS4: u32 = 0x1318;
pub const RT1015_VREF_LV1: u32 = 0x131a;
pub const RT1015_S_BST_TIMING_INTER1: u32 = 0x1322;
pub const RT1015_S_BST_TIMING_INTER2: u32 = 0x1323;
pub const RT1015_S_BST_TIMING_INTER3: u32 = 0x1324;
pub const RT1015_S_BST_TIMING_INTER4: u32 = 0x1325;
pub const RT1015_S_BST_TIMING_INTER5: u32 = 0x1326;
pub const RT1015_S_BST_TIMING_INTER6: u32 = 0x1327;
pub const RT1015_S_BST_TIMING_INTER7: u32 = 0x1328;
pub const RT1015_S_BST_TIMING_INTER8: u32 = 0x1329;
pub const RT1015_S_BST_TIMING_INTER9: u32 = 0x132a;
pub const RT1015_S_BST_TIMING_INTER10: u32 = 0x132b;
pub const RT1015_S_BST_TIMING_INTER11: u32 = 0x1330;
pub const RT1015_S_BST_TIMING_INTER12: u32 = 0x1331;
pub const RT1015_S_BST_TIMING_INTER13: u32 = 0x1332;
pub const RT1015_S_BST_TIMING_INTER14: u32 = 0x1333;
pub const RT1015_S_BST_TIMING_INTER15: u32 = 0x1334;
pub const RT1015_S_BST_TIMING_INTER16: u32 = 0x1335;
pub const RT1015_S_BST_TIMING_INTER17: u32 = 0x1336;
pub const RT1015_S_BST_TIMING_INTER18: u32 = 0x1337;
pub const RT1015_S_BST_TIMING_INTER19: u32 = 0x1338;
pub const RT1015_S_BST_TIMING_INTER20: u32 = 0x1339;
pub const RT1015_S_BST_TIMING_INTER21: u32 = 0x133a;
pub const RT1015_S_BST_TIMING_INTER22: u32 = 0x133b;
pub const RT1015_S_BST_TIMING_INTER23: u32 = 0x133c;
pub const RT1015_S_BST_TIMING_INTER24: u32 = 0x133d;
pub const RT1015_S_BST_TIMING_INTER25: u32 = 0x133e;
pub const RT1015_S_BST_TIMING_INTER26: u32 = 0x133f;
pub const RT1015_S_BST_TIMING_INTER27: u32 = 0x1340;
pub const RT1015_S_BST_TIMING_INTER28: u32 = 0x1341;
pub const RT1015_S_BST_TIMING_INTER29: u32 = 0x1342;
pub const RT1015_S_BST_TIMING_INTER30: u32 = 0x1343;
pub const RT1015_S_BST_TIMING_INTER31: u32 = 0x1344;
pub const RT1015_S_BST_TIMING_INTER32: u32 = 0x1345;
pub const RT1015_S_BST_TIMING_INTER33: u32 = 0x1346;
pub const RT1015_S_BST_TIMING_INTER34: u32 = 0x1347;
pub const RT1015_S_BST_TIMING_INTER35: u32 = 0x1348;
pub const RT1015_S_BST_TIMING_INTER36: u32 = 0x1349;

/* 0x0004 */
pub const RT1015_CLK_SYS_PRE_SEL_MASK: u32 = 0x3 << 14;
pub const RT1015_CLK_SYS_PRE_SEL_SFT: u32 = 14;
pub const RT1015_CLK_SYS_PRE_SEL_MCLK: u32 = 0x0 << 14;
pub const RT1015_CLK_SYS_PRE_SEL_PLL: u32 = 0x2 << 14;
pub const RT1015_PLL_SEL_MASK: u32 = 0x1 << 13;
pub const RT1015_PLL_SEL_SFT: u32 = 13;
pub const RT1015_PLL_SEL_PLL_SRC2: u32 = 0x0 << 13;
pub const RT1015_PLL_SEL_BCLK: u32 = 0x1 << 13;
pub const RT1015_FS_PD_MASK: u32 = 0x7 << 4;
pub const RT1015_FS_PD_SFT: u32 = 4;

/* 0x000a */
pub const RT1015_PLL_M_MAX: u32 = 0xf;
pub const RT1015_PLL_M_MASK: u32 = RT1015_PLL_M_MAX << 12;
pub const RT1015_PLL_M_SFT: u32 = 12;
pub const RT1015_PLL_M_BP: u32 = 0x1 << 11;
pub const RT1015_PLL_M_BP_SFT: u32 = 11;
pub const RT1015_PLL_N_MAX: u32 = 0x1ff;
pub const RT1015_PLL_N_MASK: u32 = RT1015_PLL_N_MAX << 0;
pub const RT1015_PLL_N_SFT: u32 = 0;

/* 0x000c */
pub const RT1015_PLL_BPK_MASK: u32 = 0x1 << 5;
pub const RT1015_PLL_BPK: u32 = 0x0 << 5;
pub const RT1015_PLL_K_MAX: u32 = 0x1f;
pub const RT1015_PLL_K_MASK: u32 = RT1015_PLL_K_MAX;
pub const RT1015_PLL_K_SFT: u32 = 0;

/* 0x0020 */
pub const RT1015_EN_BCLK_DET_MASK: u32 = 0x1 << 15;
pub const RT1015_EN_BCLK_DET: u32 = 0x1 << 15;
pub const RT1015_DIS_BCLK_DET: u32 = 0x0 << 15;

/* 0x007a */
pub const RT1015_ID_MASK: u32 = 0xff;
pub const RT1015_ID_VERA: u32 = 0x0;
pub const RT1015_ID_VERB: u32 = 0x1;

/* 0x00f2 */
pub const RT1015_MONO_LR_SEL_MASK: u32 = 0x3 << 4;
pub const RT1015_MONO_L_CHANNEL: u32 = 0x0 << 4;
pub const RT1015_MONO_R_CHANNEL: u32 = 0x1 << 4;
pub const RT1015_MONO_LR_MIX_CHANNEL: u32 = 0x2 << 4;

/* 0x0102 */
pub const RT1015_DAC_VOL_MASK: u32 = 0x7f << 9;
pub const RT1015_DAC_VOL_SFT: u32 = 9;

/* 0x0104 */
pub const RT1015_DAC_CLK: u32 = 0x1 << 13;
pub const RT1015_DAC_CLK_BIT: u32 = 13;

/* 0x0106 */
pub const RT1015_DAC_MUTE_MASK: u32 = 0x1 << 15;
pub const RT1015_DA_MUTE_SFT: u32 = 15;
pub const RT1015_DVOL_MUTE_FLAG_SFT: u32 = 12;

/* 0x0111 */
pub const RT1015_TCON_TDM_MS_MASK: u32 = 0x1 << 14;
pub const RT1015_TCON_TDM_MS_SFT: u32 = 14;
pub const RT1015_TCON_TDM_MS_S: u32 = 0x0 << 14;
pub const RT1015_TCON_TDM_MS_M: u32 = 0x1 << 14;
pub const RT1015_I2S_DL_MASK: u32 = 0x7 << 8;
pub const RT1015_I2S_DL_SFT: u32 = 8;
pub const RT1015_I2S_DL_16: u32 = 0x0 << 8;
pub const RT1015_I2S_DL_20: u32 = 0x1 << 8;
pub const RT1015_I2S_DL_24: u32 = 0x2 << 8;
pub const RT1015_I2S_DL_8: u32 = 0x3 << 8;
pub const RT1015_I2S_M_DF_MASK: u32 = 0x7 << 0;
pub const RT1015_I2S_M_DF_SFT: u32 = 0;
pub const RT1015_I2S_M_DF_I2S: u32 = 0x0;
pub const RT1015_I2S_M_DF_LEFT: u32 = 0x1;
pub const RT1015_I2S_M_DF_PCM_A: u32 = 0x2;
pub const RT1015_I2S_M_DF_PCM_B: u32 = 0x3;
pub const RT1015_I2S_M_DF_PCM_A_N: u32 = 0x6;
pub const RT1015_I2S_M_DF_PCM_B_N: u32 = 0x7;

/* TDM_tcon Setting (0x0112) */
pub const RT1015_I2S_TCON_DF_MASK: u32 = 0x7 << 13;
pub const RT1015_I2S_TCON_DF_SFT: u32 = 13;
pub const RT1015_I2S_TCON_DF_I2S: u32 = 0x0 << 13;
pub const RT1015_I2S_TCON_DF_LEFT: u32 = 0x1 << 13;
pub const RT1015_I2S_TCON_DF_PCM_A: u32 = 0x2 << 13;
pub const RT1015_I2S_TCON_DF_PCM_B: u32 = 0x3 << 13;
pub const RT1015_I2S_TCON_DF_PCM_A_N: u32 = 0x6 << 13;
pub const RT1015_I2S_TCON_DF_PCM_B_N: u32 = 0x7 << 13;
pub const RT1015_TCON_BCLK_SEL_MASK: u32 = 0x3 << 10;
pub const RT1015_TCON_BCLK_SEL_SFT: u32 = 10;
pub const RT1015_TCON_BCLK_SEL_32FS: u32 = 0x0 << 10;
pub const RT1015_TCON_BCLK_SEL_64FS: u32 = 0x1 << 10;
pub const RT1015_TCON_BCLK_SEL_128FS: u32 = 0x2 << 10;
pub const RT1015_TCON_BCLK_SEL_256FS: u32 = 0x3 << 10;
pub const RT1015_TCON_CH_LEN_MASK: u32 = 0x3 << 5;
pub const RT1015_TCON_CH_LEN_SFT: u32 = 5;
pub const RT1015_TCON_CH_LEN_16B: u32 = 0x0 << 5;
pub const RT1015_TCON_CH_LEN_20B: u32 = 0x1 << 5;
pub const RT1015_TCON_CH_LEN_24B: u32 = 0x2 << 5;
pub const RT1015_TCON_CH_LEN_32B: u32 = 0x3 << 5;
pub const RT1015_TCON_BCLK_MST_MASK: u32 = 0x1 << 4;
pub const RT1015_TCON_BCLK_MST_SFT: u32 = 4;
pub const RT1015_TCON_BCLK_MST_INV: u32 = 0x1 << 4;

/* TDM1 Setting-1 (0x0114) */
pub const RT1015_TDM_INV_BCLK_MASK: u32 = 0x1 << 15;
pub const RT1015_TDM_INV_BCLK_SFT: u32 = 15;
pub const RT1015_TDM_INV_BCLK: u32 = 0x1 << 15;
pub const RT1015_I2S_CH_TX_MASK: u32 = 0x3 << 10;
pub const RT1015_I2S_CH_TX_SFT: u32 = 10;
pub const RT1015_I2S_TX_2CH: u32 = 0x0 << 10;
pub const RT1015_I2S_TX_4CH: u32 = 0x1 << 10;
pub const RT1015_I2S_TX_6CH: u32 = 0x2 << 10;
pub const RT1015_I2S_TX_8CH: u32 = 0x3 << 10;
pub const RT1015_I2S_CH_RX_MASK: u32 = 0x3 << 8;
pub const RT1015_I2S_CH_RX_SFT: u32 = 8;
pub const RT1015_I2S_RX_2CH: u32 = 0x0 << 8;
pub const RT1015_I2S_RX_4CH: u32 = 0x1 << 8;
pub const RT1015_I2S_RX_6CH: u32 = 0x2 << 8;
pub const RT1015_I2S_RX_8CH: u32 = 0x3 << 8;
pub const RT1015_I2S_LR_CH_SEL_MASK: u32 = 0x1 << 7;
pub const RT1015_I2S_LR_CH_SEL_SFT: u32 = 7;
pub const RT1015_I2S_LEFT_CH_SEL: u32 = 0x0 << 7;
pub const RT1015_I2S_RIGHT_CH_SEL: u32 = 0x1 << 7;
pub const RT1015_I2S_CH_TX_LEN_MASK: u32 = 0x7 << 4;
pub const RT1015_I2S_CH_TX_LEN_SFT: u32 = 4;
pub const RT1015_I2S_CH_TX_LEN_16B: u32 = 0x0 << 4;
pub const RT1015_I2S_CH_TX_LEN_20B: u32 = 0x1 << 4;
pub const RT1015_I2S_CH_TX_LEN_24B: u32 = 0x2 << 4;
pub const RT1015_I2S_CH_TX_LEN_32B: u32 = 0x3 << 4;
pub const RT1015_I2S_CH_TX_LEN_8B: u32 = 0x4 << 4;
pub const RT1015_I2S_CH_RX_LEN_MASK: u32 = 0x7 << 0;
pub const RT1015_I2S_CH_RX_LEN_SFT: u32 = 0;
pub const RT1015_I2S_CH_RX_LEN_16B: u32 = 0x0 << 0;
pub const RT1015_I2S_CH_RX_LEN_20B: u32 = 0x1 << 0;
pub const RT1015_I2S_CH_RX_LEN_24B: u32 = 0x2 << 0;
pub const RT1015_I2S_CH_RX_LEN_32B: u32 = 0x3 << 0;
pub const RT1015_I2S_CH_RX_LEN_8B: u32 = 0x4 << 0;

/* TDM1 Setting-4 (0x011a) */
pub const RT1015_TDM_I2S_TX_L_DAC1_1_MASK: u32 = 0x7 << 12;
pub const RT1015_TDM_I2S_TX_R_DAC1_1_MASK: u32 = 0x7 << 8;
pub const RT1015_TDM_I2S_TX_L_DAC1_1_SFT: u32 = 12;
pub const RT1015_TDM_I2S_TX_R_DAC1_1_SFT: u32 = 8;

/* 0x0330 */
pub const RT1015_ABST_AUTO_EN_MASK: u32 = 0x1 << 13;
pub const RT1015_ABST_AUTO_MODE: u32 = 0x1 << 13;
pub const RT1015_ABST_REG_MODE: u32 = 0x0 << 13;
pub const RT1015_ABST_FIX_TGT_MASK: u32 = 0x1 << 12;
pub const RT1015_ABST_FIX_TGT_EN: u32 = 0x1 << 12;
pub const RT1015_ABST_FIX_TGT_DIS: u32 = 0x0 << 12;
pub const RT1015_BYPASS_SWR_REG_MASK: u32 = 0x1 << 7;
pub const RT1015_BYPASS_SWRREG_BYPASS: u32 = 0x1 << 7;
pub const RT1015_BYPASS_SWRREG_PASS: u32 = 0x0 << 7;

/* 0x0322 */
pub const RT1015_PWR_LDO2: u32 = 0x1 << 15;
pub const RT1015_PWR_LDO2_BIT: u32 = 15;
pub const RT1015_PWR_DAC: u32 = 0x1 << 14;
pub const RT1015_PWR_DAC_BIT: u32 = 14;
pub const RT1015_PWR_INTCLK: u32 = 0x1 << 13;
pub const RT1015_PWR_INTCLK_BIT: u32 = 13;
pub const RT1015_PWR_ISENSE: u32 = 0x1 << 12;
pub const RT1015_PWR_ISENSE_BIT: u32 = 12;
pub const RT1015_PWR_VSENSE: u32 = 0x1 << 10;
pub const RT1015_PWR_VSENSE_BIT: u32 = 10;
pub const RT1015_PWR_PLL: u32 = 0x1 << 9;
pub const RT1015_PWR_PLL_BIT: u32 = 9;
pub const RT1015_PWR_BG_1_2: u32 = 0x1 << 8;
pub const RT1015_PWR_BG_1_2_BIT: u32 = 8;
pub const RT1015_PWR_MBIAS_BG: u32 = 0x1 << 7;
pub const RT1015_PWR_MBIAS_BG_BIT: u32 = 7;
pub const RT1015_PWR_VBAT: u32 = 0x1 << 6;
pub const RT1015_PWR_VBAT_BIT: u32 = 6;
pub const RT1015_PWR_MBIAS: u32 = 0x1 << 4;
pub const RT1015_PWR_MBIAS_BIT: u32 = 4;
pub const RT1015_PWR_ADCV: u32 = 0x1 << 3;
pub const RT1015_PWR_ADCV_BIT: u32 = 3;
pub const RT1015_PWR_MIXERV: u32 = 0x1 << 2;
pub const RT1015_PWR_MIXERV_BIT: u32 = 2;
pub const RT1015_PWR_SUMV: u32 = 0x1 << 1;
pub const RT1015_PWR_SUMV_BIT: u32 = 1;
pub const RT1015_PWR_VREFLV: u32 = 0x1 << 0;
pub const RT1015_PWR_VREFLV_BIT: u32 = 0;

/* 0x0324 */
pub const RT1015_PWR_BASIC: u32 = 0x1 << 15;
pub const RT1015_PWR_BASIC_BIT: u32 = 15;
pub const RT1015_PWR_SD: u32 = 0x1 << 14;
pub const RT1015_PWR_SD_BIT: u32 = 14;
pub const RT1015_PWR_IBIAS: u32 = 0x1 << 13;
pub const RT1015_PWR_IBIAS_BIT: u32 = 13;
pub const RT1015_PWR_VCM: u32 = 0x1 << 11;
pub const RT1015_PWR_VCM_BIT: u32 = 11;

/* 0x0328 */
pub const RT1015_PWR_SWR: u32 = 0x1 << 12;
pub const RT1015_PWR_SWR_BIT: u32 = 12;

/* 0x0519 */
pub const RT1015_EN_CLA_D_DC_DET_MASK: u32 = 0x1 << 12;
pub const RT1015_EN_CLA_D_DC_DET: u32 = 0x1 << 12;
pub const RT1015_DIS_CLA_D_DC_DET: u32 = 0x0 << 12;

/* 0x1300 */
pub const RT1015_PWR_CLSD: u32 = 0x1 << 12;
pub const RT1015_PWR_CLSD_BIT: u32 = 12;

/* 0x007a */
// RT1015_ID_MASK, RT1015_ID_VERA, and RT1015_ID_VERB are repeated in the C header.

/* System Clock Source */
pub const RT1015_SCLK_S_MCLK: i32 = 0;
pub const RT1015_SCLK_S_PLL: i32 = 1;

/* PLL1 Source */
pub const RT1015_PLL_S_MCLK: i32 = 0;
pub const RT1015_PLL_S_BCLK: i32 = 1;

pub const RT1015_AIF1: i32 = 0;
pub const RT1015_AIFS: i32 = 1;

pub const RT1015_VERA: i32 = 0;
pub const RT1015_VERB: i32 = 1;

pub const BYPASS: i32 = 0;
pub const ADAPTIVE: i32 = 1;
pub const FIXED_ADAPTIVE: i32 = 2;

pub const RT1015_Enable_Boost: i32 = 0;
pub const RT1015_Bypass_Boost: i32 = 1;

pub const RT1015_HW_28: i32 = 0;
pub const RT1015_HW_29: i32 = 1;

#[repr(C)]
pub struct rt1015_priv {
    pub component: *mut snd_soc_component,
    pub pdata: rt1015_platform_data,
    pub regmap: *mut regmap,
    pub sysclk: i32,
    pub sysclk_src: i32,
    pub pll_src: i32,
    pub pll_in: i32,
    pub pll_out: i32,
    pub boost_mode: i32,
    pub bypass_boost: i32,
    pub dac_is_used: i32,
    pub cali_done: i32,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
