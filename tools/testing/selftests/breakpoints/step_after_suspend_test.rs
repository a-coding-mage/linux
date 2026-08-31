// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2016 Google, Inc.
 */

// C source defined _GNU_SOURCE and included Linux/POSIX headers plus "kselftest.h".

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::null_mut;

const EIO: c_int = 5;
const SIGSTOP: c_int = 19;
const SIGTRAP: c_int = 5;
const CLOCK_BOOTTIME_ALARM: c_int = 9;
const PTRACE_TRACEME: c_uint = 0;
const PTRACE_CONT: c_uint = 7;
const PTRACE_SINGLESTEP: c_uint = 9;
const __WALL: c_int = 0x40000000;
const CPU_SETSIZE: usize = 1024;

const KSFT_PASS: c_int = 0;
const KSFT_FAIL: c_int = 1;
const KSFT_SKIP: c_int = 4;

type pid_t = c_int;
type time_t = c_long;

#[repr(C)]
struct timespec {
    tv_sec: time_t,
    tv_nsec: c_long,
}

#[repr(C)]
struct itimerspec {
    it_interval: timespec,
    it_value: timespec,
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct cpu_set_t {
    __bits: [c_ulong; CPU_SETSIZE / (8 * size_of::<c_ulong>())],
}

unsafe extern "C" {
    fn sched_setaffinity(pid: pid_t, cpusetsize: usize, mask: *const cpu_set_t) -> c_int;
    fn sched_getaffinity(pid: pid_t, cpusetsize: usize, mask: *mut cpu_set_t) -> c_int;
    fn ptrace(request: c_uint, ...) -> c_long;
    fn raise(sig: c_int) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn _exit(status: c_int) -> !;
    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, stat_loc: *mut c_int, options: c_int) -> pid_t;
    fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn getuid() -> c_uint;
    fn timerfd_create(clockid: c_int, flags: c_int) -> c_int;
    fn timerfd_settime(
        fd: c_int,
        flags: c_int,
        new_value: *const itimerspec,
        old_value: *mut itimerspec,
    ) -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn getopt(argc: c_int, argv: *const *mut c_char, optstring: *const c_char) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn __errno_location() -> *mut c_int;

    fn ksft_print_header();
    fn ksft_print_msg(format: *const c_char, ...);
    fn ksft_exit_fail_msg(format: *const c_char, ...) -> !;
    fn ksft_exit_skip(format: *const c_char, ...) -> !;
    fn ksft_set_plan(plan: c_uint);
    fn ksft_test_result_pass(format: *const c_char, ...);
    fn ksft_test_result_skip(format: *const c_char, ...);
    fn ksft_test_result_fail(format: *const c_char, ...);
    fn ksft_exit_pass() -> !;
    fn ksft_exit_fail() -> !;
}

unsafe fn errno() -> c_int {
    unsafe { *__errno_location() }
}

fn WIFSTOPPED(status: c_int) -> bool {
    (status & 0xff) == 0x7f
}

