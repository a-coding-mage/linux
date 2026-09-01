// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021, Oracle and/or its affiliates. */

/* Dependencies from:
 * #include <test_progs.h>
 * #include "exhandler_kern.skel.h"
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_void};

type pid_t = c_int;

#[repr(C)]
pub struct exhandler_kern_bss {
    pub test_pid: pid_t,
    pub exception_triggered: c_int,
}

#[repr(C)]
pub struct exhandler_kern {
    pub bss: *mut exhandler_kern_bss,
}

extern "C" {
    fn exhandler_kern__open_and_load() -> *mut exhandler_kern;
    fn exhandler_kern__attach(skel: *mut exhandler_kern) -> c_int;
    fn exhandler_kern__destroy(skel: *mut exhandler_kern);

    fn getpid() -> pid_t;
    fn fork() -> pid_t;
    fn _exit(status: c_int) -> !;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;

    fn CHECK(condition: bool, name: *const c_char, fmt: *const c_char, ...) -> bool;
    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(actual: pid_t, expected: pid_t, name: *const c_char) -> bool;
    fn ASSERT_NEQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
}

/* Test that verifies exception handling is working. fork()
 * triggers task_newtask tracepoint; that new task will have a
 * NULL pointer task_works, and the associated task->task_works->func
 * should not be NULL if task_works itself is non-NULL.
 *
 * So to verify exception handling we want to see a NULL task_works
 * and task_works->func; if we see this we can conclude that the
 * exception handler ran when we attempted to dereference task->task_works
 * and zeroed the destination register.
 */
#[no_mangle]
pub unsafe extern "C" fn test_exhandler() {
    let mut err: c_int = 0;
    let mut _duration: c_int = 0;
    let mut status: c_int;
    let skel: *mut exhandler_kern;
    let cpid: pid_t;

    skel = exhandler_kern__open_and_load();
    if CHECK(
        skel.is_null(),
        b"skel_load\0".as_ptr() as *const c_char,
        b"skeleton failed: %d\n\0".as_ptr() as *const c_char,
        err,
    ) {
        exhandler_kern__destroy(skel);
        return;
    }

    (*(*skel).bss).test_pid = getpid();

    err = exhandler_kern__attach(skel);
    if !ASSERT_OK(err, b"attach\0".as_ptr() as *const c_char) {
        exhandler_kern__destroy(skel);
        return;
    }
    cpid = fork();
    if !ASSERT_GT(cpid, -1, b"fork failed\0".as_ptr() as *const c_char) {
        exhandler_kern__destroy(skel);
        return;
    }
    if cpid == 0 {
        _exit(0);
    }
    waitpid(cpid, &mut status as *mut c_int, 0);

    ASSERT_NEQ(
        (*(*skel).bss).exception_triggered,
        0,
        b"verify exceptions occurred\0".as_ptr() as *const c_char,
    );

    exhandler_kern__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
