/* SPDX-License-Identifier: GPL-2.0 */

// Translation of the C header declaration. The C header guard is omitted
// because Rust items are naturally scoped by the containing module.

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

extern "C" {
    pub fn unzstd(
        inbuf: *mut u8,
        len: c_long,
        fill: Option<unsafe extern "C" fn(*mut c_void, c_ulong) -> c_long>,
        flush: Option<unsafe extern "C" fn(*mut c_void, c_ulong) -> c_long>,
        output: *mut u8,
        pos: *mut c_long,
        error_fn: Option<unsafe extern "C" fn(*mut c_char)>,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
