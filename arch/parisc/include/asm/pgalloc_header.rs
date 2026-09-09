/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux/Rust translation units:
// linux/gfp.h, linux/mm.h, linux/threads.h, asm/processor.h, asm/fixmap.h,
// asm/cache.h, and asm-generic/pgalloc.h.

pub const __HAVE_ARCH_PMD_ALLOC_ONE: bool = true;

/* Allocate the top level pgd (page directory) */
#[inline]
pub unsafe fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t {
    __pgd_alloc(mm, PGD_TABLE_ORDER)
}

/* Three Level Page Table Support for pmd's.
 * The original code is enabled when CONFIG_PGTABLE_LEVELS == 3.
 */
#[cfg(CONFIG_PGTABLE_LEVELS_3)]
#[inline]
pub unsafe fn pud_populate(mm: *mut mm_struct, pud: *mut pud_t, pmd: *mut pmd_t) {
    set_pud(
        pud,
        __pud(
            (PxD_FLAG_PRESENT | PxD_FLAG_VALID)
                .wrapping_add((__pa(pmd as unsigned_long) >> PxD_VALUE_SHIFT) as __u32),
        ),
    );
}

#[cfg(CONFIG_PGTABLE_LEVELS_3)]
#[inline]
pub unsafe fn pmd_alloc_one(mm: *mut mm_struct, address: unsigned_long) -> *mut pmd_t {
    let mut ptdesc: *mut ptdesc;
    let mut gfp: gfp_t = GFP_PGTABLE_USER;

    if mm == &raw mut init_mm {
        gfp = GFP_PGTABLE_KERNEL;
    }
    ptdesc = pagetable_alloc(gfp, PMD_TABLE_ORDER);
    if ptdesc.is_null() {
        return core::ptr::null_mut();
    }
    if !pagetable_pmd_ctor(mm, ptdesc) {
        pagetable_free(ptdesc);
        return core::ptr::null_mut();
    }
    ptdesc_address(ptdesc)
}

#[inline]
pub unsafe fn pmd_populate_kernel(
    mm: *mut mm_struct,
    pmd: *mut pmd_t,
    pte: *mut pte_t,
) {
    set_pmd(
        pmd,
        __pmd(
            (PxD_FLAG_PRESENT | PxD_FLAG_VALID)
                .wrapping_add((__pa(pte as unsigned_long) >> PxD_VALUE_SHIFT) as __u32),
        ),
    );
}

#[inline]
pub unsafe fn pmd_populate(
    mm: *mut mm_struct,
    pmd: *mut pmd_t,
    pte_page: *mut page,
) {
    pmd_populate_kernel(mm, pmd, page_address(pte_page));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
