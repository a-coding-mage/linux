// SPDX-License-Identifier: GPL-2.0
//! Native owner of the stable list_sort C ABI and unrestricted export.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "list_sort.rs"]
mod implementation;

pub use implementation::{declarations, list_sort};
ffi_export::export_symbol!(list_sort, list_sort, "", "");
