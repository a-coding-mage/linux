/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit:
// linux/kernel.h, linux/sched.h, linux/pgtable.h,
// asm/pgtsrmmu.h, asm/vaddrs.h, and asm/page.h.

#[allow(non_camel_case_types)]
pub type c_int = i32;

// struct page;
pub enum page {}

extern "C" {
    pub fn srmmu_get_nocache(size: c_int, align: c_int) -> *mut core::ffi::c_void;
    pub fn srmmu_free_nocache(addr: *mut core::ffi::c_void, size: c_int);

    pub static mut sparc_iomap: resource;

    pub fn get_pgd_fast() -> *mut pgd_t;
}

// The concrete definitions of these C-compatible types are supplied by the
// corresponding translated kernel headers.
#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pgd_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pud_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pmd_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pte_t {
    _private: [u8; 0],
}

pub type pgtable_t = *mut core::ffi::c_void;

#[repr(C)]
pub struct mm_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mmu_gather {
    pub mm: *mut mm_struct,
}

extern "C" {
    pub fn __nocache_pa(addr: *mut core::ffi::c_void) -> c_ulong;
    pub fn set_pte(ptep: *mut pte_t, pte: pte_t);
    pub fn __pte(value: c_ulong) -> pte_t;
    pub fn pmd_set(pmdp: *mut pmd_t, ptep: *mut pte_t);
    pub fn pte_alloc_one(mm: *mut mm_struct) -> pgtable_t;
    pub fn pte_free(mm: *mut mm_struct, pte: pgtable_t);
}

#[allow(non_camel_case_types)]
pub type c_ulong = usize;

pub unsafe fn free_pgd_fast(pgd: *mut pgd_t) {
    srmmu_free_nocache(pgd.cast(), SRMMU_PGD_TABLE_SIZE);
}

pub unsafe fn pgd_free(_mm: *mut mm_struct, pgd: *mut pgd_t) {
    free_pgd_fast(pgd);
}

pub unsafe fn pgd_alloc(_mm: *mut mm_struct) -> *mut pgd_t {
    get_pgd_fast()
}

pub unsafe fn pud_set(pudp: *mut pud_t, pmdp: *mut pmd_t) {
    let pa: c_ulong = __nocache_pa(pmdp.cast());
    set_pte(
        pudp.cast::<pte_t>(),
        __pte(SRMMU_ET_PTD | (pa >> 4)),
    );
}

pub unsafe fn pud_populate(_mm: *mut mm_struct, pgd: *mut pud_t, pmd: *mut pmd_t) {
    pud_set(pgd, pmd);
}

pub unsafe fn pmd_alloc_one(
    _mm: *mut mm_struct,
    _address: c_ulong,
) -> *mut pmd_t {
    srmmu_get_nocache(SRMMU_PMD_TABLE_SIZE, SRMMU_PMD_TABLE_SIZE).cast()
}

pub unsafe fn free_pmd_fast(pmd: *mut pmd_t) {
    srmmu_free_nocache(pmd.cast(), SRMMU_PMD_TABLE_SIZE);
}

pub unsafe fn pmd_free(_mm: *mut mm_struct, pmd: *mut pmd_t) {
    free_pmd_fast(pmd);
}

pub unsafe fn __pmd_free_tlb(
    tlb: *mut mmu_gather,
    pmd: *mut pmd_t,
    _addr: c_ulong,
) {
    pmd_free((*tlb).mm, pmd);
}

pub unsafe fn pmd_populate(
    _mm: *mut mm_struct,
    pmd: *mut pmd_t,
    pte: *mut pte_t,
) {
    pmd_set(pmd, pte);
}

pub unsafe fn pmd_populate_kernel(
    mm: *mut mm_struct,
    pmd: *mut pmd_t,
    pte: *mut pte_t,
) {
    pmd_populate(mm, pmd, pte);
}

pub unsafe fn pte_alloc_one_kernel(_mm: *mut mm_struct) -> *mut pte_t {
    srmmu_get_nocache(SRMMU_PTE_TABLE_SIZE, SRMMU_PTE_TABLE_SIZE).cast()
}

pub unsafe fn free_pte_fast(pte: *mut pte_t) {
    srmmu_free_nocache(pte.cast(), SRMMU_PTE_TABLE_SIZE);
}

pub unsafe fn pte_free_kernel(_mm: *mut mm_struct, pte: *mut pte_t) {
    free_pte_fast(pte);
}

pub unsafe fn __pte_free_tlb(
    tlb: *mut mmu_gather,
    pte: pgtable_t,
    _addr: c_ulong,
) {
    pte_free((*tlb).mm, pte);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
