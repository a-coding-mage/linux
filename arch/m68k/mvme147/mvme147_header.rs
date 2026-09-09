/* SPDX-License-Identifier: GPL-2.0-only */

use core::ffi::{c_char, c_uint};

#[repr(C)]
pub struct console {
    _private: [u8; 0],
}

/* config.c */
unsafe extern "C" {
    pub fn mvme147_scc_write(co: *mut console, str_: *const c_char, count: c_uint);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
