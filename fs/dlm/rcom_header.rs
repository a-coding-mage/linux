/* SPDX-License-Identifier: GPL-2.0-only */
/******************************************************************************
*******************************************************************************
**
**  Copyright (C) Sistina Software, Inc.  1997-2003  All rights reserved.
**  Copyright (C) 2005-2007 Red Hat, Inc.  All rights reserved.
**
**
*******************************************************************************
******************************************************************************/

// C header guard: __RCOM_DOT_H__

// Opaque types declared in other headers.
#[repr(C)]
pub struct dlm_ls {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dlm_rsb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dlm_lkb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dlm_rcom {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn dlm_rcom_status(
        ls: *mut dlm_ls,
        nodeid: ::core::ffi::c_int,
        status_flags: u32,
        seq: u64,
    ) -> ::core::ffi::c_int;

    pub fn dlm_rcom_names(
        ls: *mut dlm_ls,
        nodeid: ::core::ffi::c_int,
        last_name: *mut ::core::ffi::c_char,
        last_len: ::core::ffi::c_int,
        seq: u64,
    ) -> ::core::ffi::c_int;

    pub fn dlm_send_rcom_lookup(
        r: *mut dlm_rsb,
        dir_nodeid: ::core::ffi::c_int,
        seq: u64,
    ) -> ::core::ffi::c_int;

    pub fn dlm_send_rcom_lock(
        r: *mut dlm_rsb,
        lkb: *mut dlm_lkb,
        seq: u64,
    ) -> ::core::ffi::c_int;

    pub fn dlm_receive_rcom(
        ls: *mut dlm_ls,
        rc: *const dlm_rcom,
        nodeid: ::core::ffi::c_int,
    );

    pub fn dlm_send_ls_not_ready(
        nodeid: ::core::ffi::c_int,
        rc_in: *const dlm_rcom,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
