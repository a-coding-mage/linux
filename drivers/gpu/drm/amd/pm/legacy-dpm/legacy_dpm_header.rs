/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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

// Opaque types supplied by the surrounding kernel/driver translation.
#[repr(C)]
pub struct amdgpu_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amdgpu_ps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amd_vce_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn amdgpu_dpm_dbg_print_class_info(
        adev: *mut amdgpu_device,
        class: u32,
        class2: u32,
    );
    pub fn amdgpu_dpm_dbg_print_cap_info(adev: *mut amdgpu_device, caps: u32);
    pub fn amdgpu_dpm_dbg_print_ps_status(adev: *mut amdgpu_device, rps: *mut amdgpu_ps);
    pub fn amdgpu_get_platform_caps(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_parse_extended_power_table(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_free_extended_power_table(adev: *mut amdgpu_device);
    pub fn amdgpu_add_thermal_controller(adev: *mut amdgpu_device);
    pub fn amdgpu_get_vce_clock_state(handle: *mut core::ffi::c_void, idx: u32) -> *mut amd_vce_state;
    pub fn amdgpu_pm_print_power_states(adev: *mut amdgpu_device);
    pub fn amdgpu_legacy_dpm_compute_clocks(handle: *mut core::ffi::c_void);
    pub fn amdgpu_dpm_thermal_work_handler(work: *mut work_struct);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
