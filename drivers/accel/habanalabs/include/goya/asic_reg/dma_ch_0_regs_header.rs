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
 *   DMA_CH_0 (Prototype: DMA_CH)
 *****************************************
 */

pub const mmDMA_CH_0_CFG0: u32 = 0x401000;
pub const mmDMA_CH_0_CFG1: u32 = 0x401004;
pub const mmDMA_CH_0_ERRMSG_ADDR_LO: u32 = 0x401008;
pub const mmDMA_CH_0_ERRMSG_ADDR_HI: u32 = 0x40100C;
pub const mmDMA_CH_0_ERRMSG_WDATA: u32 = 0x401010;
pub const mmDMA_CH_0_RD_COMP_ADDR_LO: u32 = 0x401014;
pub const mmDMA_CH_0_RD_COMP_ADDR_HI: u32 = 0x401018;
pub const mmDMA_CH_0_RD_COMP_WDATA: u32 = 0x40101C;
pub const mmDMA_CH_0_WR_COMP_ADDR_LO: u32 = 0x401020;
pub const mmDMA_CH_0_WR_COMP_ADDR_HI: u32 = 0x401024;
pub const mmDMA_CH_0_WR_COMP_WDATA: u32 = 0x401028;
pub const mmDMA_CH_0_LDMA_SRC_ADDR_LO: u32 = 0x40102C;
pub const mmDMA_CH_0_LDMA_SRC_ADDR_HI: u32 = 0x401030;
pub const mmDMA_CH_0_LDMA_DST_ADDR_LO: u32 = 0x401034;
pub const mmDMA_CH_0_LDMA_DST_ADDR_HI: u32 = 0x401038;
pub const mmDMA_CH_0_LDMA_TSIZE: u32 = 0x40103C;
pub const mmDMA_CH_0_COMIT_TRANSFER: u32 = 0x401040;
pub const mmDMA_CH_0_STS0: u32 = 0x401044;
pub const mmDMA_CH_0_STS1: u32 = 0x401048;
pub const mmDMA_CH_0_STS2: u32 = 0x40104C;
pub const mmDMA_CH_0_STS3: u32 = 0x401050;
pub const mmDMA_CH_0_STS4: u32 = 0x401054;
pub const mmDMA_CH_0_SRC_ADDR_LO_STS: u32 = 0x401058;
pub const mmDMA_CH_0_SRC_ADDR_HI_STS: u32 = 0x40105C;
pub const mmDMA_CH_0_SRC_TSIZE_STS: u32 = 0x401060;
pub const mmDMA_CH_0_DST_ADDR_LO_STS: u32 = 0x401064;
pub const mmDMA_CH_0_DST_ADDR_HI_STS: u32 = 0x401068;
pub const mmDMA_CH_0_DST_TSIZE_STS: u32 = 0x40106C;
pub const mmDMA_CH_0_RD_RATE_LIM_EN: u32 = 0x401070;
pub const mmDMA_CH_0_RD_RATE_LIM_RST_TOKEN: u32 = 0x401074;
pub const mmDMA_CH_0_RD_RATE_LIM_SAT: u32 = 0x401078;
pub const mmDMA_CH_0_RD_RATE_LIM_TOUT: u32 = 0x40107C;
pub const mmDMA_CH_0_WR_RATE_LIM_EN: u32 = 0x401080;
pub const mmDMA_CH_0_WR_RATE_LIM_RST_TOKEN: u32 = 0x401084;
pub const mmDMA_CH_0_WR_RATE_LIM_SAT: u32 = 0x401088;
pub const mmDMA_CH_0_WR_RATE_LIM_TOUT: u32 = 0x40108C;
pub const mmDMA_CH_0_CFG2: u32 = 0x401090;
pub const mmDMA_CH_0_TDMA_CTL: u32 = 0x401100;
pub const mmDMA_CH_0_TDMA_SRC_BASE_ADDR_LO: u32 = 0x401104;
pub const mmDMA_CH_0_TDMA_SRC_BASE_ADDR_HI: u32 = 0x401108;
pub const mmDMA_CH_0_TDMA_SRC_ROI_BASE_0: u32 = 0x40110C;
pub const mmDMA_CH_0_TDMA_SRC_ROI_SIZE_0: u32 = 0x401110;
pub const mmDMA_CH_0_TDMA_SRC_VALID_ELEMENTS_0: u32 = 0x401114;
pub const mmDMA_CH_0_TDMA_SRC_START_OFFSET_0: u32 = 0x401118;
pub const mmDMA_CH_0_TDMA_SRC_STRIDE_0: u32 = 0x40111C;
pub const mmDMA_CH_0_TDMA_SRC_ROI_BASE_1: u32 = 0x401120;
pub const mmDMA_CH_0_TDMA_SRC_ROI_SIZE_1: u32 = 0x401124;
pub const mmDMA_CH_0_TDMA_SRC_VALID_ELEMENTS_1: u32 = 0x401128;
pub const mmDMA_CH_0_TDMA_SRC_START_OFFSET_1: u32 = 0x40112C;
pub const mmDMA_CH_0_TDMA_SRC_STRIDE_1: u32 = 0x401130;
pub const mmDMA_CH_0_TDMA_SRC_ROI_BASE_2: u32 = 0x401134;
pub const mmDMA_CH_0_TDMA_SRC_ROI_SIZE_2: u32 = 0x401138;
pub const mmDMA_CH_0_TDMA_SRC_VALID_ELEMENTS_2: u32 = 0x40113C;
pub const mmDMA_CH_0_TDMA_SRC_START_OFFSET_2: u32 = 0x401140;
pub const mmDMA_CH_0_TDMA_SRC_STRIDE_2: u32 = 0x401144;
pub const mmDMA_CH_0_TDMA_SRC_ROI_BASE_3: u32 = 0x401148;
pub const mmDMA_CH_0_TDMA_SRC_ROI_SIZE_3: u32 = 0x40114C;
pub const mmDMA_CH_0_TDMA_SRC_VALID_ELEMENTS_3: u32 = 0x401150;
pub const mmDMA_CH_0_TDMA_SRC_START_OFFSET_3: u32 = 0x401154;
pub const mmDMA_CH_0_TDMA_SRC_STRIDE_3: u32 = 0x401158;
pub const mmDMA_CH_0_TDMA_SRC_ROI_BASE_4: u32 = 0x40115C;
pub const mmDMA_CH_0_TDMA_SRC_ROI_SIZE_4: u32 = 0x401160;
pub const mmDMA_CH_0_TDMA_SRC_VALID_ELEMENTS_4: u32 = 0x401164;
pub const mmDMA_CH_0_TDMA_SRC_START_OFFSET_4: u32 = 0x401168;
pub const mmDMA_CH_0_TDMA_SRC_STRIDE_4: u32 = 0x40116C;
pub const mmDMA_CH_0_TDMA_DST_BASE_ADDR_LO: u32 = 0x401170;
pub const mmDMA_CH_0_TDMA_DST_BASE_ADDR_HI: u32 = 0x401174;
pub const mmDMA_CH_0_TDMA_DST_ROI_BASE_0: u32 = 0x401178;
pub const mmDMA_CH_0_TDMA_DST_ROI_SIZE_0: u32 = 0x40117C;
pub const mmDMA_CH_0_TDMA_DST_VALID_ELEMENTS_0: u32 = 0x401180;
pub const mmDMA_CH_0_TDMA_DST_START_OFFSET_0: u32 = 0x401184;
pub const mmDMA_CH_0_TDMA_DST_STRIDE_0: u32 = 0x401188;
pub const mmDMA_CH_0_TDMA_DST_ROI_BASE_1: u32 = 0x40118C;
pub const mmDMA_CH_0_TDMA_DST_ROI_SIZE_1: u32 = 0x401190;
pub const mmDMA_CH_0_TDMA_DST_VALID_ELEMENTS_1: u32 = 0x401194;
pub const mmDMA_CH_0_TDMA_DST_START_OFFSET_1: u32 = 0x401198;
pub const mmDMA_CH_0_TDMA_DST_STRIDE_1: u32 = 0x40119C;
pub const mmDMA_CH_0_TDMA_DST_ROI_BASE_2: u32 = 0x4011A0;
pub const mmDMA_CH_0_TDMA_DST_ROI_SIZE_2: u32 = 0x4011A4;
pub const mmDMA_CH_0_TDMA_DST_VALID_ELEMENTS_2: u32 = 0x4011A8;
pub const mmDMA_CH_0_TDMA_DST_START_OFFSET_2: u32 = 0x4011AC;
pub const mmDMA_CH_0_TDMA_DST_STRIDE_2: u32 = 0x4011B0;
pub const mmDMA_CH_0_TDMA_DST_ROI_BASE_3: u32 = 0x4011B4;
pub const mmDMA_CH_0_TDMA_DST_ROI_SIZE_3: u32 = 0x4011B8;
pub const mmDMA_CH_0_TDMA_DST_VALID_ELEMENTS_3: u32 = 0x4011BC;
pub const mmDMA_CH_0_TDMA_DST_START_OFFSET_3: u32 = 0x4011C0;
pub const mmDMA_CH_0_TDMA_DST_STRIDE_3: u32 = 0x4011C4;
pub const mmDMA_CH_0_TDMA_DST_ROI_BASE_4: u32 = 0x4011C8;
pub const mmDMA_CH_0_TDMA_DST_ROI_SIZE_4: u32 = 0x4011CC;
pub const mmDMA_CH_0_TDMA_DST_VALID_ELEMENTS_4: u32 = 0x4011D0;
pub const mmDMA_CH_0_TDMA_DST_START_OFFSET_4: u32 = 0x4011D4;
pub const mmDMA_CH_0_TDMA_DST_STRIDE_4: u32 = 0x4011D8;
pub const mmDMA_CH_0_MEM_INIT_BUSY: u32 = 0x4011FC;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
