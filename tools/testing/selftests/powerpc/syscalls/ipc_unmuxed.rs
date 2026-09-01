// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2015, Michael Ellerman, IBM Corp.
 *
 * This test simply tests that certain syscalls are implemented. It doesn't
 * actually exercise their logic in any way.
 */

// C source defined _GNU_SOURCE and included:
// <errno.h>, <stdio.h>, <unistd.h>, <sys/syscall.h>, "utils.h", and "ipc.h".

use core::ffi::c_long;

const ENOSYS: i32 = 38;

extern "C" {
    static mut errno: i32;

    fn printf(format: *const i8, ...) -> i32;
    fn syscall(num: c_long, ...) -> c_long;

    fn test_harness(
        test_function: unsafe extern "C" fn() -> i32,
        name: *const i8,
    ) -> i32;
}

macro_rules! FAIL_IF {
    ($condition:expr) => {
        if $condition {
            return 1;
        }
    };
}

macro_rules! SKIP_IF {
    ($condition:expr) => {
        if $condition {
            return 4;
        }
    };
}

macro_rules! DO_TEST {
    ($name:ident, $num:expr) => {
        unsafe extern "C" fn $name() -> i32 {
            let rc: i32;

            unsafe {
                printf(concat!("Testing ", stringify!($name), "\0").as_ptr() as *const i8);
                errno = 0;
                rc = syscall($num as c_long, -1, 0, 0, 0, 0, 0) as i32;
                printf(
                    b"\treturned %d, errno %d\n\0".as_ptr() as *const i8,
                    rc,
                    errno,
                );
                (errno == ENOSYS) as i32
            }
        }
    };
}

/*
 * In the C source, "ipc.h" is included here while DO_TEST is defined as:
 *
 * static int test_##_name(void)
 * {
 *     int rc;
 *     printf("Testing " #_name);
 *     errno = 0;
 *     rc = syscall(_num, -1, 0, 0, 0, 0, 0);
 *     printf("\treturned %d, errno %d\n", rc, errno);
 *     return errno == ENOSYS;
 * }
 *
 * That header supplies the per-syscall DO_TEST(_name, _num) invocations.
 */

unsafe extern "C" fn ipc_unmuxed() -> i32 {
    let mut tests_done: i32 = 0;

    /*
     * In the C source, "ipc.h" is included a second time while DO_TEST is
     * defined as:
     *
     * FAIL_IF(test_##_name());
     * tests_done++;
     *
     * The generated calls are intentionally left dependent on the external
     * translated ipc.h content rather than invented in this isolated file.
     */

    /*
     * If we ran no tests then it means none of the syscall numbers were
     * defined, possibly because we were built against old headers. But it
     * means we didn't really test anything, so instead of passing mark it
     * as a skip to give the user a clue.
     */
    SKIP_IF!(tests_done == 0);

    0
}

pub unsafe fn main() -> i32 {
    unsafe { test_harness(ipc_unmuxed, b"ipc_unmuxed\0".as_ptr() as *const i8) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
