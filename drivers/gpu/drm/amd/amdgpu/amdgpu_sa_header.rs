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
 */

// Dependency supplied by drm_suballoc.h.

pub struct amdgpu_device;
pub struct amdgpu_bo;

#[repr(C)]
pub struct amdgpu_sa_manager {
    pub base: drm_suballoc_manager,
    pub bo: *mut amdgpu_bo,
    pub gpu_addr: u64,
    pub cpu_ptr: *mut core::ffi::c_void,
    pub gfp_flags: gfp_t,
}

#[inline]
pub unsafe fn to_amdgpu_sa_manager(
    manager: *mut drm_suballoc_manager,
) -> *mut amdgpu_sa_manager {
    // `base` is the first field, so this is the equivalent of container_of.
    manager as *mut amdgpu_sa_manager
}

#[inline]
pub unsafe fn amdgpu_sa_bo_gpu_addr(sa_bo: *mut drm_suballoc) -> u64 {
    (*to_amdgpu_sa_manager((*sa_bo).manager)).gpu_addr
        .wrapping_add(drm_suballoc_soffset(sa_bo))
}

#[inline]
pub unsafe fn amdgpu_sa_bo_cpu_addr(
    sa_bo: *mut drm_suballoc,
) -> *mut core::ffi::c_void {
    ((*to_amdgpu_sa_manager((*sa_bo).manager)).cpu_ptr as *mut u8)
        .add(drm_suballoc_soffset(sa_bo) as usize) as *mut core::ffi::c_void
}

extern "C" {
    pub fn amdgpu_sa_bo_manager_init(
        adev: *mut amdgpu_device,
        sa_manager: *mut amdgpu_sa_manager,
        size: u32,
        gfp_flags: gfp_t,
    ) -> i32;
    pub fn amdgpu_sa_bo_manager_fini(
        adev: *mut amdgpu_device,
        sa_manager: *mut amdgpu_sa_manager,
    );
    pub fn amdgpu_sa_bo_manager_start(
        adev: *mut amdgpu_device,
        sa_manager: *mut amdgpu_sa_manager,
    ) -> i32;
    pub fn amdgpu_sa_bo_new(
        sa_manager: *mut amdgpu_sa_manager,
        sa_bo: *mut *mut drm_suballoc,
        size: u32,
    ) -> i32;
    pub fn amdgpu_sa_bo_free(
        sa_bo: *mut *mut drm_suballoc,
        fence: *mut dma_fence,
    );

    // Preserved build-time condition: CONFIG_DEBUG_FS.
    #[cfg(feature = "CONFIG_DEBUG_FS")]
    pub fn amdgpu_sa_bo_dump_debug_info(
        sa_manager: *mut amdgpu_sa_manager,
        m: *mut seq_file,
    );
    #[cfg(feature = "CONFIG_DEBUG_FS")]
    pub fn amdgpu_bo_print_info(
        id: i32,
        bo: *mut amdgpu_bo,
        m: *mut seq_file,
    ) -> u64;

    pub fn amdgpu_debugfs_sa_init(adev: *mut amdgpu_device);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
