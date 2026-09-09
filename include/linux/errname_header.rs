/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: <linux/stddef.h>

use core::ffi::{c_char, c_int};

// Corresponds to CONFIG_SYMBOLIC_ERRNAME.
#[cfg(CONFIG_SYMBOLIC_ERRNAME)]
unsafe extern "C" {
    pub fn errname(err: c_int) -> *const c_char;
}

#[cfg(not(CONFIG_SYMBOLIC_ERRNAME))]
#[inline]
pub fn errname(_err: c_int) -> *const c_char {
    core::ptr::null()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
