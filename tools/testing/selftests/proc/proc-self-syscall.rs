/*
 * Copyright © 2018 Alexey Dobriyan <adobriyan@gmail.com>
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

use libc::{c_int, c_void, size_t, ssize_t};
use std::ffi::CStr;
use std::mem;
use std::ptr;

#[inline]
unsafe fn sys_read(fd: c_int, buf: *mut c_void, len: size_t) -> ssize_t {
    libc::syscall(libc::SYS_read, fd, buf, len) as ssize_t
}

fn main() {
    let mut buf1 = [0 as libc::c_char; 64];
    let mut buf2 = [0 as libc::c_char; 64];
    let fd: c_int;
    let rv: ssize_t;

    unsafe {
        fd = libc::open(c"/proc/self/syscall".as_ptr(), libc::O_RDONLY);
        if fd == -1 {
            if *libc::__errno_location() == libc::ENOENT {
                std::process::exit(4);
            }
            std::process::exit(1);
        }

        /* Do direct system call as libc can wrap anything. */
        libc::snprintf(
            buf1.as_mut_ptr(),
            mem::size_of_val(&buf1),
            c"%ld 0x%lx 0x%lx 0x%lx".as_ptr(),
            libc::SYS_read as libc::c_long,
            fd as libc::c_long,
            buf2.as_mut_ptr() as libc::c_long,
            mem::size_of_val(&buf2) as libc::c_long,
        );

        ptr::write_bytes(buf2.as_mut_ptr(), 0, buf2.len());
        rv = sys_read(
            fd,
            buf2.as_mut_ptr() as *mut c_void,
            mem::size_of_val(&buf2),
        );
        if rv < 0 {
            std::process::exit(1);
        }
        if rv < libc::strlen(buf1.as_ptr()) as ssize_t {
            std::process::exit(1);
        }
        if libc::strncmp(buf1.as_ptr(), buf2.as_ptr(), CStr::from_ptr(buf1.as_ptr()).to_bytes().len()) != 0 {
            std::process::exit(1);
        }

        std::process::exit(0);
    }
}
