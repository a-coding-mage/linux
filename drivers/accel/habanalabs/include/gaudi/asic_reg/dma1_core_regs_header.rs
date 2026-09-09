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
 *   DMA1_CORE (Prototype: DMA_CORE)
 *****************************************
 */

pub const mmDMA1_CORE_CFG_0: u32 = 0x520000;
pub const mmDMA1_CORE_CFG_1: u32 = 0x520004;
pub const mmDMA1_CORE_LBW_MAX_OUTSTAND: u32 = 0x520008;
pub const mmDMA1_CORE_SRC_BASE_LO: u32 = 0x520014;
pub const mmDMA1_CORE_SRC_BASE_HI: u32 = 0x520018;
pub const mmDMA1_CORE_DST_BASE_LO: u32 = 0x52001C;
pub const mmDMA1_CORE_DST_BASE_HI: u32 = 0x520020;
pub const mmDMA1_CORE_SRC_TSIZE_1: u32 = 0x52002C;
pub const mmDMA1_CORE_SRC_STRIDE_1: u32 = 0x520030;
pub const mmDMA1_CORE_SRC_TSIZE_2: u32 = 0x520034;
pub const mmDMA1_CORE_SRC_STRIDE_2: u32 = 0x520038;
pub const mmDMA1_CORE_SRC_TSIZE_3: u32 = 0x52003C;
pub const mmDMA1_CORE_SRC_STRIDE_3: u32 = 0x520040;
pub const mmDMA1_CORE_SRC_TSIZE_4: u32 = 0x520044;
pub const mmDMA1_CORE_SRC_STRIDE_4: u32 = 0x520048;
pub const mmDMA1_CORE_SRC_TSIZE_0: u32 = 0x52004C;
pub const mmDMA1_CORE_DST_TSIZE_1: u32 = 0x520054;
pub const mmDMA1_CORE_DST_STRIDE_1: u32 = 0x520058;
pub const mmDMA1_CORE_DST_TSIZE_2: u32 = 0x52005C;
pub const mmDMA1_CORE_DST_STRIDE_2: u32 = 0x520060;
pub const mmDMA1_CORE_DST_TSIZE_3: u32 = 0x520064;
pub const mmDMA1_CORE_DST_STRIDE_3: u32 = 0x520068;
pub const mmDMA1_CORE_DST_TSIZE_4: u32 = 0x52006C;
pub const mmDMA1_CORE_DST_STRIDE_4: u32 = 0x520070;
pub const mmDMA1_CORE_DST_TSIZE_0: u32 = 0x520074;
pub const mmDMA1_CORE_COMMIT: u32 = 0x520078;
pub const mmDMA1_CORE_WR_COMP_WDATA: u32 = 0x52007C;
pub const mmDMA1_CORE_WR_COMP_ADDR_LO: u32 = 0x520080;
pub const mmDMA1_CORE_WR_COMP_ADDR_HI: u32 = 0x520084;
pub const mmDMA1_CORE_WR_COMP_AWUSER_31_11: u32 = 0x520088;
pub const mmDMA1_CORE_TE_NUMROWS: u32 = 0x520094;
pub const mmDMA1_CORE_PROT: u32 = 0x5200B8;
pub const mmDMA1_CORE_SECURE_PROPS: u32 = 0x5200F0;
pub const mmDMA1_CORE_NON_SECURE_PROPS: u32 = 0x5200F4;
pub const mmDMA1_CORE_RD_MAX_OUTSTAND: u32 = 0x520100;
pub const mmDMA1_CORE_RD_MAX_SIZE: u32 = 0x520104;
pub const mmDMA1_CORE_RD_ARCACHE: u32 = 0x520108;
pub const mmDMA1_CORE_RD_ARUSER_31_11: u32 = 0x520110;
pub const mmDMA1_CORE_RD_INFLIGHTS: u32 = 0x520114;
pub const mmDMA1_CORE_WR_MAX_OUTSTAND: u32 = 0x520120;
pub const mmDMA1_CORE_WR_MAX_AWID: u32 = 0x520124;
pub const mmDMA1_CORE_WR_AWCACHE: u32 = 0x520128;
pub const mmDMA1_CORE_WR_AWUSER_31_11: u32 = 0x520130;
pub const mmDMA1_CORE_WR_INFLIGHTS: u32 = 0x520134;
pub const mmDMA1_CORE_RD_RATE_LIM_CFG_0: u32 = 0x520150;
pub const mmDMA1_CORE_RD_RATE_LIM_CFG_1: u32 = 0x520154;
pub const mmDMA1_CORE_WR_RATE_LIM_CFG_0: u32 = 0x520158;
pub const mmDMA1_CORE_WR_RATE_LIM_CFG_1: u32 = 0x52015C;
pub const mmDMA1_CORE_ERR_CFG: u32 = 0x520160;
pub const mmDMA1_CORE_ERR_CAUSE: u32 = 0x520164;
pub const mmDMA1_CORE_ERRMSG_ADDR_LO: u32 = 0x520170;
pub const mmDMA1_CORE_ERRMSG_ADDR_HI: u32 = 0x520174;
pub const mmDMA1_CORE_ERRMSG_WDATA: u32 = 0x520178;
pub const mmDMA1_CORE_STS0: u32 = 0x520190;
pub const mmDMA1_CORE_STS1: u32 = 0x520194;
pub const mmDMA1_CORE_RD_DBGMEM_ADD: u32 = 0x520200;
pub const mmDMA1_CORE_RD_DBGMEM_DATA_WR: u32 = 0x520204;
pub const mmDMA1_CORE_RD_DBGMEM_DATA_RD: u32 = 0x520208;
pub const mmDMA1_CORE_RD_DBGMEM_CTRL: u32 = 0x52020C;
pub const mmDMA1_CORE_RD_DBGMEM_RC: u32 = 0x520210;
pub const mmDMA1_CORE_DBG_HBW_AXI_AR_CNT: u32 = 0x520220;
pub const mmDMA1_CORE_DBG_HBW_AXI_AW_CNT: u32 = 0x520224;
pub const mmDMA1_CORE_DBG_LBW_AXI_AW_CNT: u32 = 0x520228;
pub const mmDMA1_CORE_DBG_DESC_CNT: u32 = 0x52022C;
pub const mmDMA1_CORE_DBG_STS: u32 = 0x520230;
pub const mmDMA1_CORE_DBG_RD_DESC_ID: u32 = 0x520234;
pub const mmDMA1_CORE_DBG_WR_DESC_ID: u32 = 0x520238;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
