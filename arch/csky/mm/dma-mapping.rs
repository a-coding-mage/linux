// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// Dependencies supplied by the corresponding kernel headers are intentionally
// referenced here as external symbols.

use core::ffi::c_void;

extern "C" {
    fn phys_to_page(paddr: phys_addr_t) -> *mut page;
    fn page_to_phys(page: *mut page) -> phys_addr_t;
    fn __va(addr: phys_addr_t) -> *mut c_void;
    fn offset_in_page(addr: phys_addr_t) -> usize;
    fn PageHighMem(page: *mut page) -> bool;
    fn kmap_atomic(page: *mut page) -> *mut c_void;
    fn kunmap_atomic(addr: *mut c_void);
    fn dma_wbinv_range(start: c_ulong, end: c_ulong);
    fn dma_wb_range(start: c_ulong, end: c_ulong);
    fn dma_inv_range(start: c_ulong, end: c_ulong);
    fn memset(dest: *mut c_void, value: c_int, count: usize) -> *mut c_void;
    fn BUG() -> !;
}

type phys_addr_t = usize;
type c_ulong = usize;
type c_int = i32;

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

pub type dma_data_direction = i32;

pub const DMA_TO_DEVICE: dma_data_direction = 1;
pub const DMA_FROM_DEVICE: dma_data_direction = 2;
pub const DMA_BIDIRECTIONAL: dma_data_direction = 3;

// PAGE_SIZE is supplied by the kernel build configuration.
extern "C" {
    static PAGE_SIZE: usize;
}

#[inline]
unsafe fn cache_op(
    paddr: phys_addr_t,
    size: usize,
    fn_: unsafe extern "C" fn(start: c_ulong, end: c_ulong),
) {
    let mut page = phys_to_page(paddr);
    let mut start = __va(page_to_phys(page));
    let mut offset = offset_in_page(paddr);
    let mut left = size;

    loop {
        let mut len = left;

        if offset + len > PAGE_SIZE {
            len = PAGE_SIZE - offset;
        }

        if PageHighMem(page) {
            start = kmap_atomic(page);

            fn_(
                (start as c_ulong).wrapping_add(offset),
                (start as c_ulong).wrapping_add(offset).wrapping_add(len),
            );

            kunmap_atomic(start);
        } else {
            fn_(
                (start as c_ulong).wrapping_add(offset),
                (start as c_ulong).wrapping_add(offset).wrapping_add(len),
            );
        }
        offset = 0;

        page = page.add(1);
        start = (start as *mut u8).add(PAGE_SIZE) as *mut c_void;
        left -= len;
        if left == 0 {
            break;
        }
    }
}

unsafe extern "C" fn dma_wbinv_set_zero_range(start: c_ulong, end: c_ulong) {
    memset(start as *mut c_void, 0, end.wrapping_sub(start));
    dma_wbinv_range(start, end);
}

#[no_mangle]
pub unsafe extern "C" fn arch_dma_prep_coherent(page: *mut page, size: usize) {
    cache_op(page_to_phys(page), size, dma_wbinv_set_zero_range);
}

#[no_mangle]
pub unsafe extern "C" fn arch_sync_dma_for_device(
    paddr: phys_addr_t,
    size: usize,
    dir: dma_data_direction,
) {
    match dir {
        DMA_TO_DEVICE => cache_op(paddr, size, dma_wb_range),
        DMA_FROM_DEVICE | DMA_BIDIRECTIONAL => cache_op(paddr, size, dma_wbinv_range),
        _ => BUG(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn arch_sync_dma_for_cpu(
    paddr: phys_addr_t,
    size: usize,
    dir: dma_data_direction,
) {
    match dir {
        DMA_TO_DEVICE => return,
        DMA_FROM_DEVICE | DMA_BIDIRECTIONAL => cache_op(paddr, size, dma_inv_range),
        _ => BUG(),
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
