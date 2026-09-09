// SPDX-License-Identifier: GPL-2.0-only
/*
 * DMA implementation for Hexagon
 *
 * Copyright (c) 2010-2012, The Linux Foundation. All rights reserved.
 */

// C dependencies supplied by the surrounding kernel:
// linux/dma-map-ops.h, linux/memblock.h, and asm/page.h

use core::ffi::c_void;

extern "C" {
    fn phys_to_virt(paddr: phys_addr_t) -> *mut c_void;
    fn hexagon_clean_dcache_range(start: c_ulong, end: c_ulong);
    fn hexagon_inv_dcache_range(start: c_ulong, end: c_ulong);
    fn flush_dcache_range(start: c_ulong, end: c_ulong);
    fn dma_init_global_coherent(phys: phys_addr_t, size: usize) -> c_int;
    static max_low_pfn: c_ulong;
    static hexagon_coherent_pool_size: usize;
    fn BUG() -> !;
}

type phys_addr_t = usize;
type c_ulong = usize;
type c_int = i32;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_data_direction {
    DMA_BIDIRECTIONAL = 0,
    DMA_TO_DEVICE = 1,
    DMA_FROM_DEVICE = 2,
}

pub unsafe fn arch_sync_dma_for_device(
    paddr: phys_addr_t,
    size: usize,
    dir: dma_data_direction,
) {
    let addr = phys_to_virt(paddr);

    match dir {
        dma_data_direction::DMA_TO_DEVICE => {
            hexagon_clean_dcache_range(
                addr as c_ulong,
                (addr as c_ulong).wrapping_add(size),
            );
        }
        dma_data_direction::DMA_FROM_DEVICE => {
            hexagon_inv_dcache_range(
                addr as c_ulong,
                (addr as c_ulong).wrapping_add(size),
            );
        }
        dma_data_direction::DMA_BIDIRECTIONAL => {
            flush_dcache_range(
                addr as c_ulong,
                (addr as c_ulong).wrapping_add(size),
            );
        }
        _ => BUG(),
    }
}

/*
 * Our max_low_pfn should have been backed off by 16MB in mm/init.c to create
 * DMA coherent space.  Use that for the pool.
 */
#[allow(non_snake_case)]
unsafe fn hexagon_dma_init() -> c_int {
    dma_init_global_coherent(
        max_low_pfn.wrapping_mul(4096), // PFN_PHYS(max_low_pfn)
        hexagon_coherent_pool_size,
    )
}

// C registration: core_initcall(hexagon_dma_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
