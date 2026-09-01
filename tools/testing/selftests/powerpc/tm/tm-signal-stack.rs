// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2015, Michael Neuling, IBM Corp.
 *
 * Test the kernel's signal delievery code to ensure that we don't
 * trelaim twice in the kernel signal delivery code.  This can happen
 * if we trigger a signal when in a transaction and the stack pointer
 * is bogus.
 *
 * This test case registers a SEGV handler, sets the stack pointer
 * (r1) to NULL, starts a transaction and then generates a SEGV.  The
 * SEGV should be handled but we exit here as the stack pointer is
 * invalid and hance we can't sigreturn.  We only need to check that
 * this flow doesn't crash the kernel.
 */

use core::arch::asm;
use core::ffi::{c_char, c_int, c_void};

const SIGSEGV: c_int = 11;
const SIG_ERR: usize = usize::MAX;

type SighandlerT = Option<extern "C" fn(c_int)>;

unsafe extern "C" {
    fn exit(status: c_int) -> !;
    fn fork() -> c_int;
    fn wait(status: *mut c_int) -> c_int;
    fn signal(signum: c_int, handler: SighandlerT) -> usize;

    fn have_htm() -> c_int;
    fn htm_is_synthetic() -> c_int;
    fn test_harness(test: extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

macro_rules! SKIP_IF {
    ($cond:expr) => {
        if $cond {
            return 0;
        }
    };
}

extern "C" fn signal_segv(_signum: c_int) {
    /*
     * This should never actually run since stack is foobar
     */
    unsafe {
        exit(1);
    }
}

extern "C" fn tm_signal_stack() -> c_int {
    let pid: c_int;

    unsafe {
        SKIP_IF!(have_htm() == 0);
        SKIP_IF!(htm_is_synthetic() != 0);

        pid = fork();
        if pid < 0 {
            exit(1);
        }

        if pid != 0 {
            /* Parent */
            /*
             * It's likely the whole machine will crash here so if
             * the child ever exits, we are good.
             */
            wait(core::ptr::null_mut());
            return 0;
        }

        /*
         * The flow here is:
         * 1) register a signal handler (so signal delievery occurs)
         * 2) make stack pointer (r1) = NULL
         * 3) start transaction
         * 4) cause segv
         */
        if signal(SIGSEGV, Some(signal_segv)) == SIG_ERR {
            exit(1);
        }
        asm!(
            "li 1, 0 ;",
            "1:",
            "tbegin.;",
            "beq 1b ;",
            "tsuspend.;",
            "ld 2, 0(1) ;",
            options(nostack, preserves_flags),
        );
    }

    /*
     * This should never get here due to above segv
     */
    1
}

fn main() -> c_int {
    unsafe { test_harness(tm_signal_stack, c"tm_signal_stack".as_ptr()) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
