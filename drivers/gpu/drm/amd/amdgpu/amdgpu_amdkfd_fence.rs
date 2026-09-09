/*
 * Copyright 2016-2018 Advanced Micro Devices, Inc.
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

// Linux kernel dependencies and local headers are supplied by other translation units.

static mut amdkfd_fence_ops: dma_fence_ops = dma_fence_ops {
    get_driver_name: Some(amdkfd_fence_get_driver_name),
    get_timeline_name: Some(amdkfd_fence_get_timeline_name),
    enable_signaling: Some(amdkfd_fence_enable_signaling),
    release: Some(amdkfd_fence_release),
};
static mut fence_seq: atomic_t = ATOMIC_INIT(0);

/* Eviction Fence
 * Fence helper functions to deal with KFD memory eviction.
 * Big Idea - Since KFD submissions are done by user queues, a BO cannot be
 *  evicted unless all the user queues for that process are evicted.
 *
 * All the BOs in a process share an eviction fence. When process X wants to
 * map VRAM memory but TTM can't find enough space, TTM will attempt to
 * evict BOs from its LRU list. TTM checks if the BO is valuable to evict
 * by calling ttm_device_funcs->eviction_valuable().
 *
 * ttm_device_funcs->eviction_valuable() - will return false if the BO belongs
 *  to process X. Otherwise, it will return true to indicate BO can be
 *  evicted by TTM.
 *
 * If ttm_device_funcs->eviction_valuable returns true, then TTM will continue
 * the evcition process for that BO by calling ttm_bo_evict --> amdgpu_bo_move
 * --> amdgpu_copy_buffer(). This sets up job in GPU scheduler.
 *
 * GPU Scheduler (amd_sched_main) - sets up a cb (fence_add_callback) to
 *  nofity when the BO is free to move. fence_add_callback --> enable_signaling
 *  --> amdgpu_amdkfd_fence.enable_signaling
 *
 * amdgpu_amdkfd_fence.enable_signaling - Start a work item that will quiesce
 * user queues and signal fence. The work item will also start another delayed
 * work item to restore BOs
 */

pub unsafe fn amdgpu_amdkfd_fence_create(
    context: u64,
    mm: *mut mm_struct,
    context_id: u16,
) -> *mut amdgpu_amdkfd_fence {
    let fence: *mut amdgpu_amdkfd_fence = kzalloc_obj();

    if fence.is_null() {
        return core::ptr::null_mut();
    }

    /* This reference gets released in amdkfd_fence_release */
    mmgrab(mm);
    (*fence).mm = mm;
    get_task_comm((*fence).timeline_name, current);
    spin_lock_init(&mut (*fence).lock);
    (*fence).context_id = context_id;
    dma_fence_init(
        &mut (*fence).base,
        &raw const amdkfd_fence_ops,
        &mut (*fence).lock,
        context,
        atomic_inc_return(&mut fence_seq),
    );

    fence
}

pub unsafe fn to_amdgpu_amdkfd_fence(
    f: *mut dma_fence,
) -> *mut amdgpu_amdkfd_fence {
    if f.is_null() {
        return core::ptr::null_mut();
    }

    let fence = container_of!(f, amdgpu_amdkfd_fence, base);
    if rcu_access_pointer((*f).ops) == &raw const amdkfd_fence_ops {
        return fence;
    }

    core::ptr::null_mut()
}

unsafe extern "C" fn amdkfd_fence_get_driver_name(_f: *mut dma_fence) -> *const c_char {
    b"amdgpu_amdkfd_fence\0".as_ptr() as *const c_char
}

unsafe extern "C" fn amdkfd_fence_get_timeline_name(
    f: *mut dma_fence,
) -> *const c_char {
    let fence = to_amdgpu_amdkfd_fence(f);

    if !fence.is_null() {
        (*fence).timeline_name.as_ptr() as *const c_char
    } else {
        core::ptr::null()
    }
}

/**
 * amdkfd_fence_enable_signaling - This gets called when TTM wants to evict
 *  a KFD BO and schedules a job to move the BO.
 *  If fence is already signaled return true.
 *  If fence is not signaled schedule a evict KFD process work item.
 *
 *  @f: dma_fence
 */
unsafe extern "C" fn amdkfd_fence_enable_signaling(f: *mut dma_fence) -> bool {
    let fence = to_amdgpu_amdkfd_fence(f);

    if fence.is_null() {
        return false;
    }

    if dma_fence_is_signaled(f) {
        return true;
    }

    if !kgd2kfd_schedule_evict_and_restore_process((*fence).mm, (*fence).context_id, f) {
        return true;
    }
    false
}

/**
 * amdkfd_fence_release - callback that fence can be freed
 *
 * @f: dma_fence
 *
 * This function is called when the reference count becomes zero.
 * Drops the mm_struct reference and RCU schedules freeing up the fence.
 */
unsafe extern "C" fn amdkfd_fence_release(f: *mut dma_fence) {
    let fence = to_amdgpu_amdkfd_fence(f);

    /* Unconditionally signal the fence. The process is getting
     * terminated.
     */
    if WARN_ON(fence.is_null()) {
        return; /* Not an amdgpu_amdkfd_fence */
    }

    mmdrop((*fence).mm);
    kfree_rcu!(f, rcu);
}

/**
 * amdkfd_fence_check_mm - Check whether to prevent eviction of @f by @mm
 *
 * @f: [IN] fence
 * @mm: [IN] mm that needs to be verified
 *
 * Check if @mm is same as that of the fence @f, if same return TRUE else
 * return FALSE.
 */
pub unsafe fn amdkfd_fence_check_mm(
    f: *mut dma_fence,
    mm: *mut mm_struct,
) -> bool {
    let fence = to_amdgpu_amdkfd_fence(f);

    if fence.is_null() {
        false
    } else if (*fence).mm == mm {
        true
    } else {
        false
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
