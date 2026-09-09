/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Qualcomm MSM8937 interconnect IDs
 */

/* BIMC fabric */
pub mod bimc {
    pub const MAS_APPS_PROC: u32 = 0;
    pub const MAS_OXILI: u32 = 1;
    pub const MAS_SNOC_BIMC_0: u32 = 2;
    pub const MAS_SNOC_BIMC_2: u32 = 3;
    pub const MAS_SNOC_BIMC_1: u32 = 4;
    pub const MAS_TCU_0: u32 = 5;
    pub const SLV_EBI: u32 = 6;
    pub const SLV_BIMC_SNOC: u32 = 7;
}

/* PCNOC fabric */
pub mod pcnoc {
    pub const MAS_SPDM: u32 = 0;
    pub const MAS_BLSP_1: u32 = 1;
    pub const MAS_BLSP_2: u32 = 2;
    pub const MAS_USB_HS1: u32 = 3;
    pub const MAS_XI_USB_HS1: u32 = 4;
    pub const MAS_CRYPTO: u32 = 5;
    pub const MAS_SDCC_1: u32 = 6;
    pub const MAS_SDCC_2: u32 = 7;
    pub const MAS_SNOC_PCNOC: u32 = 8;
    pub const PCNOC_M_0: u32 = 9;
    pub const PCNOC_M_1: u32 = 10;
    pub const PCNOC_INT_0: u32 = 11;
    pub const PCNOC_INT_1: u32 = 12;
    pub const PCNOC_INT_2: u32 = 13;
    pub const PCNOC_INT_3: u32 = 14;
    pub const PCNOC_S_0: u32 = 15;
    pub const PCNOC_S_1: u32 = 16;
    pub const PCNOC_S_2: u32 = 17;
    pub const PCNOC_S_3: u32 = 18;
    pub const PCNOC_S_4: u32 = 19;
    pub const PCNOC_S_6: u32 = 20;
    pub const PCNOC_S_7: u32 = 21;
    pub const PCNOC_S_8: u32 = 22;
    pub const SLV_SDCC_2: u32 = 23;
    pub const SLV_SPDM: u32 = 24;
    pub const SLV_PDM: u32 = 25;
    pub const SLV_PRNG: u32 = 26;
    pub const SLV_TCSR: u32 = 27;
    pub const SLV_SNOC_CFG: u32 = 28;
    pub const SLV_MESSAGE_RAM: u32 = 29;
    pub const SLV_CAMERA_SS_CFG: u32 = 30;
    pub const SLV_DISP_SS_CFG: u32 = 31;
    pub const SLV_VENUS_CFG: u32 = 32;
    pub const SLV_GPU_CFG: u32 = 33;
    pub const SLV_TLMM: u32 = 34;
    pub const SLV_BLSP_1: u32 = 35;
    pub const SLV_BLSP_2: u32 = 36;
    pub const SLV_PMIC_ARB: u32 = 37;
    pub const SLV_SDCC_1: u32 = 38;
    pub const SLV_CRYPTO_0_CFG: u32 = 39;
    pub const SLV_USB_HS: u32 = 40;
    pub const SLV_TCU: u32 = 41;
    pub const SLV_PCNOC_SNOC: u32 = 42;
}

/* SNOC fabric */
pub mod snoc {
    pub const MAS_QDSS_BAM: u32 = 0;
    pub const MAS_BIMC_SNOC: u32 = 1;
    pub const MAS_PCNOC_SNOC: u32 = 2;
    pub const MAS_QDSS_ETR: u32 = 3;
    pub const QDSS_INT: u32 = 4;
    pub const SNOC_INT_0: u32 = 5;
    pub const SNOC_INT_1: u32 = 6;
    pub const SNOC_INT_2: u32 = 7;
    pub const SLV_KPSS_AHB: u32 = 8;
    pub const SLV_WCSS: u32 = 9;
    pub const SLV_SNOC_BIMC_1: u32 = 10;
    pub const SLV_IMEM: u32 = 11;
    pub const SLV_SNOC_PCNOC: u32 = 12;
    pub const SLV_QDSS_STM: u32 = 13;
    pub const SLV_CATS_1: u32 = 14;
    pub const SLV_LPASS: u32 = 15;
}

/* SNOC-MM fabric */
pub mod snoc_mm {
    pub const MAS_JPEG: u32 = 0;
    pub const MAS_MDP: u32 = 1;
    pub const MAS_VENUS: u32 = 2;
    pub const MAS_VFE0: u32 = 3;
    pub const MAS_VFE1: u32 = 4;
    pub const MAS_CPP: u32 = 5;
    pub const SLV_SNOC_BIMC_0: u32 = 6;
    pub const SLV_SNOC_BIMC_2: u32 = 7;
    pub const SLV_CATS_0: u32 = 8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
