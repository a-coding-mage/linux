/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the PowerPC page-table allocation header.
// The original MODULE conditional is preserved through Rust cfg attributes.

#[cfg(not(feature = "MODULE"))]
#[inline]
pub unsafe fn pgtable_gfp_flags(mm: *mut mm_struct, gfp: gfp_t) -> gfp_t {
    if mm == (&init_mm as *const mm_struct as *mut mm_struct) {
        gfp
    } else {
        gfp | __GFP_ACCOUNT
    }
}

#[cfg(feature = "MODULE")]
#[inline]
pub unsafe fn pgtable_gfp_flags(_mm: *mut mm_struct, gfp: gfp_t) -> gfp_t {
    gfp | __GFP_ACCOUNT
}

pub const PGALLOC_GFP: gfp_t = GFP_KERNEL | __GFP_ZERO;

extern "C" {
    pub fn pte_fragment_alloc(mm: *mut mm_struct, kernel: ::core::ffi::c_int) -> *mut pte_t;

    pub fn pte_frag_destroy(pte_frag: *mut ::core::ffi::c_void);
    pub fn pte_fragment_free(table: *mut ::core::ffi::c_ulong, kernel: ::core::ffi::c_int);

    // arch use pte_free_defer() implementation in arch/powerpc/mm/pgtable-frag.c
    pub fn pte_free_defer(mm: *mut mm_struct, pgtable: pgtable_t);

    pub static init_mm: mm_struct;
}

#[inline]
pub unsafe fn pte_alloc_one_kernel(mm: *mut mm_struct) -> *mut pte_t {
    pte_fragment_alloc(mm, 1)
}

#[inline]
pub unsafe fn pte_alloc_one(mm: *mut mm_struct) -> pgtable_t {
    pte_fragment_alloc(mm, 0) as pgtable_t
}

#[inline]
pub unsafe fn pte_free_kernel(_mm: *mut mm_struct, pte: *mut pte_t) {
    pte_fragment_free(pte as *mut ::core::ffi::c_ulong, 1);
}

#[inline]
pub unsafe fn pte_free(_mm: *mut mm_struct, ptepage: pgtable_t) {
    pte_fragment_free(ptepage as *mut ::core::ffi::c_ulong, 0);
}

pub const MAX_PGTABLE_INDEX_SIZE: u32 = 0xf;

extern "C" {
    pub static mut pgtable_cache: *mut *mut kmem_cache;
}

#[inline]
pub unsafe fn PGT_CACHE(shift: usize) -> *mut kmem_cache {
    *pgtable_cache.add(shift)
}

/*
 * Functions that deal with pagetables that could be at any level of
 * the table need to be passed an "index_size" so they know how to
 * handle allocation.  For PTE pages, the allocation size will be
 * (2^index_size * sizeof(pointer)) and allocations are drawn from
 * the kmem_cache in PGT_CACHE(index_size).
 *
 * The maximum index size needs to be big enough to allow any
 * pagetable sizes we need, but small enough to fit in the low bits of
 * any page table pointer.  In other words all pagetables, even tiny
 * ones, must be aligned to allow at least enough low 0 bits to
 * contain this value.  This value is also used as a mask, so it must
 * be one less than a power of two.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
