/* SPDX-License-Identifier: GPL-2.0 */

// <linux/types.h>: bool
// <linux/compiler.h>: __pure

unsafe extern "C" {
    pub fn glob_match(pat: *const core::ffi::c_char, str_: *const core::ffi::c_char) -> bool;
    pub fn glob_match_len(
        pat: *const core::ffi::c_char,
        str_: *const core::ffi::c_char,
        len: usize,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
