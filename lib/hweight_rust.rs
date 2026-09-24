// SPDX-License-Identifier: GPL-2.0
//! Native owner of the generic software Hamming-weight C ABI.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
/// Pure Rust implementations with the original unsigned arithmetic semantics.
#[path = "hweight.rs"]
pub mod implementation;

/// Preserves the original algorithm for every unsigned-int input.
#[no_mangle]
pub extern "C" fn __sw_hweight8(w: core::ffi::c_uint) -> core::ffi::c_uint {
    implementation::__sw_hweight8(w)
}

/// Preserves the original algorithm for every unsigned-int input.
#[no_mangle]
pub extern "C" fn __sw_hweight16(w: core::ffi::c_uint) -> core::ffi::c_uint {
    implementation::__sw_hweight16(w)
}

/// Counts the set bits in an unsigned-int input.
#[no_mangle]
pub extern "C" fn __sw_hweight32(w: core::ffi::c_uint) -> core::ffi::c_uint {
    implementation::__sw_hweight32(w)
}

/// Counts the set bits in a 64-bit input, returning a native unsigned long.
#[no_mangle]
pub extern "C" fn __sw_hweight64(w: u64) -> usize {
    implementation::__sw_hweight64(w)
}

ffi_export::export_symbol!(__sw_hweight8, __sw_hweight8, "", "");
ffi_export::export_symbol!(__sw_hweight16, __sw_hweight16, "", "");
ffi_export::export_symbol!(__sw_hweight32, __sw_hweight32, "", "");
ffi_export::export_symbol!(__sw_hweight64, __sw_hweight64, "", "");
