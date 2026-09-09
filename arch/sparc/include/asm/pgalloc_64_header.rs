/* SPDX-License-Identifier: GPL-2.0 */
// Translated from sparc/include/asm/pgalloc_64.h.
// C header includes and build-time configuration are supplied by other units.

/* Page table allocation/freeing. */

extern "C" {
    pub static mut pgtable_cache: *mut kmem_cache;

    pub fn p4d_set(p4d: *mut p4d_t, pud: *mut pud_t);
    pub fn pud_set(pud: *mut pud_t, pmd: *mut pmd_t);
    pub fn pmd_set(mm: *mut mm_struct, pmd: *mut pmd_t, pte: *mut pte_t);
    pub fn kmem_cache_alloc(cache: *mut kmem_cache, flags: gfp_t) -> *mut core::ffi::c_void;
    pub fn kmem_cache_free(cache: *mut kmem_cache, obj: *mut core::ffi::c_void);

    pub fn pte_alloc_one_kernel(mm: *mut mm_struct) -> *mut pte_t;
    pub fn pte_alloc_one(mm: *mut mm_struct) -> pgtable_t;
    pub fn pte_free_kernel(mm: *mut mm_struct, pte: *mut pte_t);
    pub fn pte_free(mm: *mut mm_struct, ptepage: pgtable_t);
    pub fn pte_free_defer(mm: *mut mm_struct, pgtable: pgtable_t);

    pub fn pgtable_free(table: *mut core::ffi::c_void, is_page: bool);
}

pub unsafe fn __p4d_populate(p4d: *mut p4d_t, pud: *mut pud_t) {
    p4d_set(p4d, pud);
}

pub unsafe fn p4d_populate(_mm: *mut mm_struct, p4d: *mut p4d_t, pud: *mut pud_t) {
    __p4d_populate(p4d, pud);
}

pub unsafe fn pgd_alloc(_mm: *mut mm_struct) -> *mut pgd_t {
    kmem_cache_alloc(pgtable_cache, GFP_KERNEL as gfp_t) as *mut pgd_t
}

pub unsafe fn pgd_free(_mm: *mut mm_struct, pgd: *mut pgd_t) {
    kmem_cache_free(pgtable_cache, pgd as *mut core::ffi::c_void);
}

pub unsafe fn __pud_populate(pud: *mut pud_t, pmd: *mut pmd_t) {
    pud_set(pud, pmd);
}

pub unsafe fn pud_populate(_mm: *mut mm_struct, pud: *mut pud_t, pmd: *mut pmd_t) {
    __pud_populate(pud, pmd);
}

pub unsafe fn pud_alloc_one(_mm: *mut mm_struct, _addr: c_ulong) -> *mut pud_t {
    kmem_cache_alloc(pgtable_cache, GFP_KERNEL as gfp_t) as *mut pud_t
}

pub unsafe fn pud_free(_mm: *mut mm_struct, pud: *mut pud_t) {
    kmem_cache_free(pgtable_cache, pud as *mut core::ffi::c_void);
}

pub unsafe fn pmd_alloc_one(_mm: *mut mm_struct, _addr: c_ulong) -> *mut pmd_t {
    kmem_cache_alloc(pgtable_cache, GFP_KERNEL as gfp_t) as *mut pmd_t
}

pub unsafe fn pmd_free(_mm: *mut mm_struct, pmd: *mut pmd_t) {
    kmem_cache_free(pgtable_cache, pmd as *mut core::ffi::c_void);
}

pub unsafe fn pmd_populate_kernel(mm: *mut mm_struct, pmd: *mut pmd_t, pte: *mut pte_t) {
    pmd_set(mm, pmd, pte);
}

pub unsafe fn pmd_populate(mm: *mut mm_struct, pmd: *mut pmd_t, pte: *mut pte_t) {
    pmd_set(mm, pmd, pte);
}

// CONFIG_SMP variant; the non-SMP variant has the same public operation below.
#[repr(C)]
pub struct mmu_gather {
    _private: [u8; 0],
}

extern "C" {
    pub fn tlb_remove_table(tlb: *mut mmu_gather, table: *mut core::ffi::c_void);
}

pub unsafe fn pgtable_free_tlb(
    tlb: *mut mmu_gather,
    table: *mut core::ffi::c_void,
    is_page: bool,
) {
    let mut pgf = table as c_ulong;
    if is_page {
        pgf |= 0x1;
    }
    tlb_remove_table(tlb, pgf as *mut core::ffi::c_void);
}

pub unsafe fn __tlb_remove_table(table: *mut core::ffi::c_void) {
    let raw_table = table as c_ulong;
    let table = (raw_table & !0x1) as *mut core::ffi::c_void;
    let is_page = (raw_table & 0x1) != 0;
    pgtable_free(table, is_page);
}

pub unsafe fn __pte_free_tlb(
    tlb: *mut mmu_gather,
    pte: *mut pte_t,
    _address: c_ulong,
) {
    pgtable_free_tlb(tlb, pte as *mut core::ffi::c_void, true);
}

pub unsafe fn __pmd_free_tlb(
    tlb: *mut mmu_gather,
    pmd: *mut pmd_t,
    _addr: c_ulong,
) {
    pgtable_free_tlb(tlb, pmd as *mut core::ffi::c_void, false);
}

pub unsafe fn __pud_free_tlb(
    tlb: *mut mmu_gather,
    pud: *mut pud_t,
    _addr: c_ulong,
) {
    pgtable_free_tlb(tlb, pud as *mut core::ffi::c_void, false);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
