/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Qualcomm SDX55 interconnect IDs
 *
 * Copyright (c) 2021, Linaro Ltd.
 * Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>
 */

pub const MASTER_LLCC: u32 = 0;
pub const SLAVE_EBI_CH0: u32 = 1;

pub const MASTER_TCU_0: u32 = 0;
pub const MASTER_SNOC_GC_MEM_NOC: u32 = 1;
pub const MASTER_AMPSS_M0: u32 = 2;
pub const SLAVE_LLCC: u32 = 3;
pub const SLAVE_MEM_NOC_SNOC: u32 = 4;
pub const SLAVE_MEM_NOC_PCIE_SNOC: u32 = 5;

pub const MASTER_AUDIO: u32 = 0;
pub const MASTER_BLSP_1: u32 = 1;
pub const MASTER_QDSS_BAM: u32 = 2;
pub const MASTER_QPIC: u32 = 3;
pub const MASTER_SNOC_CFG: u32 = 4;
pub const MASTER_SPMI_FETCHER: u32 = 5;
pub const MASTER_ANOC_SNOC: u32 = 6;
pub const MASTER_IPA: u32 = 7;
pub const MASTER_MEM_NOC_SNOC: u32 = 8;
pub const MASTER_MEM_NOC_PCIE_SNOC: u32 = 9;
pub const MASTER_CRYPTO_CORE_0: u32 = 10;
pub const MASTER_EMAC: u32 = 11;
pub const MASTER_IPA_PCIE: u32 = 12;
pub const MASTER_PCIE: u32 = 13;
pub const MASTER_QDSS_ETR: u32 = 14;
pub const MASTER_SDCC_1: u32 = 15;
pub const MASTER_USB3: u32 = 16;
pub const SLAVE_AOP: u32 = 17;
pub const SLAVE_AOSS: u32 = 18;
pub const SLAVE_APPSS: u32 = 19;
pub const SLAVE_AUDIO: u32 = 20;
pub const SLAVE_BLSP_1: u32 = 21;
pub const SLAVE_CLK_CTL: u32 = 22;
pub const SLAVE_CRYPTO_0_CFG: u32 = 23;
pub const SLAVE_CNOC_DDRSS: u32 = 24;
pub const SLAVE_ECC_CFG: u32 = 25;
pub const SLAVE_EMAC_CFG: u32 = 26;
pub const SLAVE_IMEM_CFG: u32 = 27;
pub const SLAVE_IPA_CFG: u32 = 28;
pub const SLAVE_CNOC_MSS: u32 = 29;
pub const SLAVE_PCIE_PARF: u32 = 30;
pub const SLAVE_PDM: u32 = 31;
pub const SLAVE_PRNG: u32 = 32;
pub const SLAVE_QDSS_CFG: u32 = 33;
pub const SLAVE_QPIC: u32 = 34;
pub const SLAVE_SDCC_1: u32 = 35;
pub const SLAVE_SNOC_CFG: u32 = 36;
pub const SLAVE_SPMI_FETCHER: u32 = 37;
pub const SLAVE_SPMI_VGI_COEX: u32 = 38;
pub const SLAVE_TCSR: u32 = 39;
pub const SLAVE_TLMM: u32 = 40;
pub const SLAVE_USB3: u32 = 41;
pub const SLAVE_USB3_PHY_CFG: u32 = 42;
pub const SLAVE_ANOC_SNOC: u32 = 43;
pub const SLAVE_SNOC_MEM_NOC_GC: u32 = 44;
pub const SLAVE_OCIMEM: u32 = 45;
pub const SLAVE_SERVICE_SNOC: u32 = 46;
pub const SLAVE_PCIE_0: u32 = 47;
pub const SLAVE_QDSS_STM: u32 = 48;
pub const SLAVE_TCU: u32 = 49;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
