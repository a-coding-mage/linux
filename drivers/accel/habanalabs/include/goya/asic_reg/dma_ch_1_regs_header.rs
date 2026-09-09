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
 *   DMA_CH_1 (Prototype: DMA_CH)
 *****************************************
 */

pub const mmDMA_CH_1_CFG0: u32 = 0x409000;
pub const mmDMA_CH_1_CFG1: u32 = 0x409004;
pub const mmDMA_CH_1_ERRMSG_ADDR_LO: u32 = 0x409008;
pub const mmDMA_CH_1_ERRMSG_ADDR_HI: u32 = 0x40900C;
pub const mmDMA_CH_1_ERRMSG_WDATA: u32 = 0x409010;
pub const mmDMA_CH_1_RD_COMP_ADDR_LO: u32 = 0x409014;
pub const mmDMA_CH_1_RD_COMP_ADDR_HI: u32 = 0x409018;
pub const mmDMA_CH_1_RD_COMP_WDATA: u32 = 0x40901C;
pub const mmDMA_CH_1_WR_COMP_ADDR_LO: u32 = 0x409020;
pub const mmDMA_CH_1_WR_COMP_ADDR_HI: u32 = 0x409024;
pub const mmDMA_CH_1_WR_COMP_WDATA: u32 = 0x409028;
pub const mmDMA_CH_1_LDMA_SRC_ADDR_LO: u32 = 0x40902C;
pub const mmDMA_CH_1_LDMA_SRC_ADDR_HI: u32 = 0x409030;
pub const mmDMA_CH_1_LDMA_DST_ADDR_LO: u32 = 0x409034;
pub const mmDMA_CH_1_LDMA_DST_ADDR_HI: u32 = 0x409038;
pub const mmDMA_CH_1_LDMA_TSIZE: u32 = 0x40903C;
pub const mmDMA_CH_1_COMIT_TRANSFER: u32 = 0x409040;
pub const mmDMA_CH_1_STS0: u32 = 0x409044;
pub const mmDMA_CH_1_STS1: u32 = 0x409048;
pub const mmDMA_CH_1_STS2: u32 = 0x40904C;
pub const mmDMA_CH_1_STS3: u32 = 0x409050;
pub const mmDMA_CH_1_STS4: u32 = 0x409054;
pub const mmDMA_CH_1_SRC_ADDR_LO_STS: u32 = 0x409058;
pub const mmDMA_CH_1_SRC_ADDR_HI_STS: u32 = 0x40905C;
pub const mmDMA_CH_1_SRC_TSIZE_STS: u32 = 0x409060;
pub const mmDMA_CH_1_DST_ADDR_LO_STS: u32 = 0x409064;
pub const mmDMA_CH_1_DST_ADDR_HI_STS: u32 = 0x409068;
pub const mmDMA_CH_1_DST_TSIZE_STS: u32 = 0x40906C;
pub const mmDMA_CH_1_RD_RATE_LIM_EN: u32 = 0x409070;
pub const mmDMA_CH_1_RD_RATE_LIM_RST_TOKEN: u32 = 0x409074;
pub const mmDMA_CH_1_RD_RATE_LIM_SAT: u32 = 0x409078;
pub const mmDMA_CH_1_RD_RATE_LIM_TOUT: u32 = 0x40907C;
pub const mmDMA_CH_1_WR_RATE_LIM_EN: u32 = 0x409080;
pub const mmDMA_CH_1_WR_RATE_LIM_RST_TOKEN: u32 = 0x409084;
pub const mmDMA_CH_1_WR_RATE_LIM_SAT: u32 = 0x409088;
pub const mmDMA_CH_1_WR_RATE_LIM_TOUT: u32 = 0x40908C;
pub const mmDMA_CH_1_CFG2: u32 = 0x409090;
pub const mmDMA_CH_1_TDMA_CTL: u32 = 0x409100;
pub const mmDMA_CH_1_TDMA_SRC_BASE_ADDR_LO: u32 = 0x409104;
pub const mmDMA_CH_1_TDMA_SRC_BASE_ADDR_HI: u32 = 0x409108;
pub const mmDMA_CH_1_TDMA_DST_BASE_ADDR_LO: u32 = 0x409170;
pub const mmDMA_CH_1_TDMA_DST_BASE_ADDR_HI: u32 = 0x409174;
pub const mmDMA_CH_1_MEM_INIT_BUSY: u32 = 0x4091FC;

