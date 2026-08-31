// SPDX-License-Identifier: GPL-2.0
// C source used _GNU_SOURCE and included system/timens selftest headers.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_void};
use core::ptr;

type pid_t = c_int;
type size_t = usize;
type ssize_t = isize;
type clockid_t = c_int;
type timer_t = *mut c_void;

const SIGEV_NONE: c_int = 1;
const TIMER_ABSTIME: c_int = 1;
const ENOSYS: c_int = 38;
const O_WRONLY: c_int = 1;

const CLOCK_MONOTONIC: clockid_t = 1;
const CLOCK_BOOTTIME: clockid_t = 7;
const CLOCK_BOOTTIME_ALARM: clockid_t = 9;

#[repr(C)]
#[derive(Copy, Clone)]
struct timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct itimerspec {
    it_interval: timespec,
    it_value: timespec,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct sigevent {
    sigev_value: [usize; 4],
    sigev_signo: c_int,
    sigev_notify: c_int,
    sigev_un: [usize; 12],
}

unsafe extern "C" {
    fn check_skip(clockid: clockid_t) -> c_int;
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn pr_perror(fmt: *const c_char, ...) -> c_int;
    fn ksft_print_header();
    fn nscheck();
    fn check_supported_timers();
    fn ksft_set_plan(cnt: c_int);
    fn unshare_timens() -> c_int;
    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;

    fn clock_gettime(clk_id: clockid_t, tp: *mut timespec) -> c_int;
    fn timer_create(clockid: clockid_t, sevp: *mut sigevent, timerid: *mut timer_t) -> c_int;
    fn timer_settime(
        timerid: timer_t,
        flags: c_int,
        new_value: *const itimerspec,
        old_value: *mut itimerspec,
    ) -> c_int;
    fn timer_gettime(timerid: timer_t, curr_value: *mut itimerspec) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;
    fn __errno_location() -> *mut c_int;
}

unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn run_test(clockid: c_int, now: timespec) -> c_int {
    let mut new_value: itimerspec = unsafe { core::mem::zeroed() };
    let mut elapsed: i64;
    let mut fd: timer_t = ptr::null_mut();
    let mut i: c_int;

    if unsafe { check_skip(clockid) } != 0 {
        return 0;
    }

    i = 0;
    while i < 2 {
        let mut sevp: sigevent = unsafe { core::mem::zeroed() };
        sevp.sigev_notify = SIGEV_NONE;
        let mut flags: c_int = 0;

        new_value.it_value.tv_sec = 3600;
        new_value.it_value.tv_nsec = 0;
        new_value.it_interval.tv_sec = 1;
        new_value.it_interval.tv_nsec = 0;

        if i == 1 {
            new_value.it_value.tv_sec += now.tv_sec;
            new_value.it_value.tv_nsec += now.tv_nsec;
        }

        if unsafe { timer_create(clockid, &mut sevp, &mut fd) } == -1 {
            if unsafe { errno() } == ENOSYS {
                unsafe {
                    ksft_test_result_skip(c"Posix Clocks & timers are supported\n".as_ptr());
                }
                return 0;
            }
            return unsafe { pr_perror(c"timerfd_create".as_ptr()) };
        }

        if i == 1 {
            flags |= TIMER_ABSTIME;
        }
        if unsafe { timer_settime(fd, flags, &new_value, ptr::null_mut()) } == -1 {
            return unsafe { pr_perror(c"timerfd_settime".as_ptr()) };
        }

        if unsafe { timer_gettime(fd, &mut new_value) } == -1 {
            return unsafe { pr_perror(c"timerfd_gettime".as_ptr()) };
        }

        elapsed = new_value.it_value.tv_sec as i64;
        if (elapsed - 3600).abs() > 60 {
            unsafe {
                ksft_test_result_fail(
                    c"clockid: %d elapsed: %lld\n".as_ptr(),
                    clockid,
                    elapsed,
                );
            }
            return 1;
        }

        i += 1;
    }

    unsafe {
        ksft_test_result_pass(c"clockid=%d\n".as_ptr(), clockid);
    }

    0
}

unsafe fn c_main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut ret: c_int;
    let mut status: c_int = 0;
    let mut len: c_int;
    let mut fd: c_int;
    let mut buf = [0 as c_char; 4096];
    let mut pid: pid_t;
    let mut btime_now: timespec = unsafe { core::mem::zeroed() };
    let mut mtime_now: timespec = unsafe { core::mem::zeroed() };

    unsafe {
        ksft_print_header();
    }

    unsafe {
        nscheck();
    }

    unsafe {
        check_supported_timers();
    }

    unsafe {
        ksft_set_plan(3);
    }

    unsafe {
        clock_gettime(CLOCK_MONOTONIC, &mut mtime_now);
        clock_gettime(CLOCK_BOOTTIME, &mut btime_now);
    }

    if unsafe { unshare_timens() } != 0 {
        return 1;
    }

    len = unsafe {
        snprintf(
            buf.as_mut_ptr(),
            buf.len(),
            c"%d %d 0\n%d %d 0".as_ptr(),
            CLOCK_MONOTONIC,
            70 * 24 * 3600,
            CLOCK_BOOTTIME,
            9 * 24 * 3600,
        )
    };
    fd = unsafe { open(c"/proc/self/timens_offsets".as_ptr(), O_WRONLY) };
    if fd < 0 {
        return unsafe { pr_perror(c"/proc/self/timens_offsets".as_ptr()) };
    }

    if unsafe { write(fd, buf.as_ptr() as *const c_void, len as size_t) } != len as ssize_t {
        return unsafe { pr_perror(c"/proc/self/timens_offsets".as_ptr()) };
    }

    unsafe {
        close(fd);
    }
    mtime_now.tv_sec += 70 * 24 * 3600;
    btime_now.tv_sec += 9 * 24 * 3600;

    pid = unsafe { fork() };
    if pid < 0 {
        return unsafe { pr_perror(c"Unable to fork".as_ptr()) };
    }
    if pid == 0 {
        ret = 0;
        ret |= unsafe { run_test(CLOCK_BOOTTIME, btime_now) };
        ret |= unsafe { run_test(CLOCK_MONOTONIC, mtime_now) };
        ret |= unsafe { run_test(CLOCK_BOOTTIME_ALARM, btime_now) };

        if ret != 0 {
            unsafe {
                ksft_exit_fail();
            }
        }
        unsafe {
            ksft_exit_pass();
        }
    }

    if unsafe { waitpid(pid, &mut status, 0) } != pid {
        return unsafe { pr_perror(c"Unable to wait the child process".as_ptr()) };
    }

    if WIFEXITED(status) {
        return WEXITSTATUS(status);
    }

    1
}

fn main() {
    let ret = unsafe { c_main(0, ptr::null_mut()) };
    std::process::exit(ret);
}
