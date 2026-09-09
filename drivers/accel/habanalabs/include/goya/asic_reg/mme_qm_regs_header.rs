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
 *   MME_QM (Prototype: QMAN)
 *****************************************
 */

pub const mmMME_QM_GLBL_CFG0: u32 = 0xD8000;
pub const mmMME_QM_GLBL_CFG1: u32 = 0xD8004;
pub const mmMME_QM_GLBL_PROT: u32 = 0xD8008;
pub const mmMME_QM_GLBL_ERR_CFG: u32 = 0xD800C;
pub const mmMME_QM_GLBL_ERR_ADDR_LO: u32 = 0xD8010;
pub const mmMME_QM_GLBL_ERR_ADDR_HI: u32 = 0xD8014;
pub const mmMME_QM_GLBL_ERR_WDATA: u32 = 0xD8018;
pub const mmMME_QM_GLBL_SECURE_PROPS: u32 = 0xD801C;
pub const mmMME_QM_GLBL_NON_SECURE_PROPS: u32 = 0xD8020;
pub const mmMME_QM_GLBL_STS0: u32 = 0xD8024;
pub const mmMME_QM_GLBL_STS1: u32 = 0xD8028;
pub const mmMME_QM_PQ_BASE_LO: u32 = 0xD8060;
pub const mmMME_QM_PQ_BASE_HI: u32 = 0xD8064;
pub const mmMME_QM_PQ_SIZE: u32 = 0xD8068;
pub const mmMME_QM_PQ_PI: u32 = 0xD806C;
pub const mmMME_QM_PQ_CI: u32 = 0xD8070;
pub const mmMME_QM_PQ_CFG0: u32 = 0xD8074;
pub const mmMME_QM_PQ_CFG1: u32 = 0xD8078;
pub const mmMME_QM_PQ_ARUSER: u32 = 0xD807C;
pub const mmMME_QM_PQ_PUSH0: u32 = 0xD8080;
pub const mmMME_QM_PQ_PUSH1: u32 = 0xD8084;
pub const mmMME_QM_PQ_PUSH2: u32 = 0xD8088;
pub const mmMME_QM_PQ_PUSH3: u32 = 0xD808C;
pub const mmMME_QM_PQ_STS0: u32 = 0xD8090;
pub const mmMME_QM_PQ_STS1: u32 = 0xD8094;
pub const mmMME_QM_PQ_RD_RATE_LIM_EN: u32 = 0xD80A0;
pub const mmMME_QM_PQ_RD_RATE_LIM_RST_TOKEN: u32 = 0xD80A4;
pub const mmMME_QM_PQ_RD_RATE_LIM_SAT: u32 = 0xD80A8;
pub const mmMME_QM_PQ_RD_RATE_LIM_TOUT: u32 = 0xD80AC;
pub const mmMME_QM_CQ_CFG0: u32 = 0xD80B0;
pub const mmMME_QM_CQ_CFG1: u32 = 0xD80B4;
pub const mmMME_QM_CQ_ARUSER: u32 = 0xD80B8;
pub const mmMME_QM_CQ_PTR_LO: u32 = 0xD80C0;
pub const mmMME_QM_CQ_PTR_HI: u32 = 0xD80C4;
pub const mmMME_QM_CQ_TSIZE: u32 = 0xD80C8;
pub const mmMME_QM_CQ_CTL: u32 = 0xD80CC;
pub const mmMME_QM_CQ_PTR_LO_STS: u32 = 0xD80D4;
pub const mmMME_QM_CQ_PTR_HI_STS: u32 = 0xD80D8;
pub const mmMME_QM_CQ_TSIZE_STS: u32 = 0xD80DC;
pub const mmMME_QM_CQ_CTL_STS: u32 = 0xD80E0;
pub const mmMME_QM_CQ_STS0: u32 = 0xD80E4;
pub const mmMME_QM_CQ_STS1: u32 = 0xD80E8;
pub const mmMME_QM_CQ_RD_RATE_LIM_EN: u32 = 0xD80F0;
pub const mmMME_QM_CQ_RD_RATE_LIM_RST_TOKEN: u32 = 0xD80F4;
pub const mmMME_QM_CQ_RD_RATE_LIM_SAT: u32 = 0xD80F8;
pub const mmMME_QM_CQ_RD_RATE_LIM_TOUT: u32 = 0xD80FC;
pub const mmMME_QM_CQ_IFIFO_CNT: u32 = 0xD8108;
pub const mmMME_QM_CP_MSG_BASE0_ADDR_LO: u32 = 0xD8120;
pub const mmMME_QM_CP_MSG_BASE0_ADDR_HI: u32 = 0xD8124;
pub const mmMME_QM_CP_MSG_BASE1_ADDR_LO: u32 = 0xD8128;
pub const mmMME_QM_CP_MSG_BASE1_ADDR_HI: u32 = 0xD812C;
pub const mmMME_QM_CP_MSG_BASE2_ADDR_LO: u32 = 0xD8130;
pub const mmMME_QM_CP_MSG_BASE2_ADDR_HI: u32 = 0xD8134;
pub const mmMME_QM_CP_MSG_BASE3_ADDR_LO: u32 = 0xD8138;
pub const mmMME_QM_CP_MSG_BASE3_ADDR_HI: u32 = 0xD813C;
pub const mmMME_QM_CP_LDMA_TSIZE_OFFSET: u32 = 0xD8140;
pub const mmMME_QM_CP_LDMA_SRC_BASE_LO_OFFSET: u32 = 0xD8144;
pub const mmMME_QM_CP_LDMA_SRC_BASE_HI_OFFSET: u32 = 0xD8148;
pub const mmMME_QM_CP_LDMA_DST_BASE_LO_OFFSET: u32 = 0xD814C;
pub const mmMME_QM_CP_LDMA_DST_BASE_HI_OFFSET: u32 = 0xD8150;
pub const mmMME_QM_CP_LDMA_COMMIT_OFFSET: u32 = 0xD8154;
pub const mmMME_QM_CP_FENCE0_RDATA: u32 = 0xD8158;
pub const mmMME_QM_CP_FENCE1_RDATA: u32 = 0xD815C;
pub const mmMME_QM_CP_FENCE2_RDATA: u32 = 0xD8160;
pub const mmMME_QM_CP_FENCE3_RDATA: u32 = 0xD8164;
pub const mmMME_QM_CP_FENCE0_CNT: u32 = 0xD8168;
pub const mmMME_QM_CP_FENCE1_CNT: u32 = 0xD816C;
pub const mmMME_QM_CP_FENCE2_CNT: u32 = 0xD8170;
pub const mmMME_QM_CP_FENCE3_CNT: u32 = 0xD8174;
pub const mmMME_QM_CP_STS: u32 = 0xD8178;
pub const mmMME_QM_CP_CURRENT_INST_LO: u32 = 0xD817C;
pub const mmMME_QM_CP_CURRENT_INST_HI: u32 = 0xD8180;
pub const mmMME_QM_CP_BARRIER_CFG: u32 = 0xD8184;
pub const mmMME_QM_CP_DBG_0: u32 = 0xD8188;
pub const mmMME_QM_PQ_BUF_ADDR: u32 = 0xD8300;
pub const mmMME_QM_PQ_BUF_RDATA: u32 = 0xD8304;
pub const mmMME_QM_CQ_BUF_ADDR: u32 = 0xD8308;
pub const mmMME_QM_CQ_BUF_RDATA: u32 = 0xD830C;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
