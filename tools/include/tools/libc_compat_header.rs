// SPDX-License-Identifier: (LGPL-2.0+ OR BSD-2-Clause)
/* Copyright (C) 2018 Netronome Systems, Inc. */

// C header dependencies:
// #include <stdlib.h>
// #include <linux/overflow.h>

use core::ffi::c_void;

unsafe extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

// Original C condition: #ifdef COMPAT_NEED_REALLOCARRAY
#[cfg(COMPAT_NEED_REALLOCARRAY)]
pub unsafe fn reallocarray(ptr: *mut c_void, nmemb: usize, size: usize) -> *mut c_void {
    let mut bytes: usize = 0;

    if unlikely(check_mul_overflow(nmemb, size, &mut bytes as *mut usize)) {
        return core::ptr::null_mut();
    }
    unsafe { realloc(ptr, bytes) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
