/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2022, Xilinx, Inc.
 * Copyright (C) 2022 - 2026, Advanced Micro Devices, Inc.
 */

// Dependency: declarations from "xlnx-versal-power.h" are supplied externally.

pub const PM_DEV_USB_1: u32 = 0x182240d7_u32;
pub const PM_DEV_FPD_SWDT_0: u32 = 0x182240db_u32;
pub const PM_DEV_FPD_SWDT_1: u32 = 0x182240dc_u32;
pub const PM_DEV_FPD_SWDT_2: u32 = 0x182240dd_u32;
pub const PM_DEV_FPD_SWDT_3: u32 = 0x182240de_u32;
pub const PM_DEV_TCM_A_0A: u32 = 0x183180cb_u32;
pub const PM_DEV_TCM_A_0B: u32 = 0x183180cc_u32;
pub const PM_DEV_TCM_A_0C: u32 = 0x183180cd_u32;
pub const PM_DEV_RPU_A_0: u32 = 0x181100bf_u32;
pub const PM_DEV_LPD_SWDT_0: u32 = 0x182240d9_u32;
pub const PM_DEV_LPD_SWDT_1: u32 = 0x182240da_u32;

// Remove Versal specific node IDs. The corresponding C preprocessor symbols
// are undefined by the original header.
// #undef PM_DEV_RPU0_0
// #undef PM_DEV_RPU0_1
// #undef PM_DEV_OCM_0
// #undef PM_DEV_OCM_1
// #undef PM_DEV_OCM_2
// #undef PM_DEV_OCM_3
// #undef PM_DEV_TCM_0_A
// #undef PM_DEV_TCM_1_A
// #undef PM_DEV_TCM_0_B
// #undef PM_DEV_TCM_1_B
// #undef PM_DEV_SWDT_FPD
// #undef PM_DEV_AI

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
