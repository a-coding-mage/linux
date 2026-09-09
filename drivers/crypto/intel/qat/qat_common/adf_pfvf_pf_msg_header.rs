/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2021 Intel Corporation */

// Dependency supplied by adf_accel_devices.h in the C source.
use crate::adf_accel_dev;

// CONFIG_PCI_IOV is a build-time C configuration condition.  The equivalent
// Rust configuration feature is used here to preserve the conditional API.
#[cfg(feature = "CONFIG_PCI_IOV")]
extern "C" {
    pub fn adf_pf2vf_notify_restarting(accel_dev: *mut adf_accel_dev);
    pub fn adf_pf2vf_wait_for_restarting_complete(accel_dev: *mut adf_accel_dev);
    pub fn adf_pf2vf_notify_restarted(accel_dev: *mut adf_accel_dev);
    pub fn adf_pf2vf_notify_fatal_error(accel_dev: *mut adf_accel_dev);
}

#[cfg(not(feature = "CONFIG_PCI_IOV"))]
#[inline]
pub fn adf_pf2vf_notify_restarting(_accel_dev: *mut adf_accel_dev) {}

#[cfg(not(feature = "CONFIG_PCI_IOV"))]
#[inline]
pub fn adf_pf2vf_wait_for_restarting_complete(_accel_dev: *mut adf_accel_dev) {}

#[cfg(not(feature = "CONFIG_PCI_IOV"))]
#[inline]
pub fn adf_pf2vf_notify_restarted(_accel_dev: *mut adf_accel_dev) {}

#[cfg(not(feature = "CONFIG_PCI_IOV"))]
#[inline]
pub fn adf_pf2vf_notify_fatal_error(_accel_dev: *mut adf_accel_dev) {}

pub type adf_pf2vf_blkmsg_provider = unsafe extern "C" fn(
    accel_dev: *mut adf_accel_dev,
    buffer: *mut u8,
    compat: u8,
) -> i32;

extern "C" {
    pub fn adf_pf_capabilities_msg_provider(
        accel_dev: *mut adf_accel_dev,
        buffer: *mut u8,
        comapt: u8,
    ) -> i32;

    pub fn adf_pf_ring_to_svc_msg_provider(
        accel_dev: *mut adf_accel_dev,
        buffer: *mut u8,
        comapt: u8,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
