/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2021 Intel Corporation */

// The C header selects these declarations when CONFIG_PCI_IOV is defined.
// This Rust translation uses the corresponding feature configuration.

#[allow(non_camel_case_types)]
pub struct adf_accel_dev;

#[cfg(feature = "CONFIG_PCI_IOV")]
extern "C" {
    pub fn adf_vf2pf_notify_init(accel_dev: *mut adf_accel_dev) -> ::core::ffi::c_int;
    pub fn adf_vf2pf_notify_shutdown(accel_dev: *mut adf_accel_dev);
    pub fn adf_vf2pf_notify_restart_complete(accel_dev: *mut adf_accel_dev);
    pub fn adf_vf2pf_request_version(accel_dev: *mut adf_accel_dev) -> ::core::ffi::c_int;
    pub fn adf_vf2pf_get_capabilities(accel_dev: *mut adf_accel_dev) -> ::core::ffi::c_int;
    pub fn adf_vf2pf_get_ring_to_svc(accel_dev: *mut adf_accel_dev) -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_PCI_IOV"))]
#[inline]
pub fn adf_vf2pf_notify_init(_accel_dev: *mut adf_accel_dev) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_PCI_IOV"))]
#[inline]
pub fn adf_vf2pf_notify_shutdown(_accel_dev: *mut adf_accel_dev) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
