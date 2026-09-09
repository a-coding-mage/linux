/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Qualcomm SDM845 interconnect IDs
 *
 * Copyright (c) 2018, Linaro Ltd.
 * Author: Georgi Djakov <georgi.djakov@linaro.org>
 */

// Translated from the C header; include guards and preprocessor syntax omitted.

pub const MASTER_A1NOC_CFG: u32 = 0;
pub const MASTER_TSIF: u32 = 1;
pub const MASTER_SDCC_2: u32 = 2;
pub const MASTER_SDCC_4: u32 = 3;
pub const MASTER_UFS_CARD: u32 = 4;
pub const MASTER_UFS_MEM: u32 = 5;
pub const MASTER_PCIE_0: u32 = 6;
pub const SLAVE_A1NOC_SNOC: u32 = 7;
pub const SLAVE_SERVICE_A1NOC: u32 = 8;
pub const SLAVE_ANOC_PCIE_A1NOC_SNOC: u32 = 9;
pub const MASTER_QUP_1: u32 = 10;

pub const MASTER_A2NOC_CFG: u32 = 0;
pub const MASTER_QDSS_BAM: u32 = 1;
pub const MASTER_CNOC_A2NOC: u32 = 2;
pub const MASTER_CRYPTO: u32 = 3;
pub const MASTER_IPA: u32 = 4;
pub const MASTER_PCIE_1: u32 = 5;
pub const MASTER_QDSS_ETR: u32 = 6;
pub const MASTER_USB3_0: u32 = 7;
pub const MASTER_USB3_1: u32 = 8;
pub const SLAVE_A2NOC_SNOC: u32 = 9;
pub const SLAVE_ANOC_PCIE_SNOC: u32 = 10;
pub const SLAVE_SERVICE_A2NOC: u32 = 11;
pub const MASTER_QUP_2: u32 = 12;

pub const MASTER_SPDM: u32 = 0;
pub const MASTER_TIC: u32 = 1;
pub const MASTER_SNOC_CNOC: u32 = 2;
pub const MASTER_QDSS_DAP: u32 = 3;
pub const SLAVE_A1NOC_CFG: u32 = 4;
pub const SLAVE_A2NOC_CFG: u32 = 5;
pub const SLAVE_AOP: u32 = 6;
pub const SLAVE_AOSS: u32 = 7;
pub const SLAVE_CAMERA_CFG: u32 = 8;
pub const SLAVE_CLK_CTL: u32 = 9;
pub const SLAVE_CDSP_CFG: u32 = 10;
pub const SLAVE_RBCPR_CX_CFG: u32 = 11;
pub const SLAVE_CRYPTO_0_CFG: u32 = 12;
pub const SLAVE_DCC_CFG: u32 = 13;
pub const SLAVE_CNOC_DDRSS: u32 = 14;
pub const SLAVE_DISPLAY_CFG: u32 = 15;
pub const SLAVE_GLM: u32 = 16;
pub const SLAVE_GFX3D_CFG: u32 = 17;
pub const SLAVE_IMEM_CFG: u32 = 18;
pub const SLAVE_IPA_CFG: u32 = 19;
pub const SLAVE_CNOC_MNOC_CFG: u32 = 20;
pub const SLAVE_PCIE_0_CFG: u32 = 21;
pub const SLAVE_PCIE_1_CFG: u32 = 22;
pub const SLAVE_PDM: u32 = 23;
pub const SLAVE_SOUTH_PHY_CFG: u32 = 24;
pub const SLAVE_PIMEM_CFG: u32 = 25;
pub const SLAVE_PRNG: u32 = 26;
pub const SLAVE_QDSS_CFG: u32 = 27;
pub const SLAVE_BLSP_2: u32 = 28;
pub const SLAVE_BLSP_1: u32 = 29;
pub const SLAVE_SDCC_2: u32 = 30;
pub const SLAVE_SDCC_4: u32 = 31;
pub const SLAVE_SNOC_CFG: u32 = 32;
pub const SLAVE_SPDM_WRAPPER: u32 = 33;
pub const SLAVE_SPSS_CFG: u32 = 34;
pub const SLAVE_TCSR: u32 = 35;
pub const SLAVE_TLMM_NORTH: u32 = 36;
pub const SLAVE_TLMM_SOUTH: u32 = 37;
pub const SLAVE_TSIF: u32 = 38;
pub const SLAVE_UFS_CARD_CFG: u32 = 39;
pub const SLAVE_UFS_MEM_CFG: u32 = 40;
pub const SLAVE_USB3_0: u32 = 41;
pub const SLAVE_USB3_1: u32 = 42;
pub const SLAVE_VENUS_CFG: u32 = 43;
pub const SLAVE_VSENSE_CTRL_CFG: u32 = 44;
pub const SLAVE_CNOC_A2NOC: u32 = 45;
pub const SLAVE_SERVICE_CNOC: u32 = 46;

