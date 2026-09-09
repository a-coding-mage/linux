// SPDX-License-Identifier: GPL-2.0
/* DMA operations that map physical memory directly without using an IOMMU. */

pub static mut zone_dma_limit: u64 = DMA_BIT_MASK(24);

#[inline]
unsafe fn phys_to_dma_direct(dev: *mut device, phys: phys_addr_t, unencrypted: bool) -> dma_addr_t {
    if unencrypted { phys_to_dma_unencrypted(dev, phys) } else { phys_to_dma_encrypted(dev, phys) }
}

#[inline]
unsafe fn dma_direct_to_page(dev: *mut device, dma_addr: dma_addr_t) -> *mut page {
    pfn_to_page(PHYS_PFN(dma_to_phys(dev, dma_addr)))
}

pub unsafe fn dma_direct_get_required_mask(dev: *mut device) -> u64 {
    let require_decrypted = force_dma_unencrypted(dev);
    let phys = ((max_pfn as phys_addr_t) << PAGE_SHIFT) - 1;
    let max_dma = phys_to_dma_direct(dev, phys, require_decrypted);
    (1u64 << (fls64(max_dma) - 1)) * 2 - 1
}

unsafe fn dma_direct_optimal_gfp_mask(dev: *mut device, phys_limit: *mut u64) -> gfp_t {
    let dma_limit = min_not_zero((*dev).coherent_dma_mask, (*dev).bus_dma_limit);
    *phys_limit = dma_to_phys(dev, dma_limit);
    if *phys_limit <= zone_dma_limit { return GFP_DMA; }
    if *phys_limit <= DMA_BIT_MASK(32) { return GFP_DMA32; }
    0
}

pub unsafe fn dma_coherent_ok(dev: *mut device, phys: phys_addr_t, size: usize) -> bool {
    let dma_addr = phys_to_dma_direct(dev, phys, force_dma_unencrypted(dev));
    if dma_addr == DMA_MAPPING_ERROR { return false; }
    dma_addr + size - 1 <= min_not_zero((*dev).coherent_dma_mask, (*dev).bus_dma_limit)
}

unsafe fn dma_set_decrypted(_dev: *mut device, vaddr: *mut core::ffi::c_void, size: usize) -> i32 {
    let ret = set_memory_decrypted(vaddr as usize, PFN_UP(size));
    if ret != 0 { pr_warn_ratelimited!("leaking DMA memory that can't be decrypted\n"); }
    ret
}
unsafe fn dma_set_encrypted(_dev: *mut device, vaddr: *mut core::ffi::c_void, size: usize) -> i32 {
    let ret = set_memory_encrypted(vaddr as usize, PFN_UP(size));
    if ret != 0 { pr_warn_ratelimited!("leaking DMA memory that can't be re-encrypted\n"); }
    ret
}

unsafe fn dma_direct_alloc_swiotlb(dev: *mut device, size: usize, attrs: usize) -> *mut page {
    let page = swiotlb_alloc(dev, size, attrs);
    if !page.is_null() && !dma_coherent_ok(dev, page_to_phys(page), size) {
        swiotlb_free(dev, page, size); return core::ptr::null_mut();
    }
    page
}

unsafe fn __dma_direct_alloc_pages(dev: *mut device, size: usize, mut gfp: gfp_t, allow_highmem: bool) -> *mut page {
    let node = dev_to_node(dev); let mut phys_limit = 0u64;
    WARN_ON_ONCE!(!PAGE_ALIGNED(size));
    gfp |= dma_direct_optimal_gfp_mask(dev, &mut phys_limit);
    let page = dma_alloc_contiguous(dev, size, gfp);
    if !page.is_null() {
        if dma_coherent_ok(dev, page_to_phys(page), size) && (allow_highmem || !PageHighMem(page)) { return page; }
        dma_free_contiguous(dev, page, size);
    }
    loop {
        let page = alloc_pages_node(node, gfp, get_order(size));
        if page.is_null() { return page; }
        if dma_coherent_ok(dev, page_to_phys(page), size) { return page; }
        __free_pages(page, get_order(size));
        // IS_ENABLED(CONFIG_ZONE_DMA32), IS_ENABLED(CONFIG_ZONE_DMA): build-time conditions.
        if phys_limit < DMA_BIT_MASK(64) && (gfp & (GFP_DMA32 | GFP_DMA)) == 0 { gfp |= GFP_DMA32; }
        else if (gfp & GFP_DMA) == 0 { gfp = (gfp & !GFP_DMA32) | GFP_DMA; }
        else { return core::ptr::null_mut(); }
    }
}

