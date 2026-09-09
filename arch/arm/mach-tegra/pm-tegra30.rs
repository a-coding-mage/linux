// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2013, NVIDIA Corporation. All rights reserved.
 */

// Dependency supplied by linux/kernel.h and pm.h in the surrounding tree.

#[cfg(feature = "CONFIG_PM_SLEEP")]
extern "C" {
    static mut tegra30_iram_start: u32;
    static mut tegra30_iram_end: u32;
    fn tegra30_sleep_core_finish(arg: ::core::ffi::c_ulong);
}

#[cfg(feature = "CONFIG_PM_SLEEP")]
#[repr(C)]
pub struct TegraLp1Iram {
    pub start_addr: *mut u32,
    pub end_addr: *mut u32,
}

#[cfg(feature = "CONFIG_PM_SLEEP")]
extern "C" {
    static mut tegra_lp1_iram: TegraLp1Iram;
    static mut tegra_sleep_core_finish: unsafe extern "C" fn(::core::ffi::c_ulong);
}

#[cfg(feature = "CONFIG_PM_SLEEP")]
pub unsafe extern "C" fn tegra30_lp1_iram_hook() {
    tegra_lp1_iram.start_addr = &raw mut tegra30_iram_start;
    tegra_lp1_iram.end_addr = &raw mut tegra30_iram_end;
}

#[cfg(feature = "CONFIG_PM_SLEEP")]
pub unsafe extern "C" fn tegra30_sleep_core_init() {
    tegra_sleep_core_finish = tegra30_sleep_core_finish;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
