// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2016, Cyril Bur, IBM Corp.
 *
 * Sending one self a signal should always get delivered.
 */

// C dependencies: signal.h, stdio.h, stdlib.h, string.h, sys/types.h,
// sys/wait.h, unistd.h, altivec.h, and "utils.h".

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

const MAX_ATTEMPT: c_int = 500000;
const TIMEOUT: c_uint = 5;

type pid_t = c_int;
type sig_atomic_t = c_int;

const SIGUSR1: c_int = 10;
const SIGALRM: c_int = 14;

#[repr(C)]
pub struct sigset_t {
    __val: [u64; 16],
}

#[repr(C)]
pub struct sigaction {
    sa_handler: extern "C" fn(c_int),
    sa_flags: c_ulong,
    sa_restorer: Option<extern "C" fn()>,
    sa_mask: sigset_t,
}

type c_ulong = u64;

unsafe extern "C" {
    fn signal_self(pid: pid_t, sig: c_int) -> c_long;

    fn getpid() -> pid_t;
    fn fork() -> pid_t;
    fn exit(status: c_int) -> !;
    fn alarm(seconds: c_uint) -> c_uint;
    fn waitpid(pid: pid_t, stat_loc: *mut c_int, options: c_int) -> pid_t;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn perror(s: *const c_char);
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;

    static mut stderr: *mut c_void;

    fn test_harness_set_timeout(seconds: c_int);
    fn test_harness(test: extern "C" fn() -> c_int, name: *const c_char) -> c_int;

    // External translation of the FAIL_IF macro from utils.h.
    fn FAIL_IF(cond: c_int) -> c_int;
}

static mut signaled: sig_atomic_t = 0;
static mut fail: sig_atomic_t = 0;

extern "C" fn signal_handler(sig: c_int) {
    unsafe {
        if sig == SIGUSR1 {
            signaled = 1;
        } else {
            fail = 1;
        }
    }
}

extern "C" fn test_signal() -> c_int {
    unsafe {
        let mut i: c_int;
        let mut act: sigaction = core::mem::zeroed();
        let ppid: pid_t = getpid();
        let mut pid: pid_t;

        act.sa_handler = signal_handler;
        act.sa_flags = 0;
        sigemptyset(&mut act.sa_mask);
        if sigaction(SIGUSR1, &act, core::ptr::null_mut()) < 0 {
            perror(c"sigaction SIGUSR1".as_ptr());
            exit(1);
        }
        if sigaction(SIGALRM, &act, core::ptr::null_mut()) < 0 {
            perror(c"sigaction SIGALRM".as_ptr());
            exit(1);
        }

        /* Don't do this for MAX_ATTEMPT, its simply too long */
        i = 0;
        while i < 1000 {
            pid = fork();
            if pid == -1 {
                perror(c"fork".as_ptr());
                exit(1);
            }
            if pid == 0 {
                signal_self(ppid, SIGUSR1);
                exit(1);
            } else {
                alarm(0); /* Disable any pending */
                alarm(2);
                while signaled == 0 && fail == 0 {
                    asm!("", options(nostack, preserves_flags));
                }
                if signaled == 0 {
                    fprintf(stderr, c"Didn't get signal from child\n".as_ptr());
                    FAIL_IF(1); /* For the line number */
                }
                /* Otherwise we'll loop too fast and fork() will eventually fail */
                waitpid(pid, core::ptr::null_mut(), 0);
            }
            i += 1;
        }

        i = 0;
        while i < MAX_ATTEMPT {
            let rc: c_long;

            alarm(0); /* Disable any pending */
            signaled = 0;
            alarm(TIMEOUT);
            rc = signal_self(ppid, SIGUSR1);
            if rc != 0 {
                fprintf(
                    stderr,
                    c"(%d) Fail reason: %d rc=0x%lx".as_ptr(),
                    i,
                    fail,
                    rc,
                );
                FAIL_IF(1); /* For the line number */
            }
            while signaled == 0 && fail == 0 {
                asm!("", options(nostack, preserves_flags));
            }
            if signaled == 0 {
                fprintf(
                    stderr,
                    c"(%d) Fail reason: %d rc=0x%lx".as_ptr(),
                    i,
                    fail,
                    rc,
                );
                FAIL_IF(1); /* For the line number */
            }
            i += 1;
        }

        0
    }
}

pub fn main() -> c_int {
    unsafe {
        test_harness_set_timeout(300);
        test_harness(test_signal, c"signal".as_ptr())
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
