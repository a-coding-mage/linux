/*
 * Copyright 2008 Advanced Micro Devices, Inc.
 * Copyright 2008 Red Hat Inc.
 * Copyright 2009 Jerome Glisse.
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
 * Authors: Dave Airlie
 *          Alex Deucher
 *          Jerome Glisse
 */

// C headers and build-time configuration dependencies are supplied externally.

/*
 * GART
 * The GART (Graphics Aperture Remapping Table) is an aperture
 * in the GPU's address space.  System pages can be mapped into
 * the aperture and look like contiguous pages from the GPU's
 * perspective.  A page table maps the pages in the aperture
 * to the actual backing pages in system memory.
 *
 * Radeon GPUs support both an internal GART, as described above,
 * and AGP.  AGP works similarly, but the GART table is configured
 * and maintained by the northbridge rather than the driver.
 * Radeon hw has a separate AGP aperture that is programmed to
 * point to the AGP aperture provided by the northbridge and the
 * requests are passed through to the northbridge aperture.
 * Both AGP and internal GART can be used at the same time, however
 * that is not currently supported by the driver.
 *
 * This file handles the common internal GART management.
 */

unsafe fn amdgpu_gart_dummy_page_init(adev: *mut amdgpu_device) -> i32 {
    let dummy_page = ttm_glob.dummy_read_page;
    if (*adev).dummy_page_addr != 0 { return 0; }
    (*adev).dummy_page_addr = dma_map_page_attrs(&mut (*(*adev).pdev).dev, dummy_page, 0, PAGE_SIZE, DMA_BIDIRECTIONAL, DMA_ATTR_SKIP_CPU_SYNC);
    if dma_mapping_error(&mut (*(*adev).pdev).dev, (*adev).dummy_page_addr) {
        dev_err(&mut (*(*adev).pdev).dev, "Failed to DMA MAP the dummy page\n");
        (*adev).dummy_page_addr = 0;
        return -ENOMEM;
    }
    0
}

pub unsafe fn amdgpu_gart_dummy_page_fini(adev: *mut amdgpu_device) {
    if (*adev).dummy_page_addr == 0 { return; }
    dma_unmap_page_attrs(&mut (*(*adev).pdev).dev, (*adev).dummy_page_addr, PAGE_SIZE, DMA_BIDIRECTIONAL, DMA_ATTR_SKIP_CPU_SYNC);
    (*adev).dummy_page_addr = 0;
}

pub unsafe fn amdgpu_gart_table_ram_alloc(adev: *mut amdgpu_device) -> i32 {
    let order = get_order((*adev).gart.table_size);
    let gfp_flags = GFP_KERNEL | __GFP_ZERO;
    let mut bo: *mut amdgpu_bo = core::ptr::null_mut();
    let mut sg: *mut sg_table = core::ptr::null_mut();
    let mut bp: amdgpu_bo_param = core::mem::zeroed();
    let mut dma_addr: dma_addr_t;
    let mut p: *mut page;
    let mut ret: i32;

    if !(*adev).gart.bo.is_null() { return 0; }
    p = alloc_pages(gfp_flags, order);
    if p.is_null() { return -ENOMEM; }
    for x in 0..(1usize << order) { (*p.add(x)).mapping = (*adev).mman.bdev.dev_mapping; }
    dma_addr = dma_map_page(&mut (*(*adev).pdev).dev, p, 0, (*adev).gart.table_size, DMA_BIDIRECTIONAL);
    if dma_mapping_error(&mut (*(*adev).pdev).dev, dma_addr) {
        dev_err((*adev).dev, "Failed to DMA MAP the GART BO page\n");
        __free_pages(p, order); return -EFAULT;
    }
    dev_info((*adev).dev, "%s dma_addr:%pad\n", __func__, &dma_addr);
    sg = kmalloc_obj::<sg_table>();
    if sg.is_null() { ret = -ENOMEM; goto_error!(error); }
    ret = sg_alloc_table(sg, 1, GFP_KERNEL);
    if ret != 0 { goto_error!(error); }
    sg_dma_address((*sg).sgl) = dma_addr;
    (*(*sg).sgl).length = (*adev).gart.table_size;
    #[cfg(CONFIG_NEED_SG_DMA_LENGTH)] { (*(*sg).sgl).dma_length = (*adev).gart.table_size; }
    bp.size = (*adev).gart.table_size; bp.byte_align = PAGE_SIZE;
    bp.domain = AMDGPU_GEM_DOMAIN_CPU; bp.r#type = ttm_bo_type_sg; bp.resv = core::ptr::null_mut();
    bp.bo_ptr_size = core::mem::size_of::<amdgpu_bo>(); bp.flags = 0;
    ret = amdgpu_bo_create(adev, &mut bp, &mut bo); if ret != 0 { goto_error!(error); }
    (*bo).tbo.sg = sg; (*(*bo).tbo.ttm).sg = sg;
    (*bo).allowed_domains = AMDGPU_GEM_DOMAIN_GTT; (*bo).preferred_domains = AMDGPU_GEM_DOMAIN_GTT;
    ret = amdgpu_bo_reserve(bo, true);
    if ret != 0 { dev_err((*adev).dev, "(%d) failed to reserve bo for GART system bo\n", ret); goto_error!(error); }
    ret = amdgpu_bo_pin(bo, AMDGPU_GEM_DOMAIN_GTT); WARN(ret, "Pinning the GART table failed");
    if ret != 0 { amdgpu_bo_unreserve(bo); goto_error!(error); }
    (*adev).gart.bo = bo; (*adev).gart.ptr = page_to_virt(p);
    ret = amdgpu_ttm_alloc_gart(&mut (*(*adev).gart.bo).tbo);
    if ret != 0 { amdgpu_gart_table_ram_free(adev); }
    amdgpu_bo_unreserve(bo); return 0;

    macro_rules! goto_error { ($label:ident) => {{
        amdgpu_bo_unref(&mut bo);
        if !sg.is_null() { sg_free_table(sg); kfree(sg); }
        __free_pages(p, order); return ret;
    }}; }
}

