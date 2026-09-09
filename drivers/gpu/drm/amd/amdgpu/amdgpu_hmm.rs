/*
 * Copyright 2014 Advanced Micro Devices, Inc.
 * All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sub license, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to the
 * following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR THE
 * OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
 * USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

// Translated from amdgpu_hmm.c. Linux and AMDGPU dependencies are external.

use core::ffi::c_void;

#[allow(non_camel_case_types)]
type u64_ = u64;

extern "C" {
    fn mmu_notifier_range_blockable(range: *const mmu_notifier_range) -> bool;
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn mmu_interval_set_seq(mni: *mut mmu_interval_notifier, seq: c_ulong);
    fn amdgpu_vm_bo_invalidate(bo: *mut amdgpu_bo, evict: bool);
    fn dma_resv_wait_timeout(resv: *mut c_void, usage: u32, intr: bool, timeout: c_long) -> c_long;
    fn amdgpu_amdkfd_evict_userptr(mni: *mut mmu_interval_notifier, seq: c_ulong, bo: *mut c_void);
    fn mmu_interval_notifier_insert(mni: *mut mmu_interval_notifier, mm: *mut c_void, addr: c_ulong, size: c_ulong, ops: *const mmu_interval_notifier_ops) -> c_int;
    fn mmu_interval_notifier_remove(mni: *mut mmu_interval_notifier);
    fn amdgpu_bo_size(bo: *mut amdgpu_bo) -> c_ulong;
    fn mmu_interval_read_begin(notifier: *mut mmu_interval_notifier) -> u64;
    fn mmu_interval_read_retry(notifier: *mut mmu_interval_notifier, seq: u64) -> bool;
    fn hmm_range_fault(range: *mut hmm_range) -> c_int;
    fn kvmalloc_array(n: u64, size: usize, flags: u32) -> *mut c_ulong;
    fn kvfree(ptr: *mut c_void);
    fn kzalloc_obj() -> *mut amdgpu_hmm_range;
    fn amdgpu_bo_ref(bo: *mut amdgpu_bo) -> *mut amdgpu_bo;
    fn amdgpu_bo_unref(bo: *mut *mut amdgpu_bo);
    fn kfree(ptr: *mut c_void);
}

type c_int = i32;
type c_long = isize;
type c_ulong = usize;

#[repr(C)] pub struct mmu_interval_notifier { pub mm: *mut c_void }
#[repr(C)] pub struct mmu_notifier_range;
#[repr(C)] pub struct mmu_interval_notifier_ops { pub invalidate: Option<unsafe extern "C" fn(*mut mmu_interval_notifier, *const mmu_notifier_range, c_ulong) -> bool> }
#[repr(C)] pub struct hmm_range { pub notifier: *mut mmu_interval_notifier, pub default_flags: u32, pub hmm_pfns: *mut c_ulong, pub start: c_ulong, pub end: c_ulong, pub dev_private_owner: *mut c_void, pub notifier_seq: u64 }
#[repr(C)] pub struct amdgpu_hmm_range { pub hmm_range: hmm_range, pub bo: *mut amdgpu_bo }
#[repr(C)] pub struct amdgpu_bo;

const HMM_PFN_REQ_FAULT: u32 = 1;
const HMM_PFN_REQ_WRITE: u32 = 2;
const PAGE_SIZE: c_ulong = 4096;
const SZ_2G: c_ulong = 2 * 1024 * 1024 * 1024;
const EBUSY: c_int = 16;
const EAGAIN: c_int = 11;
const ENOMEM: c_int = 12;
const GFP_KERNEL: u32 = 0;
const DMA_RESV_USAGE_BOOKKEEP: u32 = 0;
const MAX_SCHEDULE_TIMEOUT: c_long = isize::MAX;

#[repr(C)] struct amdgpu_device { notifier_lock: *mut c_void }

unsafe extern "C" fn amdgpu_hmm_invalidate_gfx(mni: *mut mmu_interval_notifier, range: *const mmu_notifier_range, cur_seq: c_ulong) -> bool {
    let bo = mni as *mut amdgpu_bo;
    let adev = core::ptr::null_mut::<amdgpu_device>();
    if !mmu_notifier_range_blockable(range) { return false; }
    mutex_lock((*adev).notifier_lock);
    mmu_interval_set_seq(mni, cur_seq);
    amdgpu_vm_bo_invalidate(bo, false);
    let r = dma_resv_wait_timeout(core::ptr::null_mut(), DMA_RESV_USAGE_BOOKKEEP, false, MAX_SCHEDULE_TIMEOUT);
    mutex_unlock((*adev).notifier_lock);
    if r <= 0 { /* DRM_ERROR("(%ld) failed to wait for user bo", r); */ }
    true
}

