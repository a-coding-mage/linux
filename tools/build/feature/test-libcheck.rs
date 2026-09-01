// SPDX-License-Identifier: GPL-2.0
// C dependency: #include <check.h>

use std::os::raw::{c_char, c_int};

#[repr(C)]
pub struct Suite {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn suite_create(name: *const c_char) -> *mut Suite;
}

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    let s: *mut Suite = unsafe { suite_create(b"test\0".as_ptr() as *const c_char) };
    (s == std::ptr::null_mut()) as c_int
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
