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
 *   TPC6_CMDQ (Prototype: CMDQ)
 *****************************************
 */

pub const mmTPC6_CMDQ_GLBL_CFG0: u32 = 0xF89000;
pub const mmTPC6_CMDQ_GLBL_CFG1: u32 = 0xF89004;
pub const mmTPC6_CMDQ_GLBL_PROT: u32 = 0xF89008;
pub const mmTPC6_CMDQ_GLBL_ERR_CFG: u32 = 0xF8900C;
pub const mmTPC6_CMDQ_GLBL_ERR_ADDR_LO: u32 = 0xF89010;
pub const mmTPC6_CMDQ_GLBL_ERR_ADDR_HI: u32 = 0xF89014;
pub const mmTPC6_CMDQ_GLBL_ERR_WDATA: u32 = 0xF89018;
pub const mmTPC6_CMDQ_GLBL_SECURE_PROPS: u32 = 0xF8901C;
pub const mmTPC6_CMDQ_GLBL_NON_SECURE_PROPS: u32 = 0xF89020;
pub const mmTPC6_CMDQ_GLBL_STS0: u32 = 0xF89024;
pub const mmTPC6_CMDQ_GLBL_STS1: u32 = 0xF89028;
pub const mmTPC6_CMDQ_CQ_CFG0: u32 = 0xF890B0;
pub const mmTPC6_CMDQ_CQ_CFG1: u32 = 0xF890B4;
pub const mmTPC6_CMDQ_CQ_ARUSER: u32 = 0xF890B8;
pub const mmTPC6_CMDQ_CQ_PTR_LO: u32 = 0xF890C0;
pub const mmTPC6_CMDQ_CQ_PTR_HI: u32 = 0xF890C4;
pub const mmTPC6_CMDQ_CQ_TSIZE: u32 = 0xF890C8;
pub const mmTPC6_CMDQ_CQ_CTL: u32 = 0xF890CC;
pub const mmTPC6_CMDQ_CQ_PTR_LO_STS: u32 = 0xF890D4;
pub const mmTPC6_CMDQ_CQ_PTR_HI_STS: u32 = 0xF890D8;
pub const mmTPC6_CMDQ_CQ_TSIZE_STS: u32 = 0xF890DC;
pub const mmTPC6_CMDQ_CQ_CTL_STS: u32 = 0xF890E0;
pub const mmTPC6_CMDQ_CQ_STS0: u32 = 0xF890E4;
pub const mmTPC6_CMDQ_CQ_STS1: u32 = 0xF890E8;
pub const mmTPC6_CMDQ_CQ_RD_RATE_LIM_EN: u32 = 0xF890F0;
pub const mmTPC6_CMDQ_CQ_RD_RATE_LIM_RST_TOKEN: u32 = 0xF890F4;
pub const mmTPC6_CMDQ_CQ_RD_RATE_LIM_SAT: u32 = 0xF890F8;
pub const mmTPC6_CMDQ_CQ_RD_RATE_LIM_TOUT: u32 = 0xF890FC;
pub const mmTPC6_CMDQ_CQ_IFIFO_CNT: u32 = 0xF89108;
pub const mmTPC6_CMDQ_CP_MSG_BASE0_ADDR_LO: u32 = 0xF89120;
pub const mmTPC6_CMDQ_CP_MSG_BASE0_ADDR_HI: u32 = 0xF89124;
pub const mmTPC6_CMDQ_CP_MSG_BASE1_ADDR_LO: u32 = 0xF89128;
pub const mmTPC6_CMDQ_CP_MSG_BASE1_ADDR_HI: u32 = 0xF8912C;
pub const mmTPC6_CMDQ_CP_MSG_BASE2_ADDR_LO: u32 = 0xF89130;
pub const mmTPC6_CMDQ_CP_MSG_BASE2_ADDR_HI: u32 = 0xF89134;
pub const mmTPC6_CMDQ_CP_MSG_BASE3_ADDR_LO: u32 = 0xF89138;
pub const mmTPC6_CMDQ_CP_MSG_BASE3_ADDR_HI: u32 = 0xF8913C;
pub const mmTPC6_CMDQ_CP_LDMA_TSIZE_OFFSET: u32 = 0xF89140;
pub const mmTPC6_CMDQ_CP_LDMA_SRC_BASE_LO_OFFSET: u32 = 0xF89144;
pub const mmTPC6_CMDQ_CP_LDMA_SRC_BASE_HI_OFFSET: u32 = 0xF89148;
pub const mmTPC6_CMDQ_CP_LDMA_DST_BASE_LO_OFFSET: u32 = 0xF8914C;
pub const mmTPC6_CMDQ_CP_LDMA_DST_BASE_HI_OFFSET: u32 = 0xF89150;
pub const mmTPC6_CMDQ_CP_LDMA_COMMIT_OFFSET: u32 = 0xF89154;
pub const mmTPC6_CMDQ_CP_FENCE0_RDATA: u32 = 0xF89158;
pub const mmTPC6_CMDQ_CP_FENCE1_RDATA: u32 = 0xF8915C;
pub const mmTPC6_CMDQ_CP_FENCE2_RDATA: u32 = 0xF89160;
pub const mmTPC6_CMDQ_CP_FENCE3_RDATA: u32 = 0xF89164;
pub const mmTPC6_CMDQ_CP_FENCE0_CNT: u32 = 0xF89168;
pub const mmTPC6_CMDQ_CP_FENCE1_CNT: u32 = 0xF8916C;
pub const mmTPC6_CMDQ_CP_FENCE2_CNT: u32 = 0xF89170;
pub const mmTPC6_CMDQ_CP_FENCE3_CNT: u32 = 0xF89174;
pub const mmTPC6_CMDQ_CP_STS: u32 = 0xF89178;
pub const mmTPC6_CMDQ_CP_CURRENT_INST_LO: u32 = 0xF8917C;
pub const mmTPC6_CMDQ_CP_CURRENT_INST_HI: u32 = 0xF89180;
pub const mmTPC6_CMDQ_CP_BARRIER_CFG: u32 = 0xF89184;
pub const mmTPC6_CMDQ_CP_DBG_0: u32 = 0xF89188;
pub const mmTPC6_CMDQ_CQ_BUF_ADDR: u32 = 0xF89308;
pub const mmTPC6_CMDQ_CQ_BUF_RDATA: u32 = 0xF8930C;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
