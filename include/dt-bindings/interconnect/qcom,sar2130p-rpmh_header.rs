/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2021, The Linux Foundation. All rights reserved.
 * Copyright (c) 2022, Qualcomm Innovation Center, Inc. All rights reserved.
 * Copyright (c) 2024, Linaro Ltd.
 */

pub const MASTER_QUP_CORE_0: i32 = 0;
pub const MASTER_QUP_CORE_1: i32 = 1;
pub const SLAVE_QUP_CORE_0: i32 = 2;
pub const SLAVE_QUP_CORE_1: i32 = 3;

pub const MASTER_GEM_NOC_CNOC: i32 = 0;
pub const MASTER_GEM_NOC_PCIE_SNOC: i32 = 1;
pub const MASTER_QDSS_DAP: i32 = 2;
pub const SLAVE_AHB2PHY_SOUTH: i32 = 3;
pub const SLAVE_AOSS: i32 = 4;
pub const SLAVE_CAMERA_CFG: i32 = 5;
pub const SLAVE_CLK_CTL: i32 = 6;
pub const SLAVE_CDSP_CFG: i32 = 7;
pub const SLAVE_RBCPR_CX_CFG: i32 = 8;
pub const SLAVE_RBCPR_MMCX_CFG: i32 = 9;
pub const SLAVE_RBCPR_MXA_CFG: i32 = 10;
pub const SLAVE_RBCPR_MXC_CFG: i32 = 11;
pub const SLAVE_CPR_NSPCX: i32 = 12;
pub const SLAVE_CRYPTO_0_CFG: i32 = 13;
pub const SLAVE_CX_RDPM: i32 = 14;
pub const SLAVE_DISPLAY_CFG: i32 = 15;
pub const SLAVE_GFX3D_CFG: i32 = 16;
pub const SLAVE_IMEM_CFG: i32 = 17;
pub const SLAVE_IPC_ROUTER_CFG: i32 = 18;
pub const SLAVE_LPASS: i32 = 19;
pub const SLAVE_MX_RDPM: i32 = 20;
pub const SLAVE_PCIE_0_CFG: i32 = 21;
pub const SLAVE_PCIE_1_CFG: i32 = 22;
pub const SLAVE_PDM: i32 = 23;
pub const SLAVE_PIMEM_CFG: i32 = 24;
pub const SLAVE_PRNG: i32 = 25;
pub const SLAVE_QDSS_CFG: i32 = 26;
pub const SLAVE_QSPI_0: i32 = 27;
pub const SLAVE_QUP_0: i32 = 28;
pub const SLAVE_QUP_1: i32 = 29;
pub const SLAVE_SDCC_1: i32 = 30;
pub const SLAVE_TCSR: i32 = 31;
pub const SLAVE_TLMM: i32 = 32;
pub const SLAVE_TME_CFG: i32 = 33;
pub const SLAVE_USB3_0: i32 = 34;
pub const SLAVE_VENUS_CFG: i32 = 35;
pub const SLAVE_VSENSE_CTRL_CFG: i32 = 36;
pub const SLAVE_WLAN_Q6_CFG: i32 = 37;
pub const SLAVE_DDRSS_CFG: i32 = 38;
pub const SLAVE_CNOC_MNOC_CFG: i32 = 39;
pub const SLAVE_SNOC_CFG: i32 = 40;
pub const SLAVE_IMEM: i32 = 41;
pub const SLAVE_PIMEM: i32 = 42;
pub const SLAVE_SERVICE_CNOC: i32 = 43;
pub const SLAVE_PCIE_0: i32 = 44;
pub const SLAVE_PCIE_1: i32 = 45;
pub const SLAVE_QDSS_STM: i32 = 46;
pub const SLAVE_TCU: i32 = 47;

