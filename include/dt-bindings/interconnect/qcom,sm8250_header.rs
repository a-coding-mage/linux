/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Qualcomm SM8250 interconnect IDs
 *
 * Copyright (c) 2020, The Linux Foundation. All rights reserved.
 */

pub const MASTER_A1NOC_CFG: i32 = 0;
pub const MASTER_QSPI_0: i32 = 1;
pub const MASTER_QUP_1: i32 = 2;
pub const MASTER_QUP_2: i32 = 3;
pub const MASTER_TSIF: i32 = 4;
pub const MASTER_PCIE_2: i32 = 5;
pub const MASTER_SDCC_4: i32 = 6;
pub const MASTER_UFS_MEM: i32 = 7;
pub const MASTER_USB3: i32 = 8;
pub const MASTER_USB3_1: i32 = 9;
pub const A1NOC_SNOC_SLV: i32 = 10;
pub const SLAVE_ANOC_PCIE_GEM_NOC_1: i32 = 11;
pub const SLAVE_SERVICE_A1NOC: i32 = 12;

pub const MASTER_A2NOC_CFG: i32 = 0;
pub const MASTER_QDSS_BAM: i32 = 1;
pub const MASTER_QUP_0: i32 = 2;
pub const MASTER_CNOC_A2NOC: i32 = 3;
pub const MASTER_CRYPTO_CORE_0: i32 = 4;
pub const MASTER_IPA: i32 = 5;
pub const MASTER_PCIE: i32 = 6;
pub const MASTER_PCIE_1: i32 = 7;
pub const MASTER_QDSS_ETR: i32 = 8;
pub const MASTER_SDCC_2: i32 = 9;
pub const MASTER_UFS_CARD: i32 = 10;
pub const A2NOC_SNOC_SLV: i32 = 11;
pub const SLAVE_ANOC_PCIE_GEM_NOC: i32 = 12;
pub const SLAVE_SERVICE_A2NOC: i32 = 13;

pub const MASTER_NPU: i32 = 0;
pub const SLAVE_CDSP_MEM_NOC: i32 = 1;

pub const SNOC_CNOC_MAS: i32 = 0;
pub const MASTER_QDSS_DAP: i32 = 1;
pub const SLAVE_A1NOC_CFG: i32 = 2;
pub const SLAVE_A2NOC_CFG: i32 = 3;
pub const SLAVE_AHB2PHY_SOUTH: i32 = 4;
pub const SLAVE_AHB2PHY_NORTH: i32 = 5;
pub const SLAVE_AOSS: i32 = 6;
pub const SLAVE_CAMERA_CFG: i32 = 7;
pub const SLAVE_CLK_CTL: i32 = 8;
pub const SLAVE_CDSP_CFG: i32 = 9;
pub const SLAVE_RBCPR_CX_CFG: i32 = 10;
pub const SLAVE_RBCPR_MMCX_CFG: i32 = 11;
pub const SLAVE_RBCPR_MX_CFG: i32 = 12;
pub const SLAVE_CRYPTO_0_CFG: i32 = 13;
pub const SLAVE_CX_RDPM: i32 = 14;
pub const SLAVE_DCC_CFG: i32 = 15;
pub const SLAVE_CNOC_DDRSS: i32 = 16;
pub const SLAVE_DISPLAY_CFG: i32 = 17;
pub const SLAVE_GRAPHICS_3D_CFG: i32 = 18;
pub const SLAVE_IMEM_CFG: i32 = 19;
pub const SLAVE_IPA_CFG: i32 = 20;
pub const SLAVE_IPC_ROUTER_CFG: i32 = 21;
pub const SLAVE_LPASS: i32 = 22;
pub const SLAVE_CNOC_MNOC_CFG: i32 = 23;
pub const SLAVE_NPU_CFG: i32 = 24;
pub const SLAVE_PCIE_0_CFG: i32 = 25;
pub const SLAVE_PCIE_1_CFG: i32 = 26;
pub const SLAVE_PCIE_2_CFG: i32 = 27;
pub const SLAVE_PDM: i32 = 28;
pub const SLAVE_PIMEM_CFG: i32 = 29;
pub const SLAVE_PRNG: i32 = 30;
pub const SLAVE_QDSS_CFG: i32 = 31;
pub const SLAVE_QSPI_0: i32 = 32;
pub const SLAVE_QUP_0: i32 = 33;
pub const SLAVE_QUP_1: i32 = 34;
pub const SLAVE_QUP_2: i32 = 35;
pub const SLAVE_SDCC_2: i32 = 36;
pub const SLAVE_SDCC_4: i32 = 37;
pub const SLAVE_SNOC_CFG: i32 = 38;
pub const SLAVE_TCSR: i32 = 39;
pub const SLAVE_TLMM_NORTH: i32 = 40;
pub const SLAVE_TLMM_SOUTH: i32 = 41;
pub const SLAVE_TLMM_WEST: i32 = 42;
pub const SLAVE_TSIF: i32 = 43;
pub const SLAVE_UFS_CARD_CFG: i32 = 44;
pub const SLAVE_UFS_MEM_CFG: i32 = 45;
pub const SLAVE_USB3: i32 = 46;
pub const SLAVE_USB3_1: i32 = 47;
pub const SLAVE_VENUS_CFG: i32 = 48;
pub const SLAVE_VSENSE_CTRL_CFG: i32 = 49;
pub const SLAVE_CNOC_A2NOC: i32 = 50;
pub const SLAVE_SERVICE_CNOC: i32 = 51;

