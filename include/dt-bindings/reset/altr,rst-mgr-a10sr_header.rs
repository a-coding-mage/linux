/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Copyright Intel Corporation (C) 2017. All Rights Reserved
 *
 * Reset binding definitions for Altera Arria10 MAX5 System Resource Chip
 *
 * Adapted from altr,rst-mgr-a10.h
 */

/* Peripheral PHY resets */
pub const A10SR_RESET_ENET_HPS: i32 = 0;
pub const A10SR_RESET_PCIE: i32 = 1;
pub const A10SR_RESET_FILE: i32 = 2;
pub const A10SR_RESET_BQSPI: i32 = 3;
pub const A10SR_RESET_USB: i32 = 4;

pub const A10SR_RESET_NUM: i32 = 5;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
