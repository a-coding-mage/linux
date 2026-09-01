// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright Collabora Ltd., 2021
 *
 * futex cmp requeue test by Andre Almeida <andrealmeid@collabora.com>
 */

// C dependencies: <limits.h>, <pthread.h>, <string.h>,
// "futextest.h", "futex_thread.h", "kselftest_harness.h"

use core::ffi::{c_char, c_int, c_uint, c_void};

type futex_t = u32;

const INT_MAX: c_int = c_int::MAX;

extern "C" {
    static mut errno: c_int;

    fn strerror(errnum: c_int) -> *mut c_char;

    fn futex_wait(
        uaddr: *const futex_t,
        val: futex_t,
        timeout: *const timespec,
        opflags: c_int,
    ) -> c_int;
    fn futex_cmp_requeue(
        uaddr: *const futex_t,
        val: futex_t,
        uaddr2: *const futex_t,
        nr_wake: c_int,
        nr_requeue: c_int,
        opflags: c_int,
    ) -> c_int;
    fn futex_wake(uaddr: *const futex_t, nr_wake: c_int, opflags: c_int) -> c_int;

    fn futex_thread_create(
        thread: *mut futex_thread,
        fn_: extern "C" fn(*mut c_void) -> c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn futex_wait_for_thread(thread: *mut futex_thread, metadata: *mut __test_metadata) -> c_int;
    fn futex_thread_destroy(thread: *mut futex_thread) -> c_int;

    fn TH_LOG(fmt: *const c_char, ...);
}

extern "C" {
    static WAIT_FOR_THREAD_SECS: c_uint;
}

#[repr(C)]
struct timespec {
    tv_sec: isize,
    tv_nsec: isize,
}

#[repr(C)]
struct __test_metadata {
    _private: [u8; 0],
}

#[repr(C)]
struct futex_thread {
    _private: [u8; 0],
}

#[repr(C)]
struct waiter_args {
    _metadata: *mut __test_metadata,
    n_threads: c_uint,
}

static mut f1: *mut futex_t = core::ptr::null_mut();

extern "C" fn waiterfn(arg: *mut c_void) -> c_int {
    unsafe {
        let mut _metadata: *mut __test_metadata;
        let wargs: *mut waiter_args = arg as *mut waiter_args;
        let mut to: timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        let res: c_int;

        _metadata = (*wargs)._metadata;
        to.tv_sec = (((*wargs).n_threads + 1) * WAIT_FOR_THREAD_SECS) as isize;

        res = futex_wait(f1, core::ptr::read_volatile(f1), &to, 0);
        if res != 0 {
            EXPECT_EQ(res, 0);
            TH_LOG(
                b"waiter failed errno %d: %s\0".as_ptr() as *const c_char,
                errno,
                strerror(errno),
            );
        }

        0
    }
}

// TEST(requeue_single)
unsafe fn requeue_single(_metadata: *mut __test_metadata) {
    let mut wargs: waiter_args = waiter_args {
        _metadata,
        n_threads: 1,
    };
    let mut waiter: futex_thread = core::mem::zeroed();
    let mut _f1: futex_t = 0;
    let mut f2: futex_t = 0;

    f1 = &mut _f1;

    /*
     * Requeue a waiter from f1 to f2, and wake f2.
     */
    ASSERT_EQ(
        futex_thread_create(
            &mut waiter,
            waiterfn,
            &mut wargs as *mut waiter_args as *mut c_void,
        ),
        0,
    );
    TH_LOG(b"pthread_create failed\0".as_ptr() as *const c_char);

    ASSERT_EQ(futex_wait_for_thread(&mut waiter, _metadata), 0);
    TH_LOG(b"Wait for thread failed\0".as_ptr() as *const c_char);

    EXPECT_EQ(futex_cmp_requeue(f1, 0, &mut f2, 0, 1, 0), 1);
    EXPECT_EQ(futex_wake(&mut f2, 1, 0), 1);

    EXPECT_EQ(futex_thread_destroy(&mut waiter), 0);
}

// TEST(requeue_multiple)
unsafe fn requeue_multiple(_metadata: *mut __test_metadata) {
    let mut wargs: waiter_args = waiter_args {
        _metadata,
        n_threads: 10,
    };
    let mut waiter: [futex_thread; 10] = core::mem::zeroed();
    let mut _f1: futex_t = 0;
    let mut f2: futex_t = 0;

    f1 = &mut _f1;

    /*
     * Create 10 waiters at f1. At futex_requeue, wake 3 and requeue 7.
     * At futex_wake, wake INT_MAX (should be exactly 7).
     */
    for i in 0..10 {
        ASSERT_EQ(
            futex_thread_create(
                &mut waiter[i],
                waiterfn,
                &mut wargs as *mut waiter_args as *mut c_void,
            ),
            0,
        );
        TH_LOG(
            b"pthread_create failed for waiter %d\0".as_ptr() as *const c_char,
            i as c_int,
        );
    }

    for i in 0..10 {
        ASSERT_EQ(futex_wait_for_thread(&mut waiter[i], _metadata), 0);
        TH_LOG(
            b"Wait for waiter thread %d failed\0".as_ptr() as *const c_char,
            i as c_int,
        );
    }

    EXPECT_EQ(futex_cmp_requeue(f1, 0, &mut f2, 3, 7, 0), 10);
    EXPECT_EQ(futex_wake(&mut f2, INT_MAX, 0), 7);

    for i in 0..10 {
        EXPECT_EQ(futex_thread_destroy(&mut waiter[i]), 0);
    }
}

extern "C" {
    fn ASSERT_EQ(left: c_int, right: c_int);
    fn EXPECT_EQ(left: c_int, right: c_int);
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
