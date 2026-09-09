/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2000, 2001, 2002 Jeff Dike (jdike@karaya.com)
 * Copyright 2003 PathScale, Inc.
 * Derived from include/asm-i386/pgalloc.h and include/asm-i386/pgtable.h
 */

// C dependencies: <linux/mm.h> and <asm-generic/pgalloc.h>.

macro_rules! pmd_populate_kernel {
    ($mm:expr, $pmd:expr, $pte:expr) => {
        set_pmd($pmd, __pmd(_PAGE_TABLE + (__pa($pte) as usize)))
    };
}

macro_rules! pmd_populate {
    ($mm:expr, $pmd:expr, $pte:expr) => {
        set_pmd(
            $pmd,
            __pmd(
                _PAGE_TABLE
                    + (((page_to_pfn($pte) as u64) << (PAGE_SHIFT as u64)) as usize),
            ),
        )
    };
}

/*
 * Allocate and free page tables.
 */
extern "C" {
    fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t;
}

macro_rules! __pte_free_tlb {
    ($tlb:expr, $pte:expr, $address:expr) => {
        tlb_remove_ptdesc($tlb, page_ptdesc($pte))
    };
}

// C preprocessor condition: #if CONFIG_PGTABLE_LEVELS > 2
macro_rules! __pmd_free_tlb {
    ($tlb:expr, $pmd:expr, $address:expr) => {
        tlb_remove_ptdesc($tlb, virt_to_ptdesc($pmd))
    };
}

// C preprocessor condition: #if CONFIG_PGTABLE_LEVELS > 3
macro_rules! __pud_free_tlb {
    ($tlb:expr, $pud:expr, $address:expr) => {
        tlb_remove_ptdesc($tlb, virt_to_ptdesc($pud))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
