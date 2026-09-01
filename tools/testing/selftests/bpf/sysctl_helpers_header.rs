/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_char;

unsafe extern "C" {
    pub fn sysctl_set(
        sysctl_path: *const c_char,
        old_val: *mut c_char,
        new_val: *const c_char,
    ) -> i32;

    pub fn sysctl_set_or_fail(
        sysctl_path: *const c_char,
        old_val: *mut c_char,
        new_val: *const c_char,
    ) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
