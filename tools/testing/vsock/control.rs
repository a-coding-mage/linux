// SPDX-License-Identifier: GPL-2.0-only
/* Control socket for client/server test execution
 *
 * Copyright (C) 2017 Red Hat, Inc.
 *
 * Author: Stefan Hajnoczi <stefanha@redhat.com>
 */

/* The client and server may need to coordinate to avoid race conditions like
 * the client attempting to connect to a socket that the server is not
 * listening on yet.  The control socket offers a communications channel for
 * such coordination tasks.
 *
 * If the client calls control_expectln("LISTENING"), then it will block until
 * the server calls control_writeln("LISTENING").  This provides a simple
 * mechanism for coordinating between the client and the server.
 */

/* C dependencies removed from executable Rust:
 * errno.h, netdb.h, stdio.h, stdlib.h, string.h, unistd.h, sys/types.h,
 * sys/socket.h, timeout.h, control.h, util.h
 */

use core::ffi::c_void;
use std::os::raw::{c_char, c_int, c_ulong};

type size_t = usize;
type ssize_t = isize;
type socklen_t = u32;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [c_char; 14],
}

#[repr(C)]
pub struct addrinfo {
    pub ai_flags: c_int,
    pub ai_family: c_int,
    pub ai_socktype: c_int,
    pub ai_protocol: c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut c_char,
    pub ai_next: *mut addrinfo,
}

const SOCK_STREAM: c_int = 1;
const SOL_SOCKET: c_int = 1;
const SO_REUSEADDR: c_int = 2;
const MSG_MORE: c_int = 0x8000;
const EINTR: c_int = 4;
const EXIT_FAILURE: c_int = 1;

/* From timeout.h */
extern "C" {
    static TIMEOUT: c_ulong;
}

