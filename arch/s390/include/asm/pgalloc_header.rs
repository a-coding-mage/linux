/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  S390 version
 *    Copyright IBM Corp. 1999, 2000
 *    Author(s): Hartmut Penner (hp@de.ibm.com)
 *               Martin Schwidefsky (schwidefsky@de.ibm.com)
 *
 *  Derived from "include/asm-i386/pgalloc.h"
 *    Copyright (C) 1994  Linus Torvalds
 */

pub const CRST_ALLOC_ORDER: usize = 2;

unsafe extern "C" {
    pub fn crst_table_alloc_noprof(mm: *mut mm_struct) -> *mut c_ulong;
    pub fn crst_table_free(mm: *mut mm_struct, table: *mut c_ulong);
    pub fn page_table_alloc_noprof(mm: *mut mm_struct) -> *mut c_ulong;
    pub fn page_table_free(mm: *mut mm_struct, table: *mut c_ulong);
    pub fn crst_table_upgrade(mm: *mut mm_struct, limit: c_ulong) -> c_int;
    pub fn pte_free_defer(mm: *mut mm_struct, pgtable: pgtable_t);
    pub fn vmem_map_init();
    pub fn vmem_crst_alloc(val: c_ulong) -> *mut c_void;
    pub fn vmem_pte_alloc() -> *mut pte_t;
    pub fn base_asce_alloc(addr: c_ulong, num_pages: c_ulong) -> c_ulong;
    pub fn base_asce_free(asce: c_ulong);
}

macro_rules! crst_table_alloc { ($($arg:expr),* $(,)?) => { alloc_hooks!(unsafe { crst_table_alloc_noprof($($arg),*) }) }; }
macro_rules! page_table_alloc { ($($arg:expr),* $(,)?) => { alloc_hooks!(unsafe { page_table_alloc_noprof($($arg),*) }) }; }

#[inline]
pub unsafe fn crst_table_init(crst: *mut c_ulong, entry: c_ulong) {
    memset64(crst as *mut u64, entry, _CRST_ENTRIES);
}

#[inline]
pub unsafe fn check_asce_limit(mm: *mut mm_struct, addr: c_ulong, len: c_ulong) -> c_ulong {
    let mut rc: c_int;
    if addr.wrapping_add(len) > (*mm).context.asce_limit
        && addr.wrapping_add(len) <= TASK_SIZE
    {
        rc = crst_table_upgrade(mm, addr.wrapping_add(len));
        if rc != 0 { return rc as c_ulong; }
    }
    addr
}

#[inline]
pub unsafe fn p4d_alloc_one_noprof(mm: *mut mm_struct, _address: c_ulong) -> *mut p4d_t {
    let table = crst_table_alloc_noprof(mm);
    if table.is_null() { return core::ptr::null_mut(); }
    crst_table_init(table, _REGION2_ENTRY_EMPTY);
    pagetable_p4d_ctor(virt_to_ptdesc(table));
    table as *mut p4d_t
}
macro_rules! p4d_alloc_one { ($($arg:expr),* $(,)?) => { alloc_hooks!(unsafe { p4d_alloc_one_noprof($($arg),*) }) }; }

#[inline]
pub unsafe fn p4d_free(mm: *mut mm_struct, p4d: *mut p4d_t) {
    if mm_p4d_folded(mm) { return; }
    pagetable_dtor(virt_to_ptdesc(p4d));
    crst_table_free(mm, p4d as *mut c_ulong);
}

#[inline]
pub unsafe fn pud_alloc_one_noprof(mm: *mut mm_struct, _address: c_ulong) -> *mut pud_t {
    let table = crst_table_alloc_noprof(mm);
    if table.is_null() { return core::ptr::null_mut(); }
    crst_table_init(table, _REGION3_ENTRY_EMPTY);
    pagetable_pud_ctor(virt_to_ptdesc(table));
    table as *mut pud_t
}
macro_rules! pud_alloc_one { ($($arg:expr),* $(,)?) => { alloc_hooks!(unsafe { pud_alloc_one_noprof($($arg),*) }) }; }

