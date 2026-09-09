/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2024, Qualcomm Innovation Center, Inc. All rights reserved.
 */

pub const MASTER_A1NOC_CFG: i32 = 1;
pub const MASTER_QDSS_BAM: i32 = 2;
pub const MASTER_QSPI: i32 = 3;
pub const MASTER_QUP_0: i32 = 4;
pub const MASTER_BLSP_1: i32 = 5;
pub const MASTER_CNOC_A2NOC: i32 = 6;
pub const MASTER_CRYPTO: i32 = 7;
pub const MASTER_IPA: i32 = 8;
pub const MASTER_EMAC_EVB: i32 = 9;
pub const MASTER_PCIE: i32 = 10;
pub const MASTER_QDSS_ETR: i32 = 11;
pub const MASTER_SDCC_1: i32 = 12;
pub const MASTER_SDCC_2: i32 = 13;
pub const MASTER_UFS_MEM: i32 = 14;
pub const MASTER_USB2: i32 = 15;
pub const MASTER_USB3_0: i32 = 16;
pub const SLAVE_A1NOC_SNOC: i32 = 17;
pub const SLAVE_LPASS_SNOC: i32 = 18;
pub const SLAVE_ANOC_PCIE_SNOC: i32 = 19;
pub const SLAVE_SERVICE_A2NOC: i32 = 20;

pub const MASTER_CAMNOC_HF0_UNCOMP: i32 = 1;
pub const MASTER_CAMNOC_HF1_UNCOMP: i32 = 2;
pub const MASTER_CAMNOC_SF_UNCOMP: i32 = 3;
pub const SLAVE_CAMNOC_UNCOMP: i32 = 4;

pub const MASTER_SPDM: i32 = 1;
pub const MASTER_SNOC_CNOC: i32 = 2;
pub const MASTER_QDSS_DAP: i32 = 3;
pub const SLAVE_A1NOC_CFG: i32 = 4;
pub const SLAVE_AHB2PHY_EAST: i32 = 5;
pub const SLAVE_AHB2PHY_WEST: i32 = 6;
pub const SLAVE_AOP: i32 = 7;
pub const SLAVE_AOSS: i32 = 8;
pub const SLAVE_CAMERA_CFG: i32 = 9;
pub const SLAVE_CLK_CTL: i32 = 10;
pub const SLAVE_RBCPR_CX_CFG: i32 = 11;
pub const SLAVE_RBCPR_MX_CFG: i32 = 12;
pub const SLAVE_CRYPTO_0_CFG: i32 = 13;
pub const SLAVE_CNOC_DDRSS: i32 = 14;
pub const SLAVE_DISPLAY_CFG: i32 = 15;
pub const SLAVE_EMAC_AVB_CFG: i32 = 16;
pub const SLAVE_GLM: i32 = 17;
pub const SLAVE_GFX3D_CFG: i32 = 18;
pub const SLAVE_IMEM_CFG: i32 = 19;
pub const SLAVE_IPA_CFG: i32 = 20;
pub const SLAVE_CNOC_MNOC_CFG: i32 = 21;
pub const SLAVE_PCIE_CFG: i32 = 22;
pub const SLAVE_PIMEM_CFG: i32 = 23;
pub const SLAVE_PRNG: i32 = 24;
pub const SLAVE_QDSS_CFG: i32 = 25;
pub const SLAVE_QSPI: i32 = 26;
pub const SLAVE_QUP_0: i32 = 27;
pub const SLAVE_QUP_1: i32 = 28;
pub const SLAVE_SDCC_1: i32 = 29;
pub const SLAVE_SDCC_2: i32 = 30;
pub const SLAVE_SNOC_CFG: i32 = 31;
pub const SLAVE_SPDM_WRAPPER: i32 = 32;
pub const SLAVE_TCSR: i32 = 33;
pub const SLAVE_TLMM_EAST: i32 = 34;
pub const SLAVE_TLMM_SOUTH: i32 = 35;
pub const SLAVE_TLMM_WEST: i32 = 36;
pub const SLAVE_UFS_MEM_CFG: i32 = 37;
pub const SLAVE_USB2: i32 = 38;
pub const SLAVE_USB3: i32 = 39;
pub const SLAVE_VENUS_CFG: i32 = 40;
pub const SLAVE_VSENSE_CTRL_CFG: i32 = 41;
pub const SLAVE_CNOC_A2NOC: i32 = 42;
pub const SLAVE_SERVICE_CNOC: i32 = 43;

