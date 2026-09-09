/*
 * Copyright 2016 Advanced Micro Devices, Inc.
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
 * Author: Monk.liu@amd.com
 */

pub const AMDGPU_CSA_SIZE: u32 = 128 * 1024;

extern "C" {
    pub fn amdgpu_get_total_csa_size(adev: *mut amdgpu_device) -> u32;
    pub fn amdgpu_csa_vaddr(adev: *mut amdgpu_device) -> u64;
    pub fn amdgpu_allocate_static_csa(
        adev: *mut amdgpu_device,
        bo: *mut *mut amdgpu_bo,
        domain: u32,
        size: u32,
    ) -> i32;
    pub fn amdgpu_map_static_csa(
        adev: *mut amdgpu_device,
        vm: *mut amdgpu_vm,
        bo: *mut amdgpu_bo,
        bo_va: *mut *mut amdgpu_bo_va,
        csa_addr: u64,
        size: u32,
    ) -> i32;
    pub fn amdgpu_unmap_static_csa(
        adev: *mut amdgpu_device,
        vm: *mut amdgpu_vm,
        bo: *mut amdgpu_bo,
        bo_va: *mut amdgpu_bo_va,
        csa_addr: u64,
    ) -> i32;
    pub fn amdgpu_free_static_csa(bo: *mut *mut amdgpu_bo);
}

// External C types supplied by other translation units.
#[repr(C)]
pub struct amdgpu_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amdgpu_bo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amdgpu_vm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amdgpu_bo_va {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
