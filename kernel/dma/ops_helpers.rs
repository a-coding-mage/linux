// SPDX-License-Identifier: GPL-2.0
/*
 * Helpers for DMA ops implementations.  These generally rely on the fact that
 * the allocated memory contains normal pages in the direct kernel mapping.
 */

unsafe fn dma_common_vaddr_to_page(cpu_addr: *mut core::ffi::c_void) -> *mut page {
    if is_vmalloc_addr(cpu_addr) {
        return vmalloc_to_page(cpu_addr);
    }
    virt_to_page(cpu_addr)
}

/*
 * Create scatter-list for the already allocated DMA buffer.
 */
pub unsafe fn dma_common_get_sgtable(
    dev: *mut device,
    sgt: *mut sg_table,
    cpu_addr: *mut core::ffi::c_void,
    dma_addr: dma_addr_t,
    size: usize,
    attrs: libc::c_ulong,
) -> libc::c_int {
    let page = dma_common_vaddr_to_page(cpu_addr);
    let mut ret: libc::c_int;

    ret = sg_alloc_table(sgt, 1, GFP_KERNEL);
    if ret == 0 {
        sg_set_page((*sgt).sgl, page, PAGE_ALIGN(size), 0);
    }
    ret
}

/*
 * Create userspace mapping for the DMA-coherent memory.
 */
pub unsafe fn dma_common_mmap(
    dev: *mut device,
    vma: *mut vm_area_struct,
    cpu_addr: *mut core::ffi::c_void,
    dma_addr: dma_addr_t,
    size: usize,
    attrs: libc::c_ulong,
) -> libc::c_int {
    // CONFIG_MMU conditional preserved from the C source.
    #[cfg(CONFIG_MMU)]
    {
        let user_count: libc::c_ulong = vma_pages(vma);
        let count: libc::c_ulong = PAGE_ALIGN(size) >> PAGE_SHIFT;
        let off: libc::c_ulong = vma_start_pgoff(vma);
        let page = dma_common_vaddr_to_page(cpu_addr);
        let mut ret: libc::c_int = -ENXIO;

        (*vma).vm_page_prot = dma_pgprot(dev, vma.vm_page_prot, attrs);

        if dma_mmap_from_dev_coherent(dev, vma, cpu_addr, size, &mut ret) {
            return ret;
        }

        if off >= count || user_count > count - off {
            return -ENXIO;
        }

        return remap_pfn_range(
            vma,
            (*vma).vm_start,
            page_to_pfn(page) + vma_start_pgoff(vma),
            user_count << PAGE_SHIFT,
            (*vma).vm_page_prot,
        );
    }

    // CONFIG_MMU disabled.
    -ENXIO
}

pub unsafe fn dma_common_alloc_pages(
    dev: *mut device,
    size: usize,
    dma_handle: *mut dma_addr_t,
    dir: dma_data_direction,
    gfp: gfp_t,
) -> *mut page {
    let ops: *const dma_map_ops = get_dma_ops(dev);
    let mut page: *mut page;
    let phys: phys_addr_t;

    page = dma_alloc_contiguous(dev, size, gfp);
    if page.is_null() {
        page = alloc_pages_node(dev_to_node(dev), gfp, get_order(size));
    }
    if page.is_null() {
        return core::ptr::null_mut();
    }

    phys = page_to_phys(page);
    if use_dma_iommu(dev) {
        *dma_handle = iommu_dma_map_phys(dev, phys, size, dir, DMA_ATTR_SKIP_CPU_SYNC);
    } else {
        *dma_handle = ((*ops).map_phys)(dev, phys, size, dir, DMA_ATTR_SKIP_CPU_SYNC);
    }
    if *dma_handle == DMA_MAPPING_ERROR {
        dma_free_contiguous(dev, page, size);
        return core::ptr::null_mut();
    }

    memset(page_address(page), 0, size);
    page
}

pub unsafe fn dma_common_free_pages(
    dev: *mut device,
    size: usize,
    page: *mut page,
    dma_handle: dma_addr_t,
    dir: dma_data_direction,
) {
    let ops: *const dma_map_ops = get_dma_ops(dev);

    if use_dma_iommu(dev) {
        iommu_dma_unmap_phys(dev, dma_handle, size, dir, DMA_ATTR_SKIP_CPU_SYNC);
    } else if !(*ops).unmap_phys.is_none() {
        ((*ops).unmap_phys.unwrap())(dev, dma_handle, size, dir, DMA_ATTR_SKIP_CPU_SYNC);
    }
    dma_free_contiguous(dev, page, size);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
