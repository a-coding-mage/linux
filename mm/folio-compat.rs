// SPDX-License-Identifier: GPL-2.0
/*
 * Compatibility functions which bloat the callers too much to make inline.
 * All of the callers of these functions should be converted to use folios
 * eventually.
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct folio {
    _private: [u8; 0],
}

#[repr(C)]
pub struct address_space {
    _private: [u8; 0],
}

#[repr(C)]
pub struct writeback_control {
    _private: [u8; 0],
}

pub type pgoff_t = usize;
pub type gfp_t = u32;
pub type fgf_t = u32;

unsafe extern "C" {
    fn page_folio(page: *mut page) -> *mut folio;
    fn folio_unlock(folio: *mut folio);
    fn folio_end_writeback(folio: *mut folio);
    fn folio_wait_writeback(folio: *mut folio);
    fn folio_mark_accessed(folio: *mut folio);
    fn folio_start_writeback(folio: *mut folio);
    fn folio_mark_dirty(folio: *mut folio) -> bool;
    fn folio_mark_dirty_lock(folio: *mut folio) -> i32;
    fn folio_clear_dirty_for_io(folio: *mut folio) -> bool;
    fn folio_redirty_for_writepage(
        wbc: *mut writeback_control,
        folio: *mut folio,
    ) -> bool;
    fn filemap_add_folio(
        mapping: *mut address_space,
        folio: *mut folio,
        index: pgoff_t,
        gfp: gfp_t,
    ) -> i32;
    fn __filemap_get_folio(
        mapping: *mut address_space,
        index: pgoff_t,
        fgp_flags: fgf_t,
        gfp: gfp_t,
    ) -> *mut folio;
    fn IS_ERR(ptr: *mut folio) -> bool;
    fn folio_file_page(folio: *mut folio, index: pgoff_t) -> *mut page;
}

pub unsafe fn unlock_page(page: *mut page) {
    folio_unlock(page_folio(page));
}

pub unsafe fn end_page_writeback(page: *mut page) {
    folio_end_writeback(page_folio(page));
}

pub unsafe fn wait_on_page_writeback(page: *mut page) {
    folio_wait_writeback(page_folio(page));
}

pub unsafe fn mark_page_accessed(page: *mut page) {
    folio_mark_accessed(page_folio(page));
}

pub unsafe fn set_page_writeback(page: *mut page) {
    folio_start_writeback(page_folio(page));
}

/* Read the comment above folio_mark_dirty() regarding required locks! */
pub unsafe fn set_page_dirty(page: *mut page) -> bool {
    folio_mark_dirty(page_folio(page))
}

pub unsafe fn set_page_dirty_lock(page: *mut page) -> i32 {
    folio_mark_dirty_lock(page_folio(page))
}

pub unsafe fn clear_page_dirty_for_io(page: *mut page) -> bool {
    folio_clear_dirty_for_io(page_folio(page))
}

pub unsafe fn redirty_page_for_writepage(
    wbc: *mut writeback_control,
    page: *mut page,
) -> bool {
    folio_redirty_for_writepage(wbc, page_folio(page))
}

pub unsafe fn add_to_page_cache_lru(
    page: *mut page,
    mapping: *mut address_space,
    index: pgoff_t,
    gfp: gfp_t,
) -> i32 {
    filemap_add_folio(mapping, page_folio(page), index, gfp)
}

#[inline(never)]
pub unsafe fn pagecache_get_page(
    mapping: *mut address_space,
    index: pgoff_t,
    fgp_flags: fgf_t,
    gfp: gfp_t,
) -> *mut page {
    let folio = __filemap_get_folio(mapping, index, fgp_flags, gfp);
    if IS_ERR(folio) {
        return core::ptr::null_mut();
    }
    folio_file_page(folio, index)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
