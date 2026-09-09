// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2025, Advanced Micro Devices, Inc.
 */

// External Linux/DRM declarations supplied by the surrounding kernel sources.

use core::ffi::c_int;

pub const AMDXDNA_AUTOSUSPEND_DELAY: c_int = 5000; // milliseconds

extern "C" {
    pub fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    pub fn to_xdna_dev(data: *mut core::ffi::c_void) -> *mut amdxdna_dev;

    pub fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    pub fn pm_runtime_set_suspended(dev: *mut device);
    pub fn pm_runtime_put_autosuspend(dev: *mut device);
    pub fn pm_runtime_set_active(dev: *mut device) -> c_int;
    pub fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    pub fn pm_runtime_use_autosuspend(dev: *mut device);
    pub fn pm_runtime_allow(dev: *mut device);
    pub fn pm_runtime_get_noresume(dev: *mut device);
    pub fn pm_runtime_forbid(dev: *mut device);

    pub fn mutex_lock(mutex: *mut mutex);
    pub fn mutex_unlock(mutex: *mut mutex);
    pub fn xdna_suspend(xdna: *mut amdxdna_dev) -> c_int;
    pub fn xdna_resume(xdna: *mut amdxdna_dev) -> c_int;
    pub fn XDNA_DBG(xdna: *mut amdxdna_dev, fmt: *const u8, ...);
    pub fn XDNA_ERR(xdna: *mut amdxdna_dev, fmt: *const u8, ...);
}

pub const EOPNOTSUPP: c_int = 95;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_device {
    pub dev: *mut device,
}

#[repr(C)]
pub struct amdxdna_ops {
    pub suspend: Option<unsafe extern "C" fn(*mut amdxdna_dev) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut amdxdna_dev) -> c_int>,
}

#[repr(C)]
pub struct amdxdna_dev_info {
    pub ops: *mut amdxdna_ops,
}

#[repr(C)]
pub struct amdxdna_dev {
    pub dev_lock: mutex,
    pub dev_info: *mut amdxdna_dev_info,
    pub ddev: drm_device,
}

pub unsafe fn amdxdna_pm_suspend(dev: *mut device) -> c_int {
    let xdna = to_xdna_dev(dev_get_drvdata(dev));
    let mut ret = -EOPNOTSUPP;

    mutex_lock(&mut (*xdna).dev_lock);
    if let Some(suspend) = (*(*xdna).dev_info).ops.as_ref().and_then(|ops| ops.suspend) {
        ret = suspend(xdna);
    }
    mutex_unlock(&mut (*xdna).dev_lock);

    XDNA_DBG(xdna, b"Suspend done ret %d\0".as_ptr(), ret);
    ret
}

pub unsafe fn amdxdna_pm_resume(dev: *mut device) -> c_int {
    let xdna = to_xdna_dev(dev_get_drvdata(dev));
    let mut ret = -EOPNOTSUPP;

    mutex_lock(&mut (*xdna).dev_lock);
    if let Some(resume) = (*(*xdna).dev_info).ops.as_ref().and_then(|ops| ops.resume) {
        ret = resume(xdna);
    }
    mutex_unlock(&mut (*xdna).dev_lock);

    XDNA_DBG(xdna, b"Resume done ret %d\0".as_ptr(), ret);
    ret
}

pub unsafe fn amdxdna_pm_resume_get(xdna: *mut amdxdna_dev) -> c_int {
    let dev = (*xdna).ddev.dev;
    let ret = pm_runtime_resume_and_get(dev);
    if ret != 0 {
        XDNA_ERR(xdna, b"Resume failed: %d\0".as_ptr(), ret);
        pm_runtime_set_suspended(dev);
    }
    ret
}

pub unsafe fn amdxdna_pm_suspend_put(xdna: *mut amdxdna_dev) {
    let dev = (*xdna).ddev.dev;
    pm_runtime_put_autosuspend(dev);
}

pub unsafe fn amdxdna_pm_init(xdna: *mut amdxdna_dev) {
    let dev = (*xdna).ddev.dev;
    pm_runtime_set_active(dev);
    pm_runtime_set_autosuspend_delay(dev, AMDXDNA_AUTOSUSPEND_DELAY);
    pm_runtime_use_autosuspend(dev);
    pm_runtime_allow(dev);
    pm_runtime_put_autosuspend(dev);
}

pub unsafe fn amdxdna_pm_fini(xdna: *mut amdxdna_dev) {
    let dev = (*xdna).ddev.dev;
    pm_runtime_get_noresume(dev);
    pm_runtime_forbid(dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
