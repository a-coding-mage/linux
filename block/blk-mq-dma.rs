// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2025 Christoph Hellwig
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/blk-integrity.h, linux/blk-mq-dma.h, and blk.h

unsafe fn __blk_map_iter_next(iter: *mut blk_map_iter) -> bool {
    if (*iter).iter.bi_size != 0 {
        return true;
    }
    if (*iter).bio.is_null() || (*(*iter).bio).bi_next.is_null() {
        return false;
    }

    (*iter).bio = (*(*iter).bio).bi_next;
    if (*iter).is_integrity {
        (*iter).iter = (*bio_integrity((*iter).bio)).bip_iter;
        (*iter).bvecs = (*bio_integrity((*iter).bio)).bip_vec;
    } else {
        (*iter).iter = (*(*iter).bio).bi_iter;
        (*iter).bvecs = (*(*iter).bio).bi_io_vec;
    }
    true
}

unsafe fn blk_map_iter_next(
    req: *mut request,
    iter: *mut blk_map_iter,
    vec: *mut phys_vec,
) -> bool {
    let mut max_size: c_uint;
    let mut bv: bio_vec;

    if (*iter).iter.bi_size == 0 {
        return false;
    }

    bv = mp_bvec_iter_bvec((*iter).bvecs, (*iter).iter);
    (*vec).paddr = bvec_phys(&bv);
    max_size = get_max_segment_size(&(*(*req).q).limits, (*vec).paddr, UINT_MAX);
    bv.bv_len = min(bv.bv_len, max_size);
    bvec_iter_advance_single((*iter).bvecs, &mut (*iter).iter, bv.bv_len);

    /*
     * If we are entirely done with this bi_io_vec entry, check if the next
     * one could be merged into it.  This typically happens when moving to the
     * next bio, but some callers also don't pack bvecs tight.
     */
    while (*iter).iter.bi_size == 0 || (*iter).iter.bi_offset == 0 {
        let next: bio_vec;

        if !__blk_map_iter_next(iter) {
            break;
        }

        next = mp_bvec_iter_bvec((*iter).bvecs, (*iter).iter);
        if bv.bv_len + next.bv_len > max_size
            || !biovec_phys_mergeable((*req).q, &bv, &next)
        {
            break;
        }

        bv.bv_len += next.bv_len;
        bvec_iter_advance_single((*iter).bvecs, &mut (*iter).iter, next.bv_len);
    }

    (*vec).len = bv.bv_len;
    true
}

/*
 * The IOVA-based DMA API wants to be able to coalesce at the minimal IOMMU page
 * size granularity (which is guaranteed to be <= PAGE_SIZE and usually 4k), so
 * we need to ensure our segments are aligned to this as well.
 *
 * Note that there is no point in using the slightly more complicated IOVA based
 * path for single segment mappings.
 */
#[inline]
unsafe fn blk_can_dma_map_iova(req: *mut request, dma_dev: *mut device) -> bool {
    (req_phys_gap_mask(req) & dma_get_merge_boundary(dma_dev)) == 0
}

unsafe fn blk_dma_map_bus(iter: *mut blk_dma_iter, vec: *mut phys_vec) -> bool {
    (*iter).addr = pci_p2pdma_bus_addr_map((*iter).p2pdma.mem, (*vec).paddr);
    (*iter).len = (*vec).len;
    true
}

unsafe fn blk_dma_map_direct(
    req: *mut request,
    dma_dev: *mut device,
    iter: *mut blk_dma_iter,
    vec: *mut phys_vec,
) -> bool {
    let mut attrs: c_uint = 0;

    if (*iter).p2pdma.map == PCI_P2PDMA_MAP_THRU_HOST_BRIDGE {
        attrs |= DMA_ATTR_MMIO;
    }

    (*iter).addr = dma_map_phys(dma_dev, (*vec).paddr, (*vec).len, rq_dma_dir(req), attrs);
    if dma_mapping_error(dma_dev, (*iter).addr) {
        (*iter).status = BLK_STS_RESOURCE;
        return false;
    }
    (*iter).len = (*vec).len;
    true
}

