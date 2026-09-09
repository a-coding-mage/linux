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

#[repr(C)]
pub enum amdgpu_pkg_type {
    AMDGPU_PKG_TYPE_APU = 2,
    AMDGPU_PKG_TYPE_CEM = 3,
    AMDGPU_PKG_TYPE_OAM = 4,
    AMDGPU_PKG_TYPE_BB = 5,
    AMDGPU_PKG_TYPE_UNKNOWN = 6,
}

#[repr(C)]
pub struct amdgpu_smuio_mcm_config_info {
    pub socket_id: ::core::ffi::c_int,
    pub die_id: ::core::ffi::c_int,
}

#[repr(C)]
pub struct amdgpu_smuio_funcs {
    pub get_rom_index_offset:
        ::core::option::Option<unsafe extern "C" fn(adev: *mut amdgpu_device) -> u32>,
    pub get_rom_data_offset:
        ::core::option::Option<unsafe extern "C" fn(adev: *mut amdgpu_device) -> u32>,
    pub update_rom_clock_gating:
        ::core::option::Option<unsafe extern "C" fn(adev: *mut amdgpu_device, enable: bool)>,
    pub get_clock_gating_state:
        ::core::option::Option<unsafe extern "C" fn(adev: *mut amdgpu_device, flags: *mut u64)>,
    pub get_die_id:
        ::core::option::Option<unsafe extern "C" fn(adev: *mut amdgpu_device) -> u32>,
    pub get_socket_id:
        ::core::option::Option<unsafe extern "C" fn(adev: *mut amdgpu_device) -> u32>,
    pub get_pkg_type: ::core::option::Option<
        unsafe extern "C" fn(adev: *mut amdgpu_device) -> amdgpu_pkg_type,
    >,
    pub is_host_gpu_xgmi_supported:
        ::core::option::Option<unsafe extern "C" fn(adev: *mut amdgpu_device) -> bool>,
    pub is_connected_with_ethernet_switch:
        ::core::option::Option<unsafe extern "C" fn(adev: *mut amdgpu_device) -> bool>,
    pub is_custom_hbm_supported:
        ::core::option::Option<unsafe extern "C" fn(adev: *mut amdgpu_device) -> bool>,
    pub get_gpu_clock_counter:
        ::core::option::Option<unsafe extern "C" fn(adev: *mut amdgpu_device) -> u64>,
}

#[repr(C)]
pub struct amdgpu_smuio {
    pub funcs: *const amdgpu_smuio_funcs,
}

// Declaration supplied by the surrounding dependency set.
pub struct amdgpu_device;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
