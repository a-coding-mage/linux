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
 *   DMA4_CORE (Prototype: DMA_CORE)
 *****************************************
 */

pub const mmDMA4_CORE_CFG_0: u32 = 0x580000;
pub const mmDMA4_CORE_CFG_1: u32 = 0x580004;
pub const mmDMA4_CORE_LBW_MAX_OUTSTAND: u32 = 0x580008;
pub const mmDMA4_CORE_SRC_BASE_LO: u32 = 0x580014;
pub const mmDMA4_CORE_SRC_BASE_HI: u32 = 0x580018;
pub const mmDMA4_CORE_DST_BASE_LO: u32 = 0x58001C;
pub const mmDMA4_CORE_DST_BASE_HI: u32 = 0x580020;
pub const mmDMA4_CORE_SRC_TSIZE_1: u32 = 0x58002C;
pub const mmDMA4_CORE_SRC_STRIDE_1: u32 = 0x580030;
pub const mmDMA4_CORE_SRC_TSIZE_2: u32 = 0x580034;
pub const mmDMA4_CORE_SRC_STRIDE_2: u32 = 0x580038;
pub const mmDMA4_CORE_SRC_TSIZE_3: u32 = 0x58003C;
pub const mmDMA4_CORE_SRC_STRIDE_3: u32 = 0x580040;
pub const mmDMA4_CORE_SRC_TSIZE_4: u32 = 0x580044;
pub const mmDMA4_CORE_SRC_STRIDE_4: u32 = 0x580048;
pub const mmDMA4_CORE_SRC_TSIZE_0: u32 = 0x58004C;
pub const mmDMA4_CORE_DST_TSIZE_1: u32 = 0x580054;
pub const mmDMA4_CORE_DST_STRIDE_1: u32 = 0x580058;
pub const mmDMA4_CORE_DST_TSIZE_2: u32 = 0x58005C;
pub const mmDMA4_CORE_DST_STRIDE_2: u32 = 0x580060;
pub const mmDMA4_CORE_DST_TSIZE_3: u32 = 0x580064;
pub const mmDMA4_CORE_DST_STRIDE_3: u32 = 0x580068;
pub const mmDMA4_CORE_DST_TSIZE_4: u32 = 0x58006C;
pub const mmDMA4_CORE_DST_STRIDE_4: u32 = 0x580070;
pub const mmDMA4_CORE_DST_TSIZE_0: u32 = 0x580074;
pub const mmDMA4_CORE_COMMIT: u32 = 0x580078;
pub const mmDMA4_CORE_WR_COMP_WDATA: u32 = 0x58007C;
pub const mmDMA4_CORE_WR_COMP_ADDR_LO: u32 = 0x580080;
pub const mmDMA4_CORE_WR_COMP_ADDR_HI: u32 = 0x580084;
pub const mmDMA4_CORE_WR_COMP_AWUSER_31_11: u32 = 0x580088;
pub const mmDMA4_CORE_TE_NUMROWS: u32 = 0x580094;
pub const mmDMA4_CORE_PROT: u32 = 0x5800B8;
pub const mmDMA4_CORE_SECURE_PROPS: u32 = 0x5800F0;
pub const mmDMA4_CORE_NON_SECURE_PROPS: u32 = 0x5800F4;
pub const mmDMA4_CORE_RD_MAX_OUTSTAND: u32 = 0x580100;
pub const mmDMA4_CORE_RD_MAX_SIZE: u32 = 0x580104;
pub const mmDMA4_CORE_RD_ARCACHE: u32 = 0x580108;
pub const mmDMA4_CORE_RD_ARUSER_31_11: u32 = 0x580110;
pub const mmDMA4_CORE_RD_INFLIGHTS: u32 = 0x580114;
pub const mmDMA4_CORE_WR_MAX_OUTSTAND: u32 = 0x580120;
pub const mmDMA4_CORE_WR_MAX_AWID: u32 = 0x580124;
pub const mmDMA4_CORE_WR_AWCACHE: u32 = 0x580128;
pub const mmDMA4_CORE_WR_AWUSER_31_11: u32 = 0x580130;
pub const mmDMA4_CORE_WR_INFLIGHTS: u32 = 0x580134;
pub const mmDMA4_CORE_RD_RATE_LIM_CFG_0: u32 = 0x580150;
pub const mmDMA4_CORE_RD_RATE_LIM_CFG_1: u32 = 0x580154;
pub const mmDMA4_CORE_WR_RATE_LIM_CFG_0: u32 = 0x580158;
pub const mmDMA4_CORE_WR_RATE_LIM_CFG_1: u32 = 0x58015C;
pub const mmDMA4_CORE_ERR_CFG: u32 = 0x580160;
pub const mmDMA4_CORE_ERR_CAUSE: u32 = 0x580164;
pub const mmDMA4_CORE_ERRMSG_ADDR_LO: u32 = 0x580170;
pub const mmDMA4_CORE_ERRMSG_ADDR_HI: u32 = 0x580174;
pub const mmDMA4_CORE_ERRMSG_WDATA: u32 = 0x580178;
pub const mmDMA4_CORE_STS0: u32 = 0x580190;
pub const mmDMA4_CORE_STS1: u32 = 0x580194;
pub const mmDMA4_CORE_RD_DBGMEM_ADD: u32 = 0x580200;
pub const mmDMA4_CORE_RD_DBGMEM_DATA_WR: u32 = 0x580204;
pub const mmDMA4_CORE_RD_DBGMEM_DATA_RD: u32 = 0x580208;
pub const mmDMA4_CORE_RD_DBGMEM_CTRL: u32 = 0x58020C;
pub const mmDMA4_CORE_RD_DBGMEM_RC: u32 = 0x580210;
pub const mmDMA4_CORE_DBG_HBW_AXI_AR_CNT: u32 = 0x580220;
pub const mmDMA4_CORE_DBG_HBW_AXI_AW_CNT: u32 = 0x580224;
pub const mmDMA4_CORE_DBG_LBW_AXI_AW_CNT: u32 = 0x580228;
pub const mmDMA4_CORE_DBG_DESC_CNT: u32 = 0x58022C;
pub const mmDMA4_CORE_DBG_STS: u32 = 0x580230;
pub const mmDMA4_CORE_DBG_RD_DESC_ID: u32 = 0x580234;
pub const mmDMA4_CORE_DBG_WR_DESC_ID: u32 = 0x580238;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
