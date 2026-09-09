/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2012-2019 ARM Limited (or its affiliates). */

/* \file cc_pm.h */

pub const CC_SUSPEND_TIMEOUT: i32 = 3000;

/* Opaque declarations corresponding to the external C types. */
#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

/* Preserve the source CONFIG_PM build-time condition. */
#[cfg(CONFIG_PM)]
extern "C" {
    pub static ccree_pm: dev_pm_ops;

    pub fn cc_pm_get(dev: *mut device) -> i32;
    pub fn cc_pm_put_suspend(dev: *mut device);
}

/* Preserve the source !CONFIG_PM static-inline implementations. */
#[cfg(not(CONFIG_PM))]
#[inline]
pub unsafe fn cc_pm_get(_dev: *mut device) -> i32 {
    0
}

#[cfg(not(CONFIG_PM))]
#[inline]
pub unsafe fn cc_pm_put_suspend(_dev: *mut device) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
