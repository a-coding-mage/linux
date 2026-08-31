/*
 *  sync stress test: producer/consumer
 *  Copyright 2015-2016 Collabora Ltd.
 *
 *  Based on the implementation from the Android Open Source Project,
 *
 *  Copyright 2012 Google, Inc
 *
 *  Permission is hereby granted, free of charge, to any person obtaining a
 *  copy of this software and associated documentation files (the "Software"),
 *  to deal in the Software without restriction, including without limitation
 *  the rights to use, copy, modify, merge, publish, distribute, sublicense,
 *  and/or sell copies of the Software, and to permit persons to whom the
 *  Software is furnished to do so, subject to the following conditions:
 *
 *  The above copyright notice and this permission notice shall be included in
 *  all copies or substantial portions of the Software.
 *
 *  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 *  THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
 *  OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 *  ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 *  OTHER DEALINGS IN THE SOFTWARE.
 */

/* Dependencies from pthread.h, sync.h, sw_sync.h, and synctest.h are expected
 * to be provided by the surrounding translated test harness.
 */

use std::ffi::{c_char, c_int, c_long, c_void};
use std::mem::MaybeUninit;
use std::ptr;

unsafe extern "C" {
    static FENCE_STATUS_ERROR: c_int;
    static FENCE_STATUS_ACTIVE: c_int;

    fn sync_fence_count_with_status(fence: c_int, status: c_int) -> c_int;
    fn sw_sync_fence_create(timeline: c_int, name: *const c_char, value: c_int) -> c_int;
    fn sw_sync_fence_is_valid(fence: c_int) -> c_int;
    fn sync_wait(fence: c_int, timeout: c_int) -> c_int;
    fn sw_sync_timeline_inc(timeline: c_int, count: c_int) -> c_int;
    fn sw_sync_fence_destroy(fence: c_int);
    fn sync_merge(name: *const c_char, fd1: c_int, fd2: c_int) -> c_int;
    fn sw_sync_timeline_create() -> c_int;

    fn pthread_mutex_init(
        mutex: *mut libc::pthread_mutex_t,
        attr: *const libc::pthread_mutexattr_t,
    ) -> c_int;
    fn pthread_mutex_lock(mutex: *mut libc::pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut libc::pthread_mutex_t) -> c_int;
    fn pthread_create(
        thread: *mut libc::pthread_t,
        attr: *const libc::pthread_attr_t,
        start_routine: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: libc::pthread_t, retval: *mut *mut c_void) -> c_int;
}

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

#[repr(C)]
struct TestDataMpsc {
    iterations: c_int,
    threads: c_int,
    counter: c_int,
    consumer_timeline: c_int,
    producer_timelines: *mut c_int,
    lock: MaybeUninit<libc::pthread_mutex_t>,
}

static mut test_data_mpsc: TestDataMpsc = TestDataMpsc {
    iterations: 0,
    threads: 0,
    counter: 0,
    consumer_timeline: 0,
    producer_timelines: ptr::null_mut(),
    lock: MaybeUninit::uninit(),
};

/* Returns 1 on error, 0 on success */
unsafe fn busy_wait_on_fence(fence: c_int) -> c_int {
    let mut error: c_int;
    let mut active: c_int;

    loop {
        error = sync_fence_count_with_status(fence, FENCE_STATUS_ERROR);
        ASSERT!(error == 0, "Error occurred on fence\n");
        active = sync_fence_count_with_status(fence, FENCE_STATUS_ACTIVE);
        if active == 0 {
            break;
        }
    }

    0
}

