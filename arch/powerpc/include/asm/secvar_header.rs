/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2019 IBM Corporation
 * Author: Nayna Jain
 *
 * PowerPC secure variable operations.
 */

use core::ffi::c_char;

// Translated from the Linux kernel declarations included by the original header.
pub type ssize_t = isize;

extern "C" {
    pub static secvar_ops: *const secvar_operations;
}

#[repr(C)]
pub struct secvar_operations {
    pub get: Option<unsafe extern "C" fn(
        key: *const c_char,
        key_len: u64,
        data: *mut u8,
        data_size: *mut u64,
    ) -> i32>,
    pub get_next: Option<unsafe extern "C" fn(
        key: *const c_char,
        key_len: *mut u64,
        keybufsize: u64,
    ) -> i32>,
    pub set: Option<unsafe extern "C" fn(
        key: *const c_char,
        key_len: u64,
        data: *mut u8,
        data_size: u64,
    ) -> i32>,
    pub format: Option<unsafe extern "C" fn(buf: *mut c_char, bufsize: usize) -> ssize_t>,
    pub max_size: Option<unsafe extern "C" fn(max_size: *mut u64) -> i32>,

    // NULL-terminated array of fixed variable names
    // Only used if get_next() isn't provided
    pub var_names: *const *const c_char,
}

// CONFIG_PPC_SECURE_BOOT controls which declaration is active in the C build.
#[cfg(feature = "CONFIG_PPC_SECURE_BOOT")]
extern "C" {
    pub fn set_secvar_ops(ops: *const secvar_operations) -> i32;
}

#[cfg(not(feature = "CONFIG_PPC_SECURE_BOOT"))]
#[inline]
pub unsafe fn set_secvar_ops(_ops: *const secvar_operations) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
