/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2025, Advanced Micro Devices, Inc.
 */

// Dependency supplied by amdxdna_pci_drv.h.

use core::ffi::c_int;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amdxdna_dev {
    pub dev_lock: mutex,
}

unsafe extern "C" {
    pub fn amdxdna_pm_suspend(dev: *mut device) -> c_int;
    pub fn amdxdna_pm_resume(dev: *mut device) -> c_int;
    pub fn amdxdna_pm_resume_get(xdna: *mut amdxdna_dev) -> c_int;
    pub fn amdxdna_pm_suspend_put(xdna: *mut amdxdna_dev);
    pub fn amdxdna_pm_init(xdna: *mut amdxdna_dev);
    pub fn amdxdna_pm_fini(xdna: *mut amdxdna_dev);

    fn mutex_unlock(lock: *mut mutex);
    fn mutex_lock(lock: *mut mutex);
}

#[inline]
pub unsafe fn amdxdna_pm_resume_get_locked(xdna: *mut amdxdna_dev) -> c_int {
    let ret: c_int;

    mutex_unlock(core::ptr::addr_of_mut!((*xdna).dev_lock));
    ret = amdxdna_pm_resume_get(xdna);
    mutex_lock(core::ptr::addr_of_mut!((*xdna).dev_lock));

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
