/*
 * Copyright 2016 Advanced Micro Devices, Inc.
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
 * Authors: Christian König
 */

// Dependency: <linux/hashtable.h>

use core::ffi::c_void;

#[repr(C)]
pub struct dma_fence {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dma_resv {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amdgpu_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amdgpu_ring {
    _private: [u8; 0],
}

#[repr(C)]
pub struct amdgpu_job {
    _private: [u8; 0],
}

pub type gfp_t = usize;
pub type bool_t = bool;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum amdgpu_sync_mode {
    AMDGPU_SYNC_ALWAYS,
    AMDGPU_SYNC_NE_OWNER,
    AMDGPU_SYNC_EQ_OWNER,
    AMDGPU_SYNC_EXPLICIT,
}

/*
 * Container for fences used to sync command submissions.
 * DECLARE_HASHTABLE(fences, 4) expands to four hash-bucket bits, i.e. 16
 * buckets, supplied by the Linux hashtable dependency.
 */
#[repr(C)]
pub struct amdgpu_sync {
    pub fences: [usize; 1 << 4],
}

extern "C" {
    pub fn amdgpu_sync_create(sync: *mut amdgpu_sync);
    pub fn amdgpu_sync_fence(
        sync: *mut amdgpu_sync,
        f: *mut dma_fence,
        flags: gfp_t,
    ) -> i32;
    pub fn amdgpu_sync_resv(
        adev: *mut amdgpu_device,
        sync: *mut amdgpu_sync,
        resv: *mut dma_resv,
        mode: amdgpu_sync_mode,
        owner: *mut c_void,
    ) -> i32;
    pub fn amdgpu_sync_kfd(sync: *mut amdgpu_sync, resv: *mut dma_resv) -> i32;
    pub fn amdgpu_sync_peek_fence(
        sync: *mut amdgpu_sync,
        ring: *mut amdgpu_ring,
    ) -> *mut dma_fence;
    pub fn amdgpu_sync_get_fence(sync: *mut amdgpu_sync) -> *mut dma_fence;
    pub fn amdgpu_sync_clone(source: *mut amdgpu_sync, clone: *mut amdgpu_sync) -> i32;
    pub fn amdgpu_sync_move(src: *mut amdgpu_sync, dst: *mut amdgpu_sync);
    pub fn amdgpu_sync_push_to_job(sync: *mut amdgpu_sync, job: *mut amdgpu_job) -> i32;
    pub fn amdgpu_sync_wait(sync: *mut amdgpu_sync, intr: bool) -> i32;
    pub fn amdgpu_sync_free(sync: *mut amdgpu_sync);
    pub fn amdgpu_sync_init() -> i32;
    pub fn amdgpu_sync_fini();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
