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
 *   DMA0_CORE (Prototype: DMA_CORE)
 *****************************************
 */

pub const mmDMA0_CORE_CFG_0: u32 = 0x500000;
pub const mmDMA0_CORE_CFG_1: u32 = 0x500004;
pub const mmDMA0_CORE_LBW_MAX_OUTSTAND: u32 = 0x500008;
pub const mmDMA0_CORE_SRC_BASE_LO: u32 = 0x500014;
pub const mmDMA0_CORE_SRC_BASE_HI: u32 = 0x500018;
pub const mmDMA0_CORE_DST_BASE_LO: u32 = 0x50001C;
pub const mmDMA0_CORE_DST_BASE_HI: u32 = 0x500020;
pub const mmDMA0_CORE_SRC_TSIZE_1: u32 = 0x50002C;
pub const mmDMA0_CORE_SRC_STRIDE_1: u32 = 0x500030;
pub const mmDMA0_CORE_SRC_TSIZE_2: u32 = 0x500034;
pub const mmDMA0_CORE_SRC_STRIDE_2: u32 = 0x500038;
pub const mmDMA0_CORE_SRC_TSIZE_3: u32 = 0x50003C;
pub const mmDMA0_CORE_SRC_STRIDE_3: u32 = 0x500040;
pub const mmDMA0_CORE_SRC_TSIZE_4: u32 = 0x500044;
pub const mmDMA0_CORE_SRC_STRIDE_4: u32 = 0x500048;
pub const mmDMA0_CORE_SRC_TSIZE_0: u32 = 0x50004C;
pub const mmDMA0_CORE_DST_TSIZE_1: u32 = 0x500054;
pub const mmDMA0_CORE_DST_STRIDE_1: u32 = 0x500058;
pub const mmDMA0_CORE_DST_TSIZE_2: u32 = 0x50005C;
pub const mmDMA0_CORE_DST_STRIDE_2: u32 = 0x500060;
pub const mmDMA0_CORE_DST_TSIZE_3: u32 = 0x500064;
pub const mmDMA0_CORE_DST_STRIDE_3: u32 = 0x500068;
pub const mmDMA0_CORE_DST_TSIZE_4: u32 = 0x50006C;
pub const mmDMA0_CORE_DST_STRIDE_4: u32 = 0x500070;
pub const mmDMA0_CORE_DST_TSIZE_0: u32 = 0x500074;
pub const mmDMA0_CORE_COMMIT: u32 = 0x500078;
pub const mmDMA0_CORE_WR_COMP_WDATA: u32 = 0x50007C;
pub const mmDMA0_CORE_WR_COMP_ADDR_LO: u32 = 0x500080;
pub const mmDMA0_CORE_WR_COMP_ADDR_HI: u32 = 0x500084;
pub const mmDMA0_CORE_WR_COMP_AWUSER_31_11: u32 = 0x500088;
pub const mmDMA0_CORE_TE_NUMROWS: u32 = 0x500094;
pub const mmDMA0_CORE_PROT: u32 = 0x5000B8;
pub const mmDMA0_CORE_SECURE_PROPS: u32 = 0x5000F0;
pub const mmDMA0_CORE_NON_SECURE_PROPS: u32 = 0x5000F4;
pub const mmDMA0_CORE_RD_MAX_OUTSTAND: u32 = 0x500100;
pub const mmDMA0_CORE_RD_MAX_SIZE: u32 = 0x500104;
pub const mmDMA0_CORE_RD_ARCACHE: u32 = 0x500108;
pub const mmDMA0_CORE_RD_ARUSER_31_11: u32 = 0x500110;
pub const mmDMA0_CORE_RD_INFLIGHTS: u32 = 0x500114;
pub const mmDMA0_CORE_WR_MAX_OUTSTAND: u32 = 0x500120;
pub const mmDMA0_CORE_WR_MAX_AWID: u32 = 0x500124;
pub const mmDMA0_CORE_WR_AWCACHE: u32 = 0x500128;
pub const mmDMA0_CORE_WR_AWUSER_31_11: u32 = 0x500130;
pub const mmDMA0_CORE_WR_INFLIGHTS: u32 = 0x500134;
pub const mmDMA0_CORE_RD_RATE_LIM_CFG_0: u32 = 0x500150;
pub const mmDMA0_CORE_RD_RATE_LIM_CFG_1: u32 = 0x500154;
pub const mmDMA0_CORE_WR_RATE_LIM_CFG_0: u32 = 0x500158;
pub const mmDMA0_CORE_WR_RATE_LIM_CFG_1: u32 = 0x50015C;
pub const mmDMA0_CORE_ERR_CFG: u32 = 0x500160;
pub const mmDMA0_CORE_ERR_CAUSE: u32 = 0x500164;
pub const mmDMA0_CORE_ERRMSG_ADDR_LO: u32 = 0x500170;
pub const mmDMA0_CORE_ERRMSG_ADDR_HI: u32 = 0x500174;
pub const mmDMA0_CORE_ERRMSG_WDATA: u32 = 0x500178;
pub const mmDMA0_CORE_STS0: u32 = 0x500190;
pub const mmDMA0_CORE_STS1: u32 = 0x500194;
pub const mmDMA0_CORE_RD_DBGMEM_ADD: u32 = 0x500200;
pub const mmDMA0_CORE_RD_DBGMEM_DATA_WR: u32 = 0x500204;
pub const mmDMA0_CORE_RD_DBGMEM_DATA_RD: u32 = 0x500208;
pub const mmDMA0_CORE_RD_DBGMEM_CTRL: u32 = 0x50020C;
pub const mmDMA0_CORE_RD_DBGMEM_RC: u32 = 0x500210;
pub const mmDMA0_CORE_DBG_HBW_AXI_AR_CNT: u32 = 0x500220;
pub const mmDMA0_CORE_DBG_HBW_AXI_AW_CNT: u32 = 0x500224;
pub const mmDMA0_CORE_DBG_LBW_AXI_AW_CNT: u32 = 0x500228;
pub const mmDMA0_CORE_DBG_DESC_CNT: u32 = 0x50022C;
pub const mmDMA0_CORE_DBG_STS: u32 = 0x500230;
pub const mmDMA0_CORE_DBG_RD_DESC_ID: u32 = 0x500234;
pub const mmDMA0_CORE_DBG_WR_DESC_ID: u32 = 0x500238;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
