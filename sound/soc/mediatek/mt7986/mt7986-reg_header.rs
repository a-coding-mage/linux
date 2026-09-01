/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mt7986-reg.h  --  MediaTek 7986 audio driver reg definition
 *
 * Copyright (c) 2023 MediaTek Inc.
 * Authors: Vic Wu <vic.wu@mediatek.com>
 *          Maso Huang <maso.huang@mediatek.com>
 */

pub const AUDIO_TOP_CON2: u32 = 0x0008;
pub const AUDIO_TOP_CON4: u32 = 0x0010;
pub const AUDIO_ENGEN_CON0: u32 = 0x0014;
pub const AFE_IRQ_MCU_EN: u32 = 0x0100;
pub const AFE_IRQ_MCU_STATUS: u32 = 0x0120;
pub const AFE_IRQ_MCU_CLR: u32 = 0x0128;
pub const AFE_IRQ0_MCU_CFG0: u32 = 0x0140;
pub const AFE_IRQ0_MCU_CFG1: u32 = 0x0144;
pub const AFE_IRQ1_MCU_CFG0: u32 = 0x0148;
pub const AFE_IRQ1_MCU_CFG1: u32 = 0x014c;
pub const AFE_IRQ2_MCU_CFG0: u32 = 0x0150;
pub const AFE_IRQ2_MCU_CFG1: u32 = 0x0154;
pub const ETDM_IN5_CON0: u32 = 0x13f0;
pub const ETDM_IN5_CON1: u32 = 0x13f4;
pub const ETDM_IN5_CON2: u32 = 0x13f8;
pub const ETDM_IN5_CON3: u32 = 0x13fc;
pub const ETDM_IN5_CON4: u32 = 0x1400;
pub const ETDM_OUT5_CON0: u32 = 0x1570;
pub const ETDM_OUT5_CON4: u32 = 0x1580;
pub const ETDM_OUT5_CON5: u32 = 0x1584;
pub const ETDM_4_7_COWORK_CON0: u32 = 0x15e0;
pub const ETDM_4_7_COWORK_CON1: u32 = 0x15e4;
pub const AFE_CONN018_1: u32 = 0x1b44;
pub const AFE_CONN018_4: u32 = 0x1b50;
pub const AFE_CONN019_1: u32 = 0x1b64;
pub const AFE_CONN019_4: u32 = 0x1b70;
pub const AFE_CONN124_1: u32 = 0x2884;
pub const AFE_CONN124_4: u32 = 0x2890;
pub const AFE_CONN125_1: u32 = 0x28a4;
pub const AFE_CONN125_4: u32 = 0x28b0;
pub const AFE_CONN_RS_0: u32 = 0x3920;
pub const AFE_CONN_RS_3: u32 = 0x392c;
pub const AFE_CONN_16BIT_0: u32 = 0x3960;
pub const AFE_CONN_16BIT_3: u32 = 0x396c;
pub const AFE_CONN_24BIT_0: u32 = 0x3980;
pub const AFE_CONN_24BIT_3: u32 = 0x398c;
pub const AFE_MEMIF_CON0: u32 = 0x3d98;
pub const AFE_MEMIF_RD_MON: u32 = 0x3da0;
pub const AFE_MEMIF_WR_MON: u32 = 0x3da4;
pub const AFE_DL0_BASE_MSB: u32 = 0x3e40;
pub const AFE_DL0_BASE: u32 = 0x3e44;
pub const AFE_DL0_CUR_MSB: u32 = 0x3e48;
pub const AFE_DL0_CUR: u32 = 0x3e4c;
pub const AFE_DL0_END_MSB: u32 = 0x3e50;
pub const AFE_DL0_END: u32 = 0x3e54;
pub const AFE_DL0_RCH_MON: u32 = 0x3e58;
pub const AFE_DL0_LCH_MON: u32 = 0x3e5c;
pub const AFE_DL0_CON0: u32 = 0x3e60;
pub const AFE_VUL0_BASE_MSB: u32 = 0x4220;
pub const AFE_VUL0_BASE: u32 = 0x4224;
pub const AFE_VUL0_CUR_MSB: u32 = 0x4228;
pub const AFE_VUL0_CUR: u32 = 0x422c;
pub const AFE_VUL0_END_MSB: u32 = 0x4230;
pub const AFE_VUL0_END: u32 = 0x4234;
pub const AFE_VUL0_CON0: u32 = 0x4238;

