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
 *   DMA_QM_4 (Prototype: QMAN)
 *****************************************
 */

pub const mmDMA_QM_4_GLBL_CFG0: u32 = 0x420000;
pub const mmDMA_QM_4_GLBL_CFG1: u32 = 0x420004;
pub const mmDMA_QM_4_GLBL_PROT: u32 = 0x420008;
pub const mmDMA_QM_4_GLBL_ERR_CFG: u32 = 0x42000C;
pub const mmDMA_QM_4_GLBL_ERR_ADDR_LO: u32 = 0x420010;
pub const mmDMA_QM_4_GLBL_ERR_ADDR_HI: u32 = 0x420014;
pub const mmDMA_QM_4_GLBL_ERR_WDATA: u32 = 0x420018;
pub const mmDMA_QM_4_GLBL_SECURE_PROPS: u32 = 0x42001C;
pub const mmDMA_QM_4_GLBL_NON_SECURE_PROPS: u32 = 0x420020;
pub const mmDMA_QM_4_GLBL_STS0: u32 = 0x420024;
pub const mmDMA_QM_4_GLBL_STS1: u32 = 0x420028;
pub const mmDMA_QM_4_PQ_BASE_LO: u32 = 0x420060;
pub const mmDMA_QM_4_PQ_BASE_HI: u32 = 0x420064;
pub const mmDMA_QM_4_PQ_SIZE: u32 = 0x420068;
pub const mmDMA_QM_4_PQ_PI: u32 = 0x42006C;
pub const mmDMA_QM_4_PQ_CI: u32 = 0x420070;
pub const mmDMA_QM_4_PQ_CFG0: u32 = 0x420074;
pub const mmDMA_QM_4_PQ_CFG1: u32 = 0x420078;
pub const mmDMA_QM_4_PQ_ARUSER: u32 = 0x42007C;
pub const mmDMA_QM_4_PQ_PUSH0: u32 = 0x420080;
pub const mmDMA_QM_4_PQ_PUSH1: u32 = 0x420084;
pub const mmDMA_QM_4_PQ_PUSH2: u32 = 0x420088;
pub const mmDMA_QM_4_PQ_PUSH3: u32 = 0x42008C;
pub const mmDMA_QM_4_PQ_STS0: u32 = 0x420090;
pub const mmDMA_QM_4_PQ_STS1: u32 = 0x420094;
pub const mmDMA_QM_4_PQ_RD_RATE_LIM_EN: u32 = 0x4200A0;
pub const mmDMA_QM_4_PQ_RD_RATE_LIM_RST_TOKEN: u32 = 0x4200A4;
pub const mmDMA_QM_4_PQ_RD_RATE_LIM_SAT: u32 = 0x4200A8;
pub const mmDMA_QM_4_PQ_RD_RATE_LIM_TOUT: u32 = 0x4200AC;
pub const mmDMA_QM_4_CQ_CFG0: u32 = 0x4200B0;
pub const mmDMA_QM_4_CQ_CFG1: u32 = 0x4200B4;
pub const mmDMA_QM_4_CQ_ARUSER: u32 = 0x4200B8;
pub const mmDMA_QM_4_CQ_PTR_LO: u32 = 0x4200C0;
pub const mmDMA_QM_4_CQ_PTR_HI: u32 = 0x4200C4;
pub const mmDMA_QM_4_CQ_TSIZE: u32 = 0x4200C8;
pub const mmDMA_QM_4_CQ_CTL: u32 = 0x4200CC;
pub const mmDMA_QM_4_CQ_PTR_LO_STS: u32 = 0x4200D4;
pub const mmDMA_QM_4_CQ_PTR_HI_STS: u32 = 0x4200D8;
pub const mmDMA_QM_4_CQ_TSIZE_STS: u32 = 0x4200DC;
pub const mmDMA_QM_4_CQ_CTL_STS: u32 = 0x4200E0;
pub const mmDMA_QM_4_CQ_STS0: u32 = 0x4200E4;
pub const mmDMA_QM_4_CQ_STS1: u32 = 0x4200E8;
pub const mmDMA_QM_4_CQ_RD_RATE_LIM_EN: u32 = 0x4200F0;
pub const mmDMA_QM_4_CQ_RD_RATE_LIM_RST_TOKEN: u32 = 0x4200F4;
pub const mmDMA_QM_4_CQ_RD_RATE_LIM_SAT: u32 = 0x4200F8;
pub const mmDMA_QM_4_CQ_RD_RATE_LIM_TOUT: u32 = 0x4200FC;
pub const mmDMA_QM_4_CQ_IFIFO_CNT: u32 = 0x420108;
pub const mmDMA_QM_4_CP_MSG_BASE0_ADDR_LO: u32 = 0x420120;
pub const mmDMA_QM_4_CP_MSG_BASE0_ADDR_HI: u32 = 0x420124;
pub const mmDMA_QM_4_CP_MSG_BASE1_ADDR_LO: u32 = 0x420128;
pub const mmDMA_QM_4_CP_MSG_BASE1_ADDR_HI: u32 = 0x42012C;
pub const mmDMA_QM_4_CP_MSG_BASE2_ADDR_LO: u32 = 0x420130;
pub const mmDMA_QM_4_CP_MSG_BASE2_ADDR_HI: u32 = 0x420134;
pub const mmDMA_QM_4_CP_MSG_BASE3_ADDR_LO: u32 = 0x420138;
pub const mmDMA_QM_4_CP_MSG_BASE3_ADDR_HI: u32 = 0x42013C;
pub const mmDMA_QM_4_CP_LDMA_TSIZE_OFFSET: u32 = 0x420140;
pub const mmDMA_QM_4_CP_LDMA_SRC_BASE_LO_OFFSET: u32 = 0x420144;
pub const mmDMA_QM_4_CP_LDMA_SRC_BASE_HI_OFFSET: u32 = 0x420148;
pub const mmDMA_QM_4_CP_LDMA_DST_BASE_LO_OFFSET: u32 = 0x42014C;
pub const mmDMA_QM_4_CP_LDMA_DST_BASE_HI_OFFSET: u32 = 0x420150;
pub const mmDMA_QM_4_CP_LDMA_COMMIT_OFFSET: u32 = 0x420154;
pub const mmDMA_QM_4_CP_FENCE0_RDATA: u32 = 0x420158;
pub const mmDMA_QM_4_CP_FENCE1_RDATA: u32 = 0x42015C;
pub const mmDMA_QM_4_CP_FENCE2_RDATA: u32 = 0x420160;
pub const mmDMA_QM_4_CP_FENCE3_RDATA: u32 = 0x420164;
pub const mmDMA_QM_4_CP_FENCE0_CNT: u32 = 0x420168;
pub const mmDMA_QM_4_CP_FENCE1_CNT: u32 = 0x42016C;
pub const mmDMA_QM_4_CP_FENCE2_CNT: u32 = 0x420170;
pub const mmDMA_QM_4_CP_FENCE3_CNT: u32 = 0x420174;
pub const mmDMA_QM_4_CP_STS: u32 = 0x420178;
pub const mmDMA_QM_4_CP_CURRENT_INST_LO: u32 = 0x42017C;
pub const mmDMA_QM_4_CP_CURRENT_INST_HI: u32 = 0x420180;
pub const mmDMA_QM_4_CP_BARRIER_CFG: u32 = 0x420184;
pub const mmDMA_QM_4_CP_DBG_0: u32 = 0x420188;
pub const mmDMA_QM_4_PQ_BUF_ADDR: u32 = 0x420300;
pub const mmDMA_QM_4_PQ_BUF_RDATA: u32 = 0x420304;
pub const mmDMA_QM_4_CQ_BUF_ADDR: u32 = 0x420308;
pub const mmDMA_QM_4_CQ_BUF_RDATA: u32 = 0x42030C;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
