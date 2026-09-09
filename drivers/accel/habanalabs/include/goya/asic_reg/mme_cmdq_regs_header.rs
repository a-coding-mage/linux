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
 *   MME_CMDQ (Prototype: CMDQ)
 *****************************************
 */

pub const mmMME_CMDQ_GLBL_CFG0: u32 = 0xD9000;
pub const mmMME_CMDQ_GLBL_CFG1: u32 = 0xD9004;
pub const mmMME_CMDQ_GLBL_PROT: u32 = 0xD9008;
pub const mmMME_CMDQ_GLBL_ERR_CFG: u32 = 0xD900C;
pub const mmMME_CMDQ_GLBL_ERR_ADDR_LO: u32 = 0xD9010;
pub const mmMME_CMDQ_GLBL_ERR_ADDR_HI: u32 = 0xD9014;
pub const mmMME_CMDQ_GLBL_ERR_WDATA: u32 = 0xD9018;
pub const mmMME_CMDQ_GLBL_SECURE_PROPS: u32 = 0xD901C;
pub const mmMME_CMDQ_GLBL_NON_SECURE_PROPS: u32 = 0xD9020;
pub const mmMME_CMDQ_GLBL_STS0: u32 = 0xD9024;
pub const mmMME_CMDQ_GLBL_STS1: u32 = 0xD9028;
pub const mmMME_CMDQ_CQ_CFG0: u32 = 0xD90B0;
pub const mmMME_CMDQ_CQ_CFG1: u32 = 0xD90B4;
pub const mmMME_CMDQ_CQ_ARUSER: u32 = 0xD90B8;
pub const mmMME_CMDQ_CQ_PTR_LO: u32 = 0xD90C0;
pub const mmMME_CMDQ_CQ_PTR_HI: u32 = 0xD90C4;
pub const mmMME_CMDQ_CQ_TSIZE: u32 = 0xD90C8;
pub const mmMME_CMDQ_CQ_CTL: u32 = 0xD90CC;
pub const mmMME_CMDQ_CQ_PTR_LO_STS: u32 = 0xD90D4;
pub const mmMME_CMDQ_CQ_PTR_HI_STS: u32 = 0xD90D8;
pub const mmMME_CMDQ_CQ_TSIZE_STS: u32 = 0xD90DC;
pub const mmMME_CMDQ_CQ_CTL_STS: u32 = 0xD90E0;
pub const mmMME_CMDQ_CQ_STS0: u32 = 0xD90E4;
pub const mmMME_CMDQ_CQ_STS1: u32 = 0xD90E8;
pub const mmMME_CMDQ_CQ_RD_RATE_LIM_EN: u32 = 0xD90F0;
pub const mmMME_CMDQ_CQ_RD_RATE_LIM_RST_TOKEN: u32 = 0xD90F4;
pub const mmMME_CMDQ_CQ_RD_RATE_LIM_SAT: u32 = 0xD90F8;
pub const mmMME_CMDQ_CQ_RD_RATE_LIM_TOUT: u32 = 0xD90FC;
pub const mmMME_CMDQ_CQ_IFIFO_CNT: u32 = 0xD9108;
pub const mmMME_CMDQ_CP_MSG_BASE0_ADDR_LO: u32 = 0xD9120;
pub const mmMME_CMDQ_CP_MSG_BASE0_ADDR_HI: u32 = 0xD9124;
pub const mmMME_CMDQ_CP_MSG_BASE1_ADDR_LO: u32 = 0xD9128;
pub const mmMME_CMDQ_CP_MSG_BASE1_ADDR_HI: u32 = 0xD912C;
pub const mmMME_CMDQ_CP_MSG_BASE2_ADDR_LO: u32 = 0xD9130;
pub const mmMME_CMDQ_CP_MSG_BASE2_ADDR_HI: u32 = 0xD9134;
pub const mmMME_CMDQ_CP_MSG_BASE3_ADDR_LO: u32 = 0xD9138;
pub const mmMME_CMDQ_CP_MSG_BASE3_ADDR_HI: u32 = 0xD913C;
pub const mmMME_CMDQ_CP_LDMA_TSIZE_OFFSET: u32 = 0xD9140;
pub const mmMME_CMDQ_CP_LDMA_SRC_BASE_LO_OFFSET: u32 = 0xD9144;
pub const mmMME_CMDQ_CP_LDMA_SRC_BASE_HI_OFFSET: u32 = 0xD9148;
pub const mmMME_CMDQ_CP_LDMA_DST_BASE_LO_OFFSET: u32 = 0xD914C;
pub const mmMME_CMDQ_CP_LDMA_DST_BASE_HI_OFFSET: u32 = 0xD9150;
pub const mmMME_CMDQ_CP_LDMA_COMMIT_OFFSET: u32 = 0xD9154;
pub const mmMME_CMDQ_CP_FENCE0_RDATA: u32 = 0xD9158;
pub const mmMME_CMDQ_CP_FENCE1_RDATA: u32 = 0xD915C;
pub const mmMME_CMDQ_CP_FENCE2_RDATA: u32 = 0xD9160;
pub const mmMME_CMDQ_CP_FENCE3_RDATA: u32 = 0xD9164;
pub const mmMME_CMDQ_CP_FENCE0_CNT: u32 = 0xD9168;
pub const mmMME_CMDQ_CP_FENCE1_CNT: u32 = 0xD916C;
pub const mmMME_CMDQ_CP_FENCE2_CNT: u32 = 0xD9170;
pub const mmMME_CMDQ_CP_FENCE3_CNT: u32 = 0xD9174;
pub const mmMME_CMDQ_CP_STS: u32 = 0xD9178;
pub const mmMME_CMDQ_CP_CURRENT_INST_LO: u32 = 0xD917C;
pub const mmMME_CMDQ_CP_CURRENT_INST_HI: u32 = 0xD9180;
pub const mmMME_CMDQ_CP_BARRIER_CFG: u32 = 0xD9184;
pub const mmMME_CMDQ_CP_DBG_0: u32 = 0xD9188;
pub const mmMME_CMDQ_CQ_BUF_ADDR: u32 = 0xD9308;
pub const mmMME_CMDQ_CQ_BUF_RDATA: u32 = 0xD930C;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