// The following ROI register blocks are direct translations of the C macros.
pub const mmDMA_CH_1_TDMA_SRC_ROI_BASE_0: u32 = 0x40910C;
pub const mmDMA_CH_1_TDMA_SRC_ROI_SIZE_0: u32 = 0x409110;
pub const mmDMA_CH_1_TDMA_SRC_VALID_ELEMENTS_0: u32 = 0x409114;
pub const mmDMA_CH_1_TDMA_SRC_START_OFFSET_0: u32 = 0x409118;
pub const mmDMA_CH_1_TDMA_SRC_STRIDE_0: u32 = 0x40911C;
pub const mmDMA_CH_1_TDMA_SRC_ROI_BASE_1: u32 = 0x409120;
pub const mmDMA_CH_1_TDMA_SRC_ROI_SIZE_1: u32 = 0x409124;
pub const mmDMA_CH_1_TDMA_SRC_VALID_ELEMENTS_1: u32 = 0x409128;
pub const mmDMA_CH_1_TDMA_SRC_START_OFFSET_1: u32 = 0x40912C;
pub const mmDMA_CH_1_TDMA_SRC_STRIDE_1: u32 = 0x409130;
pub const mmDMA_CH_1_TDMA_SRC_ROI_BASE_2: u32 = 0x409134;
pub const mmDMA_CH_1_TDMA_SRC_ROI_SIZE_2: u32 = 0x409138;
pub const mmDMA_CH_1_TDMA_SRC_VALID_ELEMENTS_2: u32 = 0x40913C;
pub const mmDMA_CH_1_TDMA_SRC_START_OFFSET_2: u32 = 0x409140;
pub const mmDMA_CH_1_TDMA_SRC_STRIDE_2: u32 = 0x409144;
pub const mmDMA_CH_1_TDMA_SRC_ROI_BASE_3: u32 = 0x409148;
pub const mmDMA_CH_1_TDMA_SRC_ROI_SIZE_3: u32 = 0x40914C;
pub const mmDMA_CH_1_TDMA_SRC_VALID_ELEMENTS_3: u32 = 0x409150;
pub const mmDMA_CH_1_TDMA_SRC_START_OFFSET_3: u32 = 0x409154;
pub const mmDMA_CH_1_TDMA_SRC_STRIDE_3: u32 = 0x409158;
pub const mmDMA_CH_1_TDMA_SRC_ROI_BASE_4: u32 = 0x40915C;
pub const mmDMA_CH_1_TDMA_SRC_ROI_SIZE_4: u32 = 0x409160;
pub const mmDMA_CH_1_TDMA_SRC_VALID_ELEMENTS_4: u32 = 0x409164;
pub const mmDMA_CH_1_TDMA_SRC_START_OFFSET_4: u32 = 0x409168;
pub const mmDMA_CH_1_TDMA_SRC_STRIDE_4: u32 = 0x40916C;
pub const mmDMA_CH_1_TDMA_DST_ROI_BASE_0: u32 = 0x409178;
pub const mmDMA_CH_1_TDMA_DST_ROI_SIZE_0: u32 = 0x40917C;
pub const mmDMA_CH_1_TDMA_DST_VALID_ELEMENTS_0: u32 = 0x409180;
pub const mmDMA_CH_1_TDMA_DST_START_OFFSET_0: u32 = 0x409184;
pub const mmDMA_CH_1_TDMA_DST_STRIDE_0: u32 = 0x409188;
pub const mmDMA_CH_1_TDMA_DST_ROI_BASE_1: u32 = 0x40918C;
pub const mmDMA_CH_1_TDMA_DST_ROI_SIZE_1: u32 = 0x409190;
pub const mmDMA_CH_1_TDMA_DST_VALID_ELEMENTS_1: u32 = 0x409194;
pub const mmDMA_CH_1_TDMA_DST_START_OFFSET_1: u32 = 0x409198;
pub const mmDMA_CH_1_TDMA_DST_STRIDE_1: u32 = 0x40919C;
pub const mmDMA_CH_1_TDMA_DST_ROI_BASE_2: u32 = 0x4091A0;
pub const mmDMA_CH_1_TDMA_DST_ROI_SIZE_2: u32 = 0x4091A4;
pub const mmDMA_CH_1_TDMA_DST_VALID_ELEMENTS_2: u32 = 0x4091A8;
pub const mmDMA_CH_1_TDMA_DST_START_OFFSET_2: u32 = 0x4091AC;
pub const mmDMA_CH_1_TDMA_DST_STRIDE_2: u32 = 0x4091B0;
pub const mmDMA_CH_1_TDMA_DST_ROI_BASE_3: u32 = 0x4091B4;
pub const mmDMA_CH_1_TDMA_DST_ROI_SIZE_3: u32 = 0x4091B8;
pub const mmDMA_CH_1_TDMA_DST_VALID_ELEMENTS_3: u32 = 0x4091BC;
pub const mmDMA_CH_1_TDMA_DST_START_OFFSET_3: u32 = 0x4091C0;
pub const mmDMA_CH_1_TDMA_DST_STRIDE_3: u32 = 0x4091C4;
pub const mmDMA_CH_1_TDMA_DST_ROI_BASE_4: u32 = 0x4091C8;
pub const mmDMA_CH_1_TDMA_DST_ROI_SIZE_4: u32 = 0x4091CC;
pub const mmDMA_CH_1_TDMA_DST_VALID_ELEMENTS_4: u32 = 0x4091D0;
pub const mmDMA_CH_1_TDMA_DST_START_OFFSET_4: u32 = 0x4091D4;
pub const mmDMA_CH_1_TDMA_DST_STRIDE_4: u32 = 0x4091D8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
