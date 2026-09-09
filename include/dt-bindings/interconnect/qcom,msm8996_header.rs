/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * Qualcomm MSM8996 interconnect IDs
 *
 * Copyright (c) 2021 Yassine Oudjana <y.oudjana@protonmail.com>
 */

/* A0NOC */
pub const MASTER_PCIE_0: i32 = 0;
pub const MASTER_PCIE_1: i32 = 1;
pub const MASTER_PCIE_2: i32 = 2;

/* A1NOC */
pub const MASTER_CNOC_A1NOC: i32 = 0;
pub const MASTER_CRYPTO_CORE0: i32 = 1;
pub const MASTER_PNOC_A1NOC: i32 = 2;

/* A2NOC */
pub const MASTER_USB3: i32 = 0;
pub const MASTER_IPA: i32 = 1;
pub const MASTER_UFS: i32 = 2;

/* BIMC */
pub const MASTER_AMPSS_M0: i32 = 0;
pub const MASTER_GRAPHICS_3D: i32 = 1;
pub const MASTER_MNOC_BIMC: i32 = 2;
pub const MASTER_SNOC_BIMC: i32 = 3;
pub const SLAVE_EBI_CH0: i32 = 4;
pub const SLAVE_HMSS_L3: i32 = 5;
pub const SLAVE_BIMC_SNOC_0: i32 = 6;
pub const SLAVE_BIMC_SNOC_1: i32 = 7;

/* CNOC */
pub const MASTER_SNOC_CNOC: i32 = 0;
pub const MASTER_QDSS_DAP: i32 = 1;
pub const SLAVE_CNOC_A1NOC: i32 = 2;
pub const SLAVE_CLK_CTL: i32 = 3;
pub const SLAVE_TCSR: i32 = 4;
pub const SLAVE_TLMM: i32 = 5;
pub const SLAVE_CRYPTO_0_CFG: i32 = 6;
pub const SLAVE_MPM: i32 = 7;
pub const SLAVE_PIMEM_CFG: i32 = 8;
pub const SLAVE_IMEM_CFG: i32 = 9;
pub const SLAVE_MESSAGE_RAM: i32 = 10;
pub const SLAVE_BIMC_CFG: i32 = 11;
pub const SLAVE_PMIC_ARB: i32 = 12;
pub const SLAVE_PRNG: i32 = 13;
pub const SLAVE_DCC_CFG: i32 = 14;
pub const SLAVE_RBCPR_MX: i32 = 15;
pub const SLAVE_QDSS_CFG: i32 = 16;
pub const SLAVE_RBCPR_CX: i32 = 17;
pub const SLAVE_QDSS_RBCPR_APU: i32 = 18;
pub const SLAVE_CNOC_MNOC_CFG: i32 = 19;
pub const SLAVE_SNOC_CFG: i32 = 20;
pub const SLAVE_SNOC_MPU_CFG: i32 = 21;
pub const SLAVE_EBI1_PHY_CFG: i32 = 22;
pub const SLAVE_A0NOC_CFG: i32 = 23;
pub const SLAVE_PCIE_1_CFG: i32 = 24;
pub const SLAVE_PCIE_2_CFG: i32 = 25;
pub const SLAVE_PCIE_0_CFG: i32 = 26;
pub const SLAVE_PCIE20_AHB2PHY: i32 = 27;
pub const SLAVE_A0NOC_MPU_CFG: i32 = 28;
pub const SLAVE_UFS_CFG: i32 = 29;
pub const SLAVE_A1NOC_CFG: i32 = 30;
pub const SLAVE_A1NOC_MPU_CFG: i32 = 31;
pub const SLAVE_A2NOC_CFG: i32 = 32;
pub const SLAVE_A2NOC_MPU_CFG: i32 = 33;
pub const SLAVE_SSC_CFG: i32 = 34;
pub const SLAVE_A0NOC_SMMU_CFG: i32 = 35;
pub const SLAVE_A1NOC_SMMU_CFG: i32 = 36;
pub const SLAVE_A2NOC_SMMU_CFG: i32 = 37;
pub const SLAVE_LPASS_SMMU_CFG: i32 = 38;
pub const SLAVE_CNOC_MNOC_MMSS_CFG: i32 = 39;

