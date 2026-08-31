// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2015, Cyril Bur, IBM Corp.
 *
 * This test attempts to see if the FPU registers are correctly reported in a
 * signal context. Each worker just spins checking its FPU registers, at some
 * point a signal will interrupt it and C code will check the signal context
 * ensuring it is also the same.
 */

// C dependencies: stdio.h, unistd.h, sys/syscall.h, sys/time.h, sys/types.h,
// sys/wait.h, stdlib.h, pthread.h, "utils.h", "fpu.h".

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_void};

/* Number of times each thread should receive the signal */
const ITERATIONS: c_int = 10;
/*
 * Factor by which to multiply number of online CPUs for total number of
 * worker threads
 */
const THREAD_FACTOR: c_long = 8;

type PthreadT = libc::pthread_t;
type SiginfoT = libc::siginfo_t;
type UcontextT = libc::ucontext_t;
type McontextT = libc::mcontext_t;

#[thread_local]
static mut darray: [f64; 32] = [0.0; 32];

static mut bad_context: bool = false;
static mut threads_starting: c_int = 0;
static mut running: c_int = 0;

unsafe extern "C" {
    fn preempt_fpu(
        darray: *mut f64,
        threads_starting: *mut c_int,
        running: *mut c_int,
    ) -> c_long;

    fn randomise_darray(darray: *mut f64, size: usize);
    fn test_harness(
        test_function: unsafe extern "C" fn() -> c_int,
        name: *const c_char,
    ) -> c_int;
    fn FAIL_IF(condition: bool);
}

unsafe extern "C" fn signal_fpu_sig(
    _sig: c_int,
    _info: *mut SiginfoT,
    context: *mut c_void,
) {
    let uc: *mut UcontextT = context as *mut UcontextT;
    let mc: *mut McontextT = unsafe { &mut (*uc).uc_mcontext };

    // Don't check f30/f31, they're used as scratches in check_all_fprs()
    let mut i: c_int = 0;
    while i < 30 {
        if unsafe { (*mc).fp_regs[i as usize] != darray[i as usize] } {
            unsafe {
                bad_context = true;
            }
            break;
        }
        i += 1;
    }
}

unsafe extern "C" fn signal_fpu_c(p: *mut c_void) -> *mut c_void {
    let mut rc: c_long;
    let mut act: libc::sigaction = unsafe { core::mem::zeroed() };

    act.sa_sigaction = signal_fpu_sig as usize;
    act.sa_flags = libc::SA_SIGINFO;
    rc = unsafe { libc::sigaction(libc::SIGUSR1, &act, core::ptr::null_mut()) } as c_long;
    if rc != 0 {
        return p;
    }

    unsafe {
        libc::srand(libc::pthread_self() as u32);
        randomise_darray(darray.as_mut_ptr(), darray.len());
        rc = preempt_fpu(
            darray.as_mut_ptr(),
            &mut threads_starting,
            &mut running,
        );
    }

    rc as *mut c_void
}

unsafe extern "C" fn test_signal_fpu() -> c_int {
    let mut i: c_int;
    let mut j: c_int;
    let mut rc: c_int;
    let threads: c_int;
    let mut rc_p: *mut c_void = core::ptr::null_mut();
    let tids: *mut PthreadT;

    threads = (unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) } * THREAD_FACTOR) as c_int;
    tids = unsafe {
        libc::malloc((threads as usize) * core::mem::size_of::<PthreadT>()) as *mut PthreadT
    };
    unsafe {
        FAIL_IF(tids.is_null());
    }

    unsafe {
        running = true as c_int;
        threads_starting = threads;
    }
    i = 0;
    while i < threads {
        rc = unsafe {
            libc::pthread_create(
                tids.add(i as usize),
                core::ptr::null(),
                signal_fpu_c,
                core::ptr::null_mut(),
            )
        };
        unsafe {
            FAIL_IF(rc != 0);
        }
        i += 1;
    }

    unsafe {
        libc::setbuf(libc::stdout, core::ptr::null_mut());
        libc::printf(c"\tWaiting for all workers to start...".as_ptr());
    }
    while unsafe { threads_starting != 0 } {
        unsafe {
            asm!("", options(nostack, preserves_flags));
        }
    }
    unsafe {
        libc::printf(c"done\n".as_ptr());
    }

    unsafe {
        libc::printf(
            c"\tSending signals to all threads %d times...".as_ptr(),
            ITERATIONS,
        );
    }
    i = 0;
    while i < ITERATIONS {
        j = 0;
        while j < threads {
            unsafe {
                libc::pthread_kill(*tids.add(j as usize), libc::SIGUSR1);
            }
            j += 1;
        }
        unsafe {
            libc::sleep(1);
        }
        i += 1;
    }
    unsafe {
        libc::printf(c"done\n".as_ptr());
    }

    unsafe {
        libc::printf(c"\tStopping workers...".as_ptr());
        running = 0;
    }
    i = 0;
    while i < threads {
        unsafe {
            libc::pthread_join(*tids.add(i as usize), &mut rc_p);
        }

        /*
         * Harness will say the fail was here, look at why signal_fpu
         * returned
         */
        if (rc_p as c_long) != 0 || unsafe { bad_context } {
            unsafe {
                libc::printf(c"oops\n".as_ptr());
            }
        }
        if unsafe { bad_context } {
            unsafe {
                libc::fprintf(libc::stderr, c"\t!! bad_context is true\n".as_ptr());
            }
        }
        unsafe {
            FAIL_IF((rc_p as c_long) != 0 || bad_context);
        }
        i += 1;
    }
    unsafe {
        libc::printf(c"done\n".as_ptr());
        libc::free(tids as *mut c_void);
    }
    0
}

unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    unsafe { test_harness(test_signal_fpu, c"fpu_signal".as_ptr()) }
}
