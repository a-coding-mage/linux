// SPDX-License-Identifier: GPL-2.0
// C source included <string.h>, <stdio.h>, and <linux/string.h>.

use core::ffi::{c_char, c_int};

unsafe extern "C" {
    fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: usize) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
}

/*
 * The tools so far have been using the strerror_r() GNU variant, that returns
 * a string, be it the buffer passed or something else.
 *
 * But that, besides being tricky in cases where we expect that the function
 * using strerror_r() returns the error formatted in a provided buffer (we have
 * to check if it returned something else and copy that instead), breaks the
 * build on systems not using glibc, like Alpine Linux, where musl libc is
 * used.
 *
 * So, introduce yet another wrapper, str_error_r(), that has the GNU
 * interface, but uses the portable XSI variant of strerror_r(), so that users
 * rest asured that the provided buffer is used and it is what is returned.
 */
#[no_mangle]
pub unsafe extern "C" fn str_error_r(
    errnum: c_int,
    buf: *mut c_char,
    buflen: usize,
) -> *mut c_char {
    let err: c_int = unsafe { strerror_r(errnum, buf, buflen) };
    if err != 0 {
        unsafe {
            snprintf(
                buf,
                buflen,
                c"INTERNAL ERROR: strerror_r(%d, [buf], %zd)=%d".as_ptr(),
                errnum,
                buflen,
                err,
            );
        }
    }
    buf
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
