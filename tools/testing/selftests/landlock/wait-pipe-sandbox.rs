// SPDX-License-Identifier: GPL-2.0
/*
 * Write in a pipe, wait, sandbox itself, test sandboxing, and wait again.
 *
 * Used by audit_exec.flags from audit_test.c
 *
 * Copyright © 2024-2025 Microsoft Corporation
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

const LANDLOCK_ACCESS_FS_READ_DIR: u64 = 1 << 3;
const LANDLOCK_SCOPE_SIGNAL: u64 = 1 << 0;

const O_RDONLY: c_int = 0;
const O_CLOEXEC: c_int = 0o2000000;
const O_DIRECTORY: c_int = 0o200000;

#[repr(C)]
struct landlock_ruleset_attr {
    handled_access_fs: u64,
    handled_access_net: u64,
    scoped: u64,
}

unsafe extern "C" {
    fn atoi(nptr: *const c_char) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn getppid() -> c_int;
    fn kill(pid: c_int, sig: c_int) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn perror(s: *const c_char);
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;

    static mut stderr: *mut FILE;
}

// From wrappers.h.
unsafe extern "C" {
    fn landlock_create_ruleset(
        attr: *const landlock_ruleset_attr,
        size: usize,
        flags: c_uint,
    ) -> c_int;
    fn landlock_restrict_self(ruleset_fd: c_int, flags: c_uint) -> c_int;
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

fn sync_with(pipe_child: c_int, pipe_parent: c_int) -> c_int {
    let mut buf: c_char = 0;

    /* Signals that we are waiting. */
    if unsafe { write(pipe_child, c".".as_ptr().cast::<c_void>(), 1) } != 1 {
        unsafe { perror(c"Failed to write to first argument".as_ptr()) };
        return 1;
    }

    /* Waits for the parent do its test. */
    if unsafe { read(pipe_parent, (&mut buf as *mut c_char).cast::<c_void>(), 1) } != 1 {
        unsafe { perror(c"Failed to write to the second argument".as_ptr()) };
        return 1;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let layer2 = landlock_ruleset_attr {
        handled_access_fs: LANDLOCK_ACCESS_FS_READ_DIR,
        handled_access_net: 0,
        scoped: 0,
    };
    let layer3 = landlock_ruleset_attr {
        handled_access_fs: 0,
        handled_access_net: 0,
        scoped: LANDLOCK_SCOPE_SIGNAL,
    };
    let mut err: c_int;
    let pipe_child: c_int;
    let pipe_parent: c_int;
    let mut ruleset_fd: c_int;

    /* The first argument must be the file descriptor number of a pipe. */
    if argc != 3 {
        unsafe { fprintf(stderr, c"Wrong number of arguments (not two)\n".as_ptr()) };
        return 1;
    }

    pipe_child = unsafe { atoi(*argv.add(1)) };
    pipe_parent = unsafe { atoi(*argv.add(2)) };
    /* PR_SET_NO_NEW_PRIVS already set by parent. */

    /* First step to test parent's layer1. */
    err = sync_with(pipe_child, pipe_parent);
    if err != 0 {
        return err;
    }

    /* Tries to send a signal, denied by layer1. */
    if unsafe { kill(getppid(), 0) } == 0 {
        unsafe { fprintf(stderr, c"Successfully sent a signal to the parent".as_ptr()) };
        return 1;
    }

    /* Second step to test parent's layer1 and our layer2. */
    err = sync_with(pipe_child, pipe_parent);
    if err != 0 {
        return err;
    }

    ruleset_fd = unsafe {
        landlock_create_ruleset(
            &layer2 as *const landlock_ruleset_attr,
            core::mem::size_of_val(&layer2),
            0,
        )
    };
    if ruleset_fd < 0 {
        unsafe { perror(c"Failed to create the layer2 ruleset".as_ptr()) };
        return 1;
    }

    if unsafe { landlock_restrict_self(ruleset_fd, 0) } != 0 {
        unsafe { perror(c"Failed to restrict self".as_ptr()) };
        return 1;
    }
    unsafe { close(ruleset_fd) };

    /* Tries to send a signal, denied by layer1. */
    if unsafe { kill(getppid(), 0) } == 0 {
        unsafe { fprintf(stderr, c"Successfully sent a signal to the parent".as_ptr()) };
        return 1;
    }

    /* Tries to open ., denied by layer2. */
    if unsafe { open(c"/".as_ptr(), O_RDONLY | O_DIRECTORY | O_CLOEXEC) } >= 0 {
        unsafe { fprintf(stderr, c"Successfully opened /".as_ptr()) };
        return 1;
    }

    /* Third step to test our layer2 and layer3. */
    err = sync_with(pipe_child, pipe_parent);
    if err != 0 {
        return err;
    }

    ruleset_fd = unsafe {
        landlock_create_ruleset(
            &layer3 as *const landlock_ruleset_attr,
            core::mem::size_of_val(&layer3),
            0,
        )
    };
    if ruleset_fd < 0 {
        unsafe { perror(c"Failed to create the layer3 ruleset".as_ptr()) };
        return 1;
    }

    if unsafe { landlock_restrict_self(ruleset_fd, 0) } != 0 {
        unsafe { perror(c"Failed to restrict self".as_ptr()) };
        return 1;
    }
    unsafe { close(ruleset_fd) };

    /* Tries to open ., denied by layer2. */
    if unsafe { open(c"/".as_ptr(), O_RDONLY | O_DIRECTORY | O_CLOEXEC) } >= 0 {
        unsafe { fprintf(stderr, c"Successfully opened /".as_ptr()) };
        return 1;
    }

    /* Tries to send a signal, denied by layer3. */
    if unsafe { kill(getppid(), 0) } == 0 {
        unsafe { fprintf(stderr, c"Successfully sent a signal to the parent".as_ptr()) };
        return 1;
    }

    0
}
