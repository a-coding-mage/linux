// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2015, Cyril Bur, IBM Corp.
 *
 * This test attempts to see if the VSX registers change across preemption.
 * There is no way to be sure preemption happened so this test just
 * uses many threads and a long wait. As such, a successful test
 * doesn't mean much but a failure is bad.
 */

/*
 * C dependencies:
 * stdio.h, string.h, unistd.h, sys/syscall.h, sys/time.h, sys/types.h,
 * sys/wait.h, stdlib.h, pthread.h, and "utils.h".
 */

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

/* Time to wait for workers to get preempted (seconds) */
const PREEMPT_TIME: c_uint = 20;
/*
 * Factor by which to multiply number of online CPUs for total number of
 * worker threads
 */
const THREAD_FACTOR: c_long = 8;

const PPC_FEATURE_HAS_VSX: c_ulong = 0x0000_0080;
const _SC_NPROCESSORS_ONLN: c_int = 84;

type c_ulong = u64;
type pthread_t = c_ulong;

#[repr(C, align(16))]
#[derive(Copy, Clone)]
pub struct vector_int {
    pub v: [c_int; 4],
}

impl vector_int {
    const fn new(a: c_int, b: c_int, c: c_int, d: c_int) -> Self {
        Self { v: [a, b, c, d] }
    }
}

/*
 * Ensure there is twice the number of non-volatile VMX regs!
 * check_vmx() is going to use the other half as space to put the live
 * registers before calling vsx_memcmp()
 */
#[thread_local]
static mut varray: [vector_int; 24] = [
    vector_int::new(1, 2, 3, 4),
    vector_int::new(5, 6, 7, 8),
    vector_int::new(9, 10, 11, 12),
    vector_int::new(13, 14, 15, 16),
    vector_int::new(17, 18, 19, 20),
    vector_int::new(21, 22, 23, 24),
    vector_int::new(25, 26, 27, 28),
    vector_int::new(29, 30, 31, 32),
    vector_int::new(33, 34, 35, 36),
    vector_int::new(37, 38, 39, 40),
    vector_int::new(41, 42, 43, 44),
    vector_int::new(45, 46, 47, 48),
    vector_int::new(0, 0, 0, 0),
    vector_int::new(0, 0, 0, 0),
    vector_int::new(0, 0, 0, 0),
    vector_int::new(0, 0, 0, 0),
    vector_int::new(0, 0, 0, 0),
    vector_int::new(0, 0, 0, 0),
    vector_int::new(0, 0, 0, 0),
    vector_int::new(0, 0, 0, 0),
    vector_int::new(0, 0, 0, 0),
    vector_int::new(0, 0, 0, 0),
    vector_int::new(0, 0, 0, 0),
    vector_int::new(0, 0, 0, 0),
];

static mut threads_starting: c_int = 0;
static mut running: c_int = 0;

