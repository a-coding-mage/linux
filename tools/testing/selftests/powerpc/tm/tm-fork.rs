// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2015, Michael Neuling, IBM Corp.
 *
 * Edited: Rashmica Gupta, Nov 2015
 *
 * This test does a fork syscall inside a transaction. Basic sniff test
 * to see if we can enter the kernel during a transaction.
 */

// C dependencies: errno.h, inttypes.h, pthread.h, stdio.h, stdlib.h, unistd.h
// Local dependencies: utils.h, tm.h

use core::arch::asm;
use core::ffi::{c_char, c_int};

extern "C" {
    fn have_htm() -> c_int;
    fn htm_is_synthetic() -> c_int;
    fn test_harness(
        test_function: unsafe extern "C" fn() -> c_int,
        name: *const c_char,
    ) -> c_int;
}

unsafe extern "C" fn test_fork() -> c_int {
    if have_htm() == 0 {
        return 0;
    }
    if htm_is_synthetic() != 0 {
        return 0;
    }

    asm!(
        "tbegin.",
        "blt    1f",
        "li     0, 2", // fork syscall
        "sc",
        "tend.",
        "1:",
        out("r0") _,
        options(nostack)
    );
    /*
     * If we reach here, we've passed.  Otherwise we've probably crashed
     * the kernel
     */

    0
}

unsafe fn main_0(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    test_harness(test_fork, b"tm_fork\0".as_ptr() as *const c_char)
}

fn main() {
    unsafe {
        core::process::exit(main_0(0, core::ptr::null_mut()));
    }
}
