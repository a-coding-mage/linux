/*
 *  sync stress test: merging
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

// C dependencies: stdlib.h, string.h, time.h, sync.h, sw_sync.h, synctest.h.

use std::os::raw::{c_char, c_int, c_long};
use std::ptr;

extern "C" {
    fn srand(seed: c_int);
    fn rand() -> c_int;
    fn time(tloc: *mut c_long) -> c_long;

    fn sw_sync_timeline_create() -> c_int;
    fn sw_sync_timeline_destroy(timeline: c_int);
    fn sw_sync_timeline_inc(timeline: c_int, count: c_int);
    fn sw_sync_fence_create(timeline: c_int, name: *const c_char, value: c_int) -> c_int;
    fn sw_sync_fence_destroy(fence: c_int);
    fn sw_sync_fence_is_valid(fence: c_int) -> c_int;
    fn sync_merge(name: *const c_char, fd1: c_int, fd2: c_int) -> c_int;
    fn sync_fence_size(fence: c_int) -> c_int;
    fn sync_wait(fence: c_int, timeout: c_int) -> c_int;

    fn ASSERT(condition: c_int, message: *const c_char);
}

#[no_mangle]
pub unsafe extern "C" fn test_merge_stress_random_merge() -> c_int {
    let mut i: c_int;
    let mut size: c_int;
    let mut ret: c_int;
    let timeline_count: c_int = 32;
    let merge_count: c_int = 1024 * 32;
    let mut timelines: [c_int; 32] = [0; 32];
    let mut fence_map: [c_int; 32] = [0; 32];
    let mut fence: c_int;
    let mut tmpfence: c_int;
    let mut merged: c_int;
    let mut valid: c_int;
    let mut timeline: c_int;
    let mut timeline_offset: c_int;
    let mut sync_point: c_int;

    srand(time(ptr::null_mut()) as c_int);

    i = 0;
    while i < timeline_count {
        timelines[i as usize] = sw_sync_timeline_create();
        i += 1;
    }

    fence = sw_sync_fence_create(timelines[0], b"fence\0".as_ptr() as *const c_char, 0);
    valid = sw_sync_fence_is_valid(fence);
    ASSERT((valid != 0) as c_int, b"Failure creating fence\n\0".as_ptr() as *const c_char);

    fence_map.fill(-1);
    fence_map[0] = 0;

    /*
     * Randomly create sync_points out of a fixed set of timelines,
     * and merge them together
     */
    i = 0;
    while i < merge_count {
        /* Generate sync_point. */
        timeline_offset = rand() % timeline_count;
        timeline = timelines[timeline_offset as usize];
        sync_point = rand();

        /* Keep track of the latest sync_point in each timeline. */
        if fence_map[timeline_offset as usize] == -1 {
            fence_map[timeline_offset as usize] = sync_point;
        } else if fence_map[timeline_offset as usize] < sync_point {
            fence_map[timeline_offset as usize] = sync_point;
        }

        /* Merge */
        tmpfence = sw_sync_fence_create(
            timeline,
            b"fence\0".as_ptr() as *const c_char,
            sync_point,
        );
        merged = sync_merge(
            b"merge\0".as_ptr() as *const c_char,
            tmpfence,
            fence,
        );
        sw_sync_fence_destroy(tmpfence);
        sw_sync_fence_destroy(fence);
        fence = merged;

        valid = sw_sync_fence_is_valid(merged);
        ASSERT((valid != 0) as c_int, b"Failure creating fence i\n\0".as_ptr() as *const c_char);

        i += 1;
    }

    size = 0;
    i = 0;
    while i < timeline_count {
        if fence_map[i as usize] != -1 {
            size += 1;
        }
        i += 1;
    }

    /* Confirm our map matches the fence. */
    ASSERT(
        (sync_fence_size(fence) == size) as c_int,
        b"Quantity of elements not matching\n\0".as_ptr() as *const c_char,
    );

    /* Trigger the merged fence */
    i = 0;
    while i < timeline_count {
        if fence_map[i as usize] != -1 {
            ret = sync_wait(fence, 0);
            ASSERT(
                (ret == 0) as c_int,
                b"Failure waiting on fence until timeout\n\0".as_ptr() as *const c_char,
            );
            /* Increment the timeline to the last sync_point */
            sw_sync_timeline_inc(timelines[i as usize], fence_map[i as usize]);
        }
        i += 1;
    }

    /* Check that the fence is triggered. */
    ret = sync_wait(fence, 0);
    ASSERT((ret > 0) as c_int, b"Failure triggering fence\n\0".as_ptr() as *const c_char);

    sw_sync_fence_destroy(fence);

    i = 0;
    while i < timeline_count {
        sw_sync_timeline_destroy(timelines[i as usize]);
        i += 1;
    }

    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
