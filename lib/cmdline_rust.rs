// SPDX-License-Identifier: GPL-2.0-only
//! Native owner of the allocation-free command-line helpers.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "cmdline.rs"]
mod implementation;

pub use implementation::*;

ffi_export::export_symbol!(get_option, get_option, "", "");
ffi_export::export_symbol!(get_options, get_options, "", "");
ffi_export::export_symbol!(memparse, memparse, "", "");
ffi_export::export_symbol!(next_arg, next_arg, "", "");
