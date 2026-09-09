/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2011 Samsung Electronics Co., Ltd.
 *		http://www.samsung.com
 *
 * Common Header for S5PV210 machines
 */

// C header guard: __ARCH_ARM_MACH_S5PV210_COMMON_H

// Preserves the CONFIG_PM_SLEEP conditional from the source header.
#[cfg(CONFIG_PM_SLEEP)]
extern "C" {
    pub fn s5pv210_cpu_resume();
    pub fn s5pv210_pm_init();
}

#[cfg(not(CONFIG_PM_SLEEP))]
#[inline]
pub fn s5pv210_pm_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
