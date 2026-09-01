// SPDX-License-Identifier: GPL-2.0
// C dependency: #include <bfd.h>

use std::ffi::{c_int, c_void};
use std::ptr;

extern "C" {
    fn bfd_thread_init(
        lock: Option<unsafe extern "C" fn(*mut c_void) -> bool>,
        unlock: Option<unsafe extern "C" fn(*mut c_void) -> bool>,
        data: *mut c_void,
    ) -> bool;
}

unsafe extern "C" fn lock(unused: *mut c_void) -> bool {
    let _ = unused;
    true
}

unsafe extern "C" fn unlock(unused: *mut c_void) -> bool {
    let _ = unused;
    true
}

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    /* Check for presence of new thread safety API (version 2.42) */
    (!bfd_thread_init(Some(lock), Some(unlock), ptr::null_mut())) as c_int
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
