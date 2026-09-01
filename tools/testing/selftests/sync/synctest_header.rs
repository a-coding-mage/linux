/*
 *  sync tests
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

use std::os::raw::{c_char, c_int};

// C dependencies: <stdio.h> and "../kselftest.h".
// `ksft_print_msg` is provided by kselftest.
extern "C" {
    pub fn ksft_print_msg(fmt: *const c_char, ...);
}

macro_rules! ASSERT {
    ($cond:expr, $msg:expr) => {
        if !($cond) {
            unsafe {
                ksft_print_msg(b"[ERROR]\t%s\0".as_ptr() as *const c_char, $msg);
            }
            return 1;
        }
    };
}

macro_rules! RUN_TEST {
    ($x:expr) => {
        run_test($x, concat!(stringify!($x), "\0").as_ptr() as *const c_char)
    };
}

extern "C" {
    pub fn run_test(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

/* Allocation tests */
extern "C" {
    pub fn test_alloc_timeline() -> c_int;
    pub fn test_alloc_fence() -> c_int;
    pub fn test_alloc_fence_negative() -> c_int;
}

/* Fence tests with one timeline */
extern "C" {
    pub fn test_fence_one_timeline_wait() -> c_int;
    pub fn test_fence_one_timeline_merge() -> c_int;
}

/* Fence merge tests */
extern "C" {
    pub fn test_fence_merge_same_fence() -> c_int;
}

/* Fence wait tests */
extern "C" {
    pub fn test_fence_multi_timeline_wait() -> c_int;
}

/* Stress test - parallelism */
extern "C" {
    pub fn test_stress_two_threads_shared_timeline() -> c_int;
}

/* Stress test - consumer */
extern "C" {
    pub fn test_consumer_stress_multi_producer_single_consumer() -> c_int;
}

/* Stress test - merging */
extern "C" {
    pub fn test_merge_stress_random_merge() -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
