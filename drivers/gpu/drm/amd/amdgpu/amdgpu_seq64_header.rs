/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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

// Dependency equivalent of: #include "amdgpu_vm.h"

pub const AMDGPU_MAX_SEQ64_SLOTS: usize =
    AMDGPU_VA_RESERVED_SEQ64_SIZE / core::mem::size_of::<u64>();

#[repr(C)]
pub struct amdgpu_seq64 {
    pub sbo: *mut amdgpu_bo,
    pub num_sem: u32,
    pub gpu_addr: u64,
    pub cpu_base_addr: *mut u64,
    // Equivalent of DECLARE_BITMAP(used, AMDGPU_MAX_SEQ64_SLOTS).
    pub used: [u64; (AMDGPU_MAX_SEQ64_SLOTS + 63) / 64],
}

extern "C" {
    pub fn amdgpu_seq64_fini(adev: *mut amdgpu_device);
    pub fn amdgpu_seq64_init(adev: *mut amdgpu_device) -> core::ffi::c_int;
    pub fn amdgpu_seq64_alloc(
        adev: *mut amdgpu_device,
        va: *mut u64,
        gpu_addr: *mut u64,
        cpu_addr: *mut *mut u64,
    ) -> core::ffi::c_int;
    pub fn amdgpu_seq64_free(adev: *mut amdgpu_device, gpu_addr: u64);
    pub fn amdgpu_seq64_map(
        adev: *mut amdgpu_device,
        vm: *mut amdgpu_vm,
        bo_va: *mut *mut amdgpu_bo_va,
    ) -> core::ffi::c_int;
    pub fn amdgpu_seq64_unmap(adev: *mut amdgpu_device, fpriv: *mut amdgpu_fpriv);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
