// SPDX-License-Identifier: GPL-2.0
/*
 * mt2701-reg.h  --  Mediatek 2701 audio driver reg definition
 *
 * Copyright (c) 2016 MediaTek Inc.
 * Author: Garlic Tseng <garlic.tseng@mediatek.com>
 */

pub const AUDIO_TOP_CON0: u32 = 0x0000;
pub const AUDIO_TOP_CON3: u32 = 0x000c;
pub const AUDIO_TOP_CON4: u32 = 0x0010;
pub const AUDIO_TOP_CON5: u32 = 0x0014;
pub const AFE_DAIBT_CON0: u32 = 0x001c;
pub const AFE_MRGIF_CON: u32 = 0x003c;
pub const AFE_HDMI_OUT_CON0: u32 = 0x0370;
pub const AFE_HDMI_OUT_BASE: u32 = 0x0374;
pub const AFE_HDMI_OUT_CUR: u32 = 0x0378;
pub const AFE_HDMI_OUT_END: u32 = 0x037c;
pub const AFE_HDMI_CONN0: u32 = 0x0390;
pub const AFE_8CH_I2S_OUT_CON: u32 = 0x0394;
pub const ASMI_TIMING_CON1: u32 = 0x0100;
pub const ASMO_TIMING_CON1: u32 = 0x0104;
pub const PWR1_ASM_CON1: u32 = 0x0108;
pub const ASYS_TOP_CON: u32 = 0x0600;
pub const ASYS_I2SIN1_CON: u32 = 0x0604;
pub const ASYS_I2SIN2_CON: u32 = 0x0608;
pub const ASYS_I2SIN3_CON: u32 = 0x060c;
pub const ASYS_I2SIN4_CON: u32 = 0x0610;
pub const ASYS_I2SIN5_CON: u32 = 0x0614;
pub const ASYS_I2SO1_CON: u32 = 0x061C;
pub const ASYS_I2SO2_CON: u32 = 0x0620;
pub const ASYS_I2SO3_CON: u32 = 0x0624;
pub const ASYS_I2SO4_CON: u32 = 0x0628;
pub const ASYS_I2SO5_CON: u32 = 0x062c;
pub const PWR2_TOP_CON: u32 = 0x0634;
pub const AFE_CONN0: u32 = 0x06c0;
pub const AFE_CONN1: u32 = 0x06c4;
pub const AFE_CONN2: u32 = 0x06c8;
pub const AFE_CONN3: u32 = 0x06cc;
pub const AFE_CONN14: u32 = 0x06f8;
pub const AFE_CONN15: u32 = 0x06fc;
pub const AFE_CONN16: u32 = 0x0700;
pub const AFE_CONN17: u32 = 0x0704;
pub const AFE_CONN18: u32 = 0x0708;
pub const AFE_CONN19: u32 = 0x070c;
pub const AFE_CONN20: u32 = 0x0710;
pub const AFE_CONN21: u32 = 0x0714;
pub const AFE_CONN22: u32 = 0x0718;
pub const AFE_CONN23: u32 = 0x071c;
pub const AFE_CONN24: u32 = 0x0720;
pub const AFE_CONN41: u32 = 0x0764;
pub const ASYS_IRQ1_CON: u32 = 0x0780;
pub const ASYS_IRQ2_CON: u32 = 0x0784;
pub const ASYS_IRQ3_CON: u32 = 0x0788;
pub const ASYS_IRQ_CLR: u32 = 0x07c0;
pub const ASYS_IRQ_STATUS: u32 = 0x07c4;
pub const PWR2_ASM_CON1: u32 = 0x1070;
pub const AFE_DAC_CON0: u32 = 0x1200;
pub const AFE_DAC_CON1: u32 = 0x1204;
pub const AFE_DAC_CON2: u32 = 0x1208;
pub const AFE_DAC_CON3: u32 = 0x120c;
pub const AFE_DAC_CON4: u32 = 0x1210;
pub const AFE_MEMIF_HD_CON1: u32 = 0x121c;
pub const AFE_MEMIF_PBUF_SIZE: u32 = 0x1238;
pub const AFE_MEMIF_HD_CON0: u32 = 0x123c;
pub const AFE_DL1_BASE: u32 = 0x1240;
pub const AFE_DL1_CUR: u32 = 0x1244;
pub const AFE_DL2_BASE: u32 = 0x1250;
pub const AFE_DL2_CUR: u32 = 0x1254;
pub const AFE_DL3_BASE: u32 = 0x1260;
pub const AFE_DL3_CUR: u32 = 0x1264;
pub const AFE_DL4_BASE: u32 = 0x1270;
pub const AFE_DL4_CUR: u32 = 0x1274;
pub const AFE_DL5_BASE: u32 = 0x1280;
pub const AFE_DL5_CUR: u32 = 0x1284;
pub const AFE_DLMCH_BASE: u32 = 0x12a0;
pub const AFE_DLMCH_CUR: u32 = 0x12a4;
pub const AFE_ARB1_BASE: u32 = 0x12b0;
pub const AFE_ARB1_CUR: u32 = 0x12b4;
pub const AFE_VUL_BASE: u32 = 0x1300;
pub const AFE_VUL_CUR: u32 = 0x130c;
pub const AFE_UL2_BASE: u32 = 0x1310;
pub const AFE_UL2_END: u32 = 0x1318;
pub const AFE_UL2_CUR: u32 = 0x131c;
pub const AFE_UL3_BASE: u32 = 0x1320;
pub const AFE_UL3_END: u32 = 0x1328;
pub const AFE_UL3_CUR: u32 = 0x132c;
pub const AFE_UL4_BASE: u32 = 0x1330;
pub const AFE_UL4_END: u32 = 0x1338;
pub const AFE_UL4_CUR: u32 = 0x133c;
pub const AFE_UL5_BASE: u32 = 0x1340;
pub const AFE_UL5_END: u32 = 0x1348;
pub const AFE_UL5_CUR: u32 = 0x134c;
pub const AFE_DAI_BASE: u32 = 0x1370;
pub const AFE_DAI_CUR: u32 = 0x137c;

