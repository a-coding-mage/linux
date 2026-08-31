// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2016, Cyril Bur, IBM Corp.
 *
 * Syscalls can be performed provided the transactions are suspended.
 * The exec() class of syscall is unique as a new process is loaded.
 *
 * It makes little sense for after an exec() call for the previously
 * suspended transaction to still exist.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::arch::asm;
use core::ffi::{c_char, c_int, c_void};

// C includes translated as external dependencies:
// errno.h, inttypes.h, libgen.h, pthread.h, stdio.h, stdlib.h, string.h,
// unistd.h, "utils.h", and "tm.h".

unsafe extern "C" {
    fn have_htm() -> c_int;
    fn htm_is_synthetic() -> c_int;
    fn failure_is_nesting() -> c_int;
    fn test_harness(
        test_function: unsafe extern "C" fn() -> c_int,
        name: *const c_char,
    ) -> c_int;

    fn execl(path: *const c_char, arg0: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

static mut path: *mut c_char = core::ptr::null_mut();

unsafe extern "C" fn test_exec() -> c_int {
    if have_htm() == 0 {
        return 0;
    }
    if htm_is_synthetic() != 0 {
        return 0;
    }

    unsafe {
        asm!(
            "tbegin.",
            "blt    1f",
            "tsuspend.",
            "1:",
            options(nostack, preserves_flags)
        );
    }

    execl(
        path as *const c_char,
        c"tm-exec".as_ptr(),
        c"--child".as_ptr(),
        core::ptr::null::<c_void>(),
    );

    /* Shouldn't get here */
    perror(c"execl() failed".as_ptr());
    1
}

unsafe extern "C" fn after_exec() -> c_int {
    unsafe {
        asm!(
            "tbegin.",
            "blt    1f",
            "tsuspend.",
            "1:",
            options(nostack, preserves_flags)
        );
    }

    if failure_is_nesting() != 0 {
        return 1;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    path = *argv;

    if argc > 1 && strcmp(*argv.add(1), c"--child".as_ptr()) == 0 {
        return after_exec();
    }

    test_harness(test_exec, c"tm_exec".as_ptr())
}
