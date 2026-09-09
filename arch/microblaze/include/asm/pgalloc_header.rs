/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2008-2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2008-2009 PetaLogix
 * Copyright (C) 2006 Atmark Techno, Inc.
 */

// Translated from the MicroBlaze Linux header.  The C include dependencies
// and build-time header guard are intentionally omitted; their symbols are
// supplied by the surrounding translation unit.

pub const __HAVE_ARCH_PTE_ALLOC_ONE_KERNEL: bool = true;

extern "C" {
    pub fn __bad_pte(pmd: *mut pmd_t);

    pub fn __pgd_alloc(mm: *mut mm_struct, kernel: i32) -> *mut pgd_t;

    pub fn pte_alloc_one_kernel(mm: *mut mm_struct) -> *mut pte_t;

    pub fn pte_free(mm: *mut mm_struct, pte: *mut pte_t);
}

#[inline]
pub unsafe fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t {
    __pgd_alloc(mm, 0)
}

#[macro_export]
macro_rules! __pte_free_tlb {
    ($tlb:expr, $pte:expr, $addr:expr) => {
        unsafe { $crate::pte_free((*($tlb)).mm, $pte) }
    };
}

#[macro_export]
macro_rules! pmd_populate {
    ($mm:expr, $pmd:expr, $pte:expr) => {
        unsafe {
            pmd_val_set($pmd, page_address($pte) as usize);
        }
    };
}

#[macro_export]
macro_rules! pmd_populate_kernel {
    ($mm:expr, $pmd:expr, $pte:expr) => {
        unsafe {
            pmd_val_set($pmd, $pte as usize);
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
