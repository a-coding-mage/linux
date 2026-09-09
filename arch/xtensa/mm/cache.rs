/*
 * arch/xtensa/mm/cache.c
 *
 * Rust translation of the original implementation source.
 */

/* The original includes and architecture configuration are supplied externally. */

/*
 * Note:
 * PG_arch_1 is used to track instruction/data cache coherency and cache
 * aliasing, as described in the original source.
 */

#[cfg(feature = "dcache_way_size_gt_page_size")]
#[inline]
unsafe fn kmap_invalidate_coherent(page: *mut page, vaddr: c_ulong) {
    if !DCACHE_ALIAS_EQ(page_to_phys(page), vaddr) {
        let kvaddr: c_ulong;
        if !PageHighMem(page) {
            kvaddr = page_to_virt(page) as c_ulong;
            __invalidate_dcache_page(kvaddr);
        } else {
            kvaddr = TLBTEMP_BASE_1 + (page_to_phys(page) & DCACHE_ALIAS_MASK);
            preempt_disable();
            __invalidate_dcache_page_alias(kvaddr, page_to_phys(page));
            preempt_enable();
        }
    }
}

#[cfg(feature = "dcache_way_size_gt_page_size")]
#[inline]
unsafe fn coherent_kvaddr(page: *mut page, base: c_ulong, vaddr: c_ulong,
                          paddr: *mut c_ulong) -> *mut c_void {
    *paddr = page_to_phys(page);
    (base + (vaddr & DCACHE_ALIAS_MASK)) as *mut c_void
}

#[cfg(feature = "dcache_way_size_gt_page_size")]
pub unsafe fn clear_user_highpage(page: *mut page, vaddr: c_ulong) {
    let folio = page_folio(page);
    let mut paddr: c_ulong = 0;
    let kvaddr = coherent_kvaddr(page, TLBTEMP_BASE_1, vaddr, &mut paddr);
    preempt_disable();
    kmap_invalidate_coherent(page, vaddr);
    set_bit(PG_arch_1, folio_flags(folio, 0));
    clear_page_alias(kvaddr, paddr);
    preempt_enable();
}

#[cfg(feature = "dcache_way_size_gt_page_size")]
pub unsafe fn copy_user_highpage(dst: *mut page, src: *mut page, vaddr: c_ulong,
                                 vma: *mut vm_area_struct) {
    let folio = page_folio(dst);
    let mut dst_paddr: c_ulong = 0;
    let mut src_paddr: c_ulong = 0;
    let dst_vaddr = coherent_kvaddr(dst, TLBTEMP_BASE_1, vaddr, &mut dst_paddr);
    let src_vaddr = coherent_kvaddr(src, TLBTEMP_BASE_2, vaddr, &mut src_paddr);
    preempt_disable();
    kmap_invalidate_coherent(dst, vaddr);
    set_bit(PG_arch_1, folio_flags(folio, 0));
    copy_page_alias(dst_vaddr, src_vaddr, dst_paddr, src_paddr);
    preempt_enable();
}

#[cfg(feature = "dcache_way_size_gt_page_size")]
pub unsafe fn flush_dcache_folio(folio: *mut folio) {
    let mapping = folio_flush_mapping(folio);
    if !mapping.is_null() && !mapping_mapped(mapping) {
        if !test_bit(PG_arch_1, &mut (*folio).flags.f) {
            set_bit(PG_arch_1, &mut (*folio).flags.f);
        }
        return;
    }
    let mut phys = folio_pfn(folio) * PAGE_SIZE;
    let mut temp = folio_pos(folio);
    let nr = folio_nr_pages(folio);
    let alias = !(DCACHE_ALIAS_EQ(temp, phys));
    if !alias && mapping.is_null() { return; }
    preempt_disable();
    for _ in 0..nr {
        let mut virt = TLBTEMP_BASE_1 + (phys & DCACHE_ALIAS_MASK);
        __flush_invalidate_dcache_page_alias(virt, phys);
        virt = TLBTEMP_BASE_1 + (temp & DCACHE_ALIAS_MASK);
        if alias { __flush_invalidate_dcache_page_alias(virt, phys); }
        if !mapping.is_null() { __invalidate_icache_page_alias(virt, phys); }
        phys += PAGE_SIZE;
        temp += PAGE_SIZE;
    }
    preempt_enable();
}

#[cfg(feature = "dcache_way_size_gt_page_size")]
pub unsafe fn local_flush_cache_range(_vma: *mut vm_area_struct, _start: c_ulong,
                                      _end: c_ulong) {
    __flush_invalidate_dcache_all();
    __invalidate_icache_all();
}

