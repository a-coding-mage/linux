/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies: <stdio.h>, <unistd.h>

unsafe extern "C" {
    pub static mut tests_failed: ::std::os::raw::c_int;
    pub static mut tests_verbose: ::std::os::raw::c_int;

    pub static mut stdout: *mut FILE;
    pub static mut stderr: *mut FILE;
    pub static mut optind: ::std::os::raw::c_int;

    pub fn getopt(
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
        optstring: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn fprintf(
        stream: *mut FILE,
        format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
    pub fn fflush(stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn fputc(c: ::std::os::raw::c_int, stream: *mut FILE) -> ::std::os::raw::c_int;
}

#[repr(C)]
pub struct FILE {
    _unused: [u8; 0],
}

#[inline]
pub unsafe fn get_verbose(
    argv: *mut *mut ::std::os::raw::c_char,
    argc: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    let mut c: ::std::os::raw::c_int;
    let mut verbose: ::std::os::raw::c_int = 0;

    loop {
        c = unsafe { getopt(argc, argv, c"v".as_ptr()) };
        if c == -1 {
            break;
        }
        match c {
            118 => {
                verbose = 1;
            }
            _ => {}
        }
    }
    unsafe {
        optind = 1;
    }

    verbose
}

#[macro_export]
macro_rules! __T_START {
    ($argv:expr, $argc:expr) => {{
        unsafe {
            tests_verbose = get_verbose($argv, $argc);
            fprintf(stdout, c"- running %s...".as_ptr(), cstr_file!().as_ptr());
            fflush(::std::ptr::null_mut());
            tests_failed = 0;
        }
    }};
}

#[macro_export]
macro_rules! __T_END {
    () => {{
        unsafe {
            if tests_failed != 0 {
                fprintf(stdout, c"  FAILED (%d)\n".as_ptr(), tests_failed);
            } else {
                fprintf(stdout, c"OK\n".as_ptr());
            }
        }
    }};
}

#[macro_export]
macro_rules! __T {
    ($text:expr, $cond:expr) => {{
        if !$cond {
            unsafe {
                fprintf(
                    stderr,
                    c"FAILED %s:%d %s\n".as_ptr(),
                    cstr_file!().as_ptr(),
                    line!() as ::std::os::raw::c_int,
                    $text,
                );
                tests_failed += 1;
            }
            return -1;
        }
    }};
}

#[macro_export]
macro_rules! __T_VERBOSE {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {{
        unsafe {
            if tests_verbose != 0 {
                if tests_verbose == 1 {
                    fputc('\n' as ::std::os::raw::c_int, stderr);
                    tests_verbose += 1;
                }
                fprintf(stderr, $fmt $(, $arg)*);
            }
        }
    }};
}

#[macro_export]
macro_rules! cstr_file {
    () => {{
        concat!(file!(), "\0").as_ptr() as *const ::std::os::raw::c_char
    }};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
