/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from testing/selftests/net/lib/ksft.h. */
/* Original header guard omitted. */
/* Original dependencies: <stdio.h>, <stdlib.h>, <unistd.h>. */

use core::ffi::{c_char, c_int, c_void};

pub type FILE = c_void;

pub const STDIN_FILENO: c_int = 0;
pub const STDOUT_FILENO: c_int = 1;

unsafe extern "C" {
    pub static mut stderr: *mut FILE;

    pub fn getenv(name: *const c_char) -> *mut c_char;
    pub fn atoi(nptr: *const c_char) -> c_int;
    pub fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    pub fn perror(s: *const c_char);
    pub fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    pub fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    pub fn close(fd: c_int) -> c_int;
}

#[inline]
pub unsafe fn ksft_ready() {
    let msg: [c_char; 7] = [
        b'r' as c_char,
        b'e' as c_char,
        b'a' as c_char,
        b'd' as c_char,
        b'y' as c_char,
        b'\n' as c_char,
        b'\0' as c_char,
    ];
    let env_str: *mut c_char;
    let fd: c_int;

    env_str = unsafe { getenv(c"KSFT_READY_FD".as_ptr()) };
    if !env_str.is_null() {
        fd = unsafe { atoi(env_str) };
        if fd == 0 {
            unsafe {
                fprintf(
                    stderr,
                    c"invalid KSFT_READY_FD = '%s'\n".as_ptr(),
                    env_str,
                );
            }
            return;
        }
    } else {
        fd = STDOUT_FILENO;
    }

    if unsafe { write(fd, msg.as_ptr() as *const c_void, core::mem::size_of_val(&msg)) } < 0 {
        unsafe {
            perror(c"write()".as_ptr());
        }
    }
    if fd != STDOUT_FILENO {
        unsafe {
            close(fd);
        }
    }
}

#[inline]
pub unsafe fn ksft_wait() {
    let env_str: *mut c_char;
    let mut byte: c_char = 0;
    let fd: c_int;

    env_str = unsafe { getenv(c"KSFT_WAIT_FD".as_ptr()) };
    if !env_str.is_null() {
        fd = unsafe { atoi(env_str) };
        if fd == 0 {
            unsafe {
                fprintf(
                    stderr,
                    c"invalid KSFT_WAIT_FD = '%s'\n".as_ptr(),
                    env_str,
                );
            }
            return;
        }
    } else {
        /* Not running in KSFT env, wait for input from STDIN instead */
        fd = STDIN_FILENO;
    }

    if unsafe { read(fd, &mut byte as *mut c_char as *mut c_void, core::mem::size_of_val(&byte)) } < 0 {
        unsafe {
            perror(c"read()".as_ptr());
        }
    }
    if fd != STDIN_FILENO {
        unsafe {
            close(fd);
        }
    }
}
