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

// C header dependencies translated as external declarations:
// assert.h, errno.h, string.h, stdlib.h, unistd.h, time.h, and "proc.h".

#[repr(C)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

unsafe extern "C" {
    pub static CLOCK_BOOTTIME: i32;

    pub fn clock_gettime(clk_id: i32, tp: *mut timespec) -> i32;
    pub fn memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    pub fn pread(
        fd: i32,
        buf: *mut core::ffi::c_void,
        count: usize,
        offset: isize,
    ) -> isize;
    pub fn xstrtoull(
        p: *const core::ffi::c_char,
        endptr: *mut *mut core::ffi::c_char,
    ) -> u64;
}

pub unsafe fn clock_boottime() -> u64 {
    let mut ts: timespec = core::mem::zeroed();
    let err: i32;

    err = clock_gettime(unsafe { CLOCK_BOOTTIME }, &mut ts);
    assert!(err >= 0);

    ((ts.tv_sec * 100) + (ts.tv_nsec / 10000000)) as u64
}

pub unsafe fn proc_uptime(fd: i32) -> u64 {
    let val1: u64;
    let val2: u64;
    let mut buf: [core::ffi::c_char; 64] = [0; 64];
    let mut p: *mut core::ffi::c_char;
    let rv: isize;

    /* save "p < end" checks */
    memset(buf.as_mut_ptr() as *mut core::ffi::c_void, 0, core::mem::size_of_val(&buf));
    rv = pread(
        fd,
        buf.as_mut_ptr() as *mut core::ffi::c_void,
        core::mem::size_of_val(&buf),
        0,
    );
    assert!(0 <= rv && (rv as usize) <= core::mem::size_of_val(&buf));
    buf[core::mem::size_of_val(&buf) - 1] = b'\0' as core::ffi::c_char;

    p = buf.as_mut_ptr();

    val1 = xstrtoull(p, &mut p);
    assert!(*p.add(0) == b'.' as core::ffi::c_char);
    assert!(b'0' as core::ffi::c_char <= *p.add(1) && *p.add(1) <= b'9' as core::ffi::c_char);
    assert!(b'0' as core::ffi::c_char <= *p.add(2) && *p.add(2) <= b'9' as core::ffi::c_char);
    assert!(*p.add(3) == b' ' as core::ffi::c_char);

    val2 = ((*p.add(1) - b'0' as core::ffi::c_char) * 10
        + *p.add(2)
        - b'0' as core::ffi::c_char) as u64;

    val1 * 100 + val2
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
