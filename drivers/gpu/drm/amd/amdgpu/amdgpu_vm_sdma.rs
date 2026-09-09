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
 */

const AMDGPU_VM_SDMA_MIN_NUM_DW: u32 = 256;
const AMDGPU_VM_SDMA_MAX_NUM_DW: u32 = 16 * 1024;

/* External types, constants, and functions are supplied by the surrounding driver. */

unsafe fn amdgpu_vm_sdma_map_table(table: *mut amdgpu_bo_vm) -> i32 {
    amdgpu_ttm_alloc_gart(&mut (*table).bo.tbo)
}

/* Allocate a new job for @count PTE updates */
unsafe fn amdgpu_vm_sdma_alloc_job(
    p: *mut amdgpu_vm_update_params,
    count: u32,
    k_job_id: u64,
) -> i32 {
    let pool = if (*p).immediate { AMDGPU_IB_POOL_IMMEDIATE } else { AMDGPU_IB_POOL_DELAYED };
    let entity = if (*p).immediate {
        &mut (*(*p).vm).immediate
    } else {
        &mut (*(*p).vm).delayed
    };
    let mut ndw: u32;
    let r: i32;

    /* estimate how many dw we need */
    ndw = AMDGPU_VM_SDMA_MIN_NUM_DW;
    if !(*p).pages_addr.is_null() {
        ndw += count * 2;
    }
    ndw = std::cmp::min(ndw, AMDGPU_VM_SDMA_MAX_NUM_DW);

    r = amdgpu_job_alloc_with_ib(
        (*p).adev,
        entity,
        AMDGPU_FENCE_OWNER_VM,
        ndw * 4,
        pool,
        k_job_id,
        &mut (*p).job,
    );
    if r != 0 {
        return r;
    }

    (*p).num_dw_left = ndw;
    0
}

unsafe fn amdgpu_vm_sdma_prepare(
    p: *mut amdgpu_vm_update_params,
    sync: *mut amdgpu_sync,
    k_job_id: u64,
) -> i32 {
    let mut r = amdgpu_vm_sdma_alloc_job(p, 0, k_job_id);
    if r != 0 {
        return r;
    }
    if sync.is_null() {
        return 0;
    }
    r = amdgpu_sync_push_to_job(sync, (*p).job);
    if r != 0 {
        (*p).num_dw_left = 0;
        amdgpu_job_free((*p).job);
    }
    r
}

unsafe fn amdgpu_vm_sdma_commit(
    p: *mut amdgpu_vm_update_params,
    fence: *mut *mut dma_fence,
) -> i32 {
    let ib = (*p).job. as_ref().unwrap().ibs;
    let ring: *mut amdgpu_ring;
    let mut f: *mut dma_fence;

    ring = container_of((*(*p).vm).delayed.rq.as_ref().unwrap().sched, amdgpu_ring, sched);
    WARN_ON((*ib).length_dw == 0);
    amdgpu_ring_pad_ib(ring, ib);
    if (*p).needs_flush {
        atomic64_inc(&mut (*(*p).vm).tlb_seq);
    }
    WARN_ON((*ib).length_dw > (*p).num_dw_left);
    f = amdgpu_job_submit((*p).job);

    if (*p).unlocked {
        let mut tmp = dma_fence_get(f);
        std::mem::swap(&mut (*(*p).vm).last_unlocked, &mut tmp);
        dma_fence_put(tmp);
    } else {
        dma_resv_add_fence((*(*p).vm).root.bo.tbo.base.resv, f, DMA_RESV_USAGE_BOOKKEEP);
    }
    if !fence.is_null() && !(*p).immediate {
        /* Most hw generations now have a separate queue for page table updates,
         * but when the queue is shared with userspace we need the extra CPU
         * round trip to correctly flush the TLB. */
        set_bit(DRM_SCHED_FENCE_DONT_PIPELINE, &mut (*f).flags);
        std::mem::swap(&mut *fence, &mut f);
    }
    dma_fence_put(f);
    0
}

unsafe fn amdgpu_vm_sdma_copy_ptes(
    p: *mut amdgpu_vm_update_params,
    bo: *mut amdgpu_bo,
    mut pe: u64,
    count: u32,
) {
    let ib = (*p).job.ibs;
    let mut src = (*ib).gpu_addr;
    src += (*p).num_dw_left as u64 * 4;
    pe += amdgpu_bo_gpu_offset_no_check(bo);
    trace_amdgpu_vm_copy_ptes(pe, src, count, (*p).immediate);
    amdgpu_vm_copy_pte((*p).adev, ib, pe, src, count);
}

