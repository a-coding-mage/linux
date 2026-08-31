// SPDX-License-Identifier: GPL-2.0
/*
 * Tests for prctl(PR_GET_TSC, ...) / prctl(PR_SET_TSC, ...)
 *
 * Basic test to test behaviour of PR_GET_TSC and PR_SET_TSC
 */

use core::arch::asm;
use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};

/* Get/set the process' ability to use the timestamp counter instruction */
const PR_GET_TSC: c_int = 25;
const PR_SET_TSC: c_int = 26;
const PR_TSC_ENABLE: c_int = 1; /* allow the use of the timestamp counter */
const PR_TSC_SIGSEGV: c_int = 2; /* throw a SIGSEGV instead of reading the TSC */

const SIGSEGV: c_int = 11;
const EXIT_SUCCESS: c_int = 0;

type Sighandler = Option<unsafe extern "C" fn(c_int)>;

unsafe extern "C" {
    static mut stdout: *mut c_void;

    fn printf(format: *const c_char, ...) -> c_int;
    fn perror(s: *const c_char);
    fn fflush(stream: *mut c_void) -> c_int;
    fn prctl(option: c_int, ...) -> c_int;
    fn signal(signum: c_int, handler: Sighandler) -> Sighandler;
    fn exit(status: c_int) -> !;
}

const TSC_NAMES: [*const c_char; 3] = [
    c"[not set]".as_ptr(),
    c"PR_TSC_ENABLE".as_ptr(),
    c"PR_TSC_SIGSEGV".as_ptr(),
];

unsafe fn rdtsc() -> u64 {
    let lo: c_uint;
    let hi: c_uint;

    /* We cannot use "=A", since this would use %rax on x86_64 */
    unsafe {
        asm!("rdtsc", out("eax") lo, out("edx") hi, options(nomem, nostack, preserves_flags));
    }

    ((hi as u64) << 32) | (lo as u64)
}

unsafe extern "C" fn sigsegv_cb(_sig: c_int) {
    let mut tsc_val: c_int = 0;

    unsafe {
        printf(c"[ SIG_SEGV ]\n".as_ptr());
        printf(c"prctl(PR_GET_TSC, &tsc_val); ".as_ptr());
        fflush(stdout);

        if prctl(PR_GET_TSC, &mut tsc_val as *mut c_int) == -1 {
            perror(c"prctl".as_ptr());
        }

        printf(c"tsc_val == %s\n".as_ptr(), TSC_NAMES[tsc_val as usize]);
        printf(c"prctl(PR_SET_TSC, PR_TSC_ENABLE)\n".as_ptr());
        fflush(stdout);
        if prctl(PR_SET_TSC, PR_TSC_ENABLE) == -1 {
            perror(c"prctl".as_ptr());
        }

        printf(c"rdtsc() == ".as_ptr());
    }
}

fn main() {
    let mut tsc_val: c_int = 0;

    unsafe {
        signal(SIGSEGV, Some(sigsegv_cb));

        printf(
            c"rdtsc() == %llu\n".as_ptr(),
            rdtsc() as c_ulonglong,
        );
        printf(c"prctl(PR_GET_TSC, &tsc_val); ".as_ptr());
        fflush(stdout);

        if prctl(PR_GET_TSC, &mut tsc_val as *mut c_int) == -1 {
            perror(c"prctl".as_ptr());
        }

        printf(c"tsc_val == %s\n".as_ptr(), TSC_NAMES[tsc_val as usize]);
        printf(
            c"rdtsc() == %llu\n".as_ptr(),
            rdtsc() as c_ulonglong,
        );
        printf(c"prctl(PR_SET_TSC, PR_TSC_ENABLE)\n".as_ptr());
        fflush(stdout);

        if prctl(PR_SET_TSC, PR_TSC_ENABLE) == -1 {
            perror(c"prctl".as_ptr());
        }

        printf(
            c"rdtsc() == %llu\n".as_ptr(),
            rdtsc() as c_ulonglong,
        );
        printf(c"prctl(PR_SET_TSC, PR_TSC_SIGSEGV)\n".as_ptr());
        fflush(stdout);

        if prctl(PR_SET_TSC, PR_TSC_SIGSEGV) == -1 {
            perror(c"prctl".as_ptr());
        }

        printf(c"rdtsc() == ".as_ptr());
        fflush(stdout);
        printf(c"%llu\n".as_ptr(), rdtsc() as c_ulonglong);
        fflush(stdout);

        exit(EXIT_SUCCESS);
    }
}
