/*
 * Copyright 2019 Advanced Micro Devices, Inc.
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

// Dependency supplied by the surrounding translation unit: <drm/drm_gem.h>.

#[repr(C)]
pub struct dma_buf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_gem_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct drm_device {
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
pub struct dma_buf_ops {
    _private: [u8; 0],
}

extern "C" {
    pub fn amdgpu_gem_prime_export(
        gobj: *mut drm_gem_object,
        flags: i32,
    ) -> *mut dma_buf;

    pub fn amdgpu_gem_prime_import(
        dev: *mut drm_device,
        dma_buf: *mut dma_buf,
    ) -> *mut drm_gem_object;

    pub fn amdgpu_dmabuf_is_xgmi_accessible(
        adev: *mut amdgpu_device,
        bo: *mut amdgpu_bo,
    ) -> bool;

    pub static amdgpu_dmabuf_ops: dma_buf_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