pub const MASTER_CNOC_DC_NOC: u32 = 0;
pub const SLAVE_LLCC_CFG: u32 = 1;
pub const SLAVE_MEM_NOC_CFG: u32 = 2;

pub const MASTER_APPSS_PROC: u32 = 0;
pub const MASTER_GNOC_CFG: u32 = 1;
pub const SLAVE_GNOC_SNOC: u32 = 2;
pub const SLAVE_GNOC_MEM_NOC: u32 = 3;
pub const SLAVE_SERVICE_GNOC: u32 = 4;

pub const MASTER_TCU_0: u32 = 0;
pub const MASTER_MEM_NOC_CFG: u32 = 1;
pub const MASTER_GNOC_MEM_NOC: u32 = 2;
pub const MASTER_MNOC_HF_MEM_NOC: u32 = 3;
pub const MASTER_MNOC_SF_MEM_NOC: u32 = 4;
pub const MASTER_SNOC_GC_MEM_NOC: u32 = 5;
pub const MASTER_SNOC_SF_MEM_NOC: u32 = 6;
pub const MASTER_GFX3D: u32 = 7;
pub const SLAVE_MSS_PROC_MS_MPU_CFG: u32 = 8;
pub const SLAVE_MEM_NOC_GNOC: u32 = 9;
pub const SLAVE_LLCC: u32 = 10;
pub const SLAVE_MEM_NOC_SNOC: u32 = 11;
pub const SLAVE_SERVICE_MEM_NOC: u32 = 12;
pub const MASTER_LLCC: u32 = 13;
pub const SLAVE_EBI1: u32 = 14;

pub const MASTER_CNOC_MNOC_CFG: u32 = 0;
pub const MASTER_CAMNOC_HF0: u32 = 1;
pub const MASTER_CAMNOC_HF1: u32 = 2;
pub const MASTER_CAMNOC_SF: u32 = 3;
pub const MASTER_MDP0: u32 = 4;
pub const MASTER_MDP1: u32 = 5;
pub const MASTER_ROTATOR: u32 = 6;
pub const MASTER_VIDEO_P0: u32 = 7;
pub const MASTER_VIDEO_P1: u32 = 8;
pub const MASTER_VIDEO_PROC: u32 = 9;
pub const SLAVE_MNOC_SF_MEM_NOC: u32 = 10;
pub const SLAVE_MNOC_HF_MEM_NOC: u32 = 11;
pub const SLAVE_SERVICE_MNOC: u32 = 12;
pub const MASTER_CAMNOC_HF0_UNCOMP: u32 = 13;
pub const MASTER_CAMNOC_HF1_UNCOMP: u32 = 14;
pub const MASTER_CAMNOC_SF_UNCOMP: u32 = 15;
pub const SLAVE_CAMNOC_UNCOMP: u32 = 16;

pub const MASTER_SNOC_CFG: u32 = 0;
pub const MASTER_A1NOC_SNOC: u32 = 1;
pub const MASTER_A2NOC_SNOC: u32 = 2;
pub const MASTER_GNOC_SNOC: u32 = 3;
pub const MASTER_MEM_NOC_SNOC: u32 = 4;
pub const MASTER_ANOC_PCIE_SNOC: u32 = 5;
pub const MASTER_PIMEM: u32 = 6;
pub const MASTER_GIC: u32 = 7;
pub const SLAVE_APPSS: u32 = 8;
pub const SLAVE_SNOC_CNOC: u32 = 9;
pub const SLAVE_SNOC_MEM_NOC_GC: u32 = 10;
pub const SLAVE_SNOC_MEM_NOC_SF: u32 = 11;
pub const SLAVE_IMEM: u32 = 12;
pub const SLAVE_PCIE_0: u32 = 13;
pub const SLAVE_PCIE_1: u32 = 14;
pub const SLAVE_PIMEM: u32 = 15;
pub const SLAVE_SERVICE_SNOC: u32 = 16;
pub const SLAVE_QDSS_STM: u32 = 17;
pub const SLAVE_TCU: u32 = 18;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
