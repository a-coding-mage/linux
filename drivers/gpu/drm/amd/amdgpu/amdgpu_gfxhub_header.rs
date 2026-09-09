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

// Dependency supplied by the surrounding translation unit.
pub enum amdgpu_device {}

#[repr(C)]
pub struct amdgpu_gfxhub_funcs {
    pub get_fb_location: Option<unsafe extern "C" fn(adev: *mut amdgpu_device) -> u64>,
    pub get_mc_fb_offset: Option<unsafe extern "C" fn(adev: *mut amdgpu_device) -> u64>,
    pub setup_vm_pt_regs: Option<
        unsafe extern "C" fn(
            adev: *mut amdgpu_device,
            vmid: u32,
            page_table_base: u64,
        ),
    >,
    pub gart_enable: Option<unsafe extern "C" fn(adev: *mut amdgpu_device) -> i32>,

    pub gart_disable: Option<unsafe extern "C" fn(adev: *mut amdgpu_device)>,
    pub set_fault_enable_default:
        Option<unsafe extern "C" fn(adev: *mut amdgpu_device, value: bool)>,
    pub init: Option<unsafe extern "C" fn(adev: *mut amdgpu_device)>,
    pub get_xgmi_info: Option<unsafe extern "C" fn(adev: *mut amdgpu_device) -> i32>,
    pub utcl2_harvest: Option<unsafe extern "C" fn(adev: *mut amdgpu_device)>,
    pub mode2_save_regs: Option<unsafe extern "C" fn(adev: *mut amdgpu_device)>,
    pub mode2_restore_regs: Option<unsafe extern "C" fn(adev: *mut amdgpu_device)>,
    pub halt: Option<unsafe extern "C" fn(adev: *mut amdgpu_device)>,
}

#[repr(C)]
pub struct amdgpu_gfxhub {
    pub funcs: *const amdgpu_gfxhub_funcs,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
