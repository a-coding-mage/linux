/* SPDX-License-Identifier: LGPL-2.1+ */
/*
 * SSL helper functions shared by sign-file and extract-cert.
 */

extern "C" {
    fn ERR_peek_error() -> ::std::os::raw::c_ulong;
    fn ERR_peek_error_line(
        file: *mut *const ::std::os::raw::c_char,
        line: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulong;
    fn ERR_error_string(
        e: ::std::os::raw::c_ulong,
        buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    fn ERR_get_error() -> ::std::os::raw::c_ulong;
    static mut stderr: *mut ::std::ffi::c_void;
    fn fprintf(stream: *mut ::std::ffi::c_void, format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
    fn errx(status: ::std::os::raw::c_int, format: *const ::std::os::raw::c_char, ...) -> !;
}

unsafe fn drain_openssl_errors(l: ::std::os::raw::c_int, silent: bool) {
    let mut file: *const ::std::os::raw::c_char;
    let mut buf = [0 as ::std::os::raw::c_char; 120];
    let mut e: ::std::os::raw::c_ulong;
    let mut line: ::std::os::raw::c_int;

    if ERR_peek_error() == 0 {
        return;
    }
    if !silent {
        fprintf(
            stderr,
            b"At main.c:%d:\n\0".as_ptr() as *const ::std::os::raw::c_char,
            l,
        );
    }

    loop {
        e = ERR_peek_error_line(&mut file, &mut line);
        if e == 0 {
            break;
        }
        ERR_error_string(e, buf.as_mut_ptr());
        if !silent {
            fprintf(
                stderr,
                b"- SSL %s: %s:%d\n\0".as_ptr() as *const ::std::os::raw::c_char,
                buf.as_ptr(),
                file,
                line,
            );
        }
        ERR_get_error();
    }
}

macro_rules! ERR {
    ($cond:expr, $fmt:expr $(, $args:expr)*) => {{
        let __cond: bool = $cond;
        unsafe {
            drain_openssl_errors(line!() as ::std::os::raw::c_int, false);
            if __cond {
                errx(
                    1,
                    $fmt,
                    $($args,)*
                );
            }
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
