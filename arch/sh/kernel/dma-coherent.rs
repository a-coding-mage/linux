// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2004 - 2007  Paul Mundt
 */

use core::ffi::c_void;

// Types and functions supplied by the Linux memory-management and SH
// architecture headers.
#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

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

unsafe extern "C" {
    fn page_address(page: *mut page) -> *mut c_void;
    fn phys_to_virt(address: phys_addr_t) -> *mut c_void;
    fn sh_cacheop_vaddr(address: *mut c_void) -> *mut c_void;
    fn __flush_purge_region(address: *mut c_void, size: size_t);
    fn __flush_invalidate_region(address: *mut c_void, size: size_t);
    fn __flush_wback_region(address: *mut c_void, size: size_t);
    fn BUG() -> !;
}

pub unsafe fn arch_dma_prep_coherent(page: *mut page, size: size_t) {
    unsafe {
        __flush_purge_region(page_address(page), size);
    }
}

pub unsafe fn arch_sync_dma_for_device(
    paddr: phys_addr_t,
    size: size_t,
    dir: dma_data_direction,
) {
    let addr = unsafe { sh_cacheop_vaddr(phys_to_virt(paddr)) };

    match dir {
        dma_data_direction::DMA_FROM_DEVICE => {
            // invalidate only
            unsafe {
                __flush_invalidate_region(addr, size);
            }
        }
        dma_data_direction::DMA_TO_DEVICE => {
            // writeback only
            unsafe {
                __flush_wback_region(addr, size);
            }
        }
        dma_data_direction::DMA_BIDIRECTIONAL => {
            // writeback and invalidate
            unsafe {
                __flush_purge_region(addr, size);
            }
        }
        _ => unsafe {
            BUG();
        },
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
