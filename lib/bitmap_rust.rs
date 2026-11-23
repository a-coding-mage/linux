// SPDX-License-Identifier: GPL-2.0-only
//! Native kernel crate for the bitmap.h helper functions.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `bitmap.c` translation unit.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "bitmap.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(__bitmap_equal, __bitmap_equal, "", "");
ffi_export::export_symbol!(__bitmap_or_equal, __bitmap_or_equal, "", "");
ffi_export::export_symbol!(__bitmap_complement, __bitmap_complement, "", "");
ffi_export::export_symbol!(__bitmap_shift_right, __bitmap_shift_right, "", "");
ffi_export::export_symbol!(__bitmap_shift_left, __bitmap_shift_left, "", "");
ffi_export::export_symbol!(bitmap_cut, bitmap_cut, "", "");
ffi_export::export_symbol!(__bitmap_and, __bitmap_and, "", "");
ffi_export::export_symbol!(__bitmap_or, __bitmap_or, "", "");
ffi_export::export_symbol!(__bitmap_xor, __bitmap_xor, "", "");
ffi_export::export_symbol!(__bitmap_andnot, __bitmap_andnot, "", "");
ffi_export::export_symbol!(__bitmap_replace, __bitmap_replace, "", "");
ffi_export::export_symbol!(__bitmap_intersects, __bitmap_intersects, "", "");
ffi_export::export_symbol!(__bitmap_subset, __bitmap_subset, "", "");
ffi_export::export_symbol!(__bitmap_weight, __bitmap_weight, "", "");
ffi_export::export_symbol!(__bitmap_weight_and, __bitmap_weight_and, "", "");
ffi_export::export_symbol!(__bitmap_weight_andnot, __bitmap_weight_andnot, "", "");
ffi_export::export_symbol!(__bitmap_weighted_or, __bitmap_weighted_or, "", "");
ffi_export::export_symbol!(__bitmap_weighted_xor, __bitmap_weighted_xor, "", "");
ffi_export::export_symbol!(__bitmap_set, __bitmap_set, "", "");
ffi_export::export_symbol!(__bitmap_clear, __bitmap_clear, "", "");
ffi_export::export_symbol!(bitmap_find_next_zero_area_off, bitmap_find_next_zero_area_off, "", "");
ffi_export::export_symbol!(bitmap_remap, bitmap_remap, "", "");
ffi_export::export_symbol!(bitmap_bitremap, bitmap_bitremap, "", "");
ffi_export::export_symbol!(bitmap_alloc, bitmap_alloc, "", "");
ffi_export::export_symbol!(bitmap_zalloc, bitmap_zalloc, "", "");
ffi_export::export_symbol!(bitmap_alloc_node, bitmap_alloc_node, "", "");
ffi_export::export_symbol!(bitmap_zalloc_node, bitmap_zalloc_node, "", "");
ffi_export::export_symbol!(bitmap_free, bitmap_free, "", "");
ffi_export::export_symbol!(devm_bitmap_alloc, devm_bitmap_alloc, "GPL", "");
ffi_export::export_symbol!(devm_bitmap_zalloc, devm_bitmap_zalloc, "GPL", "");
ffi_export::export_symbol!(bitmap_from_arr32, bitmap_from_arr32, "", "");
ffi_export::export_symbol!(bitmap_to_arr32, bitmap_to_arr32, "", "");
