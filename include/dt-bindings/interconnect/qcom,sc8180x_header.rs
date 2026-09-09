/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Qualcomm SC8180x interconnect IDs
 *
 * Copyright (c) 2021, The Linux Foundation. All rights reserved.
 */

pub const MASTER_A1NOC_CFG: u32 = 0;
pub const MASTER_UFS_CARD: u32 = 1;
pub const MASTER_UFS_GEN4: u32 = 2;
pub const MASTER_UFS_MEM: u32 = 3;
pub const MASTER_USB3: u32 = 4;
pub const MASTER_USB3_1: u32 = 5;
pub const MASTER_USB3_2: u32 = 6;
pub const A1NOC_SNOC_SLV: u32 = 7;
pub const SLAVE_SERVICE_A1NOC: u32 = 8;

pub const MASTER_A2NOC_CFG: u32 = 0;
pub const MASTER_QDSS_BAM: u32 = 1;
pub const MASTER_QSPI_0: u32 = 2;
pub const MASTER_QSPI_1: u32 = 3;
pub const MASTER_QUP_0: u32 = 4;
pub const MASTER_QUP_1: u32 = 5;
pub const MASTER_QUP_2: u32 = 6;
pub const MASTER_SENSORS_AHB: u32 = 7;
pub const MASTER_CRYPTO_CORE_0: u32 = 8;
pub const MASTER_IPA: u32 = 9;
pub const MASTER_EMAC: u32 = 10;
pub const MASTER_PCIE: u32 = 11;
pub const MASTER_PCIE_1: u32 = 12;
pub const MASTER_PCIE_2: u32 = 13;
pub const MASTER_PCIE_3: u32 = 14;
pub const MASTER_QDSS_ETR: u32 = 15;
pub const MASTER_SDCC_2: u32 = 16;
pub const MASTER_SDCC_4: u32 = 17;
pub const A2NOC_SNOC_SLV: u32 = 18;
pub const SLAVE_ANOC_PCIE_GEM_NOC: u32 = 19;
pub const SLAVE_SERVICE_A2NOC: u32 = 20;

pub const MASTER_CAMNOC_HF0_UNCOMP: u32 = 0;
pub const MASTER_CAMNOC_HF1_UNCOMP: u32 = 1;
pub const MASTER_CAMNOC_SF_UNCOMP: u32 = 2;
pub const SLAVE_CAMNOC_UNCOMP: u32 = 3;

pub const MASTER_NPU: u32 = 0;
pub const SLAVE_CDSP_MEM_NOC: u32 = 1;

pub const SNOC_CNOC_MAS: u32 = 0;
pub const SLAVE_A1NOC_CFG: u32 = 1;
pub const SLAVE_A2NOC_CFG: u32 = 2;
pub const SLAVE_AHB2PHY_CENTER: u32 = 3;
pub const SLAVE_AHB2PHY_EAST: u32 = 4;
pub const SLAVE_AHB2PHY_WEST: u32 = 5;
pub const SLAVE_AHB2PHY_SOUTH: u32 = 6;
pub const SLAVE_AOP: u32 = 7;
pub const SLAVE_AOSS: u32 = 8;
pub const SLAVE_CAMERA_CFG: u32 = 9;
pub const SLAVE_CLK_CTL: u32 = 10;
pub const SLAVE_CDSP_CFG: u32 = 11;
pub const SLAVE_RBCPR_CX_CFG: u32 = 12;
pub const SLAVE_RBCPR_MMCX_CFG: u32 = 13;
pub const SLAVE_RBCPR_MX_CFG: u32 = 14;
pub const SLAVE_CRYPTO_0_CFG: u32 = 15;
pub const SLAVE_CNOC_DDRSS: u32 = 16;
pub const SLAVE_DISPLAY_CFG: u32 = 17;
pub const SLAVE_EMAC_CFG: u32 = 18;
pub const SLAVE_GLM: u32 = 19;
pub const SLAVE_GRAPHICS_3D_CFG: u32 = 20;
pub const SLAVE_IMEM_CFG: u32 = 21;
pub const SLAVE_IPA_CFG: u32 = 22;
pub const SLAVE_CNOC_MNOC_CFG: u32 = 23;
pub const SLAVE_NPU_CFG: u32 = 24;
pub const SLAVE_PCIE_0_CFG: u32 = 25;
pub const SLAVE_PCIE_1_CFG: u32 = 26;
pub const SLAVE_PCIE_2_CFG: u32 = 27;
pub const SLAVE_PCIE_3_CFG: u32 = 28;
pub const SLAVE_PDM: u32 = 29;
pub const SLAVE_PIMEM_CFG: u32 = 30;
pub const SLAVE_PRNG: u32 = 31;
pub const SLAVE_QDSS_CFG: u32 = 32;
pub const SLAVE_QSPI_0: u32 = 33;
pub const SLAVE_QSPI_1: u32 = 34;
pub const SLAVE_QUP_1: u32 = 35;
pub const SLAVE_QUP_2: u32 = 36;
pub const SLAVE_QUP_0: u32 = 37;
pub const SLAVE_SDCC_2: u32 = 38;
pub const SLAVE_SDCC_4: u32 = 39;
pub const SLAVE_SECURITY: u32 = 40;
pub const SLAVE_SNOC_CFG: u32 = 41;
pub const SLAVE_SPSS_CFG: u32 = 42;
pub const SLAVE_TCSR: u32 = 43;
pub const SLAVE_TLMM_EAST: u32 = 44;
pub const SLAVE_TLMM_SOUTH: u32 = 45;
pub const SLAVE_TLMM_WEST: u32 = 46;
pub const SLAVE_TSIF: u32 = 47;
pub const SLAVE_UFS_CARD_CFG: u32 = 48;
pub const SLAVE_UFS_MEM_0_CFG: u32 = 49;
pub const SLAVE_UFS_MEM_1_CFG: u32 = 50;
pub const SLAVE_USB3: u32 = 51;
pub const SLAVE_USB3_1: u32 = 52;
pub const SLAVE_USB3_2: u32 = 53;
pub const SLAVE_VENUS_CFG: u32 = 54;
pub const SLAVE_VSENSE_CTRL_CFG: u32 = 55;
pub const SLAVE_SERVICE_CNOC: u32 = 56;

