// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OpenRISC Linux
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * Modifications for the OpenRISC architecture:
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 *
 * DMA mapping callbacks...
 */

use core::ffi::c_void;

// Dependencies supplied by the surrounding kernel translation.
type Pte = usize;
type PhysAddr = usize;
type SizeT = usize;

#[repr(C)]
pub struct MmWalk {
    _private: [u8; 0],
}

type PteEntry = unsafe extern "C" fn(*mut Pte, usize, usize, *mut MmWalk) -> i32;

#[repr(C)]
struct MmWalkOps {
    pte_entry: Option<PteEntry>,
}

#[repr(i32)]
enum DmaDataDirection {
    DmaToDevice = 1,
    DmaFromDevice = 2,
    DmaBidirectional = 3,
}

const PAGE_SIZE: usize = 4096;
const _PAGE_CI: usize = 1 << 7;

unsafe extern "C" {
    static mut init_mm: c_void;

    fn flush_tlb_kernel_range(start: usize, end: usize);
    fn local_dcache_range_flush(start: usize, end: usize);
    fn local_dcache_range_inv(start: usize, end: usize);
    fn mmap_write_lock(mm: *mut c_void);
    fn mmap_write_unlock(mm: *mut c_void);
    fn walk_kernel_page_table_range(
        start: usize,
        end: usize,
        ops: *const MmWalkOps,
        walk: *mut c_void,
        private: *mut c_void,
    ) -> i32;
    fn warn_on(condition: bool) -> bool;
    fn err_ptr(error: i32) -> *mut c_void;
    fn pa(address: usize) -> usize;
}

unsafe extern "C" fn page_set_nocache(
    pte: *mut Pte,
    addr: usize,
    next: usize,
    _walk: *mut MmWalk,
) -> i32 {
    *pte |= _PAGE_CI;

    /*
     * Flush the page out of the TLB so that the new page flags get
     * picked up next time there's an access
     */
    flush_tlb_kernel_range(addr, addr.wrapping_add(PAGE_SIZE));

    /* Flush page out of dcache */
    local_dcache_range_flush(pa(addr), pa(next));

    0
}

static SET_NOCACHE_WALK_OPS: MmWalkOps = MmWalkOps {
    pte_entry: Some(page_set_nocache),
};

unsafe extern "C" fn page_clear_nocache(
    pte: *mut Pte,
    addr: usize,
    _next: usize,
    _walk: *mut MmWalk,
) -> i32 {
    *pte &= !_PAGE_CI;

    /*
     * Flush the page out of the TLB so that the new page flags get
     * picked up next time there's an access
     */
    flush_tlb_kernel_range(addr, addr.wrapping_add(PAGE_SIZE));

    0
}

static CLEAR_NOCACHE_WALK_OPS: MmWalkOps = MmWalkOps {
    pte_entry: Some(page_clear_nocache),
};

#[no_mangle]
pub unsafe extern "C" fn arch_dma_set_uncached(cpu_addr: *mut c_void, size: SizeT) -> *mut c_void {
    let va = cpu_addr as usize;
    let error: i32;

    /*
     * We need to iterate through the pages, clearing the dcache for
     * them and setting the cache-inhibit bit.
     */
    mmap_write_lock(&mut init_mm as *mut c_void);
    error = walk_kernel_page_table_range(
        va,
        va.wrapping_add(size),
        &SET_NOCACHE_WALK_OPS,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
    );
    mmap_write_unlock(&mut init_mm as *mut c_void);

    if error != 0 {
        return err_ptr(error);
    }
    cpu_addr
}

#[no_mangle]
pub unsafe extern "C" fn arch_dma_clear_uncached(cpu_addr: *mut c_void, size: SizeT) {
    let va = cpu_addr as usize;

    mmap_write_lock(&mut init_mm as *mut c_void);
    /* walk_page_range shouldn't be able to fail here */
    warn_on(
        walk_kernel_page_table_range(
            va,
            va.wrapping_add(size),
            &CLEAR_NOCACHE_WALK_OPS,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        ) != 0,
    );
    mmap_write_unlock(&mut init_mm as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn arch_sync_dma_for_device(
    addr: PhysAddr,
    size: SizeT,
    dir: DmaDataDirection,
) {
    match dir {
        DmaDataDirection::DmaToDevice => {
            /* Flush the dcache for the requested range */
            local_dcache_range_flush(addr, addr.wrapping_add(size));
        }
        DmaDataDirection::DmaFromDevice => {
            /* Invalidate the dcache for the requested range */
            local_dcache_range_inv(addr, addr.wrapping_add(size));
        }
        _ => {
            /*
             * NOTE: If dir == DMA_BIDIRECTIONAL then there's no need to
             * flush nor invalidate the cache here as the area will need
             * to be manually synced anyway.
             */
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
