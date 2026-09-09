// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * DMA coherent memory allocation.
 *
 * Copyright (C) 2002 - 2005 Tensilica Inc.
 * Copyright (C) 2015 Cadence Design Systems Inc.
 *
 * Based on version for i386.
 *
 * Chris Zankel <chris@zankel.net>
 * Joe Taylor <joe@tensilica.com, joetylr@yahoo.com>
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn do_cache_op(
    paddr: phys_addr_t,
    mut size: usize,
    fn_: unsafe extern "C" fn(unsigned long, unsigned long),
) {
    let mut off: unsigned long = paddr & (PAGE_SIZE - 1);
    let pfn: unsigned long = PFN_DOWN(paddr);
    let mut page: *mut page = pfn_to_page(pfn);

    if !PageHighMem(page) {
        fn_(phys_to_virt(paddr) as unsigned long, size as unsigned long);
    } else {
        while size > 0 {
            let sz: usize = core::cmp::min(size, PAGE_SIZE - off as usize);
            let vaddr: *mut core::ffi::c_void = kmap_atomic(page);

            fn_(vaddr as unsigned long + off, sz as unsigned long);
            kunmap_atomic(vaddr);
            off = 0;
            page = page.add(1);
            size -= sz;
        }
    }
}

pub unsafe fn arch_sync_dma_for_cpu(
    paddr: phys_addr_t,
    size: usize,
    dir: enum_dma_data_direction,
) {
    match dir {
        DMA_BIDIRECTIONAL | DMA_FROM_DEVICE => {
            do_cache_op(paddr, size, __invalidate_dcache_range);
        }
        DMA_NONE => {
            BUG();
        }
        _ => {}
    }
}

pub unsafe fn arch_sync_dma_for_device(
    paddr: phys_addr_t,
    size: usize,
    dir: enum_dma_data_direction,
) {
    match dir {
        DMA_BIDIRECTIONAL | DMA_TO_DEVICE => {
            if XCHAL_DCACHE_IS_WRITEBACK {
                do_cache_op(paddr, size, __flush_dcache_range);
            }
        }
        DMA_NONE => {
            BUG();
        }
        _ => {}
    }
}

pub unsafe fn arch_dma_prep_coherent(page: *mut page, size: usize) {
    __invalidate_dcache_range(page_address(page) as unsigned long, size as unsigned long);
}

/*
 * Memory caching is platform-dependent in noMMU xtensa configurations.
 * This function should be implemented in platform code in order to enable
 * coherent DMA memory operations when CONFIG_MMU is not enabled.
 */
#[cfg(CONFIG_MMU)]
pub unsafe fn arch_dma_set_uncached(
    p: *mut core::ffi::c_void,
    _size: usize,
) -> *mut core::ffi::c_void {
    (p as *mut u8).offset((XCHAL_KSEG_BYPASS_VADDR - XCHAL_KSEG_CACHED_VADDR) as isize)
        as *mut core::ffi::c_void
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
