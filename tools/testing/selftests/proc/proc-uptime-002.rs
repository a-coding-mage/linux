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
// monotonically while shifting across CPUs. We don't test idle time
// monotonicity due to broken iowait task counting, cf: comment above
// get_cpu_idle_time_us()
// C source used `#undef NDEBUG` so assert() remains enabled.

use libc::{
    c_int, c_uint, c_ulong, c_void, free, malloc, memset, open, pid_t, syscall, EINVAL, O_RDONLY,
    SYS_sched_getaffinity, SYS_sched_setaffinity,
};

extern "C" {
    fn proc_uptime(fd: c_int) -> u64;
    fn clock_boottime() -> u64;
}

#[inline]
unsafe fn sys_sched_getaffinity(pid: pid_t, len: c_uint, m: *mut c_ulong) -> c_int {
    syscall(SYS_sched_getaffinity, pid, len, m) as c_int
}

#[inline]
unsafe fn sys_sched_setaffinity(pid: pid_t, len: c_uint, m: *mut c_ulong) -> c_int {
    syscall(SYS_sched_setaffinity, pid, len, m) as c_int
}

fn main() {
    unsafe {
        let mut u0: u64;
        let mut u1: u64;
        let mut c0: u64;
        let mut c1: u64;
        let mut len: c_uint;
        let mut m: *mut c_ulong;
        let mut cpu: c_uint;
        let fd: c_int;

        /* find out "nr_cpu_ids" */
        m = std::ptr::null_mut();
        len = 0;
        loop {
            len = len.wrapping_add(std::mem::size_of::<c_ulong>() as c_uint);
            free(m as *mut c_void);
            m = malloc(len as usize) as *mut c_ulong;
            if !(sys_sched_getaffinity(0, len, m) == -1
                && *libc::__errno_location() == EINVAL)
            {
                break;
            }
        }

        fd = open(b"/proc/uptime\0".as_ptr() as *const libc::c_char, O_RDONLY);
        assert!(fd >= 0);

        u0 = proc_uptime(fd);
        c0 = clock_boottime();

        cpu = 0;
        while cpu < len.wrapping_mul(8) {
            memset(m as *mut c_void, 0, len as usize);
            *m.add((cpu / (8 * std::mem::size_of::<c_ulong>() as c_uint)) as usize) |=
                1 as c_ulong
                    << (cpu % (8 * std::mem::size_of::<c_ulong>() as c_uint));

            /* CPU might not exist, ignore error */
            sys_sched_setaffinity(0, len, m);

            u1 = proc_uptime(fd);
            c1 = clock_boottime();

            /* Is /proc/uptime monotonic ? */
            assert!(u1 >= u0);

            /* Is CLOCK_BOOTTIME monotonic ? */
            assert!(c1 >= c0);

            /* Is CLOCK_BOOTTIME VS /proc/uptime monotonic ? */
            assert!(c0 >= u0);

            u0 = u1;
            c0 = c1;

            cpu = cpu.wrapping_add(1);
        }
    }
}
