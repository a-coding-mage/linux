/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Mock-up PTP Hardware Clock driver for virtual network devices
 *
 * Copyright 2023 NXP
 */

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mock_phc {
    _private: [u8; 0],
}

// The following conditional preserves IS_ENABLED(CONFIG_PTP_1588_CLOCK_MOCK).
#[cfg(feature = "CONFIG_PTP_1588_CLOCK_MOCK")]
extern "C" {
    pub fn mock_phc_create(dev: *mut device) -> *mut mock_phc;
    pub fn mock_phc_destroy(phc: *mut mock_phc);
    pub fn mock_phc_index(phc: *mut mock_phc) -> i32;
}

#[cfg(not(feature = "CONFIG_PTP_1588_CLOCK_MOCK"))]
#[inline]
pub unsafe fn mock_phc_create(_dev: *mut device) -> *mut mock_phc {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_PTP_1588_CLOCK_MOCK"))]
#[inline]
pub unsafe fn mock_phc_destroy(_phc: *mut mock_phc) {}

#[cfg(not(feature = "CONFIG_PTP_1588_CLOCK_MOCK"))]
#[inline]
pub unsafe fn mock_phc_index(_phc: *mut mock_phc) -> i32 {
    -1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
