/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * This file contains the address info for various AM33XX modules.
 *
 * Copyright (C) 2011 Texas Instruments, Inc. - https://www.ti.com/
 */

// Original C header guard: __ASM_ARCH_AM33XX_H

pub const L4_SLOW_AM33XX_BASE: u32 = 0x4800_0000;

pub const AM33XX_SCM_BASE: u32 = 0x44E1_0000;
pub const AM33XX_CTRL_BASE: u32 = AM33XX_SCM_BASE;
pub const AM33XX_PRCM_BASE: u32 = 0x44E0_0000;
pub const AM43XX_PRCM_BASE: u32 = 0x44DF_0000;
pub const AM33XX_TAP_BASE: u32 = AM33XX_CTRL_BASE + 0x3FC;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
