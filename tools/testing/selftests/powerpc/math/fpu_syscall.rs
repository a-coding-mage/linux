// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2015, Cyril Bur, IBM Corp.
 *
 * This test attempts to see if the FPU registers change across a syscall (fork).
 */

// C dependencies: stdio.h, unistd.h, sys/syscall.h, sys/time.h, sys/types.h,
// sys/wait.h, stdlib.h, "utils.h", "fpu.h".

use libc::{c_char, c_double, c_int, pid_t};

unsafe extern "C" {
    fn test_fpu(darray: *mut c_double, pid: *mut pid_t) -> c_int;
    fn randomise_darray(darray: *mut c_double, size: usize);
    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn exit(status: c_int) -> !;
    fn test_harness(
        test_function: unsafe extern "C" fn() -> c_int,
        name: *const c_char,
    ) -> c_int;
}

// FAIL_IF is supplied by "utils.h" in the original C source.
macro_rules! FAIL_IF {
    ($cond:expr) => {
        if $cond {
            return 1;
        }
    };
}

const DARRAY_SIZE: usize = 32;

#[no_mangle]
pub static mut darray: [c_double; DARRAY_SIZE] = [0.0; DARRAY_SIZE];

#[no_mangle]
pub unsafe extern "C" fn syscall_fpu() -> c_int {
    let mut fork_pid: pid_t;
    let mut i: c_int;
    let mut ret: c_int;
    let mut child_ret: c_int = 0;

    unsafe {
        randomise_darray(darray.as_mut_ptr(), DARRAY_SIZE);
    }

    i = 0;
    while i < 1000 {
        /* test_fpu will fork() */
        unsafe {
            ret = test_fpu(darray.as_mut_ptr(), &mut fork_pid);
        }
        if fork_pid == -1 {
            return -1;
        }
        if fork_pid == 0 {
            unsafe {
                exit(ret);
            }
        }
        unsafe {
            waitpid(fork_pid, &mut child_ret, 0);
        }
        if ret != 0 || child_ret != 0 {
            return 1;
        }

        i += 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn test_syscall_fpu() -> c_int {
    /*
     * Setup an environment with much context switching
     */
    let mut pid2: pid_t;
    let pid: pid_t = unsafe { fork() };
    let mut ret: c_int;
    let mut child_ret: c_int = 0;
    FAIL_IF!(pid == -1);

    pid2 = unsafe { fork() };
    /* Can't FAIL_IF(pid2 == -1); because already forked once */
    if pid2 == -1 {
        /*
         * Couldn't fork, ensure test is a fail
         */
        ret = 1;
        child_ret = ret;
    } else {
        ret = unsafe { syscall_fpu() };
        if pid2 != 0 {
            unsafe {
                waitpid(pid2, &mut child_ret, 0);
            }
        } else {
            unsafe {
                exit(ret);
            }
        }
    }

    ret |= child_ret;

    if pid != 0 {
        unsafe {
            waitpid(pid, &mut child_ret, 0);
        }
    } else {
        unsafe {
            exit(ret);
        }
    }

    FAIL_IF!(ret != 0 || child_ret != 0);
    0
}

#[no_mangle]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    unsafe { test_harness(test_syscall_fpu, c"syscall_fpu".as_ptr()) }
}
