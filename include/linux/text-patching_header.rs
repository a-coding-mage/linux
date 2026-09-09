/* SPDX-License-Identifier: GPL-2.0 */

// Dependency provided by <asm/text-patching.h> in the original source.

use core::ffi::c_void;

unsafe extern "C" {
    pub fn memcpy(dst: *mut c_void, src: *const c_void, len: usize) -> *mut c_void;
}

pub unsafe fn text_poke_copy(
    dst: *mut c_void,
    src: *const c_void,
    len: usize,
) -> *mut c_void {
    unsafe { memcpy(dst, src, len) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
