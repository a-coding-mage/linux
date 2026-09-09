/* SPDX-License-Identifier: (GPL-2.0 OR MIT) */
/*
 * Copyright (C) 2018-2019 SiFive, Inc.
 * Wesley Terpstra
 * Paul Walmsley
 */

/* Clock indexes for use by Device Tree data and the PRCI driver */

pub const FU540_PRCI_CLK_COREPLL: i32 = 0;
pub const FU540_PRCI_CLK_DDRPLL: i32 = 1;
pub const FU540_PRCI_CLK_GEMGXLPLL: i32 = 2;
pub const FU540_PRCI_CLK_TLCLK: i32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
