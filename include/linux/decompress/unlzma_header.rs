/* SPDX-License-Identifier: GPL-2.0 */

use std::ffi::{c_char, c_long, c_ulong, c_void};

extern "C" {
    pub fn unlzma(
        input: *mut u8,
        input_len: c_long,
        fill: Option<unsafe extern "C" fn(*mut c_void, c_ulong) -> c_long>,
        flush: Option<unsafe extern "C" fn(*mut c_void, c_ulong) -> c_long>,
        output: *mut u8,
        posp: *mut c_long,
        error: Option<unsafe extern "C" fn(*mut c_char)>,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
