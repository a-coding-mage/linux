// SPDX-License-Identifier: GPL-2.0
/*
 * Tests for prctl(PR_GET_TSC, ...) / prctl(PR_SET_TSC, ...)
 *
 * Tests if the control register is updated correctly
 * at context switches
 *
 * Warning: this test will cause a very high load for a few seconds
 *
 */

use core::arch::asm;
use core::ffi::{c_char, c_int, c_void};
use core::ptr;

/* Get/set the process' ability to use the timestamp counter instruction */
const PR_GET_TSC: c_int = 25;
const PR_SET_TSC: c_int = 26;
const PR_TSC_ENABLE: c_int = 1; /* allow the use of the timestamp counter */
const PR_TSC_SIGSEGV: c_int = 2; /* throw a SIGSEGV instead of reading the TSC */

const SIGSEGV: c_int = 11;

unsafe extern "C" {
    static mut stderr: *mut c_void;

    fn alarm(seconds: u32) -> u32;
    fn exit(status: c_int) -> !;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn fork() -> c_int;
    fn perror(s: *const c_char);
    fn prctl(option: c_int, ...) -> c_int;
    fn signal(signum: c_int, handler: extern "C" fn(c_int)) -> extern "C" fn(c_int);
    fn wait(wstatus: *mut c_int) -> c_int;
}

unsafe fn rdtsc() -> u64 {
    let lo: u32;
    let hi: u32;

    /* We cannot use "=A", since this would use %rax on x86_64 */
    unsafe {
        asm!(
            "rdtsc",
            out("eax") lo,
            out("edx") hi,
            options(nomem, nostack, preserves_flags)
        );
    }

    ((hi as u64) << 32) | lo as u64
}

extern "C" fn sigsegv_expect(_sig: c_int) {
    /* */
}

unsafe fn segvtask() {
    if unsafe { prctl(PR_SET_TSC, PR_TSC_SIGSEGV) } < 0 {
        unsafe {
            perror(c"prctl".as_ptr());
            exit(0);
        }
    }

    unsafe {
        signal(SIGSEGV, sigsegv_expect);
        alarm(10);
        rdtsc();
        fprintf(
            stderr,
            c"FATAL ERROR, rdtsc() succeeded while disabled\n".as_ptr(),
        );
        exit(0);
    }
}

extern "C" fn sigsegv_fail(_sig: c_int) {
    unsafe {
        fprintf(
            stderr,
            c"FATAL ERROR, rdtsc() failed while enabled\n".as_ptr(),
        );
        exit(0);
    }
}

unsafe fn rdtsctask() {
    if unsafe { prctl(PR_SET_TSC, PR_TSC_ENABLE) } < 0 {
        unsafe {
            perror(c"prctl".as_ptr());
            exit(0);
        }
    }

    unsafe {
        signal(SIGSEGV, sigsegv_fail);
        alarm(10);
    }

    loop {
        unsafe {
            rdtsc();
        }
    }
}

fn main() {
    unsafe {
        let n_tasks: c_int = 100;
        let mut i: c_int;

        fprintf(stderr, c"[No further output means we're all right]\n".as_ptr());

        i = 0;
        while i < n_tasks {
            if fork() == 0 {
                if (i & 1) != 0 {
                    segvtask();
                } else {
                    rdtsctask();
                }
            }

            i += 1;
        }

        i = 0;
        while i < n_tasks {
            wait(ptr::null_mut());
            i += 1;
        }

        exit(0);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
