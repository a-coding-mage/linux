/*
 * Copyright (C) 2019  Advanced Micro Devices, Inc.
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
 * AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

use core::ffi::c_char;

// External types supplied by other translation units.
pub struct amdgpu_ras_block_object;
pub struct amdgpu_device;
pub enum amd_clockgating_state {}
pub struct ras_common_if;

#[repr(C)]
pub struct amdgpu_mmhub_ras {
    pub ras_block: amdgpu_ras_block_object,
}

#[repr(C)]
pub struct amdgpu_mmhub_funcs {
    pub get_fb_location: Option<unsafe extern "C" fn(adev: *mut amdgpu_device) -> u64>,
    pub get_mc_fb_offset: Option<unsafe extern "C" fn(adev: *mut amdgpu_device) -> u64>,
    pub init: Option<unsafe extern "C" fn(adev: *mut amdgpu_device)>,
    pub gart_enable: Option<unsafe extern "C" fn(adev: *mut amdgpu_device) -> i32>,
    pub set_fault_enable_default:
        Option<unsafe extern "C" fn(adev: *mut amdgpu_device, value: bool)>,
    pub gart_disable: Option<unsafe extern "C" fn(adev: *mut amdgpu_device)>,
    pub set_clockgating: Option<
        unsafe extern "C" fn(
            adev: *mut amdgpu_device,
            state: amd_clockgating_state,
        ) -> i32,
    >,
    pub get_clockgating:
        Option<unsafe extern "C" fn(adev: *mut amdgpu_device, flags: *mut u64)>,
    pub setup_vm_pt_regs: Option<
        unsafe extern "C" fn(
            adev: *mut amdgpu_device,
            vmid: u32,
            page_table_base: u64,
        ),
    >,
    pub update_power_gating:
        Option<unsafe extern "C" fn(adev: *mut amdgpu_device, enable: bool)>,
    pub get_xgmi_info: Option<unsafe extern "C" fn(adev: *mut amdgpu_device) -> i32>,
}

#[repr(C)]
pub struct amdgpu_mmhub_client_ids {
    pub names: *const [*const c_char; 2],
    pub size: u32,
}

#[repr(C)]
pub struct amdgpu_mmhub {
    pub ras_if: *mut ras_common_if,
    pub funcs: *const amdgpu_mmhub_funcs,
    pub ras: *mut amdgpu_mmhub_ras,
    pub client_ids: amdgpu_mmhub_client_ids,
}

#[inline]
pub unsafe fn amdgpu_mmhub_init_client_info(
    mmhub: *mut amdgpu_mmhub,
    names: *const [*const c_char; 2],
    size: u32,
) {
    (*mmhub).client_ids.names = names;
    (*mmhub).client_ids.size = size;
}

#[inline]
pub unsafe fn amdgpu_mmhub_client_name(
    mmhub: *mut amdgpu_mmhub,
    cid: u32,
    is_write: bool,
) -> *const c_char {
    if cid < (*mmhub).client_ids.size {
        return (*(*mmhub).client_ids.names.add(cid as usize))[is_write as usize];
    }

    core::ptr::null()
}

extern "C" {
    pub fn amdgpu_mmhub_ras_sw_init(adev: *mut amdgpu_device) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
