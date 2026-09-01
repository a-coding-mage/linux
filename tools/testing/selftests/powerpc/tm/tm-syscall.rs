// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2015, Sam Bobroff, IBM Corp.
 *
 * Test the kernel's system call code to ensure that a system call
 * made from within an active HTM transaction is aborted with the
 * correct failure code.
 * Conversely, ensure that a system call made from within a
 * suspended transaction can succeed.
 */

use std::os::raw::{c_char, c_int, c_long, c_ulong, c_void};
use std::ptr;

// C dependencies: stdio.h, unistd.h, sys/syscall.h, asm/tm.h, sys/time.h,
// stdlib.h, "utils.h", and "tm.h".

pub type pid_t = c_int;

#[repr(C)]
pub struct timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

// #ifndef PPC_FEATURE2_SCV
// #define PPC_FEATURE2_SCV 0x00100000 /* scv syscall */
// #endif
pub const PPC_FEATURE2_SCV: c_ulong = 0x00100000;

unsafe extern "C" {
    fn getppid_tm_active() -> c_int;
    fn getppid_tm_suspended() -> c_int;
    fn getppid_scv_tm_active() -> c_int;
    fn getppid_scv_tm_suspended() -> c_int;

    static TM_RETRIES: c_int;

    fn failure_is_persistent() -> bool;
    fn failure_is_syscall() -> bool;
    fn have_htm_nosc() -> bool;
    fn htm_is_synthetic() -> bool;
    fn have_hwcap2(feature: c_ulong) -> bool;
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;

    fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    fn setbuf(stream: *mut c_void, buf: *mut c_char);
    static mut stdout: *mut c_void;
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;

    fn __builtin_get_texasr() -> c_ulong;
    fn __builtin_get_tfiar() -> c_ulong;
}

pub static mut retries: u32 = 0;

pub const TEST_DURATION: c_int = 10; /* seconds */

unsafe fn timeradd(a: *const timeval, b: *const timeval, res: *mut timeval) {
    (*res).tv_sec = (*a).tv_sec + (*b).tv_sec;
    (*res).tv_usec = (*a).tv_usec + (*b).tv_usec;
    if (*res).tv_usec >= 1_000_000 {
        (*res).tv_sec += 1;
        (*res).tv_usec -= 1_000_000;
    }
}

unsafe fn timercmp_lt(a: *const timeval, b: *const timeval) -> bool {
    (*a).tv_sec < (*b).tv_sec || ((*a).tv_sec == (*b).tv_sec && (*a).tv_usec < (*b).tv_usec)
}

unsafe extern "C" fn getppid_tm(scv: bool, suspend: bool) -> pid_t {
    let mut pid: pid_t;

    for _i in 0..TM_RETRIES {
        if suspend {
            if scv {
                pid = getppid_scv_tm_suspended();
            } else {
                pid = getppid_tm_suspended();
            }
        } else if scv {
            pid = getppid_scv_tm_active();
        } else {
            pid = getppid_tm_active();
        }

        if pid >= 0 {
            return pid;
        }

        if failure_is_persistent() {
            if failure_is_syscall() {
                return -1;
            }

            printf(c"Unexpected persistent transaction failure.\n".as_ptr());
            printf(
                c"TEXASR 0x%016lx, TFIAR 0x%016lx.\n".as_ptr(),
                __builtin_get_texasr(),
                __builtin_get_tfiar(),
            );
            exit(-1);
        }

        retries = retries.wrapping_add(1);
    }

    printf(
        c"Exceeded limit of %d temporary transaction failures.\n".as_ptr(),
        TM_RETRIES,
    );
    printf(
        c"TEXASR 0x%016lx, TFIAR 0x%016lx.\n".as_ptr(),
        __builtin_get_texasr(),
        __builtin_get_tfiar(),
    );

    exit(-1);
}

unsafe extern "C" fn tm_syscall() -> c_int {
    let mut count: u32 = 0;
    let mut end = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut now = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };

    SKIP_IF!(!have_htm_nosc());
    SKIP_IF!(htm_is_synthetic());

    setbuf(stdout, ptr::null_mut());

    printf(
        c"Testing transactional syscalls for %d seconds...\n".as_ptr(),
        TEST_DURATION,
    );

    gettimeofday(&mut end, ptr::null_mut());
    now.tv_sec = TEST_DURATION as c_long;
    now.tv_usec = 0;
    timeradd(&end, &now, &mut end);

    while timercmp_lt(&now, &end) {
        /*
         * Test a syscall within a suspended transaction and verify
         * that it succeeds.
         */
        FAIL_IF!(getppid_tm(false, true) == -1); /* Should succeed. */

        /*
         * Test a syscall within an active transaction and verify that
         * it fails with the correct failure code.
         */
        FAIL_IF!(getppid_tm(false, false) != -1); /* Should fail... */
        FAIL_IF!(!failure_is_persistent()); /* ...persistently... */
        FAIL_IF!(!failure_is_syscall()); /* ...with code syscall. */

        /* Now do it all again with scv if it is available. */
        if have_hwcap2(PPC_FEATURE2_SCV) {
            FAIL_IF!(getppid_tm(true, true) == -1); /* Should succeed. */
            FAIL_IF!(getppid_tm(true, false) != -1); /* Should fail... */
            FAIL_IF!(!failure_is_persistent()); /* ...persistently... */
            FAIL_IF!(!failure_is_syscall()); /* ...with code syscall. */
        }

        gettimeofday(&mut now, ptr::null_mut());
        count = count.wrapping_add(1);
    }

    printf(
        c"%d active and suspended transactions behaved correctly.\n".as_ptr(),
        count,
    );
    printf(
        c"(There were %d transaction retries.)\n".as_ptr(),
        retries,
    );

    0
}

pub unsafe extern "C" fn main() -> c_int {
    test_harness(tm_syscall, c"tm_syscall".as_ptr())
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
