/* SPDX-License-Identifier: MIT */
/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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

// Dependency supplied externally: <linux/dma-fence.h>

#[repr(C)]
pub struct amdgpu_eviction_fence {
    pub base: dma_fence,
    pub lock: spinlock_t,
    pub timeline_name: [core::ffi::c_char; TASK_COMM_LEN],
    pub evf_mgr: *mut amdgpu_eviction_fence_mgr,
}

#[repr(C)]
pub struct amdgpu_eviction_fence_mgr {
    pub ev_fence_ctx: u64,
    pub ev_fence_seq: atomic_t,

    /*
     * Only updated while holding the VM resv lock.
     * Only signaled while holding the userq mutex.
     */
    pub ev_fence: *mut dma_fence,
    pub suspend_work: work_struct,
    pub shutdown: bool,
}

#[inline]
pub unsafe fn amdgpu_evf_mgr_get_fence(
    evf_mgr: *mut amdgpu_eviction_fence_mgr,
) -> *mut dma_fence {
    let ev_fence: *mut dma_fence;

    rcu_read_lock();
    ev_fence = dma_fence_get_rcu_safe(
        &mut (*evf_mgr).ev_fence as *mut *mut dma_fence,
    );
    rcu_read_unlock();
    ev_fence
}

extern "C" {
    pub fn amdgpu_evf_mgr_attach_fence(
        evf_mgr: *mut amdgpu_eviction_fence_mgr,
        bo: *mut amdgpu_bo,
    ) -> i32;
    pub fn amdgpu_evf_mgr_rearm(
        evf_mgr: *mut amdgpu_eviction_fence_mgr,
        exec: *mut drm_exec,
    ) -> i32;
    pub fn amdgpu_evf_mgr_detach_fence(
        evf_mgr: *mut amdgpu_eviction_fence_mgr,
        bo: *mut amdgpu_bo,
    );
    pub fn amdgpu_evf_mgr_init(evf_mgr: *mut amdgpu_eviction_fence_mgr);
    pub fn amdgpu_evf_mgr_shutdown(evf_mgr: *mut amdgpu_eviction_fence_mgr);
    pub fn amdgpu_evf_mgr_flush_suspend(evf_mgr: *mut amdgpu_eviction_fence_mgr);
    pub fn amdgpu_evf_mgr_fini(evf_mgr: *mut amdgpu_eviction_fence_mgr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
