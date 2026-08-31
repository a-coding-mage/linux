/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * getopt function definitions for NOLIBC, adapted from musl libc
 * Copyright (C) 2005-2020 Rich Felker, et al.
 * Copyright (C) 2025 Thomas Weißschuh <linux@weissschuh.net>
 */

/* make sure to include all global symbols: nolibc.h */

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct FILE {
    _unused: [u8; 0],
}

unsafe extern "C" {
    static stderr: *mut FILE;
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
}

/* __attribute__((weak,unused,section(".data.nolibc_getopt"))) */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".data.nolibc_getopt")]
pub static mut optarg: *mut c_char = core::ptr::null_mut();

/* __attribute__((weak,unused,section(".data.nolibc_getopt"))) */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".data.nolibc_getopt")]
pub static mut optind: c_int = 1;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".data.nolibc_getopt")]
pub static mut opterr: c_int = 1;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".data.nolibc_getopt")]
pub static mut optopt: c_int = 0;

static mut __optpos: c_int = 0;

#[allow(non_snake_case)]
pub unsafe fn getopt(argc: c_int, argv: *const *mut c_char, mut optstring: *const c_char) -> c_int {
    let mut i: c_int;
    let mut c: c_char;
    let mut d: c_char;
    let optchar: *mut c_char;

    if optind == 0 {
        __optpos = 0;
        optind = 1;
    }

    if optind >= argc || (*argv.add(optind as usize)).is_null() {
        return -1;
    }

    if *(*argv.add(optind as usize)).add(0) != b'-' as c_char {
        if *optstring.add(0) == b'-' as c_char {
            optarg = *argv.add(optind as usize);
            optind += 1;
            return 1;
        }
        return -1;
    }

    if *(*argv.add(optind as usize)).add(1) == 0 {
        return -1;
    }

    if *(*argv.add(optind as usize)).add(1) == b'-' as c_char
        && *(*argv.add(optind as usize)).add(2) == 0
    {
        optind += 1;
        return -1;
    }

    if __optpos == 0 {
        __optpos += 1;
    }
    c = *(*argv.add(optind as usize)).add(__optpos as usize);
    optchar = (*argv.add(optind as usize)).add(__optpos as usize);
    __optpos += 1;

    if *(*argv.add(optind as usize)).add(__optpos as usize) == 0 {
        optind += 1;
        __optpos = 0;
    }

    if *optstring.add(0) == b'-' as c_char || *optstring.add(0) == b'+' as c_char {
        optstring = optstring.add(1);
    }

    i = 0;
    d = 0;
    loop {
        d = *optstring.add(i as usize);
        i += 1;
        if !(d != 0 && d != c) {
            break;
        }
    }

    if d == 0 || d != c || c == b':' as c_char {
        optopt = c as c_int;
        if *optstring.add(0) != b':' as c_char && opterr != 0 {
            fprintf(
                stderr,
                c"%s: unrecognized option: %c\n".as_ptr(),
                *argv.add(0),
                *optchar as c_int,
            );
        }
        return b'?' as c_int;
    }
    if *optstring.add(i as usize) == b':' as c_char {
        optarg = core::ptr::null_mut();
        if *optstring.add((i + 1) as usize) != b':' as c_char || __optpos != 0 {
            optarg = *argv.add(optind as usize);
            optind += 1;
            if __optpos != 0 {
                optarg = optarg.add(__optpos as usize);
            }
            __optpos = 0;
        }
        if optind > argc {
            optopt = c as c_int;
            if *optstring.add(0) == b':' as c_char {
                return b':' as c_int;
            }
            if opterr != 0 {
                fprintf(
                    stderr,
                    c"%s: option requires argument: %c\n".as_ptr(),
                    *argv.add(0),
                    *optchar as c_int,
                );
            }
            return b'?' as c_int;
        }
    }
    c as c_int
}
