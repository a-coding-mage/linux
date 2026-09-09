/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by asm/tlb.h and asm/tlbflush.h.

extern "C" {
    pub fn mmu_page_ctor(page: *mut core::ffi::c_void);
    pub fn mmu_page_dtor(page: *mut core::ffi::c_void);
}

#[repr(C)]
pub enum m68k_table_types {
    TABLE_PGD,
    TABLE_PMD,
    TABLE_PTE,
}

extern "C" {
    pub fn init_pointer_table(table: *mut core::ffi::c_void, type_: i32);
    pub fn get_pointer_table(mm: *mut mm_struct, type_: i32) -> *mut core::ffi::c_void;
    pub fn free_pointer_table(table: *mut core::ffi::c_void, type_: i32) -> i32;
}

/*
 * Allocate and free page tables. The xxx_kernel() versions are
 * used to allocate a kernel page table - this turns on ASN bits
 * if any.
 */

pub unsafe fn pte_alloc_one_kernel(mm: *mut mm_struct) -> *mut pte_t {
    get_pointer_table(mm, TABLE_PTE as i32) as *mut pte_t
}

pub unsafe fn pte_free_kernel(_mm: *mut mm_struct, pte: *mut pte_t) {
    free_pointer_table(pte as *mut core::ffi::c_void, TABLE_PTE as i32);
}

pub unsafe fn pte_alloc_one(mm: *mut mm_struct) -> pgtable_t {
    get_pointer_table(mm, TABLE_PTE as i32) as pgtable_t
}

pub unsafe fn pte_free(_mm: *mut mm_struct, pgtable: pgtable_t) {
    free_pointer_table(pgtable as *mut core::ffi::c_void, TABLE_PTE as i32);
}

pub unsafe fn __pte_free_tlb(
    _tlb: *mut mmu_gather,
    pgtable: pgtable_t,
    _address: c_ulong,
) {
    free_pointer_table(pgtable as *mut core::ffi::c_void, TABLE_PTE as i32);
}

pub unsafe fn pmd_alloc_one(_mm: *mut mm_struct, _address: c_ulong) -> *mut pmd_t {
    get_pointer_table(_mm, TABLE_PMD as i32) as *mut pmd_t
}

pub unsafe fn pmd_free(_mm: *mut mm_struct, pmd: *mut pmd_t) -> i32 {
    free_pointer_table(pmd as *mut core::ffi::c_void, TABLE_PMD as i32)
}

pub unsafe fn __pmd_free_tlb(
    _tlb: *mut mmu_gather,
    pmd: *mut pmd_t,
    _address: c_ulong,
) -> i32 {
    free_pointer_table(pmd as *mut core::ffi::c_void, TABLE_PMD as i32)
}

pub unsafe fn pgd_free(_mm: *mut mm_struct, pgd: *mut pgd_t) {
    free_pointer_table(pgd as *mut core::ffi::c_void, TABLE_PGD as i32);
}

pub unsafe fn pgd_alloc(mm: *mut mm_struct) -> *mut pgd_t {
    get_pointer_table(mm, TABLE_PGD as i32) as *mut pgd_t
}

pub unsafe fn pmd_populate_kernel(
    _mm: *mut mm_struct,
    pmd: *mut pmd_t,
    pte: *mut pte_t,
) {
    pmd_set(pmd, pte);
}

pub unsafe fn pmd_populate(_mm: *mut mm_struct, pmd: *mut pmd_t, page: pgtable_t) {
    pmd_set(pmd, page);
}

pub unsafe fn pud_populate(_mm: *mut mm_struct, pud: *mut pud_t, pmd: *mut pmd_t) {
    pud_set(pud, pmd);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
