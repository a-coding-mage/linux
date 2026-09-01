// SPDX-License-Identifier: GPL-2.0
// C source dependencies: sched.h, linux/unistd.h, linux/futex.h, stdio.h,
// string.h, sys/syscall.h, sys/types.h, sys/wait.h, time.h, unistd.h,
// "log.h", and "timens.h".

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

type pid_t = c_int;
type size_t = usize;
type ssize_t = isize;

const NSEC_PER_SEC: u64 = 1000000000_u64;

const CLOCK_REALTIME: c_int = 0;
const CLOCK_MONOTONIC: c_int = 1;
const ETIMEDOUT: c_int = 110;
const FUTEX_WAIT_BITSET: c_int = 9;
const FUTEX_CLOCK_REALTIME: c_int = 256;
const FUTEX_BITSET_MATCH_ANY: c_uint = 0xffffffff;
const __NR_futex: c_long = 202;
const O_WRONLY: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
struct timespec {
    tv_sec: c_long,
    tv_nsec: c_long,
}

unsafe extern "C" {
    fn clock_gettime(clockid: c_int, tp: *mut timespec) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;
    fn __errno_location() -> *mut c_int;

    fn ksft_print_header();
    fn ksft_set_plan(cnt: c_uint);
    fn ksft_test_result_fail(msg: *const c_char, ...);
    fn ksft_test_result_pass(msg: *const c_char, ...);
    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;
    fn pr_perror(msg: *const c_char) -> c_int;
    fn nscheck();
    fn unshare_timens() -> c_int;
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

unsafe fn run_test(clockid: c_int) -> c_int {
    let mut futex_op: c_int = FUTEX_WAIT_BITSET;
    let mut timeout: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut end: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut val: c_int = 0;

    if clockid == CLOCK_REALTIME {
        futex_op |= FUTEX_CLOCK_REALTIME;
    }

    unsafe {
        clock_gettime(clockid, &mut timeout);
    }
    timeout.tv_nsec += (NSEC_PER_SEC / 10) as c_long; // 100ms
    if timeout.tv_nsec > NSEC_PER_SEC as c_long {
        timeout.tv_sec += 1;
        timeout.tv_nsec -= NSEC_PER_SEC as c_long;
    }

    if unsafe {
        syscall(
            __NR_futex,
            &mut val as *mut c_int,
            futex_op,
            0,
            &mut timeout as *mut timespec,
            0,
            FUTEX_BITSET_MATCH_ANY,
        )
    } >= 0
    {
        unsafe {
            ksft_test_result_fail(c"futex didn't return ETIMEDOUT\n".as_ptr());
        }
        return 1;
    }

    if unsafe { errno() } != ETIMEDOUT {
        unsafe {
            ksft_test_result_fail(
                c"futex didn't return ETIMEDOUT: %s\n".as_ptr(),
                strerror(errno()),
            );
        }
        return 1;
    }

    unsafe {
        clock_gettime(clockid, &mut end);
    }

    if end.tv_sec < timeout.tv_sec
        || (end.tv_sec == timeout.tv_sec && end.tv_nsec < timeout.tv_nsec)
    {
        unsafe {
            ksft_test_result_fail(c"futex slept less than 100ms\n".as_ptr());
        }
        return 1;
    }

    unsafe {
        ksft_test_result_pass(c"futex with the %d clockid\n".as_ptr(), clockid);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut status: c_int = 0;
    let mut len: c_int;
    let mut fd: c_int;
    let mut buf: [c_char; 4096] = [0; 4096];
    let mut pid: pid_t;
    let mut mtime_now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    unsafe {
        ksft_print_header();
    }

    unsafe {
        nscheck();
    }

    unsafe {
        ksft_set_plan(2);
    }

    unsafe {
        clock_gettime(CLOCK_MONOTONIC, &mut mtime_now);
    }

    if unsafe { unshare_timens() } != 0 {
        return 1;
    }

    len = unsafe {
        snprintf(
            buf.as_mut_ptr(),
            buf.len(),
            c"%d %d 0".as_ptr(),
            CLOCK_MONOTONIC,
            70 * 24 * 3600,
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

    pid = unsafe { fork() };
    if pid < 0 {
        return unsafe { pr_perror(c"Unable to fork".as_ptr()) };
    }
    if pid == 0 {
        let mut ret: c_int = 0;

        ret |= unsafe { run_test(CLOCK_REALTIME) };
        ret |= unsafe { run_test(CLOCK_MONOTONIC) };
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

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
