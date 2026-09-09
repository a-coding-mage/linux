/* SPDX-License-Identifier: MIT */
/*
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
 *
 * Authors: AMD
 */

use core::ffi::c_void;

#[repr(C)]
pub struct amdgpu_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_atomic_commit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_connector {
    _private: [u8; 0],
}

#[repr(C)]
pub struct audio_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dc_sink {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn amdgpu_dm_audio_init(adev: *mut amdgpu_device) -> i32;
    pub fn amdgpu_dm_audio_fini(adev: *mut amdgpu_device);
    pub fn amdgpu_dm_commit_audio(
        dev: *mut drm_device,
        state: *mut drm_atomic_commit,
    );
    pub fn amdgpu_dm_fill_audio_info(
        audio_info: *mut audio_info,
        drm_connector: *const drm_connector,
        dc_sink: *const dc_sink,
    );
    pub fn amdgpu_dm_audio_eld_notify(adev: *mut amdgpu_device, pin: i32);
}

// Preserved from: #if IS_ENABLED(CONFIG_DRM_AMD_DC_KUNIT_TEST)
// These declarations are enabled when CONFIG_DRM_AMD_DC_KUNIT_TEST is enabled.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn amdgpu_dm_audio_component_bind(
        kdev: *mut device,
        hda_kdev: *mut device,
        data: *mut c_void,
    ) -> i32;
    pub fn amdgpu_dm_audio_component_unbind(
        kdev: *mut device,
        hda_kdev: *mut device,
        data: *mut c_void,
    );
    pub fn amdgpu_dm_audio_get_param() -> i32;
    pub fn amdgpu_dm_audio_set_param(val: i32);
    pub fn amdgpu_dm_audio_init_pins(
        adev: *mut amdgpu_device,
        audio_count: i32,
        inst_array: *const u32,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
