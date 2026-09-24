// SPDX-License-Identifier: GPL-2.0
//! Native owner of the windowed min/max C ABI and unrestricted exports.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "win_minmax.rs"]
mod implementation;

pub use implementation::{declarations, minmax_running_max, minmax_running_min};
ffi_export::export_symbol!(minmax_running_max, minmax_running_max, "", "");
ffi_export::export_symbol!(minmax_running_min, minmax_running_min, "", "");
