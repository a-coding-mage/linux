/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020-2024 Intel Corporation
 */

// Dependencies corresponding to the included Linux kernel declarations are
// supplied by the surrounding translation unit.

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct ivpu_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct delayed_work {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rw_semaphore {
    _private: [u8; 0],
}

#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ivpu_pm_info {
    pub vdev: *mut ivpu_device,
    pub job_timeout_work: delayed_work,
    pub recovery_work: work_struct,
    pub reset_lock: rw_semaphore,
    pub reset_counter: atomic_t,
    pub reset_pending: atomic_t,
    pub engine_reset_counter: atomic_t,
    pub dct_active_percent: u8,
}

unsafe extern "C" {
    pub fn ivpu_pm_init(vdev: *mut ivpu_device);
    pub fn ivpu_pm_enable(vdev: *mut ivpu_device);
    pub fn ivpu_pm_disable(vdev: *mut ivpu_device);
    pub fn ivpu_pm_disable_recovery(vdev: *mut ivpu_device);

    pub fn ivpu_pm_suspend_cb(dev: *mut device) -> c_int;
    pub fn ivpu_pm_resume_cb(dev: *mut device) -> c_int;
    pub fn ivpu_pm_runtime_suspend_cb(dev: *mut device) -> c_int;
    pub fn ivpu_pm_runtime_resume_cb(dev: *mut device) -> c_int;

    pub fn ivpu_pm_reset_prepare_cb(pdev: *mut pci_dev);
    pub fn ivpu_pm_reset_done_cb(pdev: *mut pci_dev);

    pub fn ivpu_rpm_get(vdev: *mut ivpu_device) -> c_int;
    pub fn ivpu_rpm_put(vdev: *mut ivpu_device);

    pub fn ivpu_pm_trigger_recovery(vdev: *mut ivpu_device, reason: *const c_char);
    pub fn ivpu_start_job_timeout_detection(vdev: *mut ivpu_device);
    pub fn ivpu_stop_job_timeout_detection(vdev: *mut ivpu_device);

    pub fn ivpu_pm_dct_init(vdev: *mut ivpu_device) -> c_int;
    pub fn ivpu_pm_dct_enable(vdev: *mut ivpu_device, active_percent: u8) -> c_int;
    pub fn ivpu_pm_dct_disable(vdev: *mut ivpu_device) -> c_int;
    pub fn ivpu_pm_irq_dct_work_fn(work: *mut work_struct);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