extern "C" {
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
    static mut errno: c_int;

    fn getaddrinfo(
        node: *const c_char,
        service: *const c_char,
        hints: *const addrinfo,
        res: *mut *mut addrinfo,
    ) -> c_int;
    fn gai_strerror(errcode: c_int) -> *const c_char;
    fn freeaddrinfo(res: *mut addrinfo);

    fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int;
    fn connect(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int;
    fn listen(sockfd: c_int, backlog: c_int) -> c_int;
    fn accept(sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn send(sockfd: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t;
    fn recv(sockfd: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> ssize_t;

    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut FILE) -> c_int;
    fn perror(s: *const c_char);
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;

    fn exit(status: c_int) -> !;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);

    fn timeout_begin(seconds: c_ulong);
    fn timeout_check(name: *const c_char);
    fn timeout_end();
    fn setsockopt_int_check(
        fd: c_int,
        level: c_int,
        optname: c_int,
        val: c_int,
        errmsg: *const c_char,
    );
}

static mut control_fd: c_int = -1;

/* Open the control socket, either in server or client mode */
#[no_mangle]
pub unsafe extern "C" fn control_init(
    control_host: *const c_char,
    control_port: *const c_char,
    server: bool,
) {
    let hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: SOCK_STREAM,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: core::ptr::null_mut(),
        ai_canonname: core::ptr::null_mut(),
        ai_next: core::ptr::null_mut(),
    };
    let mut result: *mut addrinfo = core::ptr::null_mut();
    let mut ai: *mut addrinfo;
    let ret: c_int;

    ret = getaddrinfo(control_host, control_port, &hints, &mut result);
    if ret != 0 {
        fprintf(stderr, b"%s\n\0".as_ptr() as *const c_char, gai_strerror(ret));
        exit(EXIT_FAILURE);
    }

    ai = result;
    while !ai.is_null() {
        let fd: c_int;

        fd = socket((*ai).ai_family, (*ai).ai_socktype, (*ai).ai_protocol);
        if fd < 0 {
            ai = (*ai).ai_next;
            continue;
        }

        if !server {
            if connect(fd, (*ai).ai_addr, (*ai).ai_addrlen) < 0 {
                close(fd);
                ai = (*ai).ai_next;
                continue;
            }
            control_fd = fd;
            printf(
                b"Control socket connected to %s:%s.\n\0".as_ptr() as *const c_char,
                control_host,
                control_port,
            );
            break;
        }

        setsockopt_int_check(
            fd,
            SOL_SOCKET,
            SO_REUSEADDR,
            1,
            b"setsockopt SO_REUSEADDR\0".as_ptr() as *const c_char,
        );

        if bind(fd, (*ai).ai_addr, (*ai).ai_addrlen) < 0 {
            close(fd);
            ai = (*ai).ai_next;
            continue;
        }
        if listen(fd, 1) < 0 {
            close(fd);
            ai = (*ai).ai_next;
            continue;
        }

        printf(
            b"Control socket listening on %s:%s\n\0".as_ptr() as *const c_char,
            control_host,
            control_port,
        );
        fflush(stdout);

        control_fd = accept(fd, core::ptr::null_mut(), core::ptr::null_mut());
        close(fd);

        if control_fd < 0 {
            perror(b"accept\0".as_ptr() as *const c_char);
            exit(EXIT_FAILURE);
        }
        printf(b"Control socket connection accepted...\n\0".as_ptr() as *const c_char);
        break;
    }

    if control_fd < 0 {
        fprintf(
            stderr,
            b"Control socket initialization failed.  Invalid address %s:%s?\n\0".as_ptr()
                as *const c_char,
            control_host,
            control_port,
        );
        exit(EXIT_FAILURE);
    }

    freeaddrinfo(result);
}

/* Free resources */
#[no_mangle]
pub unsafe extern "C" fn control_cleanup() {
    close(control_fd);
    control_fd = -1;
}

/* Write a line to the control socket */
#[no_mangle]
pub unsafe extern "C" fn control_writeln(str_: *const c_char) {
    let len: ssize_t = strlen(str_) as ssize_t;
    let mut ret: ssize_t;

    timeout_begin(TIMEOUT);

    loop {
        ret = send(control_fd, str_ as *const c_void, len as size_t, MSG_MORE);
        timeout_check(b"send\0".as_ptr() as *const c_char);
        if !(ret < 0 && errno == EINTR) {
            break;
        }
    }

    if ret != len {
        perror(b"send\0".as_ptr() as *const c_char);
        exit(EXIT_FAILURE);
    }

    loop {
        ret = send(
            control_fd,
            b"\n\0".as_ptr() as *const c_void,
            1,
            0,
        );
        timeout_check(b"send\0".as_ptr() as *const c_char);
        if !(ret < 0 && errno == EINTR) {
            break;
        }
    }

    if ret != 1 {
        perror(b"send\0".as_ptr() as *const c_char);
        exit(EXIT_FAILURE);
    }

    timeout_end();
}

#[no_mangle]
pub unsafe extern "C" fn control_writeulong(value: c_ulong) {
    let mut str_: [c_char; 32] = [0; 32];

    if snprintf(
        str_.as_mut_ptr(),
        str_.len(),
        b"%lu\0".as_ptr() as *const c_char,
        value,
    ) as size_t
        >= str_.len()
    {
        perror(b"snprintf\0".as_ptr() as *const c_char);
        exit(EXIT_FAILURE);
    }

    control_writeln(str_.as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn control_readulong() -> c_ulong {
    let value: c_ulong;
    let str_: *mut c_char;

    str_ = control_readln();

    if str_.is_null() {
        exit(EXIT_FAILURE);
    }

    value = strtoul(str_, core::ptr::null_mut(), 10);
    free(str_ as *mut c_void);

    value
}

/* Return the next line from the control socket (without the trailing newline).
 *
 * The program terminates if a timeout occurs.
 *
 * The caller must free() the returned string.
 */
#[no_mangle]
pub unsafe extern "C" fn control_readln() -> *mut c_char {
    let mut buf: *mut c_char = core::ptr::null_mut();
    let mut idx: size_t = 0;
    let mut buflen: size_t = 0;

    timeout_begin(TIMEOUT);

    loop {
        let mut ret: ssize_t;

        if idx >= buflen {
            let new_buf: *mut c_char;

            new_buf = realloc(buf as *mut c_void, buflen + 80) as *mut c_char;
            if new_buf.is_null() {
                perror(b"realloc\0".as_ptr() as *const c_char);
                exit(EXIT_FAILURE);
            }

            buf = new_buf;
            buflen += 80;
        }

        loop {
            ret = recv(
                control_fd,
                buf.add(idx) as *mut c_void,
                1,
                0,
            );
            timeout_check(b"recv\0".as_ptr() as *const c_char);
            if !(ret < 0 && errno == EINTR) {
                break;
            }
        }

        if ret == 0 {
            fprintf(
                stderr,
                b"unexpected EOF on control socket\n\0".as_ptr() as *const c_char,
            );
            exit(EXIT_FAILURE);
        }

        if ret != 1 {
            perror(b"recv\0".as_ptr() as *const c_char);
            exit(EXIT_FAILURE);
        }

        if *buf.add(idx) == b'\n' as c_char {
            *buf.add(idx) = b'\0' as c_char;
            break;
        }

        idx += 1;
    }

    timeout_end();

    buf
}

/* Wait until a given line is received or a timeout occurs */
#[no_mangle]
pub unsafe extern "C" fn control_expectln(str_: *const c_char) {
    let line: *mut c_char;

    line = control_readln();

    control_cmpln(line, str_, true);

    free(line as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn control_cmpln(
    line: *mut c_char,
    str_: *const c_char,
    fail: bool,
) -> bool {
    if strcmp(str_, line) == 0 {
        return true;
    }

    if fail {
        fprintf(
            stderr,
            b"expected \"%s\" on control socket, got \"%s\"\n\0".as_ptr() as *const c_char,
            str_,
            line,
        );
        exit(EXIT_FAILURE);
    }

    false
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
