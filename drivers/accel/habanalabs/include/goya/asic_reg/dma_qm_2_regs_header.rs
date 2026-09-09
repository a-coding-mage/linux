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
 *   DMA_QM_2 (Prototype: QMAN)
 *****************************************
 */

pub const mmDMA_QM_2_GLBL_CFG0: u32 = 0x410000;
pub const mmDMA_QM_2_GLBL_CFG1: u32 = 0x410004;
pub const mmDMA_QM_2_GLBL_PROT: u32 = 0x410008;
pub const mmDMA_QM_2_GLBL_ERR_CFG: u32 = 0x41000C;
pub const mmDMA_QM_2_GLBL_ERR_ADDR_LO: u32 = 0x410010;
pub const mmDMA_QM_2_GLBL_ERR_ADDR_HI: u32 = 0x410014;
pub const mmDMA_QM_2_GLBL_ERR_WDATA: u32 = 0x410018;
pub const mmDMA_QM_2_GLBL_SECURE_PROPS: u32 = 0x41001C;
pub const mmDMA_QM_2_GLBL_NON_SECURE_PROPS: u32 = 0x410020;
pub const mmDMA_QM_2_GLBL_STS0: u32 = 0x410024;
pub const mmDMA_QM_2_GLBL_STS1: u32 = 0x410028;
pub const mmDMA_QM_2_PQ_BASE_LO: u32 = 0x410060;
pub const mmDMA_QM_2_PQ_BASE_HI: u32 = 0x410064;
pub const mmDMA_QM_2_PQ_SIZE: u32 = 0x410068;
pub const mmDMA_QM_2_PQ_PI: u32 = 0x41006C;
pub const mmDMA_QM_2_PQ_CI: u32 = 0x410070;
pub const mmDMA_QM_2_PQ_CFG0: u32 = 0x410074;
pub const mmDMA_QM_2_PQ_CFG1: u32 = 0x410078;
pub const mmDMA_QM_2_PQ_ARUSER: u32 = 0x41007C;
pub const mmDMA_QM_2_PQ_PUSH0: u32 = 0x410080;
pub const mmDMA_QM_2_PQ_PUSH1: u32 = 0x410084;
pub const mmDMA_QM_2_PQ_PUSH2: u32 = 0x410088;
pub const mmDMA_QM_2_PQ_PUSH3: u32 = 0x41008C;
pub const mmDMA_QM_2_PQ_STS0: u32 = 0x410090;
pub const mmDMA_QM_2_PQ_STS1: u32 = 0x410094;
pub const mmDMA_QM_2_PQ_RD_RATE_LIM_EN: u32 = 0x4100A0;
pub const mmDMA_QM_2_PQ_RD_RATE_LIM_RST_TOKEN: u32 = 0x4100A4;
pub const mmDMA_QM_2_PQ_RD_RATE_LIM_SAT: u32 = 0x4100A8;
pub const mmDMA_QM_2_PQ_RD_RATE_LIM_TOUT: u32 = 0x4100AC;
pub const mmDMA_QM_2_CQ_CFG0: u32 = 0x4100B0;
pub const mmDMA_QM_2_CQ_CFG1: u32 = 0x4100B4;
pub const mmDMA_QM_2_CQ_ARUSER: u32 = 0x4100B8;
pub const mmDMA_QM_2_CQ_PTR_LO: u32 = 0x4100C0;
pub const mmDMA_QM_2_CQ_PTR_HI: u32 = 0x4100C4;
pub const mmDMA_QM_2_CQ_TSIZE: u32 = 0x4100C8;
pub const mmDMA_QM_2_CQ_CTL: u32 = 0x4100CC;
pub const mmDMA_QM_2_CQ_PTR_LO_STS: u32 = 0x4100D4;
pub const mmDMA_QM_2_CQ_PTR_HI_STS: u32 = 0x4100D8;
pub const mmDMA_QM_2_CQ_TSIZE_STS: u32 = 0x4100DC;
pub const mmDMA_QM_2_CQ_CTL_STS: u32 = 0x4100E0;
pub const mmDMA_QM_2_CQ_STS0: u32 = 0x4100E4;
pub const mmDMA_QM_2_CQ_STS1: u32 = 0x4100E8;
pub const mmDMA_QM_2_CQ_RD_RATE_LIM_EN: u32 = 0x4100F0;
pub const mmDMA_QM_2_CQ_RD_RATE_LIM_RST_TOKEN: u32 = 0x4100F4;
pub const mmDMA_QM_2_CQ_RD_RATE_LIM_SAT: u32 = 0x4100F8;
pub const mmDMA_QM_2_CQ_RD_RATE_LIM_TOUT: u32 = 0x4100FC;
pub const mmDMA_QM_2_CQ_IFIFO_CNT: u32 = 0x410108;
pub const mmDMA_QM_2_CP_MSG_BASE0_ADDR_LO: u32 = 0x410120;
pub const mmDMA_QM_2_CP_MSG_BASE0_ADDR_HI: u32 = 0x410124;
pub const mmDMA_QM_2_CP_MSG_BASE1_ADDR_LO: u32 = 0x410128;
pub const mmDMA_QM_2_CP_MSG_BASE1_ADDR_HI: u32 = 0x41012C;
pub const mmDMA_QM_2_CP_MSG_BASE2_ADDR_LO: u32 = 0x410130;
pub const mmDMA_QM_2_CP_MSG_BASE2_ADDR_HI: u32 = 0x410134;
pub const mmDMA_QM_2_CP_MSG_BASE3_ADDR_LO: u32 = 0x410138;
pub const mmDMA_QM_2_CP_MSG_BASE3_ADDR_HI: u32 = 0x41013C;
pub const mmDMA_QM_2_CP_LDMA_TSIZE_OFFSET: u32 = 0x410140;
pub const mmDMA_QM_2_CP_LDMA_SRC_BASE_LO_OFFSET: u32 = 0x410144;
pub const mmDMA_QM_2_CP_LDMA_SRC_BASE_HI_OFFSET: u32 = 0x410148;
pub const mmDMA_QM_2_CP_LDMA_DST_BASE_LO_OFFSET: u32 = 0x41014C;
pub const mmDMA_QM_2_CP_LDMA_DST_BASE_HI_OFFSET: u32 = 0x410150;
pub const mmDMA_QM_2_CP_LDMA_COMMIT_OFFSET: u32 = 0x410154;
pub const mmDMA_QM_2_CP_FENCE0_RDATA: u32 = 0x410158;
pub const mmDMA_QM_2_CP_FENCE1_RDATA: u32 = 0x41015C;
pub const mmDMA_QM_2_CP_FENCE2_RDATA: u32 = 0x410160;
pub const mmDMA_QM_2_CP_FENCE3_RDATA: u32 = 0x410164;
pub const mmDMA_QM_2_CP_FENCE0_CNT: u32 = 0x410168;
pub const mmDMA_QM_2_CP_FENCE1_CNT: u32 = 0x41016C;
pub const mmDMA_QM_2_CP_FENCE2_CNT: u32 = 0x410170;
pub const mmDMA_QM_2_CP_FENCE3_CNT: u32 = 0x410174;
pub const mmDMA_QM_2_CP_STS: u32 = 0x410178;
pub const mmDMA_QM_2_CP_CURRENT_INST_LO: u32 = 0x41017C;
pub const mmDMA_QM_2_CP_CURRENT_INST_HI: u32 = 0x410180;
pub const mmDMA_QM_2_CP_BARRIER_CFG: u32 = 0x410184;
pub const mmDMA_QM_2_CP_DBG_0: u32 = 0x410188;
pub const mmDMA_QM_2_PQ_BUF_ADDR: u32 = 0x410300;
pub const mmDMA_QM_2_PQ_BUF_RDATA: u32 = 0x410304;
pub const mmDMA_QM_2_CQ_BUF_ADDR: u32 = 0x410308;
pub const mmDMA_QM_2_CQ_BUF_RDATA: u32 = 0x41030C;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
