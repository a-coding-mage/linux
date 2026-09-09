// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mm/pgd.c
 *
 *  Copyright (C) 1998-2005 Russell King
 */

// C dependencies supplied by the surrounding kernel translation unit.

#[cfg(CONFIG_ARM_LPAE)]
unsafe fn _pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t {
    kmalloc_objs::<pgd_t>(PTRS_PER_PGD, GFP_KERNEL | __GFP_ZERO)
}

#[cfg(CONFIG_ARM_LPAE)]
unsafe fn _pgd_free(_mm: *mut mm_struct, pgd: *mut pgd_t) {
    kfree(pgd);
}

#[cfg(not(CONFIG_ARM_LPAE))]
unsafe fn _pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t {
    __pgd_alloc(mm, 2)
}

#[cfg(not(CONFIG_ARM_LPAE))]
unsafe fn _pgd_free(mm: *mut mm_struct, pgd: *mut pgd_t) {
    __pgd_free(mm, pgd)
}

/*
 * need to get a 16k page for level 1
 */
pub unsafe fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t {
    let (mut new_pgd, mut init_pgd): (*mut pgd_t, *mut pgd_t);
    let (mut new_p4d, mut init_p4d): (*mut p4d_t, *mut p4d_t);
    let (mut new_pud, mut init_pud): (*mut pud_t, *mut pud_t);
    let (mut new_pmd, mut init_pmd): (*mut pmd_t, *mut pmd_t);
    let (mut new_pte, mut init_pte): (*mut pte_t, *mut pte_t);

    new_pgd = _pgd_alloc(mm);
    if new_pgd.is_null() { return core::ptr::null_mut(); }

    /* Copy over the kernel and IO PGD entries. */
    init_pgd = pgd_offset_k(0);
    memcpy(new_pgd.add(USER_PTRS_PER_PGD), init_pgd.add(USER_PTRS_PER_PGD),
           (PTRS_PER_PGD - USER_PTRS_PER_PGD) * core::mem::size_of::<pgd_t>());
    clean_dcache_area(new_pgd.cast(), PTRS_PER_PGD * core::mem::size_of::<pgd_t>());

    #[cfg(CONFIG_ARM_LPAE)]
    {
        /* Allocate PMD table for modules and pkmap mappings. */
        new_p4d = p4d_alloc(mm, new_pgd.add(pgd_index(MODULES_VADDR)), MODULES_VADDR);
        if new_p4d.is_null() { _pgd_free(mm, new_pgd); return core::ptr::null_mut(); }
        new_pud = pud_alloc(mm, new_p4d, MODULES_VADDR);
        if new_pud.is_null() { p4d_free(mm, new_p4d); _pgd_free(mm, new_pgd); return core::ptr::null_mut(); }
        new_pmd = pmd_alloc(mm, new_pud, 0);
        if new_pmd.is_null() { pud_free(mm, new_pud); p4d_free(mm, new_p4d); _pgd_free(mm, new_pgd); return core::ptr::null_mut(); }

        #[cfg(CONFIG_KASAN)]
        {
            /* Copy PMD table for KASAN shadow mappings. */
            init_pgd = pgd_offset_k(TASK_SIZE);
            init_p4d = p4d_offset(init_pgd, TASK_SIZE);
            init_pud = pud_offset(init_p4d, TASK_SIZE);
            init_pmd = pmd_offset(init_pud, TASK_SIZE);
            new_pmd = pmd_offset(new_pud, TASK_SIZE);
            memcpy(new_pmd, init_pmd,
                   (pmd_index(MODULES_VADDR) - pmd_index(TASK_SIZE)) * core::mem::size_of::<pmd_t>());
            clean_dcache_area(new_pmd.cast(), PTRS_PER_PMD * core::mem::size_of::<pmd_t>());
        }
    }

    if !vectors_high() {
        /* On ARM, first page must always be allocated since it contains the machine vectors. */
        new_p4d = p4d_alloc(mm, new_pgd, 0);
        if new_p4d.is_null() { _pgd_free(mm, new_pgd); return core::ptr::null_mut(); }
        new_pud = pud_alloc(mm, new_p4d, 0);
        if new_pud.is_null() { p4d_free(mm, new_p4d); _pgd_free(mm, new_pgd); return core::ptr::null_mut(); }
        new_pmd = pmd_alloc(mm, new_pud, 0);
        if new_pmd.is_null() { pud_free(mm, new_pud); p4d_free(mm, new_p4d); _pgd_free(mm, new_pgd); return core::ptr::null_mut(); }
        new_pte = pte_alloc_map(mm, new_pmd, 0);
        if new_pte.is_null() { pmd_free(mm, new_pmd); mm_dec_nr_pmds(mm); pud_free(mm, new_pud); p4d_free(mm, new_p4d); _pgd_free(mm, new_pgd); return core::ptr::null_mut(); }

        #[cfg(not(CONFIG_ARM_LPAE))]
        {
            *pmd_val_mut(new_pmd) &= !PMD_DOMAIN_MASK;
            *pmd_val_mut(new_pmd) |= PMD_DOMAIN(DOMAIN_VECTORS);
        }
        init_p4d = p4d_offset(init_pgd, 0);
        init_pud = pud_offset(init_p4d, 0);
        init_pmd = pmd_offset(init_pud, 0);
        init_pte = pte_offset_map(init_pmd, 0);
        set_pte_ext(new_pte.add(0), *init_pte.add(0), 0);
        set_pte_ext(new_pte.add(1), *init_pte.add(1), 0);
        pte_unmap(init_pte);
        pte_unmap(new_pte);
    }
    new_pgd
}

