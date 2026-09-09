// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2006 Benjamin Herrenschmidt, IBM Corporation
 *
 * Provide default implementations of the DMA mapping callbacks for
 * busses using the iommu infrastructure
 */

// Dependencies supplied by the surrounding kernel translation.

#[cfg(CONFIG_ARCH_HAS_DMA_MAP_DIRECT)]
#[inline]
unsafe fn can_map_direct(dev: *mut device, addr: phys_addr_t) -> bool {
    (*dev).bus_dma_limit >= phys_to_dma(dev, addr)
}

#[cfg(CONFIG_ARCH_HAS_DMA_MAP_DIRECT)]
pub unsafe fn arch_dma_map_phys_direct(dev: *mut device, addr: phys_addr_t) -> bool {
    if (*dev).bus_dma_limit == 0 {
        return false;
    }
    can_map_direct(dev, addr)
}

#[cfg(CONFIG_ARCH_HAS_DMA_MAP_DIRECT)]
#[inline]
unsafe fn is_direct_handle(dev: *mut device, h: dma_addr_t) -> bool {
    h >= (*dev).archdata.dma_offset
}

#[cfg(CONFIG_ARCH_HAS_DMA_MAP_DIRECT)]
pub unsafe fn arch_dma_unmap_phys_direct(dev: *mut device, dma_handle: dma_addr_t) -> bool {
    if (*dev).bus_dma_limit == 0 {
        return false;
    }
    is_direct_handle(dev, dma_handle)
}

#[cfg(CONFIG_ARCH_HAS_DMA_MAP_DIRECT)]
pub unsafe fn arch_dma_map_sg_direct(
    dev: *mut device,
    sg: *mut scatterlist,
    nents: c_int,
) -> bool {
    if (*dev).bus_dma_limit == 0 {
        return false;
    }
    let mut s = sg;
    for _ in 0..nents {
        if !can_map_direct(dev, sg_phys(s) + (*s).offset + (*s).length) {
            return false;
        }
        s = (*s).next;
    }
    true
}

#[cfg(CONFIG_ARCH_HAS_DMA_MAP_DIRECT)]
pub unsafe fn arch_dma_unmap_sg_direct(
    dev: *mut device,
    sg: *mut scatterlist,
    nents: c_int,
) -> bool {
    if (*dev).bus_dma_limit == 0 {
        return false;
    }
    let mut s = sg;
    for _ in 0..nents {
        if !is_direct_handle(dev, (*s).dma_address + (*s).length) {
            return false;
        }
        s = (*s).next;
    }
    true
}

#[cfg(CONFIG_ARCH_HAS_DMA_MAP_DIRECT)]
pub unsafe fn arch_dma_alloc_direct(dev: *mut device) -> bool {
    if dev_dma_ops_bypass(dev) && (*dev).bus_dma_limit != 0 {
        return true;
    }
    false
}

#[cfg(CONFIG_ARCH_HAS_DMA_MAP_DIRECT)]
pub unsafe fn arch_dma_free_direct(dev: *mut device, dma_handle: dma_addr_t) -> bool {
    if !dev_dma_ops_bypass(dev) || (*dev).bus_dma_limit == 0 {
        return false;
    }
    is_direct_handle(dev, dma_handle)
}

/* Generic iommu implementation */

/* Allocates a contiguous real buffer and creates mappings over it.
 * Returns the virtual address of the buffer and sets dma_handle
 * to the dma address (mapping) of the first page.
 */
unsafe fn dma_iommu_alloc_coherent(
    dev: *mut device, size: usize, dma_handle: *mut dma_addr_t,
    flag: gfp_t, attrs: c_ulong,
) -> *mut c_void {
    iommu_alloc_coherent(dev, get_iommu_table_base(dev), size, dma_handle,
                         (*dev).coherent_dma_mask, flag, dev_to_node(dev))
}

unsafe fn dma_iommu_free_coherent(
    dev: *mut device, size: usize, vaddr: *mut c_void,
    dma_handle: dma_addr_t, _attrs: c_ulong,
) {
    iommu_free_coherent(get_iommu_table_base(dev), size, vaddr, dma_handle);
}

/* Creates TCEs for a user provided buffer.  The user buffer must be
 * contiguous real kernel storage (not vmalloc).  The address passed here
 * is a physical address to that page. The dma_addr_t returned will point
 * to the same byte within the page as was passed in.
 */
