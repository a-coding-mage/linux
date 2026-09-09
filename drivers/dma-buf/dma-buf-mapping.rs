// SPDX-License-Identifier: GPL-2.0-only
/*
 * DMA BUF Mapping Helpers
 *
 */

// Dependencies supplied by the Linux DMA-buf and DMA mapping environment.

unsafe fn fill_sg_entry(
    mut sgl: *mut scatterlist,
    mut length: usize,
    addr: dma_addr_t,
) -> *mut scatterlist {
    let nents = (length + (u32::MAX as usize) - 1) / (u32::MAX as usize);
    for i in 0..nents {
        let len = core::cmp::min(length, u32::MAX as usize);
        length -= len;
        /*
         * DMABUF abuses scatterlist to create a scatterlist
         * that does not have any CPU list, only the DMA list.
         * Always set the page related values to NULL to ensure
         * importers can't use it. The phys_addr based DMA API
         * does not require the CPU list for mapping or unmapping.
         */
        sg_set_page(sgl, core::ptr::null_mut(), 0, 0);
        sg_dma_address(sgl, addr.wrapping_add((i as dma_addr_t).wrapping_mul(u32::MAX as dma_addr_t)));
        sg_dma_len(sgl, len);
        sgl = sg_next(sgl);
    }
    sgl
}

unsafe fn calc_sg_nents(
    state: *mut dma_iova_state,
    phys_vec: *mut phys_vec,
    nr_ranges: usize,
    size: usize,
) -> u32 {
    let mut nents: u32 = 0;
    if state.is_null() || !dma_use_iova(state) {
        for i in 0..nr_ranges {
            let len = (*phys_vec.add(i)).len;
            nents = nents.wrapping_add(((len + (u32::MAX as usize) - 1) / (u32::MAX as usize)) as u32);
        }
    } else {
        /*
         * In IOVA case, there is only one SG entry which spans
         * for whole IOVA address space, but we need to make sure
         * that it fits sg->length, maybe we need more.
         */
        nents = ((size + (u32::MAX as usize) - 1) / (u32::MAX as usize)) as u32;
    }
    nents
}

/**
 * struct dma_buf_dma - holds DMA mapping information
 * @sgt:    Scatter-gather table
 * @state:  DMA IOVA state relevant in IOMMU-based DMA
 * @size:   Total size of DMA transfer
 */
#[repr(C)]
struct dma_buf_dma {
    sgt: sg_table,
    state: *mut dma_iova_state,
    size: usize,
}

/**
 * dma_buf_phys_vec_to_sgt - Returns the scatterlist table of the attachment
 * from arrays of physical vectors. This funciton is intended for MMIO memory
 * only.
 */
pub unsafe fn dma_buf_phys_vec_to_sgt(
    attach: *mut dma_buf_attachment,
    provider: *mut p2pdma_provider,
    phys_vec: *mut phys_vec,
    nr_ranges: usize,
    size: usize,
    dir: dma_data_direction,
) -> *mut sg_table {
    dma_resv_assert_held((*(*attach).dmabuf).resv);

    if attach.is_null() || (*attach).dmabuf.is_null() || provider.is_null() {
        // This function is supposed to work on MMIO memory only
        return ERR_PTR(-EINVAL);
    }

    let dma = kzalloc_obj_dma_buf_dma();
    if dma.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    let mut ret: i32;
    match pci_p2pdma_map_type(provider, (*attach).dev) {
        PCI_P2PDMA_MAP_BUS_ADDR => {}
        PCI_P2PDMA_MAP_THRU_HOST_BRIDGE => {
            (*dma).state = kzalloc_obj_dma_iova_state();
            if (*dma).state.is_null() {
                ret = -ENOMEM;
                goto_free_dma(dma, ret);
            }
            dma_iova_try_alloc((*attach).dev, (*dma).state, 0, size);
        }
        _ => {
            ret = -EINVAL;
            goto_free_dma(dma, ret);
        }
    }

    let nents = calc_sg_nents((*dma).state, phys_vec, nr_ranges, size);
    ret = sg_alloc_table(&mut (*dma).sgt, nents, GFP_KERNEL | __GFP_ZERO);
    if ret != 0 {
        kfree((*dma).state);
        return goto_free_dma(dma, ret);
    }

    let mut sgl = (*dma).sgt.sgl;
    let mut mapped_len: usize = 0;
    let mut i: usize = 0;
    while i < nr_ranges {
        let entry = &*phys_vec.add(i);
        let addr: dma_addr_t;
        if (*dma).state.is_null() {
            addr = pci_p2pdma_bus_addr_map(provider, entry.paddr);
        } else if dma_use_iova((*dma).state) {
            ret = dma_iova_link((*attach).dev, (*dma).state, entry.paddr, 0, entry.len, dir, DMA_ATTR_MMIO);
            if ret != 0 { goto_unmap_dma(dma, attach, dir, i, mapped_len, sgl, ret); }
            mapped_len += entry.len;
            i += 1;
            continue;
        } else {
            addr = dma_map_phys((*attach).dev, entry.paddr, entry.len, dir, DMA_ATTR_MMIO);
            ret = dma_mapping_error((*attach).dev, addr);
            if ret != 0 { goto_unmap_dma(dma, attach, dir, i, mapped_len, sgl, ret); }
        }
        sgl = fill_sg_entry(sgl, entry.len, addr);
        i += 1;
    }

    if !(*dma).state.is_null() && dma_use_iova((*dma).state) {
        WARN_ON_ONCE(mapped_len != size);
        ret = dma_iova_sync((*attach).dev, (*dma).state, 0, mapped_len);
        if ret != 0 { goto_unmap_dma(dma, attach, dir, i, mapped_len, sgl, ret); }
        sgl = fill_sg_entry(sgl, mapped_len, (*(*dma).state).addr);
    }

    (*dma).size = size;
    // No CPU list included — set orig_nents = 0 so others can detect this via SG table (use nents only).
    (*dma).sgt.orig_nents = 0;
    // SGL must be NULL to indicate that SGL is the last one and we allocated correct number of entries.
    WARN_ON_ONCE(!sgl.is_null());
    &mut (*dma).sgt
}

/** dma_buf_free_sgt- unmaps the buffer */
pub unsafe fn dma_buf_free_sgt(
    attach: *mut dma_buf_attachment,
    sgt: *mut sg_table,
    dir: dma_data_direction,
) {
    let dma = container_of_dma_buf_dma(sgt);
    dma_resv_assert_held((*(*attach).dmabuf).resv);
    if !(*dma).state.is_null() {
        if dma_use_iova((*dma).state) {
            dma_iova_destroy((*attach).dev, (*dma).state, (*dma).size, dir, DMA_ATTR_MMIO);
        } else {
            let mut sgl: *mut scatterlist = core::ptr::null_mut();
            let mut i = 0;
            for_each_sgtable_dma_sg(sgt, sgl, i) {
                dma_unmap_phys((*attach).dev, sg_dma_address_value(sgl), sg_dma_len_value(sgl), dir, DMA_ATTR_MMIO);
            }
        }
    }
    sg_free_table(sgt);
    kfree((*dma).state);
    kfree(dma);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
