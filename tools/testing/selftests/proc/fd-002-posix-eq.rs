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
// Test that open(/proc/*/fd/*) opens the same file.

use std::ffi::CString;
use std::mem;

fn main() {
    let fd0: libc::c_int;
    let fd1: libc::c_int;
    let fd2: libc::c_int;
    let mut st0: libc::stat;
    let mut st1: libc::stat;
    let mut st2: libc::stat;
    let mut rv: libc::c_int;

    unsafe {
        st0 = mem::zeroed();
        st1 = mem::zeroed();
        st2 = mem::zeroed();

        fd0 = libc::open(c"/".as_ptr(), libc::O_DIRECTORY | libc::O_RDONLY);
        assert!(fd0 >= 0);

        let buf = CString::new(format!("/proc/self/fd/{}", fd0 as libc::c_uint)).unwrap();
        fd1 = libc::open(buf.as_ptr(), libc::O_RDONLY);
        assert!(fd1 >= 0);

        let buf = CString::new(format!("/proc/thread-self/fd/{}", fd0 as libc::c_uint)).unwrap();
        fd2 = libc::open(buf.as_ptr(), libc::O_RDONLY);
        assert!(fd2 >= 0);

        rv = libc::fstat(fd0, &mut st0);
        assert!(rv == 0);
        rv = libc::fstat(fd1, &mut st1);
        assert!(rv == 0);
        rv = libc::fstat(fd2, &mut st2);
        assert!(rv == 0);

        assert!(st0.st_dev == st1.st_dev);
        assert!(st0.st_ino == st1.st_ino);

        assert!(st0.st_dev == st2.st_dev);
        assert!(st0.st_ino == st2.st_ino);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
