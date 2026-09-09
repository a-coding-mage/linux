/* SPDX-License-Identifier: GPL-2.0 */
/*
 *    Copyright IBM Corp. 1999, 2023
 */

// Dependency supplied by the surrounding kernel translation: EOPNOTSUPP.

extern "C" {
    pub fn __pfault_init() -> ::core::ffi::c_int;
    pub fn __pfault_fini();
}

// Equivalent of the build-time IS_ENABLED(CONFIG_PFAULT) condition.
#[inline]
pub unsafe fn pfault_init() -> ::core::ffi::c_int {
    if cfg!(feature = "CONFIG_PFAULT") {
        unsafe { __pfault_init() }
    } else {
        EOPNOTSUPP
    }
}

// Equivalent of the build-time IS_ENABLED(CONFIG_PFAULT) condition.
#[inline]
pub unsafe fn pfault_fini() {
    if cfg!(feature = "CONFIG_PFAULT") {
        unsafe { __pfault_fini() };
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
