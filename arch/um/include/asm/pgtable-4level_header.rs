/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright 2003 PathScale Inc
 * Derived from include/asm-i386/pgtable.h
 */

// Dependency: <asm-generic/pgtable-nop4d.h>

/* PGDIR_SHIFT determines what a fourth-level page table entry can map */

pub const PGDIR_SHIFT: usize = 39;
pub const PGDIR_SIZE: usize = 1usize << PGDIR_SHIFT;
pub const PGDIR_MASK: usize = !(PGDIR_SIZE - 1);

/* PUD_SHIFT determines the size of the area a third-level page table can
 * map
 */

pub const PUD_SHIFT: usize = 30;
pub const PUD_SIZE: usize = 1usize << PUD_SHIFT;
pub const PUD_MASK: usize = !(PUD_SIZE - 1);

/* PMD_SHIFT determines the size of the area a second-level page table can
 * map
 */

pub const PMD_SHIFT: usize = 21;
pub const PMD_SIZE: usize = 1usize << PMD_SHIFT;
pub const PMD_MASK: usize = !(PMD_SIZE - 1);

/*
 * entries per page directory level
 */

pub const PTRS_PER_PTE: usize = 512;
pub const PTRS_PER_PMD: usize = 512;
pub const PTRS_PER_PUD: usize = 512;
pub const PTRS_PER_PGD: usize = 512;

pub const USER_PTRS_PER_PGD: usize = (TASK_SIZE + (PGDIR_SIZE - 1)) / PGDIR_SIZE;

macro_rules! pte_ERROR {
    ($e:expr) => {
        printk!("{}:{}: bad pte {:p}({:016x}).\n", file!(), line!(), &$e, pte_val($e))
    };
}
macro_rules! pmd_ERROR {
    ($e:expr) => {
        printk!("{}:{}: bad pmd {:p}({:016x}).\n", file!(), line!(), &$e, pmd_val($e))
    };
}
macro_rules! pud_ERROR {
    ($e:expr) => {
        printk!("{}:{}: bad pud {:p}({:016x}).\n", file!(), line!(), &$e, pud_val($e))
    };
}
macro_rules! pgd_ERROR {
    ($e:expr) => {
        printk!("{}:{}: bad pgd {:p}({:016x}).\n", file!(), line!(), &$e, pgd_val($e))
    };
}

macro_rules! pud_none {
    ($x:expr) => { !(pud_val($x) & !_PAGE_NEEDSYNC) };
}
macro_rules! pud_bad {
    ($x:expr) => { (pud_val($x) & (!PAGE_MASK & !_PAGE_USER)) != _KERNPG_TABLE };
}
macro_rules! pud_present {
    ($x:expr) => { pud_val($x) & _PAGE_PRESENT };
}
macro_rules! pud_populate {
    ($mm:expr, $pud:expr, $pmd:expr) => { set_pud($pud, __pud(_PAGE_TABLE + __pa($pmd))) };
}

macro_rules! set_pud {
    ($pudptr:expr, $pudval:expr) => { *$pudptr = $pudval };
}

macro_rules! p4d_none {
    ($x:expr) => { !(p4d_val($x) & !_PAGE_NEEDSYNC) };
}
macro_rules! p4d_bad {
    ($x:expr) => { (p4d_val($x) & (!PAGE_MASK & !_PAGE_USER)) != _KERNPG_TABLE };
}
macro_rules! p4d_present {
    ($x:expr) => { p4d_val($x) & _PAGE_PRESENT };
}
macro_rules! p4d_populate {
    ($mm:expr, $p4d:expr, $pud:expr) => { set_p4d($p4d, __p4d(_PAGE_TABLE + __pa($pud))) };
}

macro_rules! set_p4d {
    ($p4dptr:expr, $p4dval:expr) => { *$p4dptr = $p4dval };
}

#[inline]
pub unsafe fn pgd_needsync(pgd: pgd_t) -> i32 {
    pgd_val(pgd) & _PAGE_NEEDSYNC
}

#[inline]
pub unsafe fn pgd_mkuptodate(mut pgd: pgd_t) {
    pgd_val(pgd) &= !_PAGE_NEEDSYNC;
}

macro_rules! set_pmd {
    ($pmdptr:expr, $pmdval:expr) => { *$pmdptr = $pmdval };
}

#[inline]
pub unsafe fn pud_clear(pud: *mut pud_t) {
    set_pud!(pud, __pud(_PAGE_NEEDSYNC));
}

#[inline]
pub unsafe fn p4d_clear(p4d: *mut p4d_t) {
    set_p4d!(p4d, __p4d(_PAGE_NEEDSYNC));
}

macro_rules! pud_page {
    ($pud:expr) => { phys_to_page(pud_val($pud) & PAGE_MASK) };
}
macro_rules! pud_pgtable {
    ($pud:expr) => { __va(pud_val($pud) & PAGE_MASK) as *mut pmd_t };
}

macro_rules! p4d_page {
    ($p4d:expr) => { phys_to_page(p4d_val($p4d) & PAGE_MASK) };
}
macro_rules! p4d_pgtable {
    ($p4d:expr) => { __va(p4d_val($p4d) & PAGE_MASK) as *mut pud_t };
}

#[inline]
pub unsafe fn pte_pfn(pte: pte_t) -> c_ulong {
    phys_to_pfn(pte_val(pte))
}

#[inline]
pub unsafe fn pfn_pmd(page_nr: c_ulong, pgprot: pgprot_t) -> pmd_t {
    __pmd((page_nr << PAGE_SHIFT) | pgprot_val(pgprot))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
