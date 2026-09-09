// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2009-2010 PetaLogix
 * Copyright (C) 2006 Benjamin Herrenschmidt, IBM Corporation
 *
 * Provide default implementations of the DMA mapping callbacks for
 * directly mapped busses.
 */

// Dependencies supplied by the surrounding kernel translation.

pub type phys_addr_t = u64;
pub type size_t = usize;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_data_direction {
    DMA_BIDIRECTIONAL = 0,
    DMA_TO_DEVICE = 1,
    DMA_FROM_DEVICE = 2,
    DMA_NONE = 3,
}

extern "C" {
    fn flush_dcache_range(start: phys_addr_t, end: phys_addr_t);
    fn invalidate_dcache_range(start: phys_addr_t, end: phys_addr_t);
    fn BUG() -> !;
}

unsafe fn __dma_sync(
    paddr: phys_addr_t,
    size: size_t,
    direction: dma_data_direction,
) {
    match direction {
        dma_data_direction::DMA_TO_DEVICE
        | dma_data_direction::DMA_BIDIRECTIONAL => {
            flush_dcache_range(paddr, paddr.wrapping_add(size as phys_addr_t));
        }
        dma_data_direction::DMA_FROM_DEVICE => {
            invalidate_dcache_range(paddr, paddr.wrapping_add(size as phys_addr_t));
        }
        _ => {
            BUG();
        }
    }
}

pub unsafe fn arch_sync_dma_for_device(
    paddr: phys_addr_t,
    size: size_t,
    dir: dma_data_direction,
) {
    __dma_sync(paddr, size, dir);
}

pub unsafe fn arch_sync_dma_for_cpu(
    paddr: phys_addr_t,
    size: size_t,
    dir: dma_data_direction,
) {
    __dma_sync(paddr, size, dir);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
