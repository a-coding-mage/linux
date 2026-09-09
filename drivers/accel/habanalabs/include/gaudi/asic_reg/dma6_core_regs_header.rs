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
 *   DMA6_CORE (Prototype: DMA_CORE)
 *****************************************
 */

pub const mmDMA6_CORE_CFG_0: u32 = 0x5C0000;
pub const mmDMA6_CORE_CFG_1: u32 = 0x5C0004;
pub const mmDMA6_CORE_LBW_MAX_OUTSTAND: u32 = 0x5C0008;
pub const mmDMA6_CORE_SRC_BASE_LO: u32 = 0x5C0014;
pub const mmDMA6_CORE_SRC_BASE_HI: u32 = 0x5C0018;
pub const mmDMA6_CORE_DST_BASE_LO: u32 = 0x5C001C;
pub const mmDMA6_CORE_DST_BASE_HI: u32 = 0x5C0020;
pub const mmDMA6_CORE_SRC_TSIZE_1: u32 = 0x5C002C;
pub const mmDMA6_CORE_SRC_STRIDE_1: u32 = 0x5C0030;
pub const mmDMA6_CORE_SRC_TSIZE_2: u32 = 0x5C0034;
pub const mmDMA6_CORE_SRC_STRIDE_2: u32 = 0x5C0038;
pub const mmDMA6_CORE_SRC_TSIZE_3: u32 = 0x5C003C;
pub const mmDMA6_CORE_SRC_STRIDE_3: u32 = 0x5C0040;
pub const mmDMA6_CORE_SRC_TSIZE_4: u32 = 0x5C0044;
pub const mmDMA6_CORE_SRC_STRIDE_4: u32 = 0x5C0048;
pub const mmDMA6_CORE_SRC_TSIZE_0: u32 = 0x5C004C;
pub const mmDMA6_CORE_DST_TSIZE_1: u32 = 0x5C0054;
pub const mmDMA6_CORE_DST_STRIDE_1: u32 = 0x5C0058;
pub const mmDMA6_CORE_DST_TSIZE_2: u32 = 0x5C005C;
pub const mmDMA6_CORE_DST_STRIDE_2: u32 = 0x5C0060;
pub const mmDMA6_CORE_DST_TSIZE_3: u32 = 0x5C0064;
pub const mmDMA6_CORE_DST_STRIDE_3: u32 = 0x5C0068;
pub const mmDMA6_CORE_DST_TSIZE_4: u32 = 0x5C006C;
pub const mmDMA6_CORE_DST_STRIDE_4: u32 = 0x5C0070;
pub const mmDMA6_CORE_DST_TSIZE_0: u32 = 0x5C0074;
pub const mmDMA6_CORE_COMMIT: u32 = 0x5C0078;
pub const mmDMA6_CORE_WR_COMP_WDATA: u32 = 0x5C007C;
pub const mmDMA6_CORE_WR_COMP_ADDR_LO: u32 = 0x5C0080;
pub const mmDMA6_CORE_WR_COMP_ADDR_HI: u32 = 0x5C0084;
pub const mmDMA6_CORE_WR_COMP_AWUSER_31_11: u32 = 0x5C0088;
pub const mmDMA6_CORE_TE_NUMROWS: u32 = 0x5C0094;
pub const mmDMA6_CORE_PROT: u32 = 0x5C00B8;
pub const mmDMA6_CORE_SECURE_PROPS: u32 = 0x5C00F0;
pub const mmDMA6_CORE_NON_SECURE_PROPS: u32 = 0x5C00F4;
pub const mmDMA6_CORE_RD_MAX_OUTSTAND: u32 = 0x5C0100;
pub const mmDMA6_CORE_RD_MAX_SIZE: u32 = 0x5C0104;
pub const mmDMA6_CORE_RD_ARCACHE: u32 = 0x5C0108;
pub const mmDMA6_CORE_RD_ARUSER_31_11: u32 = 0x5C0110;
pub const mmDMA6_CORE_RD_INFLIGHTS: u32 = 0x5C0114;
pub const mmDMA6_CORE_WR_MAX_OUTSTAND: u32 = 0x5C0120;
pub const mmDMA6_CORE_WR_MAX_AWID: u32 = 0x5C0124;
pub const mmDMA6_CORE_WR_AWCACHE: u32 = 0x5C0128;
pub const mmDMA6_CORE_WR_AWUSER_31_11: u32 = 0x5C0130;
pub const mmDMA6_CORE_WR_INFLIGHTS: u32 = 0x5C0134;
pub const mmDMA6_CORE_RD_RATE_LIM_CFG_0: u32 = 0x5C0150;
pub const mmDMA6_CORE_RD_RATE_LIM_CFG_1: u32 = 0x5C0154;
pub const mmDMA6_CORE_WR_RATE_LIM_CFG_0: u32 = 0x5C0158;
pub const mmDMA6_CORE_WR_RATE_LIM_CFG_1: u32 = 0x5C015C;
pub const mmDMA6_CORE_ERR_CFG: u32 = 0x5C0160;
pub const mmDMA6_CORE_ERR_CAUSE: u32 = 0x5C0164;
pub const mmDMA6_CORE_ERRMSG_ADDR_LO: u32 = 0x5C0170;
pub const mmDMA6_CORE_ERRMSG_ADDR_HI: u32 = 0x5C0174;
pub const mmDMA6_CORE_ERRMSG_WDATA: u32 = 0x5C0178;
pub const mmDMA6_CORE_STS0: u32 = 0x5C0190;
pub const mmDMA6_CORE_STS1: u32 = 0x5C0194;
pub const mmDMA6_CORE_RD_DBGMEM_ADD: u32 = 0x5C0200;
pub const mmDMA6_CORE_RD_DBGMEM_DATA_WR: u32 = 0x5C0204;
pub const mmDMA6_CORE_RD_DBGMEM_DATA_RD: u32 = 0x5C0208;
pub const mmDMA6_CORE_RD_DBGMEM_CTRL: u32 = 0x5C020C;
pub const mmDMA6_CORE_RD_DBGMEM_RC: u32 = 0x5C0210;
pub const mmDMA6_CORE_DBG_HBW_AXI_AR_CNT: u32 = 0x5C0220;
pub const mmDMA6_CORE_DBG_HBW_AXI_AW_CNT: u32 = 0x5C0224;
pub const mmDMA6_CORE_DBG_LBW_AXI_AW_CNT: u32 = 0x5C0228;
pub const mmDMA6_CORE_DBG_DESC_CNT: u32 = 0x5C022C;
pub const mmDMA6_CORE_DBG_STS: u32 = 0x5C0230;
pub const mmDMA6_CORE_DBG_RD_DESC_ID: u32 = 0x5C0234;
pub const mmDMA6_CORE_DBG_WR_DESC_ID: u32 = 0x5C0238;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
