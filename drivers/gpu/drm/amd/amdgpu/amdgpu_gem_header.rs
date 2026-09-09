/*
 * Copyright 2018 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding DRM/amdgpu translation units.

pub const AMDGPU_GEM_DOMAIN_MAX: u32 = 0x3;

#[macro_export]
macro_rules! gem_to_amdgpu_bo {
    ($gobj:expr) => {
        container_of!($gobj, amdgpu_bo, tbo.base)
    };
}

#[repr(C)]
pub struct drm_gem_object_funcs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amdgpu_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amdgpu_bo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_mode_create_dumb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dma_resv {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_gem_object {
    _private: [u8; 0],
}

pub type ttm_bo_type = i32;

extern "C" {
    pub static amdgpu_gem_object_funcs: drm_gem_object_funcs;

    pub fn amdgpu_gem_timeout(timeout_ns: u64) -> usize;

    pub fn amdgpu_gem_force_release(adev: *mut amdgpu_device);
    pub fn amdgpu_gem_object_create(
        adev: *mut amdgpu_device,
        size: usize,
        alignment: i32,
        initial_domain: u32,
        flags: u64,
        type_: ttm_bo_type,
        resv: *mut dma_resv,
        obj: *mut *mut drm_gem_object,
        xcp_id_plus1: i8,
    ) -> i32;
    pub fn amdgpu_mode_dumb_create(
        file_priv: *mut drm_file,
        dev: *mut drm_device,
        args: *mut drm_mode_create_dumb,
    ) -> i32;
    pub fn amdgpu_mode_dumb_mmap(
        filp: *mut drm_file,
        dev: *mut drm_device,
        handle: u32,
        offset_p: *mut u64,
    ) -> i32;

    pub fn amdgpu_gem_create_ioctl(dev: *mut drm_device, data: *mut core::ffi::c_void, filp: *mut drm_file) -> i32;
    pub fn amdgpu_gem_info_ioctl(dev: *mut drm_device, data: *mut core::ffi::c_void, filp: *mut drm_file) -> i32;
    pub fn amdgpu_gem_userptr_ioctl(dev: *mut drm_device, data: *mut core::ffi::c_void, filp: *mut drm_file) -> i32;
    pub fn amdgpu_gem_mmap_ioctl(dev: *mut drm_device, data: *mut core::ffi::c_void, filp: *mut drm_file) -> i32;
    pub fn amdgpu_gem_wait_idle_ioctl(dev: *mut drm_device, data: *mut core::ffi::c_void, filp: *mut drm_file) -> i32;
    pub fn amdgpu_gem_va_ioctl(dev: *mut drm_device, data: *mut core::ffi::c_void, filp: *mut drm_file) -> i32;
    pub fn amdgpu_gem_op_ioctl(dev: *mut drm_device, data: *mut core::ffi::c_void, filp: *mut drm_file) -> i32;
    pub fn amdgpu_gem_list_handles_ioctl(dev: *mut drm_device, data: *mut core::ffi::c_void, filp: *mut drm_file) -> i32;
    pub fn amdgpu_gem_metadata_ioctl(dev: *mut drm_device, data: *mut core::ffi::c_void, filp: *mut drm_file) -> i32;
}

pub const AMDGPU_GEM_CREATE_SETTABLE_MASK: u64 =
    AMDGPU_GEM_CREATE_CPU_ACCESS_REQUIRED
    | AMDGPU_GEM_CREATE_NO_CPU_ACCESS
    | AMDGPU_GEM_CREATE_CPU_GTT_USWC
    | AMDGPU_GEM_CREATE_VRAM_CLEARED
    | AMDGPU_GEM_CREATE_VM_ALWAYS_VALID
    | AMDGPU_GEM_CREATE_EXPLICIT_SYNC
    | AMDGPU_GEM_CREATE_VRAM_WIPE_ON_RELEASE
    | AMDGPU_GEM_CREATE_ENCRYPTED
    | AMDGPU_GEM_CREATE_GFX12_DCC
    | AMDGPU_GEM_CREATE_DISCARDABLE
    | AMDGPU_GEM_CREATE_COHERENT
    | AMDGPU_GEM_CREATE_UNCACHED
    | AMDGPU_GEM_CREATE_EXT_COHERENT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
