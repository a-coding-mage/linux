// SPDX-License-Identifier: MIT
/*
 * Copyright 2014 Advanced Micro Devices, Inc.
 * All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sub license, and/or sell copies of the Software, and to permit
 * persons to whom the Software is furnished to do so, subject to the following
 * conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT.
 */

// Dependencies are supplied by the surrounding kernel translation.

#[repr(C)]
struct AmdgpuSyncEntry {
    node: HlistNode,
    fence: *mut DmaFence,
}

static mut AMDGPU_SYNC_SLAB: *mut KmemCache = core::ptr::null_mut();

pub unsafe fn amdgpu_sync_create(sync: *mut AmdgpuSync) {
    hash_init((*sync).fences);
}

unsafe fn amdgpu_sync_same_dev(adev: *mut AmdgpuDevice, f: *mut DmaFence) -> bool {
    let s_fence = to_drm_sched_fence(f);
    if !s_fence.is_null() {
        let ring = container_of((*s_fence).sched, AmdgpuRing, sched);
        return (*ring).adev == adev;
    }
    false
}

unsafe fn amdgpu_sync_get_owner(f: *mut DmaFence) -> *mut core::ffi::c_void {
    if f.is_null() { return AMDGPU_FENCE_OWNER_UNDEFINED; }
    let s_fence = to_drm_sched_fence(f);
    if !s_fence.is_null() { return (*s_fence).owner; }
    let kfd_fence = to_amdgpu_amdkfd_fence(f);
    if !kfd_fence.is_null() { return AMDGPU_FENCE_OWNER_KFD; }
    AMDGPU_FENCE_OWNER_UNDEFINED
}

unsafe fn amdgpu_sync_keep_later(keep: *mut *mut DmaFence, fence: *mut DmaFence) {
    if !(*keep).is_null() && dma_fence_is_later(*keep, fence) { return; }
    dma_fence_put(*keep);
    *keep = dma_fence_get(fence);
}

unsafe fn amdgpu_sync_add_later(sync: *mut AmdgpuSync, f: *mut DmaFence) -> bool {
    let mut e: *mut AmdgpuSyncEntry = core::ptr::null_mut();
    // hash_for_each_possible(sync->fences, e, node, f->context)
    for_each_possible_sync_entry((*sync).fences, (*f).context, &mut e) {
        if dma_fence_is_signaled((*e).fence) {
            dma_fence_put((*e).fence);
            (*e).fence = dma_fence_get(f);
            return true;
        }
        if (*e).fence.context == (*f).context {
            amdgpu_sync_keep_later(&mut (*e).fence, f);
            return true;
        }
    }
    false
}

pub unsafe fn amdgpu_sync_fence(sync: *mut AmdgpuSync, f: *mut DmaFence, flags: GfpT) -> i32 {
    if f.is_null() { return 0; }
    if amdgpu_sync_add_later(sync, f) { return 0; }
    let e = kmem_cache_alloc(AMDGPU_SYNC_SLAB, flags) as *mut AmdgpuSyncEntry;
    if e.is_null() { return -ENOMEM; }
    hash_add((*sync).fences, &mut (*e).node, (*f).context);
    (*e).fence = dma_fence_get(f);
    0
}

unsafe fn amdgpu_sync_test_fence(adev: *mut AmdgpuDevice, mode: AmdgpuSyncMode,
                                  owner: *mut core::ffi::c_void, f: *mut DmaFence) -> bool {
    let fence_owner = amdgpu_sync_get_owner(f);
    if fence_owner == AMDGPU_FENCE_OWNER_UNDEFINED { return true; }
    if fence_owner == AMDGPU_FENCE_OWNER_KFD && owner != AMDGPU_FENCE_OWNER_UNDEFINED { return false; }
    if fence_owner == AMDGPU_FENCE_OWNER_VM && owner != AMDGPU_FENCE_OWNER_UNDEFINED && owner != AMDGPU_FENCE_OWNER_KFD { return false; }
    match mode {
        AMDGPU_SYNC_ALWAYS => true,
        AMDGPU_SYNC_NE_OWNER => {
            if amdgpu_sync_same_dev(adev, f) && fence_owner == owner { return false; }
            true
        }
        AMDGPU_SYNC_EQ_OWNER => {
            if amdgpu_sync_same_dev(adev, f) && fence_owner != owner { return false; }
            true
        }
        AMDGPU_SYNC_EXPLICIT => false,
        _ => { WARN(debug_evictions && fence_owner == AMDGPU_FENCE_OWNER_KFD, "Adding eviction fence to sync obj"); true }
    }
}

pub unsafe fn amdgpu_sync_resv(adev: *mut AmdgpuDevice, sync: *mut AmdgpuSync,
                               resv: *mut DmaResv, mode: AmdgpuSyncMode,
                               owner: *mut core::ffi::c_void) -> i32 {
    if resv.is_null() { return -EINVAL; }
    let mut cursor = DmaResvIter::default();
    let mut f: *mut DmaFence = core::ptr::null_mut();
    dma_resv_for_each_fence(&mut cursor, resv, DMA_RESV_USAGE_READ, &mut f) {
        dma_fence_chain_for_each(f, f) {
            let tmp = dma_fence_chain_contained(f);
            if amdgpu_sync_test_fence(adev, mode, owner, tmp) {
                let r = amdgpu_sync_fence(sync, f, GFP_KERNEL);
                dma_fence_put(f);
                if r != 0 { return r; }
                break;
            }
        }
    }
    0
}