/* AFE_DAIBT_CON0 (0x001c) */
pub const AFE_DAIBT_CON0_DAIBT_EN: u32 = 0x1 << 0;
pub const AFE_DAIBT_CON0_BT_FUNC_EN: u32 = 0x1 << 1;
pub const AFE_DAIBT_CON0_BT_FUNC_RDY: u32 = 0x1 << 3;
pub const AFE_DAIBT_CON0_BT_WIDE_MODE_EN: u32 = 0x1 << 9;
pub const AFE_DAIBT_CON0_MRG_USE: u32 = 0x1 << 12;

/* PWR1_ASM_CON1 (0x0108) */
pub const PWR1_ASM_CON1_INIT_VAL: u32 = 0x492;

/* AFE_MRGIF_CON (0x003c) */
pub const AFE_MRGIF_CON_MRG_EN: u32 = 0x1 << 0;
pub const AFE_MRGIF_CON_MRG_I2S_EN: u32 = 0x1 << 16;
pub const AFE_MRGIF_CON_I2S_MODE_MASK: u32 = 0xf << 20;
pub const AFE_MRGIF_CON_I2S_MODE_32K: u32 = 0x4 << 20;

/* ASYS_TOP_CON (0x0600) */
pub const ASYS_TOP_CON_ASYS_TIMING_ON: u32 = 0x3 << 0;

/* PWR2_ASM_CON1 (0x1070) */
pub const PWR2_ASM_CON1_INIT_VAL: u32 = 0x492492;

/* AFE_DAC_CON0 (0x1200) */
pub const AFE_DAC_CON0_AFE_ON: u32 = 0x1 << 0;