#[inline]
pub unsafe fn pud_free(mm: *mut mm_struct, pud: *mut pud_t) {
    if mm_pud_folded(mm) { return; }
    pagetable_dtor(virt_to_ptdesc(pud));
    crst_table_free(mm, pud as *mut c_ulong);
}

#[inline]
pub unsafe fn pmd_alloc_one_noprof(mm: *mut mm_struct, _vmaddr: c_ulong) -> *mut pmd_t {
    let table = crst_table_alloc_noprof(mm);
    if table.is_null() { return core::ptr::null_mut(); }
    crst_table_init(table, _SEGMENT_ENTRY_EMPTY);
    if !pagetable_pmd_ctor(mm, virt_to_ptdesc(table)) {
        crst_table_free(mm, table);
        return core::ptr::null_mut();
    }
    table as *mut pmd_t
}
macro_rules! pmd_alloc_one { ($($arg:expr),* $(,)?) => { alloc_hooks!(unsafe { pmd_alloc_one_noprof($($arg),*) }) }; }

#[inline]
pub unsafe fn pmd_free(mm: *mut mm_struct, pmd: *mut pmd_t) {
    if mm_pmd_folded(mm) { return; }
    pagetable_dtor(virt_to_ptdesc(pmd));
    crst_table_free(mm, pmd as *mut c_ulong);
}

#[inline]
pub unsafe fn pgd_populate(_mm: *mut mm_struct, pgd: *mut pgd_t, p4d: *mut p4d_t) {
    set_pgd(pgd, __pgd(_REGION1_ENTRY | __pa(p4d)));
}
#[inline]
pub unsafe fn p4d_populate(_mm: *mut mm_struct, p4d: *mut p4d_t, pud: *mut pud_t) {
    set_p4d(p4d, __p4d(_REGION2_ENTRY | __pa(pud)));
}
#[inline]
pub unsafe fn pud_populate(_mm: *mut mm_struct, pud: *mut pud_t, pmd: *mut pmd_t) {
    set_pud(pud, __pud(_REGION3_ENTRY | __pa(pmd)));
}

#[inline]
pub unsafe fn pgd_alloc_noprof(mm: *mut mm_struct) -> *mut pgd_t {
    let table = crst_table_alloc_noprof(mm);
    if table.is_null() { return core::ptr::null_mut(); }
    pagetable_pgd_ctor(virt_to_ptdesc(table));
    table as *mut pgd_t
}
macro_rules! pgd_alloc { ($($arg:expr),* $(,)?) => { alloc_hooks!(unsafe { pgd_alloc_noprof($($arg),*) }) }; }

#[inline]
pub unsafe fn pgd_free(mm: *mut mm_struct, pgd: *mut pgd_t) {
    pagetable_dtor(virt_to_ptdesc(pgd));
    crst_table_free(mm, pgd as *mut c_ulong);
}

#[inline]
pub unsafe fn pmd_populate(_mm: *mut mm_struct, pmd: *mut pmd_t, pte: pgtable_t) {
    set_pmd(pmd, __pmd(_SEGMENT_ENTRY | __pa(pte)));
}
macro_rules! pmd_populate_kernel { ($mm:expr, $pmd:expr, $pte:expr) => { pmd_populate($mm, $pmd, $pte) }; }

/* page table entry allocation/free routines. */
macro_rules! pte_alloc_one_kernel { ($mm:expr) => { page_table_alloc!($mm) as *mut pte_t }; }
macro_rules! pte_alloc_one { ($mm:expr) => { page_table_alloc!($mm) as *mut pte_t }; }
macro_rules! pte_free_kernel { ($mm:expr, $pte:expr) => { page_table_free($mm, $pte as *mut c_ulong) }; }
macro_rules! pte_free { ($mm:expr, $pte:expr) => { page_table_free($mm, $pte as *mut c_ulong) }; }

/* arch use pte_free_defer() implementation in arch/s390/mm/pgalloc.c */
pub use pte_free_defer as pte_free_defer;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