pub unsafe fn amdgpu_sync_kfd(sync: *mut AmdgpuSync, resv: *mut DmaResv) -> i32 {
    let mut cursor = DmaResvIter::default();
    let mut f: *mut DmaFence = core::ptr::null_mut();
    let mut r = 0;
    dma_resv_iter_begin(&mut cursor, resv, DMA_RESV_USAGE_BOOKKEEP);
    dma_resv_for_each_fence_unlocked(&mut cursor, &mut f) {
        if amdgpu_sync_get_owner(f) != AMDGPU_FENCE_OWNER_KFD { continue; }
        r = amdgpu_sync_fence(sync, f, GFP_KERNEL);
        if r != 0 { break; }
    }
    dma_resv_iter_end(&mut cursor);
    r
}

unsafe fn amdgpu_sync_entry_free(e: *mut AmdgpuSyncEntry) {
    hash_del(&mut (*e).node);
    dma_fence_put((*e).fence);
    kmem_cache_free(AMDGPU_SYNC_SLAB, e as *mut core::ffi::c_void);
}

pub unsafe fn amdgpu_sync_peek_fence(sync: *mut AmdgpuSync, ring: *mut AmdgpuRing) -> *mut DmaFence {
    for_each_sync_entry_safe((*sync).fences, |e, _tmp| {
        let f = (*e).fence;
        let s = to_drm_sched_fence(f);
        if dma_fence_is_signaled(f) { amdgpu_sync_entry_free(e); return None; }
        if !ring.is_null() && !s.is_null() && (*s).sched == &mut (*ring).sched {
            if dma_fence_is_signaled(&mut (*s).scheduled) { return None; }
            return Some(&mut (*s).scheduled as *mut DmaFence);
        }
        Some(f)
    }).unwrap_or(core::ptr::null_mut())
}

pub unsafe fn amdgpu_sync_get_fence(sync: *mut AmdgpuSync) -> *mut DmaFence {
    for_each_sync_entry_safe((*sync).fences, |e, _tmp| {
        let f = (*e).fence;
        hash_del(&mut (*e).node);
        kmem_cache_free(AMDGPU_SYNC_SLAB, e as *mut core::ffi::c_void);
        if !dma_fence_is_signaled(f) { return Some(f); }
        dma_fence_put(f); None
    }).unwrap_or(core::ptr::null_mut())
}

pub unsafe fn amdgpu_sync_clone(source: *mut AmdgpuSync, clone: *mut AmdgpuSync) -> i32 {
    for_each_sync_entry_safe((*source).fences, |e, _tmp| {
        if !dma_fence_is_signaled((*e).fence) { let _ = amdgpu_sync_fence(clone, (*e).fence, GFP_KERNEL); }
        else { amdgpu_sync_entry_free(e); }
        None
    });
    0
}

pub unsafe fn amdgpu_sync_move(src: *mut AmdgpuSync, dst: *mut AmdgpuSync) {
    amdgpu_sync_free(dst);
    for i in 0..HASH_SIZE((*src).fences) { hlist_move_list(&mut (*src).fences[i], &mut (*dst).fences[i]); }
}

pub unsafe fn amdgpu_sync_push_to_job(sync: *mut AmdgpuSync, job: *mut AmdgpuJob) -> i32 {
    let mut result = 0;
    for_each_sync_entry_safe((*sync).fences, |e, _tmp| {
        let f = (*e).fence;
        if dma_fence_is_signaled(f) { amdgpu_sync_entry_free(e); return None; }
        dma_fence_get(f);
        result = drm_sched_job_add_dependency(&mut (*job).base, f);
        if result != 0 { dma_fence_put(f); }
        None
    });
    result
}

pub unsafe fn amdgpu_sync_wait(sync: *mut AmdgpuSync, intr: bool) -> i32 {
    let mut r = 0;
    for_each_sync_entry_safe((*sync).fences, |e, _tmp| {
        r = dma_fence_wait((*e).fence, intr);
        if r == 0 { amdgpu_sync_entry_free(e); }
        None
    });
    r
}

pub unsafe fn amdgpu_sync_free(sync: *mut AmdgpuSync) {
    for_each_sync_entry_safe((*sync).fences, |e, _tmp| { amdgpu_sync_entry_free(e); None });
}

pub unsafe fn amdgpu_sync_init() -> i32 {
    AMDGPU_SYNC_SLAB = KMEM_CACHE::<AmdgpuSyncEntry>(SLAB_HWCACHE_ALIGN);
    if AMDGPU_SYNC_SLAB.is_null() { return -ENOMEM; }
    0
}

pub unsafe fn amdgpu_sync_fini() { kmem_cache_destroy(AMDGPU_SYNC_SLAB); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
