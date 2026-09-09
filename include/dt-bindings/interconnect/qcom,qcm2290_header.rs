/* SPDX-License-Identifier: GPL-2.0 */
/* QCM2290 interconnect IDs */

/* BIMC */
pub const MASTER_APPSS_PROC: i32 = 0;
pub const MASTER_SNOC_BIMC_RT: i32 = 1;
pub const MASTER_SNOC_BIMC_NRT: i32 = 2;
pub const MASTER_SNOC_BIMC: i32 = 3;
pub const MASTER_TCU_0: i32 = 4;
pub const MASTER_GFX3D: i32 = 5;
pub const SLAVE_EBI1: i32 = 6;
pub const SLAVE_BIMC_SNOC: i32 = 7;

/* CNOC */
pub const MASTER_SNOC_CNOC: i32 = 0;
pub const MASTER_QDSS_DAP: i32 = 1;
pub const SLAVE_BIMC_CFG: i32 = 2;
pub const SLAVE_CAMERA_NRT_THROTTLE_CFG: i32 = 3;
pub const SLAVE_CAMERA_RT_THROTTLE_CFG: i32 = 4;
pub const SLAVE_CAMERA_CFG: i32 = 5;
pub const SLAVE_CLK_CTL: i32 = 6;
pub const SLAVE_CRYPTO_0_CFG: i32 = 7;
pub const SLAVE_DISPLAY_CFG: i32 = 8;
pub const SLAVE_DISPLAY_THROTTLE_CFG: i32 = 9;
pub const SLAVE_GPU_CFG: i32 = 10;
pub const SLAVE_HWKM: i32 = 11;
pub const SLAVE_IMEM_CFG: i32 = 12;
pub const SLAVE_IPA_CFG: i32 = 13;
pub const SLAVE_LPASS: i32 = 14;
pub const SLAVE_MESSAGE_RAM: i32 = 15;
pub const SLAVE_PDM: i32 = 16;
pub const SLAVE_PIMEM_CFG: i32 = 17;
pub const SLAVE_PKA_WRAPPER: i32 = 18;
pub const SLAVE_PMIC_ARB: i32 = 19;
pub const SLAVE_PRNG: i32 = 20;
pub const SLAVE_QDSS_CFG: i32 = 21;
pub const SLAVE_QM_CFG: i32 = 22;
pub const SLAVE_QM_MPU_CFG: i32 = 23;
pub const SLAVE_QPIC: i32 = 24;
pub const SLAVE_QUP_0: i32 = 25;
pub const SLAVE_SDCC_1: i32 = 26;
pub const SLAVE_SDCC_2: i32 = 27;
pub const SLAVE_SNOC_CFG: i32 = 28;
pub const SLAVE_TCSR: i32 = 29;
pub const SLAVE_USB3: i32 = 30;
pub const SLAVE_VENUS_CFG: i32 = 31;
pub const SLAVE_VENUS_THROTTLE_CFG: i32 = 32;
pub const SLAVE_VSENSE_CTRL_CFG: i32 = 33;
pub const SLAVE_SERVICE_CNOC: i32 = 34;

/* SNOC */
pub const MASTER_CRYPTO_CORE0: i32 = 0;
pub const MASTER_SNOC_CFG: i32 = 1;
pub const MASTER_TIC: i32 = 2;
pub const MASTER_ANOC_SNOC: i32 = 3;
pub const MASTER_BIMC_SNOC: i32 = 4;
pub const MASTER_PIMEM: i32 = 5;
pub const MASTER_QDSS_BAM: i32 = 6;
pub const MASTER_QUP_0: i32 = 7;
pub const MASTER_IPA: i32 = 8;
pub const MASTER_QDSS_ETR: i32 = 9;
pub const MASTER_SDCC_1: i32 = 10;
pub const MASTER_SDCC_2: i32 = 11;
pub const MASTER_QPIC: i32 = 12;
pub const MASTER_USB3_0: i32 = 13;
pub const SLAVE_APPSS: i32 = 14;
pub const SLAVE_SNOC_CNOC: i32 = 15;
pub const SLAVE_IMEM: i32 = 16;
pub const SLAVE_PIMEM: i32 = 17;
pub const SLAVE_SNOC_BIMC: i32 = 18;
pub const SLAVE_SERVICE_SNOC: i32 = 19;
pub const SLAVE_QDSS_STM: i32 = 20;
pub const SLAVE_TCU: i32 = 21;
pub const SLAVE_ANOC_SNOC: i32 = 22;

/* QUP Virtual */
pub const MASTER_QUP_CORE_0: i32 = 0;
pub const SLAVE_QUP_CORE_0: i32 = 1;

/* MMNRT Virtual */
pub const MASTER_CAMNOC_SF: i32 = 0;
pub const MASTER_VIDEO_P0: i32 = 1;
pub const MASTER_VIDEO_PROC: i32 = 2;
pub const SLAVE_SNOC_BIMC_NRT: i32 = 3;

/* MMRT Virtual */
pub const MASTER_CAMNOC_HF: i32 = 0;
pub const MASTER_MDP0: i32 = 1;
pub const SLAVE_SNOC_BIMC_RT: i32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