pub const AFE_MAX_REGISTER: u32 = AFE_VUL0_CON0;
pub const AFE_IRQ_STATUS_BITS: u32 = 0x7;
pub const AFE_IRQ_CNT_SHIFT: u32 = 0;
pub const AFE_IRQ_CNT_MASK: u32 = 0xffffff;

/* AUDIO_TOP_CON2 */
pub const CLK_OUT5_PDN: u32 = 1u32 << 14;
pub const CLK_OUT5_PDN_MASK: u32 = 1u32 << 14;
pub const CLK_IN5_PDN: u32 = 1u32 << 7;
pub const CLK_IN5_PDN_MASK: u32 = 1u32 << 7;

/* AUDIO_TOP_CON4 */
pub const PDN_APLL_TUNER2: u32 = 1u32 << 12;
pub const PDN_APLL_TUNER2_MASK: u32 = 1u32 << 12;

/* AUDIO_ENGEN_CON0 */
pub const AUD_APLL2_EN: u32 = 1u32 << 3;
pub const AUD_APLL2_EN_MASK: u32 = 1u32 << 3;
pub const AUD_26M_EN: u32 = 1u32 << 0;
pub const AUD_26M_EN_MASK: u32 = 1u32 << 0;

/* AFE_DL0_CON0 */
pub const DL0_ON_SFT: u32 = 28;
pub const DL0_ON_MASK: u32 = 0x1;
pub const DL0_ON_MASK_SFT: u32 = 1u32 << 28;
pub const DL0_MINLEN_SFT: u32 = 20;
pub const DL0_MINLEN_MASK: u32 = 0xf;
pub const DL0_MINLEN_MASK_SFT: u32 = 0xf << 20;
pub const DL0_MODE_SFT: u32 = 8;
pub const DL0_MODE_MASK: u32 = 0x1f;
pub const DL0_MODE_MASK_SFT: u32 = 0x1f << 8;
pub const DL0_PBUF_SIZE_SFT: u32 = 5;
pub const DL0_PBUF_SIZE_MASK: u32 = 0x3;
pub const DL0_PBUF_SIZE_MASK_SFT: u32 = 0x3 << 5;
pub const DL0_MONO_SFT: u32 = 4;
pub const DL0_MONO_MASK: u32 = 0x1;
pub const DL0_MONO_MASK_SFT: u32 = 1u32 << 4;
pub const DL0_HALIGN_SFT: u32 = 2;
pub const DL0_HALIGN_MASK: u32 = 0x1;
pub const DL0_HALIGN_MASK_SFT: u32 = 1u32 << 2;
pub const DL0_HD_MODE_SFT: u32 = 0;
pub const DL0_HD_MODE_MASK: u32 = 0x3;
pub const DL0_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_VUL0_CON0 */
pub const VUL0_ON_SFT: u32 = 28;
pub const VUL0_ON_MASK: u32 = 0x1;
pub const VUL0_ON_MASK_SFT: u32 = 1u32 << 28;
pub const VUL0_MODE_SFT: u32 = 8;
pub const VUL0_MODE_MASK: u32 = 0x1f;
pub const VUL0_MODE_MASK_SFT: u32 = 0x1f << 8;
pub const VUL0_MONO_SFT: u32 = 4;
pub const VUL0_MONO_MASK: u32 = 0x1;
pub const VUL0_MONO_MASK_SFT: u32 = 1u32 << 4;
pub const VUL0_HALIGN_SFT: u32 = 2;
pub const VUL0_HALIGN_MASK: u32 = 0x1;
pub const VUL0_HALIGN_MASK_SFT: u32 = 1u32 << 2;
pub const VUL0_HD_MODE_SFT: u32 = 0;
pub const VUL0_HD_MODE_MASK: u32 = 0x3;
pub const VUL0_HD_MODE_MASK_SFT: u32 = 0x3 << 0;

