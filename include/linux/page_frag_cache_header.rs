/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// PAGE_SIZE, PAGE_FRAG_CACHE_MAX_SIZE, gfp_t, page, page_frag_cache,
// is_power_of_2, and WARN_ON_ONCE.

// #if (PAGE_SIZE < PAGE_FRAG_CACHE_MAX_SIZE)
// Use a full byte here to enable assembler optimization as the shift
// operation is usually expecting a byte.
// #else
// Compiler should be able to figure out we don't read things as any value
// ANDed with 0 is 0.
// #endif
pub const PAGE_FRAG_CACHE_ORDER_MASK: usize = if PAGE_SIZE < PAGE_FRAG_CACHE_MAX_SIZE {
    (1usize << 8) - 1
} else {
    0
};

pub const PAGE_FRAG_CACHE_PFMEMALLOC_BIT: usize = PAGE_FRAG_CACHE_ORDER_MASK + 1;

#[inline]
pub unsafe fn encoded_page_decode_pfmemalloc(encoded_page: usize) -> bool {
    (encoded_page & PAGE_FRAG_CACHE_PFMEMALLOC_BIT) != 0
}

#[inline]
pub unsafe fn page_frag_cache_init(nc: *mut page_frag_cache) {
    (*nc).encoded_page = 0;
}

#[inline]
pub unsafe fn page_frag_cache_is_pfmemalloc(nc: *mut page_frag_cache) -> bool {
    encoded_page_decode_pfmemalloc((*nc).encoded_page)
}

unsafe extern "C" {
    pub fn page_frag_cache_drain(nc: *mut page_frag_cache);
    pub fn __page_frag_cache_drain(page: *mut page, count: u32);
    pub fn __page_frag_alloc_align(
        nc: *mut page_frag_cache,
        fragsz: u32,
        gfp_mask: gfp_t,
        align_mask: u32,
    ) -> *mut core::ffi::c_void;
}

#[inline]
pub unsafe fn page_frag_alloc_align(
    nc: *mut page_frag_cache,
    fragsz: u32,
    gfp_mask: gfp_t,
    align: u32,
) -> *mut core::ffi::c_void {
    WARN_ON_ONCE(!is_power_of_2(align));
    __page_frag_alloc_align(nc, fragsz, gfp_mask, align.wrapping_neg())
}

#[inline]
pub unsafe fn page_frag_alloc(
    nc: *mut page_frag_cache,
    fragsz: u32,
    gfp_mask: gfp_t,
) -> *mut core::ffi::c_void {
    __page_frag_alloc_align(nc, fragsz, gfp_mask, !0u32)
}

unsafe extern "C" {
    pub fn page_frag_free(addr: *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
