/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2024, Qualcomm Innovation Center, Inc. All rights reserved.
 */

// Translated from qcom,qcs8300-rpmh.h.

pub const MASTER_QUP_3: u32 = 0;
pub const MASTER_EMAC: u32 = 1;
pub const MASTER_SDC: u32 = 2;
pub const MASTER_UFS_MEM: u32 = 3;
pub const MASTER_USB2: u32 = 4;
pub const MASTER_USB3_0: u32 = 5;
pub const SLAVE_A1NOC_SNOC: u32 = 6;

pub const MASTER_QDSS_BAM: u32 = 0;
pub const MASTER_QUP_0: u32 = 1;
pub const MASTER_QUP_1: u32 = 2;
pub const MASTER_CNOC_A2NOC: u32 = 3;
pub const MASTER_CRYPTO_CORE0: u32 = 4;
pub const MASTER_CRYPTO_CORE1: u32 = 5;
pub const MASTER_IPA: u32 = 6;
pub const MASTER_QDSS_ETR_0: u32 = 7;
pub const MASTER_QDSS_ETR_1: u32 = 8;
pub const SLAVE_A2NOC_SNOC: u32 = 9;

pub const MASTER_QUP_CORE_0: u32 = 0;
pub const MASTER_QUP_CORE_1: u32 = 1;
pub const MASTER_QUP_CORE_3: u32 = 2;
pub const SLAVE_QUP_CORE_0: u32 = 3;
pub const SLAVE_QUP_CORE_1: u32 = 4;
pub const SLAVE_QUP_CORE_3: u32 = 5;

