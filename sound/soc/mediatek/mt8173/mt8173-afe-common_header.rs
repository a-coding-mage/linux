// SPDX-License-Identifier: GPL-2.0
/*
 * mt8173_afe_common.h  --  Mediatek 8173 audio driver common definitions
 *
 * Copyright (c) 2015 MediaTek Inc.
 * Author: Koro Chen <koro.chen@mediatek.com>
 *             Sascha Hauer <s.hauer@pengutronix.de>
 *             Hidalgo Huang <hidalgo.huang@mediatek.com>
 *             Ir Lian <ir.lian@mediatek.com>
 */

// C header dependencies: <linux/clk.h>, <linux/regmap.h>

pub const MT8173_AFE_MEMIF_DL1: u32 = 0;
pub const MT8173_AFE_MEMIF_DL2: u32 = 1;
pub const MT8173_AFE_MEMIF_VUL: u32 = 2;
pub const MT8173_AFE_MEMIF_DAI: u32 = 3;
pub const MT8173_AFE_MEMIF_AWB: u32 = 4;
pub const MT8173_AFE_MEMIF_MOD_DAI: u32 = 5;
pub const MT8173_AFE_MEMIF_HDMI: u32 = 6;
pub const MT8173_AFE_MEMIF_NUM: u32 = 7;
pub const MT8173_AFE_IO_MOD_PCM1: u32 = MT8173_AFE_MEMIF_NUM;
pub const MT8173_AFE_IO_MOD_PCM2: u32 = 8;
pub const MT8173_AFE_IO_PMIC: u32 = 9;
pub const MT8173_AFE_IO_I2S: u32 = 10;
pub const MT8173_AFE_IO_2ND_I2S: u32 = 11;
pub const MT8173_AFE_IO_HW_GAIN1: u32 = 12;
pub const MT8173_AFE_IO_HW_GAIN2: u32 = 13;
pub const MT8173_AFE_IO_MRG_O: u32 = 14;
pub const MT8173_AFE_IO_MRG_I: u32 = 15;
pub const MT8173_AFE_IO_DAIBT: u32 = 16;
pub const MT8173_AFE_IO_HDMI: u32 = 17;

pub const MT8173_AFE_IRQ_DL1: u32 = 0;
pub const MT8173_AFE_IRQ_DL2: u32 = 1;
pub const MT8173_AFE_IRQ_VUL: u32 = 2;
pub const MT8173_AFE_IRQ_DAI: u32 = 3;
pub const MT8173_AFE_IRQ_AWB: u32 = 4;
pub const MT8173_AFE_IRQ_MOD_DAI: u32 = 5;
pub const MT8173_AFE_IRQ_HDMI: u32 = 6;
pub const MT8173_AFE_IRQ_NUM: u32 = 7;

pub const MT8173_CLK_INFRASYS_AUD: u32 = 0;
pub const MT8173_CLK_TOP_PDN_AUD: u32 = 1;
pub const MT8173_CLK_TOP_PDN_AUD_BUS: u32 = 2;
pub const MT8173_CLK_I2S0_M: u32 = 3;
pub const MT8173_CLK_I2S1_M: u32 = 4;
pub const MT8173_CLK_I2S2_M: u32 = 5;
pub const MT8173_CLK_I2S3_M: u32 = 6;
pub const MT8173_CLK_I2S3_B: u32 = 7;
pub const MT8173_CLK_BCK0: u32 = 8;
pub const MT8173_CLK_BCK1: u32 = 9;
pub const MT8173_CLK_NUM: u32 = 10;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
