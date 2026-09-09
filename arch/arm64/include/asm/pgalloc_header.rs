/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Based on arch/arm/include/asm/pgalloc.h
 *
 * Copyright (C) 2000-2001 Russell King
 * Copyright (C) 2012 ARM Ltd.
 */

// Dependencies supplied by the surrounding kernel translation:
// asm/pgtable-hwdef.h, asm/processor.h, asm/cacheflush.h,
// asm/tlbflush.h, and asm-generic/pgalloc.h.

pub const PGD_SIZE: usize = PTRS_PER_PGD * core::mem::size_of::<pgd_t>();

// __HAVE_ARCH_PGD_FREE and __HAVE_ARCH_PUD_FREE are defined by this header.

// CONFIG_PGTABLE_LEVELS > 2
#[inline]
pub unsafe fn __pud_populate(pudp: *mut pud_t, pmdp: phys_addr_t, prot: pudval_t) {
    set_pud(pudp, __pud(__phys_to_pud_val(pmdp) | prot));
}

// CONFIG_PGTABLE_LEVELS > 2
#[inline]
pub unsafe fn pud_populate(mm: *mut mm_struct, pudp: *mut pud_t, pmdp: *mut pmd_t) {
    let mut pudval: pudval_t = PUD_TYPE_TABLE | PUD_TABLE_AF;
    pudval |= if mm == core::ptr::addr_of_mut!(init_mm) {
        PUD_TABLE_UXN
    } else {
        PUD_TABLE_PXN
    };
    __pud_populate(pudp, __pa(pmdp), pudval);
}

// CONFIG_PGTABLE_LEVELS <= 2
#[inline]
pub unsafe fn __pud_populate_disabled(
    _pudp: *mut pud_t,
    _pmdp: phys_addr_t,
    _prot: pudval_t,
) {
    BUILD_BUG!();
}

// CONFIG_PGTABLE_LEVELS > 3
#[inline]
pub unsafe fn __p4d_populate(p4dp: *mut p4d_t, pudp: phys_addr_t, prot: p4dval_t) {
    if pgtable_l4_enabled() {
        set_p4d(p4dp, __p4d(__phys_to_p4d_val(pudp) | prot));
    }
}

// CONFIG_PGTABLE_LEVELS > 3
#[inline]
pub unsafe fn p4d_populate(mm: *mut mm_struct, p4dp: *mut p4d_t, pudp: *mut pud_t) {
    let mut p4dval: p4dval_t = P4D_TYPE_TABLE | P4D_TABLE_AF;
    p4dval |= if mm == core::ptr::addr_of_mut!(init_mm) {
        P4D_TABLE_UXN
    } else {
        P4D_TABLE_PXN
    };
    __p4d_populate(p4dp, __pa(pudp), p4dval);
}

// CONFIG_PGTABLE_LEVELS > 3
#[inline]
pub unsafe fn pud_free(mm: *mut mm_struct, pud: *mut pud_t) {
    if !pgtable_l4_enabled() {
        return;
    }
    __pud_free(mm, pud);
}

// CONFIG_PGTABLE_LEVELS <= 3
#[inline]
pub unsafe fn __p4d_populate_disabled(
    _p4dp: *mut p4d_t,
    _pudp: phys_addr_t,
    _prot: p4dval_t,
) {
    BUILD_BUG!();
}

// CONFIG_PGTABLE_LEVELS > 4
#[inline]
pub unsafe fn __pgd_populate(pgdp: *mut pgd_t, p4dp: phys_addr_t, prot: pgdval_t) {
    if pgtable_l5_enabled() {
        set_pgd(pgdp, __pgd(__phys_to_pgd_val(p4dp) | prot));
    }
}

// CONFIG_PGTABLE_LEVELS > 4
#[inline]
pub unsafe fn pgd_populate(mm: *mut mm_struct, pgdp: *mut pgd_t, p4dp: *mut p4d_t) {
    let mut pgdval: pgdval_t = PGD_TYPE_TABLE | PGD_TABLE_AF;
    pgdval |= if mm == core::ptr::addr_of_mut!(init_mm) {
        PGD_TABLE_UXN
    } else {
        PGD_TABLE_PXN
    };
    __pgd_populate(pgdp, __pa(p4dp), pgdval);
}

// CONFIG_PGTABLE_LEVELS <= 4
#[inline]
pub unsafe fn __pgd_populate_disabled(
    _pgdp: *mut pgd_t,
    _p4dp: phys_addr_t,
    _prot: pgdval_t,
) {
    BUILD_BUG!();
}

extern "C" {
    pub fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t;
    pub fn pgd_free(mm: *mut mm_struct, pgdp: *mut pgd_t);
}

#[inline]
pub unsafe fn __pmd_populate(pmdp: *mut pmd_t, ptep: phys_addr_t, prot: pmdval_t) {
    set_pmd(pmdp, __pmd(__phys_to_pmd_val(ptep) | prot));
}

/*
 * Populate the pmdp entry with a pointer to the pte.  This pmd is part
 * of the mm address space.
 */
#[inline]
pub unsafe fn pmd_populate_kernel(mm: *mut mm_struct, pmdp: *mut pmd_t, ptep: *mut pte_t) {
    VM_BUG_ON!(mm != core::ptr::null_mut() && mm != core::ptr::addr_of_mut!(init_mm));
    __pmd_populate(
        pmdp,
        __pa(ptep),
        PMD_TYPE_TABLE | PMD_TABLE_AF | PMD_TABLE_UXN,
    );
}

#[inline]
pub unsafe fn pmd_populate(mm: *mut mm_struct, pmdp: *mut pmd_t, ptep: pgtable_t) {
    VM_BUG_ON!(mm == core::ptr::addr_of_mut!(init_mm));
    __pmd_populate(
        pmdp,
        page_to_phys(ptep),
        PMD_TYPE_TABLE | PMD_TABLE_AF | PMD_TABLE_PXN,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
