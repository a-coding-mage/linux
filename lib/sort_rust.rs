// SPDX-License-Identifier: GPL-2.0
//! Native owner of all four original sort exports.
#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "sort.rs"]
mod implementation;
pub use implementation::{declarations, sort, sort_nonatomic, sort_r, sort_r_nonatomic};
ffi_export::export_symbol!(sort_r, sort_r, "", "");
ffi_export::export_symbol!(sort_r_nonatomic, sort_r_nonatomic, "", "");
ffi_export::export_symbol!(sort, sort, "", "");
ffi_export::export_symbol!(sort_nonatomic, sort_nonatomic, "", "");
