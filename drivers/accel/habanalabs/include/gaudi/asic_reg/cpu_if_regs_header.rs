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
 *   CPU_IF (Prototype: CPU_IF)
 *****************************************
 */

pub const mmCPU_IF_ARUSER_OVR: u32 = 0x442104;
pub const mmCPU_IF_ARUSER_OVR_EN: u32 = 0x442108;
pub const mmCPU_IF_AWUSER_OVR: u32 = 0x44210C;
pub const mmCPU_IF_AWUSER_OVR_EN: u32 = 0x442110;
pub const mmCPU_IF_AXCACHE_OVR: u32 = 0x442114;
pub const mmCPU_IF_LOCK_OVR: u32 = 0x442118;
pub const mmCPU_IF_PROT_OVR: u32 = 0x44211C;
pub const mmCPU_IF_MAX_OUTSTANDING: u32 = 0x442120;
pub const mmCPU_IF_EARLY_BRESP_EN: u32 = 0x442124;
pub const mmCPU_IF_FORCE_RSP_OK: u32 = 0x442128;
pub const mmCPU_IF_CPU_MSB_ADDR: u32 = 0x44212C;
pub const mmCPU_IF_AXI_SPLIT_INTR: u32 = 0x442130;
pub const mmCPU_IF_TOTAL_WR_CNT: u32 = 0x442140;
pub const mmCPU_IF_INFLIGHT_WR_CNT: u32 = 0x442144;
pub const mmCPU_IF_TOTAL_RD_CNT: u32 = 0x442150;
pub const mmCPU_IF_INFLIGHT_RD_CNT: u32 = 0x442154;
pub const mmCPU_IF_PF_PQ_PI: u32 = 0x442200;
pub const mmCPU_IF_PQ_BASE_ADDR_LOW: u32 = 0x442204;
pub const mmCPU_IF_PQ_BASE_ADDR_HIGH: u32 = 0x442208;
pub const mmCPU_IF_PQ_LENGTH: u32 = 0x44220C;
pub const mmCPU_IF_CQ_BASE_ADDR_LOW: u32 = 0x442210;
pub const mmCPU_IF_CQ_BASE_ADDR_HIGH: u32 = 0x442214;
pub const mmCPU_IF_CQ_LENGTH: u32 = 0x442218;
pub const mmCPU_IF_EQ_BASE_ADDR_LOW: u32 = 0x442220;
pub const mmCPU_IF_EQ_BASE_ADDR_HIGH: u32 = 0x442224;
pub const mmCPU_IF_EQ_LENGTH: u32 = 0x442228;
pub const mmCPU_IF_EQ_RD_OFFS: u32 = 0x44222C;
pub const mmCPU_IF_QUEUE_INIT: u32 = 0x442230;
pub const mmCPU_IF_TPC_SERR_INTR_STS: u32 = 0x442300;
pub const mmCPU_IF_TPC_SERR_INTR_CLR: u32 = 0x442304;
pub const mmCPU_IF_TPC_SERR_INTR_MASK: u32 = 0x442308;
pub const mmCPU_IF_TPC_DERR_INTR_STS: u32 = 0x442310;
pub const mmCPU_IF_TPC_DERR_INTR_CLR: u32 = 0x442314;
pub const mmCPU_IF_TPC_DERR_INTR_MASK: u32 = 0x442318;
pub const mmCPU_IF_DMA_SERR_INTR_STS: u32 = 0x442320;
pub const mmCPU_IF_DMA_SERR_INTR_CLR: u32 = 0x442324;
pub const mmCPU_IF_DMA_SERR_INTR_MASK: u32 = 0x442328;
pub const mmCPU_IF_DMA_DERR_INTR_STS: u32 = 0x442330;
pub const mmCPU_IF_DMA_DERR_INTR_CLR: u32 = 0x442334;
pub const mmCPU_IF_DMA_DERR_INTR_MASK: u32 = 0x442338;
pub const mmCPU_IF_SRAM_SERR_INTR_STS: u32 = 0x442340;
pub const mmCPU_IF_SRAM_SERR_INTR_CLR: u32 = 0x442344;
pub const mmCPU_IF_SRAM_SERR_INTR_MASK: u32 = 0x442348;
pub const mmCPU_IF_SRAM_DERR_INTR_STS: u32 = 0x442350;
pub const mmCPU_IF_SRAM_DERR_INTR_CLR: u32 = 0x442354;
pub const mmCPU_IF_SRAM_DERR_INTR_MASK: u32 = 0x442358;
pub const mmCPU_IF_NIC_SERR_INTR_STS: u32 = 0x442360;
pub const mmCPU_IF_NIC_SERR_INTR_CLR: u32 = 0x442364;
pub const mmCPU_IF_NIC_SERR_INTR_MASK: u32 = 0x442368;
pub const mmCPU_IF_NIC_DERR_INTR_STS: u32 = 0x442370;
pub const mmCPU_IF_NIC_DERR_INTR_CLR: u32 = 0x442374;
pub const mmCPU_IF_NIC_DERR_INTR_MASK: u32 = 0x442378;
pub const mmCPU_IF_DMA_IF_SERR_INTR_STS: u32 = 0x442380;
pub const mmCPU_IF_DMA_IF_SERR_INTR_CLR: u32 = 0x442384;
pub const mmCPU_IF_DMA_IF_SERR_INTR_MASK: u32 = 0x442388;
pub const mmCPU_IF_DMA_IF_DERR_INTR_STS: u32 = 0x442390;
pub const mmCPU_IF_DMA_IF_DERR_INTR_CLR: u32 = 0x442394;
pub const mmCPU_IF_DMA_IF_DERR_INTR_MASK: u32 = 0x442398;
pub const mmCPU_IF_HBM_SERR_INTR_STS: u32 = 0x4423A0;
pub const mmCPU_IF_HBM_SERR_INTR_CLR: u32 = 0x4423A4;
pub const mmCPU_IF_HBM_SERR_INTR_MASK: u32 = 0x4423A8;
pub const mmCPU_IF_HBM_DERR_INTR_STS: u32 = 0x4423B0;
pub const mmCPU_IF_HBM_DERR_INTR_CLR: u32 = 0x4423B4;
pub const mmCPU_IF_HBM_DERR_INTR_MASK: u32 = 0x4423B8;
pub const mmCPU_IF_PLL_SEI_INTR_STS: u32 = 0x442400;
pub const mmCPU_IF_PLL_SEI_INTR_CLR: u32 = 0x442404;
pub const mmCPU_IF_PLL_SEI_INTR_MASK: u32 = 0x442408;
pub const mmCPU_IF_NIC_SEI_INTR_STS: u32 = 0x442410;
pub const mmCPU_IF_NIC_SEI_INTR_CLR: u32 = 0x442414;
pub const mmCPU_IF_NIC_SEI_INTR_MASK: u32 = 0x442418;
pub const mmCPU_IF_DMA_SEI_INTR_STS: u32 = 0x442420;
pub const mmCPU_IF_DMA_SEI_INTR_CLR: u32 = 0x442424;
pub const mmCPU_IF_DMA_SEI_INTR_MASK: u32 = 0x442428;
pub const mmCPU_IF_DMA_IF_SEI_INTR_STS: u32 = 0x442430;
pub const mmCPU_IF_DMA_IF_SEI_INTR_CLR: u32 = 0x442434;
pub const mmCPU_IF_DMA_IF_SEI_INTR_MASK: u32 = 0x442438;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
