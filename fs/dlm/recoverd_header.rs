/* SPDX-License-Identifier: GPL-2.0-only */
/******************************************************************************
*******************************************************************************
**
**  Copyright (C) Sistina Software, Inc.  1997-2003  All rights reserved.
**  Copyright (C) 2004-2005 Red Hat, Inc.  All rights reserved.
**
**
*******************************************************************************
******************************************************************************/

// C header guard: __RECOVERD_DOT_H__

#[repr(C)]
pub struct dlm_ls {
    _private: [u8; 0],
}

extern "C" {
    pub fn dlm_recoverd_stop(ls: *mut dlm_ls);
    pub fn dlm_recoverd_start(ls: *mut dlm_ls) -> ::core::ffi::c_int;
    pub fn dlm_recoverd_suspend(ls: *mut dlm_ls);
    pub fn dlm_recoverd_resume(ls: *mut dlm_ls);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
