/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2021, The Linux Foundation. All rights reserved.
 * Copyright (c) 2022, Linaro Ltd.
 */

/* aggre1_noc */
pub const MASTER_QSPI_0: u32 = 0;
pub const MASTER_QUP_1: u32 = 1;
pub const MASTER_QUP_2: u32 = 2;
pub const MASTER_A1NOC_CFG: u32 = 3;
pub const MASTER_IPA: u32 = 4;
pub const MASTER_EMAC_1: u32 = 5;
pub const MASTER_SDCC_4: u32 = 6;
pub const MASTER_UFS_MEM: u32 = 7;
pub const MASTER_USB3_0: u32 = 8;
pub const MASTER_USB3_1: u32 = 9;
pub const MASTER_USB3_MP: u32 = 10;
pub const MASTER_USB4_0: u32 = 11;
pub const MASTER_USB4_1: u32 = 12;
pub const SLAVE_A1NOC_SNOC: u32 = 13;
pub const SLAVE_USB_NOC_SNOC: u32 = 14;
pub const SLAVE_SERVICE_A1NOC: u32 = 15;

/* aggre2_noc */
pub const MASTER_QDSS_BAM: u32 = 0;
pub const MASTER_QUP_0: u32 = 1;
pub const MASTER_A2NOC_CFG: u32 = 2;
pub const MASTER_CRYPTO: u32 = 3;
pub const MASTER_SENSORS_PROC: u32 = 4;
pub const MASTER_SP: u32 = 5;
pub const MASTER_EMAC: u32 = 6;
pub const MASTER_PCIE_0: u32 = 7;
pub const MASTER_PCIE_1: u32 = 8;
pub const MASTER_PCIE_2A: u32 = 9;
pub const MASTER_PCIE_2B: u32 = 10;
pub const MASTER_PCIE_3A: u32 = 11;
pub const MASTER_PCIE_3B: u32 = 12;
pub const MASTER_PCIE_4: u32 = 13;
pub const MASTER_QDSS_ETR: u32 = 14;
pub const MASTER_SDCC_2: u32 = 15;
pub const MASTER_UFS_CARD: u32 = 16;
pub const SLAVE_A2NOC_SNOC: u32 = 17;
pub const SLAVE_ANOC_PCIE_GEM_NOC: u32 = 18;
pub const SLAVE_SERVICE_A2NOC: u32 = 19;

/* clk_virt */
/* 0 was used by MASTER_IPA_CORE, now represented as RPMh clock */
pub const MASTER_QUP_CORE_0: u32 = 1;
pub const MASTER_QUP_CORE_1: u32 = 2;
pub const MASTER_QUP_CORE_2: u32 = 3;
/* 4 was used by SLAVE_IPA_CORE, now represented as RPMh clock */
pub const SLAVE_QUP_CORE_0: u32 = 5;
pub const SLAVE_QUP_CORE_1: u32 = 6;
pub const SLAVE_QUP_CORE_2: u32 = 7;

