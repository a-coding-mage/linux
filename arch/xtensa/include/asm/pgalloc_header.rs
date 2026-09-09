/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * include/asm-xtensa/pgalloc.h
 *
 * Copyright (C) 2001-2007 Tensilica Inc.
 */

/* CONFIG_MMU conditional preserved from the original header. */

/* __HAVE_ARCH_PTE_ALLOC_ONE_KERNEL */
/* __HAVE_ARCH_PTE_ALLOC_ONE */
/* Dependency: asm-generic/pgalloc.h */

/*
 * Allocating and freeing a pmd is trivial: the 1-entry pmd is
 * inside the pgd, so has no extra memory associated with it.
 */

macro_rules! pmd_populate_kernel {
    ($mm:expr, $pmdp:expr, $ptep:expr) => {
        pmd_val(*$pmdp) = ($ptep as usize)
    };
}

macro_rules! pmd_populate {
    ($mm:expr, $pmdp:expr, $page:expr) => {
        pmd_val(*$pmdp) = (page_to_virt($page) as usize)
    };
}

#[inline]
unsafe fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t {
    __pgd_alloc(mm, 0)
}

#[inline]
unsafe fn ptes_clear(mut ptep: *mut pte_t) {
    let mut i: i32 = 0;
    while i < PTRS_PER_PTE {
        pte_clear(core::ptr::null_mut(), 0, ptep.add(i as usize));
        i += 1;
    }
}

#[inline]
unsafe fn pte_alloc_one_kernel(mm: *mut mm_struct) -> *mut pte_t {
    let ptep: *mut pte_t = __pte_alloc_one_kernel(mm);
    if ptep.is_null() {
        return core::ptr::null_mut();
    }
    ptes_clear(ptep);
    ptep
}

#[inline]
unsafe fn pte_alloc_one(mm: *mut mm_struct) -> pgtable_t {
    let page: *mut page = __pte_alloc_one(mm, GFP_PGTABLE_USER);
    if page.is_null() {
        return core::ptr::null_mut();
    }
    ptes_clear(page_address(page));
    page
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
