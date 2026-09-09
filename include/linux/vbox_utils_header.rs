/* SPDX-License-Identifier: (GPL-2.0 OR CDDL-1.0) */
/* Copyright (C) 2006-2016 Oracle Corporation */

/* Translated from vbox_utils.h. */

use core::ffi::{c_char, c_int};

/* Supplied by the corresponding VirtualBox VMMDev types dependency. */
use crate::{vmmdev_hgcm_function_parameter, vmmdev_hgcm_service_location};

#[repr(C)]
pub struct vbg_dev {
    _private: [u8; 0],
}

/**
 * vboxguest logging functions, these log both to the backdoor and call
 * the equivalent kernel pr_foo function.
 */
extern "C" {
    pub fn vbg_info(fmt: *const c_char, ...);
    pub fn vbg_warn(fmt: *const c_char, ...);
    pub fn vbg_err(fmt: *const c_char, ...);
    pub fn vbg_err_ratelimited(fmt: *const c_char, ...);

    /* Only use backdoor logging for non-dynamic debug builds.
     * In other configurations, the C macro maps vbg_debug to pr_debug.
     */
    pub fn vbg_debug(fmt: *const c_char, ...);

    pub fn vbg_hgcm_connect(
        gdev: *mut vbg_dev,
        requestor: u32,
        loc: *mut vmmdev_hgcm_service_location,
        client_id: *mut u32,
        vbox_status: *mut c_int,
    ) -> c_int;

    pub fn vbg_hgcm_disconnect(
        gdev: *mut vbg_dev,
        requestor: u32,
        client_id: u32,
        vbox_status: *mut c_int,
    ) -> c_int;

    pub fn vbg_hgcm_call(
        gdev: *mut vbg_dev,
        requestor: u32,
        client_id: u32,
        function: u32,
        timeout_ms: u32,
        parms: *mut vmmdev_hgcm_function_parameter,
        parm_count: u32,
        vbox_status: *mut c_int,
    ) -> c_int;

    /**
     * Convert a VirtualBox status code to a standard Linux kernel return value.
     * Return: 0 or negative errno value.
     * @rc:             VirtualBox status code to convert.
     */
    pub fn vbg_status_code_to_errno(rc: c_int) -> c_int;

    /**
     * Helper for the vboxsf driver to get a reference to the guest device.
     * Return: a pointer to the gdev; or a ERR_PTR value on error.
     */
    pub fn vbg_get_gdev() -> *mut vbg_dev;

    /**
     * Helper for the vboxsf driver to put a guest device reference.
     * @gdev:           Reference returned by vbg_get_gdev to put.
     */
    pub fn vbg_put_gdev(gdev: *mut vbg_dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
