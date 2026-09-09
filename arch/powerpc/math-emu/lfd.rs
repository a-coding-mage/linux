// SPDX-License-Identifier: GPL-2.0
//
// Dependencies corresponding to the original Linux and architecture headers
// are supplied by the surrounding translation unit.

use core::ffi::{c_char, c_int, c_void};

unsafe extern "C" {
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;

    #[cfg(DEBUG)]
    fn printk(format: *const c_char, ...);

    #[cfg(DEBUG)]
    fn dump_double(value: *const c_void);
}

const EFAULT: c_int = 14;

pub unsafe extern "C" fn lfd(frD: *mut c_void, ea: *const c_void) -> c_int {
    if copy_from_user(frD, ea, core::mem::size_of::<f64>()) != 0 {
        return -EFAULT;
    }

    #[cfg(DEBUG)]
    {
        // __func__ in the original C source.
        static FUNCTION_NAME: &[u8] = b"lfd\0";
        static DEBUG_FORMAT: &[u8] = b"%s: D %p, ea %p: \0";
        static NEWLINE: &[u8] = b"\n\0";
        printk(DEBUG_FORMAT.as_ptr() as *const c_char, FUNCTION_NAME.as_ptr(), frD, ea);
        dump_double(frD);
        printk(NEWLINE.as_ptr() as *const c_char);
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