pub const MASTER_CNOC_DC_NOC: i32 = 0;
pub const SLAVE_LLCC_CFG: i32 = 1;
pub const SLAVE_GEM_NOC_CFG: i32 = 2;

pub const MASTER_GPU_TCU: i32 = 0;
pub const MASTER_SYS_TCU: i32 = 1;
pub const MASTER_AMPSS_M0: i32 = 2;
pub const MASTER_GEM_NOC_CFG: i32 = 3;
pub const MASTER_COMPUTE_NOC: i32 = 4;
pub const MASTER_GRAPHICS_3D: i32 = 5;
pub const MASTER_MNOC_HF_MEM_NOC: i32 = 6;
pub const MASTER_MNOC_SF_MEM_NOC: i32 = 7;
pub const MASTER_ANOC_PCIE_GEM_NOC: i32 = 8;
pub const MASTER_SNOC_GC_MEM_NOC: i32 = 9;
pub const MASTER_SNOC_SF_MEM_NOC: i32 = 10;
pub const SLAVE_GEM_NOC_SNOC: i32 = 11;
pub const SLAVE_LLCC: i32 = 12;
pub const SLAVE_MEM_NOC_PCIE_SNOC: i32 = 13;
pub const SLAVE_SERVICE_GEM_NOC_1: i32 = 14;
pub const SLAVE_SERVICE_GEM_NOC_2: i32 = 15;
pub const SLAVE_SERVICE_GEM_NOC: i32 = 16;

pub const MASTER_LLCC: i32 = 0;
pub const SLAVE_EBI_CH0: i32 = 1;

pub const MASTER_CNOC_MNOC_CFG: i32 = 0;
pub const MASTER_CAMNOC_HF: i32 = 1;
pub const MASTER_CAMNOC_ICP: i32 = 2;
pub const MASTER_CAMNOC_SF: i32 = 3;
pub const MASTER_VIDEO_P0: i32 = 4;
pub const MASTER_VIDEO_P1: i32 = 5;
pub const MASTER_VIDEO_PROC: i32 = 6;
pub const MASTER_MDP_PORT0: i32 = 7;
pub const MASTER_MDP_PORT1: i32 = 8;
pub const MASTER_ROTATOR: i32 = 9;
pub const SLAVE_MNOC_HF_MEM_NOC: i32 = 10;
pub const SLAVE_MNOC_SF_MEM_NOC: i32 = 11;
pub const SLAVE_SERVICE_MNOC: i32 = 12;

pub const MASTER_NPU_SYS: i32 = 0;
pub const MASTER_NPU_CDP: i32 = 1;
pub const MASTER_NPU_NOC_CFG: i32 = 2;
pub const SLAVE_NPU_CAL_DP0: i32 = 3;
pub const SLAVE_NPU_CAL_DP1: i32 = 4;
pub const SLAVE_NPU_CP: i32 = 5;
pub const SLAVE_NPU_INT_DMA_BWMON_CFG: i32 = 6;
pub const SLAVE_NPU_DPM: i32 = 7;
pub const SLAVE_ISENSE_CFG: i32 = 8;
pub const SLAVE_NPU_LLM_CFG: i32 = 9;
pub const SLAVE_NPU_TCM: i32 = 10;
pub const SLAVE_NPU_COMPUTE_NOC: i32 = 11;
pub const SLAVE_SERVICE_NPU_NOC: i32 = 12;

pub const MASTER_SNOC_CFG: i32 = 0;
pub const A1NOC_SNOC_MAS: i32 = 1;
pub const A2NOC_SNOC_MAS: i32 = 2;
pub const MASTER_GEM_NOC_SNOC: i32 = 3;
pub const MASTER_GEM_NOC_PCIE_SNOC: i32 = 4;
pub const MASTER_PIMEM: i32 = 5;
pub const MASTER_GIC: i32 = 6;
pub const SLAVE_APPSS: i32 = 7;
pub const SNOC_CNOC_SLV: i32 = 8;
pub const SLAVE_SNOC_GEM_NOC_GC: i32 = 9;
pub const SLAVE_SNOC_GEM_NOC_SF: i32 = 10;
pub const SLAVE_OCIMEM: i32 = 11;
pub const SLAVE_PIMEM: i32 = 12;
pub const SLAVE_SERVICE_SNOC: i32 = 13;
pub const SLAVE_PCIE_0: i32 = 14;
pub const SLAVE_PCIE_1: i32 = 15;
pub const SLAVE_PCIE_2: i32 = 16;
pub const SLAVE_QDSS_STM: i32 = 17;
pub const SLAVE_TCU: i32 = 18;

pub const MASTER_QUP_CORE_0: i32 = 0;
pub const MASTER_QUP_CORE_1: i32 = 1;
pub const MASTER_QUP_CORE_2: i32 = 2;
pub const SLAVE_QUP_CORE_0: i32 = 3;
pub const SLAVE_QUP_CORE_1: i32 = 4;
pub const SLAVE_QUP_CORE_2: i32 = 5;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
