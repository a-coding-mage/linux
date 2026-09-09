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
 *   TPC3_CMDQ (Prototype: CMDQ)
 *****************************************
 */

pub const mmTPC3_CMDQ_GLBL_CFG0: u32 = 0xEC9000;
pub const mmTPC3_CMDQ_GLBL_CFG1: u32 = 0xEC9004;
pub const mmTPC3_CMDQ_GLBL_PROT: u32 = 0xEC9008;
pub const mmTPC3_CMDQ_GLBL_ERR_CFG: u32 = 0xEC900C;
pub const mmTPC3_CMDQ_GLBL_ERR_ADDR_LO: u32 = 0xEC9010;
pub const mmTPC3_CMDQ_GLBL_ERR_ADDR_HI: u32 = 0xEC9014;
pub const mmTPC3_CMDQ_GLBL_ERR_WDATA: u32 = 0xEC9018;
pub const mmTPC3_CMDQ_GLBL_SECURE_PROPS: u32 = 0xEC901C;
pub const mmTPC3_CMDQ_GLBL_NON_SECURE_PROPS: u32 = 0xEC9020;
pub const mmTPC3_CMDQ_GLBL_STS0: u32 = 0xEC9024;
pub const mmTPC3_CMDQ_GLBL_STS1: u32 = 0xEC9028;
pub const mmTPC3_CMDQ_CQ_CFG0: u32 = 0xEC90B0;
pub const mmTPC3_CMDQ_CQ_CFG1: u32 = 0xEC90B4;
pub const mmTPC3_CMDQ_CQ_ARUSER: u32 = 0xEC90B8;
pub const mmTPC3_CMDQ_CQ_PTR_LO: u32 = 0xEC90C0;
pub const mmTPC3_CMDQ_CQ_PTR_HI: u32 = 0xEC90C4;
pub const mmTPC3_CMDQ_CQ_TSIZE: u32 = 0xEC90C8;
pub const mmTPC3_CMDQ_CQ_CTL: u32 = 0xEC90CC;
pub const mmTPC3_CMDQ_CQ_PTR_LO_STS: u32 = 0xEC90D4;
pub const mmTPC3_CMDQ_CQ_PTR_HI_STS: u32 = 0xEC90D8;
pub const mmTPC3_CMDQ_CQ_TSIZE_STS: u32 = 0xEC90DC;
pub const mmTPC3_CMDQ_CQ_CTL_STS: u32 = 0xEC90E0;
pub const mmTPC3_CMDQ_CQ_STS0: u32 = 0xEC90E4;
pub const mmTPC3_CMDQ_CQ_STS1: u32 = 0xEC90E8;
pub const mmTPC3_CMDQ_CQ_RD_RATE_LIM_EN: u32 = 0xEC90F0;
pub const mmTPC3_CMDQ_CQ_RD_RATE_LIM_RST_TOKEN: u32 = 0xEC90F4;
pub const mmTPC3_CMDQ_CQ_RD_RATE_LIM_SAT: u32 = 0xEC90F8;
pub const mmTPC3_CMDQ_CQ_RD_RATE_LIM_TOUT: u32 = 0xEC90FC;
pub const mmTPC3_CMDQ_CQ_IFIFO_CNT: u32 = 0xEC9108;
pub const mmTPC3_CMDQ_CP_MSG_BASE0_ADDR_LO: u32 = 0xEC9120;
pub const mmTPC3_CMDQ_CP_MSG_BASE0_ADDR_HI: u32 = 0xEC9124;
pub const mmTPC3_CMDQ_CP_MSG_BASE1_ADDR_LO: u32 = 0xEC9128;
pub const mmTPC3_CMDQ_CP_MSG_BASE1_ADDR_HI: u32 = 0xEC912C;
pub const mmTPC3_CMDQ_CP_MSG_BASE2_ADDR_LO: u32 = 0xEC9130;
pub const mmTPC3_CMDQ_CP_MSG_BASE2_ADDR_HI: u32 = 0xEC9134;
pub const mmTPC3_CMDQ_CP_MSG_BASE3_ADDR_LO: u32 = 0xEC9138;
pub const mmTPC3_CMDQ_CP_MSG_BASE3_ADDR_HI: u32 = 0xEC913C;
pub const mmTPC3_CMDQ_CP_LDMA_TSIZE_OFFSET: u32 = 0xEC9140;
pub const mmTPC3_CMDQ_CP_LDMA_SRC_BASE_LO_OFFSET: u32 = 0xEC9144;
pub const mmTPC3_CMDQ_CP_LDMA_SRC_BASE_HI_OFFSET: u32 = 0xEC9148;
pub const mmTPC3_CMDQ_CP_LDMA_DST_BASE_LO_OFFSET: u32 = 0xEC914C;
pub const mmTPC3_CMDQ_CP_LDMA_DST_BASE_HI_OFFSET: u32 = 0xEC9150;
pub const mmTPC3_CMDQ_CP_LDMA_COMMIT_OFFSET: u32 = 0xEC9154;
pub const mmTPC3_CMDQ_CP_FENCE0_RDATA: u32 = 0xEC9158;
pub const mmTPC3_CMDQ_CP_FENCE1_RDATA: u32 = 0xEC915C;
pub const mmTPC3_CMDQ_CP_FENCE2_RDATA: u32 = 0xEC9160;
pub const mmTPC3_CMDQ_CP_FENCE3_RDATA: u32 = 0xEC9164;
pub const mmTPC3_CMDQ_CP_FENCE0_CNT: u32 = 0xEC9168;
pub const mmTPC3_CMDQ_CP_FENCE1_CNT: u32 = 0xEC916C;
pub const mmTPC3_CMDQ_CP_FENCE2_CNT: u32 = 0xEC9170;
pub const mmTPC3_CMDQ_CP_FENCE3_CNT: u32 = 0xEC9174;
pub const mmTPC3_CMDQ_CP_STS: u32 = 0xEC9178;
pub const mmTPC3_CMDQ_CP_CURRENT_INST_LO: u32 = 0xEC917C;
pub const mmTPC3_CMDQ_CP_CURRENT_INST_HI: u32 = 0xEC9180;
pub const mmTPC3_CMDQ_CP_BARRIER_CFG: u32 = 0xEC9184;
pub const mmTPC3_CMDQ_CP_DBG_0: u32 = 0xEC9188;
pub const mmTPC3_CMDQ_CQ_BUF_ADDR: u32 = 0xEC9308;
pub const mmTPC3_CMDQ_CQ_BUF_RDATA: u32 = 0xEC930C;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
