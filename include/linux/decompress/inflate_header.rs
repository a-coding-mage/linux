/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header guard:
// LINUX_DECOMPRESS_INFLATE_H

use core::ffi::{c_char, c_long, c_ulong, c_void};

pub unsafe extern "C" {
    pub fn gunzip(
        inbuf: *mut u8,
        len: c_long,
        fill: Option<unsafe extern "C" fn(*mut c_void, c_ulong) -> c_long>,
        flush: Option<unsafe extern "C" fn(*mut c_void, c_ulong) -> c_long>,
        output: *mut u8,
        pos: *mut c_long,
        error_fn: Option<unsafe extern "C" fn(*mut c_char)>,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
