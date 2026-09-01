// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2022 ARM Limited.
 * Original author: Mark Brown <broonie@kernel.org>
 */

// SPDX-License-Identifier: GPL-2.0-only

// C dependencies:
// #include <linux/sched.h>
// #include <linux/wait.h>
// #include "kselftest.h"

type c_char = i8;
type c_int = i32;
type pid_t = c_int;

const EXPECTED_TESTS: c_int = 1;
const EINTR: c_int = 4;
const O_RDONLY: c_int = 0;

unsafe extern "C" {
    fn fork_test() -> c_int;
    fn verify_fork() -> c_int;

    fn fork() -> pid_t;
    fn exit(status: c_int) -> !;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn getpid() -> pid_t;
    fn open(pathname: *const c_char, flags: c_int, mode: c_int) -> c_int;

    static mut errno: c_int;

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_test_result(result: c_int, name: *const c_char, ...);
    fn ksft_test_result_skip(name: *const c_char, ...);
    fn ksft_finished();
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

/*
 * If we fork the value in the parent should be unchanged and the
 * child should start with the same value.  This is called from the
 * fork_test() asm function.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn fork_test_c() -> c_int {
    let mut newpid: pid_t;
    let mut waiting: pid_t;
    let mut child_status: c_int = 0;
    let parent_result: c_int;

    newpid = unsafe { fork() };
    if newpid == 0 {
        /* In child */
        if unsafe { verify_fork() } == 0 {
            unsafe { ksft_print_msg(c"ZA state invalid in child\n".as_ptr()) };
            unsafe { exit(0) };
        } else {
            unsafe { exit(1) };
        }
    }
    if newpid < 0 {
        unsafe { ksft_print_msg(c"fork() failed: %d\n".as_ptr(), newpid) };

        return 0;
    }

    parent_result = unsafe { verify_fork() };
    if parent_result == 0 {
        unsafe { ksft_print_msg(c"ZA state invalid in parent\n".as_ptr()) };
    }

    loop {
        waiting = unsafe { waitpid(newpid, &mut child_status, 0) };

        if waiting < 0 {
            if unsafe { errno } == EINTR {
                continue;
            }
            unsafe { ksft_print_msg(c"waitpid() failed: %d\n".as_ptr(), errno) };
            return 0;
        }
        if waiting != newpid {
            unsafe { ksft_print_msg(c"waitpid() returned wrong PID\n".as_ptr()) };
            return 0;
        }

        if !WIFEXITED(child_status) {
            unsafe { ksft_print_msg(c"child did not exit\n".as_ptr()) };
            return 0;
        }

        return ((WEXITSTATUS(child_status) != 0) && (parent_result != 0)) as c_int;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut ret: c_int;
    let mut i: c_int;

    unsafe { ksft_print_header() };
    unsafe { ksft_set_plan(EXPECTED_TESTS) };

    unsafe { ksft_print_msg(c"PID: %d\n".as_ptr(), getpid()) };

    /*
     * This test is run with nolibc which doesn't support hwcap and
     * it's probably disproportionate to implement so instead check
     * for the default vector length configuration in /proc.
     */
    ret = unsafe {
        open(
            c"/proc/sys/abi/sme_default_vector_length".as_ptr(),
            O_RDONLY,
            0,
        )
    };
    if ret >= 0 {
        unsafe { ksft_test_result(fork_test(), c"fork_test\n".as_ptr()) };
    } else {
        unsafe { ksft_print_msg(c"SME not supported\n".as_ptr()) };
        i = 0;
        while i < EXPECTED_TESTS {
            unsafe { ksft_test_result_skip(c"fork_test\n".as_ptr()) };
            i += 1;
        }
    }

    unsafe { ksft_finished() };

    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
