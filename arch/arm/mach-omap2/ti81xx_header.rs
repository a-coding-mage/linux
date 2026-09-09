/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * This file contains the address data for various TI81XX modules.
 *
 * Copyright (C) 2010 Texas Instruments, Inc. - https://www.ti.com/
 */

pub const L4_SLOW_TI81XX_BASE: u32 = 0x4800_0000;

pub const TI81XX_SCM_BASE: u32 = 0x4814_0000;
pub const TI81XX_CTRL_BASE: u32 = TI81XX_SCM_BASE;
pub const TI81XX_PRCM_BASE: u32 = 0x4818_0000;

/*
 * Adjust TAP register base such that omap3_check_revision accesses the correct
 * TI81XX register for checking device ID (it adds 0x204 to tap base while
 * TI81XX DEVICE ID register is at offset 0x600 from control base).
 *
 * TI81XX_CONTROL_DEVICE_ID is supplied by the corresponding dependency.
 */
pub const TI81XX_TAP_BASE: u32 =
    TI81XX_CTRL_BASE + TI81XX_CONTROL_DEVICE_ID - 0x204;

pub const TI81XX_ARM_INTC_BASE: u32 = 0x4820_0000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
