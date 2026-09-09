/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *      intel-mid_wdt: generic Intel MID SCU watchdog driver
 *
 *      Copyright (C) 2014 Intel Corporation. All rights reserved.
 *      Contact: David Cohen <david.a.cohen@linux.intel.com>
 */

// Dependency supplied externally by the Linux platform-device subsystem.
use core::ffi::c_int;

#[repr(C)]
pub struct intel_mid_wdt_pdata {
    pub irq: c_int,
    pub probe: Option<unsafe extern "C" fn(pdev: *mut platform_device) -> c_int>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
