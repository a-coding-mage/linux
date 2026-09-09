/* SPDX-License-Identifier: GPL-2.0-only */
/*:
 * Address mappings and base address for OMAP5 interconnects
 * and peripherals.
 *
 * Copyright (C) 2012 Texas Instruments
 *	Santosh Shilimkar <santosh.shilimkar@ti.com>
 *	Sricharan <r.sricharan@ti.com>
 */

/*
 * Please place only base defines here and put the rest in device
 * specific headers.
 */
pub const L4_54XX_BASE: usize = 0x4a000000;
pub const L4_WK_54XX_BASE: usize = 0x4ae00000;
pub const L4_PER_54XX_BASE: usize = 0x48000000;
pub const L3_54XX_BASE: usize = 0x44000000;
pub const OMAP54XX_32KSYNCT_BASE: usize = 0x4ae04000;
pub const OMAP54XX_CM_CORE_AON_BASE: usize = 0x4a004000;
pub const OMAP54XX_CM_CORE_BASE: usize = 0x4a008000;
pub const OMAP54XX_PRM_BASE: usize = 0x4ae06000;
pub const OMAP54XX_PRCM_MPU_BASE: usize = 0x48243000;
pub const OMAP54XX_SCM_BASE: usize = 0x4a002000;
pub const OMAP54XX_CTRL_BASE: usize = 0x4a002800;
pub const OMAP54XX_SAR_RAM_BASE: usize = 0x4ae26000;

/* DRA7 specific base addresses */
pub const L3_MAIN_SN_DRA7XX_BASE: usize = 0x44000000;
pub const L4_PER1_DRA7XX_BASE: usize = 0x48000000;
pub const L4_CFG_MPU_DRA7XX_BASE: usize = 0x48210000;
pub const L4_PER2_DRA7XX_BASE: usize = 0x48400000;
pub const L4_PER3_DRA7XX_BASE: usize = 0x48800000;
pub const L4_CFG_DRA7XX_BASE: usize = 0x4A000000;
pub const L4_WKUP_DRA7XX_BASE: usize = 0x4ae00000;
pub const DRA7XX_CM_CORE_AON_BASE: usize = 0x4a005000;
pub const DRA7XX_CTRL_BASE: usize = 0x4a003400;
pub const DRA7XX_TAP_BASE: usize = 0x4ae0c000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
