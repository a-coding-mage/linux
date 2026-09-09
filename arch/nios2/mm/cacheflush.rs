/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive.
 *
 * Copyright (C) 2009, Wind River Systems Inc
 * Implemented by fredrik.markstrom@gmail.com and ivarholmqvist@gmail.com
 */

// Dependencies are supplied by the surrounding kernel translation.

unsafe fn __flush_dcache(mut start: c_ulong, mut end: c_ulong) {
    let mut addr: c_ulong;

    start &= !(cpuinfo.dcache_line_size - 1);
    end += cpuinfo.dcache_line_size - 1;
    end &= !(cpuinfo.dcache_line_size - 1);

    if end > start + cpuinfo.dcache_size {
        end = start + cpuinfo.dcache_size;
    }

    addr = start;
    while addr < end {
        core::arch::asm!("flushd 0({0})", in(reg) addr);
        addr += cpuinfo.dcache_line_size;
    }
}

unsafe fn __invalidate_dcache(mut start: c_ulong, mut end: c_ulong) {
    let mut addr: c_ulong;

    start &= !(cpuinfo.dcache_line_size - 1);
    end += cpuinfo.dcache_line_size - 1;
    end &= !(cpuinfo.dcache_line_size - 1);

    addr = start;
    while addr < end {
        core::arch::asm!("initda 0({0})", in(reg) addr);
        addr += cpuinfo.dcache_line_size;
    }
}

unsafe fn __flush_icache(mut start: c_ulong, mut end: c_ulong) {
    let mut addr: c_ulong;

    start &= !(cpuinfo.icache_line_size - 1);
    end += cpuinfo.icache_line_size - 1;
    end &= !(cpuinfo.icache_line_size - 1);

    if end > start + cpuinfo.icache_size {
        end = start + cpuinfo.icache_size;
    }

    addr = start;
    while addr < end {
        core::arch::asm!("flushi {0}", in(reg) addr);
        addr += cpuinfo.icache_line_size;
    }
    core::arch::asm!("flushp");
}

unsafe fn flush_aliases(mapping: *mut address_space, folio: *mut folio) {
    let mm = (*current).active_mm;
    let mut vma: *mut vm_area_struct;
    let mut flags: c_ulong;
    let pgoff: pgoff_t;
    let nr = folio_nr_pages(folio);

    pgoff = (*folio).index;

    flush_dcache_mmap_lock_irqsave(mapping, &mut flags);
    mapping_rmap_tree_foreach!(vma, mapping, pgoff, pgoff + nr - 1, {
        let start: c_ulong;

        if (*vma).vm_mm != mm {
            continue;
        }
        if (*vma).vm_flags & VM_MAYSHARE == 0 {
            continue;
        }

        start = (*vma).vm_start + ((pgoff - (*vma).vm_pgoff) << PAGE_SHIFT);
        flush_cache_range(vma, start, start + nr * PAGE_SIZE);
    });
    flush_dcache_mmap_unlock_irqrestore(mapping, flags);
}

pub unsafe fn flush_cache_all() {
    __flush_dcache(0, cpuinfo.dcache_size);
    __flush_icache(0, cpuinfo.icache_size);
}

pub unsafe fn flush_cache_mm(_mm: *mut mm_struct) { flush_cache_all(); }

pub unsafe fn flush_cache_dup_mm(_mm: *mut mm_struct) { flush_cache_all(); }

pub unsafe fn flush_icache_range(start: c_ulong, end: c_ulong) {
    __flush_dcache(start, end);
    __flush_icache(start, end);
}

pub unsafe fn flush_dcache_range(start: c_ulong, end: c_ulong) {
    __flush_dcache(start, end);
    __flush_icache(start, end);
}

pub unsafe fn invalidate_dcache_range(start: c_ulong, end: c_ulong) {
    __invalidate_dcache(start, end);
}

pub unsafe fn flush_cache_range(vma: *mut vm_area_struct, start: c_ulong, end: c_ulong) {
    __flush_dcache(start, end);
    if vma.is_null() || (*vma).vm_flags & VM_EXEC != 0 {
        __flush_icache(start, end);
    }
}

pub unsafe fn flush_icache_pages(vma: *mut vm_area_struct, page: *mut page, nr: c_uint) {
    let start = page_address(page) as c_ulong;
    let end = start + nr * PAGE_SIZE;
    __flush_dcache(start, end);
    __flush_icache(start, end);
}

pub unsafe fn flush_cache_page(vma: *mut vm_area_struct, vmaddr: c_ulong, _pfn: c_ulong) {
    let start = vmaddr;
    let end = start + PAGE_SIZE;
    __flush_dcache(start, end);
    if (*vma).vm_flags & VM_EXEC != 0 {
        __flush_icache(start, end);
    }
}

unsafe fn __flush_dcache_folio(folio: *mut folio) {
    /* Write back data associated with the kernel mapping of this page. */
    let start = folio_address(folio) as c_ulong;
    __flush_dcache(start, start + folio_size(folio));
}

pub unsafe fn flush_dcache_folio(folio: *mut folio) {
    let mapping: *mut address_space;

    /* The zero page is never written to and never needs flushing. */
    if is_zero_pfn(folio_pfn(folio)) {
        return;
    }

    mapping = folio_flush_mapping(folio);
    if !mapping.is_null() && !mapping_mapped(mapping) {
        clear_bit(PG_dcache_clean, &mut (*folio).flags.f);
    } else {
        __flush_dcache_folio(folio);
        if !mapping.is_null() {
            let start = folio_address(folio) as c_ulong;
            flush_aliases(mapping, folio);
            flush_icache_range(start, start + folio_size(folio));
        }
        set_bit(PG_dcache_clean, &mut (*folio).flags.f);
    }
}

pub unsafe fn flush_dcache_page(page: *mut page) {
    flush_dcache_folio(page_folio(page));
}

pub unsafe fn update_mmu_cache_range(_vmf: *mut vm_fault, vma: *mut vm_area_struct,
                                    address: c_ulong, ptep: *mut pte_t, _nr: c_uint) {
    let pte = *ptep;
    let pfn = pte_pfn(pte);
    let folio: *mut folio;
    let mapping: *mut address_space;

    reload_tlb_page(vma, address, pte);
    if !pfn_valid(pfn) || is_zero_pfn(pfn) {
        return;
    }

    folio = page_folio(pfn_to_page(pfn));
    if !test_and_set_bit(PG_dcache_clean, &mut (*folio).flags.f) {
        __flush_dcache_folio(folio);
    }

    mapping = folio_flush_mapping(folio);
    if !mapping.is_null() {
        flush_aliases(mapping, folio);
        if (*vma).vm_flags & VM_EXEC != 0 {
            flush_icache_pages(vma, &mut (*folio).page, folio_nr_pages(folio));
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
