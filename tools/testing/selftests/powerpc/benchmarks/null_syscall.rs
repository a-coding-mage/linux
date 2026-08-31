// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Test null syscall performance
 *
 * Copyright (C) 2009-2015 Anton Blanchard, IBM
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::arch::asm;
use core::ffi::{c_char, c_double, c_int, c_long, c_ulong, c_ulonglong, c_void};
use core::ptr;

const NR_LOOPS: c_ulong = 10000000;

const SIGALRM: c_int = 14;
const SIG_DFL: sighandler_t = 0 as sighandler_t;
const ITIMER_REAL: c_int = 0;
const CLOCK_MONOTONIC: clockid_t = 1;
const __NR_gettid: c_long = 207;

type size_t = usize;
type time_t = c_long;
type suseconds_t = c_long;
type clockid_t = c_int;
type sighandler_t = Option<unsafe extern "C" fn(c_int)>;

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct timeval {
    tv_sec: time_t,
    tv_usec: suseconds_t,
}

#[repr(C)]
struct itimerval {
    it_interval: timeval,
    it_value: timeval,
}

#[repr(C)]
struct timespec {
    tv_sec: time_t,
    tv_nsec: c_long,
}

unsafe extern "C" {
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulonglong;
    fn strtod(nptr: *const c_char, endptr: *mut *mut c_char) -> c_double;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fclose(stream: *mut FILE) -> c_int;
    fn getenv(name: *const c_char) -> *mut c_char;
    fn signal(signum: c_int, handler: sighandler_t) -> sighandler_t;
    fn setitimer(
        which: c_int,
        new_value: *const itimerval,
        old_value: *mut itimerval,
    ) -> c_int;
    fn syscall(number: c_long, ...) -> c_long;
    fn clock_gettime(clk_id: clockid_t, tp: *mut timespec) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
}

static mut soak_done: c_int = 0;
static mut clock_frequency: c_ulonglong = 0;
static mut timebase_frequency: c_ulonglong = 0;
static mut timebase_multiplier: c_double = 0.0;

#[inline]
unsafe fn mftb() -> c_ulong {
    let low: c_ulong;

    unsafe {
        asm!("mftb {0}", out(reg) low, options(nostack, preserves_flags));
    }

    low
}

unsafe extern "C" fn sigalrm_handler(_unused: c_int) {
    unsafe {
        ptr::write_volatile(ptr::addr_of_mut!(soak_done), 1);
    }
}

/*
 * Use a timer instead of busy looping on clock_gettime() so we don't
 * pollute profiles with glibc and VDSO hits.
 */
unsafe fn cpu_soak_usecs(usecs: c_ulong) {
    let mut val = itimerval {
        it_interval: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        it_value: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
    };

    unsafe {
        memset(
            &mut val as *mut itimerval as *mut c_void,
            0,
            core::mem::size_of::<itimerval>(),
        );
    }
    val.it_value.tv_usec = usecs as suseconds_t;

    unsafe {
        signal(SIGALRM, Some(sigalrm_handler));
        setitimer(ITIMER_REAL, &val, ptr::null_mut());

        loop {
            if ptr::read_volatile(ptr::addr_of!(soak_done)) != 0 {
                break;
            }
        }

        signal(SIGALRM, SIG_DFL);
    }
}

/*
 * This only works with recent kernels where cpufreq modifies
 * /proc/cpuinfo dynamically.
 */
unsafe fn get_proc_frequency() {
    let mut line = [0 as c_char; 128];
    let mut p: *mut c_char;
    let mut end: *mut c_char = ptr::null_mut();
    let mut v: c_ulong;
    let mut d: c_double;
    let override_: *mut c_char;

    /* Try to get out of low power/low frequency mode */
    unsafe {
        cpu_soak_usecs((0.25 * 1000000.0) as c_ulong);
    }

    let f = unsafe { fopen(c"/proc/cpuinfo".as_ptr(), c"r".as_ptr()) };
    if f.is_null() {
        return;
    }

    unsafe {
        timebase_frequency = 0;
    }

    while unsafe { !fgets(line.as_mut_ptr(), line.len() as c_int, f).is_null() } {
        if unsafe { strncmp(line.as_ptr(), c"timebase".as_ptr(), 8) } == 0 {
            p = unsafe { strchr(line.as_ptr(), ':' as c_int) };
            if !p.is_null() {
                v = unsafe { strtoull(p.add(1), &mut end, 0) as c_ulong };
                if end != unsafe { p.add(1) } {
                    unsafe {
                        timebase_frequency = v as c_ulonglong;
                    }
                }
            }
        }

        if unsafe { strncmp(line.as_ptr(), c"clock".as_ptr(), 5) } == 0
            || unsafe { strncmp(line.as_ptr(), c"cpu MHz".as_ptr(), 7) } == 0
        {
            p = unsafe { strchr(line.as_ptr(), ':' as c_int) };
            if !p.is_null() {
                d = unsafe { strtod(p.add(1), &mut end) };
                if end != unsafe { p.add(1) } {
                    /* Find fastest clock frequency */
                    if unsafe { (d * 1000000_u64 as c_double) > clock_frequency as c_double } {
                        unsafe {
                            clock_frequency = (d * 1000000_u64 as c_double) as c_ulonglong;
                        }
                    }
                }
            }
        }
    }

    unsafe {
        fclose(f);
    }

    override_ = unsafe { getenv(c"FREQUENCY".as_ptr()) };
    if !override_.is_null() {
        unsafe {
            clock_frequency = strtoull(override_, ptr::null_mut(), 10);
        }
    }

    unsafe {
        if timebase_frequency != 0 {
            timebase_multiplier = clock_frequency as c_double / timebase_frequency as c_double;
        } else {
            timebase_multiplier = 1.0;
        }
    }
}

unsafe fn do_null_syscall(nr: c_ulong) {
    let mut i: c_ulong;

    i = 0;
    while i < nr {
        unsafe {
            syscall(__NR_gettid);
        }
        i += 1;
    }
}

// #define TIME(A, STR) \

fn main() -> c_int {
    let mut tb_start: c_ulong;
    let mut tb_now: c_ulong;
    let mut tv_start = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut tv_now = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let elapsed_ns: c_ulonglong;
    let elapsed_tb: c_ulonglong;

    unsafe {
        get_proc_frequency();

        clock_gettime(CLOCK_MONOTONIC, &mut tv_start);
        tb_start = mftb();

        do_null_syscall(NR_LOOPS);

        clock_gettime(CLOCK_MONOTONIC, &mut tv_now);
        tb_now = mftb();

        elapsed_ns = ((tv_now.tv_sec - tv_start.tv_sec) as c_ulonglong)
            .wrapping_mul(1000000000_u64)
            .wrapping_add((tv_now.tv_nsec - tv_start.tv_nsec) as c_ulonglong);
        elapsed_tb = tb_now.wrapping_sub(tb_start) as c_ulonglong;

        printf(
            c"%10.2f ns %10.2f cycles\n".as_ptr(),
            elapsed_ns as f32 as c_double / NR_LOOPS as c_double,
            elapsed_tb as f32 as c_double * timebase_multiplier / NR_LOOPS as c_double,
        );
    }

    0
}
