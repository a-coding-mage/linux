/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OMAP2+ WDTIMER-specific function prototypes
 *
 * Copyright (C) 2012 Texas Instruments, Inc.
 * Paul Walmsley
 */

// Translated from the Linux kernel header; C include dependencies are supplied
// by the surrounding translation environment.

/*
 * Standardized OMAP reset source bits
 *
 * This is a subset of the ones listed in arch/arm/mach-omap2/prm.h
 * and are the only ones needed in the watchdog driver.
 */
pub const OMAP_MPU_WD_RST_SRC_ID_SHIFT: u32 = 3;

/**
 * struct omap_wd_timer_platform_data - WDTIMER integration to the host SoC
 * @read_reset_sources - fn ptr for the SoC to indicate the last reset cause
 *
 * The function pointed to by @read_reset_sources must return its data
 * in a standard format - search for RST_SRC_ID_SHIFT in
 * arch/arm/mach-omap2
 */
#[repr(C)]
pub struct omap_wd_timer_platform_data {
    pub read_reset_sources: Option<unsafe extern "C" fn() -> u32>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
