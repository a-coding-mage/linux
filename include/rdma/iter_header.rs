/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/* Copyright (c) 2026, NVIDIA CORPORATION & AFFILIATES. */

/* Translated from <linux/scatterlist.h> and <rdma/ib_umem.h>. */

/**
 * IB block DMA iterator
 *
 * Iterates the DMA-mapped SGL in contiguous memory blocks aligned
 * to a HW supported page size.
 */
#[repr(C)]
pub struct ib_block_iter {
    /* internal states */
    pub __sg: *mut scatterlist, /* sg holding the current aligned block */
    pub __dma_addr: dma_addr_t, /* unaligned DMA address of this block */
    pub __sg_numblocks: usize, /* ib_umem_num_dma_blocks() */
    pub __sg_nents: u32, /* number of SG entries */
    pub __sg_advance: u32, /* number of bytes to advance in sg in next step */
    pub __pg_bit: u32, /* alignment of current block */
}

extern "C" {
    pub fn __rdma_block_iter_start(
        biter: *mut ib_block_iter,
        sglist: *mut scatterlist,
        nents: u32,
        pgsz: usize,
    );
    pub fn __rdma_block_iter_next(biter: *mut ib_block_iter) -> bool;
}

/**
 * rdma_block_iter_dma_address - get the aligned dma address of the current
 * block held by the block iterator.
 * @biter: block iterator holding the memory block
 */
#[inline]
pub unsafe fn rdma_block_iter_dma_address(biter: *mut ib_block_iter) -> dma_addr_t {
    (*biter).__dma_addr & !((1u64 << (*biter).__pg_bit) - 1)
}

/**
 * rdma_for_each_block - iterate over contiguous memory blocks of the sg list
 * @sglist: sglist to iterate over
 * @biter: block iterator holding the memory block
 * @nents: maximum number of sg entries to iterate over
 * @pgsz: best HW supported page size to use
 *
 * Callers may use rdma_block_iter_dma_address() to get each
 * blocks aligned DMA address.
 */
#[macro_export]
macro_rules! rdma_for_each_block {
    ($sglist:expr, $biter:expr, $nents:expr, $pgsz:expr) => {
        for _ in unsafe {
            $crate::__rdma_block_iter_start($biter, $sglist, $nents, $pgsz);
            core::iter::from_fn(|| {
                if unsafe { $crate::__rdma_block_iter_next($biter) } {
                    Some(())
                } else {
                    None
                }
            })
        } {}
    };
}

#[inline]
pub unsafe fn __rdma_umem_block_iter_start(
    biter: *mut ib_block_iter,
    umem: *mut ib_umem,
    pgsz: usize,
) {
    __rdma_block_iter_start(
        biter,
        (*umem).sgt_append.sgt.sgl,
        (*umem).sgt_append.sgt.nents,
        pgsz,
    );
    (*biter).__sg_advance = ib_umem_offset(umem) & !(pgsz - 1);
    (*biter).__sg_numblocks = ib_umem_num_dma_blocks(umem, pgsz);
}

#[inline]
pub unsafe fn __rdma_umem_block_iter_next(biter: *mut ib_block_iter) -> bool {
    if !__rdma_block_iter_next(biter) {
        return false;
    }
    let previous = (*biter).__sg_numblocks;
    (*biter).__sg_numblocks = previous.wrapping_sub(1);
    previous != 0
}

/**
 * rdma_umem_for_each_dma_block - iterate over contiguous DMA blocks of the umem
 * @umem: umem to iterate over
 * @pgsz: Page size to split the list into
 *
 * pgsz must be <= PAGE_SIZE or computed by ib_umem_find_best_pgsz(). The
 * returned DMA blocks will be aligned to pgsz and span the range:
 * ALIGN_DOWN(umem->address, pgsz) to ALIGN(umem->address + umem->length, pgsz)
 *
 * Performs exactly ib_umem_num_dma_blocks() iterations.
 */
#[macro_export]
macro_rules! rdma_umem_for_each_dma_block {
    ($umem:expr, $biter:expr, $pgsz:expr) => {
        for _ in unsafe {
            $crate::__rdma_umem_block_iter_start($biter, $umem, $pgsz);
            core::iter::from_fn(|| {
                if unsafe { $crate::__rdma_umem_block_iter_next($biter) } {
                    Some(())
                } else {
                    None
                }
            })
        } {}
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
