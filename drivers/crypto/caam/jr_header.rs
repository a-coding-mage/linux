/* SPDX-License-Identifier: GPL-2.0 */
/*
 * CAAM public-level include definitions for the JobR backend
 *
 * Copyright 2008-2011 Freescale Semiconductor, Inc.
 */

/* Prototypes for backend-level services exposed to APIs */

// Opaque type supplied by the surrounding dependency context.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

extern "C" {
    pub fn caam_jr_alloc() -> *mut device;
    pub fn caam_jr_free(rdev: *mut device);
    pub fn caam_jr_enqueue(
        dev: *mut device,
        desc: *mut u32,
        cbk: Option<unsafe extern "C" fn(
            dev: *mut device,
            desc: *mut u32,
            status: u32,
            areq: *mut core::ffi::c_void,
        )>,
        areq: *mut core::ffi::c_void,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
