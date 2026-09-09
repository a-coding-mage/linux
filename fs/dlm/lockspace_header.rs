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

/* DLM_LSFL_FS
 *   The lockspace user is in the kernel (i.e. filesystem).  Enables
 *   direct bast/cast callbacks.
 *
 * internal lockspace flag - will be removed in future
 */
pub const DLM_LSFL_FS: u32 = 0x00000004;

use core::ffi::{c_char, c_int, c_void};

/* Opaque types declared by the surrounding DLM interface. */
pub enum dlm_ls {}
pub enum dlm_lockspace_ops {}

extern "C" {
    pub fn dlm_lockspace_init() -> c_int;
    pub fn dlm_lockspace_exit();
    pub fn dlm_find_lockspace_global(id: u32) -> *mut dlm_ls;
    pub fn dlm_find_lockspace_local(id: *mut c_void) -> *mut dlm_ls;
    pub fn dlm_find_lockspace_device(minor: c_int) -> *mut dlm_ls;
    pub fn dlm_put_lockspace(ls: *mut dlm_ls);
    pub fn dlm_stop_lockspaces();
    pub fn dlm_new_user_lockspace(
        name: *const c_char,
        cluster: *const c_char,
        flags: u32,
        lvblen: c_int,
        ops: *const dlm_lockspace_ops,
        ops_arg: *mut c_void,
        ops_result: *mut c_int,
        lockspace: *mut *mut dlm_lockspace_t,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
