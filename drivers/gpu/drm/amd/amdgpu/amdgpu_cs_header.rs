/*
 * Copyright 2022 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the corresponding kernel DRM/AMDGPU translation units.

pub const AMDGPU_CS_GANG_SIZE: usize = 4;

pub struct amdgpu_bo_va_mapping;

#[repr(C)]
pub struct amdgpu_cs_chunk {
    pub chunk_id: u32,
    pub length_dw: u32,
    pub kdata: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct amdgpu_cs_post_dep {
    pub syncobj: *mut drm_syncobj,
    pub chain: *mut dma_fence_chain,
    pub point: u64,
}

#[repr(C)]
pub struct amdgpu_cs_parser {
    pub adev: *mut amdgpu_device,
    pub filp: *mut drm_file,
    pub ctx: *mut amdgpu_ctx,

    /* chunks */
    pub nchunks: u32,
    pub chunks: *mut amdgpu_cs_chunk,

    /* scheduler job objects */
    pub gang_size: u32,
    pub gang_leader_idx: u32,
    pub entities: [*mut drm_sched_entity; AMDGPU_CS_GANG_SIZE],
    pub jobs: [*mut amdgpu_job; AMDGPU_CS_GANG_SIZE],
    pub gang_leader: *mut amdgpu_job,

    /* buffer objects */
    pub exec: drm_exec,
    pub bo_list: *mut amdgpu_bo_list,
    pub mn: *mut amdgpu_mn,
    pub fence: *mut dma_fence,
    pub bytes_moved_threshold: u64,
    pub bytes_moved_vis_threshold: u64,
    pub bytes_moved: u64,
    pub bytes_moved_vis: u64,

    /* user fence */
    pub uf_bo: *mut amdgpu_bo,

    pub num_post_deps: u32,
    pub post_deps: *mut amdgpu_cs_post_dep,

    pub sync: amdgpu_sync,
}

unsafe extern "C" {
    pub fn amdgpu_cs_find_mapping(
        parser: *mut amdgpu_cs_parser,
        addr: u64,
        bo: *mut *mut amdgpu_bo,
        mapping: *mut *mut amdgpu_bo_va_mapping,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