/* AFE_MEMIF_PBUF_SIZE (0x1238) */
pub const AFE_MEMIF_PBUF_SIZE_DLM_MASK: u32 = 0x1 << 29;
pub const AFE_MEMIF_PBUF_SIZE_PAIR_INTERLEAVE: u32 = 0x0 << 29;
pub const AFE_MEMIF_PBUF_SIZE_FULL_INTERLEAVE: u32 = 0x1 << 29;
pub const DLMCH_BIT_WIDTH_MASK: u32 = 0x1 << 28;
pub const AFE_MEMIF_PBUF_SIZE_DLM_CH_MASK: u32 = 0xf << 24;
pub const fn AFE_MEMIF_PBUF_SIZE_DLM_CH(x: u32) -> u32 {
    x << 24
}
pub const AFE_MEMIF_PBUF_SIZE_DLM_BYTE_MASK: u32 = 0x3 << 12;
pub const AFE_MEMIF_PBUF_SIZE_DLM_32BYTES: u32 = 0x1 << 12;

/* AUDIO_TOP_CON3 (0x000c) -- HDMI BCK divider */
pub const AUDIO_TOP_CON3_HDMI_BCK_DIV_MASK: u32 = 0x3f << 8;
pub const fn AUDIO_TOP_CON3_HDMI_BCK_DIV(x: u32) -> u32 {
    (x & 0x3f) << 8
}

/* AFE_HDMI_OUT_CON0 (0x0370) */
pub const AFE_HDMI_OUT_CON0_OUT_ON: u32 = 0x1 << 0;
pub const AFE_HDMI_OUT_CON0_BIT_WIDTH_MASK: u32 = 0x1 << 1;
pub const AFE_HDMI_OUT_CON0_BIT_WIDTH_16: u32 = 0x0 << 1;
pub const AFE_HDMI_OUT_CON0_BIT_WIDTH_32: u32 = 0x1 << 1;
pub const AFE_HDMI_OUT_CON0_CH_NUM_MASK: u32 = 0xf << 4;
pub const fn AFE_HDMI_OUT_CON0_CH_NUM(x: u32) -> u32 {
    (x & 0xf) << 4
}

/* AFE_8CH_I2S_OUT_CON (0x0394) -- on-SoC 8-channel I2S that feeds HDMI TX */
pub const AFE_8CH_I2S_OUT_CON_EN: u32 = 0x1 << 0;
pub const AFE_8CH_I2S_OUT_CON_BCK_INV: u32 = 0x1 << 1;
pub const AFE_8CH_I2S_OUT_CON_LRCK_INV: u32 = 0x1 << 2;
pub const AFE_8CH_I2S_OUT_CON_I2S_DELAY: u32 = 0x1 << 3;
pub const AFE_8CH_I2S_OUT_CON_WLEN_MASK: u32 = 0x3 << 4;
pub const AFE_8CH_I2S_OUT_CON_WLEN_16BIT: u32 = 0x1 << 4;
pub const AFE_8CH_I2S_OUT_CON_WLEN_24BIT: u32 = 0x2 << 4;
pub const AFE_8CH_I2S_OUT_CON_WLEN_32BIT: u32 = 0x3 << 4;

/* I2S in/out register bit control */
pub const ASYS_I2S_CON_FS: u32 = 0x1f << 8;
pub const fn ASYS_I2S_CON_FS_SET(x: u32) -> u32 {
    x << 8
}
pub const ASYS_I2S_CON_RESET: u32 = 0x1 << 30;
pub const ASYS_I2S_CON_I2S_EN: u32 = 0x1 << 0;
pub const ASYS_I2S_CON_ONE_HEART_MODE: u32 = 0x1 << 16;
pub const ASYS_I2S_CON_I2S_COUPLE_MODE: u32 = 0x1 << 17;
/* 0:EIAJ 1:I2S */
pub const ASYS_I2S_CON_I2S_MODE: u32 = 0x1 << 3;
pub const ASYS_I2S_CON_WIDE_MODE: u32 = 0x1 << 1;
pub const fn ASYS_I2S_CON_WIDE_MODE_SET(x: u32) -> u32 {
    x << 1
}
pub const ASYS_I2S_IN_PHASE_FIX: u32 = 0x1 << 31;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