#[cfg(feature = "dcache_way_size_gt_page_size")]
pub unsafe fn local_flush_cache_page(_vma: *mut vm_area_struct, address: c_ulong,
                                     pfn: c_ulong) {
    let phys = page_to_phys(pfn_to_page(pfn));
    let virt = TLBTEMP_BASE_1 + (address & DCACHE_ALIAS_MASK);
    preempt_disable();
    __flush_invalidate_dcache_page_alias(virt, phys);
    __invalidate_icache_page_alias(virt, phys);
    preempt_enable();
}

pub unsafe fn update_mmu_cache_range(_vmf: *mut vm_fault, vma: *mut vm_area_struct,
                                     addr: c_ulong, ptep: *mut pte_t, mut nr: c_uint) {
    let pfn = pte_pfn(*ptep);
    if !pfn_valid(pfn) { return; }
    let folio = page_folio(pfn_to_page(pfn));
    for i in 0..nr { flush_tlb_page(vma, addr + i * PAGE_SIZE); }
    nr = folio_nr_pages(folio);

    #[cfg(feature = "dcache_way_size_gt_page_size")]
    {
        if !folio_test_reserved(folio) && test_bit(PG_arch_1, &mut (*folio).flags.f) {
            let mut phys = folio_pfn(folio) * PAGE_SIZE;
            preempt_disable();
            for _ in 0..nr {
                let tmp = TLBTEMP_BASE_1 + (phys & DCACHE_ALIAS_MASK);
                __flush_invalidate_dcache_page_alias(tmp, phys);
                let tmp = TLBTEMP_BASE_1 + (addr & DCACHE_ALIAS_MASK);
                __flush_invalidate_dcache_page_alias(tmp, phys);
                __invalidate_icache_page_alias(tmp, phys);
                phys += PAGE_SIZE;
            }
            preempt_enable();
            clear_bit(PG_arch_1, &mut (*folio).flags.f);
        }
    }
    #[cfg(not(feature = "dcache_way_size_gt_page_size"))]
    {
        if !folio_test_reserved(folio) && !test_bit(PG_arch_1, &mut (*folio).flags.f)
            && ((*vma).vm_flags & VM_EXEC) != 0 {
            for i in 0..nr {
                let paddr = kmap_local_folio(folio, i * PAGE_SIZE);
                __flush_dcache_page(paddr as c_ulong);
                __invalidate_icache_page(paddr as c_ulong);
                kunmap_local(paddr);
            }
            set_bit(PG_arch_1, &mut (*folio).flags.f);
        }
    }
}

#[cfg(feature = "dcache_way_size_gt_page_size")]
pub unsafe fn copy_to_user_page(vma: *mut vm_area_struct, page: *mut page,
                                vaddr: c_ulong, dst: *mut c_void,
                                src: *const c_void, len: c_ulong) {
    let phys = page_to_phys(page);
    let alias = !DCACHE_ALIAS_EQ(vaddr, phys);
    if alias {
        let t = TLBTEMP_BASE_1 + (vaddr & DCACHE_ALIAS_MASK);
        preempt_disable(); __flush_invalidate_dcache_page_alias(t, phys); preempt_enable();
    }
    memcpy(dst, src, len);
    if alias {
        let t = TLBTEMP_BASE_1 + (vaddr & DCACHE_ALIAS_MASK);
        preempt_disable();
        __flush_invalidate_dcache_range(dst as c_ulong, len);
        if ((*vma).vm_flags & VM_EXEC) != 0 { __invalidate_icache_page_alias(t, phys); }
        preempt_enable();
    } else if ((*vma).vm_flags & VM_EXEC) != 0 {
        __flush_dcache_range(dst as c_ulong, len);
        __invalidate_icache_range(dst as c_ulong, len);
    }
}

#[cfg(feature = "dcache_way_size_gt_page_size")]
pub unsafe fn copy_from_user_page(_vma: *mut vm_area_struct, page: *mut page,
                                  vaddr: c_ulong, dst: *mut c_void,
                                  src: *const c_void, len: c_ulong) {
    let phys = page_to_phys(page);
    if !DCACHE_ALIAS_EQ(vaddr, phys) {
        let t = TLBTEMP_BASE_1 + (vaddr & DCACHE_ALIAS_MASK);
        preempt_disable(); __flush_invalidate_dcache_page_alias(t, phys); preempt_enable();
    }
    memcpy(dst, src, len);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
