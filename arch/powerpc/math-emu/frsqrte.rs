// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_void};

// Supplied by the kernel environment.
unsafe extern "C" {
    fn printk(fmt: *const c_char, ...) -> i32;
}

pub unsafe fn frsqrte(frD: *mut c_void, frB: *mut c_void) -> i32 {
    // C build-time DEBUG condition.
    #[cfg(feature = "DEBUG")]
    unsafe {
        static FUNC: &[u8] = b"frsqrte\0";
        static FORMAT: &[u8] = b"%s: %p %p\n\0";
        printk(
            FORMAT.as_ptr() as *const c_char,
            FUNC.as_ptr() as *const c_char,
            frD,
            frB,
        );
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
