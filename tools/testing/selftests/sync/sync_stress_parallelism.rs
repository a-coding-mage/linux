/*
 *  sync stress test: parallelism
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

// C dependencies: <pthread.h>, "sync.h", "sw_sync.h", "synctest.h".

use core::ffi::{c_char, c_int, c_long, c_void};

type pthread_t = usize;

unsafe extern "C" {
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;

    fn sw_sync_fence_create(timeline: c_int, name: *const c_char, value: c_int) -> c_int;
    fn sw_sync_fence_is_valid(fence: c_int) -> c_int;
    fn sw_sync_fence_destroy(fence: c_int);
    fn sw_sync_timeline_create() -> c_int;
    fn sw_sync_timeline_is_valid(timeline: c_int) -> c_int;
    fn sw_sync_timeline_inc(timeline: c_int, count: c_int) -> c_int;
    fn sw_sync_timeline_destroy(timeline: c_int);
    fn sync_wait(fd: c_int, timeout: c_int) -> c_int;

    fn ASSERT(condition: c_int, message: *const c_char);
}

#[repr(C)]
struct TestDataTwoThreads {
    iterations: c_int,
    timeline: c_int,
    counter: c_int,
}

static mut test_data_two_threads: TestDataTwoThreads = TestDataTwoThreads {
    iterations: 0,
    timeline: 0,
    counter: 0,
};

unsafe extern "C" fn test_stress_two_threads_shared_timeline_thread(d: *mut c_void) -> *mut c_void {
    let thread_id: c_int = d as c_long as c_int;
    let timeline: c_int = unsafe { test_data_two_threads.timeline };
    let iterations: c_int = unsafe { test_data_two_threads.iterations };
    let mut fence: c_int;
    let mut valid: c_int;
    let mut ret: c_int;
    let mut i: c_int;

    i = 0;
    while i < iterations {
        fence = unsafe {
            sw_sync_fence_create(
                timeline,
                c"fence".as_ptr(),
                i.wrapping_mul(2).wrapping_add(thread_id),
            )
        };
        valid = unsafe { sw_sync_fence_is_valid(fence) };
        unsafe {
            ASSERT(valid, c"Failure allocating fence\n".as_ptr());
        }

        /* Wait on the prior thread to complete */
        ret = unsafe { sync_wait(fence, -1) };
        unsafe {
            ASSERT((ret > 0) as c_int, c"Problem occurred on prior thread\n".as_ptr());
        }

        /*
         * Confirm the previous thread's writes are visible
         * and then increment
         */
        unsafe {
            ASSERT(
                (test_data_two_threads.counter == i.wrapping_mul(2).wrapping_add(thread_id))
                    as c_int,
                c"Counter got damaged!\n".as_ptr(),
            );
            test_data_two_threads.counter = test_data_two_threads.counter.wrapping_add(1);
        }

        /* Kick off the other thread */
        ret = unsafe { sw_sync_timeline_inc(timeline, 1) };
        unsafe {
            ASSERT((ret == 0) as c_int, c"Advancing timeline failed\n".as_ptr());
        }

        unsafe {
            sw_sync_fence_destroy(fence);
        }

        i = i.wrapping_add(1);
    }

    core::ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_stress_two_threads_shared_timeline() -> c_int {
    let mut a: pthread_t = 0;
    let mut b: pthread_t = 0;
    let mut valid: c_int;
    let timeline: c_int = unsafe { sw_sync_timeline_create() };

    valid = unsafe { sw_sync_timeline_is_valid(timeline) };
    unsafe {
        ASSERT(valid, c"Failure allocating timeline\n".as_ptr());
    }

    unsafe {
        test_data_two_threads.iterations = 1 << 16;
        test_data_two_threads.counter = 0;
        test_data_two_threads.timeline = timeline;
    }

    /*
     * Use a single timeline to synchronize two threads
     * hammmering on the same counter.
     */

    unsafe {
        pthread_create(
            &mut a,
            core::ptr::null(),
            Some(test_stress_two_threads_shared_timeline_thread),
            0 as *mut c_void,
        );
        pthread_create(
            &mut b,
            core::ptr::null(),
            Some(test_stress_two_threads_shared_timeline_thread),
            1 as *mut c_void,
        );

        pthread_join(a, core::ptr::null_mut());
        pthread_join(b, core::ptr::null_mut());
    }

    /* make sure the threads did not trample on one another */
    unsafe {
        ASSERT(
            (test_data_two_threads.counter == test_data_two_threads.iterations.wrapping_mul(2))
                as c_int,
            c"Counter has unexpected value\n".as_ptr(),
        );
    }

    unsafe {
        sw_sync_timeline_destroy(timeline);
    }

    0
}