pub unsafe fn pgd_free(mm: *mut mm_struct, pgd_base: *mut pgd_t) {
    if pgd_base.is_null() { return; }
    let mut pgd = pgd_base.add(pgd_index(0));
    if pgd_none_or_clear_bad(pgd) { _pgd_free(mm, pgd_base); return; }
    let mut p4d = p4d_offset(pgd, 0);
    if p4d_none_or_clear_bad(p4d) { pgd_clear(pgd); p4d_free(mm, p4d); _pgd_free(mm, pgd_base); return; }
    let mut pud = pud_offset(p4d, 0);
    if pud_none_or_clear_bad(pud) { p4d_clear(p4d); pud_free(mm, pud); pgd_clear(pgd); p4d_free(mm, p4d); _pgd_free(mm, pgd_base); return; }
    let mut pmd = pmd_offset(pud, 0);
    if pmd_none_or_clear_bad(pmd) { pud_clear(pud); pmd_free(mm, pmd); mm_dec_nr_pmds(mm); p4d_clear(p4d); pud_free(mm, pud); pgd_clear(pgd); p4d_free(mm, p4d); _pgd_free(mm, pgd_base); return; }
    let pte = pmd_pgtable(*pmd);
    pmd_clear(pmd); pte_free(mm, pte); mm_dec_nr_ptes(mm);
    pud_clear(pud); pmd_free(mm, pmd); mm_dec_nr_pmds(mm);
    p4d_clear(p4d); pud_free(mm, pud);
    pgd_clear(pgd); p4d_free(mm, p4d);

    #[cfg(CONFIG_ARM_LPAE)]
    for i in 0..PTRS_PER_PGD {
        pgd = pgd_base.add(i);
        if pgd_none_or_clear_bad(pgd) || (pgd_val(*pgd) & L_PGD_SWAPPER) != 0 { continue; }
        p4d = p4d_offset(pgd, 0);
        if p4d_none_or_clear_bad(p4d) { continue; }
        pud = pud_offset(p4d, 0);
        if pud_none_or_clear_bad(pud) { continue; }
        pmd = pmd_offset(pud, 0);
        pud_clear(pud); pmd_free(mm, pmd); mm_dec_nr_pmds(mm);
        p4d_clear(p4d); pud_free(mm, pud); mm_dec_nr_puds(mm);
        pgd_clear(pgd); p4d_free(mm, p4d);
    }
    _pgd_free(mm, pgd_base);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
