/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

pub const MASTER_QSPI_0: u32 = 0;
pub const MASTER_QUP_2: u32 = 1;
pub const MASTER_QUP_3: u32 = 2;
pub const MASTER_QUP_4: u32 = 3;
pub const MASTER_CRYPTO: u32 = 4;
pub const MASTER_IPA: u32 = 5;
pub const MASTER_QUP_1: u32 = 6;
pub const MASTER_SOCCP_PROC: u32 = 7;
pub const MASTER_QDSS_ETR: u32 = 8;
pub const MASTER_QDSS_ETR_1: u32 = 9;
pub const MASTER_SDCC_2: u32 = 10;
pub const MASTER_SDCC_4: u32 = 11;
pub const MASTER_UFS_MEM: u32 = 12;
pub const MASTER_USB3: u32 = 13;
pub const SLAVE_A1NOC_SNOC: u32 = 14;

pub const MASTER_DDR_EFF_VETO: u32 = 0;
pub const MASTER_QUP_CORE_0: u32 = 1;
pub const MASTER_QUP_CORE_1: u32 = 2;
pub const MASTER_QUP_CORE_2: u32 = 3;
pub const MASTER_QUP_CORE_3: u32 = 4;
pub const MASTER_QUP_CORE_4: u32 = 5;
pub const SLAVE_DDR_EFF_VETO: u32 = 6;
pub const SLAVE_QUP_CORE_0: u32 = 7;
pub const SLAVE_QUP_CORE_1: u32 = 8;
pub const SLAVE_QUP_CORE_2: u32 = 9;
pub const SLAVE_QUP_CORE_3: u32 = 10;
pub const SLAVE_QUP_CORE_4: u32 = 11;

pub const MASTER_GEM_NOC_CNOC: u32 = 0;
pub const MASTER_GEM_NOC_PCIE_SNOC: u32 = 1;
pub const SLAVE_AOSS: u32 = 2;
pub const SLAVE_IPA_CFG: u32 = 3;
pub const SLAVE_IPC_ROUTER_FENCE: u32 = 4;
pub const SLAVE_SOCCP: u32 = 5;
pub const SLAVE_TME_CFG: u32 = 6;
pub const SLAVE_CNOC_CFG: u32 = 7;
pub const SLAVE_DDRSS_CFG: u32 = 8;
pub const SLAVE_IMEM: u32 = 9;
pub const SLAVE_PCIE_0: u32 = 10;
pub const SLAVE_PCIE_1: u32 = 11;

pub const MASTER_GIC: u32 = 0;
pub const MASTER_GPU_TCU: u32 = 1;
pub const MASTER_SYS_TCU: u32 = 2;
pub const MASTER_APPSS_PROC: u32 = 3;
pub const MASTER_GFX3D: u32 = 4;
pub const MASTER_LPASS_GEM_NOC: u32 = 5;
pub const MASTER_MSS_PROC: u32 = 6;
pub const MASTER_MNOC_HF_MEM_NOC: u32 = 7;
pub const MASTER_MNOC_SF_MEM_NOC: u32 = 8;
pub const MASTER_COMPUTE_NOC: u32 = 9;
pub const MASTER_ANOC_PCIE_GEM_NOC: u32 = 10;
pub const MASTER_QPACE: u32 = 11;
pub const MASTER_SNOC_SF_MEM_NOC: u32 = 12;
pub const MASTER_WLAN_Q6: u32 = 13;
pub const SLAVE_GEM_NOC_CNOC: u32 = 14;
pub const SLAVE_LLCC: u32 = 15;
pub const SLAVE_MEM_NOC_PCIE_SNOC: u32 = 16;

pub const MASTER_LPIAON_NOC_LLCLPI_NOC: u32 = 0;
pub const SLAVE_LPASS_LPI_CC: u32 = 1;
pub const SLAVE_LLCC_ISLAND: u32 = 2;
pub const SLAVE_SERVICE_LLCLPI_NOC: u32 = 3;
pub const SLAVE_SERVICE_LLCLPI_NOC_CHIPCX: u32 = 4;

pub const MASTER_LPIAON_NOC: u32 = 0;
pub const SLAVE_LPASS_GEM_NOC: u32 = 1;

pub const MASTER_LPASS_LPINOC: u32 = 0;
pub const SLAVE_LPIAON_NOC_LLCLPI_NOC: u32 = 1;
pub const SLAVE_LPIAON_NOC_LPASS_AG_NOC: u32 = 2;

pub const MASTER_LPASS_PROC: u32 = 0;
pub const SLAVE_LPICX_NOC_LPIAON_NOC: u32 = 1;

pub const MASTER_LLCC: u32 = 0;
pub const MASTER_DDR_RT: u32 = 1;
pub const SLAVE_EBI1: u32 = 2;
pub const SLAVE_DDR_RT: u32 = 3;

