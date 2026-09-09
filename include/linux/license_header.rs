/* SPDX-License-Identifier: GPL-2.0-only */

use core::ffi::c_char;

extern "C" {
    fn strcmp(lhs: *const c_char, rhs: *const c_char) -> i32;
}

#[inline]
pub unsafe fn license_is_gpl_compatible(license: *const c_char) -> i32 {
    (strcmp(license, b"GPL\0".as_ptr().cast()) == 0
        || strcmp(license, b"GPL v2\0".as_ptr().cast()) == 0
        || strcmp(license, b"GPL and additional rights\0".as_ptr().cast()) == 0
        || strcmp(license, b"Dual BSD/GPL\0".as_ptr().cast()) == 0
        || strcmp(license, b"Dual MIT/GPL\0".as_ptr().cast()) == 0
        || strcmp(license, b"Dual MPL/GPL\0".as_ptr().cast()) == 0) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
