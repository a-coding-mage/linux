/* SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause */
/*
 * Copyright (c) 2020, The Linux Foundation. All rights reserved.
 * Copyright (c) 2023, Linaro Limited
 */

/* BIMC */
pub const MASTER_AMPSS_M0: u32 = 0;
pub const MASTER_SNOC_BIMC_RT: u32 = 1;
pub const MASTER_SNOC_BIMC_NRT: u32 = 2;
/* BIMC_SNOC_MAS is redefined by the source header in the SNOC section. */
pub const BIMC_SNOC_MAS: u32 = 4;
pub const MASTER_GRAPHICS_3D: u32 = 4;
pub const MASTER_TCU_0: u32 = 5;
pub const SLAVE_EBI_CH0: u32 = 6;
pub const BIMC_SNOC_SLV: u32 = 7;

/* CNOC */
pub const SNOC_CNOC_MAS: u32 = 0;
pub const MASTER_QDSS_DAP: u32 = 1;
pub const SLAVE_AHB2PHY_USB: u32 = 2;
pub const SLAVE_APSS_THROTTLE_CFG: u32 = 3;
pub const SLAVE_BIMC_CFG: u32 = 4;
pub const SLAVE_BOOT_ROM: u32 = 5;
pub const SLAVE_CAMERA_NRT_THROTTLE_CFG: u32 = 6;
pub const SLAVE_CAMERA_RT_THROTTLE_CFG: u32 = 7;
pub const SLAVE_CAMERA_CFG: u32 = 8;
pub const SLAVE_CLK_CTL: u32 = 9;
pub const SLAVE_RBCPR_CX_CFG: u32 = 10;
pub const SLAVE_RBCPR_MX_CFG: u32 = 11;
pub const SLAVE_CRYPTO_0_CFG: u32 = 12;
pub const SLAVE_DCC_CFG: u32 = 13;
pub const SLAVE_DDR_PHY_CFG: u32 = 14;
pub const SLAVE_DDR_SS_CFG: u32 = 15;
pub const SLAVE_DISPLAY_CFG: u32 = 16;
pub const SLAVE_DISPLAY_THROTTLE_CFG: u32 = 17;
pub const SLAVE_GPU_CFG: u32 = 18;
pub const SLAVE_GPU_THROTTLE_CFG: u32 = 19;
pub const SLAVE_HWKM_CORE: u32 = 20;
pub const SLAVE_IMEM_CFG: u32 = 21;
pub const SLAVE_IPA_CFG: u32 = 22;
pub const SLAVE_LPASS: u32 = 23;
pub const SLAVE_MAPSS: u32 = 24;
pub const SLAVE_MDSP_MPU_CFG: u32 = 25;
pub const SLAVE_MESSAGE_RAM: u32 = 26;
pub const SLAVE_CNOC_MSS: u32 = 27;
pub const SLAVE_PDM: u32 = 28;
pub const SLAVE_PIMEM_CFG: u32 = 29;
pub const SLAVE_PKA_CORE: u32 = 30;
pub const SLAVE_PMIC_ARB: u32 = 31;
pub const SLAVE_QDSS_CFG: u32 = 32;
pub const SLAVE_QM_CFG: u32 = 33;
pub const SLAVE_QM_MPU_CFG: u32 = 34;
pub const SLAVE_QPIC: u32 = 35;
pub const SLAVE_QUP_0: u32 = 36;
pub const SLAVE_RPM: u32 = 37;
pub const SLAVE_SDCC_1: u32 = 38;
pub const SLAVE_SDCC_2: u32 = 39;
pub const SLAVE_SECURITY: u32 = 40;
pub const SLAVE_SNOC_CFG: u32 = 41;
pub const SLAVE_TCSR: u32 = 42;
pub const SLAVE_TLMM: u32 = 43;
pub const SLAVE_USB3: u32 = 44;
pub const SLAVE_VENUS_CFG: u32 = 45;
pub const SLAVE_VENUS_THROTTLE_CFG: u32 = 46;
pub const SLAVE_VSENSE_CTRL_CFG: u32 = 47;
pub const SLAVE_SERVICE_CNOC: u32 = 48;

/* SNOC */
pub const MASTER_CRYPTO_CORE0: u32 = 0;
pub const MASTER_SNOC_CFG: u32 = 1;
pub const MASTER_TIC: u32 = 2;
pub const MASTER_ANOC_SNOC: u32 = 3;
pub const MASTER_PIMEM: u32 = 5;
pub const MASTER_QDSS_BAM: u32 = 6;
pub const MASTER_QPIC: u32 = 7;
pub const MASTER_QUP_0: u32 = 8;
pub const MASTER_IPA: u32 = 9;
pub const MASTER_QDSS_ETR: u32 = 10;
pub const MASTER_SDCC_1: u32 = 11;
pub const MASTER_SDCC_2: u32 = 12;
pub const MASTER_USB3: u32 = 13;
pub const SLAVE_APPSS: u32 = 14;
pub const SNOC_CNOC_SLV: u32 = 15;
pub const SLAVE_OCIMEM: u32 = 16;
pub const SLAVE_PIMEM: u32 = 17;
pub const SNOC_BIMC_SLV: u32 = 18;
pub const SLAVE_SERVICE_SNOC: u32 = 19;
pub const SLAVE_QDSS_STM: u32 = 20;
pub const SLAVE_TCU: u32 = 21;
pub const SLAVE_ANOC_SNOC: u32 = 22;

/* CLK Virtual */
pub const MASTER_QUP_CORE_0: u32 = 0;
pub const SLAVE_QUP_CORE_0: u32 = 1;

/* MMRT Virtual */
pub const MASTER_CAMNOC_HF: u32 = 0;
pub const MASTER_MDP_PORT0: u32 = 1;
pub const SLAVE_SNOC_BIMC_RT: u32 = 2;

/* MMNRT Virtual */
pub const MASTER_CAMNOC_SF: u32 = 0;
pub const MASTER_VIDEO_P0: u32 = 1;
pub const MASTER_VIDEO_PROC: u32 = 2;
pub const SLAVE_SNOC_BIMC_NRT: u32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