unsafe fn dma_direct_use_pool(dev: *mut device, gfp: gfp_t) -> bool { !gfpflags_allow_blocking(gfp) && !is_swiotlb_for_alloc(dev) }

unsafe fn dma_direct_alloc_from_pool(dev: *mut device, size: usize, dma_handle: *mut dma_addr_t, cpu_addr: *mut *mut core::ffi::c_void, gfp: gfp_t, attrs: usize) -> *mut page {
    let mut phys_limit = 0u64;
    if !IS_ENABLED!(CONFIG_DMA_COHERENT_POOL) { return core::ptr::null_mut(); }
    let page = dma_alloc_from_pool(dev, size, cpu_addr, gfp | dma_direct_optimal_gfp_mask(dev, &mut phys_limit), attrs, dma_coherent_ok);
    if page.is_null() { return page; }
    *dma_handle = phys_to_dma_direct(dev, page_to_phys(page), attrs & __DMA_ATTR_ALLOC_CC_SHARED != 0); page
}

unsafe fn dma_direct_alloc_no_mapping(dev: *mut device, size: usize, dma_handle: *mut dma_addr_t, gfp: gfp_t) -> *mut core::ffi::c_void {
    let page = __dma_direct_alloc_pages(dev, size, gfp & !__GFP_ZERO, true);
    if page.is_null() { return core::ptr::null_mut(); }
    if !PageHighMem(page) { arch_dma_prep_coherent(page, size); }
    *dma_handle = phys_to_dma_encrypted(dev, page_to_phys(page)); page as *mut core::ffi::c_void
}

pub unsafe fn dma_direct_alloc(dev: *mut device, mut size: usize, dma_handle: *mut dma_addr_t, mut gfp: gfp_t, mut attrs: usize) -> *mut core::ffi::c_void {
    let mut remap = false; let mut set_uncached = false; let mut mark_mem_decrypt = false; let mut allow_highmem = true;
    let mut page: *mut page; let mut cpu_addr: *mut core::ffi::c_void;
    if force_dma_unencrypted(dev) { attrs |= __DMA_ATTR_ALLOC_CC_SHARED; }
    if attrs & __DMA_ATTR_ALLOC_CC_SHARED != 0 { allow_highmem = false; mark_mem_decrypt = true; }
    size = PAGE_ALIGN(size); if attrs & DMA_ATTR_NO_WARN != 0 { gfp |= __GFP_NOWARN; }
    if (attrs & (DMA_ATTR_NO_KERNEL_MAPPING | __DMA_ATTR_ALLOC_CC_SHARED)) == DMA_ATTR_NO_KERNEL_MAPPING && !is_swiotlb_for_alloc(dev) { return dma_direct_alloc_no_mapping(dev, size, dma_handle, gfp); }
    if !dev_is_dma_coherent(dev) {
        if IS_ENABLED!(CONFIG_ARCH_HAS_DMA_ALLOC) && !is_swiotlb_for_alloc(dev) { return arch_dma_alloc(dev, size, dma_handle, gfp, attrs); }
        if IS_ENABLED!(CONFIG_DMA_GLOBAL_POOL) { return dma_alloc_from_global_coherent(dev, size, dma_handle); }
        set_uncached = IS_ENABLED!(CONFIG_ARCH_HAS_DMA_SET_UNCACHED); remap = IS_ENABLED!(CONFIG_DMA_DIRECT_REMAP);
        if !set_uncached && !remap { pr_warn_once!("coherent DMA allocations not supported on this platform.\n"); return core::ptr::null_mut(); }
    }
    if (remap || attrs & __DMA_ATTR_ALLOC_CC_SHARED != 0) && dma_direct_use_pool(dev, gfp) {
        page = dma_direct_alloc_from_pool(dev, size, dma_handle, &mut cpu_addr, gfp, attrs); return if !page.is_null() { cpu_addr } else { core::ptr::null_mut() };
    }
    if is_swiotlb_for_alloc(dev) { page = dma_direct_alloc_swiotlb(dev, size, attrs); if page.is_null() { return core::ptr::null_mut(); } mark_mem_decrypt = false; } else { page = __dma_direct_alloc_pages(dev, size, gfp & !__GFP_ZERO, allow_highmem); if page.is_null() { return core::ptr::null_mut(); } }
    if PageHighMem(page) { remap = true; set_uncached = false; }
    if mark_mem_decrypt && set_memory_decrypted(page_address(page) as usize, PFN_UP(size)) != 0 { return core::ptr::null_mut(); }
    if remap { let prot = dma_pgprot(dev, PAGE_KERNEL, attrs); arch_dma_prep_coherent(page, size); cpu_addr = dma_common_contiguous_remap(page, size, prot, builtin_return_address!(0)); if cpu_addr.is_null() { return core::ptr::null_mut(); } } else { cpu_addr = page_address(page); }
    core::ptr::write_bytes(cpu_addr, 0, size);
    if set_uncached { arch_dma_prep_coherent(page, size); let p = arch_dma_set_uncached(cpu_addr, size); if IS_ERR(p) { return core::ptr::null_mut(); } cpu_addr = p; }
    *dma_handle = phys_to_dma_direct(dev, page_to_phys(page), attrs & __DMA_ATTR_ALLOC_CC_SHARED != 0); cpu_addr
}