pub const MASTER_GEM_NOC_CNOC: u32 = 0;
pub const MASTER_GEM_NOC_PCIE_SNOC: u32 = 1;
pub const SLAVE_AHB2PHY_2: u32 = 2;
pub const SLAVE_AHB2PHY_3: u32 = 3;
pub const SLAVE_ANOC_THROTTLE_CFG: u32 = 4;
pub const SLAVE_AOSS: u32 = 5;
pub const SLAVE_APPSS: u32 = 6;
pub const SLAVE_BOOT_ROM: u32 = 7;
pub const SLAVE_CAMERA_CFG: u32 = 8;
pub const SLAVE_CAMERA_NRT_THROTTLE_CFG: u32 = 9;
pub const SLAVE_CAMERA_RT_THROTTLE_CFG: u32 = 10;
pub const SLAVE_CLK_CTL: u32 = 11;
pub const SLAVE_CDSP_CFG: u32 = 12;
pub const SLAVE_RBCPR_CX_CFG: u32 = 13;
pub const SLAVE_RBCPR_MMCX_CFG: u32 = 14;
pub const SLAVE_RBCPR_MX_CFG: u32 = 15;
pub const SLAVE_CPR_NSPCX: u32 = 16;
pub const SLAVE_CPR_NSPHMX: u32 = 17;
pub const SLAVE_CRYPTO_0_CFG: u32 = 18;
pub const SLAVE_CX_RDPM: u32 = 19;
pub const SLAVE_DISPLAY_CFG: u32 = 20;
pub const SLAVE_DISPLAY_RT_THROTTLE_CFG: u32 = 21;
pub const SLAVE_EMAC_CFG: u32 = 22;
pub const SLAVE_GP_DSP0_CFG: u32 = 23;
pub const SLAVE_GPDSP0_THROTTLE_CFG: u32 = 24;
pub const SLAVE_GPU_TCU_THROTTLE_CFG: u32 = 25;
pub const SLAVE_GFX3D_CFG: u32 = 26;
pub const SLAVE_HWKM: u32 = 27;
pub const SLAVE_IMEM_CFG: u32 = 28;
pub const SLAVE_IPA_CFG: u32 = 29;
pub const SLAVE_IPC_ROUTER_CFG: u32 = 30;
pub const SLAVE_LPASS: u32 = 31;
pub const SLAVE_LPASS_THROTTLE_CFG: u32 = 32;
pub const SLAVE_MX_RDPM: u32 = 33;
pub const SLAVE_MXC_RDPM: u32 = 34;
pub const SLAVE_PCIE_0_CFG: u32 = 35;
pub const SLAVE_PCIE_1_CFG: u32 = 36;
pub const SLAVE_PCIE_TCU_THROTTLE_CFG: u32 = 37;
pub const SLAVE_PCIE_THROTTLE_CFG: u32 = 38;
pub const SLAVE_PDM: u32 = 39;
pub const SLAVE_PIMEM_CFG: u32 = 40;
pub const SLAVE_PKA_WRAPPER_CFG: u32 = 41;
pub const SLAVE_QDSS_CFG: u32 = 42;
pub const SLAVE_QM_CFG: u32 = 43;
pub const SLAVE_QM_MPU_CFG: u32 = 44;
pub const SLAVE_QUP_0: u32 = 45;
pub const SLAVE_QUP_1: u32 = 46;
pub const SLAVE_QUP_3: u32 = 47;
pub const SLAVE_SAIL_THROTTLE_CFG: u32 = 48;
pub const SLAVE_SDC1: u32 = 49;
pub const SLAVE_SECURITY: u32 = 50;
pub const SLAVE_SNOC_THROTTLE_CFG: u32 = 51;
pub const SLAVE_TCSR: u32 = 52;
pub const SLAVE_TLMM: u32 = 53;
pub const SLAVE_TSC_CFG: u32 = 54;
pub const SLAVE_UFS_MEM_CFG: u32 = 55;
pub const SLAVE_USB2: u32 = 56;
pub const SLAVE_USB3_0: u32 = 57;
pub const SLAVE_VENUS_CFG: u32 = 58;
pub const SLAVE_VENUS_CVP_THROTTLE_CFG: u32 = 59;
pub const SLAVE_VENUS_V_CPU_THROTTLE_CFG: u32 = 60;
pub const SLAVE_VENUS_VCODEC_THROTTLE_CFG: u32 = 61;
pub const SLAVE_DDRSS_CFG: u32 = 62;
pub const SLAVE_GPDSP_NOC_CFG: u32 = 63;
pub const SLAVE_CNOC_MNOC_HF_CFG: u32 = 64;
pub const SLAVE_CNOC_MNOC_SF_CFG: u32 = 65;
pub const SLAVE_PCIE_ANOC_CFG: u32 = 66;
pub const SLAVE_SNOC_CFG: u32 = 67;
pub const SLAVE_BOOT_IMEM: u32 = 68;
pub const SLAVE_IMEM: u32 = 69;
pub const SLAVE_PIMEM: u32 = 70;
pub const SLAVE_PCIE_0: u32 = 71;
pub const SLAVE_PCIE_1: u32 = 72;
pub const SLAVE_QDSS_STM: u32 = 73;
pub const SLAVE_TCU: u32 = 74;

pub const MASTER_CNOC_DC_NOC: u32 = 0;
pub const SLAVE_LLCC_CFG: u32 = 1;
pub const SLAVE_GEM_NOC_CFG: u32 = 2;

pub const MASTER_GPU_TCU: u32 = 0;
pub const MASTER_PCIE_TCU: u32 = 1;
pub const MASTER_SYS_TCU: u32 = 2;
pub const MASTER_APPSS_PROC: u32 = 3;
pub const MASTER_COMPUTE_NOC: u32 = 4;
pub const MASTER_GEM_NOC_CFG: u32 = 5;
pub const MASTER_GPDSP_SAIL: u32 = 6;
pub const MASTER_GFX3D: u32 = 7;
pub const MASTER_MNOC_HF_MEM_NOC: u32 = 8;
pub const MASTER_MNOC_SF_MEM_NOC: u32 = 9;
pub const MASTER_ANOC_PCIE_GEM_NOC: u32 = 10;
pub const MASTER_SNOC_GC_MEM_NOC: u32 = 11;
pub const MASTER_SNOC_SF_MEM_NOC: u32 = 12;
pub const SLAVE_GEM_NOC_CNOC: u32 = 13;
pub const SLAVE_LLCC: u32 = 14;
pub const SLAVE_GEM_NOC_PCIE_CNOC: u32 = 15;
pub const SLAVE_SERVICE_GEM_NOC_1: u32 = 16;
pub const SLAVE_SERVICE_GEM_NOC_2: u32 = 17;
pub const SLAVE_SERVICE_GEM_NOC: u32 = 18;
pub const SLAVE_SERVICE_GEM_NOC2: u32 = 19;

