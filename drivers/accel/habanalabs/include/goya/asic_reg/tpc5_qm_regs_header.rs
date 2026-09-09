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
 *   TPC5_QM (Prototype: QMAN)
 *****************************************
 */

pub const mmTPC5_QM_GLBL_CFG0: u32 = 0xF48000;
pub const mmTPC5_QM_GLBL_CFG1: u32 = 0xF48004;
pub const mmTPC5_QM_GLBL_PROT: u32 = 0xF48008;
pub const mmTPC5_QM_GLBL_ERR_CFG: u32 = 0xF4800C;
pub const mmTPC5_QM_GLBL_ERR_ADDR_LO: u32 = 0xF48010;
pub const mmTPC5_QM_GLBL_ERR_ADDR_HI: u32 = 0xF48014;
pub const mmTPC5_QM_GLBL_ERR_WDATA: u32 = 0xF48018;
pub const mmTPC5_QM_GLBL_SECURE_PROPS: u32 = 0xF4801C;
pub const mmTPC5_QM_GLBL_NON_SECURE_PROPS: u32 = 0xF48020;
pub const mmTPC5_QM_GLBL_STS0: u32 = 0xF48024;
pub const mmTPC5_QM_GLBL_STS1: u32 = 0xF48028;
pub const mmTPC5_QM_PQ_BASE_LO: u32 = 0xF48060;
pub const mmTPC5_QM_PQ_BASE_HI: u32 = 0xF48064;
pub const mmTPC5_QM_PQ_SIZE: u32 = 0xF48068;
pub const mmTPC5_QM_PQ_PI: u32 = 0xF4806C;
pub const mmTPC5_QM_PQ_CI: u32 = 0xF48070;
pub const mmTPC5_QM_PQ_CFG0: u32 = 0xF48074;
pub const mmTPC5_QM_PQ_CFG1: u32 = 0xF48078;
pub const mmTPC5_QM_PQ_ARUSER: u32 = 0xF4807C;
pub const mmTPC5_QM_PQ_PUSH0: u32 = 0xF48080;
pub const mmTPC5_QM_PQ_PUSH1: u32 = 0xF48084;
pub const mmTPC5_QM_PQ_PUSH2: u32 = 0xF48088;
pub const mmTPC5_QM_PQ_PUSH3: u32 = 0xF4808C;
pub const mmTPC5_QM_PQ_STS0: u32 = 0xF48090;
pub const mmTPC5_QM_PQ_STS1: u32 = 0xF48094;
pub const mmTPC5_QM_PQ_RD_RATE_LIM_EN: u32 = 0xF480A0;
pub const mmTPC5_QM_PQ_RD_RATE_LIM_RST_TOKEN: u32 = 0xF480A4;
pub const mmTPC5_QM_PQ_RD_RATE_LIM_SAT: u32 = 0xF480A8;
pub const mmTPC5_QM_PQ_RD_RATE_LIM_TOUT: u32 = 0xF480AC;
pub const mmTPC5_QM_CQ_CFG0: u32 = 0xF480B0;
pub const mmTPC5_QM_CQ_CFG1: u32 = 0xF480B4;
pub const mmTPC5_QM_CQ_ARUSER: u32 = 0xF480B8;
pub const mmTPC5_QM_CQ_PTR_LO: u32 = 0xF480C0;
pub const mmTPC5_QM_CQ_PTR_HI: u32 = 0xF480C4;
pub const mmTPC5_QM_CQ_TSIZE: u32 = 0xF480C8;
pub const mmTPC5_QM_CQ_CTL: u32 = 0xF480CC;
pub const mmTPC5_QM_CQ_PTR_LO_STS: u32 = 0xF480D4;
pub const mmTPC5_QM_CQ_PTR_HI_STS: u32 = 0xF480D8;
pub const mmTPC5_QM_CQ_TSIZE_STS: u32 = 0xF480DC;
pub const mmTPC5_QM_CQ_CTL_STS: u32 = 0xF480E0;
pub const mmTPC5_QM_CQ_STS0: u32 = 0xF480E4;
pub const mmTPC5_QM_CQ_STS1: u32 = 0xF480E8;
pub const mmTPC5_QM_CQ_RD_RATE_LIM_EN: u32 = 0xF480F0;
pub const mmTPC5_QM_CQ_RD_RATE_LIM_RST_TOKEN: u32 = 0xF480F4;
pub const mmTPC5_QM_CQ_RD_RATE_LIM_SAT: u32 = 0xF480F8;
pub const mmTPC5_QM_CQ_RD_RATE_LIM_TOUT: u32 = 0xF480FC;
pub const mmTPC5_QM_CQ_IFIFO_CNT: u32 = 0xF48108;
pub const mmTPC5_QM_CP_MSG_BASE0_ADDR_LO: u32 = 0xF48120;
pub const mmTPC5_QM_CP_MSG_BASE0_ADDR_HI: u32 = 0xF48124;
pub const mmTPC5_QM_CP_MSG_BASE1_ADDR_LO: u32 = 0xF48128;
pub const mmTPC5_QM_CP_MSG_BASE1_ADDR_HI: u32 = 0xF4812C;
pub const mmTPC5_QM_CP_MSG_BASE2_ADDR_LO: u32 = 0xF48130;
pub const mmTPC5_QM_CP_MSG_BASE2_ADDR_HI: u32 = 0xF48134;
pub const mmTPC5_QM_CP_MSG_BASE3_ADDR_LO: u32 = 0xF48138;
pub const mmTPC5_QM_CP_MSG_BASE3_ADDR_HI: u32 = 0xF4813C;
pub const mmTPC5_QM_CP_LDMA_TSIZE_OFFSET: u32 = 0xF48140;
pub const mmTPC5_QM_CP_LDMA_SRC_BASE_LO_OFFSET: u32 = 0xF48144;
pub const mmTPC5_QM_CP_LDMA_SRC_BASE_HI_OFFSET: u32 = 0xF48148;
pub const mmTPC5_QM_CP_LDMA_DST_BASE_LO_OFFSET: u32 = 0xF4814C;
pub const mmTPC5_QM_CP_LDMA_DST_BASE_HI_OFFSET: u32 = 0xF48150;
pub const mmTPC5_QM_CP_LDMA_COMMIT_OFFSET: u32 = 0xF48154;
pub const mmTPC5_QM_CP_FENCE0_RDATA: u32 = 0xF48158;
pub const mmTPC5_QM_CP_FENCE1_RDATA: u32 = 0xF4815C;
pub const mmTPC5_QM_CP_FENCE2_RDATA: u32 = 0xF48160;
pub const mmTPC5_QM_CP_FENCE3_RDATA: u32 = 0xF48164;
pub const mmTPC5_QM_CP_FENCE0_CNT: u32 = 0xF48168;
pub const mmTPC5_QM_CP_FENCE1_CNT: u32 = 0xF4816C;
pub const mmTPC5_QM_CP_FENCE2_CNT: u32 = 0xF48170;
pub const mmTPC5_QM_CP_FENCE3_CNT: u32 = 0xF48174;
pub const mmTPC5_QM_CP_STS: u32 = 0xF48178;
pub const mmTPC5_QM_CP_CURRENT_INST_LO: u32 = 0xF4817C;
pub const mmTPC5_QM_CP_CURRENT_INST_HI: u32 = 0xF48180;
pub const mmTPC5_QM_CP_BARRIER_CFG: u32 = 0xF48184;
pub const mmTPC5_QM_CP_DBG_0: u32 = 0xF48188;
pub const mmTPC5_QM_PQ_BUF_ADDR: u32 = 0xF48300;
pub const mmTPC5_QM_PQ_BUF_RDATA: u32 = 0xF48304;
pub const mmTPC5_QM_CQ_BUF_ADDR: u32 = 0xF48308;
pub const mmTPC5_QM_CQ_BUF_RDATA: u32 = 0xF4830C;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
