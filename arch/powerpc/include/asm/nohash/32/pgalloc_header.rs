/* SPDX-License-Identifier: GPL-2.0 */

// The original header guard and C includes are omitted; their dependencies
// are supplied by the surrounding translation unit.

/*
 * We don't have any real pmd's, and this code never triggers because
 * the pgd will always be present..
 */
/* #define pmd_alloc_one(mm,address)       ({ BUG(); ((pmd_t *)2); }) */
macro_rules! pmd_free {
    ($mm:expr, $x:expr) => {{
        let _ = &$mm;
        let _ = &$x;
    }};
}

macro_rules! __pmd_free_tlb {
    ($tlb:expr, $x:expr, $a:expr) => {{
        let _ = &$tlb;
        let _ = &$x;
        let _ = &$a;
    }};
}

/* #define pgd_populate(mm, pmd, pte)      BUG() */

pub unsafe fn pmd_populate_kernel(
    _mm: *mut mm_struct,
    pmdp: *mut pmd_t,
    pte: *mut pte_t,
) {
    if IS_ENABLED(CONFIG_BOOKE) {
        *pmdp = __pmd((pte as usize) | _PMD_PRESENT);
    } else {
        *pmdp = __pmd(__pa(pte) | _PMD_PRESENT);
    }
}

pub unsafe fn pmd_populate(
    _mm: *mut mm_struct,
    pmdp: *mut pmd_t,
    pte_page: pgtable_t,
) {
    if IS_ENABLED(CONFIG_BOOKE) {
        *pmdp = __pmd((pte_page as usize) | _PMD_PRESENT);
    } else {
        *pmdp = __pmd(__pa(pte_page) | _PMD_USER | _PMD_PRESENT);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
