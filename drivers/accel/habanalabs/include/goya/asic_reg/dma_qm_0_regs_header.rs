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
 *   DMA_QM_0 (Prototype: QMAN)
 *****************************************
 */

pub const mmDMA_QM_0_GLBL_CFG0: u32 = 0x400000;
pub const mmDMA_QM_0_GLBL_CFG1: u32 = 0x400004;
pub const mmDMA_QM_0_GLBL_PROT: u32 = 0x400008;
pub const mmDMA_QM_0_GLBL_ERR_CFG: u32 = 0x40000C;
pub const mmDMA_QM_0_GLBL_ERR_ADDR_LO: u32 = 0x400010;
pub const mmDMA_QM_0_GLBL_ERR_ADDR_HI: u32 = 0x400014;
pub const mmDMA_QM_0_GLBL_ERR_WDATA: u32 = 0x400018;
pub const mmDMA_QM_0_GLBL_SECURE_PROPS: u32 = 0x40001C;
pub const mmDMA_QM_0_GLBL_NON_SECURE_PROPS: u32 = 0x400020;
pub const mmDMA_QM_0_GLBL_STS0: u32 = 0x400024;
pub const mmDMA_QM_0_GLBL_STS1: u32 = 0x400028;
pub const mmDMA_QM_0_PQ_BASE_LO: u32 = 0x400060;
pub const mmDMA_QM_0_PQ_BASE_HI: u32 = 0x400064;
pub const mmDMA_QM_0_PQ_SIZE: u32 = 0x400068;
pub const mmDMA_QM_0_PQ_PI: u32 = 0x40006C;
pub const mmDMA_QM_0_PQ_CI: u32 = 0x400070;
pub const mmDMA_QM_0_PQ_CFG0: u32 = 0x400074;
pub const mmDMA_QM_0_PQ_CFG1: u32 = 0x400078;
pub const mmDMA_QM_0_PQ_ARUSER: u32 = 0x40007C;
pub const mmDMA_QM_0_PQ_PUSH0: u32 = 0x400080;
pub const mmDMA_QM_0_PQ_PUSH1: u32 = 0x400084;
pub const mmDMA_QM_0_PQ_PUSH2: u32 = 0x400088;
pub const mmDMA_QM_0_PQ_PUSH3: u32 = 0x40008C;
pub const mmDMA_QM_0_PQ_STS0: u32 = 0x400090;
pub const mmDMA_QM_0_PQ_STS1: u32 = 0x400094;
pub const mmDMA_QM_0_PQ_RD_RATE_LIM_EN: u32 = 0x4000A0;
pub const mmDMA_QM_0_PQ_RD_RATE_LIM_RST_TOKEN: u32 = 0x4000A4;
pub const mmDMA_QM_0_PQ_RD_RATE_LIM_SAT: u32 = 0x4000A8;
pub const mmDMA_QM_0_PQ_RD_RATE_LIM_TOUT: u32 = 0x4000AC;
pub const mmDMA_QM_0_CQ_CFG0: u32 = 0x4000B0;
pub const mmDMA_QM_0_CQ_CFG1: u32 = 0x4000B4;
pub const mmDMA_QM_0_CQ_ARUSER: u32 = 0x4000B8;
pub const mmDMA_QM_0_CQ_PTR_LO: u32 = 0x4000C0;
pub const mmDMA_QM_0_CQ_PTR_HI: u32 = 0x4000C4;
pub const mmDMA_QM_0_CQ_TSIZE: u32 = 0x4000C8;
pub const mmDMA_QM_0_CQ_CTL: u32 = 0x4000CC;
pub const mmDMA_QM_0_CQ_PTR_LO_STS: u32 = 0x4000D4;
pub const mmDMA_QM_0_CQ_PTR_HI_STS: u32 = 0x4000D8;
pub const mmDMA_QM_0_CQ_TSIZE_STS: u32 = 0x4000DC;
pub const mmDMA_QM_0_CQ_CTL_STS: u32 = 0x4000E0;
pub const mmDMA_QM_0_CQ_STS0: u32 = 0x4000E4;
pub const mmDMA_QM_0_CQ_STS1: u32 = 0x4000E8;
pub const mmDMA_QM_0_CQ_RD_RATE_LIM_EN: u32 = 0x4000F0;
pub const mmDMA_QM_0_CQ_RD_RATE_LIM_RST_TOKEN: u32 = 0x4000F4;
pub const mmDMA_QM_0_CQ_RD_RATE_LIM_SAT: u32 = 0x4000F8;
pub const mmDMA_QM_0_CQ_RD_RATE_LIM_TOUT: u32 = 0x4000FC;
pub const mmDMA_QM_0_CQ_IFIFO_CNT: u32 = 0x400108;
pub const mmDMA_QM_0_CP_MSG_BASE0_ADDR_LO: u32 = 0x400120;
pub const mmDMA_QM_0_CP_MSG_BASE0_ADDR_HI: u32 = 0x400124;
pub const mmDMA_QM_0_CP_MSG_BASE1_ADDR_LO: u32 = 0x400128;
pub const mmDMA_QM_0_CP_MSG_BASE1_ADDR_HI: u32 = 0x40012C;
pub const mmDMA_QM_0_CP_MSG_BASE2_ADDR_LO: u32 = 0x400130;
pub const mmDMA_QM_0_CP_MSG_BASE2_ADDR_HI: u32 = 0x400134;
pub const mmDMA_QM_0_CP_MSG_BASE3_ADDR_LO: u32 = 0x400138;
pub const mmDMA_QM_0_CP_MSG_BASE3_ADDR_HI: u32 = 0x40013C;
pub const mmDMA_QM_0_CP_LDMA_TSIZE_OFFSET: u32 = 0x400140;
pub const mmDMA_QM_0_CP_LDMA_SRC_BASE_LO_OFFSET: u32 = 0x400144;
pub const mmDMA_QM_0_CP_LDMA_SRC_BASE_HI_OFFSET: u32 = 0x400148;
pub const mmDMA_QM_0_CP_LDMA_DST_BASE_LO_OFFSET: u32 = 0x40014C;
pub const mmDMA_QM_0_CP_LDMA_DST_BASE_HI_OFFSET: u32 = 0x400150;
pub const mmDMA_QM_0_CP_LDMA_COMMIT_OFFSET: u32 = 0x400154;
pub const mmDMA_QM_0_CP_FENCE0_RDATA: u32 = 0x400158;
pub const mmDMA_QM_0_CP_FENCE1_RDATA: u32 = 0x40015C;
pub const mmDMA_QM_0_CP_FENCE2_RDATA: u32 = 0x400160;
pub const mmDMA_QM_0_CP_FENCE3_RDATA: u32 = 0x400164;
pub const mmDMA_QM_0_CP_FENCE0_CNT: u32 = 0x400168;
pub const mmDMA_QM_0_CP_FENCE1_CNT: u32 = 0x40016C;
pub const mmDMA_QM_0_CP_FENCE2_CNT: u32 = 0x400170;
pub const mmDMA_QM_0_CP_FENCE3_CNT: u32 = 0x400174;
pub const mmDMA_QM_0_CP_STS: u32 = 0x400178;
pub const mmDMA_QM_0_CP_CURRENT_INST_LO: u32 = 0x40017C;
pub const mmDMA_QM_0_CP_CURRENT_INST_HI: u32 = 0x400180;
pub const mmDMA_QM_0_CP_BARRIER_CFG: u32 = 0x400184;
pub const mmDMA_QM_0_CP_DBG_0: u32 = 0x400188;
pub const mmDMA_QM_0_PQ_BUF_ADDR: u32 = 0x400300;
pub const mmDMA_QM_0_PQ_BUF_RDATA: u32 = 0x400304;
pub const mmDMA_QM_0_CQ_BUF_ADDR: u32 = 0x400308;
pub const mmDMA_QM_0_CQ_BUF_RDATA: u32 = 0x40030C;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
