// SPDX-License-Identifier: LGPL-2.1

use core::ffi::c_void;
use core::ptr;

// C dependencies: <stdlib.h>, <linux/zalloc.h>
unsafe extern "C" {
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn zalloc(size: usize) -> *mut c_void {
    unsafe { calloc(1, size) }
}

#[no_mangle]
pub unsafe extern "C" fn __zfree(ptr: *mut *mut c_void) {
    unsafe {
        free(*ptr);
        *ptr = ptr::null_mut();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
