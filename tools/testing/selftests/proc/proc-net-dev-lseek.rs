/*
 * Copyright (c) 2025 Alexey Dobriyan <adobriyan@gmail.com>
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

// C dependency intent: _GNU_SOURCE is required for unshare(2) and CLONE_NEWNET.

use std::ffi::{c_char, c_int, c_void};

type ssize_t = isize;
type off_t = i64;

const ENOSYS: c_int = 38;
const EPERM: c_int = 1;
const O_RDONLY: c_int = 0;
const SEEK_SET: c_int = 0;
const CLONE_NEWNET: c_int = 0x40000000;

unsafe extern "C" {
    fn unshare(flags: c_int) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> ssize_t;
    fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn __errno_location() -> *mut c_int;
}

/*
 * Test that lseek("/proc/net/dev/", 0, SEEK_SET)
 * a) works,
 * b) does what you think it does.
 */
fn main() {
    unsafe {
        /* /proc/net/dev output is deterministic in fresh netns only. */
        if unshare(CLONE_NEWNET) == -1 {
            if *__errno_location() == ENOSYS || *__errno_location() == EPERM {
                std::process::exit(4);
            }
            std::process::exit(1);
        }

        let fd: c_int = open(c"/proc/net/dev".as_ptr(), O_RDONLY);
        assert!(fd >= 0);

        let mut buf1 = [0u8; 4096];
        let rv1: ssize_t = read(fd, buf1.as_mut_ptr().cast::<c_void>(), buf1.len());
        /*
         * Not "<=", this file can't be empty:
         * there is header, "lo" interface with some zeroes.
         */
        assert!(0 < rv1);
        assert!(rv1 <= buf1.len() as ssize_t);

        /* Believe it or not, this line broke one day. */
        assert!(lseek(fd, 0, SEEK_SET) == 0);

        let mut buf2 = [0u8; 4096];
        let rv2: ssize_t = read(fd, buf2.as_mut_ptr().cast::<c_void>(), buf2.len());
        /* Not "<=", see above. */
        assert!(0 < rv2);
        assert!(rv2 <= buf2.len() as ssize_t);

        /* Test that lseek rewinds to the beginning of the file. */
        assert!(rv1 == rv2);
        assert!(memcmp(buf1.as_ptr().cast::<c_void>(), buf2.as_ptr().cast::<c_void>(), rv1 as usize) == 0);

        /* Contents of the file is not validated: this test is about lseek(). */

        std::process::exit(0);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
