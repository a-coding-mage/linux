// SPDX-License-Identifier: GPL-2.0
//! Native argument-vector owner and the original unrestricted C exports.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "argv_split.rs"]
mod implementation;

pub use implementation::{argv_free, argv_split};

ffi_export::export_symbol!(argv_free, argv_free, "", "");
ffi_export::export_symbol!(argv_split, argv_split, "", "");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
