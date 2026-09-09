/* SPDX-License-Identifier: GPL-2.0-only */

#[repr(C)]
pub struct blk_map_iter {
    pub iter: bvec_iter,
    pub bio: *mut bio,
    pub bvecs: *mut bio_vec,
    pub is_integrity: bool,
}

#[repr(C)]
pub struct blk_dma_iter {
    /* Output address range for this iteration */
    pub addr: dma_addr_t,
    pub len: u32,
    pub p2pdma: pci_p2pdma_map_state,

    /* Status code. Only valid when blk_rq_dma_map_iter_* returned false */
    pub status: blk_status_t,

    /* Internal to blk_rq_dma_map_iter_* */
    pub iter: blk_map_iter,
}

extern "C" {
    pub fn blk_rq_dma_map_iter_start(
        req: *mut request,
        dma_dev: *mut device,
        state: *mut dma_iova_state,
        iter: *mut blk_dma_iter,
    ) -> bool;
    pub fn blk_rq_dma_map_iter_next(
        req: *mut request,
        dma_dev: *mut device,
        iter: *mut blk_dma_iter,
    ) -> bool;
}

/**
 * blk_rq_dma_map_coalesce - were all segments coalesced?
 * @state: DMA state to check
 *
 * Returns true if blk_rq_dma_map_iter_start coalesced all segments into a
 * single DMA range.
 */
#[inline]
pub unsafe fn blk_rq_dma_map_coalesce(state: *mut dma_iova_state) -> bool {
    dma_use_iova(state)
}

/**
 * blk_rq_dma_unmap - try to DMA unmap a request
 * @req: request to unmap
 * @dma_dev: device to unmap from
 * @state: DMA IOVA state
 * @mapped_len: number of bytes to unmap
 * @map: peer-to-peer mapping type
 *
 * Returns %false if the callers need to manually unmap every DMA segment
 * mapped using @iter or %true if no work is left to be done.
 */
#[inline]
pub unsafe fn blk_rq_dma_unmap(
    req: *mut request,
    dma_dev: *mut device,
    state: *mut dma_iova_state,
    mapped_len: usize,
    map: pci_p2pdma_map_type,
) -> bool {
    if map == PCI_P2PDMA_MAP_BUS_ADDR {
        return true;
    }

    if dma_use_iova(state) {
        let mut attrs: c_uint = 0;

        if map == PCI_P2PDMA_MAP_THRU_HOST_BRIDGE {
            attrs |= DMA_ATTR_MMIO;
        }

        dma_iova_destroy(dma_dev, state, mapped_len, rq_dma_dir(req), attrs);
        return true;
    }

    !dma_need_unmap(dma_dev)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