/* config_noc */
pub const MASTER_GEM_NOC_CNOC: u32 = 0;
pub const MASTER_GEM_NOC_PCIE_SNOC: u32 = 1;
pub const SLAVE_AHB2PHY_0: u32 = 2;
pub const SLAVE_AHB2PHY_1: u32 = 3;
pub const SLAVE_AHB2PHY_2: u32 = 4;
pub const SLAVE_AOSS: u32 = 5;
pub const SLAVE_APPSS: u32 = 6;
pub const SLAVE_CAMERA_CFG: u32 = 7;
pub const SLAVE_CLK_CTL: u32 = 8;
pub const SLAVE_CDSP_CFG: u32 = 9;
pub const SLAVE_CDSP1_CFG: u32 = 10;
pub const SLAVE_RBCPR_CX_CFG: u32 = 11;
pub const SLAVE_RBCPR_MMCX_CFG: u32 = 12;
pub const SLAVE_RBCPR_MX_CFG: u32 = 13;
pub const SLAVE_CPR_NSPCX: u32 = 14;
pub const SLAVE_CRYPTO_0_CFG: u32 = 15;
pub const SLAVE_CX_RDPM: u32 = 16;
pub const SLAVE_DCC_CFG: u32 = 17;
pub const SLAVE_DISPLAY_CFG: u32 = 18;
pub const SLAVE_DISPLAY1_CFG: u32 = 19;
pub const SLAVE_EMAC_CFG: u32 = 20;
pub const SLAVE_EMAC1_CFG: u32 = 21;
pub const SLAVE_GFX3D_CFG: u32 = 22;
pub const SLAVE_HWKM: u32 = 23;
pub const SLAVE_IMEM_CFG: u32 = 24;
pub const SLAVE_IPA_CFG: u32 = 25;
pub const SLAVE_IPC_ROUTER_CFG: u32 = 26;
pub const SLAVE_LPASS: u32 = 27;
pub const SLAVE_MX_RDPM: u32 = 28;
pub const SLAVE_MXC_RDPM: u32 = 29;
pub const SLAVE_PCIE_0_CFG: u32 = 30;
pub const SLAVE_PCIE_1_CFG: u32 = 31;
pub const SLAVE_PCIE_2A_CFG: u32 = 32;
pub const SLAVE_PCIE_2B_CFG: u32 = 33;
pub const SLAVE_PCIE_3A_CFG: u32 = 34;
pub const SLAVE_PCIE_3B_CFG: u32 = 35;
pub const SLAVE_PCIE_4_CFG: u32 = 36;
pub const SLAVE_PCIE_RSC_CFG: u32 = 37;
pub const SLAVE_PDM: u32 = 38;
pub const SLAVE_PIMEM_CFG: u32 = 39;
pub const SLAVE_PKA_WRAPPER_CFG: u32 = 40;
pub const SLAVE_PMU_WRAPPER_CFG: u32 = 41;
pub const SLAVE_QDSS_CFG: u32 = 42;
pub const SLAVE_QSPI_0: u32 = 43;
pub const SLAVE_QUP_0: u32 = 44;
pub const SLAVE_QUP_1: u32 = 45;
pub const SLAVE_QUP_2: u32 = 46;
pub const SLAVE_SDCC_2: u32 = 47;
pub const SLAVE_SDCC_4: u32 = 48;
pub const SLAVE_SECURITY: u32 = 49;
pub const SLAVE_SMMUV3_CFG: u32 = 50;
pub const SLAVE_SMSS_CFG: u32 = 51;
pub const SLAVE_SPSS_CFG: u32 = 52;
pub const SLAVE_TCSR: u32 = 53;
pub const SLAVE_TLMM: u32 = 54;
pub const SLAVE_UFS_CARD_CFG: u32 = 55;
pub const SLAVE_UFS_MEM_CFG: u32 = 56;
pub const SLAVE_USB3_0: u32 = 57;
pub const SLAVE_USB3_1: u32 = 58;
pub const SLAVE_USB3_MP: u32 = 59;
pub const SLAVE_USB4_0: u32 = 60;
pub const SLAVE_USB4_1: u32 = 61;
pub const SLAVE_VENUS_CFG: u32 = 62;
pub const SLAVE_VSENSE_CTRL_CFG: u32 = 63;
pub const SLAVE_VSENSE_CTRL_R_CFG: u32 = 64;
pub const SLAVE_A1NOC_CFG: u32 = 65;
pub const SLAVE_A2NOC_CFG: u32 = 66;
pub const SLAVE_ANOC_PCIE_BRIDGE_CFG: u32 = 67;
pub const SLAVE_DDRSS_CFG: u32 = 68;
pub const SLAVE_CNOC_MNOC_CFG: u32 = 69;
pub const SLAVE_SNOC_CFG: u32 = 70;
pub const SLAVE_SNOC_SF_BRIDGE_CFG: u32 = 71;
pub const SLAVE_IMEM: u32 = 72;
pub const SLAVE_PIMEM: u32 = 73;
pub const SLAVE_SERVICE_CNOC: u32 = 74;
pub const SLAVE_PCIE_0: u32 = 75;
pub const SLAVE_PCIE_1: u32 = 76;
pub const SLAVE_PCIE_2A: u32 = 77;
pub const SLAVE_PCIE_2B: u32 = 78;
pub const SLAVE_PCIE_3A: u32 = 79;
pub const SLAVE_PCIE_3B: u32 = 80;
pub const SLAVE_PCIE_4: u32 = 81;
pub const SLAVE_QDSS_STM: u32 = 82;
pub const SLAVE_SMSS: u32 = 83;
pub const SLAVE_TCU: u32 = 84;

