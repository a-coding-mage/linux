// SPDX-License-Identifier: GPL-2.0
//! Native errno-name provider and unrestricted kernel export.

#[path = "../rust/ffi_export.rs"]
mod ffi_export;
#[path = "errname.rs"]
mod implementation;

pub use implementation::errname;
ffi_export::export_symbol!(errname, errname, "", "");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