unsafe fn blk_rq_dma_map_iova(
    req: *mut request,
    dma_dev: *mut device,
    state: *mut dma_iova_state,
    iter: *mut blk_dma_iter,
    vec: *mut phys_vec,
) -> bool {
    let dir = rq_dma_dir(req);
    let mut attrs: c_uint = 0;
    let mut mapped: usize = 0;
    let mut error: c_int;

    (*iter).addr = (*state).addr;
    (*iter).len = dma_iova_size(state);

    if (*iter).p2pdma.map == PCI_P2PDMA_MAP_THRU_HOST_BRIDGE {
        attrs |= DMA_ATTR_MMIO;
    }

    loop {
        error = dma_iova_link(dma_dev, state, (*vec).paddr, mapped, (*vec).len, dir, attrs);
        if error != 0 {
            break;
        }
        mapped += (*vec).len as usize;
        if !blk_map_iter_next(req, &mut (*iter).iter, vec) {
            error = dma_iova_sync(dma_dev, state, 0, mapped);
            if error == 0 {
                return true;
            }
            break;
        }
    }

    dma_iova_destroy(dma_dev, state, mapped, dir, attrs);
    (*iter).status = errno_to_blk_status(error);
    false
}

#[inline]
unsafe fn blk_rq_map_iter_init(rq: *mut request, iter: *mut blk_map_iter) {
    let bio = (*rq).bio;

    if (*rq).rq_flags & RQF_SPECIAL_PAYLOAD != 0 {
        *iter = blk_map_iter {
            bvecs: &mut (*rq).special_vec,
            iter: bio_vec_iter { bi_size: (*rq).special_vec.bv_len, ..Default::default() },
            ..Default::default()
        };
    } else if !bio.is_null() {
        *iter = blk_map_iter { bio, bvecs: (*bio).bi_io_vec, iter: (*bio).bi_iter, ..Default::default() };
    } else {
        /* the internal flush request may not have bio attached */
        *iter = blk_map_iter::default();
    }
}

unsafe fn blk_dma_map_iter_start(
    req: *mut request,
    dma_dev: *mut device,
    state: *mut dma_iova_state,
    iter: *mut blk_dma_iter,
    total_len: c_uint,
) -> bool {
    let mut vec: phys_vec = Default::default();

    memset(&mut (*iter).p2pdma as *mut _, 0, core::mem::size_of_val(&(*iter).p2pdma));
    (*iter).status = BLK_STS_OK;
    (*iter).p2pdma.map = PCI_P2PDMA_MAP_NONE;

    if !blk_map_iter_next(req, &mut (*iter).iter, &mut vec) {
        return false;
    }

    match pci_p2pdma_state(&mut (*iter).p2pdma, dma_dev, phys_to_page(vec.paddr)) {
        PCI_P2PDMA_MAP_BUS_ADDR => blk_dma_map_bus(iter, &mut vec),
        PCI_P2PDMA_MAP_THRU_HOST_BRIDGE | PCI_P2PDMA_MAP_NONE => {
            if blk_can_dma_map_iova(req, dma_dev) && dma_iova_try_alloc(dma_dev, state, vec.paddr, total_len) {
                return blk_rq_dma_map_iova(req, dma_dev, state, iter, &mut vec);
            }
            memset(state as *mut _, 0, core::mem::size_of::<dma_iova_state>());
            blk_dma_map_direct(req, dma_dev, iter, &mut vec)
        }
        _ => {
            (*iter).status = BLK_STS_INVAL;
            false
        }
    }
}

/**
 * blk_rq_dma_map_iter_start - map the first DMA segment for a request
 */
pub unsafe fn blk_rq_dma_map_iter_start(req: *mut request, dma_dev: *mut device, state: *mut dma_iova_state, iter: *mut blk_dma_iter) -> bool {
    blk_rq_map_iter_init(req, &mut (*iter).iter);
    blk_dma_map_iter_start(req, dma_dev, state, iter, blk_rq_payload_bytes(req))
}

