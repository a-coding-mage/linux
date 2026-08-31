/*
 *  sync abstraction
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

pub const FENCE_STATUS_ERROR: i32 = -1;
pub const FENCE_STATUS_ACTIVE: i32 = 0;
pub const FENCE_STATUS_SIGNALED: i32 = 1;

unsafe extern "C" {
    pub fn sync_wait(fd: i32, timeout: i32) -> i32;
    pub fn sync_merge(name: *const core::ffi::c_char, fd1: i32, fd2: i32) -> i32;
    pub fn sync_fence_size(fd: i32) -> i32;
    pub fn sync_fence_count_with_status(fd: i32, status: i32) -> i32;
}