/* dc_noc */
pub const MASTER_CNOC_DC_NOC: u32 = 0;
pub const SLAVE_LLCC_CFG: u32 = 1;
pub const SLAVE_GEM_NOC_CFG: u32 = 2;

/* gem_noc */
pub const MASTER_GPU_TCU: u32 = 0;
pub const MASTER_PCIE_TCU: u32 = 1;
pub const MASTER_SYS_TCU: u32 = 2;
pub const MASTER_APPSS_PROC: u32 = 3;
pub const MASTER_COMPUTE_NOC: u32 = 4;
pub const MASTER_COMPUTE_NOC_1: u32 = 5;
pub const MASTER_GEM_NOC_CFG: u32 = 6;
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

/* lpass_ag_noc */
pub const MASTER_CNOC_LPASS_AG_NOC: u32 = 0;
pub const MASTER_LPASS_PROC: u32 = 1;
pub const SLAVE_LPASS_CORE_CFG: u32 = 2;
pub const SLAVE_LPASS_LPI_CFG: u32 = 3;
pub const SLAVE_LPASS_MPU_CFG: u32 = 4;
pub const SLAVE_LPASS_TOP_CFG: u32 = 5;
pub const SLAVE_LPASS_SNOC: u32 = 6;
pub const SLAVE_SERVICES_LPASS_AML_NOC: u32 = 7;
pub const SLAVE_SERVICE_LPASS_AG_NOC: u32 = 8;

/* mc_virt */
pub const MASTER_LLCC: u32 = 0;
pub const SLAVE_EBI1: u32 = 1;

/*mmss_noc */
pub const MASTER_CAMNOC_HF: u32 = 0;
pub const MASTER_MDP0: u32 = 1;
pub const MASTER_MDP1: u32 = 2;
pub const MASTER_MDP_CORE1_0: u32 = 3;
pub const MASTER_MDP_CORE1_1: u32 = 4;
pub const MASTER_CNOC_MNOC_CFG: u32 = 5;
pub const MASTER_ROTATOR: u32 = 6;
pub const MASTER_ROTATOR_1: u32 = 7;
pub const MASTER_VIDEO_P0: u32 = 8;
pub const MASTER_VIDEO_P1: u32 = 9;
pub const MASTER_VIDEO_PROC: u32 = 10;
pub const MASTER_CAMNOC_ICP: u32 = 11;
pub const MASTER_CAMNOC_SF: u32 = 12;
pub const SLAVE_MNOC_HF_MEM_NOC: u32 = 13;
pub const SLAVE_MNOC_SF_MEM_NOC: u32 = 14;
pub const SLAVE_SERVICE_MNOC: u32 = 15;

/* nspa_noc */
pub const MASTER_CDSP_NOC_CFG: u32 = 0;
pub const MASTER_CDSP_PROC: u32 = 1;
pub const SLAVE_CDSP_MEM_NOC: u32 = 2;
pub const SLAVE_NSP_XFR: u32 = 3;
pub const SLAVE_SERVICE_NSP_NOC: u32 = 4;

/* nspb_noc */
pub const MASTER_CDSPB_NOC_CFG: u32 = 0;
pub const MASTER_CDSP_PROC_B: u32 = 1;
pub const SLAVE_CDSPB_MEM_NOC: u32 = 2;
pub const SLAVE_NSPB_XFR: u32 = 3;
pub const SLAVE_SERVICE_NSPB_NOC: u32 = 4;

/* system_noc */
pub const MASTER_A1NOC_SNOC: u32 = 0;
pub const MASTER_A2NOC_SNOC: u32 = 1;
pub const MASTER_USB_NOC_SNOC: u32 = 2;
pub const MASTER_LPASS_ANOC: u32 = 3;
pub const MASTER_SNOC_CFG: u32 = 4;
pub const MASTER_PIMEM: u32 = 5;
pub const MASTER_GIC: u32 = 6;
pub const SLAVE_SNOC_GEM_NOC_GC: u32 = 7;
pub const SLAVE_SNOC_GEM_NOC_SF: u32 = 8;
pub const SLAVE_SERVICE_SNOC: u32 = 9;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
