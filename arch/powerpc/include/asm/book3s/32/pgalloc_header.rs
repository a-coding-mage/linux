/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header. Symbols supplied by included kernel headers
// remain external dependencies.

pub unsafe fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t {
    let pgd = kmem_cache_alloc(
        PGT_CACHE(PGD_INDEX_SIZE),
        pgtable_gfp_flags(mm, GFP_KERNEL),
    ) as *mut pgd_t;

    // CONFIG_PPC_BOOK3S_603 controls this C preprocessor conditional.
    #[cfg(feature = "CONFIG_PPC_BOOK3S_603")]
    {
        memcpy(
            pgd.add(USER_PTRS_PER_PGD),
            swapper_pg_dir.add(USER_PTRS_PER_PGD),
            (MAX_PTRS_PER_PGD - USER_PTRS_PER_PGD) * core::mem::size_of::<pgd_t>(),
        );
    }
    pgd
}

pub unsafe fn pgd_free(_mm: *mut mm_struct, pgd: *mut pgd_t) {
    kmem_cache_free(PGT_CACHE(PGD_INDEX_SIZE), pgd as *mut core::ffi::c_void);
}

/*
 * We don't have any real pmd's, and this code never triggers because
 * the pgd will always be present..
 */
// #define pmd_alloc_one(mm,address) ({ BUG(); ((pmd_t *)2); })
#[inline]
pub unsafe fn pmd_free(_mm: *mut mm_struct, _x: *mut pmd_t) {}

#[inline]
pub unsafe fn __pmd_free_tlb(
    _tlb: *mut mmu_gather,
    _x: *mut pmd_t,
    _a: usize,
) {
}

// #define pgd_populate(mm, pmd, pte) BUG()

pub unsafe fn pmd_populate_kernel(
    _mm: *mut mm_struct,
    pmdp: *mut pmd_t,
    pte: *mut pte_t,
) {
    *pmdp = __pmd(__pa(pte) | _PMD_PRESENT);
}

pub unsafe fn pmd_populate(
    _mm: *mut mm_struct,
    pmdp: *mut pmd_t,
    pte_page: pgtable_t,
) {
    *pmdp = __pmd(__pa(pte_page) | _PMD_PRESENT);
}

pub unsafe fn pgtable_free(table: *mut core::ffi::c_void, index_size: u32) {
    if index_size == 0 {
        pte_fragment_free(table as *mut u64, 0);
    } else {
        BUG_ON(index_size > MAX_PGTABLE_INDEX_SIZE);
        kmem_cache_free(PGT_CACHE(index_size), table);
    }
}

pub unsafe fn pgtable_free_tlb(
    tlb: *mut mmu_gather,
    table: *mut core::ffi::c_void,
    shift: i32,
) {
    let mut pgf = table as usize;
    BUG_ON(shift > MAX_PGTABLE_INDEX_SIZE as i32);
    pgf |= shift as usize;
    tlb_remove_table(tlb, pgf as *mut core::ffi::c_void);
}

pub unsafe fn __tlb_remove_table(_table: *mut core::ffi::c_void) {
    let table = ((_table as usize) & !MAX_PGTABLE_INDEX_SIZE as usize)
        as *mut core::ffi::c_void;
    let shift = (_table as usize & MAX_PGTABLE_INDEX_SIZE as usize) as u32;

    pgtable_free(table, shift);
}

pub unsafe fn __pte_free_tlb(
    tlb: *mut mmu_gather,
    table: pgtable_t,
    _address: u64,
) {
    pgtable_free_tlb(tlb, table as *mut core::ffi::c_void, 0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
