/* SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause */
/*
 * Qualcomm SDM670 interconnect IDs
 *
 * Copyright (c) 2022, The Linux Foundation. All rights reserved.
 */

/* Translated from the C header; C preprocessor header guards are omitted. */

pub const MASTER_A1NOC_CFG: i32 = 0;
pub const MASTER_BLSP_1: i32 = 1;
pub const MASTER_TSIF: i32 = 2;
pub const MASTER_EMMC: i32 = 3;
pub const MASTER_SDCC_2: i32 = 4;
pub const MASTER_SDCC_4: i32 = 5;
pub const MASTER_UFS_MEM: i32 = 6;
pub const SLAVE_A1NOC_SNOC: i32 = 7;
pub const SLAVE_SERVICE_A1NOC: i32 = 8;

pub const MASTER_A2NOC_CFG: i32 = 0;
pub const MASTER_QDSS_BAM: i32 = 1;
pub const MASTER_BLSP_2: i32 = 2;
pub const MASTER_CNOC_A2NOC: i32 = 3;
pub const MASTER_CRYPTO_CORE_0: i32 = 4;
pub const MASTER_IPA: i32 = 5;
pub const MASTER_QDSS_ETR: i32 = 6;
pub const MASTER_USB3: i32 = 7;
pub const SLAVE_A2NOC_SNOC: i32 = 8;
pub const SLAVE_SERVICE_A2NOC: i32 = 9;

pub const MASTER_SPDM: i32 = 0;
pub const MASTER_SNOC_CNOC: i32 = 1;
pub const SLAVE_A1NOC_CFG: i32 = 2;
pub const SLAVE_A2NOC_CFG: i32 = 3;
pub const SLAVE_AOP: i32 = 4;
pub const SLAVE_AOSS: i32 = 5;
pub const SLAVE_CAMERA_CFG: i32 = 6;
pub const SLAVE_CLK_CTL: i32 = 7;
pub const SLAVE_CDSP_CFG: i32 = 8;
pub const SLAVE_RBCPR_CX_CFG: i32 = 9;
pub const SLAVE_CRYPTO_0_CFG: i32 = 10;
pub const SLAVE_DCC_CFG: i32 = 11;
pub const SLAVE_CNOC_DDRSS: i32 = 12;
pub const SLAVE_DISPLAY_CFG: i32 = 13;
pub const SLAVE_EMMC_CFG: i32 = 14;
pub const SLAVE_GLM: i32 = 15;
pub const SLAVE_GRAPHICS_3D_CFG: i32 = 16;
pub const SLAVE_IMEM_CFG: i32 = 17;
pub const SLAVE_IPA_CFG: i32 = 18;
pub const SLAVE_CNOC_MNOC_CFG: i32 = 19;
pub const SLAVE_PDM: i32 = 20;
pub const SLAVE_SOUTH_PHY_CFG: i32 = 21;
pub const SLAVE_PIMEM_CFG: i32 = 22;
pub const SLAVE_PRNG: i32 = 23;
pub const SLAVE_QDSS_CFG: i32 = 24;
pub const SLAVE_BLSP_2: i32 = 25;
pub const SLAVE_BLSP_1: i32 = 26;
pub const SLAVE_SDCC_2: i32 = 27;
pub const SLAVE_SDCC_4: i32 = 28;
pub const SLAVE_SNOC_CFG: i32 = 29;
pub const SLAVE_SPDM_WRAPPER: i32 = 30;
pub const SLAVE_TCSR: i32 = 31;
pub const SLAVE_TLMM_NORTH: i32 = 32;
pub const SLAVE_TLMM_SOUTH: i32 = 33;
pub const SLAVE_TSIF: i32 = 34;
pub const SLAVE_UFS_MEM_CFG: i32 = 35;
pub const SLAVE_USB3: i32 = 36;
pub const SLAVE_VENUS_CFG: i32 = 37;
pub const SLAVE_VSENSE_CTRL_CFG: i32 = 38;
pub const SLAVE_CNOC_A2NOC: i32 = 39;
pub const SLAVE_SERVICE_CNOC: i32 = 40;

