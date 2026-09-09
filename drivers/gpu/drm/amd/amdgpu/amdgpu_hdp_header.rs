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

// Dependency supplied by the surrounding translation unit: amdgpu_ras.h

#[repr(C)]
pub struct amdgpu_hdp_ras {
    pub ras_block: amdgpu_ras_block_object,
}

#[repr(C)]
pub struct amdgpu_hdp_funcs {
    pub flush_hdp:
        Option<unsafe extern "C" fn(adev: *mut amdgpu_device, ring: *mut amdgpu_ring)>,
    pub invalidate_hdp:
        Option<unsafe extern "C" fn(adev: *mut amdgpu_device, ring: *mut amdgpu_ring)>,
    pub update_clock_gating:
        Option<unsafe extern "C" fn(adev: *mut amdgpu_device, enable: bool)>,
    pub get_clock_gating_state:
        Option<unsafe extern "C" fn(adev: *mut amdgpu_device, flags: *mut u64)>,
    pub init_registers: Option<unsafe extern "C" fn(adev: *mut amdgpu_device)>,
}

#[repr(C)]
pub struct amdgpu_hdp {
    pub ras_if: *mut ras_common_if,
    pub funcs: *const amdgpu_hdp_funcs,
    pub ras: *mut amdgpu_hdp_ras,
}

extern "C" {
    pub fn amdgpu_hdp_ras_sw_init(adev: *mut amdgpu_device) -> ::std::os::raw::c_int;
    pub fn amdgpu_hdp_generic_flush(
        adev: *mut amdgpu_device,
        ring: *mut amdgpu_ring,
    );
    pub fn amdgpu_hdp_invalidate(
        adev: *mut amdgpu_device,
        ring: *mut amdgpu_ring,
    );
    pub fn amdgpu_hdp_flush(
        adev: *mut amdgpu_device,
        ring: *mut amdgpu_ring,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
