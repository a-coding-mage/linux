// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2000  Ani Joshi <ajoshi@unixbox.com>
 * Copyright (C) 2000, 2001, 06\t Ralf Baechle <ralf@linux-mips.org>
 * swiped from i386, and cloned for MIPS by Geert, polished by Ralf.
 */

// Dependencies supplied by the surrounding kernel translation unit.
use core::ffi::c_void;

extern "C" {
    fn boot_cpu_type() -> i32;
    static cpu_has_maar: bool;
    fn dma_cache_wback_inv(addr: usize, size: usize);
    fn dma_cache_wback(addr: usize, size: usize);
    fn dma_cache_inv(addr: usize, size: usize);
    fn __pa(addr: *mut c_void) -> usize;
    fn page_address(page: *mut page) -> *mut c_void;
    fn pfn_to_page(pfn: usize) -> *mut page;
    fn PageHighMem(page: *mut page) -> bool;
    fn kmap_atomic(page: *mut page) -> *mut c_void;
    fn kunmap_atomic(addr: *mut c_void);
    fn dev_assign_dma_coherent(dev: *mut device, coherent: bool);
}

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

pub type phys_addr_t = usize;
pub type dma_data_direction = i32;

const CPU_R10000: i32 = 0;
const CPU_R12000: i32 = 0;
const CPU_BMIPS5000: i32 = 0;
const CPU_LOONGSON2EF: i32 = 0;
const CPU_XBURST: i32 = 0;
const DMA_TO_DEVICE: dma_data_direction = 0;
const DMA_FROM_DEVICE: dma_data_direction = 1;
const DMA_BIDIRECTIONAL: dma_data_direction = 2;
const PAGE_SHIFT: usize = 12;
const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
const PAGE_MASK: usize = !(PAGE_SIZE - 1);
const UNCAC_BASE: usize = 0;

/*
 * The affected CPUs below in 'cpu_needs_post_dma_flush()' can speculatively
 * fill random cachelines with stale data at any time, requiring an extra
 * flush post-DMA.
 *
 * Warning on the terminology - Linux calls an uncached area coherent;  MIPS
 * terminology calls memory areas with hardware maintained coherency coherent.
 *
 * Note that the R14000 and R16000 should also be checked for in this condition.
 * However this function is only called on non-I/O-coherent systems and only the
 * R10000 and R12000 are used in such systems, the SGI IP28 Indigo² rsp.
 * SGI IP32 aka O2.
 */
unsafe fn cpu_needs_post_dma_flush() -> bool {
    match boot_cpu_type() {
        CPU_R10000 | CPU_R12000 | CPU_BMIPS5000 | CPU_LOONGSON2EF | CPU_XBURST => true,
        _ => {
            /*
             * Presence of MAARs suggests that the CPU supports
             * speculatively prefetching data, and therefore requires
             * the post-DMA flush/invalidate.
             */
            cpu_has_maar
        }
    }
}

pub unsafe fn arch_dma_prep_coherent(page: *mut page, size: usize) {
    dma_cache_wback_inv(page_address(page) as usize, size);
}

pub unsafe fn arch_dma_set_uncached(addr: *mut c_void, _size: usize) -> *mut c_void {
    (__pa(addr) + UNCAC_BASE) as *mut c_void
}

unsafe fn dma_sync_virt_for_device(
    addr: *mut c_void,
    size: usize,
    dir: dma_data_direction,
) {
    match dir {
        DMA_TO_DEVICE => dma_cache_wback(addr as usize, size),
        DMA_FROM_DEVICE => dma_cache_inv(addr as usize, size),
        DMA_BIDIRECTIONAL => dma_cache_wback_inv(addr as usize, size),
        _ => panic!("BUG"),
    }
}

unsafe fn dma_sync_virt_for_cpu(addr: *mut c_void, size: usize, dir: dma_data_direction) {
    match dir {
        DMA_TO_DEVICE => {}
        DMA_FROM_DEVICE | DMA_BIDIRECTIONAL => dma_cache_inv(addr as usize, size),
        _ => panic!("BUG"),
    }
}

/*
 * A single sg entry may refer to multiple physically contiguous pages.  But
 * we still need to process highmem pages individually.  If highmem is not
 * configured then the bulk of this loop gets optimized out.
 */
unsafe fn dma_sync_phys(
    paddr: phys_addr_t,
    size: usize,
    dir: dma_data_direction,
    for_device: bool,
) {
    let mut page = pfn_to_page(paddr >> PAGE_SHIFT);
    let mut offset = paddr & !PAGE_MASK;
    let mut left = size;

    loop {
        let mut len = left;
        let addr: *mut c_void;

        if PageHighMem(page) && offset + len > PAGE_SIZE {
            len = PAGE_SIZE - offset;
        }

        addr = kmap_atomic(page);
        let mapped = (addr as *mut u8).add(offset) as *mut c_void;
        if for_device {
            dma_sync_virt_for_device(mapped, len, dir);
        } else {
            dma_sync_virt_for_cpu(mapped, len, dir);
        }
        kunmap_atomic(addr);

        offset = 0;
        page = page.add(1);
        left -= len;
        if left == 0 {
            break;
        }
    }
}

pub unsafe fn arch_sync_dma_for_device(
    paddr: phys_addr_t,
    size: usize,
    dir: dma_data_direction,
) {
    dma_sync_phys(paddr, size, dir, true);
}

// CONFIG_ARCH_HAS_SYNC_DMA_FOR_CPU
pub unsafe fn arch_sync_dma_for_cpu(
    paddr: phys_addr_t,
    size: usize,
    dir: dma_data_direction,
) {
    if cpu_needs_post_dma_flush() {
        dma_sync_phys(paddr, size, dir, false);
    }
}

// CONFIG_ARCH_HAS_SETUP_DMA_OPS
pub unsafe fn arch_setup_dma_ops(dev: *mut device, coherent: bool) {
    dev_assign_dma_coherent(dev, coherent);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
