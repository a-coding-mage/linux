/*
 *  sw_sync abstraction
 *
 *  Copyright 2015-2016 Collabora Ltd.
 *
 *  Based on the implementation from the Android Open Source Project,
 *
 *  Copyright 2013 Google, Inc
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

/*
 * sw_sync is mainly intended for testing and should not be compiled into
 * production kernels
 */

use core::ffi::{c_char, c_int, c_uint};

unsafe extern "C" {
    pub fn sw_sync_timeline_create() -> c_int;
    pub fn sw_sync_timeline_is_valid(fd: c_int) -> c_int;
    pub fn sw_sync_timeline_inc(fd: c_int, count: c_uint) -> c_int;
    pub fn sw_sync_timeline_destroy(fd: c_int);

    pub fn sw_sync_fence_create(fd: c_int, name: *const c_char, value: c_uint) -> c_int;
    pub fn sw_sync_fence_is_valid(fd: c_int) -> c_int;
    pub fn sw_sync_fence_destroy(fd: c_int);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
