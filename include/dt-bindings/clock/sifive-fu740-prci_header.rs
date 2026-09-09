/* SPDX-License-Identifier: (GPL-2.0 OR MIT) */
/*
 * Copyright (C) 2019 SiFive, Inc.
 * Wesley Terpstra
 * Paul Walmsley
 * Zong Li
 */

/* Header guard: __DT_BINDINGS_CLOCK_SIFIVE_FU740_PRCI_H */

/* Clock indexes for use by Device Tree data and the PRCI driver */

pub const FU740_PRCI_CLK_COREPLL: u32 = 0;
pub const FU740_PRCI_CLK_DDRPLL: u32 = 1;
pub const FU740_PRCI_CLK_GEMGXLPLL: u32 = 2;
pub const FU740_PRCI_CLK_DVFSCOREPLL: u32 = 3;
pub const FU740_PRCI_CLK_HFPCLKPLL: u32 = 4;
pub const FU740_PRCI_CLK_CLTXPLL: u32 = 5;
pub const FU740_PRCI_CLK_TLCLK: u32 = 6;
pub const FU740_PRCI_CLK_PCLK: u32 = 7;
pub const FU740_PRCI_CLK_PCIE_AUX: u32 = 8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
