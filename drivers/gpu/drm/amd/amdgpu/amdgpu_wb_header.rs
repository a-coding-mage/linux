/* SPDX-License-Identifier: GPL-2.0 OR MIT
 *
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

// Linux header dependencies are supplied by other translated units.

/*
 * Writeback
 */
pub const AMDGPU_MAX_WB: usize = 1024; /* Reserve at most 1024 WB slots for amdgpu-owned rings. */

/// struct amdgpu_wb - This struct is used for small GPU memory allocation.
///
/// This struct is used to allocate a small amount of GPU memory that can be
/// used to shadow certain states into the memory. This is especially useful for
/// providing easy CPU access to some states without requiring register access
/// (e.g., if some block is power gated, reading register may be problematic).
///
/// Note: the term writeback was initially used because many of the amdgpu
/// components had some level of writeback memory, and this struct initially
/// described those components.
#[repr(C)]
pub struct amdgpu_wb {
    /// Buffer Object used for the writeback memory.
    pub wb_obj: *mut amdgpu_bo,

    /// Pointer to the first writeback slot. In terms of CPU address
    /// this value can be accessed directly by using the offset as an index.
    /// For the GPU address, it is necessary to use gpu_addr and the offset.
    pub wb: *mut u32,

    /// Writeback base address in the GPU.
    pub gpu_addr: u64,

    /// Number of writeback slots reserved for amdgpu.
    pub num_wb: u32,

    /// Track the writeback slot already used.
    pub used: [core::ffi::c_ulong; (AMDGPU_MAX_WB + (BITS_PER_LONG as usize) - 1) / (BITS_PER_LONG as usize)],

    /// Protects read and write of the used field array.
    pub lock: spinlock_t,
}

pub enum amdgpu_bo {}
pub enum amdgpu_device {}

extern "C" {
    pub fn amdgpu_wb_fini(adev: *mut amdgpu_device);
    pub fn amdgpu_wb_init(adev: *mut amdgpu_device) -> core::ffi::c_int;
    pub fn amdgpu_wb_get(adev: *mut amdgpu_device, wb: *mut u32) -> core::ffi::c_int;
    pub fn amdgpu_wb_free(adev: *mut amdgpu_device, wb: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
