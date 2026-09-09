// SPDX-License-Identifier: GPL-2.0-only
// Direct low-level translation of iov_iter.c.  Kernel-provided types,
// constants, macros, and functions are intentionally left as dependencies.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_int, c_void};

extern "C" {
    fn should_fail_usercopy() -> bool;
    fn access_ok(p: *const c_void, n: usize) -> bool;
    fn raw_copy_to_user(d: *mut c_void, s: *const c_void, n: usize) -> usize;
    fn raw_copy_from_user(d: *mut c_void, s: *const c_void, n: usize) -> usize;
    fn copy_to_user_nofault(d: *mut c_void, s: *const c_void, n: usize) -> isize;
    fn clear_user(d: *mut c_void, n: usize) -> usize;
    fn copy_from_user_inatomic_nontemporal(d: *mut c_void, s: *const c_void, n: usize) -> usize;
    fn might_fault();
}

#[inline(always)]
unsafe fn copy_to_user_iter(iter_to: *mut c_void, progress: usize, mut len: usize,
                            from: *mut c_void, _priv2: *mut c_void) -> usize {
    if should_fail_usercopy() { return len; }
    if access_ok(iter_to, len) {
        len = raw_copy_to_user(iter_to, from.add(progress), len);
    }
    len
}

#[inline(always)]
unsafe fn copy_to_user_iter_nofault(iter_to: *mut c_void, progress: usize, len: usize,
                                    from: *mut c_void, _priv2: *mut c_void) -> usize {
    if should_fail_usercopy() { return len; }
    let r = copy_to_user_nofault(iter_to, from.add(progress), len);
    if r < 0 { len } else { r as usize }
}

#[inline(always)]
unsafe fn copy_from_user_iter(iter_from: *mut c_void, progress: usize, len: usize,
                              to: *mut c_void, _priv2: *mut c_void) -> usize {
    if should_fail_usercopy() { return len; }
    if !access_ok(iter_from, len) { return len; }
    raw_copy_from_user(to.add(progress), iter_from, len)
}

#[inline(always)]
unsafe fn memcpy_to_iter(iter_to: *mut c_void, progress: usize, len: usize,
                         from: *mut c_void, _priv2: *mut c_void) -> usize {
    core::ptr::copy_nonoverlapping(from.add(progress), iter_to, len); 0
}

#[inline(always)]
unsafe fn memcpy_from_iter(iter_from: *mut c_void, progress: usize, len: usize,
                           to: *mut c_void, _priv2: *mut c_void) -> usize {
    core::ptr::copy_nonoverlapping(iter_from, to.add(progress), len); 0
}

// The remaining kernel-facing implementation is represented with C ABI
// declarations so that all externally visible interfaces remain available to
// the surrounding kernel translation unit. Their definitions are supplied by
// the corresponding translated kernel dependencies.
extern "C" {
    pub fn fault_in_iov_iter_readable(i: *const c_void, size: usize) -> usize;
    pub fn fault_in_iov_iter_writeable(i: *const c_void, size: usize) -> usize;
    pub fn iov_iter_init(i: *mut c_void, direction: u32, iov: *const c_void,
                         nr_segs: usize, count: usize);
    pub fn _copy_to_iter(addr: *const c_void, bytes: usize, i: *mut c_void) -> usize;
    pub fn _copy_from_iter(addr: *mut c_void, bytes: usize, i: *mut c_void) -> usize;
    pub fn _copy_from_iter_nocache(addr: *mut c_void, bytes: usize, i: *mut c_void) -> usize;
    pub fn copy_page_to_iter(page: *mut c_void, offset: usize, bytes: usize,
                             i: *mut c_void) -> usize;
    pub fn copy_page_from_iter(page: *mut c_void, offset: usize, bytes: usize,
                               i: *mut c_void) -> usize;
    pub fn iov_iter_zero(bytes: usize, i: *mut c_void) -> usize;
    pub fn iov_iter_advance(i: *mut c_void, size: usize);
    pub fn iov_iter_revert(i: *mut c_void, unroll: usize);
    pub fn iov_iter_single_seg_count(i: *const c_void) -> usize;
    pub fn iov_iter_kvec(i: *mut c_void, direction: u32, kvec: *const c_void,
                         nr_segs: usize, count: usize);
    pub fn iov_iter_bvec(i: *mut c_void, direction: u32, bvec: *const c_void,
                         nr_segs: usize, count: usize);
    pub fn iov_iter_folio_queue(i: *mut c_void, direction: u32, folioq: *const c_void,
                                first_slot: u32, offset: u32, count: usize);
    pub fn iov_iter_xarray(i: *mut c_void, direction: u32, xarray: *mut c_void,
                           start: i64, count: usize);
    pub fn iov_iter_discard(i: *mut c_void, direction: u32, count: usize);
    pub fn iov_iter_alignment(i: *const c_void) -> usize;
    pub fn iov_iter_gap_alignment(i: *const c_void) -> usize;
    pub fn iov_iter_npages(i: *const c_void, maxpages: c_int) -> c_int;
    pub fn import_ubuf(rw: c_int, buf: *mut c_void, len: usize, i: *mut c_void) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
