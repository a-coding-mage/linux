/*
 * Copyright © 2020 Alexey Gladkov <gladkov.alexey@gmail.com>
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

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

const __NR_fsopen: c_long = 430;
const __NR_fsconfig: c_long = 431;
const FSCONFIG_SET_STRING: c_uint = 1;
const FSCONFIG_SET_BINARY: c_uint = 3;
const EINVAL: c_int = 22;

unsafe extern "C" {
    fn syscall(num: c_long, ...) -> c_long;
    fn close(fd: c_int) -> c_int;
    fn __errno_location() -> *mut c_int;
}

#[inline]
unsafe fn fsopen(fsname: *const c_char, flags: c_uint) -> c_int {
    unsafe { syscall(__NR_fsopen, fsname, flags) as c_int }
}

#[inline]
unsafe fn fsconfig(
    fd: c_int,
    cmd: c_uint,
    key: *const c_char,
    val: *const c_void,
    aux: c_int,
) -> c_int {
    unsafe { syscall(__NR_fsconfig, fd, cmd, key, val, aux) as c_int }
}

fn main() {
    let mut fsfd: c_int;
    let mut ret: c_int;
    let hidepid: c_int = 2;

    unsafe {
        fsfd = fsopen(c"proc".as_ptr(), 0);
    }
    assert!(fsfd != -1);

    unsafe {
        ret = fsconfig(
            fsfd,
            FSCONFIG_SET_BINARY,
            c"hidepid".as_ptr(),
            (&hidepid as *const c_int).cast::<c_void>(),
            0,
        );
    }
    assert!(ret == -1);
    unsafe {
        assert!(*__errno_location() == EINVAL);
    }

    unsafe {
        assert!(
            fsconfig(
                fsfd,
                FSCONFIG_SET_STRING,
                c"hidepid".as_ptr(),
                c"2".as_ptr().cast::<c_void>(),
                0,
            ) == 0
        );
        assert!(
            fsconfig(
                fsfd,
                FSCONFIG_SET_STRING,
                c"hidepid".as_ptr(),
                c"invisible".as_ptr().cast::<c_void>(),
                0,
            ) == 0
        );

        assert!(close(fsfd) == 0);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
