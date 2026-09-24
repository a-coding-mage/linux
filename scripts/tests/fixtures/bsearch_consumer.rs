// SPDX-License-Identifier: GPL-2.0-only
//! Independent binding consumer and deliberately wrong CFI controls.
#![no_std]
use bindings::{bsearch, BsearchCmp};
use core::ffi::c_void;

/// Call via a volatile-loaded public native declaration.
#[no_mangle]
pub unsafe extern "C" fn rust_consumer(k: *const c_void, b: *const c_void, n: usize, s: usize, c: BsearchCmp) -> *mut c_void {
    let function = bsearch as unsafe extern "C" fn(_, _, _, _, _) -> _;
    // SAFETY: the caller supplies the unchanged C search contract.
    unsafe { core::ptr::read_volatile(&function)(k, b, n, s, c) }
}

/// Deliberately wrong first-argument adapter context; ordinary ABI is sound.
#[no_mangle]
pub unsafe extern "C" fn context_target(c: BsearchCmp, k: *const c_void, b: *const c_void, n: usize, s: usize) -> *mut c_void {
    // SAFETY: the caller supplies a valid search.
    unsafe { bsearch(k, b, n, s, c) }
}
/// Wrong-context protected call, expected to trap only with KCFI enabled.
#[no_mangle]
pub unsafe extern "C" fn wrong_context(k: *const c_void, b: *const c_void, n: usize, s: usize, c: BsearchCmp) -> *mut c_void {
    type Wrong = unsafe extern "C" fn(bindings::cmp_func_t, *const c_void, *const c_void, usize, usize) -> *mut c_void;
    // SAFETY: ABI layouts are identical; this is an intentional KCFI negative.
    let target: Wrong = unsafe { core::mem::transmute(context_target as unsafe extern "C" fn(_, _, _, _, _) -> _) };
    unsafe { core::ptr::read_volatile(&target)(c.into_option(), k, b, n, s) }
}
/// Raw Option outer signature is an intentional protected-call negative.
#[no_mangle]
pub unsafe extern "C" fn wrong_signature(k: *const c_void, b: *const c_void, n: usize, s: usize, c: BsearchCmp) -> *mut c_void {
    type Wrong = unsafe extern "C" fn(*const c_void, *const c_void, usize, usize, bindings::cmp_func_t) -> *mut c_void;
    // SAFETY: ordinary ABI is identical; KCFI must reject the nominal mismatch.
    let target: Wrong = unsafe { core::mem::transmute(bsearch as unsafe extern "C" fn(_, _, _, _, _) -> _) };
    unsafe { core::ptr::read_volatile(&target)(k, b, n, s, c.into_option()) }
}