pub const MASTER_CNOC_DC_NOC: i32 = 1;
pub const SLAVE_DC_NOC_GEMNOC: i32 = 2;
pub const SLAVE_LLCC_CFG: i32 = 3;

pub const MASTER_APPSS_PROC: i32 = 1;
pub const MASTER_GPU_TCU: i32 = 2;
pub const MASTER_SYS_TCU: i32 = 3;
pub const MASTER_GEM_NOC_CFG: i32 = 4;
pub const MASTER_GFX3D: i32 = 5;
pub const MASTER_MNOC_HF_MEM_NOC: i32 = 6;
pub const MASTER_MNOC_SF_MEM_NOC: i32 = 7;
pub const MASTER_SNOC_GC_MEM_NOC: i32 = 8;
pub const MASTER_SNOC_SF_MEM_NOC: i32 = 9;
pub const SLAVE_MSS_PROC_MS_MPU_CFG: i32 = 10;
pub const SLAVE_GEM_NOC_SNOC: i32 = 11;
pub const SLAVE_LLCC: i32 = 12;
pub const SLAVE_MEM_NOC_PCIE_SNOC: i32 = 13;
pub const SLAVE_SERVICE_GEM_NOC: i32 = 14;

pub const MASTER_IPA_CORE: i32 = 1;
pub const SLAVE_IPA_CORE: i32 = 2;

pub const MASTER_LLCC: i32 = 1;
pub const SLAVE_EBI1: i32 = 2;

pub const MASTER_CNOC_MNOC_CFG: i32 = 1;
pub const MASTER_CAMNOC_HF0: i32 = 2;
pub const MASTER_CAMNOC_HF1: i32 = 3;
pub const MASTER_CAMNOC_SF: i32 = 4;
pub const MASTER_MDP0: i32 = 5;
pub const MASTER_ROTATOR: i32 = 6;
pub const MASTER_VIDEO_P0: i32 = 7;
pub const MASTER_VIDEO_PROC: i32 = 8;
pub const SLAVE_MNOC_SF_MEM_NOC: i32 = 9;
pub const SLAVE_MNOC_HF_MEM_NOC: i32 = 10;
pub const SLAVE_SERVICE_MNOC: i32 = 11;

pub const MASTER_SNOC_CFG: i32 = 1;
pub const MASTER_A1NOC_SNOC: i32 = 2;
pub const MASTER_GEM_NOC_SNOC: i32 = 3;
pub const MASTER_GEM_NOC_PCIE_SNOC: i32 = 4;
pub const MASTER_LPASS_ANOC: i32 = 5;
pub const MASTER_ANOC_PCIE_SNOC: i32 = 6;
pub const MASTER_PIMEM: i32 = 7;
pub const MASTER_GIC: i32 = 8;
pub const SLAVE_APPSS: i32 = 9;
pub const SLAVE_SNOC_CNOC: i32 = 10;
pub const SLAVE_SNOC_GEM_NOC_SF: i32 = 11;
pub const SLAVE_SNOC_MEM_NOC_GC: i32 = 12;
pub const SLAVE_IMEM: i32 = 13;
pub const SLAVE_PIMEM: i32 = 14;
pub const SLAVE_SERVICE_SNOC: i32 = 15;
pub const SLAVE_PCIE_0: i32 = 16;
pub const SLAVE_QDSS_STM: i32 = 17;
pub const SLAVE_TCU: i32 = 18;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
