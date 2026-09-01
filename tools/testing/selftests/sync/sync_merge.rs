/*
 *  sync fence merge tests
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

// Dependencies from "sync.h", "sw_sync.h", and "synctest.h".

unsafe extern "C" {
    fn sw_sync_timeline_create() -> ::std::os::raw::c_int;
    fn sw_sync_timeline_is_valid(timeline: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    fn sw_sync_fence_create(
        timeline: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn sw_sync_fence_is_valid(fence: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    fn sync_merge(
        name: *const ::std::os::raw::c_char,
        fd1: ::std::os::raw::c_int,
        fd2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn sync_fence_count_with_status(
        fd: ::std::os::raw::c_int,
        status: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn sw_sync_timeline_inc(
        timeline: ::std::os::raw::c_int,
        count: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    fn sw_sync_fence_destroy(fence: ::std::os::raw::c_int);
    fn sw_sync_timeline_destroy(timeline: ::std::os::raw::c_int);
    fn ASSERT(condition: ::std::os::raw::c_int, message: *const ::std::os::raw::c_char);
}

unsafe extern "C" {
    static FENCE_STATUS_SIGNALED: ::std::os::raw::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn test_fence_merge_same_fence() -> ::std::os::raw::c_int {
    let fence: ::std::os::raw::c_int;
    let mut valid: ::std::os::raw::c_int;
    let merged: ::std::os::raw::c_int;
    let timeline: ::std::os::raw::c_int = unsafe { sw_sync_timeline_create() };

    valid = unsafe { sw_sync_timeline_is_valid(timeline) };
    unsafe { ASSERT(valid, c"Failure allocating timeline\n".as_ptr()) };

    fence = unsafe { sw_sync_fence_create(timeline, c"allocFence".as_ptr(), 5) };
    valid = unsafe { sw_sync_fence_is_valid(fence) };
    unsafe { ASSERT(valid, c"Failure allocating fence\n".as_ptr()) };

    merged = unsafe { sync_merge(c"mergeFence".as_ptr(), fence, fence) };
    valid = unsafe { sw_sync_fence_is_valid(fence) };
    unsafe { ASSERT(valid, c"Failure merging fence\n".as_ptr()) };

    unsafe {
        ASSERT(
            (sync_fence_count_with_status(merged, FENCE_STATUS_SIGNALED) == 0) as ::std::os::raw::c_int,
            c"fence signaled too early!\n".as_ptr(),
        )
    };

    unsafe { sw_sync_timeline_inc(timeline, 5) };
    unsafe {
        ASSERT(
            (sync_fence_count_with_status(merged, FENCE_STATUS_SIGNALED) == 1) as ::std::os::raw::c_int,
            c"fence did not signal!\n".as_ptr(),
        )
    };

    unsafe { sw_sync_fence_destroy(merged) };
    unsafe { sw_sync_fence_destroy(fence) };
    unsafe { sw_sync_timeline_destroy(timeline) };

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
