/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of the PowerPC 64 pgalloc header. */

/*
 * Dependencies supplied by the surrounding kernel translation are intentionally
 * referenced here rather than redefined.
 */

#[repr(C)]
pub struct vmemmap_backing {
    pub list: *mut vmemmap_backing,
    pub phys: ::core::ffi::c_ulong,
    pub virt_addr: ::core::ffi::c_ulong,
}

pub static mut vmemmap_list: *mut vmemmap_backing = ::core::ptr::null_mut();

pub unsafe fn p4d_populate(mm: *mut mm_struct, p4d: *mut p4d_t, pud: *mut pud_t) {
    let _ = mm;
    p4d_set(p4d, pud as ::core::ffi::c_ulong);
}

pub unsafe fn pud_alloc_one(
    mm: *mut mm_struct,
    addr: ::core::ffi::c_ulong,
) -> *mut pud_t {
    let _ = addr;
    kmem_cache_alloc(
        PGT_CACHE(PUD_INDEX_SIZE),
        pgtable_gfp_flags(mm, GFP_KERNEL),
    ) as *mut pud_t
}

pub unsafe fn pud_free(mm: *mut mm_struct, pud: *mut pud_t) {
    let _ = mm;
    kmem_cache_free(PGT_CACHE(PUD_INDEX_SIZE), pud);
}

pub unsafe fn pud_populate(mm: *mut mm_struct, pud: *mut pud_t, pmd: *mut pmd_t) {
    let _ = mm;
    pud_set(pud, pmd as ::core::ffi::c_ulong);
}

pub unsafe fn pmd_populate_kernel(
    mm: *mut mm_struct,
    pmd: *mut pmd_t,
    pte: *mut pte_t,
) {
    let _ = mm;
    pmd_set(pmd, pte as ::core::ffi::c_ulong);
}

pub unsafe fn pmd_populate(
    mm: *mut mm_struct,
    pmd: *mut pmd_t,
    pte_page: pgtable_t,
) {
    let _ = mm;
    pmd_set(pmd, pte_page as ::core::ffi::c_ulong);
}

pub unsafe fn pmd_alloc_one(
    mm: *mut mm_struct,
    addr: ::core::ffi::c_ulong,
) -> *mut pmd_t {
    let _ = addr;
    kmem_cache_alloc(
        PGT_CACHE(PMD_CACHE_INDEX),
        pgtable_gfp_flags(mm, GFP_KERNEL),
    ) as *mut pmd_t
}

pub unsafe fn pmd_free(mm: *mut mm_struct, pmd: *mut pmd_t) {
    let _ = mm;
    kmem_cache_free(PGT_CACHE(PMD_CACHE_INDEX), pmd);
}

#[macro_export]
macro_rules! __pmd_free_tlb {
    ($tlb:expr, $pmd:expr, $addr:expr) => {{
        let _ = $addr;
        pgtable_free_tlb($tlb, $pmd, PMD_CACHE_INDEX)
    }};
}

#[macro_export]
macro_rules! __pud_free_tlb {
    ($tlb:expr, $pud:expr, $addr:expr) => {{
        let _ = $addr;
        pgtable_free_tlb($tlb, $pud, PUD_INDEX_SIZE)
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
