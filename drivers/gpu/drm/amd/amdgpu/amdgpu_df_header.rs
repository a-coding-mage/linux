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

// __AMDGPU_DF_H__ header guard omitted; Rust items are defined once per module.

#[repr(C)]
pub struct amdgpu_df_hash_status {
    pub hash_64k: bool,
    pub hash_2m: bool,
    pub hash_1g: bool,
}

#[repr(C)]
pub struct amdgpu_df_funcs {
    pub sw_init: Option<unsafe extern "C" fn(adev: *mut amdgpu_device)>,
    pub sw_fini: Option<unsafe extern "C" fn(adev: *mut amdgpu_device)>,
    pub hw_init: Option<unsafe extern "C" fn(adev: *mut amdgpu_device)>,
    pub enable_broadcast_mode:
        Option<unsafe extern "C" fn(adev: *mut amdgpu_device, enable: bool)>,
    pub get_fb_channel_number:
        Option<unsafe extern "C" fn(adev: *mut amdgpu_device) -> u32>,
    pub get_hbm_channel_number:
        Option<unsafe extern "C" fn(adev: *mut amdgpu_device) -> u32>,
    pub update_medium_grain_clock_gating:
        Option<unsafe extern "C" fn(adev: *mut amdgpu_device, enable: bool)>,
    pub get_clockgating_state:
        Option<unsafe extern "C" fn(adev: *mut amdgpu_device, flags: *mut u64)>,
    pub enable_ecc_force_par_wr_rmw:
        Option<unsafe extern "C" fn(adev: *mut amdgpu_device, enable: bool)>,
    pub pmc_start: Option<unsafe extern "C" fn(
        adev: *mut amdgpu_device,
        config: u64,
        counter_idx: i32,
        is_add: i32,
    ) -> i32>,
    pub pmc_stop: Option<unsafe extern "C" fn(
        adev: *mut amdgpu_device,
        config: u64,
        counter_idx: i32,
        is_remove: i32,
    ) -> i32>,
    pub pmc_get_count: Option<unsafe extern "C" fn(
        adev: *mut amdgpu_device,
        config: u64,
        counter_idx: i32,
        count: *mut u64,
    )>,
    pub get_fica:
        Option<unsafe extern "C" fn(adev: *mut amdgpu_device, ficaa_val: u32) -> u64>,
    pub set_fica: Option<unsafe extern "C" fn(
        adev: *mut amdgpu_device,
        ficaa_val: u32,
        ficadl_val: u32,
        ficadh_val: u32,
    )>,
    pub query_ras_poison_mode:
        Option<unsafe extern "C" fn(adev: *mut amdgpu_device) -> bool>,
}

#[repr(C)]
pub struct amdgpu_df {
    pub hash_status: amdgpu_df_hash_status,
    pub funcs: *const amdgpu_df_funcs,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
