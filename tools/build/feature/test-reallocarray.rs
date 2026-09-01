// SPDX-License-Identifier: GPL-2.0
// C source defined _GNU_SOURCE before including <stdlib.h>.

use std::ffi::c_void;
use std::ptr;

extern "C" {
    fn reallocarray(ptr: *mut c_void, nmemb: usize, size: usize) -> *mut c_void;
}

fn main() -> i32 {
    unsafe { (!reallocarray(ptr::null_mut(), 1, 1).is_null()) as i32 }
}

// C source undefined _GNU_SOURCE after the implementation.

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
