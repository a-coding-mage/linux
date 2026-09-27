/* SPDX-License-Identifier: GPL-2.0 */

// C declaration translated from decompress/unlzo.h.
extern "C" {
    pub fn unlzo(
        inbuf: *mut u8,
        len: kernel::ffi::c_long,
        fill: Option<unsafe extern "C" fn(*mut kernel::ffi::c_void, kernel::ffi::c_ulong) -> kernel::ffi::c_long>,
        flush: Option<unsafe extern "C" fn(*mut kernel::ffi::c_void, kernel::ffi::c_ulong) -> kernel::ffi::c_long>,
        output: *mut u8,
        pos: *mut kernel::ffi::c_long,
        error: Option<unsafe extern "C" fn(*mut kernel::ffi::c_char)>,
    ) -> kernel::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
