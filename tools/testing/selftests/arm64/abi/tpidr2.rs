// SPDX-License-Identifier: GPL-2.0-only

use std::arch::asm;
use std::ffi::{c_char, c_int, c_long, c_ulong, c_void};

// C dependencies:
// #include <linux/sched.h>
// #include <linux/wait.h>
// #include "kselftest.h"

const SYS_TPIDR2: &str = "S3_3_C13_C0_5";

const EXPECTED_TESTS: c_int = 5;

type pid_t = c_int;

const CLONE_VM: c_ulong = 0x0000_0100;
const __WCLONE: c_int = 0x8000_0000u32 as c_int;
const __NR_clone: c_long = 220;
const __STACK_SIZE: usize = 8 * 1024 * 1024;
const EINTR: c_int = 4;
const O_RDONLY: c_int = 0;

unsafe extern "C" {
    fn getpid() -> pid_t;
    fn gettid() -> pid_t;
    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn exit(status: c_int) -> !;
    fn syscall(number: c_long, ...) -> c_long;
    fn malloc(size: usize) -> *mut c_void;
    fn open(pathname: *const c_char, flags: c_int, mode: c_int) -> c_int;
    fn msleep(msecs: c_ulong);

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_test_result(result: c_int, name: *const c_char);
    fn ksft_test_result_skip(name: *const c_char);
    fn ksft_finished();

    static mut errno: c_int;
}

#[inline]
unsafe fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

#[inline]
unsafe fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn set_tpidr2(val: u64) {
    asm!("msr S3_3_C13_C0_5, {val}", val = in(reg) val);
}

unsafe fn get_tpidr2() -> u64 {
    let val: u64;

    asm!("mrs {val}, S3_3_C13_C0_5", val = out(reg) val);

    val
}

/* Processes should start with TPIDR2 == 0 */
unsafe fn default_value() -> c_int {
    (get_tpidr2() == 0) as c_int
}

/* If we set TPIDR2 we should read that value */
unsafe fn write_read() -> c_int {
    set_tpidr2(getpid() as u64);

    (getpid() as u64 == get_tpidr2()) as c_int
}

/* If we set a value we should read the same value after scheduling out */
unsafe fn write_sleep_read() -> c_int {
    set_tpidr2(getpid() as u64);

    msleep(100);

    (getpid() as u64 == get_tpidr2()) as c_int
}

/*
 * If we fork the value in the parent should be unchanged and the
 * child should start with the same value and be able to set its own
 * value.
 */
unsafe fn write_fork_read() -> c_int {
    let newpid: pid_t;
    let mut waiting: pid_t;
    let oldpid: pid_t;
    let mut status: c_int = 0;

    set_tpidr2(getpid() as u64);

    oldpid = getpid();
    newpid = fork();
    if newpid == 0 {
        /* In child */
        if get_tpidr2() != oldpid as u64 {
            ksft_print_msg(
                b"TPIDR2 changed in child: %llx\n\0".as_ptr() as *const c_char,
                get_tpidr2(),
            );
            exit(0);
        }

        set_tpidr2(getpid() as u64);
        if get_tpidr2() == getpid() as u64 {
            exit(1);
        } else {
            ksft_print_msg(b"Failed to set TPIDR2 in child\n\0".as_ptr() as *const c_char);
            exit(0);
        }
    }
    if newpid < 0 {
        ksft_print_msg(b"fork() failed: %d\n\0".as_ptr() as *const c_char, newpid);
        return 0;
    }

    loop {
        waiting = waitpid(newpid, &mut status, 0);

        if waiting < 0 {
            if errno == EINTR {
                continue;
            }
            ksft_print_msg(b"waitpid() failed: %d\n\0".as_ptr() as *const c_char, errno);
            return 0;
        }
        if waiting != newpid {
            ksft_print_msg(
                b"waitpid() returned wrong PID: %d != %d\n\0".as_ptr() as *const c_char,
                waiting,
                newpid,
            );
            return 0;
        }

        if !WIFEXITED(status) {
            ksft_print_msg(b"child did not exit\n\0".as_ptr() as *const c_char);
            return 0;
        }

        if getpid() as u64 != get_tpidr2() {
            ksft_print_msg(b"TPIDR2 corrupted in parent\n\0".as_ptr() as *const c_char);
            return 0;
        }

        return WEXITSTATUS(status);
    }
}

/*
 * sys_clone() has a lot of per architecture variation so just define
 * it here rather than adding it to nolibc, plus the raw API is a
 * little more convenient for this test.
 */
