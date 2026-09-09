/* SPDX-License-Identifier: GPL-2.0-only */
/******************************************************************************
*******************************************************************************
**
**  Copyright (C) 2005-2007 Red Hat, Inc.  All rights reserved.
**
**
*******************************************************************************
*******************************************************************************/

// C header guard: __REQUESTQUEUE_DOT_H__

// These types are defined by dependent translation units.
#[repr(C)]
pub struct dlm_ls {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dlm_message {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn dlm_add_requestqueue(
        ls: *mut dlm_ls,
        nodeid: core::ffi::c_int,
        ms: *const dlm_message,
    );

    pub fn dlm_process_requestqueue(ls: *mut dlm_ls) -> core::ffi::c_int;

    pub fn dlm_wait_requestqueue(ls: *mut dlm_ls);

    pub fn dlm_purge_requestqueue(ls: *mut dlm_ls);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
