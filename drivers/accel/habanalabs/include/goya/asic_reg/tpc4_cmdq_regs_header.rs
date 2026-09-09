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
 *   TPC4_CMDQ (Prototype: CMDQ)
 *****************************************
 */

pub const mmTPC4_CMDQ_GLBL_CFG0: u32 = 0xF09000;
pub const mmTPC4_CMDQ_GLBL_CFG1: u32 = 0xF09004;
pub const mmTPC4_CMDQ_GLBL_PROT: u32 = 0xF09008;
pub const mmTPC4_CMDQ_GLBL_ERR_CFG: u32 = 0xF0900C;
pub const mmTPC4_CMDQ_GLBL_ERR_ADDR_LO: u32 = 0xF09010;
pub const mmTPC4_CMDQ_GLBL_ERR_ADDR_HI: u32 = 0xF09014;
pub const mmTPC4_CMDQ_GLBL_ERR_WDATA: u32 = 0xF09018;
pub const mmTPC4_CMDQ_GLBL_SECURE_PROPS: u32 = 0xF0901C;
pub const mmTPC4_CMDQ_GLBL_NON_SECURE_PROPS: u32 = 0xF09020;
pub const mmTPC4_CMDQ_GLBL_STS0: u32 = 0xF09024;
pub const mmTPC4_CMDQ_GLBL_STS1: u32 = 0xF09028;
pub const mmTPC4_CMDQ_CQ_CFG0: u32 = 0xF090B0;
pub const mmTPC4_CMDQ_CQ_CFG1: u32 = 0xF090B4;
pub const mmTPC4_CMDQ_CQ_ARUSER: u32 = 0xF090B8;
pub const mmTPC4_CMDQ_CQ_PTR_LO: u32 = 0xF090C0;
pub const mmTPC4_CMDQ_CQ_PTR_HI: u32 = 0xF090C4;
pub const mmTPC4_CMDQ_CQ_TSIZE: u32 = 0xF090C8;
pub const mmTPC4_CMDQ_CQ_CTL: u32 = 0xF090CC;
pub const mmTPC4_CMDQ_CQ_PTR_LO_STS: u32 = 0xF090D4;
pub const mmTPC4_CMDQ_CQ_PTR_HI_STS: u32 = 0xF090D8;
pub const mmTPC4_CMDQ_CQ_TSIZE_STS: u32 = 0xF090DC;
pub const mmTPC4_CMDQ_CQ_CTL_STS: u32 = 0xF090E0;
pub const mmTPC4_CMDQ_CQ_STS0: u32 = 0xF090E4;
pub const mmTPC4_CMDQ_CQ_STS1: u32 = 0xF090E8;
pub const mmTPC4_CMDQ_CQ_RD_RATE_LIM_EN: u32 = 0xF090F0;
pub const mmTPC4_CMDQ_CQ_RD_RATE_LIM_RST_TOKEN: u32 = 0xF090F4;
pub const mmTPC4_CMDQ_CQ_RD_RATE_LIM_SAT: u32 = 0xF090F8;
pub const mmTPC4_CMDQ_CQ_RD_RATE_LIM_TOUT: u32 = 0xF090FC;
pub const mmTPC4_CMDQ_CQ_IFIFO_CNT: u32 = 0xF09108;
pub const mmTPC4_CMDQ_CP_MSG_BASE0_ADDR_LO: u32 = 0xF09120;
pub const mmTPC4_CMDQ_CP_MSG_BASE0_ADDR_HI: u32 = 0xF09124;
pub const mmTPC4_CMDQ_CP_MSG_BASE1_ADDR_LO: u32 = 0xF09128;
pub const mmTPC4_CMDQ_CP_MSG_BASE1_ADDR_HI: u32 = 0xF0912C;
pub const mmTPC4_CMDQ_CP_MSG_BASE2_ADDR_LO: u32 = 0xF09130;
pub const mmTPC4_CMDQ_CP_MSG_BASE2_ADDR_HI: u32 = 0xF09134;
pub const mmTPC4_CMDQ_CP_MSG_BASE3_ADDR_LO: u32 = 0xF09138;
pub const mmTPC4_CMDQ_CP_MSG_BASE3_ADDR_HI: u32 = 0xF0913C;
pub const mmTPC4_CMDQ_CP_LDMA_TSIZE_OFFSET: u32 = 0xF09140;
pub const mmTPC4_CMDQ_CP_LDMA_SRC_BASE_LO_OFFSET: u32 = 0xF09144;
pub const mmTPC4_CMDQ_CP_LDMA_SRC_BASE_HI_OFFSET: u32 = 0xF09148;
pub const mmTPC4_CMDQ_CP_LDMA_DST_BASE_LO_OFFSET: u32 = 0xF0914C;
pub const mmTPC4_CMDQ_CP_LDMA_DST_BASE_HI_OFFSET: u32 = 0xF09150;
pub const mmTPC4_CMDQ_CP_LDMA_COMMIT_OFFSET: u32 = 0xF09154;
pub const mmTPC4_CMDQ_CP_FENCE0_RDATA: u32 = 0xF09158;
pub const mmTPC4_CMDQ_CP_FENCE1_RDATA: u32 = 0xF0915C;
pub const mmTPC4_CMDQ_CP_FENCE2_RDATA: u32 = 0xF09160;
pub const mmTPC4_CMDQ_CP_FENCE3_RDATA: u32 = 0xF09164;
pub const mmTPC4_CMDQ_CP_FENCE0_CNT: u32 = 0xF09168;
pub const mmTPC4_CMDQ_CP_FENCE1_CNT: u32 = 0xF0916C;
pub const mmTPC4_CMDQ_CP_FENCE2_CNT: u32 = 0xF09170;
pub const mmTPC4_CMDQ_CP_FENCE3_CNT: u32 = 0xF09174;
pub const mmTPC4_CMDQ_CP_STS: u32 = 0xF09178;
pub const mmTPC4_CMDQ_CP_CURRENT_INST_LO: u32 = 0xF0917C;
pub const mmTPC4_CMDQ_CP_CURRENT_INST_HI: u32 = 0xF09180;
pub const mmTPC4_CMDQ_CP_BARRIER_CFG: u32 = 0xF09184;
pub const mmTPC4_CMDQ_CP_DBG_0: u32 = 0xF09188;
pub const mmTPC4_CMDQ_CQ_BUF_ADDR: u32 = 0xF09308;
pub const mmTPC4_CMDQ_CQ_BUF_RDATA: u32 = 0xF0930C;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
