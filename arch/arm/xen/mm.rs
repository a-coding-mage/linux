// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the corresponding Linux, Xen, and architecture headers.

unsafe fn xen_swiotlb_gfp() -> gfp_t {
    let mut base: phys_addr_t;
    let mut i: u64;

    // C: for_each_mem_range(i, &base, NULL)
    for_each_mem_range!(i, &mut base, None, {
        if base < 0xffffffff as phys_addr_t {
            if IS_ENABLED!(CONFIG_ZONE_DMA32) {
                return __GFP_DMA32;
            }
            return __GFP_DMA;
        }
    });

    GFP_KERNEL
}

static mut hypercall_cflush: bool = false;

/* buffers in highmem or foreign pages cannot cross page boundaries */
unsafe fn dma_cache_maint(
    dev: *mut device,
    mut handle: dma_addr_t,
    mut size: usize,
    op: u32,
) {
    let mut cflush: gnttab_cache_flush;

    cflush.offset = xen_offset_in_page(handle);
    cflush.op = op;
    handle &= XEN_PAGE_MASK;

    loop {
        cflush.a.dev_bus_addr = dma_to_phys(dev, handle);

        if size + cflush.offset > XEN_PAGE_SIZE {
            cflush.length = XEN_PAGE_SIZE - cflush.offset;
        } else {
            cflush.length = size;
        }

        HYPERVISOR_grant_table_op(GNTTABOP_cache_flush, &mut cflush, 1);

        cflush.offset = 0;
        handle += cflush.length;
        size -= cflush.length;
        if size == 0 {
            break;
        }
    }
}

/*
 * Dom0 is mapped 1:1, and while the Linux page can span across multiple Xen
 * pages, it is not possible for it to contain a mix of local and foreign Xen
 * pages.  Calling pfn_valid on a foreign mfn will always return false, so if
 * pfn_valid returns true the pages is local and we can use the native
 * dma-direct functions, otherwise we call the Xen specific version.
 */
unsafe fn xen_dma_sync_for_cpu(
    dev: *mut device,
    handle: dma_addr_t,
    size: usize,
    dir: dma_data_direction,
) {
    if dir != DMA_TO_DEVICE {
        dma_cache_maint(dev, handle, size, GNTTAB_CACHE_INVAL);
    }
}

unsafe fn xen_dma_sync_for_device(
    dev: *mut device,
    handle: dma_addr_t,
    size: usize,
    dir: dma_data_direction,
) {
    if dir == DMA_FROM_DEVICE {
        dma_cache_maint(dev, handle, size, GNTTAB_CACHE_INVAL);
    } else {
        dma_cache_maint(dev, handle, size, GNTTAB_CACHE_CLEAN);
    }
}

unsafe fn xen_arch_need_swiotlb(
    dev: *mut device,
    phys: phys_addr_t,
    dev_addr: dma_addr_t,
) -> bool {
    let xen_pfn: c_uint = XEN_PFN_DOWN(phys);
    let bfn: c_uint = XEN_PFN_DOWN(dma_to_phys(dev, dev_addr));

    /*
     * The swiotlb buffer should be used if
     *     - Xen doesn't have the cache flush hypercall
     *     - The Linux page refers to foreign memory
     *     - The device doesn't support coherent DMA request
     *
     * The Linux page may be spanned acrros multiple Xen page, although
     * it's not possible to have a mix of local and foreign Xen page.
     * Furthermore, range_straddles_page_boundary is already checking
     * if buffer is physically contiguous in the host RAM.
     *
     * Therefore we only need to check the first Xen page to know if
     * we require a bounce buffer because the device doesn't support coherent
     * memory and we are not able to flush the cache.
     */
    (!hypercall_cflush && (xen_pfn != bfn) && !dev_is_dma_coherent(dev))
}

unsafe fn xen_mm_init() -> c_int {
    let mut cflush: gnttab_cache_flush;
    let rc: c_int;

    if !xen_swiotlb_detect() {
        return 0;
    }

    /* we can work with the default swiotlb */
    rc = swiotlb_init_late(swiotlb_size_or_default(), xen_swiotlb_gfp(), None);
    if rc < 0 {
        return rc;
    }

    cflush.op = 0;
    cflush.a.dev_bus_addr = 0;
    cflush.offset = 0;
    cflush.length = 0;
    if HYPERVISOR_grant_table_op(GNTTABOP_cache_flush, &mut cflush, 1) != -ENOSYS {
        hypercall_cflush = true;
    }
    0
}

// arch_initcall(xen_mm_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
