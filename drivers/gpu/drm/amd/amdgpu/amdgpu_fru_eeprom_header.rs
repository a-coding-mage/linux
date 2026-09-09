/*
 * Copyright 2020 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 */

use core::ffi::c_char;

pub const AMDGPU_PRODUCT_NAME_LEN: usize = 64;

/* FRU product information */
#[repr(C)]
pub struct amdgpu_fru_info {
    pub product_number: [c_char; 20],
    pub product_name: [c_char; AMDGPU_PRODUCT_NAME_LEN],
    pub serial: [c_char; 20],
    pub manufacturer_name: [c_char; 32],
    pub fru_id: [c_char; 50],
}

/* External declaration supplied by the surrounding codebase. */
#[repr(C)]
pub struct amdgpu_device {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn amdgpu_fru_get_product_info(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_fru_sysfs_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_fru_sysfs_fini(adev: *mut amdgpu_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