extern "C" {
    fn preempt_vsx(
        varray: *mut vector_int,
        threads_starting: *mut c_int,
        running: *mut c_int,
    ) -> c_long;

    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn pthread_self() -> pthread_t;
    fn srand(seed: c_uint);
    fn rand() -> c_int;
    fn sysconf(name: c_int) -> c_long;
    fn malloc(size: usize) -> *mut c_void;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const pthread_attr_t,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn setbuf(stream: *mut FILE, buf: *mut c_char);
    fn sleep(seconds: c_uint) -> c_uint;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn test_harness(
        test_function: unsafe extern "C" fn() -> c_int,
        name: *const c_char,
    ) -> c_int;
    fn have_hwcap(feature: c_ulong) -> c_int;

    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pthread_attr_t {
    _private: [u8; 0],
}

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

#[no_mangle]
pub unsafe extern "C" fn vsx_memcmp(a: *mut vector_int) -> c_long {
    let zero = vector_int::new(0, 0, 0, 0);
    let mut i: c_int;

    FAIL_IF!(a != ptr::addr_of_mut!(varray) as *mut vector_int);

    i = 0;
    while i < 12 {
        if memcmp(
            a.add((i + 12) as usize) as *const c_void,
            &zero as *const vector_int as *const c_void,
            size_of::<vector_int>(),
        ) == 0
        {
            fprintf(
                stderr,
                b"Detected zero from the VSX reg %d\n\0".as_ptr() as *const c_char,
                i + 12,
            );
            return 2;
        }
        i += 1;
    }

    if memcmp(
        a as *const c_void,
        a.add(12) as *const c_void,
        12 * size_of::<vector_int>(),
    ) != 0
    {
        let p = a as *mut c_long;
        fprintf(stderr, b"VSX mismatch\n\0".as_ptr() as *const c_char);
        i = 0;
        while i < 24 {
            fprintf(
                stderr,
                b"%d: 0x%08lx%08lx | 0x%08lx%08lx\n\0".as_ptr() as *const c_char,
                i / 2 + i % 2 + 20,
                *p.add(i as usize),
                *p.add((i + 1) as usize),
                *p.add((i + 24) as usize),
                *p.add((i + 25) as usize),
            );
            i += 2;
        }
        return 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn preempt_vsx_c(_p: *mut c_void) -> *mut c_void {
    let mut i: c_int;
    let mut j: c_int;
    let rc: c_long;
    srand(pthread_self() as c_uint);
    i = 0;
    while i < 12 {
        j = 0;
        while j < 4 {
            varray[i as usize].v[j as usize] = rand();
            /* Don't want zero because it hides kernel problems */
            if varray[i as usize].v[j as usize] == 0 {
                j -= 1;
            }
            j += 1;
        }
        i += 1;
    }
    rc = preempt_vsx(
        ptr::addr_of_mut!(varray) as *mut vector_int,
        ptr::addr_of_mut!(threads_starting),
        ptr::addr_of_mut!(running),
    );
    if rc == 2 {
        fprintf(
            stderr,
            b"Caught zeros in VSX compares\n\0".as_ptr() as *const c_char,
        );
    }
    rc as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn test_preempt_vsx() -> c_int {
    let mut i: c_int;
    let mut rc: c_int;
    let threads: c_int;
    let tids: *mut pthread_t;

    SKIP_IF!(have_hwcap(PPC_FEATURE_HAS_VSX) == 0);

    threads = (sysconf(_SC_NPROCESSORS_ONLN) * THREAD_FACTOR) as c_int;
    tids = malloc((threads as usize) * size_of::<pthread_t>()) as *mut pthread_t;
    FAIL_IF!(tids.is_null());

    running = 1;
    threads_starting = threads;
    i = 0;
    while i < threads {
        rc = pthread_create(
            tids.add(i as usize),
            ptr::null(),
            preempt_vsx_c,
            ptr::null_mut(),
        );
        FAIL_IF!(rc != 0);
        i += 1;
    }

    setbuf(stdout, ptr::null_mut());
    /* Not really nessesary but nice to wait for every thread to start */
    printf(
        b"\tWaiting for %d workers to start...\0".as_ptr() as *const c_char,
        threads_starting,
    );
    while threads_starting != 0 {
        asm!("", options(nostack, preserves_flags));
    }
    printf(b"done\n\0".as_ptr() as *const c_char);

    printf(
        b"\tWaiting for %d seconds to let some workers get preempted...\0".as_ptr()
            as *const c_char,
        PREEMPT_TIME,
    );
    sleep(PREEMPT_TIME);
    printf(b"done\n\0".as_ptr() as *const c_char);

    printf(b"\tStopping workers...\0".as_ptr() as *const c_char);
    /*
     * Working are checking this value every loop. In preempt_vsx 'cmpwi r5,0; bne 2b'.
     * r5 will have loaded the value of running.
     */
    running = 0;
    i = 0;
    while i < threads {
        let mut rc_p: *mut c_void = ptr::null_mut();
        pthread_join(*tids.add(i as usize), &mut rc_p);

        /*
         * Harness will say the fail was here, look at why preempt_vsx
         * returned
         */
        if (rc_p as c_long) != 0 {
            printf(b"oops\n\0".as_ptr() as *const c_char);
        }
        FAIL_IF!((rc_p as c_long) != 0);
        i += 1;
    }
    printf(b"done\n\0".as_ptr() as *const c_char);

    0
}

#[no_mangle]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    test_harness(
        test_preempt_vsx,
        b"vsx_preempt\0".as_ptr() as *const c_char,
    )
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
