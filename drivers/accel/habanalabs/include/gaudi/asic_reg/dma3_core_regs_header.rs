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
 *   DMA3_CORE (Prototype: DMA_CORE)
 *****************************************
 */

pub const mmDMA3_CORE_CFG_0: u32 = 0x560000;
pub const mmDMA3_CORE_CFG_1: u32 = 0x560004;
pub const mmDMA3_CORE_LBW_MAX_OUTSTAND: u32 = 0x560008;
pub const mmDMA3_CORE_SRC_BASE_LO: u32 = 0x560014;
pub const mmDMA3_CORE_SRC_BASE_HI: u32 = 0x560018;
pub const mmDMA3_CORE_DST_BASE_LO: u32 = 0x56001C;
pub const mmDMA3_CORE_DST_BASE_HI: u32 = 0x560020;
pub const mmDMA3_CORE_SRC_TSIZE_1: u32 = 0x56002C;
pub const mmDMA3_CORE_SRC_STRIDE_1: u32 = 0x560030;
pub const mmDMA3_CORE_SRC_TSIZE_2: u32 = 0x560034;
pub const mmDMA3_CORE_SRC_STRIDE_2: u32 = 0x560038;
pub const mmDMA3_CORE_SRC_TSIZE_3: u32 = 0x56003C;
pub const mmDMA3_CORE_SRC_STRIDE_3: u32 = 0x560040;
pub const mmDMA3_CORE_SRC_TSIZE_4: u32 = 0x560044;
pub const mmDMA3_CORE_SRC_STRIDE_4: u32 = 0x560048;
pub const mmDMA3_CORE_SRC_TSIZE_0: u32 = 0x56004C;
pub const mmDMA3_CORE_DST_TSIZE_1: u32 = 0x560054;
pub const mmDMA3_CORE_DST_STRIDE_1: u32 = 0x560058;
pub const mmDMA3_CORE_DST_TSIZE_2: u32 = 0x56005C;
pub const mmDMA3_CORE_DST_STRIDE_2: u32 = 0x560060;
pub const mmDMA3_CORE_DST_TSIZE_3: u32 = 0x560064;
pub const mmDMA3_CORE_DST_STRIDE_3: u32 = 0x560068;
pub const mmDMA3_CORE_DST_TSIZE_4: u32 = 0x56006C;
pub const mmDMA3_CORE_DST_STRIDE_4: u32 = 0x560070;
pub const mmDMA3_CORE_DST_TSIZE_0: u32 = 0x560074;
pub const mmDMA3_CORE_COMMIT: u32 = 0x560078;
pub const mmDMA3_CORE_WR_COMP_WDATA: u32 = 0x56007C;
pub const mmDMA3_CORE_WR_COMP_ADDR_LO: u32 = 0x560080;
pub const mmDMA3_CORE_WR_COMP_ADDR_HI: u32 = 0x560084;
pub const mmDMA3_CORE_WR_COMP_AWUSER_31_11: u32 = 0x560088;
pub const mmDMA3_CORE_TE_NUMROWS: u32 = 0x560094;
pub const mmDMA3_CORE_PROT: u32 = 0x5600B8;
pub const mmDMA3_CORE_SECURE_PROPS: u32 = 0x5600F0;
pub const mmDMA3_CORE_NON_SECURE_PROPS: u32 = 0x5600F4;
pub const mmDMA3_CORE_RD_MAX_OUTSTAND: u32 = 0x560100;
pub const mmDMA3_CORE_RD_MAX_SIZE: u32 = 0x560104;
pub const mmDMA3_CORE_RD_ARCACHE: u32 = 0x560108;
pub const mmDMA3_CORE_RD_ARUSER_31_11: u32 = 0x560110;
pub const mmDMA3_CORE_RD_INFLIGHTS: u32 = 0x560114;
pub const mmDMA3_CORE_WR_MAX_OUTSTAND: u32 = 0x560120;
pub const mmDMA3_CORE_WR_MAX_AWID: u32 = 0x560124;
pub const mmDMA3_CORE_WR_AWCACHE: u32 = 0x560128;
pub const mmDMA3_CORE_WR_AWUSER_31_11: u32 = 0x560130;
pub const mmDMA3_CORE_WR_INFLIGHTS: u32 = 0x560134;
pub const mmDMA3_CORE_RD_RATE_LIM_CFG_0: u32 = 0x560150;
pub const mmDMA3_CORE_RD_RATE_LIM_CFG_1: u32 = 0x560154;
pub const mmDMA3_CORE_WR_RATE_LIM_CFG_0: u32 = 0x560158;
pub const mmDMA3_CORE_WR_RATE_LIM_CFG_1: u32 = 0x56015C;
pub const mmDMA3_CORE_ERR_CFG: u32 = 0x560160;
pub const mmDMA3_CORE_ERR_CAUSE: u32 = 0x560164;
pub const mmDMA3_CORE_ERRMSG_ADDR_LO: u32 = 0x560170;
pub const mmDMA3_CORE_ERRMSG_ADDR_HI: u32 = 0x560174;
pub const mmDMA3_CORE_ERRMSG_WDATA: u32 = 0x560178;
pub const mmDMA3_CORE_STS0: u32 = 0x560190;
pub const mmDMA3_CORE_STS1: u32 = 0x560194;
pub const mmDMA3_CORE_RD_DBGMEM_ADD: u32 = 0x560200;
pub const mmDMA3_CORE_RD_DBGMEM_DATA_WR: u32 = 0x560204;
pub const mmDMA3_CORE_RD_DBGMEM_DATA_RD: u32 = 0x560208;
pub const mmDMA3_CORE_RD_DBGMEM_CTRL: u32 = 0x56020C;
pub const mmDMA3_CORE_RD_DBGMEM_RC: u32 = 0x560210;
pub const mmDMA3_CORE_DBG_HBW_AXI_AR_CNT: u32 = 0x560220;
pub const mmDMA3_CORE_DBG_HBW_AXI_AW_CNT: u32 = 0x560224;
pub const mmDMA3_CORE_DBG_LBW_AXI_AW_CNT: u32 = 0x560228;
pub const mmDMA3_CORE_DBG_DESC_CNT: u32 = 0x56022C;
pub const mmDMA3_CORE_DBG_STS: u32 = 0x560230;
pub const mmDMA3_CORE_DBG_RD_DESC_ID: u32 = 0x560234;
pub const mmDMA3_CORE_DBG_WR_DESC_ID: u32 = 0x560238;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
