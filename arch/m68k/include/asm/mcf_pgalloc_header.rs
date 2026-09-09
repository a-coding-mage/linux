/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by asm/tlb.h and asm/tlbflush.h remain external.

pub unsafe fn pte_free_kernel(mm: *mut mm_struct, pte: *mut pte_t) {
    pagetable_dtor_free(virt_to_ptdesc(pte));
}

unsafe extern "C" {
    pub static bad_pmd_string: core::ffi::c_char;
}

pub unsafe fn pte_alloc_one_kernel(mm: *mut mm_struct) -> *mut pte_t {
    let ptdesc = pagetable_alloc(
        (GFP_DMA | __GFP_ZERO) & !__GFP_HIGHMEM,
        0,
    );

    if ptdesc.is_null() {
        return core::ptr::null_mut();
    }
    if !pagetable_pte_ctor(mm, ptdesc) {
        pagetable_free(ptdesc);
        return core::ptr::null_mut();
    }

    ptdesc_address(ptdesc)
}

pub unsafe fn pmd_alloc_kernel(
    pgd: *mut pgd_t,
    address: core::ffi::c_ulong,
) -> *mut pmd_t {
    pgd as *mut pmd_t
}

pub unsafe fn pmd_populate(
    mm: *mut mm_struct,
    pmd: *mut pmd_t,
    pte: *mut pte_t,
) {
    (*pmd).val = pte as core::ffi::c_ulong;
}

pub unsafe fn pmd_populate_kernel(
    mm: *mut mm_struct,
    pmd: *mut pmd_t,
    pte: *mut pte_t,
) {
    pmd_populate(mm, pmd, pte);
}

pub unsafe fn __pte_free_tlb(
    tlb: *mut mmu_gather,
    pgtable: pgtable_t,
    address: core::ffi::c_ulong,
) {
    let ptdesc = virt_to_ptdesc(pgtable);

    pagetable_dtor(ptdesc);
    pagetable_free(ptdesc);
}

pub unsafe fn pte_alloc_one(mm: *mut mm_struct) -> pgtable_t {
    let ptdesc = pagetable_alloc(GFP_DMA | __GFP_ZERO, 0);
    let mut pte: *mut pte_t;

    if ptdesc.is_null() {
        return core::ptr::null_mut();
    }
    if !pagetable_pte_ctor(mm, ptdesc) {
        pagetable_free(ptdesc);
        return core::ptr::null_mut();
    }

    pte = ptdesc_address(ptdesc);
    pte
}

pub unsafe fn pte_free(mm: *mut mm_struct, pgtable: pgtable_t) {
    let ptdesc = virt_to_ptdesc(pgtable);

    pagetable_dtor(ptdesc);
    pagetable_free(ptdesc);
}

/*
 * In our implementation, each pgd entry contains 1 pmd that is never allocated
 * or freed.  pgd_present is always 1, so this should never be called. -NL
 */
pub unsafe fn pmd_free(mm: *mut mm_struct, pmd: *mut pmd_t) {
    BUG!();
}

pub unsafe fn pgd_free(mm: *mut mm_struct, pgd: *mut pgd_t) {
    pagetable_dtor_free(virt_to_ptdesc(pgd));
}

pub unsafe fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t {
    let mut new_pgd: *mut pgd_t;
    let ptdesc = pagetable_alloc(
        (GFP_DMA | __GFP_NOWARN) & !__GFP_HIGHMEM,
        0,
    );

    if ptdesc.is_null() {
        return core::ptr::null_mut();
    }
    pagetable_pgd_ctor(ptdesc);
    new_pgd = ptdesc_address(ptdesc);

    memcpy(
        new_pgd as *mut core::ffi::c_void,
        swapper_pg_dir as *const core::ffi::c_void,
        PTRS_PER_PGD * core::mem::size_of::<pgd_t>(),
    );
    memset(
        new_pgd as *mut core::ffi::c_void,
        0,
        PAGE_OFFSET >> PGDIR_SHIFT,
    );
    new_pgd
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