pub const MASTER_CNOC_DC_NOC: u32 = 0;
pub const SLAVE_GEM_NOC_CFG: u32 = 1;
pub const SLAVE_LLCC_CFG: u32 = 2;

pub const MASTER_AMPSS_M0: u32 = 0;
pub const MASTER_GPU_TCU: u32 = 1;
pub const MASTER_SYS_TCU: u32 = 2;
pub const MASTER_GEM_NOC_CFG: u32 = 3;
pub const MASTER_COMPUTE_NOC: u32 = 4;
pub const MASTER_GRAPHICS_3D: u32 = 5;
pub const MASTER_MNOC_HF_MEM_NOC: u32 = 6;
pub const MASTER_MNOC_SF_MEM_NOC: u32 = 7;
pub const MASTER_GEM_NOC_PCIE_SNOC: u32 = 8;
pub const MASTER_SNOC_GC_MEM_NOC: u32 = 9;
pub const MASTER_SNOC_SF_MEM_NOC: u32 = 10;
pub const MASTER_ECC: u32 = 11;
pub const SLAVE_MSS_PROC_MS_MPU_CFG: u32 = 12;
pub const SLAVE_ECC: u32 = 13;
pub const SLAVE_GEM_NOC_SNOC: u32 = 14;
pub const SLAVE_LLCC: u32 = 15;
pub const SLAVE_SERVICE_GEM_NOC: u32 = 16;
pub const SLAVE_SERVICE_GEM_NOC_1: u32 = 17;

pub const MASTER_LLCC: u32 = 0;
pub const SLAVE_EBI_CH0: u32 = 1;

pub const MASTER_CNOC_MNOC_CFG: u32 = 0;
pub const MASTER_CAMNOC_HF0: u32 = 1;
pub const MASTER_CAMNOC_HF1: u32 = 2;
pub const MASTER_CAMNOC_SF: u32 = 3;
pub const MASTER_MDP_PORT0: u32 = 4;
pub const MASTER_MDP_PORT1: u32 = 5;
pub const MASTER_ROTATOR: u32 = 6;
pub const MASTER_VIDEO_P0: u32 = 7;
pub const MASTER_VIDEO_P1: u32 = 8;
pub const MASTER_VIDEO_PROC: u32 = 9;
pub const SLAVE_MNOC_SF_MEM_NOC: u32 = 10;
pub const SLAVE_MNOC_HF_MEM_NOC: u32 = 11;
pub const SLAVE_SERVICE_MNOC: u32 = 12;

pub const MASTER_SNOC_CFG: u32 = 0;
pub const A1NOC_SNOC_MAS: u32 = 1;
pub const A2NOC_SNOC_MAS: u32 = 2;
pub const MASTER_GEM_NOC_SNOC: u32 = 3;
pub const MASTER_PIMEM: u32 = 4;
pub const MASTER_GIC: u32 = 5;
pub const SLAVE_APPSS: u32 = 6;
pub const SNOC_CNOC_SLV: u32 = 7;
pub const SLAVE_SNOC_GEM_NOC_GC: u32 = 8;
pub const SLAVE_SNOC_GEM_NOC_SF: u32 = 9;
pub const SLAVE_OCIMEM: u32 = 10;
pub const SLAVE_PIMEM: u32 = 11;
pub const SLAVE_SERVICE_SNOC: u32 = 12;
pub const SLAVE_PCIE_0: u32 = 13;
pub const SLAVE_PCIE_1: u32 = 14;
pub const SLAVE_PCIE_2: u32 = 15;
pub const SLAVE_PCIE_3: u32 = 16;
pub const SLAVE_QDSS_STM: u32 = 17;
pub const SLAVE_TCU: u32 = 18;

pub const MASTER_MNOC_HF_MEM_NOC_DISPLAY: u32 = 0;
pub const MASTER_MNOC_SF_MEM_NOC_DISPLAY: u32 = 1;
pub const SLAVE_LLCC_DISPLAY: u32 = 2;

pub const MASTER_LLCC_DISPLAY: u32 = 0;
pub const SLAVE_EBI_CH0_DISPLAY: u32 = 1;

pub const MASTER_MDP_PORT0_DISPLAY: u32 = 0;
pub const MASTER_MDP_PORT1_DISPLAY: u32 = 1;
pub const MASTER_ROTATOR_DISPLAY: u32 = 2;
pub const SLAVE_MNOC_SF_MEM_NOC_DISPLAY: u32 = 3;
pub const SLAVE_MNOC_HF_MEM_NOC_DISPLAY: u32 = 4;

pub const MASTER_QUP_CORE_0: u32 = 0;
pub const MASTER_QUP_CORE_1: u32 = 1;
pub const MASTER_QUP_CORE_2: u32 = 2;
pub const SLAVE_QUP_CORE_0: u32 = 3;
pub const SLAVE_QUP_CORE_1: u32 = 4;
pub const SLAVE_QUP_CORE_2: u32 = 5;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