pub unsafe fn dma_direct_free(dev: *mut device, size: usize, cpu_addr: *mut core::ffi::c_void, dma_addr: dma_addr_t, mut attrs: usize) {
    if force_dma_unencrypted(dev) { attrs |= __DMA_ATTR_ALLOC_CC_SHARED; }
    if (attrs & __DMA_ATTR_ALLOC_CC_SHARED != 0) && ((attrs & DMA_ATTR_NO_KERNEL_MAPPING) != 0) && !is_swiotlb_for_alloc(dev) { dma_free_contiguous(dev, cpu_addr as *mut page, size); return; }
    if IS_ENABLED!(CONFIG_ARCH_HAS_DMA_ALLOC) && !dev_is_dma_coherent(dev) && !is_swiotlb_for_alloc(dev) { arch_dma_free(dev, size, cpu_addr, dma_addr, attrs); return; }
    if IS_ENABLED!(CONFIG_DMA_COHERENT_POOL) && dma_free_from_pool(dev, cpu_addr, PAGE_ALIGN(size)) { return; }
    let phys = dma_to_phys(dev, dma_addr); let pool = swiotlb_find_pool(dev, phys);
    if is_vmalloc_addr(cpu_addr) { vunmap(cpu_addr); } else if IS_ENABLED!(CONFIG_ARCH_HAS_DMA_CLEAR_UNCACHED) { arch_dma_clear_uncached(cpu_addr, size); }
    if attrs & __DMA_ATTR_ALLOC_CC_SHARED != 0 && pool.is_null() && set_memory_encrypted(phys_to_virt(phys) as usize, PFN_UP(size)) != 0 { pr_warn_ratelimited!("leaking DMA memory that can't be re-encrypted\n"); return; }
    if !pool.is_null() { swiotlb_free_from_pool(dev, phys, pool); } else { dma_free_contiguous(dev, dma_direct_to_page(dev, dma_addr), size); }
}

// The remaining entry points retain the kernel ABI and external helper calls.
pub unsafe fn dma_direct_need_sync(dev: *mut device, dma_addr: dma_addr_t) -> bool { !dev_is_dma_coherent(dev) || !swiotlb_find_pool(dev, dma_to_phys(dev, dma_addr)).is_null() }
pub unsafe fn dma_direct_max_mapping_size(dev: *mut device) -> usize { if is_swiotlb_active(dev) && (dma_addressing_limited(dev) || is_swiotlb_force_bounce(dev) || force_dma_unencrypted(dev)) { swiotlb_max_mapping_size(dev) } else { usize::MAX } }

