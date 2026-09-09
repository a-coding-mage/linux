/* SPDX-License-Identifier: GPL-2.0 */

// C declaration translated from decompress/unlzo.h.
extern "C" {
    pub fn unlzo(
        inbuf: *mut u8,
        len: core::ffi::c_long,
        fill: Option<unsafe extern "C" fn(*mut core::ffi::c_void, core::ffi::c_ulong) -> core::ffi::c_long>,
        flush: Option<unsafe extern "C" fn(*mut core::ffi::c_void, core::ffi::c_ulong) -> core::ffi::c_long>,
        output: *mut u8,
        pos: *mut core::ffi::c_long,
        error: Option<unsafe extern "C" fn(*mut core::ffi::c_char)>,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
