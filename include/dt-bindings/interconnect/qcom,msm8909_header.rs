/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Qualcomm MSM8909 interconnect IDs
 */

/* BIMC fabric */
pub mod bimc {
    pub const MAS_APPS_PROC: i32 = 0;
    pub const MAS_OXILI: i32 = 1;
    pub const MAS_SNOC_BIMC_0: i32 = 2;
    pub const MAS_SNOC_BIMC_1: i32 = 3;
    pub const MAS_TCU_0: i32 = 4;
    pub const MAS_TCU_1: i32 = 5;
    pub const SLV_EBI: i32 = 6;
    pub const SLV_BIMC_SNOC: i32 = 7;
}

/* PCNOC fabric */
pub mod pcnoc {
    pub const MAS_AUDIO: i32 = 0;
    pub const MAS_SPDM: i32 = 1;
    pub const MAS_DEHR: i32 = 2;
    pub const MAS_QPIC: i32 = 3;
    pub const MAS_BLSP_1: i32 = 4;
    pub const MAS_USB_HS: i32 = 5;
    pub const MAS_CRYPTO: i32 = 6;
    pub const MAS_SDCC_1: i32 = 7;
    pub const MAS_SDCC_2: i32 = 8;
    pub const MAS_SNOC_PCNOC: i32 = 9;
    pub const PCNOC_M_0: i32 = 10;
    pub const PCNOC_M_1: i32 = 11;
    pub const PCNOC_INT_0: i32 = 12;
    pub const PCNOC_INT_1: i32 = 13;
    pub const PCNOC_S_0: i32 = 14;
    pub const PCNOC_S_1: i32 = 15;
    pub const PCNOC_S_2: i32 = 16;
    pub const PCNOC_S_3: i32 = 17;
    pub const PCNOC_S_4: i32 = 18;
    pub const PCNOC_S_5: i32 = 19;
    pub const PCNOC_S_7: i32 = 20;
    pub const SLV_TCSR: i32 = 21;
    pub const SLV_SDCC_1: i32 = 22;
    pub const SLV_BLSP_1: i32 = 23;
    pub const SLV_CRYPTO_0_CFG: i32 = 24;
    pub const SLV_MESSAGE_RAM: i32 = 25;
    pub const SLV_PDM: i32 = 26;
    pub const SLV_PRNG: i32 = 27;
    pub const SLV_USB_HS: i32 = 28;
    pub const SLV_QPIC: i32 = 29;
    pub const SLV_SPDM: i32 = 30;
    pub const SLV_SDCC_2: i32 = 31;
    pub const SLV_AUDIO: i32 = 32;
    pub const SLV_DEHR_CFG: i32 = 33;
    pub const SLV_SNOC_CFG: i32 = 34;
    pub const SLV_QDSS_CFG: i32 = 35;
    pub const SLV_USB_PHY: i32 = 36;
    pub const SLV_CAMERA_SS_CFG: i32 = 37;
    pub const SLV_DISP_SS_CFG: i32 = 38;
    pub const SLV_VENUS_CFG: i32 = 39;
    pub const SLV_TLMM: i32 = 40;
    pub const SLV_GPU_CFG: i32 = 41;
    pub const SLV_IMEM_CFG: i32 = 42;
    pub const SLV_BIMC_CFG: i32 = 43;
    pub const SLV_PMIC_ARB: i32 = 44;
    pub const SLV_TCU: i32 = 45;
    pub const SLV_PCNOC_SNOC: i32 = 46;
}

/* SNOC fabric */
pub mod snoc {
    pub const MAS_QDSS_BAM: i32 = 0;
    pub const MAS_BIMC_SNOC: i32 = 1;
    pub const MAS_MDP: i32 = 2;
    pub const MAS_PCNOC_SNOC: i32 = 3;
    pub const MAS_VENUS: i32 = 4;
    pub const MAS_VFE: i32 = 5;
    pub const MAS_QDSS_ETR: i32 = 6;
    pub const MM_INT_0: i32 = 7;
    pub const MM_INT_1: i32 = 8;
    pub const MM_INT_2: i32 = 9;
    pub const MM_INT_BIMC: i32 = 10;
    pub const QDSS_INT: i32 = 11;
    pub const SNOC_INT_0: i32 = 12;
    pub const SNOC_INT_1: i32 = 13;
    pub const SNOC_INT_BIMC: i32 = 14;
    pub const SLV_KPSS_AHB: i32 = 15;
    pub const SLV_SNOC_BIMC_0: i32 = 16;
    pub const SLV_SNOC_BIMC_1: i32 = 17;
    pub const SLV_IMEM: i32 = 18;
    pub const SLV_SNOC_PCNOC: i32 = 19;
    pub const SLV_QDSS_STM: i32 = 20;
    pub const SLV_CATS_0: i32 = 21;
    pub const SLV_CATS_1: i32 = 22;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
