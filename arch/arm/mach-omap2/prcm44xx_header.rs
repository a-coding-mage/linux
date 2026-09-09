/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * OMAP4 PRCM definitions
 *
 * Copyright (C) 2010 Texas Instruments, Inc.
 * Copyright (C) 2010 Nokia Corporation
 *
 * Paul Walmsley
 *
 * This file contains macros and functions that are common to all of
 * the PRM/CM/PRCM blocks on the OMAP4 devices: PRM, CM1, CM2,
 * PRCM_MPU, SCRM
 */

/*
 * OMAP4 PRCM partition IDs
 *
 * The numbers and order are arbitrary, but 0 is reserved for the
 * 'invalid' partition in case someone forgets to add a
 * .prcm_partition field.
 */
pub const OMAP4430_INVALID_PRCM_PARTITION: u32 = 0;
pub const OMAP4430_PRM_PARTITION: u32 = 1;
pub const OMAP4430_CM1_PARTITION: u32 = 2;
pub const OMAP4430_CM2_PARTITION: u32 = 3;
pub const OMAP4430_SCRM_PARTITION: u32 = 4;
pub const OMAP4430_PRCM_MPU_PARTITION: u32 = 5;

pub const OMAP54XX_PRM_PARTITION: u32 = 1;
pub const OMAP54XX_CM_CORE_AON_PARTITION: u32 = 2;
pub const OMAP54XX_CM_CORE_PARTITION: u32 = 3;
pub const OMAP54XX_SCRM_PARTITION: u32 = 4;
pub const OMAP54XX_PRCM_MPU_PARTITION: u32 = 5;

pub const DRA7XX_PRM_PARTITION: u32 = 1;
pub const DRA7XX_CM_CORE_AON_PARTITION: u32 = 2;
pub const DRA7XX_CM_CORE_PARTITION: u32 = 3;
pub const DRA7XX_MPU_PRCM_PARTITION: u32 = 5;

/*
 * OMAP4_MAX_PRCM_PARTITIONS: set to the highest value of the PRCM partition
 * IDs, plus one
 */
pub const OMAP4_MAX_PRCM_PARTITIONS: u32 = 6;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
