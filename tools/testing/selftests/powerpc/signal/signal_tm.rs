// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2016, Cyril Bur, IBM Corp.
 *
 * Sending one self a signal should always get delivered.
 */

// C dependencies: errno.h, stdlib.h, stdio.h, string.h, signal.h, unistd.h,
// altivec.h, "utils.h", "../tm/tm.h"

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_ulong};

const MAX_ATTEMPT: c_int = 500000;
const TIMEOUT: c_uint = 10;

type c_uint = u32;
type pid_t = c_int;
type sig_atomic_t = c_int;

const SIGUSR1: c_int = 10;
const SIGALRM: c_int = 14;

#[repr(C)]
struct sigset_t {
    __val: [c_ulong; 16],
}

#[repr(C)]
struct sigaction {
    sa_handler: Option<unsafe extern "C" fn(c_int)>,
    sa_flags: c_ulong,
    sa_restorer: Option<unsafe extern "C" fn()>,
    sa_mask: sigset_t,
}

unsafe extern "C" {
    fn tm_signal_self(pid: pid_t, sig: c_int, ret: *mut c_long) -> c_long;

    fn tcheck_active() -> c_int;
    fn tcheck_transactional() -> c_int;
    fn have_htm() -> c_int;
    fn htm_is_synthetic() -> c_int;
    fn test_harness(
        test_function: unsafe extern "C" fn() -> c_int,
        name: *const c_char,
    ) -> c_int;

    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn perror(s: *const c_char);
    fn exit(status: c_int) -> !;
    fn alarm(seconds: c_uint) -> c_uint;
    fn getpid() -> pid_t;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut FILE;

    fn __builtin_get_texasr() -> c_ulong;
    fn __builtin_get_tfiar() -> c_ulong;
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

static mut signaled: sig_atomic_t = 0;
static mut fail: sig_atomic_t = 0;

unsafe extern "C" fn signal_handler(sig: c_int) {
    unsafe {
        if tcheck_active() != 0 {
            fail = 2;
            return;
        }

        if sig == SIGUSR1 {
            signaled = 1;
        } else {
            fail = 1;
        }
    }
}

unsafe extern "C" fn test_signal_tm() -> c_int {
    let mut i: c_int;
    let mut act: sigaction = unsafe { core::mem::zeroed() };

    act.sa_handler = Some(signal_handler);
    act.sa_flags = 0;
    unsafe {
        sigemptyset(&mut act.sa_mask);
        if sigaction(SIGUSR1, &act, core::ptr::null_mut()) < 0 {
            perror(c"sigaction SIGUSR1".as_ptr());
            exit(1);
        }
        if sigaction(SIGALRM, &act, core::ptr::null_mut()) < 0 {
            perror(c"sigaction SIGALRM".as_ptr());
            exit(1);
        }

        SKIP_IF!(have_htm() == 0);
        SKIP_IF!(htm_is_synthetic() != 0);
    }

    i = 0;
    while i < MAX_ATTEMPT {
        /*
         * If anything bad happens in ASM and we fail to set ret
         * because *handwave* TM this will cause failure
         */
        let mut ret: c_long = 0xdead;
        let mut rc: c_long = 0xbeef;

        unsafe {
            alarm(0); /* Disable any pending */
            signaled = 0;
            alarm(TIMEOUT);
            FAIL_IF!(tcheck_transactional() != 0);
            rc = tm_signal_self(getpid(), SIGUSR1, &mut ret);
            if ret == 0xdead {
                /*
                 * This basically means the transaction aborted before we
                 * even got to the suspend... this is crazy but it
                 * happens.
                 * Yes this also means we might never make forward
                 * progress... the alarm() will trip eventually...
                 */
                i += 1;
                continue;
            }

            if rc != 0 || ret != 0 {
                /* Ret is actually an errno */
                printf(
                    c"TEXASR 0x%016lx, TFIAR 0x%016lx\n".as_ptr(),
                    __builtin_get_texasr(),
                    __builtin_get_tfiar(),
                );
                fprintf(
                    stderr,
                    c"(%d) Fail reason: %d rc=0x%lx ret=0x%lx\n".as_ptr(),
                    i,
                    fail,
                    rc,
                    ret,
                );
                FAIL_IF!(ret != 0);
            }
            while signaled == 0 && fail == 0 {
                asm!("", options(nostack, preserves_flags));
            }
            if signaled == 0 {
                fprintf(
                    stderr,
                    c"(%d) Fail reason: %d rc=0x%lx ret=0x%lx\n".as_ptr(),
                    i,
                    fail,
                    rc,
                    ret,
                );
                FAIL_IF!(fail != 0); /* For the line number */
            }
        }

        i += 1;
    }

    0
}

fn main() {
    unsafe {
        std::process::exit(test_harness(test_signal_tm, c"signal_tm".as_ptr()));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
