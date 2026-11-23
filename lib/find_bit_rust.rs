// SPDX-License-Identifier: GPL-2.0-or-later
//! Native kernel crate for the generic bit search functions.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `find_bit.c` translation unit.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "find_bit.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(_find_first_bit, _find_first_bit, "", "");
ffi_export::export_symbol!(_find_first_and_bit, _find_first_and_bit, "", "");
ffi_export::export_symbol!(_find_first_andnot_bit, _find_first_andnot_bit, "", "");
ffi_export::export_symbol!(_find_first_and_and_bit, _find_first_and_and_bit, "", "");
ffi_export::export_symbol!(_find_first_zero_bit, _find_first_zero_bit, "", "");
ffi_export::export_symbol!(_find_next_bit, _find_next_bit, "", "");
ffi_export::export_symbol!(__find_nth_bit, __find_nth_bit, "", "");
ffi_export::export_symbol!(__find_nth_and_bit, __find_nth_and_bit, "", "");
ffi_export::export_symbol!(__find_nth_and_andnot_bit, __find_nth_and_andnot_bit, "", "");
ffi_export::export_symbol!(_find_next_and_bit, _find_next_and_bit, "", "");
ffi_export::export_symbol!(_find_next_andnot_bit, _find_next_andnot_bit, "", "");
ffi_export::export_symbol!(_find_next_or_bit, _find_next_or_bit, "", "");
ffi_export::export_symbol!(_find_next_zero_bit, _find_next_zero_bit, "", "");
ffi_export::export_symbol!(_find_last_bit, _find_last_bit, "", "");
ffi_export::export_symbol!(find_next_clump8, find_next_clump8, "", "");
ffi_export::export_symbol!(find_random_bit, find_random_bit, "", "");