pub unsafe fn dma_direct_alloc_pages(dev: *mut device, size: usize, dma_handle: *mut dma_addr_t, _dir: enum_dma_data_direction, gfp: gfp_t) -> *mut page {
    let mut attrs = 0usize; let mut cpu_addr = core::ptr::null_mut();
    if force_dma_unencrypted(dev) { attrs |= __DMA_ATTR_ALLOC_CC_SHARED; }
    if attrs & __DMA_ATTR_ALLOC_CC_SHARED != 0 && dma_direct_use_pool(dev, gfp) { return dma_direct_alloc_from_pool(dev, size, dma_handle, &mut cpu_addr, gfp, attrs); }
    let page = if is_swiotlb_for_alloc(dev) { dma_direct_alloc_swiotlb(dev, size, attrs) } else { __dma_direct_alloc_pages(dev, size, gfp, false) };
    if page.is_null() { return page; }
    cpu_addr = page_address(page);
    if attrs & __DMA_ATTR_ALLOC_CC_SHARED != 0 && dma_set_decrypted(dev, cpu_addr, size) != 0 { return core::ptr::null_mut(); }
    core::ptr::write_bytes(cpu_addr, 0, size); *dma_handle = phys_to_dma_direct(dev, page_to_phys(page), attrs & __DMA_ATTR_ALLOC_CC_SHARED != 0); page
}

pub unsafe fn dma_direct_free_pages(dev: *mut device, size: usize, page: *mut page, _dma_addr: dma_addr_t, _dir: enum_dma_data_direction) {
    let vaddr = page_address(page); let phys = page_to_phys(page); let mut encrypted = force_dma_unencrypted(dev);
    if IS_ENABLED!(CONFIG_DMA_COHERENT_POOL) && dma_free_from_pool_page(dev, page, size) { return; }
    let pool = swiotlb_find_pool(dev, phys); if !pool.is_null() { encrypted = false; }
    if encrypted && dma_set_encrypted(dev, vaddr, size) != 0 { return; }
    if !pool.is_null() { swiotlb_free_from_pool(dev, phys, pool); } else { dma_free_contiguous(dev, page, size); }
}

pub unsafe fn dma_direct_get_sgtable(dev: *mut device, sgt: *mut sg_table, _cpu_addr: *mut core::ffi::c_void, dma_addr: dma_addr_t, size: usize, _attrs: usize) -> i32 {
    let page = dma_direct_to_page(dev, dma_addr); let ret = sg_alloc_table(sgt, 1, GFP_KERNEL); if ret == 0 { sg_set_page((*sgt).sgl, page, PAGE_ALIGN(size), 0); } ret
}
pub unsafe fn dma_direct_can_mmap(dev: *mut device) -> bool { dev_is_dma_coherent(dev) || IS_ENABLED!(CONFIG_DMA_NONCOHERENT_MMAP) }
pub unsafe fn dma_direct_supported(dev: *mut device, mask: u64) -> i32 { let mut min_mask = ((max_pfn as u64) << PAGE_SHIFT) - 1; if mask >= DMA_BIT_MASK(32) { return 1; } if IS_ENABLED!(CONFIG_ZONE_DMA) { min_mask = core::cmp::min(min_mask, zone_dma_limit); } if mask >= phys_to_dma_unencrypted(dev, min_mask) { 1 } else { 0 } }
pub unsafe fn dma_direct_set_offset(dev: *mut device, cpu_start: phys_addr_t, dma_start: dma_addr_t, size: u64) -> i32 { let offset = cpu_start as u64 - dma_start as u64; if !(*dev).dma_range_map.is_null() { dev_err!(dev, "attempt to add DMA range to existing map\n"); return -EINVAL; } if offset == 0 { return 0; } let map = kzalloc_objs!(*map, 2); if map.is_null() { return -ENOMEM; } (*map).cpu_start = cpu_start; (*map).dma_start = dma_start; (*map).size = size; (*dev).dma_range_map = map; 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
