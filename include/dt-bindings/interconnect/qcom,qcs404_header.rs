/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Qualcomm interconnect IDs
 *
 * Copyright (c) 2019, Linaro Ltd.
 * Author: Georgi Djakov <georgi.djakov@linaro.org>
 */

pub const MASTER_AMPSS_M0: u32 = 0;
pub const MASTER_OXILI: u32 = 1;
pub const MASTER_MDP_PORT0: u32 = 2;
pub const MASTER_SNOC_BIMC_1: u32 = 3;
pub const MASTER_TCU_0: u32 = 4;
pub const SLAVE_EBI_CH0: u32 = 5;
pub const SLAVE_BIMC_SNOC: u32 = 6;

pub const MASTER_SPDM: u32 = 0;
pub const MASTER_BLSP_1: u32 = 1;
pub const MASTER_BLSP_2: u32 = 2;
pub const MASTER_XI_USB_HS1: u32 = 3;
pub const MASTER_CRYPT0: u32 = 4;
pub const MASTER_SDCC_1: u32 = 5;
pub const MASTER_SDCC_2: u32 = 6;
pub const MASTER_SNOC_PCNOC: u32 = 7;
pub const MASTER_QPIC: u32 = 8;
pub const PCNOC_INT_0: u32 = 9;
pub const PCNOC_INT_2: u32 = 10;
pub const PCNOC_INT_3: u32 = 11;
pub const PCNOC_S_0: u32 = 12;
pub const PCNOC_S_1: u32 = 13;
pub const PCNOC_S_2: u32 = 14;
pub const PCNOC_S_3: u32 = 15;
pub const PCNOC_S_4: u32 = 16;
pub const PCNOC_S_6: u32 = 17;
pub const PCNOC_S_7: u32 = 18;
pub const PCNOC_S_8: u32 = 19;
pub const PCNOC_S_9: u32 = 20;
pub const PCNOC_S_10: u32 = 21;
pub const PCNOC_S_11: u32 = 22;
pub const SLAVE_SPDM: u32 = 23;
pub const SLAVE_PDM: u32 = 24;
pub const SLAVE_PRNG: u32 = 25;
pub const SLAVE_TCSR: u32 = 26;
pub const SLAVE_SNOC_CFG: u32 = 27;
pub const SLAVE_MESSAGE_RAM: u32 = 28;
pub const SLAVE_DISP_SS_CFG: u32 = 29;
pub const SLAVE_GPU_CFG: u32 = 30;
pub const SLAVE_BLSP_1: u32 = 31;
pub const SLAVE_BLSP_2: u32 = 32;
pub const SLAVE_TLMM_NORTH: u32 = 33;
pub const SLAVE_PCIE: u32 = 34;
pub const SLAVE_ETHERNET: u32 = 35;
pub const SLAVE_TLMM_EAST: u32 = 36;
pub const SLAVE_TCU: u32 = 37;
pub const SLAVE_PMIC_ARB: u32 = 38;
pub const SLAVE_SDCC_1: u32 = 39;
pub const SLAVE_SDCC_2: u32 = 40;
pub const SLAVE_TLMM_SOUTH: u32 = 41;
pub const SLAVE_USB_HS: u32 = 42;
pub const SLAVE_USB3: u32 = 43;
pub const SLAVE_CRYPTO_0_CFG: u32 = 44;
pub const SLAVE_PCNOC_SNOC: u32 = 45;

// The C header reuses these macro names in a later interconnect namespace.
pub const MASTER_QDSS_BAM: u32 = 0;
pub const MASTER_BIMC_SNOC: u32 = 1;
pub const MASTER_PCNOC_SNOC: u32 = 2;
pub const MASTER_QDSS_ETR: u32 = 3;
pub const MASTER_EMAC: u32 = 4;
pub const MASTER_PCIE: u32 = 5;
pub const MASTER_USB3: u32 = 6;
pub const QDSS_INT: u32 = 7;
pub const SNOC_INT_0: u32 = 8;
pub const SNOC_INT_1: u32 = 9;
pub const SNOC_INT_2: u32 = 10;
pub const SLAVE_KPSS_AHB: u32 = 11;
pub const SLAVE_WCSS: u32 = 12;
pub const SLAVE_SNOC_BIMC_1: u32 = 13;
pub const SLAVE_IMEM: u32 = 14;
pub const SLAVE_SNOC_PCNOC: u32 = 15;
pub const SLAVE_QDSS_STM: u32 = 16;
pub const SLAVE_CATS_0: u32 = 17;
pub const SLAVE_CATS_1: u32 = 18;
pub const SLAVE_LPASS: u32 = 19;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
