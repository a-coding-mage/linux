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
 *   DMA7_CORE (Prototype: DMA_CORE)
 *****************************************
 */

pub const mmDMA7_CORE_CFG_0: u32 = 0x5E0000;
pub const mmDMA7_CORE_CFG_1: u32 = 0x5E0004;
pub const mmDMA7_CORE_LBW_MAX_OUTSTAND: u32 = 0x5E0008;
pub const mmDMA7_CORE_SRC_BASE_LO: u32 = 0x5E0014;
pub const mmDMA7_CORE_SRC_BASE_HI: u32 = 0x5E0018;
pub const mmDMA7_CORE_DST_BASE_LO: u32 = 0x5E001C;
pub const mmDMA7_CORE_DST_BASE_HI: u32 = 0x5E0020;
pub const mmDMA7_CORE_SRC_TSIZE_1: u32 = 0x5E002C;
pub const mmDMA7_CORE_SRC_STRIDE_1: u32 = 0x5E0030;
pub const mmDMA7_CORE_SRC_TSIZE_2: u32 = 0x5E0034;
pub const mmDMA7_CORE_SRC_STRIDE_2: u32 = 0x5E0038;
pub const mmDMA7_CORE_SRC_TSIZE_3: u32 = 0x5E003C;
pub const mmDMA7_CORE_SRC_STRIDE_3: u32 = 0x5E0040;
pub const mmDMA7_CORE_SRC_TSIZE_4: u32 = 0x5E0044;
pub const mmDMA7_CORE_SRC_STRIDE_4: u32 = 0x5E0048;
pub const mmDMA7_CORE_SRC_TSIZE_0: u32 = 0x5E004C;
pub const mmDMA7_CORE_DST_TSIZE_1: u32 = 0x5E0054;
pub const mmDMA7_CORE_DST_STRIDE_1: u32 = 0x5E0058;
pub const mmDMA7_CORE_DST_TSIZE_2: u32 = 0x5E005C;
pub const mmDMA7_CORE_DST_STRIDE_2: u32 = 0x5E0060;
pub const mmDMA7_CORE_DST_TSIZE_3: u32 = 0x5E0064;
pub const mmDMA7_CORE_DST_STRIDE_3: u32 = 0x5E0068;
pub const mmDMA7_CORE_DST_TSIZE_4: u32 = 0x5E006C;
pub const mmDMA7_CORE_DST_STRIDE_4: u32 = 0x5E0070;
pub const mmDMA7_CORE_DST_TSIZE_0: u32 = 0x5E0074;
pub const mmDMA7_CORE_COMMIT: u32 = 0x5E0078;
pub const mmDMA7_CORE_WR_COMP_WDATA: u32 = 0x5E007C;
pub const mmDMA7_CORE_WR_COMP_ADDR_LO: u32 = 0x5E0080;
pub const mmDMA7_CORE_WR_COMP_ADDR_HI: u32 = 0x5E0084;
pub const mmDMA7_CORE_WR_COMP_AWUSER_31_11: u32 = 0x5E0088;
pub const mmDMA7_CORE_TE_NUMROWS: u32 = 0x5E0094;
pub const mmDMA7_CORE_PROT: u32 = 0x5E00B8;
pub const mmDMA7_CORE_SECURE_PROPS: u32 = 0x5E00F0;
pub const mmDMA7_CORE_NON_SECURE_PROPS: u32 = 0x5E00F4;
pub const mmDMA7_CORE_RD_MAX_OUTSTAND: u32 = 0x5E0100;
pub const mmDMA7_CORE_RD_MAX_SIZE: u32 = 0x5E0104;
pub const mmDMA7_CORE_RD_ARCACHE: u32 = 0x5E0108;
pub const mmDMA7_CORE_RD_ARUSER_31_11: u32 = 0x5E0110;
pub const mmDMA7_CORE_RD_INFLIGHTS: u32 = 0x5E0114;
pub const mmDMA7_CORE_WR_MAX_OUTSTAND: u32 = 0x5E0120;
pub const mmDMA7_CORE_WR_MAX_AWID: u32 = 0x5E0124;
pub const mmDMA7_CORE_WR_AWCACHE: u32 = 0x5E0128;
pub const mmDMA7_CORE_WR_AWUSER_31_11: u32 = 0x5E0130;
pub const mmDMA7_CORE_WR_INFLIGHTS: u32 = 0x5E0134;
pub const mmDMA7_CORE_RD_RATE_LIM_CFG_0: u32 = 0x5E0150;
pub const mmDMA7_CORE_RD_RATE_LIM_CFG_1: u32 = 0x5E0154;
pub const mmDMA7_CORE_WR_RATE_LIM_CFG_0: u32 = 0x5E0158;
pub const mmDMA7_CORE_WR_RATE_LIM_CFG_1: u32 = 0x5E015C;
pub const mmDMA7_CORE_ERR_CFG: u32 = 0x5E0160;
pub const mmDMA7_CORE_ERR_CAUSE: u32 = 0x5E0164;
pub const mmDMA7_CORE_ERRMSG_ADDR_LO: u32 = 0x5E0170;
pub const mmDMA7_CORE_ERRMSG_ADDR_HI: u32 = 0x5E0174;
pub const mmDMA7_CORE_ERRMSG_WDATA: u32 = 0x5E0178;
pub const mmDMA7_CORE_STS0: u32 = 0x5E0190;
pub const mmDMA7_CORE_STS1: u32 = 0x5E0194;
pub const mmDMA7_CORE_RD_DBGMEM_ADD: u32 = 0x5E0200;
pub const mmDMA7_CORE_RD_DBGMEM_DATA_WR: u32 = 0x5E0204;
pub const mmDMA7_CORE_RD_DBGMEM_DATA_RD: u32 = 0x5E0208;
pub const mmDMA7_CORE_RD_DBGMEM_CTRL: u32 = 0x5E020C;
pub const mmDMA7_CORE_RD_DBGMEM_RC: u32 = 0x5E0210;
pub const mmDMA7_CORE_DBG_HBW_AXI_AR_CNT: u32 = 0x5E0220;
pub const mmDMA7_CORE_DBG_HBW_AXI_AW_CNT: u32 = 0x5E0224;
pub const mmDMA7_CORE_DBG_LBW_AXI_AW_CNT: u32 = 0x5E0228;
pub const mmDMA7_CORE_DBG_DESC_CNT: u32 = 0x5E022C;
pub const mmDMA7_CORE_DBG_STS: u32 = 0x5E0230;
pub const mmDMA7_CORE_DBG_RD_DESC_ID: u32 = 0x5E0234;
pub const mmDMA7_CORE_DBG_WR_DESC_ID: u32 = 0x5E0238;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
