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
 *   TPC7_QM (Prototype: QMAN)
 *****************************************
 */

pub const mmTPC7_QM_GLBL_CFG0: u32 = 0xFC8000;
pub const mmTPC7_QM_GLBL_CFG1: u32 = 0xFC8004;
pub const mmTPC7_QM_GLBL_PROT: u32 = 0xFC8008;
pub const mmTPC7_QM_GLBL_ERR_CFG: u32 = 0xFC800C;
pub const mmTPC7_QM_GLBL_ERR_ADDR_LO: u32 = 0xFC8010;
pub const mmTPC7_QM_GLBL_ERR_ADDR_HI: u32 = 0xFC8014;
pub const mmTPC7_QM_GLBL_ERR_WDATA: u32 = 0xFC8018;
pub const mmTPC7_QM_GLBL_SECURE_PROPS: u32 = 0xFC801C;
pub const mmTPC7_QM_GLBL_NON_SECURE_PROPS: u32 = 0xFC8020;
pub const mmTPC7_QM_GLBL_STS0: u32 = 0xFC8024;
pub const mmTPC7_QM_GLBL_STS1: u32 = 0xFC8028;
pub const mmTPC7_QM_PQ_BASE_LO: u32 = 0xFC8060;
pub const mmTPC7_QM_PQ_BASE_HI: u32 = 0xFC8064;
pub const mmTPC7_QM_PQ_SIZE: u32 = 0xFC8068;
pub const mmTPC7_QM_PQ_PI: u32 = 0xFC806C;
pub const mmTPC7_QM_PQ_CI: u32 = 0xFC8070;
pub const mmTPC7_QM_PQ_CFG0: u32 = 0xFC8074;
pub const mmTPC7_QM_PQ_CFG1: u32 = 0xFC8078;
pub const mmTPC7_QM_PQ_ARUSER: u32 = 0xFC807C;
pub const mmTPC7_QM_PQ_PUSH0: u32 = 0xFC8080;
pub const mmTPC7_QM_PQ_PUSH1: u32 = 0xFC8084;
pub const mmTPC7_QM_PQ_PUSH2: u32 = 0xFC8088;
pub const mmTPC7_QM_PQ_PUSH3: u32 = 0xFC808C;
pub const mmTPC7_QM_PQ_STS0: u32 = 0xFC8090;
pub const mmTPC7_QM_PQ_STS1: u32 = 0xFC8094;
pub const mmTPC7_QM_PQ_RD_RATE_LIM_EN: u32 = 0xFC80A0;
pub const mmTPC7_QM_PQ_RD_RATE_LIM_RST_TOKEN: u32 = 0xFC80A4;
pub const mmTPC7_QM_PQ_RD_RATE_LIM_SAT: u32 = 0xFC80A8;
pub const mmTPC7_QM_PQ_RD_RATE_LIM_TOUT: u32 = 0xFC80AC;
pub const mmTPC7_QM_CQ_CFG0: u32 = 0xFC80B0;
pub const mmTPC7_QM_CQ_CFG1: u32 = 0xFC80B4;
pub const mmTPC7_QM_CQ_ARUSER: u32 = 0xFC80B8;
pub const mmTPC7_QM_CQ_PTR_LO: u32 = 0xFC80C0;
pub const mmTPC7_QM_CQ_PTR_HI: u32 = 0xFC80C4;
pub const mmTPC7_QM_CQ_TSIZE: u32 = 0xFC80C8;
pub const mmTPC7_QM_CQ_CTL: u32 = 0xFC80CC;
pub const mmTPC7_QM_CQ_PTR_LO_STS: u32 = 0xFC80D4;
pub const mmTPC7_QM_CQ_PTR_HI_STS: u32 = 0xFC80D8;
pub const mmTPC7_QM_CQ_TSIZE_STS: u32 = 0xFC80DC;
pub const mmTPC7_QM_CQ_CTL_STS: u32 = 0xFC80E0;
pub const mmTPC7_QM_CQ_STS0: u32 = 0xFC80E4;
pub const mmTPC7_QM_CQ_STS1: u32 = 0xFC80E8;
pub const mmTPC7_QM_CQ_RD_RATE_LIM_EN: u32 = 0xFC80F0;
pub const mmTPC7_QM_CQ_RD_RATE_LIM_RST_TOKEN: u32 = 0xFC80F4;
pub const mmTPC7_QM_CQ_RD_RATE_LIM_SAT: u32 = 0xFC80F8;
pub const mmTPC7_QM_CQ_RD_RATE_LIM_TOUT: u32 = 0xFC80FC;
pub const mmTPC7_QM_CQ_IFIFO_CNT: u32 = 0xFC8108;
pub const mmTPC7_QM_CP_MSG_BASE0_ADDR_LO: u32 = 0xFC8120;
pub const mmTPC7_QM_CP_MSG_BASE0_ADDR_HI: u32 = 0xFC8124;
pub const mmTPC7_QM_CP_MSG_BASE1_ADDR_LO: u32 = 0xFC8128;
pub const mmTPC7_QM_CP_MSG_BASE1_ADDR_HI: u32 = 0xFC812C;
pub const mmTPC7_QM_CP_MSG_BASE2_ADDR_LO: u32 = 0xFC8130;
pub const mmTPC7_QM_CP_MSG_BASE2_ADDR_HI: u32 = 0xFC8134;
pub const mmTPC7_QM_CP_MSG_BASE3_ADDR_LO: u32 = 0xFC8138;
pub const mmTPC7_QM_CP_MSG_BASE3_ADDR_HI: u32 = 0xFC813C;
pub const mmTPC7_QM_CP_LDMA_TSIZE_OFFSET: u32 = 0xFC8140;
pub const mmTPC7_QM_CP_LDMA_SRC_BASE_LO_OFFSET: u32 = 0xFC8144;
pub const mmTPC7_QM_CP_LDMA_SRC_BASE_HI_OFFSET: u32 = 0xFC8148;
pub const mmTPC7_QM_CP_LDMA_DST_BASE_LO_OFFSET: u32 = 0xFC814C;
pub const mmTPC7_QM_CP_LDMA_DST_BASE_HI_OFFSET: u32 = 0xFC8150;
pub const mmTPC7_QM_CP_LDMA_COMMIT_OFFSET: u32 = 0xFC8154;
pub const mmTPC7_QM_CP_FENCE0_RDATA: u32 = 0xFC8158;
pub const mmTPC7_QM_CP_FENCE1_RDATA: u32 = 0xFC815C;
pub const mmTPC7_QM_CP_FENCE2_RDATA: u32 = 0xFC8160;
pub const mmTPC7_QM_CP_FENCE3_RDATA: u32 = 0xFC8164;
pub const mmTPC7_QM_CP_FENCE0_CNT: u32 = 0xFC8168;
pub const mmTPC7_QM_CP_FENCE1_CNT: u32 = 0xFC816C;
pub const mmTPC7_QM_CP_FENCE2_CNT: u32 = 0xFC8170;
pub const mmTPC7_QM_CP_FENCE3_CNT: u32 = 0xFC8174;
pub const mmTPC7_QM_CP_STS: u32 = 0xFC8178;
pub const mmTPC7_QM_CP_CURRENT_INST_LO: u32 = 0xFC817C;
pub const mmTPC7_QM_CP_CURRENT_INST_HI: u32 = 0xFC8180;
pub const mmTPC7_QM_CP_BARRIER_CFG: u32 = 0xFC8184;
pub const mmTPC7_QM_CP_DBG_0: u32 = 0xFC8188;
pub const mmTPC7_QM_PQ_BUF_ADDR: u32 = 0xFC8300;
pub const mmTPC7_QM_PQ_BUF_RDATA: u32 = 0xFC8304;
pub const mmTPC7_QM_CQ_BUF_ADDR: u32 = 0xFC8308;
pub const mmTPC7_QM_CQ_BUF_RDATA: u32 = 0xFC830C;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
