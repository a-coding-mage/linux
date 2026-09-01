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
// Test that /proc/self gives correct TGID.
// C dependencies: assert.h, stdio.h, unistd.h, and "proc.h".

use core::ffi::{c_char, c_int, c_uint};

type pid_t = c_int;
type ssize_t = isize;

unsafe extern "C" {
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn readlink(path: *const c_char, buf: *mut c_char, bufsiz: usize) -> ssize_t;
    fn strlen(s: *const c_char) -> usize;

    fn sys_getpid() -> pid_t;
    fn streq(s1: *const c_char, s2: *const c_char) -> bool;
}

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    let mut buf1: [c_char; 64] = [0; 64];
    let mut buf2: [c_char; 64] = [0; 64];
    let mut pid: pid_t;
    let mut rv: ssize_t;

    pid = unsafe { sys_getpid() };
    unsafe {
        snprintf(
            buf1.as_mut_ptr(),
            core::mem::size_of_val(&buf1),
            b"%u\0".as_ptr() as *const c_char,
            pid as c_uint,
        );
    }

    rv = unsafe {
        readlink(
            b"/proc/self\0".as_ptr() as *const c_char,
            buf2.as_mut_ptr(),
            core::mem::size_of_val(&buf2),
        )
    };
    assert!(rv == unsafe { strlen(buf1.as_ptr()) } as ssize_t);
    buf2[rv as usize] = '\0' as c_char;
    assert!(unsafe { streq(buf1.as_ptr(), buf2.as_ptr()) });

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
