// SPDX-License-Identifier: GPL-2.0
// Translated from testing/selftests/timens/timerfd.c.
// C dependencies included: sched.h, sys/timerfd.h, sys/syscall.h,
// sys/types.h, sys/wait.h, time.h, unistd.h, stdlib.h, stdio.h, stdint.h,
// "log.h", and "timens.h".

use libc::{
    c_char, c_int, c_longlong, c_void, clockid_t, itimerspec, pid_t, timespec, CLOCK_BOOTTIME,
    CLOCK_BOOTTIME_ALARM, CLOCK_MONOTONIC, O_WRONLY, TFD_TIMER_ABSTIME,
};

unsafe extern "C" {
    fn check_skip(clockid: c_int) -> c_int;
    fn pr_perror(fmt: *const c_char, ...) -> c_int;
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_print_header();
    fn nscheck();
    fn check_supported_timers();
    fn ksft_set_plan(plan: c_int);
    fn unshare_timens() -> c_int;
    fn ksft_exit_fail() -> !;
    fn ksft_exit_pass() -> !;
}

unsafe fn tclock_gettime(mut clockid: clockid_t, now: *mut timespec) -> c_int {
    if clockid == CLOCK_BOOTTIME_ALARM {
        clockid = CLOCK_BOOTTIME;
    }
    unsafe { libc::clock_gettime(clockid, now) }
}

unsafe fn run_test(clockid: c_int, mut now: timespec) -> c_int {
    let mut new_value: itimerspec = unsafe { std::mem::zeroed() };
    let mut elapsed: c_longlong;
    let mut fd: c_int;
    let mut i: c_int;

    if unsafe { check_skip(clockid) } != 0 {
        return 0;
    }

    if unsafe { tclock_gettime(clockid, &mut now) } != 0 {
        return unsafe { pr_perror(c"clock_gettime(%d)".as_ptr(), clockid) };
    }

    i = 0;
    while i < 2 {
        let mut flags: c_int = 0;

        new_value.it_value.tv_sec = 3600;
        new_value.it_value.tv_nsec = 0;
        new_value.it_interval.tv_sec = 1;
        new_value.it_interval.tv_nsec = 0;

        if i == 1 {
            new_value.it_value.tv_sec += now.tv_sec;
            new_value.it_value.tv_nsec += now.tv_nsec;
        }

        fd = unsafe { libc::timerfd_create(clockid, 0) };
        if fd == -1 {
            return unsafe { pr_perror(c"timerfd_create(%d)".as_ptr(), clockid) };
        }

        if i == 1 {
            flags |= TFD_TIMER_ABSTIME;
        }

        if unsafe { libc::timerfd_settime(fd, flags, &new_value, std::ptr::null_mut()) } != 0 {
            return unsafe { pr_perror(c"timerfd_settime(%d)".as_ptr(), clockid) };
        }

        if unsafe { libc::timerfd_gettime(fd, &mut new_value) } != 0 {
            return unsafe { pr_perror(c"timerfd_gettime(%d)".as_ptr(), clockid) };
        }

        elapsed = new_value.it_value.tv_sec as c_longlong;
        if unsafe { libc::llabs(elapsed - 3600) } > 60 {
            unsafe {
                ksft_test_result_fail(
                    c"clockid: %d elapsed: %lld\n".as_ptr(),
                    clockid,
                    elapsed,
                );
            }
            return 1;
        }

        unsafe {
            libc::close(fd);
        }

        i += 1;
    }

    unsafe {
        ksft_test_result_pass(c"clockid=%d\n".as_ptr(), clockid);
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut ret: c_int;
    let mut status: c_int = 0;
    let mut len: c_int;
    let mut fd: c_int;
    let mut buf = [0 as c_char; 4096];
    let mut pid: pid_t;
    let mut btime_now: timespec = unsafe { std::mem::zeroed() };
    let mut mtime_now: timespec = unsafe { std::mem::zeroed() };

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
        libc::clock_gettime(CLOCK_MONOTONIC, &mut mtime_now);
        libc::clock_gettime(CLOCK_BOOTTIME, &mut btime_now);
    }

    if unsafe { unshare_timens() } != 0 {
        return 1;
    }

    len = unsafe {
        libc::snprintf(
            buf.as_mut_ptr(),
            buf.len(),
            c"%d %d 0\n%d %d 0".as_ptr(),
            CLOCK_MONOTONIC,
            70 * 24 * 3600,
            CLOCK_BOOTTIME,
            9 * 24 * 3600,
        )
    };
    fd = unsafe { libc::open(c"/proc/self/timens_offsets".as_ptr(), O_WRONLY) };
    if fd < 0 {
        return unsafe { pr_perror(c"/proc/self/timens_offsets".as_ptr()) };
    }

    if unsafe { libc::write(fd, buf.as_ptr() as *const c_void, len as usize) } != len as isize {
        return unsafe { pr_perror(c"/proc/self/timens_offsets".as_ptr()) };
    }

    unsafe {
        libc::close(fd);
    }
    mtime_now.tv_sec += 70 * 24 * 3600;
    btime_now.tv_sec += 9 * 24 * 3600;

    pid = unsafe { libc::fork() };
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

    if unsafe { libc::waitpid(pid, &mut status, 0) } != pid {
        return unsafe { pr_perror(c"Unable to wait the child process".as_ptr()) };
    }

    if libc::WIFEXITED(status) {
        return libc::WEXITSTATUS(status);
    }

    1
}
