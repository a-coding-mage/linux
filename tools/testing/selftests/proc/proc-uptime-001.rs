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
// Test that boottime value in /proc/uptime and CLOCK_BOOTTIME increment
// monotonically. We don't test idle time monotonicity due to broken iowait
// task counting, cf: comment above get_cpu_idle_time_us()
// C source included assert.h, stdint.h, sys/types.h, sys/stat.h, fcntl.h,
// and "proc-uptime.h".

use std::ffi::c_char;
use std::os::raw::c_int;

const O_RDONLY: c_int = 0;

unsafe extern "C" {
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn proc_uptime(fd: c_int) -> u64;
    fn clock_boottime() -> u64;
}

fn main() {
    let mut start: u64;
    let mut u0: u64;
    let mut u1: u64;
    let mut c0: u64;
    let mut c1: u64;
    let fd: c_int;

    unsafe {
        fd = open(c"/proc/uptime".as_ptr(), O_RDONLY);
    }
    assert!(fd >= 0);

    unsafe {
        u0 = proc_uptime(fd);
    }
    start = u0;
    unsafe {
        c0 = clock_boottime();
    }

    loop {
        unsafe {
            u1 = proc_uptime(fd);
            c1 = clock_boottime();
        }

        /* Is /proc/uptime monotonic ? */
        assert!(u1 >= u0);

        /* Is CLOCK_BOOTTIME monotonic ? */
        assert!(c1 >= c0);

        /* Is CLOCK_BOOTTIME VS /proc/uptime monotonic ? */
        assert!(c0 >= u0);

        u0 = u1;
        c0 = c1;

        if !(u1.wrapping_sub(start) < 100) {
            break;
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
