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
 *   TPC5_CMDQ (Prototype: CMDQ)
 *****************************************
 */

pub const mmTPC5_CMDQ_GLBL_CFG0: u32 = 0xF49000;
pub const mmTPC5_CMDQ_GLBL_CFG1: u32 = 0xF49004;
pub const mmTPC5_CMDQ_GLBL_PROT: u32 = 0xF49008;
pub const mmTPC5_CMDQ_GLBL_ERR_CFG: u32 = 0xF4900C;
pub const mmTPC5_CMDQ_GLBL_ERR_ADDR_LO: u32 = 0xF49010;
pub const mmTPC5_CMDQ_GLBL_ERR_ADDR_HI: u32 = 0xF49014;
pub const mmTPC5_CMDQ_GLBL_ERR_WDATA: u32 = 0xF49018;
pub const mmTPC5_CMDQ_GLBL_SECURE_PROPS: u32 = 0xF4901C;
pub const mmTPC5_CMDQ_GLBL_NON_SECURE_PROPS: u32 = 0xF49020;
pub const mmTPC5_CMDQ_GLBL_STS0: u32 = 0xF49024;
pub const mmTPC5_CMDQ_GLBL_STS1: u32 = 0xF49028;
pub const mmTPC5_CMDQ_CQ_CFG0: u32 = 0xF490B0;
pub const mmTPC5_CMDQ_CQ_CFG1: u32 = 0xF490B4;
pub const mmTPC5_CMDQ_CQ_ARUSER: u32 = 0xF490B8;
pub const mmTPC5_CMDQ_CQ_PTR_LO: u32 = 0xF490C0;
pub const mmTPC5_CMDQ_CQ_PTR_HI: u32 = 0xF490C4;
pub const mmTPC5_CMDQ_CQ_TSIZE: u32 = 0xF490C8;
pub const mmTPC5_CMDQ_CQ_CTL: u32 = 0xF490CC;
pub const mmTPC5_CMDQ_CQ_PTR_LO_STS: u32 = 0xF490D4;
pub const mmTPC5_CMDQ_CQ_PTR_HI_STS: u32 = 0xF490D8;
pub const mmTPC5_CMDQ_CQ_TSIZE_STS: u32 = 0xF490DC;
pub const mmTPC5_CMDQ_CQ_CTL_STS: u32 = 0xF490E0;
pub const mmTPC5_CMDQ_CQ_STS0: u32 = 0xF490E4;
pub const mmTPC5_CMDQ_CQ_STS1: u32 = 0xF490E8;
pub const mmTPC5_CMDQ_CQ_RD_RATE_LIM_EN: u32 = 0xF490F0;
pub const mmTPC5_CMDQ_CQ_RD_RATE_LIM_RST_TOKEN: u32 = 0xF490F4;
pub const mmTPC5_CMDQ_CQ_RD_RATE_LIM_SAT: u32 = 0xF490F8;
pub const mmTPC5_CMDQ_CQ_RD_RATE_LIM_TOUT: u32 = 0xF490FC;
pub const mmTPC5_CMDQ_CQ_IFIFO_CNT: u32 = 0xF49108;
pub const mmTPC5_CMDQ_CP_MSG_BASE0_ADDR_LO: u32 = 0xF49120;
pub const mmTPC5_CMDQ_CP_MSG_BASE0_ADDR_HI: u32 = 0xF49124;
pub const mmTPC5_CMDQ_CP_MSG_BASE1_ADDR_LO: u32 = 0xF49128;
pub const mmTPC5_CMDQ_CP_MSG_BASE1_ADDR_HI: u32 = 0xF4912C;
pub const mmTPC5_CMDQ_CP_MSG_BASE2_ADDR_LO: u32 = 0xF49130;
pub const mmTPC5_CMDQ_CP_MSG_BASE2_ADDR_HI: u32 = 0xF49134;
pub const mmTPC5_CMDQ_CP_MSG_BASE3_ADDR_LO: u32 = 0xF49138;
pub const mmTPC5_CMDQ_CP_MSG_BASE3_ADDR_HI: u32 = 0xF4913C;
pub const mmTPC5_CMDQ_CP_LDMA_TSIZE_OFFSET: u32 = 0xF49140;
pub const mmTPC5_CMDQ_CP_LDMA_SRC_BASE_LO_OFFSET: u32 = 0xF49144;
pub const mmTPC5_CMDQ_CP_LDMA_SRC_BASE_HI_OFFSET: u32 = 0xF49148;
pub const mmTPC5_CMDQ_CP_LDMA_DST_BASE_LO_OFFSET: u32 = 0xF4914C;
pub const mmTPC5_CMDQ_CP_LDMA_DST_BASE_HI_OFFSET: u32 = 0xF49150;
pub const mmTPC5_CMDQ_CP_LDMA_COMMIT_OFFSET: u32 = 0xF49154;
pub const mmTPC5_CMDQ_CP_FENCE0_RDATA: u32 = 0xF49158;
pub const mmTPC5_CMDQ_CP_FENCE1_RDATA: u32 = 0xF4915C;
pub const mmTPC5_CMDQ_CP_FENCE2_RDATA: u32 = 0xF49160;
pub const mmTPC5_CMDQ_CP_FENCE3_RDATA: u32 = 0xF49164;
pub const mmTPC5_CMDQ_CP_FENCE0_CNT: u32 = 0xF49168;
pub const mmTPC5_CMDQ_CP_FENCE1_CNT: u32 = 0xF4916C;
pub const mmTPC5_CMDQ_CP_FENCE2_CNT: u32 = 0xF49170;
pub const mmTPC5_CMDQ_CP_FENCE3_CNT: u32 = 0xF49174;
pub const mmTPC5_CMDQ_CP_STS: u32 = 0xF49178;
pub const mmTPC5_CMDQ_CP_CURRENT_INST_LO: u32 = 0xF4917C;
pub const mmTPC5_CMDQ_CP_CURRENT_INST_HI: u32 = 0xF49180;
pub const mmTPC5_CMDQ_CP_BARRIER_CFG: u32 = 0xF49184;
pub const mmTPC5_CMDQ_CP_DBG_0: u32 = 0xF49188;
pub const mmTPC5_CMDQ_CQ_BUF_ADDR: u32 = 0xF49308;
pub const mmTPC5_CMDQ_CQ_BUF_RDATA: u32 = 0xF4930C;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
