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
 *   TPC0_CMDQ (Prototype: CMDQ)
 *****************************************
 */

pub const mmTPC0_CMDQ_GLBL_CFG0: u32 = 0xE09000;
pub const mmTPC0_CMDQ_GLBL_CFG1: u32 = 0xE09004;
pub const mmTPC0_CMDQ_GLBL_PROT: u32 = 0xE09008;
pub const mmTPC0_CMDQ_GLBL_ERR_CFG: u32 = 0xE0900C;
pub const mmTPC0_CMDQ_GLBL_ERR_ADDR_LO: u32 = 0xE09010;
pub const mmTPC0_CMDQ_GLBL_ERR_ADDR_HI: u32 = 0xE09014;
pub const mmTPC0_CMDQ_GLBL_ERR_WDATA: u32 = 0xE09018;
pub const mmTPC0_CMDQ_GLBL_SECURE_PROPS: u32 = 0xE0901C;
pub const mmTPC0_CMDQ_GLBL_NON_SECURE_PROPS: u32 = 0xE09020;
pub const mmTPC0_CMDQ_GLBL_STS0: u32 = 0xE09024;
pub const mmTPC0_CMDQ_GLBL_STS1: u32 = 0xE09028;
pub const mmTPC0_CMDQ_CQ_CFG0: u32 = 0xE090B0;
pub const mmTPC0_CMDQ_CQ_CFG1: u32 = 0xE090B4;
pub const mmTPC0_CMDQ_CQ_ARUSER: u32 = 0xE090B8;
pub const mmTPC0_CMDQ_CQ_PTR_LO: u32 = 0xE090C0;
pub const mmTPC0_CMDQ_CQ_PTR_HI: u32 = 0xE090C4;
pub const mmTPC0_CMDQ_CQ_TSIZE: u32 = 0xE090C8;
pub const mmTPC0_CMDQ_CQ_CTL: u32 = 0xE090CC;
pub const mmTPC0_CMDQ_CQ_PTR_LO_STS: u32 = 0xE090D4;
pub const mmTPC0_CMDQ_CQ_PTR_HI_STS: u32 = 0xE090D8;
pub const mmTPC0_CMDQ_CQ_TSIZE_STS: u32 = 0xE090DC;
pub const mmTPC0_CMDQ_CQ_CTL_STS: u32 = 0xE090E0;
pub const mmTPC0_CMDQ_CQ_STS0: u32 = 0xE090E4;
pub const mmTPC0_CMDQ_CQ_STS1: u32 = 0xE090E8;
pub const mmTPC0_CMDQ_CQ_RD_RATE_LIM_EN: u32 = 0xE090F0;
pub const mmTPC0_CMDQ_CQ_RD_RATE_LIM_RST_TOKEN: u32 = 0xE090F4;
pub const mmTPC0_CMDQ_CQ_RD_RATE_LIM_SAT: u32 = 0xE090F8;
pub const mmTPC0_CMDQ_CQ_RD_RATE_LIM_TOUT: u32 = 0xE090FC;
pub const mmTPC0_CMDQ_CQ_IFIFO_CNT: u32 = 0xE09108;
pub const mmTPC0_CMDQ_CP_MSG_BASE0_ADDR_LO: u32 = 0xE09120;
pub const mmTPC0_CMDQ_CP_MSG_BASE0_ADDR_HI: u32 = 0xE09124;
pub const mmTPC0_CMDQ_CP_MSG_BASE1_ADDR_LO: u32 = 0xE09128;
pub const mmTPC0_CMDQ_CP_MSG_BASE1_ADDR_HI: u32 = 0xE0912C;
pub const mmTPC0_CMDQ_CP_MSG_BASE2_ADDR_LO: u32 = 0xE09130;
pub const mmTPC0_CMDQ_CP_MSG_BASE2_ADDR_HI: u32 = 0xE09134;
pub const mmTPC0_CMDQ_CP_MSG_BASE3_ADDR_LO: u32 = 0xE09138;
pub const mmTPC0_CMDQ_CP_MSG_BASE3_ADDR_HI: u32 = 0xE0913C;
pub const mmTPC0_CMDQ_CP_LDMA_TSIZE_OFFSET: u32 = 0xE09140;
pub const mmTPC0_CMDQ_CP_LDMA_SRC_BASE_LO_OFFSET: u32 = 0xE09144;
pub const mmTPC0_CMDQ_CP_LDMA_SRC_BASE_HI_OFFSET: u32 = 0xE09148;
pub const mmTPC0_CMDQ_CP_LDMA_DST_BASE_LO_OFFSET: u32 = 0xE0914C;
pub const mmTPC0_CMDQ_CP_LDMA_DST_BASE_HI_OFFSET: u32 = 0xE09150;
pub const mmTPC0_CMDQ_CP_LDMA_COMMIT_OFFSET: u32 = 0xE09154;
pub const mmTPC0_CMDQ_CP_FENCE0_RDATA: u32 = 0xE09158;
pub const mmTPC0_CMDQ_CP_FENCE1_RDATA: u32 = 0xE0915C;
pub const mmTPC0_CMDQ_CP_FENCE2_RDATA: u32 = 0xE09160;
pub const mmTPC0_CMDQ_CP_FENCE3_RDATA: u32 = 0xE09164;
pub const mmTPC0_CMDQ_CP_FENCE0_CNT: u32 = 0xE09168;
pub const mmTPC0_CMDQ_CP_FENCE1_CNT: u32 = 0xE0916C;
pub const mmTPC0_CMDQ_CP_FENCE2_CNT: u32 = 0xE09170;
pub const mmTPC0_CMDQ_CP_FENCE3_CNT: u32 = 0xE09174;
pub const mmTPC0_CMDQ_CP_STS: u32 = 0xE09178;
pub const mmTPC0_CMDQ_CP_CURRENT_INST_LO: u32 = 0xE0917C;
pub const mmTPC0_CMDQ_CP_CURRENT_INST_HI: u32 = 0xE09180;
pub const mmTPC0_CMDQ_CP_BARRIER_CFG: u32 = 0xE09184;
pub const mmTPC0_CMDQ_CP_DBG_0: u32 = 0xE09188;
pub const mmTPC0_CMDQ_CQ_BUF_ADDR: u32 = 0xE09308;
pub const mmTPC0_CMDQ_CQ_BUF_RDATA: u32 = 0xE0930C;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