unsafe fn amdgpu_vm_sdma_set_ptes(
    p: *mut amdgpu_vm_update_params,
    bo: *mut amdgpu_bo,
    mut pe: u64,
    addr: u64,
    count: u32,
    incr: u32,
    flags: u64,
) {
    let ib = (*p).job.ibs;
    pe += amdgpu_bo_gpu_offset_no_check(bo);
    trace_amdgpu_vm_set_ptes(pe, addr, count, incr, flags, (*p).immediate);
    if count < 3 {
        amdgpu_vm_write_pte((*p).adev, ib, pe, addr | flags, count, incr);
    } else {
        amdgpu_vm_set_pte_pde((*p).adev, ib, pe, addr, count, incr, flags);
    }
}

unsafe fn amdgpu_vm_sdma_update(
    p: *mut amdgpu_vm_update_params,
    vmbo: *mut amdgpu_bo_vm,
    mut pe: u64,
    mut addr: u64,
    mut count: u32,
    incr: u32,
    mut flags: u64,
) -> i32 {
    let bo = &mut (*vmbo).bo;
    let mut cursor = std::mem::MaybeUninit::<dma_resv_iter>::uninit();
    let mut i: u32;
    let mut ndw: u32;
    let mut nptes: u32;
    let mut fence: *mut dma_fence;
    let mut pte: *mut u64;
    let mut r: i32;

    dma_resv_iter_begin(cursor.as_mut_ptr(), bo.tbo.base.resv, DMA_RESV_USAGE_KERNEL);
    dma_resv_for_each_fence_unlocked(cursor.as_mut_ptr(), fence) {
        dma_fence_get(fence);
        r = drm_sched_job_add_dependency(&mut (*p).job.base, fence);
        if r != 0 {
            dma_fence_put(fence);
            dma_resv_iter_end(cursor.as_mut_ptr());
            return r;
        }
    }
    dma_resv_iter_end(cursor.as_mut_ptr());

    loop {
        ndw = (*p).num_dw_left - (*(*p).job).ibs.length_dw;
        if ndw < 32 {
            r = amdgpu_vm_sdma_commit(p, std::ptr::null_mut());
            if r != 0 { return r; }
            r = amdgpu_vm_sdma_alloc_job(p, count, AMDGPU_KERNEL_JOB_ID_VM_UPDATE);
            if r != 0 { return r; }
        }
        if (*p).pages_addr.is_null() {
            if (*p).override_pte {
                amdgpu_gmc_override_vm_pte_flags((*p).adev, (*p).vm, addr, &mut flags);
            }
            amdgpu_vm_sdma_set_ptes(p, bo, pe, addr, count, incr, flags);
            return 0;
        }
        ndw -= (*(*p).adev).vm_manager.vm_pte_funcs.copy_pte_num_dw;
        ndw -= 7;
        nptes = std::cmp::min(count, ndw / 2);
        (*p).num_dw_left -= nptes * 2;
        pte = (*(*p).job).ibs.ptr.add((*p).num_dw_left as usize) as *mut u64;
        i = 0;
        while i < nptes {
            let mut oflags = flags;
            *pte.add(i as usize) = amdgpu_vm_map_gart((*p).pages_addr, addr);
            if (*p).override_pte {
                amdgpu_gmc_override_vm_pte_flags((*p).adev, (*p).vm, *pte.add(i as usize), &mut oflags);
            }
            *pte.add(i as usize) |= oflags;
            i += 1;
            addr += incr as u64;
        }
        amdgpu_vm_sdma_copy_ptes(p, bo, pe, nptes);
        pe += nptes as u64 * 8;
        count -= nptes;
        if count == 0 { break; }
    }
    0
}

pub static amdgpu_vm_sdma_funcs: amdgpu_vm_update_funcs = amdgpu_vm_update_funcs {
    map_table: Some(amdgpu_vm_sdma_map_table),
    prepare: Some(amdgpu_vm_sdma_prepare),
    update: Some(amdgpu_vm_sdma_update),
    commit: Some(amdgpu_vm_sdma_commit),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
