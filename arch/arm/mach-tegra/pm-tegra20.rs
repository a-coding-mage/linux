// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2013, NVIDIA Corporation. All rights reserved.
 */

// <linux/kernel.h>
// "pm.h"

// CONFIG_PM_SLEEP

use core::ffi::c_ulong;

#[repr(C)]
pub struct TegraLp1Iram {
    pub start_addr: *mut u32,
    pub end_addr: *mut u32,
}

extern "C" {
    pub static mut tegra20_iram_start: u32;
    pub static mut tegra20_iram_end: u32;
    pub fn tegra20_sleep_core_finish(arg: c_ulong);

    pub static mut tegra_lp1_iram: TegraLp1Iram;
    pub static mut tegra_sleep_core_finish: Option<unsafe extern "C" fn()>;
}

pub unsafe extern "C" fn tegra20_lp1_iram_hook() {
    tegra_lp1_iram.start_addr = &raw mut tegra20_iram_start;
    tegra_lp1_iram.end_addr = &raw mut tegra20_iram_end;
}

pub unsafe extern "C" fn tegra20_sleep_core_init() {
    tegra_sleep_core_finish = Some(tegra20_sleep_core_finish);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
