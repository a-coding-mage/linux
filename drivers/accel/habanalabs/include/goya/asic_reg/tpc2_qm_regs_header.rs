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
 *   TPC2_QM (Prototype: QMAN)
 *****************************************
 */

pub const mmTPC2_QM_GLBL_CFG0: u32 = 0xE88000;
pub const mmTPC2_QM_GLBL_CFG1: u32 = 0xE88004;
pub const mmTPC2_QM_GLBL_PROT: u32 = 0xE88008;
pub const mmTPC2_QM_GLBL_ERR_CFG: u32 = 0xE8800C;
pub const mmTPC2_QM_GLBL_ERR_ADDR_LO: u32 = 0xE88010;
pub const mmTPC2_QM_GLBL_ERR_ADDR_HI: u32 = 0xE88014;
pub const mmTPC2_QM_GLBL_ERR_WDATA: u32 = 0xE88018;
pub const mmTPC2_QM_GLBL_SECURE_PROPS: u32 = 0xE8801C;
pub const mmTPC2_QM_GLBL_NON_SECURE_PROPS: u32 = 0xE88020;
pub const mmTPC2_QM_GLBL_STS0: u32 = 0xE88024;
pub const mmTPC2_QM_GLBL_STS1: u32 = 0xE88028;
pub const mmTPC2_QM_PQ_BASE_LO: u32 = 0xE88060;
pub const mmTPC2_QM_PQ_BASE_HI: u32 = 0xE88064;
pub const mmTPC2_QM_PQ_SIZE: u32 = 0xE88068;
pub const mmTPC2_QM_PQ_PI: u32 = 0xE8806C;
pub const mmTPC2_QM_PQ_CI: u32 = 0xE88070;
pub const mmTPC2_QM_PQ_CFG0: u32 = 0xE88074;
pub const mmTPC2_QM_PQ_CFG1: u32 = 0xE88078;
pub const mmTPC2_QM_PQ_ARUSER: u32 = 0xE8807C;
pub const mmTPC2_QM_PQ_PUSH0: u32 = 0xE88080;
pub const mmTPC2_QM_PQ_PUSH1: u32 = 0xE88084;
pub const mmTPC2_QM_PQ_PUSH2: u32 = 0xE88088;
pub const mmTPC2_QM_PQ_PUSH3: u32 = 0xE8808C;
pub const mmTPC2_QM_PQ_STS0: u32 = 0xE88090;
pub const mmTPC2_QM_PQ_STS1: u32 = 0xE88094;
pub const mmTPC2_QM_PQ_RD_RATE_LIM_EN: u32 = 0xE880A0;
pub const mmTPC2_QM_PQ_RD_RATE_LIM_RST_TOKEN: u32 = 0xE880A4;
pub const mmTPC2_QM_PQ_RD_RATE_LIM_SAT: u32 = 0xE880A8;
pub const mmTPC2_QM_PQ_RD_RATE_LIM_TOUT: u32 = 0xE880AC;
pub const mmTPC2_QM_CQ_CFG0: u32 = 0xE880B0;
pub const mmTPC2_QM_CQ_CFG1: u32 = 0xE880B4;
pub const mmTPC2_QM_CQ_ARUSER: u32 = 0xE880B8;
pub const mmTPC2_QM_CQ_PTR_LO: u32 = 0xE880C0;
pub const mmTPC2_QM_CQ_PTR_HI: u32 = 0xE880C4;
pub const mmTPC2_QM_CQ_TSIZE: u32 = 0xE880C8;
pub const mmTPC2_QM_CQ_CTL: u32 = 0xE880CC;
pub const mmTPC2_QM_CQ_PTR_LO_STS: u32 = 0xE880D4;
pub const mmTPC2_QM_CQ_PTR_HI_STS: u32 = 0xE880D8;
pub const mmTPC2_QM_CQ_TSIZE_STS: u32 = 0xE880DC;
pub const mmTPC2_QM_CQ_CTL_STS: u32 = 0xE880E0;
pub const mmTPC2_QM_CQ_STS0: u32 = 0xE880E4;
pub const mmTPC2_QM_CQ_STS1: u32 = 0xE880E8;
pub const mmTPC2_QM_CQ_RD_RATE_LIM_EN: u32 = 0xE880F0;
pub const mmTPC2_QM_CQ_RD_RATE_LIM_RST_TOKEN: u32 = 0xE880F4;
pub const mmTPC2_QM_CQ_RD_RATE_LIM_SAT: u32 = 0xE880F8;
pub const mmTPC2_QM_CQ_RD_RATE_LIM_TOUT: u32 = 0xE880FC;
pub const mmTPC2_QM_CQ_IFIFO_CNT: u32 = 0xE88108;
pub const mmTPC2_QM_CP_MSG_BASE0_ADDR_LO: u32 = 0xE88120;
pub const mmTPC2_QM_CP_MSG_BASE0_ADDR_HI: u32 = 0xE88124;
pub const mmTPC2_QM_CP_MSG_BASE1_ADDR_LO: u32 = 0xE88128;
pub const mmTPC2_QM_CP_MSG_BASE1_ADDR_HI: u32 = 0xE8812C;
pub const mmTPC2_QM_CP_MSG_BASE2_ADDR_LO: u32 = 0xE88130;
pub const mmTPC2_QM_CP_MSG_BASE2_ADDR_HI: u32 = 0xE88134;
pub const mmTPC2_QM_CP_MSG_BASE3_ADDR_LO: u32 = 0xE88138;
pub const mmTPC2_QM_CP_MSG_BASE3_ADDR_HI: u32 = 0xE8813C;
pub const mmTPC2_QM_CP_LDMA_TSIZE_OFFSET: u32 = 0xE88140;
pub const mmTPC2_QM_CP_LDMA_SRC_BASE_LO_OFFSET: u32 = 0xE88144;
pub const mmTPC2_QM_CP_LDMA_SRC_BASE_HI_OFFSET: u32 = 0xE88148;
pub const mmTPC2_QM_CP_LDMA_DST_BASE_LO_OFFSET: u32 = 0xE8814C;
pub const mmTPC2_QM_CP_LDMA_DST_BASE_HI_OFFSET: u32 = 0xE88150;
pub const mmTPC2_QM_CP_LDMA_COMMIT_OFFSET: u32 = 0xE88154;
pub const mmTPC2_QM_CP_FENCE0_RDATA: u32 = 0xE88158;
pub const mmTPC2_QM_CP_FENCE1_RDATA: u32 = 0xE8815C;
pub const mmTPC2_QM_CP_FENCE2_RDATA: u32 = 0xE88160;
pub const mmTPC2_QM_CP_FENCE3_RDATA: u32 = 0xE88164;
pub const mmTPC2_QM_CP_FENCE0_CNT: u32 = 0xE88168;
pub const mmTPC2_QM_CP_FENCE1_CNT: u32 = 0xE8816C;
pub const mmTPC2_QM_CP_FENCE2_CNT: u32 = 0xE88170;
pub const mmTPC2_QM_CP_FENCE3_CNT: u32 = 0xE88174;
pub const mmTPC2_QM_CP_STS: u32 = 0xE88178;
pub const mmTPC2_QM_CP_CURRENT_INST_LO: u32 = 0xE8817C;
pub const mmTPC2_QM_CP_CURRENT_INST_HI: u32 = 0xE88180;
pub const mmTPC2_QM_CP_BARRIER_CFG: u32 = 0xE88184;
pub const mmTPC2_QM_CP_DBG_0: u32 = 0xE88188;
pub const mmTPC2_QM_PQ_BUF_ADDR: u32 = 0xE88300;
pub const mmTPC2_QM_PQ_BUF_RDATA: u32 = 0xE88304;
pub const mmTPC2_QM_CQ_BUF_ADDR: u32 = 0xE88308;
pub const mmTPC2_QM_CQ_BUF_RDATA: u32 = 0xE8830C;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
