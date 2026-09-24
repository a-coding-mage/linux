// SPDX-License-Identifier: GPL-2.0
//! Native owner of memweight and its unrestricted C export.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "memweight.rs"]
mod implementation;

/// Counts set bits in a memory region using the original bitmap dependency.
///
/// # Safety
/// For nonzero `bytes`, `ptr` must identify `bytes` readable bytes, except that
/// inputs reaching the original whole-word BUG threshold after the leading
/// alignment scan need only those leading unaligned bytes to be readable.
/// Such inputs invoke BUG before reading any whole words or trailing bytes;
/// an aligned pointer needs no readable allocation in that case.
/// For zero `bytes`, no memory is accessed and `ptr` may be null.
#[no_mangle]
pub unsafe extern "C" fn memweight(ptr: *const core::ffi::c_void, bytes: usize) -> usize {
    // SAFETY: The caller supplies the readable region or threshold prefix
    // described above (zero bytes need neither); the core passes only
    // aligned whole words with a bit count strictly below the C BUG threshold.
    match unsafe {
        implementation::memweight_with(ptr.cast(), bytes, |bitmap, bits| {
            kernel::bindings::__bitmap_weight(bitmap.cast(), bits)
        })
    } {
        Some(result) => result,
        // SAFETY: This is precisely the original BUG_ON condition.
        None => unsafe { kernel::bindings::BUG() },
    }
}

ffi_export::export_symbol!(memweight, memweight, "", "");
