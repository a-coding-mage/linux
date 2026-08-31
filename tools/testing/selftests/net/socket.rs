// SPDX-License-Identifier: GPL-2.0
//
// Translated from C. Original dependencies:
// <stdio.h>, <errno.h>, <unistd.h>, <string.h>, <sys/types.h>,
// <sys/socket.h>, <netinet/in.h>, and "kselftest.h".

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_void};

type size_t = usize;

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut stderr: *mut FILE;

    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> *mut c_char;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn __errno_location() -> *mut c_int;
}

const AF_INET: c_int = 2;
const AF_MAX: c_int = 46;
const SOCK_STREAM: c_int = 1;
const SOCK_DGRAM: c_int = 2;
const IPPROTO_TCP: c_int = 6;
const IPPROTO_UDP: c_int = 17;
const EPROTONOSUPPORT: c_int = 93;
const EAFNOSUPPORT: c_int = 97;

#[repr(C)]
struct socket_testcase {
    domain: c_int,
    type_: c_int,
    protocol: c_int,

    /* 0    = valid file descriptor
     * -foo = error foo
     */
    expect: c_int,

    /* If non-zero, accept EAFNOSUPPORT to handle the case
     * of the protocol not being configured into the kernel.
     */
    nosupport_ok: c_int,
}

static tests: [socket_testcase; 5] = [
    socket_testcase {
        domain: AF_MAX,
        type_: 0,
        protocol: 0,
        expect: -EAFNOSUPPORT,
        nosupport_ok: 0,
    },
    socket_testcase {
        domain: AF_INET,
        type_: SOCK_STREAM,
        protocol: IPPROTO_TCP,
        expect: 0,
        nosupport_ok: 1,
    },
    socket_testcase {
        domain: AF_INET,
        type_: SOCK_DGRAM,
        protocol: IPPROTO_TCP,
        expect: -EPROTONOSUPPORT,
        nosupport_ok: 1,
    },
    socket_testcase {
        domain: AF_INET,
        type_: SOCK_DGRAM,
        protocol: IPPROTO_UDP,
        expect: 0,
        nosupport_ok: 1,
    },
    socket_testcase {
        domain: AF_INET,
        type_: SOCK_STREAM,
        protocol: IPPROTO_UDP,
        expect: -EPROTONOSUPPORT,
        nosupport_ok: 1,
    },
];

const ERR_STRING_SZ: usize = 64;

unsafe fn errno_value() -> c_int {
    unsafe { *__errno_location() }
}

unsafe fn run_tests() -> c_int {
    let mut err_string1 = [0 as c_char; ERR_STRING_SZ];
    let mut err_string2 = [0 as c_char; ERR_STRING_SZ];
    let mut msg1: *mut c_char;
    let mut msg2: *mut c_char;
    let mut i: usize;
    let mut err: c_int;

    err = 0;
    i = 0;
    while i < tests.len() {
        let s: *const socket_testcase = &tests[i];
        let fd: c_int;

        fd = unsafe { socket((*s).domain, (*s).type_, (*s).protocol) };
        if fd < 0 {
            if unsafe { (*s).nosupport_ok != 0 && errno_value() == EAFNOSUPPORT } {
                i += 1;
                continue;
            }

            if unsafe { (*s).expect < 0 && errno_value() == -(*s).expect } {
                i += 1;
                continue;
            }

            msg1 = unsafe {
                strerror_r(
                    -(*s).expect,
                    err_string1.as_mut_ptr(),
                    ERR_STRING_SZ as size_t,
                )
            };
            msg2 = unsafe {
                strerror_r(
                    errno_value(),
                    err_string2.as_mut_ptr(),
                    ERR_STRING_SZ as size_t,
                )
            };

            unsafe {
                fprintf(
                    stderr,
                    b"socket(%d, %d, %d) expected err (%s) got (%s)\n\0".as_ptr()
                        as *const c_char,
                    (*s).domain,
                    (*s).type_,
                    (*s).protocol,
                    msg1,
                    msg2,
                );
            }

            err = -1;
            break;
        } else {
            unsafe {
                close(fd);
            }

            if unsafe { (*s).expect < 0 } {
                msg1 = unsafe {
                    strerror_r(
                        errno_value(),
                        err_string1.as_mut_ptr(),
                        ERR_STRING_SZ as size_t,
                    )
                };

                unsafe {
                    fprintf(
                        stderr,
                        b"socket(%d, %d, %d) expected success got err (%s)\n\0"
                            .as_ptr() as *const c_char,
                        (*s).domain,
                        (*s).type_,
                        (*s).protocol,
                        msg1,
                    );
                }

                err = -1;
                break;
            }
        }

        i += 1;
    }

    err
}

fn main() -> c_int {
    let err: c_int = unsafe { run_tests() };

    err
}
