// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2015, Cyril Bur, IBM Corp.
 * Copyright 2023, Michael Ellerman, IBM Corp.
 *
 * This test attempts to see if the FPU registers change across preemption.
 * There is no way to be sure preemption happened so this test just uses many
 * threads and a long wait. As such, a successful test doesn't mean much but
 * a failure is bad.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

/* Dependencies from:
 * <stdio.h>, <unistd.h>, <sys/syscall.h>, <sys/time.h>, <sys/types.h>,
 * <sys/wait.h>, <stdlib.h>, <pthread.h>, "utils.h", and "fpu.h".
 */

/* Time to wait for workers to get preempted (seconds) */
const PREEMPT_TIME: c_uint = 60;
/*
 * Factor by which to multiply number of online CPUs for total number of
 * worker threads
 */
const THREAD_FACTOR: c_long = 8;

type pthread_t = usize;

#[thread_local]
static mut darray: [f64; 32] = [0.0; 32];

static mut threads_starting: c_int = 0;
static mut running: c_int = 0;

unsafe extern "C" {
    static mut stdout: *mut c_void;
    static _SC_NPROCESSORS_ONLN: c_int;

    fn printf(format: *const c_char, ...) -> c_int;
    fn setbuf(stream: *mut c_void, buf: *mut c_char);
    fn sleep(seconds: c_uint) -> c_uint;
    fn sysconf(name: c_int) -> c_long;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn srand(seed: c_uint);
    fn pthread_self() -> pthread_t;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn randomise_darray(darray: *mut f64, size: usize);
    fn FAIL_IF(condition: c_int);
    fn test_harness(
        test_function: unsafe extern "C" fn() -> c_int,
        name: *const c_char,
    ) -> c_int;

    fn preempt_fpu(darray: *mut f64, threads_starting: *mut c_int, running: *mut c_int) -> c_int;
}

unsafe extern "C" fn preempt_fpu_c(_p: *mut c_void) -> *mut c_void {
    let rc: c_long;

    unsafe {
        srand(pthread_self() as c_uint);
        randomise_darray(darray.as_mut_ptr(), darray.len());
        rc = preempt_fpu(
            darray.as_mut_ptr(),
            &raw mut threads_starting,
            &raw mut running,
        ) as c_long;
    }

    rc as *mut c_void
}

unsafe extern "C" fn test_preempt_fpu() -> c_int {
    let mut i: c_int;
    let mut rc: c_int;
    let threads: c_int;
    let tids: *mut pthread_t;

    unsafe {
        threads = (sysconf(_SC_NPROCESSORS_ONLN) * THREAD_FACTOR) as c_int;
        tids = malloc((threads as usize) * core::mem::size_of::<pthread_t>()) as *mut pthread_t;
        FAIL_IF(tids.is_null() as c_int);

        running = 1;
        threads_starting = threads;
        i = 0;
        while i < threads {
            rc = pthread_create(tids.add(i as usize), core::ptr::null(), preempt_fpu_c, core::ptr::null_mut());
            FAIL_IF(rc);
            i += 1;
        }

        setbuf(stdout, core::ptr::null_mut());
        /* Not really necessary but nice to wait for every thread to start */
        printf(c"\tWaiting for all workers to start...".as_ptr());
        while threads_starting != 0 {
            core::arch::asm!("", options(nostack, preserves_flags));
        }
        printf(c"done\n".as_ptr());

        printf(
            c"\tWaiting for %d seconds to let some workers get preempted...".as_ptr(),
            PREEMPT_TIME as c_int,
        );
        sleep(PREEMPT_TIME);
        printf(c"done\n".as_ptr());

        printf(c"\tStopping workers...".as_ptr());
        /*
         * Working are checking this value every loop. In preempt_fpu 'cmpwi r5,0; bne 2b'.
         * r5 will have loaded the value of running.
         */
        running = 0;
        i = 0;
        while i < threads {
            let mut rc_p: *mut c_void = core::ptr::null_mut();
            pthread_join(*tids.add(i as usize), &mut rc_p);

            /*
             * Harness will say the fail was here, look at why preempt_fpu
             * returned
             */
            if (rc_p as c_long) != 0 {
                printf(c"oops\n".as_ptr());
            }
            FAIL_IF((rc_p as c_long) as c_int);
            i += 1;
        }
        printf(c"done\n".as_ptr());

        free(tids as *mut c_void);
    }

    0
}

unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    unsafe { test_harness(test_preempt_fpu, c"fpu_preempt".as_ptr()) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
