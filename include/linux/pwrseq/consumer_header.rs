/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2024 Linaro Ltd.
 */

// Opaque declarations supplied by the surrounding kernel translation.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pwrseq_desc {
    _private: [u8; 0],
}

// CONFIG_POWER_SEQUENCING is a build-time condition from the C header.
// The enabled branch contains the external API declarations below.
#[cfg(feature = "CONFIG_POWER_SEQUENCING")]
extern "C" {
    pub fn pwrseq_get(dev: *mut device, target: *const core::ffi::c_char) -> *mut pwrseq_desc;
    pub fn pwrseq_put(desc: *mut pwrseq_desc);

    pub fn devm_pwrseq_get(
        dev: *mut device,
        target: *const core::ffi::c_char,
    ) -> *mut pwrseq_desc;

    pub fn pwrseq_enable(desc: *mut pwrseq_desc) -> core::ffi::c_int;
    pub fn pwrseq_disable(desc: *mut pwrseq_desc) -> core::ffi::c_int;

    pub fn pwrseq_to_device(desc: *mut pwrseq_desc) -> *mut device;
}

// CONFIG_POWER_SEQUENCING disabled fallback.  ERR_PTR and ENOSYS are supplied
// by the translated linux/err.h dependency.
#[cfg(not(feature = "CONFIG_POWER_SEQUENCING"))]
unsafe extern "C" {
    fn ERR_PTR(error: isize) -> *mut core::ffi::c_void;
}

#[cfg(not(feature = "CONFIG_POWER_SEQUENCING"))]
const ENOSYS: isize = 38;

#[cfg(not(feature = "CONFIG_POWER_SEQUENCING"))]
#[inline]
pub unsafe fn pwrseq_get(
    _dev: *mut device,
    _target: *const core::ffi::c_char,
) -> *mut pwrseq_desc {
    ERR_PTR(-ENOSYS) as *mut pwrseq_desc
}

#[cfg(not(feature = "CONFIG_POWER_SEQUENCING"))]
#[inline]
pub unsafe fn pwrseq_put(_desc: *mut pwrseq_desc) {}

#[cfg(not(feature = "CONFIG_POWER_SEQUENCING"))]
#[inline]
pub unsafe fn devm_pwrseq_get(
    _dev: *mut device,
    _target: *const core::ffi::c_char,
) -> *mut pwrseq_desc {
    ERR_PTR(-ENOSYS) as *mut pwrseq_desc
}

#[cfg(not(feature = "CONFIG_POWER_SEQUENCING"))]
#[inline]
pub unsafe fn pwrseq_enable(_desc: *mut pwrseq_desc) -> core::ffi::c_int {
    -ENOSYS as core::ffi::c_int
}

#[cfg(not(feature = "CONFIG_POWER_SEQUENCING"))]
#[inline]
pub unsafe fn pwrseq_disable(_desc: *mut pwrseq_desc) -> core::ffi::c_int {
    -ENOSYS as core::ffi::c_int
}

#[cfg(not(feature = "CONFIG_POWER_SEQUENCING"))]
#[inline]
pub unsafe fn pwrseq_to_device(_desc: *mut pwrseq_desc) -> *mut device {
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
