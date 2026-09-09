/*
 * Copyright (C) 2021  Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
 * AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
 * IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependency declarations supplied by amdgpu_ras.h.
use core::ffi::{c_ulong, c_void};

#[repr(C)]
pub enum amdgpu_device {}

#[repr(C)]
pub enum ras_common_if {}

#[repr(C)]
pub struct amdgpu_ras_block_object {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_mca_error_type {
    AMDGPU_MCA_ERROR_TYPE_UE = 0,
    AMDGPU_MCA_ERROR_TYPE_CE,
}

#[repr(C)]
pub struct amdgpu_mca_ras_block {
    pub ras_block: amdgpu_ras_block_object,
}

#[repr(C)]
pub struct amdgpu_mca_ras {
    pub ras_if: *mut ras_common_if,
    pub ras: *mut amdgpu_mca_ras_block,
}

#[repr(C)]
pub struct amdgpu_mca {
    pub mp0: amdgpu_mca_ras,
    pub mp1: amdgpu_mca_ras,
    pub mpio: amdgpu_mca_ras,
}

unsafe extern "C" {
    pub fn amdgpu_mca_query_correctable_error_count(
        adev: *mut amdgpu_device,
        mc_status_addr: u64,
        error_count: *mut c_ulong,
    );
    pub fn amdgpu_mca_query_uncorrectable_error_count(
        adev: *mut amdgpu_device,
        mc_status_addr: u64,
        error_count: *mut c_ulong,
    );
    pub fn amdgpu_mca_reset_error_count(adev: *mut amdgpu_device, mc_status_addr: u64);
    pub fn amdgpu_mca_query_ras_error_count(
        adev: *mut amdgpu_device,
        mc_status_addr: u64,
        ras_error_status: *mut c_void,
    );
    pub fn amdgpu_mca_mp0_ras_sw_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_mca_mp1_ras_sw_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_mca_mpio_ras_sw_init(adev: *mut amdgpu_device) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
