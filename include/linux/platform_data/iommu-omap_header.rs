/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * omap iommu: main structures
 *
 * Copyright (C) 2008-2009 Nokia Corporation
 *
 * Written by Hiroshi DOYU <Hiroshi.DOYU@nokia.com>
 */

// Dependency supplied by the platform-device declarations.
use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct iommu_platform_data {
    pub reset_name: *const c_char,
    pub assert_reset:
        Option<unsafe extern "C" fn(pdev: *mut platform_device, name: *const c_char) -> c_int>,
    pub deassert_reset:
        Option<unsafe extern "C" fn(pdev: *mut platform_device, name: *const c_char) -> c_int>,
    pub device_enable:
        Option<unsafe extern "C" fn(pdev: *mut platform_device) -> c_int>,
    pub device_idle:
        Option<unsafe extern "C" fn(pdev: *mut platform_device) -> c_int>,
    pub set_pwrdm_constraint:
        Option<unsafe extern "C" fn(
            pdev: *mut platform_device,
            request: bool,
            pwrst: *mut u8,
        ) -> c_int>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
