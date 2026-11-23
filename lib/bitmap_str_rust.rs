// SPDX-License-Identifier: GPL-2.0-only
//! Native kernel crate for the bitmap string parsing and printing helpers.
//!
//! The distinct crate basename lets Kbuild select the Rust implementation
//! without ambiguity with the retained `bitmap-str.c` translation unit.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "bitmap-str.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(bitmap_parse_user, bitmap_parse_user, "", "");
ffi_export::export_symbol!(bitmap_print_bitmask_to_buf, bitmap_print_bitmask_to_buf, "", "");
ffi_export::export_symbol!(bitmap_print_list_to_buf, bitmap_print_list_to_buf, "", "");
ffi_export::export_symbol!(bitmap_parselist, bitmap_parselist, "", "");
ffi_export::export_symbol!(bitmap_parselist_user, bitmap_parselist_user, "", "");
ffi_export::export_symbol!(bitmap_parse, bitmap_parse, "", "");