unsafe fn dma_iommu_map_phys(
    dev: *mut device, phys: phys_addr_t, size: usize,
    direction: dma_data_direction, attrs: c_ulong,
) -> dma_addr_t {
    iommu_map_phys(dev, get_iommu_table_base(dev), phys, size,
                   dma_get_mask(dev), direction, attrs)
}

unsafe fn dma_iommu_unmap_phys(
    dev: *mut device, dma_handle: dma_addr_t, size: usize,
    direction: dma_data_direction, attrs: c_ulong,
) {
    iommu_unmap_phys(get_iommu_table_base(dev), dma_handle, size, direction, attrs);
}

unsafe fn dma_iommu_map_sg(
    dev: *mut device, sglist: *mut scatterlist, nelems: c_int,
    direction: dma_data_direction, attrs: c_ulong,
) -> c_int {
    ppc_iommu_map_sg(dev, get_iommu_table_base(dev), sglist, nelems,
                     dma_get_mask(dev), direction, attrs)
}

unsafe fn dma_iommu_unmap_sg(
    dev: *mut device, sglist: *mut scatterlist, nelems: c_int,
    direction: dma_data_direction, attrs: c_ulong,
) {
    ppc_iommu_unmap_sg(get_iommu_table_base(dev), sglist, nelems, direction, attrs);
}

unsafe fn dma_iommu_bypass_supported(dev: *mut device, mask: u64) -> bool {
    let pdev = to_pci_dev(dev);
    let phb = pci_bus_to_host((*pdev).bus);
    let ops = (*phb).controller_ops;
    match ops.iommu_bypass_supported {
        None => false,
        Some(f) => f(pdev, mask),
    }
}

/* We support DMA to/from any memory page via the iommu */
pub unsafe fn dma_iommu_dma_supported(dev: *mut device, mask: u64) -> c_int {
    let mut tbl: *mut iommu_table;

    if dev_is_pci(dev) && dma_iommu_bypass_supported(dev, mask) {
        /* fixed ops will be used for RAM. This is limited by
         * bus_dma_limit which is set when RAM is pre-mapped.
         */
        dev_set_dma_ops_bypass(dev);
        dev_info(dev, "iommu: 64-bit OK but direct DMA is limited by %llx\n", (*dev).bus_dma_limit);
        return 1;
    }

    tbl = get_iommu_table_base(dev);
    if tbl.is_null() {
        dev_err(dev, "Warning: IOMMU dma not supported: mask 0x%08llx, table unavailable\n", mask);
        return 0;
    }
    if (*tbl).it_offset > (mask >> (*tbl).it_page_shift) {
        dev_info(dev, "Warning: IOMMU offset too big for device mask\n");
        dev_info(dev, "mask: 0x%08llx, table offset: 0x%08lx\n", mask,
                 (*tbl).it_offset << (*tbl).it_page_shift);
        return 0;
    }
    dev_dbg(dev, "iommu: not 64-bit, using default ops\n");
    dev_clear_dma_ops_bypass(dev);
    1
}

pub unsafe fn dma_iommu_get_required_mask(dev: *mut device) -> u64 {
    let tbl = get_iommu_table_base(dev);
    let mut mask: u64;
    if dev_is_pci(dev) {
        let bypass_mask = dma_direct_get_required_mask(dev);
        if dma_iommu_dma_supported(dev, bypass_mask) {
            dev_info(dev, "%s: returning bypass mask 0x%llx\n", __func__, bypass_mask);
            return bypass_mask;
        }
    }
    if tbl.is_null() { return 0; }
    mask = 1u64 << (fls_long((*tbl).it_offset + (*tbl).it_size)
                    + (*tbl).it_page_shift - 1);
    mask += mask - 1;
    mask
}

pub static dma_iommu_ops: dma_map_ops = dma_map_ops {
    alloc: Some(dma_iommu_alloc_coherent),
    free: Some(dma_iommu_free_coherent),
    map_sg: Some(dma_iommu_map_sg),
    unmap_sg: Some(dma_iommu_unmap_sg),
    dma_supported: Some(dma_iommu_dma_supported),
    map_phys: Some(dma_iommu_map_phys),
    unmap_phys: Some(dma_iommu_unmap_phys),
    get_required_mask: Some(dma_iommu_get_required_mask),
    mmap: Some(dma_common_mmap),
    get_sgtable: Some(dma_common_get_sgtable),
    alloc_pages_op: Some(dma_common_alloc_pages),
    free_pages: Some(dma_common_free_pages),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
