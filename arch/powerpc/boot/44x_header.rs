/* SPDX-License-Identifier: GPL-2.0 */
/*
 * PowerPC 44x related functions
 *
 * Copyright 2007 David Gibson, IBM Corporation.
 */

extern "C" {
    pub fn ebony_init(mac0: *mut core::ffi::c_void, mac1: *mut core::ffi::c_void);
    pub fn bamboo_init(mac0: *mut core::ffi::c_void, mac1: *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
