/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_ulonglong};

extern "C" {
    pub fn strtoull(
        ptr: *const c_char,
        end: *mut *mut c_char,
        base: c_int,
    ) -> c_ulonglong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
