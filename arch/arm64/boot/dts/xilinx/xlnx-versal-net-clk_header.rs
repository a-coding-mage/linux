/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2022, Xilinx, Inc.
 * Copyright (C) 2022 - 2026, Advanced Micro Devices, Inc.
 */

// Dependency intent: declarations from "xlnx-versal-clk.h" are supplied by
// the corresponding Rust translation and are not redefined here.

pub const CAN0_REF_2X: u32 = 0x9e;
pub const CAN1_REF_2X: u32 = 0xac;
pub const FPD_WWDT0: u32 = 0xb5;
pub const FPD_WWDT1: u32 = 0xb6;
pub const FPD_WWDT2: u32 = 0xb7;
pub const FPD_WWDT3: u32 = 0xb8;
pub const LPD_WWDT0: u32 = 0xb9;
pub const LPD_WWDT1: u32 = 0xba;
pub const ACPU_0: u32 = 0x98;
pub const ACPU_1: u32 = 0x9b;
pub const ACPU_2: u32 = 0x9a;
pub const ACPU_3: u32 = 0x99;
pub const I3C0_REF: u32 = 0x9d;
pub const I3C1_REF: u32 = 0x9f;
pub const USB1_BUS_REF: u32 = 0xae;
pub const LPD_WWDT: u32 = 0xad;

// Remove Versal specific node IDs. These names are intentionally undefined
// in this translation unit, matching the C preprocessor #undef directives.
// APU_PLL, RPU_PLL, CPM_PLL, APU_PRESRC, APU_POSTCLK, APU_PLL_OUT, APLL,
// RPU_PRESRC, RPU_POSTCLK, RPU_PLL_OUT, RPLL, CPM_PRESRC, CPM_POSTCLK,
// CPM_PLL_OUT, CPLL, APLL_TO_XPD, RPLL_TO_XPD, RCLK_PMC, RCLK_LPD, WDT,
// MUXED_IRO_DIV2, MUXED_IRO_DIV4, PSM_REF, CPM_CORE_REF, CPM_LSBUS_REF,
// CPM_DBG_REF, CPM_AUX0_REF, CPM_AUX1_REF, CPU_R5, CPU_R5_CORE, CPU_R5_OCM,
// CPU_R5_OCM2, CAN0_REF, CAN1_REF, I2C0_REF, I2C1_REF, CPM_TOPSW_REF,
// USB3_DUAL_REF, MUXED_IRO, PL_EXT, PL_LB, MIO_50_OR_51, MIO_24_OR_25.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
