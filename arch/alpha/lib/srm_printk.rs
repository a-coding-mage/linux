// SPDX-License-Identifier: GPL-2.0
/*
 *	arch/alpha/lib/srm_printk.c
 */

use core::ffi::{c_char, c_long, c_void};

extern "C" {
    // Supplied by the kernel formatting implementation.
    fn vsprintf(buf: *mut c_char, fmt: *const c_char, args: *mut c_void) -> c_long;
    fn srm_puts(buf: *const c_char, count: c_long);
}

#[no_mangle]
pub unsafe extern "C" fn srm_printk(fmt: *const c_char, mut _args: ...) -> c_long {
    static mut BUF: [c_char; 1024] = [0; 1024];
    let mut args: *mut c_void = core::ptr::null_mut();
    let len: c_long;
    let mut num_lf: c_long;
    let mut src: *mut c_char;
    let mut dst: *mut c_char;

    // C va_start/va_end operate on the variadic argument list.  Rust's C
    // variadic ABI supplies the corresponding list to the external formatter.
    len = vsprintf(BUF.as_mut_ptr(), fmt, args);

    /* count number of linefeeds in string: */
    num_lf = 0;
    src = BUF.as_mut_ptr();
    while *src != 0 {
        if *src == b'\n' as c_char {
            num_lf += 1;
        }
        src = src.add(1);
    }

    if num_lf != 0 {
        /* expand each linefeed into carriage-return/linefeed: */
        dst = src.add(num_lf as usize);
        while (src as usize) >= (BUF.as_mut_ptr() as usize) {
            if *src == b'\n' as c_char {
                *dst = b'\r' as c_char;
                dst = dst.sub(1);
            }
            *dst = *src;
            dst = dst.sub(1);
            src = src.sub(1);
        }
    }

    srm_puts(BUF.as_ptr(), num_lf + len);
    len
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
