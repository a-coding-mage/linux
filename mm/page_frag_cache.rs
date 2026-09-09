// SPDX-License-Identifier: GPL-2.0-only
/* Page fragment allocator
 *
 * Page Fragment:
 *  An arbitrary-length arbitrary-offset area of memory which resides within a
 *  0 or higher order page.  Multiple fragments within that page are
 *  individually refcounted, in the page's reference counter.
 *
 * The page_frag functions provide a simple allocation framework for page
 * fragments.  This is used by the network stack and network device drivers to
 * provide a backing region of memory for use as either an sk_buff->head, or to
 * be used in the "frags" portion of skb_shared_info.
 */

// Kernel declarations and constants supplied by the surrounding translation.
use core::ffi::c_void;

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct page_frag_cache {
    pub encoded_page: usize,
    pub pagecnt_bias: u32,
    pub offset: u32,
}

type gfp_t = u32;

extern "C" {
    fn page_address(page: *mut page) -> *mut c_void;
    fn virt_to_page(addr: *mut c_void) -> *mut page;
    fn __alloc_pages(gfp_mask: gfp_t, order: u32, nid: i32, nodemask: *mut c_void,
                     alloc_flags: u32) -> *mut page;
    fn numa_mem_id() -> i32;
    fn page_is_pfmemalloc(page: *mut page) -> bool;
    fn page_ref_count(page: *mut page) -> u32;
    fn page_ref_sub_and_test(page: *mut page, count: u32) -> bool;
    fn free_frozen_pages(page: *mut page, order: u32);
    fn compound_order(page: *mut page) -> u32;
    fn page_ref_add(page: *mut page, count: u32);
    fn set_page_count(page: *mut page, count: u32);
    fn put_page_testzero(page: *mut page) -> bool;
    fn virt_to_head_page(addr: *mut c_void) -> *mut page;
}

const PAGE_FRAG_CACHE_MAX_ORDER: u32 = 0;
const PAGE_FRAG_CACHE_ORDER_MASK: usize = 0;
const PAGE_FRAG_CACHE_PFMEMALLOC_BIT: usize = 0;
const PAGE_FRAG_CACHE_MAX_SIZE: u32 = 0;
const PAGE_SIZE: u32 = 0;
const __GFP_DIRECT_RECLAIM: gfp_t = 0;
const __GFP_COMP: gfp_t = 0;
const __GFP_NOWARN: gfp_t = 0;
const __GFP_NORETRY: gfp_t = 0;
const __GFP_NOMEMALLOC: gfp_t = 0;
const ALLOC_DEFAULT: u32 = 0;

unsafe fn encoded_page_create(page: *mut page, order: u32, pfmemalloc: bool) -> usize {
    (page_address(page) as usize)
        | ((order as usize) & PAGE_FRAG_CACHE_ORDER_MASK)
        | ((pfmemalloc as usize) * PAGE_FRAG_CACHE_PFMEMALLOC_BIT)
}

unsafe fn encoded_page_decode_order(encoded_page: usize) -> usize {
    encoded_page & PAGE_FRAG_CACHE_ORDER_MASK
}

unsafe fn encoded_page_decode_virt(encoded_page: usize) -> *mut c_void {
    (encoded_page & (!((PAGE_SIZE as usize) - 1))) as *mut c_void
}

unsafe fn encoded_page_decode_page(encoded_page: usize) -> *mut page {
    virt_to_page(encoded_page as *mut c_void)
}

unsafe fn encoded_page_decode_pfmemalloc(encoded_page: usize) -> bool {
    (encoded_page & PAGE_FRAG_CACHE_PFMEMALLOC_BIT) != 0
}

unsafe fn __page_frag_cache_refill(nc: *mut page_frag_cache, gfp_mask: gfp_t) -> *mut page {
    let mut order = PAGE_FRAG_CACHE_MAX_ORDER;
    let mut page: *mut page = core::ptr::null_mut();
    let gfp = gfp_mask;

    // Original conditional: #if (PAGE_SIZE < PAGE_FRAG_CACHE_MAX_SIZE)
    if PAGE_SIZE < PAGE_FRAG_CACHE_MAX_SIZE {
        let restricted = (gfp_mask & !__GFP_DIRECT_RECLAIM)
            | __GFP_COMP | __GFP_NOWARN | __GFP_NORETRY | __GFP_NOMEMALLOC;
        page = __alloc_pages(restricted, PAGE_FRAG_CACHE_MAX_ORDER,
                             numa_mem_id(), core::ptr::null_mut(), ALLOC_DEFAULT);
    }

    if page.is_null() {
        page = __alloc_pages(gfp, 0, numa_mem_id(), core::ptr::null_mut(), ALLOC_DEFAULT);
        order = 0;
    }

    (*nc).encoded_page = if !page.is_null() {
        encoded_page_create(page, order, page_is_pfmemalloc(page))
    } else { 0 };
    page
}

pub unsafe fn page_frag_cache_drain(nc: *mut page_frag_cache) {
    if (*nc).encoded_page == 0 { return; }
    __page_frag_cache_drain(encoded_page_decode_page((*nc).encoded_page), (*nc).pagecnt_bias);
    (*nc).encoded_page = 0;
}

pub unsafe fn __page_frag_cache_drain(page: *mut page, count: u32) {
    if page_ref_count(page) == 0 { return; }
    if page_ref_sub_and_test(page, count) { free_frozen_pages(page, compound_order(page)); }
}

pub unsafe fn __page_frag_alloc_align(nc: *mut page_frag_cache, fragsz: u32,
                                      gfp_mask: gfp_t, align_mask: u32) -> *mut c_void {
    let mut encoded_page = (*nc).encoded_page;
    let (mut size, mut offset): (u32, u32);
    let mut page: *mut page;

    if encoded_page == 0 {
        page = __page_frag_cache_refill(nc, gfp_mask);
        if page.is_null() { return core::ptr::null_mut(); }
        encoded_page = (*nc).encoded_page;
        page_ref_add(page, PAGE_FRAG_CACHE_MAX_SIZE);
        (*nc).pagecnt_bias = PAGE_FRAG_CACHE_MAX_SIZE + 1;
        (*nc).offset = 0;
    }

    size = PAGE_SIZE << encoded_page_decode_order(encoded_page);
    offset = ((*nc).offset + !align_mask) & align_mask.wrapping_neg();
    if offset.wrapping_add(fragsz) > size {
        if fragsz > PAGE_SIZE { return core::ptr::null_mut(); }
        page = encoded_page_decode_page(encoded_page);
        if !page_ref_sub_and_test(page, (*nc).pagecnt_bias) {
            return __page_frag_alloc_align(nc, fragsz, gfp_mask, align_mask);
        }
        if encoded_page_decode_pfmemalloc(encoded_page) {
            free_frozen_pages(page, encoded_page_decode_order(encoded_page) as u32);
            return __page_frag_alloc_align(nc, fragsz, gfp_mask, align_mask);
        }
        set_page_count(page, PAGE_FRAG_CACHE_MAX_SIZE + 1);
        (*nc).pagecnt_bias = PAGE_FRAG_CACHE_MAX_SIZE + 1;
        offset = 0;
    }
    (*nc).pagecnt_bias -= 1;
    (*nc).offset = offset + fragsz;
    encoded_page_decode_virt(encoded_page).add(offset as usize)
}

pub unsafe fn page_frag_free(addr: *mut c_void) {
    let page = virt_to_head_page(addr);
    if put_page_testzero(page) { free_frozen_pages(page, compound_order(page)); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
