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
/* Test that /proc/loadavg correctly reports last pid in pid namespace. */

use std::ffi::{c_char, c_int, c_void};

type pid_t = c_int;
type ssize_t = isize;
type off_t = i64;

const CLONE_NEWPID: c_int = 0x20000000;
const ENOSYS: c_int = 38;
const EPERM: c_int = 1;
const O_RDONLY: c_int = 0;
const SEEK_SET: c_int = 0;

unsafe extern "C" {
    fn __errno_location() -> *mut c_int;
    fn unshare(flags: c_int) -> c_int;
    fn fork() -> pid_t;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> ssize_t;
    fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;
}

fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn main_0() -> c_int {
    let mut pid: pid_t;
    let mut wstatus: c_int = 0;

    if unsafe { unshare(CLONE_NEWPID) } == -1 {
        if unsafe { *__errno_location() } == ENOSYS || unsafe { *__errno_location() } == EPERM {
            return 4;
        }
        return 1;
    }

    pid = unsafe { fork() };
    if pid == -1 {
        return 1;
    }
    if pid == 0 {
        let mut buf: [c_char; 128] = [0; 128];
        let mut p: *mut c_char;
        let fd: c_int;
        let mut rv: ssize_t;

        fd = unsafe { open(c"/proc/loadavg".as_ptr(), O_RDONLY) };
        if fd == -1 {
            return 1;
        }
        rv = unsafe { read(fd, buf.as_mut_ptr().cast::<c_void>(), buf.len()) };
        if rv < 3 {
            return 1;
        }
        p = unsafe { buf.as_mut_ptr().offset(rv) };

        /* pid 1 */
        if !(unsafe { *p.offset(-3) } == b' ' as c_char
            && unsafe { *p.offset(-2) } == b'1' as c_char
            && unsafe { *p.offset(-1) } == b'\n' as c_char)
        {
            return 1;
        }

        pid = unsafe { fork() };
        if pid == -1 {
            return 1;
        }
        if pid == 0 {
            return 0;
        }
        if unsafe { waitpid(pid, std::ptr::null_mut(), 0) } == -1 {
            return 1;
        }

        unsafe {
            lseek(fd, 0, SEEK_SET);
        }
        rv = unsafe { read(fd, buf.as_mut_ptr().cast::<c_void>(), buf.len()) };
        if rv < 3 {
            return 1;
        }
        p = unsafe { buf.as_mut_ptr().offset(rv) };

        /* pid 2 */
        if !(unsafe { *p.offset(-3) } == b' ' as c_char
            && unsafe { *p.offset(-2) } == b'2' as c_char
            && unsafe { *p.offset(-1) } == b'\n' as c_char)
        {
            return 1;
        }

        return 0;
    }

    if unsafe { waitpid(pid, &mut wstatus, 0) } == -1 {
        return 1;
    }
    if wifexited(wstatus) && wexitstatus(wstatus) == 0 {
        return 0;
    }
    1
}

fn main() {
    std::process::exit(unsafe { main_0() });
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
