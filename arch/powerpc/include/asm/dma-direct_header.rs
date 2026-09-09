/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from ASM_POWERPC_DMA_DIRECT_H. */

/// Convert a physical address to a DMA address.
#[inline]
pub unsafe fn phys_to_dma(dev: *mut device, paddr: phys_addr_t) -> dma_addr_t {
    paddr + (*dev).archdata.dma_offset
}

/// Convert a DMA address to a physical address.
#[inline]
pub unsafe fn dma_to_phys(dev: *mut device, daddr: dma_addr_t) -> phys_addr_t {
    daddr - (*dev).archdata.dma_offset
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
