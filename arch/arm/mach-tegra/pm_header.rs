/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2010 Google, Inc.
 * Copyright (c) 2010-2012 NVIDIA Corporation. All rights reserved.
 *
 * Author:
 *	Colin Cross <ccross@google.com>
 */

use core::ffi::c_void;
use core::ffi::c_ulong;

#[repr(C)]
pub struct tegra_lp1_iram {
    pub start_addr: *mut c_void,
    pub end_addr: *mut c_void,
}

unsafe extern "C" {
    pub static mut tegra_lp1_iram: tegra_lp1_iram;
    pub static mut tegra_sleep_core_finish: Option<unsafe extern "C" fn(v2p: c_ulong)>;

    pub fn tegra20_lp1_iram_hook();
    pub fn tegra20_sleep_core_init();
    pub fn tegra30_lp1_iram_hook();
    pub fn tegra30_sleep_core_init();

    pub static mut tegra_tear_down_cpu: Option<unsafe extern "C" fn()>;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
