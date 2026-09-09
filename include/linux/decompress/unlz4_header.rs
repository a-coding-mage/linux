/* SPDX-License-Identifier: GPL-2.0 */

// Original header guard: DECOMPRESS_UNLZ4_H

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

extern "C" {
    pub fn unlz4(
        inbuf: *mut u8,
        len: c_long,
        fill: Option<unsafe extern "C" fn(*mut c_void, c_ulong) -> c_long>,
        flush: Option<unsafe extern "C" fn(*mut c_void, c_ulong) -> c_long>,
        output: *mut u8,
        pos: *mut c_long,
        error: Option<unsafe extern "C" fn(*mut c_char)>,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
