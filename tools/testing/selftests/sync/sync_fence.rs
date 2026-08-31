/*
 *  sync fence tests with one timeline
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

/* Dependencies translated from:
 * #include "sync.h"
 * #include "sw_sync.h"
 * #include "synctest.h"
 */

use std::os::raw::{c_char, c_int};

unsafe extern "C" {
    fn sw_sync_timeline_create() -> c_int;
    fn sw_sync_timeline_is_valid(timeline: c_int) -> c_int;
    fn sw_sync_timeline_inc(timeline: c_int, count: c_int) -> c_int;
    fn sw_sync_timeline_destroy(timeline: c_int);

    fn sw_sync_fence_create(timeline: c_int, name: *const c_char, value: c_int) -> c_int;
    fn sw_sync_fence_is_valid(fence: c_int) -> c_int;
    fn sw_sync_fence_destroy(fence: c_int);

    fn sync_wait(fd: c_int, timeout: c_int) -> c_int;
    fn sync_merge(name: *const c_char, fd1: c_int, fd2: c_int) -> c_int;
    fn sync_fence_count_with_status(fence: c_int, status: c_int) -> c_int;
}

extern "Rust" {
    fn ASSERT(condition: bool, message: *const c_char);
}

extern "Rust" {
    static FENCE_STATUS_ACTIVE: c_int;
    static FENCE_STATUS_SIGNALED: c_int;
}

#[no_mangle]
pub unsafe extern "C" fn test_fence_one_timeline_wait() -> c_int {
    let mut fence: c_int;
    let mut valid: c_int;
    let mut ret: c_int;
    let timeline: c_int = sw_sync_timeline_create();

    valid = sw_sync_timeline_is_valid(timeline);
    ASSERT(valid != 0, c"Failure allocating timeline\n".as_ptr());

    fence = sw_sync_fence_create(timeline, c"allocFence".as_ptr(), 5);
    valid = sw_sync_fence_is_valid(fence);
    ASSERT(valid != 0, c"Failure allocating fence\n".as_ptr());

    /* Wait on fence until timeout */
    ret = sync_wait(fence, 0);
    ASSERT(ret == 0, c"Failure waiting on fence until timeout\n".as_ptr());

    /* Advance timeline from 0 -> 1 */
    ret = sw_sync_timeline_inc(timeline, 1);
    ASSERT(ret == 0, c"Failure advancing timeline\n".as_ptr());

    /* Wait on fence until timeout */
    ret = sync_wait(fence, 0);
    ASSERT(ret == 0, c"Failure waiting on fence until timeout\n".as_ptr());

    /* Signal the fence */
    ret = sw_sync_timeline_inc(timeline, 4);
    ASSERT(ret == 0, c"Failure signaling the fence\n".as_ptr());

    /* Wait successfully */
    ret = sync_wait(fence, 0);
    ASSERT(ret > 0, c"Failure waiting on fence\n".as_ptr());

    /* Go even further, and confirm wait still succeeds */
    ret = sw_sync_timeline_inc(timeline, 10);
    ASSERT(ret == 0, c"Failure going further\n".as_ptr());
    ret = sync_wait(fence, 0);
    ASSERT(ret > 0, c"Failure waiting ahead\n".as_ptr());

    sw_sync_fence_destroy(fence);
    sw_sync_timeline_destroy(timeline);

    0
}

#[no_mangle]
pub unsafe extern "C" fn test_fence_one_timeline_merge() -> c_int {
    let mut a: c_int;
    let mut b: c_int;
    let mut c: c_int;
    let mut d: c_int;
    let mut valid: c_int;
    let timeline: c_int = sw_sync_timeline_create();

    /* create fence a,b,c and then merge them all into fence d */
    a = sw_sync_fence_create(timeline, c"allocFence".as_ptr(), 1);
    b = sw_sync_fence_create(timeline, c"allocFence".as_ptr(), 2);
    c = sw_sync_fence_create(timeline, c"allocFence".as_ptr(), 3);

    valid = ((sw_sync_fence_is_valid(a) != 0)
        && (sw_sync_fence_is_valid(b) != 0)
        && (sw_sync_fence_is_valid(c) != 0)) as c_int;
    ASSERT(valid != 0, c"Failure allocating fences\n".as_ptr());

    d = sync_merge(c"mergeFence".as_ptr(), b, a);
    d = sync_merge(c"mergeFence".as_ptr(), c, d);
    valid = sw_sync_fence_is_valid(d);
    ASSERT(valid != 0, c"Failure merging fences\n".as_ptr());

    /* confirm all fences have one active point (even d) */
    ASSERT(
        sync_fence_count_with_status(a, FENCE_STATUS_ACTIVE) == 1,
        c"a has too many active fences!\n".as_ptr(),
    );
    ASSERT(
        sync_fence_count_with_status(a, FENCE_STATUS_ACTIVE) == 1,
        c"b has too many active fences!\n".as_ptr(),
    );
    ASSERT(
        sync_fence_count_with_status(a, FENCE_STATUS_ACTIVE) == 1,
        c"c has too many active fences!\n".as_ptr(),
    );
    ASSERT(
        sync_fence_count_with_status(a, FENCE_STATUS_ACTIVE) == 1,
        c"d has too many active fences!\n".as_ptr(),
    );

    /* confirm that d is not signaled until the max of a,b,c */
    sw_sync_timeline_inc(timeline, 1);
    ASSERT(
        sync_fence_count_with_status(a, FENCE_STATUS_SIGNALED) == 1,
        c"a did not signal!\n".as_ptr(),
    );
    ASSERT(
        sync_fence_count_with_status(d, FENCE_STATUS_ACTIVE) == 1,
        c"d signaled too early!\n".as_ptr(),
    );

    sw_sync_timeline_inc(timeline, 1);
    ASSERT(
        sync_fence_count_with_status(b, FENCE_STATUS_SIGNALED) == 1,
        c"b did not signal!\n".as_ptr(),
    );
    ASSERT(
        sync_fence_count_with_status(d, FENCE_STATUS_ACTIVE) == 1,
        c"d signaled too early!\n".as_ptr(),
    );

    sw_sync_timeline_inc(timeline, 1);
    ASSERT(
        sync_fence_count_with_status(c, FENCE_STATUS_SIGNALED) == 1,
        c"c did not signal!\n".as_ptr(),
    );
    ASSERT(
        sync_fence_count_with_status(d, FENCE_STATUS_ACTIVE) == 0
            && sync_fence_count_with_status(d, FENCE_STATUS_SIGNALED) == 1,
        c"d did not signal!\n".as_ptr(),
    );

    sw_sync_fence_destroy(d);
    sw_sync_fence_destroy(c);
    sw_sync_fence_destroy(b);
    sw_sync_fence_destroy(a);
    sw_sync_timeline_destroy(timeline);
    0
}
