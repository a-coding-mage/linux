/* SPDX-License-Identifier: GPL-2.0 */

//! Binary search matching the original C header, without typed element reads.

use core::ffi::c_void;
#[allow(unused_imports)]
pub use bindings::{bsearch, BsearchCmp};

/// Search with the exact pivot sequence and byte arithmetic of the C inline.
///
/// # Safety
/// The caller must satisfy the original C pointer/callback contract whenever
/// num is nonzero. No argument is accessed when num is zero. The comparator
/// determines element and key layout; this code never creates references.
#[inline(always)]
pub unsafe fn __inline_bsearch(
    key: *const c_void,
    mut base: *const c_void,
    mut num: usize,
    size: usize,
    cmp: BsearchCmp,
) -> *mut c_void {
    while num > 0 {
        let pivot = base.cast::<u8>().wrapping_add((num >> 1).wrapping_mul(size));
        // SAFETY: nonzero num requires a callable comparator, exactly as in C.
        // Do not inspect the Option before the loop: NULL is valid for num=0.
        let compare = unsafe { cmp.into_option().unwrap_unchecked() };
        // SAFETY: validity of key and each visited pivot is the caller's contract.
        let result = unsafe { compare(key, pivot.cast()) };
        if result == 0 {
            return pivot.cast_mut().cast();
        }
        if result > 0 {
            base = pivot.wrapping_add(size).cast();
            num -= 1;
        }
        num >>= 1;
    }
    core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
