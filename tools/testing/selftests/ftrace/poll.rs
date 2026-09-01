// SPDX-License-Identifier: GPL-2.0
/*
 * Simple poll on a file.
 *
 * Copyright (c) 2024 Google LLC.
 */

// C dependencies: errno.h, fcntl.h, poll.h, stdio.h, stdlib.h, string.h, unistd.h

use core::ffi::{c_char, c_int, c_short, c_uint, c_void};

const BUFSIZE: usize = 4096;

const EINTR: c_int = 4;
const O_RDONLY: c_int = 0;
const POLLIN: c_short = 0x0001;
const POLLPRI: c_short = 0x0002;

#[repr(C)]
struct pollfd {
    fd: c_int,
    events: c_short,
    revents: c_short,
}

unsafe extern "C" {
    static mut optarg: *mut c_char;
    static mut optind: c_int;
    static mut stderr: *mut FILE;

    fn __errno_location() -> *mut c_int;
    fn atoi(nptr: *const c_char) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn perror(s: *const c_char);
    fn poll(fds: *mut pollfd, nfds: c_uint, timeout: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

/*
 * Usage:
 *  poll [-I|-P] [-t timeout] FILE
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut pfd = pollfd {
        fd: 0,
        events: POLLIN,
        revents: 0,
    };
    let mut buf = [0 as c_char; BUFSIZE];
    let mut timeout: c_int = -1;
    let mut ret: c_int;
    let mut opt: c_int;

    loop {
        opt = unsafe { getopt(argc, argv, c"IPt:".as_ptr()) };
        if opt == -1 {
            break;
        }
        match opt {
            73 => {
                pfd.events = POLLIN;
            }
            80 => {
                pfd.events = POLLPRI;
            }
            116 => {
                timeout = unsafe { atoi(optarg) };
            }
            _ => {
                unsafe {
                    fprintf(
                        stderr,
                        c"Usage: %s [-I|-P] [-t timeout] FILE\n".as_ptr(),
                        *argv.offset(0),
                    );
                }
                return -1;
            }
        }
    }
    if unsafe { optind } >= argc {
        unsafe {
            fprintf(
                stderr,
                c"Error: Polling file is not specified\n".as_ptr(),
            );
        }
        return -1;
    }

    pfd.fd = unsafe { open(*argv.offset(optind as isize), O_RDONLY) };
    if pfd.fd < 0 {
        unsafe {
            fprintf(stderr, c"failed to open %s".as_ptr(), *argv.offset(optind as isize));
            perror(c"open".as_ptr());
        }
        return -1;
    }

    /* Reset poll by read if POLLIN is specified. */
    if pfd.events & POLLIN != 0 {
        while unsafe { read(pfd.fd, buf.as_mut_ptr() as *mut c_void, BUFSIZE) } == BUFSIZE as isize {}
    }

    ret = unsafe { poll(&mut pfd, 1, timeout) };
    if ret < 0 && unsafe { *__errno_location() } != EINTR {
        unsafe {
            perror(c"poll".as_ptr());
        }
        return -1;
    }
    unsafe {
        close(pfd.fd);
    }

    /* If timeout happened (ret == 0), exit code is 1 */
    if ret == 0 {
        return 1;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
