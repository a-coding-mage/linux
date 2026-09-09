/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/******************************************************************************
*******************************************************************************
**
**  Copyright (C) Sistina Software, Inc.  1997-2003  All rights reserved.
**  Copyright (C) 2004-2007 Red Hat, Inc.  All rights reserved.
**
**  This copyrighted material is made available to anyone wishing to use,
**  modify, copy, or redistribute it subject to the terms and conditions
**  of the GNU General Public License v.2.
**
*******************************************************************************
******************************************************************************/

/* This is the device interface for dlm, most users will use a library
 * interface.
 */

/* Dependencies: <linux/dlm.h> and <linux/types.h>. */

pub const DLM_USER_LVB_LEN: usize = 32;

/* Version of the device interface */
pub const DLM_DEVICE_VERSION_MAJOR: u32 = 6;
pub const DLM_DEVICE_VERSION_MINOR: u32 = 0;
pub const DLM_DEVICE_VERSION_PATCH: u32 = 2;

/* struct passed to the lock write */
#[repr(C)]
pub struct dlm_lock_params {
    pub mode: __u8,
    pub namelen: __u8,
    pub unused: __u16,
    pub flags: __u32,
    pub lkid: __u32,
    pub parent: __u32,
    pub xid: __u64,
    pub timeout: __u64,
    pub castparam: *mut core::ffi::c_void,
    pub castaddr: *mut core::ffi::c_void,
    pub bastparam: *mut core::ffi::c_void,
    pub bastaddr: *mut core::ffi::c_void,
    pub lksb: *mut dlm_lksb,
    pub lvb: [core::ffi::c_char; DLM_USER_LVB_LEN],
    pub name: [core::ffi::c_char; 0],
}

#[repr(C)]
pub struct dlm_lspace_params {
    pub flags: __u32,
    pub minor: __u32,
    pub name: [core::ffi::c_char; 0],
}

#[repr(C)]
pub struct dlm_purge_params {
    pub nodeid: __u32,
    pub pid: __u32,
}

#[repr(C)]
pub union dlm_write_request_i {
    pub lock: dlm_lock_params,
    pub lspace: dlm_lspace_params,
    pub purge: dlm_purge_params,
}

#[repr(C)]
pub struct dlm_write_request {
    pub version: [__u32; 3],
    pub cmd: __u8,
    pub is64bit: __u8,
    pub unused: [__u8; 2],
    pub i: dlm_write_request_i,
}

#[repr(C)]
pub struct dlm_device_version {
    pub version: [__u32; 3],
}

/* struct read from the "device" fd,
   consists mainly of userspace pointers for the library to use */

#[repr(C)]
pub struct dlm_lock_result {
    pub version: [__u32; 3],
    pub length: __u32,
    pub user_astaddr: *mut core::ffi::c_void,
    pub user_astparam: *mut core::ffi::c_void,
    pub user_lksb: *mut dlm_lksb,
    pub lksb: dlm_lksb,
    pub bast_mode: __u8,
    pub unused: [__u8; 3],
    /* Offsets may be zero if no data is present */
    pub lvb_offset: __u32,
}

/* Commands passed to the device */
pub const DLM_USER_LOCK: u32 = 1;
pub const DLM_USER_UNLOCK: u32 = 2;
pub const DLM_USER_QUERY: u32 = 3;
pub const DLM_USER_CREATE_LOCKSPACE: u32 = 4;
pub const DLM_USER_REMOVE_LOCKSPACE: u32 = 5;
pub const DLM_USER_PURGE: u32 = 6;
pub const DLM_USER_DEADLOCK: u32 = 7;

/* Lockspace flags */
pub const DLM_USER_LSFLG_AUTOFREE: u32 = 1;
pub const DLM_USER_LSFLG_FORCEFREE: u32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
