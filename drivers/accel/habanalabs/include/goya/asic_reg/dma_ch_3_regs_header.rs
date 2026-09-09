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
 *   DMA_CH_3 (Prototype: DMA_CH)
 *****************************************
 */

pub const mmDMA_CH_3_CFG0: u32 = 0x419000;
pub const mmDMA_CH_3_CFG1: u32 = 0x419004;
pub const mmDMA_CH_3_ERRMSG_ADDR_LO: u32 = 0x419008;
pub const mmDMA_CH_3_ERRMSG_ADDR_HI: u32 = 0x41900C;
pub const mmDMA_CH_3_ERRMSG_WDATA: u32 = 0x419010;
pub const mmDMA_CH_3_RD_COMP_ADDR_LO: u32 = 0x419014;
pub const mmDMA_CH_3_RD_COMP_ADDR_HI: u32 = 0x419018;
pub const mmDMA_CH_3_RD_COMP_WDATA: u32 = 0x41901C;
pub const mmDMA_CH_3_WR_COMP_ADDR_LO: u32 = 0x419020;
pub const mmDMA_CH_3_WR_COMP_ADDR_HI: u32 = 0x419024;
pub const mmDMA_CH_3_WR_COMP_WDATA: u32 = 0x419028;
pub const mmDMA_CH_3_LDMA_SRC_ADDR_LO: u32 = 0x41902C;
pub const mmDMA_CH_3_LDMA_SRC_ADDR_HI: u32 = 0x419030;
pub const mmDMA_CH_3_LDMA_DST_ADDR_LO: u32 = 0x419034;
pub const mmDMA_CH_3_LDMA_DST_ADDR_HI: u32 = 0x419038;
pub const mmDMA_CH_3_LDMA_TSIZE: u32 = 0x41903C;
pub const mmDMA_CH_3_COMIT_TRANSFER: u32 = 0x419040;
pub const mmDMA_CH_3_STS0: u32 = 0x419044;
pub const mmDMA_CH_3_STS1: u32 = 0x419048;
pub const mmDMA_CH_3_STS2: u32 = 0x41904C;
pub const mmDMA_CH_3_STS3: u32 = 0x419050;
pub const mmDMA_CH_3_STS4: u32 = 0x419054;
pub const mmDMA_CH_3_SRC_ADDR_LO_STS: u32 = 0x419058;
pub const mmDMA_CH_3_SRC_ADDR_HI_STS: u32 = 0x41905C;
pub const mmDMA_CH_3_SRC_TSIZE_STS: u32 = 0x419060;
pub const mmDMA_CH_3_DST_ADDR_LO_STS: u32 = 0x419064;
pub const mmDMA_CH_3_DST_ADDR_HI_STS: u32 = 0x419068;
pub const mmDMA_CH_3_DST_TSIZE_STS: u32 = 0x41906C;
pub const mmDMA_CH_3_RD_RATE_LIM_EN: u32 = 0x419070;
pub const mmDMA_CH_3_RD_RATE_LIM_RST_TOKEN: u32 = 0x419074;
pub const mmDMA_CH_3_RD_RATE_LIM_SAT: u32 = 0x419078;
pub const mmDMA_CH_3_RD_RATE_LIM_TOUT: u32 = 0x41907C;
pub const mmDMA_CH_3_WR_RATE_LIM_EN: u32 = 0x419080;
pub const mmDMA_CH_3_WR_RATE_LIM_RST_TOKEN: u32 = 0x419084;
pub const mmDMA_CH_3_WR_RATE_LIM_SAT: u32 = 0x419088;
pub const mmDMA_CH_3_WR_RATE_LIM_TOUT: u32 = 0x41908C;
pub const mmDMA_CH_3_CFG2: u32 = 0x419090;
pub const mmDMA_CH_3_TDMA_CTL: u32 = 0x419100;
pub const mmDMA_CH_3_TDMA_SRC_BASE_ADDR_LO: u32 = 0x419104;
pub const mmDMA_CH_3_TDMA_SRC_BASE_ADDR_HI: u32 = 0x419108;
pub const mmDMA_CH_3_TDMA_SRC_ROI_BASE_0: u32 = 0x41910C;
pub const mmDMA_CH_3_TDMA_SRC_ROI_SIZE_0: u32 = 0x419110;
pub const mmDMA_CH_3_TDMA_SRC_VALID_ELEMENTS_0: u32 = 0x419114;
pub const mmDMA_CH_3_TDMA_SRC_START_OFFSET_0: u32 = 0x419118;
pub const mmDMA_CH_3_TDMA_SRC_STRIDE_0: u32 = 0x41911C;
pub const mmDMA_CH_3_TDMA_SRC_ROI_BASE_1: u32 = 0x419128;
pub const mmDMA_CH_3_TDMA_SRC_ROI_SIZE_1: u32 = 0x41912C;
pub const mmDMA_CH_3_TDMA_SRC_VALID_ELEMENTS_1: u32 = 0x419130;
pub const mmDMA_CH_3_TDMA_SRC_START_OFFSET_1: u32 = 0x419134;
pub const mmDMA_CH_3_TDMA_SRC_STRIDE_1: u32 = 0x419138;
pub const mmDMA_CH_3_TDMA_SRC_ROI_BASE_2: u32 = 0x419144;
pub const mmDMA_CH_3_TDMA_SRC_ROI_SIZE_2: u32 = 0x419148;
pub const mmDMA_CH_3_TDMA_SRC_VALID_ELEMENTS_2: u32 = 0x41914C;
pub const mmDMA_CH_3_TDMA_SRC_START_OFFSET_2: u32 = 0x419150;
pub const mmDMA_CH_3_TDMA_SRC_STRIDE_2: u32 = 0x419154;
pub const mmDMA_CH_3_TDMA_SRC_ROI_BASE_3: u32 = 0x419160;
pub const mmDMA_CH_3_TDMA_SRC_ROI_SIZE_3: u32 = 0x419164;
pub const mmDMA_CH_3_TDMA_SRC_VALID_ELEMENTS_3: u32 = 0x419168;
pub const mmDMA_CH_3_TDMA_SRC_START_OFFSET_3: u32 = 0x41916C;
pub const mmDMA_CH_3_TDMA_SRC_STRIDE_3: u32 = 0x419170;
pub const mmDMA_CH_3_TDMA_SRC_ROI_BASE_4: u32 = 0x41917C;
pub const mmDMA_CH_3_TDMA_SRC_ROI_SIZE_4: u32 = 0x419180;
pub const mmDMA_CH_3_TDMA_SRC_VALID_ELEMENTS_4: u32 = 0x419184;
pub const mmDMA_CH_3_TDMA_SRC_START_OFFSET_4: u32 = 0x419188;
pub const mmDMA_CH_3_TDMA_SRC_STRIDE_4: u32 = 0x41918C;
pub const mmDMA_CH_3_TDMA_DST_ROI_BASE_0: u32 = 0x419178;
pub const mmDMA_CH_3_TDMA_DST_ROI_SIZE_0: u32 = 0x41917C;
pub const mmDMA_CH_3_TDMA_DST_VALID_ELEMENTS_0: u32 = 0x419180;
pub const mmDMA_CH_3_TDMA_DST_START_OFFSET_0: u32 = 0x419184;
pub const mmDMA_CH_3_TDMA_DST_STRIDE_0: u32 = 0x419188;
pub const mmDMA_CH_3_TDMA_DST_ROI_BASE_1: u32 = 0x419194;
pub const mmDMA_CH_3_TDMA_DST_ROI_SIZE_1: u32 = 0x419198;
pub const mmDMA_CH_3_TDMA_DST_VALID_ELEMENTS_1: u32 = 0x41919C;
pub const mmDMA_CH_3_TDMA_DST_START_OFFSET_1: u32 = 0x4191A0;
pub const mmDMA_CH_3_TDMA_DST_STRIDE_1: u32 = 0x4191A4;
pub const mmDMA_CH_3_TDMA_DST_ROI_BASE_2: u32 = 0x4191B0;
pub const mmDMA_CH_3_TDMA_DST_ROI_SIZE_2: u32 = 0x4191B4;
pub const mmDMA_CH_3_TDMA_DST_VALID_ELEMENTS_2: u32 = 0x4191B8;
pub const mmDMA_CH_3_TDMA_DST_START_OFFSET_2: u32 = 0x4191BC;
pub const mmDMA_CH_3_TDMA_DST_STRIDE_2: u32 = 0x4191C0;
pub const mmDMA_CH_3_TDMA_DST_ROI_BASE_3: u32 = 0x4191CC;
pub const mmDMA_CH_3_TDMA_DST_ROI_SIZE_3: u32 = 0x4191D0;
pub const mmDMA_CH_3_TDMA_DST_VALID_ELEMENTS_3: u32 = 0x4191D4;
pub const mmDMA_CH_3_TDMA_DST_START_OFFSET_3: u32 = 0x4191D8;
pub const mmDMA_CH_3_TDMA_DST_STRIDE_3: u32 = 0x4191DC;
pub const mmDMA_CH_3_TDMA_DST_ROI_BASE_4: u32 = 0x4191E8;
pub const mmDMA_CH_3_TDMA_DST_ROI_SIZE_4: u32 = 0x4191EC;
pub const mmDMA_CH_3_TDMA_DST_VALID_ELEMENTS_4: u32 = 0x4191F0;
pub const mmDMA_CH_3_TDMA_DST_START_OFFSET_4: u32 = 0x4191F4;
pub const mmDMA_CH_3_TDMA_DST_STRIDE_4: u32 = 0x4191F8;
pub const mmDMA_CH_3_MEM_INIT_BUSY: u32 = 0x4191FC;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
