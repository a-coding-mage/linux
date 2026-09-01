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
/* Test readlink /proc/self/map_files/... */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

type size_t = usize;
type ssize_t = isize;
type off_t = i64;

const ENOENT: c_int = 2;
const O_RDONLY: c_int = 0;
const PROT_NONE: c_int = 0x0;
const MAP_PRIVATE: c_int = 0x02;
const MAP_FILE: c_int = 0;
const _SC_PAGESIZE: c_int = 30;

unsafe extern "C" {
    fn __errno_location() -> *mut c_int;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn readlink(pathname: *const c_char, buf: *mut c_char, bufsiz: size_t) -> ssize_t;
    fn exit(status: c_int) -> !;
    fn sysconf(name: c_int) -> c_long;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: off_t,
    ) -> *mut c_void;
}

unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

unsafe fn pass(fmt: *const c_char, a: c_ulong, b: c_ulong) {
    let mut name = [0 as c_char; 64];
    let mut buf = [0 as c_char; 64];

    unsafe {
        snprintf(name.as_mut_ptr(), name.len(), fmt, a, b);
        if readlink(name.as_ptr(), buf.as_mut_ptr(), buf.len()) == -1 {
            exit(1);
        }
    }
}

unsafe fn fail(fmt: *const c_char, a: c_ulong, b: c_ulong) {
    let mut name = [0 as c_char; 64];
    let mut buf = [0 as c_char; 64];

    unsafe {
        snprintf(name.as_mut_ptr(), name.len(), fmt, a, b);
        if readlink(name.as_ptr(), buf.as_mut_ptr(), buf.len()) == -1 && errno() == ENOENT {
            return;
        }
        exit(1);
    }
}

fn main() -> c_int {
    unsafe {
        let PAGE_SIZE: c_uint = sysconf(_SC_PAGESIZE) as c_uint;
        let p: *mut c_void;
        let fd: c_int;
        let a: c_ulong;
        let b: c_ulong;

        fd = open(c"/dev/zero".as_ptr(), O_RDONLY);
        if fd == -1 {
            return 1;
        }

        p = mmap(
            core::ptr::null_mut(),
            PAGE_SIZE as size_t,
            PROT_NONE,
            MAP_PRIVATE | MAP_FILE,
            fd,
            0,
        );
        if p == (-1isize) as *mut c_void {
            return 1;
        }

        a = p as c_ulong;
        b = (p as c_ulong).wrapping_add(PAGE_SIZE as c_ulong);

        pass(c"/proc/self/map_files/%lx-%lx".as_ptr(), a, b);
        fail(c"/proc/self/map_files/ %lx-%lx".as_ptr(), a, b);
        fail(c"/proc/self/map_files/%lx -%lx".as_ptr(), a, b);
        fail(c"/proc/self/map_files/%lx- %lx".as_ptr(), a, b);
        fail(c"/proc/self/map_files/%lx-%lx ".as_ptr(), a, b);
        fail(c"/proc/self/map_files/0%lx-%lx".as_ptr(), a, b);
        fail(c"/proc/self/map_files/%lx-0%lx".as_ptr(), a, b);
        if core::mem::size_of::<c_long>() == 4 {
            fail(c"/proc/self/map_files/100000000%lx-%lx".as_ptr(), a, b);
            fail(c"/proc/self/map_files/%lx-100000000%lx".as_ptr(), a, b);
        } else if core::mem::size_of::<c_long>() == 8 {
            fail(
                c"/proc/self/map_files/10000000000000000%lx-%lx".as_ptr(),
                a,
                b,
            );
            fail(
                c"/proc/self/map_files/%lx-10000000000000000%lx".as_ptr(),
                a,
                b,
            );
        } else {
            return 1;
        }

        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
