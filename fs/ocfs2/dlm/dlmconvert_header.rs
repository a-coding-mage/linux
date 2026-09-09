/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * dlmconvert.h
 *
 * Copyright (C) 2004 Oracle.  All rights reserved.
 */

// Header guard: DLMCONVERT_H

extern "C" {
    pub fn dlmconvert_master(
        dlm: *mut dlm_ctxt,
        res: *mut dlm_lock_resource,
        lock: *mut dlm_lock,
        flags: ::core::ffi::c_int,
        type_: ::core::ffi::c_int,
    ) -> dlm_status;

    pub fn dlmconvert_remote(
        dlm: *mut dlm_ctxt,
        res: *mut dlm_lock_resource,
        lock: *mut dlm_lock,
        flags: ::core::ffi::c_int,
        type_: ::core::ffi::c_int,
    ) -> dlm_status;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
