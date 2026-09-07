// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppArmor security module
 *
 * This file contains AppArmor policy loading interface function definitions.
 *
 * Copyright 2013 Canonical Ltd.
 */

// Depends on: policy.h

use std::ffi::c_void;
use std::os::raw::c_char;

// Opaque type from policy.h
extern "C" {
    pub type aa_profile;
}

// When CONFIG_SECURITY_APPARMOR_HASH is enabled:
#[cfg(feature = "CONFIG_SECURITY_APPARMOR_HASH")]
extern "C" {
    pub fn init_profile_hash() -> i32;
    pub fn aa_hash_size() -> u32;
    pub fn aa_calc_hash(data: *mut c_void, len: usize) -> *mut c_char;
    pub fn aa_calc_profile_hash(
        profile: *mut aa_profile,
        version: u32,
        start: *mut c_void,
        len: usize,
    ) -> i32;
}

// When CONFIG_SECURITY_APPARMOR_HASH is disabled:
#[cfg(not(feature = "CONFIG_SECURITY_APPARMOR_HASH"))]
#[inline]
pub fn aa_calc_hash(_data: *mut c_void, _len: usize) -> *mut c_char {
    std::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_SECURITY_APPARMOR_HASH"))]
#[inline]
pub fn aa_calc_profile_hash(
    _profile: *mut aa_profile,
    _version: u32,
    _start: *mut c_void,
    _len: usize,
) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_SECURITY_APPARMOR_HASH"))]
#[inline]
pub fn aa_hash_size() -> u32 {
    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