/* MNOC */
pub const MASTER_CNOC_MNOC_CFG: i32 = 0;
pub const MASTER_CPP: i32 = 1;
pub const MASTER_JPEG: i32 = 2;
pub const MASTER_MDP_PORT0: i32 = 3;
pub const MASTER_MDP_PORT1: i32 = 4;
pub const MASTER_ROTATOR: i32 = 5;
pub const MASTER_VIDEO_P0: i32 = 6;
pub const MASTER_VFE: i32 = 7;
pub const MASTER_SNOC_VMEM: i32 = 8;
pub const MASTER_VIDEO_P0_OCMEM: i32 = 9;
pub const MASTER_CNOC_MNOC_MMSS_CFG: i32 = 10;
pub const SLAVE_MNOC_BIMC: i32 = 11;
pub const SLAVE_VMEM: i32 = 12;
pub const SLAVE_SERVICE_MNOC: i32 = 13;
pub const SLAVE_MMAGIC_CFG: i32 = 14;
pub const SLAVE_CPR_CFG: i32 = 15;
pub const SLAVE_MISC_CFG: i32 = 16;
pub const SLAVE_VENUS_THROTTLE_CFG: i32 = 17;
pub const SLAVE_VENUS_CFG: i32 = 18;
pub const SLAVE_VMEM_CFG: i32 = 19;
pub const SLAVE_DSA_CFG: i32 = 20;
pub const SLAVE_MMSS_CLK_CFG: i32 = 21;
pub const SLAVE_DSA_MPU_CFG: i32 = 22;
pub const SLAVE_MNOC_MPU_CFG: i32 = 23;
pub const SLAVE_DISPLAY_CFG: i32 = 24;
pub const SLAVE_DISPLAY_THROTTLE_CFG: i32 = 25;
pub const SLAVE_CAMERA_CFG: i32 = 26;
pub const SLAVE_CAMERA_THROTTLE_CFG: i32 = 27;
pub const SLAVE_GRAPHICS_3D_CFG: i32 = 28;
pub const SLAVE_SMMU_MDP_CFG: i32 = 29;
pub const SLAVE_SMMU_ROT_CFG: i32 = 30;
pub const SLAVE_SMMU_VENUS_CFG: i32 = 31;
pub const SLAVE_SMMU_CPP_CFG: i32 = 32;
pub const SLAVE_SMMU_JPEG_CFG: i32 = 33;
pub const SLAVE_SMMU_VFE_CFG: i32 = 34;

/* PNOC */
pub const MASTER_SNOC_PNOC: i32 = 0;
pub const MASTER_SDCC_1: i32 = 1;
pub const MASTER_SDCC_2: i32 = 2;
pub const MASTER_SDCC_4: i32 = 3;
pub const MASTER_USB_HS: i32 = 4;
pub const MASTER_BLSP_1: i32 = 5;
pub const MASTER_BLSP_2: i32 = 6;
pub const MASTER_TSIF: i32 = 7;
pub const SLAVE_PNOC_A1NOC: i32 = 8;
pub const SLAVE_USB_HS: i32 = 9;
pub const SLAVE_SDCC_2: i32 = 10;
pub const SLAVE_SDCC_4: i32 = 11;
pub const SLAVE_TSIF: i32 = 12;
pub const SLAVE_BLSP_2: i32 = 13;
pub const SLAVE_SDCC_1: i32 = 14;
pub const SLAVE_BLSP_1: i32 = 15;
pub const SLAVE_PDM: i32 = 16;
pub const SLAVE_AHB2PHY: i32 = 17;

/* SNOC */
pub const MASTER_HMSS: i32 = 0;
pub const MASTER_QDSS_BAM: i32 = 1;
pub const MASTER_SNOC_CFG: i32 = 2;
pub const MASTER_BIMC_SNOC_0: i32 = 3;
pub const MASTER_BIMC_SNOC_1: i32 = 4;
pub const MASTER_A0NOC_SNOC: i32 = 5;
pub const MASTER_A1NOC_SNOC: i32 = 6;
pub const MASTER_A2NOC_SNOC: i32 = 7;
pub const MASTER_QDSS_ETR: i32 = 8;
pub const SLAVE_A0NOC_SNOC: i32 = 9;
pub const SLAVE_A1NOC_SNOC: i32 = 10;
pub const SLAVE_A2NOC_SNOC: i32 = 11;
pub const SLAVE_HMSS: i32 = 12;
pub const SLAVE_LPASS: i32 = 13;
pub const SLAVE_USB3: i32 = 14;
pub const SLAVE_SNOC_BIMC: i32 = 15;
pub const SLAVE_SNOC_CNOC: i32 = 16;
pub const SLAVE_IMEM: i32 = 17;
pub const SLAVE_PIMEM: i32 = 18;
pub const SLAVE_SNOC_VMEM: i32 = 19;
pub const SLAVE_SNOC_PNOC: i32 = 20;
pub const SLAVE_QDSS_STM: i32 = 21;
pub const SLAVE_PCIE_0: i32 = 22;
pub const SLAVE_PCIE_1: i32 = 23;
pub const SLAVE_PCIE_2: i32 = 24;
pub const SLAVE_SERVICE_SNOC: i32 = 25;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
