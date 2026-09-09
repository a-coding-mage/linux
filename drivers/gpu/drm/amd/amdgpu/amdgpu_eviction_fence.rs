// SPDX-License-Identifier: MIT
/*
 * Copyright 2024 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

use crate::*;

unsafe extern "C" fn amdgpu_eviction_fence_get_driver_name(
    _fence: *mut dma_fence,
) -> *const core::ffi::c_char {
    c"amdgpu_eviction_fence".as_ptr()
}

unsafe extern "C" fn amdgpu_eviction_fence_get_timeline_name(
    f: *mut dma_fence,
) -> *const core::ffi::c_char {
    let ef = container_of!(f, amdgpu_eviction_fence, base);
    (*ef).timeline_name.as_ptr()
}

unsafe extern "C" fn amdgpu_eviction_fence_enable_signaling(
    f: *mut dma_fence,
) -> bool {
    let ev_fence = to_ev_fence(f);

    schedule_work(&mut (*(*ev_fence).evf_mgr).suspend_work);
    true
}

static amdgpu_eviction_fence_ops: dma_fence_ops = dma_fence_ops {
    get_driver_name: Some(amdgpu_eviction_fence_get_driver_name),
    get_timeline_name: Some(amdgpu_eviction_fence_get_timeline_name),
    enable_signaling: Some(amdgpu_eviction_fence_enable_signaling),
};

unsafe extern "C" fn amdgpu_eviction_fence_suspend_worker(work: *mut work_struct) {
    let evf_mgr = container_of!(work, amdgpu_eviction_fence_mgr, suspend_work);
    let fpriv = container_of!(evf_mgr, amdgpu_fpriv, evf_mgr);
    let uq_mgr = &mut (*fpriv).userq_mgr;
    let ev_fence: *mut dma_fence;
    let cookie: bool;

    mutex_lock(&mut uq_mgr.userq_mutex);

    /*
     * This is intentionally after taking the userq_mutex since we do
     * allocate memory while holding this lock, but only after ensuring that
     * the eviction fence is signaled.
     */
    cookie = dma_fence_begin_signalling();

    ev_fence = amdgpu_evf_mgr_get_fence(evf_mgr);
    amdgpu_userq_evict(uq_mgr);

    /*
     * Signaling the eviction fence must be done while holding the
     * userq_mutex. Otherwise we won't resume the queues before issuing the
     * next fence.
     */
    dma_fence_signal(ev_fence);
    dma_fence_end_signalling(cookie);
    dma_fence_put(ev_fence);

    if !(*evf_mgr).shutdown {
        schedule_delayed_work(&mut uq_mgr.resume_work, 0);
    }

    mutex_unlock(&mut uq_mgr.userq_mutex);
}

pub unsafe extern "C" fn amdgpu_evf_mgr_attach_fence(
    evf_mgr: *mut amdgpu_eviction_fence_mgr,
    bo: *mut amdgpu_bo,
) -> i32 {
    let ev_fence = amdgpu_evf_mgr_get_fence(evf_mgr);
    let mut ctx = ttm_operation_ctx { interruptible: false, no_wait_gpu: false };
    let resv = (*(*bo).tbo).base.resv;
    let ret: i32;

    if !dma_fence_is_signaled(ev_fence) {
        amdgpu_bo_placement_from_domain(bo, (*bo).allowed_domains);
        ret = ttm_bo_validate(&mut (*bo).tbo, &mut (*bo).placement, &mut ctx);
        if ret == 0 {
            dma_resv_add_fence(resv, ev_fence, DMA_RESV_USAGE_BOOKKEEP);
        }
    } else {
        ret = 0;
    }

    dma_fence_put(ev_fence);
    ret
}

pub unsafe extern "C" fn amdgpu_evf_mgr_rearm(
    evf_mgr: *mut amdgpu_eviction_fence_mgr,
    exec: *mut drm_exec,
) -> i32 {
    let ev_fence = kzalloc_obj::<amdgpu_eviction_fence>();
    if ev_fence.is_null() {
        return -12;
    }

    (*ev_fence).evf_mgr = evf_mgr;
    get_task_comm((*ev_fence).timeline_name.as_mut_ptr(), current);
    spin_lock_init(&mut (*ev_fence).lock);
    dma_fence_init64(
        &mut (*ev_fence).base,
        &amdgpu_eviction_fence_ops,
        &mut (*ev_fence).lock,
        (*evf_mgr).ev_fence_ctx,
        atomic_inc_return(&mut (*evf_mgr).ev_fence_seq),
    );

    /* Remember it for newly added BOs */
    dma_fence_put((*evf_mgr).ev_fence);
    (*evf_mgr).ev_fence = &mut (*ev_fence).base;

    /* And add it to all existing BOs */
    drm_exec_for_each_locked_object!(exec, obj, {
        let bo = gem_to_amdgpu_bo(obj);
        amdgpu_evf_mgr_attach_fence(evf_mgr, bo);
    });
    0
}

pub unsafe extern "C" fn amdgpu_evf_mgr_detach_fence(
    evf_mgr: *mut amdgpu_eviction_fence_mgr,
    bo: *mut amdgpu_bo,
) {
    let stub = dma_fence_get_stub();

    dma_resv_replace_fences(
        (*(*bo).tbo).base.resv,
        (*evf_mgr).ev_fence_ctx,
        stub,
        DMA_RESV_USAGE_BOOKKEEP,
    );
    dma_fence_put(stub);
}

pub unsafe extern "C" fn amdgpu_evf_mgr_init(
    evf_mgr: *mut amdgpu_eviction_fence_mgr,
) {
    atomic_set(&mut (*evf_mgr).ev_fence_seq, 0);
    (*evf_mgr).ev_fence_ctx = dma_fence_context_alloc(1);
    (*evf_mgr).ev_fence = dma_fence_get_stub();

    INIT_WORK(
        &mut (*evf_mgr).suspend_work,
        amdgpu_eviction_fence_suspend_worker,
    );
}

pub unsafe extern "C" fn amdgpu_evf_mgr_shutdown(
    evf_mgr: *mut amdgpu_eviction_fence_mgr,
) {
    (*evf_mgr).shutdown = true;
    /* Make sure that the shutdown is visible to the suspend work */
    flush_work(&mut (*evf_mgr).suspend_work);
}

pub unsafe extern "C" fn amdgpu_evf_mgr_flush_suspend(
    evf_mgr: *mut amdgpu_eviction_fence_mgr,
) {
    dma_fence_wait(rcu_dereference_protected((*evf_mgr).ev_fence, true), false);
    /* Make sure that we are done with the last suspend work */
    flush_work(&mut (*evf_mgr).suspend_work);
}

pub unsafe extern "C" fn amdgpu_evf_mgr_fini(
    evf_mgr: *mut amdgpu_eviction_fence_mgr,
) {
    dma_fence_put((*evf_mgr).ev_fence);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
