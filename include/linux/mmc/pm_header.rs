/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * linux/include/linux/mmc/pm.h
 *
 * Author:	Nicolas Pitre
 * Copyright:	(C) 2009 Marvell Technology Group Ltd.
 */

/*
 * These flags are used to describe power management features that
 * some cards (typically SDIO cards) might wish to benefit from when
 * the host system is being suspended.  There are several layers of
 * abstractions involved, from the host controller driver, to the MMC core
 * code, to the SDIO core code, to finally get to the actual SDIO function
 * driver.  This file is therefore used for common definitions shared across
 * all those layers.
 */

pub type mmc_pm_flag_t = ::core::ffi::c_uint;

pub const MMC_PM_KEEP_POWER: mmc_pm_flag_t = 1u32 << 0; /* preserve card power during suspend */
pub const MMC_PM_WAKE_SDIO_IRQ: mmc_pm_flag_t = 1u32 << 1; /* wake up host system on SDIO IRQ assertion */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
