// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2015, Cyril Bur, IBM Corp.
 *
 * This test attempts to see if the VMX registers change across preemption.
 * Two things should be noted here a) The check_vmx function in asm only checks
 * the non volatile registers as it is reused from the syscall test b) There is
 * no way to be sure preemption happened so this test just uses many threads
 * and a long wait. As such, a successful test doesn't mean much but a failure
 * is bad.
 */

/*
 * C dependencies:
 * stdio.h, unistd.h, sys/syscall.h, sys/time.h, sys/types.h, sys/wait.h,
 * stdlib.h, pthread.h, and "utils.h".
 */

use core::arch::asm;
use core::cell::UnsafeCell;
use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::size_of;

/* Time to wait for workers to get preempted (seconds) */
const PREEMPT_TIME: c_int = 20;
/*
 * Factor by which to multiply number of online CPUs for total number of
 * worker threads
 */
const THREAD_FACTOR: c_long = 8;

type pthread_t = usize;

#[repr(C, align(16))]
#[derive(Copy, Clone)]
pub struct vector_int {
    pub v: [c_int; 4],
}

thread_local! {
    static varray: UnsafeCell<[vector_int; 12]> = const { UnsafeCell::new([
        vector_int { v: [1, 2, 3, 4] },
        vector_int { v: [5, 6, 7, 8] },
        vector_int { v: [9, 10, 11, 12] },
        vector_int { v: [13, 14, 15, 16] },
        vector_int { v: [17, 18, 19, 20] },
        vector_int { v: [21, 22, 23, 24] },
        vector_int { v: [25, 26, 27, 28] },
        vector_int { v: [29, 30, 31, 32] },
        vector_int { v: [33, 34, 35, 36] },
        vector_int { v: [37, 38, 39, 40] },
        vector_int { v: [41, 42, 43, 44] },
        vector_int { v: [45, 46, 47, 48] },
    ]) };
}

static mut threads_starting: c_int = 0;
static mut running: c_int = 0;

unsafe extern "C" {
    fn preempt_vmx(
        varray: *mut vector_int,
        threads_starting: *mut c_int,
        running: *mut c_int,
    ) -> c_int;

    fn srand(seed: c_int);
    fn rand() -> c_int;
    fn pthread_self() -> pthread_t;
    fn sysconf(name: c_int) -> c_long;
    fn malloc(size: usize) -> *mut c_void;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn setbuf(stream: *mut c_void, buf: *mut c_char);
    fn printf(format: *const c_char, ...) -> c_int;
    fn sleep(seconds: c_int) -> c_int;

    static mut stdout: *mut c_void;

    fn have_hwcap2(feature: c_long) -> c_int;
    fn test_harness(
        test_function: unsafe extern "C" fn() -> c_int,
        name: *const c_char,
    ) -> c_int;
}

/* Constants and macros supplied by system headers / utils.h in the C source. */
const _SC_NPROCESSORS_ONLN: c_int = 84;
const PPC_FEATURE2_ARCH_2_07: c_long = 0x80000000u32 as c_long;

macro_rules! FAIL_IF {
    ($cond:expr) => {
        if $cond {
            return 1;
        }
    };
}

macro_rules! SKIP_IF {
    ($cond:expr) => {
        if $cond {
            return 0;
        }
    };
}

unsafe extern "C" fn preempt_vmx_c(_p: *mut c_void) -> *mut c_void {
    let mut i: c_int;
    let mut j: c_int;
    let rc: c_long;

    unsafe {
        srand(pthread_self() as c_int);
    }
    i = 0;
    while i < 12 {
        j = 0;
        while j < 4 {
            varray.with(|cell| unsafe {
                (*cell.get())[i as usize].v[j as usize] = rand();
            });
            j += 1;
        }
        i += 1;
    }

    rc = varray.with(|cell| unsafe {
        preempt_vmx(
            (*cell.get()).as_mut_ptr(),
            &raw mut threads_starting,
            &raw mut running,
        ) as c_long
    });

    rc as *mut c_void
}

unsafe extern "C" fn test_preempt_vmx() -> c_int {
    let mut i: c_int;
    let mut rc: c_int;
    let threads: c_int;
    let tids: *mut pthread_t;

    // vcmpequd used in vmx_asm.S is v2.07
    unsafe {
        SKIP_IF!(have_hwcap2(PPC_FEATURE2_ARCH_2_07) == 0);
    }

    unsafe {
        threads = (sysconf(_SC_NPROCESSORS_ONLN) * THREAD_FACTOR) as c_int;
        tids = malloc((threads as usize).wrapping_mul(size_of::<pthread_t>())) as *mut pthread_t;
    }
    FAIL_IF!(tids.is_null());

    unsafe {
        running = true as c_int;
        threads_starting = threads;
    }
    i = 0;
    while i < threads {
        unsafe {
            rc = pthread_create(tids.add(i as usize), core::ptr::null(), preempt_vmx_c, core::ptr::null_mut());
        }
        FAIL_IF!(rc != 0);
        i += 1;
    }

    unsafe {
        setbuf(stdout, core::ptr::null_mut());
    }
    /* Not really nessesary but nice to wait for every thread to start */
    unsafe {
        printf(c"\tWaiting for all workers to start...".as_ptr());
    }
    while unsafe { threads_starting } != 0 {
        unsafe {
            asm!("", options(nostack, preserves_flags));
        }
    }
    unsafe {
        printf(c"done\n".as_ptr());
    }

    unsafe {
        printf(
            c"\tWaiting for %d seconds to let some workers get preempted...".as_ptr(),
            PREEMPT_TIME,
        );
        sleep(PREEMPT_TIME);
        printf(c"done\n".as_ptr());
    }

    unsafe {
        printf(c"\tStopping workers...".as_ptr());
    }
    /*
     * Working are checking this value every loop. In preempt_vmx 'cmpwi r5,0; bne 2b'.
     * r5 will have loaded the value of running.
     */
    unsafe {
        running = 0;
    }
    i = 0;
    while i < threads {
        let mut rc_p: *mut c_void = core::ptr::null_mut();
        unsafe {
            pthread_join(*tids.add(i as usize), &mut rc_p);
        }

        /*
         * Harness will say the fail was here, look at why preempt_vmx
         * returned
         */
        if (rc_p as c_long) != 0 {
            unsafe {
                printf(c"oops\n".as_ptr());
            }
        }
        FAIL_IF!((rc_p as c_long) != 0);
        i += 1;
    }
    unsafe {
        printf(c"done\n".as_ptr());
    }

    0
}

fn main() {
    unsafe {
        test_harness(test_preempt_vmx, c"vmx_preempt".as_ptr());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
