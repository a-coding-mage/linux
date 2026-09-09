/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Rust translation of arch/arm64/include/asm/hugetlb.h.
 * C header dependencies are supplied by the surrounding translation unit.
 */

/* CONFIG_ARCH_ENABLE_HUGEPAGE_MIGRATION */
#[cfg(CONFIG_ARCH_ENABLE_HUGEPAGE_MIGRATION)]
extern "C" {
    pub fn arch_hugetlb_migration_supported(h: *mut hstate) -> bool;
}

#[inline]
pub unsafe fn arch_clear_hugetlb_flags(folio: *mut folio) {
    clear_bit(PG_dcache_clean, &mut (*folio).flags.f);

    /* CONFIG_ARM64_MTE */
    #[cfg(CONFIG_ARM64_MTE)]
    if system_supports_mte() {
        clear_bit(PG_mte_tagged, &mut (*folio).flags.f);
        clear_bit(PG_mte_lock, &mut (*folio).flags.f);
    }
}

extern "C" {
    pub fn arch_make_huge_pte(entry: pte_t, shift: ::core::ffi::c_uint, flags: vm_flags_t) -> pte_t;

    pub fn set_huge_pte_at(
        mm: *mut mm_struct,
        addr: ::core::ffi::c_ulong,
        ptep: *mut pte_t,
        pte: pte_t,
        sz: ::core::ffi::c_ulong,
    );

    pub fn huge_ptep_set_access_flags(
        vma: *mut vm_area_struct,
        addr: ::core::ffi::c_ulong,
        ptep: *mut pte_t,
        pte: pte_t,
        dirty: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub fn huge_ptep_get_and_clear(
        mm: *mut mm_struct,
        addr: ::core::ffi::c_ulong,
        ptep: *mut pte_t,
        sz: ::core::ffi::c_ulong,
    ) -> pte_t;

    pub fn huge_ptep_set_wrprotect(
        mm: *mut mm_struct,
        addr: ::core::ffi::c_ulong,
        ptep: *mut pte_t,
    );

    pub fn huge_ptep_clear_flush(
        vma: *mut vm_area_struct,
        addr: ::core::ffi::c_ulong,
        ptep: *mut pte_t,
    ) -> pte_t;

    pub fn huge_pte_clear(
        mm: *mut mm_struct,
        addr: ::core::ffi::c_ulong,
        ptep: *mut pte_t,
        sz: ::core::ffi::c_ulong,
    );

    pub fn huge_ptep_get(
        mm: *mut mm_struct,
        addr: ::core::ffi::c_ulong,
        ptep: *mut pte_t,
    ) -> pte_t;

    pub fn huge_ptep_modify_prot_start(
        vma: *mut vm_area_struct,
        addr: ::core::ffi::c_ulong,
        ptep: *mut pte_t,
    ) -> pte_t;

    pub fn huge_ptep_modify_prot_commit(
        vma: *mut vm_area_struct,
        addr: ::core::ffi::c_ulong,
        ptep: *mut pte_t,
        old_pte: pte_t,
        new_pte: pte_t,
    );
}

/* Declarations from <asm-generic/hugetlb.h> are supplied externally. */

#[inline]
pub unsafe fn __flush_hugetlb_tlb_range(
    vma: *mut vm_area_struct,
    start: ::core::ffi::c_ulong,
    end: ::core::ffi::c_ulong,
    stride: ::core::ffi::c_ulong,
    flags: tlbf_t,
) {
    match stride {
        /* __PAGETABLE_PMD_FOLDED is a build-time configuration condition. */
        #[cfg(not(__PAGETABLE_PMD_FOLDED))]
        PUD_SIZE => __flush_tlb_range(vma, start, end, PUD_SIZE, 1, flags),
        CONT_PMD_SIZE | PMD_SIZE => __flush_tlb_range(vma, start, end, PMD_SIZE, 2, flags),
        CONT_PTE_SIZE => __flush_tlb_range(vma, start, end, PAGE_SIZE, 3, flags),
        _ => __flush_tlb_range(vma, start, end, PAGE_SIZE, TLBI_TTL_UNKNOWN, flags),
    }
}

#[inline]
pub unsafe fn flush_hugetlb_tlb_range(
    vma: *mut vm_area_struct,
    start: ::core::ffi::c_ulong,
    end: ::core::ffi::c_ulong,
) {
    let stride = huge_page_size(hstate_vma(vma));
    __flush_hugetlb_tlb_range(vma, start, end, stride, TLBF_NONE);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
