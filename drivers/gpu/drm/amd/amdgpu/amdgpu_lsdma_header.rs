/*
 * Copyright 2022 Advanced Micro Devices, Inc.
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

// External dependency supplied by the surrounding translation unit.
#[repr(C)]
pub struct amdgpu_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amdgpu_lsdma {
    pub funcs: *const amdgpu_lsdma_funcs,
}

#[repr(C)]
pub struct amdgpu_lsdma_funcs {
    pub copy_mem: Option<unsafe extern "C" fn(
        adev: *mut amdgpu_device,
        src_addr: u64,
        dst_addr: u64,
        size: u64,
    ) -> i32>,
    pub fill_mem: Option<unsafe extern "C" fn(
        adev: *mut amdgpu_device,
        dst_addr: u64,
        data: u32,
        size: u64,
    ) -> i32>,
    pub update_memory_power_gating:
        Option<unsafe extern "C" fn(adev: *mut amdgpu_device, enable: bool)>,
}

unsafe extern "C" {
    pub fn amdgpu_lsdma_copy_mem(
        adev: *mut amdgpu_device,
        src_addr: u64,
        dst_addr: u64,
        mem_size: u64,
    ) -> i32;
    pub fn amdgpu_lsdma_fill_mem(
        adev: *mut amdgpu_device,
        dst_addr: u64,
        data: u32,
        mem_size: u64,
    ) -> i32;
    pub fn amdgpu_lsdma_wait_for(
        adev: *mut amdgpu_device,
        reg_index: u32,
        reg_val: u32,
        mask: u32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
