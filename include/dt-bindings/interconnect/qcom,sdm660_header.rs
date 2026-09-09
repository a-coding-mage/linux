/* SPDX-License-Identifier: GPL-2.0 */
/* SDM660 interconnect IDs */

/* A2NOC */
pub const MASTER_IPA: u32 = 0;
pub const MASTER_CNOC_A2NOC: u32 = 1;
pub const MASTER_SDCC_1: u32 = 2;
pub const MASTER_SDCC_2: u32 = 3;
pub const MASTER_BLSP_1: u32 = 4;
pub const MASTER_BLSP_2: u32 = 5;
pub const MASTER_UFS: u32 = 6;
pub const MASTER_USB_HS: u32 = 7;
pub const MASTER_USB3: u32 = 8;
pub const MASTER_CRYPTO_C0: u32 = 9;
pub const SLAVE_A2NOC_SNOC: u32 = 10;

/* BIMC */
pub const MASTER_GNOC_BIMC: u32 = 0;
pub const MASTER_OXILI: u32 = 1;
pub const MASTER_MNOC_BIMC: u32 = 2;
pub const MASTER_SNOC_BIMC: u32 = 3;
pub const MASTER_PIMEM: u32 = 4;
pub const SLAVE_EBI: u32 = 5;
pub const SLAVE_HMSS_L3: u32 = 6;
pub const SLAVE_BIMC_SNOC: u32 = 7;

/* CNOC */
pub const MASTER_SNOC_CNOC: u32 = 0;
pub const MASTER_QDSS_DAP: u32 = 1;
pub const SLAVE_CNOC_A2NOC: u32 = 2;
pub const SLAVE_MPM: u32 = 3;
pub const SLAVE_PMIC_ARB: u32 = 4;
pub const SLAVE_TLMM_NORTH: u32 = 5;
pub const SLAVE_TCSR: u32 = 6;
pub const SLAVE_PIMEM_CFG: u32 = 7;
pub const SLAVE_IMEM_CFG: u32 = 8;
pub const SLAVE_MESSAGE_RAM: u32 = 9;
pub const SLAVE_GLM: u32 = 10;
pub const SLAVE_BIMC_CFG: u32 = 11;
pub const SLAVE_PRNG: u32 = 12;
pub const SLAVE_SPDM: u32 = 13;
pub const SLAVE_QDSS_CFG: u32 = 14;
pub const SLAVE_CNOC_MNOC_CFG: u32 = 15;
pub const SLAVE_SNOC_CFG: u32 = 16;
pub const SLAVE_QM_CFG: u32 = 17;
pub const SLAVE_CLK_CTL: u32 = 18;
pub const SLAVE_MSS_CFG: u32 = 19;
pub const SLAVE_TLMM_SOUTH: u32 = 20;
pub const SLAVE_UFS_CFG: u32 = 21;
pub const SLAVE_A2NOC_CFG: u32 = 22;
pub const SLAVE_A2NOC_SMMU_CFG: u32 = 23;
pub const SLAVE_GPUSS_CFG: u32 = 24;
pub const SLAVE_AHB2PHY: u32 = 25;
pub const SLAVE_BLSP_1: u32 = 26;
pub const SLAVE_SDCC_1: u32 = 27;
pub const SLAVE_SDCC_2: u32 = 28;
pub const SLAVE_TLMM_CENTER: u32 = 29;
pub const SLAVE_BLSP_2: u32 = 30;
pub const SLAVE_PDM: u32 = 31;
pub const SLAVE_CNOC_MNOC_MMSS_CFG: u32 = 32;
pub const SLAVE_USB_HS: u32 = 33;
pub const SLAVE_USB3_0: u32 = 34;
pub const SLAVE_SRVC_CNOC: u32 = 35;

/* GNOC */
pub const MASTER_APSS_PROC: u32 = 0;
pub const SLAVE_GNOC_BIMC: u32 = 1;
pub const SLAVE_GNOC_SNOC: u32 = 2;

/* MNOC */
pub const MASTER_CPP: u32 = 0;
pub const MASTER_JPEG: u32 = 1;
pub const MASTER_MDP_P0: u32 = 2;
pub const MASTER_MDP_P1: u32 = 3;
pub const MASTER_VENUS: u32 = 4;
pub const MASTER_VFE: u32 = 5;
pub const SLAVE_MNOC_BIMC: u32 = 6;
pub const MASTER_CNOC_MNOC_MMSS_CFG: u32 = 7;
pub const MASTER_CNOC_MNOC_CFG: u32 = 8;
pub const SLAVE_CAMERA_CFG: u32 = 9;
pub const SLAVE_CAMERA_THROTTLE_CFG: u32 = 10;
pub const SLAVE_MISC_CFG: u32 = 11;
pub const SLAVE_VENUS_THROTTLE_CFG: u32 = 12;
pub const SLAVE_VENUS_CFG: u32 = 13;
pub const SLAVE_MMSS_CLK_XPU_CFG: u32 = 14;
pub const SLAVE_MMSS_CLK_CFG: u32 = 15;
pub const SLAVE_MNOC_MPU_CFG: u32 = 16;
pub const SLAVE_DISPLAY_CFG: u32 = 17;
pub const SLAVE_CSI_PHY_CFG: u32 = 18;
pub const SLAVE_DISPLAY_THROTTLE_CFG: u32 = 19;
pub const SLAVE_SMMU_CFG: u32 = 20;
pub const SLAVE_SRVC_MNOC: u32 = 21;

/* SNOC */
pub const MASTER_QDSS_ETR: u32 = 0;
pub const MASTER_QDSS_BAM: u32 = 1;
pub const MASTER_SNOC_CFG: u32 = 2;
pub const MASTER_BIMC_SNOC: u32 = 3;
pub const MASTER_A2NOC_SNOC: u32 = 4;
pub const MASTER_GNOC_SNOC: u32 = 5;
pub const SLAVE_HMSS: u32 = 6;
pub const SLAVE_LPASS: u32 = 7;
pub const SLAVE_WLAN: u32 = 8;
pub const SLAVE_CDSP: u32 = 9;
pub const SLAVE_IPA: u32 = 10;
pub const SLAVE_SNOC_BIMC: u32 = 11;
pub const SLAVE_SNOC_CNOC: u32 = 12;
pub const SLAVE_IMEM: u32 = 13;
pub const SLAVE_PIMEM: u32 = 14;
pub const SLAVE_QDSS_STM: u32 = 15;
pub const SLAVE_SRVC_SNOC: u32 = 16;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
