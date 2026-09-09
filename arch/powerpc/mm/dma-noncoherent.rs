// SPDX-License-Identifier: GPL-2.0-only
/*
 *  PowerPC version derived from arch/arm/mm/consistent.c
 *    Copyright (C) 2001 Dan Malek (dmalek@jlc.net)
 *
 *  Copyright (C) 2000 Russell King
 */

// Linux kernel dependencies supplied by the surrounding translation unit.
use core::ffi::c_void;

type SizeT = usize;
type PhysAddrT = usize;

#[repr(C)]
pub struct Page {
    _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum DmaDataDirection {
    DmaNone = 0,
    DmaToDevice,
    DmaFromDevice,
    DmaBidirectional,
}

const PAGE_SHIFT: usize = 12;
const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
const PAGE_MASK: usize = !(PAGE_SIZE - 1);
const L1_CACHE_BYTES: usize = 1;

unsafe extern "C" {
    fn flush_dcache_range(start: usize, end: usize);
    fn invalidate_dcache_range(start: usize, end: usize);
    fn clean_dcache_range(start: usize, end: usize);
    fn pfn_to_page(pfn: usize) -> *mut Page;
    fn page_address(page: *mut Page) -> *mut c_void;
    fn kmap_atomic(page: *mut Page) -> *mut c_void;
    fn kunmap_atomic(addr: *mut c_void);
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
}

/* make an area consistent. */
unsafe fn __dma_sync(vaddr: *mut c_void, size: SizeT, direction: i32) {
    let start = vaddr as usize;
    let end = start.wrapping_add(size);

    match direction {
        x if x == DmaDataDirection::DmaNone as i32 => {
            panic!("BUG");
        }
        x if x == DmaDataDirection::DmaFromDevice as i32 => {
            /*
             * invalidate only when cache-line aligned otherwise there is
             * the potential for discarding uncommitted data from the cache
             */
            if (start | end) & (L1_CACHE_BYTES - 1) != 0 {
                flush_dcache_range(start, end);
            } else {
                invalidate_dcache_range(start, end);
            }
        }
        x if x == DmaDataDirection::DmaToDevice as i32 => {
            /* writeback only */
            clean_dcache_range(start, end);
        }
        x if x == DmaDataDirection::DmaBidirectional as i32 => {
            /* writeback and invalidate */
            flush_dcache_range(start, end);
        }
        _ => {}
    }
}

/*
 * __dma_sync_page() implementation for systems using highmem.
 * In this case, each page of a buffer must be kmapped/kunmapped
 * in order to have a virtual address for __dma_sync(). This must
 * not sleep so kmap_atomic()/kunmap_atomic() are used.
 *
 * Note: yes, it is possible and correct to have a buffer extend
 * beyond the first page.
 */
#[cfg(feature = "CONFIG_HIGHMEM")]
unsafe fn __dma_sync_page_highmem(
    page: *mut Page,
    offset: usize,
    size: SizeT,
    direction: i32,
) {
    let mut seg_size = core::cmp::min(PAGE_SIZE - offset, size);
    let mut cur_size = seg_size;
    let mut flags = 0usize;
    let mut seg_offset = offset;
    let nr_segs = 1 + ((size - seg_size) + PAGE_SIZE - 1) / PAGE_SIZE;
    let mut seg_nr = 0usize;

    local_irq_save(&mut flags);

    loop {
        let start = (kmap_atomic(page.add(seg_nr)) as usize).wrapping_add(seg_offset);

        /* Sync this buffer segment */
        __dma_sync(start as *mut c_void, seg_size, direction);
        kunmap_atomic(start as *mut c_void);
        seg_nr += 1;

        /* Calculate next buffer segment size */
        seg_size = core::cmp::min(PAGE_SIZE, size - cur_size);

        /* Add the segment size to our running total */
        cur_size += seg_size;
        seg_offset = 0;
        if seg_nr >= nr_segs {
            break;
        }
    }

    local_irq_restore(flags);
}

/*
 * __dma_sync_page makes memory consistent. identical to __dma_sync, but
 * takes a struct page instead of a virtual address
 */
unsafe fn __dma_sync_page(paddr: PhysAddrT, size: SizeT, dir: i32) {
    let page = pfn_to_page(paddr >> PAGE_SHIFT);
    let offset = paddr & !PAGE_MASK;

    #[cfg(feature = "CONFIG_HIGHMEM")]
    {
        __dma_sync_page_highmem(page, offset, size, dir);
    }
    #[cfg(not(feature = "CONFIG_HIGHMEM"))]
    {
        let start = (page_address(page) as usize).wrapping_add(offset);
        __dma_sync(start as *mut c_void, size, dir);
    }
}

pub unsafe fn arch_sync_dma_for_device(
    paddr: PhysAddrT,
    size: SizeT,
    dir: DmaDataDirection,
) {
    __dma_sync_page(paddr, size, dir as i32);
}

pub unsafe fn arch_sync_dma_for_cpu(
    paddr: PhysAddrT,
    size: SizeT,
    dir: DmaDataDirection,
) {
    __dma_sync_page(paddr, size, dir as i32);
}

pub unsafe fn arch_dma_prep_coherent(page: *mut Page, size: SizeT) {
    let kaddr = page_address(page) as usize;

    flush_dcache_range(kaddr, kaddr.wrapping_add(size));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
