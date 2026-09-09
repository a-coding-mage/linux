/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2005-2008 Red Hat, Inc.  All rights reserved.
 */

// Dependency corresponding to <uapi/linux/dlm_plock.h>.

use core::ffi::c_int;

// Opaque types supplied by the surrounding kernel translation.
#[repr(C)]
pub struct dlm_lockspace_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file_lock {
    _private: [u8; 0],
}

extern "C" {
    pub fn dlm_posix_lock(
        lockspace: *mut dlm_lockspace_t,
        number: u64,
        file: *mut file,
        cmd: c_int,
        fl: *mut file_lock,
    ) -> c_int;

    pub fn dlm_posix_unlock(
        lockspace: *mut dlm_lockspace_t,
        number: u64,
        file: *mut file,
        fl: *mut file_lock,
    ) -> c_int;

    pub fn dlm_posix_cancel(
        lockspace: *mut dlm_lockspace_t,
        number: u64,
        file: *mut file,
        fl: *mut file_lock,
    ) -> c_int;

    pub fn dlm_posix_get(
        lockspace: *mut dlm_lockspace_t,
        number: u64,
        file: *mut file,
        fl: *mut file_lock,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
