/* SPDX-License-Identifier: 0BSD */

/*
 * Wrapper for decompressing XZ-compressed kernel, initramfs, and initrd
 *
 * Author: Lasse Collin <lasse.collin@tukaani.org>
 */

use core::ffi::c_void;
use std::os::raw::{c_char, c_int, c_long, c_ulong};

unsafe extern "C" {
    pub fn unxz(
        input: *mut u8,
        in_size: c_long,
        fill: Option<unsafe extern "C" fn(dest: *mut c_void, size: c_ulong) -> c_long>,
        flush: Option<unsafe extern "C" fn(src: *mut c_void, size: c_ulong) -> c_long>,
        out: *mut u8,
        in_used: *mut c_long,
        error: Option<unsafe extern "C" fn(x: *mut c_char)>,
    ) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
