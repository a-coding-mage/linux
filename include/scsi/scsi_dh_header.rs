/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Header file for SCSI device handler infrastructure.
 *
 * Modified version of patches posted by Mike Christie <michaelc@cs.wisc.edu>
 *
 * Copyright IBM Corporation, 2007
 *      Authors:
 *               Chandra Seetharaman <sekharan@us.ibm.com>
 *               Mike Anderson <andmike@linux.vnet.ibm.com>
 */

// Dependency supplied by the surrounding translation unit:
// #include <scsi/scsi_device.h>

pub const SCSI_DH_OK: i32 = 0;
/* device errors */
pub const SCSI_DH_DEV_FAILED: i32 = 1; /* generic device error */
pub const SCSI_DH_DEV_TEMP_BUSY: i32 = 2;
pub const SCSI_DH_DEV_UNSUPP: i32 = 3; /* device handler not supported */
pub const SCSI_DH_DEVICE_MAX: i32 = 4; /* max device blkerr definition */

/* transport errors */
pub const SCSI_DH_NOTCONN: i32 = SCSI_DH_DEVICE_MAX + 1;
pub const SCSI_DH_CONN_FAILURE: i32 = SCSI_DH_NOTCONN + 1;
pub const SCSI_DH_TRANSPORT_MAX: i32 = SCSI_DH_CONN_FAILURE + 1;

/* driver and generic errors */
pub const SCSI_DH_IO: i32 = SCSI_DH_TRANSPORT_MAX + 1; /* generic error */
pub const SCSI_DH_INVALID_IO: i32 = SCSI_DH_IO + 1;
pub const SCSI_DH_RETRY: i32 = SCSI_DH_INVALID_IO + 1; /* retry the req, but not immediately */
pub const SCSI_DH_IMM_RETRY: i32 = SCSI_DH_RETRY + 1; /* immediately retry the req */
pub const SCSI_DH_TIMED_OUT: i32 = SCSI_DH_IMM_RETRY + 1;
pub const SCSI_DH_RES_TEMP_UNAVAIL: i32 = SCSI_DH_TIMED_OUT + 1;
pub const SCSI_DH_DEV_OFFLINED: i32 = SCSI_DH_RES_TEMP_UNAVAIL + 1;
pub const SCSI_DH_NOMEM: i32 = SCSI_DH_DEV_OFFLINED + 1;
pub const SCSI_DH_NOSYS: i32 = SCSI_DH_NOMEM + 1;
pub const SCSI_DH_DRIVER_MAX: i32 = SCSI_DH_NOSYS + 1;

pub type ActivateComplete = unsafe extern "C" fn(*mut core::ffi::c_void, i32);

#[repr(C)]
pub struct scsi_device_handler {
    /* Used by the infrastructure */
    pub list: list_head, /* list of scsi_device_handlers */

    /* Filled by the hardware handler */
    pub module: *mut module,
    pub name: *const core::ffi::c_char,
    pub check_sense: Option<unsafe extern "C" fn(*mut scsi_device, *mut scsi_sense_hdr) -> scsi_disposition>,
    pub attach: Option<unsafe extern "C" fn(*mut scsi_device) -> i32>,
    pub detach: Option<unsafe extern "C" fn(*mut scsi_device)>,
    pub activate: Option<unsafe extern "C" fn(*mut scsi_device, ActivateComplete, *mut core::ffi::c_void) -> i32>,
    pub prep_fn: Option<unsafe extern "C" fn(*mut scsi_device, *mut request) -> blk_status_t>,
    pub set_params: Option<unsafe extern "C" fn(*mut scsi_device, *const core::ffi::c_char) -> i32>,
    pub rescan: Option<unsafe extern "C" fn(*mut scsi_device)>,
}

#[cfg(CONFIG_SCSI_DH)]
extern "C" {
    pub fn scsi_dh_activate(queue: *mut request_queue, complete: ActivateComplete, data: *mut core::ffi::c_void) -> i32;
    pub fn scsi_dh_attach(queue: *mut request_queue, name: *const core::ffi::c_char) -> i32;
    pub fn scsi_dh_attached_handler_name(queue: *mut request_queue, gfp: gfp_t) -> *const core::ffi::c_char;
    pub fn scsi_dh_set_params(queue: *mut request_queue, params: *const core::ffi::c_char) -> i32;
}

#[cfg(not(CONFIG_SCSI_DH))]
pub unsafe fn scsi_dh_activate(_req: *mut request_queue, func: ActivateComplete, data: *mut core::ffi::c_void) -> i32 {
    func(data, 0);
    0
}

#[cfg(not(CONFIG_SCSI_DH))]
pub unsafe fn scsi_dh_attach(_req: *mut request_queue, _name: *const core::ffi::c_char) -> i32 {
    SCSI_DH_NOSYS
}

#[cfg(not(CONFIG_SCSI_DH))]
pub unsafe fn scsi_dh_attached_handler_name(_q: *mut request_queue, _gfp: gfp_t) -> *const core::ffi::c_char {
    core::ptr::null()
}

#[cfg(not(CONFIG_SCSI_DH))]
pub unsafe fn scsi_dh_set_params(_req: *mut request_queue, _params: *const core::ffi::c_char) -> i32 {
    -SCSI_DH_NOSYS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
