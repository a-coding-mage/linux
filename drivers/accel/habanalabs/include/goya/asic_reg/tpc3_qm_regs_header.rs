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
 *   TPC3_QM (Prototype: QMAN)
 *****************************************
 */

pub const mmTPC3_QM_GLBL_CFG0: u32 = 0xEC8000;
pub const mmTPC3_QM_GLBL_CFG1: u32 = 0xEC8004;
pub const mmTPC3_QM_GLBL_PROT: u32 = 0xEC8008;
pub const mmTPC3_QM_GLBL_ERR_CFG: u32 = 0xEC800C;
pub const mmTPC3_QM_GLBL_ERR_ADDR_LO: u32 = 0xEC8010;
pub const mmTPC3_QM_GLBL_ERR_ADDR_HI: u32 = 0xEC8014;
pub const mmTPC3_QM_GLBL_ERR_WDATA: u32 = 0xEC8018;
pub const mmTPC3_QM_GLBL_SECURE_PROPS: u32 = 0xEC801C;
pub const mmTPC3_QM_GLBL_NON_SECURE_PROPS: u32 = 0xEC8020;
pub const mmTPC3_QM_GLBL_STS0: u32 = 0xEC8024;
pub const mmTPC3_QM_GLBL_STS1: u32 = 0xEC8028;
pub const mmTPC3_QM_PQ_BASE_LO: u32 = 0xEC8060;
pub const mmTPC3_QM_PQ_BASE_HI: u32 = 0xEC8064;
pub const mmTPC3_QM_PQ_SIZE: u32 = 0xEC8068;
pub const mmTPC3_QM_PQ_PI: u32 = 0xEC806C;
pub const mmTPC3_QM_PQ_CI: u32 = 0xEC8070;
pub const mmTPC3_QM_PQ_CFG0: u32 = 0xEC8074;
pub const mmTPC3_QM_PQ_CFG1: u32 = 0xEC8078;
pub const mmTPC3_QM_PQ_ARUSER: u32 = 0xEC807C;
pub const mmTPC3_QM_PQ_PUSH0: u32 = 0xEC8080;
pub const mmTPC3_QM_PQ_PUSH1: u32 = 0xEC8084;
pub const mmTPC3_QM_PQ_PUSH2: u32 = 0xEC8088;
pub const mmTPC3_QM_PQ_PUSH3: u32 = 0xEC808C;
pub const mmTPC3_QM_PQ_STS0: u32 = 0xEC8090;
pub const mmTPC3_QM_PQ_STS1: u32 = 0xEC8094;
pub const mmTPC3_QM_PQ_RD_RATE_LIM_EN: u32 = 0xEC80A0;
pub const mmTPC3_QM_PQ_RD_RATE_LIM_RST_TOKEN: u32 = 0xEC80A4;
pub const mmTPC3_QM_PQ_RD_RATE_LIM_SAT: u32 = 0xEC80A8;
pub const mmTPC3_QM_PQ_RD_RATE_LIM_TOUT: u32 = 0xEC80AC;
pub const mmTPC3_QM_CQ_CFG0: u32 = 0xEC80B0;
pub const mmTPC3_QM_CQ_CFG1: u32 = 0xEC80B4;
pub const mmTPC3_QM_CQ_ARUSER: u32 = 0xEC80B8;
pub const mmTPC3_QM_CQ_PTR_LO: u32 = 0xEC80C0;
pub const mmTPC3_QM_CQ_PTR_HI: u32 = 0xEC80C4;
pub const mmTPC3_QM_CQ_TSIZE: u32 = 0xEC80C8;
pub const mmTPC3_QM_CQ_CTL: u32 = 0xEC80CC;
pub const mmTPC3_QM_CQ_PTR_LO_STS: u32 = 0xEC80D4;
pub const mmTPC3_QM_CQ_PTR_HI_STS: u32 = 0xEC80D8;
pub const mmTPC3_QM_CQ_TSIZE_STS: u32 = 0xEC80DC;
pub const mmTPC3_QM_CQ_CTL_STS: u32 = 0xEC80E0;
pub const mmTPC3_QM_CQ_STS0: u32 = 0xEC80E4;
pub const mmTPC3_QM_CQ_STS1: u32 = 0xEC80E8;
pub const mmTPC3_QM_CQ_RD_RATE_LIM_EN: u32 = 0xEC80F0;
pub const mmTPC3_QM_CQ_RD_RATE_LIM_RST_TOKEN: u32 = 0xEC80F4;
pub const mmTPC3_QM_CQ_RD_RATE_LIM_SAT: u32 = 0xEC80F8;
pub const mmTPC3_QM_CQ_RD_RATE_LIM_TOUT: u32 = 0xEC80FC;
pub const mmTPC3_QM_CQ_IFIFO_CNT: u32 = 0xEC8108;
pub const mmTPC3_QM_CP_MSG_BASE0_ADDR_LO: u32 = 0xEC8120;
pub const mmTPC3_QM_CP_MSG_BASE0_ADDR_HI: u32 = 0xEC8124;
pub const mmTPC3_QM_CP_MSG_BASE1_ADDR_LO: u32 = 0xEC8128;
pub const mmTPC3_QM_CP_MSG_BASE1_ADDR_HI: u32 = 0xEC812C;
pub const mmTPC3_QM_CP_MSG_BASE2_ADDR_LO: u32 = 0xEC8130;
pub const mmTPC3_QM_CP_MSG_BASE2_ADDR_HI: u32 = 0xEC8134;
pub const mmTPC3_QM_CP_MSG_BASE3_ADDR_LO: u32 = 0xEC8138;
pub const mmTPC3_QM_CP_MSG_BASE3_ADDR_HI: u32 = 0xEC813C;
pub const mmTPC3_QM_CP_LDMA_TSIZE_OFFSET: u32 = 0xEC8140;
pub const mmTPC3_QM_CP_LDMA_SRC_BASE_LO_OFFSET: u32 = 0xEC8144;
pub const mmTPC3_QM_CP_LDMA_SRC_BASE_HI_OFFSET: u32 = 0xEC8148;
pub const mmTPC3_QM_CP_LDMA_DST_BASE_LO_OFFSET: u32 = 0xEC814C;
pub const mmTPC3_QM_CP_LDMA_DST_BASE_HI_OFFSET: u32 = 0xEC8150;
pub const mmTPC3_QM_CP_LDMA_COMMIT_OFFSET: u32 = 0xEC8154;
pub const mmTPC3_QM_CP_FENCE0_RDATA: u32 = 0xEC8158;
pub const mmTPC3_QM_CP_FENCE1_RDATA: u32 = 0xEC815C;
pub const mmTPC3_QM_CP_FENCE2_RDATA: u32 = 0xEC8160;
pub const mmTPC3_QM_CP_FENCE3_RDATA: u32 = 0xEC8164;
pub const mmTPC3_QM_CP_FENCE0_CNT: u32 = 0xEC8168;
pub const mmTPC3_QM_CP_FENCE1_CNT: u32 = 0xEC816C;
pub const mmTPC3_QM_CP_FENCE2_CNT: u32 = 0xEC8170;
pub const mmTPC3_QM_CP_FENCE3_CNT: u32 = 0xEC8174;
pub const mmTPC3_QM_CP_STS: u32 = 0xEC8178;
pub const mmTPC3_QM_CP_CURRENT_INST_LO: u32 = 0xEC817C;
pub const mmTPC3_QM_CP_CURRENT_INST_HI: u32 = 0xEC8180;
pub const mmTPC3_QM_CP_BARRIER_CFG: u32 = 0xEC8184;
pub const mmTPC3_QM_CP_DBG_0: u32 = 0xEC8188;
pub const mmTPC3_QM_PQ_BUF_ADDR: u32 = 0xEC8300;
pub const mmTPC3_QM_PQ_BUF_RDATA: u32 = 0xEC8304;
pub const mmTPC3_QM_CQ_BUF_ADDR: u32 = 0xEC8308;
pub const mmTPC3_QM_CQ_BUF_RDATA: u32 = 0xEC830C;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
