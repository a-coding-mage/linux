/*
 *  sync fence wait tests
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

/* Dependencies from "sync.h", "sw_sync.h", and "synctest.h". */
use std::os::raw::{c_char, c_int};

extern "C" {
    static FENCE_STATUS_ACTIVE: c_int;
    static FENCE_STATUS_SIGNALED: c_int;

    fn sw_sync_timeline_create() -> c_int;
    fn sw_sync_timeline_destroy(timeline: c_int);
    fn sw_sync_timeline_inc(timeline: c_int, count: c_int) -> c_int;
    fn sw_sync_fence_create(timeline: c_int, name: *const c_char, value: c_int) -> c_int;
    fn sw_sync_fence_destroy(fence: c_int);
    fn sw_sync_fence_is_valid(fence: c_int) -> c_int;
    fn sync_merge(name: *const c_char, fd1: c_int, fd2: c_int) -> c_int;
    fn sync_fence_count_with_status(fence: c_int, status: c_int) -> c_int;
    fn sync_wait(fence: c_int, timeout: c_int) -> c_int;
    fn ASSERT(condition: bool, message: *const c_char);
}

#[no_mangle]
pub unsafe extern "C" fn test_fence_multi_timeline_wait() -> c_int {
    let timelineA: c_int;
    let timelineB: c_int;
    let timelineC: c_int;
    let fenceA: c_int;
    let fenceB: c_int;
    let fenceC: c_int;
    let mut merged: c_int;
    let mut valid: c_int;
    let mut active: c_int;
    let mut signaled: c_int;
    let mut ret: c_int;

    timelineA = sw_sync_timeline_create();
    timelineB = sw_sync_timeline_create();
    timelineC = sw_sync_timeline_create();

    fenceA = sw_sync_fence_create(timelineA, b"fenceA\0".as_ptr() as *const c_char, 5);
    fenceB = sw_sync_fence_create(timelineB, b"fenceB\0".as_ptr() as *const c_char, 5);
    fenceC = sw_sync_fence_create(timelineC, b"fenceC\0".as_ptr() as *const c_char, 5);

    merged = sync_merge(
        b"mergeFence\0".as_ptr() as *const c_char,
        fenceB,
        fenceA,
    );
    merged = sync_merge(
        b"mergeFence\0".as_ptr() as *const c_char,
        fenceC,
        merged,
    );

    valid = sw_sync_fence_is_valid(merged);
    ASSERT(
        valid != 0,
        b"Failure merging fence from various timelines\n\0".as_ptr() as *const c_char,
    );

    /* Confirm fence isn't signaled */
    active = sync_fence_count_with_status(merged, FENCE_STATUS_ACTIVE);
    ASSERT(
        active == 3,
        b"Fence signaled too early!\n\0".as_ptr() as *const c_char,
    );

    ret = sync_wait(merged, 0);
    ASSERT(
        ret == 0,
        b"Failure waiting on fence until timeout\n\0".as_ptr() as *const c_char,
    );

    ret = sw_sync_timeline_inc(timelineA, 5);
    active = sync_fence_count_with_status(merged, FENCE_STATUS_ACTIVE);
    signaled = sync_fence_count_with_status(merged, FENCE_STATUS_SIGNALED);
    ASSERT(
        active == 2 && signaled == 1,
        b"Fence did not signal properly!\n\0".as_ptr() as *const c_char,
    );

    ret = sw_sync_timeline_inc(timelineB, 5);
    active = sync_fence_count_with_status(merged, FENCE_STATUS_ACTIVE);
    signaled = sync_fence_count_with_status(merged, FENCE_STATUS_SIGNALED);
    ASSERT(
        active == 1 && signaled == 2,
        b"Fence did not signal properly!\n\0".as_ptr() as *const c_char,
    );

    ret = sw_sync_timeline_inc(timelineC, 5);
    active = sync_fence_count_with_status(merged, FENCE_STATUS_ACTIVE);
    signaled = sync_fence_count_with_status(merged, FENCE_STATUS_SIGNALED);
    ASSERT(
        active == 0 && signaled == 3,
        b"Fence did not signal properly!\n\0".as_ptr() as *const c_char,
    );

    /* confirm you can successfully wait */
    ret = sync_wait(merged, 100);
    ASSERT(
        ret > 0,
        b"Failure waiting on signaled fence\n\0".as_ptr() as *const c_char,
    );

    sw_sync_fence_destroy(merged);
    sw_sync_fence_destroy(fenceC);
    sw_sync_fence_destroy(fenceB);
    sw_sync_fence_destroy(fenceA);
    sw_sync_timeline_destroy(timelineC);
    sw_sync_timeline_destroy(timelineB);
    sw_sync_timeline_destroy(timelineA);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
