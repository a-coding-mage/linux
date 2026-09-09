/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright 2016-2018 HabanaLabs, Ltd.
 * All Rights Reserved.
 */

/*
 ************************************
 ** This is an auto-generated file **
 **       DO NOT EDIT BELOW        **
 ************************************
 */

/*
 *****************************************
 *   TPC1_QM (Prototype: QMAN)
 *****************************************
 */

pub const mmTPC1_QM_GLBL_CFG0: u32 = 0xE48000;
pub const mmTPC1_QM_GLBL_CFG1: u32 = 0xE48004;
pub const mmTPC1_QM_GLBL_PROT: u32 = 0xE48008;
pub const mmTPC1_QM_GLBL_ERR_CFG: u32 = 0xE4800C;
pub const mmTPC1_QM_GLBL_ERR_ADDR_LO: u32 = 0xE48010;
pub const mmTPC1_QM_GLBL_ERR_ADDR_HI: u32 = 0xE48014;
pub const mmTPC1_QM_GLBL_ERR_WDATA: u32 = 0xE48018;
pub const mmTPC1_QM_GLBL_SECURE_PROPS: u32 = 0xE4801C;
pub const mmTPC1_QM_GLBL_NON_SECURE_PROPS: u32 = 0xE48020;
pub const mmTPC1_QM_GLBL_STS0: u32 = 0xE48024;
pub const mmTPC1_QM_GLBL_STS1: u32 = 0xE48028;
pub const mmTPC1_QM_PQ_BASE_LO: u32 = 0xE48060;
pub const mmTPC1_QM_PQ_BASE_HI: u32 = 0xE48064;
pub const mmTPC1_QM_PQ_SIZE: u32 = 0xE48068;
pub const mmTPC1_QM_PQ_PI: u32 = 0xE4806C;
pub const mmTPC1_QM_PQ_CI: u32 = 0xE48070;
pub const mmTPC1_QM_PQ_CFG0: u32 = 0xE48074;
pub const mmTPC1_QM_PQ_CFG1: u32 = 0xE48078;
pub const mmTPC1_QM_PQ_ARUSER: u32 = 0xE4807C;
pub const mmTPC1_QM_PQ_PUSH0: u32 = 0xE48080;
pub const mmTPC1_QM_PQ_PUSH1: u32 = 0xE48084;
pub const mmTPC1_QM_PQ_PUSH2: u32 = 0xE48088;
pub const mmTPC1_QM_PQ_PUSH3: u32 = 0xE4808C;
pub const mmTPC1_QM_PQ_STS0: u32 = 0xE48090;
pub const mmTPC1_QM_PQ_STS1: u32 = 0xE48094;
pub const mmTPC1_QM_PQ_RD_RATE_LIM_EN: u32 = 0xE480A0;
pub const mmTPC1_QM_PQ_RD_RATE_LIM_RST_TOKEN: u32 = 0xE480A4;
pub const mmTPC1_QM_PQ_RD_RATE_LIM_SAT: u32 = 0xE480A8;
pub const mmTPC1_QM_PQ_RD_RATE_LIM_TOUT: u32 = 0xE480AC;
pub const mmTPC1_QM_CQ_CFG0: u32 = 0xE480B0;
pub const mmTPC1_QM_CQ_CFG1: u32 = 0xE480B4;
pub const mmTPC1_QM_CQ_ARUSER: u32 = 0xE480B8;
pub const mmTPC1_QM_CQ_PTR_LO: u32 = 0xE480C0;
pub const mmTPC1_QM_CQ_PTR_HI: u32 = 0xE480C4;
pub const mmTPC1_QM_CQ_TSIZE: u32 = 0xE480C8;
pub const mmTPC1_QM_CQ_CTL: u32 = 0xE480CC;
pub const mmTPC1_QM_CQ_PTR_LO_STS: u32 = 0xE480D4;
pub const mmTPC1_QM_CQ_PTR_HI_STS: u32 = 0xE480D8;
pub const mmTPC1_QM_CQ_TSIZE_STS: u32 = 0xE480DC;
pub const mmTPC1_QM_CQ_CTL_STS: u32 = 0xE480E0;
pub const mmTPC1_QM_CQ_STS0: u32 = 0xE480E4;
pub const mmTPC1_QM_CQ_STS1: u32 = 0xE480E8;
pub const mmTPC1_QM_CQ_RD_RATE_LIM_EN: u32 = 0xE480F0;
pub const mmTPC1_QM_CQ_RD_RATE_LIM_RST_TOKEN: u32 = 0xE480F4;
pub const mmTPC1_QM_CQ_RD_RATE_LIM_SAT: u32 = 0xE480F8;
pub const mmTPC1_QM_CQ_RD_RATE_LIM_TOUT: u32 = 0xE480FC;
pub const mmTPC1_QM_CQ_IFIFO_CNT: u32 = 0xE48108;
pub const mmTPC1_QM_CP_MSG_BASE0_ADDR_LO: u32 = 0xE48120;
pub const mmTPC1_QM_CP_MSG_BASE0_ADDR_HI: u32 = 0xE48124;
pub const mmTPC1_QM_CP_MSG_BASE1_ADDR_LO: u32 = 0xE48128;
pub const mmTPC1_QM_CP_MSG_BASE1_ADDR_HI: u32 = 0xE4812C;
pub const mmTPC1_QM_CP_MSG_BASE2_ADDR_LO: u32 = 0xE48130;
pub const mmTPC1_QM_CP_MSG_BASE2_ADDR_HI: u32 = 0xE48134;
pub const mmTPC1_QM_CP_MSG_BASE3_ADDR_LO: u32 = 0xE48138;
pub const mmTPC1_QM_CP_MSG_BASE3_ADDR_HI: u32 = 0xE4813C;
pub const mmTPC1_QM_CP_LDMA_TSIZE_OFFSET: u32 = 0xE48140;
pub const mmTPC1_QM_CP_LDMA_SRC_BASE_LO_OFFSET: u32 = 0xE48144;
pub const mmTPC1_QM_CP_LDMA_SRC_BASE_HI_OFFSET: u32 = 0xE48148;
pub const mmTPC1_QM_CP_LDMA_DST_BASE_LO_OFFSET: u32 = 0xE4814C;
pub const mmTPC1_QM_CP_LDMA_DST_BASE_HI_OFFSET: u32 = 0xE48150;
pub const mmTPC1_QM_CP_LDMA_COMMIT_OFFSET: u32 = 0xE48154;
pub const mmTPC1_QM_CP_FENCE0_RDATA: u32 = 0xE48158;
pub const mmTPC1_QM_CP_FENCE1_RDATA: u32 = 0xE4815C;
pub const mmTPC1_QM_CP_FENCE2_RDATA: u32 = 0xE48160;
pub const mmTPC1_QM_CP_FENCE3_RDATA: u32 = 0xE48164;
pub const mmTPC1_QM_CP_FENCE0_CNT: u32 = 0xE48168;
pub const mmTPC1_QM_CP_FENCE1_CNT: u32 = 0xE4816C;
pub const mmTPC1_QM_CP_FENCE2_CNT: u32 = 0xE48170;
pub const mmTPC1_QM_CP_FENCE3_CNT: u32 = 0xE48174;
pub const mmTPC1_QM_CP_STS: u32 = 0xE48178;
pub const mmTPC1_QM_CP_CURRENT_INST_LO: u32 = 0xE4817C;
pub const mmTPC1_QM_CP_CURRENT_INST_HI: u32 = 0xE48180;
pub const mmTPC1_QM_CP_BARRIER_CFG: u32 = 0xE48184;
pub const mmTPC1_QM_CP_DBG_0: u32 = 0xE48188;
pub const mmTPC1_QM_PQ_BUF_ADDR: u32 = 0xE48300;
pub const mmTPC1_QM_PQ_BUF_RDATA: u32 = 0xE48304;
pub const mmTPC1_QM_CQ_BUF_ADDR: u32 = 0xE48308;
pub const mmTPC1_QM_CQ_BUF_RDATA: u32 = 0xE4830C;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
