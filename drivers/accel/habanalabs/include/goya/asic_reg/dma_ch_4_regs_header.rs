/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2016-2018 HabanaLabs, Ltd.
 * All Rights Reserved.
 *
 */

/************************************
 ** This is an auto-generated file **
 **       DO NOT EDIT BELOW        **
 ************************************/

/*
 *****************************************
 *   DMA_CH_4 (Prototype: DMA_CH)
 *****************************************
 */

pub const mmDMA_CH_4_CFG0: u32 = 0x421000;
pub const mmDMA_CH_4_CFG1: u32 = 0x421004;
pub const mmDMA_CH_4_ERRMSG_ADDR_LO: u32 = 0x421008;
pub const mmDMA_CH_4_ERRMSG_ADDR_HI: u32 = 0x42100C;
pub const mmDMA_CH_4_ERRMSG_WDATA: u32 = 0x421010;
pub const mmDMA_CH_4_RD_COMP_ADDR_LO: u32 = 0x421014;
pub const mmDMA_CH_4_RD_COMP_ADDR_HI: u32 = 0x421018;
pub const mmDMA_CH_4_RD_COMP_WDATA: u32 = 0x42101C;
pub const mmDMA_CH_4_WR_COMP_ADDR_LO: u32 = 0x421020;
pub const mmDMA_CH_4_WR_COMP_ADDR_HI: u32 = 0x421024;
pub const mmDMA_CH_4_WR_COMP_WDATA: u32 = 0x421028;
pub const mmDMA_CH_4_LDMA_SRC_ADDR_LO: u32 = 0x42102C;
pub const mmDMA_CH_4_LDMA_SRC_ADDR_HI: u32 = 0x421030;
pub const mmDMA_CH_4_LDMA_DST_ADDR_LO: u32 = 0x421034;
pub const mmDMA_CH_4_LDMA_DST_ADDR_HI: u32 = 0x421038;
pub const mmDMA_CH_4_LDMA_TSIZE: u32 = 0x42103C;
pub const mmDMA_CH_4_COMIT_TRANSFER: u32 = 0x421040;
pub const mmDMA_CH_4_STS0: u32 = 0x421044;
pub const mmDMA_CH_4_STS1: u32 = 0x421048;
pub const mmDMA_CH_4_STS2: u32 = 0x42104C;
pub const mmDMA_CH_4_STS3: u32 = 0x421050;
pub const mmDMA_CH_4_STS4: u32 = 0x421054;
pub const mmDMA_CH_4_SRC_ADDR_LO_STS: u32 = 0x421058;
pub const mmDMA_CH_4_SRC_ADDR_HI_STS: u32 = 0x42105C;
pub const mmDMA_CH_4_SRC_TSIZE_STS: u32 = 0x421060;
pub const mmDMA_CH_4_DST_ADDR_LO_STS: u32 = 0x421064;
pub const mmDMA_CH_4_DST_ADDR_HI_STS: u32 = 0x421068;
pub const mmDMA_CH_4_DST_TSIZE_STS: u32 = 0x42106C;
pub const mmDMA_CH_4_RD_RATE_LIM_EN: u32 = 0x421070;
pub const mmDMA_CH_4_RD_RATE_LIM_RST_TOKEN: u32 = 0x421074;
pub const mmDMA_CH_4_RD_RATE_LIM_SAT: u32 = 0x421078;
pub const mmDMA_CH_4_RD_RATE_LIM_TOUT: u32 = 0x42107C;
pub const mmDMA_CH_4_WR_RATE_LIM_EN: u32 = 0x421080;
pub const mmDMA_CH_4_WR_RATE_LIM_RST_TOKEN: u32 = 0x421084;
pub const mmDMA_CH_4_WR_RATE_LIM_SAT: u32 = 0x421088;
pub const mmDMA_CH_4_WR_RATE_LIM_TOUT: u32 = 0x42108C;
pub const mmDMA_CH_4_CFG2: u32 = 0x421090;
pub const mmDMA_CH_4_TDMA_CTL: u32 = 0x421100;
pub const mmDMA_CH_4_TDMA_SRC_BASE_ADDR_LO: u32 = 0x421104;
pub const mmDMA_CH_4_TDMA_SRC_BASE_ADDR_HI: u32 = 0x421108;
pub const mmDMA_CH_4_TDMA_SRC_ROI_BASE_0: u32 = 0x42110C;
pub const mmDMA_CH_4_TDMA_SRC_ROI_SIZE_0: u32 = 0x421110;
pub const mmDMA_CH_4_TDMA_SRC_VALID_ELEMENTS_0: u32 = 0x421114;
pub const mmDMA_CH_4_TDMA_SRC_START_OFFSET_0: u32 = 0x421118;
pub const mmDMA_CH_4_TDMA_SRC_STRIDE_0: u32 = 0x42111C;
pub const mmDMA_CH_4_TDMA_SRC_ROI_BASE_1: u32 = 0x421120;
pub const mmDMA_CH_4_TDMA_SRC_ROI_SIZE_1: u32 = 0x421124;
pub const mmDMA_CH_4_TDMA_SRC_VALID_ELEMENTS_1: u32 = 0x421128;
pub const mmDMA_CH_4_TDMA_SRC_START_OFFSET_1: u32 = 0x42112C;
pub const mmDMA_CH_4_TDMA_SRC_STRIDE_1: u32 = 0x421130;
pub const mmDMA_CH_4_TDMA_SRC_ROI_BASE_2: u32 = 0x421134;
pub const mmDMA_CH_4_TDMA_SRC_ROI_SIZE_2: u32 = 0x421138;
pub const mmDMA_CH_4_TDMA_SRC_VALID_ELEMENTS_2: u32 = 0x42113C;
pub const mmDMA_CH_4_TDMA_SRC_START_OFFSET_2: u32 = 0x421140;
pub const mmDMA_CH_4_TDMA_SRC_STRIDE_2: u32 = 0x421144;
pub const mmDMA_CH_4_TDMA_SRC_ROI_BASE_3: u32 = 0x421148;
pub const mmDMA_CH_4_TDMA_SRC_ROI_SIZE_3: u32 = 0x42114C;
pub const mmDMA_CH_4_TDMA_SRC_VALID_ELEMENTS_3: u32 = 0x421150;
pub const mmDMA_CH_4_TDMA_SRC_START_OFFSET_3: u32 = 0x421154;
pub const mmDMA_CH_4_TDMA_SRC_STRIDE_3: u32 = 0x421158;
pub const mmDMA_CH_4_TDMA_SRC_ROI_BASE_4: u32 = 0x42115C;
pub const mmDMA_CH_4_TDMA_SRC_ROI_SIZE_4: u32 = 0x421160;
pub const mmDMA_CH_4_TDMA_SRC_VALID_ELEMENTS_4: u32 = 0x421164;
pub const mmDMA_CH_4_TDMA_SRC_START_OFFSET_4: u32 = 0x421168;
pub const mmDMA_CH_4_TDMA_SRC_STRIDE_4: u32 = 0x42116C;
pub const mmDMA_CH_4_TDMA_DST_BASE_ADDR_LO: u32 = 0x421170;
pub const mmDMA_CH_4_TDMA_DST_BASE_ADDR_HI: u32 = 0x421174;
pub const mmDMA_CH_4_TDMA_DST_ROI_BASE_0: u32 = 0x421178;
pub const mmDMA_CH_4_TDMA_DST_ROI_SIZE_0: u32 = 0x42117C;
pub const mmDMA_CH_4_TDMA_DST_VALID_ELEMENTS_0: u32 = 0x421180;
pub const mmDMA_CH_4_TDMA_DST_START_OFFSET_0: u32 = 0x421184;
pub const mmDMA_CH_4_TDMA_DST_STRIDE_0: u32 = 0x421188;
pub const mmDMA_CH_4_TDMA_DST_ROI_BASE_1: u32 = 0x42118C;
pub const mmDMA_CH_4_TDMA_DST_ROI_SIZE_1: u32 = 0x421190;
pub const mmDMA_CH_4_TDMA_DST_VALID_ELEMENTS_1: u32 = 0x421194;
pub const mmDMA_CH_4_TDMA_DST_START_OFFSET_1: u32 = 0x421198;
pub const mmDMA_CH_4_TDMA_DST_STRIDE_1: u32 = 0x42119C;
pub const mmDMA_CH_4_TDMA_DST_ROI_BASE_2: u32 = 0x4211A0;
pub const mmDMA_CH_4_TDMA_DST_ROI_SIZE_2: u32 = 0x4211A4;
pub const mmDMA_CH_4_TDMA_DST_VALID_ELEMENTS_2: u32 = 0x4211A8;
pub const mmDMA_CH_4_TDMA_DST_START_OFFSET_2: u32 = 0x4211AC;
pub const mmDMA_CH_4_TDMA_DST_STRIDE_2: u32 = 0x4211B0;
pub const mmDMA_CH_4_TDMA_DST_ROI_BASE_3: u32 = 0x4211B4;
pub const mmDMA_CH_4_TDMA_DST_ROI_SIZE_3: u32 = 0x4211B8;
pub const mmDMA_CH_4_TDMA_DST_VALID_ELEMENTS_3: u32 = 0x4211BC;
pub const mmDMA_CH_4_TDMA_DST_START_OFFSET_3: u32 = 0x4211C0;
pub const mmDMA_CH_4_TDMA_DST_STRIDE_3: u32 = 0x4211C4;
pub const mmDMA_CH_4_TDMA_DST_ROI_BASE_4: u32 = 0x4211C8;
pub const mmDMA_CH_4_TDMA_DST_ROI_SIZE_4: u32 = 0x4211CC;
pub const mmDMA_CH_4_TDMA_DST_VALID_ELEMENTS_4: u32 = 0x4211D0;
pub const mmDMA_CH_4_TDMA_DST_START_OFFSET_4: u32 = 0x4211D4;
pub const mmDMA_CH_4_TDMA_DST_STRIDE_4: u32 = 0x4211D8;
pub const mmDMA_CH_4_MEM_INIT_BUSY: u32 = 0x4211FC;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
