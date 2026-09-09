/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * OMAP24XX Power/Reset Management register bits
 *
 * Copyright (C) 2007 Texas Instruments, Inc.
 * Copyright (C) 2007 Nokia Corporation
 *
 * Written by Paul Walmsley
 */

// Dependency from prm2xxx.h is supplied by the surrounding translation unit.

pub const OMAP24XX_EN_CORE_SHIFT: u32 = 0;
pub const OMAP24XX_FORCESTATE_MASK: u32 = 1 << 18;
pub const OMAP24XX_AUTOIDLE_MASK: u32 = 1 << 0;
pub const OMAP24XX_AUTO_EXTVOLT_MASK: u32 = 1 << 15;
pub const OMAP24XX_SETOFF_LEVEL_SHIFT: u32 = 12;
pub const OMAP24XX_MEMRETCTRL_MASK: u32 = 1 << 8;
pub const OMAP24XX_SETRET_LEVEL_SHIFT: u32 = 6;
pub const OMAP24XX_VOLT_LEVEL_SHIFT: u32 = 0;
pub const OMAP2420_CLKOUT2_EN_SHIFT: u32 = 15;
pub const OMAP2420_CLKOUT2_DIV_SHIFT: u32 = 11;
pub const OMAP2420_CLKOUT2_DIV_WIDTH: u32 = 3;
pub const OMAP2420_CLKOUT2_SOURCE_MASK: u32 = 0x3 << 8;
pub const OMAP24XX_CLKOUT_EN_SHIFT: u32 = 7;
pub const OMAP24XX_CLKOUT_DIV_SHIFT: u32 = 3;
pub const OMAP24XX_CLKOUT_DIV_WIDTH: u32 = 3;
pub const OMAP24XX_CLKOUT_SOURCE_MASK: u32 = 0x3 << 0;
pub const OMAP24XX_EMULATION_EN_SHIFT: u32 = 0;
pub const OMAP2430_PM_WKDEP_MPU_EN_MDM_SHIFT: u32 = 5;
pub const OMAP24XX_PM_WKDEP_MPU_EN_DSP_SHIFT: u32 = 2;
pub const OMAP24XX_EXTWMPU_RST_SHIFT: u32 = 6;
pub const OMAP24XX_SECU_WD_RST_SHIFT: u32 = 5;
pub const OMAP24XX_MPU_WD_RST_SHIFT: u32 = 4;
pub const OMAP24XX_SECU_VIOL_RST_SHIFT: u32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
