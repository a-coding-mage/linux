// SPDX-License-Identifier: GPL-2.0-only
/* Timeout API for single-threaded programs that use blocking
 * syscalls (read/write/send/recv/connect/accept).
 *
 * Copyright (C) 2017 Red Hat, Inc.
 *
 * Author: Stefan Hajnoczi <stefanha@redhat.com>
 */

/* Use the following pattern:
 *
 *   timeout_begin(TIMEOUT);
 *   do {
 *       ret = accept(...);
 *       timeout_check("accept");
 *   } while (ret < 0 && ret == EINTR);
 *   timeout_end();
 */

use libc::{self, c_char, c_int, timespec, useconds_t};
use std::ptr;

static mut timeout: bool = false;

/* SIGALRM handler function.  Do not use sleep(2), alarm(2), or
 * setitimer(2) while using this API - they may interfere with each
 * other.
 *
 * If you need to sleep, please use timeout_sleep() provided by this API.
 */
#[no_mangle]
pub unsafe extern "C" fn sigalrm(signo: c_int) {
    let _ = signo;
    unsafe {
        ptr::write_volatile(ptr::addr_of_mut!(timeout), true);
    }
}

/* Start a timeout.  Call timeout_check() to verify that the timeout hasn't
 * expired.  timeout_end() must be called to stop the timeout.  Timeouts cannot
 * be nested.
 */
#[no_mangle]
pub unsafe extern "C" fn timeout_begin(seconds: libc::c_uint) {
    unsafe {
        libc::alarm(seconds);
    }
}

/* Exit with an error message if the timeout has expired */
#[no_mangle]
pub unsafe extern "C" fn timeout_check(operation: *const c_char) {
    let timed_out = unsafe { ptr::read_volatile(ptr::addr_of!(timeout)) };

    if timed_out {
        unsafe {
            libc::fprintf(
                libc::stderr,
                b"%s timed out\n\0".as_ptr() as *const c_char,
                operation,
            );
            libc::exit(libc::EXIT_FAILURE);
        }
    }
}

/* Stop a timeout */
#[no_mangle]
pub unsafe extern "C" fn timeout_end() {
    unsafe {
        libc::alarm(0);
        ptr::write_volatile(ptr::addr_of_mut!(timeout), false);
    }
}

/* Sleep in a timeout section.
 *
 * nanosleep(2) can be used with this API since POSIX.1 explicitly
 * specifies that it does not interact with signals.
 */
#[no_mangle]
pub unsafe extern "C" fn timeout_usleep(usec: useconds_t) -> c_int {
    let ts = timespec {
        tv_sec: (usec / 1000000) as _,
        tv_nsec: ((usec % 1000000) * 1000) as _,
    };

    unsafe { libc::nanosleep(&ts, ptr::null_mut()) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
