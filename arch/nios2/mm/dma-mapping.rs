/*
 * Copyright (C) 2011 Tobias Klauser <tklauser@distanz.ch>
 * Copyright (C) 2009 Wind River Systems Inc
 *  Implemented by fredrik.markstrom@gmail.com and ivarholmqvist@gmail.com
 *
 * Based on DMA code from MIPS.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

use core::ffi::c_void;

// Types and functions supplied by the corresponding kernel headers.
pub type PhysAddr = u64;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DmaDataDirection {
    DmaBidirectional = 0,
    DmaToDevice = 1,
    DmaFromDevice = 2,
    DmaNone = 3,
}

#[repr(C)]
pub struct Page {
    _private: [u8; 0],
}

extern "C" {
    fn phys_to_virt(paddr: PhysAddr) -> *mut c_void;
    fn invalidate_dcache_range(start: usize, end: usize);
    fn flush_dcache_range(start: usize, end: usize);
    fn page_address(page: *mut Page) -> *mut c_void;
    fn BUG() -> !;
}

// CONFIG_NIOS2_IO_REGION_BASE is supplied by the build configuration.
extern "C" {
    static CONFIG_NIOS2_IO_REGION_BASE: usize;
}

pub unsafe fn arch_sync_dma_for_device(
    paddr: PhysAddr,
    size: usize,
    dir: DmaDataDirection,
) {
    let vaddr = phys_to_virt(paddr);

    match dir {
        DmaDataDirection::DmaFromDevice => {
            invalidate_dcache_range(vaddr as usize, (vaddr as usize).wrapping_add(size));
        }
        DmaDataDirection::DmaToDevice => {
            /*
             * We just need to flush the caches here , but Nios2 flush
             * instruction will do both writeback and invalidate.
             */
            flush_dcache_range(vaddr as usize, (vaddr as usize).wrapping_add(size));
        }
        DmaDataDirection::DmaBidirectional => {
            // flush and invalidate
            flush_dcache_range(vaddr as usize, (vaddr as usize).wrapping_add(size));
        }
        _ => BUG(),
    }
}

pub unsafe fn arch_sync_dma_for_cpu(
    paddr: PhysAddr,
    size: usize,
    dir: DmaDataDirection,
) {
    let vaddr = phys_to_virt(paddr);

    match dir {
        DmaDataDirection::DmaBidirectional | DmaDataDirection::DmaFromDevice => {
            invalidate_dcache_range(vaddr as usize, (vaddr as usize).wrapping_add(size));
        }
        DmaDataDirection::DmaToDevice => {}
        _ => BUG(),
    }
}

pub unsafe fn arch_dma_prep_coherent(page: *mut Page, size: usize) {
    let start = page_address(page) as usize;

    flush_dcache_range(start, start.wrapping_add(size));
}

pub unsafe fn arch_dma_set_uncached(ptr: *mut c_void, size: usize) -> *mut c_void {
    let mut addr = ptr as usize;

    addr |= CONFIG_NIOS2_IO_REGION_BASE;

    let _ = (addr, size);
    ptr
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