unsafe fn sys_clone(
    clone_flags: c_ulong,
    newsp: c_ulong,
    parent_tidptr: *mut c_int,
    tls: c_ulong,
    child_tidptr: *mut c_int,
) -> c_int {
    syscall(
        __NR_clone,
        clone_flags,
        newsp,
        parent_tidptr,
        tls,
        child_tidptr,
    ) as c_int
}

/*
 * If we clone with CLONE_VM then the value in the parent should
 * be unchanged and the child should start with zero and be able to
 * set its own value.
 */
unsafe fn write_clone_read() -> c_int {
    let mut parent_tid: c_int = 0;
    let mut child_tid: c_int = 0;
    let parent: pid_t;
    let mut waiting: pid_t;
    let ret: c_int;
    let mut status: c_int = 0;
    let stack: *mut c_void;

    parent = getpid();
    set_tpidr2(parent as u64);

    stack = malloc(__STACK_SIZE);
    if stack.is_null() {
        ksft_print_msg(b"malloc() failed\n\0".as_ptr() as *const c_char);
        return 0;
    }

    ret = sys_clone(
        CLONE_VM,
        stack as c_ulong + __STACK_SIZE as c_ulong,
        &mut parent_tid,
        0,
        &mut child_tid,
    );
    if ret == -1 {
        ksft_print_msg(b"clone() failed: %d\n\0".as_ptr() as *const c_char, errno);
        return 0;
    }

    if ret == 0 {
        /* In child */
        if get_tpidr2() != 0 {
            ksft_print_msg(
                b"TPIDR2 non-zero in child: %llx\n\0".as_ptr() as *const c_char,
                get_tpidr2(),
            );
            exit(0);
        }

        if gettid() == 0 {
            ksft_print_msg(b"Child TID==0\n\0".as_ptr() as *const c_char);
        }
        set_tpidr2(gettid() as u64);
        if get_tpidr2() == gettid() as u64 {
            exit(1);
        } else {
            ksft_print_msg(b"Failed to set TPIDR2 in child\n\0".as_ptr() as *const c_char);
            exit(0);
        }
    }

    loop {
        waiting = waitpid(ret, &mut status, __WCLONE);

        if waiting < 0 {
            if errno == EINTR {
                continue;
            }
            ksft_print_msg(b"waitpid() failed: %d\n\0".as_ptr() as *const c_char, errno);
            return 0;
        }
        if waiting != ret {
            ksft_print_msg(
                b"waitpid() returned wrong PID %d\n\0".as_ptr() as *const c_char,
                waiting,
            );
            return 0;
        }

        if !WIFEXITED(status) {
            ksft_print_msg(b"child did not exit\n\0".as_ptr() as *const c_char);
            return 0;
        }

        if parent as u64 != get_tpidr2() {
            ksft_print_msg(b"TPIDR2 corrupted in parent\n\0".as_ptr() as *const c_char);
            return 0;
        }

        return WEXITSTATUS(status);
    }
}

#[no_mangle]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let ret: c_int;

    ksft_print_header();
    ksft_set_plan(5);

    ksft_print_msg(b"PID: %d\n\0".as_ptr() as *const c_char, getpid());

    /*
     * This test is run with nolibc which doesn't support hwcap and
     * it's probably disproportionate to implement so instead check
     * for the default vector length configuration in /proc.
     */
    ret = open(
        b"/proc/sys/abi/sme_default_vector_length\0".as_ptr() as *const c_char,
        O_RDONLY,
        0,
    );
    if ret >= 0 {
        ksft_test_result(default_value(), b"default_value\n\0".as_ptr() as *const c_char);
        ksft_test_result(write_read(), b"write_read\n\0".as_ptr() as *const c_char);
        ksft_test_result(
            write_sleep_read(),
            b"write_sleep_read\n\0".as_ptr() as *const c_char,
        );
        ksft_test_result(
            write_fork_read(),
            b"write_fork_read\n\0".as_ptr() as *const c_char,
        );
        ksft_test_result(
            write_clone_read(),
            b"write_clone_read\n\0".as_ptr() as *const c_char,
        );
    } else {
        ksft_print_msg(b"SME support not present\n\0".as_ptr() as *const c_char);

        ksft_test_result_skip(b"default_value\n\0".as_ptr() as *const c_char);
        ksft_test_result_skip(b"write_read\n\0".as_ptr() as *const c_char);
        ksft_test_result_skip(b"write_sleep_read\n\0".as_ptr() as *const c_char);
        ksft_test_result_skip(b"write_fork_read\n\0".as_ptr() as *const c_char);
        ksft_test_result_skip(b"write_clone_read\n\0".as_ptr() as *const c_char);
    }

    ksft_finished();
    0
}
