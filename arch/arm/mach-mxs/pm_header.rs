/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2013 Freescale Semiconductor, Inc.
 */

// Original C header guard: __ARCH_MXS_PM_H

// CONFIG_PM is a build-time condition from the original source.
#[cfg(feature = "CONFIG_PM")]
unsafe extern "C" {
    pub fn mxs_pm_init();
}

#[cfg(not(feature = "CONFIG_PM"))]
pub const mxs_pm_init: Option<unsafe extern "C" fn()> = None;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
