// SPDX-License-Identifier: GPL-2.0
/*
 * Sandbox itself and execute another program (in a different mount point).
 *
 * Used by layout1.umount_sandboxer from fs_test.c
 *
 * Copyright © 2024-2025 Microsoft Corporation
 */

// C dependencies: errno.h, stdio.h, stdlib.h, sys/prctl.h, unistd.h, wrappers.h.
// External constants/types such as LANDLOCK_SCOPE_SIGNAL, PR_SET_NO_NEW_PRIVS,
// and landlock_ruleset_attr are expected to be supplied by the surrounding
// translated test harness/bindings.

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::ptr;

unsafe extern "C" {
    static mut stderr: *mut FILE;

    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn atoi(nptr: *const c_char) -> c_int;
    fn prctl(option: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn execve(
        pathname: *const c_char,
        argv: *const *mut c_char,
        envp: *const *mut c_char,
    ) -> c_int;

    fn landlock_create_ruleset(
        attr: *const landlock_ruleset_attr,
        size: usize,
        flags: c_uint,
    ) -> c_int;
    fn landlock_restrict_self(ruleset_fd: c_int, flags: c_uint) -> c_int;

    fn __errno_location() -> *mut c_int;
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct landlock_ruleset_attr {
    pub scoped: u64,
}

const LANDLOCK_SCOPE_SIGNAL: u64 = 1 << 0;
const PR_SET_NO_NEW_PRIVS: c_int = 38;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let ruleset_attr = landlock_ruleset_attr {
        scoped: LANDLOCK_SCOPE_SIGNAL,
    };
    let pipe_child: c_int;
    let pipe_parent: c_int;
    let ruleset_fd: c_int;
    let mut buf: c_char = 0;

    /*
     * The first argument must be the file descriptor number of a pipe.
     * The second argument must be the program to execute.
     */
    if argc != 4 {
        fprintf(
            stderr,
            c"Wrong number of arguments (not three)\n".as_ptr(),
        );
        return 1;
    }

    pipe_child = atoi(*argv.add(2));
    pipe_parent = atoi(*argv.add(3));

    ruleset_fd = landlock_create_ruleset(
        &ruleset_attr,
        core::mem::size_of_val(&ruleset_attr),
        0,
    );
    if ruleset_fd < 0 {
        perror(c"Failed to create ruleset".as_ptr());
        return 1;
    }

    if prctl(PR_SET_NO_NEW_PRIVS, 1 as c_long, 0 as c_long, 0 as c_long, 0 as c_long) != 0 {
        perror(c"Failed to call prctl()".as_ptr());
        return 1;
    }

    if landlock_restrict_self(ruleset_fd, 0) != 0 {
        perror(c"Failed to restrict self".as_ptr());
        return 1;
    }

    if close(ruleset_fd) != 0 {
        perror(c"Failed to close ruleset".as_ptr());
        return 1;
    }

    /* Signals that we are sandboxed. */
    *__errno_location() = 0;
    if write(pipe_child, c".".as_ptr().cast::<c_void>(), 1) != 1 {
        perror(c"Failed to write to the second argument".as_ptr());
        return 1;
    }

    /* Waits for the parent to try to umount. */
    if read(
        pipe_parent,
        (&mut buf as *mut c_char).cast::<c_void>(),
        1,
    ) != 1
    {
        perror(c"Failed to write to the third argument".as_ptr());
        return 1;
    }

    /* Shifts arguments. */
    *argv.add(0) = *argv.add(1);
    *argv.add(1) = *argv.add(2);
    *argv.add(2) = *argv.add(3);
    *argv.add(3) = ptr::null_mut();
    execve(*argv.add(0), argv.cast_const(), ptr::null());
    perror(c"Failed to execute the provided binary".as_ptr());
    1
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
