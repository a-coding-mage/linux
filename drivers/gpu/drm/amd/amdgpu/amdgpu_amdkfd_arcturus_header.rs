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
 */

// Types are supplied by the surrounding kernel/amdgpu translation unit.
pub struct amdgpu_device;
pub struct mm_struct;

pub unsafe extern "C" fn kgd_arcturus_hqd_sdma_load(
    adev: *mut amdgpu_device,
    mqd: *mut core::ffi::c_void,
    wptr: *mut u32,
    mm: *mut mm_struct,
) -> i32;

pub unsafe extern "C" fn kgd_arcturus_hqd_sdma_dump(
    adev: *mut amdgpu_device,
    engine_id: u32,
    queue_id: u32,
    dump: *mut *mut [u32; 2],
    n_regs: *mut u32,
) -> i32;

pub unsafe extern "C" fn kgd_arcturus_hqd_sdma_is_occupied(
    adev: *mut amdgpu_device,
    mqd: *mut core::ffi::c_void,
) -> bool;

pub unsafe extern "C" fn kgd_arcturus_hqd_sdma_destroy(
    adev: *mut amdgpu_device,
    mqd: *mut core::ffi::c_void,
    utimeout: core::ffi::c_uint,
) -> i32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
