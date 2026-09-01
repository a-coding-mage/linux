// SPDX-License-Identifier: GPL-2.0-only
/*
 * POWER Data Stream Control Register (DSCR) default test
 *
 * This test modifies the system wide default DSCR through
 * it's sysfs interface and then verifies that all threads
 * see the correct changed DSCR value immediately.
 *
 * Copyright 2012, Anton Blanchard, IBM Corporation.
 * Copyright 2015, Anshuman Khandual, IBM Corporation.
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

// Dependencies from "dscr.h" and the pthread/semaphore C headers.
// Their definitions are supplied by the surrounding translated test harness.
type pthread_t = c_ulong;
type sem_t = c_void;
type pthread_rwlock_t = c_void;
type pthread_rwlockattr_t = c_void;
type pthread_barrier_t = c_void;

unsafe extern "C" {
    static COUNT: c_int;
    static THREADS: c_int;
    static DSCR_MAX: c_ulong;
    static PPC_FEATURE2_DSCR: c_ulong;
    static BIND_CPU_ANY: c_int;
    static PTHREAD_BARRIER_SERIAL_THREAD: c_int;
    static PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: c_int;

    fn have_hwcap2(feature: c_ulong) -> c_int;
    fn get_dscr() -> c_ulong;
    fn get_dscr_usr() -> c_ulong;
    fn get_default_dscr() -> c_ulong;
    fn set_default_dscr(dscr: c_ulong);
    fn bind_to_cpu(cpu: c_int) -> c_int;
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;

    fn sem_wait(sem: *mut sem_t) -> c_int;
    fn sem_post(sem: *mut sem_t) -> c_int;
    fn sem_init(sem: *mut sem_t, pshared: c_int, value: c_uint) -> c_int;
    fn sem_destroy(sem: *mut sem_t) -> c_int;

    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_exit(retval: *mut c_void) -> !;
    fn pthread_rwlockattr_setkind_np(attr: *mut pthread_rwlockattr_t, pref: c_int) -> c_int;
    fn pthread_rwlock_init(lock: *mut pthread_rwlock_t, attr: *const pthread_rwlockattr_t) -> c_int;
    fn pthread_rwlock_rdlock(lock: *mut pthread_rwlock_t) -> c_int;
    fn pthread_rwlock_wrlock(lock: *mut pthread_rwlock_t) -> c_int;
    fn pthread_rwlock_unlock(lock: *mut pthread_rwlock_t) -> c_int;
    fn pthread_rwlock_destroy(lock: *mut pthread_rwlock_t) -> c_int;
    fn pthread_barrier_init(
        barrier: *mut pthread_barrier_t,
        attr: *const c_void,
        count: c_uint,
    ) -> c_int;
    fn pthread_barrier_wait(barrier: *mut pthread_barrier_t) -> c_int;
    fn pthread_barrier_destroy(barrier: *mut pthread_barrier_t) -> c_int;

    fn srand(seed: c_uint);
    fn rand() -> c_int;
    fn gettid() -> c_int;
}

macro_rules! FAIL_IF {
    ($cond:expr) => {
        if $cond {
            return 1;
        }
    };
}

macro_rules! FAIL_IF_EXIT {
    ($cond:expr) => {
        if $cond {
            pthread_exit(1usize as *mut c_void);
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

unsafe extern "C" fn dscr_default_lockstep_writer(arg: *mut c_void) -> *mut c_void {
    let reader_sem = arg as *mut sem_t;
    let writer_sem = (arg as *mut sem_t).add(1);
    let mut expected_dscr: c_ulong = 0;

    let mut i = 0;
    while i < COUNT {
        FAIL_IF_EXIT!(sem_wait(writer_sem) != 0);

        set_default_dscr(expected_dscr);
        expected_dscr = (expected_dscr + 1) % DSCR_MAX;

        FAIL_IF_EXIT!(sem_post(reader_sem) != 0);
        i += 1;
    }

    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn dscr_default_lockstep_test() -> c_int {
    let mut writer: pthread_t = 0;
    let mut rw_semaphores: [sem_t; 2] = core::mem::zeroed();
    let reader_sem = &mut rw_semaphores[0] as *mut sem_t;
    let writer_sem = &mut rw_semaphores[1] as *mut sem_t;
    let mut expected_dscr: c_ulong = 0;

    SKIP_IF!(have_hwcap2(PPC_FEATURE2_DSCR) == 0);

    FAIL_IF!(sem_init(reader_sem, 0, 0) != 0);
    FAIL_IF!(sem_init(writer_sem, 0, 1) != 0); /* writer starts first */
    FAIL_IF!(bind_to_cpu(BIND_CPU_ANY) < 0);
    FAIL_IF!(
        pthread_create(
            &mut writer,
            ptr::null(),
            dscr_default_lockstep_writer,
            rw_semaphores.as_mut_ptr() as *mut c_void,
        ) != 0
    );

    let mut i = 0;
    while i < COUNT {
        FAIL_IF!(sem_wait(reader_sem) != 0);

        FAIL_IF!(get_dscr() != expected_dscr);
        FAIL_IF!(get_dscr_usr() != expected_dscr);

        expected_dscr = (expected_dscr + 1) % DSCR_MAX;

        FAIL_IF!(sem_post(writer_sem) != 0);
        i += 1;
    }

    FAIL_IF!(pthread_join(writer, ptr::null_mut()) != 0);
    FAIL_IF!(sem_destroy(reader_sem) != 0);
    FAIL_IF!(sem_destroy(writer_sem) != 0);

    0
}