pub unsafe fn amdgpu_gart_table_ram_free(adev: *mut amdgpu_device) {
    let order = get_order((*adev).gart.table_size);
    let sg = (*(*adev).gart.bo).tbo.sg;
    let ret = amdgpu_bo_reserve((*adev).gart.bo, false);
    if ret == 0 { amdgpu_bo_unpin((*adev).gart.bo); amdgpu_bo_unreserve((*adev).gart.bo); }
    amdgpu_bo_unref(&mut (*adev).gart.bo); sg_free_table(sg); kfree(sg);
    let p = virt_to_page((*adev).gart.ptr);
    for x in 0..(1usize << order) { (*p.add(x)).mapping = core::ptr::null_mut(); }
    __free_pages(p, order); (*adev).gart.ptr = core::ptr::null_mut();
}

pub unsafe fn amdgpu_gart_table_vram_alloc(adev: *mut amdgpu_device) -> i32 {
    if !(*adev).gart.bo.is_null() { return 0; }
    let r = amdgpu_bo_create_kernel(adev, (*adev).gart.table_size, PAGE_SIZE, AMDGPU_GEM_DOMAIN_VRAM, &mut (*adev).gart.bo, core::ptr::null_mut(), &mut (*adev).gart.ptr as *mut _ as *mut core::ffi::c_void);
    if r != 0 { return r; }
    memset_io((*adev).gart.ptr, (*adev).gart.gart_pte_flags, (*adev).gart.table_size); 0
}

pub unsafe fn amdgpu_gart_table_vram_free(adev: *mut amdgpu_device) { amdgpu_bo_free_kernel(&mut (*adev).gart.bo, core::ptr::null_mut(), &mut (*adev).gart.ptr as *mut _ as *mut core::ffi::c_void); }

pub unsafe fn amdgpu_gart_unbind(adev: *mut amdgpu_device, offset: u64, pages: i32) {
    let mut t = offset / AMDGPU_GPU_PAGE_SIZE; let flags: u64 = 0; let mut idx = 0;
    if (*adev).gart.ptr.is_null() || !drm_dev_enter(adev_to_drm(adev), &mut idx) { return; }
    for _ in 0..pages { let mut page_base = (*adev).dummy_page_addr; if (*adev).gart.ptr.is_null() { continue; }
        for _ in 0..AMDGPU_GPU_PAGES_IN_CPU_PAGE { amdgpu_gmc_set_pte_pde(adev, (*adev).gart.ptr, t, page_base, flags); page_base += AMDGPU_GPU_PAGE_SIZE; t += 1; }
    }
    amdgpu_gart_invalidate_tlb(adev); drm_dev_exit(idx);
}

