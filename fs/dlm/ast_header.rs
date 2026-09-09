/* SPDX-License-Identifier: GPL-2.0-only */
/******************************************************************************
*******************************************************************************
**
**  Copyright (C) 2005-2010 Red Hat, Inc.  All rights reserved.
**
**
*******************************************************************************
******************************************************************************/

// C header guard: __ASTD_DOT_H__

use core::ffi::c_int;

// Opaque types declared by dependent headers.
pub enum dlm_lkb {}
pub enum dlm_ls {}
pub enum dlm_callback {}

extern "C" {
    pub fn dlm_may_skip_callback(
        lkb: *mut dlm_lkb,
        flags: u32,
        mode: c_int,
        status: c_int,
        sbflags: u32,
        copy_lvb: *mut c_int,
    ) -> bool;

    pub fn dlm_get_cb(
        lkb: *mut dlm_lkb,
        flags: u32,
        mode: c_int,
        status: c_int,
        sbflags: u32,
        cb: *mut *mut dlm_callback,
    ) -> c_int;

    pub fn dlm_add_cb(
        lkb: *mut dlm_lkb,
        flags: u32,
        mode: c_int,
        status: c_int,
        sbflags: u32,
    );

    pub fn dlm_callback_start(ls: *mut dlm_ls) -> c_int;
    pub fn dlm_callback_stop(ls: *mut dlm_ls);
    pub fn dlm_callback_suspend(ls: *mut dlm_ls);
    pub fn dlm_callback_resume(ls: *mut dlm_ls);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
