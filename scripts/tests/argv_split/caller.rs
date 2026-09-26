// SPDX-License-Identifier: GPL-2.0
//! Protected calls through the actual generated argument-vector declarations.

use kernel::bindings::gfp_t;
use kernel::ffi::{c_char, c_int};

/// Call the selected provider through its public generated declaration.
///
/// # Safety
///
/// The source, output pointer and GFP mask must satisfy `argv_split`'s contract.
#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn argv_split_rust_call(
    flags: gfp_t,
    source: *const c_char,
    count: *mut c_int,
) -> *mut *mut c_char {
    let actual: unsafe extern "C" fn(gfp_t, *const c_char, *mut c_int) -> *mut *mut c_char =
        kernel::bindings::argv_split;
    // SAFETY: the caller supplies the original public interface's contract.
    unsafe { core::ptr::read_volatile(&actual)(flags, source, count) }
}

/// Release a selected provider's vector through its public declaration.
///
/// # Safety
///
/// The vector must retain its original hidden owner and be freed exactly once.
#[inline(never)]
#[no_mangle]
pub unsafe extern "C" fn argv_free_rust_call(vector: *mut *mut c_char) {
    let actual: unsafe extern "C" fn(*mut *mut c_char) = kernel::bindings::argv_free;
    // SAFETY: the caller retains the vector's two original allocations.
    unsafe { core::ptr::read_volatile(&actual)(vector) }
}
