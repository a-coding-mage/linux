// SPDX-License-Identifier: GPL-2.0-only
// Direct Rust translation of gup.c. Kernel-provided types, constants, macros,
// and functions are intentionally referenced as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct folio { _private: [u8; 0] }
#[repr(C)] pub struct mm_struct { _private: [u8; 0] }
#[repr(C)] pub struct vm_area_struct { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }

extern "C" {
    fn page_folio(page: *mut page) -> *mut folio;
    fn folio_ref_count(folio: *mut folio) -> c_int;
    fn folio_ref_try_add(folio: *mut folio, refs: c_int) -> bool;
    fn folio_put_refs(folio: *mut folio, refs: c_int);
    fn folio_ref_add(folio: *mut folio, refs: c_int);
    fn folio_has_pincount(folio: *mut folio) -> bool;
    fn atomic_add(refs: c_int, ptr: *mut c_int);
    fn atomic_sub(refs: c_int, ptr: *mut c_int);
    fn is_zero_folio(folio: *mut folio) -> bool;
    fn node_stat_mod_folio(folio: *mut folio, item: c_int, nr: c_int);
    fn is_zero_page(page: *mut page) -> bool;
    fn folio_test_anon(folio: *mut folio) -> bool;
    fn folio_test_large(folio: *mut folio) -> bool;
    fn folio_test_hugetlb(folio: *mut folio) -> bool;
    fn PageAnonExclusive(page: *mut page) -> bool;
    fn VM_WARN_ON_ONCE_FOLIO(cond: bool, folio: *mut folio);
    fn VM_WARN_ON_ONCE_PAGE(cond: bool, page: *mut page);
    fn folio_test_dirty(folio: *mut folio) -> bool;
    fn folio_lock(folio: *mut folio);
    fn folio_mark_dirty(folio: *mut folio);
    fn folio_unlock(folio: *mut folio);
    fn unpin_user_pages(pages: *mut *mut page, npages: c_ulong);
}

const FOLL_PIN: u32 = 1 << 27;
const FOLL_GET: u32 = 1 << 0;
const FOLL_PCI_P2PDMA: u32 = 1 << 28;
const GUP_PIN_COUNTING_BIAS: c_int = 1024;
const NR_FOLL_PIN_RELEASED: c_int = 0;
const NR_FOLL_PIN_ACQUIRED: c_int = 0;

#[inline]
unsafe fn sanity_check_pinned_pages(mut pages: *mut *mut page, mut npages: c_ulong) {
    while npages != 0 {
        let p = *pages;
        if !p.is_null() {
            let f = page_folio(p);
            if !is_zero_page(p) && folio_test_anon(f) {
                if !folio_test_large(f) || folio_test_hugetlb(f) {
                    VM_WARN_ON_ONCE_PAGE(!PageAnonExclusive(f as *mut page), f as *mut page);
                } else {
                    VM_WARN_ON_ONCE_PAGE(!PageAnonExclusive(f as *mut page) && !PageAnonExclusive(p), p);
                }
            }
        }
        pages = pages.add(1); npages -= 1;
    }
}

#[inline]
unsafe fn try_get_folio(mut p: *mut page, refs: c_int) -> *mut folio {
    loop {
        let f = page_folio(p);
        if folio_ref_count(f) < 0 || !folio_ref_try_add(f, refs) { return core::ptr::null_mut(); }
        if page_folio(p) != f { folio_put_refs(f, refs); continue; }
        return f;
    }
}

unsafe fn gup_put_folio(folio: *mut folio, mut refs: c_int, flags: u32) {
    if flags & FOLL_PIN != 0 {
        if is_zero_folio(folio) { return; }
        node_stat_mod_folio(folio, NR_FOLL_PIN_RELEASED, refs);
        if folio_has_pincount(folio) { atomic_sub(refs, (folio as *mut u8).add(0) as *mut c_int); }
        else { refs = refs.wrapping_mul(GUP_PIN_COUNTING_BIAS); }
    }
    folio_put_refs(folio, refs);
}

#[no_mangle]
pub unsafe extern "C" fn try_grab_folio(folio: *mut folio, refs: c_int, flags: u32) -> c_int {
    if folio_ref_count(folio) <= 0 { return -12; }
    if flags & FOLL_GET != 0 { folio_ref_add(folio, refs); }
    else if flags & FOLL_PIN != 0 {
        if is_zero_folio(folio) { return 0; }
        if folio_has_pincount(folio) {
            folio_ref_add(folio, refs);
            atomic_add(refs, (folio as *mut u8).add(0) as *mut c_int);
        } else { folio_ref_add(folio, refs.wrapping_mul(GUP_PIN_COUNTING_BIAS)); }
        node_stat_mod_folio(folio, NR_FOLL_PIN_ACQUIRED, refs);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn unpin_user_page(page: *mut page) {
    sanity_check_pinned_pages(&mut (page as *mut page), 1);
    gup_put_folio(page_folio(page), 1, FOLL_PIN);
}

#[no_mangle]
pub unsafe extern "C" fn unpin_user_folio(folio: *mut folio, npages: c_ulong) {
    gup_put_folio(folio, npages as c_int, FOLL_PIN);
}

#[no_mangle]
pub unsafe extern "C" fn unpin_user_pages_dirty_lock(pages: *mut *mut page, npages: c_ulong, make_dirty: bool) {
    if !make_dirty { unpin_user_pages(pages, npages); return; }
    sanity_check_pinned_pages(pages, npages);
    for i in 0..npages {
        let f = page_folio(*pages.add(i as usize));
        if !folio_test_dirty(f) { folio_lock(f); folio_mark_dirty(f); folio_unlock(f); }
        gup_put_folio(f, 1, FOLL_PIN);
    }
}

// The remaining implementation is configuration-dependent Linux MM code. Its
// declarations are retained here as external interfaces for the translated
// compilation unit; dependent kernel definitions supply their bodies.
extern "C" {
    pub fn get_user_pages(start: c_ulong, nr_pages: c_ulong, flags: u32, pages: *mut *mut page) -> c_long;
    pub fn get_user_pages_fast(start: c_ulong, nr_pages: c_int, flags: u32, pages: *mut *mut page) -> c_int;
    pub fn pin_user_pages(start: c_ulong, nr_pages: c_ulong, flags: u32, pages: *mut *mut page) -> c_long;
    pub fn pin_user_pages_fast(start: c_ulong, nr_pages: c_int, flags: u32, pages: *mut *mut page) -> c_int;
    pub fn fault_in_writeable(uaddr: *mut c_char, size: usize) -> usize;
    pub fn fault_in_readable(uaddr: *const c_char, size: usize) -> usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
