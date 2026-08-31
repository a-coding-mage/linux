// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/intel_pstate/aperf.c.
// C dependencies: math.h, unistd.h, stdio.h, stdlib.h, sys/types.h,
// sys/stat.h, fcntl.h, sys/timeb.h, sched.h, errno.h, string.h, time.h,
// and kselftest.h.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use core::ffi::{c_char, c_double, c_int, c_long, c_uint, c_void};

const MSEC_PER_SEC: c_long = 1000;
const NSEC_PER_MSEC: c_long = 1000000;

const O_RDONLY: c_int = 0;
const CLOCK_MONOTONIC: clockid_t = 1;
const KSFT_SKIP: c_int = 4;

type size_t = usize;
type ssize_t = isize;
type off_t = i64;
type clockid_t = c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timespec {
    pub tv_sec: c_long,
    pub tv_nsec: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_set_t {
    pub __bits: [usize; 16],
}

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: off_t) -> ssize_t;
    fn sched_setaffinity(pid: c_int, cpusetsize: size_t, mask: *const cpu_set_t) -> c_int;
    fn clock_gettime(clk_id: clockid_t, tp: *mut timespec) -> c_int;
    fn sqrt(x: c_double) -> c_double;
    fn __errno_location() -> *mut c_int;
}

unsafe fn errno_location() -> *mut c_int {
    unsafe { __errno_location() }
}

unsafe fn CPU_ZERO(cpusetp: *mut cpu_set_t) {
    unsafe {
        for slot in (*cpusetp).__bits.iter_mut() {
            *slot = 0;
        }
    }
}

unsafe fn CPU_SET(cpu: c_uint, cpusetp: *mut cpu_set_t) {
    let bits_per_slot = 8 * core::mem::size_of::<usize>();
    let cpu = cpu as usize;

    unsafe {
        (*cpusetp).__bits[cpu / bits_per_slot] |= 1usize << (cpu % bits_per_slot);
    }
}

pub unsafe fn usage(name: *mut c_char) {
    unsafe {
        printf(c"Usage: %s cpunum\n".as_ptr(), name);
    }
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut i: c_uint;
    let mut cpu: c_uint;
    let mut fd: c_uint;
    let mut msr_file_name: [c_char; 64] = [0; 64];
    let mut tsc: i64;
    let mut old_tsc: i64 = 0;
    let mut new_tsc: i64 = 0;
    let mut aperf: i64;
    let mut old_aperf: i64 = 0;
    let mut new_aperf: i64 = 0;
    let mut mperf: i64;
    let mut old_mperf: i64 = 0;
    let mut new_mperf: i64 = 0;
    let mut before: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut after: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut start: i64;
    let mut finish: i64;
    let mut total: i64;
    let mut cpuset: cpu_set_t = cpu_set_t { __bits: [0; 16] };

    unsafe {
        if argc != 2 {
            usage(*argv.offset(0));
            return 1;
        }

        *errno_location() = 0;
        cpu = strtol(*argv.offset(1), core::ptr::null_mut(), 10) as c_uint;

        if *errno_location() != 0 {
            usage(*argv.offset(0));
            return 1;
        }

        sprintf(
            msr_file_name.as_mut_ptr(),
            c"/dev/cpu/%d/msr".as_ptr(),
            cpu,
        );
        fd = open(msr_file_name.as_ptr(), O_RDONLY) as c_uint;

        if fd == (-1i32) as c_uint {
            printf(
                c"/dev/cpu/%d/msr: %s\n".as_ptr(),
                cpu,
                strerror(*errno_location()),
            );
            return KSFT_SKIP;
        }

        CPU_ZERO(&mut cpuset);
        CPU_SET(cpu, &mut cpuset);

        if sched_setaffinity(0, core::mem::size_of::<cpu_set_t>(), &cpuset) != 0 {
            perror(c"Failed to set cpu affinity".as_ptr());
            return 1;
        }

        if clock_gettime(CLOCK_MONOTONIC, &mut before) < 0 {
            perror(c"clock_gettime".as_ptr());
            return 1;
        }
        pread(
            fd as c_int,
            &mut old_tsc as *mut i64 as *mut c_void,
            core::mem::size_of_val(&old_tsc),
            0x10,
        );
        pread(
            fd as c_int,
            &mut old_aperf as *mut i64 as *mut c_void,
            core::mem::size_of_val(&old_mperf),
            0xe7,
        );
        pread(
            fd as c_int,
            &mut old_mperf as *mut i64 as *mut c_void,
            core::mem::size_of_val(&old_aperf),
            0xe8,
        );

        i = 0;
        while i < 0x8fffffff {
            sqrt(i as c_double);
            i = i.wrapping_add(1);
        }

        if clock_gettime(CLOCK_MONOTONIC, &mut after) < 0 {
            perror(c"clock_gettime".as_ptr());
            return 1;
        }
        pread(
            fd as c_int,
            &mut new_tsc as *mut i64 as *mut c_void,
            core::mem::size_of_val(&new_tsc),
            0x10,
        );
        pread(
            fd as c_int,
            &mut new_aperf as *mut i64 as *mut c_void,
            core::mem::size_of_val(&new_mperf),
            0xe7,
        );
        pread(
            fd as c_int,
            &mut new_mperf as *mut i64 as *mut c_void,
            core::mem::size_of_val(&new_aperf),
            0xe8,
        );

        tsc = new_tsc - old_tsc;
        aperf = new_aperf - old_aperf;
        mperf = new_mperf - old_mperf;

        start = before.tv_sec * MSEC_PER_SEC as i64 + before.tv_nsec / NSEC_PER_MSEC as i64;
        finish = after.tv_sec * MSEC_PER_SEC as i64 + after.tv_nsec / NSEC_PER_MSEC as i64;
        total = finish - start;

        printf(
            c"runTime: %4.2f\n".as_ptr(),
            1.0f64 * total as f64 / MSEC_PER_SEC as f64,
        );
        printf(
            c"freq: %7.0f\n".as_ptr(),
            tsc as f64 / (1.0f64 * aperf as f64 / (1.0f64 * mperf as f64)) / total as f64,
        );
        return 0;
    }
}
