/* SPDX-License-Identifier: GPL-2.0 */

// <linux/types.h>: bool
// <linux/compiler.h>: __pure

#[cfg(not(CONFIG_RUST))]
use core::ffi::c_char;
#[cfg(CONFIG_RUST)]
use kernel::ffi::c_char;

unsafe extern "C" {
    /// Matches two readable NUL-terminated byte strings.
    pub fn glob_match(pat: *const c_char, str_: *const c_char) -> bool;
    /// Matches at most `len` input bytes, stopping at an earlier NUL.
    pub fn glob_match_len(pat: *const c_char, str_: *const c_char, len: usize) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
