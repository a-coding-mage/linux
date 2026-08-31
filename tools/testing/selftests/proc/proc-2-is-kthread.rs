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
/* Test that kernel thread is reported as such. */

use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_void};

const O_RDONLY: c_int = 0;

unsafe extern "C" {
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
}

fn main() {
    /*
     * The following solutions don't really work:
     *
     * 1) jit kernel module which creates kernel thread:
     * test becomes arch-specific,
     * problems with mandatory module signing,
     * problems with lockdown mode,
     * doesn't work with CONFIG_MODULES=n at all,
     * kthread creation API is formally unstable internal kernel API,
     * need a mechanism to report test kernel thread's PID back,
     *
     * 2) ksoftirqd/0 and kswapd0 look like stable enough kernel threads,
     * but their PIDs are unstable.
     *
     * Check against kthreadd which always seem to exist under pid 2.
     */
    unsafe {
        let fd = open(c"/proc/2/status".as_ptr(), O_RDONLY);
        assert!(fd >= 0);

        let mut buf = [0 as c_char; 4096];
        let rv = read(fd, buf.as_mut_ptr().cast::<c_void>(), buf.len());
        assert!(0 <= rv && (rv as usize) < buf.len());
        buf[rv as usize] = b'\0' as c_char;

        assert!(!strstr(
            CStr::from_ptr(buf.as_ptr()).as_ptr(),
            c"Kthread:\t1\n".as_ptr()
        )
        .is_null());
    }
}