pub const MASTER_SAILSS_MD0: u32 = 0;
pub const MASTER_DSP0: u32 = 1;
pub const SLAVE_GP_DSP_SAIL_NOC: u32 = 2;

pub const MASTER_CNOC_LPASS_AG_NOC: u32 = 0;
pub const MASTER_LPASS_PROC: u32 = 1;
pub const SLAVE_LPASS_CORE_CFG: u32 = 2;
pub const SLAVE_LPASS_LPI_CFG: u32 = 3;
pub const SLAVE_LPASS_MPU_CFG: u32 = 4;
pub const SLAVE_LPASS_TOP_CFG: u32 = 5;
pub const SLAVE_LPASS_SNOC: u32 = 6;
pub const SLAVE_SERVICES_LPASS_AML_NOC: u32 = 7;
pub const SLAVE_SERVICE_LPASS_AG_NOC: u32 = 8;

pub const MASTER_LLCC: u32 = 0;
pub const SLAVE_EBI1: u32 = 1;

pub const MASTER_CAMNOC_HF: u32 = 0;
pub const MASTER_CAMNOC_ICP: u32 = 1;
pub const MASTER_CAMNOC_SF: u32 = 2;
pub const MASTER_MDP0: u32 = 3;
pub const MASTER_MDP1: u32 = 4;
pub const MASTER_CNOC_MNOC_HF_CFG: u32 = 5;
pub const MASTER_CNOC_MNOC_SF_CFG: u32 = 6;
pub const MASTER_VIDEO_P0: u32 = 7;
pub const MASTER_VIDEO_PROC: u32 = 8;
pub const MASTER_VIDEO_V_PROC: u32 = 9;
pub const SLAVE_MNOC_HF_MEM_NOC: u32 = 10;
pub const SLAVE_MNOC_SF_MEM_NOC: u32 = 11;
pub const SLAVE_SERVICE_MNOC_HF: u32 = 12;
pub const SLAVE_SERVICE_MNOC_SF: u32 = 13;

pub const MASTER_CDSP_NOC_CFG: u32 = 0;
pub const MASTER_CDSP_PROC: u32 = 1;
pub const SLAVE_HCP_A: u32 = 2;
pub const SLAVE_CDSP_MEM_NOC: u32 = 3;
pub const SLAVE_SERVICE_NSP_NOC: u32 = 4;

pub const MASTER_PCIE_0: u32 = 0;
pub const MASTER_PCIE_1: u32 = 1;
pub const SLAVE_ANOC_PCIE_GEM_NOC: u32 = 2;

pub const MASTER_GIC_AHB: u32 = 0;
pub const MASTER_A1NOC_SNOC: u32 = 1;
pub const MASTER_A2NOC_SNOC: u32 = 2;
pub const MASTER_LPASS_ANOC: u32 = 3;
pub const MASTER_SNOC_CFG: u32 = 4;
pub const MASTER_PIMEM: u32 = 5;
pub const MASTER_GIC: u32 = 6;
pub const SLAVE_SNOC_GEM_NOC_GC: u32 = 7;
pub const SLAVE_SNOC_GEM_NOC_SF: u32 = 8;
pub const SLAVE_SERVICE_SNOC: u32 = 9;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
