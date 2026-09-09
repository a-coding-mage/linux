/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * dlmdebug.h
 *
 * Copyright (C) 2008 Oracle.  All rights reserved.
 */

// C header guard: DLMDEBUG_H

use core::ffi::c_char;

#[repr(C)]
pub struct dlm_master_list_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dlm_ctxt {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dlm_lock_resource {
    _private: [u8; 0],
}

extern "C" {
    pub fn dlm_print_one_mle(mle: *mut dlm_master_list_entry);
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
#[repr(C)]
pub struct debug_lockres {
    pub dl_len: i32,
    pub dl_buf: *mut c_char,
    pub dl_ctxt: *mut dlm_ctxt,
    pub dl_res: *mut dlm_lock_resource,
}

#[cfg(feature = "CONFIG_DEBUG_FS")]
extern "C" {
    pub fn dlm_debug_init(dlm: *mut dlm_ctxt);
    pub fn dlm_create_debugfs_subroot(dlm: *mut dlm_ctxt);
    pub fn dlm_destroy_debugfs_subroot(dlm: *mut dlm_ctxt);
    pub fn dlm_create_debugfs_root();
    pub fn dlm_destroy_debugfs_root();
}

// CONFIG_DEBUG_FS disabled: the C header provides empty static inline stubs.
#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub unsafe fn dlm_debug_init(_dlm: *mut dlm_ctxt) {}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub unsafe fn dlm_create_debugfs_subroot(_dlm: *mut dlm_ctxt) {}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub unsafe fn dlm_destroy_debugfs_subroot(_dlm: *mut dlm_ctxt) {}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub unsafe fn dlm_create_debugfs_root() {}

#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
#[inline]
pub unsafe fn dlm_destroy_debugfs_root() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
