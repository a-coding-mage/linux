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
 *   TPC0_QM (Prototype: QMAN)
 *****************************************
 */

pub const mmTPC0_QM_GLBL_CFG0: u32 = 0xE08000;
pub const mmTPC0_QM_GLBL_CFG1: u32 = 0xE08004;
pub const mmTPC0_QM_GLBL_PROT: u32 = 0xE08008;
pub const mmTPC0_QM_GLBL_ERR_CFG: u32 = 0xE0800C;
pub const mmTPC0_QM_GLBL_ERR_ADDR_LO: u32 = 0xE08010;
pub const mmTPC0_QM_GLBL_ERR_ADDR_HI: u32 = 0xE08014;
pub const mmTPC0_QM_GLBL_ERR_WDATA: u32 = 0xE08018;
pub const mmTPC0_QM_GLBL_SECURE_PROPS: u32 = 0xE0801C;
pub const mmTPC0_QM_GLBL_NON_SECURE_PROPS: u32 = 0xE08020;
pub const mmTPC0_QM_GLBL_STS0: u32 = 0xE08024;
pub const mmTPC0_QM_GLBL_STS1: u32 = 0xE08028;
pub const mmTPC0_QM_PQ_BASE_LO: u32 = 0xE08060;
pub const mmTPC0_QM_PQ_BASE_HI: u32 = 0xE08064;
pub const mmTPC0_QM_PQ_SIZE: u32 = 0xE08068;
pub const mmTPC0_QM_PQ_PI: u32 = 0xE0806C;
pub const mmTPC0_QM_PQ_CI: u32 = 0xE08070;
pub const mmTPC0_QM_PQ_CFG0: u32 = 0xE08074;
pub const mmTPC0_QM_PQ_CFG1: u32 = 0xE08078;
pub const mmTPC0_QM_PQ_ARUSER: u32 = 0xE0807C;
pub const mmTPC0_QM_PQ_PUSH0: u32 = 0xE08080;
pub const mmTPC0_QM_PQ_PUSH1: u32 = 0xE08084;
pub const mmTPC0_QM_PQ_PUSH2: u32 = 0xE08088;
pub const mmTPC0_QM_PQ_PUSH3: u32 = 0xE0808C;
pub const mmTPC0_QM_PQ_STS0: u32 = 0xE08090;
pub const mmTPC0_QM_PQ_STS1: u32 = 0xE08094;
pub const mmTPC0_QM_PQ_RD_RATE_LIM_EN: u32 = 0xE080A0;
pub const mmTPC0_QM_PQ_RD_RATE_LIM_RST_TOKEN: u32 = 0xE080A4;
pub const mmTPC0_QM_PQ_RD_RATE_LIM_SAT: u32 = 0xE080A8;
pub const mmTPC0_QM_PQ_RD_RATE_LIM_TOUT: u32 = 0xE080AC;
pub const mmTPC0_QM_CQ_CFG0: u32 = 0xE080B0;
pub const mmTPC0_QM_CQ_CFG1: u32 = 0xE080B4;
pub const mmTPC0_QM_CQ_ARUSER: u32 = 0xE080B8;
pub const mmTPC0_QM_CQ_PTR_LO: u32 = 0xE080C0;
pub const mmTPC0_QM_CQ_PTR_HI: u32 = 0xE080C4;
pub const mmTPC0_QM_CQ_TSIZE: u32 = 0xE080C8;
pub const mmTPC0_QM_CQ_CTL: u32 = 0xE080CC;
pub const mmTPC0_QM_CQ_PTR_LO_STS: u32 = 0xE080D4;
pub const mmTPC0_QM_CQ_PTR_HI_STS: u32 = 0xE080D8;
pub const mmTPC0_QM_CQ_TSIZE_STS: u32 = 0xE080DC;
pub const mmTPC0_QM_CQ_CTL_STS: u32 = 0xE080E0;
pub const mmTPC0_QM_CQ_STS0: u32 = 0xE080E4;
pub const mmTPC0_QM_CQ_STS1: u32 = 0xE080E8;
pub const mmTPC0_QM_CQ_RD_RATE_LIM_EN: u32 = 0xE080F0;
pub const mmTPC0_QM_CQ_RD_RATE_LIM_RST_TOKEN: u32 = 0xE080F4;
pub const mmTPC0_QM_CQ_RD_RATE_LIM_SAT: u32 = 0xE080F8;
pub const mmTPC0_QM_CQ_RD_RATE_LIM_TOUT: u32 = 0xE080FC;
pub const mmTPC0_QM_CQ_IFIFO_CNT: u32 = 0xE08108;
pub const mmTPC0_QM_CP_MSG_BASE0_ADDR_LO: u32 = 0xE08120;
pub const mmTPC0_QM_CP_MSG_BASE0_ADDR_HI: u32 = 0xE08124;
pub const mmTPC0_QM_CP_MSG_BASE1_ADDR_LO: u32 = 0xE08128;
pub const mmTPC0_QM_CP_MSG_BASE1_ADDR_HI: u32 = 0xE0812C;
pub const mmTPC0_QM_CP_MSG_BASE2_ADDR_LO: u32 = 0xE08130;
pub const mmTPC0_QM_CP_MSG_BASE2_ADDR_HI: u32 = 0xE08134;
pub const mmTPC0_QM_CP_MSG_BASE3_ADDR_LO: u32 = 0xE08138;
pub const mmTPC0_QM_CP_MSG_BASE3_ADDR_HI: u32 = 0xE0813C;
pub const mmTPC0_QM_CP_LDMA_TSIZE_OFFSET: u32 = 0xE08140;
pub const mmTPC0_QM_CP_LDMA_SRC_BASE_LO_OFFSET: u32 = 0xE08144;
pub const mmTPC0_QM_CP_LDMA_SRC_BASE_HI_OFFSET: u32 = 0xE08148;
pub const mmTPC0_QM_CP_LDMA_DST_BASE_LO_OFFSET: u32 = 0xE0814C;
pub const mmTPC0_QM_CP_LDMA_DST_BASE_HI_OFFSET: u32 = 0xE08150;
pub const mmTPC0_QM_CP_LDMA_COMMIT_OFFSET: u32 = 0xE08154;
pub const mmTPC0_QM_CP_FENCE0_RDATA: u32 = 0xE08158;
pub const mmTPC0_QM_CP_FENCE1_RDATA: u32 = 0xE0815C;
pub const mmTPC0_QM_CP_FENCE2_RDATA: u32 = 0xE08160;
pub const mmTPC0_QM_CP_FENCE3_RDATA: u32 = 0xE08164;
pub const mmTPC0_QM_CP_FENCE0_CNT: u32 = 0xE08168;
pub const mmTPC0_QM_CP_FENCE1_CNT: u32 = 0xE0816C;
pub const mmTPC0_QM_CP_FENCE2_CNT: u32 = 0xE08170;
pub const mmTPC0_QM_CP_FENCE3_CNT: u32 = 0xE08174;
pub const mmTPC0_QM_CP_STS: u32 = 0xE08178;
pub const mmTPC0_QM_CP_CURRENT_INST_LO: u32 = 0xE0817C;
pub const mmTPC0_QM_CP_CURRENT_INST_HI: u32 = 0xE08180;
pub const mmTPC0_QM_CP_BARRIER_CFG: u32 = 0xE08184;
pub const mmTPC0_QM_CP_DBG_0: u32 = 0xE08188;
pub const mmTPC0_QM_PQ_BUF_ADDR: u32 = 0xE08300;
pub const mmTPC0_QM_PQ_BUF_RDATA: u32 = 0xE08304;
pub const mmTPC0_QM_CQ_BUF_ADDR: u32 = 0xE08308;
pub const mmTPC0_QM_CQ_BUF_RDATA: u32 = 0xE0830C;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
