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
 *   DMA_QM_1 (Prototype: QMAN)
 *****************************************
 */

pub const mmDMA_QM_1_GLBL_CFG0: u32 = 0x408000;
pub const mmDMA_QM_1_GLBL_CFG1: u32 = 0x408004;
pub const mmDMA_QM_1_GLBL_PROT: u32 = 0x408008;
pub const mmDMA_QM_1_GLBL_ERR_CFG: u32 = 0x40800C;
pub const mmDMA_QM_1_GLBL_ERR_ADDR_LO: u32 = 0x408010;
pub const mmDMA_QM_1_GLBL_ERR_ADDR_HI: u32 = 0x408014;
pub const mmDMA_QM_1_GLBL_ERR_WDATA: u32 = 0x408018;
pub const mmDMA_QM_1_GLBL_SECURE_PROPS: u32 = 0x40801C;
pub const mmDMA_QM_1_GLBL_NON_SECURE_PROPS: u32 = 0x408020;
pub const mmDMA_QM_1_GLBL_STS0: u32 = 0x408024;
pub const mmDMA_QM_1_GLBL_STS1: u32 = 0x408028;
pub const mmDMA_QM_1_PQ_BASE_LO: u32 = 0x408060;
pub const mmDMA_QM_1_PQ_BASE_HI: u32 = 0x408064;
pub const mmDMA_QM_1_PQ_SIZE: u32 = 0x408068;
pub const mmDMA_QM_1_PQ_PI: u32 = 0x40806C;
pub const mmDMA_QM_1_PQ_CI: u32 = 0x408070;
pub const mmDMA_QM_1_PQ_CFG0: u32 = 0x408074;
pub const mmDMA_QM_1_PQ_CFG1: u32 = 0x408078;
pub const mmDMA_QM_1_PQ_ARUSER: u32 = 0x40807C;
pub const mmDMA_QM_1_PQ_PUSH0: u32 = 0x408080;
pub const mmDMA_QM_1_PQ_PUSH1: u32 = 0x408084;
pub const mmDMA_QM_1_PQ_PUSH2: u32 = 0x408088;
pub const mmDMA_QM_1_PQ_PUSH3: u32 = 0x40808C;
pub const mmDMA_QM_1_PQ_STS0: u32 = 0x408090;
pub const mmDMA_QM_1_PQ_STS1: u32 = 0x408094;
pub const mmDMA_QM_1_PQ_RD_RATE_LIM_EN: u32 = 0x4080A0;
pub const mmDMA_QM_1_PQ_RD_RATE_LIM_RST_TOKEN: u32 = 0x4080A4;
pub const mmDMA_QM_1_PQ_RD_RATE_LIM_SAT: u32 = 0x4080A8;
pub const mmDMA_QM_1_PQ_RD_RATE_LIM_TOUT: u32 = 0x4080AC;
pub const mmDMA_QM_1_CQ_CFG0: u32 = 0x4080B0;
pub const mmDMA_QM_1_CQ_CFG1: u32 = 0x4080B4;
pub const mmDMA_QM_1_CQ_ARUSER: u32 = 0x4080B8;
pub const mmDMA_QM_1_CQ_PTR_LO: u32 = 0x4080C0;
pub const mmDMA_QM_1_CQ_PTR_HI: u32 = 0x4080C4;
pub const mmDMA_QM_1_CQ_TSIZE: u32 = 0x4080C8;
pub const mmDMA_QM_1_CQ_CTL: u32 = 0x4080CC;
pub const mmDMA_QM_1_CQ_PTR_LO_STS: u32 = 0x4080D4;
pub const mmDMA_QM_1_CQ_PTR_HI_STS: u32 = 0x4080D8;
pub const mmDMA_QM_1_CQ_TSIZE_STS: u32 = 0x4080DC;
pub const mmDMA_QM_1_CQ_CTL_STS: u32 = 0x4080E0;
pub const mmDMA_QM_1_CQ_STS0: u32 = 0x4080E4;
pub const mmDMA_QM_1_CQ_STS1: u32 = 0x4080E8;
pub const mmDMA_QM_1_CQ_RD_RATE_LIM_EN: u32 = 0x4080F0;
pub const mmDMA_QM_1_CQ_RD_RATE_LIM_RST_TOKEN: u32 = 0x4080F4;
pub const mmDMA_QM_1_CQ_RD_RATE_LIM_SAT: u32 = 0x4080F8;
pub const mmDMA_QM_1_CQ_RD_RATE_LIM_TOUT: u32 = 0x4080FC;
pub const mmDMA_QM_1_CQ_IFIFO_CNT: u32 = 0x408108;
pub const mmDMA_QM_1_CP_MSG_BASE0_ADDR_LO: u32 = 0x408120;
pub const mmDMA_QM_1_CP_MSG_BASE0_ADDR_HI: u32 = 0x408124;
pub const mmDMA_QM_1_CP_MSG_BASE1_ADDR_LO: u32 = 0x408128;
pub const mmDMA_QM_1_CP_MSG_BASE1_ADDR_HI: u32 = 0x40812C;
pub const mmDMA_QM_1_CP_MSG_BASE2_ADDR_LO: u32 = 0x408130;
pub const mmDMA_QM_1_CP_MSG_BASE2_ADDR_HI: u32 = 0x408134;
pub const mmDMA_QM_1_CP_MSG_BASE3_ADDR_LO: u32 = 0x408138;
pub const mmDMA_QM_1_CP_MSG_BASE3_ADDR_HI: u32 = 0x40813C;
pub const mmDMA_QM_1_CP_LDMA_TSIZE_OFFSET: u32 = 0x408140;
pub const mmDMA_QM_1_CP_LDMA_SRC_BASE_LO_OFFSET: u32 = 0x408144;
pub const mmDMA_QM_1_CP_LDMA_SRC_BASE_HI_OFFSET: u32 = 0x408148;
pub const mmDMA_QM_1_CP_LDMA_DST_BASE_LO_OFFSET: u32 = 0x40814C;
pub const mmDMA_QM_1_CP_LDMA_DST_BASE_HI_OFFSET: u32 = 0x408150;
pub const mmDMA_QM_1_CP_LDMA_COMMIT_OFFSET: u32 = 0x408154;
pub const mmDMA_QM_1_CP_FENCE0_RDATA: u32 = 0x408158;
pub const mmDMA_QM_1_CP_FENCE1_RDATA: u32 = 0x40815C;
pub const mmDMA_QM_1_CP_FENCE2_RDATA: u32 = 0x408160;
pub const mmDMA_QM_1_CP_FENCE3_RDATA: u32 = 0x408164;
pub const mmDMA_QM_1_CP_FENCE0_CNT: u32 = 0x408168;
pub const mmDMA_QM_1_CP_FENCE1_CNT: u32 = 0x40816C;
pub const mmDMA_QM_1_CP_FENCE2_CNT: u32 = 0x408170;
pub const mmDMA_QM_1_CP_FENCE3_CNT: u32 = 0x408174;
pub const mmDMA_QM_1_CP_STS: u32 = 0x408178;
pub const mmDMA_QM_1_CP_CURRENT_INST_LO: u32 = 0x40817C;
pub const mmDMA_QM_1_CP_CURRENT_INST_HI: u32 = 0x408180;
pub const mmDMA_QM_1_CP_BARRIER_CFG: u32 = 0x408184;
pub const mmDMA_QM_1_CP_DBG_0: u32 = 0x408188;
pub const mmDMA_QM_1_PQ_BUF_ADDR: u32 = 0x408300;
pub const mmDMA_QM_1_PQ_BUF_RDATA: u32 = 0x408304;
pub const mmDMA_QM_1_CQ_BUF_ADDR: u32 = 0x408308;
pub const mmDMA_QM_1_CQ_BUF_RDATA: u32 = 0x40830C;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
