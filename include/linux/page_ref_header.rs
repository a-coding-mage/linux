/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.

extern "C" {
    fn page_ref_set();
    fn page_ref_mod();
    fn page_ref_mod_and_test();
    fn page_ref_mod_and_return();
    fn page_ref_mod_unless();
    fn page_ref_freeze();
    fn page_ref_unfreeze();
}

// Under CONFIG_DEBUG_PAGE_REF, these are external tracing hooks.  Otherwise
// the local no-op definitions below are used.
#[cfg(CONFIG_DEBUG_PAGE_REF)]
extern "C" {
    fn __page_ref_set(page: *mut page, v: i32);
    fn __page_ref_mod(page: *mut page, v: i32);
    fn __page_ref_mod_and_test(page: *mut page, v: i32, ret: i32);
    fn __page_ref_mod_and_return(page: *mut page, v: i32, ret: i32);
    fn __page_ref_mod_unless(page: *mut page, v: i32, u: i32);
    fn __page_ref_freeze(page: *mut page, v: i32, ret: i32);
    fn __page_ref_unfreeze(page: *mut page, v: i32);
}

#[cfg(not(CONFIG_DEBUG_PAGE_REF))]
unsafe fn __page_ref_set(_page: *mut page, _v: i32) {}
#[cfg(not(CONFIG_DEBUG_PAGE_REF))]
unsafe fn __page_ref_mod(_page: *mut page, _v: i32) {}
#[cfg(not(CONFIG_DEBUG_PAGE_REF))]
unsafe fn __page_ref_mod_and_test(_page: *mut page, _v: i32, _ret: i32) {}
#[cfg(not(CONFIG_DEBUG_PAGE_REF))]
unsafe fn __page_ref_mod_and_return(_page: *mut page, _v: i32, _ret: i32) {}
#[cfg(not(CONFIG_DEBUG_PAGE_REF))]
unsafe fn __page_ref_mod_unless(_page: *mut page, _v: i32, _u: i32) {}
#[cfg(not(CONFIG_DEBUG_PAGE_REF))]
unsafe fn __page_ref_freeze(_page: *mut page, _v: i32, _ret: i32) {}
#[cfg(not(CONFIG_DEBUG_PAGE_REF))]
unsafe fn __page_ref_unfreeze(_page: *mut page, _v: i32) {}

#[cfg(CONFIG_DEBUG_PAGE_REF)]
unsafe fn page_ref_tracepoint_active(_t: unsafe extern "C" fn()) -> bool { true }
#[cfg(not(CONFIG_DEBUG_PAGE_REF))]
unsafe fn page_ref_tracepoint_active(_t: unsafe extern "C" fn()) -> bool { false }

#[inline]
pub unsafe fn page_ref_count(page: *const page) -> i32 {
    atomic_read(&(*page)._refcount)
}

/**
 * folio_ref_count - The reference count on this folio.
 */
#[inline]
pub unsafe fn folio_ref_count(folio: *const folio) -> i32 {
    page_ref_count(&(*folio).page)
}

#[inline]
pub unsafe fn page_count(page: *const page) -> i32 {
    folio_ref_count(page_folio(page))
}

#[inline]
pub unsafe fn set_page_count(page: *mut page, v: i32) {
    atomic_set(&mut (*page)._refcount, v);
    if page_ref_tracepoint_active(page_ref_set) { __page_ref_set(page, v); }
}

#[inline]
pub unsafe fn folio_set_count(folio: *mut folio, v: i32) { set_page_count(&mut (*folio).page, v); }

#[inline]
pub unsafe fn init_page_count(page: *mut page) { set_page_count(page, 1); }

#[inline]
pub unsafe fn page_ref_add(page: *mut page, nr: i32) {
    atomic_add(nr, &mut (*page)._refcount);
    if page_ref_tracepoint_active(page_ref_mod) { __page_ref_mod(page, nr); }
}

#[inline]
pub unsafe fn folio_ref_add(folio: *mut folio, nr: i32) { page_ref_add(&mut (*folio).page, nr); }

#[inline]
pub unsafe fn page_ref_sub(page: *mut page, nr: i32) {
    atomic_sub(nr, &mut (*page)._refcount);
    if page_ref_tracepoint_active(page_ref_mod) { __page_ref_mod(page, -nr); }
}

#[inline]
pub unsafe fn folio_ref_sub(folio: *mut folio, nr: i32) { page_ref_sub(&mut (*folio).page, nr); }

#[inline]
pub unsafe fn folio_ref_sub_return(folio: *mut folio, nr: i32) -> i32 {
    let ret = atomic_sub_return(nr, &mut (*folio)._refcount);
    if page_ref_tracepoint_active(page_ref_mod_and_return) { __page_ref_mod_and_return(&mut (*folio).page, -nr, ret); }
    ret
}

