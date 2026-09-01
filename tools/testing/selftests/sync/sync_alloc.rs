/*
 *  sync allocation tests
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

// C dependencies: "sync.h", "sw_sync.h", and "synctest.h".
unsafe extern "C" {
    fn sw_sync_timeline_create() -> c_int;
    fn sw_sync_timeline_is_valid(timeline: c_int) -> c_int;
    fn sw_sync_timeline_destroy(timeline: c_int);
    fn sw_sync_fence_create(timeline: c_int, name: *const c_char, value: c_int) -> c_int;
    fn sw_sync_fence_is_valid(fence: c_int) -> c_int;
    fn sw_sync_fence_destroy(fence: c_int);
}

#[no_mangle]
pub unsafe extern "C" fn test_alloc_timeline() -> c_int {
    let timeline: c_int;
    let valid: c_int;

    timeline = sw_sync_timeline_create();
    valid = sw_sync_timeline_is_valid(timeline);
    ASSERT!(valid != 0, "Failure allocating timeline\n");

    sw_sync_timeline_destroy(timeline);
    0
}

#[no_mangle]
pub unsafe extern "C" fn test_alloc_fence() -> c_int {
    let timeline: c_int;
    let fence: c_int;
    let valid: c_int;

    timeline = sw_sync_timeline_create();
    valid = sw_sync_timeline_is_valid(timeline);
    ASSERT!(valid != 0, "Failure allocating timeline\n");

    fence = sw_sync_fence_create(timeline, b"allocFence\0".as_ptr() as *const c_char, 1);
    valid = sw_sync_fence_is_valid(fence);
    ASSERT!(valid != 0, "Failure allocating fence\n");

    sw_sync_fence_destroy(fence);
    sw_sync_timeline_destroy(timeline);
    0
}

#[no_mangle]
pub unsafe extern "C" fn test_alloc_fence_negative() -> c_int {
    let fence: c_int;
    let timeline: c_int;

    timeline = sw_sync_timeline_create();
    ASSERT!(timeline > 0, "Failure allocating timeline\n");

    fence = sw_sync_fence_create(-1, b"fence\0".as_ptr() as *const c_char, 1);
    ASSERT!(fence < 0, "Success allocating negative fence\n");

    sw_sync_fence_destroy(fence);
    sw_sync_timeline_destroy(timeline);
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
