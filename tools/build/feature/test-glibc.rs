// SPDX-License-Identifier: GPL-2.0
// C dependency: <stdlib.h>
//
// Original C conditional:
//   #if !defined(__UCLIBC__)
//     #include <gnu/libc-version.h>
//   #else
//     #define XSTR(s) STR(s)
//     #define STR(s) #s
//   #endif

use core::ffi::c_char;
use core::ptr;

unsafe extern "C" {
    fn gnu_get_libc_version() -> *const c_char;
}

pub fn main() -> i32 {
    // Original C conditional:
    //   #if !defined(__UCLIBC__)
    let version: *const c_char = unsafe { gnu_get_libc_version() };
    //   #else
    //     const char *version = XSTR(__GLIBC__) "." XSTR(__GLIBC_MINOR__);
    //   #endif

    (version == ptr::null()) as i32
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
