/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2014 NVIDIA Corporation
 */

// <linux/errno.h>

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum tegra_suspend_mode {
    TEGRA_SUSPEND_NONE = 0,
    TEGRA_SUSPEND_LP2, /* CPU voltage off */
    TEGRA_SUSPEND_LP1, /* CPU voltage off, DRAM self-refresh */
    TEGRA_SUSPEND_LP0, /* CPU + core voltage off, DRAM self-refresh */
    TEGRA_MAX_SUSPEND_MODE,
    TEGRA_SUSPEND_NOT_READY,
}

// Equivalent of:
// #if defined(CONFIG_PM_SLEEP) && defined(CONFIG_ARM) && defined(CONFIG_ARCH_TEGRA)
#[cfg(all(CONFIG_PM_SLEEP, CONFIG_ARM, CONFIG_ARCH_TEGRA))]
extern "C" {
    pub fn tegra_pm_validate_suspend_mode(mode: tegra_suspend_mode) -> tegra_suspend_mode;

    /* low-level resume entry point */
    pub fn tegra_resume();

    pub fn tegra30_pm_secondary_cpu_suspend(arg: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn tegra_pm_clear_cpu_in_lp2();
    pub fn tegra_pm_set_cpu_in_lp2();
    pub fn tegra_pm_enter_lp2() -> ::core::ffi::c_int;
    pub fn tegra_pm_park_secondary_cpu(cpu: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn tegra_pm_init_suspend();
}

#[cfg(not(all(CONFIG_PM_SLEEP, CONFIG_ARM, CONFIG_ARCH_TEGRA)))]
#[inline]
pub fn tegra_pm_validate_suspend_mode(mode: tegra_suspend_mode) -> tegra_suspend_mode {
    let _ = mode;
    TEGRA_SUSPEND_NONE
}

#[cfg(not(all(CONFIG_PM_SLEEP, CONFIG_ARM, CONFIG_ARCH_TEGRA)))]
#[inline]
pub fn tegra_resume() {}

#[cfg(not(all(CONFIG_PM_SLEEP, CONFIG_ARM, CONFIG_ARCH_TEGRA)))]
#[inline]
pub fn tegra30_pm_secondary_cpu_suspend(arg: ::core::ffi::c_ulong) -> ::core::ffi::c_int {
    let _ = arg;
    -ENOTSUPP
}

#[cfg(not(all(CONFIG_PM_SLEEP, CONFIG_ARM, CONFIG_ARCH_TEGRA)))]
#[inline]
pub fn tegra_pm_clear_cpu_in_lp2() {}

#[cfg(not(all(CONFIG_PM_SLEEP, CONFIG_ARM, CONFIG_ARCH_TEGRA)))]
#[inline]
pub fn tegra_pm_set_cpu_in_lp2() {}

#[cfg(not(all(CONFIG_PM_SLEEP, CONFIG_ARM, CONFIG_ARCH_TEGRA)))]
#[inline]
pub fn tegra_pm_enter_lp2() -> ::core::ffi::c_int {
    -ENOTSUPP
}

#[cfg(not(all(CONFIG_PM_SLEEP, CONFIG_ARM, CONFIG_ARCH_TEGRA)))]
#[inline]
pub fn tegra_pm_park_secondary_cpu(cpu: ::core::ffi::c_ulong) -> ::core::ffi::c_int {
    let _ = cpu;
    -ENOTSUPP
}

#[cfg(not(all(CONFIG_PM_SLEEP, CONFIG_ARM, CONFIG_ARCH_TEGRA)))]
#[inline]
pub fn tegra_pm_init_suspend() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
