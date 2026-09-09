/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Translated from the C header; included dependencies and configuration symbols
 * are supplied by the surrounding PowerPC kernel translation. */

#[repr(C)]
pub struct vmemmap_backing {
    pub list: *mut vmemmap_backing,
    pub phys: c_ulong,
    pub virt_addr: c_ulong,
}

extern "C" {
    pub static mut vmemmap_list: *mut vmemmap_backing;

    pub fn pmd_fragment_alloc(mm: *mut mm_struct, addr: c_ulong) -> *mut pmd_t;
    pub fn pmd_fragment_free(pmd: *mut c_ulong);
    pub fn pgtable_free_tlb(tlb: *mut mmu_gather, table: *mut c_void, shift: c_int);
    pub fn __tlb_remove_table(table: *mut c_void);
    pub fn pte_frag_destroy(pte_frag: *mut c_void);
}

#[inline]
pub unsafe fn radix__pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t {
    #[cfg(CONFIG_PPC_64K_PAGES)]
    {
        return __get_free_page(pgtable_gfp_flags(mm, PGALLOC_GFP)) as *mut pgd_t;
    }
    #[cfg(not(CONFIG_PPC_64K_PAGES))]
    {
        let page: *mut page = alloc_pages(
            pgtable_gfp_flags(mm, PGALLOC_GFP | __GFP_RETRY_MAYFAIL),
            4,
        );
        if page.is_null() {
            return core::ptr::null_mut();
        }
        return page_address(page) as *mut pgd_t;
    }
}

#[inline]
pub unsafe fn radix__pgd_free(_mm: *mut mm_struct, pgd: *mut pgd_t) {
    #[cfg(CONFIG_PPC_64K_PAGES)]
    {
        free_page(pgd as c_ulong);
    }
    #[cfg(not(CONFIG_PPC_64K_PAGES))]
    {
        free_pages(pgd as c_ulong, 4);
    }
}

#[inline]
pub unsafe fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t {
    if radix_enabled() {
        return radix__pgd_alloc(mm);
    }

    let pgd = kmem_cache_alloc(PGT_CACHE(PGD_INDEX_SIZE), pgtable_gfp_flags(mm, GFP_KERNEL))
        as *mut pgd_t;
    if unlikely(!pgd.is_null()) {
        return pgd;
    }

    /*
     * Don't scan the PGD for pointers, it contains references to PUDs but
     * those references are not full pointers and so can't be recognised by
     * kmemleak.
     */
    kmemleak_no_scan(pgd as *mut c_void);

    /*
     * With hugetlb, we don't clear the second half of the page table.
     * If we share the same slab cache with the pmd or pud level table,
     * we need to make sure we zero out the full table on alloc.
     * With 4K we don't store slot in the second half. Hence we don't
     * need to do this for 4k.
     */
    #[cfg(all(CONFIG_HUGETLB_PAGE, CONFIG_PPC_64K_PAGES))]
    if H_PGD_INDEX_SIZE == H_PUD_CACHE_INDEX {
        memset(pgd as *mut c_void, 0, PGD_TABLE_SIZE);
    }
    pgd
}

#[inline]
pub unsafe fn pgd_free(mm: *mut mm_struct, pgd: *mut pgd_t) {
    if radix_enabled() {
        return radix__pgd_free(mm, pgd);
    }
    kmem_cache_free(PGT_CACHE(PGD_INDEX_SIZE), pgd as *mut c_void);
}

#[inline]
pub unsafe fn p4d_populate(_mm: *mut mm_struct, pgd: *mut p4d_t, pud: *mut pud_t) {
    *pgd = __p4d(__pgtable_ptr_val(pud) | PGD_VAL_BITS);
}

#[inline]
pub unsafe fn pud_alloc_one(mm: *mut mm_struct, _addr: c_ulong) -> *mut pud_t {
    let pud = kmem_cache_alloc(PGT_CACHE(PUD_CACHE_INDEX), pgtable_gfp_flags(mm, GFP_KERNEL))
        as *mut pud_t;
    kmemleak_ignore(pud as *mut c_void);
    pud
}

#[inline]
pub unsafe fn __pud_free(pud: *mut pud_t) {
    let page = virt_to_page(pud as *mut c_void);
    if PageReserved(page) && !PageSlab(page) {
        free_reserved_page(page);
    } else {
        kmem_cache_free(PGT_CACHE(PUD_CACHE_INDEX), pud as *mut c_void);
    }
}

#[inline]
pub unsafe fn pud_free(_mm: *mut mm_struct, pud: *mut pud_t) {
    __pud_free(pud);
}

#[inline]
pub unsafe fn pud_populate(_mm: *mut mm_struct, pud: *mut pud_t, pmd: *mut pmd_t) {
    *pud = __pud(__pgtable_ptr_val(pmd) | PUD_VAL_BITS);
}

#[inline]
pub unsafe fn __pud_free_tlb(tlb: *mut mmu_gather, pud: *mut pud_t, _address: c_ulong) {
    pgtable_free_tlb(tlb, pud as *mut c_void, PUD_INDEX);
}

#[inline]
pub unsafe fn pmd_alloc_one(mm: *mut mm_struct, addr: c_ulong) -> *mut pmd_t {
    pmd_fragment_alloc(mm, addr)
}

#[inline]
pub unsafe fn pmd_free(_mm: *mut mm_struct, pmd: *mut pmd_t) {
    pmd_fragment_free(pmd as *mut c_ulong);
}

#[inline]
pub unsafe fn __pmd_free_tlb(tlb: *mut mmu_gather, pmd: *mut pmd_t, _address: c_ulong) {
    pgtable_free_tlb(tlb, pmd as *mut c_void, PMD_INDEX);
}

#[inline]
pub unsafe fn pmd_populate_kernel(_mm: *mut mm_struct, pmd: *mut pmd_t, pte: *mut pte_t) {
    *pmd = __pmd(__pgtable_ptr_val(pte) | PMD_VAL_BITS);
}

#[inline]
pub unsafe fn pmd_populate(_mm: *mut mm_struct, pmd: *mut pmd_t, pte_page: pgtable_t) {
    *pmd = __pmd(__pgtable_ptr_val(pte_page) | PMD_VAL_BITS);
}

#[inline]
pub unsafe fn __pte_free_tlb(tlb: *mut mmu_gather, table: pgtable_t, _address: c_ulong) {
    pgtable_free_tlb(tlb, table as *mut c_void, PTE_INDEX);
}

extern "C" {
    pub static mut direct_pages_count: [atomic_long_t; MMU_PAGE_COUNT];
}

#[inline]
pub unsafe fn update_page_count(psize: c_int, count: c_long) {
    if IS_ENABLED(CONFIG_PROC_FS) {
        atomic_long_add(count, &mut direct_pages_count[psize as usize]);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
