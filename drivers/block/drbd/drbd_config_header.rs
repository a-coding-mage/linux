/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * drbd_config.h
 * DRBD's compile time configuration.
 */

unsafe extern "C" {
    pub fn drbd_buildtag() -> *const core::ffi::c_char;
}

pub const REL_VERSION: &str = "8.4.11";
pub const PRO_VERSION_MIN: i32 = 86;
pub const PRO_VERSION_MAX: i32 = 101;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
