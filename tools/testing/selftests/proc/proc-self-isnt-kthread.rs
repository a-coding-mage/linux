/*
 * Copyright (c) 2024 Alexey Dobriyan <adobriyan@gmail.com>
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
/* Test that userspace program is not kernel thread. */

use core::ffi::{c_char, c_int, c_void};

const O_RDONLY: c_int = 0;

unsafe extern "C" {
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
}

fn main() -> c_int {
    unsafe {
        let fd = open(c"/proc/self/status".as_ptr(), O_RDONLY);
        assert!(fd >= 0);

        let mut buf = [0 as c_char; 4096];
        let rv = read(fd, buf.as_mut_ptr().cast::<c_void>(), buf.len());
        assert!(0 <= rv && (rv as usize) < buf.len());
        buf[rv as usize] = '\0' as c_char;

        /* This test is very much not kernel thread. */
        assert!(!strstr(buf.as_ptr(), c"Kthread:\t0\n".as_ptr()).is_null());

        0
    }
}