pub unsafe fn blk_rq_dma_map_iter_next(req: *mut request, dma_dev: *mut device, iter: *mut blk_dma_iter) -> bool {
    let mut vec: phys_vec = Default::default();
    if !blk_map_iter_next(req, &mut (*iter).iter, &mut vec) { return false; }
    if (*iter).p2pdma.map == PCI_P2PDMA_MAP_BUS_ADDR { blk_dma_map_bus(iter, &mut vec) } else { blk_dma_map_direct(req, dma_dev, iter, &mut vec) }
}

#[inline]
unsafe fn blk_next_sg(sg: *mut *mut scatterlist, sglist: *mut scatterlist) -> *mut scatterlist {
    if (*sg).is_null() { return sglist; }
    sg_unmark_end(*sg);
    sg_next(*sg)
}

pub unsafe fn __blk_rq_map_sg(rq: *mut request, sglist: *mut scatterlist, last_sg: *mut *mut scatterlist) -> c_int {
    let mut iter: blk_map_iter = Default::default();
    let mut vec: phys_vec = Default::default();
    let mut nsegs: c_int = 0;
    blk_rq_map_iter_init(rq, &mut iter);
    while blk_map_iter_next(rq, &mut iter, &mut vec) {
        *last_sg = blk_next_sg(last_sg, sglist);
        WARN_ON_ONCE(overflows_type(vec.len, core::mem::size_of::<c_uint>()));
        sg_set_page(*last_sg, phys_to_page(vec.paddr), vec.len, offset_in_page(vec.paddr));
        nsegs += 1;
    }
    if !(*last_sg).is_null() { sg_mark_end(*last_sg); }
    WARN_ON(nsegs > blk_rq_nr_phys_segments(rq));
    nsegs
}

#[cfg(CONFIG_BLK_DEV_INTEGRITY)]
pub unsafe fn blk_rq_integrity_dma_map_iter_start(req: *mut request, dma_dev: *mut device, state: *mut dma_iova_state, iter: *mut blk_dma_iter) -> bool {
    let len = bio_integrity_bytes(&(*(*req).q).limits.integrity, blk_rq_sectors(req));
    let bio = (*req).bio;
    (*iter).iter = blk_map_iter { bio, iter: (*bio_integrity(bio)).bip_iter, bvecs: (*bio_integrity(bio)).bip_vec, is_integrity: true, ..Default::default() };
    blk_dma_map_iter_start(req, dma_dev, state, iter, len)
}

#[cfg(CONFIG_BLK_DEV_INTEGRITY)]
pub unsafe fn blk_rq_integrity_dma_map_iter_next(req: *mut request, dma_dev: *mut device, iter: *mut blk_dma_iter) -> bool {
    let mut vec: phys_vec = Default::default();
    if !blk_map_iter_next(req, &mut (*iter).iter, &mut vec) { return false; }
    if (*iter).p2pdma.map == PCI_P2PDMA_MAP_BUS_ADDR { blk_dma_map_bus(iter, &mut vec) } else { blk_dma_map_direct(req, dma_dev, iter, &mut vec) }
}

#[cfg(CONFIG_BLK_DEV_INTEGRITY)]
pub unsafe fn blk_rq_map_integrity_sg(rq: *mut request, sglist: *mut scatterlist) -> c_int {
    let q = (*rq).q;
    let mut sg: *mut scatterlist = core::ptr::null_mut();
    let bio = (*rq).bio;
    let mut segments: c_uint = 0;
    let mut vec: phys_vec = Default::default();
    let mut iter = blk_map_iter { bio, iter: (*bio_integrity(bio)).bip_iter, bvecs: (*bio_integrity(bio)).bip_vec, is_integrity: true, ..Default::default() };
    while blk_map_iter_next(rq, &mut iter, &mut vec) {
        sg = blk_next_sg(&mut sg, sglist);
        WARN_ON_ONCE(overflows_type(vec.len, core::mem::size_of::<c_uint>()));
        sg_set_page(sg, phys_to_page(vec.paddr), vec.len, offset_in_page(vec.paddr));
        segments += 1;
    }
    if !sg.is_null() { sg_mark_end(sg); }
    BUG_ON(segments > (*rq).nr_integrity_segments);
    BUG_ON(segments > queue_max_integrity_segments(q));
    segments as c_int
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
