/* SPDX-License-Identifier: GPL-2.0 */
/*
 * SED key operations.
 *
 * Copyright (C) 2023 IBM Corporation
 *
 * These are the accessor functions (read/write) for SED Opal
 * keys. Specific keystores can provide overrides.
 *
 */

// Dependency supplied by the Linux kernel: EOPNOTSUPP.

// CONFIG_PSERIES_PLPKS_SED is a build-time configuration condition from the
// original header and is represented here with a Rust cfg feature.
#[cfg(feature = "CONFIG_PSERIES_PLPKS_SED")]
unsafe extern "C" {
    pub fn sed_read_key(keyname: *mut core::ffi::c_char,
                        key: *mut core::ffi::c_char,
                        keylen: *mut u_int) -> i32;
    pub fn sed_write_key(keyname: *mut core::ffi::c_char,
                         key: *mut core::ffi::c_char,
                         keylen: u_int) -> i32;
}

#[cfg(not(feature = "CONFIG_PSERIES_PLPKS_SED"))]
#[allow(non_camel_case_types)]
pub type u_int = u32;

#[cfg(feature = "CONFIG_PSERIES_PLPKS_SED")]
#[allow(non_camel_case_types)]
pub type u_int = u32;

#[cfg(not(feature = "CONFIG_PSERIES_PLPKS_SED"))]
pub unsafe fn sed_read_key(
    _keyname: *mut core::ffi::c_char,
    _key: *mut core::ffi::c_char,
    _keylen: *mut u_int,
) -> i32 {
    -(EOPNOTSUPP as i32)
}

#[cfg(not(feature = "CONFIG_PSERIES_PLPKS_SED"))]
pub unsafe fn sed_write_key(
    _keyname: *mut core::ffi::c_char,
    _key: *mut core::ffi::c_char,
    _keylen: u_int,
) -> i32 {
    -(EOPNOTSUPP as i32)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