fn WSTOPSIG(status: c_int) -> c_int {
    (status >> 8) & 0xff
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn CPU_ZERO(set: *mut cpu_set_t) {
    unsafe {
        (*set).__bits.fill(0);
    }
}

unsafe fn CPU_SET(cpu: c_int, set: *mut cpu_set_t) {
    let cpu = cpu as usize;
    let bits_per_word = 8 * size_of::<c_ulong>();
    unsafe {
        (*set).__bits[cpu / bits_per_word] |= (1 as c_ulong) << (cpu % bits_per_word);
    }
}

unsafe fn CPU_ISSET(cpu: c_int, set: *const cpu_set_t) -> bool {
    let cpu = cpu as usize;
    let bits_per_word = 8 * size_of::<c_ulong>();
    unsafe { ((*set).__bits[cpu / bits_per_word] & ((1 as c_ulong) << (cpu % bits_per_word))) != 0 }
}

unsafe fn child(cpu: c_int) {
    let mut set: cpu_set_t = unsafe { zeroed() };

    unsafe {
        CPU_ZERO(&mut set);
        CPU_SET(cpu, &mut set);
        if sched_setaffinity(0, size_of::<cpu_set_t>(), &set) != 0 {
            ksft_print_msg(
                c"sched_setaffinity() failed: %s\n".as_ptr(),
                strerror(errno()),
            );
            _exit(1);
        }

        if ptrace(PTRACE_TRACEME, 0, null_mut::<c_void>(), null_mut::<c_void>()) != 0 {
            ksft_print_msg(
                c"ptrace(PTRACE_TRACEME) failed: %s\n".as_ptr(),
                strerror(errno()),
            );
            _exit(1);
        }

        if raise(SIGSTOP) != 0 {
            ksft_print_msg(c"raise(SIGSTOP) failed: %s\n".as_ptr(), strerror(errno()));
            _exit(1);
        }

        _exit(0);
    }
}

unsafe fn run_test(cpu: c_int) -> c_int {
    let mut status: c_int = 0;
    let pid: pid_t = unsafe { fork() };
    let mut wpid: pid_t;

    unsafe {
        if pid < 0 {
            ksft_print_msg(c"fork() failed: %s\n".as_ptr(), strerror(errno()));
            return KSFT_FAIL;
        }
        if pid == 0 {
            child(cpu);
        }

        wpid = waitpid(pid, &mut status, __WALL);
        if wpid != pid {
            ksft_print_msg(c"waitpid() failed: %s\n".as_ptr(), strerror(errno()));
            return KSFT_FAIL;
        }
        if !WIFSTOPPED(status) {
            ksft_print_msg(c"child did not stop: %s\n".as_ptr(), strerror(errno()));
            return KSFT_FAIL;
        }
        if WSTOPSIG(status) != SIGSTOP {
            ksft_print_msg(
                c"child did not stop with SIGSTOP: %s\n".as_ptr(),
                strerror(errno()),
            );
            return KSFT_FAIL;
        }

        if ptrace(
            PTRACE_SINGLESTEP,
            pid,
            null_mut::<c_void>(),
            null_mut::<c_void>(),
        ) < 0
        {
            if errno() == EIO {
                ksft_print_msg(
                    c"ptrace(PTRACE_SINGLESTEP) not supported on this architecture: %s\n".as_ptr(),
                    strerror(errno()),
                );
                return KSFT_SKIP;
            }
            ksft_print_msg(
                c"ptrace(PTRACE_SINGLESTEP) failed: %s\n".as_ptr(),
                strerror(errno()),
            );
            return KSFT_FAIL;
        }

        wpid = waitpid(pid, &mut status, __WALL);
        if wpid != pid {
            ksft_print_msg(c"waitpid() failed: %s\n".as_ptr(), strerror(errno()));
            return KSFT_FAIL;
        }
        if WIFEXITED(status) {
            ksft_print_msg(
                c"child did not single-step: %s\n".as_ptr(),
                strerror(errno()),
            );
            return KSFT_FAIL;
        }
        if !WIFSTOPPED(status) {
            ksft_print_msg(c"child did not stop: %s\n".as_ptr(), strerror(errno()));
            return KSFT_FAIL;
        }
        if WSTOPSIG(status) != SIGTRAP {
            ksft_print_msg(
                c"child did not stop with SIGTRAP: %s\n".as_ptr(),
                strerror(errno()),
            );
            return KSFT_FAIL;
        }

        if ptrace(PTRACE_CONT, pid, null_mut::<c_void>(), null_mut::<c_void>()) < 0 {
            ksft_print_msg(
                c"ptrace(PTRACE_CONT) failed: %s\n".as_ptr(),
                strerror(errno()),
            );
            return KSFT_FAIL;
        }

        wpid = waitpid(pid, &mut status, __WALL);
        if wpid != pid {
            ksft_print_msg(c"waitpid() failed: %s\n".as_ptr(), strerror(errno()));
            return KSFT_FAIL;
        }
        if !WIFEXITED(status) {
            ksft_print_msg(
                c"child did not exit after PTRACE_CONT: %s\n".as_ptr(),
                strerror(errno()),
            );
            return KSFT_FAIL;
        }

        KSFT_PASS
    }
}

/*
 * Reads the suspend success count from sysfs.
 * Returns the count on success or exits on failure.
 */
unsafe fn get_suspend_success_count_or_fail() -> c_int {
    let mut val: c_int = 0;

    unsafe {
        let fp = fopen(
            c"/sys/power/suspend_stats/success".as_ptr(),
            c"r".as_ptr(),
        );
        if fp.is_null() {
            ksft_exit_fail_msg(
                c"Failed to open suspend_stats/success: %s\n".as_ptr(),
                strerror(errno()),
            );
        }

        if fscanf(fp, c"%d".as_ptr(), &mut val) != 1 {
            fclose(fp);
            ksft_exit_fail_msg(c"Failed to read suspend success count\n".as_ptr());
        }

        fclose(fp);
        val
    }
}

unsafe fn suspend() {
    let timerfd: c_int;
    let err: c_int;
    let count_before: c_int;
    let count_after: c_int;
    let mut spec: itimerspec = unsafe { zeroed() };

    unsafe {
        if getuid() != 0 {
            ksft_exit_skip(c"Please run the test as root - Exiting.\n".as_ptr());
        }

        timerfd = timerfd_create(CLOCK_BOOTTIME_ALARM, 0);
        if timerfd < 0 {
            ksft_exit_fail_msg(c"timerfd_create() failed\n".as_ptr());
        }

        spec.it_value.tv_sec = 5;
        err = timerfd_settime(timerfd, 0, &spec, null_mut());
        if err < 0 {
            ksft_exit_fail_msg(c"timerfd_settime() failed\n".as_ptr());
        }

        count_before = get_suspend_success_count_or_fail();

        system(c"(echo mem > /sys/power/state) 2> /dev/null".as_ptr());

        count_after = get_suspend_success_count_or_fail();
        if count_after <= count_before {
            ksft_exit_fail_msg(c"Failed to enter Suspend state\n".as_ptr());
        }

        close(timerfd);
    }
}

unsafe fn main_impl(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut opt: c_int;
    let mut do_suspend: bool = true;
    let mut succeeded: bool = true;
    let mut tests: c_uint = 0;
    let mut available_cpus: cpu_set_t = unsafe { zeroed() };
    let mut err: c_int;
    let mut cpu: c_int;

    unsafe {
        ksft_print_header();

        loop {
            opt = getopt(argc, argv, c"n".as_ptr());
            if opt == -1 {
                break;
            }
            match opt {
                110 => {
                    do_suspend = false;
                }
                _ => {
                    printf(c"Usage: %s [-n]\n".as_ptr(), *argv);
                    printf(
                        c"        -n: do not trigger a suspend/resume cycle before the test\n"
                            .as_ptr(),
                    );
                    return -1;
                }
            }
        }

        err = sched_getaffinity(0, size_of::<cpu_set_t>(), &mut available_cpus);
        if err < 0 {
            ksft_exit_fail_msg(c"sched_getaffinity() failed\n".as_ptr());
        }

        cpu = 0;
        while cpu < CPU_SETSIZE as c_int {
            if !CPU_ISSET(cpu, &available_cpus) {
                cpu += 1;
                continue;
            }
            tests += 1;
            cpu += 1;
        }

        if do_suspend {
            suspend();
        }

        ksft_set_plan(tests);
        cpu = 0;
        while cpu < CPU_SETSIZE as c_int {
            let test_success: c_int;

            if !CPU_ISSET(cpu, &available_cpus) {
                cpu += 1;
                continue;
            }

            test_success = run_test(cpu);
            match test_success {
                KSFT_PASS => {
                    ksft_test_result_pass(c"CPU %d\n".as_ptr(), cpu);
                }
                KSFT_SKIP => {
                    ksft_test_result_skip(c"CPU %d\n".as_ptr(), cpu);
                }
                KSFT_FAIL => {
                    ksft_test_result_fail(c"CPU %d\n".as_ptr(), cpu);
                    succeeded = false;
                }
                _ => {}
            }
            cpu += 1;
        }

        if succeeded {
            ksft_exit_pass();
        } else {
            ksft_exit_fail();
        }
    }
}

fn main() {
    let args: Vec<*mut c_char> = std::env::args()
        .map(|arg| std::ffi::CString::new(arg).unwrap().into_raw())
        .collect();
    let mut argv = args;
    argv.push(null_mut());

    unsafe {
        let code = main_impl((argv.len() - 1) as c_int, argv.as_mut_ptr());
        std::process::exit(code);
    }
}