#[inline]
pub unsafe fn page_ref_inc(page: *mut page) { atomic_inc(&mut (*page)._refcount); if page_ref_tracepoint_active(page_ref_mod) { __page_ref_mod(page, 1); } }
#[inline]
pub unsafe fn folio_ref_inc(folio: *mut folio) { page_ref_inc(&mut (*folio).page); }
#[inline]
pub unsafe fn page_ref_dec(page: *mut page) { atomic_dec(&mut (*page)._refcount); if page_ref_tracepoint_active(page_ref_mod) { __page_ref_mod(page, -1); } }
#[inline]
pub unsafe fn folio_ref_dec(folio: *mut folio) { page_ref_dec(&mut (*folio).page); }

#[inline]
pub unsafe fn page_ref_sub_and_test(page: *mut page, nr: i32) -> i32 {
    let ret = atomic_sub_and_test(nr, &mut (*page)._refcount);
    if page_ref_tracepoint_active(page_ref_mod_and_test) { __page_ref_mod_and_test(page, -nr, ret); }
    ret
}
#[inline]
pub unsafe fn folio_ref_sub_and_test(folio: *mut folio, nr: i32) -> i32 { page_ref_sub_and_test(&mut (*folio).page, nr) }

#[inline]
pub unsafe fn page_ref_inc_return(page: *mut page) -> i32 {
    let ret = atomic_inc_return(&mut (*page)._refcount);
    if page_ref_tracepoint_active(page_ref_mod_and_return) { __page_ref_mod_and_return(page, 1, ret); }
    ret
}
#[inline]
pub unsafe fn folio_ref_inc_return(folio: *mut folio) -> i32 { page_ref_inc_return(&mut (*folio).page) }

#[inline]
pub unsafe fn page_ref_dec_and_test(page: *mut page) -> i32 {
    let ret = atomic_dec_and_test(&mut (*page)._refcount);
    if page_ref_tracepoint_active(page_ref_mod_and_test) { __page_ref_mod_and_test(page, -1, ret); }
    ret
}
#[inline]
pub unsafe fn folio_ref_dec_and_test(folio: *mut folio) -> i32 { page_ref_dec_and_test(&mut (*folio).page) }

#[inline]
pub unsafe fn page_ref_dec_return(page: *mut page) -> i32 {
    let ret = atomic_dec_return(&mut (*page)._refcount);
    if page_ref_tracepoint_active(page_ref_mod_and_return) { __page_ref_mod_and_return(page, -1, ret); }
    ret
}
#[inline]
pub unsafe fn folio_ref_dec_return(folio: *mut folio) -> i32 { page_ref_dec_return(&mut (*folio).page) }

#[inline]
pub unsafe fn page_ref_add_unless_zero(page: *mut page, nr: i32) -> bool {
    let ret = atomic_add_unless(&mut (*page)._refcount, nr, 0);
    if page_ref_tracepoint_active(page_ref_mod_unless) { __page_ref_mod_unless(page, nr, ret as i32); }
    ret
}
#[inline]
pub unsafe fn folio_ref_add_unless_zero(folio: *mut folio, nr: i32) -> bool { page_ref_add_unless_zero(&mut (*folio).page, nr) }
#[inline]
pub unsafe fn folio_try_get(folio: *mut folio) -> bool { folio_ref_add_unless_zero(folio, 1) }
#[inline]
pub unsafe fn folio_ref_try_add(folio: *mut folio, count: i32) -> bool { folio_ref_add_unless_zero(folio, count) }

#[inline]
pub unsafe fn page_ref_freeze(page: *mut page, count: i32) -> i32 {
    let ret = likely(atomic_cmpxchg(&mut (*page)._refcount, count, 0) == count) as i32;
    if page_ref_tracepoint_active(page_ref_freeze) { __page_ref_freeze(page, count, ret); }
    ret
}
#[inline]
pub unsafe fn folio_ref_freeze(folio: *mut folio, count: i32) -> i32 { page_ref_freeze(&mut (*folio).page, count) }

#[inline]
pub unsafe fn page_ref_unfreeze(page: *mut page, count: i32) {
    VM_BUG_ON_PAGE(page_count(page) != 0, page);
    VM_BUG_ON(count == 0);
    atomic_set_release(&mut (*page)._refcount, count);
    if page_ref_tracepoint_active(page_ref_unfreeze) { __page_ref_unfreeze(page, count); }
}
#[inline]
pub unsafe fn folio_ref_unfreeze(folio: *mut folio, count: i32) { page_ref_unfreeze(&mut (*folio).page, count); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