pub const MASTER_CAMNOC_HF: u32 = 0;
pub const MASTER_CAMNOC_NRT_ICP_SF: u32 = 1;
pub const MASTER_CAMNOC_RT_CDM_SF: u32 = 2;
pub const MASTER_CAMNOC_SF: u32 = 3;
pub const MASTER_MDP: u32 = 4;
pub const MASTER_MDSS_DCP: u32 = 5;
pub const MASTER_CDSP_HCP: u32 = 6;
pub const MASTER_VIDEO_CV_PROC: u32 = 7;
pub const MASTER_VIDEO_EVA: u32 = 8;
pub const MASTER_VIDEO_MVP: u32 = 9;
pub const MASTER_VIDEO_V_PROC: u32 = 10;
pub const SLAVE_MNOC_HF_MEM_NOC: u32 = 11;
pub const SLAVE_MNOC_SF_MEM_NOC: u32 = 12;

pub const MASTER_CDSP_PROC: u32 = 0;
pub const SLAVE_CDSP_MEM_NOC: u32 = 1;

pub const MASTER_PCIE_ANOC_CFG: u32 = 0;
pub const MASTER_PCIE_0: u32 = 1;
pub const MASTER_PCIE_1: u32 = 2;
pub const SLAVE_ANOC_PCIE_GEM_NOC: u32 = 3;
pub const SLAVE_SERVICE_PCIE_ANOC: u32 = 4;

pub const MASTER_CFG_CENTER: u32 = 0;
pub const MASTER_CFG_EAST: u32 = 1;
pub const MASTER_CFG_MM: u32 = 2;
pub const MASTER_CFG_NORTH: u32 = 3;
pub const MASTER_CFG_SOUTH: u32 = 4;
pub const MASTER_CFG_SOUTHWEST: u32 = 5;
pub const SLAVE_AHB2PHY_SOUTH: u32 = 6;
pub const SLAVE_BOOT_ROM: u32 = 7;
pub const SLAVE_CAMERA_CFG: u32 = 8;
pub const SLAVE_CLK_CTL: u32 = 9;
pub const SLAVE_CRYPTO_CFG: u32 = 10;
pub const SLAVE_DISPLAY_CFG: u32 = 11;
pub const SLAVE_EVA_CFG: u32 = 12;
pub const SLAVE_GFX3D_CFG: u32 = 13;
pub const SLAVE_I2C: u32 = 14;
pub const SLAVE_IMEM_CFG: u32 = 15;
pub const SLAVE_IPC_ROUTER_CFG: u32 = 16;
pub const SLAVE_IRIS_CFG: u32 = 17;
pub const SLAVE_CNOC_MSS: u32 = 18;
pub const SLAVE_PCIE_0_CFG: u32 = 19;
pub const SLAVE_PCIE_1_CFG: u32 = 20;
pub const SLAVE_PRNG: u32 = 21;
pub const SLAVE_QSPI_0: u32 = 22;
pub const SLAVE_QUP_1: u32 = 23;
pub const SLAVE_QUP_2: u32 = 24;
pub const SLAVE_QUP_3: u32 = 25;
pub const SLAVE_QUP_4: u32 = 26;
pub const SLAVE_SDCC_2: u32 = 27;
pub const SLAVE_SDCC_4: u32 = 28;
pub const SLAVE_TLMM: u32 = 29;
pub const SLAVE_UFS_MEM_CFG: u32 = 30;
pub const SLAVE_USB3: u32 = 31;
pub const SLAVE_VSENSE_CTRL_CFG: u32 = 32;
pub const SLAVE_PCIE_ANOC_CFG: u32 = 33;
pub const SLAVE_QDSS_CFG: u32 = 34;
pub const SLAVE_QDSS_STM: u32 = 35;
pub const SLAVE_TCSR: u32 = 36;
pub const SLAVE_TCU: u32 = 37;

pub const MASTER_CNOC_STARDUST: u32 = 0;
pub const SLAVE_STARDUST_CENTER_CFG: u32 = 1;
pub const SLAVE_STARDUST_EAST_CFG: u32 = 2;
pub const SLAVE_STARDUST_MM_CFG: u32 = 3;
pub const SLAVE_STARDUST_NORTH_CFG: u32 = 4;
pub const SLAVE_STARDUST_SOUTH_CFG: u32 = 5;
pub const SLAVE_STARDUST_SOUTHWEST_CFG: u32 = 6;

pub const MASTER_A1NOC_SNOC: u32 = 0;
pub const MASTER_APSS_NOC: u32 = 1;
pub const MASTER_CNOC_SNOC: u32 = 2;
pub const SLAVE_SNOC_GEM_NOC_SF: u32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
