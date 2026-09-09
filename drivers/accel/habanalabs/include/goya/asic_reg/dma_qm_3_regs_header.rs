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
 *   DMA_QM_3 (Prototype: QMAN)
 *****************************************
 */

pub const mmDMA_QM_3_GLBL_CFG0: u32 = 0x418000;
pub const mmDMA_QM_3_GLBL_CFG1: u32 = 0x418004;
pub const mmDMA_QM_3_GLBL_PROT: u32 = 0x418008;
pub const mmDMA_QM_3_GLBL_ERR_CFG: u32 = 0x41800C;
pub const mmDMA_QM_3_GLBL_ERR_ADDR_LO: u32 = 0x418010;
pub const mmDMA_QM_3_GLBL_ERR_ADDR_HI: u32 = 0x418014;
pub const mmDMA_QM_3_GLBL_ERR_WDATA: u32 = 0x418018;
pub const mmDMA_QM_3_GLBL_SECURE_PROPS: u32 = 0x41801C;
pub const mmDMA_QM_3_GLBL_NON_SECURE_PROPS: u32 = 0x418020;
pub const mmDMA_QM_3_GLBL_STS0: u32 = 0x418024;
pub const mmDMA_QM_3_GLBL_STS1: u32 = 0x418028;
pub const mmDMA_QM_3_PQ_BASE_LO: u32 = 0x418060;
pub const mmDMA_QM_3_PQ_BASE_HI: u32 = 0x418064;
pub const mmDMA_QM_3_PQ_SIZE: u32 = 0x418068;
pub const mmDMA_QM_3_PQ_PI: u32 = 0x41806C;
pub const mmDMA_QM_3_PQ_CI: u32 = 0x418070;
pub const mmDMA_QM_3_PQ_CFG0: u32 = 0x418074;
pub const mmDMA_QM_3_PQ_CFG1: u32 = 0x418078;
pub const mmDMA_QM_3_PQ_ARUSER: u32 = 0x41807C;
pub const mmDMA_QM_3_PQ_PUSH0: u32 = 0x418080;
pub const mmDMA_QM_3_PQ_PUSH1: u32 = 0x418084;
pub const mmDMA_QM_3_PQ_PUSH2: u32 = 0x418088;
pub const mmDMA_QM_3_PQ_PUSH3: u32 = 0x41808C;
pub const mmDMA_QM_3_PQ_STS0: u32 = 0x418090;
pub const mmDMA_QM_3_PQ_STS1: u32 = 0x418094;
pub const mmDMA_QM_3_PQ_RD_RATE_LIM_EN: u32 = 0x4180A0;
pub const mmDMA_QM_3_PQ_RD_RATE_LIM_RST_TOKEN: u32 = 0x4180A4;
pub const mmDMA_QM_3_PQ_RD_RATE_LIM_SAT: u32 = 0x4180A8;
pub const mmDMA_QM_3_PQ_RD_RATE_LIM_TOUT: u32 = 0x4180AC;
pub const mmDMA_QM_3_CQ_CFG0: u32 = 0x4180B0;
pub const mmDMA_QM_3_CQ_CFG1: u32 = 0x4180B4;
pub const mmDMA_QM_3_CQ_ARUSER: u32 = 0x4180B8;
pub const mmDMA_QM_3_CQ_PTR_LO: u32 = 0x4180C0;
pub const mmDMA_QM_3_CQ_PTR_HI: u32 = 0x4180C4;
pub const mmDMA_QM_3_CQ_TSIZE: u32 = 0x4180C8;
pub const mmDMA_QM_3_CQ_CTL: u32 = 0x4180CC;
pub const mmDMA_QM_3_CQ_PTR_LO_STS: u32 = 0x4180D4;
pub const mmDMA_QM_3_CQ_PTR_HI_STS: u32 = 0x4180D8;
pub const mmDMA_QM_3_CQ_TSIZE_STS: u32 = 0x4180DC;
pub const mmDMA_QM_3_CQ_CTL_STS: u32 = 0x4180E0;
pub const mmDMA_QM_3_CQ_STS0: u32 = 0x4180E4;
pub const mmDMA_QM_3_CQ_STS1: u32 = 0x4180E8;
pub const mmDMA_QM_3_CQ_RD_RATE_LIM_EN: u32 = 0x4180F0;
pub const mmDMA_QM_3_CQ_RD_RATE_LIM_RST_TOKEN: u32 = 0x4180F4;
pub const mmDMA_QM_3_CQ_RD_RATE_LIM_SAT: u32 = 0x4180F8;
pub const mmDMA_QM_3_CQ_RD_RATE_LIM_TOUT: u32 = 0x4180FC;
pub const mmDMA_QM_3_CQ_IFIFO_CNT: u32 = 0x418108;
pub const mmDMA_QM_3_CP_MSG_BASE0_ADDR_LO: u32 = 0x418120;
pub const mmDMA_QM_3_CP_MSG_BASE0_ADDR_HI: u32 = 0x418124;
pub const mmDMA_QM_3_CP_MSG_BASE1_ADDR_LO: u32 = 0x418128;
pub const mmDMA_QM_3_CP_MSG_BASE1_ADDR_HI: u32 = 0x41812C;
pub const mmDMA_QM_3_CP_MSG_BASE2_ADDR_LO: u32 = 0x418130;
pub const mmDMA_QM_3_CP_MSG_BASE2_ADDR_HI: u32 = 0x418134;
pub const mmDMA_QM_3_CP_MSG_BASE3_ADDR_LO: u32 = 0x418138;
pub const mmDMA_QM_3_CP_MSG_BASE3_ADDR_HI: u32 = 0x41813C;
pub const mmDMA_QM_3_CP_LDMA_TSIZE_OFFSET: u32 = 0x418140;
pub const mmDMA_QM_3_CP_LDMA_SRC_BASE_LO_OFFSET: u32 = 0x418144;
pub const mmDMA_QM_3_CP_LDMA_SRC_BASE_HI_OFFSET: u32 = 0x418148;
pub const mmDMA_QM_3_CP_LDMA_DST_BASE_LO_OFFSET: u32 = 0x41814C;
pub const mmDMA_QM_3_CP_LDMA_DST_BASE_HI_OFFSET: u32 = 0x418150;
pub const mmDMA_QM_3_CP_LDMA_COMMIT_OFFSET: u32 = 0x418154;
pub const mmDMA_QM_3_CP_FENCE0_RDATA: u32 = 0x418158;
pub const mmDMA_QM_3_CP_FENCE1_RDATA: u32 = 0x41815C;
pub const mmDMA_QM_3_CP_FENCE2_RDATA: u32 = 0x418160;
pub const mmDMA_QM_3_CP_FENCE3_RDATA: u32 = 0x418164;
pub const mmDMA_QM_3_CP_FENCE0_CNT: u32 = 0x418168;
pub const mmDMA_QM_3_CP_FENCE1_CNT: u32 = 0x41816C;
pub const mmDMA_QM_3_CP_FENCE2_CNT: u32 = 0x418170;
pub const mmDMA_QM_3_CP_FENCE3_CNT: u32 = 0x418174;
pub const mmDMA_QM_3_CP_STS: u32 = 0x418178;
pub const mmDMA_QM_3_CP_CURRENT_INST_LO: u32 = 0x41817C;
pub const mmDMA_QM_3_CP_CURRENT_INST_HI: u32 = 0x418180;
pub const mmDMA_QM_3_CP_BARRIER_CFG: u32 = 0x418184;
pub const mmDMA_QM_3_CP_DBG_0: u32 = 0x418188;
pub const mmDMA_QM_3_PQ_BUF_ADDR: u32 = 0x418300;
pub const mmDMA_QM_3_PQ_BUF_RDATA: u32 = 0x418304;
pub const mmDMA_QM_3_CQ_BUF_ADDR: u32 = 0x418308;
pub const mmDMA_QM_3_CQ_BUF_RDATA: u32 = 0x41830C;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