pub unsafe fn amdgpu_gart_map(adev: *mut amdgpu_device, offset: u64, pages: i32, dma_addr: *mut dma_addr_t, flags: u64, dst: *mut core::ffi::c_void) {
    let mut t = offset / AMDGPU_GPU_PAGE_SIZE; let mut idx = 0;
    if !drm_dev_enter(adev_to_drm(adev), &mut idx) { return; }
    for i in 0..pages { let mut page_base = *dma_addr.add(i as usize); for _ in 0..AMDGPU_GPU_PAGES_IN_CPU_PAGE { amdgpu_gmc_set_pte_pde(adev, dst, t, page_base, flags); page_base += AMDGPU_GPU_PAGE_SIZE; t += 1; } }
    drm_dev_exit(idx);
}

pub unsafe fn amdgpu_gart_map_vram_range(adev: *mut amdgpu_device, pa: u64, start_page: u64, num_pages: u64, flags: u64, dst: *mut core::ffi::c_void) {
    let mut page_base = pa; let mut t = 0; let mut idx = 0; WARN_ON_ONCE(flags & AMDGPU_PTE_SYSTEM);
    if !drm_dev_enter(adev_to_drm(adev), &mut idx) { return; }
    for _ in 0..num_pages { for _ in 0..AMDGPU_GPU_PAGES_IN_CPU_PAGE { amdgpu_gmc_set_pte_pde(adev, dst, start_page + t, page_base, flags); page_base += AMDGPU_GPU_PAGE_SIZE; t += 1; } }
    drm_dev_exit(idx);
}

pub unsafe fn amdgpu_gart_map_gfx9_mqd(adev: *mut amdgpu_device, offset: u64, pages: i32, dma_addr: *mut dma_addr_t, flags: u64) {
    if (*adev).gart.ptr.is_null() { return; } let mut idx = 0; if !drm_dev_enter(adev_to_drm(adev), &mut idx) { return; }
    let ctrl_flags = AMDGPU_PTE_MTYPE_VG10(flags, AMDGPU_MTYPE_NC); let mut t = offset / AMDGPU_GPU_PAGE_SIZE; let dst = (*adev).gart.ptr;
    for i in 0..pages { let mut page_base = *dma_addr.add(i as usize); for j in 0..AMDGPU_GPU_PAGES_IN_CPU_PAGE { amdgpu_gmc_set_pte_pde(adev, dst, t, page_base, if i == 0 && j == 0 { flags } else { ctrl_flags }); page_base += AMDGPU_GPU_PAGE_SIZE; t += 1; } }
    drm_dev_exit(idx);
}

pub unsafe fn amdgpu_gart_bind(adev: *mut amdgpu_device, offset: u64, pages: i32, dma_addr: *mut dma_addr_t, flags: u64) { if !(*adev).gart.ptr.is_null() { amdgpu_gart_map(adev, offset, pages, dma_addr, flags, (*adev).gart.ptr); } }

pub unsafe fn amdgpu_gart_invalidate_tlb(adev: *mut amdgpu_device) {
    if (*adev).gart.ptr.is_null() { return; } mb();
    if down_read_trylock(&mut (*(*adev).reset_domain).sem) { amdgpu_device_flush_hdp(adev, core::ptr::null_mut()); up_read(&mut (*(*adev).reset_domain).sem); }
    for_each_set_bit!(i, (*adev).vmhubs_mask, AMDGPU_MAX_VMHUBS, { amdgpu_gmc_flush_gpu_tlb(adev, 0, i, 0); });
}

pub unsafe fn amdgpu_gart_init(adev: *mut amdgpu_device) -> i32 {
    if (*adev).dummy_page_addr != 0 { return 0; }
    if PAGE_SIZE < AMDGPU_GPU_PAGE_SIZE { DRM_ERROR("Page size is smaller than GPU page size!\n"); return -EINVAL; }
    let r = amdgpu_gart_dummy_page_init(adev); if r != 0 { return r; }
    (*adev).gart.num_cpu_pages = (*adev).gmc.gart_size / PAGE_SIZE;
    (*adev).gart.num_gpu_pages = (*adev).gmc.gart_size / AMDGPU_GPU_PAGE_SIZE;
    drm_info(adev_to_drm(adev), "GART: num cpu pages %u, num gpu pages %u\n", (*adev).gart.num_cpu_pages, (*adev).gart.num_gpu_pages); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