unsafe extern "C" fn mpsc_producer_thread(d: *mut c_void) -> *mut c_void {
    let id: c_int = d as c_long as c_int;
    let mut fence: c_int;
    let mut valid: c_int;
    let producer_timelines: *mut c_int = test_data_mpsc.producer_timelines;
    let consumer_timeline: c_int = test_data_mpsc.consumer_timeline;
    let iterations: c_int = test_data_mpsc.iterations;

    let mut i: c_int = 0;
    while i < iterations {
        fence = sw_sync_fence_create(consumer_timeline, c_str!("fence"), i);
        valid = sw_sync_fence_is_valid(fence);
        ASSERT!(valid != 0, "Failure creating fence\n");

        /*
         * Wait for the consumer to finish. Use alternate
         * means of waiting on the fence
         */

        if (iterations + id) % 8 != 0 {
            ASSERT!(sync_wait(fence, -1) > 0, "Failure waiting on fence\n");
        } else {
            ASSERT!(
                busy_wait_on_fence(fence) == 0,
                "Failure waiting on fence\n"
            );
        }

        /*
         * Every producer increments the counter, the consumer
         * checks and erases it
         */
        pthread_mutex_lock(test_data_mpsc.lock.as_mut_ptr());
        test_data_mpsc.counter += 1;
        pthread_mutex_unlock(test_data_mpsc.lock.as_mut_ptr());

        ASSERT!(
            sw_sync_timeline_inc(*producer_timelines.add(id as usize), 1) == 0,
            "Error advancing producer timeline\n"
        );

        sw_sync_fence_destroy(fence);
        i += 1;
    }

    ptr::null_mut()
}

unsafe fn mpcs_consumer_thread() -> c_int {
    let mut fence: c_int;
    let mut merged: c_int;
    let mut tmp: c_int;
    let mut valid: c_int;
    let producer_timelines: *mut c_int = test_data_mpsc.producer_timelines;
    let consumer_timeline: c_int = test_data_mpsc.consumer_timeline;
    let iterations: c_int = test_data_mpsc.iterations;
    let n: c_int = test_data_mpsc.threads;

    let mut it: c_int = 1;
    while it <= iterations {
        fence = sw_sync_fence_create(*producer_timelines.add(0), c_str!("name"), it);
        let mut i: c_int = 1;
        while i < n {
            tmp = sw_sync_fence_create(*producer_timelines.add(i as usize), c_str!("name"), it);
            merged = sync_merge(c_str!("name"), tmp, fence);
            sw_sync_fence_destroy(tmp);
            sw_sync_fence_destroy(fence);
            fence = merged;
            i += 1;
        }

        valid = sw_sync_fence_is_valid(fence);
        ASSERT!(valid != 0, "Failure merging fences\n");

        /*
         * Make sure we see an increment from every producer thread.
         * Vary the means by which we wait.
         */
        if iterations % 8 != 0 {
            ASSERT!(
                sync_wait(fence, -1) > 0,
                "Producers did not increment as expected\n"
            );
        } else {
            ASSERT!(
                busy_wait_on_fence(fence) == 0,
                "Producers did not increment as expected\n"
            );
        }

        ASSERT!(test_data_mpsc.counter == n * it, "Counter value mismatch!\n");

        /* Release the producer threads */
        ASSERT!(
            sw_sync_timeline_inc(consumer_timeline, 1) == 0,
            "Failure releasing producer threads\n"
        );

        sw_sync_fence_destroy(fence);
        it += 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn test_consumer_stress_multi_producer_single_consumer() -> c_int {
    let iterations: c_int = 1 << 12;
    let n: c_int = 5;
    let mut i: c_long;
    let ret: c_long;
    let mut producer_timelines: [c_int; 5] = [0; 5];
    let consumer_timeline: c_int;
    let mut threads: [MaybeUninit<libc::pthread_t>; 5] = MaybeUninit::uninit_array();

    consumer_timeline = sw_sync_timeline_create();
    i = 0;
    while i < n as c_long {
        producer_timelines[i as usize] = sw_sync_timeline_create();
        i += 1;
    }

    test_data_mpsc.producer_timelines = producer_timelines.as_mut_ptr();
    test_data_mpsc.consumer_timeline = consumer_timeline;
    test_data_mpsc.iterations = iterations;
    test_data_mpsc.threads = n;
    test_data_mpsc.counter = 0;
    pthread_mutex_init(test_data_mpsc.lock.as_mut_ptr(), ptr::null());

    i = 0;
    while i < n as c_long {
        pthread_create(
            threads[i as usize].as_mut_ptr(),
            ptr::null(),
            Some(mpsc_producer_thread),
            i as *mut c_void,
        );
        i += 1;
    }

    /* Consumer thread runs here */
    ret = mpcs_consumer_thread() as c_long;

    i = 0;
    while i < n as c_long {
        pthread_join(threads[i as usize].assume_init(), ptr::null_mut());
        i += 1;
    }

    ret as c_int
}
