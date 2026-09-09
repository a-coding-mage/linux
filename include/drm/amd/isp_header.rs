/* SPDX-License-Identifier: MIT */
/*
 * Copyright (C) 2025 Advanced Micro Devices, Inc. All rights reserved.
 * All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sub license, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to the
 * following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
 * OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
 * USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 * The above copyright notice and this permission notice (including the
 * next paragraph) shall be included in all copies or substantial portions
 * of the Software.
 *
 */

// Dependency supplied by the surrounding kernel translation.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct isp_platform_data {
    pub adev: *mut core::ffi::c_void,
    pub asic_type: u32,
    pub base_rmmio_size: resource_size_t,
}

extern "C" {
    pub fn isp_user_buffer_alloc(
        dev: *mut device,
        dmabuf: *mut core::ffi::c_void,
        buf_obj: *mut *mut core::ffi::c_void,
        buf_addr: *mut u64,
    ) -> i32;

    pub fn isp_user_buffer_free(buf_obj: *mut core::ffi::c_void);

    pub fn isp_kernel_buffer_alloc(
        dev: *mut device,
        size: u64,
        buf_obj: *mut *mut core::ffi::c_void,
        gpu_addr: *mut u64,
        cpu_addr: *mut *mut core::ffi::c_void,
    ) -> i32;

    pub fn isp_kernel_buffer_free(
        buf_obj: *mut *mut core::ffi::c_void,
        gpu_addr: *mut u64,
        cpu_addr: *mut *mut core::ffi::c_void,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
