/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

pub const MASTER_QUP_CORE_0: i32 = 0;
pub const SLAVE_QUP_CORE_0: i32 = 1;

pub const SNOC_CNOC_MAS: i32 = 0;
pub const MASTER_QDSS_DAP: i32 = 1;
pub const SLAVE_AHB2PHY_USB: i32 = 2;
pub const SLAVE_APSS_THROTTLE_CFG: i32 = 3;
pub const SLAVE_AUDIO: i32 = 4;
pub const SLAVE_BOOT_ROM: i32 = 5;
pub const SLAVE_CAMERA_NRT_THROTTLE_CFG: i32 = 6;
pub const SLAVE_CAMERA_CFG: i32 = 7;
pub const SLAVE_CDSP_THROTTLE_CFG: i32 = 8;
pub const SLAVE_CLK_CTL: i32 = 9;
pub const SLAVE_DSP_CFG: i32 = 10;
pub const SLAVE_RBCPR_CX_CFG: i32 = 11;
pub const SLAVE_RBCPR_MX_CFG: i32 = 12;
pub const SLAVE_CRYPTO_0_CFG: i32 = 13;
pub const SLAVE_DDR_SS_CFG: i32 = 14;
pub const SLAVE_DISPLAY_CFG: i32 = 15;
pub const SLAVE_EMAC0_CFG: i32 = 16;
pub const SLAVE_EMAC1_CFG: i32 = 17;
pub const SLAVE_GPU_CFG: i32 = 18;
pub const SLAVE_GPU_THROTTLE_CFG: i32 = 19;
pub const SLAVE_HWKM: i32 = 20;
pub const SLAVE_IMEM_CFG: i32 = 21;
pub const SLAVE_MAPSS: i32 = 22;
pub const SLAVE_MDSP_MPU_CFG: i32 = 23;
pub const SLAVE_MESSAGE_RAM: i32 = 24;
pub const SLAVE_MSS: i32 = 25;
pub const SLAVE_PCIE_CFG: i32 = 26;
pub const SLAVE_PDM: i32 = 27;
pub const SLAVE_PIMEM_CFG: i32 = 28;
pub const SLAVE_PKA_WRAPPER_CFG: i32 = 29;
pub const SLAVE_PMIC_ARB: i32 = 30;
pub const SLAVE_QDSS_CFG: i32 = 31;
pub const SLAVE_QM_CFG: i32 = 32;
pub const SLAVE_QM_MPU_CFG: i32 = 33;
pub const SLAVE_QPIC: i32 = 34;
pub const SLAVE_QUP_0: i32 = 35;
pub const SLAVE_RPM: i32 = 36;
pub const SLAVE_SDCC_1: i32 = 37;
pub const SLAVE_SDCC_2: i32 = 38;
pub const SLAVE_SECURITY: i32 = 39;
pub const SLAVE_SNOC_CFG: i32 = 40;
pub const SNOC_SF_THROTTLE_CFG: i32 = 41;
pub const SLAVE_TLMM: i32 = 42;
pub const SLAVE_TSCSS: i32 = 43;
pub const SLAVE_USB2: i32 = 44;
pub const SLAVE_USB3: i32 = 45;
pub const SLAVE_VENUS_CFG: i32 = 46;
pub const SLAVE_VENUS_THROTTLE_CFG: i32 = 47;
pub const SLAVE_VSENSE_CTRL_CFG: i32 = 48;
pub const SLAVE_SERVICE_CNOC: i32 = 49;

pub const MASTER_LLCC: i32 = 0;
pub const SLAVE_EBI_CH0: i32 = 1;

pub const MASTER_GRAPHICS_3D: i32 = 0;
pub const MASTER_MNOC_HF_MEM_NOC: i32 = 1;
pub const MASTER_ANOC_PCIE_MEM_NOC: i32 = 2;
pub const MASTER_SNOC_SF_MEM_NOC: i32 = 3;
pub const MASTER_AMPSS_M0: i32 = 4;
pub const MASTER_SYS_TCU: i32 = 5;
pub const SLAVE_LLCC: i32 = 6;
pub const SLAVE_MEMNOC_SNOC: i32 = 7;
pub const SLAVE_MEM_NOC_PCIE_SNOC: i32 = 8;

pub const MASTER_CAMNOC_SF: i32 = 0;
pub const MASTER_VIDEO_P0: i32 = 1;
pub const MASTER_VIDEO_PROC: i32 = 2;
pub const SLAVE_MMNRT_VIRT: i32 = 3;

pub const MASTER_CAMNOC_HF: i32 = 0;
pub const MASTER_MDP_PORT0: i32 = 1;
pub const MASTER_MMRT_VIRT: i32 = 2;
pub const SLAVE_MM_MEMNOC: i32 = 3;

pub const MASTER_SNOC_CFG: i32 = 0;
pub const MASTER_TIC: i32 = 1;
pub const MASTER_ANOC_SNOC: i32 = 2;
pub const MASTER_MEMNOC_PCIE: i32 = 3;
pub const MASTER_MEMNOC_SNOC: i32 = 4;
pub const MASTER_PIMEM: i32 = 5;
pub const MASTER_PCIE2_0: i32 = 6;
pub const MASTER_QDSS_BAM: i32 = 7;
pub const MASTER_QPIC: i32 = 8;
pub const MASTER_QUP_0: i32 = 9;
pub const CNOC_SNOC_MAS: i32 = 10;
pub const MASTER_AUDIO: i32 = 11;
pub const MASTER_EMAC_0: i32 = 12;
pub const MASTER_EMAC_1: i32 = 13;
pub const MASTER_QDSS_ETR: i32 = 14;
pub const MASTER_SDCC_1: i32 = 15;
pub const MASTER_SDCC_2: i32 = 16;
pub const MASTER_USB2_0: i32 = 17;
pub const MASTER_USB3: i32 = 18;
pub const MASTER_CRYPTO_CORE0: i32 = 19;
pub const SLAVE_APPSS: i32 = 20;
pub const SLAVE_MCUSS: i32 = 21;
pub const SLAVE_WCSS: i32 = 22;
pub const SLAVE_MEMNOC_SF: i32 = 23;
pub const SNOC_CNOC_SLV: i32 = 24;
pub const SLAVE_BOOTIMEM: i32 = 25;
pub const SLAVE_OCIMEM: i32 = 26;
pub const SLAVE_PIMEM: i32 = 27;
pub const SLAVE_SERVICE_SNOC: i32 = 28;
pub const SLAVE_PCIE2_0: i32 = 29;
pub const SLAVE_QDSS_STM: i32 = 30;
pub const SLAVE_TCU: i32 = 31;
pub const SLAVE_PCIE_MEMNOC: i32 = 32;
pub const SLAVE_ANOC_SNOC: i32 = 33;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
