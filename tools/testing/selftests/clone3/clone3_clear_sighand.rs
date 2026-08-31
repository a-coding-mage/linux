/* SPDX-License-Identifier: GPL-2.0 */

/* C dependencies: errno.h, sched.h, signal.h, stdio.h, stdlib.h, string.h,
 * unistd.h, linux/sched.h, linux/types.h, sys/syscall.h, sys/wait.h,
 * kselftest.h, clone3_selftests.h
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

type pid_t = c_int;

const EINTR: c_int = 4;
const SIGCHLD: u64 = 17;
const SIGUSR1: c_int = 10;
const SIGUSR2: c_int = 12;
const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;

const CLONE_SIGHAND: u64 = 0x0000_0800;
const CLONE_CLEAR_SIGHAND: u64 = 0x1_0000_0000;

const SIG_DFL: usize = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigset_t {
    __val: [u64; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigaction {
    pub sa_handler: usize,
    pub sa_flags: u64,
    pub sa_restorer: Option<unsafe extern "C" fn()>,
    pub sa_mask: sigset_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __clone_args {
    pub flags: u64,
    pub pidfd: u64,
    pub child_tid: u64,
    pub parent_tid: u64,
    pub exit_signal: u64,
    pub stack: u64,
    pub stack_size: u64,
    pub tls: u64,
    pub set_tid: u64,
    pub set_tid_size: u64,
    pub cgroup: u64,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn exit(status: c_int) -> !;

    fn sys_clone3(args: *mut __clone_args, size: usize) -> pid_t;
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn test_clone3_supported();
    fn ksft_exit_pass() -> !;
    fn ksft_exit_fail_msg(msg: *const c_char, ...) -> !;
    fn ksft_test_result_pass(msg: *const c_char, ...);
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe extern "C" fn nop_handler(_signo: c_int) {}

unsafe fn wait_for_pid(pid: pid_t) -> c_int {
    let mut status: c_int = 0;
    let mut ret: c_int;

    loop {
        ret = unsafe { waitpid(pid, &mut status, 0) };
        if ret == -1 {
            if unsafe { errno } == EINTR {
                continue;
            }

            return -1;
        }

        break;
    }

    if !WIFEXITED(status) {
        return -1;
    }

    WEXITSTATUS(status)
}

unsafe fn test_clone3_clear_sighand() {
    let mut ret: c_int;
    let mut pid: pid_t;
    let mut args: __clone_args = unsafe { core::mem::zeroed() };
    let mut act: sigaction = unsafe { core::mem::zeroed() };

    /*
     * Check that CLONE_CLEAR_SIGHAND and CLONE_SIGHAND are mutually
     * exclusive.
     */
    args.flags |= CLONE_CLEAR_SIGHAND | CLONE_SIGHAND;
    args.exit_signal = SIGCHLD;
    pid = unsafe { sys_clone3(&mut args, size_of::<__clone_args>()) };
    if pid > 0 {
        unsafe {
            ksft_exit_fail_msg(
                c"clone3(CLONE_CLEAR_SIGHAND | CLONE_SIGHAND) succeeded\n".as_ptr(),
            );
        }
    }

    act.sa_handler = nop_handler as usize;
    ret = unsafe { sigemptyset(&mut act.sa_mask) };
    if ret < 0 {
        unsafe {
            ksft_exit_fail_msg(
                c"%s - sigemptyset() failed\n".as_ptr(),
                strerror(errno) as *mut c_void,
            );
        }
    }

    act.sa_flags = 0;

    /* Register signal handler for SIGUSR1 */
    ret = unsafe { sigaction(SIGUSR1, &act, ptr::null_mut()) };
    if ret < 0 {
        unsafe {
            ksft_exit_fail_msg(
                c"%s - sigaction(SIGUSR1, &act, NULL) failed\n".as_ptr(),
                strerror(errno) as *mut c_void,
            );
        }
    }

    /* Register signal handler for SIGUSR2 */
    ret = unsafe { sigaction(SIGUSR2, &act, ptr::null_mut()) };
    if ret < 0 {
        unsafe {
            ksft_exit_fail_msg(
                c"%s - sigaction(SIGUSR2, &act, NULL) failed\n".as_ptr(),
                strerror(errno) as *mut c_void,
            );
        }
    }

    /* Check that CLONE_CLEAR_SIGHAND works. */
    args.flags = CLONE_CLEAR_SIGHAND;
    pid = unsafe { sys_clone3(&mut args, size_of::<__clone_args>()) };
    if pid < 0 {
        unsafe {
            ksft_exit_fail_msg(
                c"%s - clone3(CLONE_CLEAR_SIGHAND) failed\n".as_ptr(),
                strerror(errno) as *mut c_void,
            );
        }
    }

    if pid == 0 {
        ret = unsafe { sigaction(SIGUSR1, ptr::null(), &mut act) };
        if ret < 0 {
            unsafe { exit(EXIT_FAILURE) };
        }

        if act.sa_handler != SIG_DFL {
            unsafe { exit(EXIT_FAILURE) };
        }

        ret = unsafe { sigaction(SIGUSR2, ptr::null(), &mut act) };
        if ret < 0 {
            unsafe { exit(EXIT_FAILURE) };
        }

        if act.sa_handler != SIG_DFL {
            unsafe { exit(EXIT_FAILURE) };
        }

        unsafe { exit(EXIT_SUCCESS) };
    }

    ret = unsafe { wait_for_pid(pid) };
    if ret != 0 {
        unsafe {
            ksft_exit_fail_msg(
                c"Failed to clear signal handler for child process\n".as_ptr(),
            );
        }
    }

    unsafe {
        ksft_test_result_pass(c"Cleared signal handlers for child process\n".as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    unsafe {
        ksft_print_header();
        ksft_set_plan(1);
        test_clone3_supported();

        test_clone3_clear_sighand();

        ksft_exit_pass();
    }
}