pub const MASTER_GPU_TCU: i32 = 0;
pub const MASTER_SYS_TCU: i32 = 1;
pub const MASTER_APPSS_PROC: i32 = 2;
pub const MASTER_GFX3D: i32 = 3;
pub const MASTER_MNOC_HF_MEM_NOC: i32 = 4;
pub const MASTER_MNOC_SF_MEM_NOC: i32 = 5;
pub const MASTER_COMPUTE_NOC: i32 = 6;
pub const MASTER_ANOC_PCIE_GEM_NOC: i32 = 7;
pub const MASTER_SNOC_GC_MEM_NOC: i32 = 8;
pub const MASTER_SNOC_SF_MEM_NOC: i32 = 9;
pub const MASTER_WLAN_Q6: i32 = 10;
pub const SLAVE_GEM_NOC_CNOC: i32 = 11;
pub const SLAVE_LLCC: i32 = 12;
pub const SLAVE_MEM_NOC_PCIE_SNOC: i32 = 13;

pub const MASTER_CNOC_LPASS_AG_NOC: i32 = 0;
pub const MASTER_LPASS_PROC: i32 = 1;
pub const SLAVE_LPASS_CORE_CFG: i32 = 2;
pub const SLAVE_LPASS_LPI_CFG: i32 = 3;
pub const SLAVE_LPASS_MPU_CFG: i32 = 4;
pub const SLAVE_LPASS_TOP_CFG: i32 = 5;
pub const SLAVE_LPASS_SNOC: i32 = 6;
pub const SLAVE_SERVICES_LPASS_AML_NOC: i32 = 7;
pub const SLAVE_SERVICE_LPASS_AG_NOC: i32 = 8;

pub const MASTER_LLCC: i32 = 0;
pub const SLAVE_EBI1: i32 = 1;

pub const MASTER_CAMNOC_HF: i32 = 0;
pub const MASTER_CAMNOC_ICP: i32 = 1;
pub const MASTER_CAMNOC_SF: i32 = 2;
pub const MASTER_LSR: i32 = 3;
pub const MASTER_MDP: i32 = 4;
pub const MASTER_CNOC_MNOC_CFG: i32 = 5;
pub const MASTER_VIDEO: i32 = 6;
pub const MASTER_VIDEO_CV_PROC: i32 = 7;
pub const MASTER_VIDEO_PROC: i32 = 8;
pub const MASTER_VIDEO_V_PROC: i32 = 9;
pub const SLAVE_MNOC_HF_MEM_NOC: i32 = 10;
pub const SLAVE_MNOC_SF_MEM_NOC: i32 = 11;
pub const SLAVE_SERVICE_MNOC: i32 = 12;

pub const MASTER_CDSP_NOC_CFG: i32 = 0;
pub const MASTER_CDSP_PROC: i32 = 1;
pub const SLAVE_CDSP_MEM_NOC: i32 = 2;
pub const SLAVE_SERVICE_NSP_NOC: i32 = 3;

pub const MASTER_PCIE_0: i32 = 0;
pub const MASTER_PCIE_1: i32 = 1;
pub const SLAVE_ANOC_PCIE_GEM_NOC: i32 = 2;

pub const MASTER_GIC_AHB: i32 = 0;
pub const MASTER_QDSS_BAM: i32 = 1;
pub const MASTER_QSPI_0: i32 = 2;
pub const MASTER_QUP_0: i32 = 3;
pub const MASTER_QUP_1: i32 = 4;
pub const MASTER_A2NOC_SNOC: i32 = 5;
pub const MASTER_CNOC_DATAPATH: i32 = 6;
pub const MASTER_LPASS_ANOC: i32 = 7;
pub const MASTER_SNOC_CFG: i32 = 8;
pub const MASTER_CRYPTO: i32 = 9;
pub const MASTER_PIMEM: i32 = 10;
pub const MASTER_GIC: i32 = 11;
pub const MASTER_QDSS_ETR: i32 = 12;
pub const MASTER_QDSS_ETR_1: i32 = 13;
pub const MASTER_SDCC_1: i32 = 14;
pub const MASTER_USB3_0: i32 = 15;
pub const SLAVE_A2NOC_SNOC: i32 = 16;
pub const SLAVE_SNOC_GEM_NOC_GC: i32 = 17;
pub const SLAVE_SNOC_GEM_NOC_SF: i32 = 18;
pub const SLAVE_SERVICE_SNOC: i32 = 19;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