/* AFE_IRQ_MCU_CON */
pub const IRQ_MCU_MODE_SFT: u32 = 4;
pub const IRQ_MCU_MODE_MASK: u32 = 0x1f;
pub const IRQ_MCU_MODE_MASK_SFT: u32 = 0x1f << 4;
pub const IRQ_MCU_ON_SFT: u32 = 0;
pub const IRQ_MCU_ON_MASK: u32 = 0x1;
pub const IRQ_MCU_ON_MASK_SFT: u32 = 1u32 << 0;
pub const IRQ0_MCU_CLR_SFT: u32 = 0;
pub const IRQ0_MCU_CLR_MASK: u32 = 0x1;
pub const IRQ0_MCU_CLR_MASK_SFT: u32 = 1u32 << 0;
pub const IRQ1_MCU_CLR_SFT: u32 = 1;
pub const IRQ1_MCU_CLR_MASK: u32 = 0x1;
pub const IRQ1_MCU_CLR_MASK_SFT: u32 = 1u32 << 1;
pub const IRQ2_MCU_CLR_SFT: u32 = 2;
pub const IRQ2_MCU_CLR_MASK: u32 = 0x1;
pub const IRQ2_MCU_CLR_MASK_SFT: u32 = 1u32 << 2;

/* ETDM_IN5_CON2 */
pub const fn IN_CLK_SRC(x: u32) -> u32 {
    x << 10
}
pub const IN_CLK_SRC_SFT: u32 = 10;
pub const IN_CLK_SRC_MASK: u32 = ((1u32 << (12 - 10 + 1)) - 1) << 10;

/* ETDM_IN5_CON3 */
pub const fn IN_SEL_FS(x: u32) -> u32 {
    x << 26
}
pub const IN_SEL_FS_SFT: u32 = 26;
pub const IN_SEL_FS_MASK: u32 = ((1u32 << (30 - 26 + 1)) - 1) << 26;

/* ETDM_IN5_CON4 */
pub const fn IN_RELATCH(x: u32) -> u32 {
    x << 20
}
pub const IN_RELATCH_SFT: u32 = 20;
pub const IN_RELATCH_MASK: u32 = ((1u32 << (24 - 20 + 1)) - 1) << 20;
pub const IN_CLK_INV: u32 = 1u32 << 18;
pub const IN_CLK_INV_MASK: u32 = 1u32 << 18;

/* ETDM_IN5_CON0 & ETDM_OUT5_CON0 */
pub const RELATCH_SRC_MASK: u32 = ((1u32 << (30 - 28 + 1)) - 1) << 28;
pub const ETDM_CH_NUM_MASK: u32 = ((1u32 << (27 - 23 + 1)) - 1) << 23;
pub const ETDM_WRD_LEN_MASK: u32 = ((1u32 << (20 - 16 + 1)) - 1) << 16;
pub const ETDM_BIT_LEN_MASK: u32 = ((1u32 << (15 - 11 + 1)) - 1) << 11;
pub const ETDM_FMT_MASK: u32 = ((1u32 << (8 - 6 + 1)) - 1) << 6;
pub const ETDM_SYNC: u32 = 1u32 << 1;
pub const ETDM_SYNC_MASK: u32 = 1u32 << 1;
pub const ETDM_EN: u32 = 1u32 << 0;
pub const ETDM_EN_MASK: u32 = 1u32 << 0;

/* ETDM_OUT5_CON4 */
pub const fn OUT_RELATCH(x: u32) -> u32 {
    x << 24
}
pub const OUT_RELATCH_SFT: u32 = 24;
pub const OUT_RELATCH_MASK: u32 = ((1u32 << (28 - 24 + 1)) - 1) << 24;
pub const fn OUT_CLK_SRC(x: u32) -> u32 {
    x << 6
}
pub const OUT_CLK_SRC_SFT: u32 = 6;
pub const OUT_CLK_SRC_MASK: u32 = ((1u32 << (8 - 6 + 1)) - 1) << 6;
pub const fn OUT_SEL_FS(x: u32) -> u32 {
    x
}
pub const OUT_SEL_FS_SFT: u32 = 0;
pub const OUT_SEL_FS_MASK: u32 = ((1u32 << (4 - 0 + 1)) - 1) << 0;

/* ETDM_OUT5_CON5 */
pub const ETDM_CLK_DIV: u32 = 1u32 << 12;
pub const ETDM_CLK_DIV_MASK: u32 = 1u32 << 12;
pub const OUT_CLK_INV: u32 = 1u32 << 9;
pub const OUT_CLK_INV_MASK: u32 = 1u32 << 9;

/* ETDM_4_7_COWORK_CON0 */
pub const fn OUT_SEL(x: u32) -> u32 {
    x << 12
}
pub const OUT_SEL_SFT: u32 = 12;
pub const OUT_SEL_MASK: u32 = ((1u32 << (15 - 12 + 1)) - 1) << 12;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
