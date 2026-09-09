/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * TI PRM (Power & Reset Manager) platform data
 *
 * Copyright (C) 2019 Texas Instruments, Inc.
 *
 * Tero Kristo <t-kristo@ti.com>
 */

use core::ffi::c_char;

#[repr(C)]
pub struct clockdomain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ti_prm_platform_data {
    pub clkdm_deny_idle: Option<unsafe extern "C" fn(clkdm: *mut clockdomain)>,
    pub clkdm_allow_idle: Option<unsafe extern "C" fn(clkdm: *mut clockdomain)>,
    pub clkdm_lookup:
        Option<unsafe extern "C" fn(name: *const c_char) -> *mut clockdomain>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