#[repr(C)]
struct random_thread_args {
    thread_id: pthread_t,
    expected_system_dscr: *mut c_ulong,
    rw_lock: *mut pthread_rwlock_t,
    barrier: *mut pthread_barrier_t,
}

unsafe extern "C" fn dscr_default_random_thread(in_: *mut c_void) -> *mut c_void {
    let args = in_ as *mut random_thread_args;
    let expected_dscr_p = (*args).expected_system_dscr;
    let rw_lock = (*args).rw_lock;
    let mut err: c_int;

    srand(gettid() as c_uint);

    err = pthread_barrier_wait((*args).barrier);
    FAIL_IF_EXIT!(err != 0 && err != PTHREAD_BARRIER_SERIAL_THREAD);

    let mut i = 0;
    while i < COUNT {
        let expected_dscr: c_ulong;
        let current_dscr: c_ulong;
        let current_dscr_usr: c_ulong;

        FAIL_IF_EXIT!(pthread_rwlock_rdlock(rw_lock) != 0);
        expected_dscr = *expected_dscr_p;
        current_dscr = get_dscr();
        current_dscr_usr = get_dscr_usr();
        FAIL_IF_EXIT!(pthread_rwlock_unlock(rw_lock) != 0);

        FAIL_IF_EXIT!(current_dscr != expected_dscr);
        FAIL_IF_EXIT!(current_dscr_usr != expected_dscr);

        if rand() % 10 == 0 {
            let next_dscr: c_ulong;

            FAIL_IF_EXIT!(pthread_rwlock_wrlock(rw_lock) != 0);
            next_dscr = (*expected_dscr_p + 1) % DSCR_MAX;
            set_default_dscr(next_dscr);
            *expected_dscr_p = next_dscr;
            FAIL_IF_EXIT!(pthread_rwlock_unlock(rw_lock) != 0);
        }

        i += 1;
    }

    pthread_exit(0usize as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn dscr_default_random_test() -> c_int {
    let mut threads: [random_thread_args; THREADS as usize] = core::mem::zeroed();
    let mut expected_system_dscr: c_ulong = 0;
    let mut rwlock_attr: pthread_rwlockattr_t = core::mem::zeroed();
    let mut rw_lock: pthread_rwlock_t = core::mem::zeroed();
    let mut barrier: pthread_barrier_t = core::mem::zeroed();

    SKIP_IF!(have_hwcap2(PPC_FEATURE2_DSCR) == 0);

    FAIL_IF!(
        pthread_rwlockattr_setkind_np(
            &mut rwlock_attr,
            PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP,
        ) != 0
    );
    FAIL_IF!(pthread_rwlock_init(&mut rw_lock, &rwlock_attr) != 0);
    FAIL_IF!(pthread_barrier_init(&mut barrier, ptr::null(), THREADS as c_uint) != 0);

    set_default_dscr(expected_system_dscr);

    let mut i = 0;
    while i < THREADS {
        threads[i as usize].expected_system_dscr = &mut expected_system_dscr;
        threads[i as usize].rw_lock = &mut rw_lock;
        threads[i as usize].barrier = &mut barrier;

        FAIL_IF!(
            pthread_create(
                &mut threads[i as usize].thread_id,
                ptr::null(),
                dscr_default_random_thread,
                &mut threads[i as usize] as *mut random_thread_args as *mut c_void,
            ) != 0
        );
        i += 1;
    }

    i = 0;
    while i < THREADS {
        FAIL_IF!(pthread_join(threads[i as usize].thread_id, ptr::null_mut()) != 0);
        i += 1;
    }

    FAIL_IF!(pthread_barrier_destroy(&mut barrier) != 0);
    FAIL_IF!(pthread_rwlock_destroy(&mut rw_lock) != 0);

    0
}

#[no_mangle]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut orig_dscr_default: c_ulong = 0;
    let mut err: c_int = 0;

    if have_hwcap2(PPC_FEATURE2_DSCR) != 0 {
        orig_dscr_default = get_default_dscr();
    }

    err |= test_harness(
        dscr_default_lockstep_test,
        c"dscr_default_lockstep_test".as_ptr(),
    );
    err |= test_harness(
        dscr_default_random_test,
        c"dscr_default_random_test".as_ptr(),
    );

    if have_hwcap2(PPC_FEATURE2_DSCR) != 0 {
        set_default_dscr(orig_dscr_default);
    }

    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
