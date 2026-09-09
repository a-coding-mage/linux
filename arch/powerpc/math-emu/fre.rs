// SPDX-License-Identifier: GPL-2.0

use core::ffi::c_void;

// Supplied by the kernel environment when DEBUG is enabled.
#[cfg(feature = "DEBUG")]
extern "C" {
    fn printk(fmt: *const i8, ...) -> i32;
}

const ENOSYS: i32 = 38;

pub unsafe fn fre(frD: *mut c_void, frB: *mut c_void) -> i32 {
    #[cfg(feature = "DEBUG")]
    {
        static FUNC: &[u8] = b"fre\0";
        static FORMAT: &[u8] = b"%s: %p %p\n\0";
        printk(
            FORMAT.as_ptr() as *const i8,
            FUNC.as_ptr() as *const i8,
            frD,
            frB,
        );
    }
    -ENOSYS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
