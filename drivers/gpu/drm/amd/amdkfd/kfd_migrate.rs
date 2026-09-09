// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Copyright 2020-2021 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// Linux and AMDGPU declarations are supplied by the surrounding kernel port.

unsafe fn svm_migrate_direct_mapping_addr(adev: *mut amdgpu_device, addr: u64) -> u64 {
    addr.wrapping_add(amdgpu_ttm_domain_start(adev, TTM_PL_VRAM))
}

unsafe fn svm_migrate_gart_map(
    ring: *mut amdgpu_ring, entity: *mut amdgpu_ttm_buffer_entity,
    npages: u64, addr: *mut dma_addr_t, gart_addr: *mut u64, flags: u64,
) -> i32 {
    let adev = (*ring).adev;
    let mut job: *mut amdgpu_job = core::ptr::null_mut();
    let num_dw = ALIGN((*(*adev).mman.buffer_funcs).copy_num_dw, 8);
    let num_bytes = npages * 8 * AMDGPU_GPU_PAGES_IN_CPU_PAGE;
    let mut fence: *mut dma_fence;
    *gart_addr = amdgpu_compute_gart_address(&(*adev).gmc, entity, 0);
    let r = amdgpu_job_alloc_with_ib(adev, &mut (*entity).base,
        AMDGPU_FENCE_OWNER_UNDEFINED, num_dw * 4 + num_bytes,
        AMDGPU_IB_POOL_DELAYED, AMDGPU_KERNEL_JOB_ID_KFD_GART_MAP, &mut job);
    if r != 0 { return r; }
    let src_addr = num_dw * 4 + (*job).ibs[0].gpu_addr;
    let dst_addr = amdgpu_bo_gpu_offset((*adev).gart.bo)
        + (((*entity).gart_window_offs[0] >> AMDGPU_GPU_PAGE_SHIFT) * 8);
    amdgpu_emit_copy_buffer(adev, &mut (*job).ibs[0], src_addr, dst_addr, num_bytes, 0);
    amdgpu_ring_pad_ib(ring, &mut (*job).ibs[0]);
    WARN_ON((*job).ibs[0].length_dw > num_dw);
    let mut pte_flags = AMDGPU_PTE_VALID | AMDGPU_PTE_READABLE;
    pte_flags |= AMDGPU_PTE_SYSTEM | AMDGPU_PTE_SNOOPED;
    if flags & KFD_IOCTL_SVM_FLAG_GPU_RO == 0 { pte_flags |= AMDGPU_PTE_WRITEABLE; }
    pte_flags |= (*adev).gart.gart_pte_flags;
    let cpu_addr = (*job).ibs[0].ptr.add(num_dw) as *mut core::ffi::c_void;
    amdgpu_gart_map(adev, 0, npages, addr, pte_flags, cpu_addr);
    fence = amdgpu_job_submit(job);
    dma_fence_put(fence);
    r
}

unsafe fn svm_migrate_copy_memory_gart(
    adev: *mut amdgpu_device, sys: *mut dma_addr_t, vram: *mut u64,
    mut npages: u64, direction: MIGRATION_COPY_DIR,
    mfence: *mut *mut dma_fence,
) -> i32 {
    let gtt_max_pages = AMDGPU_GTT_MAX_TRANSFER_SIZE >> PAGE_SHIFT;
    let ring = to_amdgpu_ring((*adev).mman.buffer_funcs_scheds[0]);
    let entity = &mut (*adev).mman.move_entities[0] as *mut _;
    let mut r = 0;
    mutex_lock(&mut (*entity).lock);
    while npages != 0 {
        let size = core::cmp::min(gtt_max_pages, npages);
        let (gart_s, gart_d);
        if direction == FROM_VRAM_TO_RAM {
            gart_s = svm_migrate_direct_mapping_addr(adev, *vram);
            r = svm_migrate_gart_map(ring, entity, size, sys, &mut gart_d, 0);
        } else {
            r = svm_migrate_gart_map(ring, entity, size, sys, &mut gart_s,
                                     KFD_IOCTL_SVM_FLAG_GPU_RO);
            gart_d = svm_migrate_direct_mapping_addr(adev, *vram);
        }
        if r != 0 { dev_err((*adev).dev, "fail %d create gart mapping\n", r); break; }
        let mut next = core::ptr::null_mut();
        r = amdgpu_copy_buffer(adev, entity, gart_s, gart_d, size * PAGE_SIZE,
                               core::ptr::null_mut(), &mut next, true, 0);
        if r != 0 { dev_err((*adev).dev, "fail %d to copy memory\n", r); break; }
        dma_fence_put(*mfence); *mfence = next;
        npages -= size;
        if npages != 0 { sys = sys.add(size as usize); vram = vram.add(size as usize); }
    }
    mutex_unlock(&mut (*entity).lock);
    r
}

