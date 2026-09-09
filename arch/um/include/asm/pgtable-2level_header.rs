/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2000, 2001, 2002 Jeff Dike (jdike@karaya.com)
 * Copyright 2003 PathScale, Inc.
 * Derived from include/asm-i386/pgtable.h
 */

// Dependency supplied by the generic page-table implementation:
// asm-generic/pgtable-nopmd.h

/* PGDIR_SHIFT determines what a third-level page table entry can map */

pub const PGDIR_SHIFT: usize = 22;
pub const PGDIR_SIZE: usize = 1usize << PGDIR_SHIFT;
pub const PGDIR_MASK: usize = !(PGDIR_SIZE - 1);

/*
 * entries per page directory level: the i386 is two-level, so
 * we don't really have any PMD directory physically.
 */
pub const PTRS_PER_PTE: usize = 1024;
pub const USER_PTRS_PER_PGD: usize = (TASK_SIZE + (PGDIR_SIZE - 1)) / PGDIR_SIZE;
pub const PTRS_PER_PGD: usize = 1024;

#[macro_export]
macro_rules! pte_ERROR {
    ($e:expr) => {
        printk!("{}:{}: bad pte {:p}({:08lx}).\n", file!(), line!(), &$e, pte_val($e))
    };
}

#[macro_export]
macro_rules! pgd_ERROR {
    ($e:expr) => {
        printk!("{}:{}: bad pgd {:p}({:08lx}).\n", file!(), line!(), &$e, pgd_val($e))
    };
}

pub unsafe fn pgd_needsync(pgd: pgd_t) -> i32 {
    0
}

pub unsafe fn pgd_mkuptodate(pgd: pgd_t) {
}

#[macro_export]
macro_rules! set_pmd {
    ($pmdptr:expr, $pmdval:expr) => {
        *$pmdptr = $pmdval
    };
}

#[macro_export]
macro_rules! pte_pfn {
    ($x:expr) => {
        phys_to_pfn(pte_val($x))
    };
}

#[macro_export]
macro_rules! pfn_pmd {
    ($pfn:expr, $prot:expr) => {
        __pmd(pfn_to_phys($pfn) | pgprot_val($prot))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
