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
 *   DMA_CH_2 (Prototype: DMA_CH)
 *****************************************
 */

pub const mmDMA_CH_2_CFG0: u32 = 0x411000;
pub const mmDMA_CH_2_CFG1: u32 = 0x411004;
pub const mmDMA_CH_2_ERRMSG_ADDR_LO: u32 = 0x411008;
pub const mmDMA_CH_2_ERRMSG_ADDR_HI: u32 = 0x41100C;
pub const mmDMA_CH_2_ERRMSG_WDATA: u32 = 0x411010;
pub const mmDMA_CH_2_RD_COMP_ADDR_LO: u32 = 0x411014;
pub const mmDMA_CH_2_RD_COMP_ADDR_HI: u32 = 0x411018;
pub const mmDMA_CH_2_RD_COMP_WDATA: u32 = 0x41101C;
pub const mmDMA_CH_2_WR_COMP_ADDR_LO: u32 = 0x411020;
pub const mmDMA_CH_2_WR_COMP_ADDR_HI: u32 = 0x411024;
pub const mmDMA_CH_2_WR_COMP_WDATA: u32 = 0x411028;
pub const mmDMA_CH_2_LDMA_SRC_ADDR_LO: u32 = 0x41102C;
pub const mmDMA_CH_2_LDMA_SRC_ADDR_HI: u32 = 0x411030;
pub const mmDMA_CH_2_LDMA_DST_ADDR_LO: u32 = 0x411034;
pub const mmDMA_CH_2_LDMA_DST_ADDR_HI: u32 = 0x411038;
pub const mmDMA_CH_2_LDMA_TSIZE: u32 = 0x41103C;
pub const mmDMA_CH_2_COMIT_TRANSFER: u32 = 0x411040;
pub const mmDMA_CH_2_STS0: u32 = 0x411044;
pub const mmDMA_CH_2_STS1: u32 = 0x411048;
pub const mmDMA_CH_2_STS2: u32 = 0x41104C;
pub const mmDMA_CH_2_STS3: u32 = 0x411050;
pub const mmDMA_CH_2_STS4: u32 = 0x411054;
pub const mmDMA_CH_2_SRC_ADDR_LO_STS: u32 = 0x411058;
pub const mmDMA_CH_2_SRC_ADDR_HI_STS: u32 = 0x41105C;
pub const mmDMA_CH_2_SRC_TSIZE_STS: u32 = 0x411060;
pub const mmDMA_CH_2_DST_ADDR_LO_STS: u32 = 0x411064;
pub const mmDMA_CH_2_DST_ADDR_HI_STS: u32 = 0x411068;
pub const mmDMA_CH_2_DST_TSIZE_STS: u32 = 0x41106C;
pub const mmDMA_CH_2_RD_RATE_LIM_EN: u32 = 0x411070;
pub const mmDMA_CH_2_RD_RATE_LIM_RST_TOKEN: u32 = 0x411074;
pub const mmDMA_CH_2_RD_RATE_LIM_SAT: u32 = 0x411078;
pub const mmDMA_CH_2_RD_RATE_LIM_TOUT: u32 = 0x41107C;
pub const mmDMA_CH_2_WR_RATE_LIM_EN: u32 = 0x411080;
pub const mmDMA_CH_2_WR_RATE_LIM_RST_TOKEN: u32 = 0x411084;
pub const mmDMA_CH_2_WR_RATE_LIM_SAT: u32 = 0x411088;
pub const mmDMA_CH_2_WR_RATE_LIM_TOUT: u32 = 0x41108C;
pub const mmDMA_CH_2_CFG2: u32 = 0x411090;
pub const mmDMA_CH_2_TDMA_CTL: u32 = 0x411100;
pub const mmDMA_CH_2_TDMA_SRC_BASE_ADDR_LO: u32 = 0x411104;
pub const mmDMA_CH_2_TDMA_SRC_BASE_ADDR_HI: u32 = 0x411108;
pub const mmDMA_CH_2_TDMA_SRC_ROI_BASE_0: u32 = 0x41110C;
pub const mmDMA_CH_2_TDMA_SRC_ROI_SIZE_0: u32 = 0x411110;
pub const mmDMA_CH_2_TDMA_SRC_VALID_ELEMENTS_0: u32 = 0x411114;
pub const mmDMA_CH_2_TDMA_SRC_START_OFFSET_0: u32 = 0x411118;
pub const mmDMA_CH_2_TDMA_SRC_STRIDE_0: u32 = 0x41111C;
pub const mmDMA_CH_2_TDMA_SRC_ROI_BASE_1: u32 = 0x411120;
pub const mmDMA_CH_2_TDMA_SRC_ROI_SIZE_1: u32 = 0x411124;
pub const mmDMA_CH_2_TDMA_SRC_VALID_ELEMENTS_1: u32 = 0x411128;
pub const mmDMA_CH_2_TDMA_SRC_START_OFFSET_1: u32 = 0x41112C;
pub const mmDMA_CH_2_TDMA_SRC_STRIDE_1: u32 = 0x411130;
pub const mmDMA_CH_2_TDMA_SRC_ROI_BASE_2: u32 = 0x411134;
pub const mmDMA_CH_2_TDMA_SRC_ROI_SIZE_2: u32 = 0x411138;
pub const mmDMA_CH_2_TDMA_SRC_VALID_ELEMENTS_2: u32 = 0x41113C;
pub const mmDMA_CH_2_TDMA_SRC_START_OFFSET_2: u32 = 0x411140;
pub const mmDMA_CH_2_TDMA_SRC_STRIDE_2: u32 = 0x411144;
pub const mmDMA_CH_2_TDMA_SRC_ROI_BASE_3: u32 = 0x411148;
pub const mmDMA_CH_2_TDMA_SRC_ROI_SIZE_3: u32 = 0x41114C;
pub const mmDMA_CH_2_TDMA_SRC_VALID_ELEMENTS_3: u32 = 0x411150;
pub const mmDMA_CH_2_TDMA_SRC_START_OFFSET_3: u32 = 0x411154;
pub const mmDMA_CH_2_TDMA_SRC_STRIDE_3: u32 = 0x411158;
pub const mmDMA_CH_2_TDMA_SRC_ROI_BASE_4: u32 = 0x41115C;
pub const mmDMA_CH_2_TDMA_SRC_ROI_SIZE_4: u32 = 0x411160;
pub const mmDMA_CH_2_TDMA_SRC_VALID_ELEMENTS_4: u32 = 0x411164;
pub const mmDMA_CH_2_TDMA_SRC_START_OFFSET_4: u32 = 0x411168;
pub const mmDMA_CH_2_TDMA_SRC_STRIDE_4: u32 = 0x41116C;
pub const mmDMA_CH_2_TDMA_DST_BASE_ADDR_LO: u32 = 0x411170;
pub const mmDMA_CH_2_TDMA_DST_BASE_ADDR_HI: u32 = 0x411174;
pub const mmDMA_CH_2_TDMA_DST_ROI_BASE_0: u32 = 0x411178;
pub const mmDMA_CH_2_TDMA_DST_ROI_SIZE_0: u32 = 0x41117C;
pub const mmDMA_CH_2_TDMA_DST_VALID_ELEMENTS_0: u32 = 0x411180;
pub const mmDMA_CH_2_TDMA_DST_START_OFFSET_0: u32 = 0x411184;
pub const mmDMA_CH_2_TDMA_DST_STRIDE_0: u32 = 0x411188;
pub const mmDMA_CH_2_TDMA_DST_ROI_BASE_1: u32 = 0x41118C;
pub const mmDMA_CH_2_TDMA_DST_ROI_SIZE_1: u32 = 0x411190;
pub const mmDMA_CH_2_TDMA_DST_VALID_ELEMENTS_1: u32 = 0x411194;
pub const mmDMA_CH_2_TDMA_DST_START_OFFSET_1: u32 = 0x411198;
pub const mmDMA_CH_2_TDMA_DST_STRIDE_1: u32 = 0x41119C;
pub const mmDMA_CH_2_TDMA_DST_ROI_BASE_2: u32 = 0x4111A0;
pub const mmDMA_CH_2_TDMA_DST_ROI_SIZE_2: u32 = 0x4111A4;
pub const mmDMA_CH_2_TDMA_DST_VALID_ELEMENTS_2: u32 = 0x4111A8;
pub const mmDMA_CH_2_TDMA_DST_START_OFFSET_2: u32 = 0x4111AC;
pub const mmDMA_CH_2_TDMA_DST_STRIDE_2: u32 = 0x4111B0;
pub const mmDMA_CH_2_TDMA_DST_ROI_BASE_3: u32 = 0x4111B4;
pub const mmDMA_CH_2_TDMA_DST_ROI_SIZE_3: u32 = 0x4111B8;
pub const mmDMA_CH_2_TDMA_DST_VALID_ELEMENTS_3: u32 = 0x4111BC;
pub const mmDMA_CH_2_TDMA_DST_START_OFFSET_3: u32 = 0x4111C0;
pub const mmDMA_CH_2_TDMA_DST_STRIDE_3: u32 = 0x4111C4;
pub const mmDMA_CH_2_TDMA_DST_ROI_BASE_4: u32 = 0x4111C8;
pub const mmDMA_CH_2_TDMA_DST_ROI_SIZE_4: u32 = 0x4111CC;
pub const mmDMA_CH_2_TDMA_DST_VALID_ELEMENTS_4: u32 = 0x4111D0;
pub const mmDMA_CH_2_TDMA_DST_START_OFFSET_4: u32 = 0x4111D4;
pub const mmDMA_CH_2_TDMA_DST_STRIDE_4: u32 = 0x4111D8;
pub const mmDMA_CH_2_MEM_INIT_BUSY: u32 = 0x4111FC;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
