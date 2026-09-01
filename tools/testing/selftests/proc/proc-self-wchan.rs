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

use std::ffi::c_void;
use std::os::raw::{c_char, c_int};

const O_RDONLY: c_int = 0;
const ENOENT: c_int = 2;

unsafe extern "C" {
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn __errno_location() -> *mut c_int;
}

fn main() -> std::process::ExitCode {
    let mut buf: [c_char; 64] = [0; 64];
    let fd: c_int;

    unsafe {
        fd = open(c"/proc/self/wchan".as_ptr(), O_RDONLY);
        if fd == -1 {
            if *__errno_location() == ENOENT {
                return std::process::ExitCode::from(4);
            }
            return std::process::ExitCode::from(1);
        }

        buf[0] = b'\0' as c_char;
        if read(fd, buf.as_mut_ptr().cast::<c_void>(), std::mem::size_of_val(&buf)) != 1 {
            return std::process::ExitCode::from(1);
        }
        if buf[0] != b'0' as c_char {
            return std::process::ExitCode::from(1);
        }
        std::process::ExitCode::from(0)
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
