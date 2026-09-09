// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// C dependencies:
// <linux/kernel.h>, <linux/mm.h>, <linux/fs.h>, <linux/pagemap.h>,
// <linux/syscalls.h>, <linux/spinlock.h>, <asm/page.h>, <asm/cache.h>,
// <asm/cacheflush.h>, <asm/cachectl.h>, and <asm/tlbflush.h>.

const PG_DCACHE_CLEAN: usize = PG_arch_1;

extern "C" {
    fn is_zero_pfn(pfn: ::core::ffi::c_ulong) -> bool;
    fn folio_pfn(folio: *mut folio) -> ::core::ffi::c_ulong;
    fn folio_flush_mapping(folio: *mut folio) -> *mut address_space;
    fn folio_mapped(folio: *mut folio) -> bool;
    fn clear_bit(nr: usize, addr: *mut ::core::ffi::c_ulong);
    fn dcache_wbinv_all();
    fn icache_inv_all();
    fn set_bit(nr: usize, addr: *mut ::core::ffi::c_ulong);
    fn page_folio(page: *mut page) -> *mut folio;
    fn pte_pfn(pte: pte_t) -> ::core::ffi::c_ulong;
    fn flush_tlb_page(vma: *mut vm_area_struct, addr: ::core::ffi::c_ulong);
    fn pfn_valid(pfn: ::core::ffi::c_ulong) -> bool;
    fn pfn_to_page(pfn: ::core::ffi::c_ulong) -> *mut page;
    fn test_and_set_bit(nr: usize, addr: *mut ::core::ffi::c_ulong) -> bool;
}

// Types and constants are supplied by the corresponding kernel dependencies.
#[allow(non_camel_case_types)]
type PG_arch_1_type = usize;
use PG_arch_1_type as PG_arch_1;

unsafe extern "C" {
    type folio;
    type address_space;
    type page;
    type pte_t;
    type vm_fault;
    type vm_area_struct;
}

// The `flags.f` member and `vm_flags` field are supplied by the kernel types.
const VM_EXEC: ::core::ffi::c_ulong = 0x4;

pub unsafe fn flush_dcache_folio(folio: *mut folio) {
    let mapping: *mut address_space;

    if is_zero_pfn(folio_pfn(folio)) {
        return;
    }

    mapping = folio_flush_mapping(folio);

    if !mapping.is_null() && !folio_mapped(folio) {
        clear_bit(PG_DCACHE_CLEAN, &mut (*folio).flags.f);
    } else {
        dcache_wbinv_all();
        if !mapping.is_null() {
            icache_inv_all();
        }
        set_bit(PG_DCACHE_CLEAN, &mut (*folio).flags.f);
    }
}

// EXPORT_SYMBOL(flush_dcache_folio);

pub unsafe fn flush_dcache_page(page: *mut page) {
    flush_dcache_folio(page_folio(page));
}

// EXPORT_SYMBOL(flush_dcache_page);

pub unsafe fn update_mmu_cache_range(
    _vmf: *mut vm_fault,
    vma: *mut vm_area_struct,
    addr: ::core::ffi::c_ulong,
    ptep: *mut pte_t,
    _nr: u32,
) {
    let pfn: ::core::ffi::c_ulong = pte_pfn(*ptep);
    let folio: *mut folio;

    flush_tlb_page(vma, addr);

    if !pfn_valid(pfn) {
        return;
    }

    if is_zero_pfn(pfn) {
        return;
    }

    folio = page_folio(pfn_to_page(pfn));
    if !test_and_set_bit(PG_DCACHE_CLEAN, &mut (*folio).flags.f) {
        dcache_wbinv_all();
    }

    if !folio_flush_mapping(folio).is_null() {
        if (*vma).vm_flags & VM_EXEC != 0 {
            icache_inv_all();
        }
    }
}

pub unsafe fn flush_cache_range(
    vma: *mut vm_area_struct,
    _start: ::core::ffi::c_ulong,
    _end: ::core::ffi::c_ulong,
) {
    dcache_wbinv_all();

    if (*vma).vm_flags & VM_EXEC != 0 {
        icache_inv_all();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