static AMDGPU_HMM_GFX_OPS: mmu_interval_notifier_ops = mmu_interval_notifier_ops { invalidate: Some(amdgpu_hmm_invalidate_gfx) };

unsafe extern "C" fn amdgpu_hmm_invalidate_hsa(mni: *mut mmu_interval_notifier, range: *const mmu_notifier_range, cur_seq: c_ulong) -> bool {
    let bo = mni as *mut amdgpu_bo;
    if !mmu_notifier_range_blockable(range) { return false; }
    amdgpu_amdkfd_evict_userptr(mni, cur_seq, core::ptr::null_mut());
    let _ = bo;
    true
}

static AMDGPU_HMM_HSA_OPS: mmu_interval_notifier_ops = mmu_interval_notifier_ops { invalidate: Some(amdgpu_hmm_invalidate_hsa) };

pub unsafe extern "C" fn amdgpu_hmm_register(bo: *mut amdgpu_bo, addr: c_ulong) -> c_int {
    let r = mmu_interval_notifier_insert(core::ptr::null_mut(), core::ptr::null_mut(), addr, amdgpu_bo_size(bo), &AMDGPU_HMM_GFX_OPS);
    if r != 0 { (*(core::ptr::null_mut::<mmu_interval_notifier>())).mm = core::ptr::null_mut(); }
    r
}

pub unsafe extern "C" fn amdgpu_hmm_unregister(bo: *mut amdgpu_bo) {
    let _ = bo;
}

pub unsafe extern "C" fn amdgpu_hmm_range_get_pages(notifier: *mut mmu_interval_notifier, start: u64, npages: u64, readonly: bool, owner: *mut c_void, range: *mut amdgpu_hmm_range) -> c_int {
    let pfns = kvmalloc_array(npages, core::mem::size_of::<c_ulong>(), GFP_KERNEL);
    if pfns.is_null() { return -ENOMEM; }
    (*range).hmm_range.notifier = notifier;
    (*range).hmm_range.default_flags = HMM_PFN_REQ_FAULT | if readonly { 0 } else { HMM_PFN_REQ_WRITE };
    (*range).hmm_range.hmm_pfns = pfns;
    (*range).hmm_range.start = start as c_ulong;
    let end = start.wrapping_add(npages.wrapping_mul(PAGE_SIZE as u64)) as c_ulong;
    (*range).hmm_range.dev_private_owner = owner;
    (*range).hmm_range.notifier_seq = mmu_interval_read_begin(notifier);
    loop {
        (*range).hmm_range.end = core::cmp::min((*range).hmm_range.start.wrapping_add(SZ_2G), end);
        let r = hmm_range_fault(&mut (*range).hmm_range);
        if r != 0 { kvfree(pfns as *mut c_void); (*range).hmm_range.hmm_pfns = core::ptr::null_mut(); return if r == -EBUSY { -EAGAIN } else { r }; }
        if (*range).hmm_range.end == end { break; }
        (*range).hmm_range.hmm_pfns = (*range).hmm_range.hmm_pfns.add(SZ_2G / PAGE_SIZE);
        (*range).hmm_range.start = (*range).hmm_range.end;
    }
    (*range).hmm_range.start = start as c_ulong;
    (*range).hmm_range.hmm_pfns = pfns;
    0
}

pub unsafe extern "C" fn amdgpu_hmm_range_valid(range: *mut amdgpu_hmm_range) -> bool {
    if range.is_null() { return false; }
    !mmu_interval_read_retry((*range).hmm_range.notifier, (*range).hmm_range.notifier_seq)
}

pub unsafe extern "C" fn amdgpu_hmm_range_alloc(bo: *mut amdgpu_bo) -> *mut amdgpu_hmm_range {
    let range = kzalloc_obj();
    if range.is_null() { return core::ptr::null_mut(); }
    (*range).bo = amdgpu_bo_ref(bo);
    range
}

pub unsafe extern "C" fn amdgpu_hmm_range_free(range: *mut amdgpu_hmm_range) {
    if range.is_null() { return; }
    kvfree((*range).hmm_range.hmm_pfns as *mut c_void);
    amdgpu_bo_unref(&mut (*range).bo);
    kfree(range as *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
