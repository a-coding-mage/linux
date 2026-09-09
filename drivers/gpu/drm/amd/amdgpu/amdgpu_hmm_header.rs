/*
 * Copyright 2017 Advanced Micro Devices, Inc.
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
 * Authors: Christian König
 */

use core::ffi::c_void;

// Supplied by the Linux HMM, MMU notifier, and amdgpu dependencies.
#[repr(C)]
pub struct hmm_range {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mmu_interval_notifier {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amdgpu_bo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amdgpu_hmm_range {
    pub hmm_range: hmm_range,
    pub bo: *mut amdgpu_bo,
}

extern "C" {
    pub fn amdgpu_hmm_range_get_pages(
        notifier: *mut mmu_interval_notifier,
        start: u64,
        npages: u64,
        readonly: bool,
        owner: *mut c_void,
        range: *mut amdgpu_hmm_range,
    ) -> i32;
}

// CONFIG_HMM_MIRROR controls whether these external implementations exist.
#[cfg(feature = "CONFIG_HMM_MIRROR")]
extern "C" {
    pub fn amdgpu_hmm_range_valid(range: *mut amdgpu_hmm_range) -> bool;
    pub fn amdgpu_hmm_range_alloc(bo: *mut amdgpu_bo) -> *mut amdgpu_hmm_range;
    pub fn amdgpu_hmm_range_free(range: *mut amdgpu_hmm_range);
    pub fn amdgpu_hmm_register(bo: *mut amdgpu_bo, addr: usize) -> i32;
    pub fn amdgpu_hmm_unregister(bo: *mut amdgpu_bo);
}

#[cfg(not(feature = "CONFIG_HMM_MIRROR"))]
pub unsafe fn amdgpu_hmm_register(_bo: *mut amdgpu_bo, _addr: usize) -> i32 {
    // DRM_WARN_ONCE("HMM_MIRROR kernel config option is not enabled, "
    //               "add CONFIG_ZONE_DEVICE=y in config file to fix this\\n");
    -19 // -ENODEV
}

#[cfg(not(feature = "CONFIG_HMM_MIRROR"))]
pub unsafe fn amdgpu_hmm_unregister(_bo: *mut amdgpu_bo) {}

#[cfg(not(feature = "CONFIG_HMM_MIRROR"))]
pub unsafe fn amdgpu_hmm_range_valid(_range: *mut amdgpu_hmm_range) -> bool {
    false
}

#[cfg(not(feature = "CONFIG_HMM_MIRROR"))]
pub unsafe fn amdgpu_hmm_range_alloc(_bo: *mut amdgpu_bo) -> *mut amdgpu_hmm_range {
    core::ptr::null_mut()
}

#[cfg(not(feature = "CONFIG_HMM_MIRROR"))]
pub unsafe fn amdgpu_hmm_range_free(_range: *mut amdgpu_hmm_range) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
