/* SPDX-License-Identifier: GPL-2.0 */
/* zpdesc.h: zsmalloc pool memory descriptor
 *
 * Written by Alex Shi <alexs@kernel.org>
 *          Hyeonggon Yoo <42.hyeyoo@gmail.com>
 */

// Dependencies supplied by the corresponding kernel headers.
use core::ffi::c_void;

#[repr(C)]
pub struct list_head;
#[repr(C)]
pub struct zspage;
#[repr(C)]
pub struct page;
#[repr(C)]
pub struct folio;
#[repr(C)]
pub struct zone;
#[repr(C)]
pub struct atomic_t;

/*
 * struct zpdesc - Memory descriptor for zsmalloc pool memory.
 * This struct overlays struct page for now. Do not modify without a good
 * understanding of the issues. In particular, do not expand into the overlap
 * with memcg_data.
 */
#[repr(C)]
pub struct zpdesc {
    pub flags: usize,
    pub lru: list_head,
    pub movable_ops: usize,
    pub next_or_handle: zpdesc_next_or_handle,
    pub zspage: *mut zspage,
    pub first_obj_offset: u32,
    pub _refcount: atomic_t,
}

#[repr(C)]
pub union zpdesc_next_or_handle {
    pub next: *mut zpdesc,
    pub handle: usize,
}

/* Page flags used: PG_private identifies the first component page;
 * PG_locked is used by page migration code. */

// The following assertions mirror the C offsetof/sizeof checks; their exact
// validity depends on the definitions supplied by the kernel headers.
// static_assert(offsetof(struct page, flags) == offsetof(struct zpdesc, flags));
// static_assert(offsetof(struct page, lru) == offsetof(struct zpdesc, lru));
// static_assert(offsetof(struct page, mapping) == offsetof(struct zpdesc, movable_ops));
// static_assert(offsetof(struct page, __folio_index) == offsetof(struct zpdesc, next));
// static_assert(offsetof(struct page, __folio_index) == offsetof(struct zpdesc, handle));
// static_assert(offsetof(struct page, private) == offsetof(struct zpdesc, zspage));
// static_assert(offsetof(struct page, page_type) == offsetof(struct zpdesc, first_obj_offset));
// static_assert(offsetof(struct page, _refcount) == offsetof(struct zpdesc, _refcount));
// static_assert(sizeof(struct zpdesc) <= sizeof(struct page));

#[inline]
pub unsafe fn zpdesc_page(zp: *mut zpdesc) -> *mut page {
    zp as *mut page
}

#[inline]
pub unsafe fn zpdesc_page_const(zp: *const zpdesc) -> *const page {
    zp as *const page
}

#[inline]
pub unsafe fn zpdesc_folio(zp: *mut zpdesc) -> *mut folio {
    zp as *mut folio
}

#[inline]
pub unsafe fn zpdesc_folio_const(zp: *const zpdesc) -> *const folio {
    zp as *const folio
}

#[inline]
pub unsafe fn page_zpdesc(p: *mut page) -> *mut zpdesc {
    p as *mut zpdesc
}

#[inline]
pub unsafe fn page_zpdesc_const(p: *const page) -> *const zpdesc {
    p as *const zpdesc
}

extern "C" {
    pub fn folio_lock(folio: *mut folio);
    pub fn folio_trylock(folio: *mut folio) -> bool;
    pub fn folio_unlock(folio: *mut folio);
    pub fn folio_wait_locked(folio: *mut folio);
    pub fn folio_get(folio: *mut folio);
    pub fn folio_put(folio: *mut folio);
    pub fn kmap_local_page(page: *mut page) -> *mut c_void;
    pub fn page_to_pfn(page: *mut page) -> usize;
    pub fn pfn_to_page(pfn: usize) -> *mut page;
    pub fn SetPageMovableOps(page: *mut page);
    pub fn __SetPageZsmalloc(page: *mut page);
    pub fn page_zone(page: *mut page) -> *mut zone;
    pub fn folio_test_locked(folio: *mut folio) -> bool;
}

#[inline]
pub unsafe fn zpdesc_lock(zpdesc: *mut zpdesc) {
    folio_lock(zpdesc_folio(zpdesc));
}

#[inline]
pub unsafe fn zpdesc_trylock(zpdesc: *mut zpdesc) -> bool {
    folio_trylock(zpdesc_folio(zpdesc))
}

#[inline]
pub unsafe fn zpdesc_unlock(zpdesc: *mut zpdesc) {
    folio_unlock(zpdesc_folio(zpdesc));
}

#[inline]
pub unsafe fn zpdesc_wait_locked(zpdesc: *mut zpdesc) {
    folio_wait_locked(zpdesc_folio(zpdesc));
}

#[inline]
pub unsafe fn zpdesc_get(zpdesc: *mut zpdesc) {
    folio_get(zpdesc_folio(zpdesc));
}

#[inline]
pub unsafe fn zpdesc_put(zpdesc: *mut zpdesc) {
    folio_put(zpdesc_folio(zpdesc));
}

#[inline]
pub unsafe fn kmap_local_zpdesc(zpdesc: *mut zpdesc) -> *mut c_void {
    kmap_local_page(zpdesc_page(zpdesc))
}

#[inline]
pub unsafe fn zpdesc_pfn(zpdesc: *mut zpdesc) -> usize {
    page_to_pfn(zpdesc_page(zpdesc))
}

#[inline]
pub unsafe fn pfn_zpdesc(pfn: usize) -> *mut zpdesc {
    page_zpdesc(pfn_to_page(pfn))
}

#[inline]
pub unsafe fn __zpdesc_set_movable(zpdesc: *mut zpdesc) {
    SetPageMovableOps(zpdesc_page(zpdesc));
}

#[inline]
pub unsafe fn __zpdesc_set_zsmalloc(zpdesc: *mut zpdesc) {
    __SetPageZsmalloc(zpdesc_page(zpdesc));
}

#[inline]
pub unsafe fn zpdesc_zone(zpdesc: *mut zpdesc) -> *mut zone {
    page_zone(zpdesc_page(zpdesc))
}

#[inline]
pub unsafe fn zpdesc_is_locked(zpdesc: *mut zpdesc) -> bool {
    folio_test_locked(zpdesc_folio(zpdesc))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
