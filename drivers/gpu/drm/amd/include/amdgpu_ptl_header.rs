/*
 * Copyright 2026 Advanced Micro Devices, Inc.
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
 */

#[repr(C)]
pub enum amdgpu_ptl_fmt {
    AMDGPU_PTL_FMT_I8 = 0,
    AMDGPU_PTL_FMT_F16 = 1,
    AMDGPU_PTL_FMT_BF16 = 2,
    AMDGPU_PTL_FMT_F32 = 3,
    AMDGPU_PTL_FMT_F64 = 4,
    AMDGPU_PTL_FMT_F8 = 5,
    AMDGPU_PTL_FMT_VECTOR = 6,
    AMDGPU_PTL_FMT_INVALID = 7,
}

#[repr(C)]
pub enum amdgpu_ptl_disable_source {
    AMDGPU_PTL_DISABLE_SYSFS = 0,
    AMDGPU_PTL_DISABLE_PROFILER,
    AMDGPU_PTL_DISABLE_MAX,
}

#[repr(C)]
pub enum amdgpu_ptl_hw_supported_state {
    AMDGPU_PTL_HW_UNINIT = 0,       /* Not yet initialized */
    AMDGPU_PTL_HW_SUPPORTED,        /* Initialized and supported */
    AMDGPU_PTL_HW_NOT_SUPPORTED,    /* Initialized and not supported */
}

#[repr(C)]
pub struct amdgpu_ptl {
    pub fmt1: amdgpu_ptl_fmt,
    pub fmt2: amdgpu_ptl_fmt,
    pub enabled: bool,
    pub hw_supported_state: amdgpu_ptl_hw_supported_state,
    pub permanently_disabled: bool,
    /* PTL disable reference counting */
    pub disable_ref: atomic_t,
    pub mutex: mutex,
    pub disable_bitmap: [core::ffi::c_ulong; 1],
    pub ptl_sysfs_created: bool,
}

extern "C" {
    pub fn amdgpu_ptl_perf_monitor_ctrl(
        adev: *mut amdgpu_device,
        req_code: u32,
        ptl_state: *mut u32,
        fmt1: *mut amdgpu_ptl_fmt,
        fmt2: *mut amdgpu_ptl_fmt,
    ) -> i32;

    pub fn amdgpu_ptl_sysfs_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_ptl_sysfs_fini(adev: *mut amdgpu_device);

    pub static amdgpu_ptl_attr_group: attribute_group;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
