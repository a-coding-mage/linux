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
/* Test readlink /proc/self/map_files/... with minimum address. */

use std::ffi::c_void;
use std::process::exit;

unsafe extern "C" {
    fn snprintf(
        s: *mut libc::c_char,
        maxlen: libc::size_t,
        format: *const libc::c_char,
        ...
    ) -> libc::c_int;
    fn readlink(
        path: *const libc::c_char,
        buf: *mut libc::c_char,
        bufsiz: libc::size_t,
    ) -> libc::ssize_t;
    fn sysconf(name: libc::c_int) -> libc::c_long;
    fn open(pathname: *const libc::c_char, flags: libc::c_int, ...) -> libc::c_int;
    fn mmap(
        addr: *mut c_void,
        length: libc::size_t,
        prot: libc::c_int,
        flags: libc::c_int,
        fd: libc::c_int,
        offset: libc::off_t,
    ) -> *mut c_void;
    fn fprintf(stream: *mut libc::FILE, format: *const libc::c_char, ...) -> libc::c_int;
    static mut stderr: *mut libc::FILE;
    fn __errno_location() -> *mut libc::c_int;
}

const ENOENT: libc::c_int = 2;
const O_RDONLY: libc::c_int = 0;
const PROT_NONE: libc::c_int = 0x0;
const MAP_PRIVATE: libc::c_int = 0x02;
const MAP_FIXED: libc::c_int = 0x10;
const MAP_FILE: libc::c_int = 0;
const _SC_PAGESIZE: libc::c_int = 30;

unsafe fn pass(fmt: *const libc::c_char, a: libc::c_ulong, b: libc::c_ulong) {
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut buf: [libc::c_char; 64] = [0; 64];

    unsafe {
        snprintf(name.as_mut_ptr(), name.len(), fmt, a, b);
        if readlink(name.as_ptr(), buf.as_mut_ptr(), buf.len()) == -1 {
            exit(1);
        }
    }
}

unsafe fn fail(fmt: *const libc::c_char, a: libc::c_ulong, b: libc::c_ulong) {
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut buf: [libc::c_char; 64] = [0; 64];

    unsafe {
        snprintf(name.as_mut_ptr(), name.len(), fmt, a, b);
        if readlink(name.as_ptr(), buf.as_mut_ptr(), buf.len()) == -1
            && *__errno_location() == ENOENT
        {
            return;
        }
        exit(1);
    }
}

fn main() {
    unsafe {
        let page_size: libc::c_int = sysconf(_SC_PAGESIZE) as libc::c_int;
        /*
         * va_max must be enough bigger than vm.mmap_min_addr, which is
         * 64KB/32KB by default. (depends on CONFIG_LSM_MMAP_MIN_ADDR)
         */
        let va_max: libc::c_ulong = 1u64.wrapping_shl(20) as libc::c_ulong;
        let mut va: libc::c_ulong;
        let mut p: *mut c_void;
        let fd: libc::c_int;
        let a: libc::c_ulong;
        let b: libc::c_ulong;

        fd = open(c"/dev/zero".as_ptr(), O_RDONLY);
        if fd == -1 {
            exit(1);
        }

        va = 0;
        while va < va_max {
            p = mmap(
                va as *mut c_void,
                page_size as libc::size_t,
                PROT_NONE,
                MAP_PRIVATE | MAP_FILE | MAP_FIXED,
                fd,
                0,
            );
            if p == va as *mut c_void {
                break;
            }
            va = va.wrapping_add(page_size as libc::c_ulong);
        }
        if va == va_max {
            fprintf(
                stderr,
                c"error: mmap doesn't like you\n".as_ptr(),
            );
            exit(1);
        }

        p = va as *mut c_void;
        a = p as libc::c_ulong;
        b = (p as libc::c_ulong).wrapping_add(page_size as libc::c_ulong);

        pass(c"/proc/self/map_files/%lx-%lx".as_ptr(), a, b);
        fail(c"/proc/self/map_files/ %lx-%lx".as_ptr(), a, b);
        fail(c"/proc/self/map_files/%lx -%lx".as_ptr(), a, b);
        fail(c"/proc/self/map_files/%lx- %lx".as_ptr(), a, b);
        fail(c"/proc/self/map_files/%lx-%lx ".as_ptr(), a, b);
        fail(c"/proc/self/map_files/0%lx-%lx".as_ptr(), a, b);
        fail(c"/proc/self/map_files/%lx-0%lx".as_ptr(), a, b);
        if std::mem::size_of::<libc::c_long>() == 4 {
            fail(c"/proc/self/map_files/100000000%lx-%lx".as_ptr(), a, b);
            fail(c"/proc/self/map_files/%lx-100000000%lx".as_ptr(), a, b);
        } else if std::mem::size_of::<libc::c_long>() == 8 {
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
            exit(1);
        }

        exit(0);
    }
}
