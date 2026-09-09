/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2014 Samsung Electronics Co., Ltd.
 *              http://www.samsung.com
 */

use core::ffi::c_int;

#[repr(C)]
pub struct cpuidle_exynos_data {
    pub cpu0_enter_aftr: Option<unsafe extern "C" fn() -> c_int>,
    pub cpu1_powerdown: Option<unsafe extern "C" fn() -> c_int>,
    pub pre_enter_aftr: Option<unsafe extern "C" fn()>,
    pub post_enter_aftr: Option<unsafe extern "C" fn()>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