pub const MASTER_CNOC_DC_NOC: i32 = 0;
pub const SLAVE_LLCC_CFG: i32 = 1;
pub const SLAVE_MEM_NOC_CFG: i32 = 2;

pub const MASTER_AMPSS_M0: i32 = 0;
pub const MASTER_GNOC_CFG: i32 = 1;
pub const SLAVE_GNOC_SNOC: i32 = 2;
pub const SLAVE_GNOC_MEM_NOC: i32 = 3;
pub const SLAVE_SERVICE_GNOC: i32 = 4;

pub const MASTER_TCU_0: i32 = 0;
pub const MASTER_MEM_NOC_CFG: i32 = 1;
pub const MASTER_GNOC_MEM_NOC: i32 = 2;
pub const MASTER_MNOC_HF_MEM_NOC: i32 = 3;
pub const MASTER_MNOC_SF_MEM_NOC: i32 = 4;
pub const MASTER_SNOC_GC_MEM_NOC: i32 = 5;
pub const MASTER_SNOC_SF_MEM_NOC: i32 = 6;
pub const MASTER_GRAPHICS_3D: i32 = 7;
pub const SLAVE_MSS_PROC_MS_MPU_CFG: i32 = 8;
pub const SLAVE_MEM_NOC_GNOC: i32 = 9;
pub const SLAVE_LLCC: i32 = 10;
pub const SLAVE_MEM_NOC_SNOC: i32 = 11;
pub const SLAVE_SERVICE_MEM_NOC: i32 = 12;
pub const MASTER_LLCC: i32 = 13;
pub const SLAVE_EBI_CH0: i32 = 14;

pub const MASTER_CNOC_MNOC_CFG: i32 = 0;
pub const MASTER_CAMNOC_HF0: i32 = 1;
pub const MASTER_CAMNOC_HF1: i32 = 2;
pub const MASTER_CAMNOC_SF: i32 = 3;
pub const MASTER_MDP_PORT0: i32 = 4;
pub const MASTER_MDP_PORT1: i32 = 5;
pub const MASTER_ROTATOR: i32 = 6;
pub const MASTER_VIDEO_P0: i32 = 7;
pub const MASTER_VIDEO_P1: i32 = 8;
pub const MASTER_VIDEO_PROC: i32 = 9;
pub const SLAVE_MNOC_SF_MEM_NOC: i32 = 10;
pub const SLAVE_MNOC_HF_MEM_NOC: i32 = 11;
pub const SLAVE_SERVICE_MNOC: i32 = 12;

pub const MASTER_SNOC_CFG: i32 = 0;
pub const MASTER_A1NOC_SNOC: i32 = 1;
pub const MASTER_A2NOC_SNOC: i32 = 2;
pub const MASTER_GNOC_SNOC: i32 = 3;
pub const MASTER_MEM_NOC_SNOC: i32 = 4;
pub const MASTER_PIMEM: i32 = 5;
pub const MASTER_GIC: i32 = 6;
pub const SLAVE_APPSS: i32 = 7;
pub const SLAVE_SNOC_CNOC: i32 = 8;
pub const SLAVE_SNOC_MEM_NOC_GC: i32 = 9;
pub const SLAVE_SNOC_MEM_NOC_SF: i32 = 10;
pub const SLAVE_OCIMEM: i32 = 11;
pub const SLAVE_PIMEM: i32 = 12;
pub const SLAVE_SERVICE_SNOC: i32 = 13;
pub const SLAVE_QDSS_STM: i32 = 14;
pub const SLAVE_TCU: i32 = 15;
pub const MASTER_CAMNOC_HF0_UNCOMP: i32 = 16;
pub const MASTER_CAMNOC_HF1_UNCOMP: i32 = 17;
pub const MASTER_CAMNOC_SF_UNCOMP: i32 = 18;
pub const SLAVE_CAMNOC_UNCOMP: i32 = 19;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
