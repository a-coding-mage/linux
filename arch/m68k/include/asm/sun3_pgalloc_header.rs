/* SPDX-License-Identifier: GPL-2.0 */
/* sun3_pgalloc.h --
 * reorganization around 2.3.39, routines moved from sun3_pgtable.h
 *
 *
 * 02/27/2002 -- Modified to support "highpte" implementation in 2.5.5 (Sam)
 *
 * moved 1/26/2000 Sam Creasey
 */

// Dependency intent from <asm/tlb.h> and <asm-generic/pgalloc.h> is preserved
// through the externally supplied symbols referenced below.

extern "C" {
    pub static bad_pmd_string: [core::ffi::c_char; 0];
}

#[macro_export]
macro_rules! __pte_free_tlb {
    ($tlb:expr, $pte:expr, $addr:expr) => {
        tlb_remove_ptdesc(($tlb), page_ptdesc($pte))
    };
}

#[inline]
pub unsafe fn pmd_populate_kernel(mm: *mut mm_struct, pmd: *mut pmd_t, pte: *mut pte_t) {
    let _ = mm;
    (*pmd).val = __pa(pte as usize as ::core::ffi::c_ulong);
}

#[inline]
pub unsafe fn pmd_populate(mm: *mut mm_struct, pmd: *mut pmd_t, page: pgtable_t) {
    let _ = mm;
    (*pmd).val = __pa(page_address(page) as usize as ::core::ffi::c_ulong);
}

/*
 * allocating and freeing a pmd is trivial: the 1-entry pmd is
 * inside the pgd, so has no extra memory associated with it.
 */
#[macro_export]
macro_rules! pmd_free {
    ($mm:expr, $x:expr) => {{
        let _ = ($mm, $x);
    }};
}

#[inline]
pub unsafe fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t {
    let new_pgd: *mut pgd_t;

    new_pgd = __pgd_alloc(mm, 0);
    if likely(!new_pgd.is_null()) {
        memcpy(new_pgd as *mut core::ffi::c_void,
               swapper_pg_dir as *const core::ffi::c_void,
               PAGE_SIZE);
        memset(new_pgd as *mut core::ffi::c_void,
               0,
               (PAGE_OFFSET >> PGDIR_SHIFT) as usize);
    }
    new_pgd
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
