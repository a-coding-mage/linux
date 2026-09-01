// SPDX-License-Identifier: GPL-2.0
// C dependency: #include <stdio.h>

use std::os::raw::{c_char, c_int};

unsafe extern "C" {
    fn puts(s: *const c_char) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    unsafe { puts(c"hi".as_ptr()) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