unsafe fn svm_migrate_copy_done(_adev: *mut amdgpu_device, mfence: *mut dma_fence) -> i32 {
    if !mfence.is_null() { let r = dma_fence_wait(mfence, false); dma_fence_put(mfence); pr_debug!("sdma copy memory fence done\n"); r } else { 0 }
}

pub unsafe fn svm_migrate_addr_to_pfn(adev: *mut amdgpu_device, addr: usize) -> usize {
    (addr + (*adev).kfd.pgmap.range.start as usize) >> PAGE_SHIFT
}

unsafe fn svm_migrate_get_vram_page(prange: *mut svm_range, pfn: usize) {
    let page = pfn_to_page(pfn);
    svm_range_bo_ref((*prange).svm_bo);
    (*page).zone_device_data = (*prange).svm_bo as *mut _;
    zone_device_page_init(page, page_pgmap(page), 0);
}

unsafe fn svm_migrate_put_vram_page(adev: *mut amdgpu_device, addr: usize) {
    let page = pfn_to_page(svm_migrate_addr_to_pfn(adev, addr));
    unlock_page(page); put_page(page);
}

unsafe fn svm_migrate_addr(adev: *mut amdgpu_device, page: *mut page) -> usize {
    (page_to_pfn(page) << PAGE_SHIFT) - (*adev).kfd.pgmap.range.start as usize
}

unsafe fn svm_migrate_get_sys_page(vma: *mut vm_area_struct, addr: usize) -> *mut page {
    let page = alloc_page_vma(GFP_HIGHUSER, vma, addr);
    if !page.is_null() { lock_page(page); }
    page
}

unsafe fn svm_migrate_successful_pages(migrate: *mut migrate_vma) -> usize {
    let mut n = 0;
    for i in 0..(*migrate).npages as usize {
        if (*migrate).dst.add(i).read() & MIGRATE_PFN_VALID != 0 &&
           (*migrate).src.add(i).read() & MIGRATE_PFN_MIGRATE != 0 { n += 1; }
    }
    n
}

// The remaining migration callbacks retain the kernel implementation's
// externally supplied structures and helpers; their control flow is kept
// literal, with pointer arithmetic and synchronization expressed in unsafe Rust.
pub unsafe fn svm_migrate_to_vram(prange: *mut svm_range, best_loc: u32,
    start: usize, last: usize, mm: *mut mm_struct, trigger: u32) -> i32 {
    if (*prange).actual_loc == 0 || (*prange).actual_loc == best_loc {
        svm_migrate_ram_to_vram(prange, best_loc, start, last, mm, trigger)
    } else {
        svm_migrate_vram_to_vram(prange, best_loc, start, last, mm, trigger)
    }
}

pub unsafe fn svm_migrate_vram_to_ram(prange: *mut svm_range, mm: *mut mm_struct,
    start_mgr: usize, last_mgr: usize, trigger: u32, fault_page: *mut page) -> i32 {
    if (*prange).actual_loc == 0 { return 0; }
    if start_mgr < (*prange).start || last_mgr > (*prange).last { return -EFAULT; }
    let node = svm_range_get_node_by_id(prange, (*prange).actual_loc);
    if node.is_null() { return -ENODEV; }
    let mut addr = start_mgr << PAGE_SHIFT;
    let end = (last_mgr + 1) << PAGE_SHIFT;
    let mut r = 0;
    while addr < end {
        let vma = vma_lookup(mm, addr);
        if vma.is_null() { r = -EFAULT; break; }
        let next = core::cmp::min((*vma).vm_end as usize, end);
        let x = svm_migrate_vma_to_ram(node, prange, vma, addr, next, trigger, fault_page);
        if x < 0 { r = x; break; }
        addr = next;
    }
    if r < 0 { r } else { 0 }
}

// External kernel declarations intentionally remain unresolved here.
extern "C" {
    fn svm_migrate_ram_to_vram(*mut svm_range, u32, usize, usize, *mut mm_struct, u32) -> i32;
    fn svm_migrate_vram_to_vram(*mut svm_range, u32, usize, usize, *mut mm_struct, u32) -> i32;
    fn svm_migrate_vma_to_ram(*mut kfd_node, *mut svm_range, *mut vm_area_struct, u64, u64, u32, *mut page) -> i32;
    fn amdgpu_ttm_domain_start(*mut amdgpu_device, u32) -> u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
