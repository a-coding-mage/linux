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
 *   TPC2_CMDQ (Prototype: CMDQ)
 *****************************************
 */

pub const mmTPC2_CMDQ_GLBL_CFG0: u32 = 0xE89000;
pub const mmTPC2_CMDQ_GLBL_CFG1: u32 = 0xE89004;
pub const mmTPC2_CMDQ_GLBL_PROT: u32 = 0xE89008;
pub const mmTPC2_CMDQ_GLBL_ERR_CFG: u32 = 0xE8900C;
pub const mmTPC2_CMDQ_GLBL_ERR_ADDR_LO: u32 = 0xE89010;
pub const mmTPC2_CMDQ_GLBL_ERR_ADDR_HI: u32 = 0xE89014;
pub const mmTPC2_CMDQ_GLBL_ERR_WDATA: u32 = 0xE89018;
pub const mmTPC2_CMDQ_GLBL_SECURE_PROPS: u32 = 0xE8901C;
pub const mmTPC2_CMDQ_GLBL_NON_SECURE_PROPS: u32 = 0xE89020;
pub const mmTPC2_CMDQ_GLBL_STS0: u32 = 0xE89024;
pub const mmTPC2_CMDQ_GLBL_STS1: u32 = 0xE89028;
pub const mmTPC2_CMDQ_CQ_CFG0: u32 = 0xE890B0;
pub const mmTPC2_CMDQ_CQ_CFG1: u32 = 0xE890B4;
pub const mmTPC2_CMDQ_CQ_ARUSER: u32 = 0xE890B8;
pub const mmTPC2_CMDQ_CQ_PTR_LO: u32 = 0xE890C0;
pub const mmTPC2_CMDQ_CQ_PTR_HI: u32 = 0xE890C4;
pub const mmTPC2_CMDQ_CQ_TSIZE: u32 = 0xE890C8;
pub const mmTPC2_CMDQ_CQ_CTL: u32 = 0xE890CC;
pub const mmTPC2_CMDQ_CQ_PTR_LO_STS: u32 = 0xE890D4;
pub const mmTPC2_CMDQ_CQ_PTR_HI_STS: u32 = 0xE890D8;
pub const mmTPC2_CMDQ_CQ_TSIZE_STS: u32 = 0xE890DC;
pub const mmTPC2_CMDQ_CQ_CTL_STS: u32 = 0xE890E0;
pub const mmTPC2_CMDQ_CQ_STS0: u32 = 0xE890E4;
pub const mmTPC2_CMDQ_CQ_STS1: u32 = 0xE890E8;
pub const mmTPC2_CMDQ_CQ_RD_RATE_LIM_EN: u32 = 0xE890F0;
pub const mmTPC2_CMDQ_CQ_RD_RATE_LIM_RST_TOKEN: u32 = 0xE890F4;
pub const mmTPC2_CMDQ_CQ_RD_RATE_LIM_SAT: u32 = 0xE890F8;
pub const mmTPC2_CMDQ_CQ_RD_RATE_LIM_TOUT: u32 = 0xE890FC;
pub const mmTPC2_CMDQ_CQ_IFIFO_CNT: u32 = 0xE89108;
pub const mmTPC2_CMDQ_CP_MSG_BASE0_ADDR_LO: u32 = 0xE89120;
pub const mmTPC2_CMDQ_CP_MSG_BASE0_ADDR_HI: u32 = 0xE89124;
pub const mmTPC2_CMDQ_CP_MSG_BASE1_ADDR_LO: u32 = 0xE89128;
pub const mmTPC2_CMDQ_CP_MSG_BASE1_ADDR_HI: u32 = 0xE8912C;
pub const mmTPC2_CMDQ_CP_MSG_BASE2_ADDR_LO: u32 = 0xE89130;
pub const mmTPC2_CMDQ_CP_MSG_BASE2_ADDR_HI: u32 = 0xE89134;
pub const mmTPC2_CMDQ_CP_MSG_BASE3_ADDR_LO: u32 = 0xE89138;
pub const mmTPC2_CMDQ_CP_MSG_BASE3_ADDR_HI: u32 = 0xE8913C;
pub const mmTPC2_CMDQ_CP_LDMA_TSIZE_OFFSET: u32 = 0xE89140;
pub const mmTPC2_CMDQ_CP_LDMA_SRC_BASE_LO_OFFSET: u32 = 0xE89144;
pub const mmTPC2_CMDQ_CP_LDMA_SRC_BASE_HI_OFFSET: u32 = 0xE89148;
pub const mmTPC2_CMDQ_CP_LDMA_DST_BASE_LO_OFFSET: u32 = 0xE8914C;
pub const mmTPC2_CMDQ_CP_LDMA_DST_BASE_HI_OFFSET: u32 = 0xE89150;
pub const mmTPC2_CMDQ_CP_LDMA_COMMIT_OFFSET: u32 = 0xE89154;
pub const mmTPC2_CMDQ_CP_FENCE0_RDATA: u32 = 0xE89158;
pub const mmTPC2_CMDQ_CP_FENCE1_RDATA: u32 = 0xE8915C;
pub const mmTPC2_CMDQ_CP_FENCE2_RDATA: u32 = 0xE89160;
pub const mmTPC2_CMDQ_CP_FENCE3_RDATA: u32 = 0xE89164;
pub const mmTPC2_CMDQ_CP_FENCE0_CNT: u32 = 0xE89168;
pub const mmTPC2_CMDQ_CP_FENCE1_CNT: u32 = 0xE8916C;
pub const mmTPC2_CMDQ_CP_FENCE2_CNT: u32 = 0xE89170;
pub const mmTPC2_CMDQ_CP_FENCE3_CNT: u32 = 0xE89174;
pub const mmTPC2_CMDQ_CP_STS: u32 = 0xE89178;
pub const mmTPC2_CMDQ_CP_CURRENT_INST_LO: u32 = 0xE8917C;
pub const mmTPC2_CMDQ_CP_CURRENT_INST_HI: u32 = 0xE89180;
pub const mmTPC2_CMDQ_CP_BARRIER_CFG: u32 = 0xE89184;
pub const mmTPC2_CMDQ_CP_DBG_0: u32 = 0xE89188;
pub const mmTPC2_CMDQ_CQ_BUF_ADDR: u32 = 0xE89308;
pub const mmTPC2_CMDQ_CQ_BUF_RDATA: u32 = 0xE8930C;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
