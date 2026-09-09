// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OpenRISC cache.c
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * Modifications for the OpenRISC architecture:
 * Copyright (C) 2015 Jan Henrik Weinstock <jan.weinstock@rwth-aachen.de>
 */

// Dependencies supplied by the corresponding architecture and kernel modules.

extern "C" {
    fn mfspr(reg: u16) -> usize;
    fn mtspr(reg: u16, value: usize);
    fn page_to_pfn(page: *mut page) -> usize;
    fn pte_val(pte: pte_t) -> usize;
    fn pfn_to_page(pfn: usize) -> *mut page;
    fn page_folio(page: *mut page) -> *mut folio;
    fn test_and_set_bit(bit: usize, addr: *mut usize) -> bool;
    fn folio_nr_pages(folio: *mut folio) -> usize;
    fn folio_page(folio: *mut folio, index: usize) -> *mut page;
    fn sync_icache_dcache(page: *mut page);
}

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct folio {
    pub flags: folio_flags,
}

#[repr(C)]
pub struct folio_flags {
    pub f: usize,
}

#[repr(C)]
pub struct vm_area_struct {
    pub vm_flags: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pte_t {
    _private: [u8; 0],
}

// SPR_UPR, SPR_UPR_UP, SPR_UPR_DCP, SPR_UPR_ICP, SPR_DCBFR, SPR_ICBIR,
// SPR_DCBIR, SPR_ICCFGR, SPR_ICCFGR_NCS, SPR_ICCFGR_CBS, L1_CACHE_BYTES,
// PAGE_SHIFT, PAGE_SIZE, VM_EXEC, and PG_dc_clean are supplied by the
// corresponding architecture and kernel headers.

/*
 * Check if the cache component exists.
 */
pub unsafe fn cpu_cache_is_present(cache_type: usize) -> bool {
    let upr = mfspr(SPR_UPR);
    let mask = SPR_UPR_UP | cache_type;

    !((upr & mask) ^ mask != 0)
}

unsafe fn cache_loop(
    mut paddr: usize,
    end: usize,
    reg: u16,
    cache_type: usize,
) {
    if !cpu_cache_is_present(cache_type) {
        return;
    }

    while paddr < end {
        mtspr(reg, paddr);
        paddr = paddr.wrapping_add(L1_CACHE_BYTES);
    }
}

unsafe fn cache_loop_page(page: *mut page, reg: u16, cache_type: usize) {
    let mut paddr = page_to_pfn(page) << PAGE_SHIFT;
    let end = paddr.wrapping_add(PAGE_SIZE);

    paddr &= !(L1_CACHE_BYTES - 1);

    cache_loop(paddr, end, reg, cache_type);
}

pub unsafe fn local_dcache_page_flush(page: *mut page) {
    cache_loop_page(page, SPR_DCBFR, SPR_UPR_DCP);
}

// EXPORT_SYMBOL(local_dcache_page_flush);

pub unsafe fn local_icache_page_inv(page: *mut page) {
    cache_loop_page(page, SPR_ICBIR, SPR_UPR_ICP);
}

// EXPORT_SYMBOL(local_icache_page_inv);

pub unsafe fn local_icache_all_inv() {
    if cpu_cache_is_present(SPR_UPR_ICP) {
        let iccfgr = mfspr(SPR_ICCFGR);
        let sets = 1usize << ((iccfgr & SPR_ICCFGR_NCS) >> 3);
        let block_size = 16usize << ((iccfgr & SPR_ICCFGR_CBS) >> 7);
        let mut paddr = 0usize;
        let end = sets * block_size;

        while paddr < end {
            mtspr(SPR_ICBIR, paddr);
            paddr = paddr.wrapping_add(block_size);
        }
    }
}

pub unsafe fn local_dcache_range_flush(mut start: usize, end: usize) {
    cache_loop(start, end, SPR_DCBFR, SPR_UPR_DCP);
}

pub unsafe fn local_dcache_range_inv(start: usize, end: usize) {
    cache_loop(start, end, SPR_DCBIR, SPR_UPR_DCP);
}

pub unsafe fn local_icache_range_inv(start: usize, end: usize) {
    cache_loop(start, end, SPR_ICBIR, SPR_UPR_ICP);
}

pub unsafe fn update_cache(vma: *mut vm_area_struct, address: usize, pte: *mut pte_t) {
    let _ = address;
    let pfn = pte_val(*pte) >> PAGE_SHIFT;
    let folio = page_folio(pfn_to_page(pfn));
    let dirty = !test_and_set_bit(PG_dc_clean, &mut (*folio).flags.f);

    /*
     * Since icaches do not snoop for updated data on OpenRISC, we
     * must write back and invalidate any dirty pages manually. We
     * can skip data pages, since they will not end up in icaches.
     */
    if ((*vma).vm_flags & VM_EXEC) != 0 && dirty {
        let mut nr = folio_nr_pages(folio);

        while nr != 0 {
            nr -= 1;
            sync_icache_dcache(folio_page(folio, nr));
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
